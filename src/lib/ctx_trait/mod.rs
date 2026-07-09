mod to_channel;
mod to_slack_ts;
mod to_slack_thread_ts;
pub mod sendable;
mod to_user_id;

use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
pub use to_channel::ToChannelId;
pub use to_slack_ts::ToMessageTs;
pub use to_slack_thread_ts::ToThreadTs;
pub use to_user_id::ToUserId;
pub use sendable::{Sendable, ThreadSendable};

#[derive(Clone, Debug, Default)]
pub struct Multi {
    pub channel_id: Option<SlackChannelId>,
    pub message_ts: Option<SlackTs>,
    pub thread_ts: Option<SlackTs>,
    pub user_id: Option<SlackUserId>
}


pub trait ToMulti : Sized {
    fn get_multi(&self) -> Multi {
        Multi::default()
    }
}

impl ToMulti for Multi {
    fn get_multi(&self) -> Multi {
        self.clone()
    }
}


macro_rules! auto_impl_multi {
    ($t:ty, $field:ident) => {
        impl $crate::lib::ctx_trait::ToMulti for $t {
            fn get_multi(&self) -> $crate::lib::ctx_trait::Multi {
                Multi {
                    $field: Some(self.clone()),
                    ..Default::default()
                }
            }
        }
    };
}

auto_impl_multi!(SlackTs, message_ts);
auto_impl_multi!(SlackChannelId, channel_id);
auto_impl_multi!(SlackUserId, user_id);


/// Auto implement `ToMulti` for enum if every variant contain a single value in tuple which implement `ToMulti`
#[macro_export]
macro_rules! impl_multi_propagate {
    ($ev:ty, $($variant:ident)+) => {
        impl $crate::lib::ctx_trait::ToMulti for $ev {
            fn get_multi(&self) -> $crate::lib::ctx_trait::Multi {
                match self {
                    $(
                        Self::$variant(event) => event.get_multi(),
                    )+
                }
            }
        }
    };
}