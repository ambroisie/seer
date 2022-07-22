use crate::board::{Bitboard, Square};
use crate::movegen::bishop::bishop_moves;
use crate::movegen::rook::rook_moves;
use crate::movegen::Magic;

use super::mask::{generate_bishop_mask, generate_rook_mask};

type MagicGenerationType = (Vec<Magic>, Vec<Bitboard>);

#[allow(unused)] // FIXME: remove when used
pub fn generate_bishop_magics(rng: &mut dyn random::Source) -> MagicGenerationType {
    generate_magics(rng, generate_bishop_mask, bishop_moves)
}

#[allow(unused)] // FIXME: remove when used
pub fn generate_rook_magics(rng: &mut dyn random::Source) -> MagicGenerationType {
    generate_magics(rng, generate_rook_mask, rook_moves)
}

fn generate_magics(
    rng: &mut dyn random::Source,
    mask_fn: impl Fn(Square) -> Bitboard,
    moves_fn: impl Fn(Square, Bitboard) -> Bitboard,
) -> MagicGenerationType {
    let mut offset = 0;

    let mut magics = Vec::new();
    let mut boards = Vec::new();

    for square in Square::iter() {
        let mask = mask_fn(square);
        let mut candidate: Magic;
        let potential_occupancy: Vec<_> = mask.iter_power_set().collect();
        let moves_len = potential_occupancy.len();

        'candidate_search: loop {
            candidate = Magic {
                magic: magic_candidate(rng),
                offset,
                mask,
                shift: (64 - mask.count()) as u8,
            };
            let mut candidate_moves = Vec::new();
            candidate_moves.resize(moves_len, Bitboard::EMPTY);

            for occupancy in potential_occupancy.iter().cloned() {
                let index = candidate.get_index(occupancy);
                let moves = moves_fn(square, occupancy);
                if candidate_moves[index] != Bitboard::EMPTY && candidate_moves[index] != moves {
                    continue 'candidate_search;
                }
                candidate_moves[index] = moves;
            }

            boards.append(&mut candidate_moves);
            break;
        }

        magics.push(candidate);
        offset += moves_len;
    }

    (magics, boards)
}

fn magic_candidate(rng: &mut dyn random::Source) -> u64 {
    rng.read_u64() & rng.read_u64() & rng.read_u64()
}
