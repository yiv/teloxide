use serde::{Deserialize, Serialize};


/// This object represents guild credit of user
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GuildCredit {
    pub authority: CreditAuthority,
    pub title: Option<CreditTitle>,
    pub slots: Vec<Vec<CreditSlot>>,
}

/// This object represents profile of a guild credit authority
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreditAuthority {
    pub icon: String,
    pub name: String,
}

impl CreditAuthority {
    pub fn new(name: String, icon: String) -> Self {
        CreditAuthority { icon, name }
    }
}

/// This object represents guild credit title of user
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreditTitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
}

impl CreditTitle {
    pub fn img(img: String) -> Self {
        CreditTitle {
            img: Some(img),
        }
    }
}

/// This object represents guild credit slots
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreditSlot {
    #[serde(flatten)]
    pub title: SlotTitle,
    pub value: String,
}

impl CreditSlot {
    pub fn img(img: String, value: String) -> Self {
        CreditSlot {
            title: SlotTitle::Img(img),
            value,
        }
    }
    pub fn label(label: String, value: String) -> Self {
        CreditSlot {
            title: SlotTitle::Label(label),
            value,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotTitle {
    Label(String),
    Img(String),
}




