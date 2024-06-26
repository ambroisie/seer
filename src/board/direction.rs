use super::{Bitboard, Rank, Square};

/// A direction on the board. Either along the rook, bishop, or knight directions
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,

    NorthWest,
    SouthWest,
    SouthEast,
    NorthEast,

    NorthNorthWest,
    NorthWestWest,
    SouthWestWest,
    SouthSouthWest,
    SouthSouthEast,
    SouthEastEast,
    NorthEastEast,
    NorthNorthEast,
}

impl Direction {
    /// Directions that a rook could use.
    pub const ROOK_DIRECTIONS: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];

    /// Directions that a bishop could use.
    pub const BISHOP_DIRECTIONS: [Self; 4] = [
        Self::NorthWest,
        Self::SouthWest,
        Self::SouthEast,
        Self::NorthEast,
    ];

    /// Directions that a knight could use.
    pub const KNIGHT_DIRECTIONS: [Self; 8] = [
        Self::NorthNorthWest,
        Self::NorthWestWest,
        Self::SouthWestWest,
        Self::SouthSouthWest,
        Self::SouthSouthEast,
        Self::SouthEastEast,
        Self::NorthEastEast,
        Self::NorthNorthEast,
    ];

    /// Iterate over all directions a rook can take.
    pub fn iter_rook() -> impl Iterator<Item = Direction> {
        Self::ROOK_DIRECTIONS.iter().cloned()
    }

    /// Iterate over all directions a bishop can take.
    pub fn iter_bishop() -> impl Iterator<Item = Direction> {
        Self::BISHOP_DIRECTIONS.iter().cloned()
    }

    /// Iterate over all directions a queen or king can take.
    pub fn iter_royalty() -> impl Iterator<Item = Direction> {
        Self::iter_rook().chain(Self::iter_bishop())
    }

    /// Iterate over all directions a knight can take.
    pub fn iter_knight() -> impl Iterator<Item = Direction> {
        Self::KNIGHT_DIRECTIONS.iter().cloned()
    }

    /// Move a [Square] along the given [Direction], unless it would wrap at the end of the board
    pub fn move_square(self, square: Square) -> Option<Square> {
        let res = self.move_board(square.into_bitboard());
        res.into_iter().next()
    }

    /// Move every piece on a board along the given direction. Do not wrap around the board.
    #[inline(always)]
    pub fn move_board(self, board: Bitboard) -> Bitboard {
        // No need to filter for A/H ranks thanks to wrapping
        match self {
            Self::North => (board - Rank::Eighth.into_bitboard()) << 1,
            Self::West => board >> 8,
            Self::South => (board - Rank::First.into_bitboard()) >> 1,
            Self::East => board << 8,

            Self::NorthWest => (board - Rank::Eighth.into_bitboard()) >> 7,
            Self::SouthWest => (board - Rank::First.into_bitboard()) >> 9,
            Self::SouthEast => (board - Rank::First.into_bitboard()) << 7,
            Self::NorthEast => (board - Rank::Eighth.into_bitboard()) << 9,

            Self::NorthNorthWest => {
                (board - Rank::Eighth.into_bitboard() - Rank::Seventh.into_bitboard()) >> 6
            }
            Self::NorthWestWest => (board - Rank::Eighth.into_bitboard()) >> 15,
            Self::SouthWestWest => (board - Rank::First.into_bitboard()) >> 17,
            Self::SouthSouthWest => {
                (board - Rank::First.into_bitboard() - Rank::Second.into_bitboard()) >> 10
            }
            Self::SouthSouthEast => {
                (board - Rank::First.into_bitboard() - Rank::Second.into_bitboard()) << 6
            }
            Self::SouthEastEast => (board - Rank::First.into_bitboard()) << 15,
            Self::NorthEastEast => (board - Rank::Eighth.into_bitboard()) << 17,
            Self::NorthNorthEast => {
                (board - Rank::Eighth.into_bitboard() - Rank::Seventh.into_bitboard()) << 10
            }
        }
    }

    /// Slide a board along the given [Direction], i.e: return all successive applications of
    /// [Direction::move_square] until no new squares can be reached.
    /// It does not make sense to use this method with knight-only directions, and it will panic in
    /// debug-mode if it happens.
    #[inline(always)]
    pub fn slide_square(self, square: Square) -> Bitboard {
        self.slide_board(square.into_bitboard())
    }

    /// Slide a board along the given [Direction], i.e: return all successive applications of
    /// [Direction::move_board] until no new squares can be reached.
    /// It does not make sense to use this method with knight-only directions, and it will panic in
    /// debug-mode if it happens.
    #[inline(always)]
    pub fn slide_board(self, board: Bitboard) -> Bitboard {
        self.slide_board_with_blockers(board, Bitboard::EMPTY)
    }

    /// Slide a board along the given [Direction], i.e: return all successive applications of
    /// [Direction::move_board] until no new squares can be reached.
    /// Take into account the `blockers` [Bitboard]: a combination of all pieces on the board which
    /// cannot be slid over. The slide is over once a square that is part of `blockers` is reached.
    /// It does not make sense to use this method with knight-only directions, and it will panic in
    /// debug-mode if it happens.
    #[inline(always)]
    pub fn slide_board_with_blockers(self, mut board: Bitboard, blockers: Bitboard) -> Bitboard {
        debug_assert!(!Self::KNIGHT_DIRECTIONS.contains(&self));

        let mut res = Default::default();

        while !board.is_empty() {
            board = self.move_board(board);
            res |= board;
            if !(board & blockers).is_empty() {
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{File, Rank};

    #[test]
    fn north() {
        assert_eq!(
            Direction::North.move_board(Square::A1.into_bitboard()),
            Square::A2.into_bitboard()
        );
        assert_eq!(
            Direction::North.move_board(Square::A2.into_bitboard()),
            Square::A3.into_bitboard()
        );
        assert_eq!(
            Direction::North.move_board(Square::A7.into_bitboard()),
            Square::A8.into_bitboard()
        );
        assert_eq!(
            Direction::North.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY
        );
    }

    #[test]
    fn west() {
        assert_eq!(
            Direction::West.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::West.move_board(Square::B1.into_bitboard()),
            Square::A1.into_bitboard()
        );
        assert_eq!(
            Direction::West.move_board(Square::G1.into_bitboard()),
            Square::F1.into_bitboard()
        );
        assert_eq!(
            Direction::West.move_board(Square::H1.into_bitboard()),
            Square::G1.into_bitboard()
        );
    }

    #[test]
    fn south() {
        assert_eq!(
            Direction::South.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::South.move_board(Square::A2.into_bitboard()),
            Square::A1.into_bitboard()
        );
        assert_eq!(
            Direction::South.move_board(Square::A7.into_bitboard()),
            Square::A6.into_bitboard()
        );
        assert_eq!(
            Direction::South.move_board(Square::A8.into_bitboard()),
            Square::A7.into_bitboard()
        );
    }

    #[test]
    fn east() {
        assert_eq!(
            Direction::East.move_board(Square::A1.into_bitboard()),
            Square::B1.into_bitboard()
        );
        assert_eq!(
            Direction::East.move_board(Square::B1.into_bitboard()),
            Square::C1.into_bitboard()
        );
        assert_eq!(
            Direction::East.move_board(Square::G1.into_bitboard()),
            Square::H1.into_bitboard()
        );
        assert_eq!(
            Direction::East.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY
        );
    }

    #[test]
    fn north_west() {
        assert_eq!(
            Direction::NorthWest.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::NorthWest.move_board(Square::B1.into_bitboard()),
            Square::A2.into_bitboard()
        );
        assert_eq!(
            Direction::NorthWest.move_board(Square::H1.into_bitboard()),
            Square::G2.into_bitboard()
        );
        assert_eq!(
            Direction::NorthWest.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::NorthWest.move_board(Square::B8.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::NorthWest.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY
        );
    }

    #[test]
    fn south_west() {
        assert_eq!(
            Direction::SouthWest.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthWest.move_board(Square::B1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthWest.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthWest.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthWest.move_board(Square::B8.into_bitboard()),
            Square::A7.into_bitboard()
        );
        assert_eq!(
            Direction::SouthWest.move_board(Square::H8.into_bitboard()),
            Square::G7.into_bitboard()
        );
    }

    #[test]
    fn south_east() {
        assert_eq!(
            Direction::SouthEast.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthEast.move_board(Square::B1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthEast.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::SouthEast.move_board(Square::A8.into_bitboard()),
            Square::B7.into_bitboard()
        );
        assert_eq!(
            Direction::SouthEast.move_board(Square::B8.into_bitboard()),
            Square::C7.into_bitboard()
        );
        assert_eq!(
            Direction::SouthEast.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY
        );
    }

    #[test]
    fn north_east() {
        assert_eq!(
            Direction::NorthEast.move_board(Square::A1.into_bitboard()),
            Square::B2.into_bitboard()
        );
        assert_eq!(
            Direction::NorthEast.move_board(Square::B1.into_bitboard()),
            Square::C2.into_bitboard()
        );
        assert_eq!(
            Direction::NorthEast.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::NorthEast.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::NorthEast.move_board(Square::B8.into_bitboard()),
            Bitboard::EMPTY
        );
        assert_eq!(
            Direction::NorthEast.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY
        );
    }

    #[test]
    fn north_north_west() {
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::B2.into_bitboard()),
            Square::A4.into_bitboard()
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::H1.into_bitboard()),
            Square::G3.into_bitboard()
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::G2.into_bitboard()),
            Square::F4.into_bitboard()
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::B7.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthWest.move_board(Square::G7.into_bitboard()),
            Bitboard::EMPTY,
        );
    }

    #[test]
    fn north_west_west() {
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::B2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::H1.into_bitboard()),
            Square::F2.into_bitboard()
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::G2.into_bitboard()),
            Square::E3.into_bitboard()
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::B7.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthWestWest.move_board(Square::G7.into_bitboard()),
            Square::E8.into_bitboard()
        );
    }

    #[test]
    fn south_west_west() {
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::B2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::G2.into_bitboard()),
            Square::E1.into_bitboard()
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::B7.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::H8.into_bitboard()),
            Square::F7.into_bitboard()
        );
        assert_eq!(
            Direction::SouthWestWest.move_board(Square::G7.into_bitboard()),
            Square::E6.into_bitboard()
        );
    }

    #[test]
    fn south_south_west() {
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::B2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::G2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::B7.into_bitboard()),
            Square::A5.into_bitboard()
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::H8.into_bitboard()),
            Square::G6.into_bitboard()
        );
        assert_eq!(
            Direction::SouthSouthWest.move_board(Square::G7.into_bitboard()),
            Square::F5.into_bitboard()
        );
    }

    #[test]
    fn south_south_east() {
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::B2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::G2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::A8.into_bitboard()),
            Square::B6.into_bitboard()
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::B7.into_bitboard()),
            Square::C5.into_bitboard()
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthSouthEast.move_board(Square::G7.into_bitboard()),
            Square::H5.into_bitboard()
        );
    }

    #[test]
    fn south_east_east() {
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::A1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::B2.into_bitboard()),
            Square::D1.into_bitboard()
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::G2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::A8.into_bitboard()),
            Square::C7.into_bitboard()
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::B7.into_bitboard()),
            Square::D6.into_bitboard()
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::SouthEastEast.move_board(Square::G7.into_bitboard()),
            Bitboard::EMPTY,
        );
    }

    #[test]
    fn north_east_east() {
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::A1.into_bitboard()),
            Square::C2.into_bitboard()
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::B2.into_bitboard()),
            Square::D3.into_bitboard()
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::G2.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::B7.into_bitboard()),
            Square::D8.into_bitboard()
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthEastEast.move_board(Square::G7.into_bitboard()),
            Bitboard::EMPTY,
        );
    }

    #[test]
    fn north_north_east() {
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::A1.into_bitboard()),
            Square::B3.into_bitboard()
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::B2.into_bitboard()),
            Square::C4.into_bitboard()
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::H1.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::G2.into_bitboard()),
            Square::H4.into_bitboard()
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::A8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::B7.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::H8.into_bitboard()),
            Bitboard::EMPTY,
        );
        assert_eq!(
            Direction::NorthNorthEast.move_board(Square::G7.into_bitboard()),
            Bitboard::EMPTY,
        );
    }

    #[test]
    fn slide() {
        assert_eq!(
            Direction::North.slide_square(Square::A1),
            File::A.into_bitboard() - Square::A1
        );
        assert_eq!(
            Direction::West.slide_square(Square::H1),
            Rank::First.into_bitboard() - Square::H1
        );
        assert_eq!(
            Direction::South.slide_square(Square::A8),
            File::A.into_bitboard() - Square::A8
        );
        assert_eq!(
            Direction::East.slide_square(Square::A1),
            Rank::First.into_bitboard() - Square::A1
        );
        assert_eq!(
            Direction::NorthWest.slide_square(Square::H1),
            Bitboard::ANTI_DIAGONAL - Square::H1
        );
        assert_eq!(
            Direction::SouthWest.slide_square(Square::H8),
            Bitboard::DIAGONAL - Square::H8
        );
        assert_eq!(
            Direction::SouthEast.slide_square(Square::A8),
            Bitboard::ANTI_DIAGONAL - Square::A8
        );
        assert_eq!(
            Direction::NorthEast.slide_square(Square::A1),
            Bitboard::DIAGONAL - Square::A1
        );
    }

    #[test]
    fn blocked_slides() {
        assert_eq!(
            Direction::North
                .slide_board_with_blockers(Square::A1.into_bitboard(), Square::A2.into_bitboard()),
            Square::A2.into_bitboard()
        );
        assert_eq!(
            Direction::North
                .slide_board_with_blockers(Square::A1.into_bitboard(), Square::A3.into_bitboard()),
            Square::A2 | Square::A3
        );
        assert_eq!(
            Direction::North
                .slide_board_with_blockers(Square::A1.into_bitboard(), Square::A4.into_bitboard()),
            Square::A2 | Square::A3 | Square::A4
        );
        // Ensure that the starting square being in `blockers` is not an issue
        assert_eq!(
            Direction::North
                .slide_board_with_blockers(Square::A1.into_bitboard(), Square::A1.into_bitboard()),
            File::A.into_bitboard() - Square::A1
        );
    }
}
