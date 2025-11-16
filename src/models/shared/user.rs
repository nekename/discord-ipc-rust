use serde::{Deserialize, Serialize};

/// <https://discord.com/developers/docs/resources/user#user-object-user-structure>
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// snowflake - the user's id
    pub id: String,
    /// string - the user's username, not unique across the platform
    pub username: String,
    /// ?string - the user's avatar hash
    pub avatar: Option<String>,
    /// boolean - whether the user belongs to an OAuth2 application
    pub bot: Option<bool>,
    /// integer - the flags on a user's account
    pub flags: Option<u32>,
    /// integer - the type of Nitro subscription on a user's account
    pub premium_type: Option<u32>,
}
