//! Translated from `src/battle_intro.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::type_complexity,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gGameVersion;
use crate::battle_anim_mons::GetBattlerAtPosition;
use crate::battle_main::gBattleMonForms;
use crate::battle_main::{
    SpriteCB_VsLetterInit, gBattle_BG0_Y, gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X,
    gBattle_BG2_Y, gBattle_WIN0V, gBattleStruct, gBattleTypeFlags, gIntroSlideFlags,
    gMonSpritesGfxPtr,
};
use crate::battle_setup::gPartnerTrainerId;
use crate::bg::SetBgAttribute;
use crate::bg::{LoadBgTilemap, LoadBgTiles};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{GetGpuReg, SetGpuReg};
use crate::sprite::gSprites;
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
use crate::trig::Cos2;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tEnvironment: usize = 1;
// Data tables (translate with cdata.py): sBattleIntroSlideFuncs

static sBattleIntroSlideFuncs: Table<CArray<Option<unsafe fn(u8)>, 10>> =
    Table((&raw const crate::data::battle_intro::sBattleIntroSlideFuncs).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn HandleIntroSlide(mut environment: u8) {
    let mut taskId: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId != TRAINER_STEVEN_PARTNER
    {
        taskId = CreateTask(Some(BattleIntroSlidePartner), 0);
    } else if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        taskId = CreateTask(Some(BattleIntroSlideLink), 0);
    } else if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
        taskId = CreateTask(Some(BattleIntroSlide3), 0);
    } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE_GROUDON != 0
        && gGameVersion != VERSION_RUBY as u8
    {
        environment = BATTLE_ENVIRONMENT_UNDERWATER;
        taskId = CreateTask(Some(BattleIntroSlide2), 0);
    } else {
        taskId = CreateTask(sBattleIntroSlideFuncs[environment], 0);
    }
    task_set(taskId, tState, 0);
    task_set(taskId, tEnvironment, environment as i16);
    task_set(taskId, 2, 0);
    task_set(taskId, 3, 0);
    task_set(taskId, 4, 0);
    task_set(taskId, 5, 0);
    task_set(taskId, 6, 0);
}
unsafe fn BattleIntroSlideEnd(taskId: u8) {
    DestroyTask(taskId);
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16191);
}
pub(crate) unsafe fn BattleIntroSlide1(taskId: u8) {
    let mut i: i32 = 0;
    gBattle_BG1_X += 6;
    match task_get(taskId, tState) {
        0 => {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                task_set(taskId, 2, 16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            } else {
                task_set(taskId, 2, 1);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if ({
                task_set(taskId, 2, task_get(taskId, 2) - 1);
                task_get(taskId, 2)
            }) == 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                SetGpuReg(REG_OFFSET_WININ, 63);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                task_set(taskId, 2, DISPLAY_WIDTH as i16);
                task_set(taskId, 3, 32);
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if task_get(taskId, 3) != 0 {
                task_set(taskId, 3, task_get(taskId, 3) - 1);
            } else {
                if task_get(taskId, tEnvironment) == BATTLE_ENVIRONMENT_LONG_GRASS as i16 {
                    if gBattle_BG1_Y != 65456 {
                        gBattle_BG1_Y -= 2;
                    }
                } else {
                    if gBattle_BG1_Y != 65480 {
                        gBattle_BG1_Y -= 1;
                    }
                }
            }
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if task_get(taskId, 2) != 0 {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
            }
            i = 0;
            while i < 80 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = task_get(taskId, 2) as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = (task_get(taskId, 2) as u16).wrapping_neg();
                i += 1;
            }
            if task_get(taskId, 2) == 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000_usize as *mut c_void,
                            0x5000200,
                        );
                    }
                }
                SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
                SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
                SetGpuReg(REG_OFFSET_BG1CNT, 39936);
                SetGpuReg(REG_OFFSET_BG2CNT, 24064);
            }
        }
        4 => {
            BattleIntroSlideEnd(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn BattleIntroSlide2(taskId: u8) {
    let mut i: i32 = 0;
    match task_get(taskId, tEnvironment) {
        2 | 4 => {
            gBattle_BG1_X += 8;
        }
        3 => {
            gBattle_BG1_X += 6;
        }
        _ => {}
    }
    if task_get(taskId, tEnvironment) == BATTLE_ENVIRONMENT_WATER as i16 {
        gBattle_BG1_Y = (Cos2(task_get(taskId, 6) as u16) / 512) as u16 - 8;
        if task_get(taskId, 6) < 180 {
            task_set(taskId, 6, task_get(taskId, 6) + 4);
        } else {
            task_set(taskId, 6, task_get(taskId, 6) + 6);
        }
        if task_get(taskId, 6) == 360 {
            task_set(taskId, 6, 0);
        }
    }
    match task_get(taskId, tState) {
        0 => {
            task_set(taskId, 4, 16);
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                task_set(taskId, 2, 16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            } else {
                task_set(taskId, 2, 1);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if ({
                task_set(taskId, 2, task_get(taskId, 2) - 1);
                task_get(taskId, 2)
            }) == 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                SetGpuReg(REG_OFFSET_WININ, 63);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                task_set(taskId, 2, DISPLAY_WIDTH as i16);
                task_set(taskId, 3, 32);
                task_set(taskId, 5, 1);
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if task_get(taskId, 3) != 0 {
                if ({
                    task_set(taskId, 3, task_get(taskId, 3) - 1);
                    task_get(taskId, 3)
                }) == 0
                {
                    SetGpuReg(REG_OFFSET_BLDCNT, 6210);
                    SetGpuReg(REG_OFFSET_BLDALPHA, 15);
                    SetGpuReg(REG_OFFSET_BLDY, 0);
                }
            } else {
                if task_get(taskId, 4) as i32 & 0x1F != 0
                    && ({
                        task_set(taskId, 5, task_get(taskId, 5) - 1);
                        task_get(taskId, 5)
                    }) == 0
                {
                    task_set(taskId, 4, task_get(taskId, 4) + 0xFF);
                    task_set(taskId, 5, 4);
                }
            }
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if task_get(taskId, 2) != 0 {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
            }
            i = 0;
            while i < 80 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = task_get(taskId, 2) as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = (task_get(taskId, 2) as u16).wrapping_neg();
                i += 1;
            }
            if task_get(taskId, 2) == 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000_usize as *mut c_void,
                            0x5000200,
                        );
                    }
                }
                SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
                SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
                SetGpuReg(REG_OFFSET_BG1CNT, 39936);
                SetGpuReg(REG_OFFSET_BG2CNT, 24064);
            }
        }
        4 => {
            BattleIntroSlideEnd(taskId);
        }
        _ => {}
    }
    if task_get(taskId, tState) != 4 {
        SetGpuReg(REG_OFFSET_BLDALPHA, task_get(taskId, 4) as u16);
    }
}
pub(crate) unsafe fn BattleIntroSlide3(taskId: u8) {
    let mut i: i32 = 0;
    gBattle_BG1_X += 8;
    match task_get(taskId, tState) {
        0 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 6210);
            SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            task_set(taskId, 4, 2056);
            if gBattleTypeFlags & 0x2000002 != 0 {
                task_set(taskId, 2, 16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            } else {
                task_set(taskId, 2, 1);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if ({
                task_set(taskId, 2, task_get(taskId, 2) - 1);
                task_get(taskId, 2)
            }) == 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                SetGpuReg(REG_OFFSET_WININ, 63);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                task_set(taskId, 2, DISPLAY_WIDTH as i16);
                task_set(taskId, 3, 32);
                task_set(taskId, 5, 1);
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if task_get(taskId, 3) != 0 {
                task_set(taskId, 3, task_get(taskId, 3) - 1);
            } else {
                if task_get(taskId, 4) as i32 & 0xF != 0
                    && ({
                        task_set(taskId, 5, task_get(taskId, 5) - 1);
                        task_get(taskId, 5)
                    }) == 0
                {
                    task_set(taskId, 4, task_get(taskId, 4) + 0xFF);
                    task_set(taskId, 5, 6);
                }
            }
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if task_get(taskId, 2) != 0 {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
            }
            i = 0;
            while i < 80 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = task_get(taskId, 2) as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = (task_get(taskId, 2) as u16).wrapping_neg();
                i += 1;
            }
            if task_get(taskId, 2) == 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000_usize as *mut c_void,
                            0x5000200,
                        );
                    }
                }
                SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
                SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
                SetGpuReg(REG_OFFSET_BG1CNT, 39936);
                SetGpuReg(REG_OFFSET_BG2CNT, 24064);
            }
        }
        4 => {
            BattleIntroSlideEnd(taskId);
        }
        _ => {}
    }
    if task_get(taskId, tState) != 4 {
        SetGpuReg(REG_OFFSET_BLDALPHA, task_get(taskId, 4) as u16);
    }
}
pub(crate) unsafe fn BattleIntroSlideLink(taskId: u8) {
    let mut i: i32 = 0;
    if task_get(taskId, tState) > 1 && task_get(taskId, 4) == 0 {
        let var0: u16 = gBattle_BG1_X & 0x8000;
        if var0 != 0 || gBattle_BG1_X < 80 {
            gBattle_BG1_X += 3;
            gBattle_BG2_X -= 3;
        } else {
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        0x600e000_usize as *mut c_void,
                        0x5000200,
                    );
                }
            }
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        0x600f000_usize as *mut c_void,
                        0x5000200,
                    );
                }
            }
            task_set(taskId, 4, 1);
        }
    }
    match task_get(taskId, tState) {
        0 => {
            task_set(taskId, 2, 32);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            if ({
                task_set(taskId, 2, task_get(taskId, 2) - 1);
                task_get(taskId, 2)
            }) == 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_V]
                    .oam
                    .set_objMode(ST_OAM_OBJ_WINDOW);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].callback =
                    Some(SpriteCB_VsLetterInit);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S]
                    .oam
                    .set_objMode(ST_OAM_OBJ_WINDOW);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].callback =
                    Some(SpriteCB_VsLetterInit);
                SetGpuReg(REG_OFFSET_WININ, 63);
                SetGpuReg(REG_OFFSET_WINOUT, 16134);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                task_set(taskId, 2, DISPLAY_WIDTH as i16);
                task_set(taskId, 3, 32);
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if task_get(taskId, 2) != 0 {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
            }
            i = 0;
            while i < 80 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = task_get(taskId, 2) as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = (task_get(taskId, 2) as u16).wrapping_neg();
                i += 1;
            }
            if task_get(taskId, 2) == 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
                SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
                SetGpuReg(REG_OFFSET_BG1CNT, 39936);
                SetGpuReg(REG_OFFSET_BG2CNT, 24064);
            }
        }
        4 => {
            BattleIntroSlideEnd(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn BattleIntroSlidePartner(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            task_set(taskId, 2, 1);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            if ({
                task_set(taskId, 2, task_get(taskId, 2) - 1);
                task_get(taskId, 2)
            }) == 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                SetGpuReg(REG_OFFSET_BG1CNT, 23562);
                SetGpuReg(REG_OFFSET_BG2CNT, 24074);
                SetGpuReg(
                    REG_OFFSET_DISPCNT,
                    GetGpuReg(REG_OFFSET_DISPCNT)
                        | DISPCNT_OBJ_1D_MAP
                        | DISPCNT_OBJ_ON
                        | DISPCNT_WIN0_ON
                        | DISPCNT_WIN1_ON
                        | DISPCNT_OBJWIN_ON,
                );
                SetGpuReg(REG_OFFSET_WININ, 15872);
                SetGpuReg(REG_OFFSET_WINOUT, 16191);
                gBattle_BG0_Y = 65488;
                gBattle_BG1_X = DISPLAY_WIDTH;
                gBattle_BG2_X = 65296;
            }
        }
        2 => {
            gBattle_WIN0V += 0x100;
            if gBattle_WIN0V as i32 & 0xFF00 != 0x100 {
                gBattle_WIN0V -= 1;
            }
            if gBattle_WIN0V as i32 & 0xFF00 == 0x2000 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                task_set(taskId, 2, DISPLAY_WIDTH as i16);
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gBattle_WIN0V as i32 & 0xFF00 != 0x4C00 {
                gBattle_WIN0V += 0x3FC;
            }
            if task_get(taskId, 2) != 0 {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
            }
            gBattle_BG1_X = task_get(taskId, 2) as u16;
            gBattle_BG2_X = (task_get(taskId, 2) as u16).wrapping_neg();
            if task_get(taskId, 2) == 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        4 => {
            gBattle_BG0_Y += 2;
            gBattle_BG2_Y += 2;
            if gBattle_WIN0V as i32 & 0xFF00 != 0x5000 {
                gBattle_WIN0V += 0xFF;
            }
            if gBattle_BG0_Y == 0 {
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000_usize as *mut c_void,
                            0x5000800,
                        );
                    }
                }
                SetGpuReg(REG_OFFSET_DISPCNT, GetGpuReg(REG_OFFSET_DISPCNT) & 49151);
                SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
                SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
                SetGpuReg(REG_OFFSET_BG1CNT, 39936);
                SetGpuReg(REG_OFFSET_BG2CNT, 24064);
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        5 => {
            BattleIntroSlideEnd(taskId);
        }
        _ => {}
    }
}
pub unsafe fn DrawBattlerOnBg(
    bgId: i32,
    x: u8,
    y: u8,
    battlerPosition: u8,
    paletteId: u8,
    tiles: *mut u8,
    tilemap: *mut u16,
    tilesOffset: u16,
) {
    let battler: u8 = GetBattlerAtPosition(battlerPosition);
    let mut offset: i32 = tilesOffset as i32;
    CpuSet(
        ((*gMonSpritesGfxPtr).sprites.ptr[battlerPosition] as *mut u8)
            .at(BG_SCREEN_SIZE as i32 * gBattleMonForms[battler] as i32) as *mut c_void,
        tiles as *mut c_void,
        1024,
    );
    LoadBgTiles(bgId as u8, tiles as *mut c_void, 0x1000, tilesOffset);
    for i in (y as i32)..(y as i32 + 8) {
        for j in (x as i32)..(x as i32 + 8) {
            *tilemap.at(i * 32 + j) = offset as u16 | (paletteId as u16) << 12;
            offset += 1;
        }
    }
    LoadBgTilemap(bgId as u8, tilemap as *mut c_void, BG_SCREEN_SIZE as u16, 0);
}
unsafe fn DrawBattlerOnBgDMA(
    x: u8,
    y: u8,
    battlerPosition: u8,
    arg3: u8,
    paletteId: u8,
    arg5: u16,
    arg6: u8,
    arg7: u8,
) {
    {
        {
            {
                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                volatile_write(
                    dmaRegs,
                    ((*gMonSpritesGfxPtr).sprites.ptr[battlerPosition] as *mut u8)
                        .at(BG_SCREEN_SIZE as i32 * arg3 as i32) as *mut c_void
                        as usize as u32,
                );
                volatile_write(
                    dmaRegs.at(1),
                    (0x6000000_usize as *mut c_void as *mut u8).at(arg5) as *mut c_void as usize
                        as u32,
                );
                volatile_write(dmaRegs.at(2), 0x80000400);
                let _ = (dmaRegs.at(2)).read_volatile();
            }
        }
    }
    let mut offset: i32 = (arg5 >> 5) as i32 - ((arg7 as i32) << 9);
    for i in (y as i32)..(y as i32 + 8) {
        for j in (x as i32)..(x as i32 + 8) {
            *(BG_VRAM as usize as *mut u16)
                .at(i * 32)
                .at(j + ((arg6 as i32) << 10)) = offset as u16 | (paletteId as u16) << 12;
            offset += 1;
        }
    }
}
