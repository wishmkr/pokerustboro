//! Translated from `src/union_room_chat.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sChatMainFunctions sKeyboardPageMaxRow sCaseToggleTable sUnionRoomKeyboardText sUnusedPalette sChatMessagesWindow_Pal sBgTemplates sWinTemplates sDisplaySubtasks sDisplayStdMessages sText_Ellipsis sKeyboardPageTitleTexts sUnionRoomChatInterfacePal sKeyboardCursorTiles sTextEntryCursorTiles sTextEntryArrowTiles sRButtonGfxTiles sSpriteSheets sSpritePalette sOam_KeyboardCursor sAnim_KeyboardCursor_Open sAnim_KeyboardCursor_Closed sAnim_KeyboardCursorWide_Open sAnim_KeyboardCursorWide_Closed sAnims_KeyboardCursor sSpriteTemplate_KeyboardCursor sOam_TextEntrySprite sSpriteTemplate_TextEntryCursor sSpriteTemplate_TextEntryArrow sOam_RButtonIcon sOam_RButtonLabel sAnim_ToggleCaseIcon sAnim_ToggleCaseIcon_Duplicate1 sAnim_ToggleCaseIcon_Duplicate2 sAnim_RegisterIcon sAnims_RButtonLabels sSpriteTemplate_RButtonIcon sSpriteTemplate_RButtonLabels

/// `struct UnionRoomChat`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnionRoomChat {
    pub filler1: u32,
    pub funcId: u16,
    pub funcState: u16,
    pub filler2: u16,
    pub exitDelayTimer: u16,
    pub filler3: u8,
    pub linkPlayerCount: u8,
    pub handleInputTask: u8,
    pub receiveMessagesTask: u8,
    pub currentPage: u8,
    pub currentCol: u8,
    pub currentRow: u8,
    pub multiplayerId: u8,
    pub lastBufferCursorPos: u8,
    pub bufferCursorPos: u8,
    pub receivedPlayerIndex: u8,
    pub exitType: u8,
    pub changedRegisteredTexts: u8,
    pub afterSaveTimer: u8,
    pub messageEntryBuffer: CArray<u8, 31>,
    pub receivedMessage: CArray<u8, 64>,
    pub hostName: CArray<u8, 64>,
    pub registeredTexts: CArray<CArray<u8, 21>, 10>,
    pub filler4: CArray<u8, 5>,
    pub sendMessageBuffer: CArray<u8, 40>,
    pub tryQuitAgainTimer: u16,
}

unsafe impl Sync for UnionRoomChat {}

/// `struct UnionRoomChatDisplay`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnionRoomChatDisplay {
    pub subtasks: CArray<UnionRoomChatDisplay_Subtask, 3>,
    pub yesNoMenuWindowId: u16,
    pub currLine: u16,
    pub scrollCount: u16,
    pub messageWindowId: u16,
    pub bg1hofs: i16,
    pub expandedPlaceholdersBuffer: CArray<u8, 262>,
    pub bg0Buffer: CArray<u8, 2048>,
    pub bg1Buffer: CArray<u8, 2048>,
    pub bg3Buffer: CArray<u8, 2048>,
    pub bg2Buffer: CArray<u8, 2048>,
    pub textEntryTiles: CArray<u8, 64>,
}

unsafe impl Sync for UnionRoomChatDisplay {}

/// `struct UnionRoomChatSprites`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnionRoomChatSprites {
    pub keyboardCursor: *mut Sprite,
    pub textEntryArrow: *mut Sprite,
    pub textEntryCursor: *mut Sprite,
    pub rButtonIcon: *mut Sprite,
    pub rButtonLabel: *mut Sprite,
    pub cursorBlinkTimer: u16,
}

unsafe impl Sync for UnionRoomChatSprites {}

/// `struct MessageWindowInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MessageWindowInfo {
    pub text: *mut u8,
    pub boxType: u8,
    pub x: u8,
    pub y: u8,
    pub letterSpacing: u8,
    pub lineSpacing: u8,
    pub hasPlaceholders: u8,
    pub useWiderBox: u8,
}

unsafe impl Sync for MessageWindowInfo {}

/// `struct SubtaskInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SubtaskInfo {
    pub idx: u16,
    pub callback: Option<unsafe extern "C" fn(*mut u8) -> u32>,
}

unsafe impl Sync for SubtaskInfo {}

/// `struct UnionRoomChatDisplay_Subtask`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnionRoomChatDisplay_Subtask {
    pub callback: Option<unsafe extern "C" fn(*mut u8) -> u32>,
    pub active: u8,
    pub state: u8,
}

unsafe impl Sync for UnionRoomChatDisplay_Subtask {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<UnionRoomChat>() == 444);
    assert!(offset_of!(UnionRoomChat, filler1) == 0);
    assert!(offset_of!(UnionRoomChat, funcId) == 4);
    assert!(offset_of!(UnionRoomChat, funcState) == 6);
    assert!(offset_of!(UnionRoomChat, filler2) == 8);
    assert!(offset_of!(UnionRoomChat, exitDelayTimer) == 10);
    assert!(offset_of!(UnionRoomChat, filler3) == 12);
    assert!(offset_of!(UnionRoomChat, linkPlayerCount) == 13);
    assert!(offset_of!(UnionRoomChat, handleInputTask) == 14);
    assert!(offset_of!(UnionRoomChat, receiveMessagesTask) == 15);
    assert!(offset_of!(UnionRoomChat, currentPage) == 16);
    assert!(offset_of!(UnionRoomChat, currentCol) == 17);
    assert!(offset_of!(UnionRoomChat, currentRow) == 18);
    assert!(offset_of!(UnionRoomChat, multiplayerId) == 19);
    assert!(offset_of!(UnionRoomChat, lastBufferCursorPos) == 20);
    assert!(offset_of!(UnionRoomChat, bufferCursorPos) == 21);
    assert!(offset_of!(UnionRoomChat, receivedPlayerIndex) == 22);
    assert!(offset_of!(UnionRoomChat, exitType) == 23);
    assert!(offset_of!(UnionRoomChat, changedRegisteredTexts) == 24);
    assert!(offset_of!(UnionRoomChat, afterSaveTimer) == 25);
    assert!(offset_of!(UnionRoomChat, messageEntryBuffer) == 26);
    assert!(offset_of!(UnionRoomChat, receivedMessage) == 57);
    assert!(offset_of!(UnionRoomChat, hostName) == 121);
    assert!(offset_of!(UnionRoomChat, registeredTexts) == 185);
    assert!(offset_of!(UnionRoomChat, filler4) == 395);
    assert!(offset_of!(UnionRoomChat, sendMessageBuffer) == 400);
    assert!(offset_of!(UnionRoomChat, tryQuitAgainTimer) == 440);
    assert!(size_of::<UnionRoomChatDisplay>() == 8552);
    assert!(offset_of!(UnionRoomChatDisplay, subtasks) == 0);
    assert!(offset_of!(UnionRoomChatDisplay, yesNoMenuWindowId) == 24);
    assert!(offset_of!(UnionRoomChatDisplay, currLine) == 26);
    assert!(offset_of!(UnionRoomChatDisplay, scrollCount) == 28);
    assert!(offset_of!(UnionRoomChatDisplay, messageWindowId) == 30);
    assert!(offset_of!(UnionRoomChatDisplay, bg1hofs) == 32);
    assert!(offset_of!(UnionRoomChatDisplay, expandedPlaceholdersBuffer) == 34);
    assert!(offset_of!(UnionRoomChatDisplay, bg0Buffer) == 296);
    assert!(offset_of!(UnionRoomChatDisplay, bg1Buffer) == 2344);
    assert!(offset_of!(UnionRoomChatDisplay, bg3Buffer) == 4392);
    assert!(offset_of!(UnionRoomChatDisplay, bg2Buffer) == 6440);
    assert!(offset_of!(UnionRoomChatDisplay, textEntryTiles) == 8488);
    assert!(size_of::<UnionRoomChatSprites>() == 24);
    assert!(offset_of!(UnionRoomChatSprites, keyboardCursor) == 0);
    assert!(offset_of!(UnionRoomChatSprites, textEntryArrow) == 4);
    assert!(offset_of!(UnionRoomChatSprites, textEntryCursor) == 8);
    assert!(offset_of!(UnionRoomChatSprites, rButtonIcon) == 12);
    assert!(offset_of!(UnionRoomChatSprites, rButtonLabel) == 16);
    assert!(offset_of!(UnionRoomChatSprites, cursorBlinkTimer) == 20);
    assert!(size_of::<MessageWindowInfo>() == 12);
    assert!(offset_of!(MessageWindowInfo, text) == 0);
    assert!(offset_of!(MessageWindowInfo, boxType) == 4);
    assert!(offset_of!(MessageWindowInfo, x) == 5);
    assert!(offset_of!(MessageWindowInfo, y) == 6);
    assert!(offset_of!(MessageWindowInfo, letterSpacing) == 7);
    assert!(offset_of!(MessageWindowInfo, lineSpacing) == 8);
    assert!(offset_of!(MessageWindowInfo, hasPlaceholders) == 9);
    assert!(offset_of!(MessageWindowInfo, useWiderBox) == 10);
    assert!(size_of::<SubtaskInfo>() == 8);
    assert!(offset_of!(SubtaskInfo, idx) == 0);
    assert!(offset_of!(SubtaskInfo, callback) == 4);
    assert!(size_of::<UnionRoomChatDisplay_Subtask>() == 8);
    assert!(offset_of!(UnionRoomChatDisplay_Subtask, callback) == 0);
    assert!(offset_of!(UnionRoomChatDisplay_Subtask, active) == 4);
    assert!(offset_of!(UnionRoomChatDisplay_Subtask, state) == 5);
};

