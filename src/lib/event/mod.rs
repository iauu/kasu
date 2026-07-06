use crate::lib::context::Context;

#[derive(Clone, Debug)]
pub enum Event {}

pub trait FromEvent: Send + Sync + 'static {
    fn from_event(event: Event) -> Option<Self> where Self: Sized;
}

impl FromEvent for Event {
    fn from_event(event: Event) -> Option<Self> {
        Some(event)
    }
}
