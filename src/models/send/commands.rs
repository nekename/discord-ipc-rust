use super::events::SubscribeableEvent;

use crate::{models::shared::SetUserVoiceSettingsData, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "args")]
pub enum SentCommand {
    Dispatch(SubscribeableEvent), // ???
    Authorize(AuthorizeData),
    Authenticate { access_token: String },
    GetGuild(GetGuildData),
    GetGuilds, // No args
    GetChannel { channel_id: String },
    GetChannels { guild_id: String },
    Subscribe(SubscribeableEvent),
    Unsubscribe(SubscribeableEvent),
    SetUserVoiceSettings(SetUserVoiceSettingsData),
    SelectVoiceChannel(SelectVoiceChannelData),
    GetSelectedVoiceChannel, // No args
    SelectTextChannel(SelectTextChannelData),
    GetVoiceSettings,       // No args
    SetVoiceSettings,       // !!! CURRENTLY UNSUPPORTED DUE TO LIMITATIONS IMPOSED BY DISCORD !!!
    SetCertifiedDevices,    // !!! CURRENTLY UNSUPPORTED DUE TO A LACK OF DOCUMENTATION !!!
    SetActivity,            // TODO
    SendActivityJoinInvite, // TODO
    CloseActivityRequest,   // TODO
}

impl SentCommand {
    pub(crate) fn to_json(&self) -> Result<Value> {
        let command_json = match self {
            Self::Dispatch(event) => {
                let mut event_json = serde_json::to_value(event)?;
                match &mut event_json {
                    serde_json::Value::Object(object) => {
                        object.insert("cmd".to_string(), "DISPATCH".into());
                        object
                    }
                    _ => panic!("Expected event to be an object"),
                };
                event_json
            }

            Self::Subscribe(event) => {
                let mut event_json = serde_json::to_value(event)?;
                match &mut event_json {
                    serde_json::Value::Object(object) => {
                        object.insert("cmd".to_string(), "SUBSCRIBE".into());
                        object
                    }
                    _ => panic!("Expected event to be an object"),
                };
                event_json
            }

            Self::Unsubscribe(event) => {
                let mut event_json = serde_json::to_value(event)?;
                match &mut event_json {
                    serde_json::Value::Object(object) => {
                        object.insert("cmd".to_string(), "UNSUBSCRIBE".into());
                        object
                    }
                    _ => panic!("Expected event to be an object"),
                };
                event_json
            }

            _ => serde_json::to_value(self)?,
        };

        Ok(command_json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeData {
    /// Array of OAuth2 scopes - Scopes to authorize
    pub scopes: Vec<String>,
    /// string - OAuth2 application ID
    pub client_id: String,
    /// string - One-time use RPC token
    pub rpc_token: String,
    /// string - Username to create a guest account with if the user does not have Discord
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildData {
    /// string - Guild ID
    pub guild_id: String,
    /// integer - Asynchronously get guild with time to wait before timing out
    pub timeout: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectVoiceChannelData {
    /// string - Channel ID
    pub channel_id: String,
    /// integer - Asynchronously join voice channel with time to wait before timing out
    pub timeout: i32,
    /// boolean - Forces a user to join a voice channel
    pub force: bool,
    /// boolean - After joining the voice channel, navigate to it in the client
    pub navigate: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectTextChannelData {
    /// string - Channel ID
    pub channel_id: String,
    /// integer - Asynchronously join text channel with time to wait before timing out
    pub timeout: i32,
}
