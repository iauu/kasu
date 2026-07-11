use tokio::sync::broadcast::{Sender, Receiver};
use crate::lib::context::{AsyncSafe, Context};
use crate::lib::event::Event;

#[derive(Clone, Debug)]
pub struct EventDispatcher<T>
where T : AsyncSafe {
    tx: Sender<(Event, Context<T>)>
}



impl<T> EventDispatcher<T>
where T : AsyncSafe {
    pub fn new(size: usize) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(size);
        Self {
            tx
        }
    }

    pub(crate) fn send(&self, event: Event, context: Context<T>) -> () {
        self.tx.send((event, context)).unwrap();
    }

    pub(crate) fn subscribe(&self) -> Receiver<(Event, Context<T>)> {
        self.tx.subscribe()
    }
}