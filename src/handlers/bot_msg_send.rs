use tracing::instrument;
use crate::fail_ignore_handle;
use crate::lib::api::error::Error;
use crate::lib::client::PartialClient;
use crate::lib::ctx_item::{Messageable, PartialUser};
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::lib::context::State;
use crate::state::{BotState, Profile};

#[instrument(level = "info", fields(module = module_path!()), target = "bot_msg_send")]
pub(crate) async fn bot_msg_send(event: WebsocketMessageReceivedEvent, user: Option<PartialUser>, State::State(state): State<BotState>, partial_client: PartialClient) -> Result<(), Error> {
    if event.user_id.0 != partial_client.read().await.user_id.0 {
        return Ok(());
    }
    state.write().await.last_message = std::time::Instant::now();
    let curr = state.read().await.current_pfp;
    if let Profile::Shy = curr {
        state.write().await.current_pfp = Profile::Katie;
        fail_ignore_handle!(partial_client.read().await.api_client.set_profile(Profile::Katie.as_path()).await, "Set pfp successful: {res:?}", "Set pfp failed: {e}");
    }
    Ok(())
}