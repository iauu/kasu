# kasu

### How to setup
Create `.env` with the following content
```env
XOXC=
XOXD=
HOST=hackclub.enterprise.slack.com 
```
(The host is the domain for sending API request to your the slack server. This might be different depends on your workspace, or in the case of enterprise, the enterprise the workspace is in)

`XOXC` and `XOXD` token can be obtained from inspecting an API request (such as posting message)

the XOXD token located at cookie at `d=` section. Please make sure the token is URL-encoded (do ot manually decode it)

the XOXC token located at the payload for the multipart form data. You can find this from a `chat.postMessage` when it is posting to a channel (not thread), which the xoxc token is `token` in the myltipart form data payload.

### What does this do

Mostly right now, this lay down the framework for the communication and websocket to be extended.

What if do right now is it automatically reply in thread `Hello!` when you send `hi kasu` in a channel the bot is in

### How to make new interaction

You should create a handler in `./src/handlers` as a new module

Example:

```rust
use tracing::instrument;
use crate::lib::api::MessageData;
use crate::lib::ws::event::WebsocketMessageReceivedEvent;
use crate::lib::ctx_item::Messageable;
use crate::lib::ctx_trait::{Sendable, ThreadSendable};

#[instrument(level = "info", fields(module = module_path!()), target = "msg_respond")]
pub(crate) async fn msg_respond(event: WebsocketMessageReceivedEvent, messagable: Messageable) -> () {
    let text = event.text.unwrap_or("N/A".to_string());
    tracing::info!("Received text: {text}");
    if text.to_lowercase().contains("hi kasu") {
        match  messagable.reply_in_thread(MessageData::Raw("Hello!".to_string())).await {
            Ok(ts) => {
                tracing::info!("Send successful: {}", ts);
            }
            Err(e) => {
                tracing::error!("Failed to send message: {}", e);
            }
        }

    }
}
```

This would only receive message received event from websocket, and only when the channel is messageable (defined by the channel_id and the message_ts presence in the event such that it cana reply correctly)

Then you should spawn the handler in `main`

Example:

```rust
    spawn_handler(&client.read().await.event_dispatcher, handlers::msg_respond::msg_respond);
```

This would allow the handler to listen to income event and enact on it.

### TODO

API List:
- [ ] Slash command invoke
- [ ] Blockkit interaction
- [x] Channel Send Restriction
- [ ] Channel Restriction
- [ ] group Restriction
- [x] Channel manager read

Websocket:
- [ ] User channel join
- [ ] User group join
- [ ] Message subtype