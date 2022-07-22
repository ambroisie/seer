use crate::board::{Bitboard, CastleRights, Color, Direction, File, Square};

/// Compute a king's movement. No castling moves included
#[allow(unused)]
pub fn king_moves(square: Square) -> Bitboard {
    let board = square.into_bitboard();

    Direction::iter_royalty()
        .map(|dir| dir.move_board(board))
        .fold(Bitboard::EMPTY, |lhs, rhs| lhs | rhs)
}

/// Compute a king's castling moves, given its [Color] and [CastleRights].
#[allow(unused)]
pub fn king_castling_moves(color: Color, castle_rights: CastleRights) -> Bitboard {
    let rank = color.first_rank();

    let king_side_square = Square::new(File::G, rank);
    let queen_side_square = Square::new(File::C, rank);

    match castle_rights {
        CastleRights::NoSide => Bitboard::EMPTY,
        CastleRights::KingSide => king_side_square.into_bitboard(),
        CastleRights::QueenSide => queen_side_square.into_bitboard(),
        CastleRights::BothSides => king_side_square | queen_side_square,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moves_first_rank() {
        assert_eq!(king_moves(Square::A1), Square::A2 | Square::B1 | Square::B2);
        assert_eq!(
            king_moves(Square::B1),
            Square::A1 | Square::A2 | Square::B2 | Square::C1 | Square::C2
        );
        assert_eq!(
            king_moves(Square::C1),
            Square::B1 | Square::B2 | Square::C2 | Square::D1 | Square::D2
        );
        assert_eq!(
            king_moves(Square::D1),
            Square::C1 | Square::C2 | Square::D2 | Square::E1 | Square::E2
        );
        assert_eq!(
            king_moves(Square::E1),
            Square::D1 | Square::D2 | Square::E2 | Square::F1 | Square::F2
        );
        assert_eq!(
            king_moves(Square::F1),
            Square::E1 | Square::E2 | Square::F2 | Square::G1 | Square::G2
        );
        assert_eq!(
            king_moves(Square::G1),
            Square::F1 | Square::F2 | Square::G2 | Square::H1 | Square::H2
        );
        assert_eq!(king_moves(Square::H1), Square::G1 | Square::G2 | Square::H2);
    }

    #[test]
    fn moves_last_rank() {
        assert_eq!(king_moves(Square::A8), Square::A7 | Square::B8 | Square::B7);
        assert_eq!(
            king_moves(Square::B8),
            Square::A8 | Square::A7 | Square::B7 | Square::C8 | Square::C7
        );
        assert_eq!(
            king_moves(Square::C8),
            Square::B8 | Square::B7 | Square::C7 | Square::D8 | Square::D7
        );
        assert_eq!(
            king_moves(Square::D8),
            Square::C8 | Square::C7 | Square::D7 | Square::E8 | Square::E7
        );
        assert_eq!(
            king_moves(Square::E8),
            Square::D8 | Square::D7 | Square::E7 | Square::F8 | Square::F7
        );
        assert_eq!(
            king_moves(Square::F8),
            Square::E8 | Square::E7 | Square::F7 | Square::G8 | Square::G7
        );
        assert_eq!(
            king_moves(Square::G8),
            Square::F8 | Square::F7 | Square::G7 | Square::H8 | Square::H7
        );
        assert_eq!(king_moves(Square::H8), Square::G8 | Square::G7 | Square::H7);
    }

    #[test]
    fn moves_first_file() {
        assert_eq!(king_moves(Square::A1), Square::A2 | Square::B1 | Square::B2);
        assert_eq!(
            king_moves(Square::A2),
            Square::A1 | Square::A3 | Square::B1 | Square::B2 | Square::B3
        );
        assert_eq!(
            king_moves(Square::A3),
            Square::A2 | Square::A4 | Square::B2 | Square::B3 | Square::B4
        );
        assert_eq!(
            king_moves(Square::A4),
            Square::A3 | Square::A5 | Square::B3 | Square::B4 | Square::B5
        );
        assert_eq!(
            king_moves(Square::A5),
            Square::A4 | Square::A6 | Square::B4 | Square::B5 | Square::B6
        );
        assert_eq!(
            king_moves(Square::A6),
            Square::A5 | Square::A7 | Square::B5 | Square::B6 | Square::B7
        );
        assert_eq!(
            king_moves(Square::A7),
            Square::A6 | Square::A8 | Square::B6 | Square::B7 | Square::B8
        );
        assert_eq!(king_moves(Square::A8), Square::A7 | Square::B7 | Square::B8);
    }

    #[test]
    fn moves_last_file() {
        assert_eq!(king_moves(Square::H1), Square::H2 | Square::G1 | Square::G2);
        assert_eq!(
            king_moves(Square::H2),
            Square::H1 | Square::H3 | Square::G1 | Square::G2 | Square::G3
        );
        assert_eq!(
            king_moves(Square::H3),
            Square::H2 | Square::H4 | Square::G2 | Square::G3 | Square::G4
        );
        assert_eq!(
            king_moves(Square::H4),
            Square::H3 | Square::H5 | Square::G3 | Square::G4 | Square::G5
        );
        assert_eq!(
            king_moves(Square::H5),
            Square::H4 | Square::H6 | Square::G4 | Square::G5 | Square::G6
        );
        assert_eq!(
            king_moves(Square::H6),
            Square::H5 | Square::H7 | Square::G5 | Square::G6 | Square::G7
        );
        assert_eq!(
            king_moves(Square::H7),
            Square::H6 | Square::H8 | Square::G6 | Square::G7 | Square::G8
        );
        assert_eq!(king_moves(Square::H8), Square::H7 | Square::G7 | Square::G8);
    }

    #[test]
    fn moves_middle() {
        assert_eq!(
            king_moves(Square::D4),
            Square::C3
                | Square::C4
                | Square::C5
                | Square::D3
                | Square::D5
                | Square::E3
                | Square::E4
                | Square::E5
        );
        assert_eq!(
            king_moves(Square::D5),
            Square::C4
                | Square::C5
                | Square::C6
                | Square::D4
                | Square::D6
                | Square::E4
                | Square::E5
                | Square::E6
        );
        assert_eq!(
            king_moves(Square::E5),
            Square::D4
                | Square::D5
                | Square::D6
                | Square::E4
                | Square::E6
                | Square::F4
                | Square::F5
                | Square::F6
        );
    }

    #[test]
    fn castling_moves() {
        assert_eq!(
            king_castling_moves(Color::White, CastleRights::NoSide),
            Bitboard::EMPTY
        );
        assert_eq!(
            king_castling_moves(Color::Black, CastleRights::NoSide),
            Bitboard::EMPTY
        );
        assert_eq!(
            king_castling_moves(Color::White, CastleRights::KingSide),
            Square::G1.into_bitboard()
        );
        assert_eq!(
            king_castling_moves(Color::Black, CastleRights::KingSide),
            Square::G8.into_bitboard()
        );
        assert_eq!(
            king_castling_moves(Color::White, CastleRights::QueenSide),
            Square::C1.into_bitboard()
        );
        assert_eq!(
            king_castling_moves(Color::Black, CastleRights::QueenSide),
            Square::C8.into_bitboard()
        );
        assert_eq!(
            king_castling_moves(Color::White, CastleRights::BothSides),
            Square::C1 | Square::G1
        );
        assert_eq!(
            king_castling_moves(Color::Black, CastleRights::BothSides),
            Square::C8 | Square::G8
        );
    }
}
