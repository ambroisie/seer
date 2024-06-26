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
    half_move_clock: u32, // Should *probably* never go higher than 100.
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
    half_move_clock: u32, // Should *probably* never go higher than 100.
    captured_piece: Option<Piece>,
}

impl ChessBoard {
    /// Which player's turn is it.
    #[inline(always)]
    pub fn current_player(&self) -> Color {
        self.side
    }

    /// Return the target [Square] that can be captured en-passant, or `None`
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
    pub fn half_move_clock(&self) -> u32 {
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

    /// Quickly add/remove a piece on the [Bitboard]s that are part of the [ChessBoard] state.
    #[inline(always)]
    fn xor(&mut self, color: Color, piece: Piece, square: Square) {
        *self.piece_occupancy_mut(piece) ^= square;
        *self.color_occupancy_mut(color) ^= square;
        self.combined_occupancy ^= square;
    }

    /// Compute the change of [CastleRights] from moving/taking a piece.
    fn update_castling(&mut self, color: Color, piece: Piece, file: File) {
        let original = self.castle_rights(color);
        let new_rights = match (piece, file) {
            (Piece::Rook, File::A) => original.without_queen_side(),
            (Piece::Rook, File::H) => original.without_king_side(),
            (Piece::King, _) => CastleRights::NoSide,
            _ => return,
        };
        if new_rights != original {
            *self.castle_rights_mut(color) = new_rights;
        }
    }

    /// Play the given [Move], return a copy of the board with the resulting state.
    #[inline(always)]
    pub fn play_move(&self, chess_move: Move) -> Self {
        let mut res = self.clone();
        res.play_move_inplace(chess_move);
        res
    }

    /// Play the given [Move] in place, returning all non-revertible state (e.g: en-passant,
    /// etc...).
    #[inline(always)]
    pub fn play_move_inplace(&mut self, chess_move: Move) -> NonReversibleState {
        let opponent = !self.current_player();
        let move_piece = Piece::iter()
            .find(|&p| !(self.piece_occupancy(p) & chess_move.start()).is_empty())
            .unwrap();
        let captured_piece = Piece::iter()
            .skip(1) // No need to check for the king here
            .find(|&p| !(self.occupancy(p, opponent) & chess_move.destination()).is_empty());
        let is_double_step = move_piece == Piece::Pawn
            && chess_move.start().rank() == self.current_player().second_rank()
            && chess_move.destination().rank() == self.current_player().fourth_rank();

        // Save non-revertible state
        let state = NonReversibleState {
            castle_rights: self.castle_rights,
            en_passant: self.en_passant,
            half_move_clock: self.half_move_clock,
            captured_piece,
        };

        // Non-revertible state modification
        if captured_piece.is_some() || move_piece == Piece::Pawn {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }
        if is_double_step {
            let target_square = Square::new(
                chess_move.destination().file(),
                self.current_player().third_rank(),
            );
            self.en_passant = Some(target_square);
        } else {
            self.en_passant = None;
        }
        self.update_castling(self.current_player(), move_piece, chess_move.start().file());
        if let Some(piece) = captured_piece {
            self.xor(opponent, piece, chess_move.destination());
            // If a rook is captured, it loses its castling rights
            self.update_castling(opponent, piece, chess_move.destination().file());
        }

        // Revertible state modification
        let dest_piece = chess_move.promotion().unwrap_or(move_piece);
        self.xor(self.current_player(), move_piece, chess_move.start());
        self.xor(self.current_player(), dest_piece, chess_move.destination());
        self.total_plies += 1;
        self.side = !self.side;

        state
    }

    /// Reverse the effect of playing the given [Move], and return to the given
    /// [NonReversibleState].
    #[inline(always)]
    pub fn unplay_move(&mut self, chess_move: Move, previous: NonReversibleState) {
        // Restore non-revertible state
        self.castle_rights = previous.castle_rights;
        self.en_passant = previous.en_passant;
        self.half_move_clock = previous.half_move_clock;

        let move_piece = Piece::iter()
            // We're looking for the *destination* as this is *undoing* the move
            .find(|&p| !(self.piece_occupancy(p) & chess_move.destination()).is_empty())
            .unwrap();

        if let Some(piece) = previous.captured_piece {
            // The capture affected the *current* player, from our post-move POV
            self.xor(self.current_player(), piece, chess_move.destination());
        }

        // Restore revertible state
        let start_piece = chess_move.promotion().map_or(move_piece, |_| Piece::Pawn);
        self.xor(!self.current_player(), move_piece, chess_move.destination());
        self.xor(!self.current_player(), start_piece, chess_move.start());
        self.total_plies -= 1;
        self.side = !self.side;
    }

    /// Return true if the current state of the board looks valid, false if something is definitely
    /// wrong.
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Validate the state of the board. Return Err([ValidationError]) if an issue is found.
    pub fn validate(&self) -> Result<(), ValidationError> {
        // The current plie count should be odd on white's turn, and vice-versa.
        if self.total_plies() % 2 != self.current_player().index() as u32 {
            return Err(ValidationError::IncoherentPlieCount);
        }

        // Make sure the clocks are in agreement.
        if self.half_move_clock() > self.total_plies() {
            return Err(ValidationError::HalfMoveClockTooHigh);
        }

        // Don't overlap pieces.
        for piece in Piece::iter() {
            #[allow(clippy::collapsible_if)]
            for other in Piece::iter() {
                if piece != other {
                    if !(self.piece_occupancy(piece) & self.piece_occupancy(other)).is_empty() {
                        return Err(ValidationError::OverlappingPieces);
                    }
                }
            }
        }

        // Don't overlap colors.
        if !(self.color_occupancy(Color::White) & self.color_occupancy(Color::Black)).is_empty() {
            return Err(ValidationError::OverlappingColors);
        }

        // Calculate the union of all pieces.
        let combined =
            Piece::iter().fold(Bitboard::EMPTY, |board, p| board | self.piece_occupancy(p));

        // Ensure that the pre-computed version is accurate.
        if combined != self.combined_occupancy() {
            return Err(ValidationError::ErroneousCombinedOccupancy);
        }

        // Ensure that all pieces belong to a color, and no color has pieces that don't exist.
        if combined != (self.color_occupancy(Color::White) | self.color_occupancy(Color::Black)) {
            return Err(ValidationError::ErroneousCombinedOccupancy);
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
                    return Err(ValidationError::TooManyPieces);
                }
            }

            // Check that we have a king
            if self.occupancy(Piece::King, color).count() != 1 {
                return Err(ValidationError::MissingKing);
            }

            // Check that don't have too many pieces in total
            if self.color_occupancy(color).count() > 16 {
                return Err(ValidationError::TooManyPieces);
            }
        }

