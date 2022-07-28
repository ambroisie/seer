use super::{Bitboard, CastleRights, Color, File, Move, Piece, Rank, Square};

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
    /// Either `None` if no double-step pawn move was made in the previous half-turn, or
    /// `Some(target_square)` if a double-step move was made.
    en_passant: Option<Square>,
    /// The number of half-turns without either a pawn push or capture.
    half_move_clock: u8, // Should never go higher than 50.
    /// The number of half-turns so far.
    total_plies: u32, // Should be plenty.
    /// The current player turn.
    side: Color,
}

/// The state which can't be reversed when doing/un-doing a [Move].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
        self.castle_rights[color.index()]
    }

    /// Return the [CastleRights] for the given [Color]. Allow mutations.
    #[inline(always)]
    fn castle_rights_mut(&mut self, color: Color) -> &mut CastleRights {
        &mut self.castle_rights[color.index()]
    }

    /// Get the [Bitboard] representing all pieces of the given [Piece] type, discarding color.
    #[inline(always)]
    pub fn piece_occupancy(&self, piece: Piece) -> Bitboard {
        self.piece_occupancy[piece.index()]
    }

    /// Get the [Bitboard] representing all pieces of the given [Piece] type, discarding color.
    /// Allow mutating the state.
    #[inline(always)]
    fn piece_occupancy_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.piece_occupancy[piece.index()]
    }

    /// Get the [Bitboard] representing all colors of the given [Color] type, discarding piece
    /// type.
    #[inline(always)]
    pub fn color_occupancy(&self, color: Color) -> Bitboard {
        self.color_occupancy[color.index()]
    }

    /// Get the [Bitboard] representing all colors of the given [Color] type, discarding piece
    /// type. Allow mutating the state.
    #[inline(always)]
    fn color_occupancy_mut(&mut self, color: Color) -> &mut Bitboard {
        &mut self.color_occupancy[color.index()]
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
    pub fn undo_move(&mut self, chess_move: Move, previous: NonReversibleState) {
        // Restore non-revertible state
        self.castle_rights = previous.castle_rights;
        self.en_passant = previous.en_passant;
        self.half_move_clock = previous.half_move_clock;

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
    pub fn is_valid(&self) -> bool {
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

        // FIXME: check for opponent being in check.
        // FIXME: check for kings touching.

        true
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

#[cfg(test)]
mod test {
    use super::*;

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
}
