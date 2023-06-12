use super::*;

/// Get's the initial board of the game.
pub fn get_starting_board() -> Board {
    let (white_pieces, black_pieces, cells) = generate_starting_chess_cells();
    let white_king_position = "e1".try_into().unwrap();
    let black_king_position = "e8".try_into().unwrap();
    Board::new(
        cells,
        white_pieces,
        black_pieces,
        white_king_position,
        black_king_position,
        None,
    )
}

fn generate_starting_chess_cells() -> (Vec<ChessPiece>, Vec<ChessPiece>, Vec<Vec<ChessCell>>) {
    let white_pawns: Vec<ChessCell> = ('a'..='h')
        .map(|c| format!("{}2", c))
        .map(|c| c.as_str().try_into().unwrap())
        .map(|position| ChessPiece::new(PieceTypes::Pawn, position, PieceColors::White))
        .map(ChessCell::some)
        .collect();
    let black_pawns: Vec<ChessCell> = ('a'..='h')
        .map(|c| format!("{}7", c))
        .map(|c| c.as_str().try_into().unwrap())
        .map(|position| ChessPiece::new(PieceTypes::Pawn, position, PieceColors::Black))
        .map(ChessCell::some)
        .collect();
    let mut white_space: Vec<Vec<ChessCell>> = (3..=6)
        .map(|_| {
            ('a'..='h')
                .map(|_| ChessCell::none())
                .collect::<Vec<ChessCell>>()
        })
        .collect();

    let mut white_back_rank: Vec<ChessCell> = ('a'..='h')
        .map(|c| {
            let position: BoardPosition = format!("{}1", c).as_str().try_into().unwrap();
            match c {
                'a' => (position, PieceTypes::Rook),
                'b' => (position, PieceTypes::Knight),
                'c' => (position, PieceTypes::Bishop),
                'd' => (position, PieceTypes::Queen),
                'e' => (position, PieceTypes::King),
                'f' => (position, PieceTypes::Bishop),
                'g' => (position, PieceTypes::Knight),
                'h' => (position, PieceTypes::Rook),
                _ => unreachable!(),
            }
        })
        .map(|(pos, kind)| ChessPiece::new(kind, pos, PieceColors::White))
        .map(ChessCell::some)
        .collect();

    let mut black_back_rank: Vec<ChessCell> = ('a'..='h')
        .map(|c| {
            let position: BoardPosition = format!("{}1", c).as_str().try_into().unwrap();
            match c {
                'a' => (position, PieceTypes::Rook),
                'b' => (position, PieceTypes::Knight),
                'c' => (position, PieceTypes::Bishop),
                'd' => (position, PieceTypes::Queen),
                'e' => (position, PieceTypes::King),
                'f' => (position, PieceTypes::Bishop),
                'g' => (position, PieceTypes::Knight),
                'h' => (position, PieceTypes::Rook),
                _ => unreachable!(),
            }
        })
        .map(|(pos, kind)| ChessPiece::new(kind, pos, PieceColors::White))
        .map(ChessCell::some)
        .collect();

    white_back_rank.append(&mut white_pawns.clone());
    black_back_rank.append(&mut black_pawns.clone());

    let white_pieces = white_back_rank
        .clone()
        .into_iter()
        .map(|cell| cell.0.unwrap())
        .collect();
    let black_pieces = black_back_rank
        .clone()
        .into_iter()
        .map(|cell| cell.0.unwrap())
        .collect();
    (
        white_pieces,
        black_pieces,
        vec![
            white_back_rank,
            white_pawns,
            white_space.pop().unwrap(),
            white_space.pop().unwrap(),
            white_space.pop().unwrap(),
            white_space.pop().unwrap(),
            black_pawns,
            black_back_rank,
        ],
    )
}
