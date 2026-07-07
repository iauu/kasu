use crate::lib::client::Client;
use crate::lib::event::Event;

#[derive(Clone, Debug)]
pub struct Context {
    pub client: Client
}

pub trait FromContext: Send + Sync + 'static {
    fn from_ctx(event: &Context) -> Option<Self> where Self: Sized;
}

impl FromContext for Context {
    fn from_ctx(event: &Context) -> Option<Self> {
        Some(event.clone())
    }
}

impl FromContext for Client {
    fn from_ctx(event: &Context) -> Option<Self>
    {
        Some(event.client.clone())
    }
}

pub async fn translate_to_ctx(event: Event, client: Client) -> (Event, Context) {
    (event, Context { client: client.clone() })
}
