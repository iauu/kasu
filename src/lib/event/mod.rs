use async_trait::async_trait;
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

#[async_trait::async_trait]
pub trait TransformFromEvent: Send + Sync + 'static {
    async fn transform_from_event(event: Event) -> Option<Self> where Self: Sized;
}

#[async_trait::async_trait]
impl<T: FromEvent> TransformFromEvent for T {
    async fn transform_from_event(event: Event) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_event(event)
    }
}

impl_metadata_propagate!(Event, Websocket Cmd);