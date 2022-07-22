use crate::board::{Bitboard, Direction, Square};

#[allow(unused)]
pub fn knight_moves(square: Square) -> Bitboard {
    let board = square.into_bitboard();

    Direction::iter_knight()
        .map(|dir| dir.move_board(board))
        .fold(Bitboard::EMPTY, |lhs, rhs| lhs | rhs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moves_first_rank() {
        assert_eq!(knight_moves(Square::A1), Square::B3 | Square::C2);
        assert_eq!(
            knight_moves(Square::B1),
            Square::A3 | Square::C3 | Square::D2
        );
        assert_eq!(
            knight_moves(Square::C1),
            Square::A2 | Square::B3 | Square::D3 | Square::E2
        );
        assert_eq!(
            knight_moves(Square::D1),
            Square::B2 | Square::C3 | Square::E3 | Square::F2
        );
        assert_eq!(
            knight_moves(Square::E1),
            Square::C2 | Square::D3 | Square::F3 | Square::G2
        );
        assert_eq!(
            knight_moves(Square::F1),
            Square::D2 | Square::E3 | Square::G3 | Square::H2
        );
        assert_eq!(
            knight_moves(Square::G1),
            Square::E2 | Square::F3 | Square::H3
        );
        assert_eq!(knight_moves(Square::H1), Square::F2 | Square::G3);
    }

    #[test]
    fn moves_last_rank() {
        assert_eq!(knight_moves(Square::A8), Square::B6 | Square::C7);
        assert_eq!(
            knight_moves(Square::B8),
            Square::A6 | Square::C6 | Square::D7
        );
        assert_eq!(
            knight_moves(Square::C8),
            Square::A7 | Square::B6 | Square::D6 | Square::E7
        );
        assert_eq!(
            knight_moves(Square::D8),
            Square::B7 | Square::C6 | Square::E6 | Square::F7
        );
        assert_eq!(
            knight_moves(Square::E8),
            Square::C7 | Square::D6 | Square::F6 | Square::G7
        );
        assert_eq!(
            knight_moves(Square::F8),
            Square::D7 | Square::E6 | Square::G6 | Square::H7
        );
        assert_eq!(
            knight_moves(Square::G8),
            Square::E7 | Square::F6 | Square::H6
        );
        assert_eq!(knight_moves(Square::H8), Square::F7 | Square::G6);
    }

    #[test]
    fn moves_first_file() {
        assert_eq!(knight_moves(Square::A1), Square::B3 | Square::C2);
        assert_eq!(
            knight_moves(Square::A2),
            Square::B4 | Square::C1 | Square::C3
        );
        assert_eq!(
            knight_moves(Square::A3),
            Square::B1 | Square::B5 | Square::C2 | Square::C4
        );
        assert_eq!(
            knight_moves(Square::A4),
            Square::B2 | Square::B6 | Square::C3 | Square::C5
        );
        assert_eq!(
            knight_moves(Square::A5),
            Square::B3 | Square::B7 | Square::C4 | Square::C6
        );
        assert_eq!(
            knight_moves(Square::A6),
            Square::B4 | Square::B8 | Square::C5 | Square::C7
        );
        assert_eq!(
            knight_moves(Square::A7),
            Square::B5 | Square::C6 | Square::C8
        );
        assert_eq!(knight_moves(Square::A8), Square::B6 | Square::C7);
    }

    #[test]
    fn moves_last_file() {
        assert_eq!(knight_moves(Square::H1), Square::G3 | Square::F2);
        assert_eq!(
            knight_moves(Square::H2),
            Square::G4 | Square::F1 | Square::F3
        );
        assert_eq!(
            knight_moves(Square::H3),
            Square::G1 | Square::G5 | Square::F2 | Square::F4
        );
        assert_eq!(
            knight_moves(Square::H4),
            Square::G2 | Square::G6 | Square::F3 | Square::F5
        );
        assert_eq!(
            knight_moves(Square::H5),
            Square::G3 | Square::G7 | Square::F4 | Square::F6
        );
        assert_eq!(
            knight_moves(Square::H6),
            Square::G4 | Square::G8 | Square::F5 | Square::F7
        );
        assert_eq!(
            knight_moves(Square::H7),
            Square::G5 | Square::F6 | Square::F8
        );
        assert_eq!(knight_moves(Square::H8), Square::G6 | Square::F7);
    }

    #[test]
    fn moves_middle() {
        assert_eq!(
            knight_moves(Square::D4),
            Square::B3
                | Square::B5
                | Square::C2
                | Square::C6
                | Square::E2
                | Square::E6
                | Square::F3
                | Square::F5
        );
        assert_eq!(
            knight_moves(Square::D5),
            Square::B4
                | Square::B6
                | Square::C3
                | Square::C7
                | Square::E3
                | Square::E7
                | Square::F4
                | Square::F6
        );
        assert_eq!(
            knight_moves(Square::E4),
            Square::C3
                | Square::C5
                | Square::D2
                | Square::D6
                | Square::F2
                | Square::F6
                | Square::G3
                | Square::G5
        );
        assert_eq!(
            knight_moves(Square::E5),
            Square::C4
                | Square::C6
                | Square::D3
                | Square::D7
                | Square::F3
                | Square::F7
                | Square::G4
                | Square::G6
        );
    }
}
