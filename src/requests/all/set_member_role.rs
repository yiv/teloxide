use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{GuildRole, ChatId, True},
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
pub struct SetMemberRoles {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub guild_id: i64,
    pub user_id: i64,
    pub roles: Vec<i64>,
}

#[async_trait::async_trait]
impl Request for SetMemberRoles {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(self.bot.client(), self.bot.token(), "setMemberRoles", &self)
            .await
    }
}

impl SetMemberRoles {
    pub(crate) fn new(bot: Arc<Bot>, guild_id: i64, user_id:i64, roles: Vec<i64>) -> Self {
        Self { bot, guild_id , user_id, roles}
    }
}
