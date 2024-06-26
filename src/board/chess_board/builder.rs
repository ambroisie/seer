use crate::board::{Bitboard, CastleRights, ChessBoard, Color, Piece, Square, ValidationError};

/// Build a [ChessBoard] one piece at a time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessBoardBuilder {
    /// The list of [Piece] on the board. Indexed by [Square::index].
    pieces: [Option<(Piece, Color)>; Square::NUM_VARIANTS],
    // Same fields as [ChessBoard].
    castle_rights: [CastleRights; Color::NUM_VARIANTS],
    en_passant: Option<Square>,
    half_move_clock: u32,
    side: Color,
    // 1-based, a turn is *two* half-moves (i.e: both players have played).
    turn_count: u32,
}

impl ChessBoardBuilder {
    pub fn new() -> Self {
        Self {
            pieces: [None; Square::NUM_VARIANTS],
            castle_rights: [CastleRights::NoSide; Color::NUM_VARIANTS],
            en_passant: Default::default(),
            half_move_clock: Default::default(),
            side: Color::White,
            turn_count: 1,
        }
    }

    pub fn with_castle_rights(&mut self, rights: CastleRights, color: Color) -> &mut Self {
        self.castle_rights[color.index()] = rights;
        self
    }

    pub fn with_en_passant(&mut self, square: Square) -> &mut Self {
        self.en_passant = Some(square);
        self
    }

    pub fn without_en_passant(&mut self) -> &mut Self {
        self.en_passant = None;
        self
    }

    pub fn with_half_move_clock(&mut self, clock: u32) -> &mut Self {
        self.half_move_clock = clock;
        self
    }

    pub fn with_turn_count(&mut self, count: u32) -> &mut Self {
        self.turn_count = count;
        self
    }

    pub fn with_current_player(&mut self, color: Color) -> &mut Self {
        self.side = color;
        self
    }
}

impl Default for ChessBoardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Index a [ChessBoardBuilder] with a [Square] to access its pieces.
impl std::ops::Index<Square> for ChessBoardBuilder {
    type Output = Option<(Piece, Color)>;

    fn index(&self, square: Square) -> &Self::Output {
        &self.pieces[square.index()]
    }
}

/// Index a [ChessBoardBuilder] with a [Square] to access its pieces.
impl std::ops::IndexMut<Square> for ChessBoardBuilder {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.pieces[square.index()]
    }
}

impl TryFrom<ChessBoardBuilder> for ChessBoard {
    type Error = ValidationError;

    fn try_from(builder: ChessBoardBuilder) -> Result<Self, Self::Error> {
        let mut piece_occupancy: [Bitboard; Piece::NUM_VARIANTS] = Default::default();
        let mut color_occupancy: [Bitboard; Color::NUM_VARIANTS] = Default::default();
        let mut combined_occupancy: Bitboard = Default::default();
        let ChessBoardBuilder {
            pieces,
            castle_rights,
            en_passant,
            half_move_clock,
            side,
            turn_count,
        } = builder;

        for square in Square::iter() {
            let Some((piece, color)) = pieces[square.index()] else {
                continue;
            };
            piece_occupancy[piece.index()] |= square;
            color_occupancy[color.index()] |= square;
            combined_occupancy |= square;
        }

        let total_plies = (turn_count - 1) * 2 + if side == Color::White { 0 } else { 1 };

        let board = ChessBoard {
            piece_occupancy,
            color_occupancy,
            combined_occupancy,
            castle_rights,
            en_passant,
            half_move_clock,
            total_plies,
            side,
        };

        board.validate()?;
        Ok(board)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn from_board(board: &ChessBoard) -> ChessBoardBuilder {
        let mut builder = ChessBoardBuilder::new();

        for piece in Piece::iter() {
            for color in Color::iter() {
                for square in board.occupancy(piece, color) {
                    builder[square] = Some((piece, color));
                }
            }
        }

        for color in Color::iter() {
            builder.with_castle_rights(board.castle_rights(color), color);
        }

        if let Some(square) = board.en_passant() {
            builder.with_en_passant(square);
        } else {
            builder.without_en_passant();
        }

        builder
            .with_half_move_clock(board.half_move_clock())
            .with_turn_count(board.total_plies() / 2 + 1)
            .with_current_player(board.current_player());

        builder
    }

    #[test]
    fn default_board() {
        let board = ChessBoard::default();
        let builder = from_board(&board);
        assert_eq!(board, builder.try_into().unwrap())
    }
}
