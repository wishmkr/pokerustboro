//! Translated from `src/use_pokeblock.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetVBlankCallback, gKeyRepeatStartDelay};
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
use crate::bg::{CopyToBgTilemapBufferRect, LoadBgTilemap, LoadBgTiles};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::item_menu::gSpecialVar_ItemId;
use crate::menu::{
    AddTextPrinterParameterized3, CreateYesNoMenu, Menu_ProcessInputNoWrapClearOnChoose,
};
use crate::menu_specialized::{
    ConditionGraph_CalcPositions, ConditionGraph_Draw, ConditionGraph_Init,
    ConditionGraph_InitResetScanline, ConditionGraph_InitWindow, ConditionGraph_ResetScanline,
    ConditionGraph_SetNewPositions, ConditionGraph_TryUpdate, ConditionGraph_Update,
    ConditionMenu_UpdateMonEnter, ConditionMenu_UpdateMonExit, CreateConditionSparkleSprites,
    DestroyConditionSparkleSprites, FreeConditionSparkles, GetBoxOrPartyMonData,
    GetConditionMenuMonConditions, GetConditionMenuMonGfx, GetConditionMenuMonNameAndLocString,
    LoadConditionMonPicTemplate, LoadConditionSelectionIcons, LoadConditionSparkle,
    MoveConditionMonOffscreen, MoveConditionMonOnscreen, ResetConditionSparkleSprites,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadPalette, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::pokeblock::TryClearPokeblock;
use crate::pokeblock_feed::PreparePokeblockFeedScene;
use crate::pokemon::{
    CalculatePlayerPartyCount, GetMonData2, GetMonData3, GetMonFlavorRelation, GetNature,
    SetMonData, gPlayerParty,
};
use crate::scanline_effect::ScanlineEffect_InitHBlankDmaTransfer;
use crate::scanline_effect::gScanlineEffect;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeSpritePaletteByTag,
    FreeSpriteTilesByTag, IndexOfSpritePaletteTag, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::StringGet_Nickname;
use crate::string_util::gStringVar4;
use crate::string_util::{StringAppend, StringCopy};
use crate::text::{DeactivateAllTextPrinters, RunTextPrinters};
use crate::text_window::{DrawTextBorderOuter, LoadUserWindowBorderGfx, rbox_fill_rectangle};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers,
    PutWindowTilemap,
};
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
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
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
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
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
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sSpeed: usize = 0;
const tTimer: usize = 0;
const sTargetX: usize = 1;
// Data tables (translate with cdata.py): sMonFrame_Pal sMonFrame_Gfx sMonFrame_Tilemap sGraphData_Tilemap sConditionToMonData sConditionToFlavor sNatureTextColors sBgTemplates sWindowTemplates sUsePokeblockYesNoWinTemplate sConditionNames sSpriteSheet_UpDown sSpritePalette_UpDown sUpDownCoordsOnGraph sOam_UpDown sAnim_Up sAnim_Down sAnims_UpDown sSpriteTemplate_UpDown sOam_Condition sAnim_Condition_0 sAnim_Condition_1 sAnim_Condition_2 sAnims_Condition sSpriteTemplate_Condition sSpritePalette_Condition

/// `struct UsePokeblockSession`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsePokeblockSession {
    pub callback: Option<unsafe fn()>,
    pub exitCallback: Option<unsafe fn()>,
    pub pokeblock: *mut Pokeblock,
    pub mon: *mut Pokemon,
    pub stringBuffer: CArray<u8, 64>,
    pub mainState: u8,
    pub unused1: u8,
    pub timer: u8,
    pub condition: u8,
    pub numEnhancements: u8,
    pub unused2: u8,
    pub monInTopHalf: u8,
    pub conditionsBeforeBlock: CArray<u8, 5>,
    pub conditionsAfterBlock: CArray<u8, 5>,
    pub enhancements: CArray<u8, 5>,
    pub pokeblockStatBoosts: CArray<i16, 5>,
    pub numSelections: u8,
    pub curSelection: u8,
    pub loadNewSelection: Option<unsafe fn() -> u8>,
    pub helperState: u8,
    pub unused3: u8,
    pub natureText: CArray<u8, 34>,
}

unsafe impl Sync for UsePokeblockSession {}

/// `struct UsePokeblockMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsePokeblockMenu {
    pub unused: u32,
    pub partyPalettes: CArray<CArray<u16, 64>, 6>,
    pub partySheets: CArray<CArray<u8, 8192>, 3>,
    pub unusedBuffer: CArray<u8, 4096>,
    pub tilemapBuffer: CArray<u8, 2050>,
    pub selectionIconSpriteIds: CArray<u8, 7>,
    pub curMonXOffset: i16,
    pub curMonSpriteId: u8,
    pub curMonPalette: u16,
    pub curMonSheet: u16,
    pub curMonTileStart: *mut u8,
    pub sparkles: CArray<*mut Sprite, 10>,
    pub condition: CArray<*mut Sprite, 2>,
    pub toLoadSelection: u8,
    pub locationStrings: CArray<CArray<u8, 24>, 3>,
    pub monNameStrings: CArray<CArray<u8, 64>, 3>,
    pub graph: ConditionGraph,
    pub numSparkles: CArray<u8, 3>,
    pub curLoadId: i8,
    pub nextLoadId: i8,
    pub prevLoadId: i8,
    pub toLoadId: i8,
    pub party: CArray<UsePokeblockMenuPokemon, 6>,
    pub info: UsePokeblockSession,
}

unsafe impl Sync for UsePokeblockMenu {}

/// `struct UsePokeblockMenuPokemon`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct UsePokeblockMenuPokemon {
    pub boxId: u8,
    pub monId: u8,
    pub data: u16,
}

unsafe impl Sync for UsePokeblockMenuPokemon {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<UsePokeblockSession>() == 156);
    assert!(offset_of!(UsePokeblockSession, callback) == 0);
    assert!(offset_of!(UsePokeblockSession, exitCallback) == 4);
    assert!(offset_of!(UsePokeblockSession, pokeblock) == 8);
    assert!(offset_of!(UsePokeblockSession, mon) == 12);
    assert!(offset_of!(UsePokeblockSession, stringBuffer) == 16);
    assert!(offset_of!(UsePokeblockSession, mainState) == 80);
    assert!(offset_of!(UsePokeblockSession, unused1) == 81);
    assert!(offset_of!(UsePokeblockSession, timer) == 82);
    assert!(offset_of!(UsePokeblockSession, condition) == 83);
    assert!(offset_of!(UsePokeblockSession, numEnhancements) == 84);
    assert!(offset_of!(UsePokeblockSession, unused2) == 85);
    assert!(offset_of!(UsePokeblockSession, monInTopHalf) == 86);
    assert!(offset_of!(UsePokeblockSession, conditionsBeforeBlock) == 87);
    assert!(offset_of!(UsePokeblockSession, conditionsAfterBlock) == 92);
    assert!(offset_of!(UsePokeblockSession, enhancements) == 97);
    assert!(offset_of!(UsePokeblockSession, pokeblockStatBoosts) == 102);
    assert!(offset_of!(UsePokeblockSession, numSelections) == 112);
    assert!(offset_of!(UsePokeblockSession, curSelection) == 113);
    assert!(offset_of!(UsePokeblockSession, loadNewSelection) == 116);
    assert!(offset_of!(UsePokeblockSession, helperState) == 120);
    assert!(offset_of!(UsePokeblockSession, unused3) == 121);
    assert!(offset_of!(UsePokeblockSession, natureText) == 122);
    assert!(size_of::<UsePokeblockMenu>() == 32876);
    assert!(offset_of!(UsePokeblockMenu, unused) == 0);
    assert!(offset_of!(UsePokeblockMenu, partyPalettes) == 4);
    assert!(offset_of!(UsePokeblockMenu, partySheets) == 772);
    assert!(offset_of!(UsePokeblockMenu, unusedBuffer) == 25348);
    assert!(offset_of!(UsePokeblockMenu, tilemapBuffer) == 29444);
    assert!(offset_of!(UsePokeblockMenu, selectionIconSpriteIds) == 31494);
    assert!(offset_of!(UsePokeblockMenu, curMonXOffset) == 31502);
    assert!(offset_of!(UsePokeblockMenu, curMonSpriteId) == 31504);
    assert!(offset_of!(UsePokeblockMenu, curMonPalette) == 31506);
    assert!(offset_of!(UsePokeblockMenu, curMonSheet) == 31508);
    assert!(offset_of!(UsePokeblockMenu, curMonTileStart) == 31512);
    assert!(offset_of!(UsePokeblockMenu, sparkles) == 31516);
    assert!(offset_of!(UsePokeblockMenu, condition) == 31556);
    assert!(offset_of!(UsePokeblockMenu, toLoadSelection) == 31564);
    assert!(offset_of!(UsePokeblockMenu, locationStrings) == 31565);
    assert!(offset_of!(UsePokeblockMenu, monNameStrings) == 31637);
    assert!(offset_of!(UsePokeblockMenu, graph) == 31832);
    assert!(offset_of!(UsePokeblockMenu, numSparkles) == 32688);
    assert!(offset_of!(UsePokeblockMenu, curLoadId) == 32691);
    assert!(offset_of!(UsePokeblockMenu, nextLoadId) == 32692);
    assert!(offset_of!(UsePokeblockMenu, prevLoadId) == 32693);
    assert!(offset_of!(UsePokeblockMenu, toLoadId) == 32694);
    assert!(offset_of!(UsePokeblockMenu, party) == 32696);
    assert!(offset_of!(UsePokeblockMenu, info) == 32720);
    assert!(size_of::<UsePokeblockMenuPokemon>() == 4);
    assert!(offset_of!(UsePokeblockMenuPokemon, boxId) == 0);
    assert!(offset_of!(UsePokeblockMenuPokemon, monId) == 1);
    assert!(offset_of!(UsePokeblockMenuPokemon, data) == 2);
};

