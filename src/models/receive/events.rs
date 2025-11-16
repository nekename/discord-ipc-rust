pub use super::data::*;

use crate::models::shared::voice::VoiceSettings;

use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/topics/rpc#commands-and-events-rpc-events>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "data")]
pub enum ReturnedEvent {
    Ready(ReadyData),
    Error(ErrorData),
    GuildStatus(GuildStatusData),
    GuildCreate(GuildCreateData),
    ChannelCreate(ChannelCreateData),
    VoiceChannelSelect(VoiceChannelSelectData),
    VoiceSettingsUpdate(VoiceSettings),
    VoiceStateCreate(VoiceStateData),
    VoiceStateUpdate(VoiceStateData),
    VoiceStateDelete(VoiceStateData),
    VoiceConnectionStatus(VoiceConnectionStatusData),
    MessageCreate(MessageData),
    MessageUpdate(MessageData),
    MessageDelete(MessageData),
    SpeakingStart(SpeakingData),
    SpeakingStop(SpeakingData),
    NotificationCreate(NotificationCreateData),
    ActivityJoin(ActivityJoinData),
    ActivitySpectate(ActivitySpectateData),
    ActivityJoinRequest(ActivityJoinRequestData),
}
