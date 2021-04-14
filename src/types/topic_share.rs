use serde::{Deserialize, Serialize};

use crate::types::{MaskPosition, PhotoSize};

/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicShare {
    pub user_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
}

impl TopicShare {
    pub fn new(user_id: i64, channel_id: i64, message_id: i64) -> Self {
        TopicShare{
            user_id,channel_id, message_id
        }
    }
}