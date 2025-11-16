pub mod utils;

pub mod errors;
pub mod models;
pub mod opcodes;

pub mod ipc;
pub mod ipc_socket;

use errors::DiscordRPCError;
pub use ipc::DiscordIpcClient;
pub use utils::*;

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;
