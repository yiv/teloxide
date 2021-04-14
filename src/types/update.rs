#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

use crate::types::{CallbackQuery, Chat, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, User, ChatKind,
                   ChatPublic, PublicChatKind, PublicChatChannel, MessageKind, MessageCommon, ForwardKind, ForwardOrigin, MediaKind, MediaText,
                   MessageEntity, PublicChatGroup, MediaPhoto, PhotoSize, ChatPrivate, VideoNote, MediaVideoNote, Voice, MediaVoice, MessageNewChatMembers,
                   MessageChatMembersOnline, MessageChatMembersOffline, MessageLeftChatMember, MediaSticker, Sticker, MessagePinned, MessageReaction,
                   Reaction, MediaTopicSahre, TopicShare, MediaCircleShareEntity, CircleShareEntity, CirclePost, CircleLike, MediaCircleLike, MediaRichText,
                   RichText, MediaCircleComment, MediaCirclePost, CircleComment};
use serde_json::Value;

/// This [object] represents an incoming update.
///
/// [The official docs](https://core.telegram.org/bots/api#update).
///
/// [object]: https://core.telegram.org/bots/api#available-types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially. This ID becomes especially
    /// handy if you’re using [Webhooks], since it allows you to ignore
    /// repeated updates or to restore the correct update sequence, should
    /// they get out of order. If there are no new updates for at least a
    /// week, then identifier of the next update will be chosen randomly
    /// instead of sequentially.
    ///
    /// [Webhooks]: crate::Bot::set_webhook
    #[serde(rename = "update_id")]
    pub id: i64,

    #[serde(flatten)]
    pub kind: UpdateKind,
}

impl Update {
    /// Tries to parse `value` into `Update`, logging an error if failed.
    ///
    /// It is used to implement update listeners.
    pub fn try_parse(value: &Value) -> Result<Self, serde_json::Error> {
        match serde_json::from_str(&value.to_string()) {
            Ok(update) => Ok(update),
            Err(error) => {
                log::error!("Cannot parse an update.\nError: {:?}\nValue: {}\n\
                        This is a bug in teloxide, please open an issue here: \
                        https://github.com/teloxide/teloxide/issues.", error, value);
                Err(error)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),

    /// New version of a message that is known to the bot and was edited.
    EditedMessage(Message),

    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),

    /// New version of a channel post that is known to the bot and was edited.
    EditedChannelPost(Message),

    /// New incoming [inline] query.
    ///
    /// [inline]: https://core.telegram.org/bots/api#inline-mode
    InlineQuery(InlineQuery),

    /// The result of an [inline] query that was chosen by a user and sent to
    /// their chat partner. Please see our documentation on the [feedback
    /// collecting] for details on how to enable these updates for your bot.
    ///
    /// [inline]: https://core.telegram.org/bots/api#inline-mode
    /// [feedback collecting]: https://core.telegram.org/bots/inline#collecting-feedback
    ChosenInlineResult(ChosenInlineResult),

    /// New incoming callback query.
    CallbackQuery(CallbackQuery),

    /// New incoming shipping query. Only for invoices with flexible price.
    ShippingQuery(ShippingQuery),

    /// New incoming pre-checkout query. Contains full information about
    /// checkout.
    PreCheckoutQuery(PreCheckoutQuery),

    /// New poll state. Bots receive only updates about stopped polls and
    /// polls, which are sent by the bot.
    Poll(Poll),

    /// A user changed their answer in a non-anonymous poll. Bots receive new
    /// votes only in polls that were sent by the bot itself.
    PollAnswer(PollAnswer),
}

impl Update {
    pub fn user(&self) -> Option<&User> {
        match &self.kind {
            UpdateKind::Message(m) => m.from(),
            UpdateKind::EditedMessage(m) => m.from(),
            UpdateKind::CallbackQuery(query) => Some(&query.from),
            UpdateKind::ChosenInlineResult(chosen) => Some(&chosen.from),
            UpdateKind::InlineQuery(query) => Some(&query.from),
            UpdateKind::ShippingQuery(query) => Some(&query.from),
            UpdateKind::PreCheckoutQuery(query) => Some(&query.from),
            UpdateKind::PollAnswer(answer) => Some(&answer.user),
            _ => None,
        }
    }

