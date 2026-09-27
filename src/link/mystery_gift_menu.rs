//! Translated from `src/mystery_gift_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sTextboxBorder_Pal sTextboxBorder_Gfx sBGTemplates sMainWindows sWindowTemplate_YesNoMsg_Wide sWindowTemplate_YesNoMsg sWindowTemplate_GiftSelect sWindowTemplate_ThreeOptions sWindowTemplate_YesNoBox sWindowTemplate_GiftSelect_3Options sWindowTemplate_GiftSelect_2Options sWindowTemplate_GiftSelect_1Option sListMenuItems_CardsOrNews sListMenuItems_WirelessOrFriend sListMenuTemplate_ThreeOptions sListMenuItems_ReceiveSendToss sListMenuItems_ReceiveToss sListMenuItems_ReceiveSend sListMenuItems_Receive sListMenu_ReceiveSendToss sListMenu_ReceiveToss sListMenu_ReceiveSend sListMenu_Receive sUnusedMenuTexts sTextColors_Header sTextColors_Header_Copy sMG_Ereader_TextColor_2
#[allow(unused_imports)]
use crate::data::mystery_gift_menu::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDownArrowCounterAndYCoordIdx: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gGiftIsFromEReader: u8 = 0u8;

unsafe extern "C" {
    static mut gJPText_DecideStop: u8;
    static mut gJPText_MysteryGift: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AlreadyHadCard: u8;
    static mut gText_AlreadyHadNews: u8;
    static mut gText_AlreadyHadStamp: u8;
    static mut gText_CantAcceptCardFromTrainer: u8;
    static mut gText_CantAcceptNewsFromTrainer: u8;
    static mut gText_CantSendGiftToTrainer: u8;
    static mut gText_Communicating: u8;
    static mut gText_CommunicationCanceled: u8;
    static mut gText_CommunicationCompleted: u8;
    static mut gText_CommunicationError: u8;
    static mut gText_DataWillBeSaved: u8;
    static mut gText_DontHaveCardNewOneInput: u8;
    static mut gText_DontHaveNewsNewOneInput: u8;
    static mut gText_GiftSentTo: u8;
    static mut gText_HaventReceivedCardsGift: u8;
    static mut gText_HaventReceivedGiftOkayToDiscard: u8;
    static mut gText_IfThrowAwayCardEventWontHappen: u8;
    static mut gText_MysteryGift: u8;
    static mut gText_NewStampReceived: u8;
    static mut gText_NewTrainerReceived: u8;
    static mut gText_NoMoreRoomForStamps: u8;
    static mut gText_NothingSentOver: u8;
    static mut gText_OkayToDiscardNews: u8;
    static mut gText_OtherTrainerCanceled: u8;
    static mut gText_OtherTrainerHasCard: u8;
    static mut gText_OtherTrainerHasNews: u8;
    static mut gText_OtherTrainerHasStamp: u8;
    static mut gText_PickOKCancel: u8;
    static mut gText_PickOKExit: u8;
    static mut gText_RecordUploadedViaWireless: u8;
    static mut gText_SaveCompletedPressA: u8;
    static mut gText_SendingWonderCard: u8;
    static mut gText_SendingWonderNews: u8;
    static mut gText_StampSentTo: u8;
    static mut gText_ThrowAwayWonderCard: u8;
    static mut gText_WhatToDoWithCards: u8;
    static mut gText_WhatToDoWithNews: u8;
    static mut gText_WhereShouldCardBeAccessed: u8;
    static mut gText_WhereShouldNewsBeAccessed: u8;
    static mut gText_WonderCardReceived: u8;
    static mut gText_WonderCardReceivedFrom: u8;
    static mut gText_WonderCardSentTo: u8;
    static mut gText_WonderCardThrownAway: u8;
    static mut gText_WonderNewsReceived: u8;
    static mut gText_WonderNewsReceivedFrom: u8;
    static mut gText_WonderNewsSentTo: u8;
    static mut gText_WonderNewsThrownAway: u8;
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
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
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoMysteryGiftListMenu(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16, a4: u16) -> i32;
    fn DrawDownArrow(a0: u8, a1: u16, a2: u16, a3: u8, a4: u8, a5: *mut u8, a6: *mut u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn EnableInterrupts(a0: u16);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetSavedWonderCard() -> *mut u8;
    fn GetSavedWonderCardMetadata() -> *mut u8;
    fn GetSavedWonderNews() -> *mut u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn Intl_GetListMenuWidth(a0: *mut u8) -> i32;
    fn IsFanfareTaskInactive() -> u8;
    fn IsSavedWonderCardGiftNotReceived() -> u32;
    fn IsSendingSavedWonderCardAllowed() -> u32;
    fn IsSendingSavedWonderNewsAllowed() -> u32;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn Menu_LoadStdPalAt(a0: u16);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MysterGiftServer_CreateForCard();
    fn MysterGiftServer_CreateForNews();
    fn MysterGiftServer_Run(a0: *mut u16) -> u32;
    fn MysteryGiftClient_AdvanceState();
    fn MysteryGiftClient_Create(a0: u32);
    fn MysteryGiftClient_GetMsg() -> *mut u8;
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
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
    fn WonderCard_Init(a0: *mut u8, a1: *mut u8) -> u32;
    fn WonderNews_AddScrollIndicatorArrowPair();
    fn WonderNews_Destroy();
    fn WonderNews_Enter() -> i32;
    fn WonderNews_Exit(a0: u32) -> i32;
    fn WonderNews_GetInput(a0: u16) -> u32;
    fn WonderNews_Init(a0: *mut u8) -> u32;
    fn WonderNews_RemoveScrollIndicatorArrowPair();
    fn WonderNews_SetReward(a0: u32);
    fn rbox_fill_rectangle(a0: u8);
}

pub(crate) unsafe extern "C" fn VBlankCB_MysteryGiftEReader() {
    unsafe {
        ProcessSpriteCopyRequests();
        LoadOam();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_MysteryGiftEReader() {
    unsafe {
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
    }
}
pub(crate) unsafe extern "C" fn HandleMysteryGiftOrEReaderSetup(isEReader: i32) -> u32 {
    unsafe {
        let mut isEReader = isEReader;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ResetPaletteFade();
                ResetSpriteData();
                FreeAllSpritePalettes();
                ResetTasks();
                ScanlineEffect_Stop();
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBGTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                SetBgTilemapBuffer(3u8, Alloc(2048u32));
                SetBgTilemapBuffer(2u8, Alloc(2048u32));
                SetBgTilemapBuffer(1u8, Alloc(2048u32));
                SetBgTilemapBuffer(0u8, Alloc(2048u32));
                LoadMysteryGiftTextboxBorder(3u8);
                InitWindows(((&raw const sMainWindows).cast::<u8>().cast_mut()).cast::<u8>());
                DeactivateAllTextPrinters();
                ClearGpuRegBits(0u8, 24576u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadPalette(
                    (((&raw const sTextboxBorder_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    32u16,
                );
                LoadPalette((GetTextWindowPalette(2u8)).cast::<u8>(), 208u16, 32u16);
                Menu_LoadStdPalAt(192u16);
                LoadUserWindowBorderGfx(0u8, 10u16, 224u8);
                LoadUserWindowBorderGfx_(0u8, 1u16, 240u8);
                FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 32u8, 17u8);
                FillBgTilemapBufferRect(1u8, 0u16, 0u8, 0u8, 32u8, 32u8, 17u8);
                FillBgTilemapBufferRect(2u8, 0u16, 0u8, 0u8, 32u8, 32u8, 17u8);
                MG_DrawCheckerboardPattern(3u32);
                PrintMysteryGiftOrEReaderHeader(((isEReader) as u8), 0u32);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                CopyBgTilemapBufferToVram(3u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(0u8);
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ShowBg(0u8);
                ShowBg(3u8);
                PlayBGM(541u16);
                SetVBlankCallback(Some(VBlankCB_MysteryGiftEReader));
                EnableInterrupts(197u16);
                return 1u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitMysteryGift() {
    unsafe {
        if (HandleMysteryGiftOrEReaderSetup(0i32)) != 0 {
            SetMainCallback2(Some(CB2_MysteryGiftEReader));
            ((&raw mut gGiftIsFromEReader).cast::<u8>().cast::<u8>()).write(0u8);
            CreateMysteryGiftTask();
        }
        RunTasks();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitEReader() {
    unsafe {
        if (HandleMysteryGiftOrEReaderSetup(1i32)) != 0 {
            SetMainCallback2(Some(CB2_MysteryGiftEReader));
            ((&raw mut gGiftIsFromEReader).cast::<u8>().cast::<u8>()).write(1u8);
            CreateEReaderTask();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MainCB_FreeAllBuffersAndReturnToInitTitleScreen() {
    unsafe {
        ((&raw mut gGiftIsFromEReader).cast::<u8>().cast::<u8>()).write(0u8);
        FreeAllWindowBuffers();
        Free(GetBgTilemapBuffer(0u8));
        Free(GetBgTilemapBuffer(1u8));
        Free(GetBgTilemapBuffer(2u8));
        Free(GetBgTilemapBuffer(3u8));
        SetMainCallback2(Some(CB2_InitTitleScreen));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMysteryGiftOrEReaderHeader(isEReader: u8, useCancel: u32) {
    unsafe {
        let mut isEReader = isEReader;
        let mut useCancel = useCancel;
        let mut title: *mut u8 = core::ptr::null_mut();
        let mut options: *mut u8 = core::ptr::null_mut();
        FillWindowPixelBuffer(0u8, 0u8);
        if !((isEReader) != 0) {
            title = (&raw mut gText_MysteryGift).cast::<u8>();
            options = (if !((useCancel) != 0) {
                (&raw mut gText_PickOKExit).cast::<u8>()
            } else {
                (&raw mut gText_PickOKCancel).cast::<u8>()
            });
        } else {
            title = (&raw mut gJPText_MysteryGift).cast::<u8>();
            options = (&raw mut gJPText_DecideStop).cast::<u8>();
        }
        AddTextPrinterParameterized4(
            0u8,
            1u8,
            4u8,
            1u8,
            0u8,
            0u8,
            ((&raw const sTextColors_Header).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            title,
        );
        AddTextPrinterParameterized4(
            0u8,
            0u8,
            ((GetStringRightAlignXOffset(0i32, options, 222i32)) as u8),
            1u8,
            0u8,
            0u8,
            ((&raw const sTextColors_Header).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            options,
        );
        CopyWindowToVram(0u8, 2u8);
        PutWindowTilemap(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MG_DrawTextBorder(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        DrawTextBorderOuter(windowId, 1u16, 15u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MG_DrawCheckerboardPattern(bg: u32) {
    unsafe {
        let mut bg = bg;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        FillBgTilemapBufferRect(((bg) as u8), 3u16, 0u8, 0u8, 32u8, 2u8, 17u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 18i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (i & 1i32) != (j & 1i32) {
                                    FillBgTilemapBufferRect(
                                        ((bg) as u8),
                                        1u16,
                                        ((j) as u8),
                                        (((i).wrapping_add(2i32)) as u8),
                                        1u8,
                                        1u8,
                                        17u8,
                                    );
                                } else {
                                    FillBgTilemapBufferRect(
                                        ((bg) as u8),
                                        2u16,
                                        ((j) as u8),
                                        (((i).wrapping_add(2i32)) as u8),
                                        1u8,
                                        1u8,
                                        17u8,
                                    );
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearScreenInBg0(ignoreTopTwoRows: u32) {
    unsafe {
        let mut ignoreTopTwoRows = ignoreTopTwoRows;
        'l1: {
            let __sw1 = ignoreTopTwoRows;
            if __sw1 == 0u32 {
                FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 32u8, 17u8);
                break 'l1;
            }
            if __sw1 == 1u32 {
                FillBgTilemapBufferRect(0u8, 0u16, 0u8, 2u8, 32u8, 30u8, 17u8);
                break 'l1;
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MG_AddMessageTextPrinter(str: *mut u8) {
    unsafe {
        let mut str = str;
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str);
        FillWindowPixelBuffer(1u8, 17u8);
        AddTextPrinterParameterized4(
            1u8,
            1u8,
            0u8,
            1u8,
            0u8,
            0u8,
            ((&raw const sMG_Ereader_TextColor_2).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gStringVar4).cast::<u8>(),
        );
        DrawTextBorderOuter(1u8, 1u16, 15u8);
        PutWindowTilemap(1u8);
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn ClearMessage() {
    unsafe {
        rbox_fill_rectangle(1u8);
        ClearWindowTilemap(1u8);
        CopyWindowToVram(1u8, 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMysteryGiftMenuMessage(textState: *mut u8, str: *mut u8) -> u32 {
    unsafe {
        let mut textState = textState;
        let mut str = str;
        'l1: {
            let __sw1 = (((textState).read()) as i32);
            if __sw1 == 0i32 {
                MG_AddMessageTextPrinter(str);
                (textState).write(((textState).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                DrawDownArrow(
                    1u8,
                    208u16,
                    20u16,
                    1u8,
                    0u8,
                    ((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>(),
                    (((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1),
                );
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    (textState).write(((textState).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DrawDownArrow(
                    1u8,
                    208u16,
                    20u16,
                    1u8,
                    1u8,
                    ((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>(),
                    (((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1),
                );
                (textState).write(0u8);
                ClearMessage();
                return 1u32;
            }
            if __sw1 == 255i32 {
                (textState).write(2u8);
                return 0u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HideDownArrow() {
    unsafe {
        DrawDownArrow(
            1u8,
            208u16,
            20u16,
            1u8,
            0u8,
            ((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>(),
            (((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>())
                .wrapping_offset(1),
        );
    }
}
pub(crate) unsafe extern "C" fn ShowDownArrow() {
    unsafe {
        DrawDownArrow(
            1u8,
            208u16,
            20u16,
            1u8,
            1u8,
            ((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>(),
            (((&raw mut sDownArrowCounterAndYCoordIdx).cast::<u8>()).cast::<u8>())
                .wrapping_offset(1),
        );
    }
}
pub(crate) unsafe extern "C" fn HideDownArrowAndWaitButton(textState: *mut u8) -> u32 {
    unsafe {
        let mut textState = textState;
        'l1: {
            let __sw1 = (((textState).read()) as i32);
            if __sw1 == 0i32 {
                HideDownArrow();
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    (textState).write(((textState).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ShowDownArrow();
                (textState).write(0u8);
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn PrintStringAndWait2Seconds(counter: *mut u8, str: *mut u8) -> u32 {
    unsafe {
        let mut counter = counter;
        let mut str = str;
        if (((counter).read()) as i32) == 0i32 {
            MG_AddMessageTextPrinter(str);
        }
        if (({
            let __t1 = ((counter).read()).wrapping_add(1);
            (counter).write(__t1);
            __t1
        }) as i32)
            > 120i32
        {
            (counter).write(0u8);
            ClearMessage();
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
pub(crate) unsafe extern "C" fn MysteryGift_HandleThreeOptionMenu(
    unused0: *mut u8,
    unused1: *mut u16,
    whichMenu: u8,
) -> u32 {
    unsafe {
        let mut unused0 = unused0;
        let mut unused1 = unused1;
        let mut whichMenu = whichMenu;
        let mut listMenuTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut listMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sListMenuTemplate_ThreeOptions)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        let mut windowTemplate = crate::ffi::Align4([0u8; 8]);
        (&raw mut windowTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sWindowTemplate_ThreeOptions)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        let mut width: i32 = 0i32;
        let mut response: i32 = 0i32;
        if ((whichMenu) as i32) == 0i32 {
            (((&raw mut listMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(
                ((&raw const sListMenuItems_CardsOrNews)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
        } else {
            (((&raw mut listMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(
                ((&raw const sListMenuItems_WirelessOrFriend)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
        }
        width = Intl_GetListMenuWidth((&raw mut listMenuTemplate).cast::<u8>());
        if (width & 1i32) != 0 {
            width = (width).wrapping_add(1);
        }
        (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(3)).write(((width) as u8));
        if width < crate::c::div_i32(240i32, 8i32) {
            (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(1)).write(
                ((crate::c::div_i32((crate::c::div_i32(240i32, 8i32)).wrapping_sub(width), 2i32))
                    as u8),
            );
        } else {
            (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(1)).write(0u8);
        }
        response = DoMysteryGiftListMenu(
            (&raw mut windowTemplate).cast::<u8>(),
            (&raw mut listMenuTemplate).cast::<u8>(),
            1u8,
            10u16,
            224u16,
        );
        if response != (-1i32) {
            ClearWindowTilemap(2u8);
            CopyWindowToVram(2u8, 1u8);
        }
        return ((response) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMysteryGiftYesNo(
    textState: *mut u8,
    windowId: *mut u16,
    yesNoBoxPlacement: u8,
    str: *mut u8,
) -> i8 {
    unsafe {
        let mut textState = textState;
        let mut windowId = windowId;
        let mut yesNoBoxPlacement = yesNoBoxPlacement;
        let mut str = str;
        let mut windowTemplate = crate::ffi::Align4([0u8; 8]);
        let mut input: i8 = 0i8;
        'l1: {
            let __sw1 = (((textState).read()) as i32);
            if __sw1 == 0i32 {
                StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str);
                if ((yesNoBoxPlacement) as i32) == 0i32 {
                    (windowId).write(AddWindow(
                        (&raw const sWindowTemplate_YesNoMsg_Wide)
                            .cast::<u8>()
                            .cast_mut(),
                    ));
                } else {
                    (windowId).write(AddWindow(
                        (&raw const sWindowTemplate_YesNoMsg)
                            .cast::<u8>()
                            .cast_mut(),
                    ));
                }
                FillWindowPixelBuffer((((windowId).read()) as u8), 17u8);
                AddTextPrinterParameterized4(
                    (((windowId).read()) as u8),
                    1u8,
                    0u8,
                    1u8,
                    0u8,
                    0u8,
                    ((&raw const sMG_Ereader_TextColor_2).cast::<u8>().cast_mut()).cast::<u8>(),
                    0i8,
                    (&raw mut gStringVar4).cast::<u8>(),
                );
                DrawTextBorderOuter((((windowId).read()) as u8), 1u16, 15u8);
                CopyWindowToVram((((windowId).read()) as u8), 2u8);
                PutWindowTilemap((((windowId).read()) as u8));
                (textState).write(((textState).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                (&raw mut windowTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplate_YesNoBox)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                if ((yesNoBoxPlacement) as i32) == 0i32 {
                    (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(2)).write(9u8);
                } else {
                    (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(2)).write(15u8);
                }
                CreateYesNoMenu((&raw mut windowTemplate).cast::<u8>(), 10u16, 14u8, 0u8);
                (textState).write(((textState).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = Menu_ProcessInputNoWrapClearOnChoose();
                if ((((input) as i32) == (-1i32)) || (((input) as i32) == 0i32))
                    || (((input) as i32) == 1i32)
                {
                    (textState).write(0u8);
                    rbox_fill_rectangle((((windowId).read()) as u8));
                    ClearWindowTilemap((((windowId).read()) as u8));
                    CopyWindowToVram((((windowId).read()) as u8), 1u8);
                    RemoveWindow((((windowId).read()) as u8));
                    return input;
                }
                break 'l1;
            }
            if __sw1 == 255i32 {
                (textState).write(0u8);
                rbox_fill_rectangle((((windowId).read()) as u8));
                ClearWindowTilemap((((windowId).read()) as u8));
                CopyWindowToVram((((windowId).read()) as u8), 1u8);
                RemoveWindow((((windowId).read()) as u8));
                return (-1i8);
            }
        }
        return (-2i8);
    }
}
pub(crate) unsafe extern "C" fn HandleGiftSelectMenu(
    textState: *mut u8,
    windowId: *mut u16,
    cannotToss: u32,
    cannotSend: u32,
) -> i32 {
    unsafe {
        let mut textState = textState;
        let mut windowId = windowId;
        let mut cannotToss = cannotToss;
        let mut cannotSend = cannotSend;
        let mut windowTemplate = crate::ffi::Align4([0u8; 8]);
        let mut input: i32 = 0i32;
        'l1: {
            let __sw1 = (((textState).read()) as i32);
            if __sw1 == 0i32 {
                if !((cannotToss) != 0) {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_WhatToDoWithCards).cast::<u8>(),
                    );
                } else {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_WhatToDoWithNews).cast::<u8>(),
                    );
                }
                (windowId).write(AddWindow(
                    (&raw const sWindowTemplate_GiftSelect)
                        .cast::<u8>()
                        .cast_mut(),
                ));
                FillWindowPixelBuffer((((windowId).read()) as u8), 17u8);
                AddTextPrinterParameterized4(
                    (((windowId).read()) as u8),
                    1u8,
                    0u8,
                    1u8,
                    0u8,
                    0u8,
                    ((&raw const sMG_Ereader_TextColor_2).cast::<u8>().cast_mut()).cast::<u8>(),
                    0i8,
                    (&raw mut gStringVar4).cast::<u8>(),
                );
                DrawTextBorderOuter((((windowId).read()) as u8), 1u16, 15u8);
                CopyWindowToVram((((windowId).read()) as u8), 2u8);
                PutWindowTilemap((((windowId).read()) as u8));
                (textState).write(((textState).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                (&raw mut windowTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplate_YesNoBox)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                if (cannotSend) != 0 {
                    if !((cannotToss) != 0) {
                        input = DoMysteryGiftListMenu(
                            (&raw const sWindowTemplate_GiftSelect_2Options)
                                .cast::<u8>()
                                .cast_mut(),
                            (&raw const sListMenu_ReceiveToss).cast::<u8>().cast_mut(),
                            1u8,
                            10u16,
                            224u16,
                        );
                    } else {
                        input = DoMysteryGiftListMenu(
                            (&raw const sWindowTemplate_GiftSelect_1Option)
                                .cast::<u8>()
                                .cast_mut(),
                            (&raw const sListMenu_Receive).cast::<u8>().cast_mut(),
                            1u8,
                            10u16,
                            224u16,
                        );
                    }
                } else {
                    if !((cannotToss) != 0) {
                        input = DoMysteryGiftListMenu(
                            (&raw const sWindowTemplate_GiftSelect_3Options)
                                .cast::<u8>()
                                .cast_mut(),
                            (&raw const sListMenu_ReceiveSendToss)
                                .cast::<u8>()
                                .cast_mut(),
                            1u8,
                            10u16,
                            224u16,
                        );
                    } else {
                        input = DoMysteryGiftListMenu(
                            (&raw const sWindowTemplate_GiftSelect_2Options)
                                .cast::<u8>()
                                .cast_mut(),
                            (&raw const sListMenu_ReceiveSend).cast::<u8>().cast_mut(),
                            1u8,
                            10u16,
                            224u16,
                        );
                    }
                }
                if input != (-1i32) {
                    (textState).write(0u8);
                    rbox_fill_rectangle((((windowId).read()) as u8));
                    ClearWindowTilemap((((windowId).read()) as u8));
                    CopyWindowToVram((((windowId).read()) as u8), 1u8);
                    RemoveWindow((((windowId).read()) as u8));
                    return input;
                }
                break 'l1;
            }
            if __sw1 == 255i32 {
                (textState).write(0u8);
                rbox_fill_rectangle((((windowId).read()) as u8));
                ClearWindowTilemap((((windowId).read()) as u8));
                CopyWindowToVram((((windowId).read()) as u8), 1u8);
                RemoveWindow((((windowId).read()) as u8));
                return (-2i32);
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn ValidateCardOrNews(isWonderNews: u32) -> u32 {
    unsafe {
        let mut isWonderNews = isWonderNews;
        if !((isWonderNews) != 0) {
            return ValidateSavedWonderCard();
        } else {
            return ValidateSavedWonderNews();
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn HandleLoadWonderCardOrNews(
    state: *mut u8,
    isWonderNews: u32,
) -> u32 {
    unsafe {
        let mut state = state;
        let mut isWonderNews = isWonderNews;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                if !((isWonderNews) != 0) {
                    WonderCard_Init(GetSavedWonderCard(), GetSavedWonderCardMetadata());
                } else {
                    WonderNews_Init(GetSavedWonderNews());
                }
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((isWonderNews) != 0) {
                    if !((WonderCard_Enter()) != 0) {
                        return 0u32;
                    }
                } else {
                    if !((WonderNews_Enter()) != 0) {
                        return 0u32;
                    }
                }
                (state).write(0u8);
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ClearSavedNewsOrCard(isWonderNews: u32) -> u32 {
    unsafe {
        let mut isWonderNews = isWonderNews;
        if !((isWonderNews) != 0) {
            ClearSavedWonderCardAndRelated();
        } else {
            ClearSavedWonderNewsAndRelated();
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ExitWonderCardOrNews(isWonderNews: u32, useCancel: u32) -> u32 {
    unsafe {
        let mut isWonderNews = isWonderNews;
        let mut useCancel = useCancel;
        if !((isWonderNews) != 0) {
            if (WonderCard_Exit(useCancel)) != 0 {
                WonderCard_Destroy();
                return 1u32;
            } else {
                return 0u32;
            }
        } else {
            if (WonderNews_Exit(useCancel)) != 0 {
                WonderNews_Destroy();
                return 1u32;
            } else {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn AskDiscardGift(
    textState: *mut u8,
    windowId: *mut u16,
    isWonderNews: u32,
) -> i32 {
    unsafe {
        let mut textState = textState;
        let mut windowId = windowId;
        let mut isWonderNews = isWonderNews;
        if !((isWonderNews) != 0) {
            return ((DoMysteryGiftYesNo(
                textState,
                windowId,
                1u8,
                (&raw mut gText_IfThrowAwayCardEventWontHappen).cast::<u8>(),
            )) as i32);
        } else {
            return ((DoMysteryGiftYesNo(
                textState,
                windowId,
                1u8,
                (&raw mut gText_OkayToDiscardNews).cast::<u8>(),
            )) as i32);
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn PrintThrownAway(textState: *mut u8, isWonderNews: u32) -> u32 {
    unsafe {
        let mut textState = textState;
        let mut isWonderNews = isWonderNews;
        if !((isWonderNews) != 0) {
            return PrintMysteryGiftMenuMessage(
                textState,
                (&raw mut gText_WonderCardThrownAway).cast::<u8>(),
            );
        } else {
            return PrintMysteryGiftMenuMessage(
                textState,
                (&raw mut gText_WonderNewsThrownAway).cast::<u8>(),
            );
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn SaveOnMysteryGiftMenu(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                MG_AddMessageTextPrinter((&raw mut gText_DataWillBeSaved).cast::<u8>());
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                TrySavingData(0u8);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                MG_AddMessageTextPrinter((&raw mut gText_SaveCompletedPressA).cast::<u8>());
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                (state).write(0u8);
                ClearMessage();
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetClientResultMessage(
    successMsg: *mut u32,
    isWonderNews: u8,
    sourceIsFriend: u8,
    msgId: u32,
) -> *mut u8 {
    unsafe {
        let mut successMsg = successMsg;
        let mut isWonderNews = isWonderNews;
        let mut sourceIsFriend = sourceIsFriend;
        let mut msgId = msgId;
        let mut msg: *mut u8 = core::ptr::null_mut();
        (successMsg).write(0u32);
        'l1: {
            let __sw1 = msgId;
            if __sw1 == 0u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_NothingSentOver).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 1u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_RecordUploadedViaWireless).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 2u32 {
                (successMsg).write(1u32);
                msg = (if !((sourceIsFriend) != 0) {
                    (&raw mut gText_WonderCardReceived).cast::<u8>()
                } else {
                    (&raw mut gText_WonderCardReceivedFrom).cast::<u8>()
                });
                break 'l1;
            }
            if __sw1 == 3u32 {
                (successMsg).write(1u32);
                msg = (if !((sourceIsFriend) != 0) {
                    (&raw mut gText_WonderNewsReceived).cast::<u8>()
                } else {
                    (&raw mut gText_WonderNewsReceivedFrom).cast::<u8>()
                });
                break 'l1;
            }
            if __sw1 == 4u32 {
                (successMsg).write(1u32);
                msg = (&raw mut gText_NewStampReceived).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 5u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_AlreadyHadCard).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 6u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_AlreadyHadStamp).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 7u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_AlreadyHadNews).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 8u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_NoMoreRoomForStamps).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 9u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_CommunicationCanceled).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 10u32 {
                (successMsg).write(0u32);
                msg = (if !((isWonderNews) != 0) {
                    (&raw mut gText_CantAcceptCardFromTrainer).cast::<u8>()
                } else {
                    (&raw mut gText_CantAcceptNewsFromTrainer).cast::<u8>()
                });
                break 'l1;
            }
            if __sw1 == 11u32 {
                (successMsg).write(0u32);
                msg = (&raw mut gText_CommunicationError).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 12u32 {
                (successMsg).write(1u32);
                msg = (&raw mut gText_NewTrainerReceived).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 13u32 {
                (successMsg).write(1u32);
                break 'l1;
            }
            if __sw1 == 14u32 {
                (successMsg).write(0u32);
                break 'l1;
            }
        }
        return msg;
    }
}
pub(crate) unsafe extern "C" fn PrintSuccessMessage(
    state: *mut u8,
    msg: *mut u8,
    timer: *mut u16,
) -> u32 {
    unsafe {
        let mut state = state;
        let mut msg = msg;
        let mut timer = timer;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                if ((msg) as usize) != 0usize {
                    MG_AddMessageTextPrinter(msg);
                }
                PlayFanfare(370u16);
                (timer).write(0u16);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __t2 = ((timer).read()).wrapping_add(1);
                    (timer).write(__t2);
                    __t2
                }) as i32)
                    > 240i32
                {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsFanfareTaskInactive()) != 0 {
                    (state).write(0u8);
                    ClearMessage();
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetServerResultMessage(
    wonderSuccess: *mut u32,
    sourceIsFriend: u8,
    msgId: u32,
) -> *mut u8 {
    unsafe {
        let mut wonderSuccess = wonderSuccess;
        let mut sourceIsFriend = sourceIsFriend;
        let mut msgId = msgId;
        let mut result: *mut u8 = (&raw mut gText_CommunicationError).cast::<u8>();
        (wonderSuccess).write(0u32);
        'l1: {
            let __sw1 = msgId;
            if __sw1 == 0u32 {
                result = (&raw mut gText_NothingSentOver).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 1u32 {
                result = (&raw mut gText_RecordUploadedViaWireless).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 2u32 {
                result = (&raw mut gText_WonderCardSentTo).cast::<u8>();
                (wonderSuccess).write(1u32);
                break 'l1;
            }
            if __sw1 == 3u32 {
                result = (&raw mut gText_WonderNewsSentTo).cast::<u8>();
                (wonderSuccess).write(1u32);
                break 'l1;
            }
            if __sw1 == 4u32 {
                result = (&raw mut gText_StampSentTo).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 5u32 {
                result = (&raw mut gText_OtherTrainerHasCard).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 6u32 {
                result = (&raw mut gText_OtherTrainerHasStamp).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 7u32 {
                result = (&raw mut gText_OtherTrainerHasNews).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 8u32 {
                result = (&raw mut gText_NoMoreRoomForStamps).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 9u32 {
                result = (&raw mut gText_OtherTrainerCanceled).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 10u32 {
                result = (&raw mut gText_CantSendGiftToTrainer).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 11u32 {
                result = (&raw mut gText_CommunicationError).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 12u32 {
                result = (&raw mut gText_GiftSentTo).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 13u32 {
                result = (&raw mut gText_GiftSentTo).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 14u32 {
                result = (&raw mut gText_CantSendGiftToTrainer).cast::<u8>();
                break 'l1;
            }
        }
        return result;
    }
}
pub(crate) unsafe extern "C" fn PrintServerResultMessage(
    state: *mut u8,
    timer: *mut u16,
    sourceIsFriend: u8,
    msgId: u32,
) -> u32 {
    unsafe {
        let mut state = state;
        let mut timer = timer;
        let mut sourceIsFriend = sourceIsFriend;
        let mut msgId = msgId;
        let mut wonderSuccess: u32 = 0u32;
        let mut str: *mut u8 =
            GetServerResultMessage(&raw mut wonderSuccess, sourceIsFriend, msgId);
        if (wonderSuccess) != 0 {
            return PrintSuccessMessage(state, str, timer);
        } else {
            return PrintMysteryGiftMenuMessage(state, str);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMysteryGiftTask() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_MysteryGift), 0u8);
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        ((data).wrapping_add(8)).write(0u8);
        ((data).wrapping_add(9)).write(0u8);
        ((data).wrapping_add(10)).write(0u8);
        ((data).wrapping_add(11)).write(0u8);
        ((data).wrapping_add(12)).write(0u8);
        ((data).wrapping_add(13)).write(0u8);
        ((data).cast::<u16>()).write(0u16);
        ((data).wrapping_add(2).cast::<u16>()).write(0u16);
        ((data).wrapping_add(4).cast::<u16>()).write(0u16);
        ((data).wrapping_add(6).cast::<u16>()).write(0u16);
        ((data).wrapping_add(14)).write(0u8);
        ((data).wrapping_add(16).cast::<*mut u8>()).write(AllocZeroed(64u32));
    }
}
pub(crate) unsafe extern "C" fn Task_MysteryGift(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        let mut successMsg: u32 = 0u32;
        let mut input: u32 = 0u32;
        let mut msg: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((((data).wrapping_add(8)).read()) as i32);
            if __sw1 == 0i32 {
                ((data).wrapping_add(8)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: {
                    let __sw2 = MysteryGift_HandleThreeOptionMenu(
                        (data).wrapping_add(9),
                        (data).cast::<u16>(),
                        0u8,
                    );
                    if __sw2 == 0u32 {
                        ((data).wrapping_add(12)).write(0u8);
                        if ValidateSavedWonderCard() == 1u32 {
                            ((data).wrapping_add(8)).write(18u8);
                        } else {
                            ((data).wrapping_add(8)).write(2u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 1u32 {
                        ((data).wrapping_add(12)).write(1u8);
                        if ValidateSavedWonderNews() == 1u32 {
                            ((data).wrapping_add(8)).write(18u8);
                        } else {
                            ((data).wrapping_add(8)).write(2u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 4294967294u32 {
                        ((data).wrapping_add(8)).write(37u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    if !((((data).wrapping_add(12)).read()) != 0) {
                        if (PrintMysteryGiftMenuMessage(
                            (data).wrapping_add(9),
                            (&raw mut gText_DontHaveCardNewOneInput).cast::<u8>(),
                        )) != 0
                        {
                            ((data).wrapping_add(8)).write(3u8);
                            PrintMysteryGiftOrEReaderHeader(0u8, 1u32);
                        }
                    } else {
                        if (PrintMysteryGiftMenuMessage(
                            (data).wrapping_add(9),
                            (&raw mut gText_DontHaveNewsNewOneInput).cast::<u8>(),
                        )) != 0
                        {
                            ((data).wrapping_add(8)).write(3u8);
                            PrintMysteryGiftOrEReaderHeader(0u8, 1u32);
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3i32 {
                if !((((data).wrapping_add(12)).read()) != 0) {
                    MG_AddMessageTextPrinter(
                        (&raw mut gText_WhereShouldCardBeAccessed).cast::<u8>(),
                    );
                } else {
                    MG_AddMessageTextPrinter(
                        (&raw mut gText_WhereShouldNewsBeAccessed).cast::<u8>(),
                    );
                }
                ((data).wrapping_add(8)).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                'l3: {
                    let __sw3 = MysteryGift_HandleThreeOptionMenu(
                        (data).wrapping_add(9),
                        (data).cast::<u16>(),
                        1u8,
                    );
                    if __sw3 == 0u32 {
                        ClearMessage();
                        ((data).wrapping_add(8)).write(5u8);
                        ((data).wrapping_add(13)).write(0u8);
                        break 'l3;
                    }
                    if __sw3 == 1u32 {
                        ClearMessage();
                        ((data).wrapping_add(8)).write(5u8);
                        ((data).wrapping_add(13)).write(1u8);
                        break 'l3;
                    }
                    if __sw3 == 4294967294u32 {
                        ClearMessage();
                        if (ValidateCardOrNews(((((data).wrapping_add(12)).read()) as u32))) != 0 {
                            ((data).wrapping_add(8)).write(18u8);
                        } else {
                            ((data).wrapping_add(8)).write(0u8);
                            PrintMysteryGiftOrEReaderHeader(0u8, 0u32);
                        }
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gStringVar1).cast::<u8>()).write(255u8);
                ((&raw mut gStringVar2).cast::<u8>()).write(255u8);
                ((&raw mut gStringVar3).cast::<u8>()).write(255u8);
                'l4: {
                    let __sw4 = ((((data).wrapping_add(12)).read()) as i32);
                    if __sw4 == 0i32 {
                        if ((((data).wrapping_add(13)).read()) as i32) == 1i32 {
                            CreateTask_LinkMysteryGiftWithFriend(21u32);
                        } else {
                            if ((((data).wrapping_add(13)).read()) as i32) == 0i32 {
                                CreateTask_LinkMysteryGiftOverWireless(21u32);
                            }
                        }
                        break 'l4;
                    }
                    if __sw4 == 1i32 {
                        if ((((data).wrapping_add(13)).read()) as i32) == 1i32 {
                            CreateTask_LinkMysteryGiftWithFriend(22u32);
                        } else {
                            if ((((data).wrapping_add(13)).read()) as i32) == 0i32 {
                                CreateTask_LinkMysteryGiftOverWireless(22u32);
                            }
                        }
                        break 'l4;
                    }
                }
                ((data).wrapping_add(8)).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ClearScreenInBg0(1u32);
                    ((data).wrapping_add(8)).write(7u8);
                    MysteryGiftClient_Create(((((data).wrapping_add(12)).read()) as u32));
                } else {
                    if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 5i32 {
                        ClearScreenInBg0(1u32);
                        ((data).wrapping_add(8)).write(3u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                MG_AddMessageTextPrinter((&raw mut gText_Communicating).cast::<u8>());
                ((data).wrapping_add(8)).write(8u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                'l5: {
                    let __sw5 = MysteryGiftClient_Run((data).cast::<u16>());
                    if __sw5 == 6u32 {
                        Rfu_SetCloseLinkCallback();
                        ((data).wrapping_add(14)).write(((((data).cast::<u16>()).read()) as u8));
                        ((data).wrapping_add(8)).write(13u8);
                        break 'l5;
                    }
                    if __sw5 == 5u32 {
                        crate::c::memcpy(
                            ((data).wrapping_add(16).cast::<*mut u8>()).read(),
                            MysteryGiftClient_GetMsg(),
                            64u32,
                        );
                        MysteryGiftClient_AdvanceState();
                        break 'l5;
                    }
                    if __sw5 == 3u32 {
                        ((data).wrapping_add(8)).write(10u8);
                        break 'l5;
                    }
                    if __sw5 == 2u32 {
                        ((data).wrapping_add(8)).write(9u8);
                        break 'l5;
                    }
                    if __sw5 == 4u32 {
                        ((data).wrapping_add(8)).write(11u8);
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                        );
                        break 'l5;
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                input = ((DoMysteryGiftYesNo(
                    (data).wrapping_add(9),
                    (data).cast::<u16>(),
                    0u8,
                    MysteryGiftClient_GetMsg(),
                )) as u32);
                'l6: {
                    let __sw6 = input;
                    if __sw6 == 0u32 {
                        MysteryGiftClient_SetParam(0u32);
                        MysteryGiftClient_AdvanceState();
                        ((data).wrapping_add(8)).write(7u8);
                        break 'l6;
                    }
                    if __sw6 == 1u32 || __sw6 == 4294967295u32 {
                        MysteryGiftClient_SetParam(1u32);
                        MysteryGiftClient_AdvanceState();
                        ((data).wrapping_add(8)).write(7u8);
                        break 'l6;
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (PrintMysteryGiftMenuMessage((data).wrapping_add(9), MysteryGiftClient_GetMsg()))
                    != 0
                {
                    MysteryGiftClient_AdvanceState();
                    ((data).wrapping_add(8)).write(7u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                input = ((DoMysteryGiftYesNo(
                    (data).wrapping_add(9),
                    (data).cast::<u16>(),
                    0u8,
                    (&raw mut gText_ThrowAwayWonderCard).cast::<u8>(),
                )) as u32);
                'l7: {
                    let __sw7 = input;
                    if __sw7 == 0u32 {
                        if IsSavedWonderCardGiftNotReceived() == 1u32 {
                            ((data).wrapping_add(8)).write(12u8);
                        } else {
                            MysteryGiftClient_SetParam(0u32);
                            MysteryGiftClient_AdvanceState();
                            ((data).wrapping_add(8)).write(7u8);
                        }
                        break 'l7;
                    }
                    if __sw7 == 1u32 || __sw7 == 4294967295u32 {
                        MysteryGiftClient_SetParam(1u32);
                        MysteryGiftClient_AdvanceState();
                        ((data).wrapping_add(8)).write(7u8);
                        break 'l7;
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                input = ((DoMysteryGiftYesNo(
                    (data).wrapping_add(9),
                    (data).cast::<u16>(),
                    0u8,
                    (&raw mut gText_HaventReceivedCardsGift).cast::<u8>(),
                )) as u32);
                'l8: {
                    let __sw8 = input;
                    if __sw8 == 0u32 {
                        MysteryGiftClient_SetParam(0u32);
                        MysteryGiftClient_AdvanceState();
                        ((data).wrapping_add(8)).write(7u8);
                        break 'l8;
                    }
                    if __sw8 == 1u32 || __sw8 == 4294967295u32 {
                        MysteryGiftClient_SetParam(1u32);
                        MysteryGiftClient_AdvanceState();
                        ((data).wrapping_add(8)).write(7u8);
                        break 'l8;
                    }
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    DestroyWirelessStatusIndicatorSprite();
                    ((data).wrapping_add(8)).write(14u8);
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (PrintStringAndWait2Seconds(
                    (data).wrapping_add(9),
                    (&raw mut gText_CommunicationCompleted).cast::<u8>(),
                )) != 0
                {
                    if ((((data).wrapping_add(13)).read()) as i32) == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                        );
                    }
                    ((data).wrapping_add(8)).write(15u8);
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                msg = GetClientResultMessage(
                    &raw mut successMsg,
                    ((data).wrapping_add(12)).read(),
                    ((data).wrapping_add(13)).read(),
                    ((((data).wrapping_add(14)).read()) as u32),
                );
                if ((msg) as usize) == 0usize {
                    msg = ((data).wrapping_add(16).cast::<*mut u8>()).read();
                }
                if (successMsg) != 0 {
                    input = PrintSuccessMessage((data).wrapping_add(9), msg, (data).cast::<u16>());
                } else {
                    input = PrintMysteryGiftMenuMessage((data).wrapping_add(9), msg);
                }
                if (input) != 0 {
                    if ((((data).wrapping_add(14)).read()) as i32) == 3i32 {
                        if ((((data).wrapping_add(13)).read()) as i32) == 1i32 {
                            WonderNews_SetReward(1u32);
                        } else {
                            WonderNews_SetReward(2u32);
                        }
                    }
                    if !((successMsg) != 0) {
                        ((data).wrapping_add(8)).write(0u8);
                        PrintMysteryGiftOrEReaderHeader(0u8, 0u32);
                    } else {
                        ((data).wrapping_add(8)).write(17u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                if (SaveOnMysteryGiftMenu((data).wrapping_add(9))) != 0 {
                    ((data).wrapping_add(8)).write(18u8);
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                if (HandleLoadWonderCardOrNews(
                    (data).wrapping_add(9),
                    ((((data).wrapping_add(12)).read()) as u32),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(20u8);
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                if !((((data).wrapping_add(12)).read()) != 0) {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        ((data).wrapping_add(8)).write(21u8);
                    }
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        ((data).wrapping_add(8)).write(27u8);
                    }
                } else {
                    'l9: {
                        let __sw9 = WonderNews_GetInput(
                            (((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read(),
                        );
                        if __sw9 == 0u32 {
                            WonderNews_RemoveScrollIndicatorArrowPair();
                            ((data).wrapping_add(8)).write(21u8);
                            break 'l9;
                        }
                        if __sw9 == 1u32 {
                            ((data).wrapping_add(8)).write(27u8);
                            break 'l9;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                {
                    let mut result: u32 = 0u32;
                    if !((((data).wrapping_add(12)).read()) != 0) {
                        if (IsSendingSavedWonderCardAllowed()) != 0 {
                            result = ((HandleGiftSelectMenu(
                                (data).wrapping_add(9),
                                (data).cast::<u16>(),
                                ((((data).wrapping_add(12)).read()) as u32),
                                0u32,
                            )) as u32);
                        } else {
                            result = ((HandleGiftSelectMenu(
                                (data).wrapping_add(9),
                                (data).cast::<u16>(),
                                ((((data).wrapping_add(12)).read()) as u32),
                                1u32,
                            )) as u32);
                        }
                    } else {
                        if (IsSendingSavedWonderNewsAllowed()) != 0 {
                            result = ((HandleGiftSelectMenu(
                                (data).wrapping_add(9),
                                (data).cast::<u16>(),
                                ((((data).wrapping_add(12)).read()) as u32),
                                0u32,
                            )) as u32);
                        } else {
                            result = ((HandleGiftSelectMenu(
                                (data).wrapping_add(9),
                                (data).cast::<u16>(),
                                ((((data).wrapping_add(12)).read()) as u32),
                                1u32,
                            )) as u32);
                        }
                    }
                    'l10: {
                        let __sw10 = result;
                        if __sw10 == 0u32 {
                            ((data).wrapping_add(8)).write(28u8);
                            break 'l10;
                        }
                        if __sw10 == 1u32 {
                            ((data).wrapping_add(8)).write(29u8);
                            break 'l10;
                        }
                        if __sw10 == 2u32 {
                            ((data).wrapping_add(8)).write(22u8);
                            break 'l10;
                        }
                        if __sw10 == 4294967294u32 {
                            if ((((data).wrapping_add(12)).read()) as i32) == 1i32 {
                                WonderNews_AddScrollIndicatorArrowPair();
                            }
                            ((data).wrapping_add(8)).write(20u8);
                            break 'l10;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 22i32 {
                'l11: {
                    let __sw11 = AskDiscardGift(
                        (data).wrapping_add(9),
                        (data).cast::<u16>(),
                        ((((data).wrapping_add(12)).read()) as u32),
                    );
                    if __sw11 == 0i32 {
                        if (!((((data).wrapping_add(12)).read()) != 0))
                            && (IsSavedWonderCardGiftNotReceived() == 1u32)
                        {
                            ((data).wrapping_add(8)).write(23u8);
                        } else {
                            ((data).wrapping_add(8)).write(24u8);
                        }
                        break 'l11;
                    }
                    if __sw11 == 1i32 || __sw11 == (-1i32) {
                        ((data).wrapping_add(8)).write(21u8);
                        break 'l11;
                    }
                }
                break 'l1;
            }
            if __sw1 == 23i32 {
                'l12: {
                    let __sw12 = ((DoMysteryGiftYesNo(
                        (data).wrapping_add(9),
                        (data).cast::<u16>(),
                        1u8,
                        (&raw mut gText_HaventReceivedGiftOkayToDiscard).cast::<u8>(),
                    )) as u32);
                    if __sw12 == 0u32 {
                        ((data).wrapping_add(8)).write(24u8);
                        break 'l12;
                    }
                    if __sw12 == 1u32 || __sw12 == 4294967295u32 {
                        ((data).wrapping_add(8)).write(21u8);
                        break 'l12;
                    }
                }
                break 'l1;
            }
            if __sw1 == 24i32 {
                if (ExitWonderCardOrNews(((((data).wrapping_add(12)).read()) as u32), 1u32)) != 0 {
                    ClearSavedNewsOrCard(((((data).wrapping_add(12)).read()) as u32));
                    ((data).wrapping_add(8)).write(25u8);
                }
                break 'l1;
            }
            if __sw1 == 25i32 {
                if (SaveOnMysteryGiftMenu((data).wrapping_add(9))) != 0 {
                    ((data).wrapping_add(8)).write(26u8);
                }
                break 'l1;
            }
            if __sw1 == 26i32 {
                if (PrintThrownAway(
                    (data).wrapping_add(9),
                    ((((data).wrapping_add(12)).read()) as u32),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(0u8);
                    PrintMysteryGiftOrEReaderHeader(0u8, 0u32);
                }
                break 'l1;
            }
            if __sw1 == 27i32 {
                if (ExitWonderCardOrNews(((((data).wrapping_add(12)).read()) as u32), 0u32)) != 0 {
                    ((data).wrapping_add(8)).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 28i32 {
                if (ExitWonderCardOrNews(((((data).wrapping_add(12)).read()) as u32), 1u32)) != 0 {
                    ((data).wrapping_add(8)).write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 29i32 {
                if (ExitWonderCardOrNews(((((data).wrapping_add(12)).read()) as u32), 1u32)) != 0 {
                    'l13: {
                        let __sw13 = ((((data).wrapping_add(12)).read()) as i32);
                        if __sw13 == 0i32 {
                            CreateTask_SendMysteryGift(21u32);
                            break 'l13;
                        }
                        if __sw13 == 1i32 {
                            CreateTask_SendMysteryGift(22u32);
                            break 'l13;
                        }
                    }
                    ((data).wrapping_add(13)).write(1u8);
                    ((data).wrapping_add(8)).write(30u8);
                }
                break 'l1;
            }
            if __sw1 == 30i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ClearScreenInBg0(1u32);
                    ((data).wrapping_add(8)).write(31u8);
                } else {
                    if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 5i32 {
                        ClearScreenInBg0(1u32);
                        ((data).wrapping_add(8)).write(18u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 31i32 {
                ((&raw mut gStringVar1).cast::<u8>()).write(255u8);
                ((&raw mut gStringVar2).cast::<u8>()).write(255u8);
                ((&raw mut gStringVar3).cast::<u8>()).write(255u8);
                if !((((data).wrapping_add(12)).read()) != 0) {
                    MG_AddMessageTextPrinter((&raw mut gText_SendingWonderCard).cast::<u8>());
                    MysterGiftServer_CreateForCard();
                } else {
                    MG_AddMessageTextPrinter((&raw mut gText_SendingWonderNews).cast::<u8>());
                    MysterGiftServer_CreateForNews();
                }
                ((data).wrapping_add(8)).write(32u8);
                break 'l1;
            }
            if __sw1 == 32i32 {
                if MysterGiftServer_Run((data).cast::<u16>()) == 3u32 {
                    ((data).wrapping_add(14)).write(((((data).cast::<u16>()).read()) as u8));
                    ((data).wrapping_add(8)).write(33u8);
                }
                break 'l1;
            }
            if __sw1 == 33i32 {
                Rfu_SetCloseLinkCallback();
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28)).wrapping_add(8))
                        .cast::<u8>(),
                );
                ((data).wrapping_add(8)).write(34u8);
                break 'l1;
            }
            if __sw1 == 34i32 {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    DestroyWirelessStatusIndicatorSprite();
                    ((data).wrapping_add(8)).write(35u8);
                }
                break 'l1;
            }
            if __sw1 == 35i32 {
                if (PrintServerResultMessage(
                    (data).wrapping_add(9),
                    (data).cast::<u16>(),
                    ((data).wrapping_add(13)).read(),
                    ((((data).wrapping_add(14)).read()) as u32),
                )) != 0
                {
                    if (((((data).wrapping_add(13)).read()) as i32) == 1i32)
                        && (((((data).wrapping_add(14)).read()) as i32) == 3i32)
                    {
                        WonderNews_SetReward(3u32);
                        ((data).wrapping_add(8)).write(17u8);
                    } else {
                        ((data).wrapping_add(8)).write(0u8);
                        PrintMysteryGiftOrEReaderHeader(0u8, 0u32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 16i32 || __sw1 == 36i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gText_CommunicationError).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(0u8);
                    PrintMysteryGiftOrEReaderHeader(0u8, 0u32);
                }
                break 'l1;
            }
            if __sw1 == 37i32 {
                CloseLink();
                Free(((data).wrapping_add(16).cast::<*mut u8>()).read());
                DestroyTask(taskId);
                SetMainCallback2(Some(MainCB_FreeAllBuffersAndReturnToInitTitleScreen));
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMysteryGiftBaseBlock() -> u16 {
    unsafe {
        return 425u16;
    }
}
pub(crate) unsafe extern "C" fn LoadMysteryGiftTextboxBorder(bgId: u8) {
    unsafe {
        let mut bgId = bgId;
        DecompressAndLoadBgGfxUsingHeap(
            bgId,
            (((&raw const sTextboxBorder_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            256u32,
            0u16,
            0u8,
        );
    }
}
