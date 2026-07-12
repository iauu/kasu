use slack_morphism::SlackUserId;
use tracing::instrument;
use crate::lib::api::MessageData;
use crate::lib::cmd::event::CmdParsedEvent;
use crate::lib::ctx_item::{Messageable, PartialUser};
use crate::lib::ctx_trait::ThreadSendable;
use crate::lib::ws::event::WebsocketMessageReceivedEvent;

#[instrument(level = "info", fields(module = module_path!()), target = "get_user_id")]
pub(crate) async fn get_user_id(event: CmdParsedEvent<(SlackUserId,)>, messagable: Messageable) -> () {
    let _ = messagable.reply_in_thread(MessageData::Raw(format!("Extracted user id: {}", event.arg.0))).await;
}