const CHATDISPLAY_FUNC_ASK_CONFIRM_LEADER_LEAVE: u16 = 20;
const CHATDISPLAY_FUNC_ASK_OVERWRITE_SAVE: u16 = 15;
const CHATDISPLAY_FUNC_ASK_QUIT_CHATTING: u16 = 6;
const CHATDISPLAY_FUNC_ASK_REGISTER_TEXT: u16 = 9;
const CHATDISPLAY_FUNC_ASK_SAVE: u16 = 14;
const CHATDISPLAY_FUNC_CANCEL_REGISTER: u16 = 10;
const CHATDISPLAY_FUNC_CURSOR_BLINK: u16 = 2;
const CHATDISPLAY_FUNC_DESTROY_YESNO: u16 = 7;
const CHATDISPLAY_FUNC_HIDE_KB_SWAP_MENU: u16 = 4;
const CHATDISPLAY_FUNC_LOAD_GFX: u16 = 0;
const CHATDISPLAY_FUNC_MOVE_KB_CURSOR: u16 = 1;
const CHATDISPLAY_FUNC_PRINT_EXITING_CHAT: u16 = 18;
const CHATDISPLAY_FUNC_PRINT_INPUT_TEXT: u16 = 13;
const CHATDISPLAY_FUNC_PRINT_LEADER_LEFT: u16 = 19;
const CHATDISPLAY_FUNC_PRINT_SAVED_GAME: u16 = 17;
const CHATDISPLAY_FUNC_PRINT_SAVING: u16 = 16;
const CHATDISPLAY_FUNC_RETURN_TO_KB: u16 = 11;
const CHATDISPLAY_FUNC_SCROLL_CHAT: u16 = 12;
const CHATDISPLAY_FUNC_SHOW_KB_SWAP_MENU: u16 = 3;
const CHATDISPLAY_FUNC_SWITCH_PAGES: u16 = 5;
const CHATDISPLAY_FUNC_UPDATE_MSG: u16 = 8;
const CHAT_EXIT_DISBANDED: u8 = 3;
const CHAT_EXIT_DROPPED: u8 = 2;
const CHAT_EXIT_NONE: u8 = 0;
const CHAT_EXIT_ONLY_LEADER: u8 = 1;
const CHAT_FUNC_ASK_QUIT: u16 = 3;
const CHAT_FUNC_DISBANDED: u16 = 8;
const CHAT_FUNC_DROP: u16 = 7;
const CHAT_FUNC_EXIT: u16 = 6;
const CHAT_FUNC_HANDLE_INPUT: u16 = 1;
const CHAT_FUNC_JOIN: u16 = 0;
const CHAT_FUNC_REGISTER: u16 = 5;
const CHAT_FUNC_SAVE_AND_EXIT: u16 = 9;
const CHAT_FUNC_SEND: u16 = 4;
const CHAT_FUNC_SWITCH: u16 = 2;
const CHAT_MESSAGE_CHAT: u8 = 1;
const CHAT_MESSAGE_DISBAND: u8 = 5;
const CHAT_MESSAGE_DROP: u8 = 4;
const CHAT_MESSAGE_JOIN: u8 = 2;
const CHAT_MESSAGE_LEAVE: u8 = 3;
const CHAT_MESSAGE_NONE: u8 = 0;
const KEYBOARD_HOFS_END: i16 = 56;
const MAX_MESSAGE_LENGTH: u8 = 15;
const PALTAG_INTERFACE: u16 = 0;
const STDMESSAGE_ASK_OVERWRITE: i32 = 7;
const STDMESSAGE_ASK_SAVE: i32 = 6;
const STDMESSAGE_EXITING_CHAT: i32 = 4;
const STDMESSAGE_INPUT_TEXT: i32 = 3;
const STDMESSAGE_LEADER_LEFT: i32 = 5;
const STDMESSAGE_QUIT_CHATTING: i32 = 0;
const STDMESSAGE_REGISTER_WHERE: i32 = 1;
const STDMESSAGE_SAVED_THE_GAME: i32 = 9;
const STDMESSAGE_SAVING_NO_OFF: i32 = 8;
const STDMESSAGE_WARN_LEADER_LEAVE: i32 = 10;
const UNION_ROOM_KB_PAGE_EMOJI: u8 = 2;
const UNION_ROOM_KB_PAGE_REGISTER: u8 = 3;
const WIN_CHAT_HISTORY: u8 = 0;
const WIN_KEYBOARD: u8 = 2;
const WIN_SWAP_MENU: u8 = 3;
const WIN_TEXT_ENTRY: u8 = 1;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::union_room_chat::sBgTemplates).cast());
static sCaseToggleTable: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::union_room_chat::sCaseToggleTable).cast());
static sChatMainFunctions: Table<CArray<Option<unsafe extern "C" fn()>, 10>> =
    Table((&raw const crate::data::union_room_chat::sChatMainFunctions).cast());
static sChatMessagesWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::union_room_chat::sChatMessagesWindow_Pal).cast());
static sDisplayStdMessages: Table<CArray<MessageWindowInfo, 11>> =
    Table((&raw const crate::data::union_room_chat::sDisplayStdMessages).cast());
static sDisplaySubtasks: Table<CArray<SubtaskInfo, 21>> =
    Table((&raw const crate::data::union_room_chat::sDisplaySubtasks).cast());
static sKeyboardPageMaxRow: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::union_room_chat::sKeyboardPageMaxRow).cast());
static sKeyboardPageTitleTexts: Table<CArray<MenuAction, 5>> =
    Table((&raw const crate::data::union_room_chat::sKeyboardPageTitleTexts).cast());
static sSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::union_room_chat::sSpritePalette).cast());
static sSpriteSheets: Table<CArray<CompressedSpriteSheet, 5>> =
    Table((&raw const crate::data::union_room_chat::sSpriteSheets).cast());
static sSpriteTemplate_KeyboardCursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::union_room_chat::sSpriteTemplate_KeyboardCursor).cast());
static sSpriteTemplate_RButtonIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::union_room_chat::sSpriteTemplate_RButtonIcon).cast());
static sSpriteTemplate_RButtonLabels: Table<SpriteTemplate> =
    Table((&raw const crate::data::union_room_chat::sSpriteTemplate_RButtonLabels).cast());
static sSpriteTemplate_TextEntryArrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::union_room_chat::sSpriteTemplate_TextEntryArrow).cast());
static sSpriteTemplate_TextEntryCursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::union_room_chat::sSpriteTemplate_TextEntryCursor).cast());
static sText_Ellipsis: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::union_room_chat::sText_Ellipsis).cast());
static sUnionRoomChatInterfacePal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::union_room_chat::sUnionRoomChatInterfacePal).cast());
static sUnionRoomKeyboardText: Table<CArray<CArray<*mut u8, 10>, 3>> =
    Table((&raw const crate::data::union_room_chat::sUnionRoomKeyboardText).cast());
static sUnusedPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::union_room_chat::sUnusedPalette).cast());
static sWinTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::union_room_chat::sWinTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sChat: *mut UnionRoomChat = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDisplay: *mut UnionRoomChatDisplay = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSprites: *mut UnionRoomChatSprites = null_mut();

