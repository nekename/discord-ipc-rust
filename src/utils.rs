use crate::Result;

use std::collections::HashSet;
use std::convert::TryInto;
use std::path::{Path, PathBuf};

use serde_json::Value;
use uuid::Uuid;

pub fn create_packet_json(value: &mut serde_json::Value) -> Result<String> {
    let uuid = Uuid::new_v4().to_string();

    let payload = value.as_object_mut().expect("payload must be an object");
    payload.insert("nonce".to_string(), Value::String(uuid));

    Ok(serde_json::to_string(&payload)?)
}

pub fn pack(opcode: u32, data_len: u32) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    for byte_array in &[opcode.to_le_bytes(), data_len.to_le_bytes()] {
        bytes.extend_from_slice(byte_array);
    }

    Ok(bytes)
}

pub fn unpack(data: Vec<u8>) -> Result<(u32, u32)> {
    let data = data.as_slice();
    let (opcode, header) = data.split_at(std::mem::size_of::<u32>());

    let opcode = u32::from_le_bytes(opcode.try_into()?);
    let header = u32::from_le_bytes(header.try_into()?);

    Ok((opcode, header))
}

/// Finds the active pipe path of the RPC server
pub fn get_pipe_path() -> Option<PathBuf> {
    let possible_paths = get_os_pipe_paths();

    for i in 0..10 {
        for p in &possible_paths {
            let path: String = format!("{}{}", p, i);
            let path = Path::new(&path);

            if path.exists() {
                return Some(path.to_path_buf());
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn get_os_pipe_paths() -> HashSet<String> {
    let mut possible_paths = HashSet::new();
    possible_paths.insert(r"\\?\pipe\discord-ipc-".to_owned());
    possible_paths
}

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
fn get_os_pipe_paths() -> HashSet<String> {
    let mut possible_paths = HashSet::new();
    possible_paths.insert("/tmp/discord-ipc-".to_owned());

    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        possible_paths.insert(runtime_dir.clone() + "/app/com.discordapp.Discord/discord-ipc-"); // Discord installed via Flatpak
        possible_paths.insert(runtime_dir + "/discord-ipc-");
    }

    possible_paths
}

#[cfg(target_os = "macos")]
fn get_os_pipe_paths() -> HashSet<String> {
    let mut possible_paths = HashSet::new();
    if let Ok(runtime_dir) = std::env::var("TMPDIR") {
        possible_paths.insert(runtime_dir + "/discord-ipc-");
    }
    possible_paths
}
