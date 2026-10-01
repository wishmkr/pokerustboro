//! Translated from `src/contest_util.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_gfx_sfx_util::{AllocateMonSpritesGfx, FreeMonSpritesGfx};
use crate::battle_main::gDisplayedStringBattle;
use crate::battle_main::{
    gBattle_BG0_X, gBattle_BG0_Y, gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y,
    gBattle_BG3_X, gBattle_BG3_Y, gBattle_WIN0H, gBattle_WIN0V, gBattle_WIN1H, gBattle_WIN1V,
    gMonSpritesGfxPtr,
};
use crate::bg::{
    CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, ResetBgsAndClearDma3BusyFlags,
    ShowBg, WriteSequenceToBgTilemapBuffer,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{
    CB2_StartContest, CalculateRound1Points, CreateContestMonFromParty, GetContestEntryEligibility,
    GetContestWinnerSaveIdx, SaveContestWinner, SaveLinkContestResults, SetContestants,
    SortContestants, gContestLinkLeaderIndex, gContestMonPartyIndex, gContestMons,
    gContestPlayerMonIndex, gContestRngValue, gCurContestWinnerIsForArtist,
    gCurContestWinnerSaveIdx, gLinkContestFlags, gNumLinkContestPlayers,
    gSpecialVar_ContestCategory, gSpecialVar_ContestRank,
};
use crate::contest::{
    gContestFinalStandings, gContestMonRound1Points, gContestMonRound2Points,
    gContestMonTotalPoints,
};
use crate::contest_link::{
    Task_LinkContest_CommunicateCategoryRS, Task_LinkContest_CommunicateLeaderIdsRS,
    Task_LinkContest_CommunicateMonIdxs, Task_LinkContest_CommunicateMonsRS,
    Task_LinkContest_CommunicateRngRS, Task_LinkContest_CommunicateRound1Points,
    Task_LinkContest_CommunicateTurnOrder, Task_LinkContest_Init,
};
use crate::contest_link_util::Task_LinkContest_StartCommunicationEm;
use crate::contest_painting::CB2_ContestPainting;
use crate::event_data::{VarGet, VarSet};
use crate::event_object_movement::GetObjectEventIdByLocalIdAndMap;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result};
use crate::field_player_avatar::gObjectEvents;
use crate::field_specials::TryGainNewFanFromCounter;
use crate::gpu_regs::{SetGpuReg, SetGpuRegBits};
use crate::international_string_util::ConvertInternationalContestantName;
use crate::link::{
    GetMultiplayerId, IsLinkTaskFinished, SetCloseLinkCallback, SetLinkStandbyCallback,
    gLinkPlayers, gReceivedRemoteLinkPlayers,
};
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, DestroyWirelessStatusIndicatorSprite,
    LoadWirelessStatusIndicatorSpriteGfx, gWirelessStatusIndicatorSpriteId,
};
use crate::load_save::gSaveBlock1Ptr;
use crate::load_save::{ClearContinueGameWarpStatus2, SetContinueGameWarpStatusToDynamicWarp};
use crate::menu::{AddTextPrinterParameterized3, SetStandardWindowBorderStyle};
use crate::overworld::{
    CB2_ReturnToFieldContinueScriptPlayMapMusic, IncrementGameStat, SetDynamicWarp,
};
use crate::palette::{
    BeginHardwarePaletteFade, BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette,
    LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon::{
    DoMonFrontSpriteAnimation, GetMonData2, GetMonSpritePalStructFromOtIdPersonality, SetMonData,
    SetMultiuseSpriteTemplateToPokemon, SpeciesToNationalPokedexNum, gMultiuseSpriteTemplate,
    gPlayerParty,
};
use crate::pokemon_icon::{GetIconSpecies, GetMonIconPtr};
use crate::random::Random;
use crate::save::TrySavingData;
use crate::scanline_effect::{ScanlineEffect_Clear, ScanlineEffect_InitHBlankDmaTransfer};
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::script_menu::{ClearToTransparentAndRemoveWindow, CreateWindowFromRect};
use crate::sound::{PlayBGM, PlayCry_Normal, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, FreeSpritePaletteByTag,
    GetSpritePaletteTagByPaletteNum, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
    gReservedSpritePaletteCount,
};
use crate::string_util::StringGet_Nickname;
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3};
use crate::task::{DestroyTask, ResetTasks, RunTasks, TaskDummy};
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::trig::Sin;
use crate::tv::{
    BravoTrainerPokemonProfile_BeforeInterview2, GetRibbonCount, InterviewAfter, InterviewBefore,
    TryPutSpotTheCutiesOnAir,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
use crate::window::{
    FillWindowPixelBuffer, FreeAllWindowBuffers, GetWindowAttribute, PutWindowTilemap, RemoveWindow,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
    }
}
/// `CopyToBgTilemapBufferRect` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect(a0, a1 as _, a2, a3, a4, a5);
    }
}
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
}
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
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
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
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `HandleLoadSpecialPokePic_2` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_2(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_2(a0 as _, a1 as _, a2, a3);
    }
}
/// `HandleLoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `RequestDma3Copy` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Copy(a0 as _, a1 as _, a2, a3) }
}
/// `RequestDma3Fill` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Fill(a0, a1 as _, a2, a3) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
unsafe fn SetTaskFuncWithFollowupFunc(
    a0: u8,
    a1: Option<unsafe fn(u8)>,
    a2: Option<unsafe fn(u8)>,
) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StringAppend` with this module's view of its types.
#[inline]
unsafe fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppend(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tDelay: usize = 0;
const tFinalStanding: usize = 0;
const tMonId: usize = 0;
const tCoeff: usize = 1;
const tNumFrames: usize = 1;
const tTarget: usize = 1;
const tCounter: usize = 2;
const tDecreasing: usize = 2;
const tSpecies: usize = 2;
const sTargetX: usize = 4;
const sSlideOutTimer: usize = 5;
const sSlideIncrement: usize = 6;
const sDistance: usize = 7;
const tBounced: usize = 11;
// Data tables (translate with cdata.py): sResultsTextWindow_Pal sResultsTextWindow_Gfx sMiscBlank_Pal sOamData_ResultsTextWindow sSpriteTemplate_ResultsTextWindow sSpriteSheets_ResultsTextWindow sSpritePalette_ResultsTextWindow sOamData_Confetti sSpriteTemplate_Confetti sSpriteSheet_Confetti sSpritePalette_Confetti sBgTemplates sWindowTemplates sOamData_WirelessIndicatorWindow sSpriteTemplate_WirelessIndicatorWindow sSpriteSheet_WirelessIndicatorWindow sContestLinkTextColors sContestantLocalIds.0

/// `struct ContestResults`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContestResults {
    pub data: *mut ContestResultsInternal,
    pub monResults: *mut CArray<ContestMonResults, 4>,
    pub unusedBg: *mut u8,
    pub tilemapBuffers: CArray<*mut u8, 4>,
    pub unused: *mut u8,
}

unsafe impl Sync for ContestResults {}

/// `struct ContestResultsInternal`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ContestResultsInternal {
    pub slidingTextBoxSpriteId: u8,
    pub linkTextBoxSpriteId: u8,
    pub showResultsTaskId: u8,
    pub highlightWinnerTaskId: u8,
    pub slidingTextBoxState: u8,
    pub numStandingsPrinted: u8,
    pub winnerMonSlidingState: u8,
    pub confettiCount: u8,
    pub winnerMonSpriteId: u8,
    pub destroyConfetti: u8,
    pub pointsFlashing: u8,
    pub barLength: CArray<i16, 4>,
    pub numBarsUpdating: u8,
}

unsafe impl Sync for ContestResultsInternal {}

/// `struct ContestMonResults`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContestMonResults {
    pub relativePreliminaryPoints: i32,
    pub relativeRound2Points: i32,
    pub barLengthPreliminary: u32,
    pub barLengthRound2: u32,
    pub lostPoints: u8,
    pub numStars: u8,
    pub numHearts: u8,
}

unsafe impl Sync for ContestMonResults {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ContestResults>() == 32);
    assert!(offset_of!(ContestResults, data) == 0);
    assert!(offset_of!(ContestResults, monResults) == 4);
    assert!(offset_of!(ContestResults, unusedBg) == 8);
    assert!(offset_of!(ContestResults, tilemapBuffers) == 12);
    assert!(offset_of!(ContestResults, unused) == 28);
    assert!(size_of::<ContestResultsInternal>() == 24);
    assert!(offset_of!(ContestResultsInternal, slidingTextBoxSpriteId) == 0);
    assert!(offset_of!(ContestResultsInternal, linkTextBoxSpriteId) == 1);
    assert!(offset_of!(ContestResultsInternal, showResultsTaskId) == 2);
    assert!(offset_of!(ContestResultsInternal, highlightWinnerTaskId) == 3);
    assert!(offset_of!(ContestResultsInternal, slidingTextBoxState) == 4);
    assert!(offset_of!(ContestResultsInternal, numStandingsPrinted) == 5);
    assert!(offset_of!(ContestResultsInternal, winnerMonSlidingState) == 6);
    assert!(offset_of!(ContestResultsInternal, confettiCount) == 7);
    assert!(offset_of!(ContestResultsInternal, winnerMonSpriteId) == 8);
    assert!(offset_of!(ContestResultsInternal, destroyConfetti) == 9);
    assert!(offset_of!(ContestResultsInternal, pointsFlashing) == 10);
    assert!(offset_of!(ContestResultsInternal, barLength) == 12);
    assert!(offset_of!(ContestResultsInternal, numBarsUpdating) == 20);
    assert!(size_of::<ContestMonResults>() == 20);
    assert!(offset_of!(ContestMonResults, relativePreliminaryPoints) == 0);
    assert!(offset_of!(ContestMonResults, relativeRound2Points) == 4);
    assert!(offset_of!(ContestMonResults, barLengthPreliminary) == 8);
    assert!(offset_of!(ContestMonResults, barLengthRound2) == 12);
    assert!(offset_of!(ContestMonResults, lostPoints) == 16);
    assert!(offset_of!(ContestMonResults, numStars) == 17);
    assert!(offset_of!(ContestMonResults, numHearts) == 18);
};

const BAR_SEGMENT_LENGTH: i32 = 8;
const MAX_BAR_LENGTH: i32 = 88;
const NUM_BAR_SEGMENTS: i32 = 11;
const SLIDING_MON_ENTERED: u8 = 1;
const SLIDING_MON_EXITED: u8 = 2;
const SLIDING_TEXT_ARRIVED: u8 = 2;
const SLIDING_TEXT_ENTERING: u8 = 1;
const SLIDING_TEXT_EXITING: u8 = 3;
const SLIDING_TEXT_OFFSCREEN: u8 = 0;
const TEXT_BOX_X: i16 = 272;
const TEXT_BOX_Y: u16 = 144;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::contest_util::sBgTemplates).cast());
static sContestLinkTextColors: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::contest_util::sContestLinkTextColors).cast());
static sContestantLocalIds_0: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::contest_util::sContestantLocalIds_0).cast());
static sResultsTextWindow_Gfx: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::contest_util::sResultsTextWindow_Gfx).cast());
static sResultsTextWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::contest_util::sResultsTextWindow_Pal).cast());
static sSpritePalette_Confetti: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::contest_util::sSpritePalette_Confetti).cast());
static sSpritePalette_ResultsTextWindow: Table<SpritePalette> =
    Table((&raw const crate::data::contest_util::sSpritePalette_ResultsTextWindow).cast());
static sSpriteSheet_Confetti: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::contest_util::sSpriteSheet_Confetti).cast());
static sSpriteSheet_WirelessIndicatorWindow: Table<SpriteSheet> =
    Table((&raw const crate::data::contest_util::sSpriteSheet_WirelessIndicatorWindow).cast());
static sSpriteSheets_ResultsTextWindow: Table<CArray<SpriteSheet, 8>> =
    Table((&raw const crate::data::contest_util::sSpriteSheets_ResultsTextWindow).cast());
static sSpriteTemplate_Confetti: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest_util::sSpriteTemplate_Confetti).cast());
static sSpriteTemplate_ResultsTextWindow: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest_util::sSpriteTemplate_ResultsTextWindow).cast());
static sSpriteTemplate_WirelessIndicatorWindow: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest_util::sSpriteTemplate_WirelessIndicatorWindow).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::contest_util::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestResults: *mut ContestResults = null_mut();

/// `AddTextPrinter` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinter(
    a0: *mut TextPrinterTemplate,
    a1: u8,
    a2: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe { crate::text::AddTextPrinter(a0 as _, a1, core::mem::transmute(a2)) }
}
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
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn InitContestResultsDisplay() {
    SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_OBJ_1D_MAP);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    for i in 0..4i32 {
        SetBgTilemapBuffer(i as u8, (*sContestResults).tilemapBuffers[i] as *mut c_void);
    }
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16174);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuRegBits(REG_OFFSET_DISPCNT, 65280);
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 0;
    gBattle_BG3_Y = 0;
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    gBattle_WIN1H = 0;
    gBattle_WIN1V = 0;
}
unsafe fn LoadContestResultsBgGfx() {
    let mut numStars: i8 = 0;
    let mut round2Points: i8 = 0;
    let mut tile1: u16 = 0;
    let mut tile2: u16 = 0;
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gContestResults_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x6000000_usize as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        3,
        (*(&raw const crate::data::graphics::gContestResults_Bg_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        2,
        (*(&raw const crate::data::graphics::gContestResults_Interface_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        0,
        (*(&raw const crate::data::graphics::gContestResults_WinnerBanner_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        0,
        0,
    );
    LoadContestResultsTitleBarTilemaps();
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gContestResults_Pal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        BG_PLTT_OFFSET,
        BG_PLTT_SIZE,
    );
    LoadPalette(
        sResultsTextWindow_Pal.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    for i in 0..CONTESTANT_COUNT {
        numStars = GetNumPreliminaryPoints(i as u8, TRUE) as i8;
        round2Points = GetNumRound2Points(i as u8, TRUE);
        for j in 0..10i32 {
            tile1 = 0x60B2;
            if j < numStars as i32 {
                tile1 += 2;
            }
            if j < (if round2Points < 0 {
                -(round2Points as i32)
            } else {
                round2Points as i32
            }) {
                tile2 = 0x60A4;
                if round2Points < 0 {
                    tile2 += 2;
                }
            } else {
                tile2 = 0x60A2;
            }
            FillBgTilemapBufferRect_Palette0(1, tile1, j as u8 + 19, i as u8 * 3 + 5, 1, 1);
            FillBgTilemapBufferRect_Palette0(1, tile2, j as u8 + 19, i as u8 * 3 + 6, 1, 1);
        }
    }
    CopyBgTilemapBufferToVram(0);
    CopyBgTilemapBufferToVram(1);
    CopyBgTilemapBufferToVram(2);
    CopyBgTilemapBufferToVram(3);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
unsafe fn LoadContestMonName(monIndex: u8) {
    let mon: *mut ContestPokemon = &raw mut gContestMons[monIndex];
    let mut str: *mut u8 = gDisplayedStringBattle.as_mut_ptr();
    if monIndex == gContestPlayerMonIndex {
        str = StringCopy(
            gDisplayedStringBattle.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_ColorDarkGray).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    StringCopy(str, (*mon).nickname.as_mut_ptr());
    AddContestTextPrinter(monIndex as i32, gDisplayedStringBattle.as_mut_ptr(), 0);
    StringCopy(
        str,
        (*(&raw const crate::data::strings::gText_Slash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringAppend(str, (*mon).trainerName.as_mut_ptr());
    AddContestTextPrinter(monIndex as i32, gDisplayedStringBattle.as_mut_ptr(), 50);
}
unsafe fn LoadAllContestMonNames() {
    for i in 0..CONTESTANT_COUNT {
        LoadContestMonName(i as u8);
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe fn CB2_StartShowContestResults() {
    gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    SetVBlankCallback(None);
    AllocContestResults();
    InitContestResultsDisplay();
    ScanlineEffect_Clear();
    ResetPaletteFade();
    ResetSpriteData();
    ResetTasks();
    FreeAllSpritePalettes();
    LoadContestResultsBgGfx();
    LoadAllContestMonIconPalettes();
    LoadAllContestMonIcons(0, TRUE);
    LoadAllContestMonNames();
    memset((*sContestResults).data as *mut u8, 0, 24);
    memset((*sContestResults).monResults as *mut u8, 0, 80);
    CreateResultsTextWindowSprites();
    TryCreateWirelessSprites();
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
    (*(*sContestResults).data).showResultsTaskId = CreateTask(Some(Task_ShowContestResults), 5);
    SetMainCallback2(Some(CB2_ShowContestResults));
    gBattle_WIN1H = DISPLAY_WIDTH;
    gBattle_WIN1V = 32928;
    CreateTask(Some(Task_SlideContestResultsBg), 20);
    CalculateContestantsResultData();
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    } else {
        PlayBGM(MUS_CONTEST_RESULTS);
    }
    SetVBlankCallback(Some(VBlankCB_ShowContestResults));
}
pub(crate) unsafe fn CB2_ShowContestResults() {
    AnimateSprites();
    BuildOamBuffer();
    RunTasks();
    UpdatePaletteFade();
    CopyBgTilemapBufferToVram(1);
    CopyBgTilemapBufferToVram(2);
}
pub(crate) unsafe fn VBlankCB_ShowContestResults() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X);
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y);
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    SetGpuReg(REG_OFFSET_WIN1H, gBattle_WIN1H);
    SetGpuReg(REG_OFFSET_WIN1V, gBattle_WIN1V);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe fn Task_ShowContestResults(taskId: u8) {
    let mut var: u16 = 0;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        'l1: {
            match task_get(taskId, 0) {
                0 => {
                    SaveLinkContestResults();
                    if gContestFinalStandings[gContestPlayerMonIndex] == 0 {
                        IncrementGameStat(GAME_STAT_WON_LINK_CONTEST);
                        gSpecialVar_0x8005 = TVSHOW_CONTEST_LIVE_UPDATES as u16;
                        InterviewBefore();
                        if gSpecialVar_Result != TRUE as u16 {
                            InterviewAfter();
                        }
                    }
                    TryGainNewFanFromCounter(FANCOUNTER_FINISHED_CONTEST);
                    SaveContestWinner(gSpecialVar_ContestRank as u8);
                    SaveContestWinner(CONTEST_SAVE_FOR_ARTIST as u8);
                    gCurContestWinnerIsForArtist = TRUE;
                    gCurContestWinnerSaveIdx =
                        GetContestWinnerSaveIdx(CONTEST_SAVE_FOR_ARTIST as u8, FALSE);
                    var = VarGet(VAR_CONTEST_HALL_STATE);
                    VarSet(VAR_CONTEST_HALL_STATE, 0);
                    SetContinueGameWarpStatusToDynamicWarp();
                    TrySavingData(SAVE_LINK);
                    ClearContinueGameWarpStatus2();
                    VarSet(VAR_CONTEST_HALL_STATE, var);
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
                1 => {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS == 0 {
                        task_set(taskId, 0, 100);
                    }
                }
                2 => {
                    if IsLinkTaskFinished() != 0 {
                        SetLinkStandbyCallback();
                        task_set(taskId, 0, task_get(taskId, 0) + 1);
                    }
                    return;
                }
                3 => {
                    if IsLinkTaskFinished() == TRUE {
                        PlayBGM(MUS_CONTEST_RESULTS);
                        gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
                        task_set(taskId, 0, task_get(taskId, 0) + 1);
                        break 'l1;
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    if gPaletteFade.active() == 0 {
        task_set(taskId, 0, 0);
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            ShowLinkResultsTextBox(
                (*(&raw const crate::data::strings::gText_CommunicationStandby)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            task_set_func(taskId, Some(Task_WaitForLinkPartnersBeforeResults));
        } else {
            IncrementGameStat(GAME_STAT_ENTERED_CONTEST);
            if gContestFinalStandings[gContestPlayerMonIndex] == 0 {
                IncrementGameStat(GAME_STAT_WON_CONTEST);
            }
            SaveContestWinner(gSpecialVar_ContestRank as u8);
            SaveContestWinner(CONTEST_SAVE_FOR_ARTIST as u8);
            gCurContestWinnerIsForArtist = TRUE;
            gCurContestWinnerSaveIdx =
                GetContestWinnerSaveIdx(CONTEST_SAVE_FOR_ARTIST as u8, FALSE);
            TryGainNewFanFromCounter(FANCOUNTER_FINISHED_CONTEST);
            task_set_func(taskId, Some(Task_AnnouncePreliminaryResults));
        }
    }
}
pub(crate) unsafe fn Task_WaitForLinkPartnersBeforeResults(taskId: u8) {
    if gReceivedRemoteLinkPlayers != 0 {
        CreateTask(Some(Task_CommunicateMonIdxsForResults), 0);
        task_set_func(taskId, Some(TaskDummy));
    }
}
pub(crate) unsafe fn Task_CommunicateMonIdxsForResults(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateMonIdxs),
        Some(Task_WaitForLinkPartnerMonIdxs),
    );
}
pub(crate) unsafe fn Task_WaitForLinkPartnerMonIdxs(taskId: u8) {
    if IsLinkTaskFinished() != 0 {
        DestroyTask(taskId);
        task_set_func(
            (*(*sContestResults).data).showResultsTaskId,
            Some(Task_AnnouncePreliminaryResults),
        );
        HideLinkResultsTextBox();
    }
}
pub(crate) unsafe fn Task_AnnouncePreliminaryResults(taskId: u8) {
    let mut x: i16 = 0;
    if task_get(taskId, 0) == 0 {
        CreateTask(Some(Task_FlashStarsAndHearts), 20);
        x = DrawResultsTextWindow(
            (*(&raw const crate::data::strings::gText_AnnouncingResults).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            (*(*sContestResults).data).slidingTextBoxSpriteId,
        ) as i16;
        StartTextBoxSlideIn(x, TEXT_BOX_Y, 120, 1088);
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else if task_get(taskId, 0) == 1 {
        if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_OFFSCREEN {
            task_set(taskId, 1, 0);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
    } else if task_get(taskId, 0) == 2 {
        if ({
            task_set(taskId, 1, task_get(taskId, 1) + 1);
            task_get(taskId, 1)
        }) == 21
        {
            task_set(taskId, 1, 0);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
    } else if task_get(taskId, 0) == 3 {
        x = DrawResultsTextWindow(
            (*(&raw const crate::data::strings::gText_PreliminaryResults).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            (*(*sContestResults).data).slidingTextBoxSpriteId,
        ) as i16;
        StartTextBoxSlideIn(x, TEXT_BOX_Y, 65535, 1088);
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else if task_get(taskId, 0) == 4
        && (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_ARRIVED
    {
        task_set(taskId, 0, 0);
        task_set_func(taskId, Some(Task_ShowPreliminaryResults));
    }
}
pub(crate) unsafe fn Task_ShowPreliminaryResults(taskId: u8) {
    match task_get(taskId, 0) {
        0 => {
            if (*(*sContestResults).data).pointsFlashing == 0 {
                UpdateContestResultBars(
                    FALSE,
                    ({
                        let t1 = task_get(taskId, tCounter);
                        task_set(taskId, tCounter, task_get(taskId, tCounter) + 1);
                        t1
                    }) as u8,
                );
                if (*(*sContestResults).data).numBarsUpdating == 0 {
                    task_set(taskId, 0, 2);
                } else {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        1 => {
            if (*(*sContestResults).data).numBarsUpdating == 0 {
                task_set(taskId, 0, 0);
            }
        }
        2 => {
            StartTextBoxSlideOut(1088);
            task_set(taskId, 0, 0);
            task_set(taskId, tCounter, 0);
            task_set_func(taskId, Some(Task_AnnounceRound2Results));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_AnnounceRound2Results(taskId: u8) {
    let mut x: i16 = 0;
    if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_OFFSCREEN {
        if ({
            task_set(taskId, 1, task_get(taskId, 1) + 1);
            task_get(taskId, 1)
        }) == 21
        {
            task_set(taskId, 1, 0);
            x = DrawResultsTextWindow(
                (*(&raw const crate::data::strings::gText_Round2Results).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                (*(*sContestResults).data).slidingTextBoxSpriteId,
            ) as i16;
            StartTextBoxSlideIn(x, TEXT_BOX_Y, 65535, 1088);
        }
    } else if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_ARRIVED {
        task_set_func(taskId, Some(Task_ShowRound2Results));
    }
}
pub(crate) unsafe fn Task_ShowRound2Results(taskId: u8) {
    match task_get(taskId, 0) {
        0 => {
            if (*(*sContestResults).data).pointsFlashing == 0 {
                UpdateContestResultBars(
                    TRUE,
                    ({
                        let t1 = task_get(taskId, tCounter);
                        task_set(taskId, tCounter, task_get(taskId, tCounter) + 1);
                        t1
                    }) as u8,
                );
                if (*(*sContestResults).data).numBarsUpdating == 0 {
                    task_set(taskId, 0, 2);
                } else {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        1 => {
            if (*(*sContestResults).data).numBarsUpdating == 0 {
                task_set(taskId, 0, 0);
            }
        }
        2 => {
            StartTextBoxSlideOut(1088);
            task_set(taskId, 0, 0);
            task_set_func(taskId, Some(Task_AnnounceWinner));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_AnnounceWinner(taskId: u8) {
    let mut i: i32 = 0;
    match task_get(taskId, 0) {
        0 => {
            if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_OFFSCREEN {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        1 => {
            if ({
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                task_get(taskId, 1)
            }) == 31
            {
                task_set(taskId, 1, 0);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        2 => {
            for i in 0..CONTESTANT_COUNT {
                let newTaskId: u8 = CreateTask(Some(Task_DrawFinalStandingNumber), 10);
                task_set(newTaskId, 0, gContestFinalStandings[i] as i16);
                task_set(newTaskId, 1, i as i16);
            }
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        3 => {
            if (*(*sContestResults).data).numStandingsPrinted == CONTESTANT_COUNT as u8
                && ({
                    task_set(taskId, 1, task_get(taskId, 1) + 1);
                    task_get(taskId, 1)
                }) == 31
            {
                task_set(taskId, 1, 0);
                CreateTask(Some(Task_StartHighlightWinnersBox), 10);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
                i = 0;
                while i < 4
                    && (*(&raw const crate::contest::gContestFinalStandings)
                        .cast::<CArray<u8, 4>>()
                        .cast_mut())[i]
                        != 0
                {
                    i += 1;
                }
                BounceMonIconInBox(i as u8, 14);
            }
        }
        4 => {
            if ({
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                task_get(taskId, 1)
            }) == 21
            {
                let mut winnerTextBuffer: CArray<u8, 100> = zeroed();
                task_set(taskId, 1, 0);
                i = 0;
                while i < 4
                    && (*(&raw const crate::contest::gContestFinalStandings)
                        .cast::<CArray<u8, 4>>()
                        .cast_mut())[i]
                        != 0
                {
                    i += 1;
                }
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gContestMons[i].trainerName.as_mut_ptr(),
                );
                ConvertInternationalContestantName(gStringVar1.as_mut_ptr());
                StringCopy(
                    gStringVar2.as_mut_ptr(),
                    gContestMons[i].nickname.as_mut_ptr(),
                );
                StringExpandPlaceholders(
                    winnerTextBuffer.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_ContestantsMonWon)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                let x: i16 = DrawResultsTextWindow(
                    winnerTextBuffer.as_mut_ptr(),
                    (*(*sContestResults).data).slidingTextBoxSpriteId,
                ) as i16;
                StartTextBoxSlideIn(x, TEXT_BOX_Y, 65535, 1088);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        5 => {
            task_set(taskId, 0, 0);
            task_set_func(taskId, Some(Task_ShowWinnerMonBanner));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_ShowWinnerMonBanner(taskId: u8) {
    let mut i: i32 = 0;
    let mut spriteId: u8 = 0;
    let mut species: u16 = 0;
    let mut otId: u32 = 0;
    let mut personality: u32 = 0;
    let mut pokePal: *mut CompressedSpritePalette = null_mut();
    match task_get(taskId, 0) {
        0 => {
            gBattle_WIN0H = DISPLAY_WIDTH;
            gBattle_WIN0V = 20560;
            i = 0;
            while i < 4
                && (*(&raw const crate::contest::gContestFinalStandings)
                    .cast::<CArray<u8, 4>>()
                    .cast_mut())[i]
                    != 0
            {
                i += 1;
            }
            species = gContestMons[i].species;
            personality = gContestMons[i].personality;
            otId = gContestMons[i].otId;
            if i == gContestPlayerMonIndex as i32 {
                HandleLoadSpecialPokePic_2(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                        .cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[1],
                    species as i32,
                    personality,
                );
            } else {
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                        .cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[1],
                    species as i32,
                    personality,
                );
            }
            pokePal = GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
            LoadCompressedSpritePalette(pokePal);
            SetMultiuseSpriteTemplateToPokemon(species, B_POSITION_OPPONENT_LEFT);
            gMultiuseSpriteTemplate.paletteTag = (*pokePal).tag;
            spriteId = CreateSprite(&raw mut gMultiuseSpriteTemplate, 272, 80, 10);
            gSprites[spriteId].data[1] = species as i16;
            gSprites[spriteId].oam.set_priority(0);
            gSprites[spriteId].callback = Some(SpriteCB_WinnerMonSlideIn);
            (*(*sContestResults).data).winnerMonSpriteId = spriteId;
            LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Confetti).cast_mut());
            LoadCompressedSpritePalette((&raw const *sSpritePalette_Confetti).cast_mut());
            CreateTask(Some(Task_CreateConfetti), 10);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 => {
            if ({
                task_set(taskId, 3, task_get(taskId, 3) + 1);
                task_get(taskId, 3)
            }) == 1
            {
                task_set(taskId, 3, 0);
                task_set(taskId, tCounter, task_get(taskId, tCounter) + 2);
                if task_get(taskId, tCounter) > 32 {
                    task_set(taskId, tCounter, 32);
                }
                let counter: u8 = task_get(taskId, tCounter) as u8;
                gBattle_WIN0V = ((80 - counter as u16) << 8) | (80 + counter as u16);
                if counter == 32 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        2 => {
            if (*(*sContestResults).data).winnerMonSlidingState == SLIDING_MON_ENTERED {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        3 => {
            if ({
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                task_get(taskId, 1)
            }) == 121
            {
                task_set(taskId, 1, 0);
                gSprites[(*(*sContestResults).data).winnerMonSpriteId].callback =
                    Some(SpriteCB_WinnerMonSlideOut);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        4 => {
            if (*(*sContestResults).data).winnerMonSlidingState == SLIDING_MON_EXITED {
                let mut top: u8 = (gBattle_WIN0V >> 8) as u8;
                top += 2;
                if top > 80 {
                    top = 80;
                }
                gBattle_WIN0V = ((top as u16) << 8) | (DISPLAY_HEIGHT - top as u16);
                if top == 80 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        5 if (*(*sContestResults).data).winnerMonSlidingState == SLIDING_MON_EXITED => {
            (*(*sContestResults).data).destroyConfetti = TRUE;
            task_set(taskId, 0, 0);
            task_set_func(taskId, Some(Task_SetSeenWinnerMon));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_SetSeenWinnerMon(taskId: u8) {
    let mut nationalDexNum: i32 = 0;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
            for i in 0..CONTESTANT_COUNT {
                nationalDexNum = SpeciesToNationalPokedexNum(gContestMons[i].species) as i32;
                GetSetPokedexFlag(nationalDexNum as u16, FLAG_SET_SEEN);
            }
        }
        task_set(taskId, 10, 0);
        task_set_func(taskId, Some(Task_TryDisconnectLinkPartners));
    }
}
pub(crate) unsafe fn Task_TryDisconnectLinkPartners(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        if task_get(taskId, 10) == 0 {
            ShowLinkResultsTextBox(
                (*(&raw const crate::data::strings::gText_CommunicationStandby)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            SetCloseLinkCallback();
            task_set_func(taskId, Some(Task_WaitForLinkPartnersDisconnect));
        }
    } else {
        task_set_func(taskId, Some(Task_TrySetContestInterviewData));
    }
}
pub(crate) unsafe fn Task_WaitForLinkPartnersDisconnect(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
            DestroyWirelessStatusIndicatorSprite();
        }
        HideLinkResultsTextBox();
        task_set_func(taskId, Some(Task_TrySetContestInterviewData));
    }
}
pub(crate) unsafe fn Task_TrySetContestInterviewData(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
        BravoTrainerPokemonProfile_BeforeInterview2(gContestFinalStandings[gContestPlayerMonIndex]);
    }
    BeginHardwarePaletteFade(0xFF, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_EndShowContestResults));
}
pub(crate) unsafe fn Task_EndShowContestResults(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if task_get(taskId, 1) == 0 {
            DestroyTask((*(*sContestResults).data).highlightWinnerTaskId);
            BlendPalettes(PALETTES_BG, 16, 0);
            task_set(taskId, 1, task_get(taskId, 1) + 1);
        } else if task_get(taskId, 1) == 1 {
            BlendPalettes(PALETTES_OBJECTS, 16, 0);
            task_set(taskId, 1, task_get(taskId, 1) + 1);
        } else {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            DestroyTask(taskId);
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
            FreeContestResults();
        }
    }
}
pub(crate) unsafe fn Task_SlideContestResultsBg(taskId: u8) {
    gBattle_BG3_X += 2;
    gBattle_BG3_Y += 1;
    if gBattle_BG3_X > 255 {
        gBattle_BG3_X -= 255;
    }
    if gBattle_BG3_Y > 255 {
        gBattle_BG3_Y -= 255;
    }
}
pub(crate) unsafe fn Task_FlashStarsAndHearts(taskId: u8) {
    if ({
        task_set(taskId, tDelay, task_get(taskId, tDelay) + 1);
        task_get(taskId, tDelay)
    }) == 2
    {
        task_set(taskId, tDelay, 0);
        if task_get(taskId, tDecreasing) == 0 {
            task_set(taskId, tCoeff, task_get(taskId, tCoeff) + 1);
        } else {
            task_set(taskId, tCoeff, task_get(taskId, tCoeff) - 1);
        }
        if task_get(taskId, tCoeff) == 16 {
            task_set(taskId, tDecreasing, TRUE as i16);
        } else if task_get(taskId, tCoeff) == 0 {
            task_set(taskId, tDecreasing, FALSE as i16);
        }
        BlendPalette(107, 1, task_get(taskId, tCoeff) as u8, 11998);
        BlendPalette(104, 1, task_get(taskId, tCoeff) as u8, 32767);
        BlendPalette(110, 1, task_get(taskId, tCoeff) as u8, 30654);
    }
    if task_get(taskId, tCoeff) == 0 {
        (*(*sContestResults).data).pointsFlashing = FALSE;
    } else {
        (*(*sContestResults).data).pointsFlashing = TRUE;
    }
}
unsafe fn LoadContestMonIcon(
    species: u16,
    monIndex: u8,
    srcOffset: u8,
    useDmaNow: u8,
    personality: u32,
) {
    let mut var0: u16 = 0;
    let mut var1: u16 = 0;
    let mut frameNum: u16 = 0;
    if monIndex == gContestPlayerMonIndex {
        frameNum = 1;
    } else {
        frameNum = 0;
    }
    let mut iconPtr: *mut u8 = GetMonIconPtr(species, personality, frameNum as u32);
    iconPtr = iconPtr.at(srcOffset as i32 * 0x200 + 0x80);
    if useDmaNow != 0 {
        RequestDma3Copy(
            iconPtr as *mut c_void,
            (0x6004000_usize as *mut c_void as *mut u8).at(monIndex as i32 * 0x200) as *mut c_void,
            0x180,
            1,
        );
        var0 = (monIndex as u16 + 10) << 12;
        var1 = monIndex as u16 * 0x10 + 0x200;
        WriteSequenceToBgTilemapBuffer(1, var1 | var0, 3, monIndex * 3 + 4, 4, 3, 17, 1);
    } else {
        RequestDma3Copy(
            iconPtr as *mut c_void,
            (0x6004000_usize as *mut c_void as *mut u8).at(monIndex as i32 * 0x200) as *mut c_void,
            0x180,
            1,
        );
    }
}
unsafe fn LoadAllContestMonIcons(srcOffset: u8, useDmaNow: u8) {
    for i in 0..CONTESTANT_COUNT {
        LoadContestMonIcon(
            gContestMons[i].species,
            i as u8,
            srcOffset,
            useDmaNow,
            gContestMons[i].personality,
        );
    }
}
unsafe fn LoadAllContestMonIconPalettes() {
    let mut species: i32 = 0;
    for i in 0..CONTESTANT_COUNT {
        species = gContestMons[i].species as i32;
        LoadPalette(
            (*(&raw const crate::data::graphics::gMonIconPalettes)
                .cast::<CArray<CArray<u16, 16>, 0>>())
                [(*(&raw const crate::data::pokemon_icon::gMonIconPaletteIndices)
                    .cast::<CArray<u8, 0>>())[GetIconSpecies(species as u16, 0)]]
            .as_ptr()
            .cast_mut() as *mut c_void,
            (10 + i as u16) * 16,
            32,
        );
    }
}
unsafe fn TryCreateWirelessSprites() {
    let mut sheet: u16 = 0;
    let mut spriteId: u8 = 0;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        LoadWirelessStatusIndicatorSpriteGfx();
        CreateWirelessStatusIndicatorSprite(8, 8);
        gSprites[gWirelessStatusIndicatorSpriteId].subpriority = 1;
        sheet = LoadSpriteSheet((&raw const *sSpriteSheet_WirelessIndicatorWindow).cast_mut());
        RequestDma3Fill(
            -1,
            (0x6010000_usize as *mut c_void as *mut u8).at(sheet as i32 * 0x20) as *mut c_void,
            0x80,
            1,
        );
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_WirelessIndicatorWindow).cast_mut(),
            8,
            8,
            0,
        );
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
    }
}
unsafe fn DrawResultsTextWindow(text: *mut u8, spriteId: u8) -> i32 {
    let mut spriteTilePtrs: CArray<*mut u8, 4> = zeroed();
    let mut dst: *mut u8 = null_mut();
    let mut windowTemplate: WindowTemplate = zeroed();
    memset(&raw mut windowTemplate as *mut u8, 0, 8);
    windowTemplate.width = DISPLAY_TILE_WIDTH;
    windowTemplate.height = 2;
    let windowId: u16 = AddWindow(&raw mut windowTemplate);
    FillWindowPixelBuffer(windowId as u8, 17);
    let strWidth: i32 = GetStringWidth(FONT_NORMAL, text, 0);
    let mut tileWidth: i32 = (strWidth + 9) / 8;
    if tileWidth > DISPLAY_TILE_WIDTH as i32 {
        tileWidth = DISPLAY_TILE_WIDTH as i32;
    }
    AddTextPrinterParameterized3(
        windowId as u8,
        FONT_NORMAL,
        ((tileWidth * 8 - strWidth) / 2) as u8,
        1,
        sContestLinkTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        text,
    );
    {
        let mut windowTilesPtr: *mut u8 =
            GetWindowAttribute(windowId as u8, WINDOW_TILE_DATA) as usize as *mut u8;
        let src: *mut u8 = sResultsTextWindow_Gfx.as_ptr().cast_mut();
        let sprite: *mut Sprite = &raw mut gSprites[spriteId];
        spriteTilePtrs[0] = ((*sprite).oam.tileNum() as i32 * 32 + OBJ_VRAM0) as usize as *mut u8;
        for i in 1..4i32 {
            spriteTilePtrs[i] = (gSprites[(*sprite).data[i - 1]].oam.tileNum() as i32 * 32
                + OBJ_VRAM0) as usize as *mut c_void as *mut u8;
        }
        for i in 0..4i32 {
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        spriteTilePtrs[i] as *mut c_void,
                        0x5000100,
                    );
                }
            }
        }
        dst = spriteTilePtrs[0];
        CpuSet(src as *mut c_void, dst as *mut c_void, 0x4000008);
        CpuSet(
            src.at(128) as *mut c_void,
            dst.at(256) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(128) as *mut c_void,
            dst.at(512) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(64) as *mut c_void,
            dst.at(768) as *mut c_void,
            0x4000008,
        );
        let mut i: i32 = 0;
        while i < tileWidth {
            dst = spriteTilePtrs[(i + 1) / 8].at((i + 1) % 8 * 32);
            CpuSet(src.at(192) as *mut c_void, dst as *mut c_void, 0x4000008);
            CpuSet(
                windowTilesPtr as *mut c_void,
                dst.at(256) as *mut c_void,
                0x4000008,
            );
            CpuSet(
                windowTilesPtr.at(960) as *mut c_void,
                dst.at(512) as *mut c_void,
                0x4000008,
            );
            CpuSet(
                src.at(224) as *mut c_void,
                dst.at(768) as *mut c_void,
                0x4000008,
            );
            windowTilesPtr = windowTilesPtr.at(32);
            i += 1;
        }
        dst = spriteTilePtrs[(i + 1) / 8].at((i + 1) % 8 * 32);
        CpuSet(src.at(32) as *mut c_void, dst as *mut c_void, 0x4000008);
        CpuSet(
            src.at(160) as *mut c_void,
            dst.at(256) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(160) as *mut c_void,
            dst.at(512) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(96) as *mut c_void,
            dst.at(768) as *mut c_void,
            0x4000008,
        );
    }
    RemoveWindow(windowId as u8);
    (DISPLAY_WIDTH as i32 - (tileWidth + 2) * 8) / 2
}
unsafe fn CreateResultsTextWindowSprites() {
    let mut spriteIds: CArray<u8, 8> = zeroed();
    let mut template: SpriteTemplate = *sSpriteTemplate_ResultsTextWindow;
    for i in 0..8i32 {
        LoadSpriteSheet((&raw const sSpriteSheets_ResultsTextWindow[i]).cast_mut());
    }
    LoadSpritePalette((&raw const *sSpritePalette_ResultsTextWindow).cast_mut());
    for i in 0..8i32 {
        spriteIds[i] = CreateSprite(&raw mut template, TEXT_BOX_X, TEXT_BOX_Y as i16, 10);
        template.tileTag += 1;
    }
    gSprites[spriteIds[0]].data[0] = spriteIds[1] as i16;
    gSprites[spriteIds[0]].data[1] = spriteIds[2] as i16;
    gSprites[spriteIds[0]].data[2] = spriteIds[3] as i16;
    gSprites[spriteIds[4]].data[0] = spriteIds[5] as i16;
    gSprites[spriteIds[4]].data[1] = spriteIds[6] as i16;
    gSprites[spriteIds[4]].data[2] = spriteIds[7] as i16;
    (*(*sContestResults).data).slidingTextBoxSpriteId = spriteIds[0];
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_OFFSCREEN;
    (*(*sContestResults).data).linkTextBoxSpriteId = spriteIds[4];
    HideLinkResultsTextBox();
}
unsafe fn StartTextBoxSlideIn(x: i16, y: u16, slideOutTimer: u16, slideIncrement: u16) {
    let sprite: *mut Sprite = &raw mut gSprites[(*(*sContestResults).data).slidingTextBoxSpriteId];
    (*sprite).x = TEXT_BOX_X;
    (*sprite).y = y as i16;
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    (*sprite).data[sTargetX] = x + 32;
    (*sprite).data[sSlideOutTimer] = slideOutTimer as i16;
    (*sprite).data[sSlideIncrement] = slideIncrement as i16;
    (*sprite).data[sDistance] = 0;
    (*sprite).callback = Some(SpriteCB_TextBoxSlideIn);
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_ENTERING;
}
unsafe fn StartTextBoxSlideOut(slideIncrement: u16) {
    let sprite: *mut Sprite = &raw mut gSprites[(*(*sContestResults).data).slidingTextBoxSpriteId];
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    (*sprite).data[sSlideIncrement] = slideIncrement as i16;
    (*sprite).data[sDistance] = 0;
    (*sprite).callback = Some(SpriteCB_TextBoxSlideOut);
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_EXITING;
}
unsafe fn EndTextBoxSlideOut(sprite: *mut Sprite) {
    (*sprite).x = TEXT_BOX_X;
    (*sprite).y = TEXT_BOX_Y as i16;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    (*sprite).callback = Some(SpriteCallbackDummy);
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_OFFSCREEN;
}
pub(crate) unsafe fn SpriteCB_TextBoxSlideIn(sprite: *mut Sprite) {
    let delta: i16 = (*sprite).data[sDistance] + (*sprite).data[sSlideIncrement];
    (*sprite).x -= delta >> 8;
    (*sprite).data[sDistance] += (*sprite).data[sSlideIncrement];
    (*sprite).data[sDistance] &= 0xFF;
    if (*sprite).x < (*sprite).data[sTargetX] {
        (*sprite).x = (*sprite).data[sTargetX];
    }
    for i in 0..3i32 {
        let sprite2: *mut Sprite = &raw mut gSprites[(*sprite).data[i]];
        (*sprite2).x = (*sprite).x + (*sprite).x2 + (i as i16 + 1) * 64;
    }
    if (*sprite).x == (*sprite).data[sTargetX] {
        (*sprite).callback = Some(SpriteCB_EndTextBoxSlideIn);
    }
}
pub(crate) unsafe fn SpriteCB_EndTextBoxSlideIn(sprite: *mut Sprite) {
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_ARRIVED;
    if (*sprite).data[sSlideOutTimer] as u16 != 0xFFFF
        && ({
            (*sprite).data[sSlideOutTimer] -= 1;
            (*sprite).data[sSlideOutTimer]
        }) == -1
    {
        StartTextBoxSlideOut((*sprite).data[sSlideIncrement] as u16);
    }
}
pub(crate) unsafe fn SpriteCB_TextBoxSlideOut(sprite: *mut Sprite) {
    let delta: i16 = (*sprite).data[sDistance] + (*sprite).data[sSlideIncrement];
    (*sprite).x -= delta >> 8;
    (*sprite).data[sDistance] += (*sprite).data[sSlideIncrement];
    (*sprite).data[sDistance] &= 0xFF;
    for i in 0..3i32 {
        let sprite2: *mut Sprite = &raw mut gSprites[(*sprite).data[i]];
        (*sprite2).x = (*sprite).x + (*sprite).x2 + (i as i16 + 1) * 64;
    }
    if ((*sprite).x as i32 + (*sprite).x2 as i32) < -224 {
        EndTextBoxSlideOut(sprite);
    }
}
unsafe fn ShowLinkResultsTextBox(text: *mut u8) {
    let x: u16 = DrawResultsTextWindow(text, (*(*sContestResults).data).linkTextBoxSpriteId) as u16;
    let sprite: *mut Sprite = &raw mut gSprites[(*(*sContestResults).data).linkTextBoxSpriteId];
    (*sprite).x = x as i16 + 32;
    (*sprite).y = 80;
    (*sprite).set_invisible(FALSE as u16);
    for i in 0..3i32 {
        gSprites[(*sprite).data[i]].x = (*sprite).x + (*sprite).x2 + (i as i16 + 1) * 64;
        gSprites[(*sprite).data[i]].y = (*sprite).y;
        gSprites[(*sprite).data[i]].set_invisible(FALSE as u16);
    }
    gBattle_WIN0H = DISPLAY_WIDTH;
    gBattle_WIN0V = (((*sprite).y as u16 - 16) << 8) | ((*sprite).y as u16 + 16);
    SetGpuReg(REG_OFFSET_WININ, 16190);
}
unsafe fn HideLinkResultsTextBox() {
    let sprite: *mut Sprite = &raw mut gSprites[(*(*sContestResults).data).linkTextBoxSpriteId];
    (*sprite).set_invisible(TRUE as u16);
    for i in 0..3i32 {
        gSprites[(*sprite).data[i]].set_invisible(TRUE as u16);
    }
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    SetGpuReg(REG_OFFSET_WININ, 16191);
}
unsafe fn LoadContestResultsTitleBarTilemaps() {
    let mut palette: u8 = 0;
    let mut x: i32 = 5;
    let y: i32 = 1;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Link_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            5,
            1,
            5,
            2,
        );
        x = 10;
    } else if gSpecialVar_ContestRank == CONTEST_RANK_NORMAL as u16 {
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Normal_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    } else if gSpecialVar_ContestRank == CONTEST_RANK_SUPER as u16 {
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Super_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    } else if gSpecialVar_ContestRank == CONTEST_RANK_HYPER as u16 {
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Hyper_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    } else {
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Master_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    }
    if gSpecialVar_ContestCategory == CONTEST_CATEGORY_COOL as u16 {
        palette = 0;
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Cool_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else if gSpecialVar_ContestCategory == CONTEST_CATEGORY_BEAUTY as u16 {
        palette = 1;
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Beauty_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else if gSpecialVar_ContestCategory == CONTEST_CATEGORY_CUTE as u16 {
        palette = 2;
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Cute_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else if gSpecialVar_ContestCategory == CONTEST_CATEGORY_SMART as u16 {
        palette = 3;
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Smart_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else {
        palette = 4;
        CopyToBgTilemapBufferRect(
            2,
            (*(&raw const crate::data::graphics::gContestResultsTitle_Tough_Tilemap)
                .cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    }
    x += 5;
    CopyToBgTilemapBufferRect(
        2,
        (*(&raw const crate::data::graphics::gContestResultsTitle_Tilemap).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        x as u8,
        y as u8,
        6,
        2,
    );
    CopyToBgTilemapBufferRect_ChangePalette(
        2,
        (*sContestResults).tilemapBuffers[2] as *mut c_void,
        0,
        0,
        32,
        4,
        palette,
    );
}
unsafe fn GetNumPreliminaryPoints(monIndex: u8, capPoints: u8) -> u8 {
    let condition: u32 = (gContestMonRound1Points[monIndex] as u32) << 16;
    let mut numStars: u32 = condition / 63;
    if numStars & 0xFFFF != 0 {
        numStars += 0x10000;
    }
    numStars >>= 16;
    if numStars == 0 && condition != 0 {
        numStars = 1;
    }
    if capPoints != 0 && numStars > 10 {
        numStars = 10;
    }
    numStars as u8
}
unsafe fn GetNumRound2Points(monIndex: u8, capPoints: u8) -> i8 {
    let mut r4: u32 = 0;
    let mut points: i8 = 0;
    let results: i16 = gContestMonRound2Points[monIndex];
    if results < 0 {
        r4 = (results as u32).wrapping_neg() << 16;
    } else {
        r4 = (results as u32) << 16;
    }
    let mut numHearts: u32 = r4 / 80;
    if numHearts & 0xFFFF != 0 {
        numHearts += 0x10000;
    }
    numHearts >>= 16;
    if numHearts == 0 && r4 != 0 {
        numHearts = 1;
    }
    if capPoints != 0 && numHearts > 10 {
        numHearts = 10;
    }
    if gContestMonRound2Points[monIndex] < 0 {
        points = -(numHearts as i8);
    } else {
        points = numHearts as i8;
    }
    points
}
pub(crate) unsafe fn Task_DrawFinalStandingNumber(taskId: u8) {
    let mut firstTileNum: u16 = 0;
    if task_get(taskId, 10) == 0 {
        task_set(taskId, 11, (3 - task_get(taskId, tFinalStanding)) * 40);
        task_set(taskId, 10, task_get(taskId, 10) + 1);
    } else if task_get(taskId, 10) == 1
        && ({
            task_set(taskId, 11, task_get(taskId, 11) - 1);
            task_get(taskId, 11)
        }) == -1
    {
        firstTileNum = task_get(taskId, tFinalStanding) as u16 * 2 + 0x5043;
        WriteSequenceToBgTilemapBuffer(
            2,
            firstTileNum,
            1,
            task_get(taskId, 1) as u8 * 3 + 5,
            2,
            1,
            17,
            1,
        );
        WriteSequenceToBgTilemapBuffer(
            2,
            firstTileNum + 0x10,
            1,
            task_get(taskId, 1) as u8 * 3 + 6,
            2,
            1,
            17,
            1,
        );
        (*(*sContestResults).data).numStandingsPrinted += 1;
        DestroyTask(taskId);
        PlaySE(SE_CONTEST_PLACE);
    }
}
pub(crate) unsafe fn Task_StartHighlightWinnersBox(taskId: u8) {
    let mut i: i32 = 0;
    while i < 4
        && (*(&raw const crate::contest::gContestFinalStandings)
            .cast::<CArray<u8, 4>>()
            .cast_mut())[i]
            != 0
    {
        i += 1;
    }
    CopyToBgTilemapBufferRect_ChangePalette(
        2,
        (*sContestResults).tilemapBuffers[2].at(i * 0xC0 + 0x100) as *mut c_void,
        0,
        i as u8 * 3 + 4,
        32,
        3,
        9,
    );
    task_set(taskId, 10, i as i16);
    task_set(taskId, 12, 1);
    task_set_func(taskId, Some(Task_HighlightWinnersBox));
    (*(*sContestResults).data).highlightWinnerTaskId = taskId;
}
pub(crate) unsafe fn Task_HighlightWinnersBox(taskId: u8) {
    if ({
        task_set(taskId, 11, task_get(taskId, 11) + 1);
        task_get(taskId, 11)
    }) == 1
    {
        task_set(taskId, 11, 0);
        BlendPalette(145, 1, task_get(taskId, 12) as u8, 28557);
        if task_get(taskId, 13) == 0 {
            if ({
                task_set(taskId, 12, task_get(taskId, 12) + 1);
                task_get(taskId, 12)
            }) == 16
            {
                task_set(taskId, 13, 1);
            }
        } else {
            if ({
                task_set(taskId, 12, task_get(taskId, 12) - 1);
                task_get(taskId, 12)
            }) == 0
            {
                task_set(taskId, 13, 0);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_WinnerMonSlideIn(sprite: *mut Sprite) {
    if (*sprite).data[0] < 10 {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 10
        {
            PlayCry_Normal((*sprite).data[1] as u16, 0);
            (*sprite).data[1] = 0;
        }
    } else {
        let delta: i16 = (*sprite).data[1] + 0x600;
        (*sprite).x -= delta >> 8;
        (*sprite).data[1] += 0x600;
        (*sprite).data[1] &= 0xFF;
        if (*sprite).x < 120 {
            (*sprite).x = 120;
        }
        if (*sprite).x == 120 {
            (*sprite).callback = Some(SpriteCallbackDummy);
            (*sprite).data[1] = 0;
            (*(*sContestResults).data).winnerMonSlidingState = SLIDING_MON_ENTERED;
        }
    }
}
pub(crate) unsafe fn SpriteCB_WinnerMonSlideOut(sprite: *mut Sprite) {
    let delta: i16 = (*sprite).data[1] + 0x600;
    (*sprite).x -= delta >> 8;
    (*sprite).data[1] += 0x600;
    (*sprite).data[1] &= 0xFF;
    if (*sprite).x < -32 {
        (*sprite).callback = Some(SpriteCallbackDummy);
        (*sprite).set_invisible(TRUE as u16);
        (*(*sContestResults).data).winnerMonSlidingState = SLIDING_MON_EXITED;
    }
}
pub(crate) unsafe fn Task_CreateConfetti(taskId: u8) {
    if ({
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_get(taskId, 0)
    }) == 5
    {
        task_set(taskId, 0, 0);
        if (*(*sContestResults).data).confettiCount < 40 {
            let spriteId: u8 = CreateSprite(
                (&raw const *sSpriteTemplate_Confetti).cast_mut(),
                (Random() as i32 % 240) as i16 - 20,
                44,
                5,
            );
            gSprites[spriteId].data[0] = (Random() as i32 % 512) as i16;
            gSprites[spriteId].data[1] = (Random() as i32 % 24) as i16 + 16;
            gSprites[spriteId].data[2] = (Random() as i32 % 256) as i16 + 48;
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].oam.tileNum() + (Random() as i32 % 17) as u16);
            (*(*sContestResults).data).confettiCount += 1;
        }
    }
    if (*(*sContestResults).data).destroyConfetti != 0 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn SpriteCB_Confetti(sprite: *mut Sprite) {
    (*sprite).data[3] += (*sprite).data[0];
    (*sprite).x2 = Sin((*sprite).data[3] >> 8, (*sprite).data[1]);
    let delta: i16 = (*sprite).data[4] + (*sprite).data[2];
    (*sprite).x += delta >> 8;
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).data[4] &= 0xff;
    (*sprite).y += 1;
    if (*(*sContestResults).data).destroyConfetti != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if (*sprite).x > 248 || (*sprite).y > 116 {
        DestroySprite(sprite);
        (*(*sContestResults).data).confettiCount -= 1;
    }
}
unsafe fn BounceMonIconInBox(monIndex: u8, numFrames: u8) {
    let taskId: u8 = CreateTask(Some(Task_BounceMonIconInBox), 8);
    task_set(taskId, 0, monIndex as i16);
    task_set(taskId, tNumFrames, numFrames as i16);
    task_set(taskId, tSpecies, gContestMons[monIndex].species as i16);
}
pub(crate) unsafe fn Task_BounceMonIconInBox(taskId: u8) {
    let monIndex: u8 = task_get(taskId, 0) as u8;
    if ({
        let t1 = task_get(taskId, 10);
        task_set(taskId, 10, task_get(taskId, 10) + 1);
        t1
    }) == task_get(taskId, tNumFrames)
    {
        task_set(taskId, 10, 0);
        LoadContestMonIcon(
            task_get(taskId, tSpecies) as u16,
            monIndex,
            task_get(taskId, tBounced) as u8,
            FALSE,
            gContestMons[monIndex].personality,
        );
        task_set(taskId, tBounced, task_get(taskId, tBounced) ^ 1);
    }
}
unsafe fn CalculateContestantsResultData() {
    let mut relativePoints: i32 = 0;
    let mut barLength: u32 = 0;
    let mut round2Points: i8 = 0;
    let mut highestPoints: i16 = gContestMonTotalPoints[0];
    let mut i: i32 = 1;
    while i < CONTESTANT_COUNT {
        if highestPoints < gContestMonTotalPoints[i] {
            highestPoints = gContestMonTotalPoints[i];
        }
        i += 1;
    }
    if highestPoints < 0 {
        highestPoints = gContestMonTotalPoints[0];
        for i in 1..CONTESTANT_COUNT {
            if highestPoints > gContestMonTotalPoints[i] {
                highestPoints = gContestMonTotalPoints[i];
            }
        }
    }
    for i in 0..CONTESTANT_COUNT {
        relativePoints = div_i32(
            gContestMonRound1Points[i] as i32 * 1000,
            if highestPoints < 0 {
                -(highestPoints as i32)
            } else {
                highestPoints as i32
            },
        );
        if relativePoints % 10 > 4 {
            relativePoints += 10;
        }
        (*(*sContestResults).monResults)[i].relativePreliminaryPoints = relativePoints / 10;
        relativePoints = div_i32(
            (if gContestMonRound2Points[i] < 0 {
                -(gContestMonRound2Points[i] as i32)
            } else {
                gContestMonRound2Points[i] as i32
            }) * 1000,
            if highestPoints < 0 {
                -(highestPoints as i32)
            } else {
                highestPoints as i32
            },
        );
        if relativePoints % 10 > 4 {
            relativePoints += 10;
        }
        (*(*sContestResults).monResults)[i].relativeRound2Points = relativePoints / 10;
        if gContestMonRound2Points[i] < 0 {
            (*(*sContestResults).monResults)[i].lostPoints = TRUE;
        }
        barLength =
            ((*(*sContestResults).monResults)[i].relativePreliminaryPoints * 0x5800 / 100) as u32;
        if barLength & 0xFF > 0x7F {
            barLength += 0x100;
        }
        (*(*sContestResults).monResults)[i].barLengthPreliminary = barLength >> 8;
        barLength =
            ((*(*sContestResults).monResults)[i].relativeRound2Points * 0x5800 / 100) as u32;
        if barLength & 0xFF > 0x7F {
            barLength += 0x100;
        }
        (*(*sContestResults).monResults)[i].barLengthRound2 = barLength >> 8;
        (*(*sContestResults).monResults)[i].numStars = GetNumPreliminaryPoints(i as u8, TRUE);
        round2Points = GetNumRound2Points(i as u8, TRUE);
        (*(*sContestResults).monResults)[i].numHearts = (if round2Points < 0 {
            -(round2Points as i32)
        } else {
            round2Points as i32
        }) as u8;
        if gContestFinalStandings[i] != 0 {
            let barLengthPreliminary: i16 =
                (*(*sContestResults).monResults)[i].barLengthPreliminary as i16;
            let mut barLengthRound2: i16 =
                (*(*sContestResults).monResults)[i].barLengthRound2 as i16;
            if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                barLengthRound2 *= -1;
            }
            if barLengthPreliminary as i32 + barLengthRound2 as i32 == MAX_BAR_LENGTH {
                if barLengthRound2 > 0 {
                    (*(*sContestResults).monResults)[i].barLengthRound2 -= 1;
                } else if barLengthPreliminary > 0 {
                    (*(*sContestResults).monResults)[i].barLengthPreliminary -= 1;
                }
            }
        }
    }
}
unsafe fn UpdateContestResultBars(isRound2: u8, numUpdates: u8) {
    let mut taskId: i32 = 0;
    let mut target: u32 = 0;
    let mut numIncreasing: u8 = 0;
    let mut numDecreasing: u8 = 0;
    if isRound2 == 0 {
        for i in 0..CONTESTANT_COUNT {
            let numStars: u8 = (*(*sContestResults).monResults)[i].numStars;
            if numUpdates < numStars {
                FillBgTilemapBufferRect_Palette0(
                    1,
                    0x60B3,
                    19 + numStars - numUpdates - 1,
                    i as u8 * 3 + 5,
                    1,
                    1,
                );
                taskId = CreateTask(Some(Task_UpdateContestResultBar), 10) as i32;
                target = div_u32(
                    (*(*sContestResults).monResults)[i].barLengthPreliminary << 16,
                    (*(*sContestResults).monResults)[i].numStars as u32,
                ) * (numUpdates as u32 + 1);
                if target & 0xFFFF > 0x7FFF {
                    target += 0x10000;
                }
                task_set(taskId, tMonId, i as i16);
                task_set(taskId, tTarget, (target >> 16) as i16);
                (*(*sContestResults).data).numBarsUpdating += 1;
                numIncreasing += 1;
            }
        }
    } else {
        for i in 0..CONTESTANT_COUNT {
            let numHearts: i8 = (*(*sContestResults).monResults)[i].numHearts as i8;
            let tile: u32 = (if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                0x60A5
            } else {
                0x60A3
            }) as u32;
            if (numUpdates as i32) < numHearts as i32 {
                FillBgTilemapBufferRect_Palette0(
                    1,
                    tile as u16,
                    19 + numHearts as u8 - numUpdates - 1,
                    i as u8 * 3 + 6,
                    1,
                    1,
                );
                taskId = CreateTask(Some(Task_UpdateContestResultBar), 10) as i32;
                target = div_u32(
                    (*(*sContestResults).monResults)[i].barLengthRound2 << 16,
                    (*(*sContestResults).monResults)[i].numHearts as u32,
                ) * (numUpdates as u32 + 1);
                if target & 0xFFFF > 0x7FFF {
                    target += 0x10000;
                }
                task_set(taskId, tMonId, i as i16);
                if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                    task_set(taskId, tDecreasing, TRUE as i16);
                    numDecreasing += 1;
                } else {
                    numIncreasing += 1;
                }
                if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                    task_set(
                        taskId,
                        tTarget,
                        -((target >> 16) as i16)
                            + (*(*sContestResults).monResults)[i].barLengthPreliminary as i16,
                    );
                } else {
                    task_set(
                        taskId,
                        tTarget,
                        (target >> 16) as i16
                            + (*(*sContestResults).monResults)[i].barLengthPreliminary as i16,
                    );
                }
                (*(*sContestResults).data).numBarsUpdating += 1;
            }
        }
    }
    if numDecreasing != 0 {
        PlaySE(SE_BOO);
    }
    if numIncreasing != 0 {
        PlaySE(SE_PIN);
    }
}
pub(crate) unsafe fn Task_UpdateContestResultBar(taskId: u8) {
    let mut minMaxReached: u32 = FALSE as u32;
    let mut targetReached: u32 = FALSE as u32;
    let monId: u8 = task_get(taskId, tMonId) as u8;
    let target: i16 = task_get(taskId, tTarget);
    let decreasing: i16 = task_get(taskId, tDecreasing);
    if decreasing != 0 {
        if (*(*sContestResults).data).barLength[monId] <= 0 {
            minMaxReached = TRUE as u32;
        }
    } else {
        if (*(*sContestResults).data).barLength[monId] >= MAX_BAR_LENGTH as i16 {
            minMaxReached = TRUE as u32;
        }
    }
    if (*(*sContestResults).data).barLength[monId] == target {
        targetReached = TRUE as u32;
    }
    if targetReached == 0 {
        if minMaxReached != 0 {
            (*(*sContestResults).data).barLength[monId] = target;
        } else if decreasing != 0 {
            (*(*sContestResults).data).barLength[monId] -= 1;
        } else {
            (*(*sContestResults).data).barLength[monId] += 1;
        }
    }
    if minMaxReached == 0 && targetReached == 0 {
        let mut tileOffset: u8 = 0;
        let mut tileNum: u16 = 0;
        for i in 0..NUM_BAR_SEGMENTS {
            if (*(*sContestResults).data).barLength[monId] as i32 >= (i + 1) * BAR_SEGMENT_LENGTH {
                tileOffset = 8;
            } else if (*(*sContestResults).data).barLength[monId] as i32 >= i * BAR_SEGMENT_LENGTH {
                tileOffset = ((*(*sContestResults).data).barLength[monId] % 8) as u8;
            } else {
                tileOffset = 0;
            }
            if tileOffset < 4 {
                tileNum = 0x504C + tileOffset as u16;
            } else {
                tileNum = 0x5057 + tileOffset as u16;
            }
            FillBgTilemapBufferRect_Palette0(2, tileNum, i as u8 + 7, monId * 3 + 6, 1, 1);
        }
    }
    if targetReached != 0 {
        (*(*sContestResults).data).numBarsUpdating -= 1;
        DestroyTask(taskId);
    }
}
unsafe fn AllocContestResults() {
    sContestResults = AllocZeroed(32) as *mut ContestResults;
    (*sContestResults).data = AllocZeroed(24) as *mut ContestResultsInternal;
    (*sContestResults).monResults = AllocZeroed(80) as *mut CArray<ContestMonResults, 4>;
    (*sContestResults).unusedBg = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[0] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[1] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[2] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[3] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).unused = AllocZeroed(0x1000) as *mut u8;
    AllocateMonSpritesGfx();
}
unsafe fn FreeContestResults() {
    Free((*sContestResults).data as *mut c_void);
    (*sContestResults).data = null_mut();
    Free((*sContestResults).monResults as *mut c_void);
    (*sContestResults).monResults = null_mut();
    Free((*sContestResults).unusedBg as *mut c_void);
    (*sContestResults).unusedBg = null_mut();
    Free((*sContestResults).tilemapBuffers[0] as *mut c_void);
    (*sContestResults).tilemapBuffers[0] = null_mut();
    Free((*sContestResults).tilemapBuffers[1] as *mut c_void);
    (*sContestResults).tilemapBuffers[1] = null_mut();
    Free((*sContestResults).tilemapBuffers[2] as *mut c_void);
    (*sContestResults).tilemapBuffers[2] = null_mut();
    Free((*sContestResults).tilemapBuffers[3] as *mut c_void);
    (*sContestResults).tilemapBuffers[3] = null_mut();
    Free((*sContestResults).unused as *mut c_void);
    (*sContestResults).unused = null_mut();
    Free(sContestResults as *mut c_void);
    sContestResults = null_mut();
    FreeMonSpritesGfx();
}
unsafe fn AddContestTextPrinter(windowId: i32, str: *mut u8, x: i32) {
    let mut textPrinter: TextPrinterTemplate = zeroed();
    textPrinter.currentChar = str;
    textPrinter.windowId = windowId as u8;
    textPrinter.fontId = FONT_NARROW;
    textPrinter.x = x as u8;
    textPrinter.y = 2;
    textPrinter.currentX = x as u8;
    textPrinter.currentY = 2;
    textPrinter.letterSpacing = 0;
    textPrinter.lineSpacing = 0;
    textPrinter.set_unk(0);
    textPrinter.set_fgColor(1);
    textPrinter.set_bgColor(0);
    textPrinter.set_shadowColor(8);
    AddTextPrinter(&raw mut textPrinter, 0, None);
    PutWindowTilemap(windowId as u8);
}
#[unsafe(no_mangle)]
pub unsafe fn TryEnterContestMon() {
    let eligibility: u8 = GetContestEntryEligibility(&raw mut gPlayerParty[gContestMonPartyIndex]);
    if eligibility != 0 {
        SetContestants(
            gSpecialVar_ContestCategory as u8,
            gSpecialVar_ContestRank as u8,
        );
        CalculateRound1Points(gSpecialVar_ContestCategory as u8);
    }
    gSpecialVar_Result = eligibility as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn HasMonWonThisContestBefore() -> u16 {
    let mut hasRankRibbon: u16 = FALSE as u16;
    let mon: *mut Pokemon = &raw mut gPlayerParty[gContestMonPartyIndex];
    match gSpecialVar_ContestCategory {
        0 => {
            if GetMonData2(mon, MON_DATA_COOL_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        1 => {
            if GetMonData2(mon, MON_DATA_BEAUTY_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        2 => {
            if GetMonData2(mon, MON_DATA_CUTE_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        3 => {
            if GetMonData2(mon, MON_DATA_SMART_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        4 if GetMonData2(mon, MON_DATA_TOUGH_RIBBON) > gSpecialVar_ContestRank as u32 => {
            hasRankRibbon = TRUE as u16;
        }
        _ => {}
    }
    hasRankRibbon
}
#[unsafe(no_mangle)]
pub unsafe fn GiveMonContestRibbon() {
    let mut ribbonData: u8 = 0;
    if gContestFinalStandings[gContestPlayerMonIndex] != 0 {
        return;
    }
    match gSpecialVar_ContestCategory {
        0 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_COOL_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_COOL_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_COOL_RIBBON as u8,
                    );
                }
            }
        }
        1 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_BEAUTY_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_BEAUTY_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_BEAUTY_RIBBON as u8,
                    );
                }
            }
        }
        2 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_CUTE_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_CUTE_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_CUTE_RIBBON as u8,
                    );
                }
            }
        }
        3 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_SMART_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_SMART_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_SMART_RIBBON as u8,
                    );
                }
            }
        }
        4 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_TOUGH_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_TOUGH_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_TOUGH_RIBBON as u8,
                    );
                }
            }
        }
        _ => {}
    }
}
pub unsafe fn BufferContestantTrainerName() {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
        .trainerName
        .as_mut_ptr(),
    );
    ConvertInternationalContestantName(gStringVar1.as_mut_ptr());
}
pub unsafe fn BufferContestantMonNickname() {
    StringCopy(
        gStringVar3.as_mut_ptr(),
        gContestMons[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
        .nickname
        .as_mut_ptr(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestMonConditionRanking() {
    let mut rank: u8 = 0;
    for i in 0..(CONTESTANT_COUNT as u8) {
        if gContestMonRound1Points[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
            < gContestMonRound1Points[i]
        {
            rank += 1;
        }
    }
    gSpecialVar_0x8004 = rank as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestMonCondition() {
    gSpecialVar_0x8004 = gContestMonRound1Points[*(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut()] as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestWinnerId() {
    let mut i: u8 = 0;
    while i < 4
        && (*(&raw const crate::contest::gContestFinalStandings)
            .cast::<CArray<u8, 4>>()
            .cast_mut())[i]
            != 0
    {
        i += 1;
    }
    gSpecialVar_0x8005 = i as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn BufferContestWinnerTrainerName() {
    let mut i: u8 = 0;
    while i < 4
        && (*(&raw const crate::contest::gContestFinalStandings)
            .cast::<CArray<u8, 4>>()
            .cast_mut())[i]
            != 0
    {
        i += 1;
    }
    StringCopy(
        gStringVar3.as_mut_ptr(),
        gContestMons[i].trainerName.as_mut_ptr(),
    );
    ConvertInternationalContestantName(gStringVar3.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe fn BufferContestWinnerMonName() {
    let mut i: u8 = 0;
    while i < 4
        && (*(&raw const crate::contest::gContestFinalStandings)
            .cast::<CArray<u8, 4>>()
            .cast_mut())[i]
            != 0
    {
        i += 1;
    }
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[i].nickname.as_mut_ptr(),
    );
}
pub unsafe fn CB2_SetStartContestCallback() {
    SetMainCallback2(Some(CB2_StartContest));
}
pub(crate) unsafe fn Task_StartContest(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        SetMainCallback2(Some(CB2_SetStartContestCallback));
    }
}
pub unsafe fn StartContest() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_StartContest), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
pub unsafe fn BufferContestantMonSpecies() {
    gSpecialVar_0x8004 = gContestMons[*(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut()]
    .species;
}
pub(crate) unsafe fn Task_StartShowContestResults(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        SetMainCallback2(Some(CB2_StartShowContestResults));
    }
}
pub unsafe fn ShowContestResults() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_StartShowContestResults), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestPlayerId() {
    gSpecialVar_0x8004 = gContestPlayerMonIndex as u16;
}
pub unsafe fn ContestLinkTransfer(category: u8) {
    LockPlayerFieldControls();
    let newTaskId: u8 = CreateTask(Some(Task_LinkContest_Init), 0);
    SetTaskFuncWithFollowupFunc(
        newTaskId,
        Some(Task_LinkContest_Init),
        Some(Task_StartCommunication),
    );
    task_set(newTaskId, 9, category as i16);
}
pub(crate) unsafe fn Task_StartCommunication(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER != 0 {
        CreateContestMonFromParty(gContestMonPartyIndex);
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateMonsRS),
            Some(Task_StartCommunicateRngRS),
        );
    } else {
        CreateContestMonFromParty(gContestMonPartyIndex);
        task_set_func(taskId, Some(Task_LinkContest_StartCommunicationEm));
    }
}
pub(crate) unsafe fn Task_StartCommunicateRngRS(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRngRS),
        Some(Task_StartCommunicateLeaderIdsRS),
    );
}
pub(crate) unsafe fn Task_StartCommunicateLeaderIdsRS(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateLeaderIdsRS),
        Some(Task_StartCommunicateCategoryRS),
    );
}
pub(crate) unsafe fn Task_StartCommunicateCategoryRS(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateCategoryRS),
        Some(Task_LinkContest_SetUpContestRS),
    );
}
pub(crate) unsafe fn Task_LinkContest_SetUpContestRS(taskId: u8) {
    let mut categories: CArray<u8, 4> = zeroed();
    let mut leaderIds: CArray<u8, 4> = zeroed();
    memset(categories.as_mut_ptr(), 0, 4);
    memset(leaderIds.as_mut_ptr(), 0, 4);
    for i in 0..gNumLinkContestPlayers {
        categories[i] = task_get(taskId, i as i32 + 1) as u8;
    }
    let mut i: u8 = 0;
    while i < gNumLinkContestPlayers && categories[0] == categories[i] {
        i += 1;
    }
    if i == gNumLinkContestPlayers {
        gSpecialVar_0x8004 = FALSE as u16;
    } else {
        gSpecialVar_0x8004 = TRUE as u16;
    }
    for i in 0..gNumLinkContestPlayers {
        leaderIds[i] = task_get(taskId, i as i32 + 5) as u8;
    }
    gContestLinkLeaderIndex = LinkContest_GetLeaderIndex(leaderIds.as_mut_ptr());
    CalculateRound1Points(gSpecialVar_ContestCategory as u8);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRound1Points),
        Some(Task_LinkContest_CalculateTurnOrderRS),
    );
}
pub(crate) unsafe fn Task_LinkContest_CalculateTurnOrderRS(taskId: u8) {
    SortContestants(FALSE);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateTurnOrder),
        Some(Task_LinkContest_FinalizeConnection),
    );
}
pub unsafe fn LinkContest_GetLeaderIndex(ids: *mut u8) -> u8 {
    let mut leaderIdx: u8 = 0;
    for i in 1..(gNumLinkContestPlayers as i32) {
        if *ids.at(leaderIdx) < *ids.at(i) {
            leaderIdx = i as u8;
        }
    }
    leaderIdx
}
pub unsafe fn Task_LinkContest_FinalizeConnection(taskId: u8) {
    if gSpecialVar_0x8004 == TRUE as u16 {
        if IsLinkTaskFinished() != 0 {
            task_set_func(taskId, Some(Task_LinkContest_Disconnect));
        }
    } else {
        for i in 0..CONTESTANT_COUNT {
            StringGet_Nickname(gContestMons[i].nickname.as_mut_ptr());
        }
        DestroyTask(taskId);
        SetDynamicWarp(
            0,
            (*gSaveBlock1Ptr).location.mapGroup,
            (*gSaveBlock1Ptr).location.mapNum,
            WARP_ID_NONE,
        );
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
pub(crate) unsafe fn Task_LinkContest_Disconnect(taskId: u8) {
    SetCloseLinkCallback();
    task_set_func(taskId, Some(Task_LinkContest_WaitDisconnect));
}
pub(crate) unsafe fn Task_LinkContest_WaitDisconnect(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        DestroyTask(taskId);
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetContestTrainerGfxIds() {
    (*gSaveBlock1Ptr).vars[16] = gContestMons[0].trainerGfxId as u16;
    (*gSaveBlock1Ptr).vars[17] = gContestMons[1].trainerGfxId as u16;
    (*gSaveBlock1Ptr).vars[18] = gContestMons[2].trainerGfxId as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn GetNpcContestantLocalId() {
    let mut localId: u16 = 0;
    let contestant: u8 = gSpecialVar_0x8005 as u8;
    match contestant {
        0 => {
            localId = LOCALID_CONTESTANT_1;
        }
        1 => {
            localId = LOCALID_CONTESTANT_2;
        }
        2 => {
            localId = LOCALID_CONTESTANT_3;
        }
        _ => {
            localId = 100;
        }
    }
    gSpecialVar_0x8004 = localId;
}
#[unsafe(no_mangle)]
pub unsafe fn BufferContestTrainerAndMonNames() {
    BufferContestantTrainerName();
    BufferContestantMonNickname();
    BufferContestantMonSpecies();
}
#[unsafe(no_mangle)]
pub unsafe fn DoesContestCategoryHaveMuseumPainting() {
    let mut contestWinner: i32 = 0;
    match gSpecialVar_ContestCategory {
        0 => {
            contestWinner = 8;
        }
        1 => {
            contestWinner = 9;
        }
        2 => {
            contestWinner = 10;
        }
        3 => {
            contestWinner = 11;
        }
        _ => {
            contestWinner = 12;
        }
    }
    if (*gSaveBlock1Ptr).contestWinners[contestWinner].species == SPECIES_NONE {
        gSpecialVar_0x8004 = FALSE as u16;
    } else {
        gSpecialVar_0x8004 = TRUE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SaveMuseumContestPainting() {
    SaveContestWinner(CONTEST_SAVE_FOR_MUSEUM as u8);
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldReadyContestArtist() {
    if gContestFinalStandings[gContestPlayerMonIndex] == 0
        && gSpecialVar_ContestRank == CONTEST_RANK_MASTER as u16
        && (*(&raw const crate::contest::gContestMonTotalPoints)
            .cast::<CArray<i16, 4>>()
            .cast_mut())[gContestPlayerMonIndex]
            >= 800
    {
        gSpecialVar_0x8004 = TRUE as u16;
    } else {
        gSpecialVar_0x8004 = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CountPlayerMuseumPaintings() -> u8 {
    let mut count: u8 = 0;
    for i in 0..5i32 {
        if (*gSaveBlock1Ptr).contestWinners[MUSEUM_CONTEST_WINNERS_START as i32 + i].species != 0 {
            count += 1;
        }
    }
    count
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestantNamesAtRank() {
    let mut conditions: CArray<i16, 4> = zeroed();
    let mut j: i32 = 0;
    for i in 0..CONTESTANT_COUNT {
        conditions[i] = gContestMonRound1Points[i];
    }
    let mut i: i32 = 0;
    while i < 3 {
        j = 3;
        while j > i {
            if conditions[j - 1] < conditions[j] {
                let temp: i32 = conditions[j] as i32;
                conditions[j] = conditions[j - 1];
                conditions[j - 1] = temp as i16;
            }
            j -= 1;
        }
        i += 1;
    }
    let condition: i16 = conditions[*(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut()];
    let mut numAtCondition: i8 = 0;
    let mut tieRank: u8 = 0;
    for i in 0..CONTESTANT_COUNT {
        if conditions[i] == condition {
            numAtCondition += 1;
            if i == gSpecialVar_0x8006 as i32 {
                tieRank = numAtCondition as u8;
            }
        }
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        if conditions[i] == condition {
            break;
        }
        i += 1;
    }
    let rank: u8 = i as u8;
    let mut contestantOffset: u8 = tieRank;
    i = 0;
    while i < CONTESTANT_COUNT {
        if condition == gContestMonRound1Points[i] {
            if contestantOffset == 1 {
                break;
            }
            contestantOffset -= 1;
        }
        i += 1;
    }
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[i].nickname.as_mut_ptr(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gContestMons[i].trainerName.as_mut_ptr(),
    );
    ConvertInternationalContestantName(gStringVar2.as_mut_ptr());
    if numAtCondition == 1 {
        gSpecialVar_0x8006 = rank as u16;
    } else if tieRank as i32 == numAtCondition as i32 {
        gSpecialVar_0x8006 = rank as u16;
    } else {
        gSpecialVar_0x8006 = rank as u16 + CONTESTANT_COUNT as u16;
    }
}
pub(crate) unsafe fn ExitContestPainting() {
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
#[unsafe(no_mangle)]
pub unsafe fn ShowContestPainting() {
    SetMainCallback2(Some(CB2_ContestPainting));
    gMain.savedCallback = Some(ExitContestPainting);
}
#[unsafe(no_mangle)]
pub unsafe fn SetLinkContestPlayerGfx() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        for i in 0..(gNumLinkContestPlayers as i32) {
            let version: i32 = gLinkPlayers[i].version as u8 as i32;
            if version == VERSION_RUBY || version == VERSION_SAPPHIRE {
                if gLinkPlayers[i].gender == MALE {
                    gContestMons[i].trainerGfxId = OBJ_EVENT_GFX_LINK_RS_BRENDAN;
                } else {
                    gContestMons[i].trainerGfxId = OBJ_EVENT_GFX_LINK_RS_MAY;
                }
            }
        }
        VarSet(VAR_OBJ_GFX_ID_0, gContestMons[0].trainerGfxId as u16);
        VarSet(VAR_OBJ_GFX_ID_1, gContestMons[1].trainerGfxId as u16);
        VarSet(VAR_OBJ_GFX_ID_2, gContestMons[2].trainerGfxId as u16);
        VarSet(VAR_OBJ_GFX_ID_3, gContestMons[3].trainerGfxId as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LoadLinkContestPlayerPalettes() {
    let mut i: i32 = 0;
    let mut objectEventId: u8 = 0;
    let mut version: i32 = 0;
    let mut sprite: *mut Sprite = null_mut();
    gReservedSpritePaletteCount = 12;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        i = 0;
        while i < gNumLinkContestPlayers as i32 {
            objectEventId = GetObjectEventIdByLocalIdAndMap(
                sContestantLocalIds_0[i],
                (*gSaveBlock1Ptr).location.mapNum as u8,
                (*gSaveBlock1Ptr).location.mapGroup as u8,
            );
            sprite = &raw mut gSprites[gObjectEvents[objectEventId].spriteId];
            (*sprite).oam.set_paletteNum(6 + i as u16);
            version = gLinkPlayers[i].version as u8 as i32;
            if version == VERSION_RUBY || version == VERSION_SAPPHIRE {
                if gLinkPlayers[i].gender == MALE {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_RubySapphireBrendan).cast::<CArray<u16, 0>>()).as_ptr().cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                } else {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_RubySapphireMay).cast::<CArray<u16, 0>>()).as_ptr().cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                }
            } else {
                if gLinkPlayers[i].gender == MALE {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_Brendan)
                            .cast::<CArray<u16, 0>>())
                        .as_ptr()
                        .cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                } else {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_May)
                            .cast::<CArray<u16, 0>>())
                        .as_ptr()
                        .cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                }
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GiveMonArtistRibbon() -> u8 {
    let mut hasArtistRibbon: u8 = GetMonData2(
        &raw mut gPlayerParty[gContestMonPartyIndex],
        MON_DATA_ARTIST_RIBBON,
    ) as u8;
    if hasArtistRibbon == 0
        && (*(&raw const crate::contest::gContestFinalStandings)
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gContestPlayerMonIndex]
            == 0
        && gSpecialVar_ContestRank == CONTEST_RANK_MASTER as u16
        && (*(&raw const crate::contest::gContestMonTotalPoints)
            .cast::<CArray<i16, 4>>()
            .cast_mut())[gContestPlayerMonIndex]
            >= 800
    {
        hasArtistRibbon = 1;
        SetMonData(
            &raw mut gPlayerParty[gContestMonPartyIndex],
            MON_DATA_ARTIST_RIBBON,
            &raw mut hasArtistRibbon as *mut c_void,
        );
        if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS {
            TryPutSpotTheCutiesOnAir(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_ARTIST_RIBBON as u8,
            );
        }
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub fn IsContestDebugActive() -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ShowContestEntryMonPic() {
    let mut palette: *mut CompressedSpritePalette = null_mut();
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    let mut species: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut taskId: u8 = 0;
    let mut left: u8 = 0;
    let mut top: u8 = 0;
    if FindTaskIdByFunc(Some(Task_ShowContestEntryMonPic)) == TASK_NONE {
        AllocateMonSpritesGfx();
        left = 10;
        top = 3;
        species = gContestMons[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
        .species;
        personality = gContestMons[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
        .personality;
        otId = gContestMons[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
        .otId;
        taskId = CreateTask(Some(Task_ShowContestEntryMonPic), 0x50);
        task_set(taskId, 0, 0);
        task_set(taskId, 1, species as i16);
        if gSpecialVar_0x8006 == gContestPlayerMonIndex as u16 {
            HandleLoadSpecialPokePic_2(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[1],
                species as i32,
                personality,
            );
        } else {
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[1],
                species as i32,
                personality,
            );
        }
        palette = GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
        LoadCompressedSpritePalette(palette);
        SetMultiuseSpriteTemplateToPokemon(species, B_POSITION_OPPONENT_LEFT);
        gMultiuseSpriteTemplate.paletteTag = (*palette).tag;
        spriteId = CreateSprite(
            &raw mut gMultiuseSpriteTemplate,
            (left as i16 + 1) * 8 + 32,
            top as i16 * 8 + 40,
            0,
        );
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER == 0 {
                DoMonFrontSpriteAnimation(&raw mut gSprites[spriteId], species, 0, 0);
            }
        } else {
            DoMonFrontSpriteAnimation(&raw mut gSprites[spriteId], species, 0, 0);
        }
        task_set(taskId, 2, spriteId as i16);
        task_set(taskId, 3, left as i16);
        task_set(taskId, 4, top as i16);
        gSprites[spriteId].callback = Some(SpriteCallbackDummy);
        gSprites[spriteId].oam.set_priority(0);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn HideContestEntryMonPic() {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_ShowContestEntryMonPic));
    if taskId != TASK_NONE {
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        FreeMonSpritesGfx();
    }
}
pub(crate) unsafe fn Task_ShowContestEntryMonPic(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let mut sprite: *mut Sprite = null_mut();
    match (*task).data[0] {
        0 => {
            (*task).data[0] += 1;
        }
        1 => {
            (*task).data[5] = CreateWindowFromRect(10, 3, 8, 8) as i16;
            SetStandardWindowBorderStyle((*task).data[5] as u8, TRUE);
            (*task).data[0] += 1;
        }
        2 => {}
        3 => {
            sprite = &raw mut gSprites[(*task).data[2]];
            FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
                (*sprite).oam.paletteNum() as u8
            ));
            if (*sprite).oam.affineMode() != 0 {
                FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            }
            DestroySprite(sprite);
            (*task).data[0] += 1;
        }
        4 => {
            ClearToTransparentAndRemoveWindow(task_get(taskId, 5) as u8);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetContestMultiplayerId() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0
        && gNumLinkContestPlayers == CONTESTANT_COUNT as u8
        && gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS == 0
    {
        gSpecialVar_Result = GetMultiplayerId() as u16;
    } else {
        gSpecialVar_Result = MAX_LINK_PLAYERS as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GenerateContestRand() {
    let mut random: u16 = 0;
    let mut result: *mut u16 = null_mut();
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        gContestRngValue = 0x41c64e6d * gContestRngValue + 24691;
        random = (gContestRngValue >> 16) as u16;
        result = &raw mut gSpecialVar_Result;
    } else {
        result = &raw mut gSpecialVar_Result;
        random = Random();
    }
    *result = rem_i32(random as i32, *result as i32) as u16;
}
pub unsafe fn GetContestRand() -> u16 {
    gContestRngValue = 0x41c64e6d * gContestRngValue + 24691;
    (gContestRngValue >> 16) as u16
}
#[unsafe(no_mangle)]
pub unsafe fn LinkContestWaitForConnection() -> u8 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        CreateTask(Some(Task_LinkContestWaitForConnection), 5);
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_LinkContestWaitForConnection(taskId: u8) {
    match task_get(taskId, 0) {
        0 => {
            if IsLinkTaskFinished() != 0 {
                SetLinkStandbyCallback();
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        1 => {
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        _ => {
            if IsLinkTaskFinished() == 1 {
                ScriptContext_Enable();
                DestroyTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LinkContestTryShowWirelessIndicator() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0
        && gReceivedRemoteLinkPlayers != 0
    {
        LoadWirelessStatusIndicatorSpriteGfx();
        CreateWirelessStatusIndicatorSprite(8, 8);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LinkContestTryHideWirelessIndicator() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0
        && gReceivedRemoteLinkPlayers != 0
    {
        DestroyWirelessStatusIndicatorSprite();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn IsContestWithRSPlayer() -> u8 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ClearLinkContestFlags() {
    gLinkContestFlags = 0;
}
#[unsafe(no_mangle)]
pub unsafe fn IsWirelessContest() -> u8 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
