use super::Bitboard;

/// An enum representing a singular rank on a chess board (i.e: the rows).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    const ALL: [Rank; 8] = [
        Rank::First,
        Rank::Second,
        Rank::Third,
        Rank::Fourth,
        Rank::Fifth,
        Rank::Sixth,
        Rank::Seventh,
        Rank::Eighth,
    ];

    /// Iterate over all ranks in order.
    pub fn iter() -> impl Iterator<Item = Rank> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a rank index into a [Rank] type.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        assert!(index < 8);
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(index) }
    }

    /// Convert from a rank index into a [Rank] type, no bounds checking.
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [Rank].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }

    /// Turn a [Rank] into a [Bitboard] of all squares in that rank.
    #[inline(always)]
    pub fn into_bitboard(self) -> Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { *Bitboard::RANKS.get_unchecked(self.index()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_index() {
        assert_eq!(Rank::from_index(0), Rank::First);
        assert_eq!(Rank::from_index(1), Rank::Second);
        assert_eq!(Rank::from_index(7), Rank::Eighth);
    }

    #[test]
    fn index() {
        assert_eq!(Rank::First.index(), 0);
        assert_eq!(Rank::Second.index(), 1);
        assert_eq!(Rank::Eighth.index(), 7);
    }

    #[test]
    fn into_bitboard() {
        assert_eq!(Rank::First.into_bitboard(), Bitboard::RANKS[0]);
        assert_eq!(Rank::Second.into_bitboard(), Bitboard::RANKS[1]);
        assert_eq!(Rank::Eighth.into_bitboard(), Bitboard::RANKS[7]);
    }
}
