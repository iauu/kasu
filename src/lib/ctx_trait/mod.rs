mod to_channel;
mod to_slack_ts;
mod to_slack_thread_ts;
pub mod sendable;

use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
pub use to_channel::ToChannelId;
pub use to_slack_ts::ToMessageTs;
pub use to_slack_thread_ts::ToThreadTs;
pub use sendable::{Sendable, ThreadSendable};

#[derive(Clone, Debug, Default)]
pub struct Multi {
    pub channel_id: Option<SlackChannelId>,
    pub message_ts: Option<SlackTs>,
    pub thread_ts: Option<SlackTs>,
    pub user_id: Option<SlackUserId>
}


pub trait ToMulti {
    fn get_multi(&self) -> Multi {
        Multi::default()
    }
}