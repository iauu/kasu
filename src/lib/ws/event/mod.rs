use serde::Deserialize;
use crate::lib::event::{Event, FromEvent};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketEvent {
    #[serde(rename="user_typing")]
    Typing(WebsocketUserTypingEvent),
    #[serde(rename="reconnect_url")]
    ReconnectUrl(WebsocketReconnectUrlEvent),
}

impl FromEvent for WebsocketEvent {
    fn from_event(event: Event) -> Option<Self> {
        match event {
            Event::Websocket(event) => Some(event),
            _ => None
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct  WebsocketUserTypingEvent {
    #[serde(rename="channel")]
    pub channel_id: String,
    #[serde(default)]
    pub thread_ts: Option<String>,
    #[serde(default)]
    pub id: Option<usize>,
    #[serde(rename="user")]
    pub user_id: String
}

impl FromEvent for WebsocketUserTypingEvent {
    fn from_event(event: Event) -> Option<Self>
    where
        Self: Sized,
    {
        let ws: WebsocketEvent = WebsocketEvent::from_event(event)?;
        match ws {
            WebsocketEvent::Typing(event) => Some(event),
            _ => None
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketReconnectUrlEvent {
    pub url: String,
}

impl FromEvent for WebsocketReconnectUrlEvent {
    fn from_event(event: Event) -> Option<Self>
    where
        Self: Sized,
    {
        let ws: WebsocketEvent = WebsocketEvent::from_event(event)?;
        match ws {
            WebsocketEvent::ReconnectUrl(event) => Some(event),
            _ => None
        }
    }
}

impl Into<Event> for WebsocketEvent {
    fn into(self) -> Event {
        Event::Websocket(self)
    }
}