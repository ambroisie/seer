// Magic bitboard
pub mod magic;
pub use magic::*;

// Move generation implementation details
pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod rook;

// Magic bitboard generation
pub(crate) mod wizardry;
