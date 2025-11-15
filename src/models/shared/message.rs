use super::User;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: Option<String>,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedMessage {
    pub id: String,
}
