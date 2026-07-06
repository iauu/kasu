use std::future::Future;
use async_trait::async_trait;
use crate::lib::context::Context;
use crate::lib::event::Event;
use tokio::sync::broadcast::{Sender, Receiver};

#[async_trait]
pub trait EventHandler<Args> : Send + Sync + 'static {
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
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            #[async_trait]
            impl <F, Fut, __event $(, $arg_name)*> EventHandler<(__event, $($arg_name ,)*)>  for F
            where
                F: Fn(__event, $($arg_name,)*) -> Fut + Send + Sync + 'static,
                Fut: Future<Output = Option<()>> + Send + 'static,
                __event: $crate::lib::event::FromEvent,
            $(
                $arg_name: $crate::lib::context::FromContext,
            )*
            {
                async fn call(&self, event: $crate::lib::event::Event, context: $crate::lib::context::Context) -> Option<()> {
                    let event = __event::from_event(event);
                    if let Some(event) = event {
                        $(
                            let [<$arg_name _a>] = $arg_name::from_ctx(&context);
                        )*
                        return (self)(event, $([<$arg_name _a>], )*).await;
                    }
                    None
                }
            }
        }
    };
}

macro_rules! multi_impl_event_handler {
    () => {};

    ($head:ident $($tail:ident)*) => {
        multi_impl_event_handler!($($tail)*);

        impl_event_handler!($head $($tail)*);
    };

}

multi_impl_event_handler!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10);