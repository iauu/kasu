use slack_morphism::{SlackChannelId, SlackTs};
use crate::lib::ctx_trait::{ToChannelId, ToMulti};

pub trait ToThreadTs {
    fn get_thread_ts(&self) -> Option<SlackTs> {
        None
    }
}

impl<T> ToThreadTs for T
where T : ToMulti {
    fn get_thread_ts(&self) -> Option<SlackTs> {
        self.get_multi().thread_ts
    }
}