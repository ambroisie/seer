use super::Magic;
use crate::board::{Bitboard, Color, Square};

include!(concat!(env!("OUT_DIR"), "/magic_tables.rs"));

pub fn quiet_pawn_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    // If there is a piece in front of the pawn, it can't advance
    if !(color.backward_direction().move_board(blockers) & square).is_empty() {
        return Bitboard::EMPTY;
    }
    // SAFETY: we know the values are in-bounds
    unsafe {
        *PAWN_MOVES
            .get_unchecked(color.index())
            .get_unchecked(square.index())
    }
}

pub fn pawn_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    let attacks = unsafe {
        *PAWN_ATTACKS
            .get_unchecked(color.index())
            .get_unchecked(square.index())
    };
    quiet_pawn_moves(color, square, blockers) | attacks
}

pub fn knight_moves(square: Square) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    unsafe { *KNIGHT_MOVES.get_unchecked(square.index()) }
}

pub fn bishop_moves(square: Square, blockers: Bitboard) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    unsafe {
        let index = BISHOP_MAGICS
            .get_unchecked(square.index())
            .get_index(blockers);
        *BISHOP_MOVES.get_unchecked(index)
    }
}

pub fn rook_moves(square: Square, blockers: Bitboard) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    unsafe {
        let index = ROOK_MAGICS
            .get_unchecked(square.index())
            .get_index(blockers);
        *ROOK_MOVES.get_unchecked(index)
    }
}

pub fn queen_moves(square: Square, blockers: Bitboard) -> Bitboard {
    bishop_moves(square, blockers) | rook_moves(square, blockers)
}

pub fn king_moves(square: Square) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    unsafe { *KING_MOVES.get_unchecked(square.index()) }
}

pub fn king_side_castle_blockers(color: Color) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    unsafe { *KING_SIDE_CASTLE_BLOCKERS.get_unchecked(color.index()) }
}

pub fn queen_side_castle_blockers(color: Color) -> Bitboard {
    // SAFETY: we know the values are in-bounds
    unsafe { *QUEEN_SIDE_CASTLE_BLOCKERS.get_unchecked(color.index()) }
}
