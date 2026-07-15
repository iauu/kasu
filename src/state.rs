use std::sync::Arc;
use async_lock::RwLock;
use sqlx::SqlitePool;
use crate::lib::context;
use crate::lib::context::{AsyncSafe, StateUnwrappedMarker};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum Profile {
    Shy,
    Katie
}

impl Profile {
    pub(crate) fn as_path(&self) -> &'static str {
        match self {
            Profile::Shy => "assets/kasu_shy.png",
            Profile::Katie => "assets/kasu_katie.png"
        }
    }
}

#[derive(Debug)]
pub struct BotStateInternal {
    pub last_message: std::time::Instant,
    pub current_pfp: Profile,
    pub db: SqlitePool
}


pub type BotState = Arc<RwLock<BotStateInternal>>;

impl context::StateUnwrappedMarker for BotStateInternal {}

// impl Default for BotStateInternal {
//     fn default() -> Self {
//         Self {
//             last_message: std::time::Instant::now(),
//             current_pfp: Profile::Katie
//         }
//     }
// }

impl BotStateInternal {
    pub fn init(pool: SqlitePool) -> Self {
        Self {
            last_message: std::time::Instant::now(),
            current_pfp: Profile::Katie,
            db: pool
        }
    }
}
