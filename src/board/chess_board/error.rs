/// A singular type for all errors that could happen during [ChessBoard::is_valid].
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InvalidError {
    /// Too many pieces.
    TooManyPieces,
    /// Missing king.
    MissingKing,
    /// Pawns on the first/last rank.
    InvalidPawnPosition,
    /// Castling rights do not match up with the state of the board.
    InvalidCastlingRights,
    /// En-passant target square is not empty and behind an opponent's pawn.
    InvalidEnPassant,
    /// The two kings are next to each other.
    NeighbouringKings,
    /// The opponent is currently in check.
    OpponentInCheck,
    /// The piece-specific boards are overlapping.
    OverlappingPieces,
    /// The color-specific boards are overlapping.
    OverlappingColors,
    /// The pre-computed combined occupancy boards does not match the other boards.
    ErroneousCombinedOccupancy,
}

impl std::fmt::Display for InvalidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            Self::TooManyPieces => "Too many pieces.",
            Self::MissingKing => "Missing king.",
            Self::InvalidPawnPosition => "Pawns on the first/last rank.",
            Self::InvalidCastlingRights => {
                "Castling rights do not match up with the state of the board."
            }
            Self::InvalidEnPassant => {
                "En-passant target square is not empty and behind an opponent's pawn."
            }
            Self::NeighbouringKings => "The two kings are next to each other.",
            Self::OpponentInCheck => "The opponent is currently in check.",
            Self::OverlappingPieces => "The piece-specific boards are overlapping.",
            Self::OverlappingColors => "The color-specific boards are overlapping.",
            Self::ErroneousCombinedOccupancy => {
                "The pre-computed combined occupancy boards does not match the other boards."
            }
        };
        write!(f, "{}", error_msg)
    }
}

impl std::error::Error for InvalidError {}
