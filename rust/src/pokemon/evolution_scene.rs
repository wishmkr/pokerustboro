//! Translated from `src/evolution_scene.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::battle_bg::{InitBattleBgsVideo, LoadBattleTextboxAndBackground};
use crate::battle_gfx_sfx_util::{AllocateMonSpritesGfx, FreeMonSpritesGfx};
use crate::battle_main::{
    GetBattleBgTemplateData, SpriteCallbackDummy_2, gBattle_BG0_X, gBattle_BG0_Y, gBattle_BG1_X,
    gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattle_BG3_X, gBattle_BG3_Y, gBattleEnvironment,
    gMonSpritesGfxPtr, gMoveToLearn,
};
use crate::battle_main::{
    gBattleCommunication, gBattleTextBuff1, gBattleTextBuff2, gDisplayedStringBattle,
};
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString,
};
use crate::battle_script_commands::{
    BattleCreateYesNoCursorAt, BattleDestroyYesNoCursorAt, BufferMoveToLearnIntoBattleTextBuff2,
    HandleBattleWindow,
};
use crate::bg::CopyToBgTilemapBuffer;
use crate::bg::{CopyBgTilemapBufferToVram, FillBgTilemapBufferRect, SetBgAttribute, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::evolution_graphics::{
    CycleEvolutionMonSprite, EvolutionSparkles_ArcDown, EvolutionSparkles_CircleInward,
    EvolutionSparkles_SpiralUpward, EvolutionSparkles_SprayAndFlash,
    EvolutionSparkles_SprayAndFlash_Trade, LoadEvoSparkleSpriteAndPal,
};
use crate::gpu_regs::SetGpuReg;
use crate::link::gWirelessCommType;
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, DestroyWirelessStatusIndicatorSprite,
    LoadWirelessStatusIndicatorSpriteGfx,
};
use crate::m4a::{m4aMPlayAllStop, m4aSongNumStop};
use crate::menu::{
    CreateYesNoMenu, DecompressAndLoadBgGfxUsingHeap, Menu_ProcessInputNoWrapClearOnChoose,
};
use crate::overworld::{IncrementGameStat, Overworld_PlaySpecialMapMusic};
use crate::palette::gPlttBufferUnfaded;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, FillPalette, LoadCompressedPalette, LoadPalette,
    ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon::{
    CalculateMonStats, CalculatePlayerPartyCount, CopyMon, DoMonFrontSpriteAnimation,
    EvolutionRenameMon, GetMonData2, GetMonData3, GetMonSpritePalStructFromOtIdPersonality,
    IsHMMove2, MonTryLearningNewMove, RemoveMonPPBonus, SetMonData, SetMonMoveSlot,
    SetMultiuseSpriteTemplateToPokemon, SpeciesToNationalPokedexNum, gMultiuseSpriteTemplate,
    gPlayerParty, gPlayerPartyCount,
};
use crate::pokemon_summary_screen::{GetMoveSlotToReplace, ShowSelectMovePokemonSummaryScreen};
use crate::scanline_effect::{ScanlineEffect_InitHBlankDmaTransfer, ScanlineEffect_Stop};
use crate::sound::{
    IsCryFinished, IsFanfareTaskInactive, IsSEPlaying, PlayBGM, PlayCry_Normal, PlayFanfare,
    PlayNewMapMusic, PlaySE, StopMapMusic,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData, gAffineAnimsDisabled, gReservedSpritePaletteCount,
};
use crate::string_util::{StringCopy, StringCopy_Nickname, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_data_ptr, task_get, task_set};
use crate::text::{IsTextPrinterActive, RunTextPrinters};
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trade::{
    DrawTextOnTradeWindow, InitTradeSequenceBgGpuRegs, LinkTradeDrawWindow, LoadTradeAnimGfx,
};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::FreeAllWindowBuffers;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DecompressPicFromTable_2` with this module's view of its types.
#[inline]
unsafe fn DecompressPicFromTable_2(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32) {
    unsafe {
        crate::decompress::DecompressPicFromTable_2(a0 as _, a1 as _, a2);
    }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tPreEvoSpecies: usize = 1;
const tIsLink: usize = 2;
const tPostEvoSpecies: usize = 2;
const tBits: usize = 3;
const tCanStop: usize = 3;
const tLearnsFirstMove: usize = 4;
const tLearnMoveState: usize = 6;
const tPaused: usize = 6;
const tLearnMoveYesState: usize = 7;
const tLearnMoveNoState: usize = 8;
const tEvoWasStopped: usize = 9;
const tPartyId: usize = 10;
// Data tables (translate with cdata.py): sUnusedPal1 sBgAnim_Gfx sBgAnim_Inner_Tilemap sBgAnim_Outer_Tilemap sBgAnim_Intro_Pal sUnusedPal2 sUnusedPal3 sUnusedPal4 sBgAnim_Pal sText_ShedinjaJapaneseName sBgAnim_PaletteControl sBgAnim_PalIndexes

/// `struct EvoInfo`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct EvoInfo {
    pub preEvoSpriteId: u8,
    pub postEvoSpriteId: u8,
    pub evoTaskId: u8,
    pub delayTimer: u8,
    pub savedPalette: CArray<u16, 48>,
}

unsafe impl Sync for EvoInfo {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<EvoInfo>() == 100);
    assert!(offset_of!(EvoInfo, preEvoSpriteId) == 0);
    assert!(offset_of!(EvoInfo, postEvoSpriteId) == 1);
    assert!(offset_of!(EvoInfo, evoTaskId) == 2);
    assert!(offset_of!(EvoInfo, delayTimer) == 3);
    assert!(offset_of!(EvoInfo, savedPalette) == 4);
};

const EVOSTATE_CANCEL: i16 = 17;
const EVOSTATE_CANCEL_MON_ANIM: i16 = 18;
const EVOSTATE_CANCEL_MSG: i16 = 19;
const EVOSTATE_CYCLE_MON_SPRITE: i16 = 7;
const EVOSTATE_END: i16 = 16;
const EVOSTATE_EVO_MON_ANIM: i16 = 13;
const EVOSTATE_EVO_SOUND: i16 = 11;
const EVOSTATE_FADE_IN: i16 = 0;
const EVOSTATE_INTRO_MON_ANIM: i16 = 2;
const EVOSTATE_INTRO_MSG: i16 = 1;
const EVOSTATE_INTRO_SOUND: i16 = 3;
const EVOSTATE_LEARNED_MOVE: i16 = 20;
const EVOSTATE_REPLACE_MOVE: i16 = 22;
const EVOSTATE_RESTORE_SCREEN: i16 = 12;
const EVOSTATE_SET_MON_EVOLVED: i16 = 14;
const EVOSTATE_SPARKLE_ARC: i16 = 6;
const EVOSTATE_SPARKLE_CIRCLE: i16 = 9;
const EVOSTATE_SPARKLE_SPRAY: i16 = 10;
const EVOSTATE_START_BG_AND_SPARKLE_SPIRAL: i16 = 5;
const EVOSTATE_START_MUSIC: i16 = 4;
const EVOSTATE_TRY_LEARN_ANOTHER_MOVE: i16 = 21;
const EVOSTATE_TRY_LEARN_MOVE: i16 = 15;
const EVOSTATE_WAIT_CYCLE_MON_SPRITE: i16 = 8;
const MVSTATE_ASK_CANCEL: i16 = 10;
const MVSTATE_CANCEL: i16 = 11;
const MVSTATE_FORGET_MSG_1: i16 = 7;
const MVSTATE_FORGET_MSG_2: i16 = 8;
const MVSTATE_HANDLE_MOVE_SELECT: i16 = 6;
const MVSTATE_HANDLE_YES_NO: i16 = 4;
const MVSTATE_INTRO_MSG_1: i16 = 0;
const MVSTATE_INTRO_MSG_2: i16 = 1;
const MVSTATE_INTRO_MSG_3: i16 = 2;
const MVSTATE_LEARNED_MOVE: i16 = 9;
const MVSTATE_PRINT_YES_NO: i16 = 3;
const MVSTATE_RETRY_AFTER_HM: i16 = 12;
const MVSTATE_SHOW_MOVE_SELECT: i16 = 5;
const TASK_BIT_CAN_STOP: i32 = 1;
const TASK_BIT_LEARN_MOVE: i32 = 128;
const T_EVOSTATE_CANCEL: i16 = 15;
const T_EVOSTATE_CANCEL_MON_ANIM: i16 = 16;
const T_EVOSTATE_CANCEL_MSG: i16 = 17;
const T_EVOSTATE_CYCLE_MON_SPRITE: i16 = 6;
const T_EVOSTATE_END: i16 = 14;
const T_EVOSTATE_EVO_MON_ANIM: i16 = 11;
const T_EVOSTATE_EVO_SOUND: i16 = 10;
const T_EVOSTATE_INTRO_CRY: i16 = 1;
const T_EVOSTATE_INTRO_MSG: i16 = 0;
const T_EVOSTATE_INTRO_SOUND: i16 = 2;
const T_EVOSTATE_LEARNED_MOVE: i16 = 18;
const T_EVOSTATE_REPLACE_MOVE: i16 = 20;
const T_EVOSTATE_SET_MON_EVOLVED: i16 = 12;
const T_EVOSTATE_SPARKLE_ARC: i16 = 5;
const T_EVOSTATE_SPARKLE_CIRCLE: i16 = 8;
const T_EVOSTATE_SPARKLE_SPRAY: i16 = 9;
const T_EVOSTATE_START_BG_AND_SPARKLE_SPIRAL: i16 = 4;
const T_EVOSTATE_START_MUSIC: i16 = 3;
const T_EVOSTATE_TRY_LEARN_ANOTHER_MOVE: i16 = 19;
const T_EVOSTATE_TRY_LEARN_MOVE: i16 = 13;
const T_EVOSTATE_WAIT_CYCLE_MON_SPRITE: i16 = 7;
const T_MVSTATE_ASK_CANCEL: i16 = 9;
const T_MVSTATE_CANCEL: i16 = 10;
const T_MVSTATE_FORGET_MSG: i16 = 7;
const T_MVSTATE_HANDLE_MOVE_SELECT: i16 = 6;
const T_MVSTATE_HANDLE_YES_NO: i16 = 4;
const T_MVSTATE_INTRO_MSG_1: i16 = 0;
const T_MVSTATE_INTRO_MSG_2: i16 = 1;
const T_MVSTATE_INTRO_MSG_3: i16 = 2;
const T_MVSTATE_LEARNED_MOVE: i16 = 8;
const T_MVSTATE_PRINT_YES_NO: i16 = 3;
const T_MVSTATE_RETRY_AFTER_HM: i16 = 11;
const T_MVSTATE_SHOW_MOVE_SELECT: i16 = 5;

static sBgAnim_Gfx: Table<CArray<u32, 446>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Gfx).cast());
static sBgAnim_Inner_Tilemap: Table<CArray<u32, 313>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Inner_Tilemap).cast());
static sBgAnim_Intro_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Intro_Pal).cast());
static sBgAnim_Outer_Tilemap: Table<CArray<u32, 309>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Outer_Tilemap).cast());
static sBgAnim_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Pal).cast());
static sBgAnim_PalIndexes: Table<CArray<CArray<u8, 16>, 50>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_PalIndexes).cast());
static sBgAnim_PaletteControl: Table<CArray<CArray<u8, 4>, 4>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_PaletteControl).cast());
static sText_ShedinjaJapaneseName: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::evolution_scene::sText_ShedinjaJapaneseName).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEvoStructPtr: *mut EvoInfo = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBgAnimPal: *mut u16 = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gCB2_AfterEvolution: Option<unsafe fn()> = None;

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn CB2_BeginEvolutionScene() {
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe fn Task_BeginEvolutionScene(taskId: u8) {
    let mut mon: *mut Pokemon = null_mut();
    match task_get(taskId, tState) {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 if gPaletteFade.active() == 0 => {
            mon = &raw mut gPlayerParty[task_get(taskId, tPartyId)];
            let postEvoSpecies: u16 = task_get(taskId, tPostEvoSpecies) as u16;
            let canStopEvo: u8 = task_get(taskId, tCanStop) as u8;
            let partyId: u8 = task_get(taskId, tPartyId) as u8;
            DestroyTask(taskId);
            EvolutionScene(mon, postEvoSpecies, canStopEvo, partyId);
        }
        _ => {}
    }
}
pub unsafe fn BeginEvolutionScene(
    mon: *mut Pokemon,
    postEvoSpecies: u16,
    canStopEvo: u8,
    partyId: u8,
) {
    let taskId: u8 = CreateTask(Some(Task_BeginEvolutionScene), 0);
    task_set(taskId, tState, 0);
    task_set(taskId, tPostEvoSpecies, postEvoSpecies as i16);
    task_set(taskId, tCanStop, canStopEvo as i16);
    task_set(taskId, tPartyId, partyId as i16);
    SetMainCallback2(Some(CB2_BeginEvolutionScene));
}
pub unsafe fn EvolutionScene(mon: *mut Pokemon, postEvoSpecies: u16, canStopEvo: u8, partyId: u8) {
    let mut name: CArray<u8, 20> = zeroed();
    let mut id: u8 = 0;
    SetHBlankCallback(None);
    SetVBlankCallback(None);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                VRAM as usize as *mut c_void,
                0x5006000,
            );
        }
    }
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    ResetPaletteFade();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 256;
    gBattle_BG3_Y = 0;
    gBattleEnvironment = BATTLE_ENVIRONMENT_PLAIN;
    InitBattleBgsVideo();
    LoadBattleTextboxAndBackground();
    ResetSpriteData();
    ScanlineEffect_Stop();
    ResetTasks();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = 4;
    sEvoStructPtr = AllocZeroed(100) as *mut EvoInfo;
    AllocateMonSpritesGfx();
    GetMonData3(mon, MON_DATA_NICKNAME, name.as_mut_ptr());
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), name.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [postEvoSpecies]
            .as_ptr()
            .cast_mut(),
    );
    let currSpecies: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    let trainerId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
    let personality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
    DecompressPicFromTable_2(
        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[currSpecies])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[1],
        currSpecies as i32,
    );
    let mut pokePal: *mut CompressedSpritePalette =
        GetMonSpritePalStructFromOtIdPersonality(currSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 272, 32);
    SetMultiuseSpriteTemplateToPokemon(currSpecies, B_POSITION_OPPONENT_LEFT);
    gMultiuseSpriteTemplate.affineAnims =
        (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
            .cast::<CArray<*mut AffineAnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
    (*sEvoStructPtr).preEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(1);
    gSprites[id].set_invisible(TRUE as u16);
    DecompressPicFromTable_2(
        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[postEvoSpecies])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[3],
        postEvoSpecies as i32,
    );
    pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 288, 32);
    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_RIGHT);
    gMultiuseSpriteTemplate.affineAnims =
        (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
            .cast::<CArray<*mut AffineAnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
    (*sEvoStructPtr).postEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(2);
    gSprites[id].set_invisible(TRUE as u16);
    LoadEvoSparkleSpriteAndPal();
    (*sEvoStructPtr).evoTaskId = {
        id = CreateTask(Some(Task_EvolutionScene), 0);
        id
    };
    task_set(id, tState, 0);
    task_set(id, tPreEvoSpecies, currSpecies as i16);
    task_set(id, tPostEvoSpecies, postEvoSpecies as i16);
    task_set(id, tCanStop, canStopEvo as i16);
    task_set(id, tLearnsFirstMove, TRUE as i16);
    task_set(id, tEvoWasStopped, FALSE as i16);
    task_set(id, tPartyId, partyId as i16);
    memcpy(
        &raw mut (*sEvoStructPtr).savedPalette as *mut u8,
        &raw mut gPlttBufferUnfaded[32] as *mut u8,
        96,
    );
    SetGpuReg(REG_OFFSET_DISPCNT, 8000);
    SetHBlankCallback(Some(EvoDummyFunc));
    SetVBlankCallback(Some(VBlankCB_EvolutionScene));
    m4aMPlayAllStop();
    SetMainCallback2(Some(CB2_EvolutionSceneUpdate));
}
pub(crate) unsafe fn CB2_EvolutionSceneLoadGraphics() {
    let mut id: u8 = 0;
    let mon: *mut Pokemon = &raw mut gPlayerParty[task_get((*sEvoStructPtr).evoTaskId, tPartyId)];
    let postEvoSpecies: u16 = task_get((*sEvoStructPtr).evoTaskId, tPostEvoSpecies) as u16;
    let trainerId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
    let personality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
    SetHBlankCallback(None);
    SetVBlankCallback(None);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                VRAM as usize as *mut c_void,
                0x5006000,
            );
        }
    }
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    ResetPaletteFade();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 256;
    gBattle_BG3_Y = 0;
    gBattleEnvironment = BATTLE_ENVIRONMENT_PLAIN;
    InitBattleBgsVideo();
    LoadBattleTextboxAndBackground();
    ResetSpriteData();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = 4;
    DecompressPicFromTable_2(
        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[postEvoSpecies])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[3],
        postEvoSpecies as i32,
    );
    let pokePal: *mut CompressedSpritePalette =
        GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 288, 32);
    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_RIGHT);
    gMultiuseSpriteTemplate.affineAnims =
        (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
            .cast::<CArray<*mut AffineAnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
    (*sEvoStructPtr).postEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 8000);
    SetHBlankCallback(Some(EvoDummyFunc));
    SetVBlankCallback(Some(VBlankCB_EvolutionScene));
    SetMainCallback2(Some(CB2_EvolutionSceneUpdate));
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
pub(crate) unsafe fn CB2_TradeEvolutionSceneLoadGraphics() {
    let mon: *mut Pokemon = &raw mut gPlayerParty[task_get((*sEvoStructPtr).evoTaskId, tPartyId)];
    let postEvoSpecies: u16 = task_get((*sEvoStructPtr).evoTaskId, tPostEvoSpecies) as u16;
    match gMain.state {
        0 => {
            SetGpuReg(0x0, 0);
            SetHBlankCallback(None);
            SetVBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            gReservedSpritePaletteCount = 4;
            gBattle_BG0_X = 0;
            gBattle_BG0_Y = 0;
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            gBattle_BG2_X = 0;
            gBattle_BG2_Y = 0;
            gBattle_BG3_X = 256;
            gBattle_BG3_Y = 0;
            gMain.state += 1;
        }
        1 => {
            ResetPaletteFade();
            SetHBlankCallback(Some(EvoDummyFunc));
            SetVBlankCallback(Some(VBlankCB_TradeEvolutionScene));
            gMain.state += 1;
        }
        2 => {
            LoadTradeAnimGfx();
            gMain.state += 1;
        }
        3 => {
            FillBgTilemapBufferRect(1, 0, 0, 0, 0x20, 0x20, 0x11);
            CopyBgTilemapBufferToVram(1);
            gMain.state += 1;
        }
        4 => {
            let trainerId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
            let personality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
            DecompressPicFromTable_2(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[postEvoSpecies])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[3],
                postEvoSpecies as i32,
            );
            let pokePal: *mut CompressedSpritePalette =
                GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
            LoadCompressedPalette((*pokePal).data, 288, 32);
            gMain.state += 1;
        }
        5 => {
            let mut id: u8 = 0;
            SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_LEFT);
            gMultiuseSpriteTemplate.affineAnims =
                (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
                    .cast::<CArray<*mut AffineAnimCmd, 0>>())
                .as_ptr()
                .cast_mut();
            (*sEvoStructPtr).postEvoSpriteId = {
                id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
                id
            };
            gSprites[id].callback = Some(SpriteCallbackDummy_2);
            gSprites[id].oam.set_paletteNum(2);
            gMain.state += 1;
            LinkTradeDrawWindow();
        }
        6 => {
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            BlendPalettes(PALETTES_ALL, 0x10, 0);
            gMain.state += 1;
        }
        7 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            InitTradeSequenceBgGpuRegs();
            ShowBg(0);
            ShowBg(1);
            SetMainCallback2(Some(CB2_TradeEvolutionSceneUpdate));
            SetGpuReg(REG_OFFSET_DISPCNT, 4928);
        }
        _ => {}
    }
}
pub unsafe fn TradeEvolutionScene(
    mon: *mut Pokemon,
    postEvoSpecies: u16,
    preEvoSpriteId: u8,
    partyId: u8,
) {
    let mut name: CArray<u8, 20> = zeroed();
    let mut id: u8 = 0;
    GetMonData3(mon, MON_DATA_NICKNAME, name.as_mut_ptr());
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), name.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [postEvoSpecies]
            .as_ptr()
            .cast_mut(),
    );
    gAffineAnimsDisabled = TRUE;
    let currSpecies: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    let personality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
    let trainerId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
    sEvoStructPtr = AllocZeroed(100) as *mut EvoInfo;
    (*sEvoStructPtr).preEvoSpriteId = preEvoSpriteId;
    DecompressPicFromTable_2(
        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[postEvoSpecies])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[1],
        postEvoSpecies as i32,
    );
    let pokePal: *mut CompressedSpritePalette =
        GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 288, 32);
    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_LEFT);
    gMultiuseSpriteTemplate.affineAnims =
        (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
            .cast::<CArray<*mut AffineAnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
    (*sEvoStructPtr).postEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(2);
    gSprites[id].set_invisible(TRUE as u16);
    LoadEvoSparkleSpriteAndPal();
    (*sEvoStructPtr).evoTaskId = {
        id = CreateTask(Some(Task_TradeEvolutionScene), 0);
        id
    };
    task_set(id, tState, 0);
    task_set(id, tPreEvoSpecies, currSpecies as i16);
    task_set(id, tPostEvoSpecies, postEvoSpecies as i16);
    task_set(id, tLearnsFirstMove, TRUE as i16);
    task_set(id, tEvoWasStopped, FALSE as i16);
    task_set(id, tPartyId, partyId as i16);
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 256;
    gBattle_BG3_Y = 0;
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_useAlternateDownArrow(TRUE);
    SetVBlankCallback(Some(VBlankCB_TradeEvolutionScene));
    SetMainCallback2(Some(CB2_TradeEvolutionSceneUpdate));
}
pub(crate) unsafe fn CB2_EvolutionSceneUpdate() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe fn CB2_TradeEvolutionSceneUpdate() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
unsafe fn CreateShedinja(preEvoSpecies: u16, mon: *mut Pokemon) {
    let mut data: u32 = 0;
    if (*(&raw const crate::data::pokemon::gEvolutionTable)
        .cast::<CArray<CArray<Evolution, 5>, 0>>()
        .cast_mut())[preEvoSpecies][0]
        .method
        == EVO_LEVEL_NINJASK
        && gPlayerPartyCount < PARTY_SIZE as u8
    {
        let shedinja: *mut Pokemon = &raw mut gPlayerParty[gPlayerPartyCount];
        CopyMon(
            &raw mut gPlayerParty[gPlayerPartyCount] as *mut c_void,
            mon as *mut c_void,
            100,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_SPECIES,
            &raw mut (*(&raw const crate::data::pokemon::gEvolutionTable)
                .cast::<CArray<CArray<Evolution, 5>, 0>>()
                .cast_mut())[preEvoSpecies][1]
                .targetSpecies as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_NICKNAME,
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())
                [(*(&raw const crate::data::pokemon::gEvolutionTable)
                    .cast::<CArray<CArray<Evolution, 5>, 0>>()
                    .cast_mut())[preEvoSpecies][1]
                    .targetSpecies]
                .as_ptr()
                .cast_mut() as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_HELD_ITEM,
            &raw mut data as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_MARKINGS,
            &raw mut data as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_ENCRYPT_SEPARATOR,
            &raw mut data as *mut c_void,
        );
        for i in MON_DATA_COOL_RIBBON..55 {
            SetMonData(
                &raw mut gPlayerParty[gPlayerPartyCount],
                i,
                &raw mut data as *mut c_void,
            );
        }
        let mut i: i32 = MON_DATA_CHAMPION_RIBBON;
        while i <= MON_DATA_UNUSED_RIBBONS {
            SetMonData(
                &raw mut gPlayerParty[gPlayerPartyCount],
                i,
                &raw mut data as *mut c_void,
            );
            i += 1;
        }
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_STATUS,
            &raw mut data as *mut c_void,
        );
        data = MAIL_NONE;
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_MAIL,
            &raw mut data as *mut c_void,
        );
        CalculateMonStats(&raw mut gPlayerParty[gPlayerPartyCount]);
        CalculatePlayerPartyCount();
        GetSetPokedexFlag(
            SpeciesToNationalPokedexNum(
                (*(&raw const crate::data::pokemon::gEvolutionTable)
                    .cast::<CArray<CArray<Evolution, 5>, 0>>()
                    .cast_mut())[preEvoSpecies][1]
                    .targetSpecies,
            ),
            FLAG_SET_SEEN,
        );
        GetSetPokedexFlag(
            SpeciesToNationalPokedexNum(
                (*(&raw const crate::data::pokemon::gEvolutionTable)
                    .cast::<CArray<CArray<Evolution, 5>, 0>>()
                    .cast_mut())[preEvoSpecies][1]
                    .targetSpecies,
            ),
            FLAG_SET_CAUGHT,
        );
        if GetMonData2(shedinja, MON_DATA_SPECIES) == SPECIES_SHEDINJA as u32
            && GetMonData2(shedinja, MON_DATA_LANGUAGE) == LANGUAGE_JAPANESE as u32
            && GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_NINJASK
        {
            SetMonData(
                shedinja,
                MON_DATA_NICKNAME,
                sText_ShedinjaJapaneseName.as_ptr().cast_mut() as *mut c_void,
            );
        }
    }
}
pub(crate) unsafe fn Task_EvolutionScene(taskId: u8) {
    let mut var: u32 = 0;
    let mon: *mut Pokemon = &raw mut gPlayerParty[task_get(taskId, tPartyId)];
    if gMain.heldKeys == B_BUTTON as u16
        && task_get(taskId, tState) == EVOSTATE_WAIT_CYCLE_MON_SPRITE
        && (*gTasks.as_ptr())[gBattleCommunication[2]].isActive != 0
        && task_get(taskId, tBits) as i32 & TASK_BIT_CAN_STOP != 0
    {
        task_set(taskId, tState, EVOSTATE_CANCEL);
        task_set(gBattleCommunication[2], 8, TRUE as i16);
        StopBgAnimation();
        return;
    }
    'l1: {
        match task_get(taskId, tState) {
            EVOSTATE_FADE_IN => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                gSprites[(*sEvoStructPtr).preEvoSpriteId].set_invisible(FALSE as u16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
            }
            EVOSTATE_INTRO_MSG => {
                if gPaletteFade.active() == 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_PkmnIsEvolving)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    BattlePutTextOnWindow(gStringVar4.as_mut_ptr(), B_WIN_MSG);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_INTRO_MON_ANIM => {
                if IsTextPrinterActive(0) == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).preEvoSpriteId,
                        task_get(taskId, tPreEvoSpecies) as u16,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_INTRO_SOUND => {
                if EvoScene_IsMonAnimFinished((*sEvoStructPtr).preEvoSpriteId) != 0 {
                    PlaySE(MUS_EVOLUTION_INTRO);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_START_MUSIC => {
                if IsSEPlaying() == 0 {
                    PlayNewMapMusic(MUS_EVOLUTION);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    BeginNormalPaletteFade(0x1C, 4, 0, 0x10, 0);
                }
            }
            EVOSTATE_START_BG_AND_SPARKLE_SPIRAL => {
                if gPaletteFade.active() == 0 {
                    StartBgAnimation(FALSE);
                    gBattleCommunication[2] = EvolutionSparkles_SpiralUpward(17);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_SPARKLE_ARC => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    (*sEvoStructPtr).delayTimer = 1;
                    gBattleCommunication[2] = EvolutionSparkles_ArcDown();
                }
            }
            EVOSTATE_CYCLE_MON_SPRITE => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] = CycleEvolutionMonSprite(
                        (*sEvoStructPtr).preEvoSpriteId,
                        (*sEvoStructPtr).postEvoSpriteId,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_WAIT_CYCLE_MON_SPRITE => {
                if ({
                    (*sEvoStructPtr).delayTimer -= 1;
                    (*sEvoStructPtr).delayTimer
                }) == 0
                {
                    (*sEvoStructPtr).delayTimer = 3;
                    if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                        task_set(taskId, tState, task_get(taskId, tState) + 1);
                    }
                }
            }
            EVOSTATE_SPARKLE_CIRCLE => {
                gBattleCommunication[2] = EvolutionSparkles_CircleInward();
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            EVOSTATE_SPARKLE_SPRAY => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] =
                        EvolutionSparkles_SprayAndFlash(task_get(taskId, tPostEvoSpecies) as u16);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_EVO_SOUND => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    PlaySE(SE_EXP);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_RESTORE_SCREEN => {
                if IsSEPlaying() != 0 {
                    m4aMPlayAllStop();
                    memcpy(
                        &raw mut gPlttBufferUnfaded[32] as *mut u8,
                        (*sEvoStructPtr).savedPalette.as_mut_ptr() as *mut u8,
                        96,
                    );
                    RestoreBgAfterAnim();
                    BeginNormalPaletteFade(0x1C, 0, 0x10, 0, 0);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_EVO_MON_ANIM => {
                if gPaletteFade.active() == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).postEvoSpriteId,
                        task_get(taskId, tPostEvoSpecies) as u16,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_SET_MON_EVOLVED => {
                if IsCryFinished() != 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_CongratsPkmnEvolved)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    BattlePutTextOnWindow(gStringVar4.as_mut_ptr(), B_WIN_MSG);
                    PlayBGM(MUS_EVOLVED);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    SetMonData(
                        mon,
                        MON_DATA_SPECIES,
                        task_data_ptr(taskId, tPostEvoSpecies) as *mut c_void,
                    );
                    CalculateMonStats(mon);
                    EvolutionRenameMon(
                        mon,
                        task_get(taskId, tPreEvoSpecies) as u16,
                        task_get(taskId, tPostEvoSpecies) as u16,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(task_get(taskId, tPostEvoSpecies) as u16),
                        FLAG_SET_SEEN,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(task_get(taskId, tPostEvoSpecies) as u16),
                        FLAG_SET_CAUGHT,
                    );
                    IncrementGameStat(GAME_STAT_EVOLVED_POKEMON);
                }
            }
            EVOSTATE_TRY_LEARN_MOVE => {
                if IsTextPrinterActive(0) == 0 {
                    var =
                        MonTryLearningNewMove(mon, task_get(taskId, tLearnsFirstMove) as u8) as u32;
                    if var != MOVE_NONE as u32 && task_get(taskId, tEvoWasStopped) == 0 {
                        let mut nickname: CArray<u8, 20> = zeroed();
                        if task_get(taskId, tBits) as i32 & TASK_BIT_LEARN_MOVE == 0 {
                            StopMapMusic();
                            Overworld_PlaySpecialMapMusic();
                        }
                        task_set(
                            taskId,
                            tBits,
                            task_get(taskId, tBits) | (TASK_BIT_LEARN_MOVE as i16),
                        );
                        task_set(taskId, tLearnsFirstMove, FALSE as i16);
                        task_set(taskId, tLearnMoveState, MVSTATE_INTRO_MSG_1);
                        GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
                        StringCopy_Nickname(gBattleTextBuff1.as_mut_ptr(), nickname.as_mut_ptr());
                        if var == MON_HAS_MAX_MOVES as u32 {
                            task_set(taskId, tState, EVOSTATE_REPLACE_MOVE);
                        } else if var == MON_ALREADY_KNOWS_MOVE as u32 {
                            break 'l1;
                        } else {
                            task_set(taskId, tState, EVOSTATE_LEARNED_MOVE);
                        }
                    } else {
                        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                        task_set(taskId, tState, task_get(taskId, tState) + 1);
                    }
                }
            }
            EVOSTATE_END => {
                if gPaletteFade.active() == 0 {
                    if task_get(taskId, tBits) as i32 & TASK_BIT_LEARN_MOVE == 0 {
                        StopMapMusic();
                        Overworld_PlaySpecialMapMusic();
                    }
                    if task_get(taskId, tEvoWasStopped) == 0 {
                        CreateShedinja(task_get(taskId, tPreEvoSpecies) as u16, mon);
                    }
                    DestroyTask(taskId);
                    FreeMonSpritesGfx();
                    Free(sEvoStructPtr as *mut c_void);
                    sEvoStructPtr = null_mut();
                    FreeAllWindowBuffers();
                    SetMainCallback2(gCB2_AfterEvolution);
                }
            }
            EVOSTATE_CANCEL => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    m4aMPlayAllStop();
                    BeginNormalPaletteFade(0x6001C, 0, 0x10, 0, 32767);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_CANCEL_MON_ANIM => {
                if gPaletteFade.active() == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).preEvoSpriteId,
                        task_get(taskId, tPreEvoSpecies) as u16,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_CANCEL_MSG => {
                if EvoScene_IsMonAnimFinished((*sEvoStructPtr).preEvoSpriteId) != 0 {
                    if task_get(taskId, tEvoWasStopped) != 0 {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            (*(&raw const crate::data::battle_message::gText_EllipsisQuestionMark)
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                    } else {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            (*(&raw const crate::data::battle_message::gText_PkmnStoppedEvolving)
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                    }
                    BattlePutTextOnWindow(gStringVar4.as_mut_ptr(), B_WIN_MSG);
                    task_set(taskId, tEvoWasStopped, TRUE as i16);
                    task_set(taskId, tState, EVOSTATE_TRY_LEARN_MOVE);
                }
            }
            EVOSTATE_LEARNED_MOVE => {
                if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                    BufferMoveToLearnIntoBattleTextBuff2();
                    PlayFanfare(MUS_LEVEL_UP);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[3],
                    );
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    task_set(taskId, tLearnsFirstMove, 0x40);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            EVOSTATE_TRY_LEARN_ANOTHER_MOVE => {
                if IsTextPrinterActive(0) == 0
                    && IsSEPlaying() == 0
                    && ({
                        task_set(
                            taskId,
                            tLearnsFirstMove,
                            task_get(taskId, tLearnsFirstMove) - 1,
                        );
                        task_get(taskId, tLearnsFirstMove)
                    }) == 0
                {
                    task_set(taskId, tState, EVOSTATE_TRY_LEARN_MOVE);
                }
            }
            EVOSTATE_REPLACE_MOVE => 'l2: {
                let sw3: i16 = task_get(taskId, tLearnMoveState);
                let mut fall = false;
                if sw3 == MVSTATE_INTRO_MSG_1 {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BufferMoveToLearnIntoBattleTextBuff2();
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[4],
                        );
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_INTRO_MSG_2 {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[5],
                        );
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_INTRO_MSG_3 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[6],
                        );
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        task_set(taskId, tLearnMoveYesState, MVSTATE_SHOW_MOVE_SELECT);
                        task_set(taskId, 8, MVSTATE_ASK_CANCEL);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                }
                if fall || sw3 == MVSTATE_PRINT_YES_NO {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        HandleBattleWindow(24, 8, 29, 13, 0);
                        BattlePutTextOnWindow(
                            (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                            B_WIN_YESNO,
                        );
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                        gBattleCommunication[1] = 0;
                        BattleCreateYesNoCursorAt(0);
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_HANDLE_YES_NO {
                    if gMain.newKeys as i32 & DPAD_UP != 0 && gBattleCommunication[1] != 0 {
                        PlaySE(SE_SELECT);
                        BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                        gBattleCommunication[1] = 0;
                        BattleCreateYesNoCursorAt(0);
                    }
                    if gMain.newKeys as i32 & DPAD_DOWN != 0 && gBattleCommunication[1] == 0 {
                        PlaySE(SE_SELECT);
                        BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                        gBattleCommunication[1] = 1;
                        BattleCreateYesNoCursorAt(1);
                    }
                    if gMain.newKeys as i32 & A_BUTTON != 0 {
                        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                        PlaySE(SE_SELECT);
                        if gBattleCommunication[1] != 0 {
                            task_set(taskId, tLearnMoveState, task_get(taskId, 8));
                        } else {
                            task_set(
                                taskId,
                                tLearnMoveState,
                                task_get(taskId, tLearnMoveYesState),
                            );
                            if task_get(taskId, tLearnMoveState) == MVSTATE_SHOW_MOVE_SELECT {
                                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                            }
                        }
                    }
                    if gMain.newKeys as i32 & B_BUTTON != 0 {
                        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                        PlaySE(SE_SELECT);
                        task_set(taskId, tLearnMoveState, task_get(taskId, 8));
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_SHOW_MOVE_SELECT {
                    if gPaletteFade.active() == 0 {
                        FreeAllWindowBuffers();
                        ShowSelectMovePokemonSummaryScreen(
                            gPlayerParty.as_mut_ptr(),
                            task_get(taskId, tPartyId) as u8,
                            gPlayerPartyCount - 1,
                            Some(CB2_EvolutionSceneLoadGraphics),
                            gMoveToLearn,
                        );
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_HANDLE_MOVE_SELECT {
                    if gPaletteFade.active() == 0
                        && gMain.callback2 == Some(CB2_EvolutionSceneUpdate as unsafe fn())
                    {
                        var = GetMoveSlotToReplace() as u32;
                        if var == MAX_MON_MOVES as u32 {
                            task_set(taskId, tLearnMoveState, MVSTATE_ASK_CANCEL);
                        } else {
                            let r#move: u16 = GetMonData2(mon, var as i32 + MON_DATA_MOVE1) as u16;
                            if IsHMMove2(r#move) != 0 {
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    (*(&raw const crate::data::battle_message::gBattleStringsTable).cast::<CArray<*mut u8, 0>>())[307],
                                );
                                BattlePutTextOnWindow(
                                    gDisplayedStringBattle.as_mut_ptr(),
                                    B_WIN_MSG,
                                );
                                task_set(taskId, tLearnMoveState, MVSTATE_RETRY_AFTER_HM);
                            } else {
                                gBattleTextBuff2[0] = 0xFD;
                                gBattleTextBuff2[1] = 2;
                                gBattleTextBuff2[2] = r#move as u8;
                                gBattleTextBuff2[3] = ((r#move as i32 & 0xFF00) >> 8) as u8;
                                gBattleTextBuff2[4] = 0xFF;
                                RemoveMonPPBonus(mon, var as u8);
                                SetMonMoveSlot(mon, gMoveToLearn, var as u8);
                                task_set(
                                    taskId,
                                    tLearnMoveState,
                                    task_get(taskId, tLearnMoveState) + 1,
                                );
                            }
                        }
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_FORGET_MSG_1 {
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[207],
                    );
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    task_set(
                        taskId,
                        tLearnMoveState,
                        task_get(taskId, tLearnMoveState) + 1,
                    );
                    break 'l2;
                }
                if sw3 == MVSTATE_FORGET_MSG_2 {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[7],
                        );
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_LEARNED_MOVE {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[208],
                        );
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        task_set(taskId, tState, EVOSTATE_LEARNED_MOVE);
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_ASK_CANCEL {
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[8],
                    );
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    task_set(taskId, tLearnMoveYesState, MVSTATE_CANCEL);
                    task_set(taskId, 8, MVSTATE_INTRO_MSG_1);
                    task_set(taskId, tLearnMoveState, MVSTATE_PRINT_YES_NO);
                    break 'l2;
                }
                if sw3 == MVSTATE_CANCEL {
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[9],
                    );
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    task_set(taskId, tState, EVOSTATE_TRY_LEARN_MOVE);
                    break 'l2;
                }
                if sw3 == MVSTATE_RETRY_AFTER_HM {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        task_set(taskId, tLearnMoveState, MVSTATE_SHOW_MOVE_SELECT);
                    }
                    break 'l2;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn Task_TradeEvolutionScene(taskId: u8) {
    let mut var: u32 = 0;
    let mon: *mut Pokemon = &raw mut gPlayerParty[task_get(taskId, tPartyId)];
    'l1: {
        match task_get(taskId, tState) {
            T_EVOSTATE_INTRO_MSG => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::battle_message::gText_PkmnIsEvolving)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 1);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            T_EVOSTATE_INTRO_CRY => {
                if IsTextPrinterActive(0) == 0 {
                    PlayCry_Normal(task_get(taskId, tPreEvoSpecies) as u16, 0);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_INTRO_SOUND => {
                if IsCryFinished() != 0 {
                    m4aSongNumStop(MUS_EVOLUTION);
                    PlaySE(MUS_EVOLUTION_INTRO);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_START_MUSIC => {
                if IsSEPlaying() == 0 {
                    PlayBGM(MUS_EVOLUTION);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    BeginNormalPaletteFade(0x1C, 4, 0, 0x10, 0);
                }
            }
            T_EVOSTATE_START_BG_AND_SPARKLE_SPIRAL => {
                if gPaletteFade.active() == 0 {
                    StartBgAnimation(TRUE);
                    var = gSprites[(*sEvoStructPtr).preEvoSpriteId].oam.paletteNum() as u32 + 16;
                    gBattleCommunication[2] = EvolutionSparkles_SpiralUpward(var as u16);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    SetGpuReg(REG_OFFSET_BG3CNT, 1539);
                }
            }
            T_EVOSTATE_SPARKLE_ARC => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    (*sEvoStructPtr).delayTimer = 1;
                    gBattleCommunication[2] = EvolutionSparkles_ArcDown();
                }
            }
            T_EVOSTATE_CYCLE_MON_SPRITE => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] = CycleEvolutionMonSprite(
                        (*sEvoStructPtr).preEvoSpriteId,
                        (*sEvoStructPtr).postEvoSpriteId,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_WAIT_CYCLE_MON_SPRITE => {
                if ({
                    (*sEvoStructPtr).delayTimer -= 1;
                    (*sEvoStructPtr).delayTimer
                }) == 0
                {
                    (*sEvoStructPtr).delayTimer = 3;
                    if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                        task_set(taskId, tState, task_get(taskId, tState) + 1);
                    }
                }
            }
            T_EVOSTATE_SPARKLE_CIRCLE => {
                gBattleCommunication[2] = EvolutionSparkles_CircleInward();
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            T_EVOSTATE_SPARKLE_SPRAY => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] =
                        EvolutionSparkles_SprayAndFlash_Trade(
                            task_get(taskId, tPostEvoSpecies) as u16
                        );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_EVO_SOUND => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    PlaySE(SE_EXP);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_EVO_MON_ANIM => {
                if IsSEPlaying() != 0 {
                    Free(sBgAnimPal as *mut c_void);
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).postEvoSpriteId,
                        task_get(taskId, tPostEvoSpecies) as u16,
                    );
                    memcpy(
                        &raw mut gPlttBufferUnfaded[32] as *mut u8,
                        (*sEvoStructPtr).savedPalette.as_mut_ptr() as *mut u8,
                        96,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_SET_MON_EVOLVED => {
                if IsCryFinished() != 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_CongratsPkmnEvolved)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 1);
                    PlayFanfare(MUS_EVOLVED);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    SetMonData(
                        mon,
                        MON_DATA_SPECIES,
                        task_data_ptr(taskId, tPostEvoSpecies) as *mut c_void,
                    );
                    CalculateMonStats(mon);
                    EvolutionRenameMon(
                        mon,
                        task_get(taskId, tPreEvoSpecies) as u16,
                        task_get(taskId, tPostEvoSpecies) as u16,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(task_get(taskId, tPostEvoSpecies) as u16),
                        FLAG_SET_SEEN,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(task_get(taskId, tPostEvoSpecies) as u16),
                        FLAG_SET_CAUGHT,
                    );
                    IncrementGameStat(GAME_STAT_EVOLVED_POKEMON);
                }
            }
            T_EVOSTATE_TRY_LEARN_MOVE => {
                if IsTextPrinterActive(0) == 0 && IsFanfareTaskInactive() == TRUE {
                    var =
                        MonTryLearningNewMove(mon, task_get(taskId, tLearnsFirstMove) as u8) as u32;
                    if var != MOVE_NONE as u32 && task_get(taskId, tEvoWasStopped) == 0 {
                        let mut nickname: CArray<u8, 20> = zeroed();
                        task_set(
                            taskId,
                            tBits,
                            task_get(taskId, tBits) | (TASK_BIT_LEARN_MOVE as i16),
                        );
                        task_set(taskId, tLearnsFirstMove, FALSE as i16);
                        task_set(taskId, tLearnMoveState, 0);
                        GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
                        StringCopy_Nickname(gBattleTextBuff1.as_mut_ptr(), nickname.as_mut_ptr());
                        if var == MON_HAS_MAX_MOVES as u32 {
                            task_set(taskId, tState, T_EVOSTATE_REPLACE_MOVE);
                        } else if var == MON_ALREADY_KNOWS_MOVE as u32 {
                            break 'l1;
                        } else {
                            task_set(taskId, tState, T_EVOSTATE_LEARNED_MOVE);
                        }
                    } else {
                        PlayBGM(MUS_EVOLUTION);
                        DrawTextOnTradeWindow(
                            0,
                            (*(&raw const crate::data::strings::gText_CommunicationStandby5)
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                            1,
                        );
                        task_set(taskId, tState, task_get(taskId, tState) + 1);
                    }
                }
            }
            T_EVOSTATE_END => {
                if IsTextPrinterActive(0) == 0 {
                    DestroyTask(taskId);
                    Free(sEvoStructPtr as *mut c_void);
                    sEvoStructPtr = null_mut();
                    (*(&raw const crate::text::gTextFlags)
                        .cast::<TextFlags>()
                        .cast_mut())
                    .set_useAlternateDownArrow(FALSE);
                    SetMainCallback2(gCB2_AfterEvolution);
                }
            }
            T_EVOSTATE_CANCEL => {
                if (*gTasks.as_ptr())[gBattleCommunication[2]].isActive == 0 {
                    m4aMPlayAllStop();
                    BeginNormalPaletteFade(
                        shl_i32(
                            1,
                            gSprites[(*sEvoStructPtr).preEvoSpriteId].oam.paletteNum() as u32 + 16,
                        ) as u32
                            | 0x4001C,
                        0,
                        0x10,
                        0,
                        32767,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_CANCEL_MON_ANIM => {
                if gPaletteFade.active() == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).preEvoSpriteId,
                        task_get(taskId, tPreEvoSpecies) as u16,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_CANCEL_MSG => {
                if EvoScene_IsMonAnimFinished((*sEvoStructPtr).preEvoSpriteId) != 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_EllipsisQuestionMark)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 1);
                    task_set(taskId, tEvoWasStopped, TRUE as i16);
                    task_set(taskId, tState, T_EVOSTATE_TRY_LEARN_MOVE);
                }
            }
            T_EVOSTATE_LEARNED_MOVE => {
                if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                    BufferMoveToLearnIntoBattleTextBuff2();
                    PlayFanfare(MUS_LEVEL_UP);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[3],
                    );
                    DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                    task_set(taskId, tLearnsFirstMove, 0x40);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            T_EVOSTATE_TRY_LEARN_ANOTHER_MOVE => {
                if IsTextPrinterActive(0) == 0
                    && IsFanfareTaskInactive() == TRUE
                    && ({
                        task_set(
                            taskId,
                            tLearnsFirstMove,
                            task_get(taskId, tLearnsFirstMove) - 1,
                        );
                        task_get(taskId, tLearnsFirstMove)
                    }) == 0
                {
                    task_set(taskId, tState, T_EVOSTATE_TRY_LEARN_MOVE);
                }
            }
            T_EVOSTATE_REPLACE_MOVE => 'l2: {
                let sw3: i16 = task_get(taskId, tLearnMoveState);
                let mut fall = false;
                if sw3 == T_MVSTATE_INTRO_MSG_1 {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BufferMoveToLearnIntoBattleTextBuff2();
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[4],
                        );
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_INTRO_MSG_2 {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[5],
                        );
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_INTRO_MSG_3 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[6],
                        );
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        task_set(taskId, tLearnMoveYesState, T_MVSTATE_SHOW_MOVE_SELECT);
                        task_set(taskId, tLearnMoveNoState, T_MVSTATE_ASK_CANCEL);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                }
                if fall || sw3 == T_MVSTATE_PRINT_YES_NO {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        LoadUserWindowBorderGfx(0, 0xA8, 224);
                        CreateYesNoMenu(
                            (&raw const (*(&raw const crate::data::trade::gTradeEvolutionSceneYesNoWindowTemplate).cast::<WindowTemplate>())).cast_mut(),
                            0xA8,
                            0xE,
                            0,
                        );
                        gBattleCommunication[1] = 0;
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                        gBattleCommunication[1] = 0;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_HANDLE_YES_NO {
                    match Menu_ProcessInputNoWrapClearOnChoose() {
                        0 => {
                            gBattleCommunication[1] = 0;
                            BattleStringExpandPlaceholdersToDisplayedString(
                                (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                    .cast::<CArray<*mut u8, 0>>())[292],
                            );
                            DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                            task_set(
                                taskId,
                                tLearnMoveState,
                                task_get(taskId, tLearnMoveYesState),
                            );
                            if task_get(taskId, tLearnMoveState) == T_MVSTATE_SHOW_MOVE_SELECT {
                                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                            }
                        }
                        1 | MENU_B_PRESSED => {
                            gBattleCommunication[1] = 1;
                            BattleStringExpandPlaceholdersToDisplayedString(
                                (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                    .cast::<CArray<*mut u8, 0>>())[292],
                            );
                            DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                            task_set(taskId, tLearnMoveState, task_get(taskId, tLearnMoveNoState));
                        }
                        _ => {}
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_SHOW_MOVE_SELECT {
                    if gPaletteFade.active() == 0 {
                        if gWirelessCommType != 0 {
                            DestroyWirelessStatusIndicatorSprite();
                        }
                        Free(GetBgTilemapBuffer(3));
                        Free(GetBgTilemapBuffer(1));
                        Free(GetBgTilemapBuffer(0));
                        FreeAllWindowBuffers();
                        ShowSelectMovePokemonSummaryScreen(
                            gPlayerParty.as_mut_ptr(),
                            task_get(taskId, tPartyId) as u8,
                            gPlayerPartyCount - 1,
                            Some(CB2_TradeEvolutionSceneLoadGraphics),
                            gMoveToLearn,
                        );
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_HANDLE_MOVE_SELECT {
                    if gPaletteFade.active() == 0
                        && gMain.callback2 == Some(CB2_TradeEvolutionSceneUpdate as unsafe fn())
                    {
                        var = GetMoveSlotToReplace() as u32;
                        if var == MAX_MON_MOVES as u32 {
                            task_set(taskId, tLearnMoveState, T_MVSTATE_ASK_CANCEL);
                        } else {
                            let r#move: u16 = GetMonData2(mon, var as i32 + MON_DATA_MOVE1) as u16;
                            if IsHMMove2(r#move) != 0 {
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    (*(&raw const crate::data::battle_message::gBattleStringsTable).cast::<CArray<*mut u8, 0>>())[307],
                                );
                                DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                                task_set(taskId, tLearnMoveState, T_MVSTATE_RETRY_AFTER_HM);
                            } else {
                                gBattleTextBuff2[0] = 0xFD;
                                gBattleTextBuff2[1] = 2;
                                gBattleTextBuff2[2] = r#move as u8;
                                gBattleTextBuff2[3] = ((r#move as i32 & 0xFF00) >> 8) as u8;
                                gBattleTextBuff2[4] = 0xFF;
                                RemoveMonPPBonus(mon, var as u8);
                                SetMonMoveSlot(mon, gMoveToLearn, var as u8);
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    (*(&raw const crate::data::battle_message::gBattleStringsTable).cast::<CArray<*mut u8, 0>>())[207],
                                );
                                DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                                task_set(
                                    taskId,
                                    tLearnMoveState,
                                    task_get(taskId, tLearnMoveState) + 1,
                                );
                            }
                        }
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_FORGET_MSG {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[7],
                        );
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        task_set(
                            taskId,
                            tLearnMoveState,
                            task_get(taskId, tLearnMoveState) + 1,
                        );
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_LEARNED_MOVE {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(
                            (*(&raw const crate::data::battle_message::gBattleStringsTable)
                                .cast::<CArray<*mut u8, 0>>())[208],
                        );
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        task_set(taskId, tState, T_EVOSTATE_LEARNED_MOVE);
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_ASK_CANCEL {
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[8],
                    );
                    DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                    task_set(taskId, tLearnMoveYesState, T_MVSTATE_CANCEL);
                    task_set(taskId, tLearnMoveNoState, T_MVSTATE_INTRO_MSG_1);
                    task_set(taskId, tLearnMoveState, T_MVSTATE_PRINT_YES_NO);
                    break 'l2;
                }
                if sw3 == T_MVSTATE_CANCEL {
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (*(&raw const crate::data::battle_message::gBattleStringsTable)
                            .cast::<CArray<*mut u8, 0>>())[9],
                    );
                    DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                    task_set(taskId, tState, T_EVOSTATE_TRY_LEARN_MOVE);
                    break 'l2;
                }
                if sw3 == T_MVSTATE_RETRY_AFTER_HM {
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        task_set(taskId, tLearnMoveState, T_MVSTATE_SHOW_MOVE_SELECT);
                    }
                    break 'l2;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn EvoDummyFunc() {}
pub(crate) unsafe fn VBlankCB_EvolutionScene() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X);
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe fn VBlankCB_TradeEvolutionScene() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X);
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe fn Task_UpdateBgPalette(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(6) != 0 {
        return;
    }
    if ({
        let t1 = *data.at(5);
        *data.at(5) += 1;
        t1
    }) < 20
    {
        return;
    }
    if ({
        let t2 = *data;
        *data += 1;
        t2
    }) > sBgAnim_PaletteControl[*data.at(2)][3] as i16
    {
        if sBgAnim_PaletteControl[*data.at(2)][1] as i16 == *data.at(1) {
            *data.at(3) += 1;
            if *data.at(3) == sBgAnim_PaletteControl[*data.at(2)][2] as i16 {
                *data.at(3) = 0;
                *data.at(2) += 1;
            }
            *data.at(1) = sBgAnim_PaletteControl[*data.at(2)][0] as i16;
        } else {
            LoadPalette(
                sBgAnimPal.at(*data.at(1) as i32 * 16) as *mut c_void,
                160,
                32,
            );
            *data = 0;
            *data.at(1) += 1;
        }
    }
    if *data.at(2) == 4 {
        DestroyTask(taskId);
    }
}
unsafe fn CreateBgAnimTask(isLink: u8) {
    let taskId: u8 = CreateTask(Some(Task_AnimateBg), 7);
    if isLink == 0 {
        task_set(taskId, tIsLink, FALSE as i16);
    } else {
        task_set(taskId, tIsLink, TRUE as i16);
    }
}
pub(crate) unsafe fn Task_AnimateBg(taskId: u8) {
    let mut outer_X: *mut u16 = null_mut();
    let mut outer_Y: *mut u16 = null_mut();
    let inner_X: *mut u16 = &raw mut gBattle_BG1_X;
    let inner_Y: *mut u16 = &raw mut gBattle_BG1_Y;
    if task_get(taskId, tIsLink) == 0 {
        outer_X = &raw mut gBattle_BG2_X;
        outer_Y = &raw mut gBattle_BG2_Y;
    } else {
        outer_X = &raw mut gBattle_BG3_X;
        outer_Y = &raw mut gBattle_BG3_Y;
    }
    task_set(taskId, 0, (task_get(taskId, 0) + 5) & 0xFF);
    task_set(taskId, 1, (task_get(taskId, 0) + 0x80) & 0xFF);
    *inner_X = Cos(task_get(taskId, 0), 4) as u16 + 8;
    *inner_Y = Sin(task_get(taskId, 0), 4) as u16 + 16;
    *outer_X = Cos(task_get(taskId, 1), 4) as u16 + 8;
    *outer_Y = Sin(task_get(taskId, 1), 4) as u16 + 16;
    if FuncIsActiveTask(Some(Task_UpdateBgPalette)) == 0 {
        DestroyTask(taskId);
        *inner_X = 0;
        *inner_Y = 0;
        *outer_X = 256;
        *outer_Y = 0;
    }
}
unsafe fn InitMovingBgPalette(palette: *mut u16) {
    for i in 0..50i32 {
        for j in 0..16i32 {
            *palette.at(i * 16 + j) = sBgAnim_Pal[sBgAnim_PalIndexes[i][j]];
        }
    }
}
unsafe fn StartBgAnimation(isLink: u8) {
    let mut innerBgId: u8 = 0;
    let mut outerBgId: u8 = 0;
    sBgAnimPal = AllocZeroed(0x640) as *mut u16;
    InitMovingBgPalette(sBgAnimPal);
    if isLink == 0 {
        innerBgId = 1;
        outerBgId = 2;
    } else {
        innerBgId = 1;
        outerBgId = 3;
    }
    LoadPalette(
        sBgAnim_Intro_Pal.as_ptr().cast_mut() as *mut c_void,
        160,
        32,
    );
    DecompressAndLoadBgGfxUsingHeap(1, sBgAnim_Gfx.as_ptr().cast_mut() as *mut c_void, 0, 0, 0);
    CopyToBgTilemapBuffer(
        innerBgId,
        sBgAnim_Inner_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        outerBgId,
        sBgAnim_Outer_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(innerBgId);
    CopyBgTilemapBufferToVram(outerBgId);
    if isLink == 0 {
        SetGpuReg(REG_OFFSET_BLDCNT, 1090);
        SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
        SetGpuReg(REG_OFFSET_DISPCNT, 5952);
        SetBgAttribute(innerBgId, BG_ATTR_PRIORITY, 2);
        SetBgAttribute(outerBgId, BG_ATTR_PRIORITY, 2);
        ShowBg(1);
        ShowBg(2);
    } else {
        SetGpuReg(REG_OFFSET_BLDCNT, 2114);
        SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
        SetGpuReg(REG_OFFSET_DISPCNT, 6976);
    }
    CreateTask(Some(Task_UpdateBgPalette), 5);
    CreateBgAnimTask(isLink);
}
unsafe fn PauseBgPaletteAnim() {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_UpdateBgPalette));
    if taskId != TASK_NONE {
        task_set(taskId, tPaused, TRUE as i16);
    }
    FillPalette(0, 160, 32);
}
unsafe fn StopBgAnimation() {
    let mut taskId: u8 = 0;
    if ({
        taskId = FindTaskIdByFunc(Some(Task_UpdateBgPalette));
        taskId
    }) != TASK_NONE
    {
        DestroyTask(taskId);
    }
    if ({
        taskId = FindTaskIdByFunc(Some(Task_AnimateBg));
        taskId
    }) != TASK_NONE
    {
        DestroyTask(taskId);
    }
    FillPalette(0, 160, 32);
    RestoreBgAfterAnim();
}
unsafe fn RestoreBgAfterAnim() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    SetBgAttribute(1, BG_ATTR_PRIORITY, GetBattleBgTemplateData(1, 5) as u8);
    SetBgAttribute(2, BG_ATTR_PRIORITY, GetBattleBgTemplateData(2, 5) as u8);
    SetGpuReg(REG_OFFSET_DISPCNT, 6464);
    Free(sBgAnimPal as *mut c_void);
}
unsafe fn EvoScene_DoMonAnimAndCry(monSpriteId: u8, speciesId: u16) {
    DoMonFrontSpriteAnimation(&raw mut gSprites[monSpriteId], speciesId, 0, 0);
}
unsafe fn EvoScene_IsMonAnimFinished(monSpriteId: u8) -> u32 {
    if gSprites[monSpriteId].callback == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite)) {
        return TRUE as u32;
    }
    FALSE as u32
}
