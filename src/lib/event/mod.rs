use crate::impl_metadata_propagate;
use crate::lib::cmd::CmdEvent;
use crate::lib::ctx_trait::{Metadata, ToMetadata};
use crate::lib::ws::event::WebsocketEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Websocket(WebsocketEvent),
    Cmd(CmdEvent)
}

pub trait FromEvent: Send + Sync + 'static {
    fn from_event(event: Event) -> Option<Self> where Self: Sized;
}

impl FromEvent for Event {
    fn from_event(event: Event) -> Option<Self> {
        Some(event)
    }
}

impl_metadata_propagate!(Event, Websocket Cmd);