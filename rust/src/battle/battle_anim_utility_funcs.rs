//! Translated from `src/battle_anim_utility_funcs.c` by tools/rustport/c2rs.py.
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
    unused_assignments,
    unused_variables
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    BattleAnimAdjustPanning2, DestroyAnimVisualTask, IsBattlerSpriteVisible, IsContest,
    ResetBattleAnimBg, gAnimVisualTaskCount, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimLoadCompressedBgGfx, AnimLoadCompressedBgTilemapHandleContest, ClearBattleAnimBg,
    CloneBattlerSpriteWithBlend, CreateInvisibleSpriteCopy, DestroySpriteWithActiveSheet,
    GetAnimBattlerSpriteId, GetBattleAnimBg1Data, GetBattleMonSpritePalettesMask,
    GetBattlePalettesMask, GetBattlerPosition, GetBattlerSide, GetSpritePalIdxByBattler,
    IsDoubleBattle, UpdateAnimBg3ScreenSize,
};
use crate::battle_anim_normal::UnpackSelectedBattlePalettes;
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG3_X, gBattle_BG3_Y, gBattle_WIN0H, gBattle_WIN0V,
    gBattleEnvironment, gBattleSpritesDataPtr, gBattlerAttacker, gBattlerTarget, gEffectBattler,
    gMonSpritesGfxPtr,
};
use crate::battle_main::{gBattlerPartyIndexes, gBattlerSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestResources;
use crate::gpu_regs::{GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::palette::{BeginHardwarePaletteFade, LoadCompressedPalette, LoadPalette, gPaletteFade};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon::{GetMonData2, gEnemyParty, gPlayerParty};
use crate::sound::PlaySE12WithPanning;
use crate::sprite::IndexOfSpritePaletteTag;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
use crate::task::{gTasks, task_func, task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
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
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const tAnimSpriteId1: usize = 0;
const tVelocity: usize = 1;
const aIsTarget: usize = 2;
const tMultipleBattlers: usize = 2;
const aMultipleBattlers: usize = 3;
const tAnimSpriteId2: usize = 3;
const tTargetBlend: usize = 4;
const tWaitTime: usize = 5;
const tHidBattler2: usize = 6;
const tBattler2SpriteId: usize = 7;
const tWaitTimer: usize = 10;
const tFadeTimer: usize = 11;
const tBlend: usize = 12;
const tState: usize = 15;
// Data tables (translate with cdata.py): sCurseLinesPalette sBattleAnimBgCntSet sBattleAnimBgCntGet

/// `struct AnimStatsChangeData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct AnimStatsChangeData {
    pub battler1: u8,
    pub battler2: u8,
    pub hidBattler2: u8,
    pub data: CArray<i16, 8>,
    pub species: u16,
}

unsafe impl Sync for AnimStatsChangeData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<AnimStatsChangeData>() == 24);
    assert!(offset_of!(AnimStatsChangeData, battler1) == 0);
    assert!(offset_of!(AnimStatsChangeData, battler2) == 1);
    assert!(offset_of!(AnimStatsChangeData, hidBattler2) == 2);
    assert!(offset_of!(AnimStatsChangeData, data) == 4);
    assert!(offset_of!(AnimStatsChangeData, species) == 20);
};

static sBattleAnimBgCntGet: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_anim_utility_funcs::sBattleAnimBgCntGet).cast());
static sBattleAnimBgCntSet: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_anim_utility_funcs::sBattleAnimBgCntSet).cast());
static sCurseLinesPalette: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::battle_anim_utility_funcs::sCurseLinesPalette).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sAnimStatsChangeData: crate::global::Global<*mut AnimStatsChangeData> =
    crate::global::Global::new(null_mut());
static SetAnimBgAttribute_sBgCnt: crate::global::Global<u16> = crate::global::Global::new(0);

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}

