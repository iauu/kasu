use tracing::instrument;
use crate::lib::ws::event::WebsocketMessageReceivedEvent;

#[instrument(level = "info", fields(module = module_path!()), target = "test_msg_listen")]
pub(crate) async fn test_msg_listen(event: WebsocketMessageReceivedEvent) -> () {
    let text = event.text.unwrap_or("N/A".to_string());
    tracing::info!("Message received: {} from {} in {}", text, event.user_id, event.channel_id);
}