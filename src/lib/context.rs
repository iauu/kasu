use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::client::Client;
use crate::lib::ctx_trait::{ToChannelId, ToThreadTs, ToMessageTs};
use crate::lib::event::Event;

#[derive(Clone, Debug)]
pub struct Context {
    pub client: Client,
    pub channel_id: Option<SlackChannelId>,
    pub thread_ts: Option<SlackTs>,
    pub message_ts: Option<SlackTs>,
}

pub trait FromContext: Send + Sync + 'static {
    fn from_ctx(ctx: &Context) -> Option<Self> where Self: Sized;
}

impl FromContext for Context {
    fn from_ctx(ctx: &Context) -> Option<Self> {
        Some(ctx.clone())
    }
}

impl FromContext for Client {
    fn from_ctx(ctx: &Context) -> Option<Self>
    {
        Some(ctx.client.clone())
    }
}

pub async fn translate_to_ctx(event: Event, client: Client) -> (Event, Context) {
    let ctx = Context { client: client.clone(), channel_id: event.get_channel_id(), thread_ts: event.get_thread_ts(), message_ts: event.get_ts() };
    (event, ctx)
}

impl<T: FromContext> FromContext for Option<T> {
    fn from_ctx(ctx: &Context) -> Option<Self>
    where
        Self: Sized
    {
        Some(T::from_ctx(ctx))
    }
}