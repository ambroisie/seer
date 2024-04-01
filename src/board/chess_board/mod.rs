use crate::movegen;

use super::{Bitboard, CastleRights, Color, File, Move, Piece, Rank, Square};

mod builder;
pub use builder::*;

mod error;
pub use error::*;

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

    /// Get the [Bitboard] representing all pieces of the given [Piece] and [Color] type.
    #[inline(always)]
    pub fn occupancy(&self, piece: Piece, color: Color) -> Bitboard {
        self.piece_occupancy(piece) & self.color_occupancy(color)
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
        self.validate().is_ok()
    }

    /// Validate the state of the board. Return Err([InvalidError]) if an issue is found.
    pub fn validate(&self) -> Result<(), InvalidError> {
        // Don't overlap pieces.
        for piece in Piece::iter() {
            #[allow(clippy::collapsible_if)]
            for other in Piece::iter() {
                if piece != other {
                    if !(self.piece_occupancy(piece) & self.piece_occupancy(other)).is_empty() {
                        return Err(InvalidError::OverlappingPieces);
                    }
                }
            }
        }

        // Don't overlap colors.
        if !(self.color_occupancy(Color::White) & self.color_occupancy(Color::Black)).is_empty() {
            return Err(InvalidError::OverlappingColors);
        }

        // Calculate the union of all pieces.
        let combined =
            Piece::iter().fold(Bitboard::EMPTY, |board, p| board | self.piece_occupancy(p));

        // Ensure that the pre-computed version is accurate.
        if combined != self.combined_occupancy() {
            return Err(InvalidError::ErroneousCombinedOccupancy);
        }

        // Ensure that all pieces belong to a color, and no color has pieces that don't exist.
        if combined != (self.color_occupancy(Color::White) | self.color_occupancy(Color::Black)) {
            return Err(InvalidError::ErroneousCombinedOccupancy);
        }

        for color in Color::iter() {
            for piece in Piece::iter() {
                // Check that we have the expected number of piecese.
                let count = self.occupancy(piece, color).count();
                let possible = match piece {
                    Piece::King => count <= 1,
                    Piece::Pawn => count <= 8,
                    Piece::Queen => count <= 9,
                    _ => count <= 10,
                };
                if !possible {
                    return Err(InvalidError::TooManyPieces);
                }
            }

            // Check that we have a king
            if self.occupancy(Piece::King, color).count() != 1 {
                return Err(InvalidError::MissingKing);
            }

            // Check that don't have too many pieces in total
            if self.color_occupancy(color).count() > 16 {
                return Err(InvalidError::TooManyPieces);
            }
        }

        // Check that pawns aren't in first/last rank.
        if !(self.piece_occupancy(Piece::Pawn)
            & (Rank::First.into_bitboard() | Rank::Eighth.into_bitboard()))
        .is_empty()
        {
            return Err(InvalidError::InvalidPawnPosition);
        }

        // Verify that rooks and kings that are allowed to castle have not been moved.
        for color in Color::iter() {
            let castle_rights = self.castle_rights(color);

            // Nothing to check if there are no castlings allowed.
            if castle_rights == CastleRights::NoSide {
                continue;
            }

            let actual_rooks = self.occupancy(Piece::Rook, color);
            let expected_rooks = castle_rights.unmoved_rooks(color);
            // We must check the intersection, in case there are more than 2 rooks on the board.
            if (expected_rooks & actual_rooks) != expected_rooks {
                return Err(InvalidError::InvalidCastlingRights);
            }

            let actual_king = self.occupancy(Piece::King, color);
            let expected_king = Square::new(File::E, color.first_rank());
            // We have checked that there is exactly one king, no need for intersecting the sets.
            if actual_king != expected_king.into_bitboard() {
                return Err(InvalidError::InvalidCastlingRights);
            }
        }

        // En-passant validation
        if let Some(square) = self.en_passant() {
            // Must be empty
            if !(self.combined_occupancy() & square).is_empty() {
                return Err(InvalidError::InvalidEnPassant);
            }

            let opponent = !self.current_player();

            // Must be on the opponent's third rank
            if (square & opponent.third_rank().into_bitboard()).is_empty() {
                return Err(InvalidError::InvalidEnPassant);
            }

            // Must be behind a pawn
            let opponent_pawns = self.occupancy(Piece::Pawn, opponent);
            let double_pushed_pawn = self
                .current_player()
                .backward_direction()
                .move_board(square.into_bitboard());
            if (opponent_pawns & double_pushed_pawn).is_empty() {
                return Err(InvalidError::InvalidEnPassant);
            }
        }

        // Check that kings don't touch each other.
        let white_king = self.occupancy(Piece::King, Color::White);
        let black_king = self.occupancy(Piece::King, Color::Black);
        // Unwrap is fine, we already checked that there is exactly one king of each color
        if !(movegen::king_moves(white_king.try_into().unwrap()) & black_king).is_empty() {
            return Err(InvalidError::NeighbouringKings);
        }

        // Check that the opponent is not currently in check.
        if !self.compute_checkers(!self.current_player()).is_empty() {
            return Err(InvalidError::OpponentInCheck);
        }

        Ok(())
    }

    /// Compute all pieces that are currently threatening the given [Color]'s king.
    fn compute_checkers(&self, color: Color) -> Bitboard {
        // Unwrap is fine, there should always be exactly one king per color
        let king = (self.occupancy(Piece::King, color)).try_into().unwrap();

        let opponent = !color;

        // No need to remove our pieces from the generated moves, we just want to check if we
        // intersect with the opponent's pieces, rather than generate only valid moves.
        let bishops = {
            let queens = self.occupancy(Piece::Queen, opponent);
            let bishops = self.occupancy(Piece::Bishop, opponent);
            let bishop_attacks = movegen::bishop_moves(king, self.combined_occupancy());
            (queens | bishops) & bishop_attacks
        };
        let rooks = {
            let queens = self.occupancy(Piece::Queen, opponent);
            let rooks = self.occupancy(Piece::Rook, opponent);
            let rook_attacks = movegen::rook_moves(king, self.combined_occupancy());
            (queens | rooks) & rook_attacks
        };
        let knights = {
            let knights = self.occupancy(Piece::Knight, opponent);
            let knight_attacks = movegen::knight_moves(king);
            knights & knight_attacks
        };
        let pawns = {
            let pawns = self.occupancy(Piece::Pawn, opponent);
            let pawn_attacks = movegen::pawn_attacks(color, king);
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

#[cfg(test)]
mod test {
    use crate::board::MoveBuilder;
    use crate::fen::FromFen;

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
        assert_eq!(
            position.validate().err().unwrap(),
            InvalidError::OverlappingPieces,
        );
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
        assert_eq!(
            position.validate().err().unwrap(),
            InvalidError::OverlappingColors,
        );
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
        assert_eq!(
            position.validate().err().unwrap(),
            InvalidError::ErroneousCombinedOccupancy,
        );
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
        assert_eq!(
            position.validate().err().unwrap(),
            InvalidError::ErroneousCombinedOccupancy,
        );
    }

    #[test]
    fn invalid_multiple_kings() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E2] = Some((Piece::King, Color::White));
            builder[Square::E7] = Some((Piece::King, Color::Black));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::TooManyPieces);
    }

    #[test]
    fn invalid_castling_rights_no_rooks() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            builder.with_castle_rights(CastleRights::BothSides, Color::White);
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::InvalidCastlingRights);
    }

    #[test]
    fn invalid_castling_rights_moved_king() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E2] = Some((Piece::King, Color::White));
            builder[Square::A1] = Some((Piece::Rook, Color::White));
            builder[Square::H1] = Some((Piece::Rook, Color::White));
            builder[Square::E7] = Some((Piece::King, Color::Black));
            builder[Square::A8] = Some((Piece::Rook, Color::Black));
            builder[Square::H8] = Some((Piece::Rook, Color::Black));
            builder.with_castle_rights(CastleRights::BothSides, Color::White);
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::InvalidCastlingRights);
    }

    #[test]
    fn valid_en_passant() {
        let mut builder = ChessBoardBuilder::new();
        builder[Square::E1] = Some((Piece::King, Color::White));
        builder[Square::E8] = Some((Piece::King, Color::Black));
        builder[Square::A5] = Some((Piece::Pawn, Color::Black));
        builder.with_en_passant(Square::A6);
        TryInto::<ChessBoard>::try_into(builder).unwrap();
    }

    #[test]
    fn invalid_en_passant_not_empty() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            builder[Square::A6] = Some((Piece::Rook, Color::Black));
            builder[Square::A5] = Some((Piece::Pawn, Color::Black));
            builder.with_en_passant(Square::A6);
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::InvalidEnPassant);
    }

    #[test]
    fn invalid_en_passant_not_behind_pawn() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            builder[Square::A5] = Some((Piece::Rook, Color::Black));
            builder.with_en_passant(Square::A6);
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::InvalidEnPassant);
    }

    #[test]
    fn invalid_en_passant_incorrect_rank() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            builder[Square::A4] = Some((Piece::Pawn, Color::Black));
            builder.with_en_passant(Square::A5);
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::InvalidEnPassant);
    }

    #[test]
    fn invalid_kings_next_to_each_other() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E2] = Some((Piece::King, Color::Black));
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::NeighbouringKings);
    }

    #[test]
    fn invalid_opponent_in_check() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E7] = Some((Piece::Queen, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::OpponentInCheck);
    }

    #[test]
    fn invalid_pawn_on_first_rank() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::H1] = Some((Piece::King, Color::White));
            builder[Square::A1] = Some((Piece::Pawn, Color::White));
            builder[Square::H8] = Some((Piece::King, Color::Black));
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::InvalidPawnPosition);
    }

    #[test]
    fn invalid_too_many_pieces() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::H1] = Some((Piece::King, Color::White));
            builder[Square::H8] = Some((Piece::King, Color::Black));
            for square in (File::B.into_bitboard() | File::C.into_bitboard()).into_iter() {
                builder[square] = Some((Piece::Pawn, Color::White));
            }
            for square in (File::F.into_bitboard() | File::G.into_bitboard()).into_iter() {
                builder[square] = Some((Piece::Pawn, Color::Black));
            }
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), InvalidError::TooManyPieces);
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
