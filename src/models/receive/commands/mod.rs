use crate::models::shared::{channel::Channel, Guild, SetUserVoiceSettingsData, User};

use serde::{Deserialize, Serialize};

/// All command responses that come back from the discord RPC
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "data")]
pub enum ReturnedCommand {
    Authorize { code: String },
    Authenticate(AuthenticateData),
    GetGuild(Guild),
    GetGuilds(Vec<Guild>),
    GetChannel(Channel),
    GetChannels(Vec<Channel>),
    Subscribe { evt: String },
    Unsuscribe { evt: String },
    SetUserVoiceSettings(SetUserVoiceSettingsData),
    SelectVoiceChannel(Channel),
    GetSelectedVoiceChannel(Option<Channel>),
    SelectTextChannel(Channel),
    GetVoiceSettings,       // TODO
    SetVoiceSettings,       // !!! CURRENTLY UNSUPPORTED DUE TO LIMITATIONS IMPOSED BY DISCORD !!!
    SetCertifiedDevices,    // !!! CURRENTLY UNSUPPORTED DUE TO A LACK OF DOCUMENTATION !!!
    SetActivity,            // TODO
    SendActivityJoinInvite, // TODO
    CloseActivityRequest,   // TODO
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticateData {
    /// partial user object - The authed user
    pub user: User,
    /// array of oauth2 scopes - Authorized scopes
    pub scopes: Vec<String>,
    /// date - Expiration date of OAuth2 token
    pub expires: String,
    /// oauth2 application object - Application the user authorized
    pub application: OAuth2Application,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuth2Application {
    pub description: String,
    pub icon: String,
    pub id: String,
    pub rpc_origins: Vec<String>,
    pub name: String,
}
