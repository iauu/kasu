use serde::Deserialize;
use crate::lib::event::Event;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketEvent {
    #[serde(rename="user_typing")]
    Typing(WebsocketUserTypingEvent),
    #[serde(rename="reconnect_url")]
    ReconnectUrl(WebsocketReconnectUrlEvent),
}

#[derive(Clone, Debug, Deserialize)]
pub struct  WebsocketUserTypingEvent {
    #[serde(rename="channel")]
    channel_id: String,
    #[serde(default)]
    thread_ts: Option<String>,
    #[serde(default)]
    id: Option<usize>,
    #[serde(rename="user")]
    user_id: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketReconnectUrlEvent {
    url: String,
}

impl Into<Event> for WebsocketEvent {
    fn into(self) -> Event {
        todo!()
    }
}