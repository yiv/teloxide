use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{GuildRole, ChatId},
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
pub struct GetGuildRoles {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub guild_id: i64,
}

#[async_trait::async_trait]
impl Request for GetGuildRoles {
    type Output = Vec<GuildRole>;

    async fn send(&self) -> ResponseResult<Vec<GuildRole>> {
        net::request_json(self.bot.client(), self.bot.token(), "getGuildRoles", &self)
            .await
    }
}

impl GetGuildRoles {
    pub(crate) fn new(bot: Arc<Bot>, guild_id: i64) -> Self {
        Self { bot, guild_id }
    }
}
