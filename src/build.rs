use std::io::{Result, Write};

pub mod board;
pub mod movegen;
pub mod utils;

use crate::{
    board::{Bitboard, Color, File, Square},
    movegen::{
        naive::{
            king::king_moves,
            knight::knight_moves,
            pawn::{pawn_captures, pawn_moves},
        },
        wizardry::generation::{generate_bishop_magics, generate_rook_magics},
        Magic,
    },
};

fn print_magics(out: &mut dyn Write, var_name: &str, magics: &[Magic]) -> Result<()> {
    writeln!(out, "static {}: [Magic; {}] = [", var_name, magics.len())?;
    for magic in magics.iter() {
        writeln!(
            out,
            "    Magic{{magic: {}, offset: {}, mask: Bitboard({}), shift: {},}},",
            magic.magic, magic.offset, magic.mask.0, magic.shift
        )?;
    }
    writeln!(out, "];")?;
    Ok(())
}

fn print_boards(out: &mut dyn Write, var_name: &str, boards: &[Bitboard]) -> Result<()> {
    writeln!(out, "static {}: [Bitboard; {}] = [", var_name, boards.len())?;
    for board in boards.iter().cloned() {
        writeln!(out, "    Bitboard({}),", board.0)?;
    }
    writeln!(out, "];")?;
    Ok(())
}

fn print_double_sided_boards(
    out: &mut dyn Write,
    var_name: &str,
    white_boards: &[Bitboard],
    black_boards: &[Bitboard],
) -> Result<()> {
    assert_eq!(white_boards.len(), black_boards.len());
    writeln!(
        out,
        "static {}: [[Bitboard; {}]; 2] = [",
        var_name,
        white_boards.len()
    )?;
    for color in Color::iter() {
        let boards = if color == Color::White {
            white_boards
        } else {
            black_boards
        };
        writeln!(out, "    [")?;
        for square in Square::iter() {
            writeln!(out, "        Bitboard({}),", boards[square.index()].0)?;
        }
        writeln!(out, "    ],")?;
    }
    writeln!(out, "];")?;
    Ok(())
}

#[allow(clippy::redundant_clone)]
fn main() -> Result<()> {
    // FIXME: rerun-if-changed directives

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let magic_path = std::path::Path::new(&out_dir).join("magic_tables.rs");
    let mut out = std::fs::File::create(&magic_path).unwrap();

    let rng = random::default().seed([12, 27]);

    {
        let (magics, moves) = generate_bishop_magics(&mut rng.clone());
        print_magics(&mut out, "BISHOP_MAGICS", &magics)?;
        print_boards(&mut out, "BISHOP_MOVES", &moves)?;
    }

    {
        let (magics, moves) = generate_rook_magics(&mut rng.clone());
        print_magics(&mut out, "ROOK_MAGICS", &magics)?;
        print_boards(&mut out, "ROOK_MOVES", &moves)?;
    }

    {
        let moves: Vec<_> = Square::iter().map(knight_moves).collect();
        print_boards(&mut out, "KNIGHT_MOVES", &moves)?;
    }

    {
        let white_moves: Vec<_> = Square::iter()
            .map(|square| pawn_moves(Color::White, square, Bitboard::EMPTY))
            .collect();
        let black_moves: Vec<_> = Square::iter()
            .map(|square| pawn_moves(Color::Black, square, Bitboard::EMPTY))
            .collect();
        print_double_sided_boards(&mut out, "PAWN_MOVES", &white_moves, &black_moves)?;
        let white_attacks: Vec<_> = Square::iter()
            .map(|square| pawn_captures(Color::White, square))
            .collect();
        let black_attacks: Vec<_> = Square::iter()
            .map(|square| pawn_captures(Color::Black, square))
            .collect();
        print_double_sided_boards(&mut out, "PAWN_ATTACKS", &white_attacks, &black_attacks)?;
    }

    {
        let moves: Vec<_> = Square::iter().map(king_moves).collect();
        print_boards(&mut out, "KING_MOVES", &moves)?;
        let king_blockers: Vec<_> = Color::iter()
            .map(|color| {
                Square::new(File::F, color.first_rank()) | Square::new(File::G, color.first_rank())
            })
            .collect();
        let queen_blockers: Vec<_> = Color::iter()
            .map(|color| {
                Square::new(File::B, color.first_rank())
                    | Square::new(File::C, color.first_rank())
                    | Square::new(File::D, color.first_rank())
            })
            .collect();
        print_boards(&mut out, "KING_SIDE_CASTLE_BLOCKERS", &king_blockers)?;
        print_boards(&mut out, "QUEEN_SIDE_CASTLE_BLOCKERS", &queen_blockers)?;
    }

    // Include the generated files now that the build script has run.
    println!("cargo:rustc-cfg=generated_boards");

    Ok(())
}
