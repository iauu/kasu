use crate::lib::event::Event;

pub struct Context;

pub trait FromContext: Send + Sync + 'static {
    fn from_ctx(event: &Context) -> Self where Self: Sized;
}
