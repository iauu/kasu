use tracing::Level;
use crate::env::Env;
use serde_env::from_env;
use tracing::field::{Field, Visit};
use crate::lib::client::Client;
use urlencoding;

mod lib;
mod env;
mod handlers;
mod state;
mod tasks;

use std::io;
use std::ptr::replace;
use std::str::FromStr;
use std::sync::Arc;
use async_lock::RwLock;
use cfg_if::cfg_if;
use sqlx::sqlite::SqliteConnectOptions;
use tracing_subscriber::{EnvFilter, prelude::*};
use crate::lib::handler::spawn_handler;
use crate::state::{BotState, BotStateInternal};
use crate::tasks::pfp_task::pfp_task;

struct RedactingWriter<W> {
    inner: W,
    sub_token_c: Option<String>,
    token_c: String,
    token_d: String,
    decoded_token_d: String
}

impl<W: io::Write> io::Write for RedactingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let log_str = String::from_utf8_lossy(buf);

        let mut redacted = log_str
            .replace(&self.token_c, "[REDACTED_XOXC]")
            .replace(&self.token_d, "[REDACTED_XOXD]")
            .replace(&self.decoded_token_d, "[REDACTED_XOXD]");

        if let Some(x) = self.sub_token_c.clone() {
            redacted = redacted.replace(&x, "[REDACTED_SUB_XOXC]");
        }

        self.inner.write_all(redacted.as_bytes())?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
struct RedactingMakeWriter {
    sub_token_c: Option<String>,
    token_c: String,
    token_d: String,
    decoded_token_d: String
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for RedactingMakeWriter {
    type Writer = RedactingWriter<io::Stdout>;

    fn make_writer(&'a self) -> Self::Writer {
        RedactingWriter {
            inner: io::stdout(),
            sub_token_c: self.sub_token_c.clone(),
            token_c: self.token_c.clone(),
            token_d: self.token_d.clone(),
            decoded_token_d: self.decoded_token_d.clone()
        }
    }
}

#[tokio::main]
async fn main() {
    let env : Env = from_env().expect("deserialize from env");

    let lookup_writer = RedactingMakeWriter {
        sub_token_c: env.sub_xoxc.clone(),
        token_c: env.xoxc.clone(),
        token_d: env.xoxd.clone(),
        decoded_token_d: urlencoding::decode(&env.xoxd).unwrap().parse().unwrap()
    };

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(true) // Shows the module path/logger name
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE) // Tracks span life
        .with_writer(lookup_writer)
        .init();

    let options = SqliteConnectOptions::from_str("sqlite://data/data.db")
        .unwrap()
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(options).await.unwrap();
    
    let state = Arc::new(RwLock::new(BotStateInternal::init(pool)));

    let client: Client<BotState> = Client::new_with_state(env.sub_xoxc, env.xoxc, env.xoxd, env.host, env.team_id, state.clone(), env.user_id);

    spawn_handler(&client.read().await.event_dispatcher, handlers::test_msg_listen::test_msg_listen);
    spawn_handler(&client.read().await.event_dispatcher, handlers::msg_respond::msg_respond);
    spawn_handler(&client.read().await.event_dispatcher, ("!get_user_id", handlers::get_user_id::get_user_id));
    spawn_handler(&client.read().await.event_dispatcher, ("k!init", handlers::init::init_channel));

    cfg_if! {
        if #[cfg(feature = "photo")] {
            spawn_handler(&client.read().await.event_dispatcher, handlers::bot_msg_send::bot_msg_send);
            let partial_client = client.get_partial();
            tokio::task::spawn(async move {
                pfp_task(partial_client, state).await
            });
            let _ = client.get_partial().read().await.api_client.set_profile("assets/kasu_katie.png").await;
        }
    }


    client.run().await;
}
