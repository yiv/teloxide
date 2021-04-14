use serde::{Deserialize, Serialize};


/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct GuildRole {
    pub id: i64,
    pub name: String,
    pub position: i32,
    pub permissions: i64,
    pub color: i64,
}

impl GuildRole {
    pub fn new(id: i64, name: String, position: i32, permissions: i64, color: i64) -> Self {
        GuildRole {
            id,
            name,
            position,
            permissions,
            color,
        }
    }
}