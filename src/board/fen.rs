/// A trait to mark items that can be converted from a FEN input.
pub trait FromFen: Sized {
    type Err;

    fn from_fen(s: &str) -> Result<Self, Self::Err>;
}
