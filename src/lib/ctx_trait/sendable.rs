use async_trait::async_trait;
use slack_morphism::SlackTs;
use crate::lib::api::error::Error;
use crate::lib::api::MessageData;

#[async_trait]
pub trait Sendable {
    async fn reply(&self, message: MessageData) -> Result<SlackTs, Error>;
}

#[async_trait]
pub trait ThreadSendable {
    async fn reply_in_thread(&self, message: MessageData) -> Result<SlackTs, Error>;
}