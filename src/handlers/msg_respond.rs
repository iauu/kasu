use slack_morphism::SlackTextFormat;
use tracing::instrument;
use crate::fail_ignore_handle;
use crate::lib::api::{ChannelRestriction, MessageData, SendRestriction};
use crate::lib::api::model::Preference;
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::lib::ctx_item::{Messageable, PartialChannel, PartialUser};
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
pub(crate) async fn msg_respond(event: WebsocketMessageReceivedEvent, messagable: Messageable, user: Option<PartialUser>) -> () {
    let text = event.text.unwrap_or("N/A".to_string());
    tracing::info!("Received text: {text}");
    let lower = text.to_lowercase();
    if contains!(lower, "hi kasu") {
        fail_ignore_handle!(messagable.reply_in_thread(MessageData::Raw("Hello!".to_string())).await, "Send success: {res}", "Send failed: {e}");
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
        fail_ignore_handle!(msg, "Send success: {res}", "Send failed: {e}");
    }
    if contains!(lower, "kasu" "channel permission") && let Some(user) = user && user.user_id.0 == "U092BGL0UUQ" {
        let channel = messagable.channel_id.clone();
        let rest = ChannelRestriction {
            restriction: SendRestriction::CertainUser {
                user: vec!["U092BGL0UUQ".into(), "U0BGXBNBKNU".into()],
                allow_thread: false,
            },
            ..Default::default()
        };
        let result = messagable.client.read().await.api_client.update_channel_permission(channel.clone(), rest.clone()).await;
        if let Err(e) = &result {
            tracing::error!("Failed to update channel permission: {}, payload: {}", e, serde_json::to_string::<Preference>(&rest.into()).unwrap());
        }
        fail_ignore_handle!(messagable.reply_in_thread(MessageData::Raw(if result.is_ok() {"Success"} else {"Failed"}.to_string())).await, "Send success: {res}", "Send failed: {e}");
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        let result = messagable.client.read().await.api_client.update_channel_permission(channel, ChannelRestriction::default()).await;
        if let Err(e) = &result {
            tracing::error!("Failed to update channel permission: {}", e);
        }
        fail_ignore_handle!(messagable.reply_in_thread(MessageData::Raw(if result.is_ok() {"Success"} else {"Failed"}.to_string())).await, "Send success: {res}", "Send failed: {e}");
    }
}