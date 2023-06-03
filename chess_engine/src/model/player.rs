use super::PieceColors;

#[derive(Debug)]
pub struct Player {
    name: String,
    color: PieceColors,
    ms_remaining: u64,
}

impl Player {
    pub fn new(name: String, color: PieceColors, ms_remaining: u64) -> Self {
        Player {
            name,
            color,
            ms_remaining,
        }
    }
}
