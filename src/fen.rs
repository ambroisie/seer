/// A trait to mark items that can be converted from a FEN input.
pub trait FromFen: Sized {
    type Err;

    fn from_fen(s: &str) -> Result<Self, Self::Err>;
}

/// A singular type for all errors that could happen during FEN parsing.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FenError {
    /// Invalid FEN input.
    InvalidFen,
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            Self::InvalidFen => "Invalid FEN input",
        };
        write!(f, "{}", error_msg)
    }
}

impl std::error::Error for FenError {}
