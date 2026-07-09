use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::client::Client;
use crate::lib::ctx_trait::{ToChannelId, ToThreadTs, ToMessageTs, ToUserId, Metadata, ToMetadata};
use crate::lib::event::Event;

#[derive(Clone, Debug)]
pub struct Context {
    pub client: Client,
    pub channel_id: Option<SlackChannelId>,
    pub thread_ts: Option<SlackTs>,
    pub message_ts: Option<SlackTs>,
    pub user_id: Option<SlackUserId>
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
    let ctx = multi_to_ctx(&event, client).await;
    (event, ctx)
}

pub async fn multi_to_ctx(multi: &impl ToMetadata, client: Client) -> Context {
    Context {
        client: client.clone(),
        channel_id: multi.get_channel_id(),
        thread_ts: multi.get_thread_ts(),
        message_ts: multi.get_ts(),
        user_id: multi.get_user_id()
    }
}

impl<T: FromContext> FromContext for Option<T> {
    fn from_ctx(ctx: &Context) -> Option<Self>
    where
        Self: Sized
    {
        Some(T::from_ctx(ctx))
    }
}

impl Context {
    pub async fn from(client: Client, item: &impl ToMetadata) -> Self {
        multi_to_ctx(item, client).await
    }
}