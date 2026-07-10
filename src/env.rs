use url::Host;
use serde::Deserialize;
use slack_morphism::SlackTeamId;

#[derive(Clone, Deserialize)]
pub struct Env {
    pub xoxc: String,
    pub xoxd: String,
    pub host: String,
    pub team_id: SlackTeamId
}