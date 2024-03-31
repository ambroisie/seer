/// An [Iterator](std::iter::Iterator) of [Square](crate::board::Square) contained in a
/// [Bitboard].
use crate::board::Bitboard;

pub struct BitboardIterator(u64);

impl BitboardIterator {
    pub fn new(board: Bitboard) -> Self {
        Self(board.0)
    }
}

impl Iterator for BitboardIterator {
    type Item = crate::board::Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let lsb = self.0.trailing_zeros() as usize;
            self.0 ^= 1 << lsb;
            Some(crate::board::Square::from_index(lsb))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.count_ones() as usize;

        (size, Some(size))
    }
}

impl ExactSizeIterator for BitboardIterator {}

impl std::iter::FusedIterator for BitboardIterator {}
