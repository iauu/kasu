use async_trait::async_trait;
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::api::error::Error;
use crate::lib::api::MessageData;
use crate::lib::client::{Client, PartialClient};
use crate::lib::context::{AsyncSafe, FromContext};
use crate::lib::ctx_trait::Sendable;

#[derive(Clone, Debug)]
pub struct PartialUser {
    pub user_id: SlackUserId,
    pub client: PartialClient
}

impl<T> FromContext<T> for PartialUser 
where T : AsyncSafe {
    fn from_ctx(ctx: &crate::lib::context::Context<T>) -> Option<Self> {
        match ctx.user_id.clone() {
            Some(user_id) => Some(PartialUser { user_id, client: ctx.client.get_partial() }),
            None => None
        }
    }
}
