//! Translated from `src/naming_screen.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sPCIconOff_Gfx sPCIconOn_Gfx sKeyboard_Pal sRival_Pal sTransferredToPCMessages sText_AlphabetUpperLower sBgTemplates sWindowTemplates sKeyboardChars sPageColumnCounts sPageColumnXPos sNamingScreenTemplates sSubspriteTable_PageSwapFrame sSubspriteTable_PageSwapText sSubspriteTable_Button sSubspriteTable_PCIcon sSpriteTemplate_PageSwapFrame sSpriteTemplate_PageSwapButton sSpriteTemplate_PageSwapText sSpriteTemplate_BackButton sSpriteTemplate_OkButton sSpriteTemplate_Cursor sSpriteTemplate_InputArrow sSpriteTemplate_Underscore sSpriteTemplate_PCIcon sNamingScreenKeyboardText sSpriteSheets sSpritePalettes sPageToNextGfxId sPageToNextKeyboardId sPageToKeyboardId sPageSwapAnimStateFuncs sButtonKeyRoles sPageSwapSpriteFuncs sPageSwapPalTags sPageSwapGfxTags sIconFunctions sKeyboardKeyHandlers sInputFuncs sDrawTextEntryBoxFuncs sDrawGenderIconFuncs sGenderColors sTextColorStruct sFillValues sKeyboardTextColors sNextKeyboardPageTilemaps sPlayerNamingScreenTemplate sPCBoxNamingTemplate sMonNamingScreenTemplate sWaldaWordsScreenTemplate sNamingScreenTemplates sOam_8x8 sOam_16x16 sOam_32x16 sSubsprites_PageSwapFrame sSubsprites_PageSwapText sSubsprites_Button sSubsprites_PCIcon sSubspriteTable_PageSwapFrame sSubspriteTable_PageSwapText sSubspriteTable_Button sSubspriteTable_PCIcon sImageTable_PCIcon sAnim_Loop sAnim_CursorSquish sAnim_PCIcon sAnims_Loop sAnims_Cursor sAnims_PCIcon sSpriteTemplate_PageSwapFrame sSpriteTemplate_PageSwapButton sSpriteTemplate_PageSwapText sSpriteTemplate_BackButton sSpriteTemplate_OkButton sSpriteTemplate_Cursor sSpriteTemplate_InputArrow sSpriteTemplate_Underscore sSpriteTemplate_PCIcon sNamingScreenKeyboardText sSpriteSheets sSpritePalettes

/// `struct NamingScreenData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NamingScreenData {
    pub tilemapBuffer1: CArray<u8, 2048>,
    pub tilemapBuffer2: CArray<u8, 2048>,
    pub tilemapBuffer3: CArray<u8, 2048>,
    pub textBuffer: CArray<u8, 16>,
    pub tileBuffer: CArray<u8, 1536>,
    pub state: u8,
    pub windows: CArray<u8, 5>,
    pub inputCharBaseXPos: u16,
    pub bg1vOffset: u16,
    pub bg2vOffset: u16,
    pub bg1Priority: u16,
    pub bg2Priority: u16,
    pub bgToReveal: u8,
    pub bgToHide: u8,
    pub currentPage: u8,
    pub cursorSpriteId: u8,
    pub swapBtnFrameSpriteId: u8,
    pub keyRepeatStartDelayCopy: u8,
    pub template: *mut NamingScreenTemplate,
    pub templateNum: u8,
    pub destBuffer: *mut u8,
    pub monSpecies: u16,
    pub monGender: u16,
    pub monPersonality: u32,
    pub returnCallback: Option<unsafe extern "C" fn()>,
}

unsafe impl Sync for NamingScreenData {}

/// `struct NamingScreenTemplate`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NamingScreenTemplate {
    pub copyExistingString: u8,
    pub maxChars: u8,
    pub iconFunction: u8,
    pub addGenderIcon: u8,
    pub initialPage: u8,
    pub unused: u8,
    pub title: *mut u8,
}

unsafe impl Sync for NamingScreenTemplate {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<NamingScreenData>() == 7744);
    assert!(offset_of!(NamingScreenData, tilemapBuffer1) == 0);
    assert!(offset_of!(NamingScreenData, tilemapBuffer2) == 2048);
    assert!(offset_of!(NamingScreenData, tilemapBuffer3) == 4096);
    assert!(offset_of!(NamingScreenData, textBuffer) == 6144);
    assert!(offset_of!(NamingScreenData, tileBuffer) == 6160);
    assert!(offset_of!(NamingScreenData, state) == 7696);
    assert!(offset_of!(NamingScreenData, windows) == 7697);
    assert!(offset_of!(NamingScreenData, inputCharBaseXPos) == 7702);
    assert!(offset_of!(NamingScreenData, bg1vOffset) == 7704);
    assert!(offset_of!(NamingScreenData, bg2vOffset) == 7706);
    assert!(offset_of!(NamingScreenData, bg1Priority) == 7708);
    assert!(offset_of!(NamingScreenData, bg2Priority) == 7710);
    assert!(offset_of!(NamingScreenData, bgToReveal) == 7712);
    assert!(offset_of!(NamingScreenData, bgToHide) == 7713);
    assert!(offset_of!(NamingScreenData, currentPage) == 7714);
    assert!(offset_of!(NamingScreenData, cursorSpriteId) == 7715);
    assert!(offset_of!(NamingScreenData, swapBtnFrameSpriteId) == 7716);
    assert!(offset_of!(NamingScreenData, keyRepeatStartDelayCopy) == 7717);
    assert!(offset_of!(NamingScreenData, template) == 7720);
    assert!(offset_of!(NamingScreenData, templateNum) == 7724);
    assert!(offset_of!(NamingScreenData, destBuffer) == 7728);
    assert!(offset_of!(NamingScreenData, monSpecies) == 7732);
    assert!(offset_of!(NamingScreenData, monGender) == 7734);
    assert!(offset_of!(NamingScreenData, monPersonality) == 7736);
    assert!(offset_of!(NamingScreenData, returnCallback) == 7740);
    assert!(size_of::<NamingScreenTemplate>() == 12);
    assert!(offset_of!(NamingScreenTemplate, copyExistingString) == 0);
    assert!(offset_of!(NamingScreenTemplate, maxChars) == 1);
    assert!(offset_of!(NamingScreenTemplate, iconFunction) == 2);
    assert!(offset_of!(NamingScreenTemplate, addGenderIcon) == 3);
    assert!(offset_of!(NamingScreenTemplate, initialPage) == 4);
    assert!(offset_of!(NamingScreenTemplate, unused) == 5);
    assert!(offset_of!(NamingScreenTemplate, title) == 8);
};

