use crate::ipc_socket::DiscordIpcSocket;
use crate::models::receive::{events::ReturnedEvent, ReceivedItem};
use crate::models::send::commands::{AuthenticateArgs, SentCommand};
use crate::models::shared::User;
use crate::opcodes::OpCodes;
use crate::{create_packet_json, Result};

use serde_json::json;

// Environment keys to search for the Discord pipe

#[allow(dead_code)]
#[allow(missing_docs)]
/// A wrapper struct for the functionality contained in the
/// underlying [`DiscordIpc`](trait@DiscordIpc) trait.
pub struct DiscordIpcClient {
    /// Client ID of the IPC client.
    pub client_id: String,
    pub access_token: String,
    // Socket ref to the open socket
    socket: DiscordIpcSocket,
}

impl DiscordIpcClient {
    /// Returns a pair of your newly constructed client and the self user
    pub async fn create(client_id: &str, access_token: &str) -> Result<(DiscordIpcClient, User)> {
        let socket = DiscordIpcSocket::new().await?;

        let mut client = Self {
            client_id: client_id.to_string(),
            access_token: access_token.to_owned(),
            socket,
        };

        // Create connection to the Discord client and obtain the self user
        let self_user = client.create_connection().await?;

        // Use the access token to authenticate the client
        client.authenticate(access_token).await.ok();

        Ok((client, self_user))
    }

    /// Connects the client to the Discord IPC
    ///
    /// This method attempts to first establish a connection,
    /// and then sends a handshake
    async fn create_connection(&mut self) -> Result<User> {
        self.send_handshake().await?;

        let (_opcode, payload) = self.socket.recv().await?;
        let mut self_user: Option<User> = None;

        // spooky line is not working
        let payload = serde_json::from_str(&payload)?;
        match payload {
            ReturnedEvent::Ready(data) => {
                self_user = Some(data.user);
            }
            _ => {
                println!("Could not connect to discord...");
            }
        }

        Ok(self_user.unwrap())
    }

    /// Handshakes the Discord IPC.
    ///
    /// Returns an `Err` variant if sending the handshake failed.
    async fn send_handshake(&mut self) -> Result<()> {
        self.socket
            .send(
                &json!({
                  "v": 1,
                  "client_id": self.client_id
                })
                .to_string(),
                OpCodes::Handshake as u8,
            )
            .await?;

        // // TODO: Return an Err if the handshake is rejected
        // NOTE: this prolly shouldnt be done here as we dont want to consume messages here
        // self.recv()?;

        Ok(())
    }

    /// Send auth
    ///
    /// This method sends the auth token to the IPC.
    ///
    /// Returns an `Err` variant if sending the handshake failed.
    pub async fn authenticate(&mut self, access_token: &str) -> Result<()> {
        let command = SentCommand::Authenticate(AuthenticateArgs {
            access_token: access_token.to_string(),
        });

        self.emit_command(&command).await?;

        self.socket.recv().await?;

        Ok(())
    }

    /// send a json string payload to the socket
    pub async fn emit_string(&mut self, payload: String) -> Result<()> {
        self.socket
            .send(&payload, OpCodes::Frame as u8)
            .await
            .unwrap();
        Ok(())
    }

    /// send a json string payload to the socket
    pub async fn emit_command(&mut self, command: &SentCommand) -> Result<()> {
        let mut command_json = command.to_json()?;
        let json_string = &create_packet_json(&mut command_json)?;
        self.socket
            .send(json_string, OpCodes::Frame as u8)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn setup_event_handler<F>(&mut self, func: F)
    where
        F: Fn(ReceivedItem) + Send + Sync + 'static,
    {
        let mut socket_clone = self.socket.clone();
        tokio::spawn(async move {
            loop {
                let (_opcode, payload) = socket_clone.recv().await.unwrap();

                match serde_json::from_str::<ReceivedItem>(&payload) {
                    Ok(e) => {
                        // TODO: give the consumer a ready event so they can sub to events
                        func(e);
                    }
                    Err(err) => {
                        println!("Unable to deserialize {:?}\n{:?}", err, payload);
                    }
                }
            }
        });
    }
}
