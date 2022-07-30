use crate::{
    error::Error,
    movegen::{
        bishop_moves, knight_moves, magic::king_moves, naive::pawn::pawn_captures, rook_moves,
    },
};

use super::{Bitboard, CastleRights, Color, File, FromFen, Move, Piece, Rank, Square};

/// Represent an on-going chess game.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessBoard {
    /// A [Bitboard] of occupancy for each piece type, discarding color. Indexed by [Piece::index].
    piece_occupancy: [Bitboard; Piece::NUM_VARIANTS],
    /// A [Bitboard] of occupancy for each color, discarding piece type. Indexed by [Piece::index].
    color_occupancy: [Bitboard; Color::NUM_VARIANTS],
    /// A [Bitboard] representing all squares currently occupied by a piece.
    combined_occupancy: Bitboard,
    /// The allowed [CastleRights] for either color. Indexed by [Color::index].
    castle_rights: [CastleRights; Color::NUM_VARIANTS],
    /// A potential en-passant attack.
    /// Either `None` if no 2-square pawn move was made in the previous half-turn, or
    /// `Some(target_square)` if a 2-square move was made.
    en_passant: Option<Square>,
    /// The number of half-turns without either a pawn push or capture.
    half_move_clock: u8, // Should never go higher than 50.
    /// The number of half-turns so far.
    total_plies: u32, // Should be plenty.
    /// The current player turn.
    side: Color,
}

pub struct NonReversibleState {
    castle_rights: [CastleRights; Color::NUM_VARIANTS],
    en_passant: Option<Square>,
    half_move_clock: u8, // Should never go higher than 50.
}

impl ChessBoard {
    /// Which player's turn is it.
    #[inline(always)]
    pub fn current_player(&self) -> Color {
        self.side
    }

    /// Return the [Square] currently occupied by a pawn that can be captured en-passant, or `None`
    #[inline(always)]
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    /// Return the [CastleRights] for the given [Color].
    #[inline(always)]
    pub fn castle_rights(&self, color: Color) -> CastleRights {
        // SAFETY: we know the value is in-bounds
        unsafe { *self.castle_rights.get_unchecked(color.index()) }
    }

    /// Return the [CastleRights] for the given [Color]. Allow mutations.
    #[inline(always)]
    fn castle_rights_mut(&mut self, color: Color) -> &mut CastleRights {
        // SAFETY: we know the value is in-bounds
        unsafe { &mut *self.castle_rights.get_unchecked_mut(color.index()) }
    }

