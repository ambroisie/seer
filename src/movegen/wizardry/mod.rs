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
    4908958787341189172,
    1157496606860279808,
    289395876198088778,
    649648646467355137,
    19162426089930848,
    564067194896448,
    18586170375029026,
    9185354800693760,
    72172012436987968,
    317226351607872,
    2597178509285688384,
    1162205282238464,
    144154788211329152,
    172197832046936160,
    4625762105940000802,
    1477217245166903296,
    2251937789583872,
    289373902621379585,
    4616200855845409024,
    2251909637357568,
    3532510975437640064,
    563517968228352,
    562953309660434,
    1196005458310201856,
    2350914225914520576,
    2287018679861376,
    13836188353273790593,
    11267795163676832,
    297519119119499264,
    18588344158519552,
    10453428171813953792,
    72128237668534272,
    1298164929055953920,
    865575144395900952,
    9293076573325312,
    108104018148197376,
    578503662094123152,
    4665870505495102224,
    6066493872259301520,
    285877477613857,
    2328941618281318466,
    721165292771739652,
    4899973577790523400,
    75050392749184,
    2305878200632215680,
    11530099074925593616,
    290561512873919880,
    18652187227888000,
    3379933716168704,
    9223409493537718272,
    22273835729926,
    1152921524003672064,
    4647812741240848385,
    1244225087719112712,
    7367907171013001728,
    9263922034316951570,
    300758214358598160,
    4611686331973636096,
    2377900605806479360,
    6958097192913601024,
    864691130877743617,
    703824948904066,
    612700674899317536,
    180742128018784384,
];

/// A set of magic numbers for rook move generation.
pub(crate) const ROOK_SEED: [u64; Square::NUM_VARIANTS] = [
    2341871943948451840,
    18015635528220736,
    72066665545773824,
    1188959097794342912,
    12141713393631625314,
    720649693658353672,
    36029896538981888,
    36033359356363520,
    140746619355268,
    1158339898446446661,
    36591886560003650,
    578853633228023808,
    2392554490300416,
    140814806160384,
    180706952366596608,
    10696087878779396,
    1153260703948210820,
    310748649170673678,
    36311372044308544,
    9223444604757615104,
    1267187285230592,
    282574622818306,
    18722484274726152,
    2271591090110593,
    1153063519847989248,
    10168327557107712,
    4507998211276833,
    1153203035420233728,
    4631961017139660032,
    2454499182462107776,
    289367288355753288,
    18015815850820609,
    9268726066908758912,
    11547264697673728000,
    2314929519368081536,
    140943655192577,
    20266215511427202,
    180706969441535248,
    1302683805944911874,
    11534000122299940994,
    22676602724843520,
    4639271120198041668,
    1302104069046927376,
    9184220895313928,
    4612249105954373649,
    562984581726212,
    2312678200579457040,
    4647736876550193157,
    3170604524138139776,
    4684447574787096704,
    20283792725901696,
    1152992019380963840,
    117383863558471808,
    1153488854922068096,
    17596884583424,
    90074759127192064,
    4900502436426416706,
    4573968656793901,
    1161084564408385,
    1657887889314811910,
    4614501455660058690,
    4612530729109422081,
    642458506527236,
    1116704154754,
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

    fn array_string(piece_type: &str, values: &[Magic]) -> Result<String, std::fmt::Error> {
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
    }

    #[test]
    #[ignore = "slow"]
    // Regenerates the magic bitboard numbers.
    fn regen_magic_seeds() {
        // We only care about the magics, the moves can be recomputed at runtime ~cheaply.
        let (bishop_magics, _) = generate_bishop_magics(&mut SimpleRng::new());
        let (rook_magics, _) = generate_rook_magics(&mut SimpleRng::new());

        let original_text = std::fs::read_to_string(file!()).unwrap();

        let bishop_array = array_string("bishop", &bishop_magics[..]).unwrap();
        let rook_array = array_string("rook", &rook_magics[..]).unwrap();

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
