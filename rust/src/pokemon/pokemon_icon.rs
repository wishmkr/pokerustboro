//! Translated from `src/pokemon_icon.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_braces,
    unused_labels,
    unused_comparisons,
    overflowing_literals,
    unused_unsafe,
    dead_code,
    unreachable_code,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): gMonIconTable gMonIconPaletteIndices gMonIconPaletteTable sMonIconOamData sAnim_0 sAnim_1 sAnim_2 sAnim_3 sAnim_4 sMonIconAnims sAffineAnim_0 sAffineAnim_1 sMonIconAffineAnims sSpriteImageSizes
#[allow(unused_imports)]
use crate::data::pokemon_icon::*;

unsafe extern "C" {
    static mut gSprites: u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn MailSpeciesToSpecies(a0: u16, a1: *mut u16) -> u16;
    fn RequestSpriteCopy(a0: *mut u8, a1: *mut u8, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonIcon(
    species: u16,
    callback: Option<unsafe extern "C" fn(*mut u8)>,
    x: i16,
    y: i16,
    subpriority: u8,
    personality: u32,
    handleDeoxys: u32,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut callback = callback;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut personality = personality;
        let mut handleDeoxys = handleDeoxys;
        let mut spriteId: u8 = 0u8;
        let mut iconTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write((&raw const sMonIconOamData).cast::<u8>().cast_mut());
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write(GetMonIconPtr(species, personality, handleDeoxys));
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sMonIconAnims)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sMonIconAffineAnims)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(callback);
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<u16>()
            .write(
                (((56000i32).wrapping_add(
                    ((((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read()) as i32),
                )) as u16),
            );
        if ((species) as i32) > 412i32 {
            (((&raw mut iconTemplate).cast::<u8>())
                .wrapping_add(20)
                .cast::<u16>())
            .write(56000u16);
        }
        spriteId = CreateMonIconSprite((&raw mut iconTemplate).cast::<u8>(), x, y, subpriority);
        UpdateMonIconFrame(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonIconNoPersonality(
    species: u16,
    callback: Option<unsafe extern "C" fn(*mut u8)>,
    x: i16,
    y: i16,
    subpriority: u8,
    handleDeoxys: u32,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut callback = callback;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut handleDeoxys = handleDeoxys;
        let mut spriteId: u8 = 0u8;
        let mut iconTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write((&raw const sMonIconOamData).cast::<u8>().cast_mut());
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sMonIconAnims)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sMonIconAffineAnims)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(callback);
        (&raw mut iconTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<u16>()
            .write(
                (((56000i32).wrapping_add(
                    ((((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read()) as i32),
                )) as u16),
            );
        (((&raw mut iconTemplate).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(GetMonIconTiles(species, handleDeoxys));
        spriteId = CreateMonIconSprite((&raw mut iconTemplate).cast::<u8>(), x, y, subpriority);
        UpdateMonIconFrame(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIconSpecies(species: u16, personality: u32) -> u16 {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        let mut result: u16 = 0u16;
        if ((species) as i32) == 201i32 {
            let mut letter: u16 = GetUnownLetterByPersonality(personality);
            if ((letter) as i32) == 0i32 {
                letter = 201u16;
            } else {
                letter = ((((letter) as i32).wrapping_add(412i32)) as u16);
            }
            result = letter;
        } else {
            if ((species) as i32) > 412i32 {
                result = 260u16;
            } else {
                result = species;
            }
        }
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnownLetterByPersonality(personality: u32) -> u16 {
    unsafe {
        let mut personality = personality;
        if !((personality) != 0) {
            return 0u16;
        } else {
            return ((crate::c::rem_u32(
                (((((personality & 50331648u32) >> 18) | ((personality & 196608u32) >> 12))
                    | ((personality & 768u32) >> 6))
                    | ((personality & 3u32) >> 0)),
                28u32,
            )) as u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIconSpeciesNoPersonality(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        let mut value: u16 = 0u16;
        if ((MailSpeciesToSpecies(species, &raw mut value)) as i32) == 201i32 {
            if ((value) as i32) == 0i32 {
                value = ((((value) as i32).wrapping_add(201i32)) as u16);
            } else {
                value = ((((value) as i32).wrapping_add(412i32)) as u16);
            }
            return value;
        } else {
            if ((species) as i32) > 412i32 {
                species = 260u16;
            }
            return GetIconSpecies(species, 0u32);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonIconPtr(
    species: u16,
    personality: u32,
    handleDeoxys: u32,
) -> *mut u8 {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        let mut handleDeoxys = handleDeoxys;
        return GetMonIconTiles(GetIconSpecies(species, personality), handleDeoxys);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeAndDestroyMonIconSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FreeAndDestroyMonIconSprite_(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMonIconPalettes() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(48u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadSpritePalette(
                        (((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafeLoadMonIconPalette(species: u16) {
    unsafe {
        let mut species = species;
        let mut palIndex: u8 = 0u8;
        if ((species) as i32) > 412i32 {
            species = 260u16;
        }
        palIndex = ((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
        if ((IndexOfSpritePaletteTag(
            (((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((palIndex) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        )) as i32)
            == 255i32
        {
            LoadSpritePalette(
                (((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((palIndex) as i32) as isize * 8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMonIconPalette(species: u16) {
    unsafe {
        let mut species = species;
        let mut palIndex: u8 = ((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((species) as i32) as isize))
        .read();
        if ((IndexOfSpritePaletteTag(
            (((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((palIndex) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        )) as i32)
            == 255i32
        {
            LoadSpritePalette(
                (((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((palIndex) as i32) as isize * 8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonIconPalettes() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(48u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    FreeSpritePaletteByTag(
                        (((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafeFreeMonIconPalette(species: u16) {
    unsafe {
        let mut species = species;
        let mut palIndex: u8 = 0u8;
        if ((species) as i32) > 412i32 {
            species = 260u16;
        }
        palIndex = ((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
        FreeSpritePaletteByTag(
            (((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((palIndex) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonIconPalette(species: u16) {
    unsafe {
        let mut species = species;
        let mut palIndex: u8 = 0u8;
        palIndex = ((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
        FreeSpritePaletteByTag(
            (((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((palIndex) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_MonIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateMonIconFrame(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonIconTiles(species: u16, handleDeoxys: u32) -> *mut u8 {
    unsafe {
        let mut species = species;
        let mut handleDeoxys = handleDeoxys;
        let mut iconSprite: *mut u8 = ((((&raw const gMonIconTable)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((species) as i32) as isize))
        .read();
        if (((species) as i32) == 410i32) && (handleDeoxys == 1u32) {
            iconSprite =
                (((1024u32).wrapping_add(((iconSprite) as usize as u32))) as usize as *mut u8);
        }
        return iconSprite;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoadAllMonIconPalettesAtOffset(offset: u16) {
    unsafe {
        let mut offset = offset;
        let mut i: i32 = 0i32;
        if ((offset) as u32)
            <= (0u32).wrapping_add(
                ((16u32).wrapping_sub(crate::c::div_u32(48u32, 8u32))).wrapping_mul(16u32),
            )
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((crate::c::div_u32(48u32, 8u32)) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        LoadPalette(
                            ((((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 8))
                            .cast::<*mut u16>())
                            .read())
                            .cast::<u8>(),
                            offset,
                            32u16,
                        );
                        offset = ((((offset) as i32).wrapping_add(16i32)) as u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetValidMonIconPalIndex(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        if ((species) as i32) > 412i32 {
            species = 260u16;
        }
        return ((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonIconPaletteIndexFromSpecies(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        return ((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetValidMonIconPalettePtr(species: u16) -> *mut u16 {
    unsafe {
        let mut species = species;
        if ((species) as i32) > 412i32 {
            species = 260u16;
        }
        return (((((&raw const gMonIconPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw const gMonIconPaletteIndices).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize))
                .read()) as i32) as isize
                    * 8,
            ))
        .cast::<*mut u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateMonIconFrame(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut result: u8 = 0u8;
        if ((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32) == 0i32 {
            let mut frame: i16 = ((crate::c::bf_read(
                ((((((sprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
                    .wrapping_offset(((((sprite).wrapping_add(42)).read()) as i32) as isize))
                .read())
                .wrapping_offset(((((sprite).wrapping_add(43)).read()) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                16,
                false,
            ) as u32) as i16);
            'l1: {
                let __sw1 = ((frame) as i32);
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    ((sprite).wrapping_add(43)).write(0u8);
                    break 'l1;
                }
                if !__matched {
                    RequestSpriteCopy(
                        (((sprite).wrapping_add(12).cast::<*mut u8>()).read()).wrapping_offset(
                            (((((((((&raw const sSpriteImageSizes).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32)
                                    as i32) as isize
                                    * 8,
                            ))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32)
                                    as i32) as isize,
                            ))
                            .read()) as i32)
                                .wrapping_mul(((frame) as i32)))
                                as isize,
                        ),
                        (((100728832i32).wrapping_add(
                            ((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16)
                                as i32)
                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                        )) as usize as *mut u8),
                        ((((((&raw const sSpriteImageSizes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32)
                                as i32) as isize
                                * 8,
                        ))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32)
                                as i32) as isize,
                        ))
                        .read(),
                    );
                    crate::c::bf_write(
                        (sprite).wrapping_add(44),
                        0,
                        6,
                        (((crate::c::bf_read(
                            ((((((sprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
                                .wrapping_offset(
                                    ((((sprite).wrapping_add(42)).read()) as i32) as isize,
                                ))
                            .read())
                            .wrapping_offset(
                                ((((sprite).wrapping_add(43)).read()) as i32) as isize * 4,
                            ))
                            .wrapping_add(2),
                            0,
                            6,
                            false,
                        ) as u32)
                            & 255u32) as u8) as i32,
                    );
                    let __p2 = (sprite).wrapping_add(43);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    result = ((sprite).wrapping_add(43)).read();
                    break 'l1;
                }
            }
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(44),
                0,
                6,
                ((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8).wrapping_sub(1))
                    as i32,
            );
        }
        return result;
    }
}
pub(crate) unsafe extern "C" fn CreateMonIconSprite(
    iconTemplate: *mut u8,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    unsafe {
        let mut iconTemplate = iconTemplate;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut spriteId: u8 = 0u8;
        let mut image = crate::ffi::Align4([0u8; 8]);
        (&raw mut image)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut image)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(
                ((((((&raw const sSpriteImageSizes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            (((iconTemplate).cast::<*mut u8>()).read()).wrapping_add(1),
                            6,
                            2,
                            false,
                        ) as u32) as i32) as isize
                            * 8,
                    ))
                .cast::<u16>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        (((iconTemplate).cast::<*mut u8>()).read()).wrapping_add(3),
                        6,
                        2,
                        false,
                    ) as u32) as i32) as isize,
                ))
                .read(),
            );
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(65535u16);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(((iconTemplate).wrapping_add(20).cast::<u16>()).read());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write(((iconTemplate).cast::<*mut u8>()).read());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(((iconTemplate).wrapping_add(8).cast::<*mut *mut u8>()).read());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write((&raw mut image).cast::<u8>());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((iconTemplate).wrapping_add(12).cast::<*mut *mut u8>()).read());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(
                ((iconTemplate)
                    .wrapping_add(16)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            );
        spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), x, y, subpriority);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
            2,
            1,
            (0u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(((iconTemplate).wrapping_add(4).cast::<*mut u8>()).read());
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn FreeAndDestroyMonIconSprite_(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut image = crate::ffi::Align4([0u8; 8]);
        (&raw mut image)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut image)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(
                ((((((&raw const sSpriteImageSizes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as i32)
                            as isize
                            * 8,
                    ))
                .cast::<u16>())
                .wrapping_offset(
                    ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as i32)
                        as isize,
                ))
                .read(),
            );
        ((sprite).wrapping_add(12).cast::<*mut u8>()).write((&raw mut image).cast::<u8>());
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPartyHPBarSprite(sprite: *mut u8, animNum: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut animNum = animNum;
        ((sprite).wrapping_add(42)).write(animNum);
        crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
        ((sprite).wrapping_add(43)).write(0u8);
    }
}
