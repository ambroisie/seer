use super::Bitboard;

/// Iterator over a [Bitboard] mask, which yields all potential subsets of the given board.
/// In other words, for each square that belongs to the mask, this will yield all sets that do
/// contain the square, and all sets that do not.
pub struct BitboardPowerSetIterator {
    /// The starting board.
    board: Bitboard,
    /// The next subset.
    subset: Bitboard,
    /// Whether or not iteration is done.
    done: bool,
}

impl BitboardPowerSetIterator {
    pub fn new(board: Bitboard) -> Self {
        Self {
            board,
            subset: Bitboard::EMPTY,
            done: false,
        }
    }
}

impl Iterator for BitboardPowerSetIterator {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let res = self.subset;
        self.subset = Bitboard(self.subset.0.wrapping_sub(self.board.0)) & self.board;
        self.done = self.subset.is_empty();
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = 1 << self.board.count();
        (size, Some(size))
    }
}

impl ExactSizeIterator for BitboardPowerSetIterator {}

impl std::iter::FusedIterator for BitboardPowerSetIterator {}
