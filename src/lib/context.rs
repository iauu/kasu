use std::fmt::Debug;
use std::sync::{Arc};
use async_lock::{Mutex, RwLock};
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::client::{Client, PartialClient};
use crate::lib::ctx_trait::{ToChannelId, ToThreadTs, ToMessageTs, ToUserId, Metadata, ToMetadata};
use crate::lib::event::Event;

pub trait AsyncSafe :  Send + Sync + Clone + Debug + 'static {}

impl<T> AsyncSafe for T where T: Send + Sync + Clone + Debug + 'static {}


/// Marker trait for the state struct
pub trait StateMarker: AsyncSafe {}

/// Marker trait for the state struct before applying Arc with access lock
pub trait StateUnwrappedMarker: Send + Sync + Debug + 'static {}

impl<T: StateMarker> StateUnwrappedMarker for T {}

macro_rules! extend_state {
    ( $( $t:ty ),* $( , )? ) => {
        $(
            impl<T> StateMarker for $t where T : StateUnwrappedMarker {}
        )+
    };
}

extend_state!(Arc<Mutex<T>>, Arc<RwLock<T>>, Arc<std::sync::Mutex<T>>, Arc<std::sync::RwLock<T>>, Arc<tokio::sync::Mutex<T>>, Arc<tokio::sync::RwLock<T>>);

#[derive(Clone, Debug)]
pub enum State<T>
where T : AsyncSafe {
    State(T)
}


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

pub(crate) trait AsyncTranslate {}


#[async_trait::async_trait]
pub trait TransformFromContext<T>: Send + Sync + 'static
where T : AsyncSafe {
    async fn transform_from_ctx(ctx: &Context<T>) -> Option<Self> where Self: Sized;
}

#[async_trait::async_trait]
impl<X: AsyncSafe, T: FromContext<X> + AsyncTranslate> TransformFromContext<X> for T {
    async fn transform_from_ctx(ctx: &Context<X>) -> Option<Self> {
        Self::from_ctx(ctx)
    }
}

impl<T> FromContext<T> for Context<T>
where T: AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self> {
        Some(ctx.clone())
    }
}

impl<T> AsyncTranslate for Context<T>
where T: AsyncSafe {}

impl<T> FromContext<T> for Client<T>
where T: AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self>
    {
        Some(ctx.client.clone())
    }
}

impl<T> AsyncTranslate for Client<T>
where T : AsyncSafe {}

impl <T> FromContext<T> for PartialClient
where T : AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(ctx.client.get_partial())
    }
}

impl AsyncTranslate for PartialClient {}

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

// impl<X, T> FromContext<X> for Option<T>
// where T: FromContext<X>,
//  X: AsyncSafe {
//     fn from_ctx(ctx: &Context<X>) -> Option<Self>
//     where
//         Self: Sized
//     {
//         Some(T::from_ctx(ctx))
//     }
// }

#[async_trait::async_trait]
impl<X, T> TransformFromContext<X> for Option<T>
where T: TransformFromContext<X>,
      X: AsyncSafe {
    async fn transform_from_ctx(ctx: &Context<X>) -> Option<Self>
    where
        Self: Sized
    {
        Some(T::transform_from_ctx(ctx).await)
    }
}

impl<T> Context<T>
where T: AsyncSafe {
    pub async fn from(client: Client<T>, item: &impl ToMetadata) -> Self {
        multi_to_ctx(item, client).await
    }
}

impl<T> FromContext<T> for T
where T : StateMarker
{
    fn from_ctx(ctx: &Context<T>) -> Option<Self> {
        Some(ctx.client.read_blocking().state.clone())
    }
}

impl<T> AsyncTranslate for T
where T : StateMarker {}

impl<T> FromContext<T> for State<T>
where T : AsyncSafe {
    fn from_ctx(ctx: &Context<T>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(State::State(ctx.client.read_blocking().state.clone()))
    }
}

impl<T> AsyncTranslate for State<T>
where T : AsyncSafe {}