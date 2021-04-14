use serde::{Deserialize, Serialize};



#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CirclePost {
    pub guild_id: i64,
    pub channel_id: i64,
    pub post_id: i64,
    pub topic_id: Option<i64>,
    pub topic_name: Option<String>,
}

impl CirclePost {
    pub fn new(guild_id: i64, channel_id: i64, post_id: i64, topic_id: Option<i64>, topic_name: Option<String>) -> Self {
        CirclePost {
            guild_id,
            channel_id,
            post_id,
            topic_id,
            topic_name
        }
    }
}