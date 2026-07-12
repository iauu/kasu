use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use slack_morphism::{SlackAppId, SlackBotId, SlackChannelId, SlackChannelType, SlackClientMessageId, SlackEnterpriseId, SlackTeamId, SlackTs, SlackUserId};
use crate::impl_metadata_propagate;
use crate::lib::event::{Event, FromEvent};
use crate::lib::blocks::SlackBlock;
use crate::lib::ctx_trait::{Metadata, ToChannelId, ToMetadata};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketEvent {
    #[serde(rename="user_typing")]
    Typing(WebsocketUserTypingEvent),
    #[serde(rename="reconnect_url")]
    ReconnectUrl(WebsocketReconnectUrlEvent),
    #[serde(rename="message")]
    Message(WebsocketMessageEvent),
    #[serde(rename = "emoji_changed")]
    Emoji(WebsocketEmojiChangedEvent),
    #[serde(rename = "member_joined_channel")]
    ChannelMemberJoin(WebsocketChannelMemberJoinEvent),
    #[serde(rename = "member_left_channel")]
    ChannelMemberLeft(WebsocketChannelMemberLeaveEvent)
}

impl FromEvent for WebsocketEvent {
    fn from_event(event: Event) -> Option<Self> {
        match event {
            Event::Websocket(event) => Some(event),
            _ => None
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct  WebsocketUserTypingEvent {
    #[serde(rename="channel")]
    pub channel_id: SlackChannelId,
    #[serde(default)]
    pub thread_ts: Option<SlackTs>,
    #[serde(default)]
    pub id: Option<usize>,
    #[serde(rename="user")]
    pub user_id: SlackUserId
}

macro_rules! from_event_impl {
    ($ori:path, $name:ty, $event:ident) => {
        impl $crate::lib::event::FromEvent for $name {
            fn from_event(event: $crate::lib::event::Event) -> Option<Self>
            where
                Self: Sized,
            {
                use $ori as C;
                let ws: C = C::from_event(event)?;
                match ws {
                    C::$event(event) => Some(event),
                    _ => None
                }
            }
        }
    };
}

macro_rules! ws_from_event_impl {
    ($name:ty, $event:ident) => {
        from_event_impl!($crate::lib::ws::event::WebsocketEvent, $name, $event);
    };
}

macro_rules! ws_message_from_event_impl {
    ($name:ty, $event:ident) => {
        from_event_impl!($crate::lib::ws::event::WebsocketMessageEvent, $name, $event);
    };
}




#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketReconnectUrlEvent {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum WebsocketMessageEvent {
    Incoming(WebsocketMessageReceivedEvent),
    // SubTyped
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketMessageReceivedEvent {
    #[serde(flatten)]
    pub bot: Option<WebsocketMessageReceivedBotMetadata>,
    #[serde(rename="channel")]
    pub channel_id: SlackChannelId,
    pub text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    #[serde(rename="user")]
    pub user_id: SlackUserId,
    pub client_msg_id: Option<SlackClientMessageId>,
    #[serde(rename="team")]
    pub team_id: SlackTeamId,
    #[serde(rename="source_team")]
    pub source_team_id: SlackTeamId,
    #[serde(rename="user_team")]
    pub user_team_id: SlackTeamId,
    #[serde(default)]
    pub suppress_notification: bool,
    pub event_ts: String,
    pub ts: SlackTs,
    pub thread_ts: Option<SlackTs>,
    #[serde(default)]
    pub is_ephemeral: bool
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "subtype", rename_all = "snake_case")]
pub enum WebsocketEmojiChangedEvent {
    Add {
        name: String,
        value: String,
        event_ts: SlackTs
    },
    Remove {
        names: Vec<String>,
        event_ts: SlackTs
    },
    Rename {
        old_name: String,
        new_name: String,
        value: String,
        event_ts: SlackTs
    }
}


// #[derive(Clone, Debug, Deserialize)]
// pub struct WebsocketMessageSubtypedEvent {
//     #[serde(rename="channel")]
//     pub channel_id: SlackChannelId,
//     pub text: Option<String>,
//     pub blocks: Option<Vec<SlackBlock>>,
//     #[serde(rename="user")]
//     pub user_id: SlackUserId,
//     pub client_msg_id: Option<SlackClientMessageId>,
//     #[serde(rename="team")]
//     pub team_id: SlackTeamId,
//     #[serde(rename="source_team")]
//     pub source_team_id: SlackTeamId,
//     #[serde(rename="user_team")]
//     pub user_team_id: SlackTeamId,
//     #[serde(default)]
//     pub suppress_notification: bool,
//     event_ts: String,
//     ts: SlackTs
// }

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketMessageReceivedBotMetadata {
    pub subtype: BotMessageTag,
    pub bot_id: SlackBotId,
    pub bot_profile: BotProfile
}

#[derive(Clone, Debug, Deserialize)]
pub struct BotProfile {
    pub id: SlackBotId,
    pub deleted: bool,
    pub name: String,
    pub updated: i64,
    pub app_id: SlackAppId,
    pub team_id: SlackTeamId,
    pub icons: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BotMessageTag {
    BotMessage,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketChannelMemberJoinEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    pub channel_type: SlackChannelType,
    pub team: SlackTeamId,
    pub inviter: Option<SlackUserId>,
    #[serde(default)]
    pub enterprise: Option<SlackEnterpriseId>,
    pub event_ts: SlackTs,
    pub ts: SlackTs
}

impl ToMetadata for WebsocketChannelMemberJoinEvent {
    fn get_metadata(&self) -> Metadata {
        Metadata {
            channel_id: Some(self.channel.clone()),
            user_id: Some(self.user.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketChannelMemberLeaveEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    pub channel_type: SlackChannelType,
    pub team: SlackTeamId,
    pub event_ts: SlackTs,
    pub ts: SlackTs
}

impl ToMetadata for WebsocketChannelMemberLeaveEvent {
    fn get_metadata(&self) -> Metadata {
        Metadata {
            channel_id: Some(self.channel.clone()),
            user_id: Some(self.user.clone()),
            ..Default::default()
        }
    }
}

ws_from_event_impl!(WebsocketMessageEvent, Message);
ws_from_event_impl!(WebsocketUserTypingEvent, Typing);
ws_from_event_impl!(WebsocketReconnectUrlEvent, ReconnectUrl);
ws_from_event_impl!(WebsocketEmojiChangedEvent, Emoji);
ws_from_event_impl!(WebsocketChannelMemberJoinEvent, ChannelMemberJoin);
ws_from_event_impl!(WebsocketChannelMemberLeaveEvent, ChannelMemberLeft);
ws_message_from_event_impl!(WebsocketMessageReceivedEvent, Incoming);

impl ToMetadata for WebsocketMessageReceivedEvent {
    fn get_metadata(&self) -> Metadata {
        Metadata {
            channel_id: Some(self.channel_id.clone()),
            thread_ts: self.thread_ts.clone(),
            message_ts: Some(self.ts.clone()),
            user_id: Some(self.user_id.clone()),
            ..Default::default()
        }
    }
}

impl ToMetadata for WebsocketUserTypingEvent {
    fn get_metadata(&self) -> Metadata {
        Metadata {
            channel_id: Some(self.channel_id.clone()),
            thread_ts: self.thread_ts.clone(),
            user_id: Some(self.user_id.clone()),
            ..Default::default()
        }
    }
}

impl ToMetadata for WebsocketReconnectUrlEvent {}
impl ToMetadata for WebsocketEmojiChangedEvent {}

impl_metadata_propagate!(WebsocketMessageEvent, Incoming);
impl_metadata_propagate!(WebsocketEvent, Typing Message ReconnectUrl Emoji ChannelMemberJoin ChannelMemberLeft);

impl Into<Event> for WebsocketEvent {
    fn into(self) -> Event {
        Event::Websocket(self)
    }
}