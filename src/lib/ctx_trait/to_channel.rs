use slack_morphism::SlackChannelId;

pub trait ToChannelId {
    fn get_channel_id(&self) -> Option<SlackChannelId> {
        None
    }
}