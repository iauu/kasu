use slack_morphism::blocks::SlackBlock;
use slack_morphism::SlackUserId;
use crate::lib::api::model::Preference;

pub enum MessageData {
    Raw(String),
    Blockkit(Vec<SlackBlock>),
    Multi(String, Vec<SlackBlock>)
}

impl From<String> for MessageData {
    fn from(s: String) -> Self {
        Self::Raw(s)
    }
}

impl From<Vec<SlackBlock>> for MessageData {
    fn from(value: Vec<SlackBlock>) -> Self {
        Self::Blockkit(value)
    }
}

impl From<(String, Vec<SlackBlock>)> for MessageData {
    fn from(value: (String, Vec<SlackBlock>)) -> Self {
        Self::Multi(value.0, value.1)
    }
}

pub enum SendRestriction {
    NoRestriction,
    CertainUser{user: Vec<SlackUserId>, allow_thread: bool}
}

impl SendRestriction {
    pub fn pref_str(&self) -> (String, String) {
        match self {
            Self::NoRestriction => ("type:ra".to_string(), "type:ra".to_string()),
            Self::CertainUser {user, allow_thread} => {
                let rest_str = if user.len() > 0 {
                    format!("type:admin,{}", user.iter().map(|x| format!("user:{}", x.0)).collect::<Vec<String>>().join(","))
                } else {
                    "type:admin".to_string()
                };
                if *allow_thread {
                    (rest_str, "type:ra".to_string())
                } else {
                    (rest_str.clone(), rest_str)
                }
            }
        }
    }
}

impl Default for SendRestriction {
    fn default() -> Self {
        Self::NoRestriction
    }
}

pub struct ChannelRestriction {
    pub restriction: SendRestriction,
    pub allow_channel_ping: bool,
    pub allow_here_ping: bool
}

impl Default for ChannelRestriction {
    fn default() -> Self {
        Self {
            restriction: SendRestriction::default(),
            allow_channel_ping: true,
            allow_here_ping: true
        }
    }
}

impl Into<Preference> for ChannelRestriction {
    fn into(self) -> Preference {
        Preference {
            who_can_send: self.restriction.pref_str().0,
            can_thread: self.restriction.pref_str().1,
            enable_at_here: self.allow_here_ping.into(),
            enable_at_channel: self.allow_channel_ping.into()
        }
    }
}