use serde::{Deserialize, Serialize};

use crate::types::{User};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LikeReaction {
    PostLike,
    CommentLike,
}

impl LikeReaction {
    pub fn from_str(like: &str) -> Self {
        match like {
            "post_like" => LikeReaction::PostLike,
            "comment_like" => LikeReaction::CommentLike,
            _ => LikeReaction::PostLike,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleLike {
    releated_id: i64,
    reaction_type: LikeReaction,
}

impl CircleLike {
    pub fn new(releated_id: i64, reaction_type: &str) -> Self {
        CircleLike {
            releated_id,
            reaction_type: LikeReaction::from_str(reaction_type),
        }
    }
}

