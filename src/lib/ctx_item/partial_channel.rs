use slack_morphism::{SlackChannelId};
use crate::lib::context::FromContext;

#[derive(Clone)]
pub struct PartialChannel {
    pub channel_id: SlackChannelId
}

impl FromContext for PartialChannel {
    fn from_ctx(ctx: &crate::lib::context::Context) -> Option<Self> {
        match ctx.channel_id.clone() {
            Some(channel_id) => Some(PartialChannel { channel_id }),
            None => None
        }
    }
}