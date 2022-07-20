use super::Bitboard;
use crate::utils::static_assert;

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
    /// The number of [Rank] variants.
    pub const NUM_VARIANTS: usize = 8;

    const ALL: [Self; Self::NUM_VARIANTS] = [
        Self::First,
        Self::Second,
        Self::Third,
        Self::Fourth,
        Self::Fifth,
        Self::Sixth,
        Self::Seventh,
        Self::Eighth,
    ];

    /// Iterate over all ranks in order.
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a rank index into a [Rank] type.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        assert!(index < Self::NUM_VARIANTS);
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(index) }
    }

    /// Convert from a rank index into a [Rank] type, no bounds checking.
    ///
    /// # Safety
    ///
    /// Should only be called with values that can be output by [Rank::index()].
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [Rank].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }

    /// Return the [Rank] one-row up, as seen from white's perspective. Wraps around the board.
    pub fn up(self) -> Self {
        // SAFETY: we know the value is in-bounds, through masking
        unsafe { Self::from_index_unchecked(self.index().wrapping_add(1) & 7) }
    }

    /// Return the [Rank] one-row down, as seen from white's perspective. Wraps around the board.
    pub fn down(self) -> Self {
        // SAFETY: we know the value is in-bounds, through masking
        unsafe { Self::from_index_unchecked(self.index().wrapping_sub(1) & 7) }
    }

    /// Turn a [Rank] into a [Bitboard] of all squares in that rank.
    #[inline(always)]
    pub fn into_bitboard(self) -> Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { *Bitboard::RANKS.get_unchecked(self.index()) }
    }
}

// Ensure that niche-optimization is in effect.
static_assert!(std::mem::size_of::<Option<Rank>>() == std::mem::size_of::<Rank>());

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
    fn up() {
        assert_eq!(Rank::First.up(), Rank::Second);
        assert_eq!(Rank::Second.up(), Rank::Third);
        assert_eq!(Rank::Eighth.up(), Rank::First);
    }

    #[test]
    fn down() {
        assert_eq!(Rank::First.down(), Rank::Eighth);
        assert_eq!(Rank::Second.down(), Rank::First);
        assert_eq!(Rank::Eighth.down(), Rank::Seventh);
    }

    #[test]
    fn into_bitboard() {
        assert_eq!(Rank::First.into_bitboard(), Bitboard::RANKS[0]);
        assert_eq!(Rank::Second.into_bitboard(), Bitboard::RANKS[1]);
        assert_eq!(Rank::Eighth.into_bitboard(), Bitboard::RANKS[7]);
    }
}
