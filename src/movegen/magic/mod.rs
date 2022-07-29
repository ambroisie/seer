use crate::board::Bitboard;

/// A type representing the magic board indexing a given [crate::board::Square].
pub struct Magic {
    /// Magic number.
    pub(crate) magic: u64,
    /// Base offset into the magic square table.
    pub(crate) offset: usize,
    /// Mask to apply to the blocker board before applying the magic.
    pub(crate) mask: Bitboard,
    /// Length of the resulting mask after applying the magic.
    pub(crate) shift: u8,
}

impl Magic {
    pub fn get_index(&self, blockers: Bitboard) -> usize {
        let relevant_occupancy = (blockers & self.mask).0;
        let base_index = ((relevant_occupancy.wrapping_mul(self.magic)) >> self.shift) as usize;
        base_index + self.offset
    }
}

#[cfg(generated_boards)]
mod moves;
pub use moves::*;

#[cfg(not(generated_boards))]
#[allow(unused_variables)]
mod moves {
    use crate::board::{Bitboard, Color, Square};

    pub fn quiet_pawn_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
        unreachable!()
    }

    pub fn pawn_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
        unreachable!()
    }

    pub fn knight_moves(square: Square) -> Bitboard {
        unreachable!()
    }

    pub fn bishop_moves(square: Square, blockers: Bitboard) -> Bitboard {
        unreachable!()
    }

    pub fn rook_moves(square: Square, blockers: Bitboard) -> Bitboard {
        unreachable!()
    }

    pub fn queen_moves(square: Square, blockers: Bitboard) -> Bitboard {
        unreachable!()
    }

    pub fn king_moves(square: Square) -> Bitboard {
        unreachable!()
    }

    pub fn king_side_castle_blockers(color: Color) -> Bitboard {
        unreachable!()
    }

    pub fn queen_side_castle_blockers(color: Color) -> Bitboard {
        unreachable!()
    }
}
