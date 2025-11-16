use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    /// snowflake - guild id
    pub id: String,
    /// string - guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,
    /// ?string - icon hash
    pub icon: Option<String>,
    /// snowflake - id of owner
    pub owner_id: Option<String>,
}