const BUTTON_BACK: u8 = 1;
const BUTTON_COUNT: i16 = 3;
const BUTTON_OK: u8 = 2;
const BUTTON_PAGE: u8 = 0;
const INPUT_A_BUTTON: u8 = 5;
const INPUT_B_BUTTON: u8 = 6;
const INPUT_DPAD_DOWN: u16 = 2;
const INPUT_DPAD_LEFT: u16 = 3;
const INPUT_DPAD_RIGHT: u16 = 4;
const INPUT_DPAD_UP: u16 = 1;
const INPUT_NONE: i16 = 0;
const INPUT_SELECT: u8 = 8;
const INPUT_START: u8 = 9;
const INPUT_STATE_DISABLED: u8 = 0;
const INPUT_STATE_ENABLED: u8 = 1;
const INPUT_STATE_OVERRIDE: u8 = 2;
const KBPAGE_COUNT: i32 = 3;
const KBPAGE_LETTERS_UPPER: u8 = 1;
const KBROW_COUNT: u8 = 4;
const KEYBOARD_LETTERS_LOWER: u8 = 0;
const KEYBOARD_LETTERS_UPPER: u8 = 1;
const KEY_ROLE_BACKSPACE: u8 = 2;
const KEY_ROLE_CHAR: u8 = 0;
const PALTAG_BACK_BUTTON: u16 = 6;
const PALTAG_CURSOR: u16 = 5;
const PALTAG_OK_BUTTON: u16 = 7;
const PALTAG_PAGE_SWAP: u16 = 4;
const STATE_EXIT: u8 = 9;
const STATE_FADE_IN: u8 = 0;
const STATE_FADE_OUT: u8 = 8;
const STATE_HANDLE_INPUT: u8 = 2;
const STATE_MOVE_TO_OK_BUTTON: u8 = 3;
const STATE_PRESSED_OK: u8 = 6;
const STATE_START_PAGE_SWAP: u8 = 4;
const STATE_WAIT_FADE_IN: u8 = 1;
const STATE_WAIT_PAGE_SWAP: u8 = 5;
const STATE_WAIT_SENT_TO_PC_MESSAGE: u8 = 7;
const WIN_BANNER: i32 = 4;
const WIN_COUNT: u8 = 5;
const WIN_KB_PAGE_1: i32 = 0;
const WIN_KB_PAGE_2: i32 = 1;
const WIN_TEXT_ENTRY: i32 = 2;
const WIN_TEXT_ENTRY_BOX: i32 = 3;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::naming_screen::sBgTemplates).cast());
static sButtonKeyRoles: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sButtonKeyRoles).cast());
static sDrawGenderIconFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 2>> =
    Table((&raw const crate::data::naming_screen::sDrawGenderIconFuncs).cast());
static sDrawTextEntryBoxFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 5>> =
    Table((&raw const crate::data::naming_screen::sDrawTextEntryBoxFuncs).cast());
static sFillValues: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sFillValues).cast());
static sGenderColors: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::naming_screen::sGenderColors).cast());
static sIconFunctions: Table<CArray<Option<unsafe extern "C" fn()>, 5>> =
    Table((&raw const crate::data::naming_screen::sIconFunctions).cast());
static sInputFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 3>> =
    Table((&raw const crate::data::naming_screen::sInputFuncs).cast());
static sKeyboardChars: Table<CArray<CArray<CArray<u8, 8>, 4>, 3>> =
    Table((&raw const crate::data::naming_screen::sKeyboardChars).cast());
static sKeyboardKeyHandlers: Table<CArray<Option<unsafe extern "C" fn(u8) -> u8>, 4>> =
    Table((&raw const crate::data::naming_screen::sKeyboardKeyHandlers).cast());
static sKeyboardTextColors: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::naming_screen::sKeyboardTextColors).cast());
static sKeyboard_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::naming_screen::sKeyboard_Pal).cast());
static sNamingScreenKeyboardText: Table<CArray<CArray<*mut u8, 4>, 3>> =
    Table((&raw const crate::data::naming_screen::sNamingScreenKeyboardText).cast());
static sNamingScreenTemplates: Table<CArray<*mut NamingScreenTemplate, 5>> =
    Table((&raw const crate::data::naming_screen::sNamingScreenTemplates).cast());
static sNextKeyboardPageTilemaps: Table<CArray<*mut u32, 3>> =
    Table((&raw const crate::data::naming_screen::sNextKeyboardPageTilemaps).cast());
static sPageColumnCounts: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageColumnCounts).cast());
static sPageColumnXPos: Table<CArray<CArray<u8, 8>, 3>> =
    Table((&raw const crate::data::naming_screen::sPageColumnXPos).cast());
static sPageSwapAnimStateFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 4>> =
    Table((&raw const crate::data::naming_screen::sPageSwapAnimStateFuncs).cast());
static sPageSwapGfxTags: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::naming_screen::sPageSwapGfxTags).cast());
static sPageSwapPalTags: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::naming_screen::sPageSwapPalTags).cast());
static sPageSwapSpriteFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Sprite) -> u8>, 4>> =
    Table((&raw const crate::data::naming_screen::sPageSwapSpriteFuncs).cast());
static sPageToKeyboardId: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageToKeyboardId).cast());
static sPageToNextGfxId: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageToNextGfxId).cast());
static sPageToNextKeyboardId: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageToNextKeyboardId).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 9>> =
    Table((&raw const crate::data::naming_screen::sSpritePalettes).cast());
static sSpriteSheets: Table<CArray<SpriteSheet, 13>> =
    Table((&raw const crate::data::naming_screen::sSpriteSheets).cast());
