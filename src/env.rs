use url::Host;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Env {
    pub xoxc: String,
    pub xoxd: String,
    pub host: String
}