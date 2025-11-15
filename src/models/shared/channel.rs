use super::{message::Message, voice_state::VoiceState};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub topic: String,
    pub bitrate: u32,
    pub user_limit: u32,
    pub guild_id: String,
    pub position: u32,
    pub voice_states: Vec<VoiceState>,
    pub messages: Vec<Message>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u32)]
pub enum ChannelType {
    GuildText = 0,
    DirectMessage = 1,
    GuildVoice = 2,
    GroupDirectMessage = 3,
}
