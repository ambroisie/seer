use crate::board::{Bitboard, Direction, Square};

/// Compute a rook's movement given a set of blockers that cannot be moved past.
#[allow(unused)]
pub fn rook_moves(square: Square, blockers: Bitboard) -> Bitboard {
    Direction::iter_rook()
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
            rook_moves(Square::A1, Bitboard::EMPTY),
            (File::A.into_bitboard() | Rank::First.into_bitboard()) - Square::A1
        );
        assert_eq!(
            rook_moves(Square::A1, Bitboard::ALL),
            Square::A2 | Square::B1
        );
        assert_eq!(
            rook_moves(Square::A1, Rank::First.into_bitboard()),
            (File::A.into_bitboard() | Square::B1) - Square::A1
        );
        assert_eq!(
            rook_moves(Square::A1, File::A.into_bitboard()),
            (Rank::First.into_bitboard() | Square::A2) - Square::A1
        );
    }

    #[test]
    fn moves_middle() {
        assert_eq!(
            rook_moves(Square::D4, Bitboard::EMPTY),
            (File::D.into_bitboard() | Rank::Fourth.into_bitboard()) - Square::D4
        );
        assert_eq!(
            rook_moves(Square::D4, Bitboard::ALL),
            Square::C4 | Square::D3 | Square::D5 | Square::E4
        );
        assert_eq!(
            rook_moves(Square::D4, Rank::Fourth.into_bitboard()),
            (File::D.into_bitboard() | Square::C4 | Square::E4) - Square::D4
        );
        assert_eq!(
            rook_moves(Square::D4, File::D.into_bitboard()),
            (Rank::Fourth.into_bitboard() | Square::D3 | Square::D5) - Square::D4
        );
    }
}
