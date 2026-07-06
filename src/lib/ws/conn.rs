use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::lib::ws::event::WebsocketEvent;
use thiserror::Error;
use serde_json;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

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
            let event: WebsocketEvent = serde_json::from_str(message).unwrap();
            tx.send(event).unwrap();
        }
    });
    Ok(rx)
}