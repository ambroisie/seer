mod generation;
pub(crate) use generation::*;
mod mask;

use crate::board::{Bitboard, Square};

/// A type representing the magic board indexing a given [crate::board::Square].
#[derive(Clone, Debug)]
pub(crate) struct Magic {
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
    /// Compute the index into the magics database for this set of `blockers`.
    pub fn get_index(&self, blockers: Bitboard) -> usize {
        let relevant_occupancy = (blockers & self.mask).0;
        let base_index = ((relevant_occupancy.wrapping_mul(self.magic)) >> self.shift) as usize;
        base_index + self.offset
    }
}

/// A type encapsulating a database of [Magic] bitboard moves.
#[derive(Clone, Debug)]
#[allow(unused)] // FIXME: remove when used
pub(crate) struct MagicMoves {
    magics: Vec<Magic>,
    moves: Vec<Bitboard>,
}

#[allow(unused)] // FIXME: remove when used
impl MagicMoves {
    /// Initialize a new [MagicMoves] given a matching list of [Magic] and its corresponding moves
    /// as a [Bitboard].
    ///
    /// # Safety
    ///
    /// This should only be called with values generated by [crate::movegen::wizardry::generation].
    pub unsafe fn new(magics: Vec<Magic>, moves: Vec<Bitboard>) -> Self {
        Self { magics, moves }
    }

    /// Get the set of valid moves for a piece standing on a [Square], given a set of blockers.
    pub fn query(&self, square: Square, blockers: Bitboard) -> Bitboard {
        // SAFETY: indices are in range by construction
        unsafe {
            let index = self
                .magics
                .get_unchecked(square.index())
                .get_index(blockers);
            *self.moves.get_unchecked(index)
        }
    }
}
