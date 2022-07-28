use super::{Bitboard, CastleRights, Color, Piece, Square};

/// Represent an on-going chess game.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessBoard {
    /// A [Bitboard] of occupancy for each piece type, discarding color. Indexed by [Piece::index].
    piece_occupancy: [Bitboard; Piece::NUM_VARIANTS],
    /// A [Bitboard] of occupancy for each color, discarding piece type. Indexed by [Piece::index].
    color_occupancy: [Bitboard; Color::NUM_VARIANTS],
    /// A [Bitboard] representing all squares currently occupied by a piece.
    combined_occupancy: Bitboard,
    /// The allowed [CastleRights] for either color. Indexed by [Color::index].
    castle_rights: [CastleRights; Color::NUM_VARIANTS],
    /// A potential en-passant attack.
    /// Either `None` if no double-step pawn move was made in the previous half-turn, or
    /// `Some(target_square)` if a double-step move was made.
    en_passant: Option<Square>,
    /// The number of half-turns without either a pawn push or capture.
    half_move_clock: u8, // Should never go higher than 50.
    /// The number of half-turns so far.
    total_plies: u32, // Should be plenty.
    /// The current player turn.
    side: Color,
}

impl ChessBoard {
    /// Which player's turn is it.
    #[inline(always)]
    pub fn current_player(&self) -> Color {
        self.side
    }

    /// Return the [Square] currently occupied by a pawn that can be captured en-passant, or `None`
    #[inline(always)]
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    /// Return the [CastleRights] for the given [Color].
    #[inline(always)]
    pub fn castle_rights(&self, color: Color) -> CastleRights {
        self.castle_rights[color.index()]
    }

    /// Return the [CastleRights] for the given [Color]. Allow mutations.
    #[inline(always)]
    #[allow(unused)]
    fn castle_rights_mut(&mut self, color: Color) -> &mut CastleRights {
        &mut self.castle_rights[color.index()]
    }

    /// Get the [Bitboard] representing all pieces of the given [Piece] type, discarding color.
    #[inline(always)]
    pub fn piece_occupancy(&self, piece: Piece) -> Bitboard {
        self.piece_occupancy[piece.index()]
    }

    /// Get the [Bitboard] representing all pieces of the given [Piece] type, discarding color.
    /// Allow mutating the state.
    #[inline(always)]
    #[allow(unused)]
    fn piece_occupancy_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.piece_occupancy[piece.index()]
    }

    /// Get the [Bitboard] representing all colors of the given [Color] type, discarding piece
    /// type.
    #[inline(always)]
    pub fn color_occupancy(&self, color: Color) -> Bitboard {
        self.color_occupancy[color.index()]
    }

    /// Get the [Bitboard] representing all colors of the given [Color] type, discarding piece
    /// type. Allow mutating the state.
    #[inline(always)]
    #[allow(unused)]
    fn color_occupancy_mut(&mut self, color: Color) -> &mut Bitboard {
        &mut self.color_occupancy[color.index()]
    }

    /// Get the [Bitboard] representing all pieces on the board.
    #[inline(always)]
    pub fn combined_occupancy(&self) -> Bitboard {
        self.combined_occupancy
    }

    /// Return the number of half-turns without either a pawn push or a capture.
    #[inline(always)]
    pub fn half_move_clock(&self) -> u8 {
        self.half_move_clock
    }

    /// Return the total number of plies (i.e: half-turns) played so far.
    #[inline(always)]
    pub fn total_plies(&self) -> u32 {
        self.total_plies
    }
}
