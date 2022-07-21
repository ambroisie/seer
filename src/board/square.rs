use super::{Bitboard, File, Rank};
use crate::utils::static_assert;

/// Represent a square on a chessboard. Defined in the same order as the
/// [Bitboard] squares.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Square {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl Square {
    /// The number of [Square] variants.
    pub const NUM_VARIANTS: usize = 64;

    #[rustfmt::skip]
    const ALL: [Self; Self::NUM_VARIANTS] = [
        Self::A1, Self::A2, Self::A3, Self::A4, Self::A5, Self::A6, Self::A7, Self::A8,
        Self::B1, Self::B2, Self::B3, Self::B4, Self::B5, Self::B6, Self::B7, Self::B8,
        Self::C1, Self::C2, Self::C3, Self::C4, Self::C5, Self::C6, Self::C7, Self::C8,
        Self::D1, Self::D2, Self::D3, Self::D4, Self::D5, Self::D6, Self::D7, Self::D8,
        Self::E1, Self::E2, Self::E3, Self::E4, Self::E5, Self::E6, Self::E7, Self::E8,
        Self::F1, Self::F2, Self::F3, Self::F4, Self::F5, Self::F6, Self::F7, Self::F8,
        Self::G1, Self::G2, Self::G3, Self::G4, Self::G5, Self::G6, Self::G7, Self::G8,
        Self::H1, Self::H2, Self::H3, Self::H4, Self::H5, Self::H6, Self::H7, Self::H8,
    ];

    /// Construct a [Square] from a [File] and [Rank].
    #[inline(always)]
    pub fn new(file: File, rank: Rank) -> Self {
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(file.index() * 8 + rank.index()) }
    }

    /// Iterate over all squares in order.
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    /// Convert from a square index into a [Square] type.
    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        assert!(index < 64);
        // SAFETY: we know the value is in-bounds
        unsafe { Self::from_index_unchecked(index) }
    }

    /// Convert from a square index into a [Square] type, no bounds checking.
    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: usize) -> Self {
        std::mem::transmute(index as u8)
    }

    /// Return the index of a given [Square].
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }

    /// Return the index of the rank of this square (0 -> rank 1, ..., 7 -> rank 8).
    #[inline(always)]
    pub fn rank_index(self) -> usize {
        self.index() % 8
    }

    /// Return the index of the rank of this square (0 -> file A, ..., 7 -> file H).
    #[inline(always)]
    pub fn file_index(self) -> usize {
        self.index() / 8
    }

    /// Return a [Rank] representing the rank of this square.
    #[inline(always)]
    pub fn rank(self) -> Rank {
        // SAFETY: we know the value is in-bounds
        unsafe { Rank::from_index_unchecked(self.rank_index()) }
    }

    /// Return a [File] representing the rank of this square.
    #[inline(always)]
    pub fn file(self) -> File {
        // SAFETY: we know the value is in-bounds
        unsafe { File::from_index_unchecked(self.file_index()) }
    }

    /// Turn a square into a singleton bitboard.
    #[inline(always)]
    pub fn into_bitboard(self) -> Bitboard {
        Bitboard(1 << (self as usize))
    }
}

/// Shift the square's index left by the amount given.
impl std::ops::Shl<usize> for Square {
    type Output = Square;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        Square::from_index(self as usize + rhs)
    }
}

/// Shift the square's index right by the amount given.
impl std::ops::Shr<usize> for Square {
    type Output = Square;

    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        Square::from_index(self as usize - rhs)
    }
}

/// Return a board containing all squares but the one given.
impl std::ops::Not for Square {
    type Output = Bitboard;

    #[inline(always)]
    fn not(self) -> Self::Output {
        !self.into_bitboard()
    }
}

/// Treat the square as a singleton board, and apply the operator.
impl std::ops::BitOr<Square> for Square {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Square) -> Self::Output {
        self.into_bitboard() | rhs.into_bitboard()
    }
}

/// Treat the square as a singleton board, and apply the operator.
impl std::ops::BitOr<Bitboard> for Square {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        self.into_bitboard() | rhs
    }
}

/// Treat the square as a singleton board, and apply the operator.
impl std::ops::BitAnd<Bitboard> for Square {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        self.into_bitboard() & rhs
    }
}

/// Treat the square as a singleton board, and apply the operator.
impl std::ops::BitXor<Bitboard> for Square {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        self.into_bitboard() ^ rhs
    }
}

/// Treat the square as a singleton board, and apply the operator.
impl std::ops::Sub<Bitboard> for Square {
    type Output = Bitboard;

    #[inline(always)]
    fn sub(self, rhs: Bitboard) -> Self::Output {
        self.into_bitboard() - rhs
    }
}

// Ensure that niche-optimization is in effect.
static_assert!(std::mem::size_of::<Option<Square>>() == std::mem::size_of::<Square>());

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::bitboard::*;
    use crate::board::file::*;
    use crate::board::rank::*;

    #[test]
    fn new() {
        assert_eq!(Square::new(File::A, Rank::First), Square::A1);
        assert_eq!(Square::new(File::A, Rank::Second), Square::A2);
        assert_eq!(Square::new(File::B, Rank::First), Square::B1);
        assert_eq!(Square::new(File::H, Rank::Eighth), Square::H8);
    }

    #[test]
    fn index() {
        assert_eq!(Square::A1.index(), 0);
        assert_eq!(Square::A2.index(), 1);
        assert_eq!(Square::B1.index(), 8);
        assert_eq!(Square::H8.index(), 63);
    }

    #[test]
    fn file() {
        assert_eq!(Square::A1.file(), File::A);
        assert_eq!(Square::A2.file(), File::A);
        assert_eq!(Square::B1.file(), File::B);
        assert_eq!(Square::H8.file(), File::H);
    }

    #[test]
    fn rank() {
        assert_eq!(Square::A1.rank(), Rank::First);
        assert_eq!(Square::A2.rank(), Rank::Second);
        assert_eq!(Square::B1.rank(), Rank::First);
        assert_eq!(Square::H8.rank(), Rank::Eighth);
    }

    #[test]
    fn left_shift() {
        assert_eq!(Square::A1 << 1, Square::A2);
        assert_eq!(Square::A1 << 8, Square::B1);
    }

    #[test]
    fn right_shift() {
        assert_eq!(Square::A2 >> 1, Square::A1);
        assert_eq!(Square::B1 >> 8, Square::A1);
    }

    #[test]
    fn not() {
        assert_eq!(!Square::A1, Bitboard(u64::MAX - 1));
    }

    #[test]
    fn or() {
        assert_eq!(Square::A1 | Square::A2, Bitboard(0b11));
    }

    #[test]
    fn and() {
        assert_eq!(Square::A1 & Bitboard::FILES[0], Square::A1.into_bitboard());
    }

    #[test]
    fn xor() {
        assert_eq!(Square::A1 ^ Bitboard::FILES[0], Bitboard(0xff - 1));
    }

    #[test]
    fn sub() {
        assert_eq!(Square::A1 - Bitboard::FILES[0], Bitboard::EMPTY);
    }
}
