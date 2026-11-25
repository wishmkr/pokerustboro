//! Translated from `src/pokeblock_feed.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_gfx_sfx_util::{AllocateMonSpritesGfx, FreeMonSpritesGfx};
use crate::battle_main::gMonSpritesGfxPtr;
use crate::bg::{ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::item_menu::gSpecialVar_ItemId;
use crate::load_save::gSaveBlock1Ptr;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayVolumeControl};
use crate::menu::{
    AddTextPrinterParameterized2, ClearScheduledBgCopiesToVram, DecompressAndCopyTileDataToVram,
    DoScheduledBgTilemapCopiesToVram, DrawStdFrameWithCustomTileAndPalette,
    FreeTempTileDataBuffersIfPossible, GetPlayerTextSpeedDelay, ResetTempTileDataBuffers,
    ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{
    MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv, ResetAllBgsCoordinates,
    ResetVramOamAndBgCntRegs, RunTextPrintersRetIsActive, SetVBlankHBlankCallbacksToNull,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::party_menu::GetMonNickname;
use crate::pokeblock::{
    CreatePokeblockCaseSprite, GetPokeblockData, PokeblockCopyName, PokeblockGetGain,
};
use crate::pokemon::{
    GetMonData2, GetMonSpritePalStructFromOtIdPersonality, GetNature, IsMonSpriteNotFlipped,
    SetMultiuseSpriteTemplateToPokemon, gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::sound::PlayCry_Normal;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, LoadOam,
    ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::{DestroyTask, RunTasks};
use crate::task::{task_get, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
use crate::use_pokeblock::{gPokeblockGain, gPokeblockMonId};
use crate::window::{FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap};
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `HandleLoadSpecialPokePic_2` with this module's view of its types.
#[inline]
fn HandleLoadSpecialPokePic_2(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32, a3: u32) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_2(a0 as _, a1 as _, a2, a3);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const sSpeed: usize = 0;
const tState: usize = 0;
const sAccel: usize = 1;
const tHorizontalThrow: usize = 1;
const sSpecies: usize = 2;
// Data tables (translate with cdata.py): sNatureToMonPokeblockAnim sMonPokeblockAnims sAffineAnim_Mon_None sAffineAnim_Mon_TurnUp sAffineAnim_Mon_TurnUp_Flipped sAffineAnim_Mon_TurnUpAndDown sAffineAnim_Mon_TurnUpAndDown_Flipped sAffineAnim_Mon_TurnDown sAffineAnim_Mon_TurnDown_Flipped sAffineAnim_Mon_TurnDownSlow sAffineAnim_Mon_TurnDownSlow_Flipped sAffineAnim_Mon_TurnDownSlight sAffineAnim_Mon_TurnDownSlight_Flipped sAffineAnim_Mon_TurnUpHigh sAffineAnim_Mon_TurnUpHigh_Flipped sAffineAnims_Mon sBackgroundTemplates sWindowTemplates sPokeblocksPals sAffineAnim_Still sSpriteAffineAnimTable_MonNoFlip sAffineAnim_PokeblockCase_ThrowFromVertical sAffineAnim_PokeblockCase_ThrowFromHorizontal sAffineAnims_PokeblockCase_Still sAffineAnims_PokeblockCase_ThrowFromVertical sAffineAnims_PokeblockCase_ThrowFromHorizontal sOamData_Pokeblock sAnim_Pokeblock sAnims_Pokeblock sAffineAnim_Pokeblock sAffineAnims_Pokeblock sSpriteSheet_Pokeblock sSpriteTemplate_Pokeblock

/// `struct PokeblockFeed`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokeblockFeed {
    pub monSpritePtr: *mut Sprite,
    pub savedMonSprite: Sprite,
    pub tilemapBuffer: CArray<u8, 2048>,
    pub unused1: CArray<u8, 8>,
    pub monAnimX: CArray<i16, 512>,
    pub monAnimY: CArray<i16, 512>,
    pub animRunState: u8,
    pub animId: u8,
    pub unused2: u8,
    pub noMonFlip: u8,
    pub species: u16,
    pub monAnimLength: u16,
    pub timer: u16,
    pub nature: u8,
    pub monSpriteId_: u8,
    pub unused3: u8,
    pub monSpriteId: u8,
    pub pokeblockCaseSpriteId: u8,
    pub pokeblockSpriteId: u8,
    pub animData: CArray<i16, 10>,
    pub monInitX: i16,
    pub monInitY: i16,
    pub maxAnimStageTime: i16,
    pub monX: i16,
    pub monY: i16,
    pub loadGfxState: i16,
    pub unused4: u8,
}

unsafe impl Sync for PokeblockFeed {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokeblockFeed>() == 4228);
    assert!(offset_of!(PokeblockFeed, monSpritePtr) == 0);
    assert!(offset_of!(PokeblockFeed, savedMonSprite) == 4);
    assert!(offset_of!(PokeblockFeed, tilemapBuffer) == 72);
    assert!(offset_of!(PokeblockFeed, unused1) == 2120);
    assert!(offset_of!(PokeblockFeed, monAnimX) == 2128);
    assert!(offset_of!(PokeblockFeed, monAnimY) == 3152);
    assert!(offset_of!(PokeblockFeed, animRunState) == 4176);
    assert!(offset_of!(PokeblockFeed, animId) == 4177);
    assert!(offset_of!(PokeblockFeed, unused2) == 4178);
    assert!(offset_of!(PokeblockFeed, noMonFlip) == 4179);
    assert!(offset_of!(PokeblockFeed, species) == 4180);
    assert!(offset_of!(PokeblockFeed, monAnimLength) == 4182);
    assert!(offset_of!(PokeblockFeed, timer) == 4184);
    assert!(offset_of!(PokeblockFeed, nature) == 4186);
    assert!(offset_of!(PokeblockFeed, monSpriteId_) == 4187);
    assert!(offset_of!(PokeblockFeed, unused3) == 4188);
    assert!(offset_of!(PokeblockFeed, monSpriteId) == 4189);
    assert!(offset_of!(PokeblockFeed, pokeblockCaseSpriteId) == 4190);
    assert!(offset_of!(PokeblockFeed, pokeblockSpriteId) == 4191);
    assert!(offset_of!(PokeblockFeed, animData) == 4192);
    assert!(offset_of!(PokeblockFeed, monInitX) == 4212);
    assert!(offset_of!(PokeblockFeed, monInitY) == 4214);
    assert!(offset_of!(PokeblockFeed, maxAnimStageTime) == 4216);
    assert!(offset_of!(PokeblockFeed, monX) == 4218);
    assert!(offset_of!(PokeblockFeed, monY) == 4220);
    assert!(offset_of!(PokeblockFeed, loadGfxState) == 4222);
    assert!(offset_of!(PokeblockFeed, unused4) == 4224);
};

const AFFINE_NONE: u8 = 0;
const ANIMDATA_APPR_TIME: i32 = 8;
const ANIMDATA_COS_AMPLITUDE: i32 = 3;
const ANIMDATA_IS_LAST: i32 = 9;
const ANIMDATA_ROT_ACCEL: i32 = 5;
const ANIMDATA_ROT_IDX: i32 = 0;
const ANIMDATA_ROT_SPEED: i32 = 1;
const ANIMDATA_SIN_AMPLITUDE: i32 = 2;
const ANIMDATA_TARGET_X: i32 = 6;
const ANIMDATA_TARGET_Y: i32 = 7;
const ANIMDATA_TIME: i32 = 4;
const MON_X: i16 = 48;
const MON_Y: i16 = 80;
const NUM_ANIMDATA: u8 = 10;
const NUM_MON_AFFINES: u8 = 10;
const STATE_PRINT_MSG: i16 = 297;
const STATE_SPAWN_PBLOCK: i16 = 269;
const STATE_START_JUMP: i16 = 281;
const STATE_START_THROW: i16 = 255;

static sAffineAnims_Mon: Table<CArray<*mut AffineAnimCmd, 21>> =
    Table((&raw const crate::data::pokeblock_feed::sAffineAnims_Mon).cast());
static sAffineAnims_PokeblockCase_Still: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::pokeblock_feed::sAffineAnims_PokeblockCase_Still).cast());
static sAffineAnims_PokeblockCase_ThrowFromHorizontal: Table<CArray<*mut AffineAnimCmd, 1>> = Table(
    (&raw const crate::data::pokeblock_feed::sAffineAnims_PokeblockCase_ThrowFromHorizontal).cast(),
);
static sAffineAnims_PokeblockCase_ThrowFromVertical: Table<CArray<*mut AffineAnimCmd, 1>> = Table(
    (&raw const crate::data::pokeblock_feed::sAffineAnims_PokeblockCase_ThrowFromVertical).cast(),
);
static sBackgroundTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::pokeblock_feed::sBackgroundTemplates).cast());
static sMonPokeblockAnims: Table<CArray<CArray<i16, 10>, 55>> =
    Table((&raw const crate::data::pokeblock_feed::sMonPokeblockAnims).cast());
