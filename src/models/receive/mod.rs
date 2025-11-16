pub mod commands;
pub mod events;

mod data;

/// Represents values received from the RPC server, either events or command responses
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum ReceivedItem {
    Event(Box<events::ReturnedEvent>),
    Command(Box<commands::ReturnedCommand>),
}
