use serde::{Deserialize, Serialize};
use slack_morphism::{SlackChannelId, SlackChannelInfo, SlackTs, SlackUserId};
use slack_morphism::blocks::SlackBlock;
use crate::lib::api::error::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct PostMessageResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub message: serde_json::Value
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoleAssignment {
    pub role_id: String,
    pub users: Vec<SlackUserId>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListAssignmentsResponse {
    pub role_assignments: Vec<RoleAssignment>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StrBoolean {
    True,
    False
}

impl From<bool> for StrBoolean {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl Into<bool> for StrBoolean {
    fn into(self) -> bool {
        match self {
            Self::True => true,
            Self::False => false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preference {
    pub who_can_post: String,
    pub can_thread: String,
    pub enable_at_here: StrBoolean,
    pub enable_at_channel: StrBoolean
}

#[derive(Deserialize, Debug, Clone)]
pub struct OkResp {
    ok: bool
}

impl OkResp {
    pub(crate) fn as_result(&self) -> Result<(), Error> {
        match self.ok {
            true => Ok(()),
            false => Err(Error::RespNotOk)
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConversationsCreateResponse {
    pub channel: SlackChannelInfo
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreparePhotoResponse {
    pub id: String,
    pub url: String
}