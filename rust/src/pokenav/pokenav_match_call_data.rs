//! Translated from `src/pokenav_match_call_data.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMrStoneTextScripts sMrStoneMatchCallHeader sNormanTextScripts sNormanMatchCallHeader sProfBirchMatchCallHeader sMomTextScripts sMomMatchCallHeader sStevenTextScripts sStevenMatchCallHeader sMayTextScripts sMayMatchCallHeader sBrendanTextScripts sBrendanMatchCallHeader sWallyTextScripts sWallyLocationData sWallyMatchCallHeader sScottTextScripts sScottMatchCallHeader sRoxanneTextScripts sRoxanneMatchCallHeader sBrawlyTextScripts sBrawlyMatchCallHeader sWattsonTextScripts sWattsonMatchCallHeader sFlanneryTextScripts sFlanneryMatchCallHeader sWinonaTextScripts sWinonaMatchCallHeader sTateLizaTextScripts sTateLizaMatchCallHeader sJuanTextScripts sJuanMatchCallHeader sSidneyTextScripts sSidneyMatchCallHeader sPhoebeTextScripts sPhoebeMatchCallHeader sGlaciaTextScripts sGlaciaMatchCallHeader sDrakeTextScripts sDrakeMatchCallHeader sWallaceTextScripts sWallaceMatchCallHeader sMatchCallHeaders sMatchCallGetEnabledFuncs sMatchCallGetMapSecFuncs sMatchCall_IsRematchableFunctions sMatchCall_HasCheckPageFunctions sMatchCall_GetRematchTableIdxFunctions sMatchCall_GetMessageFunctions sMatchCall_GetNameAndDescFunctions sCheckPageOverrides
#[allow(unused_imports)]
use crate::data::pokenav_match_call_data::*;

unsafe extern "C" {
    static mut gRematchTable: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gTrainerClassNames: u8;
    static mut gTrainers: u8;
    fn BufferPokedexRatingForMatchCall(a0: *mut u8);
    fn CountBattledRematchTeams(a0: u16) -> u16;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
}

