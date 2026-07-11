use std::sync::Arc;
use async_lock::RwLock;
use crate::lib::context;

#[derive(Copy, Clone, Debug)]
pub enum Profile {
    Shy,
    Katie
}

impl Profile {
    fn as_path(&self) -> &'static str {
        match self {
            Profile::Shy => "assets/kasu_shy.png",
            Profile::Katie => "assets/kasu_katie.png"
        }
    }
}

#[derive(Debug)]
pub struct StateInternal {
    pub last_message: std::time::Instant,
    pub current_pfp: Profile
}

pub type State = Arc<RwLock<StateInternal>>;

impl context::StateUnwrapped for StateInternal {}

impl Default for StateInternal {
    fn default() -> Self {
        Self {
            last_message: std::time::Instant::now(),
            current_pfp: Profile::Katie
        }
    }
}
