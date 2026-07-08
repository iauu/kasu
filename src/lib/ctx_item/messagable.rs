use slack_morphism::{SlackChannelId, SlackTs};
use crate::lib::client::Client;
use crate::lib::context::FromContext;

#[derive(Clone)]
pub struct Messageable {
    pub channel_id: SlackChannelId,
    pub thread_ts: Option<SlackTs>,
    pub client: Client
}

impl FromContext for Messageable {
    fn from_ctx(ctx: &crate::lib::context::Context) -> Option<Self> {
        match ctx.channel_id.clone() {
            Some(channel_id) => Some(Messageable { channel_id, client: ctx.client.clone(), thread_ts: ctx.thread_ts.clone() }),
            None => None
        }
    }
}