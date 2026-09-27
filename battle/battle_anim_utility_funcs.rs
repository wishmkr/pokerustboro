//! Translated from `src/battle_anim_utility_funcs.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sCurseLinesPalette sBattleAnimBgCntSet sBattleAnimBgCntGet
#[allow(unused_imports)]
use crate::data::battle_anim_utility_funcs::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimStatsChangeData: *mut u8 = core::ptr::null_mut();
static mut SETANIMBGATTRIBUTE_SBGCNT: u16 = 0u16;

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimMaskImage_Curse: u8;
    static mut gBattleAnimMaskTilemap_Curse: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleEnvironment: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlerTarget: u8;
    static mut gContestResources: u8;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSprites: u8;
    static mut gStatAnim_Accuracy_Pal: u8;
    static mut gStatAnim_Attack_Pal: u8;
    static mut gStatAnim_Decrease_Tilemap: u8;
    static mut gStatAnim_Defense_Pal: u8;
    static mut gStatAnim_Evasion_Pal: u8;
    static mut gStatAnim_Gfx: u8;
    static mut gStatAnim_Increase_Tilemap: u8;
    static mut gStatAnim_Multiple_Pal: u8;
    static mut gStatAnim_SpAttack_Pal: u8;
    static mut gStatAnim_SpDefense_Pal: u8;
    static mut gStatAnim_Speed_Pal: u8;
    static mut gTasks: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemapHandleContest(a0: *mut u8, a1: *mut u8, a2: u32);
    fn BattleAnimAdjustPanning2(a0: i8) -> i8;
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn ClearBattleAnimBg(a0: u32);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn CreateInvisibleSpriteCopy(a0: i32, a1: u8, a2: i32) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteWithActiveSheet(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn Free(a0: *mut u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattleMonSpritePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8) -> u32;
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetSpritePalIdxByBattler(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn ResetBattleAnimBg(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn UnpackSelectedBattlePalettes(a0: i16) -> u32;
    fn UpdateAnimBg3ScreenSize(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendBattleAnimPal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
        );
        selectedPalettes = (selectedPalettes
            | GetBattleMonSpritePalettesMask(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    >> 7)
                    & 1i32) as u8),
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    >> 8)
                    & 1i32) as u8),
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    >> 9)
                    & 1i32) as u8),
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    >> 10)
                    & 1i32) as u8),
            ));
        StartBlendAnimSpriteColor(taskId, selectedPalettes);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendBattleAnimPalExclude(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = 0u8;
        let mut selectedPalettes: u32 = 0u32;
        let mut animBattlers = crate::ffi::Align4([0u8; 2]);
        (((&raw mut animBattlers).cast::<u8>()).wrapping_offset(1)).write(255u8);
        selectedPalettes = UnpackSelectedBattlePalettes(1i16);
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 2i32
                || __sw1 == 0i32
                || __sw1 == 3i32
                || __sw1 == 1i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            let mut __fall = false;
            if __sw1 == 2i32 {
                __fall = true;
                selectedPalettes = 0u32;
            }
            if __fall || __sw1 == 0i32 || !__matched {
                __fall = true;
                ((&raw mut animBattlers).cast::<u8>())
                    .write(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                selectedPalettes = 0u32;
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((&raw mut animBattlers).cast::<u8>())
                    .write(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                ((&raw mut animBattlers).cast::<u8>())
                    .write(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
                (((&raw mut animBattlers).cast::<u8>()).wrapping_offset(1))
                    .write(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                ((&raw mut animBattlers).cast::<u8>()).write(255u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                selectedPalettes = 0u32;
                ((&raw mut animBattlers).cast::<u8>()).write(
                    ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                        as u8),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                selectedPalettes = 0u32;
                ((&raw mut animBattlers).cast::<u8>()).write(
                    ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                );
                break 'l1;
            }
        }
        {
            battler = 0u8;
            'l2: loop {
                if !(((battler) as i32) < 4i32) {
                    break 'l2;
                }
                'l3: {
                    if ((((battler) as i32)
                        != ((((&raw mut animBattlers).cast::<u8>()).read()) as i32))
                        && (((battler) as i32)
                            != (((((&raw mut animBattlers).cast::<u8>()).wrapping_offset(1)).read())
                                as i32)))
                        && ((IsBattlerSpriteVisible(battler)) != 0)
                    {
                        selectedPalettes = (selectedPalettes
                            | ((crate::c::shl_i32(
                                65536i32,
                                ((GetSpritePalIdxByBattler(battler)) as u32),
                            )) as u32));
                    }
                }
                battler = (battler).wrapping_add(1);
            }
        }
        StartBlendAnimSpriteColor(taskId, selectedPalettes);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetCamouflageBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
        );
        'l1: {
            let __sw1 = ((((&raw mut gBattleEnvironment).cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(2828i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(2528i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(12062i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(18432i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(32459i16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(32459i16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(10774i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(3374i16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(32767i16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .write(32767i16);
                break 'l1;
            }
        }
        StartBlendAnimSpriteColor(taskId, selectedPalettes);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendParticle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut paletteIndex: u8 = IndexOfSpritePaletteTag(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u16),
        );
        let mut selectedPalettes: u32 =
            ((crate::c::shl_i32(1i32, ((((paletteIndex) as i32).wrapping_add(16i32)) as u32)))
                as u32);
        StartBlendAnimSpriteColor(taskId, selectedPalettes);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartBlendAnimSpriteColor(taskId: u8, selectedPalettes: u32) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes = selectedPalettes;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((selectedPalettes) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((selectedPalettes >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_BlendSpriteColor_Step2));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BlendSpriteColor_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = 0u32;
        let mut singlePaletteOffset: u16 = 0u16;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .read()) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .write(0i16);
            selectedPalettes = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                | (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    << 16)) as u32);
            'l1: loop {
                if !(selectedPalettes != 0u32) {
                    break 'l1;
                }
                if (selectedPalettes & 1u32) != 0 {
                    BlendPalette(
                        singlePaletteOffset,
                        16u16,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as u8),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as u16),
                    );
                }
                singlePaletteOffset = ((((singlePaletteOffset) as i32).wrapping_add(16i32)) as u16);
                selectedPalettes = (selectedPalettes >> 1);
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                < ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    > ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    DestroyAnimVisualTask(taskId);
                }
            }
        } else {
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HardwarePaletteFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginHardwarePaletteFade(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as u8),
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .read()) as u8),
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_HardwarePaletteFade_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_HardwarePaletteFade_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TraceMonBlended(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_TraceMonBlended_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_TraceMonBlended_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) != 0 {
            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                    CloneBattlerSpriteWithBlend(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                    ),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    >= 0i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((if ((((task).wrapping_add(8)).cast::<i16>()).read()) != 0 {
                            1i32
                        } else {
                            2i32
                        }) as u16) as i32,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read());
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((taskId) as i16));
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(5i16);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimMonTrace));
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p3).write(((__p3).read()).wrapping_sub(1));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
            }
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 0i32
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMonTrace(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize,
            );
            (__p2).write(((__p2).read()).wrapping_sub(1));
            DestroySpriteWithActiveSheet(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DrawFallingWhiteLinesOnAttacker(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species: u16 = 0u16;
        let mut spriteId: i32 = 0i32;
        let mut newSpriteId: i32 = 0i32;
        let mut var0: u16 = 0u16;
        let mut bg1Cnt: u16 = 0u16;
        let mut animBgData = crate::ffi::Align4([0u8; 16]);
        var0 = 0u16;
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16189u16);
        SetGpuRegBits(0u8, 32768u16);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 3080u16);
        bg1Cnt = GetGpuReg(10u8);
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            6,
            2,
            (0u16) as i32,
        );
        SetGpuReg(10u8, bg1Cnt);
        if !((IsContest()) != 0) {
            crate::c::bf_write(
                ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
                2,
                2,
                (1u16) as i32,
            );
            SetGpuReg(10u8, bg1Cnt);
        }
        if ((IsDoubleBattle()) != 0) && (!((IsContest()) != 0)) {
            if (((GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 3i32)
                || (((GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as i32)
                    == 0i32)
            {
                if ((IsBattlerSpriteVisible(
                    ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                        as u8),
                )) as i32)
                    == 1i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    ^ 2i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            false,
                        ) as u16) as i32)
                            .wrapping_sub(1i32)) as u16) as i32,
                    );
                    crate::c::bf_write(
                        ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
                        0,
                        2,
                        (1u16) as i32,
                    );
                    SetGpuReg(10u8, bg1Cnt);
                    var0 = 1u16;
                }
            }
        }
        if (IsContest()) != 0 {
            species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read();
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                species = ((GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            } else {
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            }
        }
        spriteId = ((GetAnimBattlerSpriteId(0u8)) as i32);
        newSpriteId = ((CreateInvisibleSpriteCopy(
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32),
            ((spriteId) as u8),
            ((species) as i32),
        )) as i32);
        GetBattleAnimBg1Data((&raw mut animBgData).cast::<u8>());
        AnimLoadCompressedBgTilemapHandleContest(
            (&raw mut animBgData).cast::<u8>(),
            (((&raw mut gBattleAnimMaskTilemap_Curse).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
        );
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBgData).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gBattleAnimMaskImage_Curse).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBgData).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        LoadPalette(
            (((&raw const sCurseLinesPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            ((((0i32).wrapping_add(
                (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            ))
            .wrapping_add(1i32)) as u16),
            2u16,
        );
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
            (((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((spriteId) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_neg())
            .wrapping_add(32i32)) as u16),
        );
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
            (((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((spriteId) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_neg())
            .wrapping_add(32i32)) as u16),
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((newSpriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((var0) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_DrawFallingWhiteLinesOnAttacker_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DrawFallingWhiteLinesOnAttacker_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBgData = crate::ffi::Align4([0u8; 16]);
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut bg1Cnt: u16 = 0u16;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        let __p2 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(4i32)) as u16));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 64i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            let __p3 = (&raw mut gBattle_BG1_Y).cast::<u16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(64i32)) as u16));
            if (({
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == 4i32
            {
                ResetBattleAnimBg(0u8);
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                SetGpuReg(72u8, 16191u16);
                SetGpuReg(74u8, 16191u16);
                if !((IsContest()) != 0) {
                    bg1Cnt = GetGpuReg(10u8);
                    crate::c::bf_write(
                        ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
                        2,
                        2,
                        (0u16) as i32,
                    );
                    SetGpuReg(10u8, bg1Cnt);
                }
                SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) ^ 32768i32) as u16));
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                sprite = ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68);
                sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize
                        * 68,
                );
                DestroySprite(sprite);
                GetBattleAnimBg1Data((&raw mut animBgData).cast::<u8>());
                ClearBattleAnimBg(
                    (((((&raw mut animBgData).cast::<u8>()).wrapping_add(9)).read()) as u32),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as i32)
                    == 1i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    ^ 2i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            false,
                        ) as u16)
                            .wrapping_add(1)) as i32,
                    );
                }
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitStatsChangeAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        ((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(24u32));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sAnimStatsChangeData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4))
                    .cast::<i16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(StatsChangeAnimation_Step1));
    }
}
pub(crate) unsafe extern "C" fn StatsChangeAnimation_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i16>())
        .wrapping_offset(2))
        .read())
            != 0)
        {
            (((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .write(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
        } else {
            (((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .write(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
        }
        ((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .write(
            (((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                ^ 2i32) as u8),
        );
        if ((IsContest()) != 0)
            || (((((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i16>())
            .wrapping_offset(3))
            .read())
                != 0)
                && (!((IsBattlerSpriteVisible(
                    ((((&raw mut sAnimStatsChangeData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read(),
                )) != 0)))
        {
            ((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
        }
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16189u16);
        SetGpuRegBits(0u8, 32768u16);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        SetAnimBgAttribute(1u8, 4u8, 0u8);
        SetAnimBgAttribute(1u8, 0u8, 0u8);
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 3u8, 1u8);
        }
        if ((IsDoubleBattle()) != 0)
            && (!((((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i16>())
            .wrapping_offset(3))
            .read())
                != 0))
        {
            if (((GetBattlerPosition(
                (((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read(),
            )) as i32)
                == 3i32)
                || (((GetBattlerPosition(
                    (((&raw mut sAnimStatsChangeData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .read(),
                )) as i32)
                    == 0i32)
            {
                if ((IsBattlerSpriteVisible(
                    ((((&raw mut sAnimStatsChangeData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read(),
                )) as i32)
                    == 1i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sAnimStatsChangeData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut sAnimStatsChangeData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            false,
                        ) as u16)
                            .wrapping_sub(1)) as i32,
                    );
                    SetAnimBgAttribute(1u8, 4u8, 1u8);
                    ((((&raw mut sAnimStatsChangeData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2))
                    .write(1u8);
                }
            }
        }
        if (IsContest()) != 0 {
            ((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<u16>())
            .write(
                ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read(),
            );
        } else {
            if ((GetBattlerSide(
                (((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read(),
            )) as i32)
                != 0i32
            {
                ((((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(20)
                .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut sAnimStatsChangeData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .read()) as i32) as isize,
                                ))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16),
                );
            } else {
                ((((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(20)
                .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut sAnimStatsChangeData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .read()) as i32) as isize,
                                ))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16),
                );
            }
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(StatsChangeAnimation_Step2));
    }
}
pub(crate) unsafe extern "C" fn StatsChangeAnimation_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBgData = crate::ffi::Align4([0u8; 16]);
        let mut spriteId: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut battlerSpriteId: u8 = 0u8;
        spriteId2 = 0u8;
        battlerSpriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            (((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32) as isize,
        ))
        .read();
        spriteId = CreateInvisibleSpriteCopy(
            (((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32),
            battlerSpriteId,
            ((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<u16>())
            .read()) as i32),
        );
        if (((((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i16>())
        .wrapping_offset(3))
        .read())
            != 0
        {
            battlerSpriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize,
            ))
            .read();
            spriteId2 = CreateInvisibleSpriteCopy(
                ((((((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32),
                battlerSpriteId,
                ((((((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(20)
                .cast::<u16>())
                .read()) as i32),
            );
        }
        GetBattleAnimBg1Data((&raw mut animBgData).cast::<u8>());
        if !(((((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i16>())
        .read())
            != 0)
        {
            AnimLoadCompressedBgTilemapHandleContest(
                (&raw mut animBgData).cast::<u8>(),
                (((&raw mut gStatAnim_Increase_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                0u32,
            );
        } else {
            AnimLoadCompressedBgTilemapHandleContest(
                (&raw mut animBgData).cast::<u8>(),
                (((&raw mut gStatAnim_Decrease_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                0u32,
            );
        }
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBgData).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gStatAnim_Gfx).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBgData).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        'l1: {
            let __sw1 = ((((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_Attack_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_Defense_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_Accuracy_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_Speed_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_Evasion_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_SpAttack_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_SpDefense_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
            if !__matched {
                LoadCompressedPalette(
                    ((&raw mut gStatAnim_Multiple_Pal).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                break 'l1;
            }
        }
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        if (((((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i16>())
        .read()) as i32)
            == 1i32
        {
            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(64u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((-3i16));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(3i16);
        }
        if !((((((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i16>())
        .wrapping_offset(4))
        .read())
            != 0)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(10i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(20i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(13i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(30i16);
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((spriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i16>())
            .wrapping_offset(3))
            .read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((spriteId2) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(
            ((((((&raw mut sAnimStatsChangeData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sAnimStatsChangeData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(StatsChangeAnimation_Step3));
        if !(((((((&raw mut sAnimStatsChangeData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i16>())
        .read())
            != 0)
        {
            PlaySE12WithPanning(239u16, BattleAnimAdjustPanning2((-64i8)));
        } else {
            PlaySE12WithPanning(245u16, BattleAnimAdjustPanning2((-64i8)));
        }
    }
}
pub(crate) unsafe extern "C" fn StatsChangeAnimation_Step3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
            )) as u16),
        );
        'l1: {
            let __sw2 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32);
            if __sw2 == 0i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t4 = (__p3).read();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    __t4
                }) as i32)
                    > 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32)
                        == ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as i32)
                    {
                        let __p6 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                if (({
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32)
                {
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                if (({
                    let __p10 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t11 = (__p10).read();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    __t11
                }) as i32)
                    > 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p12).write(((__p12).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32)
                        == 0i32
                    {
                        ResetBattleAnimBg(0u8);
                        let __p13 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p13).write(((__p13).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                SetGpuReg(72u8, 16191u16);
                SetGpuReg(74u8, 16191u16);
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) ^ 32768i32) as u16));
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as i32)
                    == 1i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(7))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(7))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            false,
                        ) as u16)
                            .wrapping_add(1)) as i32,
                    );
                }
                {
                    Free(
                        ((&raw mut sAnimStatsChangeData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((&raw mut sAnimStatsChangeData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                }
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Flash(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = GetBattleMonSpritePalettesMask(1u8, 1u8, 1u8, 1u8);
        SetPalettesToColor(selectedPalettes, 0u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(((selectedPalettes >> 16) as i16));
        selectedPalettes = (GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8) & 65535u32);
        SetPalettesToColor(selectedPalettes, 65535u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((selectedPalettes) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_Flash_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Flash_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 6i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(16i16);
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) < 16i32) {
                                break 'l2;
                            }
                            'l3: {
                                if (crate::c::shr_i32(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(15))
                                    .read()) as i32),
                                    ((i) as u32),
                                ) & 1i32)
                                    != 0
                                {
                                    BlendPalette(
                                        (((0i32).wrapping_add(((i) as i32).wrapping_mul(16i32)))
                                            as u16),
                                        16u16,
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(2))
                                        .read()) as u8),
                                        65535u16,
                                    );
                                }
                                if (crate::c::shr_i32(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(14))
                                    .read()) as i32),
                                    ((i) as u32),
                                ) & 1i32)
                                    != 0
                                {
                                    BlendPalette(
                                        (((256i32).wrapping_add(((i) as i32).wrapping_mul(16i32)))
                                            as u16),
                                        16u16,
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(2))
                                        .read()) as u8),
                                        0u16,
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 0i32
                    {
                        let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPalettesToColor(selectedPalettes: u32, color: u16) {
    unsafe {
        let mut selectedPalettes = selectedPalettes;
        let mut color = color;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    if (selectedPalettes & 1u32) != 0 {
                        let mut curOffset: u16 = ((((i) as i32).wrapping_mul(16i32)) as u16);
                        let mut paletteOffset: u16 = curOffset;
                        'l3: loop {
                            if !(((curOffset) as i32)
                                < ((paletteOffset) as i32).wrapping_add(16i32))
                            {
                                break 'l3;
                            }
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((curOffset) as i32) as isize))
                            .write(color);
                            curOffset = (curOffset).wrapping_add(1);
                        }
                    }
                    selectedPalettes = (selectedPalettes >> 1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendNonAttackerPalettes(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u32 = 0u32;
        let mut j: i32 = 0i32;
        let mut selectedPalettes: u32 = 0u32;
        {
            battler = 0u32;
            'l1: loop {
                if !(battler < 4u32) {
                    break 'l1;
                }
                'l2: {
                    if ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as u32) != battler {
                        selectedPalettes = (selectedPalettes
                            | ((crate::c::shl_i32(1i32, (battler).wrapping_add(16u32))) as u32));
                    }
                }
                battler = (battler).wrapping_add(1);
            }
        }
        {
            j = 5i32;
            'l3: loop {
                if !(j != 0i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset((j) as isize))
                    .write(
                        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((j).wrapping_sub(1i32)) as isize))
                        .read(),
                    );
                }
                j = (j).wrapping_sub(1);
            }
        }
        StartBlendAnimSpriteColor(taskId, selectedPalettes);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StartSlidingBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut newTaskId: u8 = 0u8;
        UpdateAnimBg3ScreenSize(0u8);
        newTaskId = CreateTask(Some(AnimTask_UpdateSlidingBg), 5u8);
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
            .read())
            != 0)
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32)
        {
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((newTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((newTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((newTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((newTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_UpdateSlidingBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
            )) as i16),
        );
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32),
            )) as i16),
        );
        let __p3 = (&raw mut gBattle_BG3_X).cast::<u16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    >> 8),
            )) as u16),
        );
        let __p4 = (&raw mut gBattle_BG3_Y).cast::<u16>();
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as i32)
                    >> 8),
            )) as u16),
        );
        let __p5 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p5).write((((((__p5).read()) as i32) & 255i32) as i16));
        let __p6 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11);
        (__p6).write((((((__p6).read()) as i32) & 255i32) as i16));
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
        {
            ((&raw mut gBattle_BG3_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
            UpdateAnimBg3ScreenSize(1u8);
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetAttackerSide(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .write(((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i16));
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetTargetSide(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .write(((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i16));
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetTargetIsAttackerPartner(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(
            (((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                == ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32))
                as i16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAllNonAttackersInvisiblity(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u16 = 0u16;
        {
            battler = 0u16;
            'l1: loop {
                if !(((battler) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((battler) as i32)
                        != ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32))
                        && ((IsBattlerSpriteVisible(((battler) as u8))) != 0)
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as u16) as i32,
                        );
                    }
                }
                battler = (battler).wrapping_add(1);
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMonScrollingBgMask(
    taskId: u8,
    unused: i32,
    scrollSpeed: u16,
    battler: u8,
    includePartner: u8,
    numFadeSteps: u8,
    fadeStepDelay: u8,
    duration: u8,
    gfx: *mut u32,
    tilemap: *mut u32,
    palette: *mut u32,
) {
    unsafe {
        let mut taskId = taskId;
        let mut unused = unused;
        let mut scrollSpeed = scrollSpeed;
        let mut battler = battler;
        let mut includePartner = includePartner;
        let mut numFadeSteps = numFadeSteps;
        let mut fadeStepDelay = fadeStepDelay;
        let mut duration = duration;
        let mut gfx = gfx;
        let mut tilemap = tilemap;
        let mut palette = palette;
        let mut species: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut bg1Cnt: u16 = 0u16;
        let mut animBgData = crate::ffi::Align4([0u8; 16]);
        let mut battler2: u8 = 0u8;
        spriteId2 = 0u8;
        battler2 = ((((battler) as i32) ^ 2i32) as u8);
        if ((IsContest()) != 0)
            || (((includePartner) != 0) && (!((IsBattlerSpriteVisible(battler2)) != 0)))
        {
            includePartner = 0u8;
        }
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16189u16);
        SetGpuRegBits(0u8, 32768u16);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        bg1Cnt = GetGpuReg(10u8);
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            6,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            5,
            1,
            (1u16) as i32,
        );
        if !((IsContest()) != 0) {
            crate::c::bf_write(
                ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
                2,
                2,
                (1u16) as i32,
            );
        }
        SetGpuReg(10u8, bg1Cnt);
        if (IsContest()) != 0 {
            species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read();
        } else {
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                species = ((GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            } else {
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            }
        }
        spriteId = CreateInvisibleSpriteCopy(
            ((battler) as i32),
            (((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read(),
            ((species) as i32),
        );
        if (includePartner) != 0 {
            spriteId2 = CreateInvisibleSpriteCopy(
                ((battler2) as i32),
                (((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler2) as i32) as isize))
                .read(),
                ((species) as i32),
            );
        }
        GetBattleAnimBg1Data((&raw mut animBgData).cast::<u8>());
        AnimLoadCompressedBgTilemapHandleContest(
            (&raw mut animBgData).cast::<u8>(),
            (tilemap).cast::<u8>(),
            0u32,
        );
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBgData).cast::<u8>()).wrapping_add(9)).read()) as u32),
            gfx,
            (((((&raw mut animBgData).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        LoadCompressedPalette(
            palette,
            (((0i32).wrapping_add(
                (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((scrollSpeed) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((numFadeSteps) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((duration) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((fadeStepDelay) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((spriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((includePartner) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((spriteId2) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(UpdateMonScrollingBgMask));
    }
}
pub(crate) unsafe extern "C" fn UpdateMonScrollingBgMask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    < 0i32
                {
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_neg()
                } else {
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                }),
            )) as i16),
        );
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            < 0i32
        {
            let __p2 = (&raw mut gBattle_BG1_Y).cast::<u16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .read()) as i32)
                        >> 8),
                )) as u16),
            );
        } else {
            let __p3 = (&raw mut gBattle_BG1_Y).cast::<u16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .read()) as i32)
                        >> 8),
                )) as u16),
            );
        }
        let __p4 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13);
        (__p4).write((((((__p4).read()) as i32) & 255i32) as i16));
        'l1: {
            let __sw5 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32);
            if __sw5 == 0i32 {
                if (({
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t7 = (__p6).read();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    __t7
                }) as i32)
                    >= ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32)
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32)
                        == ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as i32)
                    {
                        let __p9 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw5 == 1i32 {
                if (({
                    let __p10 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32)
                {
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw5 == 2i32 {
                if (({
                    let __p13 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t14 = (__p13).read();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                    __t14
                }) as i32)
                    >= ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32)
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    let __p15 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p15).write(((__p15).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32)
                        == 0i32
                    {
                        ResetBattleAnimBg(0u8);
                        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                        SetGpuReg(72u8, 16191u16);
                        SetGpuReg(74u8, 16191u16);
                        if !((IsContest()) != 0) {
                            let mut bg1Cnt: u16 = GetGpuReg(10u8);
                            crate::c::bf_write(
                                ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
                                2,
                                2,
                                (0u16) as i32,
                            );
                            SetGpuReg(10u8, bg1Cnt);
                        }
                        SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) ^ 32768i32) as u16));
                        SetGpuReg(80u8, 0u16);
                        SetGpuReg(82u8, 0u16);
                        DestroySprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            DestroySprite(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(3))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                            );
                        }
                        DestroyAnimVisualTask(taskId);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetBattleEnvironment(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .write(((((&raw mut gBattleEnvironment).cast::<u8>()).read()) as i16));
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AllocBackupPalBuffer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
            .wrapping_add(380)
            .cast::<*mut u16>())
        .write(
            (AllocZeroed((((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(4i32)) as u32)))
                .cast::<u16>(),
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FreeBackupPalBuffer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        {
            Free(
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(380)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CopyPalUnfadedToBackup(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = 0u32;
        let mut paletteIndex: i32 = 0i32;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            selectedPalettes = GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8);
            'l1: loop {
                if !((selectedPalettes & 1u32) == 0u32) {
                    break 'l1;
                }
                selectedPalettes = (selectedPalettes >> 1);
                paletteIndex = (paletteIndex).wrapping_add(1);
            }
        } else {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 1i32
            {
                paletteIndex = ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                    .wrapping_add(16i32);
            } else {
                if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 2i32
                {
                    paletteIndex = ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                        .wrapping_add(16i32);
                }
            }
        }
        crate::c::memcpy(
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_mul(16i32)) as isize,
            ))
            .cast::<u8>(),
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((paletteIndex).wrapping_mul(16i32)) as isize))
            .cast::<u8>(),
            32u32,
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CopyPalUnfadedFromBackup(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = 0u32;
        let mut paletteIndex: i32 = 0i32;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            selectedPalettes = GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8);
            'l1: loop {
                if !((selectedPalettes & 1u32) == 0u32) {
                    break 'l1;
                }
                selectedPalettes = (selectedPalettes >> 1);
                paletteIndex = (paletteIndex).wrapping_add(1);
            }
        } else {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 1i32
            {
                paletteIndex = ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                    .wrapping_add(16i32);
            } else {
                if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 2i32
                {
                    paletteIndex = ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                        .wrapping_add(16i32);
                }
            }
        }
        crate::c::memcpy(
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((paletteIndex).wrapping_mul(16i32)) as isize))
            .cast::<u8>(),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_mul(16i32)) as isize,
            ))
            .cast::<u8>(),
            32u32,
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CopyPalFadedToUnfaded(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = 0u32;
        let mut paletteIndex: i32 = 0i32;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            selectedPalettes = GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8);
            'l1: loop {
                if !((selectedPalettes & 1u32) == 0u32) {
                    break 'l1;
                }
                selectedPalettes = (selectedPalettes >> 1);
                paletteIndex = (paletteIndex).wrapping_add(1);
            }
        } else {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 1i32
            {
                paletteIndex = ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                    .wrapping_add(16i32);
            } else {
                if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 2i32
                {
                    paletteIndex = ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                        .wrapping_add(16i32);
                }
            }
        }
        crate::c::memcpy(
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((paletteIndex).wrapping_mul(16i32)) as isize))
            .cast::<u8>(),
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((paletteIndex).wrapping_mul(16i32)) as isize))
            .cast::<u8>(),
            32u32,
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsContest(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (IsContest()) != 0 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        } else {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAnimAttackerAndTargetForEffectTgt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gBattleAnimAttacker).cast::<u8>())
            .write(((&raw mut gBattlerTarget).cast::<u8>()).read());
        ((&raw mut gBattleAnimTarget).cast::<u8>())
            .write(((&raw mut gEffectBattler).cast::<u8>()).read());
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsTargetSameSide(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
            == ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        } else {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAnimTargetToBattlerTarget(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gBattleAnimTarget).cast::<u8>())
            .write(((&raw mut gBattlerTarget).cast::<u8>()).read());
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAnimAttackerAndTargetForEffectAtk(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gBattleAnimAttacker).cast::<u8>())
            .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
        ((&raw mut gBattleAnimTarget).cast::<u8>())
            .write(((&raw mut gEffectBattler).cast::<u8>()).read());
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAttackerInvisibleWaitForSignal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (IsContest()) != 0 {
            DestroyAnimVisualTask(taskId);
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(0),
                    0,
                    1,
                    false,
                ) as u16) as i16),
            );
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(0),
                0,
                1,
                (1u16) as i32,
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_WaitAndRestoreVisibility));
            let __p1 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_WaitAndRestoreVisibility(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            == 4096i32
        {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(0),
                0,
                1,
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8) as i32)
                    & 1i32) as u16) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetAnimBgAttribute(bgId: u8, attributeId: u8, value: u8) {
    unsafe {
        let mut bgId = bgId;
        let mut attributeId = attributeId;
        let mut value = value;
        if ((bgId) as i32) < 4i32 {
            (&raw mut SETANIMBGATTRIBUTE_SBGCNT).write(GetGpuReg(
                ((((&raw const sBattleAnimBgCntSet).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((bgId) as i32) as isize))
                .read(),
            ));
            'l1: {
                let __sw1 = ((attributeId) as i32);
                if __sw1 == 0i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(1),
                        6,
                        2,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(1),
                        5,
                        1,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(0),
                        6,
                        1,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(0),
                        2,
                        2,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(0),
                        0,
                        2,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(0),
                        7,
                        1,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    crate::c::bf_write(
                        ((&raw mut SETANIMBGATTRIBUTE_SBGCNT).cast::<u8>()).wrapping_add(1),
                        0,
                        5,
                        ((value) as u16) as i32,
                    );
                    break 'l1;
                }
            }
            SetGpuReg(
                ((((&raw const sBattleAnimBgCntSet).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((bgId) as i32) as isize))
                .read(),
                (&raw mut SETANIMBGATTRIBUTE_SBGCNT).read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAnimBgAttribute(bgId: u8, attributeId: u8) -> i32 {
    unsafe {
        let mut bgId = bgId;
        let mut attributeId = attributeId;
        let mut bgCnt: u16 = 0u16;
        if ((bgId) as i32) < 4i32 {
            bgCnt = GetGpuReg(
                ((((&raw const sBattleAnimBgCntGet).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((bgId) as i32) as isize))
                .read(),
            );
            'l1: {
                let __sw1 = ((attributeId) as i32);
                if __sw1 == 0i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(1),
                        6,
                        2,
                        false,
                    ) as u16) as i32);
                }
                if __sw1 == 1i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(1),
                        5,
                        1,
                        false,
                    ) as u16) as i32);
                }
                if __sw1 == 2i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(0),
                        6,
                        1,
                        false,
                    ) as u16) as i32);
                }
                if __sw1 == 3i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(0),
                        2,
                        2,
                        false,
                    ) as u16) as i32);
                }
                if __sw1 == 4i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u16) as i32);
                }
                if __sw1 == 5i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(0),
                        7,
                        1,
                        false,
                    ) as u16) as i32);
                }
                if __sw1 == 6i32 {
                    return ((crate::c::bf_read(
                        ((&raw mut bgCnt).cast::<u8>()).wrapping_add(1),
                        0,
                        5,
                        false,
                    ) as u16) as i32);
                }
            }
        }
        return 0i32;
    }
}
