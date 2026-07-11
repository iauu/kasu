use std::fmt::Debug;
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::client::{Client, PartialClient};
use crate::lib::ctx_trait::{ToChannelId, ToThreadTs, ToMessageTs, ToUserId, Metadata, ToMetadata};
use crate::lib::event::Event;

pub trait AsyncSafe :  Send + Sync + Clone + Debug + 'static {}

impl<T> AsyncSafe for T where T: Send + Sync + Clone + Debug + 'static {}


#[derive(Clone, Debug)]
pub struct Context<T>
where T: AsyncSafe {
    pub client: Client<T>,
    pub channel_id: Option<SlackChannelId>,
    pub thread_ts: Option<SlackTs>,
    pub message_ts: Option<SlackTs>,
    pub user_id: Option<SlackUserId>
}

pub trait FromContext<T>: Send + Sync + 'static
where T : AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self> where Self: Sized;
}

impl<T> FromContext<T> for Context<T>
where T: AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self> {
        Some(ctx.clone())
    }
}

impl<T> FromContext<T> for Client<T>
where T: AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self>
    {
        Some(ctx.client.clone())
    }
}

impl <T> FromContext<T> for PartialClient
where T : AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(ctx.client.get_partial())
    }
}

pub async fn translate_to_ctx<T>(event: Event, client: Client<T>) -> (Event, Context<T>)
where T: AsyncSafe {
    let ctx = multi_to_ctx(&event, client).await;
    (event, ctx)
}

pub async fn multi_to_ctx<T>(multi: &impl ToMetadata, client: Client<T>) -> Context<T>
where T: AsyncSafe {
    Context {
        client: client.clone(),
        channel_id: multi.get_channel_id(),
        thread_ts: multi.get_thread_ts(),
        message_ts: multi.get_ts(),
        user_id: multi.get_user_id()
    }
}

impl<X, T> FromContext<X> for Option<T>
where T: FromContext<X>,
 X: AsyncSafe {
    fn from_ctx(ctx: &Context<X>) -> Option<Self>
    where
        Self: Sized
    {
        Some(T::from_ctx(ctx))
    }
}

impl<T> Context<T>
where T: AsyncSafe {
    pub async fn from(client: Client<T>, item: &impl ToMetadata) -> Self {
        multi_to_ctx(item, client).await
    }
}