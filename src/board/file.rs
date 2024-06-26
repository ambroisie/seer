use super::Bitboard;
use crate::utils::static_assert;

/// An enum representing a singular file on a chess board (i.e: the columns).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    /// The number of [File] variants.
    pub const NUM_VARIANTS: usize = 8;

    const ALL: [Self; Self::NUM_VARIANTS] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
    ];

    /// Iterate over all files in order.
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a file index into a [File] type.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        Self::try_from_index(index).expect("index out of bouds")
    }

    /// Convert from a file index into a [File] type. Returns [None] if the index is out of bounds.
    pub fn try_from_index(index: usize) -> Option<Self> {
        if index < Self::NUM_VARIANTS {
            // SAFETY: we know the value is in-bounds
            Some(unsafe { Self::from_index_unchecked(index) })
        } else {
            None
        }
    }

    /// Convert from a file index into a [File] type, no bounds checking.
    ///
    /// # Safety
    ///
    /// Should only be called with values that can be output by [File::index()].
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [File].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }

    /// Return the [File] to the left, as seen from white's perspective. Wraps around the board.
    pub fn left(self) -> Self {
        // SAFETY: we know the value is in-bounds, through masking
        unsafe { Self::from_index_unchecked(self.index().wrapping_sub(1) & 7) }
    }

    /// Return the [File] to the right, as seen from white's perspective. Wraps around the board.
    pub fn right(self) -> Self {
        // SAFETY: we know the value is in-bounds, through masking
        unsafe { Self::from_index_unchecked(self.index().wrapping_add(1) & 7) }
    }

    /// Turn a [File] into a [Bitboard] of all squares in that file.
    #[inline(always)]
    pub fn into_bitboard(self) -> Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { *Bitboard::FILES.get_unchecked(self.index()) }
    }
}

// Ensure that niche-optimization is in effect.
static_assert!(std::mem::size_of::<Option<File>>() == std::mem::size_of::<File>());

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_index() {
        assert_eq!(File::from_index(0), File::A);
        assert_eq!(File::from_index(1), File::B);
        assert_eq!(File::from_index(7), File::H);
    }

    #[test]
    fn index() {
        assert_eq!(File::A.index(), 0);
        assert_eq!(File::B.index(), 1);
        assert_eq!(File::H.index(), 7);
    }

    #[test]
    fn left() {
        assert_eq!(File::A.left(), File::H);
        assert_eq!(File::B.left(), File::A);
        assert_eq!(File::H.left(), File::G);
    }

    #[test]
    fn right() {
        assert_eq!(File::A.right(), File::B);
        assert_eq!(File::B.right(), File::C);
        assert_eq!(File::H.right(), File::A);
    }

    #[test]
    fn into_bitboard() {
        assert_eq!(File::A.into_bitboard(), Bitboard::FILES[0]);
        assert_eq!(File::B.into_bitboard(), Bitboard::FILES[1]);
        assert_eq!(File::H.into_bitboard(), Bitboard::FILES[7]);
    }
}
