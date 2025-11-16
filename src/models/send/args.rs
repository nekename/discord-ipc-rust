use crate::models::shared::voice::{UserVoiceSettings, VoiceSettings};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// <https://discord.com/developers/docs/topics/rpc#authorize-authorize-argument-structure>
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeArgs {
    /// array of OAuth2 scopes - scopes to authorize
    pub scopes: Vec<String>,
    /// string - OAuth2 application ID
    pub client_id: String,
    /// string - one-time use RPC token
    pub rpc_token: Option<String>,
    /// string - username to create a guest account with if the user does not have Discord
    pub username: Option<String>,
}

/// <https://discord.com/developers/docs/topics/rpc#authenticate-authenticate-argument-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticateArgs {
    /// string - OAuth2 access token
    pub access_token: String,
}

/// <https://discord.com/developers/docs/topics/rpc#getguild-get-guild-argument-structure>
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildArgs {
    /// string - id of the guild to get
    pub guild_id: String,
    /// integer - asynchronously get guild with time to wait before timing out
    pub timeout: Option<i32>,
}

/// <https://discord.com/developers/docs/topics/rpc#getchannel-get-channel-argument-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChannelArgs {
    /// string - id of the channel to get
    pub channel_id: String,
}

/// <https://discord.com/developers/docs/topics/rpc#getchannels-get-channels-argument-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChannelsArgs {
    /// string - id of the guild to get channels for
    pub guild_id: String,
}

/// <https://discord.com/developers/docs/topics/rpc#setuservoicesettings-set-user-voice-settings-argument-and-response-structure>
pub type SetUserVoiceSettingsArgs = UserVoiceSettings;

/// <https://discord.com/developers/docs/topics/rpc#selectvoicechannel-select-voice-channel-argument-structure>
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct SelectVoiceChannelArgs {
    /// string - channel id to join (or null to leave)
    #[serialize_always]
    pub channel_id: Option<String>,
    /// integer - asynchronously join channel with time to wait before timing out
    pub timeout: Option<i32>,
    /// boolean - forces a user to join a voice channel
    pub force: Option<bool>,
    /// boolean - after joining the voice channel, navigate to it in the client
    pub navigate: Option<bool>,
}

/// <https://discord.com/developers/docs/topics/rpc#selecttextchannel-select-text-channel-argument-structure>
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct SelectTextChannelArgs {
    /// string - channel id to join (or null to leave)
    #[serialize_always]
    pub channel_id: Option<String>,
    /// integer - asynchronously join channel with time to wait before timing out
    pub timeout: Option<i32>,
}

/// <https://discord.com/developers/docs/topics/rpc#setvoicesettings-set-voice-settings-argument-and-response-structure>
pub type SetVoiceSettingsArgs = VoiceSettings;

/// <https://discord.com/developers/docs/topics/rpc#sendactivityjoininvite-example-send-activity-join-invite-payload>
#[derive(Serialize, Deserialize, Debug)]
pub struct SendActivityJoinInviteArgs {
    /// snowflake - the id of the requesting user
    pub user_id: String,
}

/// <https://discord.com/developers/docs/topics/rpc#closeactivityrequest-close-activity-request-argument-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct CloseActivityRequestArgs {
    /// snowflake - the id of the requesting user
    pub user_id: String,
}
