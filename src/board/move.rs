use super::{Piece, Square};

type Bitset = u32;

/// A chess move, containing:
/// * Piece type.
/// * Starting square.
/// * Destination square.
/// * Optional capture type.
/// * Optional promotion type.
/// * Optional captured type.
/// * Whether the move was an en-passant capture.
/// * Whether the move was a double-step.
/// * Whether the move was a castling.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move(Bitset);

/// A builder for [Move]. This is the prefered and only way of building a [Move].
pub struct MoveBuilder {
    pub piece: Piece,
    pub start: Square,
    pub destination: Square,
    pub capture: Option<Piece>,
    pub promotion: Option<Piece>,
    pub en_passant: bool,
    pub double_step: bool,
    pub castling: bool,
}

impl From<MoveBuilder> for Move {
    #[inline(always)]
    fn from(builder: MoveBuilder) -> Self {
        Self::new(
            builder.piece,
            builder.start,
            builder.destination,
            builder.capture,
            builder.promotion,
            builder.en_passant,
            builder.double_step,
            builder.castling,
        )
    }
}

/// A [Move] is structured as a bitset with the following fields:
/// | Field       | Size | Range of values | Note                                            |
/// |-------------|------|-----------------|-------------------------------------------------|
/// | Piece       | 3    | 0-6             | Can be interpreted as a [Piece] index           |
/// | Start       | 6    | 0-63            | Can be interpreted as a [Square] index          |
/// | Destination | 6    | 0-63            | Can be interpreted as a [Square] index          |
/// | Capture     | 3    | 0-7             | Can be interpreted as a [Piece] index if not 7  |
/// | Promotion   | 3    | 0-7             | Can be interpreted as a [Piece] index if not 7  |
/// | En-pasant   | 1    | 0-1             | Boolean value                                   |
/// | Double-step | 1    | 0-1             | Boolean value                                   |
/// | Castling    | 1    | 0-1             | Boolean value                                   |
mod shift {
    use super::Bitset;

    pub const PIECE: usize = 0;
    pub const PIECE_MASK: Bitset = 0b111;

    pub const START: usize = 3;
    pub const START_MASK: Bitset = 0b11_1111;

    pub const DESTINATION: usize = 9;
    pub const DESTINATION_MASK: Bitset = 0b11_1111;

    pub const CAPTURE: usize = 15;
    pub const CAPTURE_MASK: Bitset = 0b111;

    pub const PROMOTION: usize = 18;
    pub const PROMOTION_MASK: Bitset = 0b111;

    pub const EN_PASSANT: usize = 21;
    pub const EN_PASSANT_MASK: Bitset = 0b1;

    pub const DOUBLE_STEP: usize = 22;
    pub const DOUBLE_STEP_MASK: Bitset = 0b1;

    pub const CASTLING: usize = 23;
    pub const CASTLING_MASK: Bitset = 0b1;
}

