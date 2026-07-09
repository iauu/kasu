use slack_morphism::blocks::SlackBlock;

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