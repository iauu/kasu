use async_trait::async_trait;
use slack_morphism::{SlackChannelId, SlackTs};
use crate::lib::api::error::Error;
use crate::lib::api::MessageData;
use crate::lib::client::Client;
use crate::lib::context::FromContext;
use crate::lib::ctx_trait::Sendable;

#[derive(Clone)]
pub struct PartialChannel {
    pub channel_id: SlackChannelId,
    pub client: Client
}

impl FromContext for PartialChannel {
    fn from_ctx(ctx: &crate::lib::context::Context) -> Option<Self> {
        match ctx.channel_id.clone() {
            Some(channel_id) => Some(PartialChannel { channel_id, client: ctx.client.clone() }),
            None => None
        }
    }
}

#[async_trait]
impl Sendable for PartialChannel {
    async fn reply(&self, message: MessageData) -> Result<SlackTs, Error> {
        self.client.read().await.api_client.chat_post_message(self.channel_id.clone(), None, message).await
    }
}