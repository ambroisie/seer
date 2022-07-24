use crate::board::{Bitboard, Direction, Square};

/// Compute a bishop's movement given a set of blockers that cannot be moved past.
pub fn bishop_moves(square: Square, blockers: Bitboard) -> Bitboard {
    Direction::iter_bishop()
        .map(|dir| dir.slide_board_with_blockers(square.into_bitboard(), blockers))
        .fold(Bitboard::EMPTY, |lhs, rhs| lhs | rhs)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{File, Rank};

    #[test]
    fn moves_lower_left_square() {
        assert_eq!(
            bishop_moves(Square::A1, Bitboard::EMPTY),
            Bitboard::DIAGONAL - Square::A1
        );
        assert_eq!(
            bishop_moves(Square::A1, Bitboard::ALL),
            Square::B2.into_bitboard()
        );
        assert_eq!(
            bishop_moves(Square::A1, Square::D4.into_bitboard()),
            Square::B2 | Square::C3 | Square::D4
        );
        assert_eq!(
            bishop_moves(Square::A1, File::D.into_bitboard()),
            Square::B2 | Square::C3 | Square::D4
        );
    }

    #[test]
    fn moves_middle() {
        let cross = Bitboard::DIAGONAL | Direction::South.move_board(Bitboard::ANTI_DIAGONAL);
        assert_eq!(
            bishop_moves(Square::D4, Bitboard::EMPTY),
            cross - Square::D4
        );
        assert_eq!(
            bishop_moves(Square::D4, Bitboard::ALL),
            Square::C3 | Square::C5 | Square::E3 | Square::E5
        );
        assert_eq!(
            bishop_moves(Square::D4, Rank::Fifth.into_bitboard()),
            Square::A1
                | Square::B2
                | Square::C3
                | Square::C5
                | Square::E3
                | Square::E5
                | Square::F2
                | Square::G1
        );
        assert_eq!(
            bishop_moves(Square::D4, File::E.into_bitboard()),
            Square::A1
                | Square::A7
                | Square::B2
                | Square::B6
                | Square::C3
                | Square::C5
                | Square::E3
                | Square::E5
        );
    }
}
