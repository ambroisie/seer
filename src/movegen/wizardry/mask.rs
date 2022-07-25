use crate::board::{Bitboard, File, Rank, Square};
use crate::movegen::naive::{bishop::bishop_moves, rook::rook_moves};

/// Compute the relevancy mask for a bishop on a given [Square].
pub fn generate_bishop_mask(square: Square) -> Bitboard {
    let rays = bishop_moves(square, Bitboard::EMPTY);

    let mask = File::A.into_bitboard()
        | File::H.into_bitboard()
        | Rank::First.into_bitboard()
        | Rank::Eighth.into_bitboard();

    rays - mask
}

/// Compute the relevancy mask for a rook on a given [Square].
pub fn generate_rook_mask(square: Square) -> Bitboard {
    let rays = rook_moves(square, Bitboard::EMPTY);

    let mask = {
        let mut mask = Bitboard::EMPTY;
        if square.file() != File::A {
            mask |= File::A.into_bitboard()
        };
        if square.file() != File::H {
            mask |= File::H.into_bitboard()
        };
        if square.rank() != Rank::First {
            mask |= Rank::First.into_bitboard()
        };
        if square.rank() != Rank::Eighth {
            mask |= Rank::Eighth.into_bitboard()
        };
        mask
    };

    rays - mask
}
