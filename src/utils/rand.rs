/// A trait to represent RNG for u64 values.
pub trait RandGen {
    fn gen(&mut self) -> u64;
}

// A simple pcg64_fast RNG implementation, for code-generation.
#[cfg(test)]
pub struct SimpleRng(u128);

#[cfg(test)]
impl SimpleRng {
    pub fn new() -> Self {
        Self(0xcafef00dd15ea5e5 | 1) // https://xkcd.com/221/
    }

    pub fn gen(&mut self) -> u64 {
        const MULTIPLIER: u128 = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645;
        const XSHIFT: u32 = 64; // (128 - 64 + 64) / 2
        const ROTATE: u32 = 122; // 128 - 6

        self.0 = self.0.wrapping_mul(MULTIPLIER);
        let rot = (self.0 >> ROTATE) as u32;
        let xsl = (self.0 >> XSHIFT) as u64 ^ (self.0 as u64);
        xsl.rotate_right(rot)
    }
}

#[cfg(test)]
impl RandGen for SimpleRng {
    fn gen(&mut self) -> u64 {
        self.gen()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rng() {
        let mut rng = SimpleRng::new();

        assert_eq!(rng.gen(), 64934999470316615);
        assert_eq!(rng.gen(), 15459456780870779090);
        assert_eq!(rng.gen(), 13715484424881807779);
        assert_eq!(rng.gen(), 17718572936700675021);
        assert_eq!(rng.gen(), 14587996314750246637);
    }
}
