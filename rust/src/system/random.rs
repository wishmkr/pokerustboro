//! The game's random number generators (was src/random.c).

const ISO_MULTIPLIER: u32 = 1_103_515_245;
const ISO_INCREMENT: u32 = 24_691;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
static mut sUnknown: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
static mut sRandCount: u32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRngValue: u32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRng2Value: u32 = 0;

#[inline]
const fn advance(value: u32) -> u32 {
    value
        .wrapping_mul(ISO_MULTIPLIER)
        .wrapping_add(ISO_INCREMENT)
}

unsafe fn next(state: *mut u32) -> u16 {
    let value = advance(unsafe { state.read_volatile() });
    unsafe { state.write_volatile(value) };
    (value >> 16) as u16
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Random() -> u16 {
    let value = unsafe { next(&raw mut gRngValue) };
    let count = unsafe { (&raw mut sRandCount).read_volatile() };
    unsafe { (&raw mut sRandCount).write_volatile(count.wrapping_add(1)) };
    value
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SeedRng(seed: u16) {
    unsafe { (&raw mut gRngValue).write_volatile(seed as u32) };
    unsafe { (&raw mut sUnknown).write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SeedRng2(seed: u16) {
    unsafe { (&raw mut gRng2Value).write_volatile(seed as u32) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Random2() -> u16 {
    unsafe { next(&raw mut gRng2Value) }
}

#[cfg(test)]
mod tests {
    use super::advance;

    #[test]
    fn matches_the_iso_c_rng_sequence() {
        let first = advance(0);
        let second = advance(first);

        assert_eq!(first, 24_691);
        assert_eq!(second, 3_917_380_458);
        assert_eq!((second >> 16) as u16, 59_774);
    }
}
