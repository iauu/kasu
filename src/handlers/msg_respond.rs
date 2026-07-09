use slack_morphism::SlackTextFormat;
use tracing::instrument;
use crate::lib::api::MessageData;
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::lib::ctx_item::Messageable;
use crate::lib::ctx_trait::{Sendable, ThreadSendable};

macro_rules! contains {
    ($t:expr, $($m:expr)+) => {
        {
            let s_ref: &str = $t.as_ref();
            true $(&& s_ref.contains($m))*
        }
    };
}

#[instrument(level = "info", fields(module = module_path!()), target = "msg_respond")]
pub(crate) async fn msg_respond(event: WebsocketMessageReceivedEvent, messagable: Messageable) -> () {
    let text = event.text.unwrap_or("N/A".to_string());
    tracing::info!("Received text: {text}");
    let lower = text.to_lowercase();
    if contains!(lower, "hi kasu") {
        match  messagable.reply_in_thread(MessageData::Raw("Hello!".to_string())).await {
            Ok(ts) => {
                tracing::info!("Send successful: {}", ts);
            }
            Err(e) => {
                tracing::error!("Failed to send message: {}", e);
            }
        }
    }
    if contains!(lower, "kasu" "channel manager") {
        let channel = messagable.channel_id.clone();
        let msg = match messagable.client.read().await.api_client.get_channel_manager(channel).await {
            Ok(users) => {
                messagable.reply_in_thread(MessageData::Raw(users.iter().map(|x| x.to_slack_format()).collect::<Vec<String>>().join(", "))).await
            },
            Err(e) => {
                tracing::error!("Failed to get channel manager: {}", e);
                messagable.reply_in_thread(MessageData::Raw("Failed to get channel manager".to_string())).await
            }
        };
        match msg {
            Ok(ts) => {
                tracing::info!("Send successful: {}", ts);
            }
            Err(e) => {
                tracing::error!("Failed to send message: {}", e);
            }
        }
    }
}