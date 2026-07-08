use serde::Deserialize;
use slack_morphism::{SlackChannelId, SlackClientMessageId, SlackTeamId, SlackTs, SlackUserId};
use crate::lib::event::{Event, FromEvent};
use crate::lib::blocks::SlackBlock;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketEvent {
    #[serde(rename="user_typing")]
    Typing(WebsocketUserTypingEvent),
    #[serde(rename="reconnect_url")]
    ReconnectUrl(WebsocketReconnectUrlEvent),
    #[serde(rename="message")]
    Message(WebsocketMessageReceivedEvent)
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
    pub channel_id: SlackChannelId,
    #[serde(default)]
    pub thread_ts: Option<SlackTs>,
    #[serde(default)]
    pub id: Option<usize>,
    #[serde(rename="user")]
    pub user_id: SlackUserId
}

macro_rules! ws_from_event_impl {
    ($name:ty, $event:ident) => {
        impl $crate::lib::event::FromEvent for $name {
            fn from_event(event: $crate::lib::event::Event) -> Option<Self>
            where
                Self: Sized,
            {
                let ws: WebsocketEvent = $crate::lib::ws::event::WebsocketEvent::from_event(event)?;
                match ws {
                    $crate::lib::ws::event::WebsocketEvent::$event(event) => Some(event),
                    _ => None
                }
            }
        }
    };
}




#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketReconnectUrlEvent {
    pub url: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketMessageReceivedEvent {
    #[serde(rename="channel")]
    pub channel_id: SlackChannelId,
    pub text: Option<String>,
    pub blocks: Option<SlackBlock>,
    #[serde(rename="user")]
    pub user_id: SlackUserId,
    pub client_msg_id: SlackClientMessageId,
    #[serde(rename="team")]
    pub team_id: SlackTeamId,
    #[serde(rename="source_team")]
    pub source_team_id: SlackTeamId,
    #[serde(rename="user_team")]
    pub user_team_id: SlackTeamId,
    pub suppress_notification: bool,
    event_ts: String,
    ts: SlackTs
}

ws_from_event_impl!(WebsocketMessageReceivedEvent, Message);
ws_from_event_impl!(WebsocketUserTypingEvent, Typing);
ws_from_event_impl!(WebsocketReconnectUrlEvent, ReconnectUrl);

impl Into<Event> for WebsocketEvent {
    fn into(self) -> Event {
        Event::Websocket(self)
    }
}