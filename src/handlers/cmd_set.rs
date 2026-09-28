use slack_morphism::{SlackChannelId, SlackUserId};
use sqlx::Row;
use tracing::instrument;
use crate::lib::api::{ChannelRestriction, MessageData, SendRestriction};
use crate::lib::callback::Middleware;
use crate::lib::callback_impl::reply_in_thread::ReplyInThread;
use crate::lib::client::PartialClient;
use crate::lib::cmd::event::CmdParsedEvent;
use crate::lib::context::State;
use crate::lib::ctx_item::{Messageable, PartialChannel, PartialUser};
use crate::lib::ctx_trait::{ThreadSendable, ToChannelId};
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::state::BotState;

#[instrument(level = "info", fields(module = module_path!()), target = "set_perm")]
pub(crate) async fn set_perm(
    event: CmdParsedEvent<(String,)>,
    messageable: Messageable,
    sender: Option<PartialUser>,
    State::State(state): State<BotState>,
    partial_client: PartialClient
) -> Middleware<String, ReplyInThread, BotState> {
    let pool = state.read().await.db.clone();
    let channel = event.channel_id.get_channel_id().unwrap_or(messageable.channel_id.clone());

    let sender = match sender {
        Some(u) => u,
        None => {
            return "Unable to identify sender".into()
        }
    };

    let channel_managers = partial_client.read().await.api_client.get_channel_manager(channel.clone()).await;

    let channel_managers = match channel_managers {
        Ok(x) => x,
        Err(e) => {
            return "Unable to identify channel manager".into()
        }
    };

    if !channel_managers.contains(&sender.user_id) {
        return "Request failed: you are not a channel manager".into()
    }
    
    let Ok(perm_v) =  event.arg.0.parse::<u8>() else {
        return "An integer must be input as the argument".into()
    };
    if perm_v >= 32 {
        return "Input an integer between 0 to 31 (inclusive) as a permission value".into()
    }

    let query_check_existing_config = sqlx::query("SELECT COUNT(*) FROM channel_managed WHERE channel_id = ?")
        .bind(channel.0.clone())
        .fetch_one(&pool).await.unwrap();

    let existing_config: u32 = query_check_existing_config.get(0);
    if existing_config == 0 {
        return "This channel have already not been registered. Run `k!init` to register.".into()
    }

    let up_count = sqlx::query("UPDATE main.channel_managed SET config = ? WHERE channel_id = ?")
        .bind(perm_v)
        .bind(channel.0.clone())
        .execute(&pool).await;
    let up_success = match up_count {
        Ok(v) => {
            v.rows_affected() > 0
        },
        Err(e) => false
    };
    if up_success {
        format!("Successfully set permission value to {perm_v}").into()
    } else {
        "Fail to set permission value".into()
    }
    
}