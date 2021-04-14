use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
///
/// [The official docs](https://core.telegram.org/bots/api#user).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,

    /// `true`, if this user is a bot.
    pub is_bot: bool,

    /// User‘s or bot’s first name.
    pub first_name: String,

    /// User‘s or bot’s last name.
    #[serde(default)]
    pub last_name: Option<String>,

    /// User‘s or bot’s username.
    #[serde(default)]
    pub username: Option<String>,

    /// User‘s or bot’s gender.
    #[serde(default)]
    pub gender: Option<u8>,

    #[serde(default)]
    pub avatar: Option<String>,

    /// [IETF language tag] of the user's language.
    ///
    /// [IETF language tag]: https://en.wikipedia.org/wiki/IETF_language_tag
    #[serde(default)]
    pub language_code: Option<String>,

    #[serde(default)]
    pub user_token: Option<String>,
}

impl User {
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last_name) => (format!("{0} {1}", self.first_name, last_name)),
            None => self.first_name.clone(),
        }
    }

    pub fn mention(&self) -> Option<String> {
        Some(format!("@{}", self.username.as_ref()?))
    }

    pub fn url(&self) -> reqwest::Url {
        reqwest::Url::parse(format!("tg://user/?id={}", self.id).as_str())
            .unwrap()
    }
}

/// Returned only in [`Bot::get_me`].
///
/// [`Bot::get_me`]: crate::Bot::get_me
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub struct Me {
    #[serde(flatten)]
    pub user: User,

    #[serde(default)]
    pub owner_id : i64,
    /// `true`, if the bot can be invited to groups.
    #[serde(default)]
    pub can_join_groups: bool,

    /// `true`, if [privacy mode] is disabled for the bot.
    ///
    /// [privacy mode]: https://core.telegram.org/bots#privacy-mode
    #[serde(default)]
    pub can_read_all_group_messages: bool,

    /// `true`, if the bot supports inline queries.
    #[serde(default)]
    pub supports_inline_queries: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":12345,
            "is_bot":false,
            "first_name":"firstName",
            "last_name":"lastName",
            "username":"Username",
            "language_code":"ru"
        }"#;
        let expected = User {
            id: 12345,
            is_bot: false,
            user_token: Some("test".to_string()),
            first_name: "firstName".to_string(),
            last_name: Some("lastName".to_string()),
            username: Some("Username".to_string()),
            gender: None,
            avatar: None,
            language_code: Some(String::from("ru")),
        };
        let actual = serde_json::from_str::<User>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
