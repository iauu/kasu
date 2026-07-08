use slack_morphism::SlackTs;
use crate::lib::ctx_trait::ToMulti;

pub trait ToMessageTs {
    fn get_ts(&self) -> Option<SlackTs> {
        None
    }
}

impl<T> ToMessageTs for T
where T : ToMulti {
    fn get_ts(&self) -> Option<SlackTs> {
        self.get_multi().message_ts
    }
}