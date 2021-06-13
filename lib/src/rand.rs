use rand::{thread_rng, Rng};

/// Source of randomness for generating states and scrambles
pub trait RandomSource {
    /// Returns a random number 0..`range`
    fn next(&mut self, range: u32) -> u32;
}

/// Simple repeatable pseudorandom source for testing
pub struct SimpleSeededRandomSource {
    seed: u32,
}

/// Random source using the `rand` crate
pub struct StandardRandomSource;

impl SimpleSeededRandomSource {
    /// Creates a new random source. This always starts at the same seed, and is intended
    /// for use in repeatable testing. Do not use for generating scrambles for a user.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { seed: 42 }
    }
}

impl RandomSource for SimpleSeededRandomSource {
    fn next(&mut self, range: u32) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed % range
    }
}

impl RandomSource for StandardRandomSource {
    fn next(&mut self, range: u32) -> u32 {
        thread_rng().gen_range(0..range)
    }
}
