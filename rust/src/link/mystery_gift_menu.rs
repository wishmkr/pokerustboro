//! Translated from `src/mystery_gift_menu.c` by tools/rustport/c2rs.py.
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

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ereader_screen::CreateEReaderTask;
use crate::ffi::gSpecialVar_Result;
use crate::gpu_regs::{ClearGpuRegBits, EnableInterrupts, SetGpuReg};
use crate::international_string_util::GetStringRightAlignXOffset;
use crate::link::{CloseLink, gLinkPlayers, gReceivedRemoteLinkPlayers};
use crate::link_rfu_2::Rfu_SetCloseLinkCallback;
use crate::link_rfu_3::DestroyWirelessStatusIndicatorSprite;
use crate::list_menu::DoMysteryGiftListMenu;
use crate::menu::{
    AddTextPrinterParameterized4, CreateYesNoMenu, DecompressAndLoadBgGfxUsingHeap,
    Menu_LoadStdPalAt, Menu_ProcessInputNoWrapClearOnChoose,
};
use crate::mystery_gift::{
    ClearSavedWonderCardAndRelated, ClearSavedWonderNewsAndRelated, GetSavedWonderCard,
    GetSavedWonderCardMetadata, GetSavedWonderNews, IsSavedWonderCardGiftNotReceived,
    IsSendingSavedWonderCardAllowed, IsSendingSavedWonderNewsAllowed, ValidateSavedWonderCard,
    ValidateSavedWonderNews,
};
use crate::mystery_gift_client::{
    MysteryGiftClient_AdvanceState, MysteryGiftClient_Create, MysteryGiftClient_Run,
    MysteryGiftClient_SetParam,
};
use crate::mystery_gift_server::{
    MysterGiftServer_CreateForCard, MysterGiftServer_CreateForNews, MysterGiftServer_Run,
};
use crate::mystery_gift_view::{
    WonderCard_Destroy, WonderCard_Enter, WonderCard_Exit, WonderCard_Init,
    WonderNews_AddScrollIndicatorArrowPair, WonderNews_Destroy, WonderNews_Enter, WonderNews_Exit,
    WonderNews_GetInput, WonderNews_Init, WonderNews_RemoveScrollIndicatorArrowPair,
};
use crate::palette::{LoadPalette, ResetPaletteFade, TransferPlttBuffer};
use crate::save::TrySavingData;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::{IsFanfareTaskInactive, PlayBGM, PlayFanfare};
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{StringCopy, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::text::{DeactivateAllTextPrinters, DrawDownArrow, RunTextPrinters};
use crate::text_window::{
    DrawTextBorderOuter, LoadUserWindowBorderGfx, LoadUserWindowBorderGfx_, rbox_fill_rectangle,
};
use crate::title_screen::CB2_InitTitleScreen;
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::{
    CreateTask_LinkMysteryGiftOverWireless, CreateTask_LinkMysteryGiftWithFriend,
    CreateTask_SendMysteryGift,
};
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers,
    PutWindowTilemap, RemoveWindow,
};
use crate::wonder_news::WonderNews_SetReward;
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
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
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
/// `Intl_GetListMenuWidth` with this module's view of its types.
#[inline]
unsafe fn Intl_GetListMenuWidth(a0: *mut ListMenuTemplate) -> i32 {
    unsafe { crate::international_string_util::Intl_GetListMenuWidth(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
// Data tables (translate with cdata.py): sTextboxBorder_Pal sTextboxBorder_Gfx sBGTemplates sMainWindows sWindowTemplate_YesNoMsg_Wide sWindowTemplate_YesNoMsg sWindowTemplate_GiftSelect sWindowTemplate_ThreeOptions sWindowTemplate_YesNoBox sWindowTemplate_GiftSelect_3Options sWindowTemplate_GiftSelect_2Options sWindowTemplate_GiftSelect_1Option sListMenuItems_CardsOrNews sListMenuItems_WirelessOrFriend sListMenuTemplate_ThreeOptions sListMenuItems_ReceiveSendToss sListMenuItems_ReceiveToss sListMenuItems_ReceiveSend sListMenuItems_Receive sListMenu_ReceiveSendToss sListMenu_ReceiveToss sListMenu_ReceiveSend sListMenu_Receive sUnusedMenuTexts sTextColors_Header sTextColors_Header_Copy sMG_Ereader_TextColor_2

/// `struct MysteryGiftTaskData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MysteryGiftTaskData {
    pub var: u16,
    pub unused1: u16,
    pub unused2: u16,
    pub unused3: u16,
    pub state: u8,
    pub textState: u8,
    pub unused4: u8,
    pub unused5: u8,
    pub isWonderNews: u8,
    pub sourceIsFriend: u8,
    pub msgId: u8,
    pub clientMsg: *mut u8,
}

unsafe impl Sync for MysteryGiftTaskData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<MysteryGiftTaskData>() == 20);
    assert!(offset_of!(MysteryGiftTaskData, var) == 0);
    assert!(offset_of!(MysteryGiftTaskData, unused1) == 2);
    assert!(offset_of!(MysteryGiftTaskData, unused2) == 4);
    assert!(offset_of!(MysteryGiftTaskData, unused3) == 6);
    assert!(offset_of!(MysteryGiftTaskData, state) == 8);
    assert!(offset_of!(MysteryGiftTaskData, textState) == 9);
    assert!(offset_of!(MysteryGiftTaskData, unused4) == 10);
    assert!(offset_of!(MysteryGiftTaskData, unused5) == 11);
    assert!(offset_of!(MysteryGiftTaskData, isWonderNews) == 12);
    assert!(offset_of!(MysteryGiftTaskData, sourceIsFriend) == 13);
    assert!(offset_of!(MysteryGiftTaskData, msgId) == 14);
    assert!(offset_of!(MysteryGiftTaskData, clientMsg) == 16);
};

const DOWN_ARROW_X: u16 = 208;
const DOWN_ARROW_Y: u16 = 20;
const LIST_MENU_TILE_NUM: u16 = 10;
const MG_STATE_ASK_TOSS: u8 = 22;
const MG_STATE_ASK_TOSS_UNRECEIVED: u8 = 23;
const MG_STATE_CLIENT_ASK_TOSS: u8 = 11;
const MG_STATE_CLIENT_ASK_TOSS_UNRECEIVED: u8 = 12;
const MG_STATE_CLIENT_COMMUNICATING: u8 = 7;
const MG_STATE_CLIENT_COMM_COMPLETED: u8 = 14;
const MG_STATE_CLIENT_ERROR: u8 = 16;
const MG_STATE_CLIENT_LINK: u8 = 8;
const MG_STATE_CLIENT_LINK_END: u8 = 13;
const MG_STATE_CLIENT_LINK_START: u8 = 5;
const MG_STATE_CLIENT_LINK_WAIT: u8 = 6;
const MG_STATE_CLIENT_MESSAGE: u8 = 10;
const MG_STATE_CLIENT_RESULT_MSG: u8 = 15;
const MG_STATE_CLIENT_YES_NO: u8 = 9;
const MG_STATE_DONT_HAVE_ANY: u8 = 2;
const MG_STATE_EXIT: u8 = 37;
const MG_STATE_GIFT_INPUT_EXIT: u8 = 27;
const MG_STATE_HANDLE_GIFT_INPUT: u8 = 20;
const MG_STATE_HANDLE_GIFT_SELECT: u8 = 21;
const MG_STATE_LOAD_GIFT: u8 = 18;
const MG_STATE_MAIN_MENU: u8 = 1;
const MG_STATE_RECEIVE: u8 = 28;
const MG_STATE_SAVE_LOAD_GIFT: u8 = 17;
const MG_STATE_SEND: u8 = 29;
const MG_STATE_SERVER_ERROR: u8 = 36;
const MG_STATE_SERVER_LINK: u8 = 32;
const MG_STATE_SERVER_LINK_END: u8 = 33;
const MG_STATE_SERVER_LINK_END_WAIT: u8 = 34;
const MG_STATE_SERVER_LINK_START: u8 = 31;
const MG_STATE_SERVER_LINK_WAIT: u8 = 30;
const MG_STATE_SERVER_RESULT_MSG: u8 = 35;
const MG_STATE_SOURCE_PROMPT: u8 = 3;
const MG_STATE_SOURCE_PROMPT_INPUT: u8 = 4;
const MG_STATE_TOSS: u8 = 24;
const MG_STATE_TOSSED: u8 = 26;
const MG_STATE_TOSS_SAVE: u8 = 25;
const MG_STATE_TO_MAIN_MENU: u8 = 0;
const WIN_HEADER: u8 = 0;
const WIN_MSG: u8 = 1;
const WIN_UNK: u8 = 2;

static sBGTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::mystery_gift_menu::sBGTemplates).cast());
static sListMenuItems_CardsOrNews: Table<CArray<ListMenuItem, 3>> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenuItems_CardsOrNews).cast());
static sListMenuItems_WirelessOrFriend: Table<CArray<ListMenuItem, 3>> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenuItems_WirelessOrFriend).cast());
static sListMenuTemplate_ThreeOptions: Table<ListMenuTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenuTemplate_ThreeOptions).cast());
static sListMenu_Receive: Table<ListMenuTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenu_Receive).cast());
static sListMenu_ReceiveSend: Table<ListMenuTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenu_ReceiveSend).cast());
static sListMenu_ReceiveSendToss: Table<ListMenuTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenu_ReceiveSendToss).cast());
static sListMenu_ReceiveToss: Table<ListMenuTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sListMenu_ReceiveToss).cast());
static sMG_Ereader_TextColor_2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::mystery_gift_menu::sMG_Ereader_TextColor_2).cast());
static sMainWindows: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::mystery_gift_menu::sMainWindows).cast());
static sTextColors_Header: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::mystery_gift_menu::sTextColors_Header).cast());
static sTextboxBorder_Gfx: Table<CArray<u32, 12>> =
    Table((&raw const crate::data::mystery_gift_menu::sTextboxBorder_Gfx).cast());