static sNatureToMonPokeblockAnim: Table<CArray<CArray<u8, 2>, 25>> =
    Table((&raw const crate::data::pokeblock_feed::sNatureToMonPokeblockAnim).cast());
static sPokeblocksPals: Table<CArray<*mut u32, 14>> =
    Table((&raw const crate::data::pokeblock_feed::sPokeblocksPals).cast());
static sSpriteAffineAnimTable_MonNoFlip: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::pokeblock_feed::sSpriteAffineAnimTable_MonNoFlip).cast());
static sSpriteSheet_Pokeblock: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokeblock_feed::sSpriteSheet_Pokeblock).cast());
static sSpriteTemplate_Pokeblock: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokeblock_feed::sSpriteTemplate_Pokeblock).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::pokeblock_feed::sWindowTemplates).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokeblockFeed: crate::global::Global<*mut PokeblockFeed> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokeblockSpritePal: crate::global::Global<CompressedSpritePalette> =
    crate::global::Global::new(unsafe { zeroed() });

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn CB2_PokeblockFeed() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_PokeblockFeed() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn LoadPokeblockFeedScene() -> u8 {
    match crate::gl!(gMain.state) {
        0 => {
            crate::gl!(sPokeblockFeed = AllocZeroed(4228) as *mut PokeblockFeed);
            SetVBlankHBlankCallbacksToNull();
            ClearScheduledBgCopiesToVram();
            crate::gl!(gMain.state += 1);
        }
        1 => {
            ResetPaletteFade();
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
            crate::gl!(gMain.state += 1);
        }
        2 => {
            ResetSpriteData();
            crate::gl!(gMain.state += 1);
        }
        3 => {
            FreeAllSpritePalettes();
            crate::gl!(gMain.state += 1);
        }
        4 => {
            AllocateMonSpritesGfx();
            crate::gl!(gMain.state += 1);
        }
        5 => {
            HandleInitBackgrounds();
            crate::gl!(gMain.state += 1);
        }
        6 => {
            HandleInitWindows();
            crate::gl!(gMain.state += 1);
        }
        7 => {
            if LoadMonAndSceneGfx({
                let gl_i0 = gPokeblockMonId.get();
                crate::gl!(&raw gPlayerParty[gl_i0])
            }) != 0
            {
                crate::gl!(gMain.state += 1);
            }
        }
        8 => {
            (*crate::gl!(sPokeblockFeed)).pokeblockCaseSpriteId =
                CreatePokeblockCaseSpriteForFeeding();
            crate::gl!(gMain.state += 1);
        }
        9 => {
            (*crate::gl!(sPokeblockFeed)).monSpriteId = CreateMonSprite({
                let gl_i1 = gPokeblockMonId.get();
                crate::gl!(&raw gPlayerParty[gl_i1])
            });
            crate::gl!(gMain.state += 1);
        }
        10 => {
            DrawStdFrameWithCustomTileAndPalette(0, 1, 1, 14);
            crate::gl!(gMain.state += 1);
        }
        11 => {
            LaunchPokeblockFeedTask();
            crate::gl!(gMain.state += 1);
        }
        12 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            crate::gl!(gMain.state += 1);
        }
        13 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            crate::gl!(gMain.state += 1);
        }
        _ => {
            SetVBlankCallback(Some(VBlankCB_PokeblockFeed));
            SetMainCallback2(Some(CB2_PokeblockFeed));
            return TRUE;
        }
    }
    FALSE
}
pub unsafe fn PreparePokeblockFeedScene() {
    loop {
        if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
            break;
        }
        if LoadPokeblockFeedScene() == TRUE {
            break;
        }
        if MenuHelpers_IsLinkActive() == TRUE {
            break;
        }
    }
}
pub(crate) unsafe fn HandleInitBackgrounds() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBackgroundTemplates.as_ptr().cast_mut(), 2);
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sPokeblockFeed)).tilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(1);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadMonAndSceneGfx(mon: *mut Pokemon) -> u8 {
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut trainerId: u32 = 0;
    let mut palette: *mut CompressedSpritePalette = null_mut();
    match (*crate::gl!(sPokeblockFeed)).loadGfxState {
        0 => {
            species = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
            personality = GetMonData2(mon, MON_DATA_PERSONALITY);
            HandleLoadSpecialPokePic_2(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*crate::gl!(gMonSpritesGfxPtr)).sprites.ptr[1],
                species as i32,
                personality,
            );
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        1 => {
            species = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
            personality = GetMonData2(mon, MON_DATA_PERSONALITY);
            trainerId = GetMonData2(mon, MON_DATA_OT_ID);
            palette = GetMonSpritePalStructFromOtIdPersonality(species, trainerId, personality);
            LoadCompressedSpritePalette(palette);
            SetMultiuseSpriteTemplateToPokemon((*palette).tag, B_POSITION_OPPONENT_LEFT);
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        2 => {
            LoadCompressedSpriteSheet(
                (&raw const (*(&raw const crate::data::pokeblock::gPokeblockCase_SpriteSheet)
                    .cast::<CompressedSpriteSheet>()))
                    .cast_mut(),
            );
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        3 => {
            LoadCompressedSpritePalette(
                (&raw const (*(&raw const crate::data::pokeblock::gPokeblockCase_SpritePal)
                    .cast::<CompressedSpritePalette>()))
                    .cast_mut(),
            );
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        4 => {
            LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Pokeblock).cast_mut());
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        5 => {
            SetPokeblockSpritePal(gSpecialVar_ItemId.get() as u8);
            LoadCompressedSpritePalette(crate::gl!(&raw sPokeblockSpritePal));
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        6 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                1,
                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
        }
        7 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gPokeblockFeedBg_Tilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    (*crate::gl!(sPokeblockFeed)).tilemapBuffer.as_mut_ptr() as *mut c_void,
                );
                (*crate::gl!(sPokeblockFeed)).loadGfxState += 1;
            }
        }
        8 => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Frontier)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                32,
                96,
            );
            (*crate::gl!(sPokeblockFeed)).loadGfxState = 0;
            return TRUE;
        }
        _ => {}
    }
    FALSE
}
pub(crate) unsafe fn HandleInitWindows() {
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 1, 224);
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        240,
        32,
    );
    FillWindowPixelBuffer(0, 0);
    PutWindowTilemap(0);
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn SetPokeblockSpritePal(pokeblockCaseId: u8) {
    let colorId: u8 = GetPokeblockData(
        &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pokeblockCaseId],
        PBLOCK_COLOR,
    ) as u8;
    crate::gl!(sPokeblockSpritePal.data = sPokeblocksPals[colorId as i32 - 1]);
    crate::gl!(sPokeblockSpritePal.tag = TAG_POKEBLOCK);
}
pub(crate) unsafe fn Task_HandlePokeblockFeed(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        match task_get(taskId, tState) {
            0 => {
                (*crate::gl!(sPokeblockFeed)).animRunState = 0;
                (*crate::gl!(sPokeblockFeed)).timer = 0;
                CalculateMonAnimLength();
            }
            STATE_START_THROW => {
                DoPokeblockCaseThrowEffect(
                    (*crate::gl!(sPokeblockFeed)).pokeblockCaseSpriteId,
                    task_get(taskId, tHorizontalThrow) as u8,
                );
            }
            STATE_SPAWN_PBLOCK => {
                (*crate::gl!(sPokeblockFeed)).pokeblockSpriteId = CreatePokeblockSprite();
            }
            STATE_START_JUMP => {
                StartMonJumpForPokeblock((*crate::gl!(sPokeblockFeed)).monSpriteId);
            }
            STATE_PRINT_MSG => {
                task_set_func(taskId, Some(Task_PrintAtePokeblockMessage));
                return;
            }
            _ => {}
        }
        if (*crate::gl!(sPokeblockFeed)).timer < (*crate::gl!(sPokeblockFeed)).monAnimLength {
            UpdateMonAnim();
        } else if (*crate::gl!(sPokeblockFeed)).timer == (*crate::gl!(sPokeblockFeed)).monAnimLength
        {
            task_set(taskId, tState, 254);
        }
        (*crate::gl!(sPokeblockFeed)).timer += 1;
        task_set(taskId, tState, task_get(taskId, tState) + 1);
    }
}
unsafe fn LaunchPokeblockFeedTask() {
    let taskId: u8 = CreateTask(Some(Task_HandlePokeblockFeed), 0);
    task_set(taskId, tState, 0);
    task_set(taskId, tHorizontalThrow, TRUE as i16);
}
pub(crate) unsafe fn Task_WaitForAtePokeblockMessage(taskId: u8) {
    if RunTextPrintersRetIsActive(0) != TRUE as u16 {
        task_set_func(taskId, Some(Task_FadeOutPokeblockFeed));
    }
}
pub(crate) unsafe fn Task_PrintAtePokeblockMessage(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i2 = gPokeblockMonId.get();
        crate::gl!(&raw gPlayerParty[gl_i2])
    };
    let pokeblock: *mut Pokeblock =
        &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()];
    gPokeblockGain.set(PokeblockGetGain(GetNature(mon), pokeblock));
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    PokeblockCopyName(pokeblock, (*gStringVar2.as_ptr()).as_mut_ptr());
    if gPokeblockGain.get() == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Var1AteTheVar2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else if gPokeblockGain.get() > 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Var1HappilyAteVar2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Var1DisdainfullyAteVar2)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    (*(crate::gl!(&raw @at crate::text::gTextFlags;).cast_const())
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(TRUE);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        GetPlayerTextSpeedDelay(),
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    task_set_func(taskId, Some(Task_WaitForAtePokeblockMessage));
}
pub(crate) unsafe fn Task_ExitPokeblockFeed(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        ResetSpriteData();
        FreeAllSpritePalettes();
        m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x100);
        SetMainCallback2(crate::gl!(gMain.savedCallback));
        DestroyTask(taskId);
        FreeAllWindowBuffers();
        Free(crate::gl!(sPokeblockFeed) as *mut c_void);
        FreeMonSpritesGfx();
    }
}
pub(crate) unsafe fn Task_FadeOutPokeblockFeed(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_ExitPokeblockFeed));
}
pub(crate) unsafe fn CreateMonSprite(mon: *mut Pokemon) -> u8 {
    let species: u16 = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
    let spriteId: u8 = CreateSprite(crate::gl!(&raw gMultiuseSpriteTemplate), MON_X, MON_Y, 2);
    (*crate::gl!(sPokeblockFeed)).species = species;
    (*crate::gl!(sPokeblockFeed)).monSpriteId_ = spriteId;
    (*crate::gl!(sPokeblockFeed)).nature = GetNature(mon);
    {
        let gl_i3 = sSpecies;
        crate::gl!(gSprites[spriteId].data[gl_i3] = species as i16)
    };
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCallbackDummy));
    (*crate::gl!(sPokeblockFeed)).noMonFlip = TRUE;
    if IsMonSpriteNotFlipped(species) == 0 {
        crate::gl!(
            gSprites[spriteId].affineAnims = sSpriteAffineAnimTable_MonNoFlip.as_ptr().cast_mut()
        );
        (*gSprites.as_ptr())[spriteId]
            .oam
            .set_affineMode(ST_OAM_AFFINE_DOUBLE);
        CalcCenterToCornerVec(
            crate::gl!(&raw gSprites[spriteId]),
            (*gSprites.as_ptr())[spriteId].oam.shape() as u8,
            (*gSprites.as_ptr())[spriteId].oam.size() as u8,
            (*gSprites.as_ptr())[spriteId].oam.affineMode() as u8,
        );
        (*crate::gl!(sPokeblockFeed)).noMonFlip = FALSE;
    }
    spriteId
}
unsafe fn StartMonJumpForPokeblock(spriteId: u8) {
    crate::gl!(gSprites[spriteId].x = MON_X);
    crate::gl!(gSprites[spriteId].y = MON_Y);
    {
        let gl_i4 = sSpeed;
        crate::gl!(gSprites[spriteId].data[gl_i4] = -8)
    };
    {
        let gl_i5 = sAccel;
        crate::gl!(gSprites[spriteId].data[gl_i5] = 1)
    };
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_MonJumpForPokeblock));
}
pub(crate) unsafe fn SpriteCB_MonJumpForPokeblock(sprite: *mut Sprite) {
    (*sprite).x += 4;
    (*sprite).y += (*sprite).data[sSpeed];
    (*sprite).data[sSpeed] += (*sprite).data[sAccel];
    if (*sprite).data[sSpeed] == 0 {
        PlayCry_Normal((*sprite).data[sSpecies] as u16, 0);
    }
    if (*sprite).data[sSpeed] == 9 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
unsafe fn CreatePokeblockCaseSpriteForFeeding() -> u8 {
    let spriteId: u8 = CreatePokeblockCaseSprite(188, 100, 2);
    (*gSprites.as_ptr())[spriteId]
        .oam
        .set_affineMode(ST_OAM_AFFINE_NORMAL);
    crate::gl!(
        gSprites[spriteId].affineAnims = sAffineAnims_PokeblockCase_Still.as_ptr().cast_mut()
    );
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCallbackDummy));
    InitSpriteAffineAnim(crate::gl!(&raw gSprites[spriteId]));
    spriteId
}
unsafe fn DoPokeblockCaseThrowEffect(spriteId: u8, horizontalThrow: u8) {
    FreeOamMatrix((*gSprites.as_ptr())[spriteId].oam.matrixNum() as u8);
    (*gSprites.as_ptr())[spriteId]
        .oam
        .set_affineMode(ST_OAM_AFFINE_DOUBLE);
    if horizontalThrow == 0 {
        crate::gl!(
            gSprites[spriteId].affineAnims = sAffineAnims_PokeblockCase_ThrowFromVertical
                .as_ptr()
                .cast_mut()
        );
    } else {
        crate::gl!(
            gSprites[spriteId].affineAnims = sAffineAnims_PokeblockCase_ThrowFromHorizontal
                .as_ptr()
                .cast_mut()
        );
    }
    InitSpriteAffineAnim(crate::gl!(&raw gSprites[spriteId]));
}
unsafe fn CreatePokeblockSprite() -> u8 {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeblock).cast_mut(),
        174,
        84,
        1,
    );
    {
        let gl_i6 = sSpeed;
        crate::gl!(gSprites[spriteId].data[gl_i6] = -12)
    };
    {
        let gl_i7 = sAccel;
        crate::gl!(gSprites[spriteId].data[gl_i7] = 1)
    };
    spriteId
}
pub(crate) unsafe fn SpriteCB_ThrownPokeblock(sprite: *mut Sprite) {
    (*sprite).x -= 4;
    (*sprite).y += (*sprite).data[sSpeed];
    (*sprite).data[sSpeed] += (*sprite).data[sAccel];
    if (*sprite).data[sSpeed] == 10 {
        DestroySprite(sprite);
    }
}
unsafe fn CalculateMonAnimLength() {
    let pokeblockFeed: *mut PokeblockFeed = crate::gl!(sPokeblockFeed);
    (*pokeblockFeed).monAnimLength = 1;
    let mut animId: u8 = sNatureToMonPokeblockAnim[(*pokeblockFeed).nature][0];
    let mut i: u8 = 0;
    while i < 8 {
        (*pokeblockFeed).monAnimLength += sMonPokeblockAnims[animId][4] as u16;
        if sMonPokeblockAnims[animId][9] == TRUE as i16 {
            break;
        }
        i += 1;
        animId += 1;
    }
}
unsafe fn UpdateMonAnim() {
    let pokeblockFeed: *mut PokeblockFeed = crate::gl!(sPokeblockFeed);
    'l1: {
        let sw1: u8 = (*pokeblockFeed).animRunState;
        let mut fall = false;
        if sw1 == 0 {
            (*pokeblockFeed).animId = sNatureToMonPokeblockAnim[(*pokeblockFeed).nature][0];
            (*pokeblockFeed).monSpritePtr = {
                let gl_i8 = (*pokeblockFeed).monSpriteId_;
                crate::gl!(&raw gSprites[gl_i8])
            };
            (*pokeblockFeed).savedMonSprite = *(*pokeblockFeed).monSpritePtr;
            (*pokeblockFeed).animRunState = 10;
            break 'l1;
        }
        if (1..=9).contains(&sw1) {
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            InitMonAnimStage();
            if sNatureToMonPokeblockAnim[(*pokeblockFeed).nature][1] != AFFINE_NONE {
                (*(*pokeblockFeed).monSpritePtr)
                    .oam
                    .set_affineMode(ST_OAM_AFFINE_DOUBLE);
                (*(*pokeblockFeed).monSpritePtr).oam.set_matrixNum(0);
                (*(*pokeblockFeed).monSpritePtr).affineAnims = sAffineAnims_Mon.as_ptr().cast_mut();
                InitSpriteAffineAnim((*pokeblockFeed).monSpritePtr);
            }
            (*pokeblockFeed).animRunState = 50;
        }
        if fall || sw1 == 50 {
            if sNatureToMonPokeblockAnim[(*pokeblockFeed).nature][1] != AFFINE_NONE {
                if (*pokeblockFeed).noMonFlip == 0 {
                    StartSpriteAffineAnim(
                        (*pokeblockFeed).monSpritePtr,
                        sNatureToMonPokeblockAnim[(*pokeblockFeed).nature][1] + NUM_MON_AFFINES,
                    );
                } else {
                    StartSpriteAffineAnim(
                        (*pokeblockFeed).monSpritePtr,
                        sNatureToMonPokeblockAnim[(*pokeblockFeed).nature][1],
                    );
                }
            }
            (*pokeblockFeed).animRunState = 60;
            break 'l1;
        }
        if sw1 == 60 {
            if DoMonAnimStep() == TRUE {
                if (*pokeblockFeed).animData[9] == 0 {
                    (*pokeblockFeed).animId += 1;
                    InitMonAnimStage();
                    (*pokeblockFeed).animRunState = 60;
                } else {
                    FreeOamMatrix((*(*pokeblockFeed).monSpritePtr).oam.matrixNum() as u8);
                    (*pokeblockFeed).animRunState = 70;
                }
            }
            break 'l1;
        }
        if sw1 == 70 {
            FreeMonSpriteOamMatrix();
            (*pokeblockFeed).animId = 0;
            (*pokeblockFeed).animRunState = 0;
            break 'l1;
        }
        if (71..=90).contains(&sw1) {
            break 'l1;
        }
    }
}
unsafe fn InitMonAnimStage() -> u8 {
    let pokeblockFeed: *mut PokeblockFeed = crate::gl!(sPokeblockFeed);
    for i in 0..NUM_ANIMDATA {
        (*pokeblockFeed).animData[i] = sMonPokeblockAnims[(*pokeblockFeed).animId][i];
    }
    if (*pokeblockFeed).animData[4] == 0 {
        return TRUE;
    } else {
        (*pokeblockFeed).monInitX = Sin((*pokeblockFeed).animData[0], (*pokeblockFeed).animData[2]);
        (*pokeblockFeed).monInitY = Cos((*pokeblockFeed).animData[0], (*pokeblockFeed).animData[3]);
        (*pokeblockFeed).maxAnimStageTime = (*pokeblockFeed).animData[4];
        (*pokeblockFeed).monX = (*(*pokeblockFeed).monSpritePtr).x2;
        (*pokeblockFeed).monY = (*(*pokeblockFeed).monSpritePtr).y2;
        CalculateMonAnimMovement();
        (*pokeblockFeed).animData[4] = (*pokeblockFeed).maxAnimStageTime;
        CalculateMonAnimMovementEnd();
        (*pokeblockFeed).animData[4] = (*pokeblockFeed).maxAnimStageTime;
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn DoMonAnimStep() -> u8 {
    let time: u16 = (*crate::gl!(sPokeblockFeed)).maxAnimStageTime as u16
        - (*crate::gl!(sPokeblockFeed)).animData[4] as u16;
    (*(*crate::gl!(sPokeblockFeed)).monSpritePtr).x2 = (*crate::gl!(sPokeblockFeed)).monAnimX[time];
    (*(*crate::gl!(sPokeblockFeed)).monSpritePtr).y2 = (*crate::gl!(sPokeblockFeed)).monAnimY[time];
    if ({
        (*crate::gl!(sPokeblockFeed)).animData[4] -= 1;
        (*crate::gl!(sPokeblockFeed)).animData[4]
    }) == 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn FreeMonSpriteOamMatrix() -> u8 {
    FreeSpriteOamMatrix((*crate::gl!(sPokeblockFeed)).monSpritePtr);
    FALSE
}
unsafe fn CalculateMonAnimMovementEnd() {
    let pokeblockFeed: *mut PokeblockFeed = crate::gl!(sPokeblockFeed);
    let approachTime: u16 = (*pokeblockFeed).animData[8] as u16;
    let time: u16 = (*pokeblockFeed).maxAnimStageTime as u16 - approachTime;
    let x: i16 = (*pokeblockFeed).monX + (*pokeblockFeed).animData[6];
    let y: i16 = (*pokeblockFeed).monY + (*pokeblockFeed).animData[7];
    let mut i: u16 = 0;
    while (i as i32) < time as i32 - 1 {
        let xOffset: i16 = (*pokeblockFeed).monAnimX[approachTime as i32 + i as i32] - x;
        let yOffset: i16 = (*pokeblockFeed).monAnimY[approachTime as i32 + i as i32] - y;
        (*pokeblockFeed).monAnimX[approachTime as i32 + i as i32] -=
            div_i32(xOffset as i32 * (i as i32 + 1), time as i32) as i16;
        (*pokeblockFeed).monAnimY[approachTime as i32 + i as i32] -=
            div_i32(yOffset as i32 * (i as i32 + 1), time as i32) as i16;
        i += 1;
    }
    (*pokeblockFeed).monAnimX[approachTime as i32 + time as i32 - 1] = x;
    (*pokeblockFeed).monAnimY[approachTime as i32 + time as i32 - 1] = y;
}
unsafe fn CalculateMonAnimMovement() {
    let pokeblockFeed: *mut PokeblockFeed = crate::gl!(sPokeblockFeed);
    let mut negative: u8 = FALSE;
    let x: i16 = (*pokeblockFeed).monX - (*pokeblockFeed).monInitX;
    let y: i16 = (*pokeblockFeed).monY - (*pokeblockFeed).monInitY;
    loop {
        let acceleration: u16 = (if (*pokeblockFeed).animData[5] < 0 {
            -((*pokeblockFeed).animData[5] as i32)
        } else {
            (*pokeblockFeed).animData[5] as i32
        }) as u16;
        let amplitude: u16 = acceleration + (*pokeblockFeed).animData[3] as u16;
        (*pokeblockFeed).animData[3] = amplitude as i16;
        if (*pokeblockFeed).animData[2] < 0 {
            negative = TRUE;
        }
        let time: u16 =
            (*pokeblockFeed).maxAnimStageTime as u16 - (*pokeblockFeed).animData[4] as u16;
        if (*pokeblockFeed).animData[4] == 0 {
            break;
        }
        if negative == 0 {
            (*pokeblockFeed).monAnimX[time] = Sin(
                (*pokeblockFeed).animData[0],
                (*pokeblockFeed).animData[2] + (amplitude as i32 / 256) as i16,
            ) + x;
            (*pokeblockFeed).monAnimY[time] = Cos(
                (*pokeblockFeed).animData[0],
                (*pokeblockFeed).animData[3] + (amplitude as i32 / 256) as i16,
            ) + y;
        } else {
            (*pokeblockFeed).monAnimX[time] = Sin(
                (*pokeblockFeed).animData[0],
                (*pokeblockFeed).animData[2] - (amplitude as i32 / 256) as i16,
            ) + x;
            (*pokeblockFeed).monAnimY[time] = Cos(
                (*pokeblockFeed).animData[0],
                (*pokeblockFeed).animData[3] - (amplitude as i32 / 256) as i16,
            ) + y;
        }
        (*pokeblockFeed).animData[0] += (*pokeblockFeed).animData[1];
        (*pokeblockFeed).animData[0] &= 0xFF;
        (*pokeblockFeed).animData[4] -= 1;
    }
}