        // Check that pawns aren't in first/last rank.
        if !(self.piece_occupancy(Piece::Pawn)
            & (Rank::First.into_bitboard() | Rank::Eighth.into_bitboard()))
        .is_empty()
        {
            return Err(ValidationError::InvalidPawnPosition);
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
                return Err(ValidationError::InvalidCastlingRights);
            }

            let actual_king = self.occupancy(Piece::King, color);
            let expected_king = Square::new(File::E, color.first_rank());
            // We have checked that there is exactly one king, no need for intersecting the sets.
            if actual_king != expected_king.into_bitboard() {
                return Err(ValidationError::InvalidCastlingRights);
            }
        }

        // En-passant validation
        if let Some(square) = self.en_passant() {
            // Must be empty
            if !(self.combined_occupancy() & square).is_empty() {
                return Err(ValidationError::InvalidEnPassant);
            }

            let opponent = !self.current_player();

            // Must be on the opponent's third rank
            if (square & opponent.third_rank().into_bitboard()).is_empty() {
                return Err(ValidationError::InvalidEnPassant);
            }

            // Must be behind a pawn
            let opponent_pawns = self.occupancy(Piece::Pawn, opponent);
            let double_pushed_pawn = self
                .current_player()
                .backward_direction()
                .move_board(square.into_bitboard());
            if (opponent_pawns & double_pushed_pawn).is_empty() {
                return Err(ValidationError::InvalidEnPassant);
            }
        }

        // Check that kings don't touch each other.
        let white_king = self.occupancy(Piece::King, Color::White);
        let black_king = self.occupancy(Piece::King, Color::Black);
        // Unwrap is fine, we already checked that there is exactly one king of each color
        if !(movegen::king_moves(white_king.try_into().unwrap()) & black_king).is_empty() {
            return Err(ValidationError::NeighbouringKings);
        }

        // Check that the opponent is not currently in check.
        if !self.compute_checkers(!self.current_player()).is_empty() {
            return Err(ValidationError::OpponentInCheck);
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
            castle_rights: [CastleRights::BothSides; Color::NUM_VARIANTS],
            en_passant: None,
            half_move_clock: 0,
            total_plies: 0,
            side: Color::White,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::fen::FromFen;

    use super::*;

    #[test]
    fn valid() {
        let default_position = ChessBoard::default();
        assert!(default_position.is_valid());
    }

    #[test]
    fn invalid_incoherent_plie_count() {
        let position = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            let mut board = TryInto::<ChessBoard>::try_into(builder).unwrap();
            board.total_plies = 1;
            board
        };
        assert_eq!(
            position.validate().err().unwrap(),
            ValidationError::IncoherentPlieCount,
        );
    }

    #[test]
    fn invalid_half_moves_clock() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            builder.with_half_move_clock(10);
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), ValidationError::HalfMoveClockTooHigh);
    }

    #[test]
    fn invalid_overlapping_pieces() {
        let position = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            let mut board: ChessBoard = builder.try_into().unwrap();
            *board.piece_occupancy_mut(Piece::Queen) |= Square::E1.into_bitboard();
            board
        };
        assert_eq!(
            position.validate().err().unwrap(),
            ValidationError::OverlappingPieces,
        );
    }

    #[test]
    fn invalid_overlapping_colors() {
        let position = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            let mut board: ChessBoard = builder.try_into().unwrap();
            *board.color_occupancy_mut(Color::White) |= Square::E8.into_bitboard();
            board
        };
        assert_eq!(
            position.validate().err().unwrap(),
            ValidationError::OverlappingColors,
        );
    }

    #[test]
    fn invalid_combined_does_not_equal_pieces() {
        let position = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            let mut board: ChessBoard = builder.try_into().unwrap();
            *board.piece_occupancy_mut(Piece::Pawn) |= Square::E2.into_bitboard();
            board
        };
        assert_eq!(
            position.validate().err().unwrap(),
            ValidationError::ErroneousCombinedOccupancy,
        );
    }

    #[test]
    fn invalid_combined_does_not_equal_colors() {
        let position = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            let mut board: ChessBoard = builder.try_into().unwrap();
            *board.color_occupancy_mut(Color::Black) |= Square::E2.into_bitboard();
            board
        };
        assert_eq!(
            position.validate().err().unwrap(),
            ValidationError::ErroneousCombinedOccupancy,
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
        assert_eq!(res.err().unwrap(), ValidationError::TooManyPieces);
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
        assert_eq!(res.err().unwrap(), ValidationError::InvalidCastlingRights);
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
        assert_eq!(res.err().unwrap(), ValidationError::InvalidCastlingRights);
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
        assert_eq!(res.err().unwrap(), ValidationError::InvalidEnPassant);
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
        assert_eq!(res.err().unwrap(), ValidationError::InvalidEnPassant);
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
        assert_eq!(res.err().unwrap(), ValidationError::InvalidEnPassant);
    }

    #[test]
    fn invalid_kings_next_to_each_other() {
        let res = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::E1] = Some((Piece::King, Color::White));
            builder[Square::E2] = Some((Piece::King, Color::Black));
            TryInto::<ChessBoard>::try_into(builder)
        };
        assert_eq!(res.err().unwrap(), ValidationError::NeighbouringKings);
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
        assert_eq!(res.err().unwrap(), ValidationError::OpponentInCheck);
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
        assert_eq!(res.err().unwrap(), ValidationError::InvalidPawnPosition);
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
        assert_eq!(res.err().unwrap(), ValidationError::TooManyPieces);
    }

    #[test]
    fn checkers() {
        let position = {
            let mut builder = ChessBoardBuilder::new();
            builder[Square::C1] = Some((Piece::Knight, Color::White));
            builder[Square::D3] = Some((Piece::Bishop, Color::White));
            builder[Square::E1] = Some((Piece::Rook, Color::White));
            builder[Square::E2] = Some((Piece::King, Color::White));
            builder[Square::H2] = Some((Piece::Queen, Color::White));
            builder[Square::G1] = Some((Piece::Knight, Color::Black));
            builder[Square::F3] = Some((Piece::Bishop, Color::Black));
            builder[Square::A2] = Some((Piece::Rook, Color::Black));
            builder[Square::E8] = Some((Piece::King, Color::Black));
            builder[Square::E7] = Some((Piece::Queen, Color::Black));
            TryInto::<ChessBoard>::try_into(builder).unwrap()
        };
        assert_eq!(
            position.checkers(),
            Square::A2 | Square::E7 | Square::F3 | Square::G1
        );
    }

    #[test]
    fn play_move() {
        // Start from default position
        let mut position = ChessBoard::default();
        // Modify it to account for e4 move
        position.play_move_inplace(Move::new(Square::E2, Square::E4, None));
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
                .unwrap()
        );
        // And now c5
        position.play_move_inplace(Move::new(Square::C7, Square::C5, None));
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
                .unwrap()
        );
        // Finally, Nf3
        position.play_move_inplace(Move::new(Square::G1, Square::F3, None));
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ")
                .unwrap()
        );
    }

    #[test]
    fn play_move_capture_changes_castling() {
        let mut position = ChessBoard::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let expected = ChessBoard::from_fen("r3k2R/8/8/8/8/8/8/R3K3 b Qq - 0 1").unwrap();

        let capture = Move::new(Square::H1, Square::H8, None);

        position.play_move_inplace(capture);
        assert_eq!(position, expected);
    }

    #[test]
    fn play_move_and_undo() {
        // Start from default position
        let mut position = ChessBoard::default();
        // Modify it to account for e4 move
        let move_1 = Move::new(Square::E2, Square::E4, None);
        let state_1 = position.play_move_inplace(move_1);
        // And now c5
        let move_2 = Move::new(Square::C7, Square::C5, None);
        let state_2 = position.play_move_inplace(move_2);
        // Finally, Nf3
        let move_3 = Move::new(Square::G1, Square::F3, None);
        let state_3 = position.play_move_inplace(move_3);
        // Now revert each move one-by-one
        position.unplay_move(move_3, state_3);
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
                .unwrap()
        );
        position.unplay_move(move_2, state_2);
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
                .unwrap()
        );
        position.unplay_move(move_1, state_1);
        assert_eq!(
            position,
            ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap()
        );
    }

    #[test]
    fn play_move_undo_capture() {
        let mut position = ChessBoard::from_fen("3q3k/8/8/8/8/8/8/K2Q4 w - - 0 1").unwrap();
        let expected = ChessBoard::from_fen("3Q3k/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
        let original = position.clone();

        let capture = Move::new(Square::D1, Square::D8, None);

        let state = position.play_move_inplace(capture);
        assert_eq!(position, expected);

        position.unplay_move(capture, state);
        assert_eq!(position, original);
    }

    #[test]
    fn play_move_undo_promotion() {
        let mut position = ChessBoard::from_fen("7k/P7/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let expected = ChessBoard::from_fen("N6k/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
        let original = position.clone();

        let promotion = Move::new(Square::A7, Square::A8, Some(Piece::Knight));

        let state = position.play_move_inplace(promotion);
        assert_eq!(position, expected);

        position.unplay_move(promotion, state);
        assert_eq!(position, original);
    }
}
