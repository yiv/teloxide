use crate::{
    dispatching::dialogue::GetChatId,
    requests::{
        DeleteMessage, EditMessageCaption, EditMessageText, ForwardMessage,
        PinChatMessage, Request, ResponseResult, SendAnimation, SendAudio,
        SendContact, SendDocument, SendLocation, SendMediaGroup, SendMessage,
        SendPhoto, SendSticker, SendVenue, SendVideo, SendVideoNote, SendVoice,
        SetGuildCredit, DeleteGuildCredit, GetPrivateChat, GetGuildRoles, GetGuildMembers,
        SetMemberRoles, GetChatMember, GetRoleMembers,
    },
    types::{ChatId, ChatOrInlineMessage, InputFile, InputMedia, Message, GuildCredit},
    Bot,
};
use std::sync::Arc;
use crate::requests::{SearchGuildMember, SearchGuildMemberByUsername};

/// A [`Dispatcher`]'s handler's context of a bot and an update.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[derive(Debug)]
pub struct UpdateWithCx<Upd> {
    pub bot: Arc<Bot>,
    pub update: Upd,
}

impl<Upd> GetChatId for UpdateWithCx<Upd>
    where
        Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

impl UpdateWithCx<Message> {
    pub async fn answer_str<T>(&self, text: T) -> ResponseResult<Message>
        where
            T: Into<String>,
    {
        self.answer(text).send().await
    }

    pub fn answer<T>(&self, text: T) -> SendMessage
        where
            T: Into<String>,
    {
        self.bot.send_message(self.chat_id(), text)
    }

    pub fn reply_to<T>(&self, text: T) -> SendMessage
        where
            T: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .reply_to_message_id(self.update.id)
    }

    pub fn answer_photo(&self, photo: InputFile) -> SendPhoto {
        self.bot.send_photo(self.update.chat.id, photo)
    }

    pub fn answer_audio(&self, audio: InputFile) -> SendAudio {
        self.bot.send_audio(self.update.chat.id, audio)
    }

    pub fn answer_animation(&self, animation: InputFile) -> SendAnimation {
        self.bot.send_animation(self.update.chat.id, animation)
    }

    pub fn answer_document(&self, document: InputFile) -> SendDocument {
        self.bot.send_document(self.update.chat.id, document)
    }

    pub fn answer_video(&self, video: InputFile) -> SendVideo {
        self.bot.send_video(self.update.chat.id, video)
    }

    pub fn answer_voice(&self, voice: InputFile) -> SendVoice {
        self.bot.send_voice(self.update.chat.id, voice)
    }

    pub fn answer_media_group<T>(&self, media_group: T) -> SendMediaGroup
        where
            T: Into<Vec<InputMedia>>,
    {
        self.bot.send_media_group(self.update.chat.id, media_group)
    }

    pub fn answer_location(
        &self,
        latitude: f32,
        longitude: f32,
    ) -> SendLocation {
        self.bot.send_location(self.update.chat.id, latitude, longitude)
    }

    pub fn answer_venue<T, U>(
        &self,
        latitude: f32,
        longitude: f32,
        title: T,
        address: U,
    ) -> SendVenue
        where
            T: Into<String>,
            U: Into<String>,
    {
        self.bot.send_venue(
            self.update.chat.id,
            latitude,
            longitude,
            title,
            address,
        )
    }

    pub fn answer_video_note(&self, video_note: InputFile) -> SendVideoNote {
        self.bot.send_video_note(self.update.chat.id, video_note)
    }

    pub fn answer_contact<T, U>(
        &self,
        phone_number: T,
        first_name: U,
    ) -> SendContact
        where
            T: Into<String>,
            U: Into<String>,
    {
        self.bot.send_contact(self.chat_id(), phone_number, first_name)
    }

    pub fn answer_sticker(&self, sticker: InputFile) -> SendSticker {
        self.bot.send_sticker(self.update.chat.id, sticker)
    }

    pub fn forward_to<T>(&self, chat_id: T) -> ForwardMessage
        where
            T: Into<ChatId>,
    {
        self.bot.forward_message(chat_id, self.update.chat.id, self.update.id)
    }

    pub fn edit_message_text<T>(&self, text: T) -> EditMessageText
        where
            T: Into<String>,
    {
        self.bot.edit_message_text(
            ChatOrInlineMessage::Chat {
                chat_id: self.update.chat.id.into(),
                message_id: self.update.id,
            },
            text,
        )
    }

    pub fn edit_message_caption(&self) -> EditMessageCaption {
        self.bot.edit_message_caption(ChatOrInlineMessage::Chat {
            chat_id: self.update.chat.id.into(),
            message_id: self.update.id,
        })
    }

    pub fn delete_message(&self) -> DeleteMessage {
        self.bot.delete_message(self.update.chat.id, self.update.id)
    }

    pub fn pin_message(&self) -> PinChatMessage {
        self.bot.pin_chat_message(self.update.chat.id, self.update.id)
    }

    pub fn set_guild_credit(&self, guild_id: Option<i64>, user_id: i64, guild_credit: GuildCredit) -> SetGuildCredit {
        self.bot.set_guild_credit(Some(self.update.chat.id), guild_id, user_id, guild_credit)
    }

    pub fn delete_guild_credit(&self, guild_id: Option<i64>, user_id: i64) -> DeleteGuildCredit {
        self.bot.delete_guild_credit(Some(self.update.chat.id), guild_id, user_id)
    }

    pub fn get_private_chat(&self, user_id: i64) -> GetPrivateChat {
        self.bot.get_private_chat(user_id)
    }

    pub fn get_guild_roles(&self, guild_id: i64) -> GetGuildRoles {
        self.bot.get_guild_roles(guild_id)
    }

    pub fn get_guild_members(&self, guild_id: i64, after: Option<i64>, limit: Option<i32>) -> GetGuildMembers {
        self.bot.get_guild_members(guild_id, after, limit)
    }

    pub fn set_member_roles(&self, guild_id: i64, user_id:i64, roles: Vec<i64>) -> SetMemberRoles {
        self.bot.set_member_roles(guild_id, user_id, roles)
    }

    pub fn search_guild_member(&self, guild_id: i64, query: String) -> SearchGuildMember {
        self.bot.search_guild_member(guild_id, query)
    }

    pub fn search_guild_member_by_username(&self, guild_id: i64, usernames: Vec<String>) -> SearchGuildMemberByUsername {
        self.bot.search_guild_member_by_username(guild_id, usernames)
    }

    pub fn get_chat_member(&self, guild_id: Option<i64>, user_id: i64) -> GetChatMember {
        self.bot.get_chat_member(Some(self.update.chat.id), guild_id, user_id)
    }

    pub fn get_role_members(&self, guild_id: i64, role_id: i64) -> GetRoleMembers {
        self.bot.get_role_members(guild_id, role_id)
    }
}
