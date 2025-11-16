use super::User;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/message#message-object-message-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u32)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
    PollResult = 46,
}

/// <https://discord.com/developers/docs/resources/message#message-object-message-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// snowflake - id of the message
    pub id: String,
    /// snowflake - id of the channel the message was sent in
    pub channel_id: String,
    /// user object - the author of this message (not guaranteed to be a valid user, see below)
    pub author: User,
    /// string - contents of the message
    pub content: Option<String>,
    /// ISO8601 timestamp - when this message was sent
    pub timestamp: String,
    /// ?ISO8601 timestamp - when this message was edited (or null if never)
    pub edited_timestamp: Option<String>,
    /// boolean - whether this was a TTS message
    pub tts: bool,
    /// boolean - whether this message mentions everyone
    pub mention_everyone: bool,
    /// integer - type of message
    #[serde(rename = "type")]
    pub message_type: MessageType,
}
