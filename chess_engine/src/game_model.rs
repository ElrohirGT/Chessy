use crate::model::*;
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

fn generate_starting_state() -> (Board, Vec<ChessPiece>) {
    let mut pieces: Vec<ChessPiece> = vec![];

    let white_back_row: Vec<ChessCell> = ('a'..='h')
        .map(|i| {
            let position: &str = &format!("{}1", i);
            let position = position.try_into().unwrap();
            let owner = PieceColors::White;

            let kind = match i {
                'a' | 'h' => PieceTypes::Rook,
                'b' | 'g' => PieceTypes::Knight,
                'c' | 'f' => PieceTypes::Bishop,
                'd' => PieceTypes::Queen,
                'e' => PieceTypes::King,
                _ => unreachable!(),
            };

            let piece = ChessPiece::new(kind, position, owner);
            pieces.push(piece.clone());
            ChessCell::some(piece)
        })
        .collect();

    let white_pawn_row: Vec<ChessCell> = ('a'..='h')
        .map(|i| {
            let position: &str = &format!("{}2", i);
            let piece = ChessPiece::new(
                PieceTypes::Pawn,
                position.try_into().unwrap(),
                PieceColors::White,
            );

            pieces.push(piece.clone());
            ChessCell::some(piece)
        })
        .collect();
    let mut between_rows: Vec<Vec<ChessCell>> = (3..=6)
        .map(|_| {
            (0..8)
                .map(|_| ChessCell::none())
                .collect::<Vec<ChessCell>>()
        })
        .collect();
    let black_pawn_row: Vec<ChessCell> = ('a'..='h')
        .map(|i| {
            let position: &str = &format!("{}7", i);
            let position: BoardPosition = position.try_into().unwrap();

            let piece = ChessPiece::new(PieceTypes::Pawn, position, PieceColors::White);
            pieces.push(piece.clone());
            ChessCell::some(piece)
        })
        .collect();

    let black_back_row: Vec<ChessCell> = ('a'..='h')
        .map(|i| {
            let position: &str = &format!("{}8", i);
            let position = position.try_into().unwrap();
            let owner = PieceColors::Black;

            let kind = match i {
                'a' | 'h' => PieceTypes::Rook,
                'b' | 'g' => PieceTypes::Knight,
                'c' | 'f' => PieceTypes::Bishop,
                'd' => PieceTypes::Queen,
                'e' => PieceTypes::King,
                _ => unreachable!(),
            };

            let piece = ChessPiece::new(kind, position, owner);
            pieces.push(piece.clone());
            ChessCell::some(piece)
        })
        .collect();

    let cells = vec![
        white_back_row,
        white_pawn_row,
        between_rows.pop().unwrap(),
        between_rows.pop().unwrap(),
        between_rows.pop().unwrap(),
        between_rows.pop().unwrap(),
        between_rows.pop().unwrap(),
        black_pawn_row,
        black_back_row,
    ];

    (Board::new(cells, None), pieces)
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
        let (board, pieces) = generate_starting_state();

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
