use async_trait::async_trait;
use slack_morphism::{SlackChannelId, SlackTs};
use crate::lib::api::error::Error;
use crate::lib::api::MessageData;
use crate::lib::client::{Client, PartialClient};
use crate::lib::context::{AsyncSafe, FromContext};
use crate::lib::ctx_trait::{Sendable, ThreadSendable};

#[derive(Clone, Debug)]
pub struct Messageable {
    pub channel_id: SlackChannelId,
    pub thread_ts: Option<SlackTs>,
    pub message_ts: SlackTs,
    pub client: PartialClient
}

impl<T> FromContext<T> for Messageable
where T : AsyncSafe {
    fn from_ctx(ctx: &crate::lib::context::Context<T>) -> Option<Self> {
        match (ctx.channel_id.clone(), ctx.message_ts.clone()) {
            (Some(channel_id), Some(message_ts)) => Some(
                Messageable { channel_id, client: ctx.client.get_partial(), thread_ts: ctx.thread_ts.clone(), message_ts }
            ),
            _ => None
        }
    }
}

#[async_trait]
impl Sendable for Messageable {
    async fn reply(&self, message: MessageData) -> Result<SlackTs, Error> {
        self.client.read().await.api_client.chat_post_message(self.channel_id.clone(), self.thread_ts.clone(), message).await
    }
}

#[async_trait]
impl ThreadSendable for Messageable {
    async fn reply_in_thread(&self, message: MessageData) -> Result<SlackTs, Error> {
        self.client.read().await.api_client.chat_post_message(self.channel_id.clone(), self.thread_ts.clone().or(Some(self.message_ts.clone())), message).await
    }
}

