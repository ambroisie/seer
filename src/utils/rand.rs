/// A trait to represent RNG for u64 values.
pub trait RandGen {
    fn gen(&mut self) -> u64;
}

// A simple XOR-shift RNG implementation, for code-generation.
#[cfg(test)]
pub struct SimpleRng(u64);

#[cfg(test)]
impl SimpleRng {
    pub fn new() -> Self {
        Self(4) // https://xkcd.com/221/
    }

    pub fn gen(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0
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

        assert_eq!(rng.gen(), 134217733);
        assert_eq!(rng.gen(), 4504699139039237);
        assert_eq!(rng.gen(), 13512173405898766);
        assert_eq!(rng.gen(), 9225626310854853124);
        assert_eq!(rng.gen(), 29836777971867270);
    }
}
