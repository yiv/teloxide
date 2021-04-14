use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{Chat, ChatId, ChatMember},
    Bot,
};
use std::sync::Arc;

/// Use this method to get up to date information about the chat (current name
/// of the user for one-on-one conversations, current username of a user, group
/// or channel, etc.).
///
/// [The official docs](https://core.telegram.org/bots/api#getchat).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGuildMembers {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub guild_id: i64,
    pub after: Option<i64>,
    pub limit: Option<i32>,
}

#[async_trait::async_trait]
impl Request for GetGuildMembers {
    type Output = Vec<ChatMember>;

    async fn send(&self) -> ResponseResult<Vec<ChatMember>> {
        net::request_json(self.bot.client(), self.bot.token(), "getGuildMembers", &self)
            .await
    }
}

impl GetGuildMembers {
    pub(crate) fn new(bot: Arc<Bot>, guild_id: i64, after: Option<i64>, limit: Option<i32>) -> Self {
        Self { bot, guild_id, after, limit }
    }
}
