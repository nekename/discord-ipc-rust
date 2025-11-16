use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
pub struct VoicePan {
    /// integer
    pub left: u8,
    /// integer
    pub right: u8,
}

/// <https://discord.com/developers/docs/topics/rpc#setuservoicesettings-set-user-voice-settings-argument-and-response-structure>
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserVoiceSettings {
    /// string - user id
    pub user_id: String,
    /// pan object - set the pan of the user
    pub pan: Option<VoicePan>,
    /// integer - set the volume of user (defaults to 100, min 0, max 200)
    pub volume: Option<i32>,
    /// boolean - set the mute state of the user
    pub mute: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceAvailableDevice {
    /// string
    pub id: String,
    /// string
    pub name: String,
}

/// <https://discord.com/developers/docs/topics/rpc#getvoicesettings-voice-settings-input-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceSettingsInput {
    /// string - device id
    pub device_id: String,
    /// float - input voice level (min: 0, max: 100)
    pub volume: f32,
    /// array of objects - array of read-only device objects containing id and name string keys
    pub available_devices: Vec<VoiceAvailableDevice>,
}

/// <https://discord.com/developers/docs/topics/rpc#getvoicesettings-voice-settings-output-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceSettingsOutput {
    /// string - device id
    pub device_id: String,
    /// float - output voice level (min: 0, max: 200)
    pub volume: f32,
    /// array of objects - array of read-only device objects containing id and name string keys
    pub available_devices: Vec<VoiceAvailableDevice>,
}

/// <https://discord.com/developers/docs/topics/rpc#getvoicesettings-voice-settings-mode-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceSettingsMode {
    /// string - voice setting mode type (can be PUSH_TO_TALK or VOICE_ACTIVITY)
    #[serde(rename = "type")]
    pub mode_type: String,
    /// boolean - voice activity threshold automatically sets its threshold
    pub auto_threshold: bool,
    /// float - threshold for voice activity (in dB) (min: -100, max: 0)
    pub threshold: f32,
    /// float - the PTT release delay (in ms) (min: 0, max: 2000)
    pub delay: f32,
}

/// <https://discord.com/developers/docs/topics/rpc#getvoicesettings-get-voice-settings-response-structure>
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VoiceSettings {
    /// voice settings input object - input settings
    pub input: Option<VoiceSettingsInput>,
    /// voice settings output object - output settings
    pub output: Option<VoiceSettingsOutput>,
    /// voice settings mode object - voice mode settings
    pub mode: Option<VoiceSettingsMode>,
    /// boolean - state of automatic gain control
    pub automatic_gain_control: Option<bool>,
    /// boolean - state of echo cancellation
    pub echo_cancellation: Option<bool>,
    /// boolean - state of noise suppression
    pub noise_suppression: Option<bool>,
    /// boolean - state of voice quality of service
    pub qos: Option<bool>,
    /// boolean - state of silence warning notice
    pub silence_warning: Option<bool>,
    /// boolean - state of self-deafen
    pub deaf: Option<bool>,
    /// boolean - state of self-mute
    pub mute: Option<bool>,
}