    /// Get the [Bitboard] representing all pieces of the given [Piece] type, discarding color.
    #[inline(always)]
    pub fn piece_occupancy(&self, piece: Piece) -> Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { *self.piece_occupancy.get_unchecked(piece.index()) }
    }

    /// Get the [Bitboard] representing all pieces of the given [Piece] type, discarding color.
    /// Allow mutating the state.
    #[inline(always)]
    fn piece_occupancy_mut(&mut self, piece: Piece) -> &mut Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { &mut *self.piece_occupancy.get_unchecked_mut(piece.index()) }
    }

    /// Get the [Bitboard] representing all colors of the given [Color] type, discarding piece
    /// type.
    #[inline(always)]
    pub fn color_occupancy(&self, color: Color) -> Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { *self.color_occupancy.get_unchecked(color.index()) }
    }

    /// Get the [Bitboard] representing all colors of the given [Color] type, discarding piece
    /// type. Allow mutating the state.
    #[inline(always)]
    fn color_occupancy_mut(&mut self, color: Color) -> &mut Bitboard {
        // SAFETY: we know the value is in-bounds
        unsafe { &mut *self.color_occupancy.get_unchecked_mut(color.index()) }
    }

    /// Get the [Bitboard] representing all pieces on the board.
    #[inline(always)]
    pub fn combined_occupancy(&self) -> Bitboard {
        self.combined_occupancy
    }

    /// Return the number of half-turns without either a pawn push or a capture.
    #[inline(always)]
    pub fn half_move_clock(&self) -> u8 {
        self.half_move_clock
    }

    /// Return the total number of plies (i.e: half-turns) played so far.
    #[inline(always)]
    pub fn total_plies(&self) -> u32 {
        self.total_plies
    }

    /// Return the [Bitboard] corresponding to all the opponent's pieces threatening the current
    /// player's king.
    #[inline(always)]
    pub fn checkers(&self) -> Bitboard {
        self.compute_checkers(self.current_player())
    }

    /// Quickly do and undo a move on the [Bitboard]s that are part of the [ChessBoard] state. Does
    /// not account for all non-revertible changes such as en-passant state or half-move clock.
    #[inline(always)]
    fn xor(&mut self, color: Color, piece: Piece, start_end: Bitboard) {
        *self.piece_occupancy_mut(piece) ^= start_end;
        *self.color_occupancy_mut(color) ^= start_end;
        self.combined_occupancy ^= start_end;
    }

    /// Play the given [Move], returning all non-revertible state (e.g: en-passant, etc...).
    #[inline(always)]
    pub fn do_move(&mut self, chess_move: Move) -> NonReversibleState {
        // Save non-revertible state
        let state = NonReversibleState {
            castle_rights: self.castle_rights,
            en_passant: self.en_passant,
            half_move_clock: self.half_move_clock,
        };

        // Non-revertible state modification
        if chess_move.capture().is_some() || chess_move.piece() == Piece::Pawn {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }
        if chess_move.is_double_step() {
            let target_square = Square::new(
                chess_move.destination().file(),
                self.current_player().third_rank(),
            );
            self.en_passant = Some(target_square);
        } else {
            self.en_passant = None;
        }
        if chess_move.is_castling() || chess_move.piece() == Piece::King {
            *self.castle_rights_mut(self.current_player()) = CastleRights::NoSide;
        }
        if chess_move.piece() == Piece::Rook {
            let castle_rights = self.castle_rights_mut(self.current_player());
            *castle_rights = match chess_move.start().file() {
                File::A => castle_rights.without_queen_side(),
                File::H => castle_rights.without_king_side(),
                _ => *castle_rights,
            }
        }

        // Revertible state modification
        self.xor(
            self.current_player(),
            chess_move.piece(),
            chess_move.start() | chess_move.destination(),
        );
        self.total_plies += 1;
        self.side = !self.side;

        state
    }

    /// Reverse the effect of playing the given [Move], and return to the given
    /// [NonReversibleState].
    #[inline(always)]
    pub fn undo_move(&mut self, chess_move: Move, state: NonReversibleState) {
        // Restore non-revertible state
        self.castle_rights = state.castle_rights;
        self.en_passant = state.en_passant;
        self.half_move_clock = state.half_move_clock;

        // Restore revertible state
        self.xor(
            // The move was applied at the turn *before* the current player
            !self.current_player(),
            chess_move.piece(),
            chess_move.start() | chess_move.destination(),
        );
        self.total_plies -= 1;
        self.side = !self.side;
    }

    /// Return true if the current state of the board looks valid, false if something is definitely
    /// wrong.
    fn is_valid(&self) -> bool {
        // Don't overlap pieces.
        for piece in Piece::iter() {
            #[allow(clippy::collapsible_if)]
            for other in Piece::iter() {
                if piece != other {
                    if !(self.piece_occupancy(piece) & self.piece_occupancy(other)).is_empty() {
                        return false;
                    }
                }
            }
        }

        // Don't overlap colors.
        if !(self.color_occupancy(Color::White) & self.color_occupancy(Color::Black)).is_empty() {
            return false;
        }

        // Calculate the union of all pieces.
        let combined =
            Piece::iter().fold(Bitboard::EMPTY, |board, p| board | self.piece_occupancy(p));

        // Ensure that the pre-computed version is accurate.
        if combined != self.combined_occupancy() {
            return false;
        }

        // Ensure that all pieces belong to a color, and no color has pieces that don't exist.
        if combined != (self.color_occupancy(Color::White) | self.color_occupancy(Color::Black)) {
            return false;
        }

        // Have exactly one king of each color.
        for color in Color::iter() {
            if (self.piece_occupancy(Piece::King) & self.color_occupancy(color)).count() != 1 {
                return false;
            }
        }

        // Verify that rooks and kings that are allowed to castle have not been moved.
        for color in Color::iter() {
            let castle_rights = self.castle_rights(color);

            // Nothing to check if there are no castlings allowed.
            if castle_rights == CastleRights::NoSide {
                continue;
            }

            let actual_rooks = self.piece_occupancy(Piece::Rook) & self.color_occupancy(color);
            let expected_rooks = castle_rights.unmoved_rooks(color);
            // We must check the intersection, in case there are more than 2 rooks on the board.
            if (expected_rooks & actual_rooks) != expected_rooks {
                return false;
            }

            let actual_king = self.piece_occupancy(Piece::King) & self.color_occupancy(color);
            let expected_king = Square::new(File::E, color.first_rank());
            // We have checked that there is exactly one king, no need for intersecting the sets.
            if actual_king != expected_king.into_bitboard() {
                return false;
            }
        }

        // The current en-passant target square must be empty, right behind an opponent's pawn.
        if let Some(square) = self.en_passant() {
            if !(self.combined_occupancy() & square).is_empty() {
                return false;
            }
            let opponent_pawns =
                self.piece_occupancy(Piece::Pawn) & self.color_occupancy(!self.current_player());
            let double_pushed_pawn = self
                .current_player()
                .backward_direction()
                .move_board(square.into_bitboard());
            if (opponent_pawns & double_pushed_pawn).is_empty() {
                return false;
            }
        }

        // Check that kings don't touch each other.
        let white_king = self.piece_occupancy(Piece::King) & self.color_occupancy(Color::White);
        let black_king = self.piece_occupancy(Piece::King) & self.color_occupancy(Color::Black);
        // Unwrap is fine, we already checked that there is exactly one king of each color
        if !(king_moves(white_king.try_into_square().unwrap()) & black_king).is_empty() {
            return false;
        }

        // Check that the opponent is not currently in check.
        if (self.compute_checkers(!self.current_player())) != Bitboard::EMPTY {
            return false;
        }

        true
    }

    /// Compute all pieces that are currently threatening the given [Color]'s king.
    fn compute_checkers(&self, color: Color) -> Bitboard {
        // Unwrap is fine, there should always be exactly one king per color
        let king = (self.piece_occupancy(Piece::King) & self.color_occupancy(color))
            .try_into_square()
            .unwrap();

        let opponent = !color;

        // No need to remove our pieces from the generated moves, we just want to check if we
        // intersect with the opponent's pieces, rather than generate only valid moves.
        let bishops = {
            let queens = self.piece_occupancy(Piece::Queen) & self.color_occupancy(opponent);
            let bishops = self.piece_occupancy(Piece::Bishop) & self.color_occupancy(opponent);
            let bishop_attacks = bishop_moves(king, self.combined_occupancy());
            (queens | bishops) & bishop_attacks
        };
        let rooks = {
            let queens = self.piece_occupancy(Piece::Queen) & self.color_occupancy(opponent);
            let rooks = self.piece_occupancy(Piece::Rook) & self.color_occupancy(opponent);
            let rook_attacks = rook_moves(king, self.combined_occupancy());
            (queens | rooks) & rook_attacks
        };
        let knights = {
            let knights = self.piece_occupancy(Piece::Knight) & self.color_occupancy(opponent);
            let knight_attacks = knight_moves(king);
            knights & knight_attacks
        };
        let pawns = {
            let pawns = self.piece_occupancy(Piece::Pawn) & self.color_occupancy(opponent);
            let pawn_attacks = pawn_captures(color, king);
            pawns & pawn_attacks
        };

        bishops | rooks | knights | pawns
    }
}

