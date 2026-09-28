//! Translated from `src/pokenav_match_call_gfx.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMatchCallUI_Pal sMatchCallUI_Gfx sMatchCallUI_Tilemap sOptionsCursor_Pal sOptionsCursor_Gfx sCallWindow_Pal sListWindow_Pal sPokeball_Pal sPokeball_Gfx sMatchCallBgTemplates sMatchCallLoopTaskFuncs sMatchCallLocationWindowTemplate sMatchCallInfoBoxWindowTemplate sMatchCallOptionTexts sText_CallingDots sCallMsgBoxWindowTemplate sOptionsCursorSpriteSheets sOptionsCursorSpritePalettes sOptionsCursorOamData sOptionsCursorSpriteTemplate sTrainerPicOamData sTrainerPicSpriteTemplate

/// `struct Pokenav_MatchCallGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_MatchCallGfx {
    pub isTaskActiveCB: Option<unsafe extern "C" fn() -> u32>,
    pub loopTaskId: u32,
    pub filler8: CArray<u8, 6>,
    pub skipHangUpSE: u8,
    pub newRematchRequest: u8,
    pub locWindowId: u16,
    pub infoBoxWindowId: u16,
    pub msgBoxWindowId: u16,
    pub pageDelta: i16,
    pub unused18: u8,
    pub unused19: u8,
    pub trainerPicPalOffset: u16,
    pub optionsCursorSprite: *mut Sprite,
    pub trainerPicSprite: *mut Sprite,
    pub bgTilemapBuffer1: CArray<u8, 2048>,
    pub unusedTilemapBuffer: CArray<u8, 2048>,
    pub bgTilemapBuffer2: CArray<u8, 2048>,
    pub trainerPicGfxPtr: *mut u8,
    pub trainerPicGfx: CArray<u8, 2048>,
    pub trainerPicPal: CArray<u8, 32>,
}

unsafe impl Sync for Pokenav_MatchCallGfx {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_MatchCallGfx>() == 8264);
    assert!(offset_of!(Pokenav_MatchCallGfx, isTaskActiveCB) == 0);
    assert!(offset_of!(Pokenav_MatchCallGfx, loopTaskId) == 4);
    assert!(offset_of!(Pokenav_MatchCallGfx, filler8) == 8);
    assert!(offset_of!(Pokenav_MatchCallGfx, skipHangUpSE) == 14);
    assert!(offset_of!(Pokenav_MatchCallGfx, newRematchRequest) == 15);
    assert!(offset_of!(Pokenav_MatchCallGfx, locWindowId) == 16);
    assert!(offset_of!(Pokenav_MatchCallGfx, infoBoxWindowId) == 18);
    assert!(offset_of!(Pokenav_MatchCallGfx, msgBoxWindowId) == 20);
    assert!(offset_of!(Pokenav_MatchCallGfx, pageDelta) == 22);
    assert!(offset_of!(Pokenav_MatchCallGfx, unused18) == 24);
    assert!(offset_of!(Pokenav_MatchCallGfx, unused19) == 25);
    assert!(offset_of!(Pokenav_MatchCallGfx, trainerPicPalOffset) == 26);
    assert!(offset_of!(Pokenav_MatchCallGfx, optionsCursorSprite) == 28);
    assert!(offset_of!(Pokenav_MatchCallGfx, trainerPicSprite) == 32);
    assert!(offset_of!(Pokenav_MatchCallGfx, bgTilemapBuffer1) == 36);
    assert!(offset_of!(Pokenav_MatchCallGfx, unusedTilemapBuffer) == 2084);
    assert!(offset_of!(Pokenav_MatchCallGfx, bgTilemapBuffer2) == 4132);
    assert!(offset_of!(Pokenav_MatchCallGfx, trainerPicGfxPtr) == 6180);
    assert!(offset_of!(Pokenav_MatchCallGfx, trainerPicGfx) == 6184);
    assert!(offset_of!(Pokenav_MatchCallGfx, trainerPicPal) == 8232);
};

const GFXTAG_CURSOR: u16 = 7;
const GFXTAG_TRAINER_PIC: u16 = 8;
const PALTAG_CURSOR: u16 = 12;
const PALTAG_TRAINER_PIC: u16 = 13;
const POKEBALL_ICON_BOTTOM: u16 = 20481;
const POKEBALL_ICON_EMPTY: u16 = 20482;
const POKEBALL_ICON_TOP: u16 = 20480;

static sCallMsgBoxWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sCallMsgBoxWindowTemplate).cast());
static sCallWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sCallWindow_Pal).cast());
static sListWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sListWindow_Pal).cast());
static sMatchCallBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallBgTemplates).cast());
static sMatchCallInfoBoxWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallInfoBoxWindowTemplate).cast());
static sMatchCallLocationWindowTemplate: Table<WindowTemplate> = Table(
    (&raw const crate::data::pokenav_match_call_gfx::sMatchCallLocationWindowTemplate).cast(),
);
static sMatchCallLoopTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(i32) -> u32>, 16>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallLoopTaskFuncs).cast());
static sMatchCallOptionTexts: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallOptionTexts).cast());
static sMatchCallUI_Gfx: Table<CArray<u32, 41>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallUI_Gfx).cast());
static sMatchCallUI_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallUI_Pal).cast());
static sMatchCallUI_Tilemap: Table<CArray<u32, 49>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sMatchCallUI_Tilemap).cast());
static sOptionsCursorSpritePalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sOptionsCursorSpritePalettes).cast());
static sOptionsCursorSpriteSheets: Table<CArray<CompressedSpriteSheet, 1>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sOptionsCursorSpriteSheets).cast());
static sOptionsCursorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sOptionsCursorSpriteTemplate).cast());
static sPokeball_Gfx: Table<CArray<u32, 11>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sPokeball_Gfx).cast());
static sPokeball_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sPokeball_Pal).cast());
static sText_CallingDots: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sText_CallingDots).cast());
static sTrainerPicSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_match_call_gfx::sTrainerPicSpriteTemplate).cast());

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_NumberOfBattles: CArray<u8, 0>;
    static gText_NumberRegistered: CArray<u8, 0>;
    static gText_TrainerCloseBy: CArray<u8, 0>;
    static gText_Unknown: CArray<u8, 0>;
    static gTrainerFrontPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gTrainerFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn BufferMatchCallNameAndDesc(a0: *mut PokenavMatchCallEntry, a1: *mut u8);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckForSpaceForDma3Request(a0: i16) -> i16;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreatePokenavList(a0: *mut BgTemplate, a1: *mut PokenavListTemplate, a2: u32) -> u32;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DecompressPicFromTable(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32);
    fn DestroyPokenavList();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DrawMatchCallTextBoxBorder(a0: u32, a1: u32, a2: u32);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FadeToBlackExceptPrimary();
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetGameStat(a0: u8) -> u32;
    fn GetIndexDeltaOfNextCheckPageDown(a0: i32) -> i32;
    fn GetIndexDeltaOfNextCheckPageUp(a0: i32) -> i32;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMatchCallList() -> *mut PokenavMatchCallEntry;
    fn GetMatchCallMapSec(a0: i32) -> u16;
    fn GetMatchCallMessageText(a0: i32, a1: *mut u8) -> *mut u8;
    fn GetMatchCallOptionCursorPos() -> u16;
    fn GetMatchCallOptionId(a0: i32) -> u16;
    fn GetMatchCallTrainerPic(a0: i32) -> i32;
    fn GetNumberRegistered() -> i32;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetSpinningPokenavSprite() -> *mut Sprite;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn HideSpinningPokenavSprite();
    fn InitBgTemplates(a0: *mut BgTemplate, a1: i32);
    fn IsCreatePokenavListTaskActive() -> u32;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsMatchCallListInitFinished() -> i32;
    fn IsPaletteFadeActive() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadMatchCallWindowGfx(a0: u32, a1: u32, a2: u32);
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn PlaySE(a0: u16);
    fn PokenavCopyPalette(a0: *mut u16, a1: *mut u16, a2: i32, a3: i32, a4: i32, a5: *mut u16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavList_DrawCurrentItemIcon();
    fn PokenavList_EraseListForCheckPage();
    fn PokenavList_GetSelectedIndex() -> u32;
    fn PokenavList_GetTopIndex() -> u32;
    fn PokenavList_IsMoveWindowTaskActive() -> u32;
    fn PokenavList_IsTaskActive() -> u32;
    fn PokenavList_MoveCursorDown() -> i32;
    fn PokenavList_MoveCursorUp() -> i32;
    fn PokenavList_PageDown() -> i32;
    fn PokenavList_PageUp() -> i32;
    fn PokenavList_ReshowListFromCheckPage();
    fn PokenavList_ToggleVerticalArrows(a0: u32);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut SpritePalette);
    fn PrintCheckPageInfo(a0: i16);
    fn PrintHelpBarText(a0: u32);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16;
    fn RunTextPrinters();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetLeftHeaderSpritesInvisibility();
    fn ShouldDrawRematchPokeballIcon(a0: i32) -> u32;
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn WaitForHelpBar() -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenMatchCall() -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN, 8264) as *mut Pokenav_MatchCallGfx;
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).unused19 = 0;
    (*gfx).loopTaskId = CreateLoopedTask(Some(LoopedTask_OpenMatchCall), 1);
    (*gfx).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMatchCallLoopedTask(index: i32) {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    (*gfx).loopTaskId = CreateLoopedTask(sMatchCallLoopTaskFuncs[index], 1);
    (*gfx).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMatchCallLoopedTaskActive() -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    return (*gfx).isTaskActiveCB.unwrap_unchecked()();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMatchCallSubstruct2() {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    FreeMatchCallSprites();
    DestroyMatchCallList();
    RemoveWindow((*gfx).infoBoxWindowId as u8);
    RemoveWindow((*gfx).locWindowId as u8);
    RemoveWindow((*gfx).msgBoxWindowId as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN);
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    return IsLoopedTaskActive((*gfx).loopTaskId);
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenMatchCall(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            InitBgTemplates(sMatchCallBgTemplates.as_ptr().cast_mut(), 3);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            DecompressAndCopyTileDataToVram(
                2,
                sMatchCallUI_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(2, (*gfx).bgTilemapBuffer2.as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                2,
                sMatchCallUI_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(2);
            CopyPaletteIntoBufferUnfaded(sMatchCallUI_Pal.as_ptr().cast_mut(), 32, 32);
            CopyBgTilemapBufferToVram(2);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            BgDmaFill(1, 0, 0, 1);
            SetBgTilemapBuffer(1, (*gfx).bgTilemapBuffer1.as_mut_ptr() as *mut c_void);
            FillBgTilemapBufferRect_Palette0(1, 0x1000, 0, 0, 32, 20);
            CopyPaletteIntoBufferUnfaded(sCallWindow_Pal.as_ptr().cast_mut(), 16, 32);
            CopyBgTilemapBufferToVram(1);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            LoadCallWindowAndFade(gfx);
            DecompressAndCopyTileDataToVram(
                3,
                sPokeball_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            CopyPaletteIntoBufferUnfaded(sListWindow_Pal.as_ptr().cast_mut(), 48, 32);
            CopyPaletteIntoBufferUnfaded(sPokeball_Pal.as_ptr().cast_mut(), 80, 32);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() != 0 || IsMatchCallListInitFinished() == 0 {
                return LT_PAUSE;
            }
            CreateMatchCallList();
            return LT_INC_AND_PAUSE;
        }
        4 => {
            if IsCreatePokenavListTaskActive() != 0 {
                return LT_PAUSE;
            }
            DrawMatchCallLeftColumnWindows(gfx);
            return LT_INC_AND_PAUSE;
        }
        5 => {
            UpdateMatchCallInfoBox(gfx);
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        6 => {
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ShowBg(2);
            ShowBg(3);
            ShowBg(1);
            AllocMatchCallSprites();
            LoadLeftHeaderGfxForIndex(3);
            ShowLeftHeaderGfx(POKENAV_GFX_MATCH_CALL_MENU, TRUE as u32, FALSE as u32);
            PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
            return LT_INC_AND_PAUSE;
        }
        7 => {
            if IsPaletteFadeActive() != 0 || AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            SetPokeballIconsFlashing(TRUE as u32);
            return LT_FINISH;
        }
        _ => {
            return LT_FINISH;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MatchCallListCursorDown(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => 'l2: {
            let sw1: i32 = PokenavList_MoveCursorDown();
            let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                break 'l2;
            }
            if sw1 == 1 {
                fall = true;
                PlaySE(SE_SELECT);
                return 7;
            }
            if sw1 == 2 {
                fall = true;
                PlaySE(SE_SELECT);
            }
            if fall || !matched {
                fall = true;
                return LT_INC_AND_PAUSE;
            }
        }
        1 => {
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn MatchCallListCursorUp(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => 'l2: {
            let sw1: i32 = PokenavList_MoveCursorUp();
            let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                break 'l2;
            }
            if sw1 == 1 {
                fall = true;
                PlaySE(SE_SELECT);
                return 7;
            }
            if sw1 == 2 {
                fall = true;
                PlaySE(SE_SELECT);
            }
            if fall || !matched {
                fall = true;
                return LT_INC_AND_PAUSE;
            }
        }
        1 => {
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn MatchCallListPageDown(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => 'l2: {
            let sw1: i32 = PokenavList_PageDown();
            let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                break 'l2;
            }
            if sw1 == 1 {
                fall = true;
                PlaySE(SE_SELECT);
                return 7;
            }
            if sw1 == 2 {
                fall = true;
                PlaySE(SE_SELECT);
            }
            if fall || !matched {
                fall = true;
                return LT_INC_AND_PAUSE;
            }
        }
        1 => {
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn MatchCallListPageUp(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => 'l2: {
            let sw1: i32 = PokenavList_PageUp();
            let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                break 'l2;
            }
            if sw1 == 1 {
                fall = true;
                PlaySE(SE_SELECT);
                return 7;
            }
            if sw1 == 2 {
                fall = true;
                PlaySE(SE_SELECT);
            }
            if fall || !matched {
                fall = true;
                return LT_INC_AND_PAUSE;
            }
        }
        1 => {
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintMatchCallLocation(gfx, 0);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn SelectMatchCallEntry(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PrintMatchCallSelectionOptions(gfx);
            PrintHelpBarText(HELPBAR_MC_CALL_MENU);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if ShowOptionsCursor(gfx) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn MoveMatchCallOptionsCursor(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx = null_mut();
    let mut cursorPos: u16 = 0;
    PlaySE(SE_SELECT);
    gfx = GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    cursorPos = GetMatchCallOptionCursorPos();
    UpdateCursorGfxPos(gfx, cursorPos as i32);
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn CancelMatchCallSelection(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            UpdateWindowsReturnToTrainerList(gfx);
            PrintHelpBarText(HELPBAR_MC_TRAINER_LIST);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy1(gfx) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn DoMatchCallMessage(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            PokenavList_ToggleVerticalArrows(TRUE as u32);
            DrawMsgBoxForMatchCallMsg(gfx);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy2(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintCallingDots(gfx);
            PlaySE(SE_POKENAV_CALL);
            (*gfx).skipHangUpSE = FALSE;
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if WaitForCallingDotsText(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallMessage(gfx);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if WaitForMatchCallMessageText(gfx) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn DoTrainerCloseByMessage(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            DrawMsgBoxForCloseByMsg(gfx);
            PokenavList_ToggleVerticalArrows(TRUE as u32);
            (*gfx).skipHangUpSE = TRUE;
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy2(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintTrainerIsCloseBy(gfx);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if WaitForTrainerIsCloseByText(gfx) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn CloseMatchCallMessage(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    let mut result: u32 = LT_INC_AND_PAUSE;
    match state {
        0 => {
            if (*gfx).skipHangUpSE == 0 {
                PlaySE(SE_POKENAV_HANG_UP);
            }
            PlaySE(SE_SELECT);
        }
        1 => {
            EraseCallMessageBox(gfx);
        }
        2 => {
            if WaitForCallMessageBoxErase(gfx) != 0 {
                result = LT_PAUSE;
            }
        }
        3 => {
            UpdateWindowsReturnToTrainerList(gfx);
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy1(gfx) != 0 {
                result = LT_PAUSE;
            }
            PrintHelpBarText(HELPBAR_MC_TRAINER_LIST);
        }
        5 => {
            if WaitForHelpBar() != 0 {
                result = LT_PAUSE;
            } else {
                if (*gfx).newRematchRequest != 0 {
                    PokenavList_DrawCurrentItemIcon();
                    result = LT_INC_AND_CONTINUE;
                } else {
                    PokenavList_ToggleVerticalArrows(FALSE as u32);
                    result = LT_FINISH;
                }
            }
        }
        6 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                result = LT_PAUSE;
            } else {
                PokenavList_ToggleVerticalArrows(FALSE as u32);
                result = LT_FINISH;
            }
        }
        _ => {}
    }
    return result;
}
pub(crate) unsafe extern "C" fn ShowCheckPage(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PokenavList_EraseListForCheckPage();
            UpdateWindowsToShowCheckPage(gfx);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if PokenavList_IsTaskActive() != 0 || IsDma3ManagerBusyWithBgCopy1(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintHelpBarText(HELPBAR_MC_CHECK_PAGE);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintCheckPageInfo(0);
            LoadCheckPageTrainerPic(gfx);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if PokenavList_IsTaskActive() != 0
                || WaitForTrainerPic(gfx) != 0
                || WaitForHelpBar() != 0
            {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn ShowCheckPageDown(state: i32) -> u32 {
    let mut topId: i32 = 0;
    let mut delta: i32 = 0;
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            topId = PokenavList_GetTopIndex() as i32;
            delta = GetIndexDeltaOfNextCheckPageDown(topId);
            if delta != 0 {
                PlaySE(SE_SELECT);
                (*gfx).pageDelta = delta as i16;
                TrainerPicSlideOffscreen(gfx);
                return LT_INC_AND_PAUSE;
            }
        }
        1 => {
            if WaitForTrainerPic(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallLocation(gfx, (*gfx).pageDelta as i32);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintCheckPageInfo((*gfx).pageDelta);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            LoadCheckPageTrainerPic(gfx);
            return LT_INC_AND_PAUSE;
        }
        4 => {
            if PokenavList_IsTaskActive() != 0 || WaitForTrainerPic(gfx) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn ExitCheckPage(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            TrainerPicSlideOffscreen(gfx);
            PokenavList_ReshowListFromCheckPage();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if PokenavList_IsTaskActive() != 0 || WaitForTrainerPic(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintHelpBarText(HELPBAR_MC_TRAINER_LIST);
            UpdateMatchCallInfoBox(gfx);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn ShowCheckPageUp(state: i32) -> u32 {
    let mut topId: i32 = 0;
    let mut delta: i32 = 0;
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    match state {
        0 => {
            topId = PokenavList_GetTopIndex() as i32;
            delta = GetIndexDeltaOfNextCheckPageUp(topId);
            if delta != 0 {
                PlaySE(SE_SELECT);
                (*gfx).pageDelta = delta as i16;
                TrainerPicSlideOffscreen(gfx);
                return LT_INC_AND_PAUSE;
            }
        }
        1 => {
            if WaitForTrainerPic(gfx) != 0 {
                return LT_PAUSE;
            }
            PrintMatchCallLocation(gfx, (*gfx).pageDelta as i32);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            PrintCheckPageInfo((*gfx).pageDelta);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            LoadCheckPageTrainerPic(gfx);
            return LT_INC_AND_PAUSE;
        }
        4 => {
            if PokenavList_IsTaskActive() != 0 || WaitForTrainerPic(gfx) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn ExitMatchCall(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            SetPokeballIconsFlashing(FALSE as u32);
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            SlideMenuHeaderDown();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsPaletteFadeActive() != 0 || MainMenuLoopedTaskIsBusy() != 0 {
                return LT_PAUSE;
            }
            SetLeftHeaderSpritesInvisibility();
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn CreateMatchCallList() {
    let mut template: PokenavListTemplate = zeroed();
    template.list = GetMatchCallList() as *mut PokenavListItem;
    template.count = GetNumberRegistered() as u16;
    template.itemSize = 4;
    template.startIndex = 0;
    template.item_X = 13;
    template.windowWidth = 16;
    template.listTop = 1;
    template.maxShowed = 8;
    template.fillValue = 3;
    template.fontId = FONT_NARROW;
    template.bufferItemFunc = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut PokenavMatchCallEntry, *mut u8)>,
        Option<unsafe extern "C" fn(*mut PokenavListItem, *mut u8)>,
    >(Some(BufferMatchCallNameAndDesc));
    template.iconDrawFunc = Some(TryDrawRematchPokeballIcon);
    CreatePokenavList(
        (&raw const sMatchCallBgTemplates[2]).cast_mut(),
        &raw mut template,
        2,
    );
    CreateTask(Some(Task_FlashPokeballIcons), 7);
}
pub(crate) unsafe extern "C" fn DestroyMatchCallList() {
    DestroyPokenavList();
    DestroyTask(FindTaskIdByFunc(Some(Task_FlashPokeballIcons)));
}
pub(crate) unsafe extern "C" fn SetPokeballIconsFlashing(active: u32) {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_FlashPokeballIcons));
    if taskId != TASK_NONE {
        gTasks[taskId].data[15] = active as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_FlashPokeballIcons(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(15) != 0 {
        *data += 4;
        *data &= 0x7F;
        *data.at(1) = gSineTable[*data] >> 4;
        PokenavCopyPalette(
            sPokeball_Pal.as_ptr().cast_mut(),
            (&raw const sPokeball_Pal[16]).cast_mut(),
            0x10,
            0x10,
            *data.at(1) as i32,
            &raw mut gPlttBufferUnfaded[80],
        );
        if gPaletteFade.active() == 0 {
            CpuSet(
                &raw mut gPlttBufferUnfaded[80] as *mut c_void,
                &raw mut gPlttBufferFaded[80] as *mut c_void,
                0x4000008,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryDrawRematchPokeballIcon(
    windowId: u16,
    rematchId: u32,
    tileOffset: u32,
) {
    let mut bg: u8 = GetWindowAttribute(windowId as u8, WINDOW_BG) as u8;
    let mut tilemap: *mut u16 = GetBgTilemapBuffer(bg) as *mut u16;
    tilemap = tilemap.at(tileOffset * 64 + 0x1D);
    if ShouldDrawRematchPokeballIcon(rematchId as i32) != 0 {
        *tilemap = POKEBALL_ICON_TOP;
        *tilemap.at(32) = POKEBALL_ICON_BOTTOM;
    } else {
        *tilemap = POKEBALL_ICON_EMPTY;
        *tilemap.at(32) = POKEBALL_ICON_EMPTY;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRematchPokeballIcon(windowId: u16, tileOffset: u32) {
    let mut bg: u8 = GetWindowAttribute(windowId as u8, WINDOW_BG) as u8;
    let mut tilemap: *mut u16 = GetBgTilemapBuffer(bg) as *mut u16;
    tilemap = tilemap.at(tileOffset * 64 + 0x1D);
    *tilemap = POKEBALL_ICON_EMPTY;
    *tilemap.at(32) = POKEBALL_ICON_EMPTY;
}
pub(crate) unsafe extern "C" fn DrawMatchCallLeftColumnWindows(gfx: *mut Pokenav_MatchCallGfx) {
    (*gfx).locWindowId = AddWindow((&raw const *sMatchCallLocationWindowTemplate).cast_mut());
    (*gfx).infoBoxWindowId = AddWindow((&raw const *sMatchCallInfoBoxWindowTemplate).cast_mut());
    FillWindowPixelBuffer((*gfx).locWindowId as u8, 17);
    PutWindowTilemap((*gfx).locWindowId as u8);
    FillWindowPixelBuffer((*gfx).infoBoxWindowId as u8, 17);
    PutWindowTilemap((*gfx).infoBoxWindowId as u8);
    CopyWindowToVram((*gfx).locWindowId as u8, COPYWIN_MAP);
}
pub(crate) unsafe extern "C" fn UpdateMatchCallInfoBox(gfx: *mut Pokenav_MatchCallGfx) {
    FillWindowPixelBuffer((*gfx).infoBoxWindowId as u8, 17);
    PrintNumberRegisteredLabel((*gfx).infoBoxWindowId);
    PrintNumberRegistered((*gfx).infoBoxWindowId);
    PrintNumberOfBattlesLabel((*gfx).infoBoxWindowId);
    PrintNumberOfBattles((*gfx).infoBoxWindowId);
    CopyWindowToVram((*gfx).infoBoxWindowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn PrintNumberRegisteredLabel(windowId: u16) {
    PrintMatchCallInfoLabel(windowId, gText_NumberRegistered.as_ptr().cast_mut(), 0);
}
pub(crate) unsafe extern "C" fn PrintNumberRegistered(windowId: u16) {
    let mut str: CArray<u8, 3> = zeroed();
    ConvertIntToDecimalStringN(
        str.as_mut_ptr(),
        GetNumberRegistered(),
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    PrintMatchCallInfoNumber(windowId, str.as_mut_ptr(), 1);
}
pub(crate) unsafe extern "C" fn PrintNumberOfBattlesLabel(windowId: u16) {
    PrintMatchCallInfoLabel(windowId, gText_NumberOfBattles.as_ptr().cast_mut(), 2);
}
pub(crate) unsafe extern "C" fn PrintNumberOfBattles(windowId: u16) {
    let mut str: CArray<u8, 5> = zeroed();
    let mut numTrainerBattles: i32 = GetGameStat(GAME_STAT_TRAINER_BATTLES) as i32;
    if numTrainerBattles > 99999 {
        numTrainerBattles = 99999;
    }
    ConvertIntToDecimalStringN(
        str.as_mut_ptr(),
        numTrainerBattles,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    PrintMatchCallInfoNumber(windowId, str.as_mut_ptr(), 3);
}
pub(crate) unsafe extern "C" fn PrintMatchCallInfoLabel(windowId: u16, str: *mut u8, top: i32) {
    let mut y: i32 = top * 16 + 1;
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NARROW,
        str,
        2,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn PrintMatchCallInfoNumber(windowId: u16, str: *mut u8, top: i32) {
    let mut x: i32 = GetStringRightAlignXOffset(FONT_NARROW as i32, str, 86);
    let mut y: i32 = top * 16 + 1;
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NARROW,
        str,
        x as u8,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn PrintMatchCallLocation(gfx: *mut Pokenav_MatchCallGfx, delta: i32) {
    let mut mapName: CArray<u8, 32> = zeroed();
    let mut x: i32 = 0;
    let mut index: i32 = PokenavList_GetSelectedIndex() as i32 + delta;
    let mut mapSec: i32 = GetMatchCallMapSec(index) as i32;
    if mapSec != MAPSEC_NONE as i32 {
        GetMapName(mapName.as_mut_ptr(), mapSec as u16, 0);
    } else {
        StringCopy(mapName.as_mut_ptr(), gText_Unknown.as_ptr().cast_mut());
    }
    x = GetStringCenterAlignXOffset(FONT_NARROW as i32, mapName.as_mut_ptr(), 88);
    FillWindowPixelBuffer((*gfx).locWindowId as u8, 17);
    AddTextPrinterParameterized(
        (*gfx).locWindowId as u8,
        FONT_NARROW,
        mapName.as_mut_ptr(),
        x as u8,
        1,
        0,
        None,
    );
}
pub(crate) unsafe extern "C" fn PrintMatchCallSelectionOptions(gfx: *mut Pokenav_MatchCallGfx) {
    let mut i: u32 = 0;
    FillWindowPixelBuffer((*gfx).infoBoxWindowId as u8, 17);
    i = 0;
    while i < MATCH_CALL_OPTION_COUNT {
        let mut optionText: i32 = GetMatchCallOptionId(i as i32) as i32;
        if optionText == MATCH_CALL_OPTION_COUNT as i32 {
            break;
        }
        AddTextPrinterParameterized(
            (*gfx).infoBoxWindowId as u8,
            FONT_NARROW,
            sMatchCallOptionTexts[optionText],
            16,
            i as u8 * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    CopyWindowToVram((*gfx).infoBoxWindowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn ShowOptionsCursor(gfx: *mut Pokenav_MatchCallGfx) -> u32 {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        CreateOptionsCursorSprite(gfx, GetMatchCallOptionCursorPos() as i32);
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn UpdateWindowsReturnToTrainerList(gfx: *mut Pokenav_MatchCallGfx) {
    CloseMatchCallSelectOptionsWindow(gfx);
    UpdateMatchCallInfoBox(gfx);
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy1(
    gfx: *mut Pokenav_MatchCallGfx,
) -> u32 {
    return IsDma3ManagerBusyWithBgCopy() as u32;
}
pub(crate) unsafe extern "C" fn UpdateWindowsToShowCheckPage(gfx: *mut Pokenav_MatchCallGfx) {
    CloseMatchCallSelectOptionsWindow(gfx);
    FillWindowPixelBuffer((*gfx).infoBoxWindowId as u8, 17);
    CopyWindowToVram((*gfx).infoBoxWindowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn LoadCallWindowAndFade(gfx: *mut Pokenav_MatchCallGfx) {
    (*gfx).msgBoxWindowId = AddWindow((&raw const *sCallMsgBoxWindowTemplate).cast_mut());
    LoadMatchCallWindowGfx((*gfx).msgBoxWindowId as u32, 1, 4);
    FadeToBlackExceptPrimary();
}
pub(crate) unsafe extern "C" fn DrawMsgBoxForMatchCallMsg(gfx: *mut Pokenav_MatchCallGfx) {
    let mut sprite: *mut Sprite = null_mut();
    LoadMatchCallWindowGfx((*gfx).msgBoxWindowId as u32, 1, 4);
    DrawMatchCallTextBoxBorder((*gfx).msgBoxWindowId as u32, 1, 4);
    FillWindowPixelBuffer((*gfx).msgBoxWindowId as u8, 17);
    PutWindowTilemap((*gfx).msgBoxWindowId as u8);
    CopyWindowToVram((*gfx).msgBoxWindowId as u8, COPYWIN_FULL);
    sprite = GetSpinningPokenavSprite();
    (*sprite).x = 24;
    (*sprite).y = 112;
    (*sprite).y2 = 0;
}
pub(crate) unsafe extern "C" fn DrawMsgBoxForCloseByMsg(gfx: *mut Pokenav_MatchCallGfx) {
    LoadUserWindowBorderGfx((*gfx).msgBoxWindowId as u8, 1, 64);
    DrawTextBorderOuter((*gfx).msgBoxWindowId as u8, 1, 4);
    FillWindowPixelBuffer((*gfx).msgBoxWindowId as u8, 17);
    PutWindowTilemap((*gfx).msgBoxWindowId as u8);
    CopyWindowToVram((*gfx).msgBoxWindowId as u8, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy2(
    gfx: *mut Pokenav_MatchCallGfx,
) -> u32 {
    return IsDma3ManagerBusyWithBgCopy() as u32;
}
pub(crate) unsafe extern "C" fn PrintCallingDots(gfx: *mut Pokenav_MatchCallGfx) {
    AddTextPrinterParameterized(
        (*gfx).msgBoxWindowId as u8,
        FONT_NORMAL,
        sText_CallingDots.as_ptr().cast_mut(),
        32,
        1,
        1,
        None,
    );
}
pub(crate) unsafe extern "C" fn WaitForCallingDotsText(gfx: *mut Pokenav_MatchCallGfx) -> u32 {
    RunTextPrinters();
    return IsTextPrinterActive((*gfx).msgBoxWindowId as u8) as u32;
}
pub(crate) unsafe extern "C" fn PrintTrainerIsCloseBy(gfx: *mut Pokenav_MatchCallGfx) {
    AddTextPrinterParameterized(
        (*gfx).msgBoxWindowId as u8,
        FONT_NORMAL,
        gText_TrainerCloseBy.as_ptr().cast_mut(),
        0,
        1,
        1,
        None,
    );
}
pub(crate) unsafe extern "C" fn WaitForTrainerIsCloseByText(gfx: *mut Pokenav_MatchCallGfx) -> u32 {
    RunTextPrinters();
    return IsTextPrinterActive((*gfx).msgBoxWindowId as u8) as u32;
}
pub(crate) unsafe extern "C" fn PrintMatchCallMessage(gfx: *mut Pokenav_MatchCallGfx) {
    let mut index: i32 = PokenavList_GetSelectedIndex() as i32;
    let mut str: *mut u8 = GetMatchCallMessageText(index, &raw mut (*gfx).newRematchRequest);
    let mut speed: u8 = GetPlayerTextSpeedDelay();
    AddTextPrinterParameterized(
        (*gfx).msgBoxWindowId as u8,
        FONT_NORMAL,
        str,
        32,
        1,
        speed,
        None,
    );
}
pub(crate) unsafe extern "C" fn WaitForMatchCallMessageText(gfx: *mut Pokenav_MatchCallGfx) -> u32 {
    if gMain.heldKeys as i32 & A_BUTTON != 0 {
        gTextFlags.set_canABSpeedUpPrint(TRUE);
    } else {
        gTextFlags.set_canABSpeedUpPrint(FALSE);
    }
    RunTextPrinters();
    return IsTextPrinterActive((*gfx).msgBoxWindowId as u8) as u32;
}
pub(crate) unsafe extern "C" fn EraseCallMessageBox(gfx: *mut Pokenav_MatchCallGfx) {
    HideSpinningPokenavSprite();
    FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 32, 20);
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn WaitForCallMessageBoxErase(gfx: *mut Pokenav_MatchCallGfx) -> u32 {
    return IsDma3ManagerBusyWithBgCopy() as u32;
}
pub(crate) unsafe extern "C" fn AllocMatchCallSprites() {
    let mut i: i32 = 0;
    let mut paletteNum: u8 = 0;
    let mut spriteSheet: SpriteSheet = zeroed();
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    i = 0;
    while i < 1 {
        LoadCompressedSpriteSheet((&raw const sOptionsCursorSpriteSheets[i]).cast_mut());
        i += 1;
    }
    Pokenav_AllocAndLoadPalettes(sOptionsCursorSpritePalettes.as_ptr().cast_mut());
    (*gfx).optionsCursorSprite = null_mut();
    spriteSheet.data = (*gfx).trainerPicGfx.as_mut_ptr() as *mut c_void;
    spriteSheet.size = 2048;
    spriteSheet.tag = GFXTAG_TRAINER_PIC;
    (*gfx).trainerPicGfxPtr =
        (OBJ_VRAM0 as usize as *mut u8).at(LoadSpriteSheet(&raw mut spriteSheet) as i32 * 0x20);
    paletteNum = AllocSpritePalette(PALTAG_TRAINER_PIC);
    (*gfx).trainerPicPalOffset = 0x100 + paletteNum as u16 * 16;
    (*gfx).trainerPicSprite = CreateTrainerPicSprite();
    (*(*gfx).trainerPicSprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn FreeMatchCallSprites() {
    let mut gfx: *mut Pokenav_MatchCallGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_OPEN) as *mut Pokenav_MatchCallGfx;
    if !(*gfx).optionsCursorSprite.is_null() {
        DestroySprite((*gfx).optionsCursorSprite);
    }
    if !(*gfx).trainerPicSprite.is_null() {
        DestroySprite((*gfx).trainerPicSprite);
    }
    FreeSpriteTilesByTag(GFXTAG_TRAINER_PIC);
    FreeSpriteTilesByTag(GFXTAG_CURSOR);
    FreeSpritePaletteByTag(PALTAG_CURSOR);
    FreeSpritePaletteByTag(PALTAG_TRAINER_PIC);
}
pub(crate) unsafe extern "C" fn CreateOptionsCursorSprite(
    gfx: *mut Pokenav_MatchCallGfx,
    top: i32,
) {
    if (*gfx).optionsCursorSprite.is_null() {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sOptionsCursorSpriteTemplate).cast_mut(),
            4,
            80,
            5,
        );
        (*gfx).optionsCursorSprite = &raw mut gSprites[spriteId];
        UpdateCursorGfxPos(gfx, top);
    }
}
pub(crate) unsafe extern "C" fn CloseMatchCallSelectOptionsWindow(gfx: *mut Pokenav_MatchCallGfx) {
    DestroySprite((*gfx).optionsCursorSprite);
    (*gfx).optionsCursorSprite = null_mut();
}
pub(crate) unsafe extern "C" fn UpdateCursorGfxPos(gfx: *mut Pokenav_MatchCallGfx, top: i32) {
    (*(*gfx).optionsCursorSprite).y2 = top as i16 * 16;
}
pub(crate) unsafe extern "C" fn SpriteCB_OptionsCursor(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 3
    {
        (*sprite).data[0] = 0;
        (*sprite).x2 = (*sprite).x2 + 1 & 7;
    }
}
pub(crate) unsafe extern "C" fn CreateTrainerPicSprite() -> *mut Sprite {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sTrainerPicSpriteTemplate).cast_mut(),
        44,
        104,
        6,
    );
    return &raw mut gSprites[spriteId];
}
pub(crate) unsafe extern "C" fn LoadCheckPageTrainerPic(gfx: *mut Pokenav_MatchCallGfx) {
    let mut cursor: u16 = 0;
    let mut trainerPic: i32 = GetMatchCallTrainerPic(PokenavList_GetSelectedIndex() as i32);
    if trainerPic >= 0 {
        DecompressPicFromTable(
            (&raw const gTrainerFrontPicTable[trainerPic]).cast_mut(),
            (*gfx).trainerPicGfx.as_mut_ptr() as *mut c_void,
            SPECIES_NONE as i32,
        );
        LZ77UnCompWram(
            gTrainerFrontPicPaletteTable[trainerPic].data,
            (*gfx).trainerPicPal.as_mut_ptr() as *mut c_void,
        );
        cursor = RequestDma3Copy(
            (*gfx).trainerPicGfx.as_mut_ptr() as *mut c_void,
            (*gfx).trainerPicGfxPtr as *mut c_void,
            2048,
            1,
        ) as u16;
        LoadPalette(
            (*gfx).trainerPicPal.as_mut_ptr() as *mut c_void,
            (*gfx).trainerPicPalOffset,
            32,
        );
        (*(*gfx).trainerPicSprite).data[0] = 0;
        (*(*gfx).trainerPicSprite).data[7] = cursor as i16;
        (*(*gfx).trainerPicSprite).callback = Some(SpriteCB_TrainerPicSlideOnscreen);
    }
}
pub(crate) unsafe extern "C" fn TrainerPicSlideOffscreen(gfx: *mut Pokenav_MatchCallGfx) {
    (*(*gfx).trainerPicSprite).callback = Some(SpriteCB_TrainerPicSlideOffscreen);
}
pub(crate) unsafe extern "C" fn WaitForTrainerPic(gfx: *mut Pokenav_MatchCallGfx) -> u32 {
    return ((*(*gfx).trainerPicSprite).callback
        != Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))) as u32;
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerPicSlideOnscreen(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if CheckForSpaceForDma3Request((*sprite).data[7]) != -1 {
                (*sprite).x2 = -80;
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            (*sprite).x2 += 8;
            if (*sprite).x2 >= 0 {
                (*sprite).x2 = 0;
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerPicSlideOffscreen(sprite: *mut Sprite) {
    (*sprite).x2 -= 8;
    if (*sprite).x2 <= -80 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
