use std::convert::Infallible;
use std::time::Instant;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::lib::ws::event::WebsocketEvent;
use thiserror::Error;
use serde_json;
use tokio_retry::strategy::jitter;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tracing::instrument;
use crate::lib::client::Client;
use crate::lib::context::translate_to_ctx;
use crate::lib::event::Event;

#[derive(Error, Debug)]
pub enum WebsocketInitialConnectionError {
    #[error("Websocket initialization failed")]
    Error(#[from] tokio_tungstenite::tungstenite::Error)
}



/// Handle only connecting the websocket and return a receiver.
/// When the websocket disconnect, the sender will be dropped and therefore
/// An error will be raised on the receiver when the receiver attempt to receive
/// In this case a reconnection should be established.
pub async fn connect_ws(xoxc: String, xoxd: String, url: Option<String>) -> Result<UnboundedReceiver<WebsocketEvent>, WebsocketInitialConnectionError> {
    let url = match url {
        Some(url) => url,
        None => format!("wss-primary.slack.com/?token={}", xoxc)
    };
    let mut request = url.into_client_request()?;
    request.headers_mut().insert("Cookie", format!("tz=0; d={}", xoxd).parse().unwrap());
    let (mut socket, _) = connect_async(request).await?;
    let (tx, rx) = unbounded_channel::<WebsocketEvent>();
    tokio::spawn(async move  {
        while let Some(message) = socket.next().await {
            let message = message.unwrap();
            let message = message.to_text().unwrap();
            let event: WebsocketEvent = serde_json::from_str(message).unwrap(); // TODO: Don't unwrap this
            tx.send(event).unwrap();
        }
    });
    Ok(rx)
}

macro_rules! expo_backoff {
    () => {tokio_retry::strategy::ExponentialBackoff::from_millis(100).map(jitter).take(20)};
}

#[instrument(level = "info", skip(client), fields(module = module_path!()), target = "ws_task")]
pub async fn ws_task(client: Client) -> Infallible {
    let mut retry = expo_backoff!();
    loop {
        let mut rx = connect_ws(
            client.get_xoxc(),
            client.get_xoxd(),
            client.read().unwrap().ws_reconnect_url.clone()
        ).await.unwrap();
        let start = Instant::now();
        'conn_loop: loop {
            let message = match rx.try_recv() {
                Ok(message) => message,
                Err(e) => {
                    tracing::error!(?e, "websocket error");
                    break 'conn_loop;
                }
            };
            let (event, context) = translate_to_ctx(message.into(), client.clone()).await;
            client.read().unwrap().event_dispatcher.send(event, context);
        }
        if (start.elapsed().as_secs_f32() > 30f32) {
            retry = expo_backoff!();
            continue;
        }
        tokio::time::sleep(retry.next().unwrap()).await;
    }
}