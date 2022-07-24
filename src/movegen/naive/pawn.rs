use crate::board::{Bitboard, Color, Direction, Rank, Square};

/// Compute a pawn's movement given its color, and a set of blockers that cannot be moved past.
pub fn pawn_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    if (square.rank() == Rank::First) || (square.rank() == Rank::Eighth) {
        return Bitboard::EMPTY;
    }

    let dir = color.forward_direction();

    let first_push = dir.move_board(square.into_bitboard());
    let second_push = if square.rank() == color.second_rank() {
        Square::new(square.file(), color.fourth_rank()).into_bitboard()
    } else {
        Bitboard::EMPTY
    };

    if (first_push & blockers).is_empty() {
        first_push | second_push
    } else {
        Bitboard::EMPTY
    }
}

/// Computes the set of squares a pawn can capture, given its color.
pub fn pawn_captures(color: Color, square: Square) -> Bitboard {
    if (square.rank() == Rank::First) || (square.rank() == Rank::Eighth) {
        return Bitboard::EMPTY;
    }

    let dir = color.forward_direction();

    let advanced = dir.move_board(square.into_bitboard());

    let attack_west = Direction::West.move_board(advanced);
    let attack_east = Direction::East.move_board(advanced);

    attack_west | attack_east
}

/// Computes the set of squares that can capture this one *en-passant*.
#[allow(unused)]
pub fn en_passant_origins(square: Square) -> Bitboard {
    let board = square.into_bitboard();

    let origin_west = Direction::West.move_board(board);
    let origin_east = Direction::East.move_board(board);

    origin_west | origin_east
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moves() {
        assert_eq!(
            pawn_moves(Color::White, Square::A2, Bitboard::EMPTY),
            Square::A3 | Square::A4
        );
        assert_eq!(
            pawn_moves(Color::Black, Square::A7, Bitboard::EMPTY),
            Square::A5 | Square::A6
        );
        assert_eq!(
            pawn_moves(Color::Black, Square::A2, Bitboard::EMPTY),
            Square::A1.into_bitboard()
        );
        assert_eq!(
            pawn_moves(Color::White, Square::A7, Bitboard::EMPTY),
            Square::A8.into_bitboard()
        );
    }

    #[test]
    fn captures() {
        assert_eq!(
            pawn_captures(Color::White, Square::A2),
            Square::B3.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::White, Square::B2),
            Square::A3 | Square::C3
        );
        assert_eq!(
            pawn_captures(Color::White, Square::H2),
            Square::G3.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::A2),
            Square::B1.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::B2),
            Square::A1 | Square::C1
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::H2),
            Square::G1.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::White, Square::A7),
            Square::B8.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::White, Square::B7),
            Square::A8 | Square::C8
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::H7),
            Square::G6.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::A7),
            Square::B6.into_bitboard()
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::B7),
            Square::A6 | Square::C6
        );
        assert_eq!(
            pawn_captures(Color::Black, Square::H7),
            Square::G6.into_bitboard()
        );
    }

    #[test]
    fn en_passant() {
        assert_eq!(en_passant_origins(Square::A4), Square::B4.into_bitboard());
        assert_eq!(en_passant_origins(Square::A5), Square::B5.into_bitboard());
        assert_eq!(en_passant_origins(Square::B4), Square::A4 | Square::C4);
        assert_eq!(en_passant_origins(Square::B5), Square::A5 | Square::C5);
        assert_eq!(en_passant_origins(Square::H4), Square::G4.into_bitboard());
        assert_eq!(en_passant_origins(Square::H5), Square::G5.into_bitboard());
    }
}
