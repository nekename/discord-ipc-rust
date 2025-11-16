use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://discord.com/developers/docs/resources/channel#channel-object-channel-types>
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u32)]
pub enum ChannelType {
    /// a text channel within a server
    GuildText = 0,
    /// a direct message between users
    DirectMessage = 1,
    /// a voice channel within a server
    GuildVoice = 2,
    /// a direct message between multiple users
    GroupDirectMessage = 3,
    /// an organizational category that contains up to 50 channels
    GuildCategory = 4,
    /// a channel that users can follow and crosspost into their own server (formerly news channels)
    GuildAnnouncement = 5,
    /// a temporary sub-channel within a GUILD_ANNOUNCEMENT channel
    AnnouncementThread = 10,
    /// a temporary sub-channel within a GUILD_TEXT or GUILD_FORUM channel
    PublicThread = 11,
    /// a temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
    PrivateThread = 12,
    /// a voice channel for hosting events with an audience
    GuildStageVoice = 13,
    /// the channel in a hub containing the listed servers
    GuildDirectory = 14,
    /// Channel that can only contain threads
    GuildForum = 15,
    /// Channel that can only contain threads, similar to GUILD_FORUM channels
    GuildMedia = 16,
}

/// <https://discord.com/developers/docs/resources/channel#channel-object-channel-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    /// snowflake - the id of this channel
    pub id: String,
    /// integer - the type of channel
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    /// snowflake - the id of the guild (may be missing for some channel objects received over gateway guild dispatches)
    pub guild_id: Option<String>,
    /// integer - sorting position of the channel (channels with the same position are sorted by id)
    pub position: Option<u32>,
    /// ?string - the name of the channel (1-100 characters)
    pub name: Option<String>,
    /// ?string - the channel topic (0-4096 characters for GUILD_FORUM and GUILD_MEDIA channels, 0-1024 characters for all others)
    pub topic: Option<String>,
    /// boolean - whether the channel is nsfw
    pub nsfw: Option<bool>,
    /// ?snowflake - the id of the last message sent in this channel (or thread for GUILD_FORUM or GUILD_MEDIA channels) (may not point to an existing or valid message or thread)
    pub last_message_id: Option<String>,
    /// integer - the bitrate (in bits) of the voice channel
    pub bitrate: Option<u32>,
    /// integer - the user limit of the voice channel
    pub user_limit: Option<u32>,
    /// integer - amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission manage_messages or manage_channel, are unaffected
    pub rate_limit_per_user: Option<u32>,
    /// array of voice state objects - (voice) channel's voice states
    pub voice_states: Option<Vec<crate::models::receive::events::VoiceStateData>>,
}
