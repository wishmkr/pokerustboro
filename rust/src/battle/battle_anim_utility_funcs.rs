//! Translated from `src/battle_anim_utility_funcs.c` by tools/rustport/c2rs.py.
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimStatsChangeData: *mut AnimStatsChangeData = null_mut();
static mut SetAnimBgAttribute_sBgCnt: u16 = 0;

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static gBattleAnimMaskImage_Curse: CArray<u32, 0>;
    static gBattleAnimMaskTilemap_Curse: CArray<u32, 0>;
    static mut gBattleAnimTarget: u8;
    static mut gBattleEnvironment: u8;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;
    static mut gBattle_WIN0H: u16;
    static mut gBattle_WIN0V: u16;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gBattlerTarget: u8;
    static mut gContestResources: *mut ContestResources;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSprites: CArray<Sprite, 65>;
    static gStatAnim_Accuracy_Pal: CArray<u32, 0>;
    static gStatAnim_Attack_Pal: CArray<u32, 0>;
    static gStatAnim_Decrease_Tilemap: CArray<u32, 0>;
    static gStatAnim_Defense_Pal: CArray<u32, 0>;
    static gStatAnim_Evasion_Pal: CArray<u32, 0>;
    static gStatAnim_Gfx: CArray<u32, 0>;
    static gStatAnim_Increase_Tilemap: CArray<u32, 0>;
    static gStatAnim_Multiple_Pal: CArray<u32, 0>;
    static gStatAnim_SpAttack_Pal: CArray<u32, 0>;
    static gStatAnim_SpDefense_Pal: CArray<u32, 0>;
    static gStatAnim_Speed_Pal: CArray<u32, 0>;
    static mut gTasks: CArray<Task, 0>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemapHandleContest(
        a0: *mut BattleAnimBgData,
        a1: *mut c_void,
        a2: u32,
    );
    fn BattleAnimAdjustPanning2(a0: i8) -> i8;
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn ClearBattleAnimBg(a0: u32);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn CreateInvisibleSpriteCopy(a0: i32, a1: u8, a2: i32) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteWithActiveSheet(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn Free(a0: *mut c_void);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut BattleAnimBgData);
    fn GetBattleMonSpritePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8) -> u32;
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetSpritePalIdxByBattler(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn ResetBattleAnimBg(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn UnpackSelectedBattlePalettes(a0: i16) -> u32;
    fn UpdateAnimBg3ScreenSize(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendBattleAnimPal(taskId: u8) {
    let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(gBattleAnimArgs[0]);
    selectedPalettes |= GetBattleMonSpritePalettesMask(
        (gBattleAnimArgs[0] >> 7) as u8 & 1,
        (gBattleAnimArgs[0] >> 8) as u8 & 1,
        (gBattleAnimArgs[0] >> 9) as u8 & 1,
        (gBattleAnimArgs[0] >> 10) as u8 & 1,
    );
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendBattleAnimPalExclude(taskId: u8) {
    let mut battler: u8 = 0;
    let mut selectedPalettes: u32 = 0;
    let mut animBattlers: CArray<u8, 2> = zeroed();
    animBattlers[1] = 0xFF;
    selectedPalettes = UnpackSelectedBattlePalettes(F_PAL_BG);
    'l1: {
        let sw1: i16 = gBattleAnimArgs[0];
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
            fall = true;
            animBattlers[0] = gBattleAnimAttacker;
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            selectedPalettes = 0;
        }
        if fall || sw1 == 1 {
            fall = true;
            animBattlers[0] = gBattleAnimTarget;
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            animBattlers[0] = gBattleAnimAttacker;
            animBattlers[1] = gBattleAnimTarget;
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            animBattlers[0] = 0xFF;
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            selectedPalettes = 0;
            animBattlers[0] = gBattleAnimAttacker ^ 2;
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            selectedPalettes = 0;
            animBattlers[0] = gBattleAnimTarget ^ 2;
            break 'l1;
        }
    }
    battler = 0;
    while battler < MAX_BATTLERS_COUNT {
        if battler != animBattlers[0]
            && battler != animBattlers[1]
            && IsBattlerSpriteVisible(battler) != 0
        {
            selectedPalettes |= shl_i32(0x10000, GetSpritePalIdxByBattler(battler) as u32) as u32;
        }
        battler += 1;
    }
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetCamouflageBlend(taskId: u8) {
    let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(gBattleAnimArgs[0]);
    match gBattleEnvironment {
        BATTLE_ENVIRONMENT_GRASS => {
            gBattleAnimArgs[4] = 2828;
        }
        BATTLE_ENVIRONMENT_LONG_GRASS => {
            gBattleAnimArgs[4] = 2528;
        }
        BATTLE_ENVIRONMENT_SAND => {
            gBattleAnimArgs[4] = 12062;
        }
        BATTLE_ENVIRONMENT_UNDERWATER => {
            gBattleAnimArgs[4] = 18432;
        }
        BATTLE_ENVIRONMENT_WATER => {
            gBattleAnimArgs[4] = 32459;
        }
        BATTLE_ENVIRONMENT_POND => {
            gBattleAnimArgs[4] = 32459;
        }
        BATTLE_ENVIRONMENT_MOUNTAIN => {
            gBattleAnimArgs[4] = 10774;
        }
        BATTLE_ENVIRONMENT_CAVE => {
            gBattleAnimArgs[4] = 3374;
        }
        BATTLE_ENVIRONMENT_BUILDING => {
            gBattleAnimArgs[4] = 32767;
        }
        BATTLE_ENVIRONMENT_PLAIN => {
            gBattleAnimArgs[4] = 32767;
        }
        _ => {}
    }
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendParticle(taskId: u8) {
    let mut paletteIndex: u8 = IndexOfSpritePaletteTag(gBattleAnimArgs[0] as u16);
    let mut selectedPalettes: u32 = shl_i32(1, paletteIndex as u32 + 16) as u32;
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartBlendAnimSpriteColor(taskId: u8, selectedPalettes: u32) {
    gTasks[taskId].data[0] = selectedPalettes as i16;
    gTasks[taskId].data[1] = (selectedPalettes >> 16) as i16;
    gTasks[taskId].data[2] = gBattleAnimArgs[1];
    gTasks[taskId].data[3] = gBattleAnimArgs[2];
    gTasks[taskId].data[4] = gBattleAnimArgs[3];
    gTasks[taskId].data[5] = gBattleAnimArgs[4];
    gTasks[taskId].data[10] = gBattleAnimArgs[2];
    gTasks[taskId].func = Some(AnimTask_BlendSpriteColor_Step2);
    gTasks[taskId].func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn AnimTask_BlendSpriteColor_Step2(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut singlePaletteOffset: u16 = 0;
    if gTasks[taskId].data[9] == gTasks[taskId].data[2] {
        gTasks[taskId].data[9] = 0;
        selectedPalettes = gTasks[taskId].data[0] as u32 | (gTasks[taskId].data[1] as u32) << 16;
        while selectedPalettes != 0 {
            if selectedPalettes & 1 != 0 {
                BlendPalette(
                    singlePaletteOffset,
                    16,
                    gTasks[taskId].data[10] as u8,
                    gTasks[taskId].data[5] as u16,
                );
            }
            singlePaletteOffset += 16;
            selectedPalettes >>= 1;
        }
        if gTasks[taskId].data[10] < gTasks[taskId].data[4] {
            gTasks[taskId].data[10] += 1;
        } else if gTasks[taskId].data[10] > gTasks[taskId].data[4] {
            gTasks[taskId].data[10] -= 1;
        } else {
            DestroyAnimVisualTask(taskId);
        }
    } else {
        gTasks[taskId].data[9] += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HardwarePaletteFade(taskId: u8) {
    BeginHardwarePaletteFade(
        gBattleAnimArgs[0] as u8,
        gBattleAnimArgs[1] as u8,
        gBattleAnimArgs[2] as u8,
        gBattleAnimArgs[3] as u8,
        gBattleAnimArgs[4] as u8,
    );
    gTasks[taskId].func = Some(AnimTask_HardwarePaletteFade_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_HardwarePaletteFade_Step(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TraceMonBlended(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[0] = gBattleAnimArgs[0];
    (*task).data[1] = 0;
    (*task).data[2] = gBattleAnimArgs[1];
    (*task).data[3] = gBattleAnimArgs[2];
    (*task).data[4] = gBattleAnimArgs[3];
    (*task).data[5] = 0;
    (*task).func = Some(AnimTask_TraceMonBlended_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_TraceMonBlended_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if (*task).data[4] != 0 {
        if (*task).data[1] != 0 {
            (*task).data[1] -= 1;
        } else {
            (*task).data[6] = CloneBattlerSpriteWithBlend((*task).data[0] as u8);
            if (*task).data[6] >= 0 {
                gSprites[(*task).data[6]]
                    .oam
                    .set_priority((if (*task).data[0] != 0 { 1 } else { 2 }) as u16);
                gSprites[(*task).data[6]].data[0] = (*task).data[3];
                gSprites[(*task).data[6]].data[1] = taskId as i16;
                gSprites[(*task).data[6]].data[2] = 5;
                gSprites[(*task).data[6]].callback = Some(AnimMonTrace);
                (*task).data[5] += 1;
            }
            (*task).data[4] -= 1;
            (*task).data[1] = (*task).data[2];
        }
    } else if (*task).data[5] == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimMonTrace(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[0] -= 1;
    } else {
        gTasks[(*sprite).data[1]].data[(*sprite).data[2]] -= 1;
        DestroySpriteWithActiveSheet(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DrawFallingWhiteLinesOnAttacker(taskId: u8) {
    let mut species: u16 = 0;
    let mut spriteId: i32 = 0;
    let mut newSpriteId: i32 = 0;
    let mut var0: u16 = 0;
    let mut bg1Cnt: u16 = 0;
    let mut animBgData: BattleAnimBgData = zeroed();
    var0 = 0;
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 3080);
    bg1Cnt = GetGpuReg(REG_OFFSET_BG1CNT);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_screenSize(0);
    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    if IsContest() == 0 {
        (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(1);
        SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    }
    if IsDoubleBattle() != 0 && IsContest() == 0 {
        if GetBattlerPosition(gBattleAnimAttacker) == B_POSITION_OPPONENT_RIGHT
            || GetBattlerPosition(gBattleAnimAttacker) == B_POSITION_PLAYER_LEFT
        {
            if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) == TRUE {
                gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                    .oam
                    .set_priority(
                        gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                            .oam
                            .priority()
                            - 1,
                    );
                (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(1);
                SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
                var0 = 1;
            }
        }
    }
    if IsContest() != 0 {
        species = (*(*gContestResources).moveAnim).species;
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i32;
    newSpriteId =
        CreateInvisibleSpriteCopy(gBattleAnimAttacker as i32, spriteId as u8, species as i32)
            as i32;
    GetBattleAnimBg1Data(&raw mut animBgData);
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBgData,
        gBattleAnimMaskTilemap_Curse.as_ptr().cast_mut() as *mut c_void,
        FALSE as u32,
    );
    AnimLoadCompressedBgGfx(
        animBgData.bgId as u32,
        gBattleAnimMaskImage_Curse.as_ptr().cast_mut(),
        animBgData.tilesOffset as u32,
    );
    LoadPalette(
        sCurseLinesPalette.as_ptr().cast_mut() as *mut c_void,
        0x000 + animBgData.paletteId as u16 * 16 + 1,
        2,
    );
    gBattle_BG1_X = (gSprites[spriteId].x as u16).wrapping_neg() + 32;
    gBattle_BG1_Y = (gSprites[spriteId].y as u16).wrapping_neg() + 32;
    gTasks[taskId].data[0] = newSpriteId as i16;
    gTasks[taskId].data[6] = var0 as i16;
    gTasks[taskId].func = Some(AnimTask_DrawFallingWhiteLinesOnAttacker_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_DrawFallingWhiteLinesOnAttacker_Step(taskId: u8) {
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut sprite: *mut Sprite = null_mut();
    let mut bg1Cnt: u16 = 0;
    gTasks[taskId].data[10] += 4;
    gBattle_BG1_Y -= 4;
    if gTasks[taskId].data[10] == 64 {
        gTasks[taskId].data[10] = 0;
        gBattle_BG1_Y += 64;
        if ({
            gTasks[taskId].data[11] += 1;
            gTasks[taskId].data[11]
        }) == 4
        {
            ResetBattleAnimBg(FALSE);
            gBattle_WIN0H = 0;
            gBattle_WIN0V = 0;
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
            sprite = &raw mut gSprites[GetAnimBattlerSpriteId(0)];
            sprite = &raw mut gSprites[gTasks[taskId].data[0]];
            DestroySprite(sprite);
            GetBattleAnimBg1Data(&raw mut animBgData);
            ClearBattleAnimBg(animBgData.bgId as u32);
            if gTasks[taskId].data[6] == 1 {
                gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                    .oam
                    .set_priority(
                        gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                            .oam
                            .priority()
                            + 1,
                    );
            }
            gBattle_BG1_Y = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitStatsChangeAnimation(taskId: u8) {
    let mut i: u8 = 0;
    sAnimStatsChangeData = AllocZeroed(24) as *mut AnimStatsChangeData;
    i = 0;
    while i < 8 {
        (*sAnimStatsChangeData).data[i] = gBattleAnimArgs[i];
        i += 1;
    }
    gTasks[taskId].func = Some(StatsChangeAnimation_Step1);
}
pub(crate) unsafe extern "C" fn StatsChangeAnimation_Step1(taskId: u8) {
    if (*sAnimStatsChangeData).data[2] == 0 {
        (*sAnimStatsChangeData).battler1 = gBattleAnimAttacker;
    } else {
        (*sAnimStatsChangeData).battler1 = gBattleAnimTarget;
    }
    (*sAnimStatsChangeData).battler2 = (*sAnimStatsChangeData).battler1 ^ 2;
    if IsContest() != 0
        || (*sAnimStatsChangeData).data[3] != 0
            && IsBattlerSpriteVisible((*sAnimStatsChangeData).battler2) == 0
    {
        (*sAnimStatsChangeData).data[3] = FALSE as i16;
    }
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
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
    if IsDoubleBattle() != 0 && (*sAnimStatsChangeData).data[3] == 0 {
        if GetBattlerPosition((*sAnimStatsChangeData).battler1) == B_POSITION_OPPONENT_RIGHT
            || GetBattlerPosition((*sAnimStatsChangeData).battler1) == B_POSITION_PLAYER_LEFT
        {
            if IsBattlerSpriteVisible((*sAnimStatsChangeData).battler2) == TRUE {
                gSprites[gBattlerSpriteIds[(*sAnimStatsChangeData).battler2]]
                    .oam
                    .set_priority(
                        gSprites[gBattlerSpriteIds[(*sAnimStatsChangeData).battler2]]
                            .oam
                            .priority()
                            - 1,
                    );
                SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
                (*sAnimStatsChangeData).hidBattler2 = TRUE;
            }
        }
    }
    if IsContest() != 0 {
        (*sAnimStatsChangeData).species = (*(*gContestResources).moveAnim).species;
    } else {
        if GetBattlerSide((*sAnimStatsChangeData).battler1) != B_SIDE_PLAYER {
            (*sAnimStatsChangeData).species = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[(*sAnimStatsChangeData).battler1]],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            (*sAnimStatsChangeData).species = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[(*sAnimStatsChangeData).battler1]],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    gTasks[taskId].func = Some(StatsChangeAnimation_Step2);
}
pub(crate) unsafe extern "C" fn StatsChangeAnimation_Step2(taskId: u8) {
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut spriteId: u8 = 0;
    let mut spriteId2: u8 = 0;
    let mut battlerSpriteId: u8 = 0;
    spriteId2 = 0;
    battlerSpriteId = gBattlerSpriteIds[(*sAnimStatsChangeData).battler1];
    spriteId = CreateInvisibleSpriteCopy(
        (*sAnimStatsChangeData).battler1 as i32,
        battlerSpriteId,
        (*sAnimStatsChangeData).species as i32,
    );
    if (*sAnimStatsChangeData).data[3] != 0 {
        battlerSpriteId = gBattlerSpriteIds[(*sAnimStatsChangeData).battler2];
        spriteId2 = CreateInvisibleSpriteCopy(
            (*sAnimStatsChangeData).battler2 as i32,
            battlerSpriteId,
            (*sAnimStatsChangeData).species as i32,
        );
    }
    GetBattleAnimBg1Data(&raw mut animBgData);
    if (*sAnimStatsChangeData).data[0] == 0 {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBgData,
            gStatAnim_Increase_Tilemap.as_ptr().cast_mut() as *mut c_void,
            FALSE as u32,
        );
    } else {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBgData,
            gStatAnim_Decrease_Tilemap.as_ptr().cast_mut() as *mut c_void,
            FALSE as u32,
        );
    }
    AnimLoadCompressedBgGfx(
        animBgData.bgId as u32,
        gStatAnim_Gfx.as_ptr().cast_mut(),
        animBgData.tilesOffset as u32,
    );
    match (*sAnimStatsChangeData).data[1] {
        STAT_ANIM_PAL_ATK => {
            LoadCompressedPalette(
                gStatAnim_Attack_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_DEF => {
            LoadCompressedPalette(
                gStatAnim_Defense_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_ACC => {
            LoadCompressedPalette(
                gStatAnim_Accuracy_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_SPEED => {
            LoadCompressedPalette(
                gStatAnim_Speed_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_EVASION => {
            LoadCompressedPalette(
                gStatAnim_Evasion_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_SPATK => {
            LoadCompressedPalette(
                gStatAnim_SpAttack_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        STAT_ANIM_PAL_SPDEF => {
            LoadCompressedPalette(
                gStatAnim_SpDefense_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
        _ => {
            LoadCompressedPalette(
                gStatAnim_Multiple_Pal.as_ptr().cast_mut(),
                0x000 + animBgData.paletteId as u16 * 16,
                32,
            );
        }
    }
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    if (*sAnimStatsChangeData).data[0] == TRUE as i16 {
        gBattle_BG1_X = 64;
        gTasks[taskId].data[1] = -3;
    } else {
        gTasks[taskId].data[1] = 3;
    }
    if (*sAnimStatsChangeData).data[4] == 0 {
        gTasks[taskId].data[4] = 10;
        gTasks[taskId].data[5] = 20;
    } else {
        gTasks[taskId].data[4] = 13;
        gTasks[taskId].data[5] = 30;
    }
    gTasks[taskId].data[0] = spriteId as i16;
    gTasks[taskId].data[2] = (*sAnimStatsChangeData).data[3];
    gTasks[taskId].data[3] = spriteId2 as i16;
    gTasks[taskId].data[6] = (*sAnimStatsChangeData).hidBattler2 as i16;
    gTasks[taskId].data[7] = gBattlerSpriteIds[(*sAnimStatsChangeData).battler2] as i16;
    gTasks[taskId].func = Some(StatsChangeAnimation_Step3);
    if (*sAnimStatsChangeData).data[0] == 0 {
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
pub(crate) unsafe extern "C" fn StatsChangeAnimation_Step3(taskId: u8) {
    gBattle_BG1_Y += gTasks[taskId].data[1] as u16;
    match gTasks[taskId].data[15] {
        0 => {
            if ({
                let t1 = gTasks[taskId].data[11];
                gTasks[taskId].data[11] += 1;
                t1
            }) > 0
            {
                gTasks[taskId].data[11] = 0;
                gTasks[taskId].data[12] += 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[12] as u16) << 8 | gTasks[taskId].data[12] as u16,
                );
                if gTasks[taskId].data[12] == gTasks[taskId].data[4] {
                    gTasks[taskId].data[15] += 1;
                }
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) == gTasks[taskId].data[5]
            {
                gTasks[taskId].data[15] += 1;
            }
        }
        2 => {
            if ({
                let t3 = gTasks[taskId].data[11];
                gTasks[taskId].data[11] += 1;
                t3
            }) > 0
            {
                gTasks[taskId].data[11] = 0;
                gTasks[taskId].data[12] -= 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[12] as u16) << 8 | gTasks[taskId].data[12] as u16,
                );
                if gTasks[taskId].data[12] == 0 {
                    ResetBattleAnimBg(FALSE);
                    gTasks[taskId].data[15] += 1;
                }
            }
        }
        3 => {
            gBattle_WIN0H = 0;
            gBattle_WIN0V = 0;
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
            DestroySprite(&raw mut gSprites[gTasks[taskId].data[0]]);
            if gTasks[taskId].data[2] != 0 {
                DestroySprite(&raw mut gSprites[gTasks[taskId].data[3]]);
            }
            if gTasks[taskId].data[6] == TRUE as i16 {
                gSprites[gTasks[taskId].data[7]]
                    .oam
                    .set_priority(gSprites[gTasks[taskId].data[7]].oam.priority() + 1);
            }
            Free(sAnimStatsChangeData as *mut c_void);
            sAnimStatsChangeData = null_mut();
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Flash(taskId: u8) {
    let mut selectedPalettes: u32 = GetBattleMonSpritePalettesMask(1, 1, 1, 1);
    SetPalettesToColor(selectedPalettes, 0);
    gTasks[taskId].data[14] = (selectedPalettes >> 16) as i16;
    selectedPalettes =
        GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE) & 0xFFFF;
    SetPalettesToColor(selectedPalettes, 65535);
    gTasks[taskId].data[15] = selectedPalettes as i16;
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].func = Some(AnimTask_Flash_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_Flash_Step(taskId: u8) {
    let mut i: u16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
                i = 0;
                while i < 16 {
                    if shr_i32((*task).data[15] as i32, i as u32) & 1 != 0 {
                        BlendPalette(0x000 + i * 16, 16, (*task).data[2] as u8, 0xFFFF);
                    }
                    if shr_i32((*task).data[14] as i32, i as u32) & 1 != 0 {
                        BlendPalette(0x100 + i * 16, 16, (*task).data[2] as u8, 0);
                    }
                    i += 1;
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
pub(crate) unsafe extern "C" fn SetPalettesToColor(mut selectedPalettes: u32, color: u16) {
    let mut i: u16 = 0;
    i = 0;
    while i < 32 {
        if selectedPalettes & 1 != 0 {
            let mut curOffset: u16 = i * 16;
            let mut paletteOffset: u16 = curOffset;
            while (curOffset as i32) < paletteOffset as i32 + 16 {
                gPlttBufferFaded[curOffset] = color;
                curOffset += 1;
            }
        }
        selectedPalettes >>= 1;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendNonAttackerPalettes(taskId: u8) {
    let mut battler: u32 = 0;
    let mut j: i32 = 0;
    let mut selectedPalettes: u32 = 0;
    battler = 0;
    while battler < MAX_BATTLERS_COUNT as u32 {
        if gBattleAnimAttacker as u32 != battler {
            selectedPalettes |= shl_i32(1, battler + 16) as u32;
        }
        battler += 1;
    }
    j = 5;
    while j != 0 {
        gBattleAnimArgs[j] = gBattleAnimArgs[j - 1];
        j -= 1;
    }
    StartBlendAnimSpriteColor(taskId, selectedPalettes);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StartSlidingBg(taskId: u8) {
    let mut newTaskId: u8 = 0;
    UpdateAnimBg3ScreenSize(FALSE);
    newTaskId = CreateTask(Some(AnimTask_UpdateSlidingBg), 5);
    if gBattleAnimArgs[2] != 0 && GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
    }
    gTasks[newTaskId].data[1] = gBattleAnimArgs[0];
    gTasks[newTaskId].data[2] = gBattleAnimArgs[1];
    gTasks[newTaskId].data[3] = gBattleAnimArgs[3];
    gTasks[newTaskId].data[0] += 1;
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe extern "C" fn AnimTask_UpdateSlidingBg(taskId: u8) {
    gTasks[taskId].data[10] += gTasks[taskId].data[1];
    gTasks[taskId].data[11] += gTasks[taskId].data[2];
    gBattle_BG3_X += (gTasks[taskId].data[10] >> 8) as u16;
    gBattle_BG3_Y += (gTasks[taskId].data[11] >> 8) as u16;
    gTasks[taskId].data[10] &= 0xFF;
    gTasks[taskId].data[11] &= 0xFF;
    if gBattleAnimArgs[7] == gTasks[taskId].data[3] {
        gBattle_BG3_X = 0;
        gBattle_BG3_Y = 0;
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetAttackerSide(taskId: u8) {
    gBattleAnimArgs[7] = GetBattlerSide(gBattleAnimAttacker) as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetTargetSide(taskId: u8) {
    gBattleAnimArgs[7] = GetBattlerSide(gBattleAnimTarget) as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetTargetIsAttackerPartner(taskId: u8) {
    gBattleAnimArgs[7] = (gBattleAnimAttacker as i32 ^ 2 == gBattleAnimTarget as i32) as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAllNonAttackersInvisiblity(taskId: u8) {
    let mut battler: u16 = 0;
    battler = 0;
    while battler < MAX_BATTLERS_COUNT as u16 {
        if battler != gBattleAnimAttacker as u16 && IsBattlerSpriteVisible(battler as u8) != 0 {
            gSprites[gBattlerSpriteIds[battler]].set_invisible(gBattleAnimArgs[0] as u16);
        }
        battler += 1;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMonScrollingBgMask(
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
    let mut spriteId: u8 = 0;
    let mut spriteId2: u8 = 0;
    let mut bg1Cnt: u16 = 0;
    let mut animBgData: BattleAnimBgData = zeroed();
    let mut battler2: u8 = 0;
    spriteId2 = 0;
    battler2 = battler ^ 2;
    if IsContest() != 0 || includePartner != 0 && IsBattlerSpriteVisible(battler2) == 0 {
        includePartner = FALSE;
    }
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    bg1Cnt = GetGpuReg(REG_OFFSET_BG1CNT);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_screenSize(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_areaOverflowMode(1);
    if IsContest() == 0 {
        (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(1);
    }
    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    if IsContest() != 0 {
        species = (*(*gContestResources).moveAnim).species;
    } else {
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    spriteId =
        CreateInvisibleSpriteCopy(battler as i32, gBattlerSpriteIds[battler], species as i32);
    if includePartner != 0 {
        spriteId2 =
            CreateInvisibleSpriteCopy(battler2 as i32, gBattlerSpriteIds[battler2], species as i32);
    }
    GetBattleAnimBg1Data(&raw mut animBgData);
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBgData,
        tilemap as *mut c_void,
        FALSE as u32,
    );
    AnimLoadCompressedBgGfx(animBgData.bgId as u32, gfx, animBgData.tilesOffset as u32);
    LoadCompressedPalette(palette, 0x000 + animBgData.paletteId as u16 * 16, 32);
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gTasks[taskId].data[1] = scrollSpeed as i16;
    gTasks[taskId].data[4] = numFadeSteps as i16;
    gTasks[taskId].data[5] = duration as i16;
    gTasks[taskId].data[6] = fadeStepDelay as i16;
    gTasks[taskId].data[0] = spriteId as i16;
    gTasks[taskId].data[2] = includePartner as i16;
    gTasks[taskId].data[3] = spriteId2 as i16;
    gTasks[taskId].func = Some(UpdateMonScrollingBgMask);
}
pub(crate) unsafe extern "C" fn UpdateMonScrollingBgMask(taskId: u8) {
    gTasks[taskId].data[13] += (if gTasks[taskId].data[1] < 0 {
        -(gTasks[taskId].data[1] as i32)
    } else {
        gTasks[taskId].data[1] as i32
    }) as i16;
    if gTasks[taskId].data[1] < 0 {
        gBattle_BG1_Y -= (gTasks[taskId].data[13] >> 8) as u16;
    } else {
        gBattle_BG1_Y += (gTasks[taskId].data[13] >> 8) as u16;
    }
    gTasks[taskId].data[13] &= 0xFF;
    match gTasks[taskId].data[15] {
        0 => {
            if ({
                let t1 = gTasks[taskId].data[11];
                gTasks[taskId].data[11] += 1;
                t1
            }) >= gTasks[taskId].data[6]
            {
                gTasks[taskId].data[11] = 0;
                gTasks[taskId].data[12] += 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[12] as u16) << 8 | gTasks[taskId].data[12] as u16,
                );
                if gTasks[taskId].data[12] == gTasks[taskId].data[4] {
                    gTasks[taskId].data[15] += 1;
                }
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) == gTasks[taskId].data[5]
            {
                gTasks[taskId].data[15] += 1;
            }
        }
        2 => {
            if ({
                let t3 = gTasks[taskId].data[11];
                gTasks[taskId].data[11] += 1;
                t3
            }) >= gTasks[taskId].data[6]
            {
                gTasks[taskId].data[11] = 0;
                gTasks[taskId].data[12] -= 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[12] as u16) << 8 | gTasks[taskId].data[12] as u16,
                );
                if gTasks[taskId].data[12] == 0 {
                    ResetBattleAnimBg(FALSE);
                    gBattle_WIN0H = 0;
                    gBattle_WIN0V = 0;
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
                    DestroySprite(&raw mut gSprites[gTasks[taskId].data[0]]);
                    if gTasks[taskId].data[2] != 0 {
                        DestroySprite(&raw mut gSprites[gTasks[taskId].data[3]]);
                    }
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetBattleEnvironment(taskId: u8) {
    gBattleAnimArgs[0] = gBattleEnvironment as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AllocBackupPalBuffer(taskId: u8) {
    (*gMonSpritesGfxPtr).buffer = AllocZeroed(8192) as *mut u16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FreeBackupPalBuffer(taskId: u8) {
    Free((*gMonSpritesGfxPtr).buffer as *mut c_void);
    (*gMonSpritesGfxPtr).buffer = null_mut();
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CopyPalUnfadedToBackup(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut paletteIndex: i32 = 0;
    if gBattleAnimArgs[0] == 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
        while selectedPalettes & 1 == 0 {
            selectedPalettes >>= 1;
            paletteIndex += 1;
        }
    } else if gBattleAnimArgs[0] == 1 {
        paletteIndex = gBattleAnimAttacker as i32 + 16;
    } else if gBattleAnimArgs[0] == 2 {
        paletteIndex = gBattleAnimTarget as i32 + 16;
    }
    memcpy(
        (*gMonSpritesGfxPtr)
            .buffer
            .at(gBattleAnimArgs[1] as i32 * 16) as *mut u8,
        &raw mut gPlttBufferUnfaded[paletteIndex * 16] as *mut u8,
        32,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CopyPalUnfadedFromBackup(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut paletteIndex: i32 = 0;
    if gBattleAnimArgs[0] == 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
        while selectedPalettes & 1 == 0 {
            selectedPalettes >>= 1;
            paletteIndex += 1;
        }
    } else if gBattleAnimArgs[0] == 1 {
        paletteIndex = gBattleAnimAttacker as i32 + 16;
    } else if gBattleAnimArgs[0] == 2 {
        paletteIndex = gBattleAnimTarget as i32 + 16;
    }
    memcpy(
        &raw mut gPlttBufferUnfaded[paletteIndex * 16] as *mut u8,
        (*gMonSpritesGfxPtr)
            .buffer
            .at(gBattleAnimArgs[1] as i32 * 16) as *mut u8,
        32,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CopyPalFadedToUnfaded(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    let mut paletteIndex: i32 = 0;
    if gBattleAnimArgs[0] == 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
        while selectedPalettes & 1 == 0 {
            selectedPalettes >>= 1;
            paletteIndex += 1;
        }
    } else if gBattleAnimArgs[0] == 1 {
        paletteIndex = gBattleAnimAttacker as i32 + 16;
    } else if gBattleAnimArgs[0] == 2 {
        paletteIndex = gBattleAnimTarget as i32 + 16;
    }
    memcpy(
        &raw mut gPlttBufferUnfaded[paletteIndex * 16] as *mut u8,
        &raw mut gPlttBufferFaded[paletteIndex * 16] as *mut u8,
        32,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsContest(taskId: u8) {
    if IsContest() != 0 {
        gBattleAnimArgs[7] = TRUE as i16;
    } else {
        gBattleAnimArgs[7] = FALSE as i16;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAnimAttackerAndTargetForEffectTgt(taskId: u8) {
    gBattleAnimAttacker = gBattlerTarget;
    gBattleAnimTarget = gEffectBattler;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsTargetSameSide(taskId: u8) {
    if GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget) {
        gBattleAnimArgs[7] = TRUE as i16;
    } else {
        gBattleAnimArgs[7] = FALSE as i16;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAnimTargetToBattlerTarget(taskId: u8) {
    gBattleAnimTarget = gBattlerTarget;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAnimAttackerAndTargetForEffectAtk(taskId: u8) {
    gBattleAnimAttacker = gBattlerAttacker;
    gBattleAnimTarget = gEffectBattler;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAttackerInvisibleWaitForSignal(taskId: u8) {
    if IsContest() != 0 {
        DestroyAnimVisualTask(taskId);
    } else {
        gTasks[taskId].data[0] =
            (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker)).invisible() as i16;
        (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker)).set_invisible(TRUE as u16);
        gTasks[taskId].func = Some(AnimTask_WaitAndRestoreVisibility);
        gAnimVisualTaskCount -= 1;
    }
}
pub(crate) unsafe extern "C" fn AnimTask_WaitAndRestoreVisibility(taskId: u8) {
    if gBattleAnimArgs[7] == 0x1000 {
        (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker))
            .set_invisible(gTasks[taskId].data[0] as u8 as u16 & 1);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetAnimBgAttribute(bgId: u8, attributeId: u8, value: u8) {
    if bgId < 4 {
        SetAnimBgAttribute_sBgCnt = GetGpuReg(sBattleAnimBgCntSet[bgId]);
        match attributeId {
            BG_ANIM_SCREEN_SIZE => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt)).set_screenSize(value as u16);
            }
            BG_ANIM_AREA_OVERFLOW_MODE => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt))
                    .set_areaOverflowMode(value as u16);
            }
            BG_ANIM_MOSAIC => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt)).set_mosaic(value as u16);
            }
            BG_ANIM_CHAR_BASE_BLOCK => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt))
                    .set_charBaseBlock(value as u16);
            }
            BG_ANIM_PRIORITY => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt)).set_priority(value as u16);
            }
            BG_ANIM_PALETTES_MODE => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt)).set_palettes(value as u16);
            }
            BG_ANIM_SCREEN_BASE_BLOCK => {
                (*(&raw mut SetAnimBgAttribute_sBgCnt as *mut BgCnt))
                    .set_screenBaseBlock(value as u16);
            }
            _ => {}
        }
        SetGpuReg(sBattleAnimBgCntSet[bgId], SetAnimBgAttribute_sBgCnt);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAnimBgAttribute(bgId: u8, attributeId: u8) -> i32 {
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
    return 0;
}
