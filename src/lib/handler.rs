use std::future::Future;
use async_trait::async_trait;
use crate::lib::context::{AsyncSafe, Context, ReduceState, StateTrait};
use crate::lib::event::Event;
use tokio::sync::broadcast::{Receiver};
use crate::lib::dispatcher::EventDispatcher;

#[async_trait]
pub trait EventHandler<Args, Ret, T> : Send + Sync + Clone + 'static
where T : StateTrait {
    async fn run(&self, mut rx: Receiver<(Event, Context<T>)>) -> std::convert::Infallible {
        loop {
            let (event, context) = rx.recv().await.unwrap();
            let _ = self.call(event, context).await;
        }
    }

    async fn call(&self, event: Event, context: Context<T>) -> Option<()>;
}

pub trait AnyRes : Send + Sync + 'static {}


impl<T: Send + Sync + 'static> AnyRes for T {}

macro_rules! impl_event_handler {
    ($($arg_name:ident)*) => {
        ::paste::paste!{
            #[allow(non_snake_case, non_camel_case_types, unused)]
            #[async_trait]
            impl <F, Fut, __event $(, $arg_name)*, R, T> EventHandler<(__event, $($arg_name ,)*), R, T>  for F
            where
                F: Fn(__event, $($arg_name,)*) -> Fut + Send + Sync + Clone + 'static,
                Fut: Future<Output = R> + Send + 'static,
                R: AnyRes + Send + Sync + 'static,
                __event: $crate::lib::event::TransformFromEvent,
                T: $crate::lib::context::StateTrait,
            $(
                $arg_name: $crate::lib::context::TransformFromContext<T>,
            )*
            {
                async fn call(&self, event: $crate::lib::event::Event, context: $crate::lib::context::Context<T>) -> Option<()> {
                    let event = __event::transform_from_event(event).await?;
                    $(
                        let [<$arg_name _a>] = $arg_name::transform_from_ctx(&context).await?;
                    )*
                    let handler = self.clone();
                    let join_handle = ::tokio::task::spawn(async move {
                        let _ = (handler)(event, $([<$arg_name _a>], )*).await;
                    });
                    Some(())
                }
            }

            #[allow(non_snake_case, non_camel_case_types, unused)]
            #[async_trait]
            impl <F, Fut, __event $(, $arg_name)*, R, T, S, C> EventHandler<(__event, $($arg_name ,)*), $crate::lib::callback::Middleware<R, C, T>, T>  for (S, F)
            where
                F: Fn(__event, $($arg_name,)*) -> Fut + Send + Sync + Clone + 'static,
                Fut: Future<Output = $crate::lib::callback::Middleware<R, C, T>> + Send + 'static,
                R: Send + Sync + 'static,
                C: $crate::lib::callback::Callback<R, T> + Send + Sync,
                __event: $crate::lib::cmd::event::TransFromEventCmd,
                T: $crate::lib::context::StateTrait,
                S: ToString + Send + Sync + Clone + 'static,
            $(
                $arg_name: $crate::lib::context::TransformFromContext<T>,
            )*
            {
                async fn call(&self, event: $crate::lib::event::Event, context: $crate::lib::context::Context<T>) -> Option<()> {
                    let event = __event::transform_from_event(event).await?;
                    if event.get_cmd() != self.0.to_string() {
                        return None;
                    }
                    $(
                        let [<$arg_name _a>] = $arg_name::transform_from_ctx(&context).await?;
                    )*
                    let handler = self.1.clone();
                    let join_handle = ::tokio::task::spawn(async move {
                        let r: $crate::lib::callback::Middleware<R, C, T> = (handler)(event, $([<$arg_name _a>], )*).await;
                        let _ = r.call_middleware(&context).await;
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

pub fn spawn_handler<Args, Ret, H, T>(dispatcher: &EventDispatcher<T>, handler: H)
where
    H: EventHandler<Args, Ret, T>,
    T: AsyncSafe + ReduceState<()>,
{
    let rx = dispatcher.subscribe();
    tokio::task::spawn(async move {
        handler.run(rx).await;
    });
}