use url::Host;
use serde::Deserialize;
use slack_morphism::{SlackTeamId, SlackUserId};

#[derive(Clone, Deserialize)]
pub struct Env {
    pub xoxc: String,
    pub sub_xoxc: Option<String>,
    pub xoxd: String,
    pub host: String,
    pub team_id: SlackTeamId,
    pub user_id: SlackUserId
}