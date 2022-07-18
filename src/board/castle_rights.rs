/// Current castle rights for a player.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastleRights {
    /// No castling allowed.
    NoSide,
    /// King-side castling only.
    KingSide,
    /// Queen-side castling only.
    QueenSide,
    /// Either side allowed.
    BothSides,
}

impl CastleRights {
    /// Convert from a castle rights index into a [CastleRights] type.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        assert!(index < 4);
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(index) }
    }

    /// Convert from a castle rights index into a [CastleRights] type, no bounds checking.
    ///
    /// # Safety
    ///
    /// This should only be called with values that can be output by [CastleRights::index()].
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [CastleRights].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }

    /// Can the player castle king-side.
    #[inline(always)]
    pub fn has_king_side(self) -> bool {
        (self.index() & 1) != 0
    }

    /// Can the player castle king-side.
    #[inline(always)]
    pub fn has_queen_side(self) -> bool {
        (self.index() & 2) != 0
    }

    /// Remove king-side castling rights.
    #[inline(always)]
    pub fn without_king_side(self) -> Self {
        self.remove(Self::KingSide)
    }

    /// Remove queen-side castling rights.
    #[inline(always)]
    pub fn without_queen_side(self) -> Self {
        self.remove(Self::QueenSide)
    }

    /// Remove some [CastleRights], and return the resulting [CastleRights].
    #[inline(always)]
    pub fn remove(self, to_remove: CastleRights) -> Self {
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(self.index() & !to_remove.index()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_index() {
        assert_eq!(CastleRights::from_index(0), CastleRights::NoSide);
        assert_eq!(CastleRights::from_index(1), CastleRights::KingSide);
        assert_eq!(CastleRights::from_index(2), CastleRights::QueenSide);
        assert_eq!(CastleRights::from_index(3), CastleRights::BothSides);
    }

    #[test]
    fn index() {
        assert_eq!(CastleRights::NoSide.index(), 0);
        assert_eq!(CastleRights::KingSide.index(), 1);
        assert_eq!(CastleRights::QueenSide.index(), 2);
        assert_eq!(CastleRights::BothSides.index(), 3);
    }

    #[test]
    fn has_kingside() {
        assert!(!CastleRights::NoSide.has_king_side());
        assert!(!CastleRights::QueenSide.has_king_side());
        assert!(CastleRights::KingSide.has_king_side());
        assert!(CastleRights::BothSides.has_king_side());
    }

    #[test]
    fn has_queenside() {
        assert!(!CastleRights::NoSide.has_queen_side());
        assert!(!CastleRights::KingSide.has_queen_side());
        assert!(CastleRights::QueenSide.has_queen_side());
        assert!(CastleRights::BothSides.has_queen_side());
    }

    #[test]
    fn without_king_side() {
        assert_eq!(
            CastleRights::NoSide.without_king_side(),
            CastleRights::NoSide
        );
        assert_eq!(
            CastleRights::KingSide.without_king_side(),
            CastleRights::NoSide
        );
        assert_eq!(
            CastleRights::QueenSide.without_king_side(),
            CastleRights::QueenSide
        );
        assert_eq!(
            CastleRights::BothSides.without_king_side(),
            CastleRights::QueenSide
        );
    }

    #[test]
    fn without_queen_side() {
        assert_eq!(
            CastleRights::NoSide.without_queen_side(),
            CastleRights::NoSide
        );
        assert_eq!(
            CastleRights::QueenSide.without_queen_side(),
            CastleRights::NoSide
        );
        assert_eq!(
            CastleRights::KingSide.without_queen_side(),
            CastleRights::KingSide
        );
        assert_eq!(
            CastleRights::BothSides.without_queen_side(),
            CastleRights::KingSide
        );
    }
}
