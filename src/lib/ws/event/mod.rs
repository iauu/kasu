use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketEvent {
    #[serde(rename="user_typing")]
    Typing(WebsocketUserTypingEvent)
}

#[derive(Clone, Debug, Deserialize)]
pub struct  WebsocketUserTypingEvent {
    #[serde(rename="channel")]
    channel_id: String,
    #[serde(default)]
    thread_ts: Option<String>,
    id: usize,
    #[serde(rename="user")]
    user_id: String
}