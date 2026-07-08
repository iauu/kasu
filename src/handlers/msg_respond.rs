use tracing::instrument;
use crate::lib::api::MessageData;
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::lib::ctx_item::Messageable;
use crate::lib::ctx_trait::{Sendable, ThreadSendable};

#[instrument(level = "info", fields(module = module_path!()), target = "msg_respond")]
pub(crate) async fn msg_respond(event: WebsocketMessageReceivedEvent, messagable: Messageable) -> () {
    let text = event.text.unwrap_or("N/A".to_string());
    tracing::info!("Received text: {text}");
    if text.to_lowercase().contains("hi kasu") {
        match  messagable.reply_in_thread(MessageData::Raw("Hello!".to_string())).await {
            Ok(ts) => {
                tracing::info!("Send successful: {}", ts);
            }
            Err(e) => {
                tracing::error!("Failed to send message: {}", e);
            }
        }

    }
}