unsafe extern "C" {
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gKeyRepeatStartDelay: u16;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gSprites: CArray<Sprite, 65>;
    static gStandardMenuPalette: CArray<u16, 0>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Battle: CArray<u8, 0>;
    static gText_ByeBye: CArray<u8, 0>;
    static gText_F700JoinedChat: CArray<u8, 0>;
    static gText_F700LeftChat: CArray<u8, 0>;
    static gText_Hello: CArray<u8, 0>;
    static gText_Lets: CArray<u8, 0>;
    static gText_No: CArray<u8, 0>;
    static gText_Ok: CArray<u8, 0>;
    static gText_Pokemon2: CArray<u8, 0>;
    static gText_Sorry: CArray<u8, 0>;
    static gText_ThankYou: CArray<u8, 0>;
    static gText_Trade: CArray<u8, 0>;
    static gText_YaySmileEmoji: CArray<u8, 0>;
    static gText_Yes: CArray<u8, 0>;
    static gUnionRoomChat_Background_Gfx: CArray<u32, 0>;
    static gUnionRoomChat_Background_Pal: CArray<u16, 0>;
    static gUnionRoomChat_Background_Tilemap: CArray<u32, 0>;
    static gUnionRoomChat_InputText_Pal: CArray<u16, 0>;
    static gUnionRoomChat_Keyboard_Gfx: CArray<u32, 0>;
    static gUnionRoomChat_Keyboard_Pal: CArray<u16, 0>;
    static gUnionRoomChat_Keyboard_Tilemap: CArray<u32, 0>;
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
    fn AddTextPrinterParameterized5(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a7: u8,
        a8: u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearContinueGameWarpStatus2();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyTask(a0: u8);
    fn DrawTextBorderInner(a0: u8, a1: u16, a2: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitMenuNormal(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_MoveCursor(a0: i8) -> u8;
    fn Menu_ProcessInput() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuActionTextsAtPos(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut MenuAction,
    );
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RfuSetNormalDisconnectMode();
    fn Rfu_DisconnectPlayerById(a0: u32);
    fn Rfu_IsPlayerExchangeActive() -> u32;
    fn Rfu_StopPartnerSearch();
    fn RunTasks();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScrollWindow(a0: u8, a1: u8, a2: u8, a3: u8);
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetUnionRoomChatPlayerData(a0: u32);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN_Multibyte(a0: *mut u8, a1: *mut u8, a2: u32) -> *mut u8;
    fn StringLength_Multibyte(a0: *mut u8) -> u32;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnterUnionRoomChat() {
    sChat = Alloc(444) as *mut UnionRoomChat;
    InitUnionRoomChat(sChat);
    gKeyRepeatStartDelay = 20;
    SetVBlankCallback(None);
    SetMainCallback2(Some(CB2_LoadInterface));
}
pub(crate) unsafe extern "C" fn InitUnionRoomChat(chat: *mut UnionRoomChat) {
    let mut i: i32 = 0;
    (*chat).funcId = CHAT_FUNC_JOIN;
    (*chat).funcState = 0;
    (*chat).currentPage = 0;
    (*chat).currentCol = 0;
    (*chat).currentRow = 0;
    (*chat).lastBufferCursorPos = 0;
    (*chat).bufferCursorPos = 0;
    (*chat).receivedPlayerIndex = 0;
    (*chat).messageEntryBuffer[0] = EOS;
    (*chat).linkPlayerCount = GetLinkPlayerCount();
    (*chat).multiplayerId = GetMultiplayerId();
    (*chat).exitType = CHAT_EXIT_NONE;
    (*chat).changedRegisteredTexts = FALSE;
    PrepareSendBuffer_Null((*chat).sendMessageBuffer.as_mut_ptr());
    i = 0;
    while i < UNION_ROOM_KB_ROW_COUNT {
        StringCopy(
            (*chat).registeredTexts[i].as_mut_ptr(),
            (*gSaveBlock1Ptr).registeredTexts[i].as_mut_ptr(),
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FreeUnionRoomChat() {
    DestroyTask((*sChat).handleInputTask);
    DestroyTask((*sChat).receiveMessagesTask);
    Free(sChat as *mut c_void);
}
pub(crate) unsafe extern "C" fn CB2_LoadInterface() {
    match gMain.state {
        0 => {
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            TryAllocDisplay();
            gMain.state += 1;
        }
        1 => {
            RunDisplaySubtasks();
            if IsDisplaySubtask0Active() == 0 {
                BlendPalettes(PALETTES_ALL, 16, 0);
                BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
                SetVBlankCallback(Some(VBlankCB_UnionRoomChatMain));
                gMain.state += 1;
            }
        }
        2 => {
            UpdatePaletteFade();
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_UnionRoomChatMain));
                (*sChat).handleInputTask = CreateTask(Some(Task_HandlePlayerInput), 8);
                (*sChat).receiveMessagesTask = CreateTask(Some(Task_ReceiveChatMessage), 7);
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(232, 150);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_UnionRoomChatMain() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn CB2_UnionRoomChatMain() {
    RunTasks();
    RunDisplaySubtasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn Task_HandlePlayerInput(taskId: u8) {
    match (*sChat).exitType {
        CHAT_EXIT_ONLY_LEADER => {
            SetChatFunction(CHAT_FUNC_EXIT);
            (*sChat).exitType = CHAT_EXIT_NONE;
        }
        CHAT_EXIT_DROPPED => {
            SetChatFunction(CHAT_FUNC_DROP);
            (*sChat).exitType = CHAT_EXIT_NONE;
        }
        CHAT_EXIT_DISBANDED => {
            SetChatFunction(CHAT_FUNC_DISBANDED);
            (*sChat).exitType = CHAT_EXIT_NONE;
        }
        _ => {}
    }
    sChatMainFunctions[(*sChat).funcId].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn Chat_Join() {
    'l1: {
        let sw1: u16 = (*sChat).funcState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PrepareSendBuffer_Join((*sChat).sendMessageBuffer.as_mut_ptr());
            (*sChat).funcState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if IsLinkTaskFinished() != 0 && Rfu_IsPlayerExchangeActive() == 0 {
                if SendBlock(
                    0,
                    (*sChat).sendMessageBuffer.as_mut_ptr() as *mut c_void,
                    40,
                ) != 0
                {
                    (*sChat).funcState += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if IsLinkTaskFinished() != 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_HandleInput() {
    let mut updateMsgActive: u8 = 0;
    let mut cursorBlinkActive: u8 = 0;
    match (*sChat).funcState {
        0 => {
            if gMain.newKeys as i32 & START_BUTTON != 0 {
                if (*sChat).bufferCursorPos != 0 {
                    SetChatFunction(CHAT_FUNC_SEND);
                }
            } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
                SetChatFunction(CHAT_FUNC_SWITCH);
            } else if gMain.newAndRepeatedKeys as i32 & B_BUTTON != 0 {
                if (*sChat).bufferCursorPos != 0 {
                    DeleteLastMessageCharacter();
                    StartDisplaySubtask(CHATDISPLAY_FUNC_UPDATE_MSG, 0);
                    (*sChat).funcState = 1;
                } else {
                    SetChatFunction(CHAT_FUNC_ASK_QUIT);
                }
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                AppendTextToMessage();
                StartDisplaySubtask(CHATDISPLAY_FUNC_UPDATE_MSG, 0);
                StartDisplaySubtask(CHATDISPLAY_FUNC_CURSOR_BLINK, 1);
                (*sChat).funcState = 1;
            } else if gMain.newKeys as i32 & R_BUTTON != 0 {
                if (*sChat).currentPage != UNION_ROOM_KB_PAGE_REGISTER {
                    SwitchCaseOfLastMessageCharacter();
                    StartDisplaySubtask(CHATDISPLAY_FUNC_UPDATE_MSG, 0);
                    (*sChat).funcState = 1;
                } else {
                    SetChatFunction(CHAT_FUNC_REGISTER);
                }
            } else if HandleDPadInput() != 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_MOVE_KB_CURSOR, 0);
                (*sChat).funcState = 1;
            }
        }
        1 => {
            updateMsgActive = IsDisplaySubtaskActive(0);
            cursorBlinkActive = IsDisplaySubtaskActive(1);
            if updateMsgActive == 0 && cursorBlinkActive == 0 {
                (*sChat).funcState = 0;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_Switch() {
    let mut input: i16 = 0;
    let mut shouldSwitchPages: u32 = 0;
    match (*sChat).funcState {
        0 => {
            StartDisplaySubtask(CHATDISPLAY_FUNC_SHOW_KB_SWAP_MENU, 0);
            (*sChat).funcState += 1;
        }
        1 => {
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState += 1;
            }
        }
        2 => {
            input = Menu_ProcessInput() as i16;
            match input {
                -2 => {
                    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
                        PlaySE(SE_SELECT);
                        Menu_MoveCursor(1);
                    }
                    return;
                }
                -1 => {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_HIDE_KB_SWAP_MENU, 0);
                    (*sChat).funcState = 3;
                    return;
                }
                _ => {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_HIDE_KB_SWAP_MENU, 0);
                    shouldSwitchPages = TRUE as u32;
                    if (*sChat).currentPage as i16 == input
                        || input > UNION_ROOM_KB_PAGE_REGISTER as i16
                    {
                        shouldSwitchPages = FALSE as u32;
                    }
                }
            }
            if shouldSwitchPages == 0 {
                (*sChat).funcState = 3;
                return;
            }
            (*sChat).currentCol = 0;
            (*sChat).currentRow = 0;
            StartDisplaySubtask(CHATDISPLAY_FUNC_SWITCH_PAGES, 1);
            (*sChat).currentPage = input as u8;
            (*sChat).funcState = 4;
        }
        3 => {
            if IsDisplaySubtaskActive(0) == 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
            }
        }
        4 => {
            if IsDisplaySubtaskActive(0) == 0 && IsDisplaySubtaskActive(1) == 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_AskQuitChatting() {
    let mut input: i8 = 0;
    match (*sChat).funcState {
        0 => {
            StartDisplaySubtask(CHATDISPLAY_FUNC_ASK_QUIT_CHATTING, 0);
            (*sChat).funcState = 1;
        }
        1 => {
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState = 2;
            }
        }
        2 => {
            input = ProcessMenuInput();
            match input {
                MENU_B_PRESSED | 1 => {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                    (*sChat).funcState = 3;
                }
                0 => {
                    if (*sChat).multiplayerId == 0 {
                        PrepareSendBuffer_Disband((*sChat).sendMessageBuffer.as_mut_ptr());
                        StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                        (*sChat).funcState = 9;
                    } else {
                        PrepareSendBuffer_Leave((*sChat).sendMessageBuffer.as_mut_ptr());
                        (*sChat).funcState = 4;
                    }
                }
                _ => {}
            }
        }
        3 => {
            if IsDisplaySubtaskActive(0) == 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
            }
        }
        9 => {
            if IsDisplaySubtaskActive(0) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_ASK_CONFIRM_LEADER_LEAVE, 0);
                (*sChat).funcState = 10;
            }
        }
        10 => {
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState = 8;
            }
        }
        8 => {
            input = ProcessMenuInput();
            match input {
                MENU_B_PRESSED | 1 => {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                    (*sChat).funcState = 3;
                }
                0 => {
                    Rfu_StopPartnerSearch();
                    PrepareSendBuffer_Disband((*sChat).sendMessageBuffer.as_mut_ptr());
                    (*sChat).funcState = 4;
                    (*sChat).tryQuitAgainTimer = 0;
                }
                _ => {}
            }
        }
        4 => {
            if IsLinkTaskFinished() != 0
                && Rfu_IsPlayerExchangeActive() == 0
                && SendBlock(
                    0,
                    (*sChat).sendMessageBuffer.as_mut_ptr() as *mut c_void,
                    40,
                ) != 0
            {
                if (*sChat).multiplayerId == 0 {
                    (*sChat).funcState = 6;
                } else {
                    (*sChat).funcState = 5;
                }
            }
            if gReceivedRemoteLinkPlayers == 0 {
                SetChatFunction(CHAT_FUNC_SAVE_AND_EXIT);
            }
        }
        5 => {
            if gReceivedRemoteLinkPlayers == 0 {
                SetChatFunction(CHAT_FUNC_SAVE_AND_EXIT);
            } else if ({
                (*sChat).tryQuitAgainTimer += 1;
                (*sChat).tryQuitAgainTimer
            }) > 300
            {
                (*sChat).tryQuitAgainTimer = 0;
                (*sChat).funcState = 4;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_Exit() {
    match (*sChat).funcState {
        0 => {
            if FuncIsActiveTask(Some(Task_ReceiveChatMessage)) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                (*sChat).funcState += 1;
            }
        }
        1 => {
            if IsDisplaySubtaskActive(0) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_PRINT_EXITING_CHAT, 0);
                (*sChat).funcState += 1;
            }
        }
        2 => {
            if IsDisplaySubtaskActive(0) == 0 {
                PrepareSendBuffer_Drop((*sChat).sendMessageBuffer.as_mut_ptr());
                (*sChat).funcState += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0
                && Rfu_IsPlayerExchangeActive() == 0
                && SendBlock(
                    0,
                    (*sChat).sendMessageBuffer.as_mut_ptr() as *mut c_void,
                    40,
                ) != 0
            {
                (*sChat).funcState += 1;
            }
        }
        4 => {
            if GetBlockReceivedStatus() as i32 & 1 != 0 && Rfu_IsPlayerExchangeActive() == 0 {
                (*sChat).funcState += 1;
            }
        }
        5 => {
            if IsLinkTaskFinished() != 0 && Rfu_IsPlayerExchangeActive() == 0 {
                SetCloseLinkCallback();
                (*sChat).exitDelayTimer = 0;
                (*sChat).funcState += 1;
            }
        }
        6 => {
            if (*sChat).exitDelayTimer < 150 {
                (*sChat).exitDelayTimer += 1;
            }
            if gReceivedRemoteLinkPlayers == 0 {
                (*sChat).funcState += 1;
            }
        }
        7 => {
            if (*sChat).exitDelayTimer >= 150 {
                SetChatFunction(CHAT_FUNC_SAVE_AND_EXIT);
            } else {
                (*sChat).exitDelayTimer += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_Drop() {
    match (*sChat).funcState {
        0 => {
            if FuncIsActiveTask(Some(Task_ReceiveChatMessage)) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                (*sChat).funcState += 1;
            }
        }
        1 => {
            if IsDisplaySubtaskActive(0) == 0
                && IsLinkTaskFinished() != 0
                && Rfu_IsPlayerExchangeActive() == 0
            {
                SetCloseLinkCallback();
                (*sChat).exitDelayTimer = 0;
                (*sChat).funcState += 1;
            }
        }
        2 => {
            if (*sChat).exitDelayTimer < 150 {
                (*sChat).exitDelayTimer += 1;
            }
            if gReceivedRemoteLinkPlayers == 0 {
                (*sChat).funcState += 1;
            }
        }
        3 => {
            if (*sChat).exitDelayTimer >= 150 {
                SetChatFunction(CHAT_FUNC_SAVE_AND_EXIT);
            } else {
                (*sChat).exitDelayTimer += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_Disbanded() {
    match (*sChat).funcState {
        0 => {
            if FuncIsActiveTask(Some(Task_ReceiveChatMessage)) == 0 {
                if (*sChat).multiplayerId != 0 {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                }
                (*sChat).funcState += 1;
            }
        }
        1 => {
            if IsDisplaySubtaskActive(0) == 0 {
                if (*sChat).multiplayerId != 0 {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_PRINT_LEADER_LEFT, 0);
                }
                (*sChat).funcState += 1;
            }
        }
        2 => {
            if IsDisplaySubtaskActive(0) != TRUE
                && IsLinkTaskFinished() != 0
                && Rfu_IsPlayerExchangeActive() == 0
            {
                SetCloseLinkCallback();
                (*sChat).exitDelayTimer = 0;
                (*sChat).funcState += 1;
            }
        }
        3 => {
            if (*sChat).exitDelayTimer < 150 {
                (*sChat).exitDelayTimer += 1;
            }
            if gReceivedRemoteLinkPlayers == 0 {
                (*sChat).funcState += 1;
            }
        }
        4 => {
            if (*sChat).exitDelayTimer >= 150 {
                SetChatFunction(CHAT_FUNC_SAVE_AND_EXIT);
            } else {
                (*sChat).exitDelayTimer += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_SendMessage() {
    'l1: {
        let sw1: u16 = (*sChat).funcState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if gReceivedRemoteLinkPlayers == 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
                break 'l1;
            }
            PrepareSendBuffer_Chat((*sChat).sendMessageBuffer.as_mut_ptr());
            (*sChat).funcState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if IsLinkTaskFinished() == TRUE
                && Rfu_IsPlayerExchangeActive() == 0
                && SendBlock(
                    0,
                    (*sChat).sendMessageBuffer.as_mut_ptr() as *mut c_void,
                    40,
                ) != 0
            {
                (*sChat).funcState += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            ResetMessageEntryBuffer();
            StartDisplaySubtask(CHATDISPLAY_FUNC_UPDATE_MSG, 0);
            (*sChat).funcState += 1;
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if IsLinkTaskFinished() != 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_Register() {
    match (*sChat).funcState {
        0 => {
            if ChatMessageIsNotEmpty() != 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_ASK_REGISTER_TEXT, 0);
                (*sChat).funcState = 2;
            } else {
                StartDisplaySubtask(CHATDISPLAY_FUNC_PRINT_INPUT_TEXT, 0);
                (*sChat).funcState = 5;
            }
        }
        1 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                RegisterTextAtRow();
                StartDisplaySubtask(CHATDISPLAY_FUNC_RETURN_TO_KB, 0);
                (*sChat).funcState = 3;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_CANCEL_REGISTER, 0);
                (*sChat).funcState = 4;
            } else if HandleDPadInput() != 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_MOVE_KB_CURSOR, 0);
                (*sChat).funcState = 2;
            }
        }
        2 => {
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState = 1;
            }
        }
        3 => {
            if IsDisplaySubtaskActive(0) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_CANCEL_REGISTER, 0);
                (*sChat).funcState = 4;
            }
        }
        4 => {
            if IsDisplaySubtaskActive(0) == 0 {
                SetChatFunction(CHAT_FUNC_HANDLE_INPUT);
            }
        }
        5 => {
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState = 6;
            }
        }
        6 => {
            if gMain.newKeys as i32 & 3 != 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                (*sChat).funcState = 4;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Chat_SaveAndExit() {
    let mut input: i8 = 0;
    match (*sChat).funcState {
        0 => {
            if (*sChat).changedRegisteredTexts == 0 {
                (*sChat).funcState = 12;
            } else {
                StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                (*sChat).funcState = 1;
            }
        }
        1 => {
            if IsDisplaySubtaskActive(0) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_ASK_SAVE, 0);
                (*sChat).funcState = 2;
            }
        }
        2 => {
            input = ProcessMenuInput();
            match input {
                MENU_B_PRESSED | 1 => {
                    (*sChat).funcState = 12;
                }
                0 => {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                    (*sChat).funcState = 3;
                }
                _ => {}
            }
        }
        3 => {
            if IsDisplaySubtaskActive(0) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_ASK_OVERWRITE_SAVE, 0);
                (*sChat).funcState = 4;
            }
        }
        4 => {
            if IsDisplaySubtaskActive(0) == 0 {
                (*sChat).funcState = 5;
            }
        }
        5 => {
            input = ProcessMenuInput();
            match input {
                MENU_B_PRESSED | 1 => {
                    (*sChat).funcState = 12;
                }
                0 => {
                    StartDisplaySubtask(CHATDISPLAY_FUNC_DESTROY_YESNO, 0);
                    (*sChat).funcState = 6;
                }
                _ => {}
            }
        }
        6 => {
            if IsDisplaySubtaskActive(0) == 0 {
                StartDisplaySubtask(CHATDISPLAY_FUNC_PRINT_SAVING, 0);
                SaveRegisteredTexts();
                (*sChat).funcState = 7;
            }
        }
        7 => {
            if IsDisplaySubtaskActive(0) == 0 {
                SetContinueGameWarpStatusToDynamicWarp();
                TrySavingData(SAVE_NORMAL);
                (*sChat).funcState = 8;
            }
        }
        8 => {
            StartDisplaySubtask(CHATDISPLAY_FUNC_PRINT_SAVED_GAME, 0);
            (*sChat).funcState = 9;
        }
        9 => {
            if IsDisplaySubtaskActive(0) == 0 {
                PlaySE(SE_SAVE);
                ClearContinueGameWarpStatus2();
                (*sChat).funcState = 10;
            }
        }
        10 => {
            (*sChat).afterSaveTimer = 0;
            (*sChat).funcState = 11;
        }
        11 => {
            (*sChat).afterSaveTimer += 1;
            if (*sChat).afterSaveTimer > 120 {
                (*sChat).funcState = 12;
            }
        }
        12 => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
            (*sChat).funcState = 13;
        }
        13 => {
            if gPaletteFade.active() == 0 {
                FreeDisplay();
                FreeUnionRoomChat();
                SetMainCallback2(Some(CB2_ReturnToField));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetChatFunction(funcId: u16) {
    (*sChat).funcId = funcId;
    (*sChat).funcState = 0;
}
pub(crate) unsafe extern "C" fn HandleDPadInput() -> u32 {
    'l2: {
        if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
            if (*sChat).currentRow > 0 {
                (*sChat).currentRow -= 1;
            } else {
                (*sChat).currentRow = sKeyboardPageMaxRow[(*sChat).currentPage];
            }
            break 'l2;
        }
        if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
            if (*sChat).currentRow < sKeyboardPageMaxRow[(*sChat).currentPage] {
                (*sChat).currentRow += 1;
            } else {
                (*sChat).currentRow = 0;
            }
            break 'l2;
        }
        if (*sChat).currentPage != UNION_ROOM_KB_PAGE_REGISTER {
            if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
                if (*sChat).currentCol > 0 {
                    (*sChat).currentCol -= 1;
                } else {
                    (*sChat).currentCol = 4;
                }
                break 'l2;
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
                if (*sChat).currentCol < 4 {
                    (*sChat).currentCol += 1;
                } else {
                    (*sChat).currentCol = 0;
                }
                break 'l2;
            }
        }
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn AppendTextToMessage() {
    let mut i: i32 = 0;
    let mut charsStr: *mut u8 = null_mut();
    let mut strLength: i32 = 0;
    let mut str: *mut u8 = null_mut();
    let mut buffer: CArray<u8, 21> = zeroed();
    if (*sChat).currentPage != UNION_ROOM_KB_PAGE_REGISTER {
        charsStr = sUnionRoomKeyboardText[(*sChat).currentPage][(*sChat).currentRow];
        i = 0;
        while i < (*sChat).currentCol as i32 {
            if *charsStr == CHAR_EXTRA_SYMBOL {
                charsStr = charsStr.at(1);
            }
            charsStr = charsStr.at(1);
            i += 1;
        }
        strLength = 1;
    } else {
        let mut tempStr: *mut u8 = StringCopy(
            buffer.as_mut_ptr(),
            (*sChat).registeredTexts[(*sChat).currentRow].as_mut_ptr(),
        );
        *tempStr = 0x00;
        *tempStr.at(1) = EOS;
        charsStr = buffer.as_mut_ptr();
        strLength = StringLength_Multibyte(buffer.as_mut_ptr()) as i32;
    }
    (*sChat).lastBufferCursorPos = (*sChat).bufferCursorPos;
    if charsStr.is_null() {
        return;
    }
    str = GetEndOfMessagePtr();
    while ({
        strLength -= 1;
        strLength
    }) != -1
        && (*sChat).bufferCursorPos < MAX_MESSAGE_LENGTH
    {
        if *charsStr == CHAR_EXTRA_SYMBOL {
            *str = *charsStr;
            charsStr = charsStr.at(1);
            str = str.at(1);
        }
        *str = *charsStr;
        charsStr = charsStr.at(1);
        str = str.at(1);
        (*sChat).bufferCursorPos += 1;
    }
    *str = EOS;
}
pub(crate) unsafe extern "C" fn DeleteLastMessageCharacter() {
    (*sChat).lastBufferCursorPos = (*sChat).bufferCursorPos;
    if (*sChat).bufferCursorPos != 0 {
        let mut str: *mut u8 = GetLastCharOfMessagePtr();
        *str = EOS;
        (*sChat).bufferCursorPos -= 1;
    }
}
pub(crate) unsafe extern "C" fn SwitchCaseOfLastMessageCharacter() {
    let mut str: *mut u8 = null_mut();
    let mut character: u8 = 0;
    (*sChat).lastBufferCursorPos = (*sChat).bufferCursorPos - 1;
    str = GetLastCharOfMessagePtr();
    if *str != CHAR_EXTRA_SYMBOL {
        character = sCaseToggleTable[*str];
        if character != 0 {
            *str = character;
        }
    }
}
pub(crate) unsafe extern "C" fn ChatMessageIsNotEmpty() -> u32 {
    if (*sChat).bufferCursorPos != 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn RegisterTextAtRow() {
    let mut src: *mut u8 = GetLimitedMessageStartPtr();
    StringCopy(
        (*sChat).registeredTexts[(*sChat).currentRow].as_mut_ptr(),
        src,
    );
    (*sChat).changedRegisteredTexts = TRUE;
}
pub(crate) unsafe extern "C" fn ResetMessageEntryBuffer() {
    (*sChat).messageEntryBuffer[0] = EOS;
    (*sChat).lastBufferCursorPos = MAX_MESSAGE_LENGTH;
    (*sChat).bufferCursorPos = 0;
}
pub(crate) unsafe extern "C" fn SaveRegisteredTexts() {
    let mut i: i32 = 0;
    i = 0;
    while i < UNION_ROOM_KB_ROW_COUNT {
        StringCopy(
            (*gSaveBlock1Ptr).registeredTexts[i].as_mut_ptr(),
            (*sChat).registeredTexts[i].as_mut_ptr(),
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetRegisteredTextByRow(row: i32) -> *mut u8 {
    return (*sChat).registeredTexts[row].as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetEndOfMessagePtr() -> *mut u8 {
    let mut str: *mut u8 = (*sChat).messageEntryBuffer.as_mut_ptr();
    while *str != EOS {
        str = str.at(1);
    }
    return str;
}
pub(crate) unsafe extern "C" fn GetLastCharOfMessagePtr() -> *mut u8 {
    let mut currChar: *mut u8 = (*sChat).messageEntryBuffer.as_mut_ptr();
    let mut lastChar: *mut u8 = currChar;
    while *currChar != EOS {
        lastChar = currChar;
        if *currChar == CHAR_EXTRA_SYMBOL {
            currChar = currChar.at(1);
        }
        currChar = currChar.at(1);
    }
    return lastChar;
}
pub(crate) unsafe extern "C" fn GetNumOverflowCharsInMessage() -> u16 {
    let mut str: *mut u8 = null_mut();
    let mut i: u32 = 0;
    let mut numChars: u32 = 0;
    let mut strLength: u32 = 0;
    strLength = StringLength_Multibyte((*sChat).messageEntryBuffer.as_mut_ptr());
    str = (*sChat).messageEntryBuffer.as_mut_ptr();
    numChars = 0;
    if strLength > 10 {
        strLength -= 10;
        i = 0;
        while i < strLength {
            if *str == CHAR_EXTRA_SYMBOL {
                str = str.at(1);
            }
            str = str.at(1);
            numChars += 1;
            i += 1;
        }
    }
    return numChars as u16;
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Null(mut buffer: *mut u8) {
    *buffer = CHAT_MESSAGE_NONE;
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Join(mut buffer: *mut u8) {
    *buffer = CHAT_MESSAGE_JOIN;
    StringCopy(buffer.at(1), (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    *buffer.at(9) = (*sChat).multiplayerId;
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Chat(mut buffer: *mut u8) {
    *buffer = CHAT_MESSAGE_CHAT;
    StringCopy(buffer.at(1), (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    StringCopy(buffer.at(9), (*sChat).messageEntryBuffer.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Leave(mut buffer: *mut u8) {
    *buffer = CHAT_MESSAGE_LEAVE;
    StringCopy(buffer.at(1), (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    *buffer.at(9) = (*sChat).multiplayerId;
    RfuSetNormalDisconnectMode();
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Drop(mut buffer: *mut u8) {
    *buffer = CHAT_MESSAGE_DROP;
    StringCopy(buffer.at(1), (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    *buffer.at(9) = (*sChat).multiplayerId;
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Disband(mut buffer: *mut u8) {
    *buffer = CHAT_MESSAGE_DISBAND;
    StringCopy(buffer.at(1), (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    *buffer.at(9) = (*sChat).multiplayerId;
}
pub(crate) unsafe extern "C" fn ProcessReceivedChatMessage(
    dest: *mut u8,
    mut recvMessage: *mut u8,
) -> u32 {
    let mut tempStr: *mut u8 = null_mut();
    let mut cmd: u8 = *recvMessage;
    let mut name: *mut u8 = recvMessage.at(1);
    recvMessage = name;
    recvMessage = recvMessage.at(8);
    'l1: {
        let sw1: u8 = cmd;
        let mut fall = false;
        if sw1 == CHAT_MESSAGE_JOIN {
            fall = true;
            if (*sChat).multiplayerId != *name.at(8) {
                DynamicPlaceholderTextUtil_Reset();
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, name);
                DynamicPlaceholderTextUtil_ExpandPlaceholders(
                    dest,
                    gText_F700JoinedChat.as_ptr().cast_mut(),
                );
                return TRUE as u32;
            }
            break 'l1;
        }
        if sw1 == CHAT_MESSAGE_CHAT {
            fall = true;
            tempStr = StringCopy(dest, name);
            *({
                let t2 = tempStr;
                tempStr = tempStr.at(1);
                t2
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t3 = tempStr;
                tempStr = tempStr.at(1);
                t3
            }) = EXT_CTRL_CODE_CLEAR_TO;
            *({
                let t4 = tempStr;
                tempStr = tempStr.at(1);
                t4
            }) = 42;
            *({
                let t5 = tempStr;
                tempStr = tempStr.at(1);
                t5
            }) = CHAR_COLON;
            StringCopy(tempStr, recvMessage);
            return TRUE as u32;
        }
        if sw1 == CHAT_MESSAGE_DISBAND {
            fall = true;
            StringCopy((*sChat).hostName.as_mut_ptr(), name);
        }
        if fall || sw1 == CHAT_MESSAGE_LEAVE {
            fall = true;
            if (*sChat).multiplayerId != *recvMessage {
                DynamicPlaceholderTextUtil_Reset();
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, name);
                DynamicPlaceholderTextUtil_ExpandPlaceholders(
                    dest,
                    gText_F700LeftChat.as_ptr().cast_mut(),
                );
                return TRUE as u32;
            }
            break 'l1;
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetCurrentKeyboardPage() -> u8 {
    return (*sChat).currentPage;
}
pub(crate) unsafe extern "C" fn GetCurrentKeyboardColAndRow(col: *mut u8, row: *mut u8) {
    *col = (*sChat).currentCol;
    *row = (*sChat).currentRow;
}
pub(crate) unsafe extern "C" fn GetMessageEntryBuffer() -> *mut u8 {
    return (*sChat).messageEntryBuffer.as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetLengthOfMessageEntry() -> i32 {
    let mut str: *mut u8 = GetMessageEntryBuffer();
    return StringLength_Multibyte(str) as i32;
}
pub(crate) unsafe extern "C" fn GetBufferSelectionRegion(x: *mut u32, width: *mut u32) {
    let mut diff: i32 = (*sChat).bufferCursorPos as i32 - (*sChat).lastBufferCursorPos as i32;
    if diff < 0 {
        diff *= -1;
        *x = (*sChat).bufferCursorPos as u32;
    } else {
        *x = (*sChat).lastBufferCursorPos as u32;
    }
    *width = diff as u32;
}
pub(crate) unsafe extern "C" fn GetLimitedMessageStartPtr() -> *mut u8 {
    let mut i: i32 = 0;
    let mut numChars: u16 = GetNumOverflowCharsInMessage();
    let mut str: *mut u8 = (*sChat).messageEntryBuffer.as_mut_ptr();
    i = 0;
    while i < numChars as i32 {
        if *str == CHAR_EXTRA_SYMBOL {
            str = str.at(1);
        }
        str = str.at(1);
        i += 1;
    }
    return str;
}
pub(crate) unsafe extern "C" fn GetLimitedMessageStartPos() -> u16 {
    let mut count: u16 = 0;
    let mut i: u32 = 0;
    let mut numChars: u16 = GetNumOverflowCharsInMessage();
    let mut str: *mut u8 = (*sChat).messageEntryBuffer.as_mut_ptr();
    count = 0;
    i = 0;
    while i < numChars as u32 {
        if *str == CHAR_EXTRA_SYMBOL {
            str = str.at(1);
        }
        str = str.at(1);
        count += 1;
        i += 1;
    }
    return count;
}
pub(crate) unsafe extern "C" fn GetLastReceivedMessage() -> *mut u8 {
    return (*sChat).receivedMessage.as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetReceivedPlayerIndex() -> u8 {
    return (*sChat).receivedPlayerIndex;
}
pub(crate) unsafe extern "C" fn GetTextEntryCursorPosition() -> i32 {
    return (*sChat).bufferCursorPos as i32;
}
pub(crate) unsafe extern "C" fn GetShouldShowCaseToggleIcon() -> i32 {
    let mut str: *mut u8 = GetLastCharOfMessagePtr();
    let mut character: u32 = *str as u32;
    if character > EOS as u32
        || sCaseToggleTable[character] as u32 == character
        || sCaseToggleTable[character] == CHAR_SPACE
    {
        return 3;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetChatHostName() -> *mut u8 {
    return (*sChat).hostName.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitUnionRoomChatRegisteredTexts() {
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[0].as_mut_ptr(),
        gText_Hello.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[1].as_mut_ptr(),
        gText_Pokemon2.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[2].as_mut_ptr(),
        gText_Trade.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[3].as_mut_ptr(),
        gText_Battle.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[4].as_mut_ptr(),
        gText_Lets.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[5].as_mut_ptr(),
        gText_Ok.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[6].as_mut_ptr(),
        gText_Sorry.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[7].as_mut_ptr(),
        gText_YaySmileEmoji.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[8].as_mut_ptr(),
        gText_ThankYou.as_ptr().cast_mut(),
    );
    StringCopy(
        (*gSaveBlock1Ptr).registeredTexts[9].as_mut_ptr(),
        gText_ByeBye.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn Task_ReceiveChatMessage(taskId: u8) {
    let mut buffer: *mut u8 = null_mut();
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if gReceivedRemoteLinkPlayers == 0 {
                DestroyTask(taskId);
                return;
            }
            *data = 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            *data.at(4) = GetLinkPlayerCount() as i16;
            if (*sChat).linkPlayerCount as i16 != *data.at(4) {
                *data = 2;
                (*sChat).linkPlayerCount = *data.at(4) as u8;
                return;
            }
            *data.at(3) = GetBlockReceivedStatus() as i16;
            if *data.at(3) == 0 && Rfu_IsPlayerExchangeActive() != 0 {
                return;
            }
            *data.at(1) = 0;
            *data = 3;
        }
        if fall || sw1 == 3 {
            fall = true;
            while *data.at(1) < MAX_RFU_PLAYERS as i16
                && shr_i32(*data.at(3) as i32, *data.at(1) as u32) & 1 == 0
            {
                *data.at(1) += 1;
            }
            if *data.at(1) == MAX_RFU_PLAYERS as i16 {
                *data = 1;
                return;
            }
            *data.at(2) = *data.at(1);
            ResetBlockReceivedFlag(*data.at(2) as u8);
            buffer = gBlockRecvBuffer[*data.at(1)].as_mut_ptr() as *mut u8;
            match *buffer {
                CHAT_MESSAGE_JOIN => {
                    *data.at(5) = 3;
                }
                CHAT_MESSAGE_LEAVE => {
                    *data.at(5) = 4;
                }
                CHAT_MESSAGE_DROP => {
                    *data.at(5) = 5;
                }
                CHAT_MESSAGE_DISBAND => {
                    *data.at(5) = 6;
                }
                _ => {
                    *data.at(5) = 3;
                }
            }
            if ProcessReceivedChatMessage(
                (*sChat).receivedMessage.as_mut_ptr(),
                gBlockRecvBuffer[*data.at(1)].as_mut_ptr() as *mut u8,
            ) != 0
            {
                (*sChat).receivedPlayerIndex = *data.at(1) as u8;
                StartDisplaySubtask(CHATDISPLAY_FUNC_SCROLL_CHAT, 2);
                *data = 7;
            } else {
                *data = *data.at(5);
            }
            *data.at(1) += 1;
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            if IsDisplaySubtaskActive(2) == 0 {
                *data = *data.at(5);
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if (*sChat).multiplayerId == 0 && *data.at(2) != 0 {
                if GetLinkPlayerCount() == 2 {
                    Rfu_StopPartnerSearch();
                    (*sChat).exitType = CHAT_EXIT_ONLY_LEADER;
                    DestroyTask(taskId);
                    return;
                }
                Rfu_DisconnectPlayerById(*data.at(2) as u32);
            }
            *data = 3;
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if (*sChat).multiplayerId != 0 {
                (*sChat).exitType = CHAT_EXIT_DROPPED;
            }
            DestroyTask(taskId);
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            (*sChat).exitType = CHAT_EXIT_DISBANDED;
            DestroyTask(taskId);
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if Rfu_IsPlayerExchangeActive() == 0 {
                if (*sChat).multiplayerId == 0 {
                    SetUnionRoomChatPlayerData((*sChat).linkPlayerCount as u32);
                }
                *data = 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn TryAllocDisplay() -> u8 {
    sDisplay = Alloc(8552) as *mut UnionRoomChatDisplay;
    if !sDisplay.is_null() && TryAllocSprites() != 0 {
        ResetBgsAndClearDma3BusyFlags(0);
        InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
        InitWindows(sWinTemplates.as_ptr().cast_mut());
        ResetTempTileDataBuffers();
        InitScanlineEffect();
        InitDisplay(sDisplay);
        ResetDisplaySubtasks();
        StartDisplaySubtask(CHATDISPLAY_FUNC_LOAD_GFX, 0);
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsDisplaySubtask0Active() -> u32 {
    return IsDisplaySubtaskActive(0) as u32;
}
pub(crate) unsafe extern "C" fn FreeDisplay() {
    FreeSprites();
    if !sDisplay.is_null() {
        Free(sDisplay as *mut c_void);
        sDisplay = null_mut();
    }
    FreeAllWindowBuffers();
    gScanlineEffect.state = 3;
}
pub(crate) unsafe extern "C" fn InitDisplay(display: *mut UnionRoomChatDisplay) {
    (*display).yesNoMenuWindowId = WINDOW_NONE as u16;
    (*display).messageWindowId = WINDOW_NONE as u16;
    (*display).currLine = 0;
}
pub(crate) unsafe extern "C" fn ResetDisplaySubtasks() {
    let mut i: i32 = 0;
    if sDisplay.is_null() {
        return;
    }
    i = 0;
    while i < 3 {
        (*sDisplay).subtasks[i].callback = Some(Display_Dummy);
        (*sDisplay).subtasks[i].active = FALSE;
        (*sDisplay).subtasks[i].state = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RunDisplaySubtasks() {
    let mut i: i32 = 0;
    if sDisplay.is_null() {
        return;
    }
    i = 0;
    while i < 3 {
        (*sDisplay).subtasks[i].active = (*sDisplay).subtasks[i].callback.unwrap_unchecked()(
            &raw mut (*sDisplay).subtasks[i].state,
        ) as u8;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartDisplaySubtask(subtaskId: u16, assignId: u8) {
    let mut i: u32 = 0;
    (*sDisplay).subtasks[assignId].callback = Some(Display_Dummy);
    i = 0;
    while i < 21 {
        if sDisplaySubtasks[i].idx == subtaskId {
            (*sDisplay).subtasks[assignId].callback = sDisplaySubtasks[i].callback;
            (*sDisplay).subtasks[assignId].active = TRUE;
            (*sDisplay).subtasks[assignId].state = 0;
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IsDisplaySubtaskActive(id: u8) -> u8 {
    return (*sDisplay).subtasks[id].active;
}
pub(crate) unsafe extern "C" fn Display_LoadGfx(state: *mut u8) -> u32 {
    if FreeTempTileDataBuffersIfPossible() == TRUE {
        return TRUE as u32;
    }
    match *state {
        0 => {
            ResetGpuBgState();
            SetBgTilemapBuffers();
        }
        1 => {
            ClearBg0();
        }
        2 => {
            LoadKeyboardWindowGfx();
        }
        3 => {
            LoadChatWindowGfx();
        }
        4 => {
            LoadChatUnkPalette();
        }
        5 => {
            LoadChatMessagesWindow();
            DrawKeyboardWindow();
            LoadKeyboardSwapWindow();
            LoadTextEntryWindow();
        }
        6 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CreateKeyboardCursorSprite();
                CreateTextEntrySprites();
                CreateRButtonSprites();
            }
        }
        _ => {
            return FALSE as u32;
        }
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_ShowKeyboardSwapMenu(state: *mut u8) -> u32 {
    match *state {
        0 => {
            ShowKeyboardSwapMenu();
            CopyWindowToVram(WIN_SWAP_MENU, COPYWIN_FULL);
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_HideKeyboardSwapMenu(state: *mut u8) -> u32 {
    match *state {
        0 => {
            HideKeyboardSwapMenu();
            CopyWindowToVram(WIN_SWAP_MENU, COPYWIN_FULL);
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_SwitchPages(state: *mut u8) -> u32 {
    match *state {
        0 => {
            SetKeyboardCursorInvisibility(TRUE as u32);
            if SlideKeyboardPageOut() != 0 {
                return TRUE as u32;
            }
            PrintCurrentKeyboardPage();
            CopyWindowToVram(WIN_KEYBOARD, COPYWIN_GFX);
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return TRUE as u32;
            }
        }
        2 => {
            if SlideKeyboardPageIn() != 0 {
                return TRUE as u32;
            }
            MoveKeyboardCursor();
            SetKeyboardCursorInvisibility(FALSE as u32);
            UpdateRButtonLabel();
            return FALSE as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_MoveKeyboardCursor(state: *mut u8) -> u32 {
    MoveKeyboardCursor();
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Display_AskQuitChatting(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_QUIT_CHATTING, 0);
            AddYesNoMenuAt(23, 11, 1);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_DestroyYesNoDialog(state: *mut u8) -> u32 {
    match *state {
        0 => {
            HideStdMessageWindow();
            HideYesNoMenuWindow();
            CopyBgTilemapBufferToVram(0);
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return TRUE as u32;
            }
            DestroyStdMessageWindow();
            DestroyYesNoMenuWindow();
            return FALSE as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_UpdateMessageBuffer(state: *mut u8) -> u32 {
    let mut x: u32 = 0;
    let mut width: u32 = 0;
    let mut str: *mut u8 = null_mut();
    match *state {
        0 => {
            GetBufferSelectionRegion(&raw mut x, &raw mut width);
            FillTextEntryWindow(x as u16, width as u16, 0);
            str = GetMessageEntryBuffer();
            DrawTextEntryMessage(0, str, 3, 1, 2);
            CopyWindowToVram(WIN_TEXT_ENTRY, COPYWIN_GFX);
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                UpdateRButtonLabel();
                return FALSE as u32;
            }
            return TRUE as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_AskRegisterText(state: *mut u8) -> u32 {
    let mut x: u16 = 0;
    let mut str: *mut u8 = null_mut();
    let mut length: u16 = 0;
    match *state {
        0 => {
            x = GetLimitedMessageStartPos();
            str = GetLimitedMessageStartPtr();
            length = StringLength_Multibyte(str) as u16;
            FillTextEntryWindow(x, length, 102);
            DrawTextEntryMessage(x, str, 0, 4, 5);
            CopyWindowToVram(WIN_TEXT_ENTRY, COPYWIN_GFX);
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                AddStdMessageWindow(STDMESSAGE_REGISTER_WHERE, 16);
                CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            } else {
                return TRUE as u32;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetRegisteredTextPalette(TRUE as u32);
            } else {
                return TRUE as u32;
            }
        }
        3 => {
            return FALSE as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_CancelRegister(state: *mut u8) -> u32 {
    let mut x: u16 = 0;
    let mut str: *mut u8 = null_mut();
    let mut length: u16 = 0;
    match *state {
        0 => {
            x = GetLimitedMessageStartPos();
            str = GetLimitedMessageStartPtr();
            length = StringLength_Multibyte(str) as u16;
            FillTextEntryWindow(x, length, 0);
            DrawTextEntryMessage(x, str, 3, 1, 2);
            CopyWindowToVram(WIN_TEXT_ENTRY, COPYWIN_GFX);
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                HideStdMessageWindow();
                CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            } else {
                return TRUE as u32;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetRegisteredTextPalette(FALSE as u32);
                DestroyStdMessageWindow();
            } else {
                return TRUE as u32;
            }
        }
        3 => {
            return FALSE as u32;
        }
        _ => {}
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_ReturnToKeyboard(state: *mut u8) -> u32 {
    match *state {
        0 => {
            PrintCurrentKeyboardPage();
            CopyWindowToVram(WIN_KEYBOARD, COPYWIN_GFX);
            *state += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return TRUE as u32;
            } else {
                return FALSE as u32;
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_ScrollChat(state: *mut u8) -> u32 {
    let mut row: u16 = 0;
    let mut str: *mut u8 = null_mut();
    let mut colorIdx: u8 = 0;
    'l1: {
        let sw1: u8 = *state;
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            row = (*sDisplay).currLine;
            str = GetLastReceivedMessage();
            colorIdx = GetReceivedPlayerIndex();
            PrintChatMessage(row, str, colorIdx);
            CopyWindowToVram(WIN_CHAT_HISTORY, COPYWIN_GFX);
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return TRUE as u32;
            }
            if (*sDisplay).currLine < 9 {
                (*sDisplay).currLine += 1;
                *state = 4;
                return FALSE as u32;
            } else {
                (*sDisplay).scrollCount = 0;
                *state += 1;
            }
        }
        if fall || sw1 == 2 {
            fall = true;
            ScrollWindow(WIN_CHAT_HISTORY, 0, 5, 17);
            CopyWindowToVram(WIN_CHAT_HISTORY, COPYWIN_GFX);
            (*sDisplay).scrollCount += 1;
            *state += 1;
        }
        if fall || sw1 == 3 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return TRUE as u32;
            }
            if (*sDisplay).scrollCount < 3 {
                *state -= 1;
                return TRUE as u32;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            return FALSE as u32;
        }
        if !matched {
            fall = true;
            return TRUE as u32;
        }
    }
    *state += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_AnimateKeyboardCursor(state: *mut u8) -> u32 {
    match *state {
        0 => {
            StartKeyboardCursorAnim();
            *state += 1;
        }
        1 => {
            return TryKeyboardCursorReopen();
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_PrintInputText(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_INPUT_TEXT, 16);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_PrintExitingChat(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_EXITING_CHAT, 0);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_PrintLeaderLeft(state: *mut u8) -> u32 {
    let mut str: *mut u8 = null_mut();
    match *state {
        0 => {
            DynamicPlaceholderTextUtil_Reset();
            str = GetChatHostName();
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, str);
            AddStdMessageWindow(STDMESSAGE_LEADER_LEFT, 0);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_AskSave(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_ASK_SAVE, 0);
            AddYesNoMenuAt(23, 10, 1);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_AskOverwriteSave(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_ASK_OVERWRITE, 0);
            AddYesNoMenuAt(23, 10, 1);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_PrintSavingDontTurnOff(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_SAVING_NO_OFF, 0);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_PrintSavedTheGame(state: *mut u8) -> u32 {
    match *state {
        0 => {
            DynamicPlaceholderTextUtil_Reset();
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                0,
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            AddStdMessageWindow(STDMESSAGE_SAVED_THE_GAME, 0);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_AskConfirmLeaderLeave(state: *mut u8) -> u32 {
    match *state {
        0 => {
            AddStdMessageWindow(STDMESSAGE_WARN_LEADER_LEAVE, 0);
            AddYesNoMenuAt(23, 10, 1);
            CopyWindowToVram((*sDisplay).messageWindowId as u8, COPYWIN_FULL);
            *state += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy() as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Display_Dummy(state: *mut u8) -> u32 {
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn AddYesNoMenuAt(left: u8, top: u8, initialCursorPos: u8) {
    let mut template: WindowTemplate = zeroed();
    template.bg = 0;
    template.tilemapLeft = left;
    template.tilemapTop = top;
    template.width = 6;
    template.height = 4;
    template.paletteNum = 14;
    template.baseBlock = 0x52;
    (*sDisplay).yesNoMenuWindowId = AddWindow(&raw mut template);
    if (*sDisplay).yesNoMenuWindowId != WINDOW_NONE as u16 {
        FillWindowPixelBuffer((*sDisplay).yesNoMenuWindowId as u8, 17);
        PutWindowTilemap((*sDisplay).yesNoMenuWindowId as u8);
        AddTextPrinterParameterized(
            (*sDisplay).yesNoMenuWindowId as u8,
            FONT_NORMAL,
            gText_Yes.as_ptr().cast_mut(),
            8,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            (*sDisplay).yesNoMenuWindowId as u8,
            FONT_NORMAL,
            gText_No.as_ptr().cast_mut(),
            8,
            17,
            TEXT_SKIP_DRAW,
            None,
        );
        DrawTextBorderOuter((*sDisplay).yesNoMenuWindowId as u8, 1, 13);
        InitMenuInUpperLeftCornerNormal((*sDisplay).yesNoMenuWindowId as u8, 2, initialCursorPos);
    }
}
pub(crate) unsafe extern "C" fn HideYesNoMenuWindow() {
    if (*sDisplay).yesNoMenuWindowId != WINDOW_NONE as u16 {
        ClearStdWindowAndFrameToTransparent((*sDisplay).yesNoMenuWindowId as u8, FALSE);
        ClearWindowTilemap((*sDisplay).yesNoMenuWindowId as u8);
    }
}
pub(crate) unsafe extern "C" fn DestroyYesNoMenuWindow() {
    if (*sDisplay).yesNoMenuWindowId != WINDOW_NONE as u16 {
        RemoveWindow((*sDisplay).yesNoMenuWindowId as u8);
        (*sDisplay).yesNoMenuWindowId = WINDOW_NONE as u16;
    }
}
pub(crate) unsafe extern "C" fn ProcessMenuInput() -> i8 {
    return Menu_ProcessInput();
}
pub(crate) unsafe extern "C" fn AddStdMessageWindow(msgId: i32, bg0vofs: u16) {
    let mut str: *mut u8 = null_mut();
    let mut windowId: i32 = 0;
    let mut template: WindowTemplate = zeroed();
    template.bg = 0;
    template.tilemapLeft = 8;
    template.tilemapTop = 16;
    template.width = 21;
    template.height = 4;
    template.paletteNum = 14;
    template.baseBlock = 0x6A;
    if sDisplayStdMessages[msgId].useWiderBox != 0 {
        template.tilemapLeft -= 7;
        template.width += 7;
    }
    (*sDisplay).messageWindowId = AddWindow(&raw mut template);
    windowId = (*sDisplay).messageWindowId as i32;
    if (*sDisplay).messageWindowId == WINDOW_NONE as u16 {
        return;
    }
    if sDisplayStdMessages[msgId].hasPlaceholders != 0 {
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            (*sDisplay).expandedPlaceholdersBuffer.as_mut_ptr(),
            sDisplayStdMessages[msgId].text,
        );
        str = (*sDisplay).expandedPlaceholdersBuffer.as_mut_ptr();
    } else {
        str = sDisplayStdMessages[msgId].text;
    }
    ChangeBgY(0, bg0vofs as i32 * 256, BG_COORD_SET);
    FillWindowPixelBuffer(windowId as u8, 17);
    PutWindowTilemap(windowId as u8);
    if sDisplayStdMessages[msgId].boxType == 1 {
        DrawTextBorderInner(windowId as u8, 0xA, 2);
        AddTextPrinterParameterized5(
            windowId as u8,
            FONT_NORMAL,
            str,
            sDisplayStdMessages[msgId].x + 8,
            sDisplayStdMessages[msgId].y + 8,
            TEXT_SKIP_DRAW,
            None,
            sDisplayStdMessages[msgId].letterSpacing,
            sDisplayStdMessages[msgId].lineSpacing,
        );
    } else {
        DrawTextBorderOuter(windowId as u8, 0xA, 2);
        AddTextPrinterParameterized5(
            windowId as u8,
            FONT_NORMAL,
            str,
            sDisplayStdMessages[msgId].x,
            sDisplayStdMessages[msgId].y,
            TEXT_SKIP_DRAW,
            None,
            sDisplayStdMessages[msgId].letterSpacing,
            sDisplayStdMessages[msgId].lineSpacing,
        );
    }
    (*sDisplay).messageWindowId = windowId as u16;
}
pub(crate) unsafe extern "C" fn HideStdMessageWindow() {
    if (*sDisplay).messageWindowId != WINDOW_NONE as u16 {
        ClearStdWindowAndFrameToTransparent((*sDisplay).messageWindowId as u8, FALSE);
        ClearWindowTilemap((*sDisplay).messageWindowId as u8);
    }
    ChangeBgY(0, 0, BG_COORD_SET);
}
pub(crate) unsafe extern "C" fn DestroyStdMessageWindow() {
    if (*sDisplay).messageWindowId != WINDOW_NONE as u16 {
        RemoveWindow((*sDisplay).messageWindowId as u8);
        (*sDisplay).messageWindowId = WINDOW_NONE as u16;
    }
}
pub(crate) unsafe extern "C" fn FillTextEntryWindow(x: u16, width: u16, fillValue: u8) {
    FillWindowPixelRect(WIN_TEXT_ENTRY, fillValue, x * 8, 1, width * 8, 14);
}
pub(crate) unsafe extern "C" fn DrawTextEntryMessage(
    x: u16,
    str: *mut u8,
    bgColor: u8,
    fgColor: u8,
    shadowColor: u8,
) {
    let mut color: CArray<u8, 3> = zeroed();
    let mut strBuffer: CArray<u8, 35> = zeroed();
    if bgColor != TEXT_COLOR_TRANSPARENT {
        FillTextEntryWindow(x, GetTextEntryCursorPosition() as u16 - x, bgColor);
    }
    color[0] = bgColor;
    color[1] = fgColor;
    color[2] = shadowColor;
    strBuffer[0] = EXT_CTRL_CODE_BEGIN;
    strBuffer[1] = EXT_CTRL_CODE_MIN_LETTER_SPACING;
    strBuffer[2] = 8;
    StringCopy(&raw mut strBuffer[3], str);
    AddTextPrinterParameterized3(
        WIN_TEXT_ENTRY,
        FONT_SHORT,
        x as u8 * 8,
        1,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        strBuffer.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn PrintCurrentKeyboardPage() {
    let mut page: u8 = 0;
    let mut i: i32 = 0;
    let mut left: u16 = 0;
    let mut top: u16 = 0;
    let mut color: CArray<u8, 3> = zeroed();
    let mut str: CArray<u8, 45> = zeroed();
    let mut str2: *mut u8 = null_mut();
    FillWindowPixelBuffer(WIN_KEYBOARD, 255);
    page = GetCurrentKeyboardPage();
    color[0] = 0x0;
    color[1] = TEXT_DYNAMIC_COLOR_5;
    color[2] = TEXT_DYNAMIC_COLOR_4;
    if page != UNION_ROOM_KB_PAGE_REGISTER {
        str[0] = EXT_CTRL_CODE_BEGIN;
        str[1] = EXT_CTRL_CODE_MIN_LETTER_SPACING;
        str[2] = 8;
        if page == UNION_ROOM_KB_PAGE_EMOJI {
            left = 6;
        } else {
            left = 8;
        }
        i = 0;
        top = 0;
        while i < UNION_ROOM_KB_ROW_COUNT {
            if sUnionRoomKeyboardText[page][i].is_null() {
                return;
            }
            StringCopy(&raw mut str[3], sUnionRoomKeyboardText[page][i]);
            AddTextPrinterParameterized3(
                WIN_KEYBOARD,
                FONT_SMALL,
                left as u8,
                top as u8,
                color.as_mut_ptr(),
                TEXT_SKIP_DRAW as i8,
                str.as_mut_ptr(),
            );
            i += 1;
            top += 12;
        }
    } else {
        left = 4;
        i = 0;
        top = 0;
        while i < UNION_ROOM_KB_ROW_COUNT {
            str2 = GetRegisteredTextByRow(i);
            if GetStringWidth(FONT_SMALL, str2, 0) <= 40 {
                AddTextPrinterParameterized3(
                    WIN_KEYBOARD,
                    FONT_SMALL,
                    left as u8,
                    top as u8,
                    color.as_mut_ptr(),
                    TEXT_SKIP_DRAW as i8,
                    str2,
                );
            } else {
                let mut length: i32 = StringLength_Multibyte(str2) as i32;
                loop {
                    length -= 1;
                    StringCopyN_Multibyte(str.as_mut_ptr(), str2, length as u32);
                    if GetStringWidth(FONT_SMALL, str.as_mut_ptr(), 0) <= 35 {
                        break;
                    }
                }
                AddTextPrinterParameterized3(
                    WIN_KEYBOARD,
                    FONT_SMALL,
                    left as u8,
                    top as u8,
                    color.as_mut_ptr(),
                    TEXT_SKIP_DRAW as i8,
                    str.as_mut_ptr(),
                );
                AddTextPrinterParameterized3(
                    WIN_KEYBOARD,
                    FONT_SMALL,
                    left as u8 + 35,
                    top as u8,
                    color.as_mut_ptr(),
                    TEXT_SKIP_DRAW as i8,
                    sText_Ellipsis.as_ptr().cast_mut(),
                );
            }
            i += 1;
            top += 12;
        }
    }
}
pub(crate) unsafe extern "C" fn SlideKeyboardPageOut() -> u32 {
    if (*sDisplay).bg1hofs < KEYBOARD_HOFS_END {
        (*sDisplay).bg1hofs += 12;
        if (*sDisplay).bg1hofs >= KEYBOARD_HOFS_END {
            (*sDisplay).bg1hofs = KEYBOARD_HOFS_END;
        }
        if (*sDisplay).bg1hofs < KEYBOARD_HOFS_END {
            UpdateSlidingKeyboard((*sDisplay).bg1hofs);
            return TRUE as u32;
        }
    }
    FinishSlidingKeyboard((*sDisplay).bg1hofs);
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn SlideKeyboardPageIn() -> u32 {
    if (*sDisplay).bg1hofs > 0 {
        (*sDisplay).bg1hofs -= 12;
        if (*sDisplay).bg1hofs <= 0 {
            (*sDisplay).bg1hofs = 0;
        }
        if (*sDisplay).bg1hofs > 0 {
            UpdateSlidingKeyboard((*sDisplay).bg1hofs);
            return TRUE as u32;
        }
    }
    FinishSlidingKeyboard((*sDisplay).bg1hofs);
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn ShowKeyboardSwapMenu() {
    FillWindowPixelBuffer(WIN_SWAP_MENU, 17);
    DrawTextBorderOuter(WIN_SWAP_MENU, 1, 13);
    PrintMenuActionTextsAtPos(
        WIN_SWAP_MENU,
        FONT_SHORT,
        8,
        1,
        14,
        5,
        sKeyboardPageTitleTexts.as_ptr().cast_mut(),
    );
    InitMenuNormal(
        WIN_SWAP_MENU,
        FONT_SHORT,
        0,
        1,
        14,
        5,
        GetCurrentKeyboardPage(),
    );
    PutWindowTilemap(WIN_SWAP_MENU);
}
pub(crate) unsafe extern "C" fn HideKeyboardSwapMenu() {
    ClearStdWindowAndFrameToTransparent(WIN_SWAP_MENU, FALSE);
    ClearWindowTilemap(WIN_SWAP_MENU);
}
pub(crate) unsafe extern "C" fn PrintChatMessage(row: u16, str: *mut u8, colorIdx: u8) {
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = TEXT_COLOR_WHITE;
    color[1] = colorIdx * 2 + 2;
    color[2] = colorIdx * 2 + 3;
    FillWindowPixelRect(WIN_CHAT_HISTORY, 17, 0, row * 15, 168, 15);
    AddTextPrinterParameterized3(
        WIN_CHAT_HISTORY,
        FONT_SHORT,
        0,
        row as u8 * 15 + 1,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
}
pub(crate) unsafe extern "C" fn ResetGpuBgState() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
    SetGpuRegBits(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ClearGpuRegBits(REG_OFFSET_DISPCNT, 57344);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
    SetGpuReg(0x40, 16624);
    SetGpuReg(REG_OFFSET_WIN0V, 144);
    SetGpuReg(REG_OFFSET_WININ, 61);
    SetGpuReg(REG_OFFSET_WINOUT, 63);
}
pub(crate) unsafe extern "C" fn SetBgTilemapBuffers() {
    SetBgTilemapBuffer(0, (*sDisplay).bg0Buffer.as_mut_ptr() as *mut c_void);
    SetBgTilemapBuffer(1, (*sDisplay).bg1Buffer.as_mut_ptr() as *mut c_void);
    SetBgTilemapBuffer(3, (*sDisplay).bg3Buffer.as_mut_ptr() as *mut c_void);
    SetBgTilemapBuffer(2, (*sDisplay).bg2Buffer.as_mut_ptr() as *mut c_void);
}
pub(crate) unsafe extern "C" fn ClearBg0() {
    RequestDma3Fill(0, 0x6000000 as usize as *mut c_void, 0x20, 1);
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn LoadKeyboardWindowGfx() {
    LoadPalette(
        gUnionRoomChat_Keyboard_Pal.as_ptr().cast_mut() as *mut c_void,
        112,
        32,
    );
    LoadPalette(
        gUnionRoomChat_InputText_Pal.as_ptr().cast_mut() as *mut c_void,
        192,
        32,
    );
    DecompressAndCopyTileDataToVram(
        1,
        gUnionRoomChat_Keyboard_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        1,
        gUnionRoomChat_Keyboard_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn LoadChatWindowGfx() {
    let mut ptr: *mut u8 = null_mut();
    LoadPalette(
        gUnionRoomChat_Background_Pal.as_ptr().cast_mut() as *mut c_void,
        0,
        32,
    );
    ptr = DecompressAndCopyTileDataToVram(
        2,
        gUnionRoomChat_Background_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    ) as *mut u8;
    if !ptr.is_null() {
        CpuFastSet(
            ptr.at(544) as *mut c_void,
            &raw mut (*sDisplay).textEntryTiles[0] as *mut c_void,
            8,
        );
        CpuFastSet(
            ptr.at(1056) as *mut c_void,
            &raw mut (*sDisplay).textEntryTiles[32] as *mut c_void,
            8,
        );
    }
    CopyToBgTilemapBuffer(
        2,
        gUnionRoomChat_Background_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(2);
}
pub(crate) unsafe extern "C" fn LoadChatUnkPalette() {
    LoadPalette(sUnusedPalette.as_ptr().cast_mut() as *mut c_void, 128, 32);
    RequestDma3Fill(
        0,
        (0x6004000 as usize as *mut c_void as *mut u8).at(32) as *mut c_void,
        32,
        1,
    );
}
pub(crate) unsafe extern "C" fn LoadChatMessagesWindow() {
    LoadPalette(
        sChatMessagesWindow_Pal.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    PutWindowTilemap(WIN_CHAT_HISTORY);
    FillWindowPixelBuffer(WIN_CHAT_HISTORY, 17);
    CopyWindowToVram(WIN_CHAT_HISTORY, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn DrawKeyboardWindow() {
    PutWindowTilemap(WIN_KEYBOARD);
    PrintCurrentKeyboardPage();
    CopyWindowToVram(WIN_KEYBOARD, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn LoadTextEntryWindow() {
    let mut i: i32 = 0;
    let mut unused: CArray<u8, 2> = zeroed();
    unused[0] = 0;
    unused[1] = 0xFF;
    i = 0;
    while i < MAX_MESSAGE_LENGTH as i32 {
        BlitBitmapToWindow(
            WIN_TEXT_ENTRY,
            (*sDisplay).textEntryTiles.as_mut_ptr(),
            i as u16 * 8,
            0,
            8,
            16,
        );
        i += 1;
    }
    FillWindowPixelBuffer(WIN_TEXT_ENTRY, 0);
    PutWindowTilemap(WIN_TEXT_ENTRY);
    CopyWindowToVram(WIN_TEXT_ENTRY, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn LoadKeyboardSwapWindow() {
    FillWindowPixelBuffer(WIN_SWAP_MENU, 17);
    LoadUserWindowBorderGfx(WIN_SWAP_MENU, 1, 208);
    LoadUserWindowBorderGfx_(WIN_SWAP_MENU, 0xA, 32);
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        224,
        32,
    );
}
pub(crate) unsafe extern "C" fn InitScanlineEffect() {
    let mut params: ScanlineEffectParams = zeroed();
    params.dmaControl = 0xa2600001;
    params.dmaDest = 67108884 as usize as *mut u16 as *mut c_void;
    params.initState = 1;
    params.unused9 = 0;
    (*sDisplay).bg1hofs = 0;
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            gScanlineEffectRegBuffers.as_mut_ptr() as *mut c_void,
            0x10003c0,
        );
    }
    ScanlineEffect_SetParams(params);
}
pub(crate) unsafe extern "C" fn UpdateSlidingKeyboard(bg1hofs: i16) {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, bg1hofs as u16);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer].as_mut_ptr() as *mut c_void,
                0x1000090,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer]
                    .as_mut_ptr()
                    .at(144) as *mut c_void,
                0x1000010,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FinishSlidingKeyboard(bg1hofs: i16) {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, bg1hofs as u16);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut c_void,
                0x1000090,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[0].as_mut_ptr().at(144) as *mut c_void,
                0x1000010,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, bg1hofs as u16);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[0].as_mut_ptr().at(960) as *mut c_void,
                0x1000090,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[0].as_mut_ptr().at(1104) as *mut c_void,
                0x1000010,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryAllocSprites() -> u32 {
    let mut i: u32 = 0;
    i = 0;
    while i < 5 {
        LoadCompressedSpriteSheet((&raw const sSpriteSheets[i]).cast_mut());
        i += 1;
    }
    LoadSpritePalette((&raw const *sSpritePalette).cast_mut());
    sSprites = Alloc(24) as *mut UnionRoomChatSprites;
    if sSprites.is_null() {
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn FreeSprites() {
    if !sSprites.is_null() {
        Free(sSprites as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn CreateKeyboardCursorSprite() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_KeyboardCursor).cast_mut(),
        10,
        24,
        0,
    );
    (*sSprites).keyboardCursor = &raw mut gSprites[spriteId];
}
pub(crate) unsafe extern "C" fn SetKeyboardCursorInvisibility(invisible: u32) {
    (*(*sSprites).keyboardCursor).set_invisible(invisible as u16);
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor() {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut page: u8 = GetCurrentKeyboardPage();
    GetCurrentKeyboardColAndRow(&raw mut x, &raw mut y);
    if page != UNION_ROOM_KB_PAGE_REGISTER {
        StartSpriteAnim((*sSprites).keyboardCursor, 0);
        (*(*sSprites).keyboardCursor).x = x as i16 * 8 + 10;
        (*(*sSprites).keyboardCursor).y = y as i16 * 12 + 24;
    } else {
        StartSpriteAnim((*sSprites).keyboardCursor, 2);
        (*(*sSprites).keyboardCursor).x = 24;
        (*(*sSprites).keyboardCursor).y = y as i16 * 12 + 24;
    }
}
pub(crate) unsafe extern "C" fn SetRegisteredTextPalette(registering: u32) {
    let mut palette: *mut u16 =
        (&raw const sUnionRoomChatInterfacePal[registering * 2 + 1]).cast_mut();
    let mut index: u8 = IndexOfSpritePaletteTag(PALTAG_INTERFACE);
    LoadPalette(palette as *mut c_void, 0x100 + index as u16 * 16 + 1, 4);
}
pub(crate) unsafe extern "C" fn StartKeyboardCursorAnim() {
    if GetCurrentKeyboardPage() != UNION_ROOM_KB_PAGE_REGISTER {
        StartSpriteAnim((*sSprites).keyboardCursor, 1);
    } else {
        StartSpriteAnim((*sSprites).keyboardCursor, 3);
    }
    (*sSprites).cursorBlinkTimer = 0;
}
pub(crate) unsafe extern "C" fn TryKeyboardCursorReopen() -> u32 {
    if (*sSprites).cursorBlinkTimer > 3 {
        return FALSE as u32;
    }
    if ({
        (*sSprites).cursorBlinkTimer += 1;
        (*sSprites).cursorBlinkTimer
    }) > 3
    {
        if GetCurrentKeyboardPage() != UNION_ROOM_KB_PAGE_REGISTER {
            StartSpriteAnim((*sSprites).keyboardCursor, 0);
        } else {
            StartSpriteAnim((*sSprites).keyboardCursor, 2);
        }
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn CreateTextEntrySprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_TextEntryCursor).cast_mut(),
        76,
        152,
        2,
    );
    (*sSprites).textEntryCursor = &raw mut gSprites[spriteId];
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_TextEntryArrow).cast_mut(),
        64,
        152,
        1,
    );
    (*sSprites).textEntryArrow = &raw mut gSprites[spriteId];
}
pub(crate) unsafe extern "C" fn SpriteCB_TextEntryCursor(sprite: *mut Sprite) {
    let mut pos: i32 = GetTextEntryCursorPosition();
    if pos == MAX_MESSAGE_LENGTH as i32 {
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).x = pos as i16 * 8 + 76;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TextEntryArrow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 4
    {
        (*sprite).data[0] = 0;
        if ({
            (*sprite).x2 += 1;
            (*sprite).x2
        }) > 4
        {
            (*sprite).x2 = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateRButtonSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_RButtonIcon).cast_mut(),
        8,
        152,
        3,
    );
    (*sSprites).rButtonIcon = &raw mut gSprites[spriteId];
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_RButtonLabels).cast_mut(),
        32,
        152,
        4,
    );
    (*sSprites).rButtonLabel = &raw mut gSprites[spriteId];
    (*(*sSprites).rButtonLabel).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn UpdateRButtonLabel() {
    if GetCurrentKeyboardPage() == UNION_ROOM_KB_PAGE_REGISTER {
        if GetLengthOfMessageEntry() != 0 {
            (*(*sSprites).rButtonLabel).set_invisible(FALSE as u16);
            StartSpriteAnim((*sSprites).rButtonLabel, 3);
        } else {
            (*(*sSprites).rButtonLabel).set_invisible(TRUE as u16);
        }
    } else {
        let mut anim: i32 = GetShouldShowCaseToggleIcon();
        if anim == 3 {
            (*(*sSprites).rButtonLabel).set_invisible(TRUE as u16);
        } else {
            (*(*sSprites).rButtonLabel).set_invisible(FALSE as u16);
            StartSpriteAnim((*sSprites).rButtonLabel, anim as u8);
        }
    }
}