impl Move {
    /// Construct a new move.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn new(
        piece: Piece,
        start: Square,
        destination: Square,
        capture: Option<Piece>,
        promotion: Option<Piece>,
        en_passant: bool,
        double_step: bool,
        castling: bool,
    ) -> Self {
        let mut value = 0;
        value |= (piece.index() as Bitset) << shift::PIECE;
        value |= (start.index() as Bitset) << shift::START;
        value |= (destination.index() as Bitset) << shift::DESTINATION;
        value |=
            (capture.map(Piece::index).unwrap_or(Piece::NUM_VARIANTS) as Bitset) << shift::CAPTURE;
        value |= (promotion.map(Piece::index).unwrap_or(Piece::NUM_VARIANTS) as Bitset)
            << shift::PROMOTION;
        value |= (en_passant as Bitset) << shift::EN_PASSANT;
        value |= (double_step as Bitset) << shift::DOUBLE_STEP;
        value |= (castling as Bitset) << shift::CASTLING;
        Self(value)
    }

    /// Get the [Piece] that is being moved.
    #[inline(always)]
    pub fn piece(self) -> Piece {
        let index = ((self.0 >> shift::PIECE) & shift::PIECE_MASK) as usize;
        // SAFETY: we know the value is in-bounds
        unsafe { Piece::from_index_unchecked(index) }
    }

    /// Get the [Square] that this move starts from.
    #[inline(always)]
    pub fn start(self) -> Square {
        let index = ((self.0 >> shift::START) & shift::START_MASK) as usize;
        // SAFETY: we know the value is in-bounds
        unsafe { Square::from_index_unchecked(index) }
    }

    /// Get the [Square] that this move ends on.
    #[inline(always)]
    pub fn destination(self) -> Square {
        let index = ((self.0 >> shift::DESTINATION) & shift::DESTINATION_MASK) as usize;
        // SAFETY: we know the value is in-bounds
        unsafe { Square::from_index_unchecked(index) }
    }

    /// Get the [Piece] that this move captures, or `None` if there are no captures.
    #[inline(always)]
    pub fn capture(self) -> Option<Piece> {
        let index = ((self.0 >> shift::CAPTURE) & shift::CAPTURE_MASK) as usize;
        if index < Piece::NUM_VARIANTS {
            // SAFETY: we know the value is in-bounds
            unsafe { Some(Piece::from_index_unchecked(index)) }
        } else {
            None
        }
    }

    /// Get the [Piece] that this move promotes to, or `None` if there are no promotions.
    #[inline(always)]
    pub fn promotion(self) -> Option<Piece> {
        let index = ((self.0 >> shift::PROMOTION) & shift::PROMOTION_MASK) as usize;
        if index < Piece::NUM_VARIANTS {
            // SAFETY: we know the value is in-bounds
            unsafe { Some(Piece::from_index_unchecked(index)) }
        } else {
            None
        }
    }

    /// Get the whether or not the move is an en-passant capture.
    #[inline(always)]
    pub fn is_en_passant(self) -> bool {
        let index = (self.0 >> shift::EN_PASSANT) & shift::EN_PASSANT_MASK;
        index != 0
    }

    /// Get the whether or not the move is a pawn double step.
    #[inline(always)]
    pub fn is_double_step(self) -> bool {
        let index = (self.0 >> shift::DOUBLE_STEP) & shift::DOUBLE_STEP_MASK;
        index != 0
    }

    /// Get the whether or not the move is a castling.
    #[inline(always)]
    pub fn is_castling(self) -> bool {
        let index = (self.0 >> shift::CASTLING) & shift::CASTLING_MASK;
        index != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builder_simple() {
        let chess_move: Move = MoveBuilder {
            piece: Piece::Queen,
            start: Square::A2,
            destination: Square::A3,
            capture: None,
            promotion: None,
            en_passant: false,
            double_step: false,
            castling: false,
        }
        .into();
        assert_eq!(chess_move.piece(), Piece::Queen);
        assert_eq!(chess_move.start(), Square::A2);
        assert_eq!(chess_move.destination(), Square::A3);
        assert_eq!(chess_move.capture(), None);
        assert_eq!(chess_move.promotion(), None);
        assert!(!chess_move.is_en_passant());
        assert!(!chess_move.is_double_step());
        assert!(!chess_move.is_castling());
    }

    #[test]
    fn builder_all_fields() {
        let chess_move: Move = MoveBuilder {
            piece: Piece::Pawn,
            start: Square::A7,
            destination: Square::B8,
            capture: Some(Piece::Queen),
            promotion: Some(Piece::Knight),
            en_passant: true,
            double_step: true,
            castling: true,
        }
        .into();
        assert_eq!(chess_move.piece(), Piece::Pawn);
        assert_eq!(chess_move.start(), Square::A7);
        assert_eq!(chess_move.destination(), Square::B8);
        assert_eq!(chess_move.capture(), Some(Piece::Queen));
        assert_eq!(chess_move.promotion(), Some(Piece::Knight));
        assert!(chess_move.is_en_passant());
        assert!(chess_move.is_double_step());
        assert!(chess_move.is_castling());
    }
}