    pub fn chat(&self) -> Option<&Chat> {
        match &self.kind {
            UpdateKind::Message(m) => Some(&m.chat),
            UpdateKind::EditedMessage(m) => Some(&m.chat),
            UpdateKind::ChannelPost(p) => Some(&p.chat),
            UpdateKind::EditedChannelPost(p) => Some(&p.chat),
            UpdateKind::CallbackQuery(q) => Some(&q.message.as_ref()?.chat),
            _ => None,
        }
    }
}


impl UpdateKind {
    pub fn new_channel_post(message_id: i64, guild_id: i64, channel_id: i64, user_id: i64, username: &str, text: &str, entities: Vec<MessageEntity>, date: i64,
                            quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Text(MediaText { text: text.to_string(), entities }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_image_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64, image_url: &str, width: i32, height: i32,
                                  quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };
        let photo = PhotoSize {
            file_id: image_url.to_string(),
            file_unique_id: image_url.to_string(),
            width,
            height,
            file_size: None,
        };
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Photo(MediaPhoto {
                    photo: vec![photo],
                    caption: None,
                    caption_entities: vec![],
                    media_group_id: None,
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_sticker_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64, image_url: &str, width: i32, height: i32,
                                    quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };

        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Sticker(MediaSticker {
                    sticker: Sticker::new(&image_url, &image_url, width as u16, height as u16)
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_rich_text_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                                      quote: Option<i64>, gender: Option<u8>, nickname: Option<String>, title: &str) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };

        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::RichText(MediaRichText {
                    rich_text: RichText::new(title)
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_topic_share_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64, quote: Option<i64>,
                                        gender: Option<u8>, nickname: Option<String>, topic_user: i64, topic_channel: i64, topic_message: i64) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };

        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::TopicSahre(MediaTopicSahre {
                    topic_share: TopicShare::new(topic_user, topic_channel, topic_message),
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_circle_share_entity_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64, quote: Option<i64>,
                                                gender: Option<u8>, nickname: Option<String>, circle_user: User, circle_post: CirclePost) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };

        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::CircleShareEntity(MediaCircleShareEntity {
                    circle_share_entity: CircleShareEntity::new(circle_user, circle_post),
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_circle_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                           gender: Option<u8>, nickname: Option<String>, post_id: i64, topic_id: Option<i64>, topic_name: Option<String>) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: None }),
                edit_date: None,
                media_kind: MediaKind::CirclePost(MediaCirclePost {
                    circle_post: CirclePost::new(guild_id, channel_id, post_id, topic_id, topic_name),
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_circle_comment(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                              gender: Option<u8>, nickname: Option<String>, post_id: i64) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: None }),
                edit_date: None,
                media_kind: MediaKind::CircleComment(MediaCircleComment {
                    circle_comment: CircleComment::new(post_id, message_id),
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_circle_like(message_id: i64, guild_id: i64, channel_id: i64, user_id: i64, username: &str, date: i64, gender: Option<u8>, nickname: Option<String>, releated_id: i64, reaction_type: &str) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: None }),
                edit_date: None,
                media_kind: MediaKind::CircleLike(MediaCircleLike {
                    circle_like: CircleLike::new(releated_id, reaction_type),
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_video_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                                  url: &str, width: i32, height: i32, duration: u32,
                                  thumb_url: &str, thumb_width: i32, thumb_height: i32, quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };
        let photo = PhotoSize {
            file_id: thumb_url.to_string(),
            file_unique_id: thumb_url.to_string(),
            width: thumb_width,
            height: thumb_height,
            file_size: None,
        };
        let video = VideoNote {
            file_id: url.to_string(),
            file_unique_id: url.to_string(),
            length: 0,
            duration,
            thumb: Some(photo),
            file_size: None,
        };
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::VideoNote(MediaVideoNote {
                    video_note: video
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_voice_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                                  url: &str, duration: u32, quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_public_without_kind(message_id, channel_id, guild_id, date))) };
        let voice = Voice {
            file_id: url.to_string(),
            file_unique_id: url.to_string(),
            duration,
            mime_type: None,
            file_size: None,
        };
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Voice(MediaVoice {
                    voice,
                    caption: None,
                    caption_entities: vec![],
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_pinned_channel_post(message_id: i64, guild_id: i64, channel_id: i64, date: i64, pinned_id: i64) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Pinned(MessagePinned {
                pinned: Box::new(Message::new_public_without_kind(pinned_id, channel_id, guild_id, date)),
            }),
        })
    }
    pub fn new_reaction_channel_post(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, date: i64, username: &str, gender: Option<u8>, nickname: Option<String>, reaction_message: i64, action: &str, emoji: &str) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::Reaction(Reaction {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                reaction: MessageReaction {
                    reaction_to_message: Box::new(Message::new_public_without_kind(reaction_message, channel_id, guild_id, date)),
                    action: action.to_string(),
                    emoji: emoji.to_string(),
                },
            }),
        })
    }
    pub fn new_chat_members(message_id: i64, guild_id: i64, channel_id: i64, user_id: i64, username: &str, date: i64, gender: Option<u8>, nickname: Option<String>) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::NewChatMembers(MessageNewChatMembers {
                new_chat_members: vec![User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }],
            }),
        })
    }
    pub fn left_chat_member(message_id: i64, guild_id: i64, channel_id: i64, user_id: i64, username: &str, date: i64, gender: Option<u8>, nickname: Option<String>) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::LeftChatMember(MessageLeftChatMember {
                left_chat_member: User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                },
            }),
        })
    }
    pub fn chat_members_online(message_id: i64, guild_id: i64, channel_id: i64, user_id: i64, username: &str, date: i64, gender: Option<u8>, nickname: Option<String>) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::ChatMembersOnline(MessageChatMembersOnline {
                chat_members_online: vec![User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }],
            }),
        })
    }
    pub fn chat_members_offline(message_id: i64, guild_id: i64, channel_id: i64, user_id: i64, username: &str, date: i64, gender: Option<u8>, nickname: Option<String>) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    description: None,
                    invite_link: None,
                    pinned_message: None,
                }),
                photo: None,
            },
            kind: MessageKind::ChatMembersOffline(MessageChatMembersOffline {
                chat_members_offline: vec![User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }],
            }),
        })
    }
    pub fn new_message(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, text: &str, entities: Vec<MessageEntity>, date: i64,
                       quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_private_without_kind(message_id, channel_id, guild_id, date))) };
        UpdateKind::Message(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Text(MediaText { text: text.to_string(), entities }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_image_message(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64, image_url: &str, width: i32, height: i32,
                             quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_private_without_kind(message_id, channel_id, guild_id, date))) };
        let photo = PhotoSize {
            file_id: image_url.to_string(),
            file_unique_id: image_url.to_string(),
            width,
            height,
            file_size: None,
        };
        UpdateKind::Message(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Photo(MediaPhoto {
                    photo: vec![photo],
                    caption: None,
                    caption_entities: vec![],
                    media_group_id: None,
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_video_message(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                             url: &str, width: i32, height: i32, duration: u32,
                             thumb_url: &str, thumb_width: i32, thumb_height: i32, quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_private_without_kind(message_id, channel_id, guild_id, date))) };
        let photo = PhotoSize {
            file_id: thumb_url.to_string(),
            file_unique_id: thumb_url.to_string(),
            width: thumb_width,
            height: thumb_height,
            file_size: None,
        };
        let video = VideoNote {
            file_id: url.to_string(),
            file_unique_id: url.to_string(),
            length: 0,
            duration,
            thumb: Some(photo),
            file_size: None,
        };
        UpdateKind::Message(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::VideoNote(MediaVideoNote {
                    video_note: video
                }),
                reply_markup: None,
            }),
        })
    }
    pub fn new_voice_message(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                             url: &str, duration: u32, quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_private_without_kind(message_id, channel_id, guild_id, date))) };
        let voice = Voice {
            file_id: url.to_string(),
            file_unique_id: url.to_string(),
            duration,
            mime_type: None,
            file_size: None,
        };
        UpdateKind::Message(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Voice(MediaVoice {
                    voice,
                    caption: None,
                    caption_entities: vec![],
                }),
                reply_markup: None,
            }),
        })
    }

    pub fn new_sticker_message(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64, image_url: &str, width: i32, height: i32,
                               quote: Option<i64>, gender: Option<u8>, nickname: Option<String>) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_private_without_kind(message_id, channel_id, guild_id, date))) };

        UpdateKind::Message(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::Sticker(MediaSticker {
                    sticker: Sticker::new(&image_url, &image_url, width as u16, height as u16)
                }),
                reply_markup: None,
            }),
        })
    }

    pub fn new_rich_text_message(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, username: &str, date: i64,
                                 quote: Option<i64>, gender: Option<u8>, nickname: Option<String>, title: &str) -> Self {
        let reply_to = if quote == None { None } else { Some(Box::new(Message::new_private_without_kind(message_id, channel_id, guild_id, date))) };

        UpdateKind::Message(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message: reply_to }),
                edit_date: None,
                media_kind: MediaKind::RichText(MediaRichText {
                    rich_text: RichText::new(title)
                }),
                reply_markup: None,
            }),
        })
    }

    pub fn new_pinned_message(message_id: i64, guild_id: i64, channel_id: i64, date: i64, username: &str, pinned_id: i64) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Pinned(MessagePinned {
                pinned: Box::new(Message::new_public_without_kind(pinned_id, channel_id, guild_id, date)),
            }),
        })
    }
    pub fn new_message_reaction(message_id: i64, user_id: i64, guild_id: i64, channel_id: i64, date: i64, username: &str, gender: Option<u8>, nickname: Option<String>, reaction_message: i64, action: &str, emoji: &str) -> Self {
        UpdateKind::ChannelPost(Message {
            id: message_id,
            date,
            chat: Chat {
                id: channel_id,
                guild_id,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some(username.to_string()),
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Reaction(Reaction {
                from: Some(User {
                    id: user_id,
                    is_bot: false,
                    first_name: nickname.unwrap_or_default(),
                    last_name: None,
                    username: Some(username.to_string()),
                    language_code: None,
                    user_token: None,
                    gender,
                    avatar: None
                }),
                reaction: MessageReaction {
                    reaction_to_message: Box::new(Message::new_public_without_kind(reaction_message, channel_id, guild_id, date)),
                    action: action.to_string(),
                    emoji: emoji.to_string(),
                },
            }),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::{
        Chat, ChatKind, ChatPrivate, ForwardKind, ForwardOrigin, MediaKind,
        MediaText, Message, MessageCommon, MessageKind, Update, UpdateKind,
        User,
    };

    // TODO: more tests for deserialization
    #[test]
    fn message() {
        let json = r#"{
            "update_id":892252934,
            "message":{
                "message_id":6557,
                "from":{
                    "id":218485655,
                    "is_bot": false,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "language_code":"en"
                },
                "chat":{
                    "id":218485655,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "type":"private"
                },
               "date":1569518342,
               "text":"hello there"
            }
        }"#;

        let expected = Update {
            id: 892_252_934,
            kind: UpdateKind::Message(Message {
                id: 6557,
                date: 1_569_518_342,
                chat: Chat {
                    id: 218_485_655,
                    guild_id: 0,
                    kind: ChatKind::Private(ChatPrivate {
                        type_: (),
                        username: Some(String::from("WaffleLapkin")),
                        first_name: Some(String::from("Waffle")),
                        last_name: None,
                    }),
                    photo: None,
                },
                kind: MessageKind::Common(MessageCommon {
                    from: Some(User {
                        id: 218_485_655,
                        is_bot: false,
                        first_name: String::from("Waffle"),
                        last_name: None,
                        username: Some(String::from("WaffleLapkin")),
                        language_code: Some(String::from("en")),
                        user_token: None,
                        gender: None,
                        avatar: None
                    }),
                    forward_kind: ForwardKind::Origin(ForwardOrigin {
                        reply_to_message: None,
                    }),
                    edit_date: None,
                    media_kind: MediaKind::Text(MediaText {
                        text: String::from("hello there"),
                        entities: vec![],
                    }),
                    reply_markup: None,
                }),
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn de_private_chat_text_message() {
        let text = r#"
  {
    "message": {
      "chat": {
        "first_name": "Hirrolot",
        "id": 408258968,
        "type": "private",
        "username": "hirrolot"
      },
      "date": 1581448857,
      "from": {
        "first_name": "Hirrolot",
        "id": 408258968,
        "is_bot": false,
        "language_code": "en",
        "username": "hirrolot"
      },
      "message_id": 154,
      "text": "4"
    },
    "update_id": 306197398
  }
"#;

        assert!(serde_json::from_str::<Update>(text).is_ok());
    }

    #[test]
    fn pinned_message_works() {
        let json = r#"{
    "message": {
        "chat": {
            "id": -1001276785818,
            "title": "teloxide dev",
            "type": "supergroup",
            "username": "teloxide_dev"
        },
        "date": 1582134655,
        "from": {
            "first_name": "Hirrolot",
            "id": 408258968,
            "is_bot": false,
            "username": "hirrolot"
        },
        "message_id": 20225,
        "pinned_message": {
            "chat": {
                "id": -1001276785818,
                "title": "teloxide dev",
                "type": "supergroup",
                "username": "teloxide_dev"
            },
            "date": 1582134643,
            "from": {
                "first_name": "Hirrolot",
                "id": 408258968,
                "is_bot": false,
                "username": "hirrolot"
            },
            "message_id": 20224,
            "text": "Faster than a bullet"
        }
    },
    "update_id": 845402291
}"#;

        serde_json::from_str::<Update>(json).unwrap();
    }
}
