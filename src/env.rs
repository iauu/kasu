use serde::{Deserialize, Deserializer};
use slack_morphism::{SlackTeamId, SlackUserId};

#[derive(Clone, Deserialize)]
pub struct Env {
    pub xoxc: String,
    pub sub_xoxc: Option<String>,
    pub xoxd: String,
    pub host: String,
    pub team_id: SlackTeamId,
    pub user_id: SlackUserId,
    #[serde(default, deserialize_with = "str_to_vec")]
    pub ignore_list: Vec<SlackUserId>,
}

fn str_to_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where D: Deserializer<'de>, T: From<String> {
    let s = String::deserialize(deserializer)?;
    Ok(s.split(",").map(|x| x.trim().to_string().into()).collect())
}