use slack_morphism::SlackChannelId;
use sqlx::Row;
use tracing::instrument;
use crate::lib::api::{ChannelRestriction, MessageData, SendRestriction};
use crate::lib::callback::{Middleware, NoAction};
use crate::lib::callback_impl::reply_in_thread::ReplyInThread;
use crate::lib::client::PartialClient;
use crate::lib::cmd::event::CmdParsedEvent;
use crate::lib::context::State;
use crate::lib::ctx_item::{Messageable, PartialChannel, PartialUser};
use crate::lib::ctx_trait::{ThreadSendable, ToChannelId};
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::state::BotState;

#[instrument(level = "info", fields(module = module_path!()), target = "init_channel")]
pub(crate) async fn init_channel(
    event: CmdParsedEvent<(Option<SlackChannelId>,)>,
    messageable: Messageable,
    user: Option<PartialUser>,
    State::State(state): State<BotState>,
    partial_client: PartialClient
) -> Middleware<String, ReplyInThread, BotState> {
    let pool = state.read().await.db.clone();
    let channel = event.channel_id.get_channel_id().unwrap_or(messageable.channel_id.clone());

    let user = match user {
        Some(u) => u,
        None => {
            return "Unable to identify sender".to_string().into();
        }
    };

    let query_check_existing_config = sqlx::query("SELECT COUNT(*) FROM channel_managed WHERE channel_id = ?")
        .bind(channel.0.clone())
        .fetch_one(&pool).await.unwrap();

    let existing_config: u32 = query_check_existing_config.get(0);
    if existing_config > 0 {
        return "This channel have already been registered".to_string().into()
    }

    let channel_managers = partial_client.read().await.api_client.get_channel_manager(channel.clone()).await;

    let channel_managers = match channel_managers {
        Ok(x) => x,
        Err(e) => {
            tracing::error!("Failed to get channel managers: {}", e);
            return "Unable to identify channel manager".to_string().into()
        }
    };

    if !channel_managers.contains(&user.user_id) {
        return "Request failed: you are not a channel manager".to_string().into()
    }

    if !channel_managers.contains(&partial_client.read().await.user_id) {
        return format!("Add me <@{}> as a channel manager and rerun the command!", partial_client.read().await.user_id.0).into()
    }

    let _ = sqlx::query("INSERT INTO channel_managed (channel_id, config) VALUES (?, 31)")
        .bind(channel.0.clone())
        .execute(&pool).await.unwrap();

    let users = partial_client.read().await.api_client.get_channel_members(channel.clone()).await.unwrap();
    for user in &users {
        let _ = sqlx::query("INSERT INTO accepted (channel_id, user_id) VALUES (?, ?) ON CONFLICT DO NOTHING ")
            .bind(channel.0.clone())
            .bind(user.0.clone())
            .execute(&pool).await.unwrap();
    }

    let _ = partial_client.read().await.api_client.update_channel_permission(channel.clone(), ChannelRestriction {
        restriction: SendRestriction::CertainUser {user: users.clone(), allow_thread: false},
        allow_channel_ping: true,
        allow_here_ping: true,
    }).await;

    format!("Setup completed with {} users added to whitelist", users.len()).into()
}