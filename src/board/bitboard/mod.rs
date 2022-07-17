use super::Square;
mod iterator;
use iterator::*;

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
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Iterate over the [Square](crate::board::Square) values included in the board.
impl IntoIterator for Bitboard {
    type IntoIter = BitboardIterator;
    type Item = Square;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator(self.0)
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

/// Treat the [Square](crate::board::Square) as a singleton bitboard, and apply the operator.
impl std::ops::BitOr<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Square) -> Self::Output {
        self | rhs.into_bitboard()
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

/// Treat the [Square](crate::board::Square) as a singleton bitboard, and apply the operator.
impl std::ops::BitAnd<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Square) -> Self::Output {
        self & rhs.into_bitboard()
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

/// Treat the [Square](crate::board::Square) as a singleton bitboard, and apply the operator.
impl std::ops::BitXor<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Square) -> Self::Output {
        self ^ rhs.into_bitboard()
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

/// Treat the [Square](crate::board::Square) as a singleton bitboard, and apply the operator.
impl std::ops::Sub<Square> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn sub(self, rhs: Square) -> Self::Output {
        self - rhs.into_bitboard()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::square::*;

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
}
