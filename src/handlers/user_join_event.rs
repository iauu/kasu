use slack_morphism::{SlackChannelId, SlackUserId};
use sqlx::Row;
use tracing::instrument;
use crate::lib::api::{ChannelRestriction, MessageData, SendRestriction};
use crate::lib::client::PartialClient;
use crate::lib::cmd::event::CmdParsedEvent;
use crate::lib::context::State;
use crate::lib::ctx_item::{Messageable, PartialChannel, PartialUser};
use crate::lib::ctx_trait::Sendable;
use crate::lib::ws::event::WebsocketChannelMemberJoinEvent;
use crate::state::BotState;

#[instrument(level = "info", fields(module = module_path!()), target = "channel_join")]
pub(crate) async fn channel_join(
    event: WebsocketChannelMemberJoinEvent,
    channel: PartialChannel,
    user: PartialUser,
    State::State(state): State<BotState>,
    partial_client: PartialClient
) -> ()  {
    let pool = state.read().await.db.clone();

    let has_enrolled = sqlx::query("SELECT COUNT(*) FROM channel_managed WHERE channel_id = ?")
        .bind(channel.channel_id.0.clone())
        .fetch_one(&pool).await.unwrap();

    let enrolled: u32 = has_enrolled.get(0);
    if enrolled == 0 {
        return;
    }

    let core_info_str = match &event.inviter {
        Some(inviter) => {
            format!("<@{}> which is invited by <@{}>", event.user.0, inviter.0)
        },
        None => {
            format!("<@{}> which is self-invited", event.user.0)
        }
    };

    let channel_managers = partial_client.read().await.api_client.get_channel_manager(channel.channel_id.clone()).await;

    let mut allowed = false;

    let accepted: Vec<String> = sqlx::query(
        "SELECT user_id FROM accepted WHERE channel_id = $1"
    )
        .bind(channel.channel_id.0.clone())
        .fetch_all(&pool)
        .await.unwrap()
        .into_iter()
        .map(|row| row.get::<String, _>("user_id"))
        .collect();

    if accepted.contains(&user.user_id.0) {
        allowed = true;
    } else {
        let channel_managers = match channel_managers {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Failed to get channel managers: {}", e);
                let _ = channel.reply(MessageData::Raw(format!("Hi {core_info_str}, you was kept here as we failed to identify the channel manager... However, you might not be able to talk right now."))).await;
                return;
            }
        };

        match &event.inviter {
            Some(inviter) => {
                if channel_managers.contains(&inviter) {
                    let _ = sqlx::query("INSERT INTO accepted (channel_id, user_id) VALUES (?, ?) ON CONFLICT DO NOTHING ")
                        .bind(channel.channel_id.0.clone())
                        .bind(user.user_id.0.clone())
                        .execute(&pool).await.unwrap();
                    allowed = true;
                } else {
                    let _ = channel.reply(MessageData::Raw(format!("Hi {core_info_str}, access have been blocked as this is a restricted channel."))).await;
                }
            },
            None => {
                let _ = channel.reply(MessageData::Raw(format!("Hi {core_info_str}, access have been blocked as this is a restricted channel."))).await;
            }
        }
    }

    if allowed {

        let accepted: Vec<SlackUserId> = sqlx::query(
            "SELECT user_id FROM accepted WHERE channel_id = $1"
        )
            .bind(channel.channel_id.0.clone())
            .fetch_all(&pool)
            .await.unwrap()
            .into_iter()
            .map(|row| row.get::<String, _>("user_id"))
            .map(|user_id| SlackUserId(user_id))
            .collect();

        let users = partial_client.read().await.api_client.get_channel_members(channel.channel_id.clone()).await.unwrap();

        let intersection: Vec<SlackUserId> = users
            .into_iter()
            .filter(|x| accepted.contains(x))
            .collect();

        let _ = partial_client.read().await.api_client.update_channel_permission(channel.channel_id.clone(), ChannelRestriction {
            restriction: SendRestriction::CertainUser {user: intersection, allow_thread: false},
            allow_channel_ping: true,
            allow_here_ping: true,
        }).await;

        let _ = channel.reply(MessageData::Raw(format!("Hi {core_info_str}, welcome to stay here!"))).await;
    } else {
        let _ = partial_client.read().await.api_client.remove_user(channel.channel_id.clone(), user.user_id).await;
    }
}