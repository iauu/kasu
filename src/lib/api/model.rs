use serde::Deserialize;
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use slack_morphism::blocks::SlackBlock;

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