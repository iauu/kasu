use std::ops::Deref;
use std::sync::Arc;
use async_lock::RwLock;
use sqlx::SqlitePool;
use crate::lib::context;
use crate::lib::context::{AsyncSafe, ReduceState, StateUnwrappedMarker};

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


#[derive(Clone, Debug)]
pub struct BotState(pub Arc<RwLock<BotStateInternal>>);

impl ReduceState<()> for BotState {
    fn reduce(self) -> ()
    where
        Self: Sized,
    {
        
    }
}

impl Deref for BotState {
    type Target = RwLock<BotStateInternal>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}


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
