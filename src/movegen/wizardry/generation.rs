use crate::board::{Bitboard, Square};
use crate::movegen::bishop::bishop_moves;
use crate::movegen::rook::rook_moves;
use crate::movegen::Magic;

use super::mask::{generate_bishop_mask, generate_rook_mask};

/// A trait to represent RNG for u64 values.
#[allow(unused)] // FIXME: remove when used
pub(crate) trait RandGen {
    fn gen(&mut self) -> u64;
}

type MagicGenerationType = (Vec<Magic>, Vec<Bitboard>);

#[allow(unused)] // FIXME: remove when used
pub fn generate_bishop_magics(rng: &mut dyn RandGen) -> MagicGenerationType {
    generate_magics(rng, generate_bishop_mask, bishop_moves)
}

#[allow(unused)] // FIXME: remove when used
pub fn generate_rook_magics(rng: &mut dyn RandGen) -> MagicGenerationType {
    generate_magics(rng, generate_rook_mask, rook_moves)
}

fn generate_magics(
    rng: &mut dyn RandGen,
    mask_fn: impl Fn(Square) -> Bitboard,
    moves_fn: impl Fn(Square, Bitboard) -> Bitboard,
) -> MagicGenerationType {
    let mut magics = Vec::new();
    let mut boards = Vec::new();

    for square in Square::iter() {
        let mask = mask_fn(square);

        let occupancy_to_moves: Vec<_> = mask
            .iter_power_set()
            .map(|occupancy| (occupancy, moves_fn(square, occupancy)))
            .collect();

        'candidate_search: loop {
            let mut candidate = Magic {
                magic: magic_candidate(rng),
                offset: 0,
                mask,
                shift: (64 - mask.count()) as u8,
            };
            let mut candidate_moves = vec![Bitboard::EMPTY; occupancy_to_moves.len()];

            for (occupancy, moves) in occupancy_to_moves.iter().cloned() {
                let index = candidate.get_index(occupancy);
                // Non-constructive collision, try with another candidate
                if !candidate_moves[index].is_empty() && candidate_moves[index] != moves {
                    continue 'candidate_search;
                }
                candidate_moves[index] = moves;
            }

            // We have filled all candidate boards, record the correct offset and add the moves
            candidate.offset = boards.len();
            magics.push(candidate);
            boards.append(&mut candidate_moves);
            break;
        }
    }

    (magics, boards)
}

fn magic_candidate(rng: &mut dyn RandGen) -> u64 {
    // Few bits makes for better candidates
    rng.gen() & rng.gen() & rng.gen()
}