/// Use the starting position as a default value, corresponding to the
/// "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" FEN string
impl Default for ChessBoard {
    fn default() -> Self {
        Self {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Square::D1 | Square::D8,
                // Rook
                Square::A1 | Square::A8 | Square::H1 | Square::H8,
                // Bishop
                Square::C1 | Square::C8 | Square::F1 | Square::F8,
                // Knight
                Square::B1 | Square::B8 | Square::G1 | Square::G8,
                // Pawn
                Rank::Second.into_bitboard() | Rank::Seventh.into_bitboard(),
            ],
            color_occupancy: [
                Rank::First.into_bitboard() | Rank::Second.into_bitboard(),
                Rank::Seventh.into_bitboard() | Rank::Eighth.into_bitboard(),
            ],
            combined_occupancy: Rank::First.into_bitboard()
                | Rank::Second.into_bitboard()
                | Rank::Seventh.into_bitboard()
                | Rank::Eighth.into_bitboard(),
            castle_rights: [CastleRights::BothSides; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        }
    }
}

/// Return a [ChessBoard] from the given FEN string.
impl FromFen for ChessBoard {
    type Err = Error;

    fn from_fen(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let piece_placement = split.next().ok_or(Error::InvalidFen)?;
        let side_to_move = split.next().ok_or(Error::InvalidFen)?;
        let castling_rights = split.next().ok_or(Error::InvalidFen)?;
        let en_passant_square = split.next().ok_or(Error::InvalidFen)?;
        let half_move_clock = split.next().ok_or(Error::InvalidFen)?;
        let full_move_counter = split.next().ok_or(Error::InvalidFen)?;

        let castle_rights = <[CastleRights; 2]>::from_fen(castling_rights)?;
        let side = Color::from_fen(side_to_move)?;
        let en_passant = Option::<Square>::from_fen(en_passant_square)?;

        let half_move_clock = half_move_clock
            .parse::<u8>()
            .map_err(|_| Error::InvalidFen)?;
        let full_move_counter = full_move_counter
            .parse::<u32>()
            .map_err(|_| Error::InvalidFen)?;
        let total_plies = (full_move_counter - 1) * 2 + if side == Color::White { 0 } else { 1 };

        let (piece_occupancy, color_occupancy, combined_occupancy) = {
            let (mut pieces, mut colors, mut combined) =
                ([Bitboard::EMPTY; 6], [Bitboard::EMPTY; 2], Bitboard::EMPTY);

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
                    let (piece_board, color_board) =
                        (&mut pieces[piece.index()], &mut colors[color.index()]);

                    // Only need to worry about underflow since those are `usize` values.
                    if file >= 8 || rank >= 8 {
                        return Err(Error::InvalidFen);
                    };
                    let square = Square::new(File::from_index(file), Rank::from_index(rank));
                    *piece_board |= square;
                    *color_board |= square;
                    combined |= square;
                    file += 1;
                }
                // We haven't read exactly 8 files.
                if file != 8 {
                    return Err(Error::InvalidFen);
                }
            }
            // We haven't read exactly 8 ranks
            if rank != 0 {
                return Err(Error::InvalidFen);
            }

