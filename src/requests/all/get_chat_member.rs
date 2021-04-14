use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatMember},
    Bot,
};
use std::sync::Arc;

/// Use this method to get information about a member of a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#getchatmember).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMember {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub chat_id: Option<ChatId>,
    pub guild_id: Option<i64>,
    pub user_id: i64,
}

#[async_trait::async_trait]
impl Request for GetChatMember {
    type Output = ChatMember;

    async fn send(&self) -> ResponseResult<ChatMember> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getChatMember",
            &self,
        )
        .await
    }
}

impl GetChatMember {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: Option<C>, guild_id: Option<i64>, user_id: i64) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.map(|c|c.into());
        Self { bot, chat_id, user_id , guild_id}
    }



    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }
}
