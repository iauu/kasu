use tracing::Level;
use crate::env::Env;
use serde_env::from_env;
use crate::lib::client::Client;

mod lib;
mod env;

#[tokio::main]
async fn main() {
    let env : Env = from_env().expect("deserialize from env");

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_target(true) // Shows the module path/logger name
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE) // Tracks span life
        .init();
    
    let client: Client = Client::new(env.xoxc, env.xoxd);
    
    client.run().await;
}
