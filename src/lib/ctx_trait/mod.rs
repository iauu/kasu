pub mod sendable;

use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
pub use sendable::{Sendable, ThreadSendable};

#[derive(Clone, Debug, Default)]
pub struct Metadata {
    pub channel_id: Option<SlackChannelId>,
    pub message_ts: Option<SlackTs>,
    pub thread_ts: Option<SlackTs>,
    pub user_id: Option<SlackUserId>
}


pub trait ToMetadata: Sized {
    fn get_metadata(&self) -> Metadata {
        Metadata::default()
    }
}

impl ToMetadata for Metadata {
    fn get_metadata(&self) -> Metadata {
        self.clone()
    }
}


macro_rules! auto_impl_metadata {
    ($t:ty, $field:ident) => {
        impl $crate::lib::ctx_trait::ToMetadata for $t {
            fn get_metadata(&self) -> $crate::lib::ctx_trait::Metadata {
                Metadata {
                    $field: Some(self.clone()),
                    ..Default::default()
                }
            }
        }
    };
}

auto_impl_metadata!(SlackTs, message_ts);
auto_impl_metadata!(SlackChannelId, channel_id);
auto_impl_metadata!(SlackUserId, user_id);


/// Auto implement `ToMulti` for enum if every variant contain a single value in tuple which implement `ToMulti`
#[macro_export]
macro_rules! impl_metadata_propagate {
    ($ev:ty, $($variant:ident)+) => {
        impl $crate::lib::ctx_trait::ToMetadata for $ev {
            fn get_metadata(&self) -> $crate::lib::ctx_trait::Metadata {
                match self {
                    $(
                        Self::$variant(event) => event.get_metadata(),
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

        impl<T: $crate::lib::ctx_trait::ToMetadata> $trait_name for T {
            fn $fn_name(&self) -> Option<$ty> {
                self.get_metadata().$field
            }
        }
    };
}

to_impl!(ToChannelId, get_channel_id, channel_id, SlackChannelId);
to_impl!(ToThreadTs, get_thread_ts, thread_ts, SlackTs);
to_impl!(ToMessageTs, get_ts, message_ts, SlackTs);
to_impl!(ToUserId, get_user_id, user_id, SlackUserId);