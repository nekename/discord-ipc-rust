use crate::models::shared::{
    Channel, Guild, User,
    voice::{UserVoiceSettings, VoiceSettings},
};

use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/topics/rpc#commands-and-events-rpc-commands>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "data")]
pub enum ReturnedCommand {
    Authorize { code: String },
    Authenticate(AuthenticateData),
    GetGuild(GetGuildData),
    GetGuilds(Vec<Guild>),
    GetChannel(Channel),
    GetChannels(Vec<Channel>),
    Subscribe { evt: String },
    Unsubscribe { evt: String },
    SetUserVoiceSettings(UserVoiceSettings),
    SelectVoiceChannel(Option<Channel>),
    GetSelectedVoiceChannel(Option<Channel>),
    SelectTextChannel(Option<Channel>),
    GetVoiceSettings(VoiceSettings),
    SetVoiceSettings(VoiceSettings),
    SetCertifiedDevices, // Restricted to hardware manufacturers
    SetActivity,         // TODO
    SendActivityJoinInvite,
    CloseActivityRequest,
}

/// <https://discord.com/developers/docs/topics/rpc#authenticate-oauth2-application-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct OAuth2Application {
    /// string - application description
    pub description: String,
    /// string - hash of the icon
    pub icon: Option<String>,
    /// snowflake - application client id
    pub id: String,
    /// array of strings - array of rpc origin urls
    pub rpc_origins: Option<Vec<String>>,
    /// string - application name
    pub name: String,
}

/// <https://discord.com/developers/docs/topics/rpc#authenticate-authenticate-response-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticateData {
    /// partial user object - the authed user
    pub user: User,
    /// array of OAuth2 scopes - authorized scopes
    pub scopes: Vec<String>,
    /// date - expiration date of OAuth2 token
    pub expires: String,
    /// OAuth2 application object - application the user authorized
    pub application: OAuth2Application,
}

/// <https://discord.com/developers/docs/topics/rpc#getguild-get-guild-response-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildData {
    /// string - guild id
    pub id: String,
    /// string - guild name
    pub name: String,
    /// string - guild icon url
    pub icon_url: String,
}
