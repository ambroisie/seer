use std::sync::OnceLock;

use crate::{
    board::{Bitboard, Color, File, Square},
    movegen::{
        naive,
        wizardry::{
            generate_bishop_magics, generate_rook_magics, MagicMoves, RandGen, BISHOP_SEED,
            ROOK_SEED,
        },
    },
};

// A pre-rolled RNG for magic bitboard generation, using pre-determined values.
struct PreRolledRng {
    numbers: [u64; 64],
    current_index: usize,
}

impl PreRolledRng {
    pub fn new(numbers: [u64; 64]) -> Self {
        Self {
            numbers,
            current_index: 0,
        }
    }
}

impl RandGen for PreRolledRng {
    fn gen(&mut self) -> u64 {
        // We roll 3 numbers per square to bitwise-and them together.
        // Just return the same one 3 times as a work-around.
        let res = self.numbers[self.current_index / 3];
        self.current_index += 1;
        res
    }
}

/// Compute the set of possible non-attack moves for a pawn on a [Square], given its [Color] and
/// set of blockers.
pub fn pawn_quiet_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    static PAWN_MOVES: OnceLock<[[Bitboard; 64]; 2]> = OnceLock::new();

    // If there is a piece in front of the pawn, it can't advance
    if !(color.backward_direction().move_board(blockers) & square).is_empty() {
        return Bitboard::EMPTY;
    }

    PAWN_MOVES.get_or_init(|| {
        let mut res = [[Bitboard::EMPTY; 64]; 2];
        for color in Color::iter() {
            for square in Square::iter() {
                res[color.index()][square.index()] =
                    naive::pawn_moves(color, square, Bitboard::EMPTY);
            }
        }
        res
    })[color.index()][square.index()]
}

/// Compute the set of possible attacks for a pawn on a [Square], given its [Color].
pub fn pawn_attacks(color: Color, square: Square) -> Bitboard {
    static PAWN_ATTACKS: OnceLock<[[Bitboard; 64]; 2]> = OnceLock::new();

    PAWN_ATTACKS.get_or_init(|| {
        let mut res = [[Bitboard::EMPTY; 64]; 2];
        for color in Color::iter() {
            for square in Square::iter() {
                res[color.index()][square.index()] = naive::pawn_captures(color, square);
            }
        }
        res
    })[color.index()][square.index()]
}

/// Compute the set of possible moves for a pawn on a [Square], given its [Color] and set of
/// blockers.
pub fn pawn_moves(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    pawn_quiet_moves(color, square, blockers) | pawn_attacks(color, square)
}

/// Compute the set of possible moves for a knight on a [Square].
pub fn knight_moves(square: Square) -> Bitboard {
    static KNIGHT_MOVES: OnceLock<[Bitboard; 64]> = OnceLock::new();
    KNIGHT_MOVES.get_or_init(|| {
        let mut res = [Bitboard::EMPTY; 64];
        for square in Square::iter() {
            res[square.index()] = naive::knight_moves(square)
        }
        res
    })[square.index()]
}

/// Compute the set of possible moves for a bishop on a [Square], given its set of blockers.
pub fn bishop_moves(square: Square, blockers: Bitboard) -> Bitboard {
    static BISHOP_MAGICS: OnceLock<MagicMoves> = OnceLock::new();
    BISHOP_MAGICS
        .get_or_init(|| {
            let (magics, moves) = generate_bishop_magics(&mut PreRolledRng::new(BISHOP_SEED));
            // SAFETY: we used the generator function to compute these values
            unsafe { MagicMoves::new(magics, moves) }
        })
        .query(square, blockers)
}

/// Compute the set of possible moves for a rook on a [Square], given its set of blockers.
pub fn rook_moves(square: Square, blockers: Bitboard) -> Bitboard {
    static ROOK_MAGICS: OnceLock<MagicMoves> = OnceLock::new();
    ROOK_MAGICS
        .get_or_init(|| {
            let (magics, moves) = generate_rook_magics(&mut PreRolledRng::new(ROOK_SEED));
            // SAFETY: we used the generator function to compute these values
            unsafe { MagicMoves::new(magics, moves) }
        })
        .query(square, blockers)
}

/// Compute the set of possible moves for a queen on a [Square], given its set of blockers.
pub fn queen_moves(square: Square, blockers: Bitboard) -> Bitboard {
    bishop_moves(square, blockers) | rook_moves(square, blockers)
}

/// Compute the set of possible moves for a king on a [Square].
pub fn king_moves(square: Square) -> Bitboard {
    static KING_MOVES: OnceLock<[Bitboard; 64]> = OnceLock::new();
    KING_MOVES.get_or_init(|| {
        let mut res = [Bitboard::EMPTY; 64];
        for square in Square::iter() {
            res[square.index()] = naive::king_moves(square)
        }
        res
    })[square.index()]
}

/// Compute the squares which should be empty for a king-side castle of the given [Color].
pub fn kind_side_castle_blockers(color: Color) -> Bitboard {
    let rank = color.first_rank();
    Square::new(File::F, rank) | Square::new(File::G, rank)
}

/// Compute the squares which should be empty for a queen-side castle of the given [Color].
pub fn queen_side_castle_blockers(color: Color) -> Bitboard {
    let rank = color.first_rank();
    Square::new(File::B, rank) | Square::new(File::C, rank) | Square::new(File::D, rank)
}
