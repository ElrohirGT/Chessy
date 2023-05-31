mod model;
use self::model::*;

pub struct Game {
    pieces: Vec<ChessPiece>,
    board: Board,
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

pub fn generate_starting_board() -> Board {
    let cells = (0..8)
        .map(|i| match i {
            0 | 1 => (0..8).map(|_| ChessCell::WhitePiece).collect(),
            6 | 7 => (0..8).map(|_| ChessCell::BlackPiece).collect(),
            _ => (0..8).map(|_| ChessCell::Empty).collect(),
        })
        .collect();
    Board::new(cells)
}

pub fn generate_starting_pieces() -> Vec<ChessPiece> {
    let pawns = vec![PieceColors::White, PieceColors::Black]
        .into_iter()
        .flat_map(|color| {
            let rank = match color {
                PieceColors::White => "2",
                PieceColors::Black => "7",
            };
            ('a'..='h')
                .map(|i| {
                    let pos: &str = &format!("{}{}", i, rank);
                    let position = pos.try_into().unwrap();
                    ChessPiece::new(PieceTypes::Pawn, position, color.clone())
                })
                .collect::<Vec<ChessPiece>>()
        })
        .collect();
    //TODO Finish implementing
    pawns
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
        let board = generate_starting_board();
        let pieces = generate_starting_pieces();

        Game {
            white_player,
            black_player,
            pieces,
            board,
            white_king,
            black_king,
        }
    }
}
