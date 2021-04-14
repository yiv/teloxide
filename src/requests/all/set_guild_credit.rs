use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True, GuildCredit},
    Bot,
};
use std::sync::Arc;

/// Use this method to change the title of a chat.
///
/// Titles can't be changed for private chats. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchattitle).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGuildCredit {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub chat_id: Option<ChatId>,
    pub guild_id: Option<i64>,
    pub user_id: i64,
    pub guild_credit: GuildCredit,
}

#[async_trait::async_trait]
impl Request for SetGuildCredit {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setGuildCredit",
            &self,
        )
            .await
    }
}

impl SetGuildCredit {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: Option<C>, guild_id: Option<i64>, user_id: i64, guild_credit: GuildCredit) -> Self
        where
            C: Into<ChatId>,
    {
        let chat_id = if chat_id.is_some() {
            let c = chat_id.unwrap();
            Some(c.into())
        } else {
            None
        };
        Self { bot, chat_id, guild_id, user_id, guild_credit }
    }

    /// New chat title, 1-255 characters.
    pub fn guild_credit<T>(mut self, guild_credit: GuildCredit) -> Self {
        self.guild_credit = guild_credit;
        self
    }
}
