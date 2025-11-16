use crate::models::shared::{Guild, Message, User, voice::VoicePan};

use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/topics/rpc#ready-rpc-server-configuration-object>
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyConfig {
    /// string - server's cdn
    pub cdn_host: String,
    /// string - server's api endpoint
    pub api_endpoint: String,
    /// string - server's environment
    pub environment: String,
}

/// <https://discord.com/developers/docs/topics/rpc#ready-ready-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyData {
    #[serde(rename = "v")]
    /// integer - RPC version
    pub version: u32,
    /// rpc server configuration object - server configuration
    pub config: ReadyConfig,
    /// partial user object - the user to whom you are connected
    pub user: User,
}

/// <https://discord.com/developers/docs/topics/rpc#error-error-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
    /// integer - RPC Error Code
    pub code: u32,
    /// string - Error description
    pub message: String,
}

/// <https://discord.com/developers/docs/topics/rpc#guildstatus-guild-status-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct GuildStatusData {
    /// partial guild object - guild with requested id
    pub guild: Guild,
    /// integer - number of online users in guild (deprecated; always 0)
    pub online: i32,
}

/// <https://discord.com/developers/docs/topics/rpc#guildcreate-guild-create-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct GuildCreateData {
    /// string - guild id
    pub id: String,
    /// string - name of the guild
    pub name: String,
}

/// <https://discord.com/developers/docs/topics/rpc#channelcreate-channel-create-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelCreateData {
    /// string - channel id
    pub id: String,
    /// string - name of the channel
    pub name: String,
    /// integer - channel type (guild text: 0, guild voice: 2, dm: 1, group dm: 3)
    #[serde(rename = "type")]
    pub channel_type: u8,
}

/// <https://discord.com/developers/docs/topics/rpc#voicechannelselect-voice-channel-select-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceChannelSelectData {
    /// string - id of channel (null if none)
    pub channel_id: String,
    /// string - id of guild (null if none)
    pub guild_id: String,
}

/// <https://discord.com/developers/docs/resources/voice#voice-state-object-voice-state-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceState {
    /// boolean - whether this user is deafened by the server
    pub deaf: bool,
    /// boolean - whether this user is muted by the server
    pub mute: bool,
    /// boolean - whether this user is locally deafened
    pub self_deaf: bool,
    /// boolean - whether this user is locally muted
    pub self_mute: bool,
    /// boolean - whether this user is streaming using "Go Live"
    pub self_stream: Option<bool>,
    /// boolean - whether this user's camera is enabled
    pub self_video: Option<bool>,
    /// boolean - whether this user's permission to speak is denied
    pub suppress: bool,
}

/// <https://discord.com/developers/docs/topics/rpc#voicestatecreatevoicestateupdatevoicestatedelete-example-voice-state-dispatch-payload>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceStateData {
    /// voice state object
    #[serde(rename = "voice_state")]
    pub state: VoiceState,
    /// partial user object
    pub user: Option<User>,
    /// string
    pub nick: String,
    /// float
    pub volume: f32,
    /// boolean
    pub mute: bool,
    /// pan object
    pub pan: VoicePan,
}

/// <https://discord.com/developers/docs/topics/rpc#voiceconnectionstatus-voice-connection-states>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceConnectionState {
    Disconnected,
    AwaitingEndpoint,
    Authenticating,
    Connecting,
    Connected,
    VoiceDisconnected,
    VoiceConnecting,
    VoiceConnected,
    NoRoute,
    IceChecking,
}

/// <https://discord.com/developers/docs/topics/rpc#voiceconnectionstatus-voice-connection-status-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceConnectionStatusData {
    /// string - one of the voice connection states listed below
    pub state: VoiceConnectionState,
    /// string - hostname of the connected voice server
    pub hostname: String,
    /// array of integers - last 20 pings (in ms)
    pub pings: Vec<i32>,
    /// integer (adjusted to f64 to match the float value in the example JSON) - average ping (in ms)
    pub average_ping: f64,
    /// integer - last ping (in ms)
    pub last_ping: Option<u64>,
}

/// <https://discord.com/developers/docs/topics/rpc#messagecreatemessageupdatemessagedelete-message-argument-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageData {
    /// string - channel id
    pub channel_id: String,
    /// message object
    pub message: Message,
}

/// <https://discord.com/developers/docs/topics/rpc#speakingstartspeakingstop-speaking-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct SpeakingData {
    /// string - id of user who started/stopped speaking
    pub user_id: String,
}

/// <https://discord.com/developers/docs/topics/rpc#notificationcreate-notification-create-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationCreateData {
    /// string - id of channel where notification occurred
    pub channel_id: String,
    /// message object - message that generated this notification
    pub message: Message,
    /// string - icon url of the notification
    pub icon_url: String,
    /// string - title of the notification
    pub title: String,
    /// string - body of the notification
    pub body: String,
}

/// <https://discord.com/developers/docs/topics/rpc#activityjoin-activity-join-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityJoinData {
    /// string - the join_secret for the given invite
    pub secret: String,
}

/// <https://discord.com/developers/docs/topics/rpc#activityspectate-activity-spectate-dispatch-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitySpectateData {
    /// string - the spectate_secret for the given invite
    pub secret: String,
}

/// <https://discord.com/developers/docs/topics/rpc#activityjoinrequest-activity-join-request-data-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityJoinRequestData {
    /// partial user object - information about the user requesting to join
    pub user: User,
}
