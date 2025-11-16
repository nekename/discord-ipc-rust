mod ipc;
mod ipc_socket;
mod utils;

pub mod models;

pub use ipc::DiscordIpcClient;

#[derive(thiserror::Error, Debug)]
pub enum DiscordRPCError {
    #[error("Could not find the IPC pipe")]
    PipeNotFound,
    #[error("Could not connect to Discord")]
    CouldNotConnect,
    #[error("Failed to convert from slice")]
    TryFromSlice(#[from] std::array::TryFromSliceError),
    #[error("An I/O error occurred")]
    Io(#[from] tokio::io::Error),
    #[error("Failed to convert UTF-8 bytes to String")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("A serde_json error occurred")]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T, E = DiscordRPCError> = std::result::Result<T, E>;
