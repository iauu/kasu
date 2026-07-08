use crate::lib::ctx_trait::{Multi, ToMulti};
use crate::lib::ws::event::WebsocketEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Websocket(WebsocketEvent)
}

pub trait FromEvent: Send + Sync + 'static {
    fn from_event(event: Event) -> Option<Self> where Self: Sized;
}

impl FromEvent for Event {
    fn from_event(event: Event) -> Option<Self> {
        Some(event)
    }
}

impl ToMulti for Event {
    fn get_multi(&self) -> Multi {
        match self {
            Event::Websocket(event) => event.get_multi(),
        }
    }
}