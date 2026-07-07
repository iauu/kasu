use std::ops::Deref;
use std::sync::Arc;
use async_lock::RwLock;
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
}

impl Client {
    pub(crate) fn get_xoxc(&self) -> String {
        self.read_blocking().xoxc_token.clone()
    }

    pub(crate) fn get_xoxd(&self) -> String {
        self.read_blocking().xoxd_token.clone()
    }

    pub fn new(xoxc: String, xoxd: String) -> Self {
        Self(Arc::new(RwLock::new(ClientBase::new(xoxc, xoxd))))
    }

    pub async fn run(&self) -> ! {
        let client = self.clone();
        tokio::task::spawn(async move {
            crate::lib::ws::conn::ws_task(client).await
        });
        loop {}
    }
}