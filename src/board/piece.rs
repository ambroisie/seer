/// An enum representing the type of a piece.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    /// The number of [Piece] variants.
    pub const NUM_VARIANTS: usize = 6;

    const ALL: [Self; Self::NUM_VARIANTS] = [
        Self::King,
        Self::Queen,
        Self::Rook,
        Self::Bishop,
        Self::Knight,
        Self::Pawn,
    ];

    /// Iterate over all piece types.
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a piece index into a [Piece] type.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        assert!(index < Self::NUM_VARIANTS);
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(index) }
    }

    /// Convert from a piece index into a [Piece] type, no bounds checking.
    ///
    /// # Safety
    ///
    /// Should only be called with values that can be output by [Piece::index()].
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [Piece].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_index() {
        assert_eq!(Piece::from_index(0), Piece::King);
        assert_eq!(Piece::from_index(1), Piece::Queen);
        assert_eq!(Piece::from_index(5), Piece::Pawn);
    }

    #[test]
    fn index() {
        assert_eq!(Piece::King.index(), 0);
        assert_eq!(Piece::Queen.index(), 1);
        assert_eq!(Piece::Pawn.index(), 5);
    }
}
