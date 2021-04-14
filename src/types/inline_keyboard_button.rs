use crate::types::{CallbackGame, LoginUrl};
use serde::{Deserialize, Serialize};

/// This object represents one button of an inline keyboard.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinekeyboardbutton).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,

    #[serde(flatten)]
    pub kind: InlineKeyboardButtonKind,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonKind {
    /// HTTP or tg:// url to be opened when button is pressed.
    #[serde(rename(serialize = "url"))]
    Url(String),

    #[serde(rename = "app_id")]
    MiniProgram(String),

    /// An HTTP URL used to automatically authorize the user. Can be used as a
    /// replacement for the [Telegram Login Widget]().
    ///
    /// [Telegram Login Widget]: https://core.telegram.org/widgets/login
    #[serde(rename(serialize = "login_url"))]
    LoginUrl(LoginUrl),

    /// Data to be sent in a [`CallbackQuery`] to the bot when button is
    /// pressed, 1-64 bytes.
    ///
    /// [`CallbackQuery`]: crate::types::CallbackQuery
    #[serde(rename(serialize = "callback_data"))]
    CallbackData(String),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot‘s username and the specified
    /// inline query in the input field. Can be empty, in which case just the
    /// bot’s username will be inserted.
    ///
    /// Note: This offers an easy way for users to start using your bot in
    /// [inline mode] when they are currently in a private chat with it.
    /// Especially useful when combined with [switch_pm…] actions – in this
    /// case the user will be automatically returned to the chat they
    /// switched from, skipping the chat selection screen.
    ///
    /// [inline mode]: https://core.telegram.org/bots/inline
    /// [switch_pm…]: https://core.telegram.org/bots/api#answerinlinequery
    #[serde(rename(serialize = "switch_inline_query"))]
    SwitchInlineQuery(String),

    /// If set, pressing the button will insert the bot‘s username and the
    /// specified inline query in the current chat's input field.
    /// Can be empty, in which case only the bot’s username will be
    /// inserted.
    ///
    ///This offers a quick way for the user to open your bot in inline mode in
    /// the same chat – good for selecting something from multiple options.
    #[serde(rename(serialize = "switch_inline_query_current_chat"))]
    SwitchInlineQueryCurrentChat(String),

    /// Description of the game that will be launched when the user presses the
    /// button.
    ///
    /// ## Note
    /// This type of button **must** always be the first button in the first
    /// row.
    #[serde(rename(serialize = "callback_game"))]
    CallbackGame(CallbackGame),

    /// Specify True, to send a [Pay button].
    ///
    /// ## Note
    /// This type of button **must** always be the first button in the first
    /// row.
    ///
    /// [Pay button]: https://core.telegram.org/bots/api#payments
    #[serde(rename(serialize = "pay"))]
    Pay(bool),
}

/// Build buttons.
///
/// # Examples
/// ```
/// use teloxide::types::InlineKeyboardButton;
///
/// let url_button = InlineKeyboardButton::url(
///     "Text".to_string(),
///     "http://url.com".to_string(),
/// );
/// ```
impl InlineKeyboardButton {
    pub fn url(text: String, url: String) -> InlineKeyboardButton {
        InlineKeyboardButton { text, kind: InlineKeyboardButtonKind::Url(url) }
    }


    pub fn mini_program(text: String, app_id: String) -> InlineKeyboardButton {
        InlineKeyboardButton { text, kind: InlineKeyboardButtonKind::MiniProgram(app_id) }
    }

    pub fn callback(
        text: String,
        callback_data: String,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::CallbackData(callback_data),
        }
    }

    pub fn switch_inline_query(
        text: String,
        switch_inline_query: String,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQuery(
                switch_inline_query,
            ),
        }
    }

    pub fn switch_inline_query_current_chat(
        text: String,
        switch_inline_query_current_chat: String,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat,
            ),
        }
    }
}