static sTextboxBorder_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::mystery_gift_menu::sTextboxBorder_Pal).cast());
static sWindowTemplate_GiftSelect: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_GiftSelect).cast());
static sWindowTemplate_GiftSelect_1Option: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_GiftSelect_1Option).cast());
static sWindowTemplate_GiftSelect_2Options: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_GiftSelect_2Options).cast());
static sWindowTemplate_GiftSelect_3Options: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_GiftSelect_3Options).cast());
static sWindowTemplate_ThreeOptions: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_ThreeOptions).cast());
static sWindowTemplate_YesNoBox: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_YesNoBox).cast());
static sWindowTemplate_YesNoMsg: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_YesNoMsg).cast());
static sWindowTemplate_YesNoMsg_Wide: Table<WindowTemplate> =
    Table((&raw const crate::data::mystery_gift_menu::sWindowTemplate_YesNoMsg_Wide).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDownArrowCounterAndYCoordIdx: Aligned<CArray<u8, 8>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gGiftIsFromEReader: u8 = 0;

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
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `GetTextWindowPalette` with this module's view of its types.
#[inline]
unsafe fn GetTextWindowPalette(a0: u8) -> *mut u16 {
    unsafe { crate::text_window::GetTextWindowPalette(a0) as *mut u16 }
}
/// `MysteryGiftClient_GetMsg` with this module's view of its types.
#[inline]
unsafe fn MysteryGiftClient_GetMsg() -> *mut c_void {
    unsafe { crate::mystery_gift_client::MysteryGiftClient_GetMsg() as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn VBlankCB_MysteryGiftEReader() {
    ProcessSpriteCopyRequests();
    LoadOam();
    TransferPlttBuffer();
}
pub unsafe fn CB2_MysteryGiftEReader() {
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
}
unsafe fn HandleMysteryGiftOrEReaderSetup(isEReader: i32) -> u32 {
    match gMain.state {
        0 => {
            SetVBlankCallback(None);
            ResetPaletteFade();
            ResetSpriteData();
            FreeAllSpritePalettes();
            ResetTasks();
            ScanlineEffect_Stop();
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBGTemplates.as_ptr().cast_mut(), 4);
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            SetBgTilemapBuffer(3, Alloc(BG_SCREEN_SIZE));
            SetBgTilemapBuffer(2, Alloc(BG_SCREEN_SIZE));
            SetBgTilemapBuffer(1, Alloc(BG_SCREEN_SIZE));
            SetBgTilemapBuffer(0, Alloc(BG_SCREEN_SIZE));
            LoadMysteryGiftTextboxBorder(3);
            InitWindows(sMainWindows.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            ClearGpuRegBits(REG_OFFSET_DISPCNT, 24576);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            gMain.state += 1;
        }
        1 => {
            LoadPalette(sTextboxBorder_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
            LoadPalette(GetTextWindowPalette(2) as *mut c_void, 208, 32);
            Menu_LoadStdPalAt(192);
            LoadUserWindowBorderGfx(0, 0xA, 224);
            LoadUserWindowBorderGfx_(0, 0x1, 240);
            FillBgTilemapBufferRect(0, 0x000, 0, 0, 32, 32, 17);
            FillBgTilemapBufferRect(1, 0x000, 0, 0, 32, 32, 17);
            FillBgTilemapBufferRect(2, 0x000, 0, 0, 32, 32, 17);
            MG_DrawCheckerboardPattern(3);
            PrintMysteryGiftOrEReaderHeader(isEReader as u8, FALSE as u32);
            gMain.state += 1;
        }
        2 => {
            CopyBgTilemapBufferToVram(3);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(0);
            gMain.state += 1;
        }
        3 => {
            ShowBg(0);
            ShowBg(3);
            PlayBGM(MUS_RG_MYSTERY_GIFT);
            SetVBlankCallback(Some(VBlankCB_MysteryGiftEReader));
            EnableInterrupts(197);
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
pub unsafe fn CB2_InitMysteryGift() {
    if HandleMysteryGiftOrEReaderSetup(FALSE as i32) != 0 {
        SetMainCallback2(Some(CB2_MysteryGiftEReader));
        gGiftIsFromEReader = FALSE;
        CreateMysteryGiftTask();
    }
    RunTasks();
}
pub unsafe fn CB2_InitEReader() {
    if HandleMysteryGiftOrEReaderSetup(TRUE as i32) != 0 {
        SetMainCallback2(Some(CB2_MysteryGiftEReader));
        gGiftIsFromEReader = TRUE;
        CreateEReaderTask();
    }
}
pub unsafe fn MainCB_FreeAllBuffersAndReturnToInitTitleScreen() {
    gGiftIsFromEReader = FALSE;
    FreeAllWindowBuffers();
    Free(GetBgTilemapBuffer(0));
    Free(GetBgTilemapBuffer(1));
    Free(GetBgTilemapBuffer(2));
    Free(GetBgTilemapBuffer(3));
    SetMainCallback2(Some(CB2_InitTitleScreen));
}
pub unsafe fn PrintMysteryGiftOrEReaderHeader(isEReader: u8, useCancel: u32) {
    let mut title: *mut u8 = null_mut();
    let mut options: *mut u8 = null_mut();
    FillWindowPixelBuffer(WIN_HEADER, 0);
    if isEReader == 0 {
        title = (*(&raw const crate::data::strings::gText_MysteryGift).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        options = if useCancel == 0 {
            (*(&raw const crate::data::strings::gText_PickOKExit).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        } else {
            (*(&raw const crate::data::strings::gText_PickOKCancel).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        };
    } else {
        title = (*(&raw const crate::data::strings::gJPText_MysteryGift).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        options = (*(&raw const crate::data::strings::gJPText_DecideStop).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    AddTextPrinterParameterized4(
        WIN_HEADER,
        FONT_NORMAL,
        4,
        1,
        0,
        0,
        sTextColors_Header.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        title,
    );
    AddTextPrinterParameterized4(
        WIN_HEADER,
        FONT_SMALL,
        GetStringRightAlignXOffset(FONT_SMALL as i32, options, 0xDE) as u8,
        1,
        0,
        0,
        sTextColors_Header.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        options,
    );
    CopyWindowToVram(WIN_HEADER, COPYWIN_GFX);
    PutWindowTilemap(WIN_HEADER);
}
pub unsafe fn MG_DrawTextBorder(windowId: u8) {
    DrawTextBorderOuter(windowId, 0x01, 0xF);
}
pub unsafe fn MG_DrawCheckerboardPattern(bg: u32) {
    FillBgTilemapBufferRect(bg as u8, 0x003, 0, 0, 32, 2, 17);
    for i in 0..18i32 {
        for j in 0..32i32 {
            if i & 1 != j & 1 {
                FillBgTilemapBufferRect(bg as u8, 1, j as u8, i as u8 + 2, 1, 1, 17);
            } else {
                FillBgTilemapBufferRect(bg as u8, 2, j as u8, i as u8 + 2, 1, 1, 17);
            }
        }
    }
}
unsafe fn ClearScreenInBg0(ignoreTopTwoRows: u32) {
    match ignoreTopTwoRows {
        0 => {
            FillBgTilemapBufferRect(0, 0, 0, 0, 32, 32, 17);
        }
        1 => {
            FillBgTilemapBufferRect(0, 0, 0, 2, 32, 30, 17);
        }
        _ => {}
    }
    CopyBgTilemapBufferToVram(0);
}
pub unsafe fn MG_AddMessageTextPrinter(str: *mut u8) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str);
    FillWindowPixelBuffer(WIN_MSG, 0x11);
    AddTextPrinterParameterized4(
        WIN_MSG,
        FONT_NORMAL,
        0,
        1,
        0,
        0,
        sMG_Ereader_TextColor_2.as_ptr().cast_mut(),
        0,
        gStringVar4.as_mut_ptr(),
    );
    DrawTextBorderOuter(WIN_MSG, 0x001, 0xF);
    PutWindowTilemap(WIN_MSG);
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
}
unsafe fn ClearMessage() {
    rbox_fill_rectangle(WIN_MSG);
    ClearWindowTilemap(WIN_MSG);
    CopyWindowToVram(WIN_MSG, COPYWIN_MAP);
}
pub unsafe fn PrintMysteryGiftMenuMessage(textState: *mut u8, str: *mut u8) -> u32 {
    match *textState {
        0 => {
            MG_AddMessageTextPrinter(str);
            *textState += 1;
        }
        1 => {
            DrawDownArrow(
                WIN_MSG,
                DOWN_ARROW_X,
                DOWN_ARROW_Y,
                1,
                0,
                &raw mut sDownArrowCounterAndYCoordIdx[0],
                &raw mut sDownArrowCounterAndYCoordIdx[1],
            );
            if gMain.newKeys as i32 & 3 != 0 {
                *textState += 1;
            }
        }
        2 => {
            DrawDownArrow(
                WIN_MSG,
                DOWN_ARROW_X,
                DOWN_ARROW_Y,
                1,
                1,
                &raw mut sDownArrowCounterAndYCoordIdx[0],
                &raw mut sDownArrowCounterAndYCoordIdx[1],
            );
            *textState = 0;
            ClearMessage();
            return TRUE as u32;
        }
        255 => {
            *textState = 2;
            return FALSE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn HideDownArrow() {
    DrawDownArrow(
        WIN_MSG,
        DOWN_ARROW_X,
        DOWN_ARROW_Y,
        1,
        0,
        &raw mut sDownArrowCounterAndYCoordIdx[0],
        &raw mut sDownArrowCounterAndYCoordIdx[1],
    );
}
unsafe fn ShowDownArrow() {
    DrawDownArrow(
        WIN_MSG,
        DOWN_ARROW_X,
        DOWN_ARROW_Y,
        1,
        1,
        &raw mut sDownArrowCounterAndYCoordIdx[0],
        &raw mut sDownArrowCounterAndYCoordIdx[1],
    );
}
unsafe fn HideDownArrowAndWaitButton(textState: *mut u8) -> u32 {
    match *textState {
        0 => {
            HideDownArrow();
            if gMain.newKeys as i32 & 3 != 0 {
                *textState += 1;
            }
        }
        1 => {
            ShowDownArrow();
            *textState = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn PrintStringAndWait2Seconds(counter: *mut u8, str: *mut u8) -> u32 {
    if *counter == 0 {
        MG_AddMessageTextPrinter(str);
    }
    if ({
        *counter += 1;
        *counter
    }) > 120
    {
        *counter = 0;
        ClearMessage();
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn MysteryGift_HandleThreeOptionMenu(
    unused0: *mut u8,
    unused1: *mut u16,
    whichMenu: u8,
) -> u32 {
    let mut listMenuTemplate: ListMenuTemplate = *sListMenuTemplate_ThreeOptions;
    let mut windowTemplate: WindowTemplate = *sWindowTemplate_ThreeOptions;
    if whichMenu == 0 {
        listMenuTemplate.items = sListMenuItems_CardsOrNews.as_ptr().cast_mut();
    } else {
        listMenuTemplate.items = sListMenuItems_WirelessOrFriend.as_ptr().cast_mut();
    }
    let mut width: i32 = Intl_GetListMenuWidth(&raw mut listMenuTemplate);
    if width & 1 != 0 {
        width += 1;
    }
    windowTemplate.width = width as u8;
    if width < DISPLAY_TILE_WIDTH as i32 {
        windowTemplate.tilemapLeft = ((DISPLAY_TILE_WIDTH as i32 - width) / 2) as u8;
    } else {
        windowTemplate.tilemapLeft = 0;
    }
    let response: i32 = DoMysteryGiftListMenu(
        &raw mut windowTemplate,
        &raw mut listMenuTemplate,
        1,
        LIST_MENU_TILE_NUM,
        224,
    );
    if response != LIST_NOTHING_CHOSEN {
        ClearWindowTilemap(WIN_UNK);
        CopyWindowToVram(WIN_UNK, COPYWIN_MAP);
    }
    response as u32
}
pub unsafe fn DoMysteryGiftYesNo(
    textState: *mut u8,
    windowId: *mut u16,
    yesNoBoxPlacement: u8,
    str: *mut u8,
) -> i8 {
    let mut windowTemplate: WindowTemplate = zeroed();
    let mut input: i8 = 0;
    match *textState {
        0 => {
            StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str);
            if yesNoBoxPlacement == 0 {
                *windowId = AddWindow((&raw const *sWindowTemplate_YesNoMsg_Wide).cast_mut());
            } else {
                *windowId = AddWindow((&raw const *sWindowTemplate_YesNoMsg).cast_mut());
            }
            FillWindowPixelBuffer(*windowId as u8, 0x11);
            AddTextPrinterParameterized4(
                *windowId as u8,
                FONT_NORMAL,
                0,
                1,
                0,
                0,
                sMG_Ereader_TextColor_2.as_ptr().cast_mut(),
                0,
                gStringVar4.as_mut_ptr(),
            );
            DrawTextBorderOuter(*windowId as u8, 0x001, 0x0F);
            CopyWindowToVram(*windowId as u8, COPYWIN_GFX);
            PutWindowTilemap(*windowId as u8);
            *textState += 1;
        }
        1 => {
            windowTemplate = *sWindowTemplate_YesNoBox;
            if yesNoBoxPlacement == 0 {
                windowTemplate.tilemapTop = 9;
            } else {
                windowTemplate.tilemapTop = 15;
            }
            CreateYesNoMenu(&raw mut windowTemplate, 10, 14, 0);
            *textState += 1;
        }
        2 => {
            input = Menu_ProcessInputNoWrapClearOnChoose();
            if input == MENU_B_PRESSED || input == 0 || input == 1 {
                *textState = 0;
                rbox_fill_rectangle(*windowId as u8);
                ClearWindowTilemap(*windowId as u8);
                CopyWindowToVram(*windowId as u8, COPYWIN_MAP);
                RemoveWindow(*windowId as u8);
                return input;
            }
        }
        255 => {
            *textState = 0;
            rbox_fill_rectangle(*windowId as u8);
            ClearWindowTilemap(*windowId as u8);
            CopyWindowToVram(*windowId as u8, COPYWIN_MAP);
            RemoveWindow(*windowId as u8);
            return MENU_B_PRESSED;
        }
        _ => {}
    }
    MENU_NOTHING_CHOSEN
}
unsafe fn HandleGiftSelectMenu(
    textState: *mut u8,
    windowId: *mut u16,
    cannotToss: u32,
    cannotSend: u32,
) -> i32 {
    let mut windowTemplate: WindowTemplate = zeroed();
    let mut input: i32 = 0;
    match *textState {
        0 => {
            if cannotToss == 0 {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_WhatToDoWithCards)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            } else {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_WhatToDoWithNews)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
            *windowId = AddWindow((&raw const *sWindowTemplate_GiftSelect).cast_mut());
            FillWindowPixelBuffer(*windowId as u8, 0x11);
            AddTextPrinterParameterized4(
                *windowId as u8,
                FONT_NORMAL,
                0,
                1,
                0,
                0,
                sMG_Ereader_TextColor_2.as_ptr().cast_mut(),
                0,
                gStringVar4.as_mut_ptr(),
            );
            DrawTextBorderOuter(*windowId as u8, 0x001, 0x0F);
            CopyWindowToVram(*windowId as u8, COPYWIN_GFX);
            PutWindowTilemap(*windowId as u8);
            *textState += 1;
        }
        1 => {
            windowTemplate = *sWindowTemplate_YesNoBox;
            if cannotSend != 0 {
                if cannotToss == 0 {
                    input = DoMysteryGiftListMenu(
                        (&raw const *sWindowTemplate_GiftSelect_2Options).cast_mut(),
                        (&raw const *sListMenu_ReceiveToss).cast_mut(),
                        1,
                        LIST_MENU_TILE_NUM,
                        224,
                    );
                } else {
                    input = DoMysteryGiftListMenu(
                        (&raw const *sWindowTemplate_GiftSelect_1Option).cast_mut(),
                        (&raw const *sListMenu_Receive).cast_mut(),
                        1,
                        LIST_MENU_TILE_NUM,
                        224,
                    );
                }
            } else {
                if cannotToss == 0 {
                    input = DoMysteryGiftListMenu(
                        (&raw const *sWindowTemplate_GiftSelect_3Options).cast_mut(),
                        (&raw const *sListMenu_ReceiveSendToss).cast_mut(),
                        1,
                        LIST_MENU_TILE_NUM,
                        224,
                    );
                } else {
                    input = DoMysteryGiftListMenu(
                        (&raw const *sWindowTemplate_GiftSelect_2Options).cast_mut(),
                        (&raw const *sListMenu_ReceiveSend).cast_mut(),
                        1,
                        LIST_MENU_TILE_NUM,
                        224,
                    );
                }
            }
            if input != LIST_NOTHING_CHOSEN {
                *textState = 0;
                rbox_fill_rectangle(*windowId as u8);
                ClearWindowTilemap(*windowId as u8);
                CopyWindowToVram(*windowId as u8, COPYWIN_MAP);
                RemoveWindow(*windowId as u8);
                return input;
            }
        }
        255 => {
            *textState = 0;
            rbox_fill_rectangle(*windowId as u8);
            ClearWindowTilemap(*windowId as u8);
            CopyWindowToVram(*windowId as u8, COPYWIN_MAP);
            RemoveWindow(*windowId as u8);
            return LIST_CANCEL;
        }
        _ => {}
    }
    LIST_NOTHING_CHOSEN
}
unsafe fn ValidateCardOrNews(isWonderNews: u32) -> u32 {
    if isWonderNews == 0 {
        return ValidateSavedWonderCard();
    } else {
        return ValidateSavedWonderNews();
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn HandleLoadWonderCardOrNews(state: *mut u8, isWonderNews: u32) -> u32 {
    match *state {
        0 => {
            if isWonderNews == 0 {
                WonderCard_Init(GetSavedWonderCard(), GetSavedWonderCardMetadata());
            } else {
                WonderNews_Init(GetSavedWonderNews());
            }
            *state += 1;
        }
        1 => {
            if isWonderNews == 0 {
                if WonderCard_Enter() == 0 {
                    return FALSE as u32;
                }
            } else {
                if WonderNews_Enter() == 0 {
                    return FALSE as u32;
                }
            }
            *state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn ClearSavedNewsOrCard(isWonderNews: u32) -> u32 {
    if isWonderNews == 0 {
        ClearSavedWonderCardAndRelated();
    } else {
        ClearSavedWonderNewsAndRelated();
    }
    TRUE as u32
}
unsafe fn ExitWonderCardOrNews(isWonderNews: u32, useCancel: u32) -> u32 {
    if isWonderNews == 0 {
        if WonderCard_Exit(useCancel) != 0 {
            WonderCard_Destroy();
            return TRUE as u32;
        } else {
            return FALSE as u32;
        }
    } else {
        if WonderNews_Exit(useCancel) != 0 {
            WonderNews_Destroy();
            return TRUE as u32;
        } else {
            return FALSE as u32;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn AskDiscardGift(textState: *mut u8, windowId: *mut u16, isWonderNews: u32) -> i32 {
    if isWonderNews == 0 {
        return DoMysteryGiftYesNo(
            textState,
            windowId,
            TRUE,
            (*(&raw const crate::data::strings::gText_IfThrowAwayCardEventWontHappen)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        ) as i32;
    } else {
        return DoMysteryGiftYesNo(
            textState,
            windowId,
            TRUE,
            (*(&raw const crate::data::strings::gText_OkayToDiscardNews).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        ) as i32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PrintThrownAway(textState: *mut u8, isWonderNews: u32) -> u32 {
    if isWonderNews == 0 {
        return PrintMysteryGiftMenuMessage(
            textState,
            (*(&raw const crate::data::strings::gText_WonderCardThrownAway)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else {
        return PrintMysteryGiftMenuMessage(
            textState,
            (*(&raw const crate::data::strings::gText_WonderNewsThrownAway)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SaveOnMysteryGiftMenu(state: *mut u8) -> u32 {
    match *state {
        0 => {
            MG_AddMessageTextPrinter(
                (*(&raw const crate::data::strings::gText_DataWillBeSaved).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            *state += 1;
        }
        1 => {
            TrySavingData(SAVE_NORMAL);
            *state += 1;
        }
        2 => {
            MG_AddMessageTextPrinter(
                (*(&raw const crate::data::strings::gText_SaveCompletedPressA)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            *state += 1;
        }
        3 => {
            if gMain.newKeys as i32 & 3 != 0 {
                *state += 1;
            }
        }
        4 => {
            *state = 0;
            ClearMessage();
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn GetClientResultMessage(
    successMsg: *mut u32,
    isWonderNews: u8,
    sourceIsFriend: u8,
    msgId: u32,
) -> *mut u8 {
    let mut msg: *mut u8 = null_mut();
    *successMsg = FALSE as u32;
    match msgId {
        CLI_MSG_NOTHING_SENT => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_NothingSentOver)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_RECORD_UPLOADED => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_RecordUploadedViaWireless)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_CARD_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = if sourceIsFriend == 0 {
                (*(&raw const crate::data::strings::gText_WonderCardReceived)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            } else {
                (*(&raw const crate::data::strings::gText_WonderCardReceivedFrom)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            };
        }
        CLI_MSG_NEWS_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = if sourceIsFriend == 0 {
                (*(&raw const crate::data::strings::gText_WonderNewsReceived)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            } else {
                (*(&raw const crate::data::strings::gText_WonderNewsReceivedFrom)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            };
        }
        CLI_MSG_STAMP_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = (*(&raw const crate::data::strings::gText_NewStampReceived)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_HAD_CARD => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_AlreadyHadCard)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_HAD_STAMP => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_AlreadyHadStamp)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_HAD_NEWS => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_AlreadyHadNews)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_NO_ROOM_STAMPS => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_NoMoreRoomForStamps)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_COMM_CANCELED => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_CommunicationCanceled)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_CANT_ACCEPT => {
            *successMsg = FALSE as u32;
            msg = if isWonderNews == 0 {
                (*(&raw const crate::data::strings::gText_CantAcceptCardFromTrainer)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            } else {
                (*(&raw const crate::data::strings::gText_CantAcceptNewsFromTrainer)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            };
        }
        CLI_MSG_COMM_ERROR => {
            *successMsg = FALSE as u32;
            msg = (*(&raw const crate::data::strings::gText_CommunicationError)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_TRAINER_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = (*(&raw const crate::data::strings::gText_NewTrainerReceived)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        CLI_MSG_BUFFER_SUCCESS => {
            *successMsg = TRUE as u32;
        }
        CLI_MSG_BUFFER_FAILURE => {
            *successMsg = FALSE as u32;
        }
        _ => {}
    }
    msg
}
unsafe fn PrintSuccessMessage(state: *mut u8, msg: *mut u8, timer: *mut u16) -> u32 {
    match *state {
        0 => {
            if !msg.is_null() {
                MG_AddMessageTextPrinter(msg);
            }
            PlayFanfare(MUS_OBTAIN_ITEM);
            *timer = 0;
            *state += 1;
        }
        1 => {
            if ({
                *timer += 1;
                *timer
            }) > 240
            {
                *state += 1;
            }
        }
        2 if IsFanfareTaskInactive() != 0 => {
            *state = 0;
            ClearMessage();
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn GetServerResultMessage(
    wonderSuccess: *mut u32,
    sourceIsFriend: u8,
    msgId: u32,
) -> *mut u8 {
    let mut result: *mut u8 = (*(&raw const crate::data::strings::gText_CommunicationError)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    *wonderSuccess = FALSE as u32;
    match msgId {
        SVR_MSG_NOTHING_SENT => {
            result = (*(&raw const crate::data::strings::gText_NothingSentOver)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_RECORD_UPLOADED => {
            result = (*(&raw const crate::data::strings::gText_RecordUploadedViaWireless)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_CARD_SENT => {
            result = (*(&raw const crate::data::strings::gText_WonderCardSentTo)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            *wonderSuccess = TRUE as u32;
        }
        SVR_MSG_NEWS_SENT => {
            result = (*(&raw const crate::data::strings::gText_WonderNewsSentTo)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            *wonderSuccess = TRUE as u32;
        }
        SVR_MSG_STAMP_SENT => {
            result = (*(&raw const crate::data::strings::gText_StampSentTo)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_HAS_CARD => {
            result = (*(&raw const crate::data::strings::gText_OtherTrainerHasCard)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_HAS_STAMP => {
            result = (*(&raw const crate::data::strings::gText_OtherTrainerHasStamp)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_HAS_NEWS => {
            result = (*(&raw const crate::data::strings::gText_OtherTrainerHasNews)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_NO_ROOM_STAMPS => {
            result = (*(&raw const crate::data::strings::gText_NoMoreRoomForStamps)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_CLIENT_CANCELED => {
            result = (*(&raw const crate::data::strings::gText_OtherTrainerCanceled)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_CANT_SEND_GIFT_1 => {
            result = (*(&raw const crate::data::strings::gText_CantSendGiftToTrainer)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_COMM_ERROR => {
            result = (*(&raw const crate::data::strings::gText_CommunicationError)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        SVR_MSG_GIFT_SENT_1 => {
            result = (*(&raw const crate::data::strings::gText_GiftSentTo).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        SVR_MSG_GIFT_SENT_2 => {
            result = (*(&raw const crate::data::strings::gText_GiftSentTo).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        SVR_MSG_CANT_SEND_GIFT_2 => {
            result = (*(&raw const crate::data::strings::gText_CantSendGiftToTrainer)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        _ => {}
    }
    result
}
unsafe fn PrintServerResultMessage(
    state: *mut u8,
    timer: *mut u16,
    sourceIsFriend: u8,
    msgId: u32,
) -> u32 {
    let mut wonderSuccess: u32 = 0;
    let str: *mut u8 = GetServerResultMessage(&raw mut wonderSuccess, sourceIsFriend, msgId);
    if wonderSuccess != 0 {
        return PrintSuccessMessage(state, str, timer);
    } else {
        return PrintMysteryGiftMenuMessage(state, str);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CreateMysteryGiftTask() {
    let taskId: u8 = CreateTask(Some(Task_MysteryGift), 0);
    let data: *mut MysteryGiftTaskData =
        (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut c_void as *mut MysteryGiftTaskData;
    (*data).state = MG_STATE_TO_MAIN_MENU;
    (*data).textState = 0;
    (*data).unused4 = 0;
    (*data).unused5 = 0;
    (*data).isWonderNews = 0;
    (*data).sourceIsFriend = 0;
    (*data).var = 0;
    (*data).unused1 = 0;
    (*data).unused2 = 0;
    (*data).unused3 = 0;
    (*data).msgId = 0;
    (*data).clientMsg = AllocZeroed(CLIENT_MAX_MSG_SIZE) as *mut u8;
}
pub(crate) unsafe fn Task_MysteryGift(taskId: u8) {
    let data: *mut MysteryGiftTaskData =
        (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut c_void as *mut MysteryGiftTaskData;
    let mut successMsg: u32 = 0;
    let mut input: u32 = 0;
    let mut msg: *mut u8 = null_mut();
    'l1: {
        match (*data).state {
            MG_STATE_TO_MAIN_MENU => {
                (*data).state = MG_STATE_MAIN_MENU;
            }
            MG_STATE_MAIN_MENU => {
                match MysteryGift_HandleThreeOptionMenu(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    FALSE,
                ) {
                    0 => {
                        (*data).isWonderNews = FALSE;
                        if ValidateSavedWonderCard() == TRUE as u32 {
                            (*data).state = MG_STATE_LOAD_GIFT;
                        } else {
                            (*data).state = MG_STATE_DONT_HAVE_ANY;
                        }
                    }
                    1 => {
                        (*data).isWonderNews = TRUE;
                        if ValidateSavedWonderNews() == TRUE as u32 {
                            (*data).state = MG_STATE_LOAD_GIFT;
                        } else {
                            (*data).state = MG_STATE_DONT_HAVE_ANY;
                        }
                    }
                    0xfffffffe => {
                        (*data).state = MG_STATE_EXIT;
                    }
                    _ => {}
                }
            }
            MG_STATE_DONT_HAVE_ANY => {
                if (*data).isWonderNews == 0 {
                    if PrintMysteryGiftMenuMessage(
                        &raw mut (*data).textState,
                        (*(&raw const crate::data::strings::gText_DontHaveCardNewOneInput)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    ) != 0
                    {
                        (*data).state = MG_STATE_SOURCE_PROMPT;
                        PrintMysteryGiftOrEReaderHeader(FALSE, TRUE as u32);
                    }
                } else {
                    if PrintMysteryGiftMenuMessage(
                        &raw mut (*data).textState,
                        (*(&raw const crate::data::strings::gText_DontHaveNewsNewOneInput)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    ) != 0
                    {
                        (*data).state = MG_STATE_SOURCE_PROMPT;
                        PrintMysteryGiftOrEReaderHeader(FALSE, TRUE as u32);
                    }
                }
                break 'l1;
            }
            MG_STATE_SOURCE_PROMPT => {
                if (*data).isWonderNews == 0 {
                    MG_AddMessageTextPrinter(
                        (*(&raw const crate::data::strings::gText_WhereShouldCardBeAccessed)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                } else {
                    MG_AddMessageTextPrinter(
                        (*(&raw const crate::data::strings::gText_WhereShouldNewsBeAccessed)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                (*data).state = MG_STATE_SOURCE_PROMPT_INPUT;
            }
            MG_STATE_SOURCE_PROMPT_INPUT => {
                match MysteryGift_HandleThreeOptionMenu(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    TRUE,
                ) {
                    0 => {
                        ClearMessage();
                        (*data).state = MG_STATE_CLIENT_LINK_START;
                        (*data).sourceIsFriend = FALSE;
                    }
                    1 => {
                        ClearMessage();
                        (*data).state = MG_STATE_CLIENT_LINK_START;
                        (*data).sourceIsFriend = TRUE;
                    }
                    0xfffffffe => {
                        ClearMessage();
                        if ValidateCardOrNews((*data).isWonderNews as u32) != 0 {
                            (*data).state = MG_STATE_LOAD_GIFT;
                        } else {
                            (*data).state = MG_STATE_TO_MAIN_MENU;
                            PrintMysteryGiftOrEReaderHeader(FALSE, FALSE as u32);
                        }
                    }
                    _ => {}
                }
            }
            MG_STATE_CLIENT_LINK_START => {
                *gStringVar1.as_mut_ptr() = EOS;
                *gStringVar2.as_mut_ptr() = EOS;
                *gStringVar3.as_mut_ptr() = EOS;
                match (*data).isWonderNews {
                    FALSE => {
                        if (*data).sourceIsFriend == TRUE {
                            CreateTask_LinkMysteryGiftWithFriend(ACTIVITY_WONDER_CARD as u32);
                        } else if (*data).sourceIsFriend == FALSE {
                            CreateTask_LinkMysteryGiftOverWireless(ACTIVITY_WONDER_CARD as u32);
                        }
                    }
                    TRUE => {
                        if (*data).sourceIsFriend == TRUE {
                            CreateTask_LinkMysteryGiftWithFriend(ACTIVITY_WONDER_NEWS as u32);
                        } else if (*data).sourceIsFriend == FALSE {
                            CreateTask_LinkMysteryGiftOverWireless(ACTIVITY_WONDER_NEWS as u32);
                        }
                    }
                    _ => {}
                }
                (*data).state = MG_STATE_CLIENT_LINK_WAIT;
            }
            MG_STATE_CLIENT_LINK_WAIT => {
                if gReceivedRemoteLinkPlayers != 0 {
                    ClearScreenInBg0(TRUE as u32);
                    (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                    MysteryGiftClient_Create((*data).isWonderNews as u32);
                } else if gSpecialVar_Result == LINKUP_FAILED {
                    ClearScreenInBg0(TRUE as u32);
                    (*data).state = MG_STATE_SOURCE_PROMPT;
                }
            }
            MG_STATE_CLIENT_COMMUNICATING => {
                MG_AddMessageTextPrinter(
                    (*(&raw const crate::data::strings::gText_Communicating)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                (*data).state = MG_STATE_CLIENT_LINK;
            }
            MG_STATE_CLIENT_LINK => match MysteryGiftClient_Run(&raw mut (*data).var) {
                CLI_RET_END => {
                    Rfu_SetCloseLinkCallback();
                    (*data).msgId = (*data).var as u8;
                    (*data).state = MG_STATE_CLIENT_LINK_END;
                }
                CLI_RET_COPY_MSG => {
                    memcpy(
                        (*data).clientMsg,
                        MysteryGiftClient_GetMsg() as *mut u8,
                        CLIENT_MAX_MSG_SIZE,
                    );
                    MysteryGiftClient_AdvanceState();
                }
                CLI_RET_PRINT_MSG => {
                    (*data).state = MG_STATE_CLIENT_MESSAGE;
                }
                CLI_RET_YES_NO => {
                    (*data).state = MG_STATE_CLIENT_YES_NO;
                }
                CLI_RET_ASK_TOSS => {
                    (*data).state = MG_STATE_CLIENT_ASK_TOSS;
                    StringCopy(gStringVar1.as_mut_ptr(), gLinkPlayers[0].name.as_mut_ptr());
                }
                _ => {}
            },
            MG_STATE_CLIENT_YES_NO => {
                input = DoMysteryGiftYesNo(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    FALSE,
                    MysteryGiftClient_GetMsg() as *mut u8,
                ) as u32;
                match input {
                    0 => {
                        MysteryGiftClient_SetParam(FALSE as u32);
                        MysteryGiftClient_AdvanceState();
                        (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                    }
                    1 | 0xffffffff => {
                        MysteryGiftClient_SetParam(TRUE as u32);
                        MysteryGiftClient_AdvanceState();
                        (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                    }
                    _ => {}
                }
            }
            MG_STATE_CLIENT_MESSAGE => {
                if PrintMysteryGiftMenuMessage(
                    &raw mut (*data).textState,
                    MysteryGiftClient_GetMsg() as *mut u8,
                ) != 0
                {
                    MysteryGiftClient_AdvanceState();
                    (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                }
            }
            MG_STATE_CLIENT_ASK_TOSS => {
                input = DoMysteryGiftYesNo(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    FALSE,
                    (*(&raw const crate::data::strings::gText_ThrowAwayWonderCard)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                ) as u32;
                match input {
                    0 => {
                        if IsSavedWonderCardGiftNotReceived() == TRUE as u32 {
                            (*data).state = MG_STATE_CLIENT_ASK_TOSS_UNRECEIVED;
                        } else {
                            MysteryGiftClient_SetParam(FALSE as u32);
                            MysteryGiftClient_AdvanceState();
                            (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                        }
                    }
                    1 | 0xffffffff => {
                        MysteryGiftClient_SetParam(TRUE as u32);
                        MysteryGiftClient_AdvanceState();
                        (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                    }
                    _ => {}
                }
            }
            MG_STATE_CLIENT_ASK_TOSS_UNRECEIVED => {
                input = DoMysteryGiftYesNo(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    FALSE,
                    (*(&raw const crate::data::strings::gText_HaventReceivedCardsGift)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                ) as u32;
                match input {
                    0 => {
                        MysteryGiftClient_SetParam(FALSE as u32);
                        MysteryGiftClient_AdvanceState();
                        (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                    }
                    1 | 0xffffffff => {
                        MysteryGiftClient_SetParam(TRUE as u32);
                        MysteryGiftClient_AdvanceState();
                        (*data).state = MG_STATE_CLIENT_COMMUNICATING;
                    }
                    _ => {}
                }
            }
            MG_STATE_CLIENT_LINK_END => {
                if gReceivedRemoteLinkPlayers == 0 {
                    DestroyWirelessStatusIndicatorSprite();
                    (*data).state = MG_STATE_CLIENT_COMM_COMPLETED;
                }
            }
            MG_STATE_CLIENT_COMM_COMPLETED => {
                if PrintStringAndWait2Seconds(
                    &raw mut (*data).textState,
                    (*(&raw const crate::data::strings::gText_CommunicationCompleted)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                ) != 0
                {
                    if (*data).sourceIsFriend == TRUE {
                        StringCopy(gStringVar1.as_mut_ptr(), gLinkPlayers[0].name.as_mut_ptr());
                    }
                    (*data).state = MG_STATE_CLIENT_RESULT_MSG;
                }
            }
            MG_STATE_CLIENT_RESULT_MSG => {
                msg = GetClientResultMessage(
                    &raw mut successMsg,
                    (*data).isWonderNews,
                    (*data).sourceIsFriend,
                    (*data).msgId as u32,
                );
                if msg.is_null() {
                    msg = (*data).clientMsg;
                }
                if successMsg != 0 {
                    input =
                        PrintSuccessMessage(&raw mut (*data).textState, msg, &raw mut (*data).var);
                } else {
                    input = PrintMysteryGiftMenuMessage(&raw mut (*data).textState, msg);
                }
                if input != 0 {
                    if (*data).msgId == CLI_MSG_NEWS_RECEIVED as u8 {
                        if (*data).sourceIsFriend == TRUE {
                            WonderNews_SetReward(WONDER_NEWS_RECV_FRIEND);
                        } else {
                            WonderNews_SetReward(WONDER_NEWS_RECV_WIRELESS);
                        }
                    }
                    if successMsg == 0 {
                        (*data).state = MG_STATE_TO_MAIN_MENU;
                        PrintMysteryGiftOrEReaderHeader(FALSE, FALSE as u32);
                    } else {
                        (*data).state = MG_STATE_SAVE_LOAD_GIFT;
                    }
                }
            }
            MG_STATE_SAVE_LOAD_GIFT => {
                if SaveOnMysteryGiftMenu(&raw mut (*data).textState) != 0 {
                    (*data).state = MG_STATE_LOAD_GIFT;
                }
            }
            MG_STATE_LOAD_GIFT => {
                if HandleLoadWonderCardOrNews(
                    &raw mut (*data).textState,
                    (*data).isWonderNews as u32,
                ) != 0
                {
                    (*data).state = MG_STATE_HANDLE_GIFT_INPUT;
                }
            }
            MG_STATE_HANDLE_GIFT_INPUT => {
                if (*data).isWonderNews == 0 {
                    if gMain.newKeys as i32 & A_BUTTON != 0 {
                        (*data).state = MG_STATE_HANDLE_GIFT_SELECT;
                    }
                    if gMain.newKeys as i32 & B_BUTTON != 0 {
                        (*data).state = MG_STATE_GIFT_INPUT_EXIT;
                    }
                } else {
                    match WonderNews_GetInput(gMain.newKeys) {
                        NEWS_INPUT_A => {
                            WonderNews_RemoveScrollIndicatorArrowPair();
                            (*data).state = MG_STATE_HANDLE_GIFT_SELECT;
                        }
                        NEWS_INPUT_B => {
                            (*data).state = MG_STATE_GIFT_INPUT_EXIT;
                        }
                        _ => {}
                    }
                }
            }
            MG_STATE_HANDLE_GIFT_SELECT => {
                let mut result: u32 = 0;
                if (*data).isWonderNews == 0 {
                    if IsSendingSavedWonderCardAllowed() != 0 {
                        result = HandleGiftSelectMenu(
                            &raw mut (*data).textState,
                            &raw mut (*data).var,
                            (*data).isWonderNews as u32,
                            FALSE as u32,
                        ) as u32;
                    } else {
                        result = HandleGiftSelectMenu(
                            &raw mut (*data).textState,
                            &raw mut (*data).var,
                            (*data).isWonderNews as u32,
                            TRUE as u32,
                        ) as u32;
                    }
                } else {
                    if IsSendingSavedWonderNewsAllowed() != 0 {
                        result = HandleGiftSelectMenu(
                            &raw mut (*data).textState,
                            &raw mut (*data).var,
                            (*data).isWonderNews as u32,
                            FALSE as u32,
                        ) as u32;
                    } else {
                        result = HandleGiftSelectMenu(
                            &raw mut (*data).textState,
                            &raw mut (*data).var,
                            (*data).isWonderNews as u32,
                            TRUE as u32,
                        ) as u32;
                    }
                }
                match result {
                    0 => {
                        (*data).state = MG_STATE_RECEIVE;
                    }
                    1 => {
                        (*data).state = MG_STATE_SEND;
                    }
                    2 => {
                        (*data).state = MG_STATE_ASK_TOSS;
                    }
                    0xfffffffe => {
                        if (*data).isWonderNews == TRUE {
                            WonderNews_AddScrollIndicatorArrowPair();
                        }
                        (*data).state = MG_STATE_HANDLE_GIFT_INPUT;
                    }
                    _ => {}
                }
                break 'l1;
            }
            MG_STATE_ASK_TOSS => {
                match AskDiscardGift(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    (*data).isWonderNews as u32,
                ) {
                    0 => {
                        if (*data).isWonderNews == 0
                            && IsSavedWonderCardGiftNotReceived() == TRUE as u32
                        {
                            (*data).state = MG_STATE_ASK_TOSS_UNRECEIVED;
                        } else {
                            (*data).state = MG_STATE_TOSS;
                        }
                    }
                    1 | -1 => {
                        (*data).state = MG_STATE_HANDLE_GIFT_SELECT;
                    }
                    _ => {}
                }
            }
            MG_STATE_ASK_TOSS_UNRECEIVED => {
                match DoMysteryGiftYesNo(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    TRUE,
                    (*(&raw const crate::data::strings::gText_HaventReceivedGiftOkayToDiscard)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                ) as u32
                {
                    0 => {
                        (*data).state = MG_STATE_TOSS;
                    }
                    1 | 0xffffffff => {
                        (*data).state = MG_STATE_HANDLE_GIFT_SELECT;
                    }
                    _ => {}
                }
            }
            MG_STATE_TOSS => {
                if ExitWonderCardOrNews((*data).isWonderNews as u32, TRUE as u32) != 0 {
                    ClearSavedNewsOrCard((*data).isWonderNews as u32);
                    (*data).state = MG_STATE_TOSS_SAVE;
                }
            }
            MG_STATE_TOSS_SAVE => {
                if SaveOnMysteryGiftMenu(&raw mut (*data).textState) != 0 {
                    (*data).state = MG_STATE_TOSSED;
                }
            }
            MG_STATE_TOSSED => {
                if PrintThrownAway(&raw mut (*data).textState, (*data).isWonderNews as u32) != 0 {
                    (*data).state = MG_STATE_TO_MAIN_MENU;
                    PrintMysteryGiftOrEReaderHeader(FALSE, FALSE as u32);
                }
            }
            MG_STATE_GIFT_INPUT_EXIT => {
                if ExitWonderCardOrNews((*data).isWonderNews as u32, FALSE as u32) != 0 {
                    (*data).state = MG_STATE_TO_MAIN_MENU;
                }
            }
            MG_STATE_RECEIVE => {
                if ExitWonderCardOrNews((*data).isWonderNews as u32, TRUE as u32) != 0 {
                    (*data).state = MG_STATE_SOURCE_PROMPT;
                }
            }
            MG_STATE_SEND => {
                if ExitWonderCardOrNews((*data).isWonderNews as u32, TRUE as u32) != 0 {
                    match (*data).isWonderNews {
                        FALSE => {
                            CreateTask_SendMysteryGift(ACTIVITY_WONDER_CARD as u32);
                        }
                        TRUE => {
                            CreateTask_SendMysteryGift(ACTIVITY_WONDER_NEWS as u32);
                        }
                        _ => {}
                    }
                    (*data).sourceIsFriend = TRUE;
                    (*data).state = MG_STATE_SERVER_LINK_WAIT;
                }
            }
            MG_STATE_SERVER_LINK_WAIT => {
                if gReceivedRemoteLinkPlayers != 0 {
                    ClearScreenInBg0(TRUE as u32);
                    (*data).state = MG_STATE_SERVER_LINK_START;
                } else if gSpecialVar_Result == LINKUP_FAILED {
                    ClearScreenInBg0(TRUE as u32);
                    (*data).state = MG_STATE_LOAD_GIFT;
                }
            }
            MG_STATE_SERVER_LINK_START => {
                *gStringVar1.as_mut_ptr() = EOS;
                *gStringVar2.as_mut_ptr() = EOS;
                *gStringVar3.as_mut_ptr() = EOS;
                if (*data).isWonderNews == 0 {
                    MG_AddMessageTextPrinter(
                        (*(&raw const crate::data::strings::gText_SendingWonderCard)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    MysterGiftServer_CreateForCard();
                } else {
                    MG_AddMessageTextPrinter(
                        (*(&raw const crate::data::strings::gText_SendingWonderNews)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    MysterGiftServer_CreateForNews();
                }
                (*data).state = MG_STATE_SERVER_LINK;
            }
            MG_STATE_SERVER_LINK => {
                if MysterGiftServer_Run(&raw mut (*data).var) == SVR_RET_END {
                    (*data).msgId = (*data).var as u8;
                    (*data).state = MG_STATE_SERVER_LINK_END;
                }
            }
            MG_STATE_SERVER_LINK_END => {
                Rfu_SetCloseLinkCallback();
                StringCopy(gStringVar1.as_mut_ptr(), gLinkPlayers[1].name.as_mut_ptr());
                (*data).state = MG_STATE_SERVER_LINK_END_WAIT;
            }
            MG_STATE_SERVER_LINK_END_WAIT => {
                if gReceivedRemoteLinkPlayers == 0 {
                    DestroyWirelessStatusIndicatorSprite();
                    (*data).state = MG_STATE_SERVER_RESULT_MSG;
                }
            }
            MG_STATE_SERVER_RESULT_MSG => {
                if PrintServerResultMessage(
                    &raw mut (*data).textState,
                    &raw mut (*data).var,
                    (*data).sourceIsFriend,
                    (*data).msgId as u32,
                ) != 0
                {
                    if (*data).sourceIsFriend == TRUE && (*data).msgId == SVR_MSG_NEWS_SENT as u8 {
                        WonderNews_SetReward(WONDER_NEWS_SENT);
                        (*data).state = MG_STATE_SAVE_LOAD_GIFT;
                    } else {
                        (*data).state = MG_STATE_TO_MAIN_MENU;
                        PrintMysteryGiftOrEReaderHeader(FALSE, FALSE as u32);
                    }
                }
            }
            MG_STATE_CLIENT_ERROR | MG_STATE_SERVER_ERROR => {
                if PrintMysteryGiftMenuMessage(
                    &raw mut (*data).textState,
                    (*(&raw const crate::data::strings::gText_CommunicationError)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                ) != 0
                {
                    (*data).state = MG_STATE_TO_MAIN_MENU;
                    PrintMysteryGiftOrEReaderHeader(FALSE, FALSE as u32);
                }
            }
            MG_STATE_EXIT => {
                CloseLink();
                Free((*data).clientMsg as *mut c_void);
                DestroyTask(taskId);
                SetMainCallback2(Some(MainCB_FreeAllBuffersAndReturnToInitTitleScreen));
            }
            _ => {}
        }
    }
}
pub fn GetMysteryGiftBaseBlock() -> u16 {
    0x1A9
}
unsafe fn LoadMysteryGiftTextboxBorder(bgId: u8) {
    DecompressAndLoadBgGfxUsingHeap(
        bgId,
        sTextboxBorder_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x100,
        0,
        0,
    );
}
