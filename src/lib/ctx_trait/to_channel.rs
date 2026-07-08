use slack_morphism::SlackChannelId;
use crate::lib::ctx_trait::ToMulti;

pub trait ToChannelId {
    fn get_channel_id(&self) -> Option<SlackChannelId> {
        None
    }
}

impl<T> ToChannelId for T
where T : ToMulti {
    fn get_channel_id(&self) -> Option<SlackChannelId> {
        self.get_multi().channel_id
    }
}
