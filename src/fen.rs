use crate::board::{CastleRights, Color, File, InvalidError, Piece, Rank, Square};

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
    /// Invalid chess position.
    InvalidPosition(InvalidError),
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFen => write!(f, "Invalid FEN input"),
            Self::InvalidPosition(err) => write!(f, "Invalid chess position: {}", err),
        }
    }
}

impl std::error::Error for FenError {}

/// Convert the castling rights segment of a FEN string to an array of [CastleRights].
impl FromFen for [CastleRights; 2] {
    type Err = FenError;

    fn from_fen(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err(FenError::InvalidFen);
        }

        let mut res = [CastleRights::NoSide; 2];

        if s == "-" {
            return Ok(res);
        }

        for b in s.chars() {
            let color = if b.is_uppercase() {
                Color::White
            } else {
                Color::Black
            };
            let rights = &mut res[color.index()];
            match b {
                'k' | 'K' => *rights = rights.with_king_side(),
                'q' | 'Q' => *rights = rights.with_queen_side(),
                _ => return Err(FenError::InvalidFen),
            }
        }

        Ok(res)
    }
}

/// Convert a side to move segment of a FEN string to a [Color].
impl FromFen for Color {
    type Err = FenError;

    fn from_fen(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::InvalidFen),
        };
        Ok(res)
    }
}

/// Convert an en-passant target square segment of a FEN string to an optional [Square].
impl FromFen for Option<Square> {
    type Err = FenError;

    fn from_fen(s: &str) -> Result<Self, Self::Err> {
        let res = match s.as_bytes() {
            [b'-'] => None,
            [file @ b'a'..=b'h', rank @ b'1'..=b'8'] => Some(Square::new(
                File::from_index((file - b'a') as usize),
                Rank::from_index((rank - b'1') as usize),
            )),
            _ => return Err(FenError::InvalidFen),
        };
        Ok(res)
    }
}

/// Convert a piece in FEN notation to a [Piece].
impl FromFen for Piece {
    type Err = FenError;

    fn from_fen(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "p" | "P" => Self::Pawn,
            "n" | "N" => Self::Knight,
            "b" | "B" => Self::Bishop,
            "r" | "R" => Self::Rook,
            "q" | "Q" => Self::Queen,
            "k" | "K" => Self::King,
            _ => return Err(FenError::InvalidFen),
        };
        Ok(res)
    }
}
