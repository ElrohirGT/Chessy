use std::sync::Arc;

use chess_engine::PieceColors;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Player {
    id: Uuid,
    name: Arc<str>,
    color: PieceColors,
    ms_remaining: u64,
}

impl Player {
    pub fn new(id: Uuid, name: Arc<str>, color: PieceColors, ms_remaining: u64) -> Self {
        Player {
            id,
            name,
            color,
            ms_remaining,
        }
    }

    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}
