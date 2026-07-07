use std::ops::Deref;
use std::sync::{Arc, RwLock};
use crate::lib::dispatcher::EventDispatcher;

#[derive(Debug)]
pub struct ClientBase {
    xoxc_token: String,
    xoxd_token: String,
    pub(crate) event_dispatcher: EventDispatcher,
    pub ws_reconnect_url: Option<String>
}

#[derive(Clone, Debug)]
pub struct Client(pub Arc<RwLock<ClientBase>>);

impl Deref for Client {
    
    type Target = RwLock<ClientBase>;
    
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl ClientBase {
    pub(crate) fn get_xoxc(&self) -> String {
        self.xoxc_token.clone()
    }

    pub(crate) fn get_xoxd(&self) -> String {
        self.xoxd_token.clone()
    }

    pub fn new(xoxc: String, xoxd: String) -> Self {
        Self {
            xoxc_token: xoxc,
            xoxd_token: xoxd,
            event_dispatcher: EventDispatcher::new(4096),
            ws_reconnect_url: None
        }
    }

    pub async fn run(&self) -> ! {
        todo!()
    }
}

impl Client {
    pub(crate) fn get_xoxc(&self) -> String {
        self.0.read().unwrap().xoxc_token.clone()
    }
    
    pub(crate) fn get_xoxd(&self) -> String {
        self.0.read().unwrap().xoxd_token.clone()
    }
    
    pub fn new(xoxc: String, xoxd: String) -> Self {
        Self(Arc::new(RwLock::new(ClientBase::new(xoxc, xoxd))))
    }
    
    pub async fn run(&self) -> ! {
        self.0.read().unwrap().run().await;
    }
}