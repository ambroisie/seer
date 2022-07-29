use super::Square;
use crate::utils::static_assert;

mod error;
use error::*;
mod iterator;
use iterator::*;
mod superset;
use superset::*;

/// Use a 64-bit number to represent a chessboard. Each bit is mapped from to a specific square, so
/// that index 0 -> A1, 1 -> A2, ..., 63 -> H8.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitboard(pub(crate) u64);

impl Bitboard {
    /// An empty bitboard.
    pub const EMPTY: Bitboard = Bitboard(0);

    /// A full bitboard.
    pub const ALL: Bitboard = Bitboard(u64::MAX);

    /// Array of bitboards representing the eight ranks, in order from rank 1 to rank 8.
    pub const RANKS: [Self; 8] = [
        Bitboard(0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001),
        Bitboard(0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010),
        Bitboard(0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100),
        Bitboard(0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000),
        Bitboard(0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000),
        Bitboard(0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000),
        Bitboard(0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000),
        Bitboard(0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000),
    ];

    /// Array of bitboards representing the eight files, in order from file A to file H.
    pub const FILES: [Self; 8] = [
        Bitboard(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111),
        Bitboard(0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000),
        Bitboard(0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000),
        Bitboard(0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000),
        Bitboard(0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000),
        Bitboard(0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000),
        Bitboard(0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000),
        Bitboard(0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    ];

    /// The diagonal from [Square::A1] to [Square::H8].
    pub const DIAGONAL: Bitboard = Bitboard(0x8040201008040201);

    /// The diagonal from [Square::A8] to [Square::H1].
    pub const ANTI_DIAGONAL: Bitboard = Bitboard(0x0102040810204080);

    /// The light [Square]s on a board, e.g: [Square::H1].
    pub const LIGHT_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);

    /// The dark [Square]s on a board, e.g: [Square::A1].
    pub const DARK_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);

    /// Count the number of pieces in the [Bitboard].
    #[inline(always)]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Return true if there are no pieces in the [Bitboard], otherwise false.
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    /// Return true if there are more than piece in the [Bitboard]. This is faster than testing
    /// `board.count() > 1`.
    #[inline(always)]
    pub fn has_more_than_one(self) -> bool {
        (self.0 & (self.0.wrapping_sub(1))) != 0
    }

    /// Iterate over the power-set of a given [Bitboard], yielding each possible sub-set of
    /// [Square] that belong to the [Bitboard]. In other words, generate all set of [Square] that
    /// contain all, some, or none of the [Square] that are in the given [Bitboard].
    /// If given an empty [Bitboard], yields the empty [Bitboard] back.
    #[inline(always)]
    pub fn iter_power_set(self) -> impl Iterator<Item = Self> {
        BitboardPowerSetIterator::new(self)
    }
}

// Ensure zero-cost (at least size-wise) wrapping.
static_assert!(std::mem::size_of::<Bitboard>() == std::mem::size_of::<u64>());

impl Default for Bitboard {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Iterate over the [Square] values included in the board.
impl IntoIterator for Bitboard {
    type IntoIter = BitboardIterator;
    type Item = Square;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator::new(self)
    }
}

/// If the given [Bitboard] is a singleton piece on a board, return the [Square] that it is
/// occupying. Otherwise return `None`.
impl TryInto<Square> for Bitboard {
    type Error = IntoSquareError;

    fn try_into(self) -> Result<Square, Self::Error> {
        let index = match self.count() {
            1 => self.0.trailing_zeros() as usize,
            0 => return Err(IntoSquareError::EmptyBoard),
            _ => return Err(IntoSquareError::TooManySquares),
        };
        Ok(Square::from_index(index))
    }
}

/// Treat bitboard as a set of squares, shift each square's index left by the amount given.
impl std::ops::Shl<usize> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

/// Treat bitboard as a set of squares, shift each square's index right by the amount given.
impl std::ops::Shr<usize> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

/// Treat bitboard as a set of squares, shift each square's index left by the amount given.
impl std::ops::ShlAssign<usize> for Bitboard {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

/// Treat bitboard as a set of squares, shift each square's index right by the amount given.
impl std::ops::ShrAssign<usize> for Bitboard {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

/// Treat bitboard as a set of squares, and invert the set.
impl std::ops::Not for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

/// Treat each bitboard as a set of squares, keep squares that are in either sets.
impl std::ops::BitOr<Bitboard> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::BitOr<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Square) -> Self::Output {
        self | rhs.into_bitboard()
    }
}

/// Treat each bitboard as a set of squares, keep squares that are in either sets.
impl std::ops::BitOrAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Bitboard) {
        *self = *self | rhs;
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::BitOrAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Square) {
        *self = *self | rhs;
    }
}

/// Treat each bitboard as a set of squares, keep squares that are in both sets.
impl std::ops::BitAnd<Bitboard> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::BitAnd<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Square) -> Self::Output {
        self & rhs.into_bitboard()
    }
}

/// Treat each bitboard as a set of squares, keep squares that are in both sets.
impl std::ops::BitAndAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Bitboard) {
        *self = *self & rhs;
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::BitAndAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Square) {
        *self = *self & rhs;
    }
}

/// Treat each bitboard as a set of squares, keep squares that are in exactly one of either set.
impl std::ops::BitXor<Bitboard> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::BitXor<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Square) -> Self::Output {
        self ^ rhs.into_bitboard()
    }
}

/// Treat each bitboard as a set of squares, keep squares that are in exactly one of either set.
impl std::ops::BitXorAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        *self = *self ^ rhs;
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::BitXorAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Square) {
        *self = *self ^ rhs;
    }
}

/// Treat each bitboard as a set of squares, and substract one set from another.
impl std::ops::Sub<Bitboard> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn sub(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & !rhs.0)
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::Sub<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn sub(self, rhs: Square) -> Self::Output {
        self - rhs.into_bitboard()
    }
}

