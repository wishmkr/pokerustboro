//! Translated from `src/union_room_chat.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sChatMainFunctions sKeyboardPageMaxRow sCaseToggleTable sUnionRoomKeyboardText sUnusedPalette sChatMessagesWindow_Pal sBgTemplates sWinTemplates sDisplaySubtasks sDisplayStdMessages sText_Ellipsis sKeyboardPageTitleTexts sUnionRoomChatInterfacePal sKeyboardCursorTiles sTextEntryCursorTiles sTextEntryArrowTiles sRButtonGfxTiles sSpriteSheets sSpritePalette sOam_KeyboardCursor sAnim_KeyboardCursor_Open sAnim_KeyboardCursor_Closed sAnim_KeyboardCursorWide_Open sAnim_KeyboardCursorWide_Closed sAnims_KeyboardCursor sSpriteTemplate_KeyboardCursor sOam_TextEntrySprite sSpriteTemplate_TextEntryCursor sSpriteTemplate_TextEntryArrow sOam_RButtonIcon sOam_RButtonLabel sAnim_ToggleCaseIcon sAnim_ToggleCaseIcon_Duplicate1 sAnim_ToggleCaseIcon_Duplicate2 sAnim_RegisterIcon sAnims_RButtonLabels sSpriteTemplate_RButtonIcon sSpriteTemplate_RButtonLabels
#[allow(unused_imports)]
use crate::data::union_room_chat::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sChat: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDisplay: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSprites: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBlockRecvBuffer: u8;
    static mut gKeyRepeatStartDelay: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gTasks: u8;
    static mut gText_Battle: u8;
    static mut gText_ByeBye: u8;
    static mut gText_F700JoinedChat: u8;
    static mut gText_F700LeftChat: u8;
    static mut gText_Hello: u8;
    static mut gText_Lets: u8;
    static mut gText_No: u8;
    static mut gText_Ok: u8;
    static mut gText_Pokemon2: u8;
    static mut gText_Sorry: u8;
    static mut gText_ThankYou: u8;
    static mut gText_Trade: u8;
    static mut gText_YaySmileEmoji: u8;
    static mut gText_Yes: u8;
    static mut gUnionRoomChat_Background_Gfx: u8;
    static mut gUnionRoomChat_Background_Pal: u8;
    static mut gUnionRoomChat_Background_Tilemap: u8;
    static mut gUnionRoomChat_InputText_Pal: u8;
    static mut gUnionRoomChat_Keyboard_Gfx: u8;
    static mut gUnionRoomChat_Keyboard_Pal: u8;
    static mut gUnionRoomChat_Keyboard_Tilemap: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a7: u8,
        a8: u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
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
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyTask(a0: u8);
    fn DrawTextBorderInner(a0: u8, a1: u16, a2: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitMenuNormal(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_MoveCursor(a0: i8) -> u8;
    fn Menu_ProcessInput() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuActionTextsAtPos(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: *mut u8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
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
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScrollWindow(a0: u8, a1: u8, a2: u8, a3: u8);
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetUnionRoomChatPlayerData(a0: u32);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN_Multibyte(a0: *mut u8, a1: *mut u8, a2: u32) -> *mut u8;
    fn StringLength_Multibyte(a0: *mut u8) -> u32;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnterUnionRoomChat() {
    unsafe {
        ((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).write(Alloc(444u32));
        InitUnionRoomChat(((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read());
        ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(20u16);
        SetVBlankCallback(None);
        SetMainCallback2(Some(CB2_LoadInterface));
    }
}
pub(crate) unsafe extern "C" fn InitUnionRoomChat(chat: *mut u8) {
    unsafe {
        let mut chat = chat;
        let mut i: i32 = 0i32;
        ((chat).wrapping_add(4).cast::<u16>()).write(0u16);
        ((chat).wrapping_add(6).cast::<u16>()).write(0u16);
        ((chat).wrapping_add(16)).write(0u8);
        ((chat).wrapping_add(17)).write(0u8);
        ((chat).wrapping_add(18)).write(0u8);
        ((chat).wrapping_add(20)).write(0u8);
        ((chat).wrapping_add(21)).write(0u8);
        ((chat).wrapping_add(22)).write(0u8);
        (((chat).wrapping_add(26)).cast::<u8>()).write(255u8);
        ((chat).wrapping_add(13)).write(GetLinkPlayerCount());
        ((chat).wrapping_add(19)).write(GetMultiplayerId());
        ((chat).wrapping_add(23)).write(0u8);
        ((chat).wrapping_add(24)).write(0u8);
        PrepareSendBuffer_Null(((chat).wrapping_add(400)).cast::<u8>());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 10i32) {
                    break 'l1;
                }
                'l2: {
                    StringCopy(
                        ((((chat).wrapping_add(185)).cast::<u8>())
                            .wrapping_offset((i) as isize * 21))
                        .cast::<u8>(),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15496))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 21))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeUnionRoomChat() {
    unsafe {
        DestroyTask(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14)).read(),
        );
        DestroyTask(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15)).read(),
        );
        Free(((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadInterface() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                TryAllocDisplay();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                RunDisplaySubtasks();
                if !((IsDisplaySubtask0Active()) != 0) {
                    BlendPalettes(4294967295u32, 16u8, 0u16);
                    BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                    SetVBlankCallback(Some(VBlankCB_UnionRoomChatMain));
                    let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetMainCallback2(Some(CB2_UnionRoomChatMain));
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
                        .write(CreateTask(Some(Task_HandlePlayerInput), 8u8));
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15))
                        .write(CreateTask(Some(Task_ReceiveChatMessage), 7u8));
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(232u8, 150u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_UnionRoomChatMain() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn CB2_UnionRoomChatMain() {
    unsafe {
        RunTasks();
        RunDisplaySubtasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePlayerInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(23))
            .read()) as i32);
            if __sw1 == 1i32 {
                SetChatFunction(6u16);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetChatFunction(7u16);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetChatFunction(8u16);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23))
                    .write(0u8);
                break 'l1;
            }
        }
        (((((&raw const sChatMainFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn Chat_Join() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                PrepareSendBuffer_Join(
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(400))
                        .cast::<u8>(),
                );
                let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((IsLinkTaskFinished()) != 0) && (!((Rfu_IsPlayerExchangeActive()) != 0)) {
                    if (SendBlock(
                        0u8,
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(400))
                        .cast::<u8>(),
                        40u16,
                    )) != 0
                    {
                        let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>();
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SetChatFunction(1u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_HandleInput() {
    unsafe {
        let mut updateMsgActive: u8 = 0u8;
        let mut cursorBlinkActive: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 8i32)
                    != 0
                {
                    if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .read())
                        != 0
                    {
                        SetChatFunction(4u16);
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 4i32)
                        != 0
                    {
                        SetChatFunction(2u16);
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(21))
                            .read())
                                != 0
                            {
                                DeleteLastMessageCharacter();
                                StartDisplaySubtask(8u16, 0u8);
                                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                .write(1u16);
                            } else {
                                SetChatFunction(3u16);
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                AppendTextToMessage();
                                StartDisplaySubtask(8u16, 0u8);
                                StartDisplaySubtask(2u16, 1u8);
                                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                .write(1u16);
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 256i32)
                                    != 0
                                {
                                    if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(16))
                                    .read()) as i32)
                                        != 3i32
                                    {
                                        SwitchCaseOfLastMessageCharacter();
                                        StartDisplaySubtask(8u16, 0u8);
                                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .write(1u16);
                                    } else {
                                        SetChatFunction(5u16);
                                    }
                                } else {
                                    if (HandleDPadInput()) != 0 {
                                        StartDisplaySubtask(1u16, 0u8);
                                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .write(1u16);
                                    }
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                updateMsgActive = IsDisplaySubtaskActive(0u8);
                cursorBlinkActive = IsDisplaySubtaskActive(1u8);
                if (!((updateMsgActive) != 0)) && (!((cursorBlinkActive) != 0)) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(0u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_Switch() {
    unsafe {
        let mut input: i16 = 0i16;
        let mut shouldSwitchPages: u32 = 0u32;
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StartDisplaySubtask(3u16, 0u8);
                let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = ((Menu_ProcessInput()) as i16);
                'l2: {
                    let __sw4 = ((input) as i32);
                    let __matched = __sw4 == (-2i32) || __sw4 == (-1i32);
                    if !__matched {
                        StartDisplaySubtask(4u16, 0u8);
                        shouldSwitchPages = 1u32;
                        if (((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            == ((input) as i32))
                            || (((input) as i32) > 3i32)
                        {
                            shouldSwitchPages = 0u32;
                        }
                        break 'l2;
                    }
                    if __sw4 == (-2i32) {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 4i32)
                            != 0
                        {
                            PlaySE(5u16);
                            Menu_MoveCursor(1i8);
                        }
                        return;
                    }
                    if __sw4 == (-1i32) {
                        StartDisplaySubtask(4u16, 0u8);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(3u16);
                        return;
                    }
                }
                if !((shouldSwitchPages) != 0) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(3u16);
                    return;
                }
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17))
                    .write(0u8);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
                    .write(0u8);
                StartDisplaySubtask(5u16, 1u8);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                    .write(((input) as u8));
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(4u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    SetChatFunction(1u16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (!((IsDisplaySubtaskActive(0u8)) != 0))
                    && (!((IsDisplaySubtaskActive(1u8)) != 0))
                {
                    SetChatFunction(1u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_AskQuitChatting() {
    unsafe {
        let mut input: i8 = 0i8;
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StartDisplaySubtask(6u16, 0u8);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(1u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(2u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = ProcessMenuInput();
                'l2: {
                    let __sw2 = ((input) as i32);
                    if __sw2 == (-1i32) || __sw2 == 1i32 {
                        StartDisplaySubtask(7u16, 0u8);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(3u16);
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(19))
                        .read()) as i32)
                            == 0i32
                        {
                            PrepareSendBuffer_Disband(
                                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(400))
                                .cast::<u8>(),
                            );
                            StartDisplaySubtask(7u16, 0u8);
                            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(6)
                                .cast::<u16>())
                            .write(9u16);
                        } else {
                            PrepareSendBuffer_Leave(
                                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(400))
                                .cast::<u8>(),
                            );
                            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(6)
                                .cast::<u16>())
                            .write(4u16);
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    SetChatFunction(1u16);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    StartDisplaySubtask(20u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(10u16);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(8u16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                input = ProcessMenuInput();
                'l3: {
                    let __sw3 = ((input) as i32);
                    if __sw3 == (-1i32) || __sw3 == 1i32 {
                        StartDisplaySubtask(7u16, 0u8);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(3u16);
                        break 'l3;
                    }
                    if __sw3 == 0i32 {
                        Rfu_StopPartnerSearch();
                        PrepareSendBuffer_Disband(
                            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(400))
                            .cast::<u8>(),
                        );
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(4u16);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(440)
                            .cast::<u16>())
                        .write(0u16);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((IsLinkTaskFinished()) != 0) && (!((Rfu_IsPlayerExchangeActive()) != 0)))
                    && ((SendBlock(
                        0u8,
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(400))
                        .cast::<u8>(),
                        40u16,
                    )) != 0)
                {
                    if !((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .read())
                        != 0)
                    {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(6u16);
                    } else {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(5u16);
                    }
                }
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    SetChatFunction(9u16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    SetChatFunction(9u16);
                } else {
                    if (({
                        let __p4 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(440)
                            .cast::<u16>();
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        > 300i32
                    {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(440)
                            .cast::<u16>())
                        .write(0u16);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(4u16);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_Exit() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((FuncIsActiveTask(Some(Task_ReceiveChatMessage))) != 0) {
                    StartDisplaySubtask(7u16, 0u8);
                    let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    StartDisplaySubtask(18u16, 0u8);
                    let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    PrepareSendBuffer_Drop(
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(400))
                        .cast::<u8>(),
                    );
                    let __p4 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((IsLinkTaskFinished()) != 0) && (!((Rfu_IsPlayerExchangeActive()) != 0)))
                    && ((SendBlock(
                        0u8,
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(400))
                        .cast::<u8>(),
                        40u16,
                    )) != 0)
                {
                    let __p5 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((GetBlockReceivedStatus()) as i32) & 1i32) != 0)
                    && (!((Rfu_IsPlayerExchangeActive()) != 0))
                {
                    let __p6 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((IsLinkTaskFinished()) != 0) && (!((Rfu_IsPlayerExchangeActive()) != 0)) {
                    SetCloseLinkCallback();
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .write(0u16);
                    let __p7 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    < 150i32
                {
                    let __p8 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    let __p9 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    >= 150i32
                {
                    SetChatFunction(9u16);
                } else {
                    let __p10 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_Drop() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((FuncIsActiveTask(Some(Task_ReceiveChatMessage))) != 0) {
                    StartDisplaySubtask(7u16, 0u8);
                    let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((!((IsDisplaySubtaskActive(0u8)) != 0)) && ((IsLinkTaskFinished()) != 0))
                    && (!((Rfu_IsPlayerExchangeActive()) != 0))
                {
                    SetCloseLinkCallback();
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .write(0u16);
                    let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    < 150i32
                {
                    let __p4 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    let __p5 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    >= 150i32
                {
                    SetChatFunction(9u16);
                } else {
                    let __p6 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_Disbanded() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((FuncIsActiveTask(Some(Task_ReceiveChatMessage))) != 0) {
                    if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .read())
                        != 0
                    {
                        StartDisplaySubtask(7u16, 0u8);
                    }
                    let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .read())
                        != 0
                    {
                        StartDisplaySubtask(19u16, 0u8);
                    }
                    let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((IsDisplaySubtaskActive(0u8)) as i32) != 1i32)
                    && ((IsLinkTaskFinished()) != 0))
                    && (!((Rfu_IsPlayerExchangeActive()) != 0))
                {
                    SetCloseLinkCallback();
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .write(0u16);
                    let __p4 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    < 150i32
                {
                    let __p5 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    let __p6 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    >= 150i32
                {
                    SetChatFunction(9u16);
                } else {
                    let __p7 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_SendMessage() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    SetChatFunction(1u16);
                    break 'l1;
                }
                PrepareSendBuffer_Chat(
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(400))
                        .cast::<u8>(),
                );
                let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((((IsLinkTaskFinished()) as i32) == 1i32)
                    && (!((Rfu_IsPlayerExchangeActive()) != 0)))
                    && ((SendBlock(
                        0u8,
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(400))
                        .cast::<u8>(),
                        40u16,
                    )) != 0)
                {
                    let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                ResetMessageEntryBuffer();
                StartDisplaySubtask(8u16, 0u8);
                let __p4 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    let __p5 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SetChatFunction(1u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_Register() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (ChatMessageIsNotEmpty()) != 0 {
                    StartDisplaySubtask(9u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(2u16);
                } else {
                    StartDisplaySubtask(13u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    RegisterTextAtRow();
                    StartDisplaySubtask(11u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(3u16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        StartDisplaySubtask(10u16, 0u8);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(4u16);
                    } else {
                        if (HandleDPadInput()) != 0 {
                            StartDisplaySubtask(1u16, 0u8);
                            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(6)
                                .cast::<u16>())
                            .write(2u16);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(1u16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    StartDisplaySubtask(10u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(4u16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    SetChatFunction(1u16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(6u16);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    StartDisplaySubtask(7u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(4u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Chat_SaveAndExit() {
    unsafe {
        let mut input: i8 = 0i8;
        'l1: {
            let __sw1 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .read())
                    != 0)
                {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(12u16);
                } else {
                    StartDisplaySubtask(7u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(1u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    StartDisplaySubtask(14u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(2u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = ProcessMenuInput();
                'l2: {
                    let __sw2 = ((input) as i32);
                    if __sw2 == (-1i32) || __sw2 == 1i32 {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(12u16);
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        StartDisplaySubtask(7u16, 0u8);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(3u16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    StartDisplaySubtask(15u16, 0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(4u16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                input = ProcessMenuInput();
                'l3: {
                    let __sw3 = ((input) as i32);
                    if __sw3 == (-1i32) || __sw3 == 1i32 {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(12u16);
                        break 'l3;
                    }
                    if __sw3 == 0i32 {
                        StartDisplaySubtask(7u16, 0u8);
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write(6u16);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    StartDisplaySubtask(16u16, 0u8);
                    SaveRegisteredTexts();
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(7u16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    SetContinueGameWarpStatusToDynamicWarp();
                    TrySavingData(0u8);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(8u16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                StartDisplaySubtask(17u16, 0u8);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(9u16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((IsDisplaySubtaskActive(0u8)) != 0) {
                    PlaySE(55u16);
                    ClearContinueGameWarpStatus2();
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(10u16);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .write(0u8);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(11u16);
                break 'l1;
            }
            if __sw1 == 11i32 {
                let __p4 =
                    (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .read()) as i32)
                    > 120i32
                {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(12u16);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(13u16);
                break 'l1;
            }
            if __sw1 == 13i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeDisplay();
                    FreeUnionRoomChat();
                    SetMainCallback2(Some(CB2_ReturnToField));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetChatFunction(funcId: u16) {
    unsafe {
        let mut funcId = funcId;
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(funcId);
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn HandleDPadInput() -> u32 {
    unsafe {
        'l1: loop {
            'l2: {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .read()) as i32)
                        > 0i32
                    {
                        let __p1 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18);
                        (__p1).write(((__p1).read()).wrapping_sub(1));
                    } else {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18))
                        .write(
                            ((((&raw const sKeyboardPageMaxRow).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(16))
                                .read()) as i32) as isize,
                            ))
                            .read(),
                        );
                    }
                    break 'l1;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .read()) as i32)
                        < ((((((&raw const sKeyboardPageMaxRow).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                    {
                        let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    } else {
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18))
                        .write(0u8);
                    }
                    break 'l1;
                }
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                    .read()) as i32)
                    != 3i32
                {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0
                    {
                        if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(17))
                        .read()) as i32)
                            > 0i32
                        {
                            let __p3 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(17);
                            (__p3).write(((__p3).read()).wrapping_sub(1));
                        } else {
                            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(17))
                            .write(4u8);
                        }
                        break 'l1;
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0
                        {
                            if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(17))
                            .read()) as i32)
                                < 4i32
                            {
                                let __p4 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(17);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                            } else {
                                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(17))
                                .write(0u8);
                            }
                            break 'l1;
                        }
                    }
                }
                return 0u32;
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn AppendTextToMessage() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut charsStr: *mut u8 = core::ptr::null_mut();
        let mut strLength: i32 = 0i32;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut buffer = crate::ffi::Align4([0u8; 21]);
        if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).read())
            as i32)
            != 3i32
        {
            charsStr = ((((((&raw const sUnionRoomKeyboardText).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                    .read()) as i32) as isize
                    * 40,
            ))
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
                    .read()) as i32) as isize,
            ))
            .read();
            {
                i = 0i32;
                'l1: loop {
                    if !(i
                        < ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(17))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if (((charsStr).read()) as i32) == 249i32 {
                            charsStr = (charsStr).wrapping_offset(1);
                        }
                        charsStr = (charsStr).wrapping_offset(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            strLength = 1i32;
        } else {
            let mut tempStr: *mut u8 = StringCopy(
                (&raw mut buffer).cast::<u8>(),
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(185))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
                        .read()) as i32) as isize
                        * 21,
                ))
                .cast::<u8>(),
            );
            (tempStr).write(0u8);
            ((tempStr).wrapping_offset(1)).write(255u8);
            charsStr = (&raw mut buffer).cast::<u8>();
            strLength = ((StringLength_Multibyte((&raw mut buffer).cast::<u8>())) as i32);
        }
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21)).read(),
        );
        if !(!(charsStr).is_null()) {
            return;
        }
        str = GetEndOfMessagePtr();
        'l3: loop {
            if !(({
                let __t1 = (strLength).wrapping_sub(1);
                strLength = __t1;
                __t1
            } != (-1i32))
                && (((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
                    .read()) as i32)
                    < 15i32))
            {
                break 'l3;
            }
            if (((charsStr).read()) as i32) == 249i32 {
                (str).write((charsStr).read());
                charsStr = (charsStr).wrapping_offset(1);
                str = (str).wrapping_offset(1);
            }
            (str).write((charsStr).read());
            charsStr = (charsStr).wrapping_offset(1);
            str = (str).wrapping_offset(1);
            let __p2 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        (str).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn DeleteLastMessageCharacter() {
    unsafe {
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21)).read(),
        );
        if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21)).read())
            != 0
        {
            let mut str: *mut u8 = GetLastCharOfMessagePtr();
            (str).write(255u8);
            let __p1 = (((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchCaseOfLastMessageCharacter() {
    unsafe {
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut character: u8 = 0u8;
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).write(
            ((((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
                .read()) as i32)
                .wrapping_sub(1i32)) as u8),
        );
        str = GetLastCharOfMessagePtr();
        if (((str).read()) as i32) != 249i32 {
            character = ((((&raw const sCaseToggleTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((str).read()) as i32) as isize))
            .read();
            if (character) != 0 {
                (str).write(character);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChatMessageIsNotEmpty() -> u32 {
    unsafe {
        if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21)).read())
            != 0
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn RegisterTextAtRow() {
    unsafe {
        let mut src: *mut u8 = GetLimitedMessageStartPtr();
        StringCopy(
            ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(185))
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
                    .read()) as i32) as isize
                    * 21,
            ))
            .cast::<u8>(),
            src,
        );
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn ResetMessageEntryBuffer() {
    unsafe {
        (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
            .cast::<u8>())
        .write(255u8);
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).write(15u8);
        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SaveRegisteredTexts() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 10i32) {
                    break 'l1;
                }
                'l2: {
                    StringCopy(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15496))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 21))
                        .cast::<u8>(),
                        ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(185))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 21))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRegisteredTextByRow(row: i32) -> *mut u8 {
    unsafe {
        let mut row = row;
        return ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(185))
            .cast::<u8>())
        .wrapping_offset((row) as isize * 21))
        .cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn GetEndOfMessagePtr() -> *mut u8 {
    unsafe {
        let mut str: *mut u8 = ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26))
        .cast::<u8>();
        'l1: loop {
            if !((((str).read()) as i32) != 255i32) {
                break 'l1;
            }
            str = (str).wrapping_offset(1);
        }
        return str;
    }
}
pub(crate) unsafe extern "C" fn GetLastCharOfMessagePtr() -> *mut u8 {
    unsafe {
        let mut currChar: *mut u8 = ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26))
        .cast::<u8>();
        let mut lastChar: *mut u8 = currChar;
        'l1: loop {
            if !((((currChar).read()) as i32) != 255i32) {
                break 'l1;
            }
            lastChar = currChar;
            if (((currChar).read()) as i32) == 249i32 {
                currChar = (currChar).wrapping_offset(1);
            }
            currChar = (currChar).wrapping_offset(1);
        }
        return lastChar;
    }
}
pub(crate) unsafe extern "C" fn GetNumOverflowCharsInMessage() -> u16 {
    unsafe {
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut i: u32 = 0u32;
        let mut numChars: u32 = 0u32;
        let mut strLength: u32 = 0u32;
        strLength = StringLength_Multibyte(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                .cast::<u8>(),
        );
        str = ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
            .cast::<u8>();
        numChars = 0u32;
        if strLength > 10u32 {
            strLength = (strLength).wrapping_sub(10u32);
            {
                i = 0u32;
                'l1: loop {
                    if !(i < strLength) {
                        break 'l1;
                    }
                    'l2: {
                        if (((str).read()) as i32) == 249i32 {
                            str = (str).wrapping_offset(1);
                        }
                        str = (str).wrapping_offset(1);
                        numChars = (numChars).wrapping_add(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return ((numChars) as u16);
    }
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Null(buffer: *mut u8) {
    unsafe {
        let mut buffer = buffer;
        (buffer).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Join(buffer: *mut u8) {
    unsafe {
        let mut buffer = buffer;
        (buffer).write(2u8);
        StringCopy(
            (buffer).wrapping_offset(1),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((buffer).wrapping_offset(9)).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Chat(buffer: *mut u8) {
    unsafe {
        let mut buffer = buffer;
        (buffer).write(1u8);
        StringCopy(
            (buffer).wrapping_offset(1),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        StringCopy(
            (buffer).wrapping_offset(9),
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
                .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Leave(buffer: *mut u8) {
    unsafe {
        let mut buffer = buffer;
        (buffer).write(3u8);
        StringCopy(
            (buffer).wrapping_offset(1),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((buffer).wrapping_offset(9)).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19)).read(),
        );
        RfuSetNormalDisconnectMode();
    }
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Drop(buffer: *mut u8) {
    unsafe {
        let mut buffer = buffer;
        (buffer).write(4u8);
        StringCopy(
            (buffer).wrapping_offset(1),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((buffer).wrapping_offset(9)).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrepareSendBuffer_Disband(buffer: *mut u8) {
    unsafe {
        let mut buffer = buffer;
        (buffer).write(5u8);
        StringCopy(
            (buffer).wrapping_offset(1),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((buffer).wrapping_offset(9)).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ProcessReceivedChatMessage(
    dest: *mut u8,
    recvMessage: *mut u8,
) -> u32 {
    unsafe {
        let mut dest = dest;
        let mut recvMessage = recvMessage;
        let mut tempStr: *mut u8 = core::ptr::null_mut();
        let mut cmd: u8 = (recvMessage).read();
        let mut name: *mut u8 = (recvMessage).wrapping_offset(1);
        recvMessage = name;
        recvMessage = (recvMessage).wrapping_offset(8);
        'l1: {
            let __sw1 = ((cmd) as i32);
            let mut __fall = false;
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19))
                    .read()) as i32)
                    != ((((name).wrapping_offset(8)).read()) as i32)
                {
                    DynamicPlaceholderTextUtil_Reset();
                    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, name);
                    DynamicPlaceholderTextUtil_ExpandPlaceholders(
                        dest,
                        (&raw mut gText_F700JoinedChat).cast::<u8>(),
                    );
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                tempStr = StringCopy(dest, name);
                ({
                    let __t2 = tempStr;
                    tempStr = (tempStr).wrapping_offset(1);
                    __t2
                })
                .write(252u8);
                ({
                    let __t3 = tempStr;
                    tempStr = (tempStr).wrapping_offset(1);
                    __t3
                })
                .write(19u8);
                ({
                    let __t4 = tempStr;
                    tempStr = (tempStr).wrapping_offset(1);
                    __t4
                })
                .write(42u8);
                ({
                    let __t5 = tempStr;
                    tempStr = (tempStr).wrapping_offset(1);
                    __t5
                })
                .write(240u8);
                StringCopy(tempStr, recvMessage);
                return 1u32;
            }
            if __sw1 == 5i32 {
                __fall = true;
                StringCopy(
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(121))
                        .cast::<u8>(),
                    name,
                );
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19))
                    .read()) as i32)
                    != (((recvMessage).read()) as i32)
                {
                    DynamicPlaceholderTextUtil_Reset();
                    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, name);
                    DynamicPlaceholderTextUtil_ExpandPlaceholders(
                        dest,
                        (&raw mut gText_F700LeftChat).cast::<u8>(),
                    );
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetCurrentKeyboardPage() -> u8 {
    unsafe {
        return ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
            .read();
    }
}
pub(crate) unsafe extern "C" fn GetCurrentKeyboardColAndRow(col: *mut u8, row: *mut u8) {
    unsafe {
        let mut col = col;
        let mut row = row;
        (col).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17)).read(),
        );
        (row).write(
            ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetMessageEntryBuffer() -> *mut u8 {
    unsafe {
        return ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(26))
            .cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn GetLengthOfMessageEntry() -> i32 {
    unsafe {
        let mut str: *mut u8 = GetMessageEntryBuffer();
        return ((StringLength_Multibyte(str)) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetBufferSelectionRegion(x: *mut u32, width: *mut u32) {
    unsafe {
        let mut x = x;
        let mut width = width;
        let mut diff: i32 = ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(21))
        .read()) as i32)
            .wrapping_sub(
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20))
                    .read()) as i32),
            );
        if diff < 0i32 {
            diff = (diff).wrapping_mul((-1i32));
            (x).write(
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
                    .read()) as u32),
            );
        } else {
            (x).write(
                ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20))
                    .read()) as u32),
            );
        }
        (width).write(((diff) as u32));
    }
}
pub(crate) unsafe extern "C" fn GetLimitedMessageStartPtr() -> *mut u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numChars: u16 = GetNumOverflowCharsInMessage();
        let mut str: *mut u8 = ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26))
        .cast::<u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numChars) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((str).read()) as i32) == 249i32 {
                        str = (str).wrapping_offset(1);
                    }
                    str = (str).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        return str;
    }
}
pub(crate) unsafe extern "C" fn GetLimitedMessageStartPos() -> u16 {
    unsafe {
        let mut count: u16 = 0u16;
        let mut i: u32 = 0u32;
        let mut numChars: u16 = GetNumOverflowCharsInMessage();
        let mut str: *mut u8 = ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26))
        .cast::<u8>();
        {
            count = 0u16;
            i = 0u32;
            'l1: loop {
                if !(i < ((numChars) as u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((str).read()) as i32) == 249i32 {
                        str = (str).wrapping_offset(1);
                    }
                    str = (str).wrapping_offset(1);
                }
                count = (count).wrapping_add(1);
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
pub(crate) unsafe extern "C" fn GetLastReceivedMessage() -> *mut u8 {
    unsafe {
        return ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(57))
            .cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn GetReceivedPlayerIndex() -> u8 {
    unsafe {
        return ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(22))
            .read();
    }
}
pub(crate) unsafe extern "C" fn GetTextEntryCursorPosition() -> i32 {
    unsafe {
        return ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
            .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetShouldShowCaseToggleIcon() -> i32 {
    unsafe {
        let mut str: *mut u8 = GetLastCharOfMessagePtr();
        let mut character: u32 = (((str).read()) as u32);
        if ((character > 255u32)
            || (((((((&raw const sCaseToggleTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((character) as i32) as isize))
            .read()) as u32)
                == character))
            || (((((((&raw const sCaseToggleTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((character) as i32) as isize))
            .read()) as i32)
                == 0i32)
        {
            return 3i32;
        } else {
            return 0i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetChatHostName() -> *mut u8 {
    unsafe {
        return ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(121))
            .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitUnionRoomChatRegisteredTexts() {
    unsafe {
        StringCopy(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .cast::<u8>(),
            (&raw mut gText_Hello).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(21))
            .cast::<u8>(),
            (&raw mut gText_Pokemon2).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(42))
            .cast::<u8>(),
            (&raw mut gText_Trade).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(63))
            .cast::<u8>(),
            (&raw mut gText_Battle).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(84))
            .cast::<u8>(),
            (&raw mut gText_Lets).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(105))
            .cast::<u8>(),
            (&raw mut gText_Ok).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(126))
            .cast::<u8>(),
            (&raw mut gText_Sorry).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(147))
            .cast::<u8>(),
            (&raw mut gText_YaySmileEmoji).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(168))
            .cast::<u8>(),
            (&raw mut gText_ThankYou).cast::<u8>(),
        );
        StringCopy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15496))
                .cast::<u8>())
            .wrapping_offset(189))
            .cast::<u8>(),
            (&raw mut gText_ByeBye).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ReceiveChatMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut buffer: *mut u8 = core::ptr::null_mut();
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    DestroyTask(taskId);
                    return;
                }
                (data).write(1i16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((data).wrapping_offset(4)).write(((GetLinkPlayerCount()) as i16));
                if ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                    .read()) as i32)
                    != ((((data).wrapping_offset(4)).read()) as i32)
                {
                    (data).write(2i16);
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                        .write(((((data).wrapping_offset(4)).read()) as u8));
                    return;
                }
                ((data).wrapping_offset(3)).write(((GetBlockReceivedStatus()) as i16));
                if (!((((data).wrapping_offset(3)).read()) != 0))
                    && ((Rfu_IsPlayerExchangeActive()) != 0)
                {
                    return;
                }
                ((data).wrapping_offset(1)).write(0i16);
                (data).write(3i16);
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                {
                    'l2: loop {
                        if !((((((data).wrapping_offset(1)).read()) as i32) < 5i32)
                            && ((crate::c::shr_i32(
                                ((((data).wrapping_offset(3)).read()) as i32),
                                ((((data).wrapping_offset(1)).read()) as u32),
                            ) & 1i32)
                                == 0i32))
                        {
                            break 'l2;
                        }
                        'l3: {}
                        let __p2 = (data).wrapping_offset(1);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                if ((((data).wrapping_offset(1)).read()) as i32) == 5i32 {
                    (data).write(1i16);
                    return;
                }
                ((data).wrapping_offset(2)).write(((data).wrapping_offset(1)).read());
                ResetBlockReceivedFlag(((((data).wrapping_offset(2)).read()) as u8));
                buffer = ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                    ((((data).wrapping_offset(1)).read()) as i32) as isize * 256,
                ))
                .cast::<u16>())
                .cast::<u8>();
                'l4: {
                    let __sw3 = (((buffer).read()) as i32);
                    let __matched = __sw3 == 1i32
                        || __sw3 == 2i32
                        || __sw3 == 3i32
                        || __sw3 == 4i32
                        || __sw3 == 5i32;
                    if __sw3 == 1i32 || !__matched {
                        ((data).wrapping_offset(5)).write(3i16);
                        break 'l4;
                    }
                    if __sw3 == 2i32 {
                        ((data).wrapping_offset(5)).write(3i16);
                        break 'l4;
                    }
                    if __sw3 == 3i32 {
                        ((data).wrapping_offset(5)).write(4i16);
                        break 'l4;
                    }
                    if __sw3 == 4i32 {
                        ((data).wrapping_offset(5)).write(5i16);
                        break 'l4;
                    }
                    if __sw3 == 5i32 {
                        ((data).wrapping_offset(5)).write(6i16);
                        break 'l4;
                    }
                }
                if (ProcessReceivedChatMessage(
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(57))
                        .cast::<u8>(),
                    ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(1)).read()) as i32) as isize * 256,
                    ))
                    .cast::<u16>())
                    .cast::<u8>(),
                )) != 0
                {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(22))
                        .write(((((data).wrapping_offset(1)).read()) as u8));
                    StartDisplaySubtask(12u16, 2u8);
                    (data).write(7i16);
                } else {
                    (data).write(((data).wrapping_offset(5)).read());
                }
                let __p4 = (data).wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                if !((IsDisplaySubtaskActive(2u8)) != 0) {
                    (data).write(((data).wrapping_offset(5)).read());
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (!((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(19))
                .read())
                    != 0))
                    && ((((data).wrapping_offset(2)).read()) != 0)
                {
                    if ((GetLinkPlayerCount()) as i32) == 2i32 {
                        Rfu_StopPartnerSearch();
                        ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(23))
                        .write(1u8);
                        DestroyTask(taskId);
                        return;
                    }
                    Rfu_DisconnectPlayerById(((((data).wrapping_offset(2)).read()) as u32));
                }
                (data).write(3i16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19))
                    .read())
                    != 0
                {
                    ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23))
                        .write(2u8);
                }
                DestroyTask(taskId);
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                ((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23))
                    .write(3u8);
                DestroyTask(taskId);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if !((Rfu_IsPlayerExchangeActive()) != 0) {
                    if !((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .read())
                        != 0)
                    {
                        SetUnionRoomChatPlayerData(
                            ((((((&raw mut sChat).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(13))
                            .read()) as u32),
                        );
                    }
                    (data).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryAllocDisplay() -> u8 {
    unsafe {
        ((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).write(Alloc(8552u32));
        if (!(((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).is_null())
            && ((TryAllocSprites()) != 0)
        {
            ResetBgsAndClearDma3BusyFlags(0u32);
            InitBgsFromTemplates(
                0u8,
                ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                ((crate::c::div_u32(16u32, 4u32)) as u8),
            );
            InitWindows(((&raw const sWinTemplates).cast::<u8>().cast_mut()).cast::<u8>());
            ResetTempTileDataBuffers();
            InitScanlineEffect();
            InitDisplay(((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read());
            ResetDisplaySubtasks();
            StartDisplaySubtask(0u16, 0u8);
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsDisplaySubtask0Active() -> u32 {
    unsafe {
        return ((IsDisplaySubtaskActive(0u8)) as u32);
    }
}
pub(crate) unsafe extern "C" fn FreeDisplay() {
    unsafe {
        FreeSprites();
        if ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            Free(((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        FreeAllWindowBuffers();
        (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
    }
}
pub(crate) unsafe extern "C" fn InitDisplay(display: *mut u8) {
    unsafe {
        let mut display = display;
        ((display).wrapping_add(24).cast::<u16>()).write(255u16);
        ((display).wrapping_add(30).cast::<u16>()).write(255u16);
        ((display).wrapping_add(26).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn ResetDisplaySubtasks() {
    unsafe {
        let mut i: i32 = 0i32;
        if !(!(((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(24u32, 8u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                    .write(Some(Display_Dummy));
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(4))
                    .write(0u8);
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(5))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RunDisplaySubtasks() {
    unsafe {
        let mut i: i32 = 0i32;
        if !(!(((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(24u32, 8u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(4))
                    .write(
                        (((((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                        .read())
                        .unwrap_unchecked()(
                            (((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 8))
                            .wrapping_add(5),
                        )) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartDisplaySubtask(subtaskId: u16, assignId: u8) {
    unsafe {
        let mut subtaskId = subtaskId;
        let mut assignId = assignId;
        let mut i: u32 = 0u32;
        ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
            .wrapping_offset(((assignId) as i32) as isize * 8))
        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(Display_Dummy));
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(168u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sDisplaySubtasks).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((subtaskId) as i32)
                    {
                        ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((assignId) as i32) as isize * 8))
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                        .write(
                            (((((&raw const sDisplaySubtasks).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                            .read(),
                        );
                        ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((assignId) as i32) as isize * 8))
                        .wrapping_add(4))
                        .write(1u8);
                        ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((assignId) as i32) as isize * 8))
                        .wrapping_add(5))
                        .write(0u8);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsDisplaySubtaskActive(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        return ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 8))
        .wrapping_add(4))
        .read();
    }
}
pub(crate) unsafe extern "C" fn Display_LoadGfx(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        if ((FreeTempTileDataBuffersIfPossible()) as i32) == 1i32 {
            return 1u32;
        }
        'l1: {
            let __sw1 = (((state).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                ResetGpuBgState();
                SetBgTilemapBuffers();
                break 'l1;
            }
            if __sw1 == 1i32 {
                ClearBg0();
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadKeyboardWindowGfx();
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadChatWindowGfx();
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadChatUnkPalette();
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadChatMessagesWindow();
                DrawKeyboardWindow();
                LoadKeyboardSwapWindow();
                LoadTextEntryWindow();
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CreateKeyboardCursorSprite();
                    CreateTextEntrySprites();
                    CreateRButtonSprites();
                }
                break 'l1;
            }
            if !__matched {
                return 0u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_ShowKeyboardSwapMenu(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                ShowKeyboardSwapMenu();
                CopyWindowToVram(3u8, 3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_HideKeyboardSwapMenu(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                HideKeyboardSwapMenu();
                CopyWindowToVram(3u8, 3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_SwitchPages(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                SetKeyboardCursorInvisibility(1u32);
                if (SlideKeyboardPageOut()) != 0 {
                    return 1u32;
                }
                PrintCurrentKeyboardPage();
                CopyWindowToVram(2u8, 2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (SlideKeyboardPageIn()) != 0 {
                    return 1u32;
                }
                MoveKeyboardCursor();
                SetKeyboardCursorInvisibility(0u32);
                UpdateRButtonLabel();
                return 0u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_MoveKeyboardCursor(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        MoveKeyboardCursor();
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Display_AskQuitChatting(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(0i32, 0u16);
                AddYesNoMenuAt(23u8, 11u8, 1u8);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_DestroyYesNoDialog(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                HideStdMessageWindow();
                HideYesNoMenuWindow();
                CopyBgTilemapBufferToVram(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 1u32;
                }
                DestroyStdMessageWindow();
                DestroyYesNoMenuWindow();
                return 0u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_UpdateMessageBuffer(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        let mut x: u32 = 0u32;
        let mut width: u32 = 0u32;
        let mut str: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                GetBufferSelectionRegion(&raw mut x, &raw mut width);
                FillTextEntryWindow(((x) as u16), ((width) as u16), 0u8);
                str = GetMessageEntryBuffer();
                DrawTextEntryMessage(0u16, str, 3u8, 1u8, 2u8);
                CopyWindowToVram(1u8, 2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    UpdateRButtonLabel();
                    return 0u32;
                }
                return 1u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_AskRegisterText(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        let mut x: u16 = 0u16;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut length: u16 = 0u16;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                x = GetLimitedMessageStartPos();
                str = GetLimitedMessageStartPtr();
                length = ((StringLength_Multibyte(str)) as u16);
                FillTextEntryWindow(x, length, 102u8);
                DrawTextEntryMessage(x, str, 0u8, 4u8, 5u8);
                CopyWindowToVram(1u8, 2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    AddStdMessageWindow(1i32, 16u16);
                    CopyWindowToVram(
                        ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(30)
                            .cast::<u16>())
                        .read()) as u8),
                        3u8,
                    );
                } else {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetRegisteredTextPalette(1u32);
                } else {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_CancelRegister(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        let mut x: u16 = 0u16;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut length: u16 = 0u16;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                x = GetLimitedMessageStartPos();
                str = GetLimitedMessageStartPtr();
                length = ((StringLength_Multibyte(str)) as u16);
                FillTextEntryWindow(x, length, 0u8);
                DrawTextEntryMessage(x, str, 3u8, 1u8, 2u8);
                CopyWindowToVram(1u8, 2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    HideStdMessageWindow();
                    CopyWindowToVram(
                        ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(30)
                            .cast::<u16>())
                        .read()) as u8),
                        3u8,
                    );
                } else {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetRegisteredTextPalette(0u32);
                    DestroyStdMessageWindow();
                } else {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_ReturnToKeyboard(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                PrintCurrentKeyboardPage();
                CopyWindowToVram(2u8, 2u8);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 1u32;
                } else {
                    return 0u32;
                }
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_ScrollChat(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        let mut row: u16 = 0u16;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut colorIdx: u8 = 0u8;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                row = ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(26)
                    .cast::<u16>())
                .read();
                str = GetLastReceivedMessage();
                colorIdx = GetReceivedPlayerIndex();
                PrintChatMessage(row, str, colorIdx);
                CopyWindowToVram(0u8, 2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 1u32;
                }
                if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(26)
                    .cast::<u16>())
                .read()) as i32)
                    < 9i32
                {
                    let __p2 = (((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(26)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    (state).write(4u8);
                    return 0u32;
                } else {
                    ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<u16>())
                    .write(0u16);
                    (state).write(((state).read()).wrapping_add(1));
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                ScrollWindow(0u8, 0u8, 5u8, 17u8);
                CopyWindowToVram(0u8, 2u8);
                let __p3 = (((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                (state).write(((state).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 1u32;
                }
                if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<u16>())
                .read()) as i32)
                    < 3i32
                {
                    (state).write(((state).read()).wrapping_sub(1));
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                return 0u32;
            }
            if !__matched {
                __fall = true;
                return 1u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_AnimateKeyboardCursor(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                StartKeyboardCursorAnim();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return TryKeyboardCursorReopen();
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_PrintInputText(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(3i32, 16u16);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_PrintExitingChat(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(4i32, 0u16);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_PrintLeaderLeft(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        let mut str: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                DynamicPlaceholderTextUtil_Reset();
                str = GetChatHostName();
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, str);
                AddStdMessageWindow(5i32, 0u16);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_AskSave(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(6i32, 0u16);
                AddYesNoMenuAt(23u8, 10u8, 1u8);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_AskOverwriteSave(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(7i32, 0u16);
                AddYesNoMenuAt(23u8, 10u8, 1u8);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_PrintSavingDontTurnOff(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(8i32, 0u16);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_PrintSavedTheGame(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                DynamicPlaceholderTextUtil_Reset();
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                    0u8,
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                AddStdMessageWindow(9i32, 0u16);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_AskConfirmLeaderLeave(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                AddStdMessageWindow(10i32, 0u16);
                AddYesNoMenuAt(23u8, 10u8, 1u8);
                CopyWindowToVram(
                    ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(30)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return ((IsDma3ManagerBusyWithBgCopy()) as u32);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Display_Dummy(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn AddYesNoMenuAt(left: u8, top: u8, initialCursorPos: u8) {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut initialCursorPos = initialCursorPos;
        let mut template = crate::ffi::Align4([0u8; 8]);
        ((&raw mut template).cast::<u8>()).write(0u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(1)).write(left);
        (((&raw mut template).cast::<u8>()).wrapping_add(2)).write(top);
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(6u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(4u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(5)).write(14u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(82u16);
        ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<u16>())
        .write(AddWindow((&raw mut template).cast::<u8>()));
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            FillWindowPixelBuffer(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
                17u8,
            );
            PutWindowTilemap(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
            );
            AddTextPrinterParameterized(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
                1u8,
                (&raw mut gText_Yes).cast::<u8>(),
                8u8,
                1u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
                1u8,
                (&raw mut gText_No).cast::<u8>(),
                8u8,
                17u8,
                255u8,
                None,
            );
            DrawTextBorderOuter(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
                1u16,
                13u8,
            );
            InitMenuInUpperLeftCornerNormal(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
                2u8,
                initialCursorPos,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn HideYesNoMenuWindow() {
    unsafe {
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            ClearStdWindowAndFrameToTransparent(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
                0u8,
            );
            ClearWindowTilemap(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyYesNoMenuWindow() {
    unsafe {
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            RemoveWindow(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read()) as u8),
            );
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<u16>())
            .write(255u16);
        }
    }
}
pub(crate) unsafe extern "C" fn ProcessMenuInput() -> i8 {
    unsafe {
        return Menu_ProcessInput();
    }
}
pub(crate) unsafe extern "C" fn AddStdMessageWindow(msgId: i32, bg0vofs: u16) {
    unsafe {
        let mut msgId = msgId;
        let mut bg0vofs = bg0vofs;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut windowId: i32 = 0i32;
        let mut template = crate::ffi::Align4([0u8; 8]);
        ((&raw mut template).cast::<u8>()).write(0u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(1)).write(8u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(2)).write(16u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(21u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(4u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(5)).write(14u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(106u16);
        if ((((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((msgId) as isize * 12))
        .wrapping_add(10))
        .read())
            != 0
        {
            let __p1 = ((&raw mut template).cast::<u8>()).wrapping_add(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(7i32)) as u8));
            let __p2 = ((&raw mut template).cast::<u8>()).wrapping_add(3);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(7i32)) as u8));
        }
        ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(30)
            .cast::<u16>())
        .write(AddWindow((&raw mut template).cast::<u8>()));
        windowId = ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(30)
            .cast::<u16>())
        .read()) as i32);
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(30)
            .cast::<u16>())
        .read()) as i32)
            == 255i32
        {
            return;
        }
        if ((((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((msgId) as isize * 12))
        .wrapping_add(9))
        .read())
            != 0
        {
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(34))
                    .cast::<u8>(),
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .cast::<*mut u8>())
                .read(),
            );
            str = ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(34))
                .cast::<u8>();
        } else {
            str = (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((msgId) as isize * 12))
            .cast::<*mut u8>())
            .read();
        }
        ChangeBgY(0u8, ((bg0vofs) as i32).wrapping_mul(256i32), 0u8);
        FillWindowPixelBuffer(((windowId) as u8), 17u8);
        PutWindowTilemap(((windowId) as u8));
        if (((((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((msgId) as isize * 12))
        .wrapping_add(4))
        .read()) as i32)
            == 1i32
        {
            DrawTextBorderInner(((windowId) as u8), 10u16, 2u8);
            AddTextPrinterParameterized5(
                ((windowId) as u8),
                1u8,
                str,
                (((((((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(5))
                .read()) as i32)
                    .wrapping_add(8i32)) as u8),
                (((((((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(6))
                .read()) as i32)
                    .wrapping_add(8i32)) as u8),
                255u8,
                None,
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(7))
                .read(),
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(8))
                .read(),
            );
        } else {
            DrawTextBorderOuter(((windowId) as u8), 10u16, 2u8);
            AddTextPrinterParameterized5(
                ((windowId) as u8),
                1u8,
                str,
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(5))
                .read(),
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(6))
                .read(),
                255u8,
                None,
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(7))
                .read(),
                (((((&raw const sDisplayStdMessages).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((msgId) as isize * 12))
                .wrapping_add(8))
                .read(),
            );
        }
        ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(30)
            .cast::<u16>())
        .write(((windowId) as u16));
    }
}
pub(crate) unsafe extern "C" fn HideStdMessageWindow() {
    unsafe {
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(30)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            ClearStdWindowAndFrameToTransparent(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(30)
                    .cast::<u16>())
                .read()) as u8),
                0u8,
            );
            ClearWindowTilemap(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(30)
                    .cast::<u16>())
                .read()) as u8),
            );
        }
        ChangeBgY(0u8, 0i32, 0u8);
    }
}
pub(crate) unsafe extern "C" fn DestroyStdMessageWindow() {
    unsafe {
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(30)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            RemoveWindow(
                ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(30)
                    .cast::<u16>())
                .read()) as u8),
            );
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(30)
                .cast::<u16>())
            .write(255u16);
        }
    }
}
pub(crate) unsafe extern "C" fn FillTextEntryWindow(x: u16, width: u16, fillValue: u8) {
    unsafe {
        let mut x = x;
        let mut width = width;
        let mut fillValue = fillValue;
        FillWindowPixelRect(
            1u8,
            fillValue,
            ((((x) as i32).wrapping_mul(8i32)) as u16),
            1u16,
            ((((width) as i32).wrapping_mul(8i32)) as u16),
            14u16,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawTextEntryMessage(
    x: u16,
    str: *mut u8,
    bgColor: u8,
    fgColor: u8,
    shadowColor: u8,
) {
    unsafe {
        let mut x = x;
        let mut str = str;
        let mut bgColor = bgColor;
        let mut fgColor = fgColor;
        let mut shadowColor = shadowColor;
        let mut color = crate::ffi::Align4([0u8; 3]);
        let mut strBuffer = crate::ffi::Align4([0u8; 35]);
        if ((bgColor) as i32) != 0i32 {
            FillTextEntryWindow(
                x,
                (((GetTextEntryCursorPosition()).wrapping_sub(((x) as i32))) as u16),
                bgColor,
            );
        }
        ((&raw mut color).cast::<u8>()).write(bgColor);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(fgColor);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(shadowColor);
        ((&raw mut strBuffer).cast::<u8>()).write(252u8);
        (((&raw mut strBuffer).cast::<u8>()).wrapping_offset(1)).write(20u8);
        (((&raw mut strBuffer).cast::<u8>()).wrapping_offset(2)).write(8u8);
        StringCopy(((&raw mut strBuffer).cast::<u8>()).wrapping_offset(3), str);
        AddTextPrinterParameterized3(
            1u8,
            2u8,
            ((((x) as i32).wrapping_mul(8i32)) as u8),
            1u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            (&raw mut strBuffer).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintCurrentKeyboardPage() {
    unsafe {
        let mut page: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut left: u16 = 0u16;
        let mut top: u16 = 0u16;
        let mut color = crate::ffi::Align4([0u8; 3]);
        let mut str = crate::ffi::Align4([0u8; 45]);
        let mut str2: *mut u8 = core::ptr::null_mut();
        FillWindowPixelBuffer(2u8, 255u8);
        page = GetCurrentKeyboardPage();
        ((&raw mut color).cast::<u8>()).write(0u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(14u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(13u8);
        if ((page) as i32) != 3i32 {
            ((&raw mut str).cast::<u8>()).write(252u8);
            (((&raw mut str).cast::<u8>()).wrapping_offset(1)).write(20u8);
            (((&raw mut str).cast::<u8>()).wrapping_offset(2)).write(8u8);
            if ((page) as i32) == 2i32 {
                left = 6u16;
            } else {
                left = 8u16;
            }
            {
                i = 0i32;
                top = 0u16;
                'l1: loop {
                    if !(i < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        if !(!(((((((&raw const sUnionRoomKeyboardText)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((page) as i32) as isize * 40))
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read())
                        .is_null())
                        {
                            return;
                        }
                        StringCopy(
                            ((&raw mut str).cast::<u8>()).wrapping_offset(3),
                            ((((((&raw const sUnionRoomKeyboardText).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((page) as i32) as isize * 40))
                            .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                        AddTextPrinterParameterized3(
                            2u8,
                            0u8,
                            ((left) as u8),
                            ((top) as u8),
                            (&raw mut color).cast::<u8>(),
                            (-1i8),
                            (&raw mut str).cast::<u8>(),
                        );
                    }
                    i = (i).wrapping_add(1);
                    top = ((((top) as i32).wrapping_add(12i32)) as u16);
                }
            }
        } else {
            left = 4u16;
            {
                i = 0i32;
                top = 0u16;
                'l3: loop {
                    if !(i < 10i32) {
                        break 'l3;
                    }
                    'l4: {
                        str2 = GetRegisteredTextByRow(i);
                        if GetStringWidth(0u8, str2, 0i16) <= 40i32 {
                            AddTextPrinterParameterized3(
                                2u8,
                                0u8,
                                ((left) as u8),
                                ((top) as u8),
                                (&raw mut color).cast::<u8>(),
                                (-1i8),
                                str2,
                            );
                        } else {
                            let mut length: i32 = ((StringLength_Multibyte(str2)) as i32);
                            'l5: loop {
                                'l6: {
                                    length = (length).wrapping_sub(1);
                                    StringCopyN_Multibyte(
                                        (&raw mut str).cast::<u8>(),
                                        str2,
                                        ((length) as u32),
                                    );
                                }
                                if !(GetStringWidth(0u8, (&raw mut str).cast::<u8>(), 0i16) > 35i32)
                                {
                                    break 'l5;
                                }
                            }
                            AddTextPrinterParameterized3(
                                2u8,
                                0u8,
                                ((left) as u8),
                                ((top) as u8),
                                (&raw mut color).cast::<u8>(),
                                (-1i8),
                                (&raw mut str).cast::<u8>(),
                            );
                            AddTextPrinterParameterized3(
                                2u8,
                                0u8,
                                ((((left) as i32).wrapping_add(35i32)) as u8),
                                ((top) as u8),
                                (&raw mut color).cast::<u8>(),
                                (-1i8),
                                ((&raw const sText_Ellipsis).cast::<u8>().cast_mut()).cast::<u8>(),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                    top = ((((top) as i32).wrapping_add(12i32)) as u16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlideKeyboardPageOut() -> u32 {
    unsafe {
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .read()) as i32)
            < 56i32
        {
            let __p1 = (((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(12i32)) as i16));
            if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read()) as i32)
                >= 56i32
            {
                ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<i16>())
                .write(56i16);
            }
            if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read()) as i32)
                < 56i32
            {
                UpdateSlidingKeyboard(
                    ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<i16>())
                    .read(),
                );
                return 1u32;
            }
        }
        FinishSlidingKeyboard(
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read(),
        );
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SlideKeyboardPageIn() -> u32 {
    unsafe {
        if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .read()) as i32)
            > 0i32
        {
            let __p1 = (((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(12i32)) as i16));
            if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read()) as i32)
                <= 0i32
            {
                ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<i16>())
                .write(0i16);
            }
            if ((((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read()) as i32)
                > 0i32
            {
                UpdateSlidingKeyboard(
                    ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<i16>())
                    .read(),
                );
                return 1u32;
            }
        }
        FinishSlidingKeyboard(
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read(),
        );
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ShowKeyboardSwapMenu() {
    unsafe {
        FillWindowPixelBuffer(3u8, 17u8);
        DrawTextBorderOuter(3u8, 1u16, 13u8);
        PrintMenuActionTextsAtPos(
            3u8,
            2u8,
            8u8,
            1u8,
            14u8,
            ((crate::c::div_u32(40u32, 8u32)) as u8),
            ((&raw const sKeyboardPageTitleTexts).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        InitMenuNormal(3u8, 2u8, 0u8, 1u8, 14u8, 5u8, GetCurrentKeyboardPage());
        PutWindowTilemap(3u8);
    }
}
pub(crate) unsafe extern "C" fn HideKeyboardSwapMenu() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(3u8, 0u8);
        ClearWindowTilemap(3u8);
    }
}
pub(crate) unsafe extern "C" fn PrintChatMessage(row: u16, str: *mut u8, colorIdx: u8) {
    unsafe {
        let mut row = row;
        let mut str = str;
        let mut colorIdx = colorIdx;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(1u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1))
            .write((((((colorIdx) as i32).wrapping_mul(2i32)).wrapping_add(2i32)) as u8));
        (((&raw mut color).cast::<u8>()).wrapping_offset(2))
            .write((((((colorIdx) as i32).wrapping_mul(2i32)).wrapping_add(3i32)) as u8));
        FillWindowPixelRect(
            0u8,
            17u8,
            0u16,
            ((((row) as i32).wrapping_mul(15i32)) as u16),
            168u16,
            15u16,
        );
        AddTextPrinterParameterized3(
            0u8,
            2u8,
            0u8,
            (((((row) as i32).wrapping_mul(15i32)).wrapping_add(1i32)) as u8),
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn ResetGpuBgState() {
    unsafe {
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
        SetGpuRegBits(0u8, 4160u16);
        SetGpuReg(80u8, 0u16);
        ClearGpuRegBits(0u8, 57344u16);
        SetGpuRegBits(0u8, 8192u16);
        SetGpuReg(64u8, 16624u16);
        SetGpuReg(68u8, 144u16);
        SetGpuReg(72u8, 61u16);
        SetGpuReg(74u8, 63u16);
    }
}
pub(crate) unsafe extern "C" fn SetBgTilemapBuffers() {
    unsafe {
        SetBgTilemapBuffer(
            0u8,
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(296))
                .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2344))
                .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4392))
                .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6440))
                .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn ClearBg0() {
    unsafe {
        RequestDma3Fill(0i32, ((100663296i32) as usize as *mut u8), 32u16, 1u8);
        FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn LoadKeyboardWindowGfx() {
    unsafe {
        LoadPalette(
            (((&raw mut gUnionRoomChat_Keyboard_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            112u16,
            32u16,
        );
        LoadPalette(
            (((&raw mut gUnionRoomChat_InputText_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            192u16,
            32u16,
        );
        DecompressAndCopyTileDataToVram(
            1u8,
            (((&raw mut gUnionRoomChat_Keyboard_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw mut gUnionRoomChat_Keyboard_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u16,
            0u16,
        );
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn LoadChatWindowGfx() {
    unsafe {
        let mut ptr: *mut u8 = core::ptr::null_mut();
        LoadPalette(
            (((&raw mut gUnionRoomChat_Background_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            0u16,
            32u16,
        );
        ptr = DecompressAndCopyTileDataToVram(
            2u8,
            (((&raw mut gUnionRoomChat_Background_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        if !(ptr).is_null() {
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (ptr).wrapping_offset(
                            ((17i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as isize,
                        ),
                        (((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8488))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(0i32)) as isize,
                        ),
                        ((crate::c::div_i32(
                            crate::c::div_i32(256i32, 8i32),
                            crate::c::div_i32(32i32, 8i32),
                        ) & 2097151i32) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            'l3: loop {
                'l4: {
                    CpuFastSet(
                        (ptr).wrapping_offset(
                            ((33i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as isize,
                        ),
                        (((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8488))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(1i32)) as isize,
                        ),
                        ((crate::c::div_i32(
                            crate::c::div_i32(256i32, 8i32),
                            crate::c::div_i32(32i32, 8i32),
                        ) & 2097151i32) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l3;
                }
            }
        }
        CopyToBgTilemapBuffer(
            2u8,
            (((&raw mut gUnionRoomChat_Background_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
            0u16,
            0u16,
        );
        CopyBgTilemapBufferToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn LoadChatUnkPalette() {
    unsafe {
        LoadPalette(
            (((&raw const sUnusedPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            128u16,
            32u16,
        );
        RequestDma3Fill(
            0i32,
            ((100679680i32) as usize as *mut u8)
                .wrapping_offset((crate::c::div_i32(256i32, 8i32)) as isize * 1),
            ((crate::c::div_i32(256i32, 8i32)) as u16),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadChatMessagesWindow() {
    unsafe {
        LoadPalette(
            (((&raw const sChatMessagesWindow_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        PutWindowTilemap(0u8);
        FillWindowPixelBuffer(0u8, 17u8);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn DrawKeyboardWindow() {
    unsafe {
        PutWindowTilemap(2u8);
        PrintCurrentKeyboardPage();
        CopyWindowToVram(2u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn LoadTextEntryWindow() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut unused = crate::ffi::Align4([0u8; 2]);
        ((&raw mut unused).cast::<u8>()).write(0u8);
        (((&raw mut unused).cast::<u8>()).wrapping_offset(1)).write(255u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 15i32) {
                    break 'l1;
                }
                'l2: {
                    BlitBitmapToWindow(
                        1u8,
                        ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8488))
                        .cast::<u8>(),
                        (((i).wrapping_mul(8i32)) as u16),
                        0u16,
                        8u16,
                        16u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        FillWindowPixelBuffer(1u8, 0u8);
        PutWindowTilemap(1u8);
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn LoadKeyboardSwapWindow() {
    unsafe {
        FillWindowPixelBuffer(3u8, 17u8);
        LoadUserWindowBorderGfx(3u8, 1u16, 208u8);
        LoadUserWindowBorderGfx_(3u8, 10u16, 32u8);
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            224u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn InitScanlineEffect() {
    unsafe {
        let mut params = crate::ffi::Align4([0u8; 12]);
        (((&raw mut params).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(2724200449u32);
        (((&raw mut params).cast::<u8>()).cast::<*mut u8>())
            .write(((67108884i32) as usize as *mut u16).cast::<u8>());
        (((&raw mut params).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut params).cast::<u8>()).wrapping_add(9)).write(0u8);
        ((((&raw mut sDisplay).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .write(0i16);
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (&raw mut gScanlineEffectRegBuffers).cast::<u8>(),
                        (16777216u32
                            | (crate::c::div_u32(
                                3840u32,
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32)),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        ScanlineEffect_SetParams(
            (&raw mut params)
                .cast::<u8>()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateSlidingKeyboard(bg1hofs: i16) {
    unsafe {
        let mut bg1hofs = bg1hofs;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(((bg1hofs) as u16));
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gScanlineEffect).cast::<u8>())
                                            .wrapping_add(20))
                                        .read()) as i32)
                                            as isize
                                            * 1920,
                                    ))
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(288i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gScanlineEffect).cast::<u8>())
                                            .wrapping_add(20))
                                        .read()) as i32)
                                            as isize
                                            * 1920,
                                    ))
                                .cast::<u16>())
                                .wrapping_offset(144))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FinishSlidingKeyboard(bg1hofs: i16) {
    unsafe {
        let mut bg1hofs = bg1hofs;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(((bg1hofs) as u16));
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                    .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(288i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .cast::<u16>())
                                .wrapping_offset(144))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(((bg1hofs) as u16));
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .cast::<u16>())
                                .wrapping_offset(960))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(288i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        'l13: loop {
            'l14: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l15: loop {
                        'l16: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .cast::<u16>())
                                .wrapping_offset(1104))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l13;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryAllocSprites() -> u32 {
    unsafe {
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(40u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        LoadSpritePalette((&raw const sSpritePalette).cast::<u8>().cast_mut());
        ((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).write(Alloc(24u32));
        if !(!(((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn FreeSprites() {
    unsafe {
        if !(((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
            Free(((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn CreateKeyboardCursorSprite() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_KeyboardCursor)
                .cast::<u8>()
                .cast_mut(),
            10i16,
            24i16,
            0u8,
        );
        ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
    }
}
pub(crate) unsafe extern "C" fn SetKeyboardCursorInvisibility(invisible: u32) {
    unsafe {
        let mut invisible = invisible;
        crate::c::bf_write(
            (((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor() {
    unsafe {
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut page: u8 = GetCurrentKeyboardPage();
        GetCurrentKeyboardColAndRow(&raw mut x, &raw mut y);
        if ((page) as i32) != 3i32 {
            StartSpriteAnim(
                ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read(),
                0u8,
            );
            ((((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write((((((x) as i32).wrapping_mul(8i32)).wrapping_add(10i32)) as i16));
            ((((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((y) as i32).wrapping_mul(12i32)).wrapping_add(24i32)) as i16));
        } else {
            StartSpriteAnim(
                ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read(),
                2u8,
            );
            ((((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(24i16);
            ((((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((y) as i32).wrapping_mul(12i32)).wrapping_add(24i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn SetRegisteredTextPalette(registering: u32) {
    unsafe {
        let mut registering = registering;
        let mut palette: *mut u16 = (((&raw const sUnionRoomChatInterfacePal)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((((registering).wrapping_mul(2u32)).wrapping_add(1u32)) as i32) as isize);
        let mut index: u8 = IndexOfSpritePaletteTag(0u16);
        LoadPalette(
            (palette).cast::<u8>(),
            ((((256i32).wrapping_add(((index) as i32).wrapping_mul(16i32))).wrapping_add(1i32))
                as u16),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn StartKeyboardCursorAnim() {
    unsafe {
        if ((GetCurrentKeyboardPage()) as i32) != 3i32 {
            StartSpriteAnim(
                ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read(),
                1u8,
            );
        } else {
            StartSpriteAnim(
                ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read(),
                3u8,
            );
        }
        ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn TryKeyboardCursorReopen() -> u32 {
    unsafe {
        if ((((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .read()) as i32)
            > 3i32
        {
            return 0u32;
        }
        if (({
            let __p1 = (((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            if ((GetCurrentKeyboardPage()) as i32) != 3i32 {
                StartSpriteAnim(
                    ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read(),
                    0u8,
                );
            } else {
                StartSpriteAnim(
                    ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read(),
                    2u8,
                );
            }
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn CreateTextEntrySprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_TextEntryCursor)
                .cast::<u8>()
                .cast_mut(),
            76i16,
            152i16,
            2u8,
        );
        ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_TextEntryArrow)
                .cast::<u8>()
                .cast_mut(),
            64i16,
            152i16,
            1u8,
        );
        ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TextEntryCursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut pos: i32 = GetTextEntryCursorPosition();
        if pos == 15i32 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((pos).wrapping_mul(8i32)).wrapping_add(76i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TextEntryArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 4i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            if (({
                let __p3 = (sprite).wrapping_add(36).cast::<i16>();
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 4i32
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateRButtonSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_RButtonIcon)
                .cast::<u8>()
                .cast_mut(),
            8i16,
            152i16,
            3u8,
        );
        ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_RButtonLabels)
                .cast::<u8>()
                .cast_mut(),
            32i16,
            152i16,
            4u8,
        );
        ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateRButtonLabel() {
    unsafe {
        if ((GetCurrentKeyboardPage()) as i32) == 3i32 {
            if GetLengthOfMessageEntry() != 0i32 {
                crate::c::bf_write(
                    (((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                StartSpriteAnim(
                    ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read(),
                    3u8,
                );
            } else {
                crate::c::bf_write(
                    (((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
        } else {
            let mut anim: i32 = GetShouldShowCaseToggleIcon();
            if anim == 3i32 {
                crate::c::bf_write(
                    (((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    (((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                StartSpriteAnim(
                    ((((&raw mut sSprites).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read(),
                    ((anim) as u8),
                );
            }
        }
    }
}
