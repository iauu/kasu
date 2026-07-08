use std::ops::Deref;
use std::sync::Arc;
use async_lock::RwLock;
use url::Host;
use crate::lib::api::APIClient;
use crate::lib::dispatcher::EventDispatcher;
use crate::lib::handler::spawn_handler;

#[derive(Debug)]
pub struct ClientBase {
    xoxc_token: String,
    xoxd_token: String,
    pub(crate) event_dispatcher: EventDispatcher,
    pub ws_reconnect_url: Option<String>,
    pub api_client: APIClient,
    pub host: String
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

    pub fn new(xoxc: String, xoxd: String, host: String) -> Self {
        Self {
            xoxc_token: xoxc.clone(),
            xoxd_token: xoxd.clone(),
            event_dispatcher: EventDispatcher::new(4096),
            ws_reconnect_url: None,
            api_client: APIClient::new(xoxc, xoxd, host.clone()),
            host
        }
    }
}

impl Client {
    pub(crate) fn get_xoxc(&self) -> String {
        self.read_blocking().get_xoxc()
    }

    pub(crate) fn get_xoxd(&self) -> String {
        self.read_blocking().get_xoxd()
    }

    pub fn new(xoxc: String, xoxd: String, host: String) -> Self {
        Self(Arc::new(RwLock::new(ClientBase::new(xoxc, xoxd, host))))
    }

    pub async fn run(&self) -> ! {
        let client = self.clone();
        tokio::task::spawn(async move {
            crate::lib::ws::conn::ws_task(client).await
        });
        spawn_handler(&self.read().await.event_dispatcher, crate::lib::ws::conn::set_reconnect);
        loop {}
    }
}