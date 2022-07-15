/// An [Iterator](std::iter::Iterator) of [Square](crate::board::Square) contained in a
/// [Bitboard](crate::board::Bitboard).
pub struct BitboardIterator(pub(crate) u64);

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
}
