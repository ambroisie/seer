use super::Bitboard;

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
    const ALL: [File; 8] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    /// Iterate over all files in order.
    pub fn iter() -> impl Iterator<Item = File> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a file index into a [File] type.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        assert!(index < 8);
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(index) }
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

    /// Turn a [File] into a [Bitboard] of all squares in that file.
    #[inline(always)]
    pub fn into_bitboard(self) -> Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { *Bitboard::FILES.get_unchecked(self.index()) }
    }
}

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
    fn into_bitboard() {
        assert_eq!(File::A.into_bitboard(), Bitboard::FILES[0]);
        assert_eq!(File::B.into_bitboard(), Bitboard::FILES[1]);
        assert_eq!(File::H.into_bitboard(), Bitboard::FILES[7]);
    }
}
