use std::sync::Arc;

use chess_engine::PieceColors;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    id: Uuid,
    name: Arc<str>,
    color: PieceColors,
    ms_remaining: u128,
}

#[derive(Debug)]
pub enum PlayerTimeChanged {
    InsufficientTime,
    LastMove,
    TimeRemaining,
}

impl Player {
    pub fn new(id: Uuid, name: Arc<str>, color: PieceColors, ms_remaining: u128) -> Self {
        Player {
            id,
            name,
            color,
            ms_remaining,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub(crate) fn reduce_time_by(&mut self, delta_ms: u128) -> PlayerTimeChanged {
        match self.ms_remaining.cmp(&delta_ms) {
            std::cmp::Ordering::Greater => {
                self.ms_remaining -= delta_ms;
                PlayerTimeChanged::TimeRemaining
            }
            ord => {
                self.ms_remaining = 0;
                match ord {
                    std::cmp::Ordering::Less => PlayerTimeChanged::InsufficientTime,
                    std::cmp::Ordering::Equal => PlayerTimeChanged::LastMove,
                    _ => unreachable!(),
                }
            }
        }
    }
}