static sSpriteTemplate_BackButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_BackButton).cast());
static sSpriteTemplate_Cursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_Cursor).cast());
static sSpriteTemplate_InputArrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_InputArrow).cast());
static sSpriteTemplate_OkButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_OkButton).cast());
static sSpriteTemplate_PCIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PCIcon).cast());
static sSpriteTemplate_PageSwapButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PageSwapButton).cast());
static sSpriteTemplate_PageSwapFrame: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PageSwapFrame).cast());
static sSpriteTemplate_PageSwapText: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PageSwapText).cast());
static sSpriteTemplate_Underscore: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_Underscore).cast());
static sSubspriteTable_Button: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_Button).cast());
static sSubspriteTable_PCIcon: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_PCIcon).cast());
static sSubspriteTable_PageSwapFrame: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_PageSwapFrame).cast());
static sSubspriteTable_PageSwapText: Table<CArray<SubspriteTable, 3>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_PageSwapText).cast());
static sText_AlphabetUpperLower: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::naming_screen::sText_AlphabetUpperLower).cast());
static sTransferredToPCMessages: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::naming_screen::sTransferredToPCMessages).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 6>> =
    Table((&raw const crate::data::naming_screen::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNamingScreen: *mut NamingScreenData = null_mut();

unsafe extern "C" {
    static mut gKeyRepeatStartDelay: u16;
    static mut gMain: Main;
    static gNamingScreenBackground_Tilemap: CArray<u32, 0>;
    static gNamingScreenKeyboardLower_Tilemap: CArray<u32, 0>;
    static gNamingScreenKeyboardUpper_Tilemap: CArray<u32, 0>;
    static gNamingScreenMenu_Gfx: CArray<u32, 0>;
    static gNamingScreenMenu_Pal: CArray<CArray<u16, 16>, 6>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_ExpandedPlaceholder_Empty: CArray<u8, 0>;
    static gText_FemaleSymbol: CArray<u8, 0>;
    static gText_MaleSymbol: CArray<u8, 0>;
    static gText_MoveOkBack: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetPCBoxToSendMon() -> u16;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetRivalAvatarGraphicsIdByStateIdAndGender(a0: u8, a1: u8) -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsDestinationBoxFull() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn MultiplyInvertedPaletteRGBComponents(a0: u16, a1: u8, a2: u8, a3: u8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn SeedRngAndSetTrainerId();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartTimer1();
    fn StringAppendN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoNamingScreen(
    templateNum: u8,
    destBuffer: *mut u8,
    monSpecies: u16,
    monGender: u16,
    monPersonality: u32,
    returnCallback: Option<unsafe extern "C" fn()>,
) {
    sNamingScreen = Alloc(7744) as *mut NamingScreenData;
    if sNamingScreen.is_null() {
        SetMainCallback2(returnCallback);
    } else {
        (*sNamingScreen).templateNum = templateNum;
        (*sNamingScreen).monSpecies = monSpecies;
        (*sNamingScreen).monGender = monGender;
        (*sNamingScreen).monPersonality = monPersonality;
        (*sNamingScreen).destBuffer = destBuffer;
        (*sNamingScreen).returnCallback = returnCallback;
        if templateNum == NAMING_SCREEN_PLAYER {
            StartTimer1();
        }
        SetMainCallback2(Some(CB2_LoadNamingScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadNamingScreen() {
    match gMain.state {
        0 => {
            ResetVHBlank();
            NamingScreen_Init();
            gMain.state += 1;
        }
        1 => {
            NamingScreen_InitBGs();
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            gMain.state += 1;
        }
        3 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            gMain.state += 1;
        }
        4 => {
            ResetTasks();
            gMain.state += 1;
        }
        5 => {
            LoadPalettes();
            gMain.state += 1;
        }
        6 => {
            LoadGfx();
            gMain.state += 1;
        }
        7 => {
            CreateSprites();
            UpdatePaletteFade();
            NamingScreen_ShowBgs();
            gMain.state += 1;
        }
        _ => {
            CreateHelperTasks();
            CreateNamingScreenTask();
        }
    }
}
pub(crate) unsafe extern "C" fn NamingScreen_Init() {
    (*sNamingScreen).state = STATE_FADE_IN;
    (*sNamingScreen).bg1vOffset = 0;
    (*sNamingScreen).bg2vOffset = 0;
    (*sNamingScreen).bg1Priority = 1;
    (*sNamingScreen).bg2Priority = 2;
    (*sNamingScreen).bgToReveal = 0;
    (*sNamingScreen).bgToHide = 1;
    (*sNamingScreen).template = sNamingScreenTemplates[(*sNamingScreen).templateNum];
    (*sNamingScreen).currentPage = (*(*sNamingScreen).template).initialPage;
    (*sNamingScreen).inputCharBaseXPos =
        ((DISPLAY_WIDTH as i32 - (*(*sNamingScreen).template).maxChars as i32 * 8) / 2) as u16 + 6;
    if (*sNamingScreen).templateNum == NAMING_SCREEN_WALDA {
        (*sNamingScreen).inputCharBaseXPos += 11;
    }
    (*sNamingScreen).keyRepeatStartDelayCopy = gKeyRepeatStartDelay as u8;
    memset((*sNamingScreen).textBuffer.as_mut_ptr(), EOS as i32, 16);
    if (*(*sNamingScreen).template).copyExistingString != 0 {
        StringCopy(
            (*sNamingScreen).textBuffer.as_mut_ptr(),
            (*sNamingScreen).destBuffer,
        );
    }
    gKeyRepeatStartDelay = 16;
}
pub(crate) unsafe extern "C" fn SetSpritesVisible() {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_SPRITES {
        if gSprites[i].inUse() != 0 {
            gSprites[i].set_invisible(FALSE as u16);
        }
        i += 1;
    }
    SetCursorInvisibility(FALSE);
}
pub(crate) unsafe extern "C" fn NamingScreen_InitBGs() {
    let mut i: u8 = 0;
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    {
        {
            let mut _dest: *mut u32 = OAM as i32 as usize as *mut c_void as *mut u32;
            let mut _size: u32 = OAM_SIZE;
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    {
        {
            let mut _dest: *mut u16 = PLTT as i32 as usize as *mut c_void as *mut u16;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    SetGpuReg(0x0, 0x0000);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    InitStandardTextBoxWindows();
    InitTextBoxGfxAndPrinters();
    i = 0;
    while i < WIN_COUNT {
        (*sNamingScreen).windows[i] = AddWindow((&raw const sWindowTemplates[i]).cast_mut()) as u8;
        i += 1;
    }
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 1600);
    SetGpuReg(REG_OFFSET_BLDALPHA, 2060);
    SetBgTilemapBuffer(
        1,
        (*sNamingScreen).tilemapBuffer1.as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sNamingScreen).tilemapBuffer2.as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sNamingScreen).tilemapBuffer3.as_mut_ptr() as *mut c_void,
    );
    FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 0x20, 0x20);
    FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, 0x20, 0x20);
    FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 0x20, 0x20);
}
pub(crate) unsafe extern "C" fn CreateNamingScreenTask() {
    CreateTask(Some(Task_NamingScreen), 2);
    SetMainCallback2(Some(CB2_NamingScreen));
}
pub(crate) unsafe extern "C" fn Task_NamingScreen(taskId: u8) {
    match (*sNamingScreen).state {
        STATE_FADE_IN => {
            MainState_FadeIn();
            SetSpritesVisible();
            SetVBlank();
        }
        STATE_WAIT_FADE_IN => {
            MainState_WaitFadeIn();
        }
        STATE_HANDLE_INPUT => {
            MainState_HandleInput();
        }
        STATE_MOVE_TO_OK_BUTTON => {
            MainState_MoveToOKButton();
            MainState_HandleInput();
        }
        STATE_START_PAGE_SWAP => {
            MainState_StartPageSwap();
        }
        STATE_WAIT_PAGE_SWAP => {
            MainState_WaitPageSwap();
        }
        STATE_PRESSED_OK => {
            MainState_PressedOKButton();
        }
        STATE_WAIT_SENT_TO_PC_MESSAGE => {
            MainState_WaitSentToPCMessage();
        }
        STATE_FADE_OUT => {
            MainState_FadeOut();
        }
        STATE_EXIT => {
            MainState_Exit();
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PageToNextGfxId(page: u8) -> u8 {
    return sPageToNextGfxId[page];
}
pub(crate) unsafe extern "C" fn CurrentPageToNextKeyboardId() -> u8 {
    return sPageToNextKeyboardId[(*sNamingScreen).currentPage];
}
pub(crate) unsafe extern "C" fn CurrentPageToKeyboardId() -> u8 {
    return sPageToKeyboardId[(*sNamingScreen).currentPage];
}
pub(crate) unsafe extern "C" fn MainState_FadeIn() -> u8 {
    DrawBgTilemap(
        3,
        gNamingScreenBackground_Tilemap.as_ptr().cast_mut() as *mut c_void,
    );
    (*sNamingScreen).currentPage = KBPAGE_LETTERS_UPPER;
    DrawBgTilemap(
        2,
        gNamingScreenKeyboardLower_Tilemap.as_ptr().cast_mut() as *mut c_void,
    );
    DrawBgTilemap(
        1,
        gNamingScreenKeyboardUpper_Tilemap.as_ptr().cast_mut() as *mut c_void,
    );
    PrintKeyboardKeys((*sNamingScreen).windows[1], KEYBOARD_LETTERS_LOWER);
    PrintKeyboardKeys((*sNamingScreen).windows[0], KEYBOARD_LETTERS_UPPER);
    NamingScreen_Dummy(2, KEYBOARD_LETTERS_LOWER);
    NamingScreen_Dummy(1, KEYBOARD_LETTERS_UPPER);
    DrawTextEntry();
    DrawTextEntryBox();
    PrintControls();
    CopyBgTilemapBufferToVram(1);
    CopyBgTilemapBufferToVram(2);
    CopyBgTilemapBufferToVram(3);
    BlendPalettes(PALETTES_ALL, 16, 0);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    (*sNamingScreen).state += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn MainState_WaitFadeIn() -> u8 {
    if gPaletteFade.active() == 0 {
        SetInputState(INPUT_STATE_ENABLED);
        SetCursorFlashing(TRUE);
        (*sNamingScreen).state += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MainState_HandleInput() -> u8 {
    return HandleKeyboardEvent();
}
pub(crate) unsafe extern "C" fn MainState_MoveToOKButton() -> u8 {
    if IsCursorAnimFinished() != 0 {
        SetInputState(INPUT_STATE_ENABLED);
        MoveCursorToOKButton();
        (*sNamingScreen).state = STATE_HANDLE_INPUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MainState_PressedOKButton() -> u8 {
    SaveInputText();
    SetInputState(INPUT_STATE_DISABLED);
    SetCursorFlashing(FALSE);
    TryStartButtonFlash(BUTTON_COUNT as u8, FALSE, TRUE);
    if (*sNamingScreen).templateNum == NAMING_SCREEN_CAUGHT_MON
        && CalculatePlayerPartyCount() >= PARTY_SIZE as u8
    {
        DisplaySentToPCMessage();
        (*sNamingScreen).state = STATE_WAIT_SENT_TO_PC_MESSAGE;
        return FALSE;
    } else {
        (*sNamingScreen).state = STATE_FADE_OUT;
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MainState_FadeOut() -> u8 {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    (*sNamingScreen).state += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn MainState_Exit() -> u8 {
    if gPaletteFade.active() == 0 {
        if (*sNamingScreen).templateNum == NAMING_SCREEN_PLAYER {
            SeedRngAndSetTrainerId();
        }
        SetMainCallback2((*sNamingScreen).returnCallback);
        DestroyTask(FindTaskIdByFunc(Some(Task_NamingScreen)));
        FreeAllWindowBuffers();
        Free(sNamingScreen as *mut c_void);
        sNamingScreen = null_mut();
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DisplaySentToPCMessage() {
    let mut stringToDisplay: u8 = 0;
    if IsDestinationBoxFull() == 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
        );
        StringCopy(gStringVar2.as_mut_ptr(), (*sNamingScreen).destBuffer);
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
        );
        StringCopy(gStringVar2.as_mut_ptr(), (*sNamingScreen).destBuffer);
        StringCopy(
            gStringVar3.as_mut_ptr(),
            GetBoxNamePtr(GetPCBoxToSendMon() as u8),
        );
        stringToDisplay = 2;
    }
    if FlagGet(FLAG_SYS_PC_LANETTE) != 0 {
        stringToDisplay += 1;
    }
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        sTransferredToPCMessages[stringToDisplay],
    );
    DrawDialogueFrame(0, 0);
    gTextFlags.set_canABSpeedUpPrint(TRUE);
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
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn MainState_WaitSentToPCMessage() -> u8 {
    RunTextPrinters();
    if IsTextPrinterActive(0) == 0 && gMain.newKeys as i32 & A_BUTTON != 0 {
        (*sNamingScreen).state = STATE_FADE_OUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MainState_StartPageSwap() -> u8 {
    SetInputState(INPUT_STATE_DISABLED);
    StartPageSwapButtonAnim();
    StartPageSwapAnim();
    SetCursorInvisibility(TRUE);
    TryStartButtonFlash(BUTTON_PAGE, FALSE, TRUE);
    PlaySE(SE_WIN_OPEN);
    (*sNamingScreen).state = STATE_WAIT_PAGE_SWAP;
    return FALSE;
}
pub(crate) unsafe extern "C" fn MainState_WaitPageSwap() -> u8 {
    let mut cursorX: i16 = 0;
    let mut cursorY: i16 = 0;
    let mut onLastColumn: u32 = 0;
    if IsPageSwapAnimNotInProgress() != 0 {
        GetCursorPos(&raw mut cursorX, &raw mut cursorY);
        onLastColumn = (cursorX == GetCurrentPageColumnCount() as i16) as u32;
        (*sNamingScreen).state = STATE_HANDLE_INPUT;
        (*sNamingScreen).currentPage += 1;
        (*sNamingScreen).currentPage = ((*sNamingScreen).currentPage as i32 % 3) as u8;
        if onLastColumn != 0 {
            cursorX = GetCurrentPageColumnCount() as i16;
        } else {
            if cursorX >= GetCurrentPageColumnCount() as i16 {
                cursorX = GetCurrentPageColumnCount() as i16 - 1;
            }
        }
        SetCursorPos(cursorX, cursorY);
        DrawKeyboardPageOnDeck();
        SetInputState(INPUT_STATE_ENABLED);
        SetCursorInvisibility(FALSE);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartPageSwapAnim() {
    let mut taskId: u8 = 0;
    taskId = CreateTask(Some(Task_HandlePageSwapAnim), 0);
    Task_HandlePageSwapAnim(taskId);
}
pub(crate) unsafe extern "C" fn Task_HandlePageSwapAnim(taskId: u8) {
    while sPageSwapAnimStateFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn IsPageSwapAnimNotInProgress() -> u8 {
    if FindTaskIdByFunc(Some(Task_HandlePageSwapAnim)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_Init(task: *mut Task) -> u8 {
    (*sNamingScreen).bg1vOffset = 0;
    (*sNamingScreen).bg2vOffset = 0;
    (*task).data[0] += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_1(task: *mut Task) -> u8 {
    let mut vOffsets: CArray<*mut u16, 2> = zeroed();
    vOffsets[0] = &raw mut (*sNamingScreen).bg2vOffset;
    vOffsets[1] = &raw mut (*sNamingScreen).bg1vOffset;
    (*task).data[1] += 4;
    *vOffsets[(*sNamingScreen).bgToReveal] = Sin((*task).data[1], 40) as u16;
    *vOffsets[(*sNamingScreen).bgToHide] = Sin((*task).data[1] + 128 & 0xFF, 40) as u16;
    if (*task).data[1] >= 64 {
        let mut temp: u8 = (*sNamingScreen).bg1Priority as u8;
        (*sNamingScreen).bg1Priority = (*sNamingScreen).bg2Priority;
        (*sNamingScreen).bg2Priority = temp as u16;
        (*task).data[0] += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_2(task: *mut Task) -> u8 {
    let mut vOffsets: CArray<*mut u16, 2> = zeroed();
    vOffsets[0] = &raw mut (*sNamingScreen).bg2vOffset;
    vOffsets[1] = &raw mut (*sNamingScreen).bg1vOffset;
    (*task).data[1] += 4;
    *vOffsets[(*sNamingScreen).bgToReveal] = Sin((*task).data[1], 40) as u16;
    *vOffsets[(*sNamingScreen).bgToHide] = Sin((*task).data[1] + 128 & 0xFF, 40) as u16;
    if (*task).data[1] >= 128 {
        let mut temp: u8 = (*sNamingScreen).bgToReveal;
        (*sNamingScreen).bgToReveal = (*sNamingScreen).bgToHide;
        (*sNamingScreen).bgToHide = temp;
        (*task).data[0] += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn PageSwapAnimState_Done(task: *mut Task) -> u8 {
    DestroyTask(FindTaskIdByFunc(Some(Task_HandlePageSwapAnim)));
    return 0;
}
pub(crate) unsafe extern "C" fn CreateButtonFlashTask() {
    let mut taskId: u8 = 0;
    taskId = CreateTask(Some(Task_UpdateButtonFlash), 3);
    gTasks[taskId].data[0] = BUTTON_COUNT;
}
pub(crate) unsafe extern "C" fn TryStartButtonFlash(
    button: u8,
    keepFlashing: u8,
    interruptCurFlash: u8,
) {
    let mut task: *mut Task = &raw mut gTasks[FindTaskIdByFunc(Some(Task_UpdateButtonFlash))];
    if button as i16 == (*task).data[0] && interruptCurFlash == 0 {
        (*task).data[1] = keepFlashing as i16;
        (*task).data[2] = TRUE as i16;
        return;
    }
    if button == BUTTON_COUNT as u8 && (*task).data[1] == 0 && interruptCurFlash == 0 {
        return;
    }
    if (*task).data[0] != BUTTON_COUNT {
        RestoreButtonColor((*task).data[0] as u8);
    }
    StartButtonFlash(task, button, keepFlashing);
}
pub(crate) unsafe extern "C" fn Task_UpdateButtonFlash(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if (*task).data[0] == BUTTON_COUNT || (*task).data[2] == 0 {
        return;
    }
    MultiplyInvertedPaletteRGBComponents(
        GetButtonPalOffset((*task).data[0] as u8),
        (*task).data[3] as u8,
        (*task).data[3] as u8,
        (*task).data[3] as u8,
    );
    if (*task).data[5] != 0
        && ({
            (*task).data[5] -= 1;
            (*task).data[5]
        }) != 0
    {
        return;
    }
    (*task).data[5] = 2;
    if (*task).data[4] >= 0 {
        if (*task).data[3] < 14 {
            (*task).data[3] += (*task).data[4];
            (*task).data[6] += (*task).data[4];
        } else {
            (*task).data[3] = 16;
            (*task).data[6] += 1;
        }
    } else {
        (*task).data[3] += (*task).data[4];
        (*task).data[6] += (*task).data[4];
    }
    if (*task).data[3] == 16 && (*task).data[6] == 22 {
        (*task).data[4] = -4;
    } else if (*task).data[3] == 0 {
        (*task).data[2] = (*task).data[1];
        (*task).data[4] = 2;
        (*task).data[6] = 0;
    }
}
pub(crate) unsafe extern "C" fn GetButtonPalOffset(button: u8) -> u16 {
    let mut palOffsets: CArray<u16, 4> = zeroed();
    palOffsets[0] = 0x100 + IndexOfSpritePaletteTag(PALTAG_PAGE_SWAP) as u16 * 16 + 14;
    palOffsets[1] = 0x100 + IndexOfSpritePaletteTag(PALTAG_BACK_BUTTON) as u16 * 16 + 14;
    palOffsets[2] = 0x100 + IndexOfSpritePaletteTag(PALTAG_OK_BUTTON) as u16 * 16 + 14;
    palOffsets[3] = 0x100 + IndexOfSpritePaletteTag(PALTAG_OK_BUTTON) as u16 * 16 + 1;
    return palOffsets[button];
}
pub(crate) unsafe extern "C" fn RestoreButtonColor(button: u8) {
    let mut index: u16 = GetButtonPalOffset(button);
    gPlttBufferFaded[index] = gPlttBufferUnfaded[index];
}
pub(crate) unsafe extern "C" fn StartButtonFlash(task: *mut Task, button: u8, keepFlashing: u8) {
    (*task).data[0] = button as i16;
    (*task).data[1] = keepFlashing as i16;
    (*task).data[2] = TRUE as i16;
    (*task).data[3] = 4;
    (*task).data[4] = 2;
    (*task).data[5] = 0;
    (*task).data[6] = 4;
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        StartSpriteAnim(sprite, 0);
    }
    (*sprite).set_invisible((*sprite).data[4] as u16 & 0x00FF);
    if (*sprite).data[0] == GetCurrentPageColumnCount() as i16 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if (*sprite).invisible() != 0
        || (*sprite).data[4] as i32 & 0xFF00 == 0
        || (*sprite).data[0] != (*sprite).data[2]
        || (*sprite).data[1] != (*sprite).data[3]
    {
        (*sprite).data[5] = 0;
        (*sprite).data[6] = 2;
        (*sprite).data[7] = 2;
    }
    (*sprite).data[7] -= 1;
    if (*sprite).data[7] == 0 {
        (*sprite).data[5] += (*sprite).data[6];
        if (*sprite).data[5] == 16 || (*sprite).data[5] == 0 {
            (*sprite).data[6] = -(*sprite).data[6];
        }
        (*sprite).data[7] = 2;
    }
    if (*sprite).data[4] as i32 & 0xFF00 != 0 {
        let mut gb: i8 = (*sprite).data[5] as i8;
        let mut r: i8 = ((*sprite).data[5] >> 1) as i8;
        let mut index: u16 = 0x100 + IndexOfSpritePaletteTag(PALTAG_CURSOR) as u16 * 16 + 1;
        MultiplyInvertedPaletteRGBComponents(index, r as u8, gb as u8, gb as u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_InputArrow(sprite: *mut Sprite) {
    let mut x: CArray<i16, 4> = CArray([0, -4, -2, -1]);
    if (*sprite).data[0] == 0
        || ({
            (*sprite).data[0] -= 1;
            (*sprite).data[0]
        }) == 0
    {
        (*sprite).data[0] = 8;
        (*sprite).data[1] = (if 0 != 0 {
            ((*sprite).data[1] as u32 + 1) % 4
        } else {
            (*sprite).data[1] as u32 + 1 & 3
        }) as i16;
    }
    (*sprite).x2 = x[(*sprite).data[1]];
}
pub(crate) unsafe extern "C" fn SpriteCB_Underscore(sprite: *mut Sprite) {
    let mut y: CArray<i16, 4> = CArray([2, 3, 2, 1]);
    let mut pos: u8 = 0;
    pos = GetTextEntryPosition();
    if pos != (*sprite).data[0] as u8 {
        (*sprite).y2 = 0;
        (*sprite).data[1] = 0;
        (*sprite).data[2] = 0;
    } else {
        (*sprite).y2 = y[(*sprite).data[1]];
        (*sprite).data[2] += 1;
        if (*sprite).data[2] > 8 {
            (*sprite).data[1] = (if 0 != 0 {
                ((*sprite).data[1] as u32 + 1) % 4
            } else {
                (*sprite).data[1] as u32 + 1 & 3
            }) as i16;
            (*sprite).data[2] = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSprites() {
    CreateCursorSprite();
    CreatePageSwapButtonSprites();
    CreateBackOkSprites();
    CreateTextEntrySprites();
    CreateInputTargetIcon();
}
pub(crate) unsafe extern "C" fn CreateCursorSprite() {
    (*sNamingScreen).cursorSpriteId =
        CreateSprite((&raw const *sSpriteTemplate_Cursor).cast_mut(), 38, 88, 1);
    SetCursorInvisibility(TRUE);
    gSprites[(*sNamingScreen).cursorSpriteId]
        .oam
        .set_priority(1);
    gSprites[(*sNamingScreen).cursorSpriteId]
        .oam
        .set_objMode(ST_OAM_OBJ_BLEND);
    gSprites[(*sNamingScreen).cursorSpriteId].data[6] = 1;
    gSprites[(*sNamingScreen).cursorSpriteId].data[6] = 2;
    SetCursorPos(0, 0);
}
pub(crate) unsafe extern "C" fn SetCursorPos(x: i16, y: i16) {
    let mut cursorSprite: *mut Sprite = &raw mut gSprites[(*sNamingScreen).cursorSpriteId];
    if x < sPageColumnCounts[CurrentPageToKeyboardId()] as i16 {
        (*cursorSprite).x = sPageColumnXPos[CurrentPageToKeyboardId()][x] as i16 + 38;
    } else {
        (*cursorSprite).x = 0;
    }
    (*cursorSprite).y = y * 16 + 88;
    (*cursorSprite).data[2] = (*cursorSprite).data[0];
    (*cursorSprite).data[3] = (*cursorSprite).data[1];
    (*cursorSprite).data[0] = x;
    (*cursorSprite).data[1] = y;
}
pub(crate) unsafe extern "C" fn GetCursorPos(x: *mut i16, y: *mut i16) {
    let mut cursorSprite: *mut Sprite = &raw mut gSprites[(*sNamingScreen).cursorSpriteId];
    *x = (*cursorSprite).data[0];
    *y = (*cursorSprite).data[1];
}
pub(crate) unsafe extern "C" fn MoveCursorToOKButton() {
    SetCursorPos(GetCurrentPageColumnCount() as i16, 2);
}
pub(crate) unsafe extern "C" fn SetCursorInvisibility(invisible: u8) {
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] &= -256;
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] |= invisible as i16;
    StartSpriteAnim(&raw mut gSprites[(*sNamingScreen).cursorSpriteId], 0);
}
pub(crate) unsafe extern "C" fn SetCursorFlashing(flashing: u8) {
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] &= 0xFF;
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] |= (flashing as i16) << 8;
}
pub(crate) unsafe extern "C" fn SquishCursor() {
    StartSpriteAnim(&raw mut gSprites[(*sNamingScreen).cursorSpriteId], 1);
}
pub(crate) unsafe extern "C" fn IsCursorAnimFinished() -> u8 {
    return gSprites[(*sNamingScreen).cursorSpriteId].animEnded() as u8;
}
pub(crate) unsafe extern "C" fn GetKeyRoleAtCursorPos() -> u8 {
    let mut cursorX: i16 = 0;
    let mut cursorY: i16 = 0;
    GetCursorPos(&raw mut cursorX, &raw mut cursorY);
    if cursorX < GetCurrentPageColumnCount() as i16 {
        return KEY_ROLE_CHAR;
    } else {
        return sButtonKeyRoles[cursorY];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetCurrentPageColumnCount() -> u8 {
    return sPageColumnCounts[CurrentPageToKeyboardId()];
}
pub(crate) unsafe extern "C" fn CreatePageSwapButtonSprites() {
    let mut frameSpriteId: u8 = 0;
    let mut textSpriteId: u8 = 0;
    let mut buttonSpriteId: u8 = 0;
    frameSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_PageSwapFrame).cast_mut(),
        204,
        88,
        0,
    );
    (*sNamingScreen).swapBtnFrameSpriteId = frameSpriteId;
    SetSubspriteTables(
        &raw mut gSprites[frameSpriteId],
        sSubspriteTable_PageSwapFrame.as_ptr().cast_mut(),
    );
    gSprites[frameSpriteId].set_invisible(TRUE as u16);
    textSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_PageSwapText).cast_mut(),
        204,
        84,
        1,
    );
    gSprites[frameSpriteId].data[6] = textSpriteId as i16;
    SetSubspriteTables(
        &raw mut gSprites[textSpriteId],
        sSubspriteTable_PageSwapText.as_ptr().cast_mut(),
    );
    gSprites[textSpriteId].set_invisible(TRUE as u16);
    buttonSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_PageSwapButton).cast_mut(),
        204,
        83,
        2,
    );
    gSprites[buttonSpriteId].oam.set_priority(1);
    gSprites[frameSpriteId].data[7] = buttonSpriteId as i16;
    gSprites[buttonSpriteId].set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn StartPageSwapButtonAnim() {
    let mut sprite: *mut Sprite = &raw mut gSprites[(*sNamingScreen).swapBtnFrameSpriteId];
    (*sprite).data[0] = 2;
    (*sprite).data[1] = (*sNamingScreen).currentPage as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_PageSwap(sprite: *mut Sprite) {
    while sPageSwapSpriteFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite) != 0 {}
}
pub(crate) unsafe extern "C" fn PageSwapSprite_Init(sprite: *mut Sprite) -> u8 {
    let mut text: *mut Sprite = &raw mut gSprites[(*sprite).data[6]];
    let mut button: *mut Sprite = &raw mut gSprites[(*sprite).data[7]];
    SetPageSwapButtonGfx(PageToNextGfxId((*sNamingScreen).currentPage), text, button);
    (*sprite).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn PageSwapSprite_Idle(sprite: *mut Sprite) -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn PageSwapSprite_SlideOff(sprite: *mut Sprite) -> u8 {
    let mut text: *mut Sprite = &raw mut gSprites[(*sprite).data[6]];
    let mut button: *mut Sprite = &raw mut gSprites[(*sprite).data[7]];
    (*text).y2 += 1;
    if (*text).y2 > 7 {
        (*sprite).data[0] += 1;
        (*text).y2 = -4;
        (*text).set_invisible(TRUE as u16);
        SetPageSwapButtonGfx(
            PageToNextGfxId((((*sprite).data[1] as u8 as i32 + 1) % 3) as u8),
            text,
            button,
        );
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PageSwapSprite_SlideOn(sprite: *mut Sprite) -> u8 {
    let mut text: *mut Sprite = &raw mut gSprites[(*sprite).data[6]];
    (*text).set_invisible(FALSE as u16);
    (*text).y2 += 1;
    if (*text).y2 >= 0 {
        (*text).y2 = 0;
        (*sprite).data[0] = 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetPageSwapButtonGfx(
    page: u8,
    text: *mut Sprite,
    button: *mut Sprite,
) {
    (*button)
        .oam
        .set_paletteNum(IndexOfSpritePaletteTag(sPageSwapPalTags[page]) as u16);
    (*text).sheetTileStart = GetSpriteTileStartByTag(sPageSwapGfxTags[page]);
    (*text).set_subspriteTableNum(page);
}
pub(crate) unsafe extern "C" fn CreateBackOkSprites() {
    let mut spriteId: u8 = 0;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_BackButton).cast_mut(),
        204,
        116,
        0,
    );
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_Button.as_ptr().cast_mut(),
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_OkButton).cast_mut(),
        204,
        140,
        0,
    );
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_Button.as_ptr().cast_mut(),
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn CreateTextEntrySprites() {
    let mut spriteId: u8 = 0;
    let mut xPos: i16 = 0;
    let mut i: u8 = 0;
    xPos = (*sNamingScreen).inputCharBaseXPos as i16 - 5;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_InputArrow).cast_mut(),
        xPos,
        56,
        0,
    );
    gSprites[spriteId].oam.set_priority(3);
    gSprites[spriteId].set_invisible(TRUE as u16);
    xPos = (*sNamingScreen).inputCharBaseXPos as i16;
    i = 0;
    while i < (*(*sNamingScreen).template).maxChars {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Underscore).cast_mut(),
            xPos + 3,
            60,
            0,
        );
        gSprites[spriteId].oam.set_priority(3);
        gSprites[spriteId].data[0] = i as i16;
        gSprites[spriteId].set_invisible(TRUE as u16);
        i += 1;
        xPos += 8;
    }
}
pub(crate) unsafe extern "C" fn CreateInputTargetIcon() {
    sIconFunctions[(*(*sNamingScreen).template).iconFunction].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn NamingScreen_NoIcon() {}
pub(crate) unsafe extern "C" fn NamingScreen_CreatePlayerIcon() {
    let mut rivalGfxId: u8 = 0;
    let mut spriteId: u8 = 0;
    rivalGfxId = GetRivalAvatarGraphicsIdByStateIdAndGender(
        PLAYER_AVATAR_STATE_NORMAL,
        (*sNamingScreen).monSpecies as u8,
    );
    spriteId = CreateObjectGraphicsSprite(rivalGfxId as u16, Some(SpriteCallbackDummy), 56, 37, 0);
    gSprites[spriteId].oam.set_priority(3);
    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_SOUTH);
}
pub(crate) unsafe extern "C" fn NamingScreen_CreatePCIcon() {
    let mut spriteId: u8 = 0;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_PCIcon).cast_mut(), 56, 41, 0);
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_PCIcon.as_ptr().cast_mut(),
    );
    gSprites[spriteId].oam.set_priority(3);
}
pub(crate) unsafe extern "C" fn NamingScreen_CreateMonIcon() {
    let mut spriteId: u8 = 0;
    LoadMonIconPalettes();
    spriteId = CreateMonIcon(
        (*sNamingScreen).monSpecies,
        Some(SpriteCallbackDummy),
        56,
        40,
        0,
        (*sNamingScreen).monPersonality,
        1,
    );
    gSprites[spriteId].oam.set_priority(3);
}
pub(crate) unsafe extern "C" fn NamingScreen_CreateWaldaDadIcon() {
    let mut spriteId: u8 = 0;
    spriteId =
        CreateObjectGraphicsSprite(OBJ_EVENT_GFX_MAN_1, Some(SpriteCallbackDummy), 56, 37, 0);
    gSprites[spriteId].oam.set_priority(3);
    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_SOUTH);
}
pub(crate) unsafe extern "C" fn HandleKeyboardEvent() -> u8 {
    let mut input: u8 = GetInputEvent();
    let mut keyRole: u8 = GetKeyRoleAtCursorPos();
    if input == INPUT_SELECT {
        return SwapKeyboardPage();
    } else if input == INPUT_B_BUTTON {
        DeleteTextCharacter();
        return FALSE;
    } else if input == INPUT_START {
        MoveCursorToOKButton();
        return FALSE;
    } else {
        return sKeyboardKeyHandlers[keyRole].unwrap_unchecked()(input);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_Character(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_COUNT as u8, FALSE, FALSE);
    if input == INPUT_A_BUTTON {
        let mut textFull: u8 = AddTextCharacter();
        SquishCursor();
        if textFull != 0 {
            SetInputState(INPUT_STATE_OVERRIDE);
            (*sNamingScreen).state = STATE_MOVE_TO_OK_BUTTON;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_Page(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_PAGE, TRUE, FALSE);
    if input == INPUT_A_BUTTON {
        return SwapKeyboardPage();
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_Backspace(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_BACK, TRUE, FALSE);
    if input == INPUT_A_BUTTON {
        DeleteTextCharacter();
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn KeyboardKeyHandler_OK(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_OK, TRUE, FALSE);
    if input == INPUT_A_BUTTON {
        PlaySE(SE_SELECT);
        (*sNamingScreen).state = STATE_PRESSED_OK;
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SwapKeyboardPage() -> u8 {
    (*sNamingScreen).state = STATE_START_PAGE_SWAP;
    return TRUE;
}
pub(crate) unsafe extern "C" fn CreateInputHandlerTask() {
    CreateTask(Some(Task_HandleInput), 1);
}
pub(crate) unsafe extern "C" fn GetInputEvent() -> u8 {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_HandleInput));
    return gTasks[taskId].data[1] as u8;
}
pub(crate) unsafe extern "C" fn SetInputState(state: u8) {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_HandleInput));
    gTasks[taskId].data[0] = state as i16;
}
pub(crate) unsafe extern "C" fn Task_HandleInput(taskId: u8) {
    sInputFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn Input_Disabled(task: *mut Task) {
    (*task).data[1] = INPUT_NONE;
}
pub(crate) unsafe extern "C" fn Input_Enabled(task: *mut Task) {
    (*task).data[1] = INPUT_NONE;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*task).data[1] = INPUT_A_BUTTON as i16;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*task).data[1] = INPUT_B_BUTTON as i16;
    } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        (*task).data[1] = INPUT_SELECT as i16;
    } else if gMain.newKeys as i32 & START_BUTTON != 0 {
        (*task).data[1] = INPUT_START as i16;
    } else {
        HandleDpadMovement(task);
    }
}
pub(crate) unsafe extern "C" fn Input_Override(task: *mut Task) {
    (*task).data[1] = INPUT_NONE;
}
pub(crate) unsafe extern "C" fn HandleDpadMovement(task: *mut Task) {
    let mut sDpadDeltaX: CArray<i16, 5> = zeroed();
    sDpadDeltaX[0] = 0;
    sDpadDeltaX[1] = 0;
    sDpadDeltaX[2] = 0;
    sDpadDeltaX[3] = -1;
    sDpadDeltaX[4] = 1;
    let mut sDpadDeltaY: CArray<i16, 5> = zeroed();
    sDpadDeltaY[0] = 0;
    sDpadDeltaY[1] = -1;
    sDpadDeltaY[2] = 1;
    sDpadDeltaY[3] = 0;
    sDpadDeltaY[4] = 0;
    let mut sKeyRowToButtonRow: CArray<i16, 4> = CArray([0, 1, 1, 2]);
    let mut sButtonRowToKeyRow: CArray<i16, 3> = CArray([0, 0, 3]);
    let mut cursorX: i16 = 0;
    let mut cursorY: i16 = 0;
    let mut input: u16 = 0;
    let mut prevCursorX: i16 = 0;
    GetCursorPos(&raw mut cursorX, &raw mut cursorY);
    input = INPUT_NONE as u16;
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        input = INPUT_DPAD_UP;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        input = INPUT_DPAD_DOWN;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        input = INPUT_DPAD_LEFT;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        input = INPUT_DPAD_RIGHT;
    }
    prevCursorX = cursorX;
    cursorX += sDpadDeltaX[input];
    cursorY += sDpadDeltaY[input];
    if cursorX < 0 {
        cursorX = GetCurrentPageColumnCount() as i16;
    }
    if cursorX > GetCurrentPageColumnCount() as i16 {
        cursorX = 0;
    }
    if sDpadDeltaX[input] != 0 {
        if cursorX == GetCurrentPageColumnCount() as i16 {
            (*task).data[2] = cursorY;
            cursorY = sKeyRowToButtonRow[cursorY];
        } else if prevCursorX == GetCurrentPageColumnCount() as i16 {
            if cursorY == 1 {
                cursorY = (*task).data[2];
            } else {
                cursorY = sButtonRowToKeyRow[cursorY];
            }
        }
    }
    if cursorX == GetCurrentPageColumnCount() as i16 {
        if cursorY < 0 {
            cursorY = 2;
        }
        if cursorY >= BUTTON_COUNT {
            cursorY = 0;
        }
        if cursorY == 0 {
            (*task).data[2] = BUTTON_BACK as i16;
        } else if cursorY == 2 {
            (*task).data[2] = BUTTON_OK as i16;
        }
    } else {
        if cursorY < 0 {
            cursorY = 3;
        }
        if cursorY > 3 {
            cursorY = 0;
        }
    }
    SetCursorPos(cursorX, cursorY);
}
pub(crate) unsafe extern "C" fn DrawNormalTextEntryBox() {
    FillWindowPixelBuffer((*sNamingScreen).windows[3], 17);
    AddTextPrinterParameterized(
        (*sNamingScreen).windows[3],
        FONT_NORMAL,
        (*(*sNamingScreen).template).title,
        8,
        1,
        0,
        None,
    );
    PutWindowTilemap((*sNamingScreen).windows[3]);
}
pub(crate) unsafe extern "C" fn DrawMonTextEntryBox() {
    let mut buffer: CArray<u8, 32> = zeroed();
    StringCopy(
        buffer.as_mut_ptr(),
        gSpeciesNames[(*sNamingScreen).monSpecies]
            .as_ptr()
            .cast_mut(),
    );
    StringAppendN(buffer.as_mut_ptr(), (*(*sNamingScreen).template).title, 15);
    FillWindowPixelBuffer((*sNamingScreen).windows[3], 17);
    AddTextPrinterParameterized(
        (*sNamingScreen).windows[3],
        FONT_NORMAL,
        buffer.as_mut_ptr(),
        8,
        1,
        0,
        None,
    );
    PutWindowTilemap((*sNamingScreen).windows[3]);
}
pub(crate) unsafe extern "C" fn DrawTextEntryBox() {
    sDrawTextEntryBoxFuncs[(*sNamingScreen).templateNum].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn TryDrawGenderIcon() {
    sDrawGenderIconFuncs[(*(*sNamingScreen).template).addGenderIcon].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn DummyGenderIcon() {}
pub(crate) unsafe extern "C" fn DrawGenderIcon() {
    let mut text: CArray<u8, 2> = zeroed();
    let mut isFemale: u8 = FALSE;
    StringCopy(text.as_mut_ptr(), gText_MaleSymbol.as_ptr().cast_mut());
    if (*sNamingScreen).monGender != MON_GENDERLESS as u16 {
        if (*sNamingScreen).monGender == MON_FEMALE as u16 {
            StringCopy(text.as_mut_ptr(), gText_FemaleSymbol.as_ptr().cast_mut());
            isFemale = TRUE;
        }
        AddTextPrinterParameterized3(
            (*sNamingScreen).windows[2],
            FONT_NORMAL,
            104,
            1,
            sGenderColors[isFemale].as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetCharAtKeyboardPos(x: i16, y: i16) -> u8 {
    return sKeyboardChars[CurrentPageToKeyboardId()][y][x];
}
pub(crate) unsafe extern "C" fn GetTextEntryPosition() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < (*(*sNamingScreen).template).maxChars {
        if (*sNamingScreen).textBuffer[i] == EOS {
            return i;
        }
        i += 1;
    }
    return (*(*sNamingScreen).template).maxChars - 1;
}
pub(crate) unsafe extern "C" fn GetPreviousTextCaretPosition() -> u8 {
    let mut i: i8 = 0;
    i = (*(*sNamingScreen).template).maxChars as i8 - 1;
    while i > 0 {
        if (*sNamingScreen).textBuffer[i] != EOS {
            return i as u8;
        }
        i -= 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn DeleteTextCharacter() {
    let mut index: u8 = 0;
    let mut keyRole: u8 = 0;
    index = GetPreviousTextCaretPosition();
    (*sNamingScreen).textBuffer[index] = 0;
    DrawTextEntry();
    CopyBgTilemapBufferToVram(3);
    (*sNamingScreen).textBuffer[index] = EOS;
    keyRole = GetKeyRoleAtCursorPos();
    if keyRole == KEY_ROLE_CHAR || keyRole == KEY_ROLE_BACKSPACE {
        TryStartButtonFlash(BUTTON_BACK, FALSE, TRUE);
    }
    PlaySE(SE_BALL);
}
pub(crate) unsafe extern "C" fn AddTextCharacter() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetCursorPos(&raw mut x, &raw mut y);
    BufferCharacter(GetCharAtKeyboardPos(x, y));
    DrawTextEntry();
    CopyBgTilemapBufferToVram(3);
    PlaySE(SE_SELECT);
    if GetPreviousTextCaretPosition() as i32 != (*(*sNamingScreen).template).maxChars as i32 - 1 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn BufferCharacter(ch: u8) {
    let mut index: u8 = GetTextEntryPosition();
    (*sNamingScreen).textBuffer[index] = ch;
}
pub(crate) unsafe extern "C" fn SaveInputText() {
    let mut i: u8 = 0;
    i = 0;
    while i < (*(*sNamingScreen).template).maxChars {
        if (*sNamingScreen).textBuffer[i] != CHAR_SPACE && (*sNamingScreen).textBuffer[i] != EOS {
            StringCopyN(
                (*sNamingScreen).destBuffer,
                (*sNamingScreen).textBuffer.as_mut_ptr(),
                (*(*sNamingScreen).template).maxChars + 1,
            );
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadGfx() {
    LZ77UnCompWram(
        gNamingScreenMenu_Gfx.as_ptr().cast_mut(),
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadBgTiles(
        1,
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
        1536,
        0,
    );
    LoadBgTiles(
        2,
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
        1536,
        0,
    );
    LoadBgTiles(
        3,
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
        1536,
        0,
    );
    LoadSpriteSheets(sSpriteSheets.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn CreateHelperTasks() {
    CreateInputHandlerTask();
    CreateButtonFlashTask();
}
pub(crate) unsafe extern "C" fn LoadPalettes() {
    LoadPalette(
        gNamingScreenMenu_Pal.as_ptr().cast_mut() as *mut c_void,
        0,
        192,
    );
    LoadPalette(sKeyboard_Pal.as_ptr().cast_mut() as *mut c_void, 160, 32);
    LoadPalette(GetTextWindowPalette(2) as *mut c_void, 176, 32);
}
pub(crate) unsafe extern "C" fn DrawBgTilemap(bg: u8, src: *mut c_void) {
    CopyToBgTilemapBuffer(bg, src, 0, 0);
}
pub(crate) unsafe extern "C" fn NamingScreen_Dummy(bg: u8, page: u8) {}
pub(crate) unsafe extern "C" fn DrawTextEntry() {
    let mut i: u8 = 0;
    let mut temp: CArray<u8, 2> = zeroed();
    let mut extraWidth: u16 = 0;
    let mut maxChars: u8 = (*(*sNamingScreen).template).maxChars;
    let mut x: u16 = (*sNamingScreen).inputCharBaseXPos - 0x40;
    FillWindowPixelBuffer((*sNamingScreen).windows[2], 17);
    i = 0;
    while i < maxChars {
        temp[0] = (*sNamingScreen).textBuffer[i];
        temp[1] = gText_ExpandedPlaceholder_Empty[0];
        extraWidth = (if IsWideLetter(temp[0]) == TRUE { 2 } else { 0 }) as u16;
        AddTextPrinterParameterized(
            (*sNamingScreen).windows[2],
            FONT_NORMAL,
            temp.as_mut_ptr(),
            i * 8 + x as u8 + extraWidth as u8,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    TryDrawGenderIcon();
    CopyWindowToVram((*sNamingScreen).windows[2], COPYWIN_GFX);
    PutWindowTilemap((*sNamingScreen).windows[2]);
}
pub(crate) unsafe extern "C" fn PrintKeyboardKeys(window: u8, page: u8) {
    let mut i: u8 = 0;
    FillWindowPixelBuffer(window, sFillValues[page]);
    i = 0;
    while i < KBROW_COUNT {
        AddTextPrinterParameterized3(
            window,
            FONT_NORMAL,
            0,
            i * 16 + 1,
            sKeyboardTextColors[page],
            0,
            sNamingScreenKeyboardText[page][i],
        );
        i += 1;
    }
    PutWindowTilemap(window);
}
pub(crate) unsafe extern "C" fn DrawKeyboardPageOnDeck() {
    let mut bg: u8 = 0;
    let mut bg_: u8 = 0;
    let mut windowId: u8 = 0;
    let mut bg1Priority: u8 = GetGpuReg(REG_OFFSET_BG1CNT) as u8 & 3;
    let mut bg2Priority: u8 = GetGpuReg(REG_OFFSET_BG2CNT) as u8 & 3;
    if bg1Priority > bg2Priority {
        bg = 1;
        bg_ = 1;
        windowId = (*sNamingScreen).windows[0];
    } else {
        bg = 2;
        bg_ = 2;
        windowId = (*sNamingScreen).windows[1];
    }
    DrawBgTilemap(
        bg,
        sNextKeyboardPageTilemaps[(*sNamingScreen).currentPage] as *mut c_void,
    );
    PrintKeyboardKeys(windowId, CurrentPageToNextKeyboardId());
    NamingScreen_Dummy(bg, CurrentPageToNextKeyboardId());
    CopyBgTilemapBufferToVram(bg_);
}
pub(crate) unsafe extern "C" fn PrintControls() {
    let mut color: CArray<u8, 3> = CArray([15, 1, 2]);
    FillWindowPixelBuffer((*sNamingScreen).windows[4], 255);
    AddTextPrinterParameterized3(
        (*sNamingScreen).windows[4],
        FONT_SMALL,
        2,
        1,
        color.as_mut_ptr(),
        0,
        gText_MoveOkBack.as_ptr().cast_mut(),
    );
    PutWindowTilemap((*sNamingScreen).windows[4]);
    CopyWindowToVram((*sNamingScreen).windows[4], COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn CB2_NamingScreen() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn ResetVHBlank() {
    SetVBlankCallback(None);
    SetHBlankCallback(None);
}
pub(crate) unsafe extern "C" fn SetVBlank() {
    SetVBlankCallback(Some(VBlankCB_NamingScreen));
}
pub(crate) unsafe extern "C" fn VBlankCB_NamingScreen() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    SetGpuReg(REG_OFFSET_BG1VOFS, (*sNamingScreen).bg1vOffset);
    SetGpuReg(REG_OFFSET_BG2VOFS, (*sNamingScreen).bg2vOffset);
    SetGpuReg(REG_OFFSET_BG1CNT, GetGpuReg(REG_OFFSET_BG1CNT) & 0xFFFC);
    SetGpuRegBits(REG_OFFSET_BG1CNT, (*sNamingScreen).bg1Priority);
    SetGpuReg(REG_OFFSET_BG2CNT, GetGpuReg(REG_OFFSET_BG2CNT) & 0xFFFC);
    SetGpuRegBits(REG_OFFSET_BG2CNT, (*sNamingScreen).bg2Priority);
}
pub(crate) unsafe extern "C" fn NamingScreen_ShowBgs() {
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
pub(crate) unsafe extern "C" fn IsWideLetter(character: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while sText_AlphabetUpperLower[i] != EOS {
        if character == sText_AlphabetUpperLower[i] {
            return FALSE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenPlayer() {
    DoNamingScreen(
        NAMING_SCREEN_PLAYER,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenBox() {
    DoNamingScreen(
        NAMING_SCREEN_BOX,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenCaughtMon() {
    DoNamingScreen(
        NAMING_SCREEN_CAUGHT_MON,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
pub(crate) unsafe extern "C" fn Debug_NamingScreenNickname() {
    DoNamingScreen(
        NAMING_SCREEN_NICKNAME,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
