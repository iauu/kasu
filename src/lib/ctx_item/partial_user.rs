use async_trait::async_trait;
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::api::error::Error;
use crate::lib::api::MessageData;
use crate::lib::client::Client;
use crate::lib::context::FromContext;
use crate::lib::ctx_trait::Sendable;

#[derive(Clone, Debug)]
pub struct PartialUser {
    pub user_id: SlackUserId,
    pub client: Client
}

impl FromContext for PartialUser {
    fn from_ctx(ctx: &crate::lib::context::Context) -> Option<Self> {
        match ctx.user_id.clone() {
            Some(user_id) => Some(PartialUser { user_id, client: ctx.client.clone() }),
            None => None
        }
    }
}
