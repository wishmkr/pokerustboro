//! Translated from `src/mystery_gift_menu.c` by tools/rustport/c2rs.py.
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
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gGiftIsFromEReader: u8 = 0;

unsafe extern "C" {
    static gJPText_DecideStop: CArray<u8, 0>;
    static gJPText_MysteryGift: CArray<u8, 0>;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSpecialVar_Result: u16;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AlreadyHadCard: CArray<u8, 0>;
    static gText_AlreadyHadNews: CArray<u8, 0>;
    static gText_AlreadyHadStamp: CArray<u8, 0>;
    static gText_CantAcceptCardFromTrainer: CArray<u8, 0>;
    static gText_CantAcceptNewsFromTrainer: CArray<u8, 0>;
    static gText_CantSendGiftToTrainer: CArray<u8, 0>;
    static gText_Communicating: CArray<u8, 0>;
    static gText_CommunicationCanceled: CArray<u8, 0>;
    static gText_CommunicationCompleted: CArray<u8, 0>;
    static gText_CommunicationError: CArray<u8, 0>;
    static gText_DataWillBeSaved: CArray<u8, 0>;
    static gText_DontHaveCardNewOneInput: CArray<u8, 0>;
    static gText_DontHaveNewsNewOneInput: CArray<u8, 0>;
    static gText_GiftSentTo: CArray<u8, 0>;
    static gText_HaventReceivedCardsGift: CArray<u8, 0>;
    static gText_HaventReceivedGiftOkayToDiscard: CArray<u8, 0>;
    static gText_IfThrowAwayCardEventWontHappen: CArray<u8, 0>;
    static gText_MysteryGift: CArray<u8, 0>;
    static gText_NewStampReceived: CArray<u8, 0>;
    static gText_NewTrainerReceived: CArray<u8, 0>;
    static gText_NoMoreRoomForStamps: CArray<u8, 0>;
    static gText_NothingSentOver: CArray<u8, 0>;
    static gText_OkayToDiscardNews: CArray<u8, 0>;
    static gText_OtherTrainerCanceled: CArray<u8, 0>;
    static gText_OtherTrainerHasCard: CArray<u8, 0>;
    static gText_OtherTrainerHasNews: CArray<u8, 0>;
    static gText_OtherTrainerHasStamp: CArray<u8, 0>;
    static gText_PickOKCancel: CArray<u8, 0>;
    static gText_PickOKExit: CArray<u8, 0>;
    static gText_RecordUploadedViaWireless: CArray<u8, 0>;
    static gText_SaveCompletedPressA: CArray<u8, 0>;
    static gText_SendingWonderCard: CArray<u8, 0>;
    static gText_SendingWonderNews: CArray<u8, 0>;
    static gText_StampSentTo: CArray<u8, 0>;
    static gText_ThrowAwayWonderCard: CArray<u8, 0>;
    static gText_WhatToDoWithCards: CArray<u8, 0>;
    static gText_WhatToDoWithNews: CArray<u8, 0>;
    static gText_WhereShouldCardBeAccessed: CArray<u8, 0>;
    static gText_WhereShouldNewsBeAccessed: CArray<u8, 0>;
    static gText_WonderCardReceived: CArray<u8, 0>;
    static gText_WonderCardReceivedFrom: CArray<u8, 0>;
    static gText_WonderCardSentTo: CArray<u8, 0>;
    static gText_WonderCardThrownAway: CArray<u8, 0>;
    static gText_WonderNewsReceived: CArray<u8, 0>;
    static gText_WonderNewsReceivedFrom: CArray<u8, 0>;
    static gText_WonderNewsSentTo: CArray<u8, 0>;
    static gText_WonderNewsThrownAway: CArray<u8, 0>;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BuildOamBuffer();
    fn CB2_InitTitleScreen();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearSavedWonderCardAndRelated();
    fn ClearSavedWonderNewsAndRelated();
    fn ClearWindowTilemap(a0: u8);
    fn CloseLink();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateEReaderTask();
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_LinkMysteryGiftOverWireless(a0: u32);
    fn CreateTask_LinkMysteryGiftWithFriend(a0: u32);
    fn CreateTask_SendMysteryGift(a0: u32);
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoMysteryGiftListMenu(
        a0: *mut WindowTemplate,
        a1: *mut ListMenuTemplate,
        a2: u8,
        a3: u16,
        a4: u16,
    ) -> i32;
    fn DrawDownArrow(a0: u8, a1: u16, a2: u16, a3: u8, a4: u8, a5: *mut u8, a6: *mut u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn EnableInterrupts(a0: u16);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetSavedWonderCard() -> *mut WonderCard;
    fn GetSavedWonderCardMetadata() -> *mut WonderCardMetadata;
    fn GetSavedWonderNews() -> *mut WonderNews;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn Intl_GetListMenuWidth(a0: *mut ListMenuTemplate) -> i32;
    fn IsFanfareTaskInactive() -> u8;
    fn IsSavedWonderCardGiftNotReceived() -> u32;
    fn IsSendingSavedWonderCardAllowed() -> u32;
    fn IsSendingSavedWonderNewsAllowed() -> u32;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn Menu_LoadStdPalAt(a0: u16);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MysterGiftServer_CreateForCard();
    fn MysterGiftServer_CreateForNews();
    fn MysterGiftServer_Run(a0: *mut u16) -> u32;
    fn MysteryGiftClient_AdvanceState();
    fn MysteryGiftClient_Create(a0: u32);
    fn MysteryGiftClient_GetMsg() -> *mut c_void;
    fn MysteryGiftClient_Run(a0: *mut u16) -> u32;
    fn MysteryGiftClient_SetParam(a0: u32);
    fn PlayBGM(a0: u16);
    fn PlayFanfare(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn Rfu_SetCloseLinkCallback();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn ValidateSavedWonderCard() -> u32;
    fn ValidateSavedWonderNews() -> u32;
    fn WonderCard_Destroy();
    fn WonderCard_Enter() -> i32;
    fn WonderCard_Exit(a0: u32) -> i32;
    fn WonderCard_Init(a0: *mut WonderCard, a1: *mut WonderCardMetadata) -> u32;
    fn WonderNews_AddScrollIndicatorArrowPair();
    fn WonderNews_Destroy();
    fn WonderNews_Enter() -> i32;
    fn WonderNews_Exit(a0: u32) -> i32;
    fn WonderNews_GetInput(a0: u16) -> u32;
    fn WonderNews_Init(a0: *mut WonderNews) -> u32;
    fn WonderNews_RemoveScrollIndicatorArrowPair();
    fn WonderNews_SetReward(a0: u32);
    fn rbox_fill_rectangle(a0: u8);
}

pub(crate) unsafe extern "C" fn VBlankCB_MysteryGiftEReader() {
    ProcessSpriteCopyRequests();
    LoadOam();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_MysteryGiftEReader() {
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
}
pub(crate) unsafe extern "C" fn HandleMysteryGiftOrEReaderSetup(isEReader: i32) -> u32 {
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
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitMysteryGift() {
    if HandleMysteryGiftOrEReaderSetup(FALSE as i32) != 0 {
        SetMainCallback2(Some(CB2_MysteryGiftEReader));
        gGiftIsFromEReader = FALSE;
        CreateMysteryGiftTask();
    }
    RunTasks();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitEReader() {
    if HandleMysteryGiftOrEReaderSetup(TRUE as i32) != 0 {
        SetMainCallback2(Some(CB2_MysteryGiftEReader));
        gGiftIsFromEReader = TRUE;
        CreateEReaderTask();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MainCB_FreeAllBuffersAndReturnToInitTitleScreen() {
    gGiftIsFromEReader = FALSE;
    FreeAllWindowBuffers();
    Free(GetBgTilemapBuffer(0));
    Free(GetBgTilemapBuffer(1));
    Free(GetBgTilemapBuffer(2));
    Free(GetBgTilemapBuffer(3));
    SetMainCallback2(Some(CB2_InitTitleScreen));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMysteryGiftOrEReaderHeader(isEReader: u8, useCancel: u32) {
    let mut title: *mut u8 = null_mut();
    let mut options: *mut u8 = null_mut();
    FillWindowPixelBuffer(WIN_HEADER, 0);
    if isEReader == 0 {
        title = gText_MysteryGift.as_ptr().cast_mut();
        options = if useCancel == 0 {
            gText_PickOKExit.as_ptr().cast_mut()
        } else {
            gText_PickOKCancel.as_ptr().cast_mut()
        };
    } else {
        title = gJPText_MysteryGift.as_ptr().cast_mut();
        options = gJPText_DecideStop.as_ptr().cast_mut();
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MG_DrawTextBorder(windowId: u8) {
    DrawTextBorderOuter(windowId, 0x01, 0xF);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MG_DrawCheckerboardPattern(bg: u32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    FillBgTilemapBufferRect(bg as u8, 0x003, 0, 0, 32, 2, 17);
    i = 0;
    while i < 18 {
        j = 0;
        while j < 32 {
            if i & 1 != j & 1 {
                FillBgTilemapBufferRect(bg as u8, 1, j as u8, i as u8 + 2, 1, 1, 17);
            } else {
                FillBgTilemapBufferRect(bg as u8, 2, j as u8, i as u8 + 2, 1, 1, 17);
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearScreenInBg0(ignoreTopTwoRows: u32) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MG_AddMessageTextPrinter(str: *mut u8) {
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
pub(crate) unsafe extern "C" fn ClearMessage() {
    rbox_fill_rectangle(WIN_MSG);
    ClearWindowTilemap(WIN_MSG);
    CopyWindowToVram(WIN_MSG, COPYWIN_MAP);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMysteryGiftMenuMessage(textState: *mut u8, str: *mut u8) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn HideDownArrow() {
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
pub(crate) unsafe extern "C" fn ShowDownArrow() {
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
pub(crate) unsafe extern "C" fn HideDownArrowAndWaitButton(textState: *mut u8) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn PrintStringAndWait2Seconds(counter: *mut u8, str: *mut u8) -> u32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MysteryGift_HandleThreeOptionMenu(
    unused0: *mut u8,
    unused1: *mut u16,
    whichMenu: u8,
) -> u32 {
    let mut listMenuTemplate: ListMenuTemplate = zeroed();
    listMenuTemplate = *sListMenuTemplate_ThreeOptions;
    let mut windowTemplate: WindowTemplate = zeroed();
    windowTemplate = *sWindowTemplate_ThreeOptions;
    let mut width: i32 = 0;
    let mut response: i32 = 0;
    if whichMenu == 0 {
        listMenuTemplate.items = sListMenuItems_CardsOrNews.as_ptr().cast_mut();
    } else {
        listMenuTemplate.items = sListMenuItems_WirelessOrFriend.as_ptr().cast_mut();
    }
    width = Intl_GetListMenuWidth(&raw mut listMenuTemplate);
    if width & 1 != 0 {
        width += 1;
    }
    windowTemplate.width = width as u8;
    if width < DISPLAY_TILE_WIDTH as i32 {
        windowTemplate.tilemapLeft = ((DISPLAY_TILE_WIDTH as i32 - width) / 2) as u8;
    } else {
        windowTemplate.tilemapLeft = 0;
    }
    response = DoMysteryGiftListMenu(
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
    return response as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMysteryGiftYesNo(
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
    return MENU_NOTHING_CHOSEN;
}
pub(crate) unsafe extern "C" fn HandleGiftSelectMenu(
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
                    gText_WhatToDoWithCards.as_ptr().cast_mut(),
                );
            } else {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_WhatToDoWithNews.as_ptr().cast_mut(),
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
    return LIST_NOTHING_CHOSEN;
}
pub(crate) unsafe extern "C" fn ValidateCardOrNews(isWonderNews: u32) -> u32 {
    if isWonderNews == 0 {
        return ValidateSavedWonderCard();
    } else {
        return ValidateSavedWonderNews();
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn HandleLoadWonderCardOrNews(
    state: *mut u8,
    isWonderNews: u32,
) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn ClearSavedNewsOrCard(isWonderNews: u32) -> u32 {
    if isWonderNews == 0 {
        ClearSavedWonderCardAndRelated();
    } else {
        ClearSavedWonderNewsAndRelated();
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ExitWonderCardOrNews(isWonderNews: u32, useCancel: u32) -> u32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AskDiscardGift(
    textState: *mut u8,
    windowId: *mut u16,
    isWonderNews: u32,
) -> i32 {
    if isWonderNews == 0 {
        return DoMysteryGiftYesNo(
            textState,
            windowId,
            TRUE,
            gText_IfThrowAwayCardEventWontHappen.as_ptr().cast_mut(),
        ) as i32;
    } else {
        return DoMysteryGiftYesNo(
            textState,
            windowId,
            TRUE,
            gText_OkayToDiscardNews.as_ptr().cast_mut(),
        ) as i32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PrintThrownAway(textState: *mut u8, isWonderNews: u32) -> u32 {
    if isWonderNews == 0 {
        return PrintMysteryGiftMenuMessage(
            textState,
            gText_WonderCardThrownAway.as_ptr().cast_mut(),
        );
    } else {
        return PrintMysteryGiftMenuMessage(
            textState,
            gText_WonderNewsThrownAway.as_ptr().cast_mut(),
        );
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SaveOnMysteryGiftMenu(state: *mut u8) -> u32 {
    match *state {
        0 => {
            MG_AddMessageTextPrinter(gText_DataWillBeSaved.as_ptr().cast_mut());
            *state += 1;
        }
        1 => {
            TrySavingData(SAVE_NORMAL);
            *state += 1;
        }
        2 => {
            MG_AddMessageTextPrinter(gText_SaveCompletedPressA.as_ptr().cast_mut());
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetClientResultMessage(
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
            msg = gText_NothingSentOver.as_ptr().cast_mut();
        }
        CLI_MSG_RECORD_UPLOADED => {
            *successMsg = FALSE as u32;
            msg = gText_RecordUploadedViaWireless.as_ptr().cast_mut();
        }
        CLI_MSG_CARD_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = if sourceIsFriend == 0 {
                gText_WonderCardReceived.as_ptr().cast_mut()
            } else {
                gText_WonderCardReceivedFrom.as_ptr().cast_mut()
            };
        }
        CLI_MSG_NEWS_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = if sourceIsFriend == 0 {
                gText_WonderNewsReceived.as_ptr().cast_mut()
            } else {
                gText_WonderNewsReceivedFrom.as_ptr().cast_mut()
            };
        }
        CLI_MSG_STAMP_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = gText_NewStampReceived.as_ptr().cast_mut();
        }
        CLI_MSG_HAD_CARD => {
            *successMsg = FALSE as u32;
            msg = gText_AlreadyHadCard.as_ptr().cast_mut();
        }
        CLI_MSG_HAD_STAMP => {
            *successMsg = FALSE as u32;
            msg = gText_AlreadyHadStamp.as_ptr().cast_mut();
        }
        CLI_MSG_HAD_NEWS => {
            *successMsg = FALSE as u32;
            msg = gText_AlreadyHadNews.as_ptr().cast_mut();
        }
        CLI_MSG_NO_ROOM_STAMPS => {
            *successMsg = FALSE as u32;
            msg = gText_NoMoreRoomForStamps.as_ptr().cast_mut();
        }
        CLI_MSG_COMM_CANCELED => {
            *successMsg = FALSE as u32;
            msg = gText_CommunicationCanceled.as_ptr().cast_mut();
        }
        CLI_MSG_CANT_ACCEPT => {
            *successMsg = FALSE as u32;
            msg = if isWonderNews == 0 {
                gText_CantAcceptCardFromTrainer.as_ptr().cast_mut()
            } else {
                gText_CantAcceptNewsFromTrainer.as_ptr().cast_mut()
            };
        }
        CLI_MSG_COMM_ERROR => {
            *successMsg = FALSE as u32;
            msg = gText_CommunicationError.as_ptr().cast_mut();
        }
        CLI_MSG_TRAINER_RECEIVED => {
            *successMsg = TRUE as u32;
            msg = gText_NewTrainerReceived.as_ptr().cast_mut();
        }
        CLI_MSG_BUFFER_SUCCESS => {
            *successMsg = TRUE as u32;
        }
        CLI_MSG_BUFFER_FAILURE => {
            *successMsg = FALSE as u32;
        }
        _ => {}
    }
    return msg;
}
pub(crate) unsafe extern "C" fn PrintSuccessMessage(
    state: *mut u8,
    msg: *mut u8,
    timer: *mut u16,
) -> u32 {
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
        2 => {
            if IsFanfareTaskInactive() != 0 {
                *state = 0;
                ClearMessage();
                return TRUE as u32;
            }
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetServerResultMessage(
    wonderSuccess: *mut u32,
    sourceIsFriend: u8,
    msgId: u32,
) -> *mut u8 {
    let mut result: *mut u8 = gText_CommunicationError.as_ptr().cast_mut();
    *wonderSuccess = FALSE as u32;
    match msgId {
        SVR_MSG_NOTHING_SENT => {
            result = gText_NothingSentOver.as_ptr().cast_mut();
        }
        SVR_MSG_RECORD_UPLOADED => {
            result = gText_RecordUploadedViaWireless.as_ptr().cast_mut();
        }
        SVR_MSG_CARD_SENT => {
            result = gText_WonderCardSentTo.as_ptr().cast_mut();
            *wonderSuccess = TRUE as u32;
        }
        SVR_MSG_NEWS_SENT => {
            result = gText_WonderNewsSentTo.as_ptr().cast_mut();
            *wonderSuccess = TRUE as u32;
        }
        SVR_MSG_STAMP_SENT => {
            result = gText_StampSentTo.as_ptr().cast_mut();
        }
        SVR_MSG_HAS_CARD => {
            result = gText_OtherTrainerHasCard.as_ptr().cast_mut();
        }
        SVR_MSG_HAS_STAMP => {
            result = gText_OtherTrainerHasStamp.as_ptr().cast_mut();
        }
        SVR_MSG_HAS_NEWS => {
            result = gText_OtherTrainerHasNews.as_ptr().cast_mut();
        }
        SVR_MSG_NO_ROOM_STAMPS => {
            result = gText_NoMoreRoomForStamps.as_ptr().cast_mut();
        }
        SVR_MSG_CLIENT_CANCELED => {
            result = gText_OtherTrainerCanceled.as_ptr().cast_mut();
        }
        SVR_MSG_CANT_SEND_GIFT_1 => {
            result = gText_CantSendGiftToTrainer.as_ptr().cast_mut();
        }
        SVR_MSG_COMM_ERROR => {
            result = gText_CommunicationError.as_ptr().cast_mut();
        }
        SVR_MSG_GIFT_SENT_1 => {
            result = gText_GiftSentTo.as_ptr().cast_mut();
        }
        SVR_MSG_GIFT_SENT_2 => {
            result = gText_GiftSentTo.as_ptr().cast_mut();
        }
        SVR_MSG_CANT_SEND_GIFT_2 => {
            result = gText_CantSendGiftToTrainer.as_ptr().cast_mut();
        }
        _ => {}
    }
    return result;
}
pub(crate) unsafe extern "C" fn PrintServerResultMessage(
    state: *mut u8,
    timer: *mut u16,
    sourceIsFriend: u8,
    msgId: u32,
) -> u32 {
    let mut wonderSuccess: u32 = 0;
    let mut str: *mut u8 = GetServerResultMessage(&raw mut wonderSuccess, sourceIsFriend, msgId);
    if wonderSuccess != 0 {
        return PrintSuccessMessage(state, str, timer);
    } else {
        return PrintMysteryGiftMenuMessage(state, str);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreateMysteryGiftTask() {
    let mut taskId: u8 = CreateTask(Some(Task_MysteryGift), 0);
    let mut data: *mut MysteryGiftTaskData =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut MysteryGiftTaskData;
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
pub(crate) unsafe extern "C" fn Task_MysteryGift(taskId: u8) {
    let mut data: *mut MysteryGiftTaskData =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut MysteryGiftTaskData;
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
                        gText_DontHaveCardNewOneInput.as_ptr().cast_mut(),
                    ) != 0
                    {
                        (*data).state = MG_STATE_SOURCE_PROMPT;
                        PrintMysteryGiftOrEReaderHeader(FALSE, TRUE as u32);
                    }
                } else {
                    if PrintMysteryGiftMenuMessage(
                        &raw mut (*data).textState,
                        gText_DontHaveNewsNewOneInput.as_ptr().cast_mut(),
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
                    MG_AddMessageTextPrinter(gText_WhereShouldCardBeAccessed.as_ptr().cast_mut());
                } else {
                    MG_AddMessageTextPrinter(gText_WhereShouldNewsBeAccessed.as_ptr().cast_mut());
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
                MG_AddMessageTextPrinter(gText_Communicating.as_ptr().cast_mut());
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
                    gText_ThrowAwayWonderCard.as_ptr().cast_mut(),
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
                    gText_HaventReceivedCardsGift.as_ptr().cast_mut(),
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
                    gText_CommunicationCompleted.as_ptr().cast_mut(),
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
                    gText_HaventReceivedGiftOkayToDiscard.as_ptr().cast_mut(),
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
                    MG_AddMessageTextPrinter(gText_SendingWonderCard.as_ptr().cast_mut());
                    MysterGiftServer_CreateForCard();
                } else {
                    MG_AddMessageTextPrinter(gText_SendingWonderNews.as_ptr().cast_mut());
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
                    gText_CommunicationError.as_ptr().cast_mut(),
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMysteryGiftBaseBlock() -> u16 {
    return 0x1A9;
}
pub(crate) unsafe extern "C" fn LoadMysteryGiftTextboxBorder(bgId: u8) {
    DecompressAndLoadBgGfxUsingHeap(
        bgId,
        sTextboxBorder_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x100,
        0,
        0,
    );
}
