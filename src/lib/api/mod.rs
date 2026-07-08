pub mod error;
mod model;

use reqwest::Client;
use reqwest::header::HeaderMap;
use slack_morphism::blocks::SlackBlock;
use slack_morphism::{SlackChannelId, SlackTs};
use url::Host;
use crate::lib::api::error::Error;
use crate::lib::api::model::PostMessageResponse;

#[derive(Clone, Debug)]
pub struct APIClient {
    host: Host,
    xoxc: String,
    xoxd: String,
    reqwest_client: Client
}

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

impl APIClient {
    pub fn new(xoxc: String, xoxd: String, host: Host) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert("Cookie", format!("tz=0; d={}", xoxd).parse().unwrap());
        Self {
            xoxc, xoxd,
            reqwest_client: Client::builder().default_headers(headers).build().unwrap(),
            host
        }
    }

    pub async fn chat_post_message(&self, channel: SlackChannelId, thread_ts: Option<SlackTs>, blocks: MessageData) -> Result<SlackTs, Error> {
        let mut form = reqwest::multipart::Form::new()
            .text("token", self.xoxc.clone())
            .text("channel", channel.0)
            .text("type", "message")
            .text("client_msg_id", uuid::Uuid::new_v4().to_string());
        form = match blocks {
            MessageData::Raw(s) => { form.text("text", s) },
            MessageData::Blockkit(blocks) => { form.text("blocks", serde_json::to_string(&blocks)?) },
            MessageData::Multi(s, blocks) => {
                form.text("text", s)
                    .text("blocks", serde_json::to_string(&blocks)?)
            }
        };
        form = match thread_ts {
            Some(ts) => {
                form.text("thread_ts", ts.0)
            },
            None => form
        };
        let req = self.reqwest_client.post(&format!("https://{}/channels/{}/postMessage", self.host, self.xoxd)).multipart(form);

        let resp = req.send().await?;

        let model : PostMessageResponse = serde_json::from_str(&resp.text().await?)?;

        Ok(model.ts)
    }
}