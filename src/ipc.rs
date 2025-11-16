use crate::ipc_socket::DiscordIpcSocket;
use crate::models::receive::{events::ReturnedEvent, ReceivedItem};
use crate::models::send::commands::{AuthenticateArgs, SentCommand};
use crate::models::shared::User;
use crate::utils::create_packet_json;
use crate::{DiscordRPCError, Result};

use serde_json::json;

#[allow(dead_code)]
enum OpCodes {
    Handshake,
    Frame,
    Close,
    Ping,
    Pong,
}

pub struct DiscordIpcClient {
    pub client_id: String,
    socket: DiscordIpcSocket,
}

impl DiscordIpcClient {
    /// Returns a newly constructed client and the active Discord user
    pub async fn create(client_id: String) -> Result<(DiscordIpcClient, User)> {
        let socket = DiscordIpcSocket::new().await?;
        let mut client = Self { client_id, socket };

        client
            .socket
            .send(
                &json!({ "v": 1, "client_id": client.client_id }).to_string(),
                OpCodes::Handshake as u8,
            )
            .await?;
        let (_opcode, payload) = client.socket.recv().await?;
        let payload = serde_json::from_str(&payload)?;

        match payload {
            ReturnedEvent::Ready(data) => Ok((client, data.user)),
            _ => Err(DiscordRPCError::CouldNotConnect),
        }
    }

    /// Authenticate with the RPC server using an OAuth2 access token
    /// This method will hang if called after setup_event_handler
    pub async fn authenticate(&mut self, access_token: String) -> Result<()> {
        let command = SentCommand::Authenticate(AuthenticateArgs { access_token });
        self.emit_command(&command).await?;
        self.socket.recv().await?;
        Ok(())
    }

    /// Send an arbitrary JSON string payload to the RPC server
    pub async fn emit_string(&mut self, payload: &str) -> Result<()> {
        self.socket.send(payload, OpCodes::Frame as u8).await
    }

    /// Send a command to the RPC server
    pub async fn emit_command(&mut self, command: &SentCommand) -> Result<()> {
        let mut command_json = command.to_json()?;
        let json_string = create_packet_json(&mut command_json)?;
        self.emit_string(&json_string).await
    }

    /// Set up an event handler that will be called whenever a value is received from the RPC server
    pub async fn setup_event_handler<F>(&mut self, func: F)
    where
        F: Fn(ReceivedItem) + Send + Sync + 'static,
    {
        let mut socket_clone = self.socket.clone();
        tokio::spawn(async move {
            loop {
                let (_opcode, payload) = socket_clone.recv().await.unwrap();
                match serde_json::from_str::<ReceivedItem>(&payload) {
                    Ok(item) => func(item),
                    Err(error) => eprintln!("Failed to deserialize payload {}: {}", payload, error),
                }
            }
        });
    }
}
