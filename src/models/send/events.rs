use serde::{Deserialize, Serialize};

// TODO: move this to somewhere else
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "args")]
pub enum SubscribeableEvent {
    GuildStatus { guild_id: String },
    GuildCreate,        // No args
    ChannelCreate,      // No args
    VoiceChannelSelect, // No args
    VoiceStateCreate { channel_id: String },
    VoiceStateUpdate { channel_id: String },
    VoiceStateDelete { channel_id: String },
    VoiceSettingsUpdate,   // No args
    VoiceConnectionStatus, // No args
    SpeakingStart { channel_id: String },
    SpeakingStop { channel_id: String },
    MessageCreate { channel_id: String },
    MessageUpdate { channel_id: String },
    MessageDelete { channel_id: String },
    NotificationCreate,  // No args
    ActivityJoin,        // No args
    ActivitySpectate,    // No args
    ActivityJoinRequest, // No args
}
