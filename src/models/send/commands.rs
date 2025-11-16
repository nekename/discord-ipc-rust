pub use super::args::*;

use super::events::SubscribeableEvent;

use crate::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// <https://discord.com/developers/docs/topics/rpc#commands-and-events-rpc-commands>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "args")]
pub enum SentCommand {
    Dispatch(SubscribeableEvent), // ???
    Authorize(AuthorizeArgs),
    Authenticate(AuthenticateArgs),
    GetGuild(GetGuildArgs),
    GetGuilds,
    GetChannel(GetChannelArgs),
    GetChannels(GetChannelsArgs),
    Subscribe(SubscribeableEvent),
    Unsubscribe(SubscribeableEvent),
    SetUserVoiceSettings(SetUserVoiceSettingsArgs),
    SelectVoiceChannel(SelectVoiceChannelArgs),
    GetSelectedVoiceChannel,
    SelectTextChannel(SelectTextChannelArgs),
    GetVoiceSettings,
    SetVoiceSettings(SetVoiceSettingsArgs),
    SetCertifiedDevices, // Restricted to hardware manufacturers
    SetActivity,         // TODO
    SendActivityJoinInvite(SendActivityJoinInviteArgs),
    CloseActivityRequest(CloseActivityRequestArgs),
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
