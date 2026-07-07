use tokio::sync::broadcast::{Sender, Receiver};
use crate::lib::context::Context;
use crate::lib::event::Event;

#[derive(Clone, Debug)]
pub struct EventDispatcher {
    tx: Sender<(Event, Context)>
}



impl EventDispatcher {
    pub fn new(size: usize) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(size);
        Self {
            tx
        }
    }

    pub(crate) fn send(&self, event: Event, context: Context) -> () {
        self.tx.send((event, context)).unwrap();
    }

    fn subscribe(&self) -> Receiver<(Event, Context)> {
        self.tx.subscribe()
    }
}