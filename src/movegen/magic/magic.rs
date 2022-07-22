use crate::board::Bitboard;

/// A type representing the magic board indexing a given [Square].
#[allow(unused)] // FIXME: remove once used
pub struct Magic {
    /// Magic number.
    magic: u64,
    /// Base offset into the magic square table.
    offset: usize,
    /// Mask to apply to the blocker board before applying the magic.
    mask: Bitboard,
    /// Length of the resulting mask after applying the magic.
    shift: u8,
}

impl Magic {
    #[allow(unused)] // FIXME: remove once used
    pub fn get_index(&self, blockers: Bitboard) -> usize {
        let relevant_occupancy = (blockers & self.mask).0;
        let base_index = ((relevant_occupancy.wrapping_mul(self.magic)) >> self.shift) as usize;
        base_index + self.offset
    }
}
