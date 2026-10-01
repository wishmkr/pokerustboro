//! The game's random number generators (was src/random.c).
//!
//! Both are the linear congruential generator of the ISO C standard's
//! example `rand()`, returning the high 16 bits of each new state. `Random`
//! drives almost everything (battles, encounters, personalities); `Random2`
//! is a second generator seeded separately.

use crate::global::Global;

const MULTIPLIER: u32 = 1_103_515_245;
const INCREMENT: u32 = 24_691;

/// The state after `state`.
pub const fn next_state(state: u32) -> u32 {
    state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT)
}

/// Advances a generator and returns its new random value.
fn step(state: &Global<u32>) -> u16 {
    (state.update(next_state) >> 16) as u16
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static gRngValue: Global<u32> = Global::new(0);

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static gRng2Value: Global<u32> = Global::new(0);

/// How many values `Random` has produced (kept from C; nothing reads it).
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
static sRandCount: Global<u32> = Global::new(0);

/// Reset by `SeedRng` (kept from C; nothing reads it).
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
static sUnknown: Global<u8> = Global::new(0);

/// A random number from the main generator.
pub fn random() -> u16 {
    sRandCount.update(|n| n.wrapping_add(1));
    step(&gRngValue)
}

/// A random number from the second generator.
pub fn random2() -> u16 {
    step(&gRng2Value)
}

/// Seeds the main generator.
pub fn seed_rng(seed: u16) {
    gRngValue.set(u32::from(seed));
    sUnknown.set(0);
}

/// Seeds the second generator.
pub fn seed_rng2(seed: u16) {
    gRng2Value.set(u32::from(seed));
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn Random() -> u16 {
    random()
}

#[unsafe(no_mangle)]
pub fn Random2() -> u16 {
    random2()
}

#[unsafe(no_mangle)]
pub fn SeedRng(seed: u16) {
    seed_rng(seed);
}

#[unsafe(no_mangle)]
pub fn SeedRng2(seed: u16) {
    seed_rng2(seed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_iso_c_rng_sequence() {
        let first = next_state(0);
        let second = next_state(first);
        assert_eq!(first, 24_691);
        assert_eq!(second, 3_917_380_458);
        assert_eq!((second >> 16) as u16, 59_774);
    }

    #[test]
    fn seeding_restarts_the_sequence() {
        seed_rng2(0);
        let a = [random2(), random2(), random2()];
        seed_rng2(0);
        assert_eq!([random2(), random2(), random2()], a);
        assert_eq!(a[1], 59_774);
    }
}
