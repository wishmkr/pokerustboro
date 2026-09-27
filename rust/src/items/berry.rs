//! Translated from `src/berry.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBerryDescriptionPart1_Cheri sBerryDescriptionPart2_Cheri sBerryDescriptionPart1_Chesto sBerryDescriptionPart2_Chesto sBerryDescriptionPart1_Pecha sBerryDescriptionPart2_Pecha sBerryDescriptionPart1_Rawst sBerryDescriptionPart2_Rawst sBerryDescriptionPart1_Aspear sBerryDescriptionPart2_Aspear sBerryDescriptionPart1_Leppa sBerryDescriptionPart2_Leppa sBerryDescriptionPart1_Oran sBerryDescriptionPart2_Oran sBerryDescriptionPart1_Persim sBerryDescriptionPart2_Persim sBerryDescriptionPart1_Lum sBerryDescriptionPart2_Lum sBerryDescriptionPart1_Sitrus sBerryDescriptionPart2_Sitrus sBerryDescriptionPart1_Figy sBerryDescriptionPart2_Figy sBerryDescriptionPart1_Wiki sBerryDescriptionPart2_Wiki sBerryDescriptionPart1_Mago sBerryDescriptionPart2_Mago sBerryDescriptionPart1_Aguav sBerryDescriptionPart2_Aguav sBerryDescriptionPart1_Iapapa sBerryDescriptionPart2_Iapapa sBerryDescriptionPart1_Razz sBerryDescriptionPart2_Razz sBerryDescriptionPart1_Bluk sBerryDescriptionPart2_Bluk sBerryDescriptionPart1_Nanab sBerryDescriptionPart2_Nanab sBerryDescriptionPart1_Wepear sBerryDescriptionPart2_Wepear sBerryDescriptionPart1_Pinap sBerryDescriptionPart2_Pinap sBerryDescriptionPart1_Pomeg sBerryDescriptionPart2_Pomeg sBerryDescriptionPart1_Kelpsy sBerryDescriptionPart2_Kelpsy sBerryDescriptionPart1_Qualot sBerryDescriptionPart2_Qualot sBerryDescriptionPart1_Hondew sBerryDescriptionPart2_Hondew sBerryDescriptionPart1_Grepa sBerryDescriptionPart2_Grepa sBerryDescriptionPart1_Tamato sBerryDescriptionPart2_Tamato sBerryDescriptionPart1_Cornn sBerryDescriptionPart2_Cornn sBerryDescriptionPart1_Magost sBerryDescriptionPart2_Magost sBerryDescriptionPart1_Rabuta sBerryDescriptionPart2_Rabuta sBerryDescriptionPart1_Nomel sBerryDescriptionPart2_Nomel sBerryDescriptionPart1_Spelon sBerryDescriptionPart2_Spelon sBerryDescriptionPart1_Pamtre sBerryDescriptionPart2_Pamtre sBerryDescriptionPart1_Watmel sBerryDescriptionPart2_Watmel sBerryDescriptionPart1_Durin sBerryDescriptionPart2_Durin sBerryDescriptionPart1_Belue sBerryDescriptionPart2_Belue sBerryDescriptionPart1_Liechi sBerryDescriptionPart2_Liechi sBerryDescriptionPart1_Ganlon sBerryDescriptionPart2_Ganlon sBerryDescriptionPart1_Salac sBerryDescriptionPart2_Salac sBerryDescriptionPart1_Petaya sBerryDescriptionPart2_Petaya sBerryDescriptionPart1_Apicot sBerryDescriptionPart2_Apicot sBerryDescriptionPart1_Lansat sBerryDescriptionPart2_Lansat sBerryDescriptionPart1_Starf sBerryDescriptionPart2_Starf sBerryDescriptionPart1_Enigma sBerryDescriptionPart2_Enigma gBerries gBerryCrush_BerryData gBlankBerryTree
#[allow(unused_imports)]
use crate::data::berry::*;

