#[used]
#[unsafe(link_section = ".rodata")]
static UNUSED: [u32; 16] = [
    0,
    0,
    (1 << 26) | (1 << 3),
    (1 << 26) | (1 << 3) | (1 << 1),
    (1 << 26) | (1 << 3) | (1 << 2),
    (1 << 26) | (1 << 3) | (1 << 2) | (1 << 1),
    (1 << 26) | (1 << 4),
    (1 << 26) | (1 << 4) | (1 << 2),
    (1 << 26) | (1 << 4) | (1 << 3),
    (1 << 26) | (1 << 4) | (1 << 3) | (1 << 2),
    (1 << 26) | (1 << 4) | (1 << 1),
    (1 << 26) | (1 << 4) | (1 << 2) | (1 << 1),
    (1 << 26) | (1 << 4) | (1 << 3) | (1 << 1),
    (1 << 26) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1),
    (1 << 25) | (1 << 8),
    (1 << 27) | (1 << 10),
];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata")]
pub static gOverworldBackgroundLayerFlags: [u16; 4] = [1 << 8, 1 << 9, 1 << 10, 1 << 11];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata")]
pub static gOrbEffectBackgroundLayerFlags: [u16; 4] = [1 << 0, 1 << 1, 1 << 2, 1 << 3];

#[cfg(test)]
mod tests {
    use super::{gOrbEffectBackgroundLayerFlags, gOverworldBackgroundLayerFlags};

    #[test]
    fn blend_target_masks_match_gba_register_bits() {
        assert_eq!(gOverworldBackgroundLayerFlags, [0x100, 0x200, 0x400, 0x800]);
        assert_eq!(gOrbEffectBackgroundLayerFlags, [1, 2, 4, 8]);
    }
}
