//! Translated from `src/use_pokeblock.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMonFrame_Pal sMonFrame_Gfx sMonFrame_Tilemap sGraphData_Tilemap sConditionToMonData sConditionToFlavor sNatureTextColors sBgTemplates sWindowTemplates sUsePokeblockYesNoWinTemplate sConditionNames sSpriteSheet_UpDown sSpritePalette_UpDown sUpDownCoordsOnGraph sOam_UpDown sAnim_Up sAnim_Down sAnims_UpDown sSpriteTemplate_UpDown sOam_Condition sAnim_Condition_0 sAnim_Condition_1 sAnim_Condition_2 sAnims_Condition sSpriteTemplate_Condition sSpritePalette_Condition

/// `struct UsePokeblockSession`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsePokeblockSession {
    pub callback: Option<unsafe extern "C" fn()>,
    pub exitCallback: Option<unsafe extern "C" fn()>,
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
    pub loadNewSelection: Option<unsafe extern "C" fn() -> u8>,
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInfo: *mut UsePokeblockSession = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sExitCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeblock: *mut Pokeblock = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokeblockMonId: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokeblockGain: i16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGraph_Tilemap: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGraph_Gfx: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonFrame_TilemapPtr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenu: *mut UsePokeblockMenu = null_mut();

unsafe extern "C" {
    static gConditionGraphData_Pal: CArray<u16, 0>;
    static gConditionText_Pal: CArray<u16, 0>;
    static mut gKeyRepeatStartDelay: u16;
    static mut gMain: Main;
    static gNatureNamePointers: CArray<*mut u8, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gSpecialVar_ItemId: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gText_GetsAPokeBlockQuestion: CArray<u8, 0>;
    static gText_NatureSlash: CArray<u8, 0>;
    static gText_NothingChanged: CArray<u8, 0>;
    static gText_WasEnhanced: CArray<u8, 0>;
    static gText_WontEatAnymore: CArray<u8, 0>;
    static gUsePokeblockCondition_Gfx: CArray<u32, 0>;
    static gUsePokeblockGraph_Gfx: CArray<u32, 0>;
    static gUsePokeblockGraph_Pal: CArray<u16, 0>;
    static gUsePokeblockGraph_Tilemap: CArray<u32, 0>;
    static gUsePokeblockNatureWin_Pal: CArray<u16, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearWindowTilemap(a0: u8);
    fn ConditionGraph_CalcPositions(a0: *mut u8, a1: *mut UCoords16);
    fn ConditionGraph_Draw(a0: *mut ConditionGraph);
    fn ConditionGraph_Init(a0: *mut ConditionGraph);
    fn ConditionGraph_InitResetScanline(a0: *mut ConditionGraph);
    fn ConditionGraph_InitWindow(a0: u8);
    fn ConditionGraph_ResetScanline(a0: *mut ConditionGraph) -> u8;
    fn ConditionGraph_SetNewPositions(
        a0: *mut ConditionGraph,
        a1: *mut UCoords16,
        a2: *mut UCoords16,
    );
    fn ConditionGraph_TryUpdate(a0: *mut ConditionGraph) -> u8;
    fn ConditionGraph_Update(a0: *mut ConditionGraph);
    fn ConditionMenu_UpdateMonEnter(a0: *mut ConditionGraph, a1: *mut i16) -> u8;
    fn ConditionMenu_UpdateMonExit(a0: *mut ConditionGraph, a1: *mut i16) -> u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateConditionSparkleSprites(a0: *mut *mut Sprite, a1: u8, a2: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroyConditionSparkleSprites(a0: *mut *mut Sprite);
    fn DestroySprite(a0: *mut Sprite);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeConditionSparkles(a0: *mut *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBoxOrPartyMonData(a0: u16, a1: u16, a2: i32, a3: *mut u8) -> i32;
    fn GetConditionMenuMonConditions(
        a0: *mut ConditionGraph,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: u16,
        a6: u16,
        a7: u8,
    );
    fn GetConditionMenuMonGfx(
        a0: *mut c_void,
        a1: *mut c_void,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: u16,
        a6: u8,
    );
    fn GetConditionMenuMonNameAndLocString(
        a0: *mut u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: u16,
        a6: u8,
    );
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonFlavorRelation(a0: *mut Pokemon, a1: u8) -> i8;
    fn GetNature(a0: *mut Pokemon) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadConditionMonPicTemplate(
        a0: *mut SpriteSheet,
        a1: *mut SpriteTemplate,
        a2: *mut SpritePalette,
    );
    fn LoadConditionSelectionIcons(
        a0: *mut SpriteSheet,
        a1: *mut SpriteTemplate,
        a2: *mut SpritePalette,
    );
    fn LoadConditionSparkle(a0: *mut SpriteSheet, a1: *mut SpritePalette);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MoveConditionMonOffscreen(a0: *mut i16) -> u8;
    fn MoveConditionMonOnscreen(a0: *mut i16) -> u8;
    fn PlaySE(a0: u16);
    fn PreparePokeblockFeedScene();
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetConditionSparkleSprites(a0: *mut *mut Sprite);
    fn ResetSpriteData();
    fn RunTextPrinters();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryClearPokeblock(a0: u8) -> u32;
    fn UpdatePaletteFade() -> u8;
    fn rbox_fill_rectangle(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonToGivePokeblock(
    pokeblock: *mut Pokeblock,
    callback: Option<unsafe extern "C" fn()>,
) {
    sMenu = AllocZeroed(32876) as *mut UsePokeblockMenu;
    sInfo = &raw mut (*sMenu).info;
    (*sInfo).pokeblock = pokeblock;
    (*sInfo).exitCallback = callback;
    SetUsePokeblockCallback(Some(LoadUsePokeblockMenu));
    SetMainCallback2(Some(CB2_UsePokeblockMenu));
}
pub(crate) unsafe extern "C" fn CB2_ReturnAndChooseMonToGivePokeblock() {
    sMenu = AllocZeroed(32876) as *mut UsePokeblockMenu;
    sInfo = &raw mut (*sMenu).info;
    (*sInfo).pokeblock = sPokeblock;
    (*sInfo).exitCallback = sExitCallback;
    gPokeblockMonId = GetSelectionIdFromPartyId(gPokeblockMonId);
    (*sInfo).monInTopHalf = (if gPokeblockMonId <= 3 {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8;
    SetUsePokeblockCallback(Some(LoadUsePokeblockMenu));
    SetMainCallback2(Some(CB2_ReturnToUsePokeblockMenu));
}
pub(crate) unsafe extern "C" fn CB2_ReturnToUsePokeblockMenu() {
    (*sInfo).callback.unwrap_unchecked()();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    if (*sInfo).callback == Some(ShowUsePokeblockMenu as unsafe extern "C" fn()) {
        (*sInfo).mainState = 0;
        SetMainCallback2(Some(CB2_ShowUsePokeblockMenuForResults));
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowUsePokeblockMenuForResults() {
    ShowUsePokeblockMenuForResults();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn CB2_UsePokeblockMenu() {
    (*sInfo).callback.unwrap_unchecked()();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_UsePokeblockMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ConditionGraph_Draw(&raw mut (*sMenu).graph);
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn SetUsePokeblockCallback(func: Option<unsafe extern "C" fn()>) {
    (*sInfo).callback = func;
    (*sInfo).mainState = 0;
}
pub(crate) unsafe extern "C" fn LoadUsePokeblockMenu() {
    match (*sInfo).mainState {
        0 => {
            (*sMenu).curMonSpriteId = SPRITE_NONE;
            ConditionGraph_Init(&raw mut (*sMenu).graph);
            (*sInfo).mainState += 1;
        }
        1 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            (*sInfo).mainState += 1;
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
            (*sInfo).mainState += 1;
        }
        3 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            LoadUserWindowBorderGfx(0, 0x97, 224);
            (*sInfo).mainState += 1;
        }
        4 => {
            (*sInfo).mainState += 1;
        }
        5 => {
            if LoadConditionTitle() == 0 {
                (*sInfo).mainState += 1;
            }
        }
        6 => {
            gKeyRepeatStartDelay = 20;
            LoadPartyInfo();
            (*sInfo).mainState += 1;
        }
        7 => {
            if LoadUsePokeblockMenuGfx() == 0 {
                (*sInfo).mainState += 1;
            }
        }
        8 => {
            UpdateMonPic(0);
            LoadAndCreateSelectionIcons();
            (*sInfo).mainState += 1;
        }
        9 => {
            if MoveConditionMonOnscreen(&raw mut (*sMenu).curMonXOffset) == 0 {
                (*sInfo).mainState += 1;
            }
        }
        10 => {
            (*sInfo).mainState += 1;
        }
        11 => {
            ConditionGraph_CalcPositions(
                (*sMenu).graph.conditions[0].as_mut_ptr(),
                (*sMenu).graph.savedPositions[0].as_mut_ptr(),
            );
            ConditionGraph_InitResetScanline(&raw mut (*sMenu).graph);
            (*sInfo).mainState += 1;
        }
        12 => {
            if ConditionGraph_ResetScanline(&raw mut (*sMenu).graph) == 0 {
                ConditionGraph_SetNewPositions(
                    &raw mut (*sMenu).graph,
                    (*sMenu).graph.savedPositions[0].as_mut_ptr(),
                    (*sMenu).graph.savedPositions[0].as_mut_ptr(),
                );
                (*sInfo).mainState += 1;
            }
        }
        13 => {
            ConditionGraph_Update(&raw mut (*sMenu).graph);
            (*sInfo).mainState += 1;
        }
        14 => {
            PutWindowTilemap(WIN_NAME);
            PutWindowTilemap(WIN_NATURE);
            UpdateMonInfoText(0, TRUE);
            (*sInfo).mainState += 1;
        }
        15 => {
            SetUsePokeblockCallback(Some(ShowUsePokeblockMenu));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ShowUsePokeblockMenu() {
    match (*sInfo).mainState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetVBlankCallback(Some(VBlankCB_UsePokeblockMenu));
            ShowBg(0);
            ShowBg(1);
            ShowBg(3);
            ShowBg(2);
            (*sInfo).mainState += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                ResetConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
                if (*sMenu).info.curSelection as i32 != (*sMenu).info.numSelections as i32 - 1 {
                    let mut numSparkles: u8 = (*sMenu).numSparkles[(*sMenu).curLoadId];
                    CreateConditionSparkleSprites(
                        (*sMenu).sparkles.as_mut_ptr(),
                        (*sMenu).curMonSpriteId,
                        numSparkles,
                    );
                }
                SetUsePokeblockCallback(Some(UsePokeblockMenu));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UsePokeblockMenu() {
    let mut loading: u8 = 0;
    match (*sInfo).mainState {
        STATE_HANDLE_INPUT => {
            if gMain.heldKeys as i32 & DPAD_UP != 0 {
                PlaySE(SE_SELECT);
                UpdateSelection(TRUE);
                DestroyConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
                (*sInfo).mainState = STATE_UPDATE_SELECTION;
            } else if gMain.heldKeys as i32 & DPAD_DOWN != 0 {
                PlaySE(SE_SELECT);
                UpdateSelection(FALSE);
                DestroyConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
                (*sInfo).mainState = STATE_UPDATE_SELECTION;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sInfo).mainState = STATE_CLOSE;
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*sMenu).info.curSelection as i32 == (*sMenu).info.numSelections as i32 - 1 {
                    (*sInfo).mainState = STATE_CLOSE;
                } else {
                    (*sInfo).mainState = STATE_CONFIRM_SELECTION;
                }
            }
        }
        STATE_UPDATE_SELECTION => {
            loading = (*sMenu).info.loadNewSelection.unwrap_unchecked()();
            if loading == 0 {
                (*sInfo).mainState = STATE_HANDLE_INPUT;
            }
        }
        STATE_2 => {}
        STATE_CLOSE => {
            SetUsePokeblockCallback(Some(CloseUsePokeblockMenu));
        }
        STATE_4 => {}
        STATE_CONFIRM_SELECTION => {
            AskUsePokeblock();
            (*sInfo).mainState += 1;
        }
        STATE_HANDLE_CONFIRMATION => match HandleAskUsePokeblockInput() {
            1 | MENU_B_PRESSED => {
                (*sInfo).mainState = STATE_HANDLE_INPUT;
            }
            0 => {
                if IsSheenMaxed() != 0 {
                    PrintWontEatAnymore();
                    (*sInfo).mainState = STATE_WAIT_MSG;
                } else {
                    SetUsePokeblockCallback(Some(FeedPokeblockToMon));
                }
            }
            _ => {}
        },
        STATE_WAIT_MSG => {
            if gMain.newKeys as i32 & 3 != 0 {
                EraseMenuWindow();
                (*sInfo).mainState = STATE_HANDLE_INPUT;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn FeedPokeblockToMon() {
    match (*sInfo).mainState {
        0 => {
            gPokeblockMonId = GetPartyIdFromSelectionId((*sMenu).info.curSelection);
            sExitCallback = (*sInfo).exitCallback;
            sPokeblock = (*sInfo).pokeblock;
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sInfo).mainState += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                SetVBlankCallback(None);
                Free(sGraph_Tilemap as *mut c_void);
                sGraph_Tilemap = null_mut();
                Free(sGraph_Gfx as *mut c_void);
                sGraph_Gfx = null_mut();
                Free(sMonFrame_TilemapPtr as *mut c_void);
                sMonFrame_TilemapPtr = null_mut();
                Free(sMenu as *mut c_void);
                sMenu = null_mut();
                FreeAllWindowBuffers();
                gMain.savedCallback = Some(CB2_ReturnAndChooseMonToGivePokeblock);
                PreparePokeblockFeedScene();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ShowUsePokeblockMenuForResults() {
    let mut loading: u8 = 0;
    match (*sInfo).mainState {
        0 => {
            if (*sMenu).info.curSelection != gPokeblockMonId {
                UpdateSelection((*sInfo).monInTopHalf);
                (*sInfo).mainState += 1;
            } else {
                (*sInfo).mainState = 3;
            }
        }
        1 => {
            loading = (*sMenu).info.loadNewSelection.unwrap_unchecked()();
            if loading == 0 {
                (*sInfo).mainState = 0;
            }
        }
        2 => {}
        3 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            (*sInfo).mainState += 1;
        }
        4 => {
            ShowBg(0);
            ShowBg(1);
            ShowBg(3);
            ShowBg(2);
            (*sInfo).mainState += 1;
        }
        5 => {
            SetVBlankCallback(Some(VBlankCB_UsePokeblockMenu));
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*sInfo).mainState += 1;
        }
        6 => {
            if gPaletteFade.active() == 0 {
                ResetConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
                SetUsePokeblockCallback(Some(ShowPokeblockResults));
                SetMainCallback2(Some(CB2_UsePokeblockMenu));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ShowPokeblockResults() {
    match (*sInfo).mainState {
        0 => {
            (*sInfo).mon = gPlayerParty.as_mut_ptr();
            (*sInfo).mon = (*sInfo)
                .mon
                .at((*sMenu).party[(*sMenu).info.curSelection].monId);
            DestroyConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
            (*sInfo).mainState += 1;
        }
        1 => {
            if gMain.newKeys as i32 & 3 != 0 {
                (*sInfo).mainState += 1;
            }
        }
        2 => {
            CalculateConditionEnhancements();
            ConditionGraph_CalcPositions(
                (*sInfo).conditionsAfterBlock.as_mut_ptr(),
                (*sMenu).graph.savedPositions[3].as_mut_ptr(),
            );
            ConditionGraph_SetNewPositions(
                &raw mut (*sMenu).graph,
                (*sMenu).graph.savedPositions[(*sMenu).curLoadId].as_mut_ptr(),
                (*sMenu).graph.savedPositions[3].as_mut_ptr(),
            );
            LoadAndCreateUpDownSprites();
            (*sInfo).mainState += 1;
        }
        3 => {
            if ConditionGraph_TryUpdate(&raw mut (*sMenu).graph) == 0 {
                CalculateNumAdditionalSparkles(GetPartyIdFromSelectionId(
                    (*sMenu).info.curSelection,
                ));
                if (*sMenu).info.curSelection as i32 != (*sMenu).info.numSelections as i32 - 1 {
                    let mut numSparkles: u8 = (*sMenu).numSparkles[(*sMenu).curLoadId];
                    CreateConditionSparkleSprites(
                        (*sMenu).sparkles.as_mut_ptr(),
                        (*sMenu).curMonSpriteId,
                        numSparkles,
                    );
                }
                (*sInfo).timer = 0;
                (*sInfo).mainState += 1;
            }
        }
        4 => {
            if ({
                (*sInfo).timer += 1;
                (*sInfo).timer
            }) > 16
            {
                PrintFirstEnhancement();
                (*sInfo).mainState += 1;
            }
        }
        5 => {
            if gMain.newKeys as i32 & 3 != 0 && TryPrintNextEnhancement() == 0 {
                TryClearPokeblock(gSpecialVar_ItemId as u8);
                SetUsePokeblockCallback(Some(CloseUsePokeblockMenu));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CloseUsePokeblockMenu() {
    let mut i: u8 = 0;
    match (*sInfo).mainState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sInfo).mainState += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                (*sInfo).mainState = 2;
            }
        }
        2 => {
            gScanlineEffect.state = 3;
            ScanlineEffect_InitHBlankDmaTransfer();
            (*sInfo).mainState += 1;
        }
        3 => {
            SetMainCallback2((*sInfo).exitCallback);
            FreeConditionSparkles((*sMenu).sparkles.as_mut_ptr());
            i = 0;
            while i < 7 {
                DestroySprite(&raw mut gSprites[(*sMenu).selectionIconSpriteIds[i]]);
                i += 1;
            }
            FreeSpriteTilesByTag(TAG_UP_DOWN);
            FreeSpriteTilesByTag(TAG_CONDITION);
            FreeSpritePaletteByTag(TAG_UP_DOWN);
            FreeSpritePaletteByTag(TAG_CONDITION);
            i = 0;
            while i < 2 {
                DestroySprite((*sMenu).condition[i]);
                i += 1;
            }
            if (*sMenu).curMonSpriteId != SPRITE_NONE {
                DestroySprite(&raw mut gSprites[(*sMenu).curMonSpriteId]);
            }
            SetVBlankCallback(None);
            Free(sGraph_Tilemap as *mut c_void);
            sGraph_Tilemap = null_mut();
            Free(sGraph_Gfx as *mut c_void);
            sGraph_Gfx = null_mut();
            Free(sMonFrame_TilemapPtr as *mut c_void);
            sMonFrame_TilemapPtr = null_mut();
            Free(sMenu as *mut c_void);
            sMenu = null_mut();
            FreeAllWindowBuffers();
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AskUsePokeblock() {
    let mut stringBuffer: CArray<u8, 64> = zeroed();
    GetMonData3(
        &raw mut gPlayerParty[GetPartyIdFromSelectionId((*sMenu).info.curSelection)],
        MON_DATA_NICKNAME,
        stringBuffer.as_mut_ptr(),
    );
    StringGet_Nickname(stringBuffer.as_mut_ptr());
    StringAppend(
        stringBuffer.as_mut_ptr(),
        gText_GetsAPokeBlockQuestion.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn HandleAskUsePokeblockInput() -> i8 {
    let mut menuItem: i8 = Menu_ProcessInputNoWrapClearOnChoose();
    match menuItem {
        0 => {}
        MENU_B_PRESSED | 1 => {
            PlaySE(SE_SELECT);
            rbox_fill_rectangle(2);
            ClearWindowTilemap(2);
        }
        _ => {}
    }
    return menuItem;
}
pub(crate) unsafe extern "C" fn PrintFirstEnhancement() {
    DrawTextBorderOuter(WIN_TEXT, 151, 14);
    FillWindowPixelBuffer(WIN_TEXT, 17);
    (*sInfo).condition = 0;
    while (*sInfo).condition < CONDITION_COUNT as u8 {
        if (*sInfo).enhancements[(*sInfo).condition] != 0 {
            break;
        }
        (*sInfo).condition += 1;
    }
    if (*sInfo).condition < CONDITION_COUNT as u8 {
        BufferEnhancedText(
            gStringVar4.as_mut_ptr(),
            (*sInfo).condition,
            (*sInfo).enhancements[(*sInfo).condition] as i16,
        );
    } else {
        BufferEnhancedText(gStringVar4.as_mut_ptr(), (*sInfo).condition, 0);
    }
    PrintMenuWindowText(gStringVar4.as_mut_ptr());
    PutWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn TryPrintNextEnhancement() -> u8 {
    FillWindowPixelBuffer(WIN_TEXT, 17);
    loop {
        (*sInfo).condition += 1;
        if (*sInfo).condition < CONDITION_COUNT as u8 {
            if (*sInfo).enhancements[(*sInfo).condition] != 0 {
                break;
            }
        } else {
            (*sInfo).condition = CONDITION_COUNT as u8;
            return FALSE;
        }
    }
    BufferEnhancedText(
        gStringVar4.as_mut_ptr(),
        (*sInfo).condition,
        (*sInfo).enhancements[(*sInfo).condition] as i16,
    );
    PrintMenuWindowText(gStringVar4.as_mut_ptr());
    CopyWindowToVram(WIN_TEXT, COPYWIN_GFX);
    return TRUE;
}
pub(crate) unsafe extern "C" fn PrintWontEatAnymore() {
    FillWindowPixelBuffer(WIN_TEXT, 17);
    DrawTextBorderOuter(WIN_TEXT, 151, 14);
    AddTextPrinterParameterized(
        WIN_TEXT,
        FONT_NORMAL,
        gText_WontEatAnymore.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn EraseMenuWindow() {
    rbox_fill_rectangle(WIN_TEXT);
    ClearWindowTilemap(WIN_TEXT);
    CopyWindowToVram(WIN_TEXT, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn PrintMenuWindowText(message: *mut u8) {
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
pub(crate) unsafe extern "C" fn BufferEnhancedText(
    mut dest: *mut u8,
    condition: u8,
    mut enhancement: i16,
) {
    'l1: {
        let sw1: i16 = enhancement;
        let mut fall = false;
        if (1..=32767).contains(&sw1) {
            fall = true;
            enhancement = 0;
        }
        if fall || (-32768..=-1).contains(&sw1) {
            fall = true;
            if enhancement != 0 {
                *dest.at(enhancement as u16) += 0;
            }
            StringCopy(dest, sConditionNames[condition]);
            StringAppend(dest, gText_WasEnhanced.as_ptr().cast_mut());
            break 'l1;
        }
        if sw1 == 0 {
            fall = true;
            StringCopy(dest, gText_NothingChanged.as_ptr().cast_mut());
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn GetMonConditions(mon: *mut Pokemon, mut data: *mut u8) {
    let mut i: u16 = 0;
    i = 0;
    while i < CONDITION_COUNT {
        *data.at(i) = GetMonData2(mon, sConditionToMonData[i] as i32) as u8;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AddPokeblockToConditions(
    pokeblock: *mut Pokeblock,
    mon: *mut Pokemon,
) {
    let mut i: u16 = 0;
    let mut stat: i16 = 0;
    let mut data: u8 = 0;
    if GetMonData2(mon, MON_DATA_SHEEN) != MAX_SHEEN as u32 {
        CalculatePokeblockEffectiveness(pokeblock, mon);
        i = 0;
        while i < CONDITION_COUNT {
            data = GetMonData2(mon, sConditionToMonData[i] as i32) as u8;
            stat = data as i16 + (*sInfo).pokeblockStatBoosts[i];
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
            i += 1;
        }
        stat = GetMonData2(mon, MON_DATA_SHEEN) as u8 as i16 + (*pokeblock).feel as i16;
        if stat > MAX_SHEEN {
            stat = MAX_SHEEN;
        }
        data = stat as u8;
        SetMonData(mon, MON_DATA_SHEEN, &raw mut data as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn CalculateConditionEnhancements() {
    let mut i: u16 = 0;
    let mut mon: *mut Pokemon = gPlayerParty.as_mut_ptr();
    mon = mon.at((*sMenu).party[(*sMenu).info.curSelection].monId);
    GetMonConditions(mon, (*sInfo).conditionsBeforeBlock.as_mut_ptr());
    AddPokeblockToConditions((*sInfo).pokeblock, mon);
    GetMonConditions(mon, (*sInfo).conditionsAfterBlock.as_mut_ptr());
    i = 0;
    while i < CONDITION_COUNT {
        (*sInfo).enhancements[i] =
            (*sInfo).conditionsAfterBlock[i] - (*sInfo).conditionsBeforeBlock[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CalculatePokeblockEffectiveness(
    pokeblock: *mut Pokeblock,
    mon: *mut Pokemon,
) {
    let mut i: i8 = 0;
    let mut direction: i8 = 0;
    let mut flavor: i8 = 0;
    (*sInfo).pokeblockStatBoosts[0] = (*pokeblock).spicy as i16;
    (*sInfo).pokeblockStatBoosts[1] = (*pokeblock).sour as i16;
    (*sInfo).pokeblockStatBoosts[2] = (*pokeblock).bitter as i16;
    (*sInfo).pokeblockStatBoosts[3] = (*pokeblock).sweet as i16;
    (*sInfo).pokeblockStatBoosts[4] = (*pokeblock).dry as i16;
    if gPokeblockGain > 0 {
        direction = 1;
    } else if gPokeblockGain < 0 {
        direction = -1;
    } else {
        return;
    }
    i = 0;
    while i < CONDITION_COUNT as i8 {
        let mut amount: i16 = (*sInfo).pokeblockStatBoosts[i];
        let mut boost: i8 = (amount / 10) as i8;
        if amount % 10 >= 5 {
            boost += 1;
        }
        flavor = GetMonFlavorRelation(mon, sConditionToFlavor[i]);
        if flavor == direction {
            (*sInfo).pokeblockStatBoosts[i] += boost as i16 * flavor as i16;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IsSheenMaxed() -> u8 {
    if GetBoxOrPartyMonData(
        (*sMenu).party[(*sMenu).info.curSelection].boxId as u16,
        (*sMenu).party[(*sMenu).info.curSelection].monId as u16,
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetPartyIdFromSelectionId(mut selectionId: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < PARTY_SIZE as u8 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0 {
            if selectionId == 0 {
                return i;
            }
            selectionId -= 1;
        }
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn GetSelectionIdFromPartyId(partyId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut numEggs: u8 = 0;
    i = 0;
    numEggs = 0;
    while i < partyId {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) != 0 {
            numEggs += 1;
        }
        i += 1;
    }
    return partyId - numEggs;
}
pub(crate) unsafe extern "C" fn GetPartyIdFromSelectionId_(selectionId: u8) -> u8 {
    return GetPartyIdFromSelectionId(selectionId);
}
pub(crate) unsafe extern "C" fn LoadAndCreateUpDownSprites() {
    let mut i: u16 = 0;
    LoadSpriteSheet((&raw const *sSpriteSheet_UpDown).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_UpDown).cast_mut());
    (*sInfo).numEnhancements = 0;
    i = 0;
    while i < CONDITION_COUNT {
        if (*sInfo).enhancements[i] != 0 {
            let mut spriteId: u16 = CreateSprite(
                (&raw const *sSpriteTemplate_UpDown).cast_mut(),
                sUpDownCoordsOnGraph[i][0],
                sUpDownCoordsOnGraph[i][1],
                0,
            ) as u16;
            if spriteId != MAX_SPRITES as u16 {
                if (*sInfo).enhancements[i] != 0 {
                    gSprites[spriteId].callback = Some(SpriteCB_UpDown);
                }
                (*sInfo).numEnhancements += 1;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpDown(sprite: *mut Sprite) {
    if (*sprite).data[0] < 6 {
        (*sprite).y2 -= 2;
    } else if (*sprite).data[0] < 12 {
        (*sprite).y2 += 2;
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 60
    {
        DestroySprite(sprite);
        (*sInfo).numEnhancements -= 1;
    }
}
pub(crate) unsafe extern "C" fn LoadPartyInfo() {
    let mut i: u16 = 0;
    let mut numMons: u16 = 0;
    i = 0;
    numMons = 0;
    while i < CalculatePlayerPartyCount() as u16 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0 {
            (*sMenu).party[numMons].boxId = TOTAL_BOXES_COUNT;
            (*sMenu).party[numMons].monId = i as u8;
            (*sMenu).party[numMons].data = 0;
            numMons += 1;
        }
        i += 1;
    }
    (*sMenu).info.curSelection = 0;
    (*sMenu).info.numSelections = numMons as u8 + 1;
    LoadInitialMonInfo();
}
pub(crate) unsafe extern "C" fn LoadInitialMonInfo() {
    let mut nextSelection: i16 = 0;
    let mut prevSelection: i16 = 0;
    LoadMonInfo((*sMenu).info.curSelection as i16, 0);
    (*sMenu).curLoadId = 0;
    (*sMenu).nextLoadId = 1;
    (*sMenu).prevLoadId = 2;
    nextSelection = (*sMenu).info.curSelection as i16 + 1;
    if nextSelection >= (*sMenu).info.numSelections as i16 {
        nextSelection = 0;
    }
    prevSelection = (*sMenu).info.curSelection as i16 - 1;
    if prevSelection < 0 {
        prevSelection = (*sMenu).info.numSelections as i16 - 1;
    }
    LoadMonInfo(nextSelection, 1);
    LoadMonInfo(prevSelection, 2);
}
pub(crate) unsafe extern "C" fn LoadMonInfo(partyId: i16, loadId: u8) {
    let mut boxId: u8 = (*sMenu).party[partyId].boxId;
    let mut monId: u8 = (*sMenu).party[partyId].monId;
    let mut numSelections: u8 = (*sMenu).info.numSelections;
    let mut excludesCancel: u8 = FALSE;
    GetConditionMenuMonNameAndLocString(
        (*sMenu).locationStrings[loadId].as_mut_ptr(),
        (*sMenu).monNameStrings[loadId].as_mut_ptr(),
        boxId as u16,
        monId as u16,
        partyId as u16,
        numSelections as u16,
        excludesCancel,
    );
    GetConditionMenuMonConditions(
        &raw mut (*sMenu).graph,
        (*sMenu).numSparkles.as_mut_ptr(),
        boxId as u16,
        monId as u16,
        partyId as u16,
        loadId as u16,
        numSelections as u16,
        excludesCancel,
    );
    GetConditionMenuMonGfx(
        (*sMenu).partySheets[loadId].as_mut_ptr() as *mut c_void,
        (*sMenu).partyPalettes[loadId].as_mut_ptr() as *mut c_void,
        boxId as u16,
        monId as u16,
        partyId as u16,
        numSelections as u16,
        excludesCancel,
    );
}
pub(crate) unsafe extern "C" fn UpdateMonPic(loadId: u8) {
    let mut spriteId: u8 = 0;
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteSheet: SpriteSheet = zeroed();
    let mut spritePal: SpritePalette = zeroed();
    if (*sMenu).curMonSpriteId == SPRITE_NONE {
        LoadConditionMonPicTemplate(
            &raw mut spriteSheet,
            &raw mut spriteTemplate,
            &raw mut spritePal,
        );
        spriteSheet.data = (*sMenu).partySheets[loadId].as_mut_ptr() as *mut c_void;
        spritePal.data = (*sMenu).partyPalettes[loadId].as_mut_ptr();
        (*sMenu).curMonPalette = LoadSpritePalette(&raw mut spritePal) as u16;
        (*sMenu).curMonSheet = LoadSpriteSheet(&raw mut spriteSheet);
        spriteId = CreateSprite(&raw mut spriteTemplate, 38, 104, 0);
        (*sMenu).curMonSpriteId = spriteId;
        if spriteId == MAX_SPRITES {
            FreeSpriteTilesByTag(TAG_CONDITION_MON);
            FreeSpritePaletteByTag(TAG_CONDITION_MON);
            (*sMenu).curMonSpriteId = SPRITE_NONE;
        } else {
            (*sMenu).curMonSpriteId = spriteId;
            gSprites[(*sMenu).curMonSpriteId].callback = Some(SpriteCB_MonPic);
            gSprites[(*sMenu).curMonSpriteId].y2 -= 34;
            (*sMenu).curMonTileStart =
                (OBJ_VRAM0 + (*sMenu).curMonSheet as i32 * 32) as usize as *mut c_void as *mut u8;
            (*sMenu).curMonPalette = 0x100 + (*sMenu).curMonPalette * 16;
        }
    } else {
        {
            let mut _src: *mut c_void = (*sMenu).partySheets[loadId].as_mut_ptr() as *mut c_void;
            let mut _dest: *mut c_void = (*sMenu).curMonTileStart as *mut c_void;
            let mut _size: u32 = MON_PIC_SIZE as u32;
            loop {
                if _size <= 0x1000 {
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    break;
                }
                {
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            (*sMenu).partyPalettes[loadId].as_mut_ptr() as *mut c_void,
            (*sMenu).curMonPalette,
            32,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadAndCreateSelectionIcons() {
    let mut i: u16 = 0;
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
    i = 0;
    while (i as i32) < (*sMenu).info.numSelections as i32 - 1 {
        spriteId = CreateSprite(&raw mut spriteTemplate, 226, i as i16 * 20 + 8, 0) as u16;
        if spriteId != MAX_SPRITES as u16 {
            (*sMenu).selectionIconSpriteIds[i] = spriteId as u8;
            gSprites[spriteId].data[0] = i as i16;
            gSprites[spriteId].callback = Some(SpriteCB_SelectionIconPokeball);
        } else {
            (*sMenu).selectionIconSpriteIds[i] = 255;
        }
        i += 1;
    }
    spriteTemplate.tileTag = TAG_CONDITION_BALL_PLACEHOLDER;
    while i < PARTY_SIZE as u16 {
        spriteId = CreateSprite(&raw mut spriteTemplate, 230, i as i16 * 20 + 8, 0) as u16;
        if spriteId != MAX_SPRITES as u16 {
            (*sMenu).selectionIconSpriteIds[i] = spriteId as u8;
            gSprites[spriteId].oam.set_size(0);
        } else {
            (*sMenu).selectionIconSpriteIds[i] = 255;
        }
        i += 1;
    }
    spriteTemplate.tileTag = TAG_CONDITION_CANCEL;
    spriteTemplate.callback = Some(SpriteCB_SelectionIconCancel);
    spriteId = CreateSprite(&raw mut spriteTemplate, 222, i as i16 * 20 + 8, 0) as u16;
    if spriteId != MAX_SPRITES as u16 {
        (*sMenu).selectionIconSpriteIds[i] = spriteId as u8;
        gSprites[spriteId].oam.set_shape(1);
        gSprites[spriteId].oam.set_size(2);
    } else {
        (*sMenu).selectionIconSpriteIds[i] = 255;
    }
    LoadConditionSparkle(&raw mut spriteSheet2, &raw mut spritePal2);
    LoadSpriteSheet(&raw mut spriteSheet2);
    LoadSpritePalette(&raw mut spritePal2);
}
pub(crate) unsafe extern "C" fn LoadUsePokeblockMenuGfx() -> u8 {
    match (*sMenu).info.helperState {
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
            sGraph_Gfx = Alloc(6656) as *mut u8;
            sGraph_Tilemap = Alloc(1280) as *mut u8;
            sMonFrame_TilemapPtr = Alloc(1280) as *mut u8;
        }
        2 => {
            LZ77UnCompVram(
                sMonFrame_Tilemap.as_ptr().cast_mut(),
                sMonFrame_TilemapPtr as *mut c_void,
            );
        }
        3 => {
            LoadBgTiles(3, sMonFrame_Gfx.as_ptr().cast_mut() as *mut c_void, 224, 0);
        }
        4 => {
            LoadBgTilemap(3, sMonFrame_TilemapPtr as *mut c_void, 1280, 0);
        }
        5 => {
            LoadPalette(sMonFrame_Pal.as_ptr().cast_mut() as *mut c_void, 208, 32);
            (*sMenu).curMonXOffset = -80;
        }
        6 => {
            LZ77UnCompVram(
                gUsePokeblockGraph_Gfx.as_ptr().cast_mut(),
                sGraph_Gfx as *mut c_void,
            );
        }
        7 => {
            LZ77UnCompVram(
                gUsePokeblockGraph_Tilemap.as_ptr().cast_mut(),
                sGraph_Tilemap as *mut c_void,
            );
            LoadPalette(
                gUsePokeblockGraph_Pal.as_ptr().cast_mut() as *mut c_void,
                32,
                32,
            );
        }
        8 => {
            LoadBgTiles(1, sGraph_Gfx as *mut c_void, 6656, 640);
        }
        9 => {
            SetBgTilemapBuffer(1, sGraph_Tilemap as *mut c_void);
            CopyToBgTilemapBufferRect(
                1,
                gUsePokeblockNatureWin_Pal.as_ptr().cast_mut() as *mut c_void,
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
                (*sMenu).tilemapBuffer.as_mut_ptr() as *mut c_void,
            );
        }
        11 => {
            LoadBgTilemap(
                2,
                (*sMenu).tilemapBuffer.as_mut_ptr() as *mut c_void,
                1280,
                0,
            );
            LoadPalette(
                gConditionGraphData_Pal.as_ptr().cast_mut() as *mut c_void,
                48,
                32,
            );
            LoadPalette(
                gConditionText_Pal.as_ptr().cast_mut() as *mut c_void,
                240,
                32,
            );
            ConditionGraph_InitWindow(2);
        }
        _ => {
            (*sMenu).info.helperState = 0;
            return FALSE;
        }
    }
    (*sMenu).info.helperState += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateMonInfoText(loadId: u16, firstPrint: u8) {
    let mut partyIndex: u8 = 0;
    let mut nature: u8 = 0;
    let mut str: *mut u8 = null_mut();
    FillWindowPixelBuffer(WIN_NAME, 0);
    FillWindowPixelBuffer(WIN_NATURE, 0);
    if (*sMenu).info.curSelection as i32 != (*sMenu).info.numSelections as i32 - 1 {
        AddTextPrinterParameterized(
            WIN_NAME,
            FONT_NORMAL,
            (*sMenu).monNameStrings[loadId].as_mut_ptr(),
            0,
            1,
            0,
            None,
        );
        partyIndex = GetPartyIdFromSelectionId((*sMenu).info.curSelection);
        nature = GetNature(&raw mut gPlayerParty[partyIndex]);
        str = StringCopy(
            (*sMenu).info.natureText.as_mut_ptr(),
            gText_NatureSlash.as_ptr().cast_mut(),
        );
        str = StringCopy(str, gNatureNamePointers[nature]);
        AddTextPrinterParameterized3(
            WIN_NATURE,
            FONT_NORMAL,
            2,
            1,
            sNatureTextColors.as_ptr().cast_mut(),
            0,
            (*sMenu).info.natureText.as_mut_ptr(),
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
pub(crate) unsafe extern "C" fn UpdateSelection(up: u8) {
    let mut newLoadId: u16 = 0;
    let mut startedOnMon: u32 = 0;
    let mut endedOnMon: u32 = 0;
    if up != 0 {
        newLoadId = (*sMenu).prevLoadId as u16;
    } else {
        newLoadId = (*sMenu).nextLoadId as u16;
    }
    ConditionGraph_SetNewPositions(
        &raw mut (*sMenu).graph,
        (*sMenu).graph.savedPositions[(*sMenu).curLoadId].as_mut_ptr(),
        (*sMenu).graph.savedPositions[newLoadId].as_mut_ptr(),
    );
    if (*sMenu).info.curSelection as i32 == (*sMenu).info.numSelections as i32 - 1 {
        startedOnMon = FALSE as u32;
    } else {
        startedOnMon = TRUE as u32;
    }
    if up != 0 {
        (*sMenu).prevLoadId = (*sMenu).nextLoadId;
        (*sMenu).nextLoadId = (*sMenu).curLoadId;
        (*sMenu).curLoadId = newLoadId as i8;
        (*sMenu).toLoadId = (*sMenu).prevLoadId;
        (*sMenu).info.curSelection = (if (*sMenu).info.curSelection == 0 {
            (*sMenu).info.numSelections as i32 - 1
        } else {
            (*sMenu).info.curSelection as i32 - 1
        }) as u8;
        (*sMenu).toLoadSelection = (if (*sMenu).info.curSelection == 0 {
            (*sMenu).info.numSelections as i32 - 1
        } else {
            (*sMenu).info.curSelection as i32 - 1
        }) as u8;
    } else {
        (*sMenu).nextLoadId = (*sMenu).prevLoadId;
        (*sMenu).prevLoadId = (*sMenu).curLoadId;
        (*sMenu).curLoadId = newLoadId as i8;
        (*sMenu).toLoadId = (*sMenu).nextLoadId;
        (*sMenu).info.curSelection =
            (if ((*sMenu).info.curSelection as i32) < (*sMenu).info.numSelections as i32 - 1 {
                (*sMenu).info.curSelection as i32 + 1
            } else {
                0
            }) as u8;
        (*sMenu).toLoadSelection =
            (if ((*sMenu).info.curSelection as i32) < (*sMenu).info.numSelections as i32 - 1 {
                (*sMenu).info.curSelection as i32 + 1
            } else {
                0
            }) as u8;
    }
    if (*sMenu).info.curSelection as i32 == (*sMenu).info.numSelections as i32 - 1 {
        endedOnMon = FALSE as u32;
    } else {
        endedOnMon = TRUE as u32;
    }
    DestroyConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
    if startedOnMon == 0 {
        (*sMenu).info.loadNewSelection = Some(LoadNewSelection_CancelToMon);
    } else if endedOnMon == 0 {
        (*sMenu).info.loadNewSelection = Some(LoadNewSelection_MonToCancel);
    } else {
        (*sMenu).info.loadNewSelection = Some(LoadNewSelection_MonToMon);
    }
}
pub(crate) unsafe extern "C" fn LoadNewSelection_CancelToMon() -> u8 {
    match (*sMenu).info.helperState {
        0 => {
            UpdateMonPic((*sMenu).curLoadId as u8);
            (*sMenu).info.helperState += 1;
        }
        1 => {
            UpdateMonInfoText((*sMenu).curLoadId as u16, FALSE);
            (*sMenu).info.helperState += 1;
        }
        2 => {
            if ConditionMenu_UpdateMonEnter(
                &raw mut (*sMenu).graph,
                &raw mut (*sMenu).curMonXOffset,
            ) == 0
            {
                LoadMonInfo((*sMenu).toLoadSelection as i16, (*sMenu).toLoadId as u8);
                (*sMenu).info.helperState += 1;
            }
        }
        3 => {
            ResetConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
            if (*sMenu).info.curSelection as i32 != (*sMenu).info.numSelections as i32 - 1 {
                let mut numSparkles: u8 = (*sMenu).numSparkles[(*sMenu).curLoadId];
                CreateConditionSparkleSprites(
                    (*sMenu).sparkles.as_mut_ptr(),
                    (*sMenu).curMonSpriteId,
                    numSparkles,
                );
            }
            (*sMenu).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn LoadNewSelection_MonToCancel() -> u8 {
    match (*sMenu).info.helperState {
        0 => {
            if ConditionMenu_UpdateMonExit(&raw mut (*sMenu).graph, &raw mut (*sMenu).curMonXOffset)
                == 0
            {
                (*sMenu).info.helperState += 1;
            }
        }
        1 => {
            UpdateMonInfoText((*sMenu).curLoadId as u16, FALSE);
            (*sMenu).info.helperState += 1;
        }
        2 => {
            LoadMonInfo((*sMenu).toLoadSelection as i16, (*sMenu).toLoadId as u8);
            (*sMenu).info.helperState += 1;
        }
        3 => {
            (*sMenu).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn LoadNewSelection_MonToMon() -> u8 {
    match (*sMenu).info.helperState {
        0 => {
            ConditionGraph_TryUpdate(&raw mut (*sMenu).graph);
            if MoveConditionMonOffscreen(&raw mut (*sMenu).curMonXOffset) == 0 {
                UpdateMonPic((*sMenu).curLoadId as u8);
                (*sMenu).info.helperState += 1;
            }
        }
        1 => {
            UpdateMonInfoText((*sMenu).curLoadId as u16, FALSE);
            (*sMenu).info.helperState += 1;
        }
        2 => {
            if ConditionMenu_UpdateMonEnter(
                &raw mut (*sMenu).graph,
                &raw mut (*sMenu).curMonXOffset,
            ) == 0
            {
                LoadMonInfo((*sMenu).toLoadSelection as i16, (*sMenu).toLoadId as u8);
                (*sMenu).info.helperState += 1;
            }
        }
        3 => {
            ResetConditionSparkleSprites((*sMenu).sparkles.as_mut_ptr());
            if (*sMenu).info.curSelection as i32 != (*sMenu).info.numSelections as i32 - 1 {
                let mut numSparkles: u8 = (*sMenu).numSparkles[(*sMenu).curLoadId];
                CreateConditionSparkleSprites(
                    (*sMenu).sparkles.as_mut_ptr(),
                    (*sMenu).curMonSpriteId,
                    numSparkles,
                );
            }
            (*sMenu).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SpriteCB_MonPic(sprite: *mut Sprite) {
    (*sprite).x = (*sMenu).curMonXOffset + 38;
}
pub(crate) unsafe extern "C" fn SpriteCB_SelectionIconPokeball(sprite: *mut Sprite) {
    if (*sprite).data[0] == (*sMenu).info.curSelection as i16 {
        StartSpriteAnim(sprite, CONDITION_ICON_SELECTED);
    } else {
        StartSpriteAnim(sprite, CONDITION_ICON_UNSELECTED);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SelectionIconCancel(sprite: *mut Sprite) {
    if (*sMenu).info.curSelection as i32 == (*sMenu).info.numSelections as i32 - 1 {
        (*sprite)
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(TAG_CONDITION_BALL) as u16);
    } else {
        (*sprite)
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(TAG_CONDITION_CANCEL) as u16);
    }
}
pub(crate) unsafe extern "C" fn CalculateNumAdditionalSparkles(monIndex: u8) {
    let mut sheen: u8 = GetMonData2(&raw mut gPlayerParty[monIndex], MON_DATA_SHEEN) as u8;
    (*sMenu).numSparkles[(*sMenu).curLoadId] =
        (if sheen != 255 { (sheen / 29) as i32 } else { 9 }) as u8;
}
pub(crate) unsafe extern "C" fn LoadConditionGfx() {
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    let mut spritePalette: SpritePalette = zeroed();
    spritePalette = *sSpritePalette_Condition;
    spriteSheet.data = gUsePokeblockCondition_Gfx.as_ptr().cast_mut();
    spriteSheet.size = 0x800;
    spriteSheet.tag = TAG_CONDITION;
    LoadCompressedSpriteSheet(&raw mut spriteSheet);
    LoadSpritePalette(&raw mut spritePalette);
}
pub(crate) unsafe extern "C" fn CreateConditionSprite() {
    let mut i: u16 = 0;
    let mut xDiff: i16 = 0;
    let mut xStart: i16 = 0;
    let mut yStart: i32 = 17;
    let mut speed: i32 = 8;
    let mut sprites: *mut *mut Sprite = (*sMenu).condition.as_mut_ptr();
    let mut template: *mut SpriteTemplate = (&raw const *sSpriteTemplate_Condition).cast_mut();
    i = 0;
    xDiff = 64;
    xStart = -96;
    while i < 2 {
        let mut spriteId: u8 = CreateSprite(template, i as i16 * xDiff + xStart, yStart as i16, 0);
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].data[0] = speed as i16;
            gSprites[spriteId].data[1] = i as i16 * xDiff | 0x20;
            gSprites[spriteId].data[2] = i as i16;
            StartSpriteAnim(&raw mut gSprites[spriteId], i as u8);
            *sprites.at(i) = &raw mut gSprites[spriteId];
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadConditionTitle() -> u8 {
    match (*sMenu).info.helperState {
        0 => {
            LoadConditionGfx();
            (*sMenu).info.helperState += 1;
            return TRUE;
        }
        1 => {
            CreateConditionSprite();
            (*sMenu).info.helperState = 0;
            return FALSE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SpriteCB_Condition(sprite: *mut Sprite) {
    let mut prevX: i16 = (*sprite).x;
    (*sprite).x += (*sprite).data[0];
    if prevX <= (*sprite).data[1] && (*sprite).x >= (*sprite).data[1]
        || prevX >= (*sprite).data[1] && (*sprite).x <= (*sprite).data[1]
    {
        (*sprite).x = (*sprite).data[1];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
