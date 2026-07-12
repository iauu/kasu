use std::env::Args;
use slack_morphism::{SlackChannelId, SlackTs, SlackUserId};
use crate::lib::cmd::parse::CmdParse;
use crate::lib::ctx_trait::{Metadata, ToMetadata};
use crate::lib::event::{Event, FromEvent};

#[derive(Clone, Debug)]
pub struct CmdEvent {
    pub command: String,
    pub arg_raw: String,
    pub user_id: SlackUserId,
    pub channel_id: SlackChannelId,
    pub thread_ts: Option<SlackTs>,
    pub message_ts: SlackTs
}

impl ToMetadata for CmdEvent {
    fn get_metadata(&self) -> Metadata {
        Metadata {
            user_id: Some(self.user_id.clone()),
            channel_id: Some(self.channel_id.clone()),
            thread_ts: self.thread_ts.clone(),
            message_ts: Some(self.message_ts.clone())
        }
    }
}

#[derive(Clone, Debug)]
pub struct CmdParsedEvent<Args> {
    pub command: String,
    pub arg_raw: String,
    pub user_id: SlackUserId,
    pub channel_id: SlackChannelId,
    pub thread_ts: Option<SlackTs>,
    pub message_ts: SlackTs,
    pub arg: Args
}

impl<Args> ToMetadata for CmdParsedEvent<Args> {
    fn get_metadata(&self) -> Metadata {
        Metadata {
            user_id: Some(self.user_id.clone()),
            channel_id: Some(self.channel_id.clone()),
            thread_ts: self.thread_ts.clone(),
            message_ts: Some(self.message_ts.clone())
        }
    }
}

impl FromEvent for CmdEvent {
    fn from_event(event: crate::lib::event::Event) -> Option<Self> {
        match event {
            crate::lib::event::Event::Cmd(event) => Some(event),
            _ => None
        }
    }
}

pub trait GetCmd {
    fn get_cmd(&self) -> String;
}

impl<Args> GetCmd for CmdParsedEvent<Args> {
    fn get_cmd(&self) -> String {
        self.command.clone()
    }
}

pub trait FromEventCmd: Send + Sync + FromEvent + GetCmd + 'static {}
