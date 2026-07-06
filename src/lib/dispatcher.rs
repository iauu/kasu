use tokio::sync::broadcast::{Sender, Receiver};
use crate::lib::context::Context;
use crate::lib::event::Event;

pub struct EventDispatcher {
    tx: Sender<(Event, Context)>
}

impl EventDispatcher {
    fn send(&self, event: Event, context: Context) -> () {
        self.tx.send((event, context)).unwrap();
    }
    
    fn subscribe(&self) -> Receiver<(Event, Context)> {
        self.tx.subscribe()
    }
}