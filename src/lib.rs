pub mod utils;

pub mod errors;
pub mod models;
pub mod opcodes;

pub mod ipc;
pub mod ipc_socket;

use models::receive::{commands::ReturnedCommand, events::ReturnedEvent};
use serde::{Deserialize, Serialize};

use errors::DiscordRPCError;
pub use ipc::DiscordIpcClient;
pub use utils::*;

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;

/// Currently this is used to allow for matching of an event or type
/// Not all events/commands are implemented so serializing can fail
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ReceivedItem {
    Event(Box<ReturnedEvent>),
    Command(Box<ReturnedCommand>),
}
