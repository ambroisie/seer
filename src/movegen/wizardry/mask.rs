use crate::board::{Bitboard, File, Rank, Square};
use crate::movegen::bishop::bishop_moves;
use crate::movegen::rook::rook_moves;

pub fn generate_bishop_mask(square: Square) -> Bitboard {
    let rays = bishop_moves(square, Bitboard::EMPTY);

    let mask = File::A.into_bitboard()
        | File::H.into_bitboard()
        | Rank::First.into_bitboard()
        | Rank::Eighth.into_bitboard();

    rays - mask
}

pub fn generate_rook_mask(square: Square) -> Bitboard {
    let rays = rook_moves(square, Bitboard::EMPTY);

    let mask = {
        let mut mask = Bitboard::EMPTY;
        if square.file() != File::A {
            mask = mask | File::A.into_bitboard()
        };
        if square.file() != File::H {
            mask = mask | File::H.into_bitboard()
        };
        if square.rank() != Rank::First {
            mask = mask | Rank::First.into_bitboard()
        };
        if square.rank() != Rank::Eighth {
            mask = mask | Rank::Eighth.into_bitboard()
        };
        mask
    };

    rays - mask
}
