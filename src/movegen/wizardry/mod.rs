mod generation;
pub(super) use generation::*;
mod mask;

use crate::board::{Bitboard, Square};

/// A type representing the magic board indexing a given [crate::board::Square].
#[derive(Clone, Debug)]
pub(super) struct Magic {
    /// Magic number.
    pub(self) magic: u64,
    /// Base offset into the magic square table.
    pub(self) offset: usize,
    /// Mask to apply to the blocker board before applying the magic.
    pub(self) mask: Bitboard,
    /// Length of the resulting mask after applying the magic.
    pub(self) shift: u8,
}

impl Magic {
    /// Compute the index into the magics database for this set of `blockers`.
    pub fn get_index(&self, blockers: Bitboard) -> usize {
        let relevant_occupancy = (blockers & self.mask).0;
        let base_index = ((relevant_occupancy.wrapping_mul(self.magic)) >> self.shift) as usize;
        base_index + self.offset
    }
}

/// A type encapsulating a database of [Magic] bitboard moves.
#[derive(Clone, Debug)]
pub(crate) struct MagicMoves {
    magics: Vec<Magic>,
    moves: Vec<Bitboard>,
}

impl MagicMoves {
    /// Initialize a new [MagicMoves] given a matching list of [Magic] and its corresponding moves
    /// as a [Bitboard].
    ///
    /// # Safety
    ///
    /// This should only be called with values generated by [crate::movegen::wizardry::generation].
    pub unsafe fn new(magics: Vec<Magic>, moves: Vec<Bitboard>) -> Self {
        Self { magics, moves }
    }

    /// Get the set of valid moves for a piece standing on a [Square], given a set of blockers.
    pub fn query(&self, square: Square, blockers: Bitboard) -> Bitboard {
        // SAFETY: indices are in range by construction
        unsafe {
            let index = self
                .magics
                .get_unchecked(square.index())
                .get_index(blockers);
            *self.moves.get_unchecked(index)
        }
    }
}

// region:sourcegen
/// A set of magic numbers for bishop move generation.
pub(crate) const BISHOP_SEED: [u64; Square::NUM_VARIANTS] = [
    4634226011293351952,
    6918109887683821586,
    76562328660738184,
    7242919606867744800,
    13871652069997347969,
    1171657252671901696,
    147001475087730752,
    1752045392763101248,
    288406435526639744,
    4612213818402029888,
    9808848818951710728,
    9223394181731320840,
    54047645651435648,
    9224780030482579712,
    9049059098626048,
    1442330840700035221,
    1126037887157508,
    1153488887004529665,
    290485130928332936,
    9226749771011592258,
    148636405693678112,
    2260596997758984,
    73470481646424336,
    2341907012146823680,
    2314955761652335121,
    2265544246165632,
    13598764778463296,
    563087425962496,
    563087425962048,
    2163991853573081088,
    567353402270020,
    6488844433713538048,
    288810987011448834,
    11830884701569344,
    2747549955031826688,
    35734665298432,
    18025943920672800,
    292892945404789012,
    1153520472160470528,
    2260949167801860,
    155446765112299521,
    379008324189818944,
    4616480181217005576,
    576461027453960704,
    2450556349601564416,
    1160556519943569536,
    4612900059821375552,
    5477089643453251617,
    9223532084785594632,
    2810391870219355200,
    36594222015453185,
    4612011546951352320,
    2392883590201344,
    1152956706186200064,
    9009415592510464,
    81077999302148128,
    576746627483043968,
    301267327789056,
    39586720976896,
    720878306081243648,
    9223512777841312257,
    5764609859566698625,
    8088544233436348496,
    4612856276794474560,
];

/// A set of magic numbers for rook move generation.
pub(crate) const ROOK_SEED: [u64; Square::NUM_VARIANTS] = [
    180144122814791812,
    10448386594766422036,
    9403533616331358856,
    108095189301858304,
    72076290316044288,
    36066182562054145,
    4647717564258980096,
    13979173385364603396,
    4620833992751489152,
    297800804633419904,
    578009002156298240,
    2450099003505838082,
    1175721046778052864,
    20406952999780864,
    1175861788231598592,
    36169538802827392,
    288371663414771712,
    423313050501155,
    604731668136450,
    580261214513399808,
    297661437206136832,
    1750211954976489600,
    9020393411186696,
    9259543770406356001,
    44532368556032,
    10376381507760693256,
    52778707714176,
    4612829512676149248,
    1882513444629184528,
    2369460754144428160,
    9223380850137104901,
    2666413562481640036,
    141012643087392,
    16735517094631719424,
    17594358702087,
    2344264412262574084,
    422813768878080,
    1126450811896320,
    54466576291772936,
    42784758060548372,
    292874851780165648,
    18015364885839937,
    282644818493504,
    1184447393488764944,
    4649966632473477184,
    563499910594566,
    17632049496086,
    18502729728001,
    140742121013504,
    9711024139665536,
    246293205270784,
    290772515771392256,
    9230131836490350720,
    73326432604127360,
    453174886517643776,
    2396271245728563712,
    324259242966026501,
    288953994406543363,
    1153557061259362338,
    40533496293515441,
    1407392197644307,
    1729945211427624002,
    587808330812164100,
    9511606812128903298,
];
// endregion:sourcegen

#[cfg(test)]
mod test {
    use std::fmt::Write as _;

    use super::*;
    use crate::utils::SimpleRng;

    fn split_twice<'a>(
        text: &'a str,
        start_marker: &str,
        end_marker: &str,
    ) -> Option<(&'a str, &'a str, &'a str)> {
        let (prefix, rest) = text.split_once(start_marker)?;
        let (mid, suffix) = rest.split_once(end_marker)?;
        Some((prefix, mid, suffix))
    }

    fn array_string(piece_type: &str, values: &[Magic]) -> String {
        let inner = || -> Result<String, std::fmt::Error> {
            let mut res = String::new();

            writeln!(
                &mut res,
                "/// A set of magic numbers for {} move generation.",
                piece_type
            )?;
            writeln!(
                &mut res,
                "pub(crate) const {}_SEED: [u64; Square::NUM_VARIANTS] = [",
                piece_type.to_uppercase()
            )?;
            for magic in values {
                writeln!(&mut res, "    {},", magic.magic)?;
            }
            writeln!(&mut res, "];")?;

            Ok(res)
        };

        inner().unwrap()
    }

    #[test]
    #[ignore = "slow"]
    // Regenerates the magic bitboard numbers.
    fn regen_magic_seeds() {
        // We only care about the magics, the moves can be recomputed at runtime ~cheaply.
        let (bishop_magics, _) = generate_bishop_magics(&mut SimpleRng::new());
        let (rook_magics, _) = generate_rook_magics(&mut SimpleRng::new());

        let original_text = std::fs::read_to_string(file!()).unwrap();

        let bishop_array = array_string("bishop", &bishop_magics[..]);
        let rook_array = array_string("rook", &rook_magics[..]);

        let new_text = {
            let start_marker = "// region:sourcegen\n";
            let end_marker = "// endregion:sourcegen\n";
            let (prefix, _, suffix) =
                split_twice(&original_text, start_marker, end_marker).unwrap();
            format!("{prefix}{start_marker}{bishop_array}\n{rook_array}{end_marker}{suffix}")
        };

        if new_text != original_text {
            std::fs::write(file!(), new_text).unwrap();
            panic!("source was not up-to-date")
        }
    }
}
