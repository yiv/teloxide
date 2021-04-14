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
pub struct DeleteGuildCredit {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub chat_id: Option<ChatId>,
    pub guild_id: Option<i64>,
    pub user_id: i64,
}

#[async_trait::async_trait]
impl Request for DeleteGuildCredit {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteGuildCredit",
            &self,
        ).await
    }
}

impl DeleteGuildCredit {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: Option<C>, guild_id: Option<i64>, user_id: i64) -> Self
        where
            C: Into<ChatId>,
    {
        let chat_id = if chat_id.is_some(){
            Some(chat_id.unwrap().into())
        }else{
            None
        };
        Self { bot, chat_id, guild_id, user_id }
    }

}
