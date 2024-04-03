use super::{Piece, Square};

/// A chess move, containing:
/// * Starting square.
/// * Destination square.
/// * Optional promotion type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    start: Square,
    destination: Square,
    promotion: Option<Piece>,
}

impl Move {
    /// Construct a new move.
    #[inline(always)]
    pub fn new(start: Square, destination: Square, promotion: Option<Piece>) -> Self {
        Self {
            start,
            destination,
            promotion,
        }
    }

    /// Get the [Square] that this move starts from.
    #[inline(always)]
    pub fn start(self) -> Square {
        self.start
    }

    /// Get the [Square] that this move ends on.
    #[inline(always)]
    pub fn destination(self) -> Square {
        self.destination
    }

    /// Get the [Piece] that this move promotes to, or `None` if there are no promotions.
    #[inline(always)]
    pub fn promotion(self) -> Option<Piece> {
        self.promotion
    }
}
