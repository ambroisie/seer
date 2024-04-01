use crate::board::{
    CastleRights, ChessBoard, ChessBoardBuilder, Color, File, InvalidError, Piece, Rank, Square,
};

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

/// Allow converting a [InvalidError] into [FenError], for use with the '?' operator.
impl From<InvalidError> for FenError {
    fn from(err: InvalidError) -> Self {
        Self::InvalidPosition(err)
    }
}

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

/// Return a [ChessBoard] from the given FEN string.
impl FromFen for ChessBoard {
    type Err = FenError;

    fn from_fen(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let piece_placement = split.next().ok_or(FenError::InvalidFen)?;
        let side_to_move = split.next().ok_or(FenError::InvalidFen)?;
        let castling_rights = split.next().ok_or(FenError::InvalidFen)?;
        let en_passant_square = split.next().ok_or(FenError::InvalidFen)?;
        let half_move_clock = split.next().ok_or(FenError::InvalidFen)?;
        let full_move_counter = split.next().ok_or(FenError::InvalidFen)?;

        let mut builder = ChessBoardBuilder::new();

        let castle_rights = <[CastleRights; 2]>::from_fen(castling_rights)?;
        for color in Color::iter() {
            builder.with_castle_rights(castle_rights[color.index()], color);
        }

        let side = Color::from_fen(side_to_move)?;
        builder.with_current_player(side);

        if let Some(square) = Option::<Square>::from_fen(en_passant_square)? {
            builder.with_en_passant(square);
        };

        let half_move_clock = half_move_clock
            .parse::<u8>()
            .map_err(|_| FenError::InvalidFen)?;
        builder.with_half_move_clock(half_move_clock);

        let full_move_counter = full_move_counter
            .parse::<u32>()
            .map_err(|_| FenError::InvalidFen)?;
        builder.with_total_plies(
            (full_move_counter - 1) * 2 + if side == Color::White { 0 } else { 1 },
        );

        {
            let mut rank: usize = 8;
            for rank_str in piece_placement.split('/') {
                rank -= 1;
                let mut file: usize = 0;
                for c in rank_str.chars() {
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let piece = match c {
                        digit @ '1'..='8' => {
                            // Unwrap is fine since this arm is only matched by digits
                            file += digit.to_digit(10).unwrap() as usize;
                            continue;
                        }
                        _ => Piece::from_fen(&c.to_string())?,
                    };

                    // Only need to worry about underflow since those are `usize` values.
                    if file >= 8 || rank >= 8 {
                        return Err(FenError::InvalidFen);
                    };

                    let square = Square::new(File::from_index(file), Rank::from_index(rank));

                    builder[square] = Some((piece, color));
                    file += 1;
                }
                // We haven't read exactly 8 files.
                if file != 8 {
                    return Err(FenError::InvalidFen);
                }
            }
            // We haven't read exactly 8 ranks
            if rank != 0 {
                return Err(FenError::InvalidFen);
            }
        };

        Ok(builder.try_into()?)
    }
}
