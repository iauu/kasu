pub mod error;
mod model;
pub mod common;

pub use crate::lib::api::common::*;

use reqwest::Client;
use reqwest::header::HeaderMap;
use slack_morphism::blocks::SlackBlock;
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use url::Host;
use crate::lib::api::error::Error;
use crate::lib::api::model::{ListAssignmentsResponse, PostMessageResponse};

#[derive(Clone, Debug)]
pub struct APIClient {
    host: String,
    xoxc: String,
    xoxd: String,
    reqwest_client: Client
}

macro_rules! parse_resp {
    ($text:expr, $t:ty) => {
        {
            let text_ref: &str = $text.as_ref();
            let model: $t = match serde_json::from_str(text_ref) {
                Ok(resp) => resp,
                Err(e) => {
                    tracing::error!("Failed request {e}, data: {text_ref}");
                    return Err(e.into());
                }
            };
            model
        }
    };
}

impl APIClient {
    pub fn new(xoxc: String, xoxd: String, host: String) -> Self {
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
        let req = self.reqwest_client.post(&format!("https://{}/api/chat.postMessage", self.host)).multipart(form);

        let resp = req.send().await?;
        let resp_text = resp.text().await?;
        let model = parse_resp!(resp_text, PostMessageResponse);

        Ok(model.ts)
    }

    pub async fn get_channel_manager(&self, channel: SlackChannelId) -> Result<Vec<SlackUserId>, Error> {
        let form = reqwest::multipart::Form::new()
            .text("token", self.xoxc.clone())
            .text("entity_id", channel.0);
        let req = self.reqwest_client.post(&format!("https://{}/api/admin.roles.entity.listAssignments", self.host)).multipart(form);
        let resp = req.send().await?;
        let resp_text = resp.text().await?;
        let model = parse_resp!(resp_text, ListAssignmentsResponse);

        let assignment = model.role_assignments.into_iter().filter(|assignment| assignment.role_id == "Rl0A").last();
        Ok(match assignment {
            Some(v) => v.users,
            None => vec![]
        })
    }

}