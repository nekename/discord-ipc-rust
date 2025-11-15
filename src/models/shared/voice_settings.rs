use super::voice_state::VoicePan;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SetUserVoiceSettingsData {
    /// string - User ID
    pub user_id: String,
    /// pan object - Set the pan of the user
    pub pan: Option<VoicePan>,
    /// integer - Set the volume of the user (defaults to 100, min 0, max 200)
    pub volume: Option<i32>,
    /// boolean - Set the mute state of the user
    pub mute: Option<bool>,
}
