use super::model::*;
pub struct Game {
    white_player: Player,
    black_player: Player,

    white_king: BoardPosition,
    black_king: BoardPosition,
}

pub struct GameConfig {
    white_player_name: String,
    black_player_name: String,
    ms_per_player: u64,
}

impl Game {
    pub fn new(
        GameConfig {
            white_player_name,
            black_player_name,
            ms_per_player,
        }: GameConfig,
    ) -> Self {
        let white_player = Player::new(white_player_name, PieceColors::White, ms_per_player);
        let black_player = Player::new(black_player_name, PieceColors::Black, ms_per_player);
        let white_king = BoardPosition::try_from("e1").unwrap();
        let black_king = BoardPosition::try_from("e8").unwrap();

        Game {
            white_player,
            black_player,
            white_king,
            black_king,
        }
    }
}
