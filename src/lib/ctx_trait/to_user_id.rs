use slack_morphism::{SlackUserId};
use crate::lib::ctx_trait::ToMulti;

pub trait ToUserId {
    fn get_user_id(&self) -> Option<SlackUserId> {
        None
    }
}

impl<T> ToUserId for T
where T : ToMulti {
    fn get_user_id(&self) -> Option<SlackUserId> {
        self.get_multi().user_id
    }
}
