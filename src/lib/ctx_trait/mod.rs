pub mod sendable;

use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
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

/// Implement To conversion trait
#[macro_export]
macro_rules! to_impl {
    ($trait_name:ident, $fn_name:ident, $field:ident, $ty:ty) => {
        pub trait $trait_name {
            fn $fn_name(&self) -> Option<$ty> {
                None
            }
        }
        
        impl<T: $crate::lib::ctx_trait::ToMulti> $trait_name for T {
            fn $fn_name(&self) -> Option<$ty> {
                self.get_multi().$field
            }
        }
    };
}

to_impl!(ToChannelId, get_channel_id, channel_id, SlackChannelId);
to_impl!(ToThreadTs, get_thread_ts, thread_ts, SlackTs);
to_impl!(ToMessageTs, get_ts, message_ts, SlackTs);
to_impl!(ToUserId, get_user_id, user_id, SlackUserId);