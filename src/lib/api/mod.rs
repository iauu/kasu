pub mod error;
pub mod model;
pub mod common;

pub use crate::lib::api::common::*;

use reqwest::Client;
use reqwest::header::HeaderMap;
use slack_morphism::blocks::SlackBlock;
use slack_morphism::{SlackChannelId, SlackChannelInfo, SlackTeamId, SlackTs, SlackUserId};
use url::Host;
use crate::lib::api::error::Error;
use crate::lib::api::model::{ConversationsCreateResponse, ListAssignmentsResponse, OkResp, PostMessageResponse, Preference};

#[derive(Clone, Debug)]
pub struct APIClient {
    host: String,
    xoxc: String,
    xoxd: String,
    reqwest_client: Client,
    team_id: SlackTeamId
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

macro_rules! as_text {
    ($req:expr) => {
        $req.send().await?.text().await?
    };
}

macro_rules! parse_req {
    ($req:expr, $t:ty) => {
        {
            let text = as_text!($req);
            parse_resp!(text, $t)
        }
    };
}

impl APIClient {
    pub fn new(xoxc: String, xoxd: String, host: String, team_id: SlackTeamId) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert("Cookie", format!("tz=0; d={}", xoxd).parse().unwrap());
        Self {
            xoxc, xoxd,
            reqwest_client: Client::builder().default_headers(headers).build().unwrap(),
            host, team_id
        }
    }

    pub async fn chat_post_message(&self, channel: SlackChannelId, thread_ts: Option<SlackTs>, blocks: MessageData) -> Result<SlackTs, Error> {
        let mut form = self.get_base_form()
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

        let model = parse_req!(req, PostMessageResponse);

        Ok(model.ts)
    }

    pub async fn get_channel_manager(&self, channel: SlackChannelId) -> Result<Vec<SlackUserId>, Error> {
        let form = self.get_base_form()
            .text("entity_id", channel.0);
        let req = self.reqwest_client.post(&format!("https://{}/api/admin.roles.entity.listAssignments", self.host)).multipart(form);

        let model = parse_req!(req, ListAssignmentsResponse);

        let assignment = model.role_assignments.into_iter().filter(|assignment| assignment.role_id == "Rl0A").last();
        Ok(match assignment {
            Some(v) => v.users,
            None => vec![]
        })
    }

    pub async fn update_channel_permission(&self, channel: SlackChannelId, channel_restriction: ChannelRestriction) -> Result<(), Error> {
        let form = self.get_base_form()
            .text("channel_id", channel.0)
            .text("prefs", serde_json::to_string::<Preference>(&channel_restriction.into())?);
        let req = self.reqwest_client.post(&format!("https://{}/api/channels.prefs.set", self.host)).multipart(form);

        parse_req!(req, OkResp).as_result()
    }

    fn get_base_form(&self) -> reqwest::multipart::Form {
        reqwest::multipart::Form::new()
            .text("token", self.xoxc.clone())
    }

    pub async fn conversation_create(&self, name: String, is_private: bool) -> Result<SlackChannelInfo, Error> {
        let form = self.get_base_form()
            .text("name", name)
            .text("team_id", self.team_id.0.clone())
            .text("is_private", is_private.to_string());
        let req = self.reqwest_client.post(&format!("https://{}/api/conversations.create", self.host)).multipart(form);

        Ok(parse_req!(req, ConversationsCreateResponse).channel)
    }

    pub async fn change_channel_manager(&self, channels: Vec<SlackChannelId>, users: Vec<SlackUserId>, action: RoleAction) -> Result<(), Error> {
        let form = self.get_base_form()
            .text("role_id", "Rl0A")
            .text("role_scopes", channels.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(","))
            .text("user_ids", users.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(","));

        let req = self.reqwest_client.post(&format!("https://{}/api/admin.roles.{}Members", self.host, action.to_string())).multipart(form);

        parse_req!(req, OkResp).as_result()
    }

    pub async fn add_user(&self, channel: SlackChannelId, users: Vec<SlackUserId>) -> Result<(), Error> {
        let form = self.get_base_form()
            .text("role_id", "Rl0A")
            .text("channel", channel.to_string())
            .text("users", users.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(","))
            .text("force", "true");

        let req = self.reqwest_client.post(&format!("https://{}/api/conversations.invite", self.host)).multipart(form);

        parse_req!(req, OkResp).as_result()
    }

    pub async fn remove_user(&self, channel: SlackChannelId, user: SlackUserId) -> Result<(), Error> {
        let form = self.get_base_form()
            .text("role_id", "Rl0A")
            .text("channel", channel.to_string())
            .text("user", user.to_string());

        let req = self.reqwest_client.post(&format!("https://{}/api/conversations.kick", self.host)).multipart(form);

        parse_req!(req, OkResp).as_result()
    }
}