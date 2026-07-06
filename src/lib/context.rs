use crate::lib::event::Event;

#[derive(Clone)]
pub struct Context;

pub trait FromContext: Send + Sync + 'static {
    fn from_ctx(event: &Context) -> Self where Self: Sized;
}

impl FromContext for Context {
    fn from_ctx(event: &Context) -> Self {
        event.clone()
    }
}