const STATE_2: u8 = 2;
const STATE_4: u8 = 4;
const STATE_CLOSE: u8 = 3;
const STATE_CONFIRM_SELECTION: u8 = 5;
const STATE_HANDLE_CONFIRMATION: u8 = 6;
const STATE_HANDLE_INPUT: u8 = 0;
const STATE_UPDATE_SELECTION: u8 = 1;
const STATE_WAIT_MSG: u8 = 7;
const TAG_CONDITION: u16 = 1;
const TAG_UP_DOWN: u16 = 0;
const WIN_NAME: u8 = 0;
const WIN_NATURE: u8 = 1;
const WIN_TEXT: u8 = 2;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::use_pokeblock::sBgTemplates).cast());
static sConditionNames: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::use_pokeblock::sConditionNames).cast());
static sConditionToFlavor: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::use_pokeblock::sConditionToFlavor).cast());
static sConditionToMonData: Table<CArray<u32, 5>> =
    Table((&raw const crate::data::use_pokeblock::sConditionToMonData).cast());
static sGraphData_Tilemap: Table<CArray<u32, 41>> =
    Table((&raw const crate::data::use_pokeblock::sGraphData_Tilemap).cast());
static sMonFrame_Gfx: Table<CArray<u32, 56>> =
    Table((&raw const crate::data::use_pokeblock::sMonFrame_Gfx).cast());
static sMonFrame_Pal: Table<CArray<u32, 8>> =
    Table((&raw const crate::data::use_pokeblock::sMonFrame_Pal).cast());
static sMonFrame_Tilemap: Table<CArray<u32, 43>> =
    Table((&raw const crate::data::use_pokeblock::sMonFrame_Tilemap).cast());
static sNatureTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::use_pokeblock::sNatureTextColors).cast());
static sSpritePalette_Condition: Table<SpritePalette> =
    Table((&raw const crate::data::use_pokeblock::sSpritePalette_Condition).cast());
static sSpritePalette_UpDown: Table<SpritePalette> =
    Table((&raw const crate::data::use_pokeblock::sSpritePalette_UpDown).cast());
static sSpriteSheet_UpDown: Table<SpriteSheet> =
    Table((&raw const crate::data::use_pokeblock::sSpriteSheet_UpDown).cast());
static sSpriteTemplate_Condition: Table<SpriteTemplate> =
    Table((&raw const crate::data::use_pokeblock::sSpriteTemplate_Condition).cast());
static sSpriteTemplate_UpDown: Table<SpriteTemplate> =
    Table((&raw const crate::data::use_pokeblock::sSpriteTemplate_UpDown).cast());
static sUpDownCoordsOnGraph: Table<CArray<CArray<i16, 2>, 5>> =
    Table((&raw const crate::data::use_pokeblock::sUpDownCoordsOnGraph).cast());
static sUsePokeblockYesNoWinTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::use_pokeblock::sUsePokeblockYesNoWinTemplate).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::use_pokeblock::sWindowTemplates).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sInfo: crate::global::Global<*mut UsePokeblockSession> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sExitCallback: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokeblock: crate::global::Global<*mut Pokeblock> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPokeblockMonId: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPokeblockGain: crate::global::Global<i16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sGraph_Tilemap: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sGraph_Gfx: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMonFrame_TilemapPtr: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMenu: crate::global::Global<*mut UsePokeblockMenu> =
    crate::global::Global::new(null_mut());

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
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

