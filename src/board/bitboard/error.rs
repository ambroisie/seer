#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntoSquareError {
    /// The board is empty.
    EmptyBoard,
    /// The board contains more than one square.
    TooManySquares,
}

impl std::fmt::Display for IntoSquareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            Self::EmptyBoard => "The board is empty",
            Self::TooManySquares => "The board contains more than one square",
        };
        write!(f, "{}", error_msg)
    }
}

impl std::error::Error for IntoSquareError {}
