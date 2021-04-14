use serde::{Deserialize, Serialize};

use crate::types::{User};
use crate::types::circle_post::CirclePost;

/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleShareEntity {
    pub user: User,
    pub post: CirclePost,
}

impl CircleShareEntity {
    pub fn new(user: User, post: CirclePost) -> Self {
        CircleShareEntity {
            user,
            post,
        }
    }
}