            (pieces, colors, combined)
        };

        let res = Self {
            piece_occupancy,
            color_occupancy,
            combined_occupancy,
            castle_rights,
            en_passant,
            half_move_clock,
            total_plies,
            side,
        };

        if !res.is_valid() {
            return Err(Error::InvalidPosition);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::MoveBuilder;

    #[test]
    fn valid() {
        let default_position = ChessBoard::default();
        assert!(default_position.is_valid());
    }

    #[test]
    fn invalid_overlapping_pieces() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Square::E1 | Square::E8,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1.into_bitboard(), Square::E8.into_bitboard()],
            combined_occupancy: Square::E1 | Square::E8,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_overlapping_colors() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1 | Square::E8, Square::E1 | Square::E8],
            combined_occupancy: Square::E1 | Square::E8,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_combined_does_not_equal_pieces() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1.into_bitboard(), Square::E8.into_bitboard()],
            combined_occupancy: Square::E1.into_bitboard(),
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_combined_does_not_equal_colors() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1 | Square::H1, Square::E8 | Square::H8],
            combined_occupancy: Square::E1 | Square::E8,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_multiple_kings() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E2 | Square::E7 | Square::E8,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1 | Square::E2, Square::E7 | Square::E8],
            combined_occupancy: Square::E1 | Square::E2 | Square::E7 | Square::E8,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_castling_rights_no_rooks() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1.into_bitboard(), Square::E8.into_bitboard()],
            combined_occupancy: Square::E1 | Square::E8,
            castle_rights: [CastleRights::BothSides; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_castling_rights_moved_king() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E2 | Square::E7,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Square::A1 | Square::A8 | Square::H1 | Square::H8,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [
                Square::A1 | Square::E2 | Square::H1,
                Square::A8 | Square::E7 | Square::H8,
            ],
            combined_occupancy: Square::A1
                | Square::A8
                | Square::E1
                | Square::E8
                | Square::H1
                | Square::H8,
            castle_rights: [CastleRights::BothSides; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_kings_next_to_each_other() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E2 | Square::E3,
                // Queen
                Bitboard::EMPTY,
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E2.into_bitboard(), Square::E3.into_bitboard()],
            combined_occupancy: Square::E2 | Square::E3,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn invalid_opponent_in_check() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E1 | Square::E8,
                // Queen
                Square::E7.into_bitboard(),
                // Rook
                Bitboard::EMPTY,
                // Bishop
                Bitboard::EMPTY,
                // Knight
                Bitboard::EMPTY,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [Square::E1 | Square::E7, Square::E8.into_bitboard()],
            combined_occupancy: Square::E1 | Square::E7 | Square::E8,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert!(!position.is_valid());
    }

    #[test]
    fn checkers() {
        let position = ChessBoard {
            piece_occupancy: [
                // King
                Square::E2 | Square::E8,
                // Queen
                Square::E7 | Square::H2,
                // Rook
                Square::A2 | Square::E1,
                // Bishop
                Square::D3 | Square::F3,
                // Knight
                Square::C1 | Square::G1,
                // Pawn
                Bitboard::EMPTY,
            ],
            color_occupancy: [
                Square::C1 | Square::D3 | Square::E1 | Square::E2 | Square::H2,
                Square::A2 | Square::E7 | Square::E8 | Square::F3 | Square::G1,
            ],
            combined_occupancy: Square::A2
                | Square::C1
                | Square::D3
                | Square::E1
                | Square::E2
                | Square::E7
                | Square::E8
                | Square::F3
                | Square::G1
                | Square::H2,
            castle_rights: [CastleRights::NoSide; 2],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        };
        assert_eq!(
            position.checkers(),
            Square::A2 | Square::E7 | Square::F3 | Square::G1
        );
    }

    #[test]
    fn fen_default_position() {
        let default_position = ChessBoard::default();
        assert_eq!(
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap(),
            default_position
        );
    }

    #[test]
    fn fen_en_passant() {
        // Start from default position
        let mut position = ChessBoard::default();
        // Modify it to account for e4 move
        position.xor(Color::White, Piece::Pawn, Square::E2 | Square::E4);
        position.en_passant = Some(Square::E3);
        position.total_plies = 1;
        position.side = Color::Black;
        assert_eq!(
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
                .unwrap(),
            position
        );
        // And now c5
        position.xor(Color::Black, Piece::Pawn, Square::C5 | Square::C7);
        position.en_passant = Some(Square::C6);
        position.total_plies = 2;
        position.side = Color::White;
        assert_eq!(
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
                .unwrap(),
            position
        );
        // Finally, Nf3
        position.xor(Color::White, Piece::Knight, Square::G1 | Square::F3);
        position.en_passant = None;
        position.total_plies = 3;
        position.half_move_clock = 1;
        position.side = Color::Black;
        assert_eq!(
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ")
                .unwrap(),
            position
        );
    }

    #[test]
    fn do_move() {
        // Start from default position
        let mut position = ChessBoard::default();
        // Modify it to account for e4 move
        position.do_move(
            MoveBuilder {
                piece: Piece::Pawn,
                start: Square::E2,
                destination: Square::E4,
                capture: None,
                promotion: None,
                en_passant: false,
                double_step: true,
                castling: false,
            }
            .into(),
        );
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
                .unwrap()
        );
        // And now c5
        position.do_move(
            MoveBuilder {
                piece: Piece::Pawn,
                start: Square::C7,
                destination: Square::C5,
                capture: None,
                promotion: None,
                en_passant: false,
                double_step: true,
                castling: false,
            }
            .into(),
        );
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
                .unwrap()
        );
        // Finally, Nf3
        position.do_move(
            MoveBuilder {
                piece: Piece::Knight,
                start: Square::G1,
                destination: Square::F3,
                capture: None,
                promotion: None,
                en_passant: false,
                double_step: false,
                castling: false,
            }
            .into(),
        );
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ")
                .unwrap()
        );
    }

    #[test]
    fn do_move_and_undo() {
        // Start from default position
        let mut position = ChessBoard::default();
        // Modify it to account for e4 move
        let move_1 = MoveBuilder {
            piece: Piece::Pawn,
            start: Square::E2,
            destination: Square::E4,
            capture: None,
            promotion: None,
            en_passant: false,
            double_step: true,
            castling: false,
        }
        .into();
        let state_1 = position.do_move(move_1);
        // And now c5
        let move_2 = MoveBuilder {
            piece: Piece::Pawn,
            start: Square::C7,
            destination: Square::C5,
            capture: None,
            promotion: None,
            en_passant: false,
            double_step: true,
            castling: false,
        }
        .into();
        let state_2 = position.do_move(move_2);
        // Finally, Nf3
        let move_3 = MoveBuilder {
            piece: Piece::Knight,
            start: Square::G1,
            destination: Square::F3,
            capture: None,
            promotion: None,
            en_passant: false,
            double_step: false,
            castling: false,
        }
        .into();
        let state_3 = position.do_move(move_3);
        // Now revert each move one-by-one
        position.undo_move(move_3, state_3);
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
                .unwrap()
        );
        position.undo_move(move_2, state_2);
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
                .unwrap()
        );
        position.undo_move(move_1, state_1);
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap()
        );
    }
}
