use std::future::Future;
use async_trait::async_trait;
use crate::lib::context::Context;
use crate::lib::event::Event;
use tokio::sync::broadcast::{Sender, Receiver};

#[async_trait]
pub trait EventHandler<Args> : Send + Sync + Clone + 'static {
    async fn run(&self, mut rx: Receiver<(Event, Context)>) -> std::convert::Infallible {
        loop {
            let (event, context) = rx.recv().await.unwrap();
            let _ = self.call(event, context).await;
        }
    }

    async fn call(&self, event: Event, context: Context) -> Option<()>;
}

macro_rules! impl_event_handler {
    ($($arg_name:ident)*) => {
        ::paste::paste!{
            #[allow(non_snake_case, non_camel_case_types, unused)]
            #[async_trait]
            impl <F, Fut, __event $(, $arg_name)*> EventHandler<(__event, $($arg_name ,)*)>  for F
            where
                F: Fn(__event, $($arg_name,)*) -> Fut + Send + Sync + Clone + 'static,
                Fut: Future<Output = Option<()>> + Send + 'static,
                __event: $crate::lib::event::FromEvent,
            $(
                $arg_name: $crate::lib::context::FromContext,
            )*
            {
                async fn call(&self, event: $crate::lib::event::Event, context: $crate::lib::context::Context) -> Option<()> {
                    let event = __event::from_event(event)?;
                    $(
                        let [<$arg_name _a>] = $arg_name::from_ctx(&context)?;
                    )*
                    let handler = self.clone();
                    let join_handle = ::tokio::task::spawn(async move {
                        let _ = (handler)(event, $([<$arg_name _a>], )*).await;
                    });
                    Some(())
                }
            }
        }
    };
}

macro_rules! multi_impl_event_handler {
    () => {
        impl_event_handler!();
    };

    ($head:ident $($tail:ident)*) => {
        multi_impl_event_handler!($($tail)*);

        impl_event_handler!($head $($tail)*);
    };

}

multi_impl_event_handler!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14);