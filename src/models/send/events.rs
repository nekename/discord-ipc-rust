use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/topics/rpc#commands-and-events-rpc-events>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "args")]
pub enum SubscribeableEvent {
    GuildStatus { guild_id: String },
    GuildCreate,
    ChannelCreate,
    VoiceChannelSelect,
    VoiceStateCreate { channel_id: String },
    VoiceStateUpdate { channel_id: String },
    VoiceStateDelete { channel_id: String },
    VoiceSettingsUpdate,
    VoiceConnectionStatus,
    SpeakingStart { channel_id: String },
    SpeakingStop { channel_id: String },
    MessageCreate { channel_id: String },
    MessageUpdate { channel_id: String },
    MessageDelete { channel_id: String },
    NotificationCreate,
    ActivityJoin,
    ActivitySpectate,
    ActivityJoinRequest,
}
