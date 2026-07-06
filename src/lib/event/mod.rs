use crate::lib::context::Context;

#[derive(Clone)]
pub enum Event {}

pub trait FromEvent: Send + Sync + 'static {
    fn from_event(event: Event) -> Option<Self> where Self: Sized;
}
