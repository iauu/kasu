use tracing::instrument;
use crate::lib::client::{Client, PartialClient};
use crate::lib::cmd::CmdEvent;
use crate::lib::context::{translate_to_ctx, AsyncSafe};
use crate::lib::ctx_trait::ToThreadTs;
use crate::lib::event::Event;
use crate::lib::ws::event::{WebsocketMessageReceivedEvent, WebsocketReconnectUrlEvent};

#[instrument(level = "info", skip(client), fields(module = module_path!()), target = "cmd_handler")]
pub async fn cmd_handler<T: AsyncSafe>(event: WebsocketMessageReceivedEvent, client: Client<T>) {
    if let Some(text) = event.text {
        let command;
        let arg_raw;
        if let Some(index) = text.find(' ') {
            command = text[..index].to_string();
            arg_raw = text[index..].to_string();
        } else {
            command = text;
            arg_raw = String::from(" ");
        }
        let event = CmdEvent {
            command,
            arg_raw,
            user_id: event.user_id,

            channel_id: event.channel_id,
            thread_ts: event.thread_ts,
            message_ts: event.ts,
        };
        tracing::info!("Dispatching {:?}", event);
        let (event, context) = translate_to_ctx(Event::Cmd(event), client.clone()).await;
        client.read().await.event_dispatcher.send(event, context);
    }
}