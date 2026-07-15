use std::ops::Deref;
use std::sync::Arc;
use async_lock::RwLock;
use slack_morphism::{SlackTeamId, SlackUserId};
use url::Host;
use crate::lib::api::APIClient;
use crate::lib::context::AsyncSafe;
use crate::lib::dispatcher::EventDispatcher;
use crate::lib::handler::spawn_handler;

#[derive(Clone, Debug)]
pub struct ClientState {
    pub(crate) sub_xoxc_token: Option<String>,
    pub(crate) xoxc_token: String,
    pub(crate) xoxd_token: String,
    pub ws_reconnect_url: Option<String>,
    pub api_client: APIClient,
    pub host: String,
    pub team_id: SlackTeamId,
    pub user_id: SlackUserId
}

#[derive(Debug)]
pub struct ClientBase<T>
where T: AsyncSafe {
    pub internal: Arc<RwLock<ClientState>>,
    pub(crate) event_dispatcher: EventDispatcher<T>,
    pub state: T,
}

pub type PartialClient = Arc<RwLock<ClientState>>;

impl<T> Deref for ClientBase<T>
where T: AsyncSafe {

    type Target = RwLock<ClientState>;

    fn deref(&self) -> &Self::Target {
        self.internal.deref()
    }
}


#[derive(Clone, Debug)]
pub struct Client<T>(pub Arc<RwLock<ClientBase<T>>>) where T: AsyncSafe;

impl<T> Deref for Client<T>
where T: AsyncSafe {

    type Target = RwLock<ClientBase<T>>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> ClientBase<T>
where T: AsyncSafe {
    pub(crate) fn get_xoxc(&self) -> String {
        self.read_blocking().xoxc_token.clone()
    }

    pub(crate) fn get_xoxd(&self) -> String {
        self.read_blocking().xoxd_token.clone()
    }

    pub fn new(sub_xoxc: Option<String>, xoxc: String, xoxd: String, host: String, team_id: SlackTeamId, state: T, user_id: SlackUserId) -> Self {
        Self {
            internal: Arc::new(RwLock::new(ClientState {
                sub_xoxc_token: sub_xoxc.clone(),
                xoxc_token: xoxc.clone(),
                xoxd_token: xoxd.clone(),
                ws_reconnect_url: None,
                api_client: APIClient::new(sub_xoxc, xoxc, xoxd, host.clone(), team_id.clone()),
                host,
                team_id,
                user_id
            })),
            event_dispatcher: EventDispatcher::new(4096),
            state
        }
    }

    pub fn get_partial(&self) -> PartialClient {
        self.internal.clone()
    }
}

impl<T> Client<T>
where T: AsyncSafe {
    pub(crate) fn get_xoxc(&self) -> String {
        self.read_blocking().get_xoxc()
    }

    pub(crate) fn get_xoxd(&self) -> String {
        self.read_blocking().get_xoxd()
    }

    pub fn new_with_state(sub_xoxc: Option<String>, xoxc: String, xoxd: String, host: String, team_id: SlackTeamId, state: T, user_id: SlackUserId) -> Self {
        Self(Arc::new(RwLock::new(ClientBase::<T>::new(sub_xoxc, xoxc, xoxd, host, team_id, state, user_id))))
    }

    pub async fn run(&self) -> ! {
        let client = self.clone();
        tokio::task::spawn(async move {
            crate::lib::ws::conn::ws_task(client).await
        });
        spawn_handler(&self.read().await.event_dispatcher, crate::lib::ws::conn::set_reconnect);
        spawn_handler(&self.read().await.event_dispatcher, crate::lib::cmd::handler::cmd_handler);
        // loop {
        //     tokio::task::yield_now().await;
        // }
        loop {
            std::future::pending::<()>().await;
        }
    }
    
    pub(crate) async fn change_ws_connecting_url(&self, url: String) -> () {
        self.write().await.write().await.ws_reconnect_url.replace(url);
    }
    
    pub(crate) async fn get_ws_connecting_url(&self) -> Option<String> {
        self.read().await.read().await.ws_reconnect_url.clone()
    }
    
    pub fn get_partial(&self) -> PartialClient {
        self.read_blocking().internal.clone()
    }
}