use serde::{Deserialize, Serialize};


/// This object represents a sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#sticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RichText {
    pub title: String,
}

impl RichText {
    pub fn new(title: &str) -> Self {
        RichText{
            title: title.to_string()
        }
    }
}