/// Treat each bitboard as a set of squares, and substract one set from another.
impl std::ops::SubAssign<Bitboard> for Bitboard {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Bitboard) {
        *self = *self - rhs;
    }
}

/// Treat the [Square] as a singleton bitboard, and apply the operator.
impl std::ops::SubAssign<Square> for Bitboard {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Square) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;
    use crate::board::{square::*, File, Rank};

    #[test]
    fn count() {
        assert_eq!(Bitboard::EMPTY.count(), 0);
        assert_eq!(Bitboard::FILES[0].count(), 8);
        assert_eq!(Bitboard::ALL.count(), 64);
    }

    #[test]
    fn iter() {
        assert_eq!(Bitboard::EMPTY.into_iter().collect::<Vec<_>>(), Vec::new());
        assert_eq!(
            Bitboard::RANKS[0].into_iter().collect::<Vec<_>>(),
            vec![
                Square::A1,
                Square::B1,
                Square::C1,
                Square::D1,
                Square::E1,
                Square::F1,
                Square::G1,
                Square::H1,
            ]
        );
        assert_eq!(
            Bitboard::FILES[0].into_iter().collect::<Vec<_>>(),
            vec![
                Square::A1,
                Square::A2,
                Square::A3,
                Square::A4,
                Square::A5,
                Square::A6,
                Square::A7,
                Square::A8,
            ]
        );
    }

    #[test]
    fn left_shift() {
        assert_eq!(Bitboard::RANKS[0] << 1, Bitboard::RANKS[1]);
        assert_eq!(Bitboard::FILES[0] << 8, Bitboard::FILES[1]);
    }

    #[test]
    fn right_shift() {
        assert_eq!(Bitboard::RANKS[1] >> 1, Bitboard::RANKS[0]);
        assert_eq!(Bitboard::FILES[1] >> 8, Bitboard::FILES[0]);
    }

    #[test]
    fn not() {
        assert_eq!(!Bitboard::EMPTY, Bitboard::ALL);
    }

    #[test]
    fn or() {
        assert_eq!(Bitboard::FILES[0] | Bitboard::FILES[1], Bitboard(0xff_ff));
        assert_eq!(Bitboard::FILES[0] | Square::B1, Bitboard(0x1_ff));
    }

    #[test]
    fn and() {
        assert_eq!(Bitboard::FILES[0] & Bitboard::FILES[1], Bitboard::EMPTY);
        assert_eq!(
            Bitboard::FILES[0] & Bitboard::RANKS[0],
            Square::A1.into_bitboard()
        );
        assert_eq!(Bitboard::FILES[0] & Square::A1, Square::A1.into_bitboard());
    }

    #[test]
    fn xor() {
        assert_eq!(Bitboard::FILES[0] ^ Square::A1, Bitboard(0xff - 1));
    }

    #[test]
    fn sub() {
        assert_eq!(Bitboard::FILES[0] - Bitboard::RANKS[0], Bitboard(0xff - 1));
        assert_eq!(Bitboard::FILES[0] - Square::A1, Bitboard(0xff - 1));
    }

    #[test]
    fn more_than_one() {
        assert!(!Bitboard::EMPTY.has_more_than_one());
        for square in Square::iter() {
            assert!(!square.into_bitboard().has_more_than_one())
        }
        assert!((Square::A1 | Square::H8).has_more_than_one());
        assert!(Bitboard::ALL.has_more_than_one());
    }

    #[test]
    fn iter_power_set_empty() {
        assert_eq!(
            Bitboard::EMPTY.iter_power_set().collect::<Vec<_>>(),
            vec![Bitboard::EMPTY]
        )
    }

    #[test]
    fn iter_power_set_one_square() {
        for square in Square::iter() {
            assert_eq!(
                square
                    .into_bitboard()
                    .iter_power_set()
                    .collect::<HashSet<_>>(),
                [Bitboard::EMPTY, square.into_bitboard()]
                    .into_iter()
                    .collect::<HashSet<_>>()
            )
        }
    }

    #[test]
    fn iter_power_set_two_squares() {
        assert_eq!(
            (Square::A1 | Square::H8)
                .iter_power_set()
                .collect::<HashSet<_>>(),
            [
                Bitboard::EMPTY,
                Square::A1.into_bitboard(),
                Square::H8.into_bitboard(),
                Square::A1 | Square::H8
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn iter_power_set_six_squares_exhaustive() {
        let mask = (0..6)
            .map(Square::from_index)
            .fold(Bitboard::EMPTY, |lhs, rhs| lhs | rhs);
        assert_eq!(
            mask.iter_power_set().collect::<HashSet<_>>(),
            (0..(1 << 6)).map(Bitboard).collect::<HashSet<_>>()
        )
    }

    #[test]
    fn iter_power_set_eight_squares_length() {
        assert_eq!(
            File::A
                .into_bitboard()
                .iter_power_set()
                .collect::<HashSet<_>>()
                .len(),
            1 << 8
        );
        assert_eq!(
            Rank::First
                .into_bitboard()
                .iter_power_set()
                .collect::<HashSet<_>>()
                .len(),
            1 << 8
        );
    }

    #[test]
    fn into_square() {
        for square in Square::iter() {
            assert_eq!(square.into_bitboard().try_into(), Ok(square));
        }
    }

    #[test]
    fn into_square_invalid() {
        assert_eq!(
            TryInto::<Square>::try_into(Bitboard::EMPTY),
            Err(IntoSquareError::EmptyBoard)
        );
        assert_eq!(
            TryInto::<Square>::try_into(Square::A1 | Square::A2),
            Err(IntoSquareError::TooManySquares)
        )
    }
}
