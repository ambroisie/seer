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
}
