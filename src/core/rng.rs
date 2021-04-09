use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

/// Random number generator.
// https://en.wikipedia.org/wiki/Permuted_congruential_generator
pub struct Rng(Cell<u64>);
impl Default for Rng {
    fn default() -> Self {
        Rng::new()
    }
}
impl Rng {
    pub fn new() -> Self {
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        8171.hash(&mut hasher);
        let hash = hasher.finish();
        Self(Cell::new(hash << 1 | 1))
    }
    pub fn gen(&self) -> u64 {
        let s = self.0.get();
        self.0.set(
            s.wrapping_mul(0xc0ca_c01a_1537_8e3f_u64)
                .wrapping_add(0x0000_cafe_f00d_23d1_u64),
        );
        s ^ (s >> 11)
    }
}

#[cfg(test)]
mod tests;
