use serde::Deserialize;
use slack_morphism::{SlackChannelId, SlackTs};

#[derive(Deserialize, Debug, Clone)]
pub struct PostMessageResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub message: serde_json::Value
}