pub(crate) unsafe extern "C" fn MatchCallGetFunctionIndex(matchCall__v: crate::c::Rec4<4>) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        'l1: {
            let __sw1 =
                ((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read()).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 5i32
                || __sw1 == 2i32
                || __sw1 == 4i32
                || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                return 0u32;
            }
            if __sw1 == 1i32 || __sw1 == 5i32 {
                return 1u32;
            }
            if __sw1 == 2i32 {
                return 2u32;
            }
            if __sw1 == 4i32 {
                return 3u32;
            }
            if __sw1 == 3i32 {
                return 4u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerIdxByRematchIdx(rematchIdx: u32) -> u32 {
    unsafe {
        let mut rematchIdx = rematchIdx;
        return ((((((&raw mut gRematchTable).cast::<u8>())
            .wrapping_offset(((rematchIdx) as i32) as isize * 16))
        .cast::<u16>())
        .read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRematchIdxByTrainerIdx(trainerIdx: i32) -> i32 {
    unsafe {
        let mut trainerIdx = trainerIdx;
        let mut rematchIdx: i32 = 0i32;
        {
            rematchIdx = 0i32;
            'l1: loop {
                if !(rematchIdx < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gRematchTable).cast::<u8>())
                        .wrapping_offset((rematchIdx) as isize * 16))
                    .cast::<u16>())
                    .read()) as i32)
                        == trainerIdx
                    {
                        return rematchIdx;
                    }
                }
                rematchIdx = (rematchIdx).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetEnabled(idx: u32) -> u32 {
    unsafe {
        let mut idx = idx;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return 0u32;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        return (((((&raw const sMatchCallGetEnabledFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetEnabled_NPC(matchCall__v: crate::c::Rec4<4>) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        if (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            == 65535i32
        {
            return 1u32;
        }
        return ((FlagGet(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        )) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetEnabled_Trainer(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        if (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            == 65535i32
        {
            return 1u32;
        }
        return ((FlagGet(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        )) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetEnabled_Wally(matchCall__v: crate::c::Rec4<4>) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        if (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            == 65535i32
        {
            return 1u32;
        }
        return ((FlagGet(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        )) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetEnabled_Rival(matchCall__v: crate::c::Rec4<4>) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        if (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            != ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
        {
            return 0u32;
        }
        if (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            == 65535i32
        {
            return 1u32;
        }
        return ((FlagGet(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        )) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetEnabled_Birch(matchCall__v: crate::c::Rec4<4>) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return ((FlagGet(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        )) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetMapSec(idx: u32) -> u8 {
    unsafe {
        let mut idx = idx;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return 0u8;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        return (((((&raw const sMatchCallGetMapSecFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u8>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMapSec_NPC(matchCall__v: crate::c::Rec4<4>) -> u8 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read()).wrapping_add(1))
            .read();
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMapSec_Trainer(matchCall__v: crate::c::Rec4<4>) -> u8 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read()).wrapping_add(1))
            .read();
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMapSec_Wally(matchCall__v: crate::c::Rec4<4>) -> u8 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset((i) as isize * 4))
                .cast::<u16>())
                .read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    if !((FlagGet(
                        ((((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 4))
                        .cast::<u16>())
                        .read(),
                    )) != 0)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset((i) as isize * 4))
        .wrapping_add(2))
        .read();
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMapSec_Rival(matchCall__v: crate::c::Rec4<4>) -> u8 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 213u8;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMapSec_Birch(matchCall__v: crate::c::Rec4<4>) -> u8 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 213u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_IsRematchable(idx: u32) -> u32 {
    unsafe {
        let mut idx = idx;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return 0u32;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        return (((((&raw const sMatchCall_IsRematchableFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_IsRematchable_NPC(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_IsRematchable_Trainer(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        if (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as i32)
            >= 73i32
        {
            return 0u32;
        }
        return ((if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2506))
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32) as isize,
        ))
        .read())
            != 0
        {
            1i32
        } else {
            0i32
        }) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_IsRematchable_Wally(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return ((if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2506))
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32) as isize,
        ))
        .read())
            != 0
        {
            1i32
        } else {
            0i32
        }) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_IsRematchable_Rival(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_IsRematchable_Birch(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_HasCheckPage(idx: u32) -> u32 {
    unsafe {
        let mut idx = idx;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return 0u32;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        if ((((((&raw const sMatchCall_HasCheckPageFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        )) != 0
        {
            return 1u32;
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(96u32, 24u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sCheckPageOverrides).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .cast::<u16>())
                    .read()) as u32)
                        == idx
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_HasCheckPage_NPC(matchCall__v: crate::c::Rec4<4>) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_HasCheckPage_Trainer(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_HasCheckPage_Wally(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_HasCheckPage_Rival(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_HasCheckPage_Birch(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetRematchTableIdx(idx: u32) -> u32 {
    unsafe {
        let mut idx = idx;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return 78u32;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        return (((((&raw const sMatchCall_GetRematchTableIdxFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>) -> u32>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetRematchTableIdx_NPC(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 78u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetRematchTableIdx_Trainer(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetRematchTableIdx_Wally(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as u32);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetRematchTableIdx_Rival(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 78u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetRematchTableIdx_Birch(
    matchCall__v: crate::c::Rec4<4>,
) -> u32 {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        return 78u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetMessage(idx: u32, dest: *mut u8) {
    unsafe {
        let mut idx = idx;
        let mut dest = dest;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        (((((&raw const sMatchCall_GetMessageFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>, *mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>, *mut u8)>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            dest,
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMessage_NPC(
    matchCall__v: crate::c::Rec4<4>,
    dest: *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut dest = dest;
        MatchCall_BufferCallMessageText(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read(),
            dest,
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMessage_Trainer(
    matchCall__v: crate::c::Rec4<4>,
    dest: *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut dest = dest;
        if ((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read()).read()) as i32)
            != 5i32
        {
            MatchCall_BufferCallMessageText(
                (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read(),
                dest,
            );
        } else {
            MatchCall_BufferCallMessageTextByRematchTeam(
                (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read(),
                (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read(),
                dest,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMessage_Wally(
    matchCall__v: crate::c::Rec4<4>,
    dest: *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut dest = dest;
        MatchCall_BufferCallMessageText(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read(),
            dest,
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMessage_Rival(
    matchCall__v: crate::c::Rec4<4>,
    dest: *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut dest = dest;
        MatchCall_BufferCallMessageText(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read(),
            dest,
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetMessage_Birch(
    matchCall__v: crate::c::Rec4<4>,
    dest: *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut dest = dest;
        BufferPokedexRatingForMatchCall(dest);
    }
}
pub(crate) unsafe extern "C" fn MatchCall_BufferCallMessageText(textData: *mut u8, dest: *mut u8) {
    unsafe {
        let mut textData = textData;
        let mut dest = dest;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !((((((textData).wrapping_offset(((i) as i32) as isize * 8)).cast::<*mut u8>())
                    .read()) as usize)
                    != 0usize)
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if (i) != 0 {
            i = (i).wrapping_sub(1);
        }
        'l3: loop {
            if !((i) != 0) {
                break 'l3;
            }
            if ((((((textData).wrapping_offset(((i) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32)
                != 65535i32)
                && (((FlagGet(
                    (((textData).wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read(),
                )) as i32)
                    == 1i32)
            {
                break 'l3;
            }
            i = (i).wrapping_sub(1);
        }
        if (((((textData).wrapping_offset(((i) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            != 65535i32
        {
            FlagSet(
                (((textData).wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(6)
                    .cast::<u16>())
                .read(),
            );
        }
        StringExpandPlaceholders(
            dest,
            (((textData).wrapping_offset(((i) as i32) as isize * 8)).cast::<*mut u8>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_BufferCallMessageTextByRematchTeam(
    textData: *mut u8,
    idx: u16,
    dest: *mut u8,
) {
    unsafe {
        let mut textData = textData;
        let mut idx = idx;
        let mut dest = dest;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !((((((textData).wrapping_offset(((i) as i32) as isize * 8)).cast::<*mut u8>())
                    .read()) as usize)
                    != 0usize)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((textData).wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32)
                        == 65534i32
                    {
                        break 'l1;
                    }
                    if ((((((textData).wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32)
                        != 65535i32)
                        && (!((FlagGet(
                            (((textData).wrapping_offset(((i) as i32) as isize * 8))
                                .wrapping_add(4)
                                .cast::<u16>())
                            .read(),
                        )) != 0))
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((textData).wrapping_offset(((i) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as i32)
            != 65534i32
        {
            if (i) != 0 {
                i = (i).wrapping_sub(1);
            }
            if (((((textData).wrapping_offset(((i) as i32) as isize * 8))
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32)
                != 65535i32
            {
                FlagSet(
                    (((textData).wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(6)
                        .cast::<u16>())
                    .read(),
                );
            }
            StringExpandPlaceholders(
                dest,
                (((textData).wrapping_offset(((i) as i32) as isize * 8)).cast::<*mut u8>()).read(),
            );
        } else {
            if (FlagGet(2148u16)) != 0 {
                'l3: loop {
                    'l4: {
                        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2506))
                        .cast::<u8>())
                        .wrapping_offset(((idx) as i32) as isize))
                        .read())
                            != 0
                        {
                            i = (i).wrapping_add(2u32);
                        } else {
                            if ((CountBattledRematchTeams(idx)) as i32) >= 2i32 {
                                i = (i).wrapping_add(3u32);
                            } else {
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            StringExpandPlaceholders(
                dest,
                (((textData).wrapping_offset(((i) as i32) as isize * 8)).cast::<*mut u8>()).read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetNameAndDesc(
    idx: u32,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut idx = idx;
        let mut desc = desc;
        let mut name = name;
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        let mut i: u32 = 0u32;
        if idx >= crate::c::div_u32(84u32, 4u32) {
            return;
        }
        (&raw mut matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw const sMatchCallHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        i = MatchCallGetFunctionIndex(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
        );
        (((((&raw const sMatchCall_GetNameAndDescFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>, *mut *mut u8, *mut *mut u8)>>(
            ))
        .cast::<Option<unsafe extern "C" fn(crate::c::Rec4<4>, *mut *mut u8, *mut *mut u8)>>())
        .wrapping_offset(((i) as i32) as isize))
        .read())
        .unwrap_unchecked()(
            (&raw mut matchCall)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            desc,
            name,
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetNameAndDesc_NPC(
    matchCall__v: crate::c::Rec4<4>,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut desc = desc;
        let mut name = name;
        (desc).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        (name).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetNameAndDesc_Trainer(
    matchCall__v: crate::c::Rec4<4>,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut desc = desc;
        let mut name = name;
        let mut _matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut _matchCall)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw mut matchCall)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        if (((((((&raw mut _matchCall).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            MatchCall_GetNameAndDescByRematchIdx(
                (((((((&raw mut _matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as u32),
                desc,
                name,
            );
        } else {
            (name).write(
                (((((&raw mut _matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read(),
            );
        }
        (desc).write(
            (((((&raw mut _matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetNameAndDesc_Wally(
    matchCall__v: crate::c::Rec4<4>,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut desc = desc;
        let mut name = name;
        MatchCall_GetNameAndDescByRematchIdx(
            (((((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as u32),
            desc,
            name,
        );
        (desc).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetNameAndDesc_Rival(
    matchCall__v: crate::c::Rec4<4>,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut desc = desc;
        let mut name = name;
        (desc).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        (name).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetNameAndDesc_Birch(
    matchCall__v: crate::c::Rec4<4>,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut matchCall = crate::ffi::Align4([0u8; 4]);
        (&raw mut matchCall)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(matchCall__v);
        let mut desc = desc;
        let mut name = name;
        (desc).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        (name).write(
            (((((&raw mut matchCall).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MatchCall_GetNameAndDescByRematchIdx(
    idx: u32,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    unsafe {
        let mut idx = idx;
        let mut desc = desc;
        let mut name = name;
        let mut trainer: *mut u8 = ((&raw mut gTrainers).cast::<u8>())
            .wrapping_offset(((GetTrainerIdxByRematchIdx(idx)) as i32) as isize * 40);
        (desc).write(
            (((&raw mut gTrainerClassNames).cast::<u8>())
                .wrapping_offset(((((trainer).wrapping_add(1)).read()) as i32) as isize * 13))
            .cast::<u8>(),
        );
        (name).write(((trainer).wrapping_add(4)).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetOverrideFlavorText(idx: u32, offset: u32) -> *mut u8 {
    unsafe {
        let mut idx = idx;
        let mut offset = offset;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(96u32, 24u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sCheckPageOverrides).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .cast::<u16>())
                    .read()) as u32)
                        == idx
                    {
                        {
                            'l3: loop {
                                if !((((i).wrapping_add(1u32) < crate::c::div_u32(96u32, 24u32))
                                    && ((((((((&raw const sCheckPageOverrides)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((i).wrapping_add(1u32)) as i32) as isize * 24,
                                    ))
                                    .cast::<u16>())
                                    .read()) as u32)
                                        == idx))
                                    && ((FlagGet(
                                        (((((((&raw const sCheckPageOverrides)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((i).wrapping_add(1u32)) as i32) as isize * 24,
                                        ))
                                        .wrapping_add(4)
                                        .cast::<u32>())
                                        .read()) as u16),
                                    )) != 0))
                                {
                                    break 'l3;
                                }
                                'l4: {}
                                i = (i).wrapping_add(1);
                            }
                        }
                        return (((((((&raw const sCheckPageOverrides)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(8))
                        .cast::<*mut u8>())
                        .wrapping_offset(((offset) as i32) as isize))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_GetOverrideFacilityClass(idx: u32) -> i32 {
    unsafe {
        let mut idx = idx;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(96u32, 24u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sCheckPageOverrides).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .cast::<u16>())
                    .read()) as u32)
                        == idx
                    {
                        return (((((((&raw const sCheckPageOverrides).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MatchCall_HasRematchId(idx: u32) -> u32 {
    unsafe {
        let mut idx = idx;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(84u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut id: u32 = MatchCall_GetRematchTableIdx(((i) as u32));
                    if (id != 78u32) && (id == idx) {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMatchCallRegisteredFlag() {
    unsafe {
        let mut index: i32 = GetRematchIdxByTrainerIdx(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32),
        );
        if index >= 0i32 {
            FlagSet((((348i32).wrapping_add(index)) as u16));
        }
    }
}
