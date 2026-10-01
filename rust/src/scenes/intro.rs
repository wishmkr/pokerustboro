//! Translated from `src/intro.c` by tools/rustport/c2rs.py.
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
    clippy::disallowed_names,
    clippy::eq_op,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetSerialCallback, SetVBlankCallback};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gcn_multiboot::GameCubeMultiBoot_Quit;
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::intro_credits_graphics::{
    CreateBicycleBgAnimationTask, CreateIntroBrendanSprite, CreateIntroFlygonSprite,
    CreateIntroMaySprite, CycleSceneryPalette, LoadIntroPart2Graphics, SetIntroPart2BgCnt,
    gIntroCredits_MovingSceneryState, gIntroCredits_MovingSceneryVBase,
    gIntroCredits_MovingSceneryVOffset,
};
use crate::link::{ResetSerial, SerialCB};
use crate::load_save::SetSaveBlocksPointers;
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::{SetPokemonCryStereo, m4aSongNumStart};
use crate::new_game::{ResetMenuAndMonGlobals, Sav2_ClearSetDefault};
use crate::palette::gPlttBufferUnfaded;
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::random::Random;
use crate::save::{
    GetSaveBlocksPointersBaseOffset, LoadGameSave, Save_ResetSaveCounters, gSaveFileStatus,
};
use crate::scanline_effect::{
    ScanlineEffect_InitHBlankDmaTransfer, ScanlineEffect_InitWave, ScanlineEffect_Stop,
};
use crate::sound::{PlayCryInternal, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData, SetOamMatrix, gReservedSpritePaletteCount,
};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::title_screen::CB2_InitTitleScreen;
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CalcCenterToCornerVec` with this module's view of its types.
#[inline]
unsafe fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8) {
    unsafe {
        crate::sprite::CalcCenterToCornerVec(a0 as _, a1, a2, a3);
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
/// `GameCubeMultiBoot_ExecuteProgram` with this module's view of its types.
#[inline]
unsafe fn GameCubeMultiBoot_ExecuteProgram(a0: *mut GcmbStruct) {
    unsafe {
        crate::gcn_multiboot::GameCubeMultiBoot_ExecuteProgram(a0 as _);
    }
}
/// `GameCubeMultiBoot_HandleSerialInterrupt` with this module's view of its types.
#[inline]
unsafe fn GameCubeMultiBoot_HandleSerialInterrupt(a0: *mut GcmbStruct) {
    unsafe {
        crate::gcn_multiboot::GameCubeMultiBoot_HandleSerialInterrupt(a0 as _);
    }
}
/// `GameCubeMultiBoot_Init` with this module's view of its types.
#[inline]
unsafe fn GameCubeMultiBoot_Init(a0: *mut GcmbStruct) {
    unsafe {
        crate::gcn_multiboot::GameCubeMultiBoot_Init(a0 as _);
    }
}
/// `GameCubeMultiBoot_Main` with this module's view of its types.
#[inline]
unsafe fn GameCubeMultiBoot_Main(a0: *mut GcmbStruct) {
    unsafe {
        crate::gcn_multiboot::GameCubeMultiBoot_Main(a0 as _);
    }
}
/// `InitHeap` with this module's view of its types.
#[inline]
unsafe fn InitHeap(a0: *mut c_void, a1: u32) {
    unsafe {
        crate::malloc::InitHeap(a0 as _, a1);
    }
}
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePaletteUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePaletteUsingHeap(a0: *mut CompressedSpritePalette) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpritePaletteUsingHeap(a0 as _) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sBigDropSpriteId: usize = 0;
const sState: usize = 0;
const tAlpha: usize = 0;
const tBgAnimTaskId: usize = 0;
const tState: usize = 0;
const tWinPos: usize = 0;
const sMoveTimer: usize = 1;
const sPalIdx: usize = 1;
const sRockId: usize = 1;
const sScale: usize = 1;
const sSinIdx: usize = 1;
const sStateDelay: usize = 1;
const tBg2PosHi: usize = 1;
const tPlayerSpriteId: usize = 1;
const tScreenX: usize = 1;
const tZoomDiv: usize = 1;
const sBaseY: usize = 2;
const sCosIdx: usize = 2;
const sLetterId: usize = 2;
const sNextState: usize = 2;
const sRot: usize = 2;
const sSpeed: usize = 2;
const tBg2PosLo: usize = 2;
const tFlygonSpriteId: usize = 2;
const tScreenY: usize = 2;
const tZoomDivSpeed: usize = 2;
const sCosYIdx: usize = 3;
const sPos: usize = 3;
const tBg1PosHi: usize = 3;
const tFlygonTimer: usize = 3;
const tZoom: usize = 3;
const sSinXIdx: usize = 4;
const tBg1PosLo: usize = 4;
const sSinYIdx: usize = 5;
const tBg3PosHi: usize = 5;
const sFig8Loops: usize = 6;
const tBg3PosLo: usize = 6;
const tCloudPos: usize = 6;
const sUnk: usize = 7;
// Data tables (translate with cdata.py): sIntroDrops_Pal sIntroLogo_Pal sIntroDropsLogo_Gfx sIntro1Bg_Pal sIntro1Bg0_Tilemap sIntro1Bg1_Tilemap sIntro1Bg2_Tilemap sIntro1Bg3_Tilemap sIntro1Bg_Gfx sIntroPokeball_Pal sIntroPokeball_Tilemap sIntroPokeball_Gfx sIntroStreaks_Pal sIntroStreaks_Gfx sIntroStreaks_Tilemap sIntroRayquzaOrb_Pal sIntroMisc_Pal sIntroMisc_Gfx sIntroFlygonSilhouette_Pal sIntroLati_Gfx sUnusedData sSpriteSheet_Sparkle sSpritePalette_Sparkle sOamData_Sparkle sAnim_Sparkle sAnims_Sparkle sSpriteTemplate_Sparkle sSparkleCoords sSpriteSheet_RunningPokemon sSpritePalettes_RunningPokemon sOamData_Volbeat sAnim_Volbeat sAnims_Volbeat sSpriteTemplate_Volbeat sOamData_Torchic sAnim_Torchic_Walk sAnim_Torchic_Run sAnim_Torchic_Trip sAnims_Torchic sSpriteTemplate_Torchic sOamData_Manectric sAnim_Manectric sAnims_Manectric sSpriteTemplate_Manectric sSpriteSheet_Lightning sSpritePalette_Lightning sOamData_Lightning sAnim_Lightning_Top sAnim_Lightning_Middle sAnim_Lightning_Bottom sAnims_Lightning sSpriteTemplate_Lightning sGroudonRockData sSpriteSheet_Bubbles sSpritePalette_Bubbles sKyogreBubbleData sOamData_Bubbles sAnim_Bubbles sAnims_Bubbles sSpriteTemplate_Bubbles sOamData_WaterDrop sAnim_WaterDrop_UpperHalf sAnim_WaterDrop_LowerHalf sAnim_WaterDrop_Reflection sAnim_WaterDrop_Ripple sAnims_WaterDrop sSpriteTemplate_WaterDrop sAnim_PlayerBicycle_Fast sAnim_PlayerBicycle_Slow sAnim_PlayerBicycle_LookBack sAnim_PlayerBicycle_LookForward sAnims_PlayerBicycle sOamData_GameFreakLetter sOamData_PresentsLetter sOamData_GameFreakLogo sAnim_GameFreakLetter_G sAnim_GameFreakLetter_A sAnim_GameFreakLetter_M sAnim_GameFreakLetter_E sAnim_GameFreakLetter_F sAnim_GameFreakLetter_R sAnim_GameFreakLetter_K sAnim_PresentsLetter_P sAnim_PresentsLetter_R sAnim_PresentsLetter_E sAnim_PresentsLetter_S sAnim_PresentsLetter_N sAnim_PresentsLetter_T sAnim_GameFreakLogo sAnims_GameFreakLetter sAnims_PresentsLetter sAnims_GameFreakLogo sGameFreakLetterData sPresentsLetterData sAffineAnim_GameFreak_Small sAffineAnim_GameFreak_GrowAndShrink sAffineAnim_GameFreak_GrowBig sAffineAnim_GameFreak_GrowMedium sAffineAnims_GameFreak sGameFreakLettersMoveSpeed sSpriteTemplate_GameFreakLetter sSpriteTemplate_PresentsLetter sSpriteTemplate_GameFreakLogo sGameFreakLetterStartDelays sOamData_FlygonSilhouette sAnim_FlygonSilhouette sAnims_FlygonSilhouette sSpriteTemplate_FlygonSilhouette sSpriteSheet_WaterDropsAndLogo sSpriteSheet_FlygonSilhouette sSpritePalettes_Intro1 sOamData_RayquazaOrb sAnim_RayquazaOrb sAnims_RayquazaOrb sSpriteTemplate_RayquazaOrb sSpriteSheet_RayquazaOrb sSpritePalette_RayquazaOrb

const COLOR_CHANGES: i16 = 9;
const COLOSSEUM_GAME_CODE: u32 = 0x65366347;
const COPYRIGHT_INITIALIZE: u8 = 0;
const COPYRIGHT_START_FADE: u8 = 140;
const COPYRIGHT_START_INTRO: u8 = 141;
const DROP_ANIM_LOWER_HALF: u8 = 1;
const DROP_ANIM_REFLECTION: u8 = 2;
const DROP_ANIM_RIPPLE: u8 = 3;
const NARROW_HEIGHT: i16 = 32;
const NUM_BUBBLES_IN_SET: i32 = 6;
const NUM_GF_LETTERS: u16 = 9;
const TIMER_BIG_DROP_FALLS: u32 = 251;
const TIMER_BIG_DROP_START: u32 = 76;
const TIMER_END_PAN_UP: u32 = 904;
const TIMER_END_SCENE_1: u32 = 1007;
const TIMER_END_SCENE_2: u32 = 1946;
const TIMER_FLYGON_ENTER: u32 = 1394;
const TIMER_FLYGON_SILHOUETTE_APPEAR: u32 = 832;
const TIMER_LOGO_APPEAR: u32 = 128;
const TIMER_LOGO_BLEND_OUT: u32 = 256;
const TIMER_LOGO_DISAPPEAR: u32 = 272;
const TIMER_LOGO_LETTERS_COLOR: u32 = 144;
const TIMER_MANECTRIC_ENTER: u32 = 1088;
const TIMER_MANECTRIC_RUN_CIRCULAR: u32 = 1168;
const TIMER_PLAYER_DRIFT_BACK: u32 = 1109;
const TIMER_PLAYER_EXIT: u32 = 1727;
const TIMER_PLAYER_HOLD_POSITION: u32 = 1576;
const TIMER_PLAYER_MOVE_BACKWARD: u32 = 1398;
const TIMER_PLAYER_MOVE_FORWARD: u32 = 1214;
const TIMER_POKEBALL_FADE: u32 = 28;
const TIMER_SMALL_DROP_1: u32 = 368;
const TIMER_SMALL_DROP_2: u32 = 384;
const TIMER_SPARKLES: u32 = 560;
const TIMER_START_LEGENDARIES: u32 = 43;
const TIMER_START_SCENE_2: u32 = 1026;
const TIMER_START_SCENE_3: u32 = 2068;
const TIMER_TORCHIC_ENTER: u32 = 1224;
const TIMER_TORCHIC_EXIT: u32 = 1856;
const TIMER_TORCHIC_SPEED_UP: u32 = 1735;
const TORCHIC_ANIM_RUN: u8 = 1;
const TORCHIC_ANIM_TRIP: u8 = 2;
const TORCHIC_ANIM_WALK: u8 = 0;
const VOLBEAT_ENTER: i16 = 1;
const VOLBEAT_EXIT: i16 = 7;
const VOLBEAT_FIGURE_8: i16 = 6;
const VOLBEAT_INIT_FIGURE_8: i16 = 5;
const VOLBEAT_WAIT_ENTER: i16 = 0;
const VOLBEAT_WAIT_STATE: i16 = 8;
const VOLBEAT_ZIP_BACKWARD: i16 = 2;
const VOLBEAT_ZIP_DOWN: i16 = 3;
const VOLBEAT_ZIP_FORWARD: i16 = 4;

static sAnims_PlayerBicycle: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::intro::sAnims_PlayerBicycle).cast());
static sGameFreakLetterData: Table<CArray<CArray<i16, 2>, 9>> =
    Table((&raw const crate::data::intro::sGameFreakLetterData).cast());
static sGameFreakLetterStartDelays: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::intro::sGameFreakLetterStartDelays).cast());
static sGameFreakLettersMoveSpeed: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::intro::sGameFreakLettersMoveSpeed).cast());
static sGroudonRockData: Table<CArray<CArray<i16, 3>, 6>> =
    Table((&raw const crate::data::intro::sGroudonRockData).cast());
static sIntro1Bg0_Tilemap: Table<CArray<u32, 237>> =
    Table((&raw const crate::data::intro::sIntro1Bg0_Tilemap).cast());
static sIntro1Bg1_Tilemap: Table<CArray<u32, 205>> =
    Table((&raw const crate::data::intro::sIntro1Bg1_Tilemap).cast());
static sIntro1Bg2_Tilemap: Table<CArray<u32, 188>> =
    Table((&raw const crate::data::intro::sIntro1Bg2_Tilemap).cast());
static sIntro1Bg3_Tilemap: Table<CArray<u32, 134>> =
    Table((&raw const crate::data::intro::sIntro1Bg3_Tilemap).cast());
static sIntro1Bg_Gfx: Table<CArray<u32, 2140>> =
    Table((&raw const crate::data::intro::sIntro1Bg_Gfx).cast());
static sIntro1Bg_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::intro::sIntro1Bg_Pal).cast());
static sIntroPokeball_Gfx: Table<CArray<u32, 725>> =
    Table((&raw const crate::data::intro::sIntroPokeball_Gfx).cast());
static sIntroPokeball_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::intro::sIntroPokeball_Pal).cast());
static sIntroPokeball_Tilemap: Table<CArray<u32, 76>> =
    Table((&raw const crate::data::intro::sIntroPokeball_Tilemap).cast());
static sKyogreBubbleData: Table<CArray<CArray<i16, 3>, 12>> =
    Table((&raw const crate::data::intro::sKyogreBubbleData).cast());
static sSparkleCoords: Table<CArray<CArray<u8, 2>, 12>> =
    Table((&raw const crate::data::intro::sSparkleCoords).cast());
static sSpritePalette_Bubbles: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_Bubbles).cast());
static sSpritePalette_Lightning: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_Lightning).cast());
static sSpritePalette_RayquazaOrb: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_RayquazaOrb).cast());
static sSpritePalette_Sparkle: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_Sparkle).cast());
static sSpritePalettes_Intro1: Table<CArray<SpritePalette, 4>> =
    Table((&raw const crate::data::intro::sSpritePalettes_Intro1).cast());
static sSpritePalettes_RunningPokemon: Table<CArray<SpritePalette, 4>> =
    Table((&raw const crate::data::intro::sSpritePalettes_RunningPokemon).cast());
static sSpriteSheet_Bubbles: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_Bubbles).cast());
static sSpriteSheet_FlygonSilhouette: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_FlygonSilhouette).cast());
static sSpriteSheet_Lightning: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_Lightning).cast());
static sSpriteSheet_RayquazaOrb: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_RayquazaOrb).cast());
static sSpriteSheet_RunningPokemon: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::intro::sSpriteSheet_RunningPokemon).cast());
static sSpriteSheet_Sparkle: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_Sparkle).cast());
static sSpriteSheet_WaterDropsAndLogo: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_WaterDropsAndLogo).cast());
static sSpriteTemplate_Bubbles: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Bubbles).cast());
static sSpriteTemplate_FlygonSilhouette: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_FlygonSilhouette).cast());
static sSpriteTemplate_GameFreakLetter: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_GameFreakLetter).cast());
static sSpriteTemplate_GameFreakLogo: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_GameFreakLogo).cast());
static sSpriteTemplate_Lightning: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Lightning).cast());
static sSpriteTemplate_Manectric: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Manectric).cast());
static sSpriteTemplate_RayquazaOrb: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_RayquazaOrb).cast());
static sSpriteTemplate_Sparkle: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Sparkle).cast());
static sSpriteTemplate_Torchic: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Torchic).cast());
static sSpriteTemplate_Volbeat: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Volbeat).cast());
static sSpriteTemplate_WaterDrop: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_WaterDrop).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sIntroCharacterGender: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sUnusedVar: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sFlygonYOffset: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gIntroFrameCounter: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gMultibootProgramStruct: GcmbStruct = unsafe { zeroed() };

/// `BgAffineSet` with this module's view of its types.
#[inline]
unsafe fn BgAffineSet(a0: *mut BgAffineSrcData, a1: *mut BgAffineDstData, a2: i32) {
    unsafe {
        crate::syscall::BgAffineSet(a0 as _, a1 as _, a2);
    }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn VBlankCB_Intro() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe fn MainCB2_Intro() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    if gMain.newKeys != 0 && gPaletteFade.active() == 0 {
        SetMainCallback2(Some(MainCB2_EndIntro));
    } else {
        gIntroFrameCounter.set(gIntroFrameCounter.get().saturating_add(1));
    }
}
pub(crate) unsafe fn MainCB2_EndIntro() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2(Some(CB2_InitTitleScreen));
    }
}
unsafe fn LoadCopyrightGraphics(tilesetAddress: u16, tilemapAddress: u16, paletteOffset: u16) {
    LZ77UnCompVram(
        (*(&raw const crate::data::graphics::gIntroCopyright_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (VRAM + tilesetAddress as i32) as usize as *mut c_void,
    );
    LZ77UnCompVram(
        (*(&raw const crate::data::graphics::gIntroCopyright_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (VRAM + tilemapAddress as i32) as usize as *mut c_void,
    );
    LoadPalette(
        (*(&raw const crate::data::graphics::gIntroCopyright_Pal).cast::<CArray<u16, 16>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        paletteOffset,
        32,
    );
}
pub(crate) unsafe fn SerialCB_CopyrightScreen() {
    GameCubeMultiBoot_HandleSerialInterrupt(&raw mut gMultibootProgramStruct);
}
unsafe fn SetUpCopyrightScreen() -> u8 {
    'l1: {
        let sw1: u8 = gMain.state;
        let matched = sw1 == COPYRIGHT_INITIALIZE
            || sw1 == COPYRIGHT_START_FADE
            || sw1 == COPYRIGHT_START_INTRO;
        let mut fall = false;
        if sw1 == COPYRIGHT_INITIALIZE {
            fall = true;
            SetVBlankCallback(None);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            *(PLTT as i32 as usize as *mut u16) = 32767;
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BG0HOFS, 0);
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
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
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        OAM as i32 as usize as *mut c_void,
                        0x5000100,
                    );
                }
            }
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        83886082_usize as *mut c_void,
                        0x10001ff,
                    );
                }
            }
            ResetPaletteFade();
            LoadCopyrightGraphics(0, 0x3800, 0);
            ScanlineEffect_Stop();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
            SetGpuReg(REG_OFFSET_BG0CNT, 1792);
            EnableInterrupts(INTR_FLAG_VBLANK);
            SetVBlankCallback(Some(VBlankCB_Intro));
            volatile_write(0x4000000_usize as *mut u16, 320);
            SetSerialCallback(Some(SerialCB_CopyrightScreen));
            GameCubeMultiBoot_Init(&raw mut gMultibootProgramStruct);
        }
        if fall || !matched {
            UpdatePaletteFade();
            gMain.state += 1;
            GameCubeMultiBoot_Main(&raw mut gMultibootProgramStruct);
            break 'l1;
        }
        if sw1 == COPYRIGHT_START_FADE {
            GameCubeMultiBoot_Main(&raw mut gMultibootProgramStruct);
            if (&raw mut gMultibootProgramStruct.gcmb_field_2).read_volatile() != 1 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == COPYRIGHT_START_INTRO {
            if UpdatePaletteFade() != 0 {
                break 'l1;
            }
            CreateTask(Some(Task_Scene1_Load), 0);
            SetMainCallback2(Some(MainCB2_Intro));
            if (&raw mut gMultibootProgramStruct.gcmb_field_2).read_volatile() != 0 {
                if (&raw mut gMultibootProgramStruct.gcmb_field_2).read_volatile() == 2 {
                    if *(33554604_usize as *mut u32) == COLOSSEUM_GAME_CODE {
                        CpuSet(
                            (&raw const (*crate::asmdata::gMultiBootProgram_PokemonColosseum_Start
                                .cast::<CArray<u16, 81920>>()))
                                .cast_mut() as *mut c_void,
                            EWRAM_START as i32 as usize as *mut c_void,
                            0x14000,
                        );
                        *(33554604_usize as *mut u32) = COLOSSEUM_GAME_CODE;
                    }
                    GameCubeMultiBoot_ExecuteProgram(&raw mut gMultibootProgramStruct);
                }
            } else {
                GameCubeMultiBoot_Quit();
                SetSerialCallback(Some(SerialCB));
            }
            return 0;
        }
    }
    1
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_InitCopyrightScreenAfterBootup() {
    if SetUpCopyrightScreen() == 0 {
        SetSaveBlocksPointers(GetSaveBlocksPointersBaseOffset());
        ResetMenuAndMonGlobals();
        Save_ResetSaveCounters();
        LoadGameSave(SAVE_NORMAL);
        if gSaveFileStatus == SAVE_STATUS_EMPTY as u16 || gSaveFileStatus == SAVE_STATUS_CORRUPT {
            Sav2_ClearSetDefault();
        }
        SetPokemonCryStereo((*gSaveBlock2Ptr).optionsSound() as u32);
        InitHeap(
            (*(&raw const crate::malloc::gHeap)
                .cast::<CArray<u8, 114688>>()
                .cast_mut())
            .as_mut_ptr() as *mut c_void,
            HEAP_SIZE,
        );
    }
}
pub unsafe fn CB2_InitCopyrightScreenAfterTitleScreen() {
    SetUpCopyrightScreen();
}
pub(crate) unsafe fn Task_Scene1_Load(taskId: u8) {
    SetVBlankCallback(None);
    sIntroCharacterGender.set(
        (if 0 != 0 {
            Random() as i32 % 2
        } else {
            Random() as i32 & 1
        }) as u16,
    );
    IntroResetGpuRegs();
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 80);
    SetGpuReg(REG_OFFSET_BG1VOFS, 24);
    SetGpuReg(REG_OFFSET_BG0VOFS, 40);
    LZ77UnCompVram(
        sIntro1Bg_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sIntro1Bg0_Tilemap.as_ptr().cast_mut(),
        0x6008000_usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x6008800_usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        sIntro1Bg1_Tilemap.as_ptr().cast_mut(),
        0x6009000_usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x6009800_usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        sIntro1Bg2_Tilemap.as_ptr().cast_mut(),
        0x600a000_usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x600a800_usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        sIntro1Bg3_Tilemap.as_ptr().cast_mut(),
        0x600b000_usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x600b800_usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LoadPalette(sIntro1Bg_Pal.as_ptr().cast_mut() as *mut c_void, 0, 512);
    SetGpuReg(REG_OFFSET_BG3CNT, 38403);
    SetGpuReg(REG_OFFSET_BG2CNT, 37890);
    SetGpuReg(REG_OFFSET_BG1CNT, 37377);
    SetGpuReg(REG_OFFSET_BG0CNT, 36864);
    LoadCompressedSpriteSheet(sSpriteSheet_WaterDropsAndLogo.as_ptr().cast_mut());
    LoadCompressedSpriteSheet(sSpriteSheet_FlygonSilhouette.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalettes_Intro1.as_ptr().cast_mut());
    LoadCompressedSpriteSheet(sSpriteSheet_Sparkle.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalette_Sparkle.as_ptr().cast_mut());
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[496] as *mut c_void,
        16,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[481] as *mut c_void,
        15,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[466] as *mut c_void,
        14,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[451] as *mut c_void,
        13,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[436] as *mut c_void,
        12,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[421] as *mut c_void,
        11,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[256] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[406] as *mut c_void,
        10,
    );
    CreateGameFreakLogoSprites(120, 80, 0);
    task_set(
        taskId,
        sBigDropSpriteId,
        CreateWaterDrop(236, -14, 0x200, 1, 0x78, FALSE) as i16,
    );
    task_set_func(taskId, Some(Task_Scene1_FadeIn));
}
pub(crate) unsafe fn Task_Scene1_FadeIn(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    SetVBlankCallback(Some(VBlankCB_Intro));
    SetGpuReg(0x0, 8000);
    task_set_func(taskId, Some(Task_Scene1_WaterDrops));
    gIntroFrameCounter.set(0);
    m4aSongNumStart(MUS_INTRO);
    ResetSerial();
}
pub(crate) unsafe fn Task_Scene1_WaterDrops(taskId: u8) {
    if gIntroFrameCounter.get() == TIMER_BIG_DROP_START {
        gSprites[task_get(taskId, 0)].data[0] = 1;
    }
    if gIntroFrameCounter.get() == TIMER_LOGO_APPEAR {
        CreateTask(Some(Task_BlendLogoIn), 0);
    }
    if gIntroFrameCounter.get() == TIMER_BIG_DROP_FALLS {
        gSprites[task_get(taskId, 0)].data[0] = 2;
    }
    if gIntroFrameCounter.get() == TIMER_LOGO_BLEND_OUT {
        CreateTask(Some(Task_BlendLogoOut), 0);
    }
    if gIntroFrameCounter.get() == TIMER_SMALL_DROP_1 {
        CreateWaterDrop(48, 0, 0x400, 5, 0x70, TRUE);
    }
    if gIntroFrameCounter.get() == TIMER_SMALL_DROP_2 {
        CreateWaterDrop(200, 60, 0x400, 9, 0x80, TRUE);
    }
    if gIntroFrameCounter.get() == TIMER_SPARKLES {
        CreateTask(Some(Task_CreateSparkles), 0);
    }
    if gIntroFrameCounter.get() > TIMER_SPARKLES {
        task_set(taskId, tBg2PosHi, 80);
        task_set(taskId, tBg2PosLo, 0);
        task_set(taskId, tBg1PosHi, 24);
        task_set(taskId, tBg1PosLo, 0);
        task_set(taskId, tBg3PosHi, 40);
        task_set(taskId, tBg3PosLo, 0);
        task_set_func(taskId, Some(Task_Scene1_PanUp));
    }
}
pub(crate) unsafe fn Task_CreateSparkles(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if ({
        *data.at(2) += 1;
        *data.at(2)
    }) as i32
        & 1
        != 0
    {
        *data.at(3) += 1;
    }
    match *data {
        0 => {
            CreateSprite(
                (&raw const *sSpriteTemplate_Sparkle).cast_mut(),
                sSparkleCoords[*data.at(4)][0] as i16,
                sSparkleCoords[*data.at(4)][1] as i16 + *data.at(3),
                0,
            );
            *data += 1;
            *data.at(1) = 12;
            *data.at(4) += 1;
        }
        1 if ({
            *data.at(1) -= 1;
            *data.at(1)
        }) == 0 =>
        {
            *data = 0;
        }
        _ => {}
    }
    if *data.at(3) > 60 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn SpriteCB_Sparkle(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 12
    {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn Task_Scene1_PanUp(taskId: u8) {
    if gIntroFrameCounter.get() < TIMER_END_PAN_UP {
        let mut offset: i32 = ((task_get(taskId, tBg2PosHi) as i32) << 16)
            + task_get(taskId, tBg2PosLo) as u16 as i32;
        offset -= 0x6000;
        task_set(taskId, tBg2PosHi, (offset >> 16) as i16);
        task_set(taskId, tBg2PosLo, offset as i16);
        SetGpuReg(REG_OFFSET_BG2VOFS, task_get(taskId, tBg2PosHi) as u16);
        offset = ((task_get(taskId, tBg1PosHi) as i32) << 16)
            + task_get(taskId, tBg1PosLo) as u16 as i32;
        offset -= 0x8000;
        task_set(taskId, tBg1PosHi, (offset >> 16) as i16);
        task_set(taskId, tBg1PosLo, offset as i16);
        SetGpuReg(REG_OFFSET_BG1VOFS, task_get(taskId, tBg1PosHi) as u16);
        offset = ((task_get(taskId, tBg3PosHi) as i32) << 16)
            + task_get(taskId, tBg3PosLo) as u16 as i32;
        offset -= 0xC000;
        task_set(taskId, tBg3PosHi, (offset >> 16) as i16);
        task_set(taskId, tBg3PosLo, offset as i16);
        SetGpuReg(REG_OFFSET_BG0VOFS, task_get(taskId, tBg3PosHi) as u16);
        if gIntroFrameCounter.get() == TIMER_FLYGON_SILHOUETTE_APPEAR {
            let spriteId: u8 = CreateSprite(
                (&raw const *sSpriteTemplate_FlygonSilhouette).cast_mut(),
                120,
                DISPLAY_HEIGHT as i16,
                10,
            );
            gSprites[spriteId].set_invisible(TRUE as u16);
        }
    } else {
        if gIntroFrameCounter.get() > TIMER_END_SCENE_1 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 65535);
            task_set_func(taskId, Some(Task_Scene1_End));
        }
    }
}
pub(crate) fn Task_Scene1_End(taskId: u8) {
    if gIntroFrameCounter.get() > TIMER_START_SCENE_2 {
        task_set_func(taskId, Some(Task_Scene2_Load));
    }
}
pub(crate) unsafe fn Task_Scene2_Load(taskId: u8) {
    IntroResetGpuRegs();
    SetVBlankCallback(None);
    ResetSpriteData();
    FreeAllSpritePalettes();
    gIntroCredits_MovingSceneryVBase = 0;
    gIntroCredits_MovingSceneryVOffset = 0;
    sFlygonYOffset.set(0);
    LoadIntroPart2Graphics(1);
    task_set_func(taskId, Some(Task_Scene2_CreateSprites));
}
pub(crate) unsafe fn Task_Scene2_CreateSprites(taskId: u8) {
    if sIntroCharacterGender.get() == MALE as u16 {
        LoadCompressedSpriteSheet(
            (*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_IntroBrendan)
                .cast::<CArray<CompressedSpriteSheet, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else {
        LoadCompressedSpriteSheet(
            (*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_IntroMay)
                .cast::<CArray<CompressedSpriteSheet, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    LoadCompressedSpriteSheet(
        (*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_IntroBicycle)
            .cast::<CArray<CompressedSpriteSheet, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    LoadCompressedSpriteSheet(
        (*(&raw const crate::data::intro_credits_graphics::gSpriteSheet_IntroFlygon)
            .cast::<CArray<CompressedSpriteSheet, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    let mut spriteId: u8 = 0;
    while spriteId < 3 {
        LoadCompressedSpriteSheet((&raw const sSpriteSheet_RunningPokemon[spriteId]).cast_mut());
        spriteId += 1;
    }
    LoadSpritePalettes(
        (*(&raw const crate::data::intro_credits_graphics::gSpritePalettes_IntroPlayerFlygon)
            .cast::<CArray<SpritePalette, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    LoadSpritePalettes(sSpritePalettes_RunningPokemon.as_ptr().cast_mut());
    CreateSprite(
        (&raw const *sSpriteTemplate_Manectric).cast_mut(),
        272,
        128,
        0,
    );
    CreateSprite(
        (&raw const *sSpriteTemplate_Torchic).cast_mut(),
        288,
        110,
        1,
    );
    if sIntroCharacterGender.get() == MALE as u16 {
        spriteId = CreateIntroBrendanSprite(272, 100);
    } else {
        spriteId = CreateIntroMaySprite(272, 100);
    }
    gSprites[spriteId].callback = Some(SpriteCB_PlayerOnBicycle);
    gSprites[spriteId].anims = sAnims_PlayerBicycle.as_ptr().cast_mut();
    task_set(taskId, tPlayerSpriteId, spriteId as i16);
    CreateSprite((&raw const *sSpriteTemplate_Volbeat).cast_mut(), 272, 80, 4);
    spriteId = CreateIntroFlygonSprite(-64, 60);
    gSprites[spriteId].callback = Some(SpriteCB_Flygon);
    task_set(taskId, tFlygonSpriteId, spriteId as i16);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
    SetVBlankCallback(Some(VBlankCB_Intro));
    task_set(
        taskId,
        tBgAnimTaskId,
        CreateBicycleBgAnimationTask(1, 0x4000, 0x400, 0x10) as i16,
    );
    SetIntroPart2BgCnt(1);
    task_set_func(taskId, Some(Task_Scene2_BikeRide));
}
pub(crate) unsafe fn Task_Scene2_BikeRide(taskId: u8) {
    if gIntroFrameCounter.get() == TIMER_TORCHIC_EXIT {
        gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_FROZEN;
        DestroyTask(task_get(taskId, 0) as u8);
    }
    if gIntroFrameCounter.get() > TIMER_END_SCENE_2 {
        BeginNormalPaletteFade(PALETTES_ALL, 8, 0, 16, 65535);
        task_set_func(taskId, Some(Task_Scene2_End));
    }
    if gIntroFrameCounter.get() == TIMER_PLAYER_DRIFT_BACK {
        gSprites[task_get(taskId, tPlayerSpriteId)].data[0] = 1;
    }
    if gIntroFrameCounter.get() == TIMER_PLAYER_MOVE_FORWARD {
        gSprites[task_get(taskId, tPlayerSpriteId)].data[0] = 0;
    }
    if gIntroFrameCounter.get() == TIMER_FLYGON_ENTER {
        gSprites[task_get(taskId, tFlygonSpriteId)].data[0] = 1;
    }
    if gIntroFrameCounter.get() == TIMER_PLAYER_MOVE_BACKWARD {
        gSprites[task_get(taskId, tPlayerSpriteId)].data[0] = 2;
    }
    if gIntroFrameCounter.get() == TIMER_PLAYER_HOLD_POSITION {
        gSprites[task_get(taskId, tPlayerSpriteId)].data[0] = 3;
    }
    if gIntroFrameCounter.get() == TIMER_PLAYER_EXIT {
        gSprites[task_get(taskId, tPlayerSpriteId)].data[0] = 4;
    }
    let offset: u16 = Sin(task_get(taskId, tFlygonTimer) >> 2 & 0x7F, 48) as u16;
    sFlygonYOffset.set(offset);
    if task_get(taskId, tFlygonTimer) < 512 {
        task_set(taskId, tFlygonTimer, task_get(taskId, tFlygonTimer) + 1);
    }
    CycleSceneryPalette(0);
}
pub(crate) fn Task_Scene2_End(taskId: u8) {
    if gIntroFrameCounter.get() > TIMER_START_SCENE_3 {
        task_set_func(taskId, Some(Task_Scene3_Load));
    }
}
pub(crate) unsafe fn SpriteCB_Volbeat(sprite: *mut Sprite) {
    (*sprite).data[sCosYIdx] += 4;
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == VOLBEAT_WAIT_ENTER {
            fall = true;
            if ({
                (*sprite).data[sStateDelay] += 1;
                (*sprite).data[sStateDelay]
            }) < 180
            {
                break 'l1;
            }
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == VOLBEAT_ENTER {
            (*sprite).x -= 4;
            if (*sprite).x == 60 {
                (*sprite).data[sState] = VOLBEAT_WAIT_STATE;
                (*sprite).data[sStateDelay] = 20;
                (*sprite).data[sNextState] = VOLBEAT_ZIP_BACKWARD;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_ZIP_BACKWARD {
            (*sprite).x += 8;
            (*sprite).y -= 2;
            if (*sprite).x == 124 {
                (*sprite).data[sState] = VOLBEAT_WAIT_STATE;
                (*sprite).data[sStateDelay] = 20;
                (*sprite).data[sNextState] = VOLBEAT_ZIP_DOWN;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_ZIP_DOWN {
            (*sprite).y += 4;
            if (*sprite).y == 80 {
                (*sprite).data[sState] = VOLBEAT_WAIT_STATE;
                (*sprite).data[sStateDelay] = 10;
                (*sprite).data[sNextState] = VOLBEAT_ZIP_FORWARD;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_ZIP_FORWARD {
            (*sprite).x -= 8;
            (*sprite).y -= 2;
            if (*sprite).x == 60 {
                (*sprite).data[sState] = VOLBEAT_WAIT_STATE;
                (*sprite).data[sStateDelay] = 10;
                (*sprite).data[sNextState] = VOLBEAT_INIT_FIGURE_8;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_INIT_FIGURE_8 {
            fall = true;
            (*sprite).x += 60;
            (*sprite).data[sSinXIdx] = 0xC0;
            (*sprite).data[sSinYIdx] = 0x80;
            (*sprite).data[sFig8Loops] = 3;
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == VOLBEAT_FIGURE_8 {
            (*sprite).x2 = Sin((*sprite).data[sSinXIdx] as u8 as i16, 0x3C);
            (*sprite).y2 = Sin((*sprite).data[sSinYIdx] as u8 as i16, 0x14);
            (*sprite).data[sSinXIdx] += 2;
            (*sprite).data[sSinYIdx] += 4;
            if (*sprite).data[sSinXIdx] as i32 & 0xFF == 64 {
                (*sprite).set_hFlip(FALSE as u16);
                if ({
                    (*sprite).data[sFig8Loops] -= 1;
                    (*sprite).data[sFig8Loops]
                }) == 0
                {
                    (*sprite).x += (*sprite).x2;
                    (*sprite).x2 = 0;
                    (*sprite).data[sState] += 1;
                }
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_EXIT {
            (*sprite).x -= 2;
            (*sprite).y2 = Sin((*sprite).data[sSinYIdx] as u8 as i16, 0x14);
            (*sprite).data[sSinYIdx] += 4;
            if (*sprite).x < -16 {
                DestroySprite(sprite);
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_WAIT_STATE {
            (*sprite).y2 = Cos((*sprite).data[sCosYIdx] as u8 as i16, 2);
            if ({
                (*sprite).data[sStateDelay] -= 1;
                (*sprite).data[sStateDelay]
            }) == 0
            {
                (*sprite).data[sState] = (*sprite).data[sNextState];
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn SpriteCB_Torchic(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            if gIntroFrameCounter.get() == TIMER_TORCHIC_ENTER {
                StartSpriteAnim(sprite, TORCHIC_ANIM_RUN);
                (*sprite).data[sState] += 1;
            }
        }
        1 => {
            if gIntroFrameCounter.get() == TIMER_PLAYER_HOLD_POSITION {
                StartSpriteAnim(sprite, TORCHIC_ANIM_WALK);
                (*sprite).data[sState] += 1;
            } else {
                (*sprite).data[sMoveTimer] += 64;
                if (*sprite).data[sMoveTimer] as i32 & 0xFF00 != 0 {
                    (*sprite).x -= 1;
                    (*sprite).data[sMoveTimer] &= 0xFF;
                }
            }
        }
        2 => {
            if gIntroFrameCounter.get() != TIMER_TORCHIC_SPEED_UP {
                (*sprite).data[sMoveTimer] += 32;
                if (*sprite).data[sMoveTimer] as i32 & 0xFF00 != 0 {
                    (*sprite).x += 1;
                    (*sprite).data[sMoveTimer] &= 0xFF;
                }
            } else {
                StartSpriteAnim(sprite, TORCHIC_ANIM_RUN);
                (*sprite).data[sState] += 1;
                (*sprite).data[2] = 80;
            }
        }
        3 => {
            if ({
                (*sprite).data[2] -= 1;
                (*sprite).data[2]
            }) != 0
            {
                (*sprite).data[sMoveTimer] += 64;
                if (*sprite).data[sMoveTimer] as i32 & 0xFF00 != 0 {
                    (*sprite).x -= 1;
                    (*sprite).data[sMoveTimer] &= 0xFF;
                }
            } else {
                StartSpriteAnim(sprite, TORCHIC_ANIM_TRIP);
                (*sprite).data[sState] += 1;
            }
        }
        4 => {
            if (*sprite).animEnded() != 0 {
                (*sprite).x += 4;
            }
            if (*sprite).x > 336 {
                StartSpriteAnim(sprite, TORCHIC_ANIM_RUN);
                (*sprite).data[sState] += 1;
            }
        }
        5 if gIntroFrameCounter.get() >= TIMER_TORCHIC_EXIT => {
            (*sprite).x -= 2;
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_Manectric(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            if gIntroFrameCounter.get() == TIMER_MANECTRIC_ENTER {
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            (*sprite).x -= 2;
            if gIntroFrameCounter.get() != TIMER_MANECTRIC_RUN_CIRCULAR {
                break 'l1;
            }
            (*sprite).y -= 12;
            (*sprite).data[sSinIdx] = 0x80;
            (*sprite).data[sCosIdx] = 0;
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 2 {
            if (*sprite).x as i32 + (*sprite).x2 as i32 <= -32 {
                DestroySprite(sprite);
            } else {
                if (*sprite).data[sSinIdx] as i32 & 0xFF < 64 {
                    (*sprite).x2 = Sin((*sprite).data[sSinIdx] as u8 as i16, 16);
                } else {
                    if (*sprite).data[sSinIdx] as i32 & 0xFF == 64 {
                        (*sprite).x -= 48;
                    }
                    (*sprite).x2 = Sin((*sprite).data[sSinIdx] as u8 as i16, 64);
                }
                (*sprite).data[sSinIdx] += 1;
                (*sprite).y2 = Cos((*sprite).data[sCosIdx] as u8 as i16, 12);
                (*sprite).data[sCosIdx] += 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_Scene3_Load(taskId: u8) {
    IntroResetGpuRegs();
    LZ77UnCompVram(
        sIntroPokeball_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sIntroPokeball_Tilemap.as_ptr().cast_mut(),
        0x6004000_usize as *mut c_void,
    );
    LoadPalette(
        sIntroPokeball_Pal.as_ptr().cast_mut() as *mut c_void,
        0,
        512,
    );
    task_set(taskId, tAlpha, 0);
    task_set(taskId, tZoomDiv, 0);
    task_set(taskId, tZoomDivSpeed, 0);
    task_set(taskId, 3, 0);
    PanFadeAndZoomScreen(120, 80, 0, 0);
    ResetSpriteData();
    FreeAllSpritePalettes();
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
    SetGpuReg(REG_OFFSET_BG2CNT, 18563);
    SetGpuReg(REG_OFFSET_DISPCNT, 5185);
    task_set_func(taskId, Some(Task_Scene3_SpinPokeball));
    gIntroFrameCounter.set(0);
    m4aSongNumStart(MUS_INTRO_BATTLE);
}
pub(crate) unsafe fn Task_Scene3_SpinPokeball(taskId: u8) {
    task_set(taskId, tAlpha, task_get(taskId, tAlpha) + 0x400);
    if task_get(taskId, tZoomDiv) <= 0x6BF {
        task_set(
            taskId,
            tZoomDiv,
            task_get(taskId, tZoomDiv) + (task_get(taskId, tZoomDivSpeed)),
        );
        task_set(taskId, tZoomDivSpeed, task_get(taskId, tZoomDivSpeed) + 2);
    } else {
        task_set_func(taskId, Some(Task_Scene3_WaitGroudon));
    }
    PanFadeAndZoomScreen(
        120,
        80,
        (if task_get(taskId, tZoomDiv) != 0 {
            div_i32(0x10000, task_get(taskId, tZoomDiv) as i32)
        } else {
            0
        }) as u16,
        task_get(taskId, tAlpha) as u16,
    );
    if gIntroFrameCounter.get() == TIMER_POKEBALL_FADE {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 65535);
    }
}
pub(crate) fn Task_Scene3_WaitGroudon(taskId: u8) {
    if gIntroFrameCounter.get() > TIMER_START_LEGENDARIES {
        task_set_func(taskId, Some(Task_Scene3_LoadGroudon));
    }
}
pub(crate) unsafe fn Task_Scene3_LoadGroudon(taskId: u8) {
    if gPaletteFade.active() == 0 {
        IntroResetGpuRegs();
        ResetSpriteData();
        FreeAllSpritePalettes();
        gReservedSpritePaletteCount = 8;
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gIntroGroudon_Gfx).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            VRAM as usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gIntroGroudon_Tilemap).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            0x600c000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gIntroLegendBg_Gfx).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            0x6004000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gIntroGroudonBg_Tilemap).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            0x600e000_usize as *mut c_void,
        );
        LoadCompressedSpriteSheetUsingHeap(
            (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPicTable)
                .cast::<CArray<CompressedSpriteSheet, 0>>())[58])
                .cast_mut(),
        );
        LoadCompressedSpritePaletteUsingHeap(
            (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPaletteTable)
                .cast::<CArray<CompressedSpritePalette, 0>>())[58])
                .cast_mut(),
        );
        CpuSet(
            (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                .cast::<CArray<CArray<u16, 16>, 16>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
            gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
            256,
        );
        task_set_func(taskId, Some(Task_Scene3_InitGroudonBg));
    }
}
pub(crate) unsafe fn Task_Scene3_InitGroudonBg(taskId: u8) {
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    SetGpuReg(REG_OFFSET_WIN0V, DISPLAY_HEIGHT);
    SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 47232);
    SetGpuReg(REG_OFFSET_BG1CNT, 7173);
    SetGpuReg(REG_OFFSET_DISPCNT, 13889);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
    task_set(taskId, tWinPos, 0);
    task_set(taskId, tScreenX, -96);
    task_set(taskId, tScreenY, -175);
    task_set(taskId, tZoom, 0x100);
    PanFadeAndZoomScreen(
        task_get(taskId, tScreenX) as u16,
        task_get(taskId, tScreenY) as u16,
        task_get(taskId, tZoom) as u16,
        0,
    );
    task_set_func(taskId, Some(Task_Scene3_NarrowWindow));
}
pub(crate) unsafe fn Task_Scene3_NarrowWindow(taskId: u8) {
    if task_get(taskId, tWinPos) != NARROW_HEIGHT {
        task_set(taskId, tWinPos, task_get(taskId, tWinPos) + 4);
        SetGpuReg(
            REG_OFFSET_WIN0V,
            task_get(taskId, tWinPos) as u16 * 256
                - (task_get(taskId, tWinPos) as u16 - DISPLAY_HEIGHT),
        );
    } else {
        SetGpuReg(REG_OFFSET_WIN0V, 8320);
        task_set_func(taskId, Some(Task_Scene3_EndNarrowWindow));
    }
}
pub(crate) fn Task_Scene3_EndNarrowWindow(taskId: u8) {
    task_set_func(taskId, Some(Task_Scene3_StartGroudon));
}
pub(crate) unsafe fn Task_Scene3_StartGroudon(taskId: u8) {
    task_set(taskId, tState, 0);
    task_set_func(taskId, Some(Task_Scene3_Groudon));
    ScanlineEffect_InitWave(0, DISPLAY_HEIGHT as u8, 4, 4, 1, 4, 0);
}
pub(crate) unsafe fn Task_Scene3_Groudon(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(5) += 1;
    if *data >= 1 && *data <= 7 && *data.at(5) % 2 == 0 {
        *data.at(4) ^= 3;
    }
    PanFadeAndZoomScreen(
        *data.at(1) as u16,
        *data.at(2) as u16 + *data.at(4) as u16,
        *data.at(3) as u16,
        0,
    );
    match *data {
        0 => {
            *data.at(1) += 16;
            if *data.at(1) == 160 {
                *data += 1;
                *data.at(6) = 2;
                *data.at(7) = 0x1E2;
                CreateGroudonRockSprites(taskId);
            }
        }
        1 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 2;
                CpuSet(
                    ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                        .cast::<CArray<CArray<u16, 16>, 16>>()))
                        .cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[31] as *mut c_void,
                    1,
                );
                *data.at(7) += 2;
                if *data.at(7) == 0x1EC {
                    *data += 1;
                }
            }
        }
        2 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 2;
                *data += 1;
            }
        }
        3 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 2;
                CpuSet(
                    ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                        .cast::<CArray<CArray<u16, 16>, 16>>()))
                        .cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[31] as *mut c_void,
                    1,
                );
                *data.at(7) -= 2;
                if *data.at(7) == 0x1E0 {
                    *data.at(6) = 8;
                    *data += 1;
                }
            }
        }
        4 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(1) = -96;
                *data.at(2) = 169;
                *data.at(6) = 3;
                *data += 1;
            }
        }
        5 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(1) = 80;
                *data.at(2) = 41;
                *data.at(6) = 16;
                PlayCryInternal(SPECIES_GROUDON, 0, 100, CRY_PRIORITY_NORMAL, 0);
                *data += 1;
            }
        }
        6 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(1) = 80;
                *data.at(2) = 40;
                *data += 1;
            }
        }
        7 => {
            *data.at(1) += 4;
            *data.at(2) += 4;
            *data.at(6) += 0x666;
            *data.at(3) = Sin(((*data.at(6) as i32 & 0xFF00) >> 8) as i16, 64) + 256;
            if *data.at(1) == 120 {
                BeginNormalPaletteFade(0xfffffffe, 3, 0, 16, 32767);
                *data.at(3) = 256;
                *data.at(4) = 0;
                *data += 1;
            }
        }
        8 => {
            if *data.at(3) != 0 {
                *data.at(3) -= 8;
            } else {
                *data += 1;
            }
        }
        9 if gPaletteFade.active() == 0 => {
            task_set_func(taskId, Some(Task_Scene3_LoadKyogre));
            (*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .state = 3;
        }
        _ => {}
    }
}
unsafe fn CreateGroudonRockSprites(taskId: u8) {
    let mut spriteId: u8 = 0;
    for i in 0..6i32 {
        spriteId = CreateSprite(
            (&raw const (*(&raw const crate::data::battle_anim_rock::gAncientPowerRockSpriteTemplate).cast::<SpriteTemplate>())).cast_mut(),
            sGroudonRockData[i][0],
            DISPLAY_HEIGHT as i16,
            i as u8,
        );
        gSprites[spriteId].callback = Some(SpriteCB_GroudonRocks);
        gSprites[spriteId].oam.set_priority(0);
        gSprites[spriteId].data[sRockId] = i as i16;
        gSprites[spriteId].data[4] = taskId as i16;
        StartSpriteAnim(&raw mut gSprites[spriteId], sGroudonRockData[i][1] as u8);
    }
}
pub(crate) unsafe fn SpriteCB_GroudonRocks(sprite: *mut Sprite) {
    (*sprite).data[3] += 1;
    if (*sprite).data[3] % 2 == 0 {
        (*sprite).y2 ^= 3;
    }
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[sSpeed] += sGroudonRockData[(*sprite).data[sRockId]][2];
            (*sprite).y -= (((*sprite).data[sSpeed] as i32 & 0xFF00) >> 8) as i16;
            (*sprite).data[sSpeed] &= 0xFF;
            if task_get((*sprite).data[4], 0) > 7 {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if (*sprite).x < 120 {
                (*sprite).x -= 2;
            } else {
                (*sprite).x += 2;
            }
            if (*sprite).y < 80 {
                (*sprite).y -= 2;
            } else {
                (*sprite).y += 2;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_Scene3_LoadKyogre(taskId: u8) {
    ResetSpriteData();
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroKyogre_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroKyogre_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x600c000_usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroKyogreBg_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x600e000_usize as *mut c_void,
    );
    LoadCompressedSpriteSheet(sSpriteSheet_Bubbles.as_ptr().cast_mut());
    LoadSpritePalette(sSpritePalette_Bubbles.as_ptr().cast_mut());
    BeginNormalPaletteFade(0xfffffffe, 0, 16, 0, 65535);
    task_set_func(taskId, Some(Task_Scene3_Kyogre));
    task_set(taskId, tState, 0);
    task_set(taskId, tScreenX, 336);
    task_set(taskId, tScreenY, 80);
    task_set(taskId, 6, 16);
    task_set(taskId, tZoom, 256);
    PanFadeAndZoomScreen(
        task_get(taskId, tScreenX) as u16,
        task_get(taskId, tScreenY) as u16,
        task_get(taskId, tZoom) as u16,
        0,
    );
    ScanlineEffect_InitWave(
        0,
        DISPLAY_HEIGHT as u8,
        4,
        4,
        1,
        SCANLINE_EFFECT_REG_BG1VOFS,
        0,
    );
}
pub(crate) unsafe fn Task_Scene3_Kyogre(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    PanFadeAndZoomScreen(
        *data.at(1) as u16,
        *data.at(2) as u16,
        *data.at(3) as u16,
        0,
    );
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) != 0
            {
                break 'l1;
            }
            *data += 1;
        }
        if fall || sw1 == 1 {
            *data.at(6) += 4;
            task_set(taskId, tScreenX, 344 - Sin(*data.at(6), 0x100));
            task_set(taskId, tScreenY, 84 - Cos(*data.at(6), 0x40));
            if *data.at(6) == 64 {
                *data.at(6) = 0x19;
                *data.at(7) = 1;
                *data += 1;
                CreateKyogreBubbleSprites_Body(0);
            }
            break 'l1;
        }
        if sw1 == 2 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                task_set(taskId, tScreenX, task_get(taskId, tScreenX) + 256);
                task_set(taskId, tScreenY, task_get(taskId, tScreenY) - 258);
                *data.at(6) = 8;
                *data += 1;
                CreateKyogreBubbleSprites_Body(0);
                CreateKyogreBubbleSprites_Fins();
            }
            break 'l1;
        }
        if sw1 == 3 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                task_set(taskId, tScreenX, task_get(taskId, tScreenX) - 256);
                task_set(taskId, tScreenY, task_get(taskId, tScreenY) + 258);
                *data.at(6) = 8;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                task_set(taskId, tScreenY, task_get(taskId, tScreenY) - 252);
                *data.at(6) = 8;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                task_set(taskId, tScreenY, task_get(taskId, tScreenY) + 252);
                if *data.at(7) != 0 {
                    *data.at(6) = 12;
                    *data.at(7) -= 1;
                    *data = 2;
                } else {
                    *data.at(6) = 1;
                    *data += 1;
                    PlayCryInternal(SPECIES_KYOGRE as u16, 0, 120, CRY_PRIORITY_NORMAL, 0);
                }
            }
            break 'l1;
        }
        if sw1 == 6 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                *data.at(7) = 0x1EA;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                CpuSet(
                    ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                        .cast::<CArray<CArray<u16, 16>, 16>>()))
                        .cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[47] as *mut c_void,
                    1,
                );
                *data.at(7) -= 2;
                if *data.at(7) == 0x1E0 {
                    *data += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 8 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                *data.at(7) = 0x1E2;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 9 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                CpuSet(
                    ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                        .cast::<CArray<CArray<u16, 16>, 16>>()))
                        .cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[47] as *mut c_void,
                    1,
                );
                *data.at(7) += 2;
                if *data.at(7) == 0x1EE {
                    *data.at(6) = 16;
                    *data += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 10 {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 0;
                *data += 1;
                CreateKyogreBubbleSprites_Body(taskId);
            }
            break 'l1;
        }
        if sw1 == 11 {
            *data.at(6) += 4;
            *data.at(3) -= 8;
            task_set(taskId, tScreenX, Sin(*data.at(6), 0x3C) + 88);
            if *data.at(6) == 64 {
                BeginNormalPaletteFade(0xfffffffe, 3, 0, 16, 32767);
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 12 {
            *data.at(6) += 4;
            *data.at(3) -= 8;
            task_set(taskId, tScreenX, Sin(*data.at(6), 0x14) + 128);
            if *data.at(6) == 128 {
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 13 {
            if gPaletteFade.active() == 0 {
                task_set_func(taskId, Some(Task_Scene3_LoadClouds1));
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
            }
            break 'l1;
        }
    }
}
unsafe fn CreateKyogreBubbleSprites_Body(taskId: u8) {
    let mut spriteId: u8 = 0;
    for i in 0..NUM_BUBBLES_IN_SET {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Bubbles).cast_mut(),
            sKyogreBubbleData[i][0],
            sKyogreBubbleData[i][1],
            i as u8,
        );
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].data[5] = taskId as i16;
        gSprites[spriteId].data[6] = sKyogreBubbleData[i][2];
        gSprites[spriteId].data[sUnk] = 64;
    }
}
unsafe fn CreateKyogreBubbleSprites_Fins() {
    let mut spriteId: u8 = 0;
    for i in 0..NUM_BUBBLES_IN_SET {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Bubbles).cast_mut(),
            sKyogreBubbleData[i + NUM_BUBBLES_IN_SET][0],
            sKyogreBubbleData[i + NUM_BUBBLES_IN_SET][1],
            i as u8,
        );
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].data[6] = sKyogreBubbleData[i][2];
        gSprites[spriteId].data[sUnk] = 64;
    }
}
pub(crate) unsafe fn SpriteCB_KyogreBubbles(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if (*sprite).data[6] == 0 {
                (*sprite).data[sSinIdx] = ((*sprite).data[sSinIdx] + 11) & 0xFF;
                (*sprite).x2 = Sin((*sprite).data[sSinIdx], 4);
                (*sprite).data[sBaseY] += 48;
                (*sprite).y2 = -((*sprite).data[sBaseY] >> 8);
                if (*sprite).animEnded() != 0 {
                    DestroySprite(sprite);
                }
            } else if ({
                (*sprite).data[6] -= 1;
                (*sprite).data[6]
            }) == 0
            {
                StartSpriteAnim(sprite, 0);
                (*sprite).set_invisible(FALSE as u16);
            }
            if task_get((*sprite).data[5], 0) > 11 {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if (*sprite).x < 120 {
                (*sprite).x -= 3;
            } else {
                (*sprite).x += 3;
            }
            if (*sprite).y < 80 {
                (*sprite).y -= 3;
            } else {
                (*sprite).y += 3;
            }
            if (*sprite).y as u16 as i32 - 20 > 140 {
                DestroySprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_Scene3_LoadClouds1(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 135);
    SetGpuReg(REG_OFFSET_BLDALPHA, 7967);
    SetGpuReg(REG_OFFSET_BLDY, 31);
    SetGpuReg(REG_OFFSET_BG0CNT, 22528);
    SetGpuReg(REG_OFFSET_BG1CNT, 23044);
    SetGpuReg(REG_OFFSET_BG2CNT, 7174);
    SetGpuReg(0x0, 14144);
    SetGpuReg(REG_OFFSET_BG0HOFS, 80);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 65456);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroClouds_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroClouds_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x6004000_usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroCloudsSun_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x600e000_usize as *mut c_void,
    );
    task_set_func(taskId, Some(Task_Scene3_LoadClouds2));
}
pub(crate) unsafe fn Task_Scene3_LoadClouds2(taskId: u8) {
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroCloudsLeft_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x600c000_usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroCloudsRight_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x600d000_usize as *mut c_void,
    );
    task_set_func(taskId, Some(Task_Scene3_InitClouds));
}
pub(crate) unsafe fn Task_Scene3_InitClouds(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    task_set_func(taskId, Some(Task_Scene3_Clouds));
    task_set(taskId, tState, 0);
    task_set(taskId, tCloudPos, 16);
}
pub(crate) unsafe fn Task_Scene3_Clouds(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    SetGpuReg(REG_OFFSET_BG0HOFS, (*data.at(6) >> 8) as u16);
    SetGpuReg(
        REG_OFFSET_BG1HOFS,
        ((*data.at(6) >> 8) as u16).wrapping_neg(),
    );
    match *data {
        0 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                BeginNormalPaletteFade(0xfffffffe, 0, 16, 0, 65535);
                *data.at(6) = 20480;
                *data += 1;
            }
        }
        1 => {
            if *data.at(6) == 10240 {
                BeginNormalPaletteFade(65534, 3, 0, 16, 10569);
            }
            if *data.at(6) != 0 {
                *data.at(6) -= 128;
            } else if gPaletteFade.active() == 0 {
                task_set_func(taskId, Some(Task_Scene3_LoadLightning));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_Scene3_LoadLightning(taskId: u8) {
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroRayquaza_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x600e000_usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroRayquazaClouds_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        0x600c000_usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroRayquaza_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x6004000_usize as *mut c_void,
    );
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gIntroRayquazaClouds_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        VRAM as usize as *mut c_void,
    );
    SetGpuReg(0x0, 13632);
    task_set_func(taskId, Some(Task_Scene3_Lightning));
    task_set(taskId, tState, 0);
    task_set(taskId, 6, 1);
    task_set(taskId, 7, 0);
    LoadCompressedSpriteSheetUsingHeap(sSpriteSheet_Lightning.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalette_Lightning.as_ptr().cast_mut());
}
pub(crate) unsafe fn Task_Scene3_Lightning(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut spriteId: u8 = 0;
    match *data {
        0 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    200,
                    48,
                    0,
                );
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    200,
                    80,
                    1,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 1);
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    200,
                    112,
                    2,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 2);
                *data += 1;
                *data.at(6) = 72;
            }
        }
        1 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    40,
                    48,
                    0,
                );
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    40,
                    80,
                    1,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 1);
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    40,
                    112,
                    2,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 2);
                *data += 1;
                *data.at(6) = 48;
            }
        }
        2 if ({
            *data.at(6) -= 1;
            *data.at(6)
        }) == 0 =>
        {
            task_set_func(taskId, Some(Task_Scene3_LoadRayquazaAttack));
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_Lightning(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[sPalIdx] = 0x1C2;
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            CpuSet(
                ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal).cast::<CArray<
                    CArray<u16, 16>,
                    16,
                >>(
                )))
                    .cast_mut() as *mut c_void as *mut u8)
                    .at((*sprite).data[sPalIdx]) as *mut c_void,
                &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[93] as *mut c_void,
                1,
            );
            (*sprite).data[sPalIdx] += 2;
            if (*sprite).data[sPalIdx] != 0x1CE {
                break 'l1;
            }
            (*sprite).data[sPalIdx] = 0x1CC;
            (*sprite).data[2] = 4;
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 2 {
            if ({
                (*sprite).data[2] -= 1;
                (*sprite).data[2]
            }) == 0
            {
                (*sprite).data[2] = 4;
                CpuSet(
                    ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                        .cast::<CArray<CArray<u16, 16>, 16>>()))
                        .cast_mut() as *mut c_void as *mut u8)
                        .at((*sprite).data[sPalIdx]) as *mut c_void,
                    &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[93] as *mut c_void,
                    1,
                );
                (*sprite).data[sPalIdx] -= 2;
                if (*sprite).data[sPalIdx] == 0x1C0 {
                    DestroySprite(sprite);
                }
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_Scene3_LoadRayquazaAttack(taskId: u8) {
    LoadCompressedSpriteSheet(sSpriteSheet_RayquazaOrb.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalette_RayquazaOrb.as_ptr().cast_mut());
    SetGpuReg(0x0, 13632);
    task_set_func(taskId, Some(Task_Scene3_Rayquaza));
    BeginNormalPaletteFade(65502, 0, 16, 0, 10569);
    task_set(taskId, tState, 0);
    task_set(taskId, 1, 0xA8);
    task_set(taskId, 2, -16);
    task_set(taskId, 3, -136);
    task_set(taskId, 4, -16);
    let attackTaskId: u8 = CreateTask(Some(Task_RayquazaAttack), 0);
    task_set(attackTaskId, 4, taskId as i16);
}
pub(crate) unsafe fn Task_Scene3_Rayquaza(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(7) % 2 == 0 {
        *data.at(6) ^= 2;
    }
    *data.at(7) += 1;
    match *data {
        0 => {
            if *data.at(7) as i32 & 1 != 0 {
                *data.at(1) -= 2;
                *data.at(2) += 1;
                *data.at(3) += 2;
                *data.at(4) += 1;
            }
            if *data.at(1) == 0x68 {
                *data += 1;
                *data.at(5) = 1;
            }
        }
        1 => {
            *data += 1;
            *data.at(5) = 4;
        }
        2 => {
            *data.at(1) += 4;
            *data.at(2) -= 2;
            *data.at(3) -= 4;
            *data.at(4) -= 2;
            if gPaletteFade.active() == 0 {
                *data.at(5) = 0x8C;
                *data += 1;
            }
        }
        3 if ({
            *data.at(5) -= 1;
            *data.at(5)
        }) == 0 =>
        {
            task_set_func(taskId, Some(Task_EndIntroMovie));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_EndIntroMovie(taskId: u8) {
    DestroyTask(taskId);
    SetMainCallback2(Some(MainCB2_EndIntro));
}
pub(crate) unsafe fn Task_RayquazaAttack(taskId: u8) {
    let mut spriteId: u8 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(2) += 1;
    match *data {
        0 => {
            if *data.at(2) as i32 & 1 != 0 {
                CpuSet(
                    (((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                        .cast::<CArray<CArray<u16, 16>, 16>>()))
                        .cast_mut() as *mut c_void as *mut u8)
                        .at(418) as *mut c_void as *mut u8)
                        .at(*data.at(1) as i32 * 2) as *mut c_void,
                    &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[94] as *mut c_void,
                    1,
                );
                *data.at(1) += 1;
            }
            if *data.at(1) == 6 {
                *data += 1;
                *data.at(1) = 0;
                *data.at(3) = 10;
            }
        }
        1 => {
            if *data.at(3) == 0 {
                if *data.at(2) as i32 & 1 != 0 {
                    CpuSet(
                        (((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                            .cast::<CArray<CArray<u16, 16>, 16>>()))
                            .cast_mut() as *mut c_void as *mut u8)
                            .at(418) as *mut c_void as *mut u8)
                            .at(*data.at(1) as i32 * 2) as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                            .cast::<CArray<u16, 512>>()
                            .cast_mut())[88] as *mut c_void,
                        1,
                    );
                    *data.at(1) += 1;
                }
                if *data.at(1) == 6 {
                    *data += 1;
                    *data.at(3) = 10;
                }
            } else {
                *data.at(3) -= 1;
            }
        }
        2 => {
            if *data.at(3) == 0 {
                if *data.at(2) as i32 & 1 != 0 {
                    CpuSet(
                        (((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                            .cast::<CArray<CArray<u16, 16>, 16>>()))
                            .cast_mut() as *mut c_void as *mut u8)
                            .at(386) as *mut c_void as *mut u8)
                            .at(*data.at(1) as i32 * 2) as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                            .cast::<CArray<u16, 512>>()
                            .cast_mut())[92] as *mut c_void,
                        1,
                    );
                    *data.at(1) += 1;
                }
                if *data.at(1) == 6 {
                    spriteId = CreateSprite(
                        (&raw const *sSpriteTemplate_RayquazaOrb).cast_mut(),
                        120,
                        88,
                        15,
                    );
                    PlaySE(SE_INTRO_BLAST);
                    gSprites[spriteId].set_invisible(TRUE as u16);
                    gSprites[spriteId].data[3] = *data.at(4);
                    *data += 1;
                    *data.at(3) = 16;
                }
            } else {
                *data.at(3) -= 1;
            }
        }
        3 => {
            if *data.at(2) as i32 & 1 != 0 {
                if ({
                    *data.at(3) -= 1;
                    *data.at(3)
                }) != 0
                {
                    BlendPalette(80, 16, *data.at(3) as u8, 10569);
                    CpuSet(
                        ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                            .cast::<CArray<CArray<u16, 16>, 16>>()))
                            .cast_mut() as *mut c_void as *mut u8)
                            .at(428) as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                            .cast::<CArray<u16, 512>>()
                            .cast_mut())[94] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                            .cast::<CArray<CArray<u16, 16>, 16>>()))
                            .cast_mut() as *mut c_void as *mut u8)
                            .at(428) as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                            .cast::<CArray<u16, 512>>()
                            .cast_mut())[88] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        ((&raw const (*(&raw const crate::data::graphics::gIntro3Bg_Pal)
                            .cast::<CArray<CArray<u16, 16>, 16>>()))
                            .cast_mut() as *mut c_void as *mut u8)
                            .at(396) as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                            .cast::<CArray<u16, 512>>()
                            .cast_mut())[92] as *mut c_void,
                        1,
                    );
                } else {
                    *data += 1;
                    *data.at(3) = 53;
                }
            }
        }
        4 => {
            if ({
                *data.at(3) -= 1;
                *data.at(3)
            }) == 0
            {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 32767);
                *data += 1;
            }
        }
        5 if gPaletteFade.active() == 0 => {
            DestroyTask(taskId);
        }
        _ => {}
    }
}
unsafe fn IntroResetGpuRegs() {
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
}
pub(crate) unsafe fn Task_BlendLogoIn(taskId: u8) {
    match task_get(taskId, tState) {
        1 => {
            if task_get(taskId, 1) != 0 {
                task_set(taskId, 1, task_get(taskId, 1) - 1);
                let tmp: u8 = (task_get(taskId, 1) / 2) as u8;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (*(&raw const crate::data::title_screen::gTitleScreenAlphaBlend)
                        .cast::<CArray<u16, 64>>())[tmp],
                );
            } else {
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (*(&raw const crate::data::title_screen::gTitleScreenAlphaBlend)
                        .cast::<CArray<u16, 64>>())[0],
                );
                task_set(taskId, 1, 16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            DestroyTask(taskId);
        }
        _ => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16192);
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (*(&raw const crate::data::title_screen::gTitleScreenAlphaBlend)
                    .cast::<CArray<u16, 64>>())[31],
            );
            SetGpuReg(REG_OFFSET_BLDY, 0);
            task_set(taskId, 1, 64);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
    }
}
pub(crate) unsafe fn Task_BlendLogoOut(taskId: u8) {
    match task_get(taskId, tState) {
        1 => {
            if task_get(taskId, 1) < 62 {
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                let tmp: u8 = (task_get(taskId, 1) / 2) as u8;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (*(&raw const crate::data::title_screen::gTitleScreenAlphaBlend)
                        .cast::<CArray<u16, 64>>())[tmp],
                );
            } else {
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (*(&raw const crate::data::title_screen::gTitleScreenAlphaBlend)
                        .cast::<CArray<u16, 64>>())[31],
                );
                task_set(taskId, 1, 16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 => {
            if task_get(taskId, 1) != 0 {
                task_set(taskId, 1, task_get(taskId, 1) - 1);
            } else {
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                SetGpuReg(REG_OFFSET_BLDY, 0);
                DestroyTask(taskId);
            }
        }
        _ => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16192);
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (*(&raw const crate::data::title_screen::gTitleScreenAlphaBlend)
                    .cast::<CArray<u16, 64>>())[0],
            );
            SetGpuReg(REG_OFFSET_BLDY, 0);
            task_set(taskId, 1, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
    }
}
pub unsafe fn PanFadeAndZoomScreen(screenX: u16, screenY: u16, zoom: u16, alpha: u16) {
    let mut src: BgAffineSrcData = zeroed();
    let mut dest: BgAffineDstData = zeroed();
    src.texX = 0x8000;
    src.texY = 0x8000;
    src.scrX = screenX as i16;
    src.scrY = screenY as i16;
    src.sx = zoom as i16;
    src.sy = zoom as i16;
    src.alpha = alpha;
    BgAffineSet(&raw mut src, &raw mut dest, 1);
    SetGpuReg(REG_OFFSET_BG2PA, dest.pa as u16);
    SetGpuReg(REG_OFFSET_BG2PB, dest.pb as u16);
    SetGpuReg(REG_OFFSET_BG2PC, dest.pc as u16);
    SetGpuReg(REG_OFFSET_BG2PD, dest.pd as u16);
    SetGpuReg(REG_OFFSET_BG2X_L, dest.dx as u16);
    SetGpuReg(REG_OFFSET_BG2X_H, (dest.dx >> 16) as u16);
    SetGpuReg(REG_OFFSET_BG2Y_L, dest.dy as u16);
    SetGpuReg(REG_OFFSET_BG2Y_H, (dest.dy >> 16) as u16);
}
pub(crate) unsafe fn SpriteCB_WaterDrop_Ripple(sprite: *mut Sprite) {
    let mut palNum: u8 = 0;
    if (*sprite).data[2] >= 192 {
        if (*sprite).data[3] != 0 {
            (*sprite).data[3] -= 1;
        } else {
            (*sprite).set_invisible(FALSE as u16);
            SetOamMatrix(
                (*sprite).data[1] as u8,
                (*sprite).data[2] as u16,
                0,
                0,
                (*sprite).data[2] as u16,
            );
            (*sprite).data[2] = ((*sprite).data[2] as i32 * 95 / 100) as i16;
            palNum = (((*sprite).data[2] as i32 - 192) / 128) as u8 + 9;
            if palNum > 15 {
                palNum = 15;
            }
            (*sprite).oam.set_paletteNum(palNum as u16);
        }
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn SpriteCB_WaterDropHalf(sprite: *mut Sprite) {
    if gSprites[(*sprite).data[7]].data[7] != 0 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        StartSpriteAnim(sprite, DROP_ANIM_RIPPLE);
        (*sprite).data[2] = 1024;
        (*sprite).data[3] = 8 * ((*sprite).data[1] & 3);
        (*sprite).callback = Some(SpriteCB_WaterDrop_Ripple);
        (*sprite).oam.set_shape(1);
        (*sprite).oam.set_size(3);
        CalcCenterToCornerVec(sprite, 1, 3, ST_OAM_AFFINE_ERASE);
    } else {
        (*sprite).x2 = gSprites[(*sprite).data[7]].x2;
        (*sprite).y2 = gSprites[(*sprite).data[7]].y2;
        (*sprite).x = gSprites[(*sprite).data[7]].x;
        (*sprite).y = gSprites[(*sprite).data[7]].y;
    }
}
pub(crate) unsafe fn SpriteCB_WaterDrop(sprite: *mut Sprite) {
    if (*sprite).data[sState] != 0 {
        (*sprite).callback = Some(SpriteCB_WaterDrop_Slide);
    }
}
pub(crate) unsafe fn SpriteCB_WaterDrop_Slide(sprite: *mut Sprite) {
    if (*sprite).x <= 116 {
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x += 4;
        (*sprite).x2 = -4;
        (*sprite).data[4] = 128;
        (*sprite).callback = Some(SpriteCB_WaterDrop_ReachLeafEnd);
    } else {
        let data4: u16 = (*sprite).data[4] as u16;
        let sin1: i16 =
            (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[data4 as u8];
        let sin2: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [data4 as u8 as i32 + 64];
        (*sprite).data[4] += 2;
        (*sprite).y2 = sin1 / 32;
        (*sprite).x -= 1;
        if (*sprite).x as i32 & 1 != 0 {
            (*sprite).y += 1;
        }
        let temp: i16 = (-(sin2 as i32) / 16) as i16;
        let data2: u16 = (*sprite).data[2] as u16;
        let data3: u16 = (*sprite).data[3] as u16;
        let sin3: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [temp as u8 as i32 - 16];
        let sin4: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [temp as u8 as i32 + 48];
        let var1: i16 = (sin4 as i32 * data2 as i32 / 256) as i16;
        let var2: i16 = (-(sin3 as i32) * data3 as i32 / 256) as i16;
        let var3: i16 = (sin3 as i32 * data2 as i32 / 256) as i16;
        let var4: i16 = (sin4 as i32 * data3 as i32 / 256) as i16;
        SetOamMatrix((*sprite).data[1] as u8, data2, 0, 0, data3);
        SetOamMatrix(
            (*sprite).data[1] as u8 + 1,
            var1 as u16,
            var3 as u16,
            var2 as u16,
            var4 as u16,
        );
        SetOamMatrix(
            (*sprite).data[1] as u8 + 2,
            var1 as u16,
            var3 as u16,
            var2 as u16 * 2,
            var4 as u16 * 2,
        );
    }
}
pub(crate) unsafe fn SpriteCB_WaterDrop_ReachLeafEnd(sprite: *mut Sprite) {
    SetOamMatrix(
        (*sprite).data[1] as u8,
        (*sprite).data[6] as u16 + 64,
        0,
        0,
        (*sprite).data[6] as u16 + 64,
    );
    SetOamMatrix(
        (*sprite).data[1] as u8 + 1,
        (*sprite).data[6] as u16 + 64,
        0,
        0,
        (*sprite).data[6] as u16 + 64,
    );
    SetOamMatrix(
        (*sprite).data[1] as u8 + 2,
        (*sprite).data[6] as u16 + 64,
        0,
        0,
        (*sprite).data[6] as u16 + 64,
    );
    if (*sprite).data[4] != MAX_SPRITES as i16 {
        (*sprite).data[4] -= 8;
        let sinIdx: u16 = (*sprite).data[4] as u16;
        (*sprite).x2 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [sinIdx as u8 as i32 + 64]
            / 64;
        (*sprite).y2 =
            (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[sinIdx as u8] / 64;
    } else {
        (*sprite).data[4] = 0;
        (*sprite).callback = Some(SpriteCB_WaterDrop_DangleFromLeaf);
    }
}
pub(crate) unsafe fn SpriteCB_WaterDrop_DangleFromLeaf(sprite: *mut Sprite) {
    if (*sprite).data[0] != 2 {
        (*sprite).data[4] += 8;
        let r2: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [(*sprite).data[4] as u8]
            / 16
            + 64;
        (*sprite).x2 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [r2 as u8 as i32 + 64]
            / 64;
        (*sprite).y2 =
            (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[r2 as u8] / 64;
    } else {
        (*sprite).callback = Some(SpriteCB_WaterDrop_Fall);
    }
}
pub(crate) unsafe fn SpriteCB_WaterDrop_Fall(sprite: *mut Sprite) {
    if (*sprite).y < (*sprite).data[5] {
        (*sprite).y += 4;
    } else {
        (*sprite).data[7] = 1;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        StartSpriteAnim(sprite, DROP_ANIM_RIPPLE);
        (*sprite).data[2] = 1024;
        (*sprite).data[3] = 8 * ((*sprite).data[1] & 3);
        (*sprite).callback = Some(SpriteCB_WaterDrop_Ripple);
        (*sprite).oam.set_shape(1);
        (*sprite).oam.set_size(3);
        CalcCenterToCornerVec(sprite, 1, 3, ST_OAM_AFFINE_ERASE);
    }
}
pub(crate) unsafe fn SpriteCB_WaterDropShort(sprite: *mut Sprite) {
    if (*sprite).y < (*sprite).data[5] {
        (*sprite).y += 4;
    } else {
        (*sprite).data[7] = 1;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        StartSpriteAnim(sprite, DROP_ANIM_RIPPLE);
        (*sprite).data[2] = 1024;
        (*sprite).data[3] = 8 * ((*sprite).data[1] & 3);
        (*sprite).callback = Some(SpriteCB_WaterDrop_Ripple);
        (*sprite).oam.set_shape(1);
        (*sprite).oam.set_size(3);
        CalcCenterToCornerVec(sprite, 1, 3, ST_OAM_AFFINE_ERASE);
    }
}
unsafe fn CreateWaterDrop(x: i16, y: i16, c: u16, d: u16, e: u16, fallImmediately: u8) -> u8 {
    let mut spriteId: u8 =
        CreateSprite((&raw const *sSpriteTemplate_WaterDrop).cast_mut(), x, y, 1);
    gSprites[spriteId].data[0] = 0;
    gSprites[spriteId].data[7] = 0;
    gSprites[spriteId].data[1] = d as i16;
    gSprites[spriteId].data[2] = c as i16;
    gSprites[spriteId].data[3] = c as i16;
    gSprites[spriteId].data[5] = e as i16;
    gSprites[spriteId].data[6] = c as i16;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(d as u32);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        0,
        ST_OAM_AFFINE_ERASE,
        ST_OAM_AFFINE_ERASE,
    );
    StartSpriteAnim(&raw mut gSprites[spriteId], DROP_ANIM_REFLECTION);
    if fallImmediately == 0 {
        gSprites[spriteId].callback = Some(SpriteCB_WaterDrop);
    } else {
        gSprites[spriteId].callback = Some(SpriteCB_WaterDropShort);
    }
    let oldSpriteId: u8 = spriteId;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_WaterDrop).cast_mut(), x, y, 1);
    gSprites[spriteId].data[7] = oldSpriteId as i16;
    gSprites[spriteId].data[1] = d as i16 + 1;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(d as u32 + 1);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        0,
        ST_OAM_AFFINE_ERASE,
        ST_OAM_AFFINE_ERASE,
    );
    gSprites[spriteId].callback = Some(SpriteCB_WaterDropHalf);
    spriteId = CreateSprite((&raw const *sSpriteTemplate_WaterDrop).cast_mut(), x, y, 1);
    gSprites[spriteId].data[7] = oldSpriteId as i16;
    gSprites[spriteId].data[1] = d as i16 + 2;
    StartSpriteAnim(&raw mut gSprites[spriteId], DROP_ANIM_LOWER_HALF);
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(d as u32 + 2);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        0,
        ST_OAM_AFFINE_ERASE,
        ST_OAM_AFFINE_ERASE,
    );
    gSprites[spriteId].callback = Some(SpriteCB_WaterDropHalf);
    SetOamMatrix(d as u8, c + 32, 0, 0, c + 32);
    SetOamMatrix(d as u8 + 1, c + 32, 0, 0, c + 32);
    SetOamMatrix(d as u8 + 2, c + 32, 0, 0, 2 * (c + 32));
    oldSpriteId
}
pub(crate) unsafe fn SpriteCB_PlayerOnBicycle(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            (*sprite).x -= 1;
        }
        1 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            if gIntroFrameCounter.get() & 7 != 0 {
                return;
            }
            (*sprite).x += 1;
        }
        2 => {
            if (*sprite).x <= 120 || gIntroFrameCounter.get() & 7 != 0 {
                (*sprite).x += 1;
            }
        }
        3 => {}
        4 if (*sprite).x > -32 => {
            (*sprite).x -= 2;
        }
        _ => {}
    }
    if gIntroFrameCounter.get() & 7 != 0 {
        return;
    }
    if (*sprite).y2 != 0 {
        (*sprite).y2 = 0;
    } else {
        match Random() as i32 & 3 {
            0 => {
                (*sprite).y2 = -1;
            }
            1 => {
                (*sprite).y2 = 1;
            }
            2 | 3 => {
                (*sprite).y2 = 0;
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn SpriteCB_Flygon(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {}
        1 => {
            if ((*sprite).x2 as i32 + (*sprite).x as i32) < 304 {
                (*sprite).x2 += 8;
            } else {
                (*sprite).data[sState] = 2;
            }
        }
        2 => {
            if (*sprite).x2 as i32 + (*sprite).x as i32 > 120 {
                (*sprite).x2 -= 1;
            } else {
                (*sprite).data[sState] = 3;
            }
        }
        3 if (*sprite).x2 > 0 => {
            (*sprite).x2 -= 2;
        }
        _ => {}
    }
    (*sprite).y2 = Sin((*sprite).data[sSinIdx] as u8 as i16, 8) - sFlygonYOffset.get() as i16;
    (*sprite).data[sSinIdx] += 4;
}
pub(crate) unsafe fn SpriteCB_LogoLetter(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            if (*sprite).data[1] != 0 {
                (*sprite).data[1] -= 1;
            } else {
                (*sprite).set_invisible(FALSE as u16);
                StartSpriteAffineAnim(sprite, 1);
                (*sprite).data[sState] += 1;
            }
        }
        1 => {
            if gIntroFrameCounter.get() == TIMER_LOGO_LETTERS_COLOR {
                (*sprite).data[sState] += 1;
                (*sprite).data[1] = COLOR_CHANGES;
                (*sprite).data[3] = 2;
            }
        }
        2 => {
            if (*sprite).data[3] == 0 {
                (*sprite).data[3] = 2;
                if (*sprite).data[1] != 0 {
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1]]).cast_mut()
                            as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[287] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1] as i32 + 16])
                            .cast_mut() as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[276] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1] as i32 + 32])
                            .cast_mut() as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[282] as *mut c_void,
                        1,
                    );
                    (*sprite).data[1] -= 1;
                } else {
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1]]).cast_mut()
                            as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[287] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1] as i32 + 16])
                            .cast_mut() as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[276] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1] as i32 + 32])
                            .cast_mut() as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[282] as *mut c_void,
                        1,
                    );
                    (*sprite).data[sState] += 1;
                }
            } else {
                (*sprite).data[3] -= 1;
            }
        }
        3 => {
            if (*sprite).data[3] != 0 {
                (*sprite).data[3] -= 1;
            } else {
                (*sprite).data[3] = 2;
                if (*sprite).data[1] <= COLOR_CHANGES {
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1]]).cast_mut()
                            as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[287] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1] as i32 + 16])
                            .cast_mut() as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[276] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const (*(&raw const crate::data::graphics::gIntroGameFreakTextFade_Pal).cast::<CArray<u16, 0>>())[(*sprite).data[1] as i32 + 32])
                            .cast_mut() as *mut c_void,
                        &raw mut (*(&raw const crate::palette::gPlttBufferFaded).cast::<CArray<u16, 512>>().cast_mut())[282] as *mut c_void,
                        1,
                    );
                    (*sprite).data[1] += 1;
                } else {
                    (*sprite).data[sState] += 1;
                }
            }
        }
        4 => {
            if gIntroFrameCounter.get() == TIMER_LOGO_DISAPPEAR {
                StartSpriteAffineAnim(sprite, 2);
                (*sprite).oam.set_objMode(ST_OAM_OBJ_BLEND);
                (*sprite).data[sState] += 1;
            }
        }
        5 => {
            (*sprite).data[3] += sGameFreakLettersMoveSpeed[(*sprite).data[sLetterId]] as i16;
            (*sprite).x2 = (((*sprite).data[3] as i32 & 0xFF00) >> 8) as i16;
            if (*sprite).data[sLetterId] < 4 {
                let temp: i16 = (*sprite).x2;
                (*sprite).x2 = -temp;
            }
            if (*sprite).affineAnimEnded() != 0 {
                DestroySprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_GameFreakLogo(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            if gIntroFrameCounter.get() == TIMER_LOGO_APPEAR {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).data[sState] += 1;
            }
        }
        1 => {
            if gIntroFrameCounter.get() == TIMER_LOGO_DISAPPEAR {
                StartSpriteAffineAnim(sprite, 3);
                (*sprite).data[sState] += 1;
            }
        }
        2 if (*sprite).affineAnimEnded() != 0 => {
            DestroySprite(sprite);
        }
        _ => {}
    }
}
unsafe fn CreateGameFreakLogoSprites(x: i16, y: i16, unused: i16) -> u8 {
    let mut spriteId: u8 = 0;
    let mut i: u16 = 0;
    while i < NUM_GF_LETTERS {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_GameFreakLetter).cast_mut(),
            sGameFreakLetterData[i][1] + x,
            y - 4,
            0,
        );
        gSprites[spriteId].data[sState] = 0;
        gSprites[spriteId].data[1] = sGameFreakLetterStartDelays[i] as i16;
        gSprites[spriteId].data[sLetterId] = i as i16;
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].oam.set_matrixNum(i as u32 + 12);
        StartSpriteAnim(
            &raw mut gSprites[spriteId],
            sGameFreakLetterData[i][0] as u8,
        );
        StartSpriteAffineAnim(&raw mut gSprites[spriteId], 0);
        i += 1;
    }
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_GameFreakLogo).cast_mut(),
        120,
        y - 6,
        0,
    );
    gSprites[spriteId].data[sState] = 0;
    gSprites[spriteId].set_invisible(TRUE as u16);
    gSprites[spriteId].oam.set_matrixNum(i as u32 + 12);
    StartSpriteAffineAnim(&raw mut gSprites[spriteId], 1);
    spriteId
}
pub(crate) unsafe fn SpriteCB_FlygonSilhouette(sprite: *mut Sprite) {
    (*sprite).data[7] += 1;
    if (*sprite).data[sState] != 0 {
        let sin: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [(*sprite).data[sRot] as u8];
        let cos: i16 = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [(*sprite).data[sRot] as u8 as i32 + 64];
        let d: i16 = (cos as i32 * (*sprite).data[sScale] as i32 / 256) as i16;
        let c: i16 = (-(sin as i32) * (*sprite).data[sScale] as i32 / 256) as i16;
        let b: i16 = (sin as i32 * (*sprite).data[sScale] as i32 / 256) as i16;
        let a: i16 = (cos as i32 * (*sprite).data[sScale] as i32 / 256) as i16;
        SetOamMatrix(1, a as u16, b as u16, c as u16, d as u16);
    }
    match (*sprite).data[sState] {
        1 => {
            (*sprite).x2 = -Sin((*sprite).data[sPos] as u8 as i16, 140);
            (*sprite).y2 = -Sin((*sprite).data[sPos] as u8 as i16, 120);
            (*sprite).data[sScale] += 7;
            (*sprite).data[sPos] += 3;
            if (*sprite).x as i32 + (*sprite).x2 as i32 <= -16 {
                (*sprite).oam.set_priority(3);
                (*sprite).data[sState] += 1;
                (*sprite).x = 20;
                (*sprite).y = 40;
                (*sprite).data[sScale] = 512;
                (*sprite).data[sRot] = 0;
                (*sprite).data[sPos] = 16;
            }
        }
        2 => {
            (*sprite).x2 = Sin((*sprite).data[sPos] as u8 as i16, 34);
            (*sprite).y2 = -Cos((*sprite).data[sPos] as u8 as i16, 60);
            (*sprite).data[sScale] += 2;
            if (*sprite).data[7] % 5 == 0 {
                (*sprite).data[sPos] += 1;
            }
        }
        _ => {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            (*sprite).oam.set_matrixNum(1);
            CalcCenterToCornerVec(
                sprite,
                1,
                ST_OAM_AFFINE_DOUBLE as u8,
                ST_OAM_AFFINE_DOUBLE as u8,
            );
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).data[sState] = 1;
            (*sprite).data[sScale] = 128;
            (*sprite).data[sRot] = 0;
            (*sprite).data[sPos] = 0;
        }
    }
}
pub(crate) unsafe fn SpriteCB_RayquazaOrb(sprite: *mut Sprite) {
    let mut foo: u16 = 0;
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let matched = sw1 == 0 || sw1 == 1;
        let mut fall = false;
        if sw1 == 0 || !matched {
            fall = true;
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            (*sprite).oam.set_matrixNum(18);
            CalcCenterToCornerVec(
                sprite,
                0,
                ST_OAM_AFFINE_DOUBLE as u8,
                ST_OAM_AFFINE_DOUBLE as u8,
            );
            (*sprite).data[1] = 0;
            (*sprite).data[sState] = 1;
        }
        if fall || sw1 == 1 {
            (*sprite).data[7] += 1;
            if (*sprite).data[7] as i32 & 1 != 0 {
                (*sprite).set_invisible(TRUE as u16);
            } else {
                (*sprite).set_invisible(FALSE as u16);
                if (*sprite).data[1] < 64 {
                    (*sprite).data[1] += 1;
                }
            }
            foo = 256
                - ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                    [(*sprite).data[1] as u8]
                    / 2) as u16;
            SetOamMatrix(18, foo, 0, 0, foo);
            break 'l1;
        }
    }
}
