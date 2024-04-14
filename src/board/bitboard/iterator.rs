/// An [Iterator](std::iter::Iterator) of [Square](crate::board::Square) contained in a
/// [Bitboard].
use crate::board::Bitboard;

pub struct BitboardIterator(Bitboard);

impl BitboardIterator {
    pub fn new(board: Bitboard) -> Self {
        Self(board)
    }
}

impl Iterator for BitboardIterator {
    type Item = crate::board::Square;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0.any_square();
        if let Some(square) = res {
            self.0 ^= square;
        };
        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.count() as usize;

        (size, Some(size))
    }
}

impl ExactSizeIterator for BitboardIterator {}

impl std::iter::FusedIterator for BitboardIterator {}
