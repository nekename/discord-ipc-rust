use discord_ipc_rust::DiscordIpcClient;
use discord_ipc_rust::models::receive::{
    ReceivedItem, commands::ReturnedCommand, events::ReturnedEvent,
};
use discord_ipc_rust::models::send::commands::SetVoiceSettingsArgs;
use discord_ipc_rust::models::send::{
    commands::{AuthorizeArgs, SentCommand},
    events::SubscribeableEvent,
};

#[tokio::main]
async fn main() {
    if let Err(error) = example().await {
        eprintln!("An error occurred: {}", error);
    }
}

async fn example() -> discord_ipc_rust::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    let client_id =
        dotenv::var("CLIENT_ID").expect("You must set the CLIENT_ID environment variable");
    let client_secret = dotenv::var("CLIENT_SECRET").ok();
    let access_token = dotenv::var("ACCESS_TOKEN").ok();
    if (client_secret.is_none() && access_token.is_none())
        || (client_secret.is_some() && access_token.is_some())
    {
        panic!("You must only set either the CLIENT_SECRET or ACCESS_TOKEN environment variable");
    }

    // Connect to the Discord RPC server
    let (mut rpc, user) = DiscordIpcClient::create(client_id.clone()).await?;
    println!("Connected to Discord as {}", user.username);

    if let Some(access_token) = access_token {
        // If we have an access token, demonstrate some features available to an authenticated client
        println!(
            "Access token environment variable present: demonstrating some features available to an authenticated client..."
        );
        rpc.authenticate(access_token).await?;
        rpc.setup_event_handler(handle_message).await; // setup_event_handler must be called after authenticate!

        // Ask Discord to notify us of updates to the voice settings and for the currently selected voice channel
        rpc.emit_command(&SentCommand::Subscribe(
            SubscribeableEvent::VoiceSettingsUpdate,
        ))
        .await?;
        rpc.emit_command(&SentCommand::GetSelectedVoiceChannel)
            .await?;
        println!("Subscribed to voice settings updates and requested selected voice channel");

        // Mute and unmute the user every 5 seconds to demonstrate changing voice settings
        println!("Toggling mute every 5 seconds...");
        let mut mute = false;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            mute = !mute;
            rpc.emit_command(&SentCommand::SetVoiceSettings(SetVoiceSettingsArgs {
                mute: Some(mute),
                ..Default::default()
            }))
            .await?;
        }
    } else {
        // If we do not have an access token, demonstrate the authorization process
        println!(
            "Access token environment variable not present: demonstrating the authorization process..."
        );

        rpc.setup_event_handler(|item| {
            let code = match &item {
                ReceivedItem::Command(command) => match &**command {
                    ReturnedCommand::Authorize { code } => code.clone(),
                    _ => return handle_message(item),
                },
                _ => return handle_message(item),
            };
            println!(
                "Received authorization code: {}, exchanging for access token using client secret",
                code
            );
            tokio::spawn(async move {
                if let Err(error) = exchange_code_for_token(&code).await {
                    eprintln!(
                        "Failed to exchange authorization code for access token: {}",
                        error
                    );
                }
                std::process::exit(0);
            });
        })
        .await;

        rpc.emit_command(&SentCommand::Authorize(AuthorizeArgs {
            client_id,
            scopes: vec!["rpc".to_owned(), "identify".to_owned()],
            rpc_token: None,
            username: None,
        }))
        .await?;
        println!("Requested authorization prompt to obtain OAuth2 authorization code");

        loop {
            // Keep the program alive to listen for authorization response
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

// Handle received messages from the RPC server
fn handle_message(item: ReceivedItem) {
    match item {
        ReceivedItem::Event(event) => match *event {
            ReturnedEvent::Error(error) => {
                eprintln!("Received error: {:?}", error);
            }
            ReturnedEvent::VoiceSettingsUpdate(voice) => {
                println!("Voice settings updated: muted: {}", voice.mute.unwrap())
            }
            _ => println!("Received other event: {:?}", event),
        },
        ReceivedItem::Command(command) => match *command {
            ReturnedCommand::Subscribe { evt } => println!("Subscribed to event: {}", evt),
            ReturnedCommand::GetSelectedVoiceChannel(channel) => match channel {
                None => println!("Not currently in a voice channel"),
                Some(channel) => println!("Current voice channel name: {}", channel.name.unwrap()),
            },
            ReturnedCommand::SetVoiceSettings(_) => println!("Voice settings update succeeded"),
            _ => println!("Received other command response: {:?}", command),
        },
    }
}

// Exchange an OAuth2 authorization code for an access token using the client secret
// Your app must have at least one redirect URI configured in the Discord Developer Portal for authentication to work
async fn exchange_code_for_token(code: &str) -> Result<(), reqwest::Error> {
    let params = [
        ("client_id", &dotenv::var("CLIENT_ID").unwrap()[..]),
        ("client_secret", &dotenv::var("CLIENT_SECRET").unwrap()),
        ("grant_type", "authorization_code"),
        ("code", code),
    ];
    let client = reqwest::Client::new();

    let response = client
        .post("https://discord.com/api/v10/oauth2/token")
        .form(&params)
        .send()
        .await?;
    let value = serde_json::from_str::<serde_json::Value>(&response.text().await?)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap();

    if let Some(access_token) = value.get("access_token").and_then(|v| v.as_str()) {
        println!("Obtained access token: {}", access_token);
        println!(
            "You can now set this value for the ACCESS_TOKEN environment variable to use it in the example"
        );
    } else if let Some(error) = value.get("error").and_then(|v| v.as_str()) {
        if let Some(error_description) = value.get("error_description").and_then(|v| v.as_str()) {
            eprintln!(
                "Failed to obtain access token: {}: {}",
                error, error_description
            );
        } else {
            eprintln!("Failed to obtain access token: {}", error);
        }
    } else {
        eprintln!(
            "Failed to obtain access token: unexpected response {:?}",
            value
        );
    }

    Ok(())
}
