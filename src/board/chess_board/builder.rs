use crate::board::{Bitboard, CastleRights, ChessBoard, Color, InvalidError, Piece, Square};

/// Build a [ChessBoard] one piece at a time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessBoardBuilder {
    /// The list of [Piece] on the board. Indexed by [Square::index].
    pieces: [Option<(Piece, Color)>; 64],
    // Same fields as [ChessBoard].
    castle_rights: [CastleRights; Color::NUM_VARIANTS],
    en_passant: Option<Square>,
    half_move_clock: u8,
    total_plies: u32,
    side: Color,
}

impl ChessBoardBuilder {
    pub fn new() -> Self {
        Self {
            pieces: [None; 64],
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: Default::default(),
            half_move_clock: Default::default(),
            total_plies: Default::default(),
            side: Color::White,
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

    pub fn with_half_move_clock(&mut self, clock: u8) -> &mut Self {
        self.half_move_clock = clock;
        self
    }

    pub fn with_total_plies(&mut self, plies: u32) -> &mut Self {
        self.total_plies = plies;
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
    type Error = InvalidError;

    fn try_from(builder: ChessBoardBuilder) -> Result<Self, Self::Error> {
        let mut piece_occupancy: [Bitboard; Piece::NUM_VARIANTS] = Default::default();
        let mut color_occupancy: [Bitboard; Color::NUM_VARIANTS] = Default::default();
        let mut combined_occupancy: Bitboard = Default::default();
        let ChessBoardBuilder {
            pieces,
            castle_rights,
            en_passant,
            half_move_clock,
            total_plies,
            side,
        } = builder;

        for square in Square::iter() {
            let Some((piece, color)) = pieces[square.index()] else {
                continue;
            };
            piece_occupancy[piece.index()] |= square;
            color_occupancy[color.index()] |= square;
            combined_occupancy |= square;
        }

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
            .with_total_plies(board.total_plies())
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
