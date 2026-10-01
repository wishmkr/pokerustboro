//! Translated from `src/battle_anim.c` by tools/rustport/c2rs.py.
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
    clippy::manual_clamp,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_anim_mons::{
    ClearBattleAnimBg, GetAnimBattlerSpriteId, GetBattleAnimBg1Data, GetBattleAnimBgData,
    GetBattleBgPaletteNum, GetBattlerPosition, GetBattlerSide, GetBattlerSpriteBGPriorityRank,
    GetBattlerSpriteCoord, GetBattlerSpriteSubpriority, InitPrioritiesForVisibleBattlers,
    IsBattlerSpritePresent, IsDoubleBattle,
};
use crate::battle_anim_utility_funcs::SetAnimBgAttribute;
use crate::battle_bg::DrawMainBattleBackground;
use crate::battle_interface::UpdateOamPriorityInAllHealthboxes;
use crate::battle_intro::DrawBattlerOnBg;
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattle_WIN0H, gBattle_WIN0V,
    gBattle_WIN1H, gBattle_WIN1V, gBattleSpritesDataPtr, gBattlerAttacker, gBattlerTarget,
};
use crate::battle_main::{gBattlerPartyIndexes, gBattlerSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{IsSpeciesNotUnown, LoadContestBgAfterMoveAnim, gContestResources};
use crate::gpu_regs::SetGpuReg;
use crate::m4a::{
    gMPlayInfo_BGM, gMPlayInfo_SE1, gMPlayInfo_SE2, m4aMPlayStop, m4aMPlayVolumeControl,
};
use crate::palette::{BeginHardwarePaletteFade, LoadCompressedPalette, LoadPalette, gPaletteFade};
use crate::pokemon::{GetMonData2, gEnemyParty, gPlayerParty};
use crate::sound::{IsSEPlaying, PlaySE, PlaySE12WithPanning, SE12PanpotControl};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::task::DestroyTask;
use crate::task::{gTasks, task_func, task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSpriteAndAnimate` with this module's view of its types.
#[inline]
unsafe fn CreateSpriteAndAnimate(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSpriteAndAnimate(a0 as _, a1, a2, a3) }
}
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
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
    }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePaletteUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePaletteUsingHeap(a0: *mut CompressedSpritePalette) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpritePaletteUsingHeap(a0 as _) }
}
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `RequestDma3Fill` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Fill(a0, a1 as _, a2, a3) }
}
// The C's names for task and sprite data slots.
const t2_SpriteId: usize = 0;
const tBackgroundId: usize = 0;
const tBattlerId: usize = 0;
const tInitialPan: usize = 0;
const tSongId: usize = 0;
const t2_SpriteX: usize = 1;
const tInBg2: usize = 1;
const tPanning: usize = 1;
const tTargetPan: usize = 1;
const t2_SpriteY: usize = 2;
const tActive: usize = 2;
const tIncrementPan: usize = 2;
const t2_BgX: usize = 3;
const tIsPartner: usize = 3;
const tNumberOfPlays: usize = 3;
const t2_BgY: usize = 4;
const tCurrentPan: usize = 4;
const t2_InBg2: usize = 5;
const t2_BattlerId: usize = 6;
const tFrameCounter: usize = 8;
const tState: usize = 10;
// Data tables (translate with cdata.py): gOamData_AffineOff_ObjNormal_8x8 gOamData_AffineOff_ObjNormal_16x16 gOamData_AffineOff_ObjNormal_32x32 gOamData_AffineOff_ObjNormal_64x64 gOamData_AffineOff_ObjNormal_16x8 gOamData_AffineOff_ObjNormal_32x8 gOamData_AffineOff_ObjNormal_32x16 gOamData_AffineOff_ObjNormal_64x32 gOamData_AffineOff_ObjNormal_8x16 gOamData_AffineOff_ObjNormal_8x32 gOamData_AffineOff_ObjNormal_16x32 gOamData_AffineOff_ObjNormal_32x64 gOamData_AffineNormal_ObjNormal_8x8 gOamData_AffineNormal_ObjNormal_16x16 gOamData_AffineNormal_ObjNormal_32x32 gOamData_AffineNormal_ObjNormal_64x64 gOamData_AffineNormal_ObjNormal_16x8 gOamData_AffineNormal_ObjNormal_32x8 gOamData_AffineNormal_ObjNormal_32x16 gOamData_AffineNormal_ObjNormal_64x32 gOamData_AffineNormal_ObjNormal_8x16 gOamData_AffineNormal_ObjNormal_8x32 gOamData_AffineNormal_ObjNormal_16x32 gOamData_AffineNormal_ObjNormal_32x64 gOamData_AffineDouble_ObjNormal_8x8 gOamData_AffineDouble_ObjNormal_16x16 gOamData_AffineDouble_ObjNormal_32x32 gOamData_AffineDouble_ObjNormal_64x64 gOamData_AffineDouble_ObjNormal_16x8 gOamData_AffineDouble_ObjNormal_32x8 gOamData_AffineDouble_ObjNormal_32x16 gOamData_AffineDouble_ObjNormal_64x32 gOamData_AffineDouble_ObjNormal_8x16 gOamData_AffineDouble_ObjNormal_8x32 gOamData_AffineDouble_ObjNormal_16x32 gOamData_AffineDouble_ObjNormal_32x64 gOamData_AffineOff_ObjBlend_8x8 gOamData_AffineOff_ObjBlend_16x16 gOamData_AffineOff_ObjBlend_32x32 gOamData_AffineOff_ObjBlend_64x64 gOamData_AffineOff_ObjBlend_16x8 gOamData_AffineOff_ObjBlend_32x8 gOamData_AffineOff_ObjBlend_32x16 gOamData_AffineOff_ObjBlend_64x32 gOamData_AffineOff_ObjBlend_8x16 gOamData_AffineOff_ObjBlend_8x32 gOamData_AffineOff_ObjBlend_16x32 gOamData_AffineOff_ObjBlend_32x64 gOamData_AffineNormal_ObjBlend_8x8 gOamData_AffineNormal_ObjBlend_16x16 gOamData_AffineNormal_ObjBlend_32x32 gOamData_AffineNormal_ObjBlend_64x64 gOamData_AffineNormal_ObjBlend_16x8 gOamData_AffineNormal_ObjBlend_32x8 gOamData_AffineNormal_ObjBlend_32x16 gOamData_AffineNormal_ObjBlend_64x32 gOamData_AffineNormal_ObjBlend_8x16 gOamData_AffineNormal_ObjBlend_8x32 gOamData_AffineNormal_ObjBlend_16x32 gOamData_AffineNormal_ObjBlend_32x64 gOamData_AffineDouble_ObjBlend_8x8 gOamData_AffineDouble_ObjBlend_16x16 gOamData_AffineDouble_ObjBlend_32x32 gOamData_AffineDouble_ObjBlend_64x64 gOamData_AffineDouble_ObjBlend_16x8 gOamData_AffineDouble_ObjBlend_32x8 gOamData_AffineDouble_ObjBlend_32x16 gOamData_AffineDouble_ObjBlend_64x32 gOamData_AffineDouble_ObjBlend_8x16 gOamData_AffineDouble_ObjBlend_8x32 gOamData_AffineDouble_ObjBlend_16x32 gOamData_AffineDouble_ObjBlend_32x64 gBattleAnimPicTable gBattleAnimPaletteTable gBattleAnimBackgroundTable sScriptCmdTable

const ANIM_SPRITE_INDEX_COUNT: i32 = 8;

static gBattleAnimBackgroundTable: Table<CArray<BattleAnimBackground, 27>> =
    Table((&raw const crate::data::battle_anim::gBattleAnimBackgroundTable).cast());
static gBattleAnimPaletteTable: Table<CArray<CompressedSpritePalette, 289>> =
    Table((&raw const crate::data::battle_anim::gBattleAnimPaletteTable).cast());
static gBattleAnimPicTable: Table<CArray<CompressedSpriteSheet, 289>> =
    Table((&raw const crate::data::battle_anim::gBattleAnimPicTable).cast());
static sScriptCmdTable: Table<CArray<Option<unsafe fn()>, 48>> =
    Table((&raw const crate::data::battle_anim::sScriptCmdTable).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleAnimScriptPtr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleAnimScriptRetAddr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimScriptCallback: Option<unsafe fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static sAnimFramesToWait: crate::global::Global<i8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimScriptActive: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimVisualTaskCount: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static gAnimSoundTaskCount: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimDisableStructPtr: *mut DisableStruct = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMoveDmg: i32 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMovePower: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimSpriteIndexArray: Aligned<CArray<u16, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimFriendship: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gWeatherMoveAnim: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimArgs: Aligned<CArray<i16, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static sSoundAnimFramesToWait: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonAnimTaskIdArray: Aligned<CArray<u8, 2>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMoveTurn: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static sAnimBackgroundFadeState: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sAnimMoveIndex: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimAttacker: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimTarget: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimBattlerSpecies: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimCustomPanning: u8 = 0;

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn ClearBattleAnimationVars() {
    sAnimFramesToWait.set(0);
    gAnimScriptActive = FALSE;
    gAnimVisualTaskCount = 0;
    gAnimSoundTaskCount.set(0);
    gAnimDisableStructPtr = null_mut();
    gAnimMoveDmg = 0;
    gAnimMovePower = 0;
    gAnimFriendship = 0;
    let mut i: i32 = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        sAnimSpriteIndexArray[i] = 0xFFFF;
        i += 1;
    }
    for i in 0..ANIM_ARGS_COUNT {
        gBattleAnimArgs[i] = 0;
    }
    sMonAnimTaskIdArray[0] = TASK_NONE;
    sMonAnimTaskIdArray[1] = TASK_NONE;
    gAnimMoveTurn = 0;
    sAnimBackgroundFadeState.set(0);
    sAnimMoveIndex.set(0);
    gBattleAnimAttacker = 0;
    gBattleAnimTarget = 0;
    gAnimCustomPanning = 0;
}
pub unsafe fn DoMoveAnim(r#move: u16) {
    gBattleAnimAttacker = gBattlerAttacker;
    gBattleAnimTarget = gBattlerTarget;
    LaunchBattleAnimation(
        (*crate::asmdata::gBattleAnims_Moves.cast::<CArray<*mut u8, 0>>())
            .as_ptr()
            .cast_mut(),
        r#move,
        TRUE,
    );
}
pub unsafe fn LaunchBattleAnimation(animsTable: *mut *mut u8, tableId: u16, isMoveAnim: u8) {
    let mut i: i32 = 0;
    if IsContest() == 0 {
        InitPrioritiesForVisibleBattlers();
        UpdateOamPriorityInAllHealthboxes(0);
        for i in 0..(MAX_BATTLERS_COUNT as i32) {
            if GetBattlerSide(i as u8) != B_SIDE_PLAYER {
                gAnimBattlerSpecies[i] = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                    MON_DATA_SPECIES,
                ) as u16;
            } else {
                gAnimBattlerSpecies[i] = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[i]],
                    MON_DATA_SPECIES,
                ) as u16;
            }
        }
    } else {
        for i in 0..CONTESTANT_COUNT {
            gAnimBattlerSpecies[i] = (*(*gContestResources).moveAnim).species;
        }
    }
    if isMoveAnim == 0 {
        sAnimMoveIndex.set(0);
    } else {
        sAnimMoveIndex.set(tableId);
    }
    for i in 0..ANIM_ARGS_COUNT {
        gBattleAnimArgs[i] = 0;
    }
    sMonAnimTaskIdArray[0] = TASK_NONE;
    sMonAnimTaskIdArray[1] = TASK_NONE;
    sBattleAnimScriptPtr = *animsTable.at(tableId);
    gAnimScriptActive = TRUE;
    sAnimFramesToWait.set(0);
    gAnimScriptCallback = Some(RunAnimScriptCommand);
    i = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        sAnimSpriteIndexArray[i] = 0xFFFF;
        i += 1;
    }
    if isMoveAnim != 0 {
        i = 0;
        while (*crate::asmdata::gMovesWithQuietBGM.cast::<CArray<u16, 0>>())[i] != 0xFFFF {
            if tableId == (*crate::asmdata::gMovesWithQuietBGM.cast::<CArray<u16, 0>>())[i] {
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 128);
                break;
            }
            i += 1;
        }
    }
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    gBattle_WIN1H = 0;
    gBattle_WIN1V = 0;
}
pub unsafe fn DestroyAnimSprite(sprite: *mut Sprite) {
    FreeSpriteOamMatrix(sprite);
    DestroySprite(sprite);
    gAnimVisualTaskCount -= 1;
}
pub unsafe fn DestroyAnimVisualTask(taskId: u8) {
    DestroyTask(taskId);
    gAnimVisualTaskCount -= 1;
}
pub fn DestroyAnimSoundTask(taskId: u8) {
    DestroyTask(taskId);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() - 1);
}
unsafe fn AddSpriteIndex(index: u16) {
    for i in 0..ANIM_SPRITE_INDEX_COUNT {
        if sAnimSpriteIndexArray[i] == 0xFFFF {
            sAnimSpriteIndexArray[i] = index;
            return;
        }
    }
}
unsafe fn ClearSpriteIndex(index: u16) {
    for i in 0..ANIM_SPRITE_INDEX_COUNT {
        if sAnimSpriteIndexArray[i] == index {
            sAnimSpriteIndexArray[i] = 0xFFFF;
            return;
        }
    }
}
pub(crate) unsafe fn WaitAnimFrameCount() {
    if sAnimFramesToWait.get() <= 0 {
        gAnimScriptCallback = Some(RunAnimScriptCommand);
        sAnimFramesToWait.set(0);
    } else {
        sAnimFramesToWait.set(sAnimFramesToWait.get() - 1);
    }
}
pub(crate) unsafe fn RunAnimScriptCommand() {
    loop {
        sScriptCmdTable[*sBattleAnimScriptPtr].unwrap_unchecked()();
        if !(sAnimFramesToWait.get() == 0 && gAnimScriptActive != 0) {
            break;
        }
    }
}
pub(crate) unsafe fn Cmd_loadspritegfx() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let index: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    LoadCompressedSpriteSheetUsingHeap(
        (&raw const gBattleAnimPicTable[index as i32 - 10000]).cast_mut(),
    );
    LoadCompressedSpritePaletteUsingHeap(
        (&raw const gBattleAnimPaletteTable[index as i32 - 10000]).cast_mut(),
    );
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    AddSpriteIndex(index - 10000);
    sAnimFramesToWait.set(1);
    gAnimScriptCallback = Some(WaitAnimFrameCount);
}
pub(crate) unsafe fn Cmd_unloadspritegfx() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let index: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    FreeSpriteTilesByTag(gBattleAnimPicTable[index as i32 - 10000].tag);
    FreeSpritePaletteByTag(gBattleAnimPicTable[index as i32 - 10000].tag);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    ClearSpriteIndex(index - 10000);
}
pub(crate) unsafe fn Cmd_createsprite() {
    let mut subpriority: i16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let template: *mut SpriteTemplate = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24))
        as usize as *mut SpriteTemplate;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    let mut argVar: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let argsCount: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    for i in 0..(argsCount as i32) {
        gBattleAnimArgs[i] =
            *sBattleAnimScriptPtr as i16 | (*sBattleAnimScriptPtr.at(1) as i16) << 8;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    }
    if argVar as i32 & ANIMSPRITE_IS_TARGET != 0 {
        argVar ^= ANIMSPRITE_IS_TARGET as u8;
        if argVar >= 64 {
            argVar -= 64;
        } else {
            argVar *= 255;
        }
        subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) as i16 + argVar as i8 as i16;
    } else {
        if argVar >= 64 {
            argVar -= 64;
        } else {
            argVar *= 255;
        }
        subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) as i16 + argVar as i8 as i16;
    }
    if subpriority < 3 {
        subpriority = 3;
    }
    CreateSpriteAndAnimate(
        template,
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16,
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16,
        subpriority as u8,
    );
    gAnimVisualTaskCount += 1;
}
pub(crate) unsafe fn Cmd_createvisualtask() {
    let mut taskFunc: Option<unsafe fn(u8)> = None;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    taskFunc = core::mem::transmute::<usize, Option<unsafe fn(u8)>>(
        (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize,
    );
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    let taskPriority: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let numArgs: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    for i in 0..(numArgs as i32) {
        gBattleAnimArgs[i] =
            *sBattleAnimScriptPtr as i16 | (*sBattleAnimScriptPtr.at(1) as i16) << 8;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    }
    let taskId: u8 = CreateTask(taskFunc, taskPriority);
    taskFunc.unwrap_unchecked()(taskId);
    gAnimVisualTaskCount += 1;
}
pub(crate) unsafe fn Cmd_delay() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sAnimFramesToWait.set(*sBattleAnimScriptPtr as i8);
    if sAnimFramesToWait.get() == 0 {
        sAnimFramesToWait.set(-1);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    gAnimScriptCallback = Some(WaitAnimFrameCount);
}
pub(crate) unsafe fn Cmd_waitforvisualfinish() {
    if gAnimVisualTaskCount == 0 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait.set(0);
    } else {
        sAnimFramesToWait.set(1);
    }
}
pub(crate) fn Cmd_nop() {}
pub(crate) fn Cmd_nop2() {}
pub(crate) unsafe fn Cmd_end() {
    let continuousAnim: u32 = FALSE as u32;
    if gAnimVisualTaskCount != 0
        || gAnimSoundTaskCount.get() != 0
        || sMonAnimTaskIdArray[0] != TASK_NONE
        || sMonAnimTaskIdArray[1] != TASK_NONE
    {
        sSoundAnimFramesToWait.set(0);
        sAnimFramesToWait.set(1);
        return;
    }
    if IsSEPlaying() != 0 {
        if ({
            sSoundAnimFramesToWait.set(sSoundAnimFramesToWait.get() + 1);
            sSoundAnimFramesToWait.get()
        }) <= 90
        {
            sAnimFramesToWait.set(1);
            return;
        } else {
            m4aMPlayStop(&raw mut gMPlayInfo_SE1);
            m4aMPlayStop(&raw mut gMPlayInfo_SE2);
        }
    }
    sSoundAnimFramesToWait.set(0);
    for i in 0..ANIM_SPRITE_INDEX_COUNT {
        if sAnimSpriteIndexArray[i] != 0xFFFF {
            FreeSpriteTilesByTag(gBattleAnimPicTable[sAnimSpriteIndexArray[i]].tag);
            FreeSpritePaletteByTag(gBattleAnimPicTable[sAnimSpriteIndexArray[i]].tag);
            sAnimSpriteIndexArray[i] = 0xFFFF;
        }
    }
    if continuousAnim == 0 {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 256);
        if IsContest() == 0 {
            InitPrioritiesForVisibleBattlers();
            UpdateOamPriorityInAllHealthboxes(1);
        }
        gAnimScriptActive = FALSE;
    }
}
pub(crate) unsafe fn Cmd_playse() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    PlaySE(*sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
}
pub(crate) unsafe fn Task_InitUpdateMonBg(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let battlerSpriteId: u8 = gBattlerSpriteIds[*data];
    gSprites[battlerSpriteId].set_invisible(TRUE as u16);
    if *data.at(2) == 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    let updateTaskId: u8 = CreateTask(Some(Task_UpdateMonBg), 10);
    task_set(updateTaskId, 0, battlerSpriteId as i16);
    task_set(
        updateTaskId,
        1,
        gSprites[battlerSpriteId].x + gSprites[battlerSpriteId].x2,
    );
    task_set(
        updateTaskId,
        2,
        gSprites[battlerSpriteId].y + gSprites[battlerSpriteId].y2,
    );
    if *data.at(1) == 0 {
        task_set(updateTaskId, 3, gBattle_BG1_X as i16);
        task_set(updateTaskId, t2_BgY, gBattle_BG1_Y as i16);
    } else {
        task_set(updateTaskId, 3, gBattle_BG2_X as i16);
        task_set(updateTaskId, t2_BgY, gBattle_BG2_Y as i16);
    }
    task_set(updateTaskId, t2_InBg2, *data.at(1));
    task_set(updateTaskId, t2_BattlerId, *data);
    sMonAnimTaskIdArray[*data.at(3)] = updateTaskId;
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn Cmd_monbg() {
    let mut toBG_2: u8 = 0;
    let mut taskId: u8 = 0;
    let mut battler: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let animBattler: u8 = *sBattleAnimScriptPtr;
    if animBattler as i32 & ANIM_TARGET as i32 != 0 {
        battler = gBattleAnimTarget;
    } else {
        battler = gBattleAnimAttacker;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        let position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
        taskId = CreateTask(Some(Task_InitUpdateMonBg), 10);
        gAnimVisualTaskCount += 1;
        task_set(taskId, tBattlerId, battler as i16);
        task_set(taskId, tInBg2, toBG_2 as i16);
        task_set(taskId, tActive, TRUE as i16);
        task_set(taskId, tIsPartner, FALSE as i16);
    }
    battler ^= BIT_FLANK;
    if IsBattlerSpriteVisible(battler) != 0 {
        let position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
        taskId = CreateTask(Some(Task_InitUpdateMonBg), 10);
        gAnimVisualTaskCount += 1;
        task_set(taskId, tBattlerId, battler as i16);
        task_set(taskId, tInBg2, toBG_2 as i16);
        task_set(taskId, tActive, TRUE as i16);
        task_set(taskId, tIsPartner, TRUE as i16);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sAnimFramesToWait.set(1);
    gAnimScriptCallback = Some(WaitAnimFrameCount);
}
pub unsafe fn IsBattlerSpriteVisible(battler: u8) -> u8 {
    if IsContest() != 0 {
        if battler == gBattleAnimAttacker {
            return TRUE;
        } else {
            return FALSE;
        }
    }
    if IsBattlerSpritePresent(battler) == 0 {
        return FALSE;
    }
    if IsContest() != 0 {
        return TRUE;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).invisible() == 0
        || gSprites[gBattlerSpriteIds[battler]].invisible() == 0
    {
        return TRUE;
    }
    FALSE
}
pub unsafe fn MoveBattlerSpriteToBG(battler: u8, toBG_2: u8, setSpriteInvisible: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut battlerSpriteId: u8 = 0;
    if toBG_2 == 0 {
        let mut battlerPosition: u8 = 0;
        if IsContest() == TRUE {
            RequestDma3Fill(0, 0x6008000_usize as *mut c_void, 0x2000, 1);
            RequestDma3Fill(0xFF, 0x600f000_usize as *mut c_void, 0x1000, 0);
        } else {
            RequestDma3Fill(0, 0x6004000_usize as *mut c_void, 0x2000, 1);
            RequestDma3Fill(0xFF, 0x600e000_usize as *mut c_void, 0x1000, 0);
        }
        GetBattleAnimBg1Data(&raw mut animBg);
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTiles as *mut c_void,
                    0x1000800,
                );
            }
        }
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 255);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTilemap as *mut c_void,
                    0x1000400,
                );
            }
        }
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 2);
        SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 1);
        SetAnimBgAttribute(1, BG_ANIM_AREA_OVERFLOW_MODE, 0);
        battlerSpriteId = gBattlerSpriteIds[battler];
        gBattle_BG1_X = (gSprites[battlerSpriteId].x as u16 + gSprites[battlerSpriteId].x2 as u16)
            .wrapping_neg()
            + 0x20;
        if IsContest() != 0 && IsSpeciesNotUnown((*(*gContestResources).moveAnim).species) != 0 {
            gBattle_BG1_X -= 1;
        }
        gBattle_BG1_Y = (gSprites[battlerSpriteId].y as u16 + gSprites[battlerSpriteId].y2 as u16)
            .wrapping_neg()
            + 0x20;
        if setSpriteInvisible != 0 {
            gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
        }
        SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
        SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
        LoadPalette(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + battler as i32 * 16] as *mut c_void,
            animBg.paletteId as u16 * 16,
            32,
        );
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + battler as i32 * 16] as *mut c_void,
            (BG_PLTT + animBg.paletteId as u32 * 32) as usize as *mut c_void,
            0x4000008,
        );
        if IsContest() != 0 {
            battlerPosition = 0;
        } else {
            battlerPosition = GetBattlerPosition(battler);
        }
        DrawBattlerOnBg(
            1,
            0,
            0,
            battlerPosition,
            animBg.paletteId,
            animBg.bgTiles,
            animBg.bgTilemap,
            animBg.tilesOffset,
        );
        if IsContest() != 0 {
            FlipBattlerBgTiles();
        }
    } else {
        RequestDma3Fill(0, 0x6006000_usize as *mut c_void, 0x2000, 1);
        RequestDma3Fill(0, 0x600f000_usize as *mut c_void, 0x1000, 1);
        GetBattleAnimBgData(&raw mut animBg, 2);
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTiles.at(4096) as *mut c_void,
                    0x1000800,
                );
            }
        }
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTilemap.at(1024) as *mut c_void,
                    0x1000400,
                );
            }
        }
        SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
        SetAnimBgAttribute(2, BG_ANIM_SCREEN_SIZE, 1);
        SetAnimBgAttribute(2, BG_ANIM_AREA_OVERFLOW_MODE, 0);
        battlerSpriteId = gBattlerSpriteIds[battler];
        gBattle_BG2_X = (gSprites[battlerSpriteId].x as u16 + gSprites[battlerSpriteId].x2 as u16)
            .wrapping_neg()
            + 0x20;
        gBattle_BG2_Y = (gSprites[battlerSpriteId].y as u16 + gSprites[battlerSpriteId].y2 as u16)
            .wrapping_neg()
            + 0x20;
        if setSpriteInvisible != 0 {
            gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
        }
        SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
        SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
        LoadPalette(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + battler as i32 * 16] as *mut c_void,
            144,
            32,
        );
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + battler as i32 * 16] as *mut c_void,
            0x5000120_usize as *mut c_void,
            0x4000008,
        );
        DrawBattlerOnBg(
            2,
            0,
            0,
            GetBattlerPosition(battler),
            animBg.paletteId,
            animBg.bgTiles.at(4096),
            animBg.bgTilemap.at(1024),
            animBg.tilesOffset,
        );
    }
}
unsafe fn FlipBattlerBgTiles() {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut ptr: *mut u16 = null_mut();
    if IsSpeciesNotUnown((*(*gContestResources).moveAnim).species) != 0 {
        GetBattleAnimBg1Data(&raw mut animBg);
        ptr = animBg.bgTilemap;
        for i in 0..8i32 {
            for j in 0..4i32 {
                let temp: u16 = *ptr.at(j + i * 32);
                *ptr.at(j + i * 32) = *ptr.at(7 - j + i * 32);
                *ptr.at(7 - j + i * 32) = temp;
            }
        }
        for i in 0..8i32 {
            for j in 0..8i32 {
                *ptr.at(j + i * 32) ^= 0x400;
            }
        }
    }
}
pub unsafe fn RelocateBattleBgPal(
    mut paletteNum: u16,
    dest: *mut u16,
    offset: u32,
    largeScreen: u8,
) {
    let mut size: i32 = 0;
    if largeScreen == 0 {
        size = 32;
    } else {
        size = 64;
    }
    paletteNum <<= 12;
    for i in 0..size {
        for j in 0..32i32 {
            *dest.at(j + i * 32) = (*dest.at(j + i * 32) & 0xFFF | paletteNum) + offset as u16;
        }
    }
}
pub unsafe fn ResetBattleAnimBg(toBG2: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    GetBattleAnimBg1Data(&raw mut animBg);
    if toBG2 == 0 || IsContest() != 0 {
        ClearBattleAnimBg(1);
        gBattle_BG1_X = 0;
        gBattle_BG1_Y = 0;
    } else {
        ClearBattleAnimBg(2);
        gBattle_BG2_X = 0;
        gBattle_BG2_Y = 0;
    }
}
pub(crate) unsafe fn Task_UpdateMonBg(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut animBg: BattleAnimBgData = zeroed();
    let spriteId: u8 = task_get(taskId, t2_SpriteId) as u8;
    let battler: u8 = task_get(taskId, t2_BattlerId) as u8;
    GetBattleAnimBg1Data(&raw mut animBg);
    x = task_get(taskId, t2_SpriteX) - (gSprites[spriteId].x + gSprites[spriteId].x2);
    y = task_get(taskId, t2_SpriteY) - (gSprites[spriteId].y + gSprites[spriteId].y2);
    if task_get(taskId, t2_InBg2) == 0 {
        gBattle_BG1_X = x as u16 + task_get(taskId, t2_BgX) as u16;
        gBattle_BG1_Y = y as u16 + task_get(taskId, t2_BgY) as u16;
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + battler as i32 * 16] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[animBg.paletteId as i32 * 16] as *mut c_void,
            0x4000008,
        );
    } else {
        gBattle_BG2_X = x as u16 + task_get(taskId, t2_BgX) as u16;
        gBattle_BG2_Y = y as u16 + task_get(taskId, t2_BgY) as u16;
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + battler as i32 * 16] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[144] as *mut c_void,
            0x4000008,
        );
    }
}
pub(crate) unsafe fn Cmd_clearmonbg() {
    let mut battler: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let mut animBattlerId: u8 = *sBattleAnimScriptPtr;
    if animBattlerId == ANIM_ATTACKER {
        animBattlerId = ANIM_ATK_PARTNER;
    } else if animBattlerId == ANIM_TARGET {
        animBattlerId = ANIM_DEF_PARTNER;
    }
    if animBattlerId == ANIM_ATTACKER || animBattlerId == ANIM_ATK_PARTNER {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if sMonAnimTaskIdArray[0] != TASK_NONE {
        gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
    }
    if animBattlerId > 1 && sMonAnimTaskIdArray[1] != TASK_NONE {
        gSprites[gBattlerSpriteIds[battler as i32 ^ 2]].set_invisible(FALSE as u16);
    } else {
        animBattlerId = 0;
    }
    let taskId: u8 = CreateTask(Some(Task_ClearMonBg), 5);
    task_set(taskId, 0, animBattlerId as i16);
    task_set(taskId, 2, battler as i16);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe fn Task_ClearMonBg(taskId: u8) {
    task_set(taskId, 1, task_get(taskId, 1) + 1);
    if task_get(taskId, 1) != 1 {
        let mut to_BG2: u8 = 0;
        let position: u8 = GetBattlerPosition(task_get(taskId, 2) as u8);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            to_BG2 = FALSE;
        } else {
            to_BG2 = TRUE;
        }
        if sMonAnimTaskIdArray[0] != TASK_NONE {
            ResetBattleAnimBg(to_BG2);
            DestroyTask(sMonAnimTaskIdArray[0]);
            sMonAnimTaskIdArray[0] = TASK_NONE;
        }
        if task_get(taskId, 0) > 1 {
            ResetBattleAnimBg(to_BG2 ^ 1);
            DestroyTask(sMonAnimTaskIdArray[1]);
            sMonAnimTaskIdArray[1] = TASK_NONE;
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Cmd_monbg_static() {
    let mut toBG_2: u8 = 0;
    let mut battler: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let mut animBattlerId: u8 = *sBattleAnimScriptPtr;
    if animBattlerId == ANIM_ATTACKER {
        animBattlerId = ANIM_ATK_PARTNER;
    } else if animBattlerId == ANIM_TARGET {
        animBattlerId = ANIM_DEF_PARTNER;
    }
    if animBattlerId == ANIM_ATTACKER || animBattlerId == ANIM_ATK_PARTNER {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        let position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
    }
    battler ^= BIT_FLANK;
    if animBattlerId > 1 && IsBattlerSpriteVisible(battler) != 0 {
        let position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe fn Cmd_clearmonbg_static() {
    let mut battler: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let mut animBattlerId: u8 = *sBattleAnimScriptPtr;
    if animBattlerId == ANIM_ATTACKER {
        animBattlerId = ANIM_ATK_PARTNER;
    } else if animBattlerId == ANIM_TARGET {
        animBattlerId = ANIM_DEF_PARTNER;
    }
    if animBattlerId == ANIM_ATTACKER || animBattlerId == ANIM_ATK_PARTNER {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
    }
    if animBattlerId > 1 && IsBattlerSpriteVisible(battler ^ 2) != 0 {
        gSprites[gBattlerSpriteIds[battler as i32 ^ 2]].set_invisible(FALSE as u16);
    } else {
        animBattlerId = 0;
    }
    let taskId: u8 = CreateTask(Some(Task_ClearMonBgStatic), 5);
    task_set(taskId, 0, animBattlerId as i16);
    task_set(taskId, 2, battler as i16);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe fn Task_ClearMonBgStatic(taskId: u8) {
    task_set(taskId, 1, task_get(taskId, 1) + 1);
    if task_get(taskId, 1) != 1 {
        let mut toBG_2: u8 = 0;
        let battler: u8 = task_get(taskId, 2) as u8;
        let position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        if IsBattlerSpriteVisible(battler) != 0 {
            ResetBattleAnimBg(toBG_2);
        }
        if task_get(taskId, 0) > 1 && IsBattlerSpriteVisible(battler ^ 2) != 0 {
            ResetBattleAnimBg(toBG_2 ^ 1);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Cmd_setalpha() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let half1: u16 = *({
        let t2 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t2
    }) as u16;
    let half2: u16 = (*({
        let t4 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t4
    }) as u16)
        << 8;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, half1 | half2);
}
pub(crate) unsafe fn Cmd_setbldcnt() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let half1: u16 = *({
        let t2 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t2
    }) as u16;
    let half2: u16 = (*({
        let t4 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t4
    }) as u16)
        << 8;
    SetGpuReg(REG_OFFSET_BLDCNT, half1 | half2);
}
pub(crate) unsafe fn Cmd_blendoff() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
}
pub(crate) unsafe fn Cmd_call() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptRetAddr = sBattleAnimScriptPtr.at(4);
    sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
}
pub(crate) unsafe fn Cmd_return() {
    sBattleAnimScriptPtr = sBattleAnimScriptRetAddr;
}
pub(crate) unsafe fn Cmd_setarg() {
    let addr: *mut u8 = sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let argId: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let value: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    sBattleAnimScriptPtr = addr.at(4);
    gBattleAnimArgs[argId] = value as i16;
}
pub(crate) unsafe fn Cmd_choosetwoturnanim() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if gAnimMoveTurn as i32 & 1 != 0 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    }
    sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
}
pub(crate) unsafe fn Cmd_jumpifmoveturn() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let toCheck: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if toCheck == gAnimMoveTurn {
        sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
            as *mut c_void as *mut u8;
    } else {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    }
}
pub(crate) unsafe fn Cmd_goto() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
}
pub unsafe fn IsContest() -> u8 {
    if gMain.inBattle() == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Cmd_fadetobg() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let backgroundId: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let taskId: u8 = CreateTask(Some(Task_FadeToBg), 5);
    task_set(taskId, tBackgroundId, backgroundId as i16);
    sAnimBackgroundFadeState.set(1);
}
pub(crate) unsafe fn Cmd_fadetobgfromset() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let bg1: u8 = *sBattleAnimScriptPtr;
    let bg2: u8 = *sBattleAnimScriptPtr.at(1);
    let bg3: u8 = *sBattleAnimScriptPtr.at(2);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(3);
    let taskId: u8 = CreateTask(Some(Task_FadeToBg), 5);
    if IsContest() != 0 {
        task_set(taskId, tBackgroundId, bg3 as i16);
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        task_set(taskId, tBackgroundId, bg2 as i16);
    } else {
        task_set(taskId, tBackgroundId, bg1 as i16);
    }
    sAnimBackgroundFadeState.set(1);
}
pub(crate) unsafe fn Task_FadeToBg(taskId: u8) {
    if task_get(taskId, tState) == 0 {
        BeginHardwarePaletteFade(0xE8, 0, 0, 16, 0);
        task_set(taskId, tState, task_get(taskId, tState) + 1);
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if task_get(taskId, tState) == 1 {
        task_set(taskId, tState, task_get(taskId, tState) + 1);
        sAnimBackgroundFadeState.set(2);
    } else if task_get(taskId, tState) == 2 {
        let bgId: i16 = task_get(taskId, tBackgroundId);
        if bgId == -1 {
            LoadDefaultBg();
        } else {
            LoadMoveBg(bgId as u16);
        }
        BeginHardwarePaletteFade(0xE8, 0, 16, 0, 1);
        task_set(taskId, tState, task_get(taskId, tState) + 1);
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if task_get(taskId, tState) == 3 {
        DestroyTask(taskId);
        sAnimBackgroundFadeState.set(0);
    }
}
unsafe fn LoadMoveBg(bgId: u16) {
    if IsContest() != 0 {
        let tilemap: *mut u32 = gBattleAnimBackgroundTable[bgId].tilemap;
        LZDecompressWram(
            tilemap,
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut c_void,
        );
        RelocateBattleBgPal(
            GetBattleBgPaletteNum() as u16,
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut c_void as *mut u16,
            0x100,
            FALSE,
        );
        let dmaSrc: *mut c_void = (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void;
        let dmaDest: *mut c_void = 0x600d000_usize as *mut c_void;
        {
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, dmaSrc as usize as u32);
                    volatile_write(dmaRegs.at(1), dmaDest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x84000200);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
        LZDecompressVram(
            gBattleAnimBackgroundTable[bgId].image,
            0x6002000_usize as *mut c_void,
        );
        LoadCompressedPalette(
            gBattleAnimBackgroundTable[bgId].palette,
            GetBattleBgPaletteNum() as u16 * 16,
            32,
        );
    } else {
        LZDecompressVram(
            gBattleAnimBackgroundTable[bgId].tilemap,
            0x600d000_usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleAnimBackgroundTable[bgId].image,
            0x6008000_usize as *mut c_void,
        );
        LoadCompressedPalette(gBattleAnimBackgroundTable[bgId].palette, 32, 32);
    }
}
unsafe fn LoadDefaultBg() {
    if IsContest() != 0 {
        LoadContestBgAfterMoveAnim();
    } else {
        DrawMainBattleBackground();
    }
}
pub(crate) unsafe fn Cmd_restorebg() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let taskId: u8 = CreateTask(Some(Task_FadeToBg), 5);
    task_set(taskId, tBackgroundId, -1);
    sAnimBackgroundFadeState.set(1);
}
pub(crate) unsafe fn Cmd_waitbgfadeout() {
    if sAnimBackgroundFadeState.get() == 2 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait.set(0);
    } else {
        sAnimFramesToWait.set(1);
    }
}
pub(crate) unsafe fn Cmd_waitbgfadein() {
    if sAnimBackgroundFadeState.get() == 0 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait.set(0);
    } else {
        sAnimFramesToWait.set(1);
    }
}
pub(crate) unsafe fn Cmd_changebg() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    LoadMoveBg(*sBattleAnimScriptPtr as u16);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub unsafe fn BattleAnimAdjustPanning(mut pan: i8) -> i8 {
    if IsContest() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gBattleAnimAttacker))
        .statusAnimActive()
            != 0
    {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            pan = SOUND_PAN_TARGET;
        } else {
            pan = SOUND_PAN_ATTACKER;
        }
    } else if IsContest() != 0 {
        if gBattleAnimAttacker != gBattleAnimTarget
            || gBattleAnimAttacker != 2
            || pan != SOUND_PAN_TARGET
        {
            pan *= -1;
        }
    } else if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
            if pan == SOUND_PAN_TARGET {
                pan = SOUND_PAN_ATTACKER;
            } else if pan != SOUND_PAN_ATTACKER {
                pan *= -1;
            }
        }
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_OPPONENT {
        if pan == SOUND_PAN_ATTACKER {
            pan = SOUND_PAN_TARGET;
        }
    } else {
        pan *= -1;
    }
    if pan > SOUND_PAN_TARGET {
        pan = SOUND_PAN_TARGET;
    } else if pan < SOUND_PAN_ATTACKER {
        pan = SOUND_PAN_ATTACKER;
    }
    pan
}
pub unsafe fn BattleAnimAdjustPanning2(mut pan: i8) -> i8 {
    if IsContest() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gBattleAnimAttacker))
        .statusAnimActive()
            != 0
    {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            pan = SOUND_PAN_TARGET;
        } else {
            pan = SOUND_PAN_ATTACKER;
        }
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER || IsContest() != 0 {
            pan = -pan;
        }
    }
    pan
}
pub fn KeepPanInRange(panArg: i16, oldPan: i32) -> i16 {
    let mut pan: i16 = panArg;
    if pan > SOUND_PAN_TARGET as i16 {
        pan = SOUND_PAN_TARGET as i16;
    } else if pan < SOUND_PAN_ATTACKER as i16 {
        pan = SOUND_PAN_ATTACKER as i16;
    }
    pan
}
pub fn CalculatePanIncrement(sourcePan: i16, targetPan: i16, incrementPan: i16) -> i16 {
    let mut ret: i16 = 0;
    if sourcePan < targetPan {
        ret = (if incrementPan < 0 {
            -(incrementPan as i32)
        } else {
            incrementPan as i32
        }) as i16;
    } else if sourcePan > targetPan {
        ret = -((if incrementPan < 0 {
            -(incrementPan as i32)
        } else {
            incrementPan as i32
        }) as i16);
    } else {
        ret = 0;
    }
    ret
}
pub(crate) unsafe fn Cmd_playsewithpan() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let songId: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    let pan: i8 = *sBattleAnimScriptPtr.at(2) as i8;
    PlaySE12WithPanning(songId, BattleAnimAdjustPanning(pan));
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(3);
}
pub(crate) unsafe fn Cmd_setpan() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let pan: i8 = *sBattleAnimScriptPtr as i8;
    SE12PanpotControl(BattleAnimAdjustPanning(pan));
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe fn Cmd_panse() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let songNum: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    let currentPanArg: i8 = *sBattleAnimScriptPtr.at(2) as i8;
    let mut incrementPan: i8 = *sBattleAnimScriptPtr.at(3) as i8;
    let incrementPanArg: i8 = *sBattleAnimScriptPtr.at(4) as i8;
    let framesToWait: u8 = *sBattleAnimScriptPtr.at(5);
    let currentPan: i8 = BattleAnimAdjustPanning(currentPanArg);
    let targetPan: i8 = BattleAnimAdjustPanning(incrementPan);
    incrementPan =
        CalculatePanIncrement(currentPan as i16, targetPan as i16, incrementPanArg as i16) as i8;
    let taskId: u8 = CreateTask(Some(Task_PanFromInitialToTarget), 1);
    task_set(taskId, tInitialPan, currentPan as i16);
    task_set(taskId, tTargetPan, targetPan as i16);
    task_set(taskId, tIncrementPan, incrementPan as i16);
    task_set(taskId, 3, framesToWait as i16);
    task_set(taskId, tCurrentPan, currentPan as i16);
    PlaySE12WithPanning(songNum, currentPan);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() + 1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(6);
}
pub unsafe fn Task_PanFromInitialToTarget(taskId: u8) {
    let mut destroyTask: u32 = FALSE as u32;
    if ({
        let t1 = task_get(taskId, tFrameCounter);
        task_set(taskId, tFrameCounter, task_get(taskId, tFrameCounter) + 1);
        t1
    }) >= task_get(taskId, 3)
    {
        task_set(taskId, tFrameCounter, 0);
        let initialPanning: i16 = task_get(taskId, tInitialPan);
        let targetPanning: i16 = task_get(taskId, tTargetPan);
        let currentPan: i16 = task_get(taskId, tCurrentPan);
        let incrementPan: i16 = task_get(taskId, tIncrementPan);
        let mut pan: i16 = currentPan + incrementPan;
        task_set(taskId, tCurrentPan, pan);
        if incrementPan == 0 {
            destroyTask = TRUE as u32;
        } else if initialPanning < targetPanning {
            if pan >= targetPanning {
                destroyTask = TRUE as u32;
            }
        } else {
            if pan <= targetPanning {
                destroyTask = TRUE as u32;
            }
        }
        if destroyTask != 0 {
            pan = targetPanning;
            DestroyTask(taskId);
            gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() - 1);
        }
        SE12PanpotControl(pan as i8);
    }
}
pub(crate) unsafe fn Cmd_panse_adjustnone() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let songId: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    let currentPan: i8 = *sBattleAnimScriptPtr.at(2) as i8;
    let targetPan: i8 = *sBattleAnimScriptPtr.at(3) as i8;
    let incrementPan: i8 = *sBattleAnimScriptPtr.at(4) as i8;
    let framesToWait: u8 = *sBattleAnimScriptPtr.at(5);
    let taskId: u8 = CreateTask(Some(Task_PanFromInitialToTarget), 1);
    task_set(taskId, tInitialPan, currentPan as i16);
    task_set(taskId, tTargetPan, targetPan as i16);
    task_set(taskId, tIncrementPan, incrementPan as i16);
    task_set(taskId, 3, framesToWait as i16);
    task_set(taskId, tCurrentPan, currentPan as i16);
    PlaySE12WithPanning(songId, currentPan);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() + 1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(6);
}
pub(crate) unsafe fn Cmd_panse_adjustall() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let songId: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    let currentPanArg: i8 = *sBattleAnimScriptPtr.at(2) as i8;
    let targetPanArg: i8 = *sBattleAnimScriptPtr.at(3) as i8;
    let incrementPanArg: i8 = *sBattleAnimScriptPtr.at(4) as i8;
    let framesToWait: u8 = *sBattleAnimScriptPtr.at(5);
    let currentPan: i8 = BattleAnimAdjustPanning2(currentPanArg);
    let targetPan: i8 = BattleAnimAdjustPanning2(targetPanArg);
    let incrementPan: i8 = BattleAnimAdjustPanning2(incrementPanArg);
    let taskId: u8 = CreateTask(Some(Task_PanFromInitialToTarget), 1);
    task_set(taskId, tInitialPan, currentPan as i16);
    task_set(taskId, tTargetPan, targetPan as i16);
    task_set(taskId, tIncrementPan, incrementPan as i16);
    task_set(taskId, 3, framesToWait as i16);
    task_set(taskId, tCurrentPan, currentPan as i16);
    PlaySE12WithPanning(songId, currentPan);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() + 1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(6);
}
pub(crate) unsafe fn Cmd_loopsewithpan() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let songId: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    let panningArg: i8 = *sBattleAnimScriptPtr.at(2) as i8;
    let framesToWait: u8 = *sBattleAnimScriptPtr.at(3);
    let numberOfPlays: u8 = *sBattleAnimScriptPtr.at(4);
    let panning: i8 = BattleAnimAdjustPanning(panningArg);
    let taskId: u8 = CreateTask(Some(Task_LoopAndPlaySE), 1);
    task_set(taskId, tSongId, songId as i16);
    task_set(taskId, tPanning, panning as i16);
    task_set(taskId, 2, framesToWait as i16);
    task_set(taskId, tNumberOfPlays, numberOfPlays as i16);
    task_set(taskId, tFrameCounter, framesToWait as i16);
    task_func(taskId).unwrap_unchecked()(taskId);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() + 1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(5);
}
pub(crate) unsafe fn Task_LoopAndPlaySE(taskId: u8) {
    if ({
        let t1 = task_get(taskId, tFrameCounter);
        task_set(taskId, tFrameCounter, task_get(taskId, tFrameCounter) + 1);
        t1
    }) >= task_get(taskId, 2)
    {
        task_set(taskId, tFrameCounter, 0);
        let songId: u16 = task_get(taskId, tSongId) as u16;
        let panning: i8 = task_get(taskId, tPanning) as i8;
        let numberOfPlays: u8 = ({
            task_set(taskId, tNumberOfPlays, task_get(taskId, tNumberOfPlays) - 1);
            task_get(taskId, tNumberOfPlays)
        }) as u8;
        PlaySE12WithPanning(songId, panning);
        if numberOfPlays == 0 {
            DestroyTask(taskId);
            gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() - 1);
        }
    }
}
pub(crate) unsafe fn Cmd_waitplaysewithpan() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let songId: u16 = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    let panningArg: i8 = *sBattleAnimScriptPtr.at(2) as i8;
    let framesToWait: u8 = *sBattleAnimScriptPtr.at(3);
    let panning: i8 = BattleAnimAdjustPanning(panningArg);
    let taskId: u8 = CreateTask(Some(Task_WaitAndPlaySE), 1);
    task_set(taskId, tSongId, songId as i16);
    task_set(taskId, tPanning, panning as i16);
    task_set(taskId, 2, framesToWait as i16);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() + 1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
}
pub(crate) unsafe fn Task_WaitAndPlaySE(taskId: u8) {
    if ({
        let t1 = task_get(taskId, 2);
        task_set(taskId, 2, task_get(taskId, 2) - 1);
        t1
    }) <= 0
    {
        PlaySE12WithPanning(
            task_get(taskId, tSongId) as u16,
            task_get(taskId, tPanning) as i8,
        );
        DestroyTask(taskId);
        gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() - 1);
    }
}
pub(crate) unsafe fn Cmd_createsoundtask() {
    let mut func: Option<unsafe fn(u8)> = None;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    func = core::mem::transmute::<usize, Option<unsafe fn(u8)>>(
        (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize,
    );
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    let numArgs: u8 = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    for i in 0..(numArgs as i32) {
        gBattleAnimArgs[i] =
            *sBattleAnimScriptPtr as i16 | (*sBattleAnimScriptPtr.at(1) as i16) << 8;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    }
    let taskId: u8 = CreateTask(func, 1);
    func.unwrap_unchecked()(taskId);
    gAnimSoundTaskCount.set(gAnimSoundTaskCount.get() + 1);
}
pub(crate) unsafe fn Cmd_waitsound() {
    if gAnimSoundTaskCount.get() != 0 {
        sSoundAnimFramesToWait.set(0);
        sAnimFramesToWait.set(1);
    } else if IsSEPlaying() != 0 {
        if ({
            sSoundAnimFramesToWait.set(sSoundAnimFramesToWait.get() + 1);
            sSoundAnimFramesToWait.get()
        }) > 90
        {
            m4aMPlayStop(&raw mut gMPlayInfo_SE1);
            m4aMPlayStop(&raw mut gMPlayInfo_SE2);
            sSoundAnimFramesToWait.set(0);
        } else {
            sAnimFramesToWait.set(1);
        }
    } else {
        sSoundAnimFramesToWait.set(0);
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait.set(0);
    }
}
pub(crate) unsafe fn Cmd_jumpargeq() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    let argId: u8 = *sBattleAnimScriptPtr;
    let valueToCheck: i16 =
        *sBattleAnimScriptPtr.at(1) as i16 | (*sBattleAnimScriptPtr.at(1).at(1) as i16) << 8;
    if valueToCheck == gBattleAnimArgs[argId] {
        sBattleAnimScriptPtr = (*sBattleAnimScriptPtr.at(3) as i32
            + ((*sBattleAnimScriptPtr.at(3).at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(3).at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3).at(3) as i32) << 24))
            as usize as *mut c_void as *mut u8;
    } else {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(7);
    }
}
pub(crate) unsafe fn Cmd_jumpifcontest() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if IsContest() != 0 {
        sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
            as *mut c_void as *mut u8;
    } else {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    }
}
pub(crate) unsafe fn Cmd_splitbgprio() {
    let mut battler: u8 = 0;
    let wantedBattler: u8 = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if wantedBattler != ANIM_ATTACKER {
        battler = gBattleAnimTarget;
    } else {
        battler = gBattleAnimAttacker;
    }
    let battlerPosition: u8 = GetBattlerPosition(battler);
    if IsContest() == 0
        && (battlerPosition == B_POSITION_PLAYER_LEFT
            || battlerPosition == B_POSITION_OPPONENT_RIGHT)
    {
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
    }
}
pub(crate) unsafe fn Cmd_splitbgprio_all() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
    }
}
pub(crate) unsafe fn Cmd_splitbgprio_foes() {
    let mut battlerPosition: u8 = 0;
    let mut battler: u8 = 0;
    let wantedBattler: u8 = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if GetBattlerSide(gBattleAnimAttacker) != GetBattlerSide(gBattleAnimTarget) {
        if wantedBattler != ANIM_ATTACKER {
            battler = gBattleAnimTarget;
        } else {
            battler = gBattleAnimAttacker;
        }
        battlerPosition = GetBattlerPosition(battler);
        if IsContest() == 0
            && (battlerPosition == B_POSITION_PLAYER_LEFT
                || battlerPosition == B_POSITION_OPPONENT_RIGHT)
        {
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
        }
    }
}
pub(crate) unsafe fn Cmd_invisible() {
    let spriteId: u8 = GetAnimBattlerSpriteId(*sBattleAnimScriptPtr.at(1));
    if spriteId != SPRITE_NONE {
        gSprites[spriteId].set_invisible(TRUE as u16);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
}
pub(crate) unsafe fn Cmd_visible() {
    let spriteId: u8 = GetAnimBattlerSpriteId(*sBattleAnimScriptPtr.at(1));
    if spriteId != SPRITE_NONE {
        gSprites[spriteId].set_invisible(FALSE as u16);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
}
pub(crate) unsafe fn Cmd_teamattack_moveback() {
    let mut priorityRank: u8 = 0;
    let mut spriteId: u8 = 0;
    let wantedBattler: u8 = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if IsContest() == 0
        && IsDoubleBattle() != 0
        && GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
    {
        if wantedBattler == ANIM_ATTACKER {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker);
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
        } else {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
            spriteId = GetAnimBattlerSpriteId(ANIM_TARGET);
        }
        if spriteId != SPRITE_NONE {
            gSprites[spriteId].set_invisible(FALSE as u16);
            if priorityRank == 2 {
                gSprites[spriteId].oam.set_priority(3);
            }
            if priorityRank == 1 {
                ResetBattleAnimBg(FALSE);
            } else {
                ResetBattleAnimBg(TRUE);
            }
        }
    }
}
pub(crate) unsafe fn Cmd_teamattack_movefwd() {
    let mut priorityRank: u8 = 0;
    let mut spriteId: u8 = 0;
    let wantedBattler: u8 = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if IsContest() == 0
        && IsDoubleBattle() != 0
        && GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
    {
        if wantedBattler == ANIM_ATTACKER {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker);
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
        } else {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
            spriteId = GetAnimBattlerSpriteId(ANIM_TARGET);
        }
        if spriteId != SPRITE_NONE && priorityRank == 2 {
            gSprites[spriteId].oam.set_priority(2);
        }
    }
}
pub(crate) unsafe fn Cmd_stopsound() {
    m4aMPlayStop(&raw mut gMPlayInfo_SE1);
    m4aMPlayStop(&raw mut gMPlayInfo_SE2);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
