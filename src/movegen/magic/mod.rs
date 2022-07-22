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
    /// Compute the index into the magics database for this set of `blockers`.
    pub fn get_index(&self, blockers: Bitboard) -> usize {
        let relevant_occupancy = (blockers & self.mask).0;
        let base_index = ((relevant_occupancy.wrapping_mul(self.magic)) >> self.shift) as usize;
        base_index + self.offset
    }
}
