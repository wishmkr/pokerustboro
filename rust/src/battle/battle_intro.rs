//! Translated from `src/battle_intro.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sBattleIntroSlideFuncs

static sBattleIntroSlideFuncs: Table<CArray<Option<unsafe extern "C" fn(u8)>, 10>> =
    Table((&raw const crate::data::battle_intro::sBattleIntroSlideFuncs).cast());

unsafe extern "C" {
    static mut gBattleMonForms: CArray<u8, 4>;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattle_BG0_Y: u16;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_WIN0V: u16;
    static gGameVersion: u8;
    static mut gIntroSlideFlags: u16;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gPartnerTrainerId: u16;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn Cos2(a0: u16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SpriteCB_VsLetterInit(a0: *mut Sprite);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleIntroSlide(mut environment: u8) {
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
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = environment as i16;
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].data[3] = 0;
    gTasks[taskId].data[4] = 0;
    gTasks[taskId].data[5] = 0;
    gTasks[taskId].data[6] = 0;
}
pub(crate) unsafe extern "C" fn BattleIntroSlideEnd(taskId: u8) {
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
pub(crate) unsafe extern "C" fn BattleIntroSlide1(taskId: u8) {
    let mut i: i32 = 0;
    gBattle_BG1_X += 6;
    match gTasks[taskId].data[0] {
        0 => {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                gTasks[taskId].data[2] = 16;
                gTasks[taskId].data[0] += 1;
            } else {
                gTasks[taskId].data[2] = 1;
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[2] -= 1;
                gTasks[taskId].data[2]
            }) == 0
            {
                gTasks[taskId].data[0] += 1;
                SetGpuReg(REG_OFFSET_WININ, 63);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                gTasks[taskId].data[0] += 1;
                gTasks[taskId].data[2] = DISPLAY_WIDTH as i16;
                gTasks[taskId].data[3] = 32;
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gTasks[taskId].data[3] != 0 {
                gTasks[taskId].data[3] -= 1;
            } else {
                if gTasks[taskId].data[1] == BATTLE_ENVIRONMENT_LONG_GRASS as i16 {
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
            if gTasks[taskId].data[2] != 0 {
                gTasks[taskId].data[2] -= 2;
            }
            i = 0;
            while i < 80 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    gTasks[taskId].data[2] as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    (gTasks[taskId].data[2] as u16).wrapping_neg();
                i += 1;
            }
            if gTasks[taskId].data[2] == 0 {
                gScanlineEffect.state = 3;
                gTasks[taskId].data[0] += 1;
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000 as usize as *mut c_void,
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
pub(crate) unsafe extern "C" fn BattleIntroSlide2(taskId: u8) {
    let mut i: i32 = 0;
    match gTasks[taskId].data[1] {
        2 | 4 => {
            gBattle_BG1_X += 8;
        }
        3 => {
            gBattle_BG1_X += 6;
        }
        _ => {}
    }
    if gTasks[taskId].data[1] == BATTLE_ENVIRONMENT_WATER as i16 {
        gBattle_BG1_Y = (Cos2(gTasks[taskId].data[6] as u16) / 512) as u16 - 8;
        if gTasks[taskId].data[6] < 180 {
            gTasks[taskId].data[6] += 4;
        } else {
            gTasks[taskId].data[6] += 6;
        }
        if gTasks[taskId].data[6] == 360 {
            gTasks[taskId].data[6] = 0;
        }
    }
    match gTasks[taskId].data[0] {
        0 => {
            gTasks[taskId].data[4] = 16;
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                gTasks[taskId].data[2] = 16;
                gTasks[taskId].data[0] += 1;
            } else {
                gTasks[taskId].data[2] = 1;
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[2] -= 1;
                gTasks[taskId].data[2]
            }) == 0
            {
                gTasks[taskId].data[0] += 1;
                SetGpuReg(REG_OFFSET_WININ, 63);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                gTasks[taskId].data[0] += 1;
                gTasks[taskId].data[2] = DISPLAY_WIDTH as i16;
                gTasks[taskId].data[3] = 32;
                gTasks[taskId].data[5] = 1;
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gTasks[taskId].data[3] != 0 {
                if ({
                    gTasks[taskId].data[3] -= 1;
                    gTasks[taskId].data[3]
                }) == 0
                {
                    SetGpuReg(REG_OFFSET_BLDCNT, 6210);
                    SetGpuReg(REG_OFFSET_BLDALPHA, 15);
                    SetGpuReg(REG_OFFSET_BLDY, 0);
                }
            } else {
                if gTasks[taskId].data[4] as i32 & 0x1F != 0
                    && ({
                        gTasks[taskId].data[5] -= 1;
                        gTasks[taskId].data[5]
                    }) == 0
                {
                    gTasks[taskId].data[4] += 0xFF;
                    gTasks[taskId].data[5] = 4;
                }
            }
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if gTasks[taskId].data[2] != 0 {
                gTasks[taskId].data[2] -= 2;
            }
            i = 0;
            while i < 80 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    gTasks[taskId].data[2] as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    (gTasks[taskId].data[2] as u16).wrapping_neg();
                i += 1;
            }
            if gTasks[taskId].data[2] == 0 {
                gScanlineEffect.state = 3;
                gTasks[taskId].data[0] += 1;
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000 as usize as *mut c_void,
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
    if gTasks[taskId].data[0] != 4 {
        SetGpuReg(REG_OFFSET_BLDALPHA, 0 | gTasks[taskId].data[4] as u16);
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlide3(taskId: u8) {
    let mut i: i32 = 0;
    gBattle_BG1_X += 8;
    match gTasks[taskId].data[0] {
        0 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 6210);
            SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            gTasks[taskId].data[4] = 2056;
            if gBattleTypeFlags & 0x2000002 != 0 {
                gTasks[taskId].data[2] = 16;
                gTasks[taskId].data[0] += 1;
            } else {
                gTasks[taskId].data[2] = 1;
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[2] -= 1;
                gTasks[taskId].data[2]
            }) == 0
            {
                gTasks[taskId].data[0] += 1;
                SetGpuReg(REG_OFFSET_WININ, 63);
            }
        }
        2 => {
            gBattle_WIN0V -= 0xFF;
            if gBattle_WIN0V as i32 & 0xFF00 == 0x3000 {
                gTasks[taskId].data[0] += 1;
                gTasks[taskId].data[2] = DISPLAY_WIDTH as i16;
                gTasks[taskId].data[3] = 32;
                gTasks[taskId].data[5] = 1;
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gTasks[taskId].data[3] != 0 {
                gTasks[taskId].data[3] -= 1;
            } else {
                if gTasks[taskId].data[4] as i32 & 0xF != 0
                    && ({
                        gTasks[taskId].data[5] -= 1;
                        gTasks[taskId].data[5]
                    }) == 0
                {
                    gTasks[taskId].data[4] += 0xFF;
                    gTasks[taskId].data[5] = 6;
                }
            }
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if gTasks[taskId].data[2] != 0 {
                gTasks[taskId].data[2] -= 2;
            }
            i = 0;
            while i < 80 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    gTasks[taskId].data[2] as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    (gTasks[taskId].data[2] as u16).wrapping_neg();
                i += 1;
            }
            if gTasks[taskId].data[2] == 0 {
                gScanlineEffect.state = 3;
                gTasks[taskId].data[0] += 1;
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            0x600e000 as usize as *mut c_void,
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
    if gTasks[taskId].data[0] != 4 {
        SetGpuReg(REG_OFFSET_BLDALPHA, 0 | gTasks[taskId].data[4] as u16);
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSlideLink(taskId: u8) {
    let mut i: i32 = 0;
    if gTasks[taskId].data[0] > 1 && gTasks[taskId].data[4] == 0 {
        let mut var0: u16 = gBattle_BG1_X & 0x8000;
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
                        0x600e000 as usize as *mut c_void,
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
                        0x600f000 as usize as *mut c_void,
                        0x5000200,
                    );
                }
            }
            gTasks[taskId].data[4] = 1;
        }
    }
    match gTasks[taskId].data[0] {
        0 => {
            gTasks[taskId].data[2] = 32;
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if ({
                gTasks[taskId].data[2] -= 1;
                gTasks[taskId].data[2]
            }) == 0
            {
                gTasks[taskId].data[0] += 1;
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
                gTasks[taskId].data[0] += 1;
                gTasks[taskId].data[2] = DISPLAY_WIDTH as i16;
                gTasks[taskId].data[3] = 32;
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gBattle_WIN0V as i32 & 0xFF00 != 0 {
                gBattle_WIN0V -= 0x3FC;
            }
            if gTasks[taskId].data[2] != 0 {
                gTasks[taskId].data[2] -= 2;
            }
            i = 0;
            while i < 80 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    gTasks[taskId].data[2] as u16;
                i += 1;
            }
            while i < DISPLAY_HEIGHT as i32 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
                    (gTasks[taskId].data[2] as u16).wrapping_neg();
                i += 1;
            }
            if gTasks[taskId].data[2] == 0 {
                gScanlineEffect.state = 3;
                gTasks[taskId].data[0] += 1;
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
pub(crate) unsafe extern "C" fn BattleIntroSlidePartner(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            gTasks[taskId].data[2] = 1;
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if ({
                gTasks[taskId].data[2] -= 1;
                gTasks[taskId].data[2]
            }) == 0
            {
                gTasks[taskId].data[0] += 1;
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
                gTasks[taskId].data[0] += 1;
                gTasks[taskId].data[2] = DISPLAY_WIDTH as i16;
                gIntroSlideFlags &= 65534;
            }
        }
        3 => {
            if gBattle_WIN0V as i32 & 0xFF00 != 0x4C00 {
                gBattle_WIN0V += 0x3FC;
            }
            if gTasks[taskId].data[2] != 0 {
                gTasks[taskId].data[2] -= 2;
            }
            gBattle_BG1_X = gTasks[taskId].data[2] as u16;
            gBattle_BG2_X = (gTasks[taskId].data[2] as u16).wrapping_neg();
            if gTasks[taskId].data[2] == 0 {
                gTasks[taskId].data[0] += 1;
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
                            0x600e000 as usize as *mut c_void,
                            0x5000800,
                        );
                    }
                }
                SetGpuReg(REG_OFFSET_DISPCNT, GetGpuReg(REG_OFFSET_DISPCNT) & 49151);
                SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
                SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
                SetGpuReg(REG_OFFSET_BG1CNT, 39936);
                SetGpuReg(REG_OFFSET_BG2CNT, 24064);
                gScanlineEffect.state = 3;
                gTasks[taskId].data[0] += 1;
            }
        }
        5 => {
            BattleIntroSlideEnd(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawBattlerOnBg(
    bgId: i32,
    x: u8,
    y: u8,
    battlerPosition: u8,
    paletteId: u8,
    tiles: *mut u8,
    mut tilemap: *mut u16,
    tilesOffset: u16,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut battler: u8 = GetBattlerAtPosition(battlerPosition);
    let mut offset: i32 = tilesOffset as i32;
    CpuSet(
        ((*gMonSpritesGfxPtr).sprites.ptr[battlerPosition] as *mut u8)
            .at(BG_SCREEN_SIZE as i32 * gBattleMonForms[battler] as i32) as *mut c_void,
        tiles as *mut c_void,
        1024,
    );
    LoadBgTiles(bgId as u8, tiles as *mut c_void, 0x1000, tilesOffset);
    i = y as i32;
    while i < y as i32 + 8 {
        j = x as i32;
        while j < x as i32 + 8 {
            *tilemap.at(i * 32 + j) = offset as u16 | (paletteId as u16) << 12;
            offset += 1;
            j += 1;
        }
        i += 1;
    }
    LoadBgTilemap(bgId as u8, tilemap as *mut c_void, BG_SCREEN_SIZE as u16, 0);
}
pub(crate) unsafe extern "C" fn DrawBattlerOnBgDMA(
    x: u8,
    y: u8,
    battlerPosition: u8,
    arg3: u8,
    paletteId: u8,
    arg5: u16,
    arg6: u8,
    arg7: u8,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut offset: i32 = 0;
    {
        {
            {
                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                volatile_write(
                    dmaRegs,
                    ((*gMonSpritesGfxPtr).sprites.ptr[battlerPosition] as *mut u8)
                        .at(BG_SCREEN_SIZE as i32 * arg3 as i32) as *mut c_void
                        as usize as u32,
                );
                volatile_write(
                    dmaRegs.at(1),
                    (0x6000000 as usize as *mut c_void as *mut u8).at(arg5) as *mut c_void as usize
                        as u32,
                );
                volatile_write(dmaRegs.at(2), 0x80000400);
                let _ = (dmaRegs.at(2)).read_volatile();
            }
        }
    }
    offset = (arg5 >> 5) as i32 - ((arg7 as i32) << 9);
    i = y as i32;
    while i < y as i32 + 8 {
        j = x as i32;
        while j < x as i32 + 8 {
            *(BG_VRAM as usize as *mut u16)
                .at(i * 32)
                .at(j + ((arg6 as i32) << 10)) = offset as u16 | (paletteId as u16) << 12;
            offset += 1;
            j += 1;
        }
        i += 1;
    }
}
