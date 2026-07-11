use std::time::Duration;
use crate::fail_ignore_handle;
use crate::lib::client::PartialClient;
use crate::state::{BotState, Profile};

pub async fn pfp_task(partial_client: PartialClient, state: BotState) -> ! {
    loop {
        if (state.read().await.current_pfp == Profile::Katie && state.read().await.last_message.elapsed().as_secs() > 300) {
            state.write().await.current_pfp = Profile::Shy;
            fail_ignore_handle!(partial_client.read().await.api_client.set_profile(Profile::Shy.as_path()).await, "Reset pfp failed {e}");
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}