#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendBattleAnimPal(taskId: u8) {
    let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(crate::gl!(gBattleAnimArgs[0]));
    selectedPalettes |= GetBattleMonSpritePalettesMask(
        (crate::gl!(gBattleAnimArgs[0]) >> 7) as u8 & 1,
        (crate::gl!(gBattleAnimArgs[0]) >> 8) as u8 & 1,
        (crate::gl!(gBattleAnimArgs[0]) >> 9) as u8 & 1,
        (crate::gl!(gBattleAnimArgs[0]) >> 10) as u8 & 1,
    );
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendBattleAnimPalExclude(taskId: u8) {
    let mut animBattlers: CArray<u8, 2> = zeroed();
    animBattlers[1] = 0xFF;
    let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(F_PAL_BG);
    'l1: {
        let sw1: i16 = crate::gl!(gBattleAnimArgs[0]);
        let matched = sw1 == 2
            || sw1 == 0
            || sw1 == 3
            || sw1 == 1
            || sw1 == 4
            || sw1 == 5
            || sw1 == 6
            || sw1 == 7;
        let mut fall = false;
        if sw1 == 2 {
            fall = true;
            selectedPalettes = 0;
        }
        if fall || sw1 == 0 || !matched {
            animBattlers[0] = gBattleAnimAttacker.get();
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            selectedPalettes = 0;
        }
        if fall || sw1 == 1 {
            animBattlers[0] = gBattleAnimTarget.get();
            break 'l1;
        }
        if sw1 == 4 {
            animBattlers[0] = gBattleAnimAttacker.get();
            animBattlers[1] = gBattleAnimTarget.get();
            break 'l1;
        }
        if sw1 == 5 {
            animBattlers[0] = 0xFF;
            break 'l1;
        }
        if sw1 == 6 {
            selectedPalettes = 0;
            animBattlers[0] = gBattleAnimAttacker.get() ^ 2;
            break 'l1;
        }
        if sw1 == 7 {
            selectedPalettes = 0;
            animBattlers[0] = gBattleAnimTarget.get() ^ 2;
            break 'l1;
        }
    }
    for battler in 0..MAX_BATTLERS_COUNT {
        if battler != animBattlers[0]
            && battler != animBattlers[1]
            && IsBattlerSpriteVisible(battler) != 0
        {
            selectedPalettes |= shl_i32(0x10000, GetSpritePalIdxByBattler(battler) as u32) as u32;
        }
    }
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetCamouflageBlend(taskId: u8) {
    let selectedPalettes: u32 = UnpackSelectedBattlePalettes(crate::gl!(gBattleAnimArgs[0]));
    match gBattleEnvironment.get() {
        BATTLE_ENVIRONMENT_GRASS => {
            crate::gl!(gBattleAnimArgs[4] = 2828);
        }
        BATTLE_ENVIRONMENT_LONG_GRASS => {
            crate::gl!(gBattleAnimArgs[4] = 2528);
        }
        BATTLE_ENVIRONMENT_SAND => {
            crate::gl!(gBattleAnimArgs[4] = 12062);
        }
        BATTLE_ENVIRONMENT_UNDERWATER => {
            crate::gl!(gBattleAnimArgs[4] = 18432);
        }
        BATTLE_ENVIRONMENT_WATER => {
            crate::gl!(gBattleAnimArgs[4] = 32459);
        }
        BATTLE_ENVIRONMENT_POND => {
            crate::gl!(gBattleAnimArgs[4] = 32459);
        }
        BATTLE_ENVIRONMENT_MOUNTAIN => {
            crate::gl!(gBattleAnimArgs[4] = 10774);
        }
        BATTLE_ENVIRONMENT_CAVE => {
            crate::gl!(gBattleAnimArgs[4] = 3374);
        }
        BATTLE_ENVIRONMENT_BUILDING => {
            crate::gl!(gBattleAnimArgs[4] = 32767);
        }
        BATTLE_ENVIRONMENT_PLAIN => {
            crate::gl!(gBattleAnimArgs[4] = 32767);
        }
        _ => {}
    }
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendParticle(taskId: u8) {
    let paletteIndex: u8 = IndexOfSpritePaletteTag(crate::gl!(gBattleAnimArgs[0]) as u16);
    let selectedPalettes: u32 = shl_i32(1, paletteIndex as u32 + 16) as u32;
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
pub unsafe fn StartBlendAnimSpriteColor(taskId: u8, selectedPalettes: u32) {
    task_set(taskId, 0, selectedPalettes as i16);
    task_set(taskId, 1, (selectedPalettes >> 16) as i16);
    task_set(taskId, 2, crate::gl!(gBattleAnimArgs[1]));
    task_set(taskId, 3, crate::gl!(gBattleAnimArgs[2]));
    task_set(taskId, 4, crate::gl!(gBattleAnimArgs[3]));
    task_set(taskId, 5, crate::gl!(gBattleAnimArgs[4]));
    task_set(taskId, 10, crate::gl!(gBattleAnimArgs[2]));
    task_set_func(taskId, Some(AnimTask_BlendSpriteColor_Step2));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_BlendSpriteColor_Step2(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut singlePaletteOffset: u16 = 0;
    if task_get(taskId, 9) == task_get(taskId, 2) {
        task_set(taskId, 9, 0);
        selectedPalettes = task_get(taskId, 0) as u32 | (task_get(taskId, 1) as u32) << 16;
        while selectedPalettes != 0 {
            if selectedPalettes & 1 != 0 {
                BlendPalette(
                    singlePaletteOffset,
                    16,
                    task_get(taskId, 10) as u8,
                    task_get(taskId, 5) as u16,
                );
            }
            singlePaletteOffset += 16;
            selectedPalettes >>= 1;
        }
        if task_get(taskId, 10) < task_get(taskId, 4) {
            task_set(taskId, 10, task_get(taskId, 10) + 1);
        } else if task_get(taskId, 10) > task_get(taskId, 4) {
            task_set(taskId, 10, task_get(taskId, 10) - 1);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 9, task_get(taskId, 9) + 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_HardwarePaletteFade(taskId: u8) {
    BeginHardwarePaletteFade(
        crate::gl!(gBattleAnimArgs[0]) as u8,
        crate::gl!(gBattleAnimArgs[1]) as u8,
        crate::gl!(gBattleAnimArgs[2]) as u8,
        crate::gl!(gBattleAnimArgs[3]) as u8,
        crate::gl!(gBattleAnimArgs[4]) as u8,
    );
    task_set_func(taskId, Some(AnimTask_HardwarePaletteFade_Step));
}
pub(crate) unsafe fn AnimTask_HardwarePaletteFade_Step(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_TraceMonBlended(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = crate::gl!(gBattleAnimArgs[0]);
    (*task).data[1] = 0;
    (*task).data[2] = crate::gl!(gBattleAnimArgs[1]);
    (*task).data[3] = crate::gl!(gBattleAnimArgs[2]);
    (*task).data[4] = crate::gl!(gBattleAnimArgs[3]);
    (*task).data[5] = 0;
    (*task).func = Some(AnimTask_TraceMonBlended_Step);
}
pub(crate) unsafe fn AnimTask_TraceMonBlended_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[4] != 0 {
        if (*task).data[1] != 0 {
            (*task).data[1] -= 1;
        } else {
            (*task).data[6] = CloneBattlerSpriteWithBlend((*task).data[0] as u8);
            if (*task).data[6] >= 0 {
                (*gSprites.as_ptr())[(*task).data[6]]
                    .oam
                    .set_priority((if (*task).data[0] != 0 { 1 } else { 2 }) as u16);
                {
                    let gl_i0 = (*task).data[6];
                    crate::gl!(gSprites[gl_i0].data[0] = (*task).data[3])
                };
                {
                    let gl_i1 = (*task).data[6];
                    crate::gl!(gSprites[gl_i1].data[1] = taskId as i16)
                };
                {
                    let gl_i2 = (*task).data[6];
                    crate::gl!(gSprites[gl_i2].data[2] = 5)
                };
                {
                    let gl_i3 = (*task).data[6];
                    crate::gl!(gSprites[gl_i3].callback = Some(AnimMonTrace))
                };
                (*task).data[5] += 1;
            }
            (*task).data[4] -= 1;
            (*task).data[1] = (*task).data[2];
        }
    } else if (*task).data[5] == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimMonTrace(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[0] -= 1;
    } else {
        task_set(
            (*sprite).data[1],
            (*sprite).data[2],
            task_get((*sprite).data[1], (*sprite).data[2]) - 1,
        );
        DestroySpriteWithActiveSheet(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DrawFallingWhiteLinesOnAttacker(taskId: u8) {
    let mut species: u16 = 0;
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut var0: u16 = 0;
    gBattle_WIN0H.set(0);
    gBattle_WIN0V.set(0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 3080);
    let mut bg1Cnt: u16 = GetGpuReg(REG_OFFSET_BG1CNT);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_screenSize(0);
    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    if IsContest() == 0 {
        (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(1);
        SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    }
    if IsDoubleBattle() != 0
        && IsContest() == 0
        && (GetBattlerPosition(gBattleAnimAttacker.get()) == B_POSITION_OPPONENT_RIGHT
            || GetBattlerPosition(gBattleAnimAttacker.get()) == B_POSITION_PLAYER_LEFT)
        && IsBattlerSpriteVisible(gBattleAnimAttacker.get() ^ 2) == TRUE
    {
        (*gSprites.as_ptr())[{
            let gl_i4 = gBattleAnimAttacker.get() as i32 ^ 2;
            crate::gl!(gBattlerSpriteIds[gl_i4])
        }]
        .oam
        .set_priority(
            (*gSprites.as_ptr())[{
                let gl_i5 = gBattleAnimAttacker.get() as i32 ^ 2;
                crate::gl!(gBattlerSpriteIds[gl_i5])
            }]
            .oam
            .priority()
                - 1,
        );
        (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(1);
        SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
        var0 = 1;
    }
    if IsContest() != 0 {
        species = (*(*crate::gl!(gContestResources)).moveAnim).species;
    } else {
        if GetBattlerSide(gBattleAnimAttacker.get()) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i6 = gBattleAnimAttacker.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i6])
                }],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i7 = gBattleAnimAttacker.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i7])
                }],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    let spriteId: i32 = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i32;
    let newSpriteId: i32 = CreateInvisibleSpriteCopy(
        gBattleAnimAttacker.get() as i32,
        spriteId as u8,
        species as i32,
    ) as i32;
    GetBattleAnimBg1Data(&raw mut animBgData);
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBgData,
        (*(&raw const crate::data::graphics::gBattleAnimMaskTilemap_Curse).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        FALSE as u32,
    );
    AnimLoadCompressedBgGfx(
        animBgData.bgId as u32,
        (*(&raw const crate::data::graphics::gBattleAnimMaskImage_Curse).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBgData.tilesOffset as u32,
    );
    LoadPalette(
        sCurseLinesPalette.as_ptr().cast_mut() as *mut c_void,
        (animBgData.paletteId as u16 * 16) + 1,
        2,
    );
    gBattle_BG1_X.set((crate::gl!(gSprites[spriteId].x) as u16).wrapping_neg() + 32);
    gBattle_BG1_Y.set((crate::gl!(gSprites[spriteId].y) as u16).wrapping_neg() + 32);
    task_set(taskId, 0, newSpriteId as i16);
    task_set(taskId, 6, var0 as i16);
    task_set_func(taskId, Some(AnimTask_DrawFallingWhiteLinesOnAttacker_Step));
}
pub(crate) unsafe fn AnimTask_DrawFallingWhiteLinesOnAttacker_Step(taskId: u8) {
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut sprite: *mut Sprite = null_mut();
    let mut bg1Cnt: u16 = 0;
    task_set(taskId, 10, task_get(taskId, 10) + 4);
    gBattle_BG1_Y.set(gBattle_BG1_Y.get() - (4));
    if task_get(taskId, 10) == 64 {
        task_set(taskId, 10, 0);
        gBattle_BG1_Y.set(gBattle_BG1_Y.get() + (64));
        if ({
            task_set(taskId, 11, task_get(taskId, 11) + 1);
            task_get(taskId, 11)
        }) == 4
        {
            ResetBattleAnimBg(FALSE);
            gBattle_WIN0H.set(0);
            gBattle_WIN0V.set(0);
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_WINOUT, 16191);
            if IsContest() == 0 {
                bg1Cnt = GetGpuReg(REG_OFFSET_BG1CNT);
                (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(0);
                SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
            }
            SetGpuReg(
                REG_OFFSET_DISPCNT,
                GetGpuReg(REG_OFFSET_DISPCNT) ^ DISPCNT_OBJWIN_ON,
            );
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            sprite = {
                let gl_i8 = GetAnimBattlerSpriteId(0);
                crate::gl!(&raw gSprites[gl_i8])
            };
            sprite = {
                let gl_i9 = task_get(taskId, 0);
                crate::gl!(&raw gSprites[gl_i9])
            };
            DestroySprite(sprite);
            GetBattleAnimBg1Data(&raw mut animBgData);
            ClearBattleAnimBg(animBgData.bgId as u32);
            if task_get(taskId, 6) == 1 {
                (*gSprites.as_ptr())[{
                    let gl_i10 = gBattleAnimAttacker.get() as i32 ^ 2;
                    crate::gl!(gBattlerSpriteIds[gl_i10])
                }]
                .oam
                .set_priority(
                    (*gSprites.as_ptr())[{
                        let gl_i11 = gBattleAnimAttacker.get() as i32 ^ 2;
                        crate::gl!(gBattlerSpriteIds[gl_i11])
                    }]
                    .oam
                    .priority()
                        + 1,
                );
            }
            gBattle_BG1_Y.set(0);
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub unsafe fn InitStatsChangeAnimation(taskId: u8) {
    crate::gl!(sAnimStatsChangeData = AllocZeroed(24) as *mut AnimStatsChangeData);
    for i in 0..8u8 {
        (*crate::gl!(sAnimStatsChangeData)).data[i] = crate::gl!(gBattleAnimArgs[i]);
    }
    task_set_func(taskId, Some(StatsChangeAnimation_Step1));
}
pub(crate) unsafe fn StatsChangeAnimation_Step1(taskId: u8) {
    if (*crate::gl!(sAnimStatsChangeData)).data[aIsTarget] == 0 {
        (*crate::gl!(sAnimStatsChangeData)).battler1 = gBattleAnimAttacker.get();
    } else {
        (*crate::gl!(sAnimStatsChangeData)).battler1 = gBattleAnimTarget.get();
    }
    (*crate::gl!(sAnimStatsChangeData)).battler2 = (*crate::gl!(sAnimStatsChangeData)).battler1 ^ 2;
    if IsContest() != 0
        || (*crate::gl!(sAnimStatsChangeData)).data[aMultipleBattlers] != 0
            && IsBattlerSpriteVisible((*crate::gl!(sAnimStatsChangeData)).battler2) == 0
    {
        (*crate::gl!(sAnimStatsChangeData)).data[aMultipleBattlers] = FALSE as i16;
    }
    gBattle_WIN0H.set(0);
    gBattle_WIN0V.set(0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 0);
    SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
    }
    if IsDoubleBattle() != 0
        && (*crate::gl!(sAnimStatsChangeData)).data[aMultipleBattlers] == 0
        && (GetBattlerPosition((*crate::gl!(sAnimStatsChangeData)).battler1)
            == B_POSITION_OPPONENT_RIGHT
            || GetBattlerPosition((*crate::gl!(sAnimStatsChangeData)).battler1)
                == B_POSITION_PLAYER_LEFT)
        && IsBattlerSpriteVisible((*crate::gl!(sAnimStatsChangeData)).battler2) == TRUE
    {
        (*gSprites.as_ptr())[{
            let gl_i12 = (*crate::gl!(sAnimStatsChangeData)).battler2;
            crate::gl!(gBattlerSpriteIds[gl_i12])
        }]
        .oam
        .set_priority(
            (*gSprites.as_ptr())[{
                let gl_i13 = (*crate::gl!(sAnimStatsChangeData)).battler2;
                crate::gl!(gBattlerSpriteIds[gl_i13])
            }]
            .oam
            .priority()
                - 1,
        );
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        (*crate::gl!(sAnimStatsChangeData)).hidBattler2 = TRUE;
    }
    if IsContest() != 0 {
        (*crate::gl!(sAnimStatsChangeData)).species =
            (*(*crate::gl!(gContestResources)).moveAnim).species;
    } else {
        if GetBattlerSide((*crate::gl!(sAnimStatsChangeData)).battler1) != B_SIDE_PLAYER {
            (*crate::gl!(sAnimStatsChangeData)).species = GetMonData2(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i14 = (*crate::gl!(sAnimStatsChangeData)).battler1;
                    crate::gl!(gBattlerPartyIndexes[gl_i14])
                }],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            (*crate::gl!(sAnimStatsChangeData)).species = GetMonData2(
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i15 = (*crate::gl!(sAnimStatsChangeData)).battler1;
                    crate::gl!(gBattlerPartyIndexes[gl_i15])
                }],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    task_set_func(taskId, Some(StatsChangeAnimation_Step2));
}
pub(crate) unsafe fn StatsChangeAnimation_Step2(taskId: u8) {
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut spriteId2: u8 = 0;
    let mut battlerSpriteId: u8 = {
        let gl_i16 = (*crate::gl!(sAnimStatsChangeData)).battler1;
        crate::gl!(gBattlerSpriteIds[gl_i16])
    };
    let spriteId: u8 = CreateInvisibleSpriteCopy(
        (*crate::gl!(sAnimStatsChangeData)).battler1 as i32,
        battlerSpriteId,
        (*crate::gl!(sAnimStatsChangeData)).species as i32,
    );
    if (*crate::gl!(sAnimStatsChangeData)).data[3] != 0 {
        battlerSpriteId = {
            let gl_i17 = (*crate::gl!(sAnimStatsChangeData)).battler2;
            crate::gl!(gBattlerSpriteIds[gl_i17])
        };
        spriteId2 = CreateInvisibleSpriteCopy(
            (*crate::gl!(sAnimStatsChangeData)).battler2 as i32,
            battlerSpriteId,
            (*crate::gl!(sAnimStatsChangeData)).species as i32,
        );
    }
    GetBattleAnimBg1Data(&raw mut animBgData);
    if (*crate::gl!(sAnimStatsChangeData)).data[0] == 0 {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBgData,
            (*(&raw const crate::data::graphics::gStatAnim_Increase_Tilemap)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            FALSE as u32,
        );
    } else {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBgData,
            (*(&raw const crate::data::graphics::gStatAnim_Decrease_Tilemap)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            FALSE as u32,
        );
    }
    AnimLoadCompressedBgGfx(
        animBgData.bgId as u32,
        (*(&raw const crate::data::graphics::gStatAnim_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBgData.tilesOffset as u32,
    );
    match (*crate::gl!(sAnimStatsChangeData)).data[1] {
        STAT_ANIM_PAL_ATK => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_Attack_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_DEF => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_Defense_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_ACC => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_Accuracy_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_SPEED => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_Speed_Pal).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_EVASION => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_Evasion_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_SPATK => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_SpAttack_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_SPDEF => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_SpDefense_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
        _ => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gStatAnim_Multiple_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBgData.paletteId as u16 * 16,
                32,
            );
        }
    }
    gBattle_BG1_X.set(0);
    gBattle_BG1_Y.set(0);
    if (*crate::gl!(sAnimStatsChangeData)).data[0] == TRUE as i16 {
        gBattle_BG1_X.set(64);
        task_set(taskId, 1, -3);
    } else {
        task_set(taskId, 1, 3);
    }
    if (*crate::gl!(sAnimStatsChangeData)).data[4] == 0 {
        task_set(taskId, 4, 10);
        task_set(taskId, tWaitTime, 20);
    } else {
        task_set(taskId, 4, 13);
        task_set(taskId, tWaitTime, 30);
    }
    task_set(taskId, 0, spriteId as i16);
    task_set(
        taskId,
        tMultipleBattlers,
        (*crate::gl!(sAnimStatsChangeData)).data[3],
    );
    task_set(taskId, 3, spriteId2 as i16);
    task_set(
        taskId,
        tHidBattler2,
        (*crate::gl!(sAnimStatsChangeData)).hidBattler2 as i16,
    );
    task_set(
        taskId,
        tBattler2SpriteId,
        ({
            let gl_i18 = (*crate::gl!(sAnimStatsChangeData)).battler2;
            crate::gl!(gBattlerSpriteIds[gl_i18])
        }) as i16,
    );
    task_set_func(taskId, Some(StatsChangeAnimation_Step3));
    if (*crate::gl!(sAnimStatsChangeData)).data[0] == 0 {
        PlaySE12WithPanning(
            SE_M_STAT_INCREASE,
            BattleAnimAdjustPanning2(SOUND_PAN_ATTACKER),
        );
    } else {
        PlaySE12WithPanning(
            SE_M_STAT_DECREASE,
            BattleAnimAdjustPanning2(SOUND_PAN_ATTACKER),
        );
    }
}
pub(crate) unsafe fn StatsChangeAnimation_Step3(taskId: u8) {
    {
        let rhs = task_get(taskId, tVelocity) as u16;
        gBattle_BG1_Y.set(gBattle_BG1_Y.get() + rhs)
    };
    match task_get(taskId, tState) {
        0 => {
            if ({
                let t1 = task_get(taskId, tFadeTimer);
                task_set(taskId, tFadeTimer, task_get(taskId, tFadeTimer) + 1);
                t1
            }) > 0
            {
                task_set(taskId, tFadeTimer, 0);
                task_set(taskId, tBlend, task_get(taskId, tBlend) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, tBlend) as u16) << 8 | task_get(taskId, tBlend) as u16,
                );
                if task_get(taskId, tBlend) == task_get(taskId, tTargetBlend) {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
        }
        1 => {
            if ({
                task_set(taskId, tWaitTimer, task_get(taskId, tWaitTimer) + 1);
                task_get(taskId, tWaitTimer)
            }) == task_get(taskId, tWaitTime)
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 => {
            if ({
                let t3 = task_get(taskId, tFadeTimer);
                task_set(taskId, tFadeTimer, task_get(taskId, tFadeTimer) + 1);
                t3
            }) > 0
            {
                task_set(taskId, tFadeTimer, 0);
                task_set(taskId, tBlend, task_get(taskId, tBlend) - 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, tBlend) as u16) << 8 | task_get(taskId, tBlend) as u16,
                );
                if task_get(taskId, tBlend) == 0 {
                    ResetBattleAnimBg(FALSE);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
        }
        3 => {
            gBattle_WIN0H.set(0);
            gBattle_WIN0V.set(0);
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_WINOUT, 16191);
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            SetGpuReg(
                REG_OFFSET_DISPCNT,
                GetGpuReg(REG_OFFSET_DISPCNT) ^ DISPCNT_OBJWIN_ON,
            );
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroySprite({
                let gl_i19 = task_get(taskId, tAnimSpriteId1);
                crate::gl!(&raw gSprites[gl_i19])
            });
            if task_get(taskId, tMultipleBattlers) != 0 {
                DestroySprite({
                    let gl_i20 = task_get(taskId, tAnimSpriteId2);
                    crate::gl!(&raw gSprites[gl_i20])
                });
            }
            if task_get(taskId, tHidBattler2) == TRUE as i16 {
                (*gSprites.as_ptr())[task_get(taskId, tBattler2SpriteId)]
                    .oam
                    .set_priority(
                        (*gSprites.as_ptr())[task_get(taskId, tBattler2SpriteId)]
                            .oam
                            .priority()
                            + 1,
                    );
            }
            Free(crate::gl!(sAnimStatsChangeData) as *mut c_void);
            crate::gl!(sAnimStatsChangeData = null_mut());
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_Flash(taskId: u8) {
    let mut selectedPalettes: u32 = GetBattleMonSpritePalettesMask(1, 1, 1, 1);
    SetPalettesToColor(selectedPalettes, 0);
    task_set(taskId, 14, (selectedPalettes >> 16) as i16);
    selectedPalettes =
        GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE) & 0xFFFF;
    SetPalettesToColor(selectedPalettes, 65535);
    task_set(taskId, 15, selectedPalettes as i16);
    task_set(taskId, 0, 0);
    task_set(taskId, 1, 0);
    task_set_func(taskId, Some(AnimTask_Flash_Step));
}
pub(crate) unsafe fn AnimTask_Flash_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 6
            {
                (*task).data[1] = 0;
                (*task).data[2] = 16;
                (*task).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                (*task).data[2] -= 1;
                for i in 0..16u16 {
                    if shr_i32((*task).data[15] as i32, i as u32) & 1 != 0 {
                        BlendPalette(i * 16, 16, (*task).data[2] as u8, 0xFFFF);
                    }
                    if shr_i32((*task).data[14] as i32, i as u32) & 1 != 0 {
                        BlendPalette(0x100 + i * 16, 16, (*task).data[2] as u8, 0);
                    }
                }
                if (*task).data[2] == 0 {
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
fn SetPalettesToColor(mut selectedPalettes: u32, color: u16) {
    for i in 0..32u16 {
        if selectedPalettes & 1 != 0 {
            let mut curOffset: u16 = i * 16;
            let paletteOffset: u16 = curOffset;
            while (curOffset as i32) < paletteOffset as i32 + 16 {
                crate::gl!(gPlttBufferFaded[curOffset] = color);
                curOffset += 1;
            }
        }
        selectedPalettes >>= 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendNonAttackerPalettes(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    for battler in 0..(MAX_BATTLERS_COUNT as u32) {
        if gBattleAnimAttacker.get() as u32 != battler {
            selectedPalettes |= shl_i32(1, battler + 16) as u32;
        }
    }
    let mut j: i32 = 5;
    while j != 0 {
        crate::gl!(
            gBattleAnimArgs[j] = {
                let gl_i0 = j - 1;
                crate::gl!(gBattleAnimArgs[gl_i0])
            }
        );
        j -= 1;
    }
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StartSlidingBg(taskId: u8) {
    UpdateAnimBg3ScreenSize(FALSE);
    let newTaskId: u8 = CreateTask(Some(AnimTask_UpdateSlidingBg), 5);
    if crate::gl!(gBattleAnimArgs[2]) != 0
        && GetBattlerSide(gBattleAnimAttacker.get()) != B_SIDE_PLAYER
    {
        crate::gl!(gBattleAnimArgs[0] = -crate::gl!(gBattleAnimArgs[0]));
        crate::gl!(gBattleAnimArgs[1] = -crate::gl!(gBattleAnimArgs[1]));
    }
    task_set(newTaskId, 1, crate::gl!(gBattleAnimArgs[0]));
    task_set(newTaskId, 2, crate::gl!(gBattleAnimArgs[1]));
    task_set(newTaskId, 3, crate::gl!(gBattleAnimArgs[3]));
    task_set(newTaskId, 0, task_get(newTaskId, 0) + 1);
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimTask_UpdateSlidingBg(taskId: u8) {
    task_set(taskId, 10, task_get(taskId, 10) + (task_get(taskId, 1)));
    task_set(taskId, 11, task_get(taskId, 11) + (task_get(taskId, 2)));
    {
        let rhs = (task_get(taskId, 10) >> 8) as u16;
        gBattle_BG3_X.set(gBattle_BG3_X.get() + rhs)
    };
    {
        let rhs = (task_get(taskId, 11) >> 8) as u16;
        gBattle_BG3_Y.set(gBattle_BG3_Y.get() + rhs)
    };
    task_set(taskId, 10, task_get(taskId, 10) & 0xFF);
    task_set(taskId, 11, task_get(taskId, 11) & 0xFF);
    if crate::gl!(gBattleAnimArgs[7]) == task_get(taskId, 3) {
        gBattle_BG3_X.set(0);
        gBattle_BG3_Y.set(0);
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetAttackerSide(taskId: u8) {
    crate::gl!(gBattleAnimArgs[7] = GetBattlerSide(gBattleAnimAttacker.get()) as i16);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetTargetSide(taskId: u8) {
    crate::gl!(gBattleAnimArgs[7] = GetBattlerSide(gBattleAnimTarget.get()) as i16);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub fn AnimTask_GetTargetIsAttackerPartner(taskId: u8) {
    crate::gl!(
        gBattleAnimArgs[7] =
            (gBattleAnimAttacker.get() as i32 ^ 2 == gBattleAnimTarget.get() as i32) as i16
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetAllNonAttackersInvisiblity(taskId: u8) {
    for battler in 0..(MAX_BATTLERS_COUNT as u16) {
        if battler != gBattleAnimAttacker.get() as u16 && IsBattlerSpriteVisible(battler as u8) != 0
        {
            (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])]
                .set_invisible(crate::gl!(gBattleAnimArgs[0]) as u16);
        }
    }
    DestroyAnimVisualTask(taskId);
}
pub unsafe fn StartMonScrollingBgMask(
    taskId: u8,
    unused: i32,
    scrollSpeed: u16,
    battler: u8,
    mut includePartner: u8,
    numFadeSteps: u8,
    fadeStepDelay: u8,
    duration: u8,
    gfx: *mut u32,
    tilemap: *mut u32,
    palette: *mut u32,
) {
    let mut species: u16 = 0;
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut spriteId2: u8 = 0;
    let battler2: u8 = battler ^ 2;
    if IsContest() != 0 || includePartner != 0 && IsBattlerSpriteVisible(battler2) == 0 {
        includePartner = FALSE;
    }
    gBattle_WIN0H.set(0);
    gBattle_WIN0V.set(0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    let mut bg1Cnt: u16 = GetGpuReg(REG_OFFSET_BG1CNT);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_screenSize(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_areaOverflowMode(1);
    if IsContest() == 0 {
        (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(1);
    }
    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    if IsContest() != 0 {
        species = (*(*crate::gl!(gContestResources)).moveAnim).species;
    } else {
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut (*gEnemyParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    let spriteId: u8 = CreateInvisibleSpriteCopy(
        battler as i32,
        crate::gl!(gBattlerSpriteIds[battler]),
        species as i32,
    );
    if includePartner != 0 {
        spriteId2 = CreateInvisibleSpriteCopy(
            battler2 as i32,
            crate::gl!(gBattlerSpriteIds[battler2]),
            species as i32,
        );
    }
    GetBattleAnimBg1Data(&raw mut animBgData);
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBgData,
        tilemap as *mut c_void,
        FALSE as u32,
    );
    AnimLoadCompressedBgGfx(animBgData.bgId as u32, gfx, animBgData.tilesOffset as u32);
    LoadCompressedPalette(palette, animBgData.paletteId as u16 * 16, 32);
    gBattle_BG1_X.set(0);
    gBattle_BG1_Y.set(0);
    task_set(taskId, 1, scrollSpeed as i16);
    task_set(taskId, 4, numFadeSteps as i16);
    task_set(taskId, 5, duration as i16);
    task_set(taskId, 6, fadeStepDelay as i16);
    task_set(taskId, 0, spriteId as i16);
    task_set(taskId, 2, includePartner as i16);
    task_set(taskId, 3, spriteId2 as i16);
    task_set_func(taskId, Some(UpdateMonScrollingBgMask));
}
pub(crate) unsafe fn UpdateMonScrollingBgMask(taskId: u8) {
    task_set(
        taskId,
        13,
        task_get(taskId, 13)
            + ((if task_get(taskId, 1) < 0 {
                -(task_get(taskId, 1) as i32)
            } else {
                task_get(taskId, 1) as i32
            }) as i16),
    );
    if task_get(taskId, 1) < 0 {
        {
            let rhs = (task_get(taskId, 13) >> 8) as u16;
            gBattle_BG1_Y.set(gBattle_BG1_Y.get() - rhs)
        };
    } else {
        {
            let rhs = (task_get(taskId, 13) >> 8) as u16;
            gBattle_BG1_Y.set(gBattle_BG1_Y.get() + rhs)
        };
    }
    task_set(taskId, 13, task_get(taskId, 13) & 0xFF);
    match task_get(taskId, 15) {
        0 => {
            if ({
                let t1 = task_get(taskId, 11);
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                t1
            }) >= task_get(taskId, 6)
            {
                task_set(taskId, 11, 0);
                task_set(taskId, 12, task_get(taskId, 12) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 12) as u16) << 8 | task_get(taskId, 12) as u16,
                );
                if task_get(taskId, 12) == task_get(taskId, 4) {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            }
        }
        1 => {
            if ({
                task_set(taskId, 10, task_get(taskId, 10) + 1);
                task_get(taskId, 10)
            }) == task_get(taskId, 5)
            {
                task_set(taskId, 15, task_get(taskId, 15) + 1);
            }
        }
        2 if ({
            let t3 = task_get(taskId, 11);
            task_set(taskId, 11, task_get(taskId, 11) + 1);
            t3
        }) >= task_get(taskId, 6) =>
        {
            task_set(taskId, 11, 0);
            task_set(taskId, 12, task_get(taskId, 12) - 1);
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (16 - task_get(taskId, 12) as u16) << 8 | task_get(taskId, 12) as u16,
            );
            if task_get(taskId, 12) == 0 {
                ResetBattleAnimBg(FALSE);
                gBattle_WIN0H.set(0);
                gBattle_WIN0V.set(0);
                SetGpuReg(REG_OFFSET_WININ, 16191);
                SetGpuReg(REG_OFFSET_WINOUT, 16191);
                if IsContest() == 0 {
                    let mut bg1Cnt: u16 = GetGpuReg(REG_OFFSET_BG1CNT);
                    (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(0);
                    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
                }
                SetGpuReg(
                    REG_OFFSET_DISPCNT,
                    GetGpuReg(REG_OFFSET_DISPCNT) ^ DISPCNT_OBJWIN_ON,
                );
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                DestroySprite({
                    let gl_i21 = task_get(taskId, 0);
                    crate::gl!(&raw gSprites[gl_i21])
                });
                if task_get(taskId, 2) != 0 {
                    DestroySprite({
                        let gl_i22 = task_get(taskId, 3);
                        crate::gl!(&raw gSprites[gl_i22])
                    });
                }
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub fn AnimTask_GetBattleEnvironment(taskId: u8) {
    crate::gl!(gBattleAnimArgs[0] = gBattleEnvironment.get() as i16);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AllocBackupPalBuffer(taskId: u8) {
    (*crate::gl!(gMonSpritesGfxPtr)).buffer = AllocZeroed(8192) as *mut u16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FreeBackupPalBuffer(taskId: u8) {
    Free((*crate::gl!(gMonSpritesGfxPtr)).buffer as *mut c_void);
    (*crate::gl!(gMonSpritesGfxPtr)).buffer = null_mut();
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CopyPalUnfadedToBackup(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut paletteIndex: i32 = 0;
    if crate::gl!(gBattleAnimArgs[0]) == 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
        while selectedPalettes & 1 == 0 {
            selectedPalettes >>= 1;
            paletteIndex += 1;
        }
    } else if crate::gl!(gBattleAnimArgs[0]) == 1 {
        paletteIndex = gBattleAnimAttacker.get() as i32 + 16;
    } else if crate::gl!(gBattleAnimArgs[0]) == 2 {
        paletteIndex = gBattleAnimTarget.get() as i32 + 16;
    }
    memcpy(
        (*crate::gl!(gMonSpritesGfxPtr))
            .buffer
            .at(crate::gl!(gBattleAnimArgs[1]) as i32 * 16) as *mut u8,
        {
            let gl_i1 = paletteIndex * 16;
            crate::gl!(&raw gPlttBufferUnfaded[gl_i1])
        } as *mut u8,
        32,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CopyPalUnfadedFromBackup(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut paletteIndex: i32 = 0;
    if crate::gl!(gBattleAnimArgs[0]) == 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
        while selectedPalettes & 1 == 0 {
            selectedPalettes >>= 1;
            paletteIndex += 1;
        }
    } else if crate::gl!(gBattleAnimArgs[0]) == 1 {
        paletteIndex = gBattleAnimAttacker.get() as i32 + 16;
    } else if crate::gl!(gBattleAnimArgs[0]) == 2 {
        paletteIndex = gBattleAnimTarget.get() as i32 + 16;
    }
    memcpy(
        {
            let gl_i2 = paletteIndex * 16;
            crate::gl!(&raw gPlttBufferUnfaded[gl_i2])
        } as *mut u8,
        (*crate::gl!(gMonSpritesGfxPtr))
            .buffer
            .at(crate::gl!(gBattleAnimArgs[1]) as i32 * 16) as *mut u8,
        32,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CopyPalFadedToUnfaded(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut paletteIndex: i32 = 0;
    if crate::gl!(gBattleAnimArgs[0]) == 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
        while selectedPalettes & 1 == 0 {
            selectedPalettes >>= 1;
            paletteIndex += 1;
        }
    } else if crate::gl!(gBattleAnimArgs[0]) == 1 {
        paletteIndex = gBattleAnimAttacker.get() as i32 + 16;
    } else if crate::gl!(gBattleAnimArgs[0]) == 2 {
        paletteIndex = gBattleAnimTarget.get() as i32 + 16;
    }
    memcpy(
        {
            let gl_i3 = paletteIndex * 16;
            crate::gl!(&raw gPlttBufferUnfaded[gl_i3])
        } as *mut u8,
        {
            let gl_i4 = paletteIndex * 16;
            crate::gl!(&raw gPlttBufferFaded[gl_i4])
        } as *mut u8,
        32,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsContest(taskId: u8) {
    if IsContest() != 0 {
        crate::gl!(gBattleAnimArgs[7] = TRUE as i16);
    } else {
        crate::gl!(gBattleAnimArgs[7] = FALSE as i16);
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub fn AnimTask_SetAnimAttackerAndTargetForEffectTgt(taskId: u8) {
    gBattleAnimAttacker.set(gBattlerTarget.get());
    gBattleAnimTarget.set(gEffectBattler.get());
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsTargetSameSide(taskId: u8) {
    if GetBattlerSide(gBattleAnimAttacker.get()) == GetBattlerSide(gBattleAnimTarget.get()) {
        crate::gl!(gBattleAnimArgs[7] = TRUE as i16);
    } else {
        crate::gl!(gBattleAnimArgs[7] = FALSE as i16);
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub fn AnimTask_SetAnimTargetToBattlerTarget(taskId: u8) {
    gBattleAnimTarget.set(gBattlerTarget.get());
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub fn AnimTask_SetAnimAttackerAndTargetForEffectAtk(taskId: u8) {
    gBattleAnimAttacker.set(gBattlerAttacker.get());
    gBattleAnimTarget.set(gEffectBattler.get());
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetAttackerInvisibleWaitForSignal(taskId: u8) {
    if IsContest() != 0 {
        DestroyAnimVisualTask(taskId);
    } else {
        task_set(
            taskId,
            0,
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .battlerData
                .at(gBattleAnimAttacker.get()))
            .invisible() as i16,
        );
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .battlerData
            .at(gBattleAnimAttacker.get()))
        .set_invisible(TRUE as u16);
        task_set_func(taskId, Some(AnimTask_WaitAndRestoreVisibility));
        gAnimVisualTaskCount.set(gAnimVisualTaskCount.get() - (1));
    }
}
pub(crate) unsafe fn AnimTask_WaitAndRestoreVisibility(taskId: u8) {
    if crate::gl!(gBattleAnimArgs[7]) == 0x1000 {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .battlerData
            .at(gBattleAnimAttacker.get()))
        .set_invisible(task_get(taskId, 0) as u8 as u16 & 1);
        DestroyTask(taskId);
    }
}
pub unsafe fn SetAnimBgAttribute(bgId: u8, attributeId: u8, value: u8) {
    if bgId < 4 {
        SetAnimBgAttribute_sBgCnt.set(GetGpuReg(sBattleAnimBgCntSet[bgId]));
        match attributeId {
            BG_ANIM_SCREEN_SIZE => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt)).set_screenSize(value as u16);
            }
            BG_ANIM_AREA_OVERFLOW_MODE => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt))
                    .set_areaOverflowMode(value as u16);
            }
            BG_ANIM_MOSAIC => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt)).set_mosaic(value as u16);
            }
            BG_ANIM_CHAR_BASE_BLOCK => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt))
                    .set_charBaseBlock(value as u16);
            }
            BG_ANIM_PRIORITY => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt)).set_priority(value as u16);
            }
            BG_ANIM_PALETTES_MODE => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt)).set_palettes(value as u16);
            }
            BG_ANIM_SCREEN_BASE_BLOCK => {
                (*(SetAnimBgAttribute_sBgCnt.as_ptr() as *mut BgCnt))
                    .set_screenBaseBlock(value as u16);
            }
            _ => {}
        }
        SetGpuReg(sBattleAnimBgCntSet[bgId], SetAnimBgAttribute_sBgCnt.get());
    }
}
pub unsafe fn GetAnimBgAttribute(bgId: u8, attributeId: u8) -> i32 {
    let mut bgCnt: u16 = 0;
    if bgId < 4 {
        bgCnt = GetGpuReg(sBattleAnimBgCntGet[bgId]);
        match attributeId {
            BG_ANIM_SCREEN_SIZE => {
                return (*(&raw mut bgCnt as *mut BgCnt)).screenSize() as i32;
            }
            BG_ANIM_AREA_OVERFLOW_MODE => {
                return (*(&raw mut bgCnt as *mut BgCnt)).areaOverflowMode() as i32;
            }
            BG_ANIM_MOSAIC => {
                return (*(&raw mut bgCnt as *mut BgCnt)).mosaic() as i32;
            }
            BG_ANIM_CHAR_BASE_BLOCK => {
                return (*(&raw mut bgCnt as *mut BgCnt)).charBaseBlock() as i32;
            }
            BG_ANIM_PRIORITY => {
                return (*(&raw mut bgCnt as *mut BgCnt)).priority() as i32;
            }
            BG_ANIM_PALETTES_MODE => {
                return (*(&raw mut bgCnt as *mut BgCnt)).palettes() as i32;
            }
            BG_ANIM_SCREEN_BASE_BLOCK => {
                return (*(&raw mut bgCnt as *mut BgCnt)).screenBaseBlock() as i32;
            }
            _ => {}
        }
    }
    0
}