unsafe extern "C" {
    static mut BerryTreeScript: u8;
    static mut gObjectEvents: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSelectedObjectEvent: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpecialVar_LastTalked: u8;
    static mut gStringVar1: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn CB2_ChooseBerry();
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn GetBerryCountString(a0: *mut u8, a1: *mut u8, a2: u32);
    fn GetCameraCoords(a0: *mut u16, a1: *mut u16);
    fn GetObjectEventBerryTreeId(a0: u8) -> u8;
    fn GetObjectEventScriptPointerPlayerFacing() -> *mut u8;
    fn IsBagPocketNonEmpty(a0: u8) -> u8;
    fn IsBerryTreeSparkling(a0: u8, a1: u8, a2: u8) -> u8;
    fn Random() -> u16;
    fn SetBerryTreeJustPicked(a0: u8, a1: u8, a2: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearEnigmaBerries() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12792),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        52u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetEnigmaBerry(src: *mut u8) {
    unsafe {
        let mut src = src;
        let mut i: u32 = 0u32;
        let mut dest: *mut u8 =
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792);
        {
            i = 0u32;
            'l1: loop {
                if !(i < 52u32) {
                    break 'l1;
                }
                'l2: {
                    ((dest).wrapping_offset(((i) as i32) as isize))
                        .write(((src).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetEnigmaBerryChecksum(enigmaBerry: *mut u8) -> u32 {
    unsafe {
        let mut enigmaBerry = enigmaBerry;
        let mut i: u32 = 0u32;
        let mut checksum: u32 = 0u32;
        let mut dest: *mut u8 = core::ptr::null_mut();
        dest = enigmaBerry;
        checksum = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 48u32) {
                    break 'l1;
                }
                'l2: {
                    checksum = (checksum).wrapping_add(
                        ((((dest).wrapping_offset(((i) as i32) as isize)).read()) as u32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return checksum;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEnigmaBerryValid() -> u32 {
    unsafe {
        if !(((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
            .wrapping_add(20))
        .read())
            != 0)
        {
            return 0u32;
        }
        if !(((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
            .wrapping_add(10))
        .read())
            != 0)
        {
            return 0u32;
        }
        if GetEnigmaBerryChecksum(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792),
        ) != (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
            .wrapping_add(48)
            .cast::<u32>())
        .read()
        {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryInfo(berry: u8) -> *mut u8 {
    unsafe {
        let mut berry = berry;
        if (((berry) as i32) == 43i32) && ((IsEnigmaBerryValid()) != 0) {
            return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792));
        } else {
            if (((berry) as i32) == 0i32) || (((berry) as i32) > 43i32) {
                berry = 1u8;
            }
            return (((&raw const gBerries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((berry) as i32).wrapping_sub(1i32)) as isize * 28);
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryTreeInfo(id: u8) -> *mut u8 {
    unsafe {
        let mut id = id;
        return (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5788))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionWaterBerryTree() -> u32 {
    unsafe {
        let mut tree: *mut u8 = GetBerryTreeInfo(GetObjectEventBerryTreeId(
            ((&raw mut gSelectedObjectEvent).cast::<u8>()).read(),
        ));
        'l1: {
            let __sw1 = ((crate::c::bf_read((tree).wrapping_add(1), 0, 7, false) as u8) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 1i32 {
                crate::c::bf_write((tree).wrapping_add(5), 4, 1, (1u8) as i32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write((tree).wrapping_add(5), 5, 1, (1u8) as i32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write((tree).wrapping_add(5), 6, 1, (1u8) as i32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                crate::c::bf_write((tree).wrapping_add(5), 7, 1, (1u8) as i32);
                break 'l1;
            }
            if !__matched {
                return 0u32;
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerFacingEmptyBerryTreePatch() -> u8 {
    unsafe {
        if (((GetObjectEventScriptPointerPlayerFacing()) as usize)
            == (((&raw mut BerryTreeScript).cast::<u8>()) as usize))
            && (((GetStageByBerryTreeId(GetObjectEventBerryTreeId(
                ((&raw mut gSelectedObjectEvent).cast::<u8>()).read(),
            ))) as i32)
                == 0i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryToWaterBerryTree() -> u8 {
    unsafe {
        if ((GetObjectEventScriptPointerPlayerFacing()) as usize)
            != (((&raw mut BerryTreeScript).cast::<u8>()) as usize)
        {
            return 0u8;
        } else {
            return ((ObjectEventInteractionWaterBerryTree()) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBerryTrees() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 128i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5788))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 8)
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const gBlankBerryTree)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BerryTreeGrow(tree: *mut u8) -> u32 {
    unsafe {
        let mut tree = tree;
        if (crate::c::bf_read((tree).wrapping_add(1), 7, 1, false) as u8) != 0 {
            return 0u32;
        }
        'l1: {
            let __sw1 = ((crate::c::bf_read((tree).wrapping_add(1), 0, 7, false) as u8) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                return 0u32;
            }
            if __sw1 == 4i32 {
                __fall = true;
                ((tree).wrapping_add(4)).write(CalcBerryYield(tree));
            }
            if __fall || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                __fall = true;
                crate::c::bf_write(
                    (tree).wrapping_add(1),
                    0,
                    7,
                    ((crate::c::bf_read((tree).wrapping_add(1), 0, 7, false) as u8).wrapping_add(1))
                        as i32,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                crate::c::bf_write((tree).wrapping_add(5), 4, 1, (0u8) as i32);
                crate::c::bf_write((tree).wrapping_add(5), 5, 1, (0u8) as i32);
                crate::c::bf_write((tree).wrapping_add(5), 6, 1, (0u8) as i32);
                crate::c::bf_write((tree).wrapping_add(5), 7, 1, (0u8) as i32);
                ((tree).wrapping_add(4)).write(0u8);
                crate::c::bf_write((tree).wrapping_add(1), 0, 7, (2u8) as i32);
                if (({
                    let __t2 = (crate::c::bf_read((tree).wrapping_add(5), 0, 4, false) as u8)
                        .wrapping_add(1);
                    crate::c::bf_write((tree).wrapping_add(5), 0, 4, (__t2) as i32);
                    __t2
                }) as i32)
                    == 10i32
                {
                    tree.cast::<crate::c::Rec4<8>>().write_unaligned(
                        (&raw const gBlankBerryTree)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BerryTreeTimeUpdate(minutes: i32) {
    unsafe {
        let mut minutes = minutes;
        let mut i: i32 = 0i32;
        let mut tree: *mut u8 = core::ptr::null_mut();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 128i32) {
                    break 'l1;
                }
                'l2: {
                    tree = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(5788))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8);
                    if ((((tree).read()) != 0)
                        && ((crate::c::bf_read((tree).wrapping_add(1), 0, 7, false) as u8) != 0))
                        && (!((crate::c::bf_read((tree).wrapping_add(1), 7, 1, false) as u8) != 0))
                    {
                        if minutes
                            >= ((GetStageDurationByBerryType((tree).read())) as i32)
                                .wrapping_mul(71i32)
                        {
                            tree.cast::<crate::c::Rec4<8>>().write_unaligned(
                                (&raw const gBlankBerryTree)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<8>>()
                                    .read_unaligned(),
                            );
                        } else {
                            let mut time: i32 = minutes;
                            'l3: loop {
                                if !(time != 0i32) {
                                    break 'l3;
                                }
                                if ((((tree).wrapping_add(2).cast::<u16>()).read()) as i32) > time {
                                    let __p1 = (tree).wrapping_add(2).cast::<u16>();
                                    (__p1).write(
                                        (((((__p1).read()) as i32).wrapping_sub(time)) as u16),
                                    );
                                    break 'l3;
                                }
                                time = (time).wrapping_sub(
                                    ((((tree).wrapping_add(2).cast::<u16>()).read()) as i32),
                                );
                                ((tree).wrapping_add(2).cast::<u16>())
                                    .write(GetStageDurationByBerryType((tree).read()));
                                if !((BerryTreeGrow(tree)) != 0) {
                                    break 'l3;
                                }
                                if ((crate::c::bf_read((tree).wrapping_add(1), 0, 7, false) as u8)
                                    as i32)
                                    == 5i32
                                {
                                    let __p2 = (tree).wrapping_add(2).cast::<u16>();
                                    (__p2).write(
                                        (((((__p2).read()) as i32).wrapping_mul(4i32)) as u16),
                                    );
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlantBerryTree(id: u8, berry: u8, stage: u8, allowGrowth: u8) {
    unsafe {
        let mut id = id;
        let mut berry = berry;
        let mut stage = stage;
        let mut allowGrowth = allowGrowth;
        let mut tree: *mut u8 = GetBerryTreeInfo(id);
        tree.cast::<crate::c::Rec4<8>>().write_unaligned(
            (&raw const gBlankBerryTree)
                .cast::<u8>()
                .cast_mut()
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
        );
        (tree).write(berry);
        ((tree).wrapping_add(2).cast::<u16>()).write(GetStageDurationByBerryType(berry));
        crate::c::bf_write((tree).wrapping_add(1), 0, 7, (stage) as i32);
        if ((stage) as i32) == 5i32 {
            ((tree).wrapping_add(4)).write(CalcBerryYield(tree));
            let __p1 = (tree).wrapping_add(2).cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_mul(4i32)) as u16));
        }
        if !((allowGrowth) != 0) {
            crate::c::bf_write((tree).wrapping_add(1), 7, 1, (1u8) as i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBerryTree(id: u8) {
    unsafe {
        let mut id = id;
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5788)).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 8)
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const gBlankBerryTree)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryTypeByBerryTreeId(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        return ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5788))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 8))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStageByBerryTreeId(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        return (crate::c::bf_read(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5788))
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 8))
            .wrapping_add(1),
            0,
            7,
            false,
        ) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemIdToBerryType(item: u16) -> u8 {
    unsafe {
        let mut item = item;
        let mut berry: u16 = ((((item) as i32).wrapping_sub(133i32)) as u16);
        if ((berry) as i32) > 42i32 {
            return 1u8;
        } else {
            return (((((item) as i32).wrapping_sub(133i32)).wrapping_add(1i32)) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BerryTypeToItemId(berry: u16) -> u16 {
    unsafe {
        let mut berry = berry;
        let mut item: u16 = ((((berry) as i32).wrapping_sub(1i32)) as u16);
        if ((item) as i32) > 42i32 {
            return 133u16;
        } else {
            return (((((berry) as i32).wrapping_add(133i32)).wrapping_sub(1i32)) as u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryNameByBerryType(berry: u8, string: *mut u8) {
    unsafe {
        let mut berry = berry;
        let mut string = string;
        crate::c::memcpy(string, (GetBerryInfo(berry)).cast::<u8>(), 6u32);
        ((string).wrapping_offset(6)).write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryCountStringByBerryType(berry: u8, dest: *mut u8, berryCount: u32) {
    unsafe {
        let mut berry = berry;
        let mut dest = dest;
        let mut berryCount = berryCount;
        GetBerryCountString(dest, (GetBerryInfo(berry)).cast::<u8>(), berryCount);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllowBerryTreeGrowth(id: u8) {
    unsafe {
        let mut id = id;
        crate::c::bf_write((GetBerryTreeInfo(id)).wrapping_add(1), 7, 1, (0u8) as i32);
    }
}
pub(crate) unsafe extern "C" fn BerryTreeGetNumStagesWatered(tree: *mut u8) -> u8 {
    unsafe {
        let mut tree = tree;
        let mut count: u8 = 0u8;
        if (crate::c::bf_read((tree).wrapping_add(5), 4, 1, false) as u8) != 0 {
            count = (count).wrapping_add(1);
        }
        if (crate::c::bf_read((tree).wrapping_add(5), 5, 1, false) as u8) != 0 {
            count = (count).wrapping_add(1);
        }
        if (crate::c::bf_read((tree).wrapping_add(5), 6, 1, false) as u8) != 0 {
            count = (count).wrapping_add(1);
        }
        if (crate::c::bf_read((tree).wrapping_add(5), 7, 1, false) as u8) != 0 {
            count = (count).wrapping_add(1);
        }
        return count;
    }
}
pub(crate) unsafe extern "C" fn GetNumStagesWateredByBerryTreeId(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        return BerryTreeGetNumStagesWatered(GetBerryTreeInfo(id));
    }
}
pub(crate) unsafe extern "C" fn CalcBerryYieldInternal(max: u16, min: u16, water: u8) -> u8 {
    unsafe {
        let mut max = max;
        let mut min = min;
        let mut water = water;
        let mut randMin: u32 = 0u32;
        let mut randMax: u32 = 0u32;
        let mut rand: u32 = 0u32;
        let mut extraYield: u32 = 0u32;
        if ((water) as i32) == 0i32 {
            return ((min) as u8);
        } else {
            randMin = (((((max) as i32).wrapping_sub(((min) as i32)))
                .wrapping_mul(((water) as i32).wrapping_sub(1i32))) as u32);
            randMax = (((((max) as i32).wrapping_sub(((min) as i32)))
                .wrapping_mul(((water) as i32))) as u32);
            rand = (randMin).wrapping_add(crate::c::rem_u32(
                ((Random()) as u32),
                ((randMax).wrapping_sub(randMin)).wrapping_add(1u32),
            ));
            if crate::c::rem_u32(rand, 4u32) >= ((crate::c::div_i32(4i32, 2i32)) as u32) {
                extraYield = (crate::c::div_u32(rand, 4u32)).wrapping_add(1u32);
            } else {
                extraYield = crate::c::div_u32(rand, 4u32);
            }
            return (((extraYield).wrapping_add(((min) as u32))) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CalcBerryYield(tree: *mut u8) -> u8 {
    unsafe {
        let mut tree = tree;
        let mut berry: *mut u8 = GetBerryInfo((tree).read());
        let mut min: u8 = ((berry).wrapping_add(11)).read();
        let mut max: u8 = ((berry).wrapping_add(10)).read();
        return CalcBerryYieldInternal(
            ((max) as u16),
            ((min) as u16),
            BerryTreeGetNumStagesWatered(tree),
        );
    }
}
pub(crate) unsafe extern "C" fn GetBerryCountByBerryTreeId(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        return (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5788))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 8))
        .wrapping_add(4))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetStageDurationByBerryType(berry: u8) -> u16 {
    unsafe {
        let mut berry = berry;
        return ((((((GetBerryInfo(berry)).wrapping_add(20)).read()) as i32).wrapping_mul(60i32))
            as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionGetBerryTreeData() {
    unsafe {
        let mut id: u8 = 0u8;
        let mut berry: u8 = 0u8;
        let mut localId: u8 = 0u8;
        let mut group: u8 = 0u8;
        let mut num: u8 = 0u8;
        id = GetObjectEventBerryTreeId(((&raw mut gSelectedObjectEvent).cast::<u8>()).read());
        berry = GetBerryTypeByBerryTreeId(id);
        AllowBerryTreeGrowth(id);
        localId = ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8);
        num = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(1)
            .cast::<i8>())
        .read()) as u8);
        group = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as u8);
        if (IsBerryTreeSparkling(localId, num, group)) != 0 {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(255u16);
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>())
                .write(((GetStageByBerryTreeId(id)) as u16));
        }
        ((&raw mut gSpecialVar_0x8005).cast::<u16>())
            .write(((GetNumStagesWateredByBerryTreeId(id)) as u16));
        ((&raw mut gSpecialVar_0x8006).cast::<u16>())
            .write(((GetBerryCountByBerryTreeId(id)) as u16));
        GetBerryCountStringByBerryType(
            berry,
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionGetBerryName() {
    unsafe {
        let mut berryType: u8 = GetBerryTypeByBerryTreeId(GetObjectEventBerryTreeId(
            ((&raw mut gSelectedObjectEvent).cast::<u8>()).read(),
        ));
        GetBerryNameByBerryType(berryType, (&raw mut gStringVar1).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionGetBerryCountString() {
    unsafe {
        let mut treeId: u8 =
            GetObjectEventBerryTreeId(((&raw mut gSelectedObjectEvent).cast::<u8>()).read());
        let mut berry: u8 = GetBerryTypeByBerryTreeId(treeId);
        let mut count: u8 = GetBerryCountByBerryTreeId(treeId);
        GetBerryCountStringByBerryType(
            berry,
            (&raw mut gStringVar1).cast::<u8>(),
            ((count) as u32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bag_ChooseBerry() {
    unsafe {
        SetMainCallback2(Some(CB2_ChooseBerry));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionPlantBerryTree() {
    unsafe {
        let mut berry: u8 = ItemIdToBerryType(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
        PlantBerryTree(
            GetObjectEventBerryTreeId(((&raw mut gSelectedObjectEvent).cast::<u8>()).read()),
            berry,
            1u8,
            1u8,
        );
        ObjectEventInteractionGetBerryTreeData();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionPickBerryTree() {
    unsafe {
        let mut id: u8 =
            GetObjectEventBerryTreeId(((&raw mut gSelectedObjectEvent).cast::<u8>()).read());
        let mut berry: u8 = GetBerryTypeByBerryTreeId(id);
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
            ((AddBagItem(
                BerryTypeToItemId(((berry) as u16)),
                ((GetBerryCountByBerryTreeId(id)) as u16),
            )) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionRemoveBerryTree() {
    unsafe {
        RemoveBerryTree(GetObjectEventBerryTreeId(
            ((&raw mut gSelectedObjectEvent).cast::<u8>()).read(),
        ));
        SetBerryTreeJustPicked(
            ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerHasBerries() -> u8 {
    unsafe {
        return IsBagPocketNonEmpty(4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBerryTreesSeen() {
    unsafe {
        let mut cam_left: i16 = 0i16;
        let mut cam_top: i16 = 0i16;
        let mut left: i16 = 0i16;
        let mut top: i16 = 0i16;
        let mut right: i16 = 0i16;
        let mut bottom: i16 = 0i16;
        let mut i: i32 = 0i32;
        GetCameraCoords(
            (&raw mut cam_left).cast::<u16>(),
            (&raw mut cam_top).cast::<u16>(),
        );
        left = cam_left;
        top = ((((cam_top) as i32).wrapping_add(3i32)) as i16);
        right = ((((cam_left) as i32).wrapping_add(14i32)) as i16);
        bottom = ((((top) as i32).wrapping_add(8i32)) as i16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                        .wrapping_add(0),
                        0,
                        1,
                        false,
                    ) as u32)
                        != 0)
                        && (((((((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                        .wrapping_add(6))
                        .read()) as i32)
                            == 12i32)
                    {
                        cam_left = (((((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                        .wrapping_add(16))
                        .cast::<i16>())
                        .read();
                        cam_top = (((((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                        .wrapping_add(16))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read();
                        if (((((left) as i32) <= ((cam_left) as i32))
                            && (((cam_left) as i32) <= ((right) as i32)))
                            && (((top) as i32) <= ((cam_top) as i32)))
                            && (((cam_top) as i32) <= ((bottom) as i32))
                        {
                            AllowBerryTreeGrowth(
                                ((((&raw mut gObjectEvents).cast::<u8>())
                                    .wrapping_offset((i) as isize * 36))
                                .wrapping_add(29))
                                .read(),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
