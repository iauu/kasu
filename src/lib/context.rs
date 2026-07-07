use crate::lib::client::Client;
use crate::lib::event::Event;

#[derive(Clone, Debug)]
pub struct Context;

pub trait FromContext: Send + Sync + 'static {
    fn from_ctx(event: &Context) -> Option<Self> where Self: Sized;
}

impl FromContext for Context {
    fn from_ctx(event: &Context) -> Option<Self> {
        Some(event.clone())
    }
}


pub async fn translate_to_ctx(event: Event, client: Client) -> (Event, Context) {
    todo!()
}