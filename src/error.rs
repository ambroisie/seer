/// A singular type for all errors that could happen when using this library.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    InvalidFen,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            Self::InvalidFen => "Invalid FEN input",
        };
        write!(f, "{}", error_msg)
    }
}

impl std::error::Error for Error {}