pub unsafe fn ChooseMonToGivePokeblock(pokeblock: *mut Pokeblock, callback: Option<unsafe fn()>) {
    crate::gl!(sMenu = AllocZeroed(32876) as *mut UsePokeblockMenu);
    crate::gl!(sInfo = &raw mut (*crate::gl!(sMenu)).info);
    (*crate::gl!(sInfo)).pokeblock = pokeblock;
    (*crate::gl!(sInfo)).exitCallback = callback;
    SetUsePokeblockCallback(Some(LoadUsePokeblockMenu));
    SetMainCallback2(Some(CB2_UsePokeblockMenu));
}
pub(crate) unsafe fn CB2_ReturnAndChooseMonToGivePokeblock() {
    crate::gl!(sMenu = AllocZeroed(32876) as *mut UsePokeblockMenu);
    crate::gl!(sInfo = &raw mut (*crate::gl!(sMenu)).info);
    (*crate::gl!(sInfo)).pokeblock = crate::gl!(sPokeblock);
    (*crate::gl!(sInfo)).exitCallback = crate::gl!(sExitCallback);
    gPokeblockMonId.set(GetSelectionIdFromPartyId(gPokeblockMonId.get()));
    (*crate::gl!(sInfo)).monInTopHalf = (if gPokeblockMonId.get() <= 3 {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8;
    SetUsePokeblockCallback(Some(LoadUsePokeblockMenu));
    SetMainCallback2(Some(CB2_ReturnToUsePokeblockMenu));
}
pub(crate) unsafe fn CB2_ReturnToUsePokeblockMenu() {
    (*crate::gl!(sInfo)).callback.unwrap_unchecked()();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    if (*crate::gl!(sInfo)).callback == Some(ShowUsePokeblockMenu as unsafe fn()) {
        (*crate::gl!(sInfo)).mainState = 0;
        SetMainCallback2(Some(CB2_ShowUsePokeblockMenuForResults));
    }
}
pub(crate) unsafe fn CB2_ShowUsePokeblockMenuForResults() {
    ShowUsePokeblockMenuForResults();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn CB2_UsePokeblockMenu() {
    (*crate::gl!(sInfo)).callback.unwrap_unchecked()();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_UsePokeblockMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ConditionGraph_Draw(&raw mut (*crate::gl!(sMenu)).graph);
    ScanlineEffect_InitHBlankDmaTransfer();
}
unsafe fn SetUsePokeblockCallback(func: Option<unsafe fn()>) {
    (*crate::gl!(sInfo)).callback = func;
    (*crate::gl!(sInfo)).mainState = 0;
}
pub(crate) unsafe fn LoadUsePokeblockMenu() {
    match (*crate::gl!(sInfo)).mainState {
        0 => {
            (*crate::gl!(sMenu)).curMonSpriteId = SPRITE_NONE;
            ConditionGraph_Init(&raw mut (*crate::gl!(sMenu)).graph);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        1 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            (*crate::gl!(sInfo)).mainState += 1;
        }
        2 => {
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
            (*crate::gl!(sInfo)).mainState += 1;
        }
        3 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            LoadUserWindowBorderGfx(0, 0x97, 224);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        4 => {
            (*crate::gl!(sInfo)).mainState += 1;
        }
        5 => {
            if LoadConditionTitle() == 0 {
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        6 => {
            gKeyRepeatStartDelay.set(20);
            LoadPartyInfo();
            (*crate::gl!(sInfo)).mainState += 1;
        }
        7 => {
            if LoadUsePokeblockMenuGfx() == 0 {
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        8 => {
            UpdateMonPic(0);
            LoadAndCreateSelectionIcons();
            (*crate::gl!(sInfo)).mainState += 1;
        }
        9 => {
            if MoveConditionMonOnscreen(&raw mut (*crate::gl!(sMenu)).curMonXOffset) == 0 {
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        10 => {
            (*crate::gl!(sInfo)).mainState += 1;
        }
        11 => {
            ConditionGraph_CalcPositions(
                (*crate::gl!(sMenu)).graph.conditions[0].as_mut_ptr(),
                (*crate::gl!(sMenu)).graph.savedPositions[0].as_mut_ptr(),
            );
            ConditionGraph_InitResetScanline(&raw mut (*crate::gl!(sMenu)).graph);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        12 => {
            if ConditionGraph_ResetScanline(&raw mut (*crate::gl!(sMenu)).graph) == 0 {
                ConditionGraph_SetNewPositions(
                    &raw mut (*crate::gl!(sMenu)).graph,
                    (*crate::gl!(sMenu)).graph.savedPositions[0].as_mut_ptr(),
                    (*crate::gl!(sMenu)).graph.savedPositions[0].as_mut_ptr(),
                );
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        13 => {
            ConditionGraph_Update(&raw mut (*crate::gl!(sMenu)).graph);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        14 => {
            PutWindowTilemap(WIN_NAME);
            PutWindowTilemap(WIN_NATURE);
            UpdateMonInfoText(0, TRUE);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        15 => {
            SetUsePokeblockCallback(Some(ShowUsePokeblockMenu));
        }
        _ => {}
    }
}
pub(crate) unsafe fn ShowUsePokeblockMenu() {
    match (*crate::gl!(sInfo)).mainState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetVBlankCallback(Some(VBlankCB_UsePokeblockMenu));
            ShowBg(0);
            ShowBg(1);
            ShowBg(3);
            ShowBg(2);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        1 if (*gPaletteFade.as_ptr()).active() == 0 => {
            ResetConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
            if (*crate::gl!(sMenu)).info.curSelection as i32
                != (*crate::gl!(sMenu)).info.numSelections as i32 - 1
            {
                let numSparkles: u8 =
                    (*crate::gl!(sMenu)).numSparkles[(*crate::gl!(sMenu)).curLoadId];
                CreateConditionSparkleSprites(
                    (*crate::gl!(sMenu)).sparkles.as_mut_ptr(),
                    (*crate::gl!(sMenu)).curMonSpriteId,
                    numSparkles,
                );
            }
            SetUsePokeblockCallback(Some(UsePokeblockMenu));
        }
        _ => {}
    }
}
pub(crate) unsafe fn UsePokeblockMenu() {
    let mut loading: u8 = 0;
    match (*crate::gl!(sInfo)).mainState {
        STATE_HANDLE_INPUT => {
            if crate::gl!(gMain.heldKeys) as i32 & DPAD_UP != 0 {
                PlaySE(SE_SELECT);
                UpdateSelection(TRUE);
                DestroyConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
                (*crate::gl!(sInfo)).mainState = STATE_UPDATE_SELECTION;
            } else if crate::gl!(gMain.heldKeys) as i32 & DPAD_DOWN != 0 {
                PlaySE(SE_SELECT);
                UpdateSelection(FALSE);
                DestroyConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
                (*crate::gl!(sInfo)).mainState = STATE_UPDATE_SELECTION;
            } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*crate::gl!(sInfo)).mainState = STATE_CLOSE;
            } else if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*crate::gl!(sMenu)).info.curSelection as i32
                    == (*crate::gl!(sMenu)).info.numSelections as i32 - 1
                {
                    (*crate::gl!(sInfo)).mainState = STATE_CLOSE;
                } else {
                    (*crate::gl!(sInfo)).mainState = STATE_CONFIRM_SELECTION;
                }
            }
        }
        STATE_UPDATE_SELECTION => {
            loading = (*crate::gl!(sMenu))
                .info
                .loadNewSelection
                .unwrap_unchecked()();
            if loading == 0 {
                (*crate::gl!(sInfo)).mainState = STATE_HANDLE_INPUT;
            }
        }
        STATE_2 => {}
        STATE_CLOSE => {
            SetUsePokeblockCallback(Some(CloseUsePokeblockMenu));
        }
        STATE_4 => {}
        STATE_CONFIRM_SELECTION => {
            AskUsePokeblock();
            (*crate::gl!(sInfo)).mainState += 1;
        }
        STATE_HANDLE_CONFIRMATION => match HandleAskUsePokeblockInput() {
            1 | MENU_B_PRESSED => {
                (*crate::gl!(sInfo)).mainState = STATE_HANDLE_INPUT;
            }
            0 => {
                if IsSheenMaxed() != 0 {
                    PrintWontEatAnymore();
                    (*crate::gl!(sInfo)).mainState = STATE_WAIT_MSG;
                } else {
                    SetUsePokeblockCallback(Some(FeedPokeblockToMon));
                }
            }
            _ => {}
        },
        STATE_WAIT_MSG if crate::gl!(gMain.newKeys) as i32 & 3 != 0 => {
            EraseMenuWindow();
            (*crate::gl!(sInfo)).mainState = STATE_HANDLE_INPUT;
        }
        _ => {}
    }
}
pub(crate) unsafe fn FeedPokeblockToMon() {
    match (*crate::gl!(sInfo)).mainState {
        0 => {
            gPokeblockMonId.set(GetPartyIdFromSelectionId(
                (*crate::gl!(sMenu)).info.curSelection,
            ));
            crate::gl!(sExitCallback = (*crate::gl!(sInfo)).exitCallback);
            crate::gl!(sPokeblock = (*crate::gl!(sInfo)).pokeblock);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        1 if (*gPaletteFade.as_ptr()).active() == 0 => {
            SetVBlankCallback(None);
            Free(crate::gl!(sGraph_Tilemap) as *mut c_void);
            crate::gl!(sGraph_Tilemap = null_mut());
            Free(crate::gl!(sGraph_Gfx) as *mut c_void);
            crate::gl!(sGraph_Gfx = null_mut());
            Free(crate::gl!(sMonFrame_TilemapPtr) as *mut c_void);
            crate::gl!(sMonFrame_TilemapPtr = null_mut());
            Free(crate::gl!(sMenu) as *mut c_void);
            crate::gl!(sMenu = null_mut());
            FreeAllWindowBuffers();
            crate::gl!(gMain.savedCallback = Some(CB2_ReturnAndChooseMonToGivePokeblock));
            PreparePokeblockFeedScene();
        }
        _ => {}
    }
}
unsafe fn ShowUsePokeblockMenuForResults() {
    let mut loading: u8 = 0;
    match (*crate::gl!(sInfo)).mainState {
        0 => {
            if (*crate::gl!(sMenu)).info.curSelection != gPokeblockMonId.get() {
                UpdateSelection((*crate::gl!(sInfo)).monInTopHalf);
                (*crate::gl!(sInfo)).mainState += 1;
            } else {
                (*crate::gl!(sInfo)).mainState = 3;
            }
        }
        1 => {
            loading = (*crate::gl!(sMenu))
                .info
                .loadNewSelection
                .unwrap_unchecked()();
            if loading == 0 {
                (*crate::gl!(sInfo)).mainState = 0;
            }
        }
        2 => {}
        3 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        4 => {
            ShowBg(0);
            ShowBg(1);
            ShowBg(3);
            ShowBg(2);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        5 => {
            SetVBlankCallback(Some(VBlankCB_UsePokeblockMenu));
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        6 if (*gPaletteFade.as_ptr()).active() == 0 => {
            ResetConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
            SetUsePokeblockCallback(Some(ShowPokeblockResults));
            SetMainCallback2(Some(CB2_UsePokeblockMenu));
        }
        _ => {}
    }
}
pub(crate) unsafe fn ShowPokeblockResults() {
    match (*crate::gl!(sInfo)).mainState {
        0 => {
            (*crate::gl!(sInfo)).mon = (*gPlayerParty.as_ptr()).as_mut_ptr();
            (*crate::gl!(sInfo)).mon = (*crate::gl!(sInfo))
                .mon
                .at((*crate::gl!(sMenu)).party[(*crate::gl!(sMenu)).info.curSelection].monId);
            DestroyConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
            (*crate::gl!(sInfo)).mainState += 1;
        }
        1 => {
            if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        2 => {
            CalculateConditionEnhancements();
            ConditionGraph_CalcPositions(
                (*crate::gl!(sInfo)).conditionsAfterBlock.as_mut_ptr(),
                (*crate::gl!(sMenu)).graph.savedPositions[3].as_mut_ptr(),
            );
            ConditionGraph_SetNewPositions(
                &raw mut (*crate::gl!(sMenu)).graph,
                (*crate::gl!(sMenu)).graph.savedPositions[(*crate::gl!(sMenu)).curLoadId]
                    .as_mut_ptr(),
                (*crate::gl!(sMenu)).graph.savedPositions[3].as_mut_ptr(),
            );
            LoadAndCreateUpDownSprites();
            (*crate::gl!(sInfo)).mainState += 1;
        }
        3 => {
            if ConditionGraph_TryUpdate(&raw mut (*crate::gl!(sMenu)).graph) == 0 {
                CalculateNumAdditionalSparkles(GetPartyIdFromSelectionId(
                    (*crate::gl!(sMenu)).info.curSelection,
                ));
                if (*crate::gl!(sMenu)).info.curSelection as i32
                    != (*crate::gl!(sMenu)).info.numSelections as i32 - 1
                {
                    let numSparkles: u8 =
                        (*crate::gl!(sMenu)).numSparkles[(*crate::gl!(sMenu)).curLoadId];
                    CreateConditionSparkleSprites(
                        (*crate::gl!(sMenu)).sparkles.as_mut_ptr(),
                        (*crate::gl!(sMenu)).curMonSpriteId,
                        numSparkles,
                    );
                }
                (*crate::gl!(sInfo)).timer = 0;
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        4 => {
            if ({
                (*crate::gl!(sInfo)).timer += 1;
                (*crate::gl!(sInfo)).timer
            }) > 16
            {
                PrintFirstEnhancement();
                (*crate::gl!(sInfo)).mainState += 1;
            }
        }
        5 if crate::gl!(gMain.newKeys) as i32 & 3 != 0 && TryPrintNextEnhancement() == 0 => {
            TryClearPokeblock(gSpecialVar_ItemId.get() as u8);
            SetUsePokeblockCallback(Some(CloseUsePokeblockMenu));
        }
        _ => {}
    }
}
pub(crate) unsafe fn CloseUsePokeblockMenu() {
    match (*crate::gl!(sInfo)).mainState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sInfo)).mainState += 1;
        }
        1 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sInfo)).mainState = 2;
            }
        }
        2 => {
            crate::gl!(gScanlineEffect.state = 3);
            ScanlineEffect_InitHBlankDmaTransfer();
            (*crate::gl!(sInfo)).mainState += 1;
        }
        3 => {
            SetMainCallback2((*crate::gl!(sInfo)).exitCallback);
            FreeConditionSparkles((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
            for i in 0..7u8 {
                DestroySprite(
                    &raw mut (*gSprites.as_ptr())[(*crate::gl!(sMenu)).selectionIconSpriteIds[i]],
                );
            }
            FreeSpriteTilesByTag(TAG_UP_DOWN);
            FreeSpriteTilesByTag(TAG_CONDITION);
            FreeSpritePaletteByTag(TAG_UP_DOWN);
            FreeSpritePaletteByTag(TAG_CONDITION);
            for i in 0..2u8 {
                DestroySprite((*crate::gl!(sMenu)).condition[i]);
            }
            if (*crate::gl!(sMenu)).curMonSpriteId != SPRITE_NONE {
                DestroySprite(&raw mut (*gSprites.as_ptr())[(*crate::gl!(sMenu)).curMonSpriteId]);
            }
            SetVBlankCallback(None);
            Free(crate::gl!(sGraph_Tilemap) as *mut c_void);
            crate::gl!(sGraph_Tilemap = null_mut());
            Free(crate::gl!(sGraph_Gfx) as *mut c_void);
            crate::gl!(sGraph_Gfx = null_mut());
            Free(crate::gl!(sMonFrame_TilemapPtr) as *mut c_void);
            crate::gl!(sMonFrame_TilemapPtr = null_mut());
            Free(crate::gl!(sMenu) as *mut c_void);
            crate::gl!(sMenu = null_mut());
            FreeAllWindowBuffers();
        }
        _ => {}
    }
}
unsafe fn AskUsePokeblock() {
    let mut stringBuffer: CArray<u8, 64> = zeroed();
    GetMonData3(
        &raw mut (*gPlayerParty.as_ptr())
            [GetPartyIdFromSelectionId((*crate::gl!(sMenu)).info.curSelection)],
        MON_DATA_NICKNAME,
        stringBuffer.as_mut_ptr(),
    );
    StringGet_Nickname(stringBuffer.as_mut_ptr());
    StringAppend(
        stringBuffer.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_GetsAPokeBlockQuestion).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringCopy(gStringVar4.as_mut_ptr(), stringBuffer.as_mut_ptr());
    FillWindowPixelBuffer(WIN_TEXT, 17);
    DrawTextBorderOuter(WIN_TEXT, 151, 14);
    AddTextPrinterParameterized(
        WIN_TEXT,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
    CreateYesNoMenu(
        (&raw const *sUsePokeblockYesNoWinTemplate).cast_mut(),
        151,
        14,
        0,
    );
}
unsafe fn HandleAskUsePokeblockInput() -> i8 {
    let menuItem: i8 = Menu_ProcessInputNoWrapClearOnChoose();
    match menuItem {
        0 => {}
        MENU_B_PRESSED | 1 => {
            PlaySE(SE_SELECT);
            rbox_fill_rectangle(2);
            ClearWindowTilemap(2);
        }
        _ => {}
    }
    menuItem
}
unsafe fn PrintFirstEnhancement() {
    DrawTextBorderOuter(WIN_TEXT, 151, 14);
    FillWindowPixelBuffer(WIN_TEXT, 17);
    (*crate::gl!(sInfo)).condition = 0;
    while (*crate::gl!(sInfo)).condition < CONDITION_COUNT as u8 {
        if (*crate::gl!(sInfo)).enhancements[(*crate::gl!(sInfo)).condition] != 0 {
            break;
        }
        (*crate::gl!(sInfo)).condition += 1;
    }
    if (*crate::gl!(sInfo)).condition < CONDITION_COUNT as u8 {
        BufferEnhancedText(
            gStringVar4.as_mut_ptr(),
            (*crate::gl!(sInfo)).condition,
            (*crate::gl!(sInfo)).enhancements[(*crate::gl!(sInfo)).condition] as i16,
        );
    } else {
        BufferEnhancedText(gStringVar4.as_mut_ptr(), (*crate::gl!(sInfo)).condition, 0);
    }
    PrintMenuWindowText(gStringVar4.as_mut_ptr());
    PutWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
}
unsafe fn TryPrintNextEnhancement() -> u8 {
    FillWindowPixelBuffer(WIN_TEXT, 17);
    loop {
        (*crate::gl!(sInfo)).condition += 1;
        if (*crate::gl!(sInfo)).condition < CONDITION_COUNT as u8 {
            if (*crate::gl!(sInfo)).enhancements[(*crate::gl!(sInfo)).condition] != 0 {
                break;
            }
        } else {
            (*crate::gl!(sInfo)).condition = CONDITION_COUNT as u8;
            return FALSE;
        }
    }
    BufferEnhancedText(
        gStringVar4.as_mut_ptr(),
        (*crate::gl!(sInfo)).condition,
        (*crate::gl!(sInfo)).enhancements[(*crate::gl!(sInfo)).condition] as i16,
    );
    PrintMenuWindowText(gStringVar4.as_mut_ptr());
    CopyWindowToVram(WIN_TEXT, COPYWIN_GFX);
    TRUE
}
unsafe fn PrintWontEatAnymore() {
    FillWindowPixelBuffer(WIN_TEXT, 17);
    DrawTextBorderOuter(WIN_TEXT, 151, 14);
    AddTextPrinterParameterized(
        WIN_TEXT,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_WontEatAnymore).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
}
unsafe fn EraseMenuWindow() {
    rbox_fill_rectangle(WIN_TEXT);
    ClearWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
}
unsafe fn PrintMenuWindowText(message: *mut u8) {
    AddTextPrinterParameterized(
        WIN_TEXT,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        0,
        None,
    );
}
unsafe fn BufferEnhancedText(dest: *mut u8, condition: u8, mut enhancement: i16) {
    'l1: {
        let sw1: i16 = enhancement;
        let mut fall = false;
        if (1..=32767).contains(&sw1) {
            fall = true;
            enhancement = 0;
        }
        if fall || (-32768..=-1).contains(&sw1) {
            if enhancement != 0 {
                *dest.at(enhancement as u16) += 0;
            }
            StringCopy(dest, sConditionNames[condition]);
            StringAppend(
                dest,
                (*(&raw const crate::data::strings::gText_WasEnhanced).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            break 'l1;
        }
        if sw1 == 0 {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_NothingChanged).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            break 'l1;
        }
    }
}
unsafe fn GetMonConditions(mon: *mut Pokemon, data: *mut u8) {
    for i in 0..CONDITION_COUNT {
        *data.at(i) = GetMonData2(mon, sConditionToMonData[i] as i32) as u8;
    }
}
unsafe fn AddPokeblockToConditions(pokeblock: *mut Pokeblock, mon: *mut Pokemon) {
    let mut stat: i16 = 0;
    let mut data: u8 = 0;
    if GetMonData2(mon, MON_DATA_SHEEN) != MAX_SHEEN as u32 {
        CalculatePokeblockEffectiveness(pokeblock, mon);
        for i in 0..CONDITION_COUNT {
            data = GetMonData2(mon, sConditionToMonData[i] as i32) as u8;
            stat = data as i16 + (*crate::gl!(sInfo)).pokeblockStatBoosts[i];
            if stat < 0 {
                stat = 0;
            }
            if stat > MAX_CONDITION {
                stat = MAX_CONDITION;
            }
            data = stat as u8;
            SetMonData(
                mon,
                sConditionToMonData[i] as i32,
                &raw mut data as *mut c_void,
            );
        }
        stat = GetMonData2(mon, MON_DATA_SHEEN) as u8 as i16 + (*pokeblock).feel as i16;
        if stat > MAX_SHEEN {
            stat = MAX_SHEEN;
        }
        data = stat as u8;
        SetMonData(mon, MON_DATA_SHEEN, &raw mut data as *mut c_void);
    }
}
unsafe fn CalculateConditionEnhancements() {
    let mut mon: *mut Pokemon = (*gPlayerParty.as_ptr()).as_mut_ptr();
    mon = mon.at((*crate::gl!(sMenu)).party[(*crate::gl!(sMenu)).info.curSelection].monId);
    GetMonConditions(mon, (*crate::gl!(sInfo)).conditionsBeforeBlock.as_mut_ptr());
    AddPokeblockToConditions((*crate::gl!(sInfo)).pokeblock, mon);
    GetMonConditions(mon, (*crate::gl!(sInfo)).conditionsAfterBlock.as_mut_ptr());
    for i in 0..CONDITION_COUNT {
        (*crate::gl!(sInfo)).enhancements[i] = (*crate::gl!(sInfo)).conditionsAfterBlock[i]
            - (*crate::gl!(sInfo)).conditionsBeforeBlock[i];
    }
}
unsafe fn CalculatePokeblockEffectiveness(pokeblock: *mut Pokeblock, mon: *mut Pokemon) {
    let mut direction: i8 = 0;
    let mut flavor: i8 = 0;
    (*crate::gl!(sInfo)).pokeblockStatBoosts[0] = (*pokeblock).spicy as i16;
    (*crate::gl!(sInfo)).pokeblockStatBoosts[1] = (*pokeblock).sour as i16;
    (*crate::gl!(sInfo)).pokeblockStatBoosts[2] = (*pokeblock).bitter as i16;
    (*crate::gl!(sInfo)).pokeblockStatBoosts[3] = (*pokeblock).sweet as i16;
    (*crate::gl!(sInfo)).pokeblockStatBoosts[4] = (*pokeblock).dry as i16;
    if gPokeblockGain.get() > 0 {
        direction = 1;
    } else if gPokeblockGain.get() < 0 {
        direction = -1;
    } else {
        return;
    }
    for i in 0..(CONDITION_COUNT as i8) {
        let amount: i16 = (*crate::gl!(sInfo)).pokeblockStatBoosts[i];
        let mut boost: i8 = (amount / 10) as i8;
        if amount % 10 >= 5 {
            boost += 1;
        }
        flavor = GetMonFlavorRelation(mon, sConditionToFlavor[i]);
        if flavor == direction {
            (*crate::gl!(sInfo)).pokeblockStatBoosts[i] += boost as i16 * flavor as i16;
        }
    }
}
unsafe fn IsSheenMaxed() -> u8 {
    if GetBoxOrPartyMonData(
        (*crate::gl!(sMenu)).party[(*crate::gl!(sMenu)).info.curSelection].boxId as u16,
        (*crate::gl!(sMenu)).party[(*crate::gl!(sMenu)).info.curSelection].monId as u16,
        MON_DATA_SHEEN,
        null_mut(),
    ) == MAX_SHEEN as i32
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
unsafe fn GetPartyIdFromSelectionId(mut selectionId: u8) -> u8 {
    for i in 0..(PARTY_SIZE as u8) {
        if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_IS_EGG) == 0 {
            if selectionId == 0 {
                return i;
            }
            selectionId -= 1;
        }
    }
    0
}
unsafe fn GetSelectionIdFromPartyId(partyId: u8) -> u8 {
    let mut numEggs: u8 = 0;
    for i in 0..partyId {
        if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_IS_EGG) != 0 {
            numEggs += 1;
        }
    }
    partyId - numEggs
}
unsafe fn GetPartyIdFromSelectionId_(selectionId: u8) -> u8 {
    GetPartyIdFromSelectionId(selectionId)
}
unsafe fn LoadAndCreateUpDownSprites() {
    LoadSpriteSheet((&raw const *sSpriteSheet_UpDown).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_UpDown).cast_mut());
    (*crate::gl!(sInfo)).numEnhancements = 0;
    for i in 0..CONDITION_COUNT {
        if (*crate::gl!(sInfo)).enhancements[i] != 0 {
            let spriteId: u16 = CreateSprite(
                (&raw const *sSpriteTemplate_UpDown).cast_mut(),
                sUpDownCoordsOnGraph[i][0],
                sUpDownCoordsOnGraph[i][1],
                0,
            ) as u16;
            if spriteId != MAX_SPRITES as u16 {
                if (*crate::gl!(sInfo)).enhancements[i] != 0 {
                    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_UpDown));
                }
                (*crate::gl!(sInfo)).numEnhancements += 1;
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_UpDown(sprite: *mut Sprite) {
    if (*sprite).data[tTimer] < 6 {
        (*sprite).y2 -= 2;
    } else if (*sprite).data[tTimer] < 12 {
        (*sprite).y2 += 2;
    }
    if ({
        (*sprite).data[tTimer] += 1;
        (*sprite).data[tTimer]
    }) > 60
    {
        DestroySprite(sprite);
        (*crate::gl!(sInfo)).numEnhancements -= 1;
    }
}
unsafe fn LoadPartyInfo() {
    let mut i: u16 = 0;
    let mut numMons: u16 = 0;
    while i < CalculatePlayerPartyCount() as u16 {
        if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_IS_EGG) == 0 {
            (*crate::gl!(sMenu)).party[numMons].boxId = TOTAL_BOXES_COUNT;
            (*crate::gl!(sMenu)).party[numMons].monId = i as u8;
            (*crate::gl!(sMenu)).party[numMons].data = 0;
            numMons += 1;
        }
        i += 1;
    }
    (*crate::gl!(sMenu)).info.curSelection = 0;
    (*crate::gl!(sMenu)).info.numSelections = numMons as u8 + 1;
    LoadInitialMonInfo();
}
unsafe fn LoadInitialMonInfo() {
    LoadMonInfo((*crate::gl!(sMenu)).info.curSelection as i16, 0);
    (*crate::gl!(sMenu)).curLoadId = 0;
    (*crate::gl!(sMenu)).nextLoadId = 1;
    (*crate::gl!(sMenu)).prevLoadId = 2;
    let mut nextSelection: i16 = (*crate::gl!(sMenu)).info.curSelection as i16 + 1;
    if nextSelection >= (*crate::gl!(sMenu)).info.numSelections as i16 {
        nextSelection = 0;
    }
    let mut prevSelection: i16 = (*crate::gl!(sMenu)).info.curSelection as i16 - 1;
    if prevSelection < 0 {
        prevSelection = (*crate::gl!(sMenu)).info.numSelections as i16 - 1;
    }
    LoadMonInfo(nextSelection, 1);
    LoadMonInfo(prevSelection, 2);
}
unsafe fn LoadMonInfo(partyId: i16, loadId: u8) {
    let boxId: u8 = (*crate::gl!(sMenu)).party[partyId].boxId;
    let monId: u8 = (*crate::gl!(sMenu)).party[partyId].monId;
    let numSelections: u8 = (*crate::gl!(sMenu)).info.numSelections;
    let excludesCancel: u8 = FALSE;
    GetConditionMenuMonNameAndLocString(
        (*crate::gl!(sMenu)).locationStrings[loadId].as_mut_ptr(),
        (*crate::gl!(sMenu)).monNameStrings[loadId].as_mut_ptr(),
        boxId as u16,
        monId as u16,
        partyId as u16,
        numSelections as u16,
        excludesCancel,
    );
    GetConditionMenuMonConditions(
        &raw mut (*crate::gl!(sMenu)).graph,
        (*crate::gl!(sMenu)).numSparkles.as_mut_ptr(),
        boxId as u16,
        monId as u16,
        partyId as u16,
        loadId as u16,
        numSelections as u16,
        excludesCancel,
    );
    GetConditionMenuMonGfx(
        (*crate::gl!(sMenu)).partySheets[loadId].as_mut_ptr() as *mut c_void,
        (*crate::gl!(sMenu)).partyPalettes[loadId].as_mut_ptr() as *mut c_void,
        boxId as u16,
        monId as u16,
        partyId as u16,
        numSelections as u16,
        excludesCancel,
    );
}
unsafe fn UpdateMonPic(loadId: u8) {
    let mut spriteId: u8 = 0;
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteSheet: SpriteSheet = zeroed();
    let mut spritePal: SpritePalette = zeroed();
    if (*crate::gl!(sMenu)).curMonSpriteId == SPRITE_NONE {
        LoadConditionMonPicTemplate(
            &raw mut spriteSheet,
            &raw mut spriteTemplate,
            &raw mut spritePal,
        );
        spriteSheet.data = (*crate::gl!(sMenu)).partySheets[loadId].as_mut_ptr() as *mut c_void;
        spritePal.data = (*crate::gl!(sMenu)).partyPalettes[loadId].as_mut_ptr();
        (*crate::gl!(sMenu)).curMonPalette = LoadSpritePalette(&raw mut spritePal) as u16;
        (*crate::gl!(sMenu)).curMonSheet = LoadSpriteSheet(&raw mut spriteSheet);
        spriteId = CreateSprite(&raw mut spriteTemplate, 38, 104, 0);
        (*crate::gl!(sMenu)).curMonSpriteId = spriteId;
        if spriteId == MAX_SPRITES {
            FreeSpriteTilesByTag(TAG_CONDITION_MON);
            FreeSpritePaletteByTag(TAG_CONDITION_MON);
            (*crate::gl!(sMenu)).curMonSpriteId = SPRITE_NONE;
        } else {
            (*crate::gl!(sMenu)).curMonSpriteId = spriteId;
            (*gSprites.as_ptr())[(*crate::gl!(sMenu)).curMonSpriteId].callback =
                Some(SpriteCB_MonPic);
            (*gSprites.as_ptr())[(*crate::gl!(sMenu)).curMonSpriteId].y2 -= 34;
            (*crate::gl!(sMenu)).curMonTileStart =
                (OBJ_VRAM0 + (*crate::gl!(sMenu)).curMonSheet as i32 * 32) as usize as *mut c_void
                    as *mut u8;
            (*crate::gl!(sMenu)).curMonPalette = 0x100 + (*crate::gl!(sMenu)).curMonPalette * 16;
        }
    } else {
        {
            let mut _src: *mut c_void =
                (*crate::gl!(sMenu)).partySheets[loadId].as_mut_ptr() as *mut c_void;
            let mut _dest: *mut c_void = (*crate::gl!(sMenu)).curMonTileStart as *mut c_void;
            let mut _size: u32 = MON_PIC_SIZE as u32;
            loop {
                if _size <= 0x1000 {
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    break;
                }
                {
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
                _src = (_src as *mut u8).at(4096) as *mut c_void;
                _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                _size -= 0x1000;
            }
        }
        LoadPalette(
            (*crate::gl!(sMenu)).partyPalettes[loadId].as_mut_ptr() as *mut c_void,
            (*crate::gl!(sMenu)).curMonPalette,
            32,
        );
    }
}
unsafe fn LoadAndCreateSelectionIcons() {
    let mut spriteId: u16 = 0;
    let mut spriteSheets: CArray<SpriteSheet, 4> = zeroed();
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spritePals: CArray<SpritePalette, 3> = zeroed();
    let mut spriteSheet2: SpriteSheet = zeroed();
    let mut spritePal2: SpritePalette = zeroed();
    LoadConditionSelectionIcons(
        spriteSheets.as_mut_ptr(),
        &raw mut spriteTemplate,
        spritePals.as_mut_ptr(),
    );
    LoadSpriteSheets(spriteSheets.as_mut_ptr());
    LoadSpritePalettes(spritePals.as_mut_ptr());
    let mut i: u16 = 0;
    while (i as i32) < (*crate::gl!(sMenu)).info.numSelections as i32 - 1 {
        spriteId = CreateSprite(&raw mut spriteTemplate, 226, i as i16 * 20 + 8, 0) as u16;
        if spriteId != MAX_SPRITES as u16 {
            (*crate::gl!(sMenu)).selectionIconSpriteIds[i] = spriteId as u8;
            crate::gl!(gSprites[spriteId].data[0] = i as i16);
            crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_SelectionIconPokeball));
        } else {
            (*crate::gl!(sMenu)).selectionIconSpriteIds[i] = 255;
        }
        i += 1;
    }
    spriteTemplate.tileTag = TAG_CONDITION_BALL_PLACEHOLDER;
    while i < PARTY_SIZE as u16 {
        spriteId = CreateSprite(&raw mut spriteTemplate, 230, i as i16 * 20 + 8, 0) as u16;
        if spriteId != MAX_SPRITES as u16 {
            (*crate::gl!(sMenu)).selectionIconSpriteIds[i] = spriteId as u8;
            (*gSprites.as_ptr())[spriteId].oam.set_size(0);
        } else {
            (*crate::gl!(sMenu)).selectionIconSpriteIds[i] = 255;
        }
        i += 1;
    }
    spriteTemplate.tileTag = TAG_CONDITION_CANCEL;
    spriteTemplate.callback = Some(SpriteCB_SelectionIconCancel);
    spriteId = CreateSprite(&raw mut spriteTemplate, 222, i as i16 * 20 + 8, 0) as u16;
    if spriteId != MAX_SPRITES as u16 {
        (*crate::gl!(sMenu)).selectionIconSpriteIds[i] = spriteId as u8;
        (*gSprites.as_ptr())[spriteId].oam.set_shape(1);
        (*gSprites.as_ptr())[spriteId].oam.set_size(2);
    } else {
        (*crate::gl!(sMenu)).selectionIconSpriteIds[i] = 255;
    }
    LoadConditionSparkle(&raw mut spriteSheet2, &raw mut spritePal2);
    LoadSpriteSheet(&raw mut spriteSheet2);
    LoadSpritePalette(&raw mut spritePal2);
}
unsafe fn LoadUsePokeblockMenuGfx() -> u8 {
    match (*crate::gl!(sMenu)).info.helperState {
        0 => {
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 8704, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_DISPCNT, 28736);
            SetGpuReg(REG_OFFSET_BLDCNT, 580);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1035);
        }
        1 => {
            crate::gl!(sGraph_Gfx = Alloc(6656) as *mut u8);
            crate::gl!(sGraph_Tilemap = Alloc(1280) as *mut u8);
            crate::gl!(sMonFrame_TilemapPtr = Alloc(1280) as *mut u8);
        }
        2 => {
            LZ77UnCompVram(
                sMonFrame_Tilemap.as_ptr().cast_mut(),
                crate::gl!(sMonFrame_TilemapPtr) as *mut c_void,
            );
        }
        3 => {
            LoadBgTiles(3, sMonFrame_Gfx.as_ptr().cast_mut() as *mut c_void, 224, 0);
        }
        4 => {
            LoadBgTilemap(3, crate::gl!(sMonFrame_TilemapPtr) as *mut c_void, 1280, 0);
        }
        5 => {
            LoadPalette(sMonFrame_Pal.as_ptr().cast_mut() as *mut c_void, 208, 32);
            (*crate::gl!(sMenu)).curMonXOffset = -80;
        }
        6 => {
            LZ77UnCompVram(
                (*(&raw const crate::data::graphics::gUsePokeblockGraph_Gfx)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                crate::gl!(sGraph_Gfx) as *mut c_void,
            );
        }
        7 => {
            LZ77UnCompVram(
                (*(&raw const crate::data::graphics::gUsePokeblockGraph_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                crate::gl!(sGraph_Tilemap) as *mut c_void,
            );
            LoadPalette(
                (*(&raw const crate::data::graphics::gUsePokeblockGraph_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                32,
                32,
            );
        }
        8 => {
            LoadBgTiles(1, crate::gl!(sGraph_Gfx) as *mut c_void, 6656, 640);
        }
        9 => {
            SetBgTilemapBuffer(1, crate::gl!(sGraph_Tilemap) as *mut c_void);
            CopyToBgTilemapBufferRect(
                1,
                (*(&raw const crate::data::graphics::gUsePokeblockNatureWin_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                13,
                12,
                4,
            );
            CopyBgTilemapBufferToVram(1);
        }
        10 => {
            LZ77UnCompVram(
                sGraphData_Tilemap.as_ptr().cast_mut(),
                (*crate::gl!(sMenu)).tilemapBuffer.as_mut_ptr() as *mut c_void,
            );
        }
        11 => {
            LoadBgTilemap(
                2,
                (*crate::gl!(sMenu)).tilemapBuffer.as_mut_ptr() as *mut c_void,
                1280,
                0,
            );
            LoadPalette(
                (*(&raw const crate::data::pokenav_conditions_gfx::gConditionGraphData_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                48,
                32,
            );
            LoadPalette(
                (*(&raw const crate::data::pokenav_conditions_gfx::gConditionText_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                240,
                32,
            );
            ConditionGraph_InitWindow(2);
        }
        _ => {
            (*crate::gl!(sMenu)).info.helperState = 0;
            return FALSE;
        }
    }
    (*crate::gl!(sMenu)).info.helperState += 1;
    TRUE
}
unsafe fn UpdateMonInfoText(loadId: u16, firstPrint: u8) {
    let mut partyIndex: u8 = 0;
    let mut nature: u8 = 0;
    let mut str: *mut u8 = null_mut();
    FillWindowPixelBuffer(WIN_NAME, 0);
    FillWindowPixelBuffer(WIN_NATURE, 0);
    if (*crate::gl!(sMenu)).info.curSelection as i32
        != (*crate::gl!(sMenu)).info.numSelections as i32 - 1
    {
        AddTextPrinterParameterized(
            WIN_NAME,
            FONT_NORMAL,
            (*crate::gl!(sMenu)).monNameStrings[loadId].as_mut_ptr(),
            0,
            1,
            0,
            None,
        );
        partyIndex = GetPartyIdFromSelectionId((*crate::gl!(sMenu)).info.curSelection);
        nature = GetNature(crate::gl!(&raw gPlayerParty[partyIndex]));
        str = StringCopy(
            (*crate::gl!(sMenu)).info.natureText.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_NatureSlash).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        str =
            StringCopy(
                str,
                (*(&raw const crate::data::pokemon_summary_screen::gNatureNamePointers)
                    .cast::<CArray<*mut u8, 0>>())[nature],
            );
        AddTextPrinterParameterized3(
            WIN_NATURE,
            FONT_NORMAL,
            2,
            1,
            sNatureTextColors.as_ptr().cast_mut(),
            0,
            (*crate::gl!(sMenu)).info.natureText.as_mut_ptr(),
        );
    }
    if firstPrint != 0 {
        CopyWindowToVram(WIN_NAME, COPYWIN_FULL);
        CopyWindowToVram(WIN_NATURE, COPYWIN_FULL);
    } else {
        CopyWindowToVram(WIN_NAME, COPYWIN_GFX);
        CopyWindowToVram(WIN_NATURE, COPYWIN_GFX);
    }
}
unsafe fn UpdateSelection(up: u8) {
    let mut newLoadId: u16 = 0;
    let mut startedOnMon: u32 = 0;
    let mut endedOnMon: u32 = 0;
    if up != 0 {
        newLoadId = (*crate::gl!(sMenu)).prevLoadId as u16;
    } else {
        newLoadId = (*crate::gl!(sMenu)).nextLoadId as u16;
    }
    ConditionGraph_SetNewPositions(
        &raw mut (*crate::gl!(sMenu)).graph,
        (*crate::gl!(sMenu)).graph.savedPositions[(*crate::gl!(sMenu)).curLoadId].as_mut_ptr(),
        (*crate::gl!(sMenu)).graph.savedPositions[newLoadId].as_mut_ptr(),
    );
    if (*crate::gl!(sMenu)).info.curSelection as i32
        == (*crate::gl!(sMenu)).info.numSelections as i32 - 1
    {
        startedOnMon = FALSE as u32;
    } else {
        startedOnMon = TRUE as u32;
    }
    if up != 0 {
        (*crate::gl!(sMenu)).prevLoadId = (*crate::gl!(sMenu)).nextLoadId;
        (*crate::gl!(sMenu)).nextLoadId = (*crate::gl!(sMenu)).curLoadId;
        (*crate::gl!(sMenu)).curLoadId = newLoadId as i8;
        (*crate::gl!(sMenu)).toLoadId = (*crate::gl!(sMenu)).prevLoadId;
        (*crate::gl!(sMenu)).info.curSelection = (if (*crate::gl!(sMenu)).info.curSelection == 0 {
            (*crate::gl!(sMenu)).info.numSelections as i32 - 1
        } else {
            (*crate::gl!(sMenu)).info.curSelection as i32 - 1
        }) as u8;
        (*crate::gl!(sMenu)).toLoadSelection = (if (*crate::gl!(sMenu)).info.curSelection == 0 {
            (*crate::gl!(sMenu)).info.numSelections as i32 - 1
        } else {
            (*crate::gl!(sMenu)).info.curSelection as i32 - 1
        }) as u8;
    } else {
        (*crate::gl!(sMenu)).nextLoadId = (*crate::gl!(sMenu)).prevLoadId;
        (*crate::gl!(sMenu)).prevLoadId = (*crate::gl!(sMenu)).curLoadId;
        (*crate::gl!(sMenu)).curLoadId = newLoadId as i8;
        (*crate::gl!(sMenu)).toLoadId = (*crate::gl!(sMenu)).nextLoadId;
        (*crate::gl!(sMenu)).info.curSelection = (if ((*crate::gl!(sMenu)).info.curSelection as i32)
            < (*crate::gl!(sMenu)).info.numSelections as i32 - 1
        {
            (*crate::gl!(sMenu)).info.curSelection as i32 + 1
        } else {
            0
        }) as u8;
        (*crate::gl!(sMenu)).toLoadSelection = (if ((*crate::gl!(sMenu)).info.curSelection as i32)
            < (*crate::gl!(sMenu)).info.numSelections as i32 - 1
        {
            (*crate::gl!(sMenu)).info.curSelection as i32 + 1
        } else {
            0
        }) as u8;
    }
    if (*crate::gl!(sMenu)).info.curSelection as i32
        == (*crate::gl!(sMenu)).info.numSelections as i32 - 1
    {
        endedOnMon = FALSE as u32;
    } else {
        endedOnMon = TRUE as u32;
    }
    DestroyConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
    if startedOnMon == 0 {
        (*crate::gl!(sMenu)).info.loadNewSelection = Some(LoadNewSelection_CancelToMon);
    } else if endedOnMon == 0 {
        (*crate::gl!(sMenu)).info.loadNewSelection = Some(LoadNewSelection_MonToCancel);
    } else {
        (*crate::gl!(sMenu)).info.loadNewSelection = Some(LoadNewSelection_MonToMon);
    }
}
pub(crate) unsafe fn LoadNewSelection_CancelToMon() -> u8 {
    match (*crate::gl!(sMenu)).info.helperState {
        0 => {
            UpdateMonPic((*crate::gl!(sMenu)).curLoadId as u8);
            (*crate::gl!(sMenu)).info.helperState += 1;
        }
        1 => {
            UpdateMonInfoText((*crate::gl!(sMenu)).curLoadId as u16, FALSE);
            (*crate::gl!(sMenu)).info.helperState += 1;
        }
        2 => {
            if ConditionMenu_UpdateMonEnter(
                &raw mut (*crate::gl!(sMenu)).graph,
                &raw mut (*crate::gl!(sMenu)).curMonXOffset,
            ) == 0
            {
                LoadMonInfo(
                    (*crate::gl!(sMenu)).toLoadSelection as i16,
                    (*crate::gl!(sMenu)).toLoadId as u8,
                );
                (*crate::gl!(sMenu)).info.helperState += 1;
            }
        }
        3 => {
            ResetConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
            if (*crate::gl!(sMenu)).info.curSelection as i32
                != (*crate::gl!(sMenu)).info.numSelections as i32 - 1
            {
                let numSparkles: u8 =
                    (*crate::gl!(sMenu)).numSparkles[(*crate::gl!(sMenu)).curLoadId];
                CreateConditionSparkleSprites(
                    (*crate::gl!(sMenu)).sparkles.as_mut_ptr(),
                    (*crate::gl!(sMenu)).curMonSpriteId,
                    numSparkles,
                );
            }
            (*crate::gl!(sMenu)).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    TRUE
}
pub(crate) unsafe fn LoadNewSelection_MonToCancel() -> u8 {
    match (*crate::gl!(sMenu)).info.helperState {
        0 => {
            if ConditionMenu_UpdateMonExit(
                &raw mut (*crate::gl!(sMenu)).graph,
                &raw mut (*crate::gl!(sMenu)).curMonXOffset,
            ) == 0
            {
                (*crate::gl!(sMenu)).info.helperState += 1;
            }
        }
        1 => {
            UpdateMonInfoText((*crate::gl!(sMenu)).curLoadId as u16, FALSE);
            (*crate::gl!(sMenu)).info.helperState += 1;
        }
        2 => {
            LoadMonInfo(
                (*crate::gl!(sMenu)).toLoadSelection as i16,
                (*crate::gl!(sMenu)).toLoadId as u8,
            );
            (*crate::gl!(sMenu)).info.helperState += 1;
        }
        3 => {
            (*crate::gl!(sMenu)).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    TRUE
}
pub(crate) unsafe fn LoadNewSelection_MonToMon() -> u8 {
    match (*crate::gl!(sMenu)).info.helperState {
        0 => {
            ConditionGraph_TryUpdate(&raw mut (*crate::gl!(sMenu)).graph);
            if MoveConditionMonOffscreen(&raw mut (*crate::gl!(sMenu)).curMonXOffset) == 0 {
                UpdateMonPic((*crate::gl!(sMenu)).curLoadId as u8);
                (*crate::gl!(sMenu)).info.helperState += 1;
            }
        }
        1 => {
            UpdateMonInfoText((*crate::gl!(sMenu)).curLoadId as u16, FALSE);
            (*crate::gl!(sMenu)).info.helperState += 1;
        }
        2 => {
            if ConditionMenu_UpdateMonEnter(
                &raw mut (*crate::gl!(sMenu)).graph,
                &raw mut (*crate::gl!(sMenu)).curMonXOffset,
            ) == 0
            {
                LoadMonInfo(
                    (*crate::gl!(sMenu)).toLoadSelection as i16,
                    (*crate::gl!(sMenu)).toLoadId as u8,
                );
                (*crate::gl!(sMenu)).info.helperState += 1;
            }
        }
        3 => {
            ResetConditionSparkleSprites((*crate::gl!(sMenu)).sparkles.as_mut_ptr());
            if (*crate::gl!(sMenu)).info.curSelection as i32
                != (*crate::gl!(sMenu)).info.numSelections as i32 - 1
            {
                let numSparkles: u8 =
                    (*crate::gl!(sMenu)).numSparkles[(*crate::gl!(sMenu)).curLoadId];
                CreateConditionSparkleSprites(
                    (*crate::gl!(sMenu)).sparkles.as_mut_ptr(),
                    (*crate::gl!(sMenu)).curMonSpriteId,
                    numSparkles,
                );
            }
            (*crate::gl!(sMenu)).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    TRUE
}
pub(crate) unsafe fn SpriteCB_MonPic(sprite: *mut Sprite) {
    (*sprite).x = (*crate::gl!(sMenu)).curMonXOffset + 38;
}
pub(crate) unsafe fn SpriteCB_SelectionIconPokeball(sprite: *mut Sprite) {
    if (*sprite).data[0] == (*crate::gl!(sMenu)).info.curSelection as i16 {
        StartSpriteAnim(sprite, CONDITION_ICON_SELECTED);
    } else {
        StartSpriteAnim(sprite, CONDITION_ICON_UNSELECTED);
    }
}
pub(crate) unsafe fn SpriteCB_SelectionIconCancel(sprite: *mut Sprite) {
    if (*crate::gl!(sMenu)).info.curSelection as i32
        == (*crate::gl!(sMenu)).info.numSelections as i32 - 1
    {
        (*sprite)
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(TAG_CONDITION_BALL) as u16);
    } else {
        (*sprite)
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(TAG_CONDITION_CANCEL) as u16);
    }
}
unsafe fn CalculateNumAdditionalSparkles(monIndex: u8) {
    let sheen: u8 = GetMonData2(crate::gl!(&raw gPlayerParty[monIndex]), MON_DATA_SHEEN) as u8;
    (*crate::gl!(sMenu)).numSparkles[(*crate::gl!(sMenu)).curLoadId] =
        (if sheen != 255 { (sheen / 29) as i32 } else { 9 }) as u8;
}
unsafe fn LoadConditionGfx() {
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    let mut spritePalette: SpritePalette = *sSpritePalette_Condition;
    spriteSheet.data = (*(&raw const crate::data::graphics::gUsePokeblockCondition_Gfx)
        .cast::<CArray<u32, 0>>())
    .as_ptr()
    .cast_mut();
    spriteSheet.size = 0x800;
    spriteSheet.tag = TAG_CONDITION;
    LoadCompressedSpriteSheet(&raw mut spriteSheet);
    LoadSpritePalette(&raw mut spritePalette);
}
unsafe fn CreateConditionSprite() {
    let yStart: i32 = 17;
    let speed: i32 = 8;
    let sprites: *mut *mut Sprite = (*crate::gl!(sMenu)).condition.as_mut_ptr();
    let template: *mut SpriteTemplate = (&raw const *sSpriteTemplate_Condition).cast_mut();
    let xDiff: i16 = 64;
    let xStart: i16 = -96;
    for i in 0..2u16 {
        let spriteId: u8 = CreateSprite(template, i as i16 * xDiff + xStart, yStart as i16, 0);
        if spriteId != MAX_SPRITES {
            {
                let gl_i0 = sSpeed;
                crate::gl!(gSprites[spriteId].data[gl_i0] = speed as i16)
            };
            {
                let gl_i1 = sTargetX;
                crate::gl!(gSprites[spriteId].data[gl_i1] = (i as i16 * xDiff) | 0x20)
            };
            crate::gl!(gSprites[spriteId].data[2] = i as i16);
            StartSpriteAnim(crate::gl!(&raw gSprites[spriteId]), i as u8);
            *sprites.at(i) = crate::gl!(&raw gSprites[spriteId]);
        }
    }
}
unsafe fn LoadConditionTitle() -> u8 {
    match (*crate::gl!(sMenu)).info.helperState {
        0 => {
            LoadConditionGfx();
            (*crate::gl!(sMenu)).info.helperState += 1;
            return TRUE;
        }
        1 => {
            CreateConditionSprite();
            (*crate::gl!(sMenu)).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    FALSE
}
pub(crate) unsafe fn SpriteCB_Condition(sprite: *mut Sprite) {
    let prevX: i16 = (*sprite).x;
    (*sprite).x += (*sprite).data[sSpeed];
    if prevX <= (*sprite).data[sTargetX] && (*sprite).x >= (*sprite).data[sTargetX]
        || prevX >= (*sprite).data[sTargetX] && (*sprite).x <= (*sprite).data[sTargetX]
    {
        (*sprite).x = (*sprite).data[sTargetX];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
