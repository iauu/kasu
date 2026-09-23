use async_trait::async_trait;
use crate::lib::api::MessageData;
use crate::lib::callback::{Callback, Creatable};
use crate::lib::context::{Context, FromContext, StateTrait};
use crate::lib::ctx_item::Messageable;
use crate::lib::ctx_trait::ThreadSendable;

pub struct ReplyInThread;

impl Creatable for ReplyInThread {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl<S: StateTrait> Callback<String, S> for ReplyInThread {
    type Return = ();

    async fn call(&self, ctx: &Context<S>, input: String) -> Self::Return {
        ReplyInThread.call(ctx, MessageData::Raw(input)).await
    }
}

#[async_trait::async_trait]
impl<S: StateTrait> Callback<Option<String>, S> for ReplyInThread {
    type Return = ();

    async fn call(&self, ctx: &Context<S>, input: Option<String>) -> Self::Return {
        if let Some(input) = input
        {
            ReplyInThread.call(ctx, input).await
        }
    }
}

#[async_trait::async_trait]
impl<S: StateTrait> Callback<MessageData, S> for ReplyInThread {
    type Return = ();

    async fn call(&self, ctx: &Context<S>, input: MessageData) -> Self::Return {
        if let Some(m) =  Messageable::from_ctx(&ctx) {
            if let Err(e) = m.reply_in_thread(input).await {
                tracing::warn!("Failed to reply in thread: {:?}", e);
            }
        }
    }
}

#[async_trait::async_trait]
impl<S: StateTrait> Callback<Option<MessageData>, S> for ReplyInThread {
    type Return = ();

    async fn call(&self, ctx: &Context<S>, input: Option<MessageData>) -> Self::Return {
        if let Some(input) = input
        {
            ReplyInThread.call(ctx, input).await
        }
    }
}