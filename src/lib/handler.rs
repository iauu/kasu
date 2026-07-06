use std::future::Future;
use crate::lib::context::Context;
use crate::lib::event::Event;

pub trait EventHandler<Args> : Send + Sync + 'static {
    fn call(&self, event: Event, context: Context) -> impl Future<Output = Option<()>> + Send;
}

macro_rules! impl_event_handler {
    ($($arg_name:ident)*) => {
        ::paste::paste!{
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
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