//! Translated from `src/dodrio_berry_picking.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sActiveColumnMap sDodrioHeadToColumnMap sDodrioNeighborMap sPlayerIdAtColumn sUnsharedColumns sDuplicateGfx sBerryFallDelays sTreeBorderXPos sDifficultyThresholds sPrizeBerryIds sLeaderFuncs sMemberFuncs sBerryScoreMultipliers sWindowTemplates_Records sRecordsTexts sRecordNumMaxDigits sRecordTextYCoords sRecordNumYCoords sDebug_BerryResults sJPText_Vowels sText_Letters sText_Digits sDebug_PlayerNames sBgTemplates sWindowTemplate_Dummy sWindowTemplates_Results sWindowTemplate_Prize sWindowTemplates_PlayAgain sWindowTemplate_DroppedOut sWindowTemplate_CommStandby sActiveColumnMap_Duplicate sDodrioHeadToColumnMap_Duplicate sDodrioNeighborMap_Duplicate sPlayerIdAtColumn_Duplicate sUnsharedColumns_Duplicate sBg_Pal sDodrioNormal_Pal sDodrioShiny_Pal sStatus_Pal sBerries_Pal sBerries_Gfx sCloud_Pal sBg_Gfx sTreeBorder_Gfx sStatus_Gfx sCloud_Gfx sDodrio_Gfx sBg_Tilemap sTreeBorderRight_Tilemap sTreeBorderLeft_Tilemap sOamData_Dodrio sOamData_16x16_Priority0 sOamData_Berry sOamData_Cloud sAnim_Dodrio_Normal sAnim_Dodrio_PickRight sAnim_Dodrio_PickMiddle sAnim_Dodrio_PickLeft sAnim_Dodrio_Down sAnims_Dodrio sAnims_StatusBar_Yellow sAnims_StatusBar_Gray sAnims_StatusBar_Red sAnims_StatusBar sAnim_Berry_Blue sAnim_Berry_Green sAnim_Berry_Gold sAnim_Berry_BlueSquished sAnim_Berry_GreenSquished sAnim_Berry_GoldSquished sAnim_Berry_Eaten sAnim_Berry_Empty1 sAnim_Berry_Empty2 sAnims_Berry sAnim_Cloud sAnims_Cloud sUnusedSounds sBerryIconXCoords sCloudStartCoords sTextColorTable sNameWindowCoords_1Player sNameWindowCoords_2Players sNameWindowCoords_3Players sNameWindowCoords_4Players sNameWindowCoords_5Players sNameWindowCoords sRankingTexts sResultsXCoords sResultsYCoords sRankingYCoords sGfxFuncs moveDelays.0
#[allow(unused_imports)]
use crate::data::dodrio_berry_picking::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGame: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDodrioSpriteIds: crate::ffi::Align4<[u8; 20]> = crate::ffi::Align4([0; 20]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCloudSpriteIds: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBerrySpriteIds: crate::ffi::Align4<[u8; 44]> = crate::ffi::Align4([0; 44]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBerryIconSpriteIds: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStatusBar: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGfx: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sExitingGame: u32 = 0u32;

unsafe extern "C" {
    static mut gBlockRecvBuffer: u8;
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_10P30P50P50P: u8;
    static mut gText_AnnouncingPrizes: u8;
    static mut gText_AnnouncingRankings: u8;
    static mut gText_BerryPickingRecords: u8;
    static mut gText_BerryPickingResults: u8;
    static mut gText_CantHoldAnyMore: u8;
    static mut gText_CommunicationStandby3: u8;
    static mut gText_FilledStorageSpace: u8;
    static mut gText_FirstPlacePrize: u8;
    static mut gText_No: u8;
    static mut gText_SavingDontTurnOffPower: u8;
    static mut gText_SelectorArrow2: u8;
    static mut gText_SomeoneDroppedOut: u8;
    static mut gText_SpacePoints: u8;
    static mut gText_WantToPlayAgain: u8;
    static mut gText_Yes: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckBagHasSpace(a0: u16, a1: u16) -> u8;
    fn ClearRecvCommands();
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FadeOutAndFadeInNewMapMusic(a0: u16, a1: u8, a2: u8);
    fn FadeOutAndPlayNewMapMusic(a0: u16, a1: u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn GetWindowFrameTilesPal(a0: u8) -> *mut u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsMinigameCountdownRunning() -> u32;
    fn IsMonShiny(a0: *mut u8) -> u8;
    fn IsSEPlaying() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn PlayFanfareByFanfareNum(a0: u8);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlags();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn Rfu_SendPacket(a0: *mut u8);
    fn Rfu_SetLinkStandbyCallback();
    fn RunTasks();
    fn ScriptContext_Enable();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartMinigameCountdown(a0: u16, a1: u16, a2: i16, a3: i16, a4: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopMapMusic();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_LinkFullSave(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn WaitFanfare(a0: u8) -> u8;
    fn m4aSongNumStop(a0: u16);
    fn rbox_fill_rectangle(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartDodrioBerryPicking(
    partyId: u16,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut partyId = partyId;
        let mut exitCallback = exitCallback;
        ((&raw mut sExitingGame).cast::<u8>().cast::<u32>()).write(0u32);
        if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
            && (!({
                let __v1 = AllocZeroed(13104u32);
                ((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).write(__v1);
                __v1
            })
            .is_null())
        {
            ResetTasksAndSprites();
            InitDodrioGame(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read());
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(exitCallback);
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                .write(GetMultiplayerId());
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(13004)
                .cast::<crate::c::Rec4<60>>()
                .write_unaligned(
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize
                            * 60,
                    )
                    .cast::<crate::c::Rec4<60>>()
                    .read_unaligned(),
                );
            InitMonInfo(
                (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12684))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize
                        * 4,
                ),
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyId) as i32) as isize * 100),
            );
            CreateTask(Some(Task_StartDodrioGame), 1u8);
            SetMainCallback2(Some(CB2_DodrioGame));
            SetRandomPrize();
            GetActiveBerryColumns(
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                    .read(),
                (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68),
                (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72),
            );
            StopMapMusic();
            PlayNewMapMusic(542u16);
        } else {
            SetMainCallback2(exitCallback);
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn ResetTasksAndSprites() {
    unsafe {
        ResetTasks();
        ResetSpriteData();
        FreeAllSpritePalettes();
    }
}
pub(crate) unsafe extern "C" fn InitDodrioGame(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        ((game).wrapping_add(12)).write(0u8);
        ((game).wrapping_add(16)).write(0u8);
        ((game).wrapping_add(20)).write(0u8);
        ((game).wrapping_add(24)).write(0u8);
        ((game).wrapping_add(28)).write(0u8);
        ((game).wrapping_add(284).cast::<u32>()).write(0u32);
        ((game).wrapping_add(288).cast::<u32>()).write(0u32);
        ((game).wrapping_add(48)).write(0u8);
        ((game).wrapping_add(64)).write(0u8);
        ((game).wrapping_add(60)).write(0u8);
        ((game).wrapping_add(300).cast::<u32>()).write(0u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l3;
                }
                'l4: {
                    ((((game).wrapping_add(168)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((game).wrapping_add(176)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    (((((game).wrapping_add(74)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .write(0u16);
                    ((((((game).wrapping_add(74)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                    ((((((game).wrapping_add(74)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(2))
                    .write(0u16);
                    ((((((game).wrapping_add(74)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(3))
                    .write(0u16);
                    ((((((game).wrapping_add(74)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(5))
                    .write(0u16);
                    ((((game).wrapping_add(268)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((game).wrapping_add(304)).cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 11i32) {
                    break 'l5;
                }
                'l6: {
                    ((((game).wrapping_add(208)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((game).wrapping_add(220)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((game).wrapping_add(196)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    (((((game).wrapping_add(244)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                    .cast::<u8>())
                    .write(255u8);
                    ((((((game).wrapping_add(244)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((game).wrapping_add(32)).write(
            ((if ((GetMultiplayerId()) as i32) == 0i32 {
                1i32
            } else {
                0i32
            }) as u8),
        );
        ((game).wrapping_add(36)).write(GetLinkPlayerCount());
        (((game).wrapping_add(52)).cast::<u8>()).write(GetMultiplayerId());
        {
            i = 1u8;
            'l7: loop {
                if !(((i) as i32) < ((((game).wrapping_add(36)).read()) as i32)) {
                    break 'l7;
                }
                'l8: {
                    ((((game).wrapping_add(52)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((game).wrapping_add(52)).cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read()) as i32)
                            .wrapping_add(1i32)) as u8),
                    );
                    if ((((((game).wrapping_add(52)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        > ((((game).wrapping_add(36)).read()) as i32).wrapping_sub(1i32)
                    {
                        let __p1 = (((game).wrapping_add(52)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize);
                        (__p1).write(
                            ((crate::c::rem_i32(
                                (((__p1).read()) as i32),
                                ((((game).wrapping_add(36)).read()) as i32),
                            )) as u8),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartDodrioGame(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                CreateTask_(Some(Task_CommunicateMonInfo), 4u8);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((FuncIsActiveTask(Some(Task_CommunicateMonInfo))) != 0) {
                    InitGameGfx(
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(352),
                    );
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsGfxFuncActive()) != 0) {
                    Rfu_SetLinkStandbyCallback();
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        LoadWirelessStatusIndicatorSpriteGfx();
                        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                    }
                    let __p5 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                numPlayers = ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .read();
                LoadDodrioGfx();
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((numPlayers) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            CreateDodrioSprite(
                                (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12684))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(52))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 4,
                                ),
                                i,
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(52))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SetAllDodrioInvisibility(
                    0u8,
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                        .read(),
                );
                let __p6 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadBerryGfx();
                CreateBerrySprites();
                CreateCloudSprites();
                CreateStatusBarSprites();
                let __p7 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(Some(VBlankCB_DodrioGame));
                let __p8 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p9 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                DestroyTask(taskId);
                CreateDodrioGameTask(Some(Task_NewGameIntro));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DodrioGame_Leader(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RecvLinkData_Leader();
        (((((&raw const sLeaderFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24)).read())
                as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
        if !((((&raw mut sExitingGame).cast::<u8>().cast::<u32>()).read()) != 0) {
            UpdateGame_Leader();
        }
        SendLinkData_Leader();
    }
}
pub(crate) unsafe extern "C" fn Task_DodrioGame_Member(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RecvLinkData_Member();
        (((((&raw const sMemberFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24)).read())
                as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
        if !((((&raw mut sExitingGame).cast::<u8>().cast::<u32>()).read()) != 0) {
            UpdateGame_Member();
        }
        SendLinkData_Member();
    }
}
pub(crate) unsafe extern "C" fn DoGameIntro() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 {
                StartDodrioIntroAnim(1u8);
                SetGfxFuncById(1u8);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsGfxFuncActive()) != 0) {
                    SetGameFunc(1u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitCountdown() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32;
            if __sw1 == 0i32 {
                InitFirstWaveOfBerries();
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(280)
                    .cast::<u32>())
                .write(1u32);
                SetGameFunc(2u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoCountdown() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 {
                StartMinigameCountdown(7u16, 8u16, 120i16, 80i16, 0u8);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                Rfu_SetLinkStandbyCallback();
                let __p3 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(48))
                        .write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsMinigameCountdownRunning()) != 0) {
                    let __p5 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p6 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(48);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 5i32
                {
                    Rfu_SetLinkStandbyCallback();
                    let __p8 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (IsLinkTaskFinished()) != 0 {
                    SetGameFunc(3u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WaitGameStart() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(284)
                    .cast::<u32>())
                .read())
                    != 0
                {
                    SetGameFunc(4u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayGame_Leader() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                    .read()) as i32)
                    < 10i32
                {
                    if (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .cast::<u8>())
                    .read()) as i32)
                        == 0i32
                    {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12704))
                            .cast::<u8>())
                            .wrapping_add(44))
                            .read()) as i32)
                                == 0i32
                            {
                                (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_add(44))
                                .wrapping_add(4))
                                .write(0u8);
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_add(44))
                                .write(UpdatePickStateQueue(2u8));
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0
                            {
                                if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_add(44))
                                .read()) as i32)
                                    == 0i32
                                {
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12704))
                                    .cast::<u8>())
                                    .wrapping_add(44))
                                    .wrapping_add(4))
                                    .write(0u8);
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12704))
                                    .cast::<u8>())
                                    .wrapping_add(44))
                                    .write(UpdatePickStateQueue(1u8));
                                }
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 32i32)
                                    != 0
                                {
                                    if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12704))
                                    .cast::<u8>())
                                    .wrapping_add(44))
                                    .read()) as i32)
                                        == 0i32
                                    {
                                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(12704))
                                        .cast::<u8>())
                                        .wrapping_add(44))
                                        .wrapping_add(4))
                                        .write(0u8);
                                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(12704))
                                        .cast::<u8>())
                                        .wrapping_add(44))
                                        .write(UpdatePickStateQueue(3u8));
                                    }
                                } else {
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12704))
                                    .cast::<u8>())
                                    .wrapping_add(44))
                                    .write(UpdatePickStateQueue(0u8));
                                }
                            }
                        }
                    }
                } else {
                    SetGameFunc(11u8);
                }
                UpdateFallingBerries();
                HandleSound_Leader();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayGame_Member() {
    unsafe {
        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read())
            as i32)
            < 10i32
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize
                        * 60,
                ))
                .wrapping_add(44))
                .read()) as i32)
                    == 0i32
                {
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13004))
                    .wrapping_add(44))
                    .write(2u8);
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 16i32)
                    != 0
                {
                    if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize
                            * 60,
                    ))
                    .wrapping_add(44))
                    .read()) as i32)
                        == 0i32
                    {
                        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13004))
                        .wrapping_add(44))
                        .write(1u8);
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0
                    {
                        if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 60,
                        ))
                        .wrapping_add(44))
                        .read()) as i32)
                            == 0i32
                        {
                            (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(13004))
                            .wrapping_add(44))
                            .write(3u8);
                        }
                    } else {
                        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13004))
                        .wrapping_add(44))
                        .write(0u8);
                    }
                }
            }
        } else {
            SetGameFunc(11u8);
        }
        HandleSound_Member();
    }
}
pub(crate) unsafe extern "C" fn WaitEndGame_Leader() {
    unsafe {
        let mut i: u8 = 0u8;
        UpdateFallingBerries();
        HandleSound_Leader();
        if ReadyToEndGame_Leader() == 1u32 {
            SetMaxBerriesPickedInRow();
            SetGameFunc(5u8);
        } else {
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(300)
                .cast::<u32>())
            .write(1u32);
            {
                i = 1u8;
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(304))
                        .cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()
                            != 1u32
                        {
                            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(300)
                                .cast::<u32>())
                            .write(0u32);
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WaitEndGame_Member() {
    unsafe {
        HandleSound_Member();
        if ReadyToEndGame_Member() == 1u32 {
            SetGameFunc(5u8);
        }
    }
}
pub(crate) unsafe extern "C" fn AllLinkBlocksReceived() -> u32 {
    unsafe {
        let mut recvStatus: u8 = GetBlockReceivedStatus();
        let mut playerFlags: u8 = GetLinkPlayerCountAsBitFlags();
        if ((recvStatus) as i32) == ((playerFlags) as i32) {
            ResetBlockReceivedFlags();
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
pub(crate) unsafe extern "C" fn InitResults_Leader() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                if (SendBlock(
                    0u8,
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(74))
                        .cast::<u8>(),
                    60u16,
                )) != 0
                {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(0u8);
                    let __p2 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (AllLinkBlocksReceived()) != 0 {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(
                            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(36))
                            .read(),
                        );
                }
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    >= ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .read()) as i32)
                {
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    let __p5 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                if (WaitFanfare(1u8)) != 0 {
                    SetGameFunc(6u8);
                    FadeOutAndPlayNewMapMusic(523u16, 4u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitResults_Member() {
    unsafe {
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                if (SendBlock(
                    0u8,
                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .read()) as i32) as isize
                            * 12,
                    ))
                    .cast::<u16>())
                    .cast::<u8>(),
                    60u16,
                )) != 0
                {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(0u8);
                    let __p2 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (AllLinkBlocksReceived()) != 0 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                crate::c::memcpy(
                                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(74))
                                    .cast::<u8>(),
                                    (&raw mut gBlockRecvBuffer).cast::<u8>(),
                                    60u32,
                                );
                                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .write(
                                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(36))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    >= ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .read()) as i32)
                {
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    let __p5 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                if (WaitFanfare(1u8)) != 0 {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(276)
                        .cast::<u16>())
                    .write(
                        ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 12,
                        ))
                        .cast::<u16>())
                        .wrapping_offset(5))
                        .read(),
                    );
                    SetGameFunc(6u8);
                    FadeOutAndPlayNewMapMusic(523u16, 4u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoResults() {
    unsafe {
        let mut playAgainState: u8 = 1u8;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                TryUpdateRecords();
                SetStatusBarInvisibility(1u8);
                ResetCloudPos();
                SetCloudInvisibility(1u8);
                SetGfxFuncById(2u8);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsGfxFuncActive()) != 0) {
                    SetGfxFuncById(5u8);
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                playAgainState = GetPlayAgainState();
                if (SendBlock(0u8, &raw mut playAgainState, 1u16)) != 0 {
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p5 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (AllLinkBlocksReceived()) != 0 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(268))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 256))
                                    .cast::<u16>())
                                    .cast::<u8>())
                                    .read(),
                                );
                                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .write(
                                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(36))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    >= ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .read()) as i32)
                {
                    if (({
                        let __p6 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20);
                        let __t7 = ((__p6).read()).wrapping_add(1);
                        (__p6).write(__t7);
                        __t7
                    }) as i32)
                        >= 120i32
                    {
                        SetGfxFuncById(6u8);
                        let __p8 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16);
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if !__matched {
                if !((IsGfxFuncActive()) != 0) {
                    SetGameFunc(7u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AskPlayAgain() {
    unsafe {
        let mut playAgainState: u8 = 0u8;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                if GetHighestScore() >= 3000u32 {
                    SetGfxFuncById(4u8);
                }
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsGfxFuncActive()) != 0) {
                    SetGfxFuncById(3u8);
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetBerryAndStatusBarSprites();
                ResetForPlayAgainPrompt();
                let __p4 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __v5 = GetPlayAgainState();
                    playAgainState = __v5;
                    __v5
                }) as i32)
                    != 0i32
                {
                    let __p6 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsGfxFuncActive()) != 0) {
                    SetGfxFuncById(5u8);
                    let __p7 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                playAgainState = GetPlayAgainState();
                if (SendBlock(0u8, &raw mut playAgainState, 1u16)) != 0 {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(0u8);
                    let __p8 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p9 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (AllLinkBlocksReceived()) != 0 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(268))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 256))
                                    .cast::<u16>())
                                    .cast::<u8>())
                                    .read(),
                                );
                                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .write(
                                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(36))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    >= ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .read()) as i32)
                {
                    if (({
                        let __p10 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20);
                        let __t11 = ((__p10).read()).wrapping_add(1);
                        (__p10).write(__t11);
                        __t11
                    }) as i32)
                        >= 120i32
                    {
                        ResetPickState();
                        SetGfxFuncById(6u8);
                        let __p12 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16);
                        (__p12).write(((__p12).read()).wrapping_add(1));
                    }
                } else {
                    HandleWaitPlayAgainInput();
                }
                break 'l1;
            }
            if !__matched {
                if !((IsGfxFuncActive()) != 0) {
                    {
                        i = 0u8;
                        'l4: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32))
                            {
                                break 'l4;
                            }
                            'l5: {
                                if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(268))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    == 2i32
                                {
                                    SetGameFunc(8u8);
                                    return;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    SetGameFunc(10u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EndLink() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                SetCloseLinkCallback();
                SetGfxFuncById(7u8);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsGfxFuncActive()) != 0) {
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((GetPlayAgainState()) as i32) == 5i32 {
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    SetGameFunc(9u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ExitGame() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                FreeBerrySprites();
                FreeStatusBar();
                FreeDodrioSprites(
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                        .read(),
                );
                FreeCloudSprites();
                ((&raw mut sExitingGame).cast::<u8>().cast::<u32>()).write(1u32);
                SetGfxFuncById(8u8);
                let __p4 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                if !((IsGfxFuncActive()) != 0) {
                    SetMainCallback2(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    DestroyTask(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read(),
                    );
                    Free(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read());
                    FreeAllWindowBuffers();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetGame() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                SetGfxFuncById(9u8);
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p3 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                let __p4 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                StopMapMusic();
                let __p5 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                PlayNewMapMusic(542u16);
                StartCloudMovement();
                let __p6 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p7 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p8 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                DestroyTask(
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                        .read(),
                );
                CreateDodrioGameTask(Some(Task_NewGameIntro));
                ResetGfxState();
                InitDodrioGame(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read());
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                        .write(1u8);
                }
                SetRandomPrize();
                SetCloudInvisibility(0u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameIntro(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                if SlideTreeBordersOut() == 1u32 {
                    let __p2 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitStatusBarPos();
                let __p3 =
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if DoStatusBarIntro() == 1u32 {
                    let __p4 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32))
                    .read())
                    != 0
                {
                    CreateDodrioGameTask(Some(Task_DodrioGame_Leader));
                } else {
                    CreateDodrioGameTask(Some(Task_DodrioGame_Member));
                }
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonInfo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if (SendBlock(
                    0u8,
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12684))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize
                            * 4,
                    )),
                    1u16,
                )) != 0
                {
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(0u8);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTaskFinished()) != 0 {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (AllLinkBlocksReceived()) != 0 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12684))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .write(
                                    (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 256))
                                    .cast::<u16>())
                                    .cast::<u8>())
                                    .read(),
                                );
                                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8))
                                .write(
                                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(36))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    >= ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .read()) as i32)
                {
                    DestroyTask(taskId);
                    SetGfxFuncById(6u8);
                    let __p2 =
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_Gameplay() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
            .cast::<u8>())
        .wrapping_add(16)
        .cast::<u32>())
        .write(RecvPacket_GameState(
            0u32,
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>(),
            (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(60))
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(120))
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(180))
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(240))
            .wrapping_add(44),
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64),
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(288)
                .cast::<u32>(),
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(300)
                .cast::<u32>(),
        ));
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(296)).write(1u8);
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32)
                        && (!((RecvPacket_PickState(
                            ((i) as u32),
                            (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12704))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 60))
                            .wrapping_add(44)),
                        )) != 0))
                    {
                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 60))
                        .wrapping_add(44))
                        .write(0u8);
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(296))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (({
            let __p1 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 60i32
        {
            if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(296))
                .read())
                != 0
            {
                ClearRecvCommands();
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292))
                    .write(0u8);
            } else {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292))
                    .read()) as i32)
                    > 70i32
                {
                    ClearRecvCommands();
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292))
                        .write(0u8);
                }
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l3;
                }
                'l4: {
                    if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 60))
                    .wrapping_add(44))
                    .read()) as i32)
                        != 0i32)
                        && (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(168))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 0i32)
                    {
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(168))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(1u8);
                    }
                    'l5: {
                        let __sw3 = ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(168))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32);
                        let __matched = __sw3 == 0i32
                            || __sw3 == 1i32
                            || __sw3 == 2i32
                            || __sw3 == 3i32
                            || __sw3 == 4i32;
                        if __sw3 == 0i32 || !__matched {
                            break 'l5;
                        }
                        if __sw3 == 1i32 || __sw3 == 2i32 || __sw3 == 3i32 {
                            if (({
                                let __p4 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(176))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize);
                                let __t5 = ((__p4).read()).wrapping_add(1);
                                (__p4).write(__t5);
                                __t5
                            }) as i32)
                                >= 6i32
                            {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(168))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(44))
                                .write(0u8);
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(44))
                                .wrapping_add(4))
                                .write(0u8);
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(44))
                                .wrapping_add(8))
                                .write(0u8);
                            }
                            break 'l5;
                        }
                        if __sw3 == 4i32 {
                            if (({
                                let __p6 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(176))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize);
                                let __t7 = ((__p6).read()).wrapping_add(1);
                                (__p6).write(__t7);
                                __t7
                            }) as i32)
                                >= 40i32
                            {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(168))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(44))
                                .write(0u8);
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(44))
                                .wrapping_add(4))
                                .write(0u8);
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(44))
                                .wrapping_add(8))
                                .write(0u8);
                            }
                            break 'l5;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_ReadyToEnd() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
            .cast::<u8>())
        .wrapping_add(16)
        .cast::<u32>())
        .write(RecvPacket_GameState(
            0u32,
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>(),
            (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(60))
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(120))
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(180))
            .wrapping_add(44),
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
                .cast::<u8>())
            .wrapping_offset(240))
            .wrapping_add(44),
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64),
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(288)
                .cast::<u32>(),
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(300)
                .cast::<u32>(),
        ));
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(296)).write(1u8);
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (RecvPacket_ReadyToEnd(((i) as u32))) != 0 {
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(304))
                        .cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(1u32);
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(296))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (({
            let __p1 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 60i32
        {
            if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(296))
                .read())
                != 0
            {
                ClearRecvCommands();
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292))
                    .write(0u8);
            } else {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292))
                    .read()) as i32)
                    > 70i32
                {
                    ClearRecvCommands();
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(292))
                        .write(0u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_Leader() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24))
            .read()) as i32);
            if __sw1 == 3i32 {
                if AllPlayersReadyToStart() == 1u32 {
                    ResetReadyToStart();
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(284)
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                RecvLinkData_Gameplay();
                break 'l1;
            }
            if __sw1 == 11i32 {
                RecvLinkData_ReadyToEnd();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SendLinkData_Leader() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24))
            .read()) as i32);
            if __sw1 == 4i32 {
                SendPacket_GameState(
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13004),
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(120))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(180))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(240))
                    .wrapping_add(44),
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                        .read(),
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(288)
                        .cast::<u32>())
                    .read(),
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(300)
                        .cast::<u32>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                SendPacket_GameState(
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13004),
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(120))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(180))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(240))
                    .wrapping_add(44),
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                        .read(),
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(288)
                        .cast::<u32>())
                    .read(),
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(300)
                        .cast::<u32>())
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_Member() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24))
            .read()) as i32);
            if __sw1 == 4i32 {
                RecvPacket_GameState(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as u32),
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize
                            * 60,
                    ),
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(120))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(180))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(240))
                    .wrapping_add(44),
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64),
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(288)
                        .cast::<u32>(),
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(300)
                        .cast::<u32>(),
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                RecvPacket_GameState(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as u32),
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize
                            * 60,
                    ),
                    (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(120))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(180))
                    .wrapping_add(44),
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(240))
                    .wrapping_add(44),
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64),
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(288)
                        .cast::<u32>(),
                    (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(300)
                        .cast::<u32>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SendLinkData_Member() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24))
            .read()) as i32);
            if __sw1 == 3i32 {
                SendPacket_ReadyToStart(1u32);
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(284)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13004))
                .wrapping_add(44))
                .read()) as i32)
                    != 0i32
                {
                    SendPacket_PickState(
                        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13004))
                        .wrapping_add(44))
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (!((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(288)
                    .cast::<u32>())
                .read())
                    != 0))
                    && (!((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(300)
                        .cast::<u32>())
                    .read())
                        != 0))
                {
                    SendPacket_ReadyToEnd(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleSound_Leader() {
    unsafe {
        if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40)).read())
                as i32) as isize
                * 60,
        ))
        .wrapping_add(44))
        .read()) as i32)
            == 0i32
        {
            if !((IsSEPlaying()) != 0) {
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(324))
                    .write(0u8);
            }
        } else {
            if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12704))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                    .read()) as i32) as isize
                    * 60,
            ))
            .wrapping_add(44))
            .wrapping_add(4))
            .read()) as i32)
                == 1i32
            {
                if !((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(324))
                .read())
                    != 0)
                {
                    m4aSongNumStop(31u16);
                    PlaySE(31u16);
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(324))
                        .write(1u8);
                }
            } else {
                if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize
                        * 60,
                ))
                .wrapping_add(44))
                .wrapping_add(8))
                .read()) as i32)
                    == 1i32
                {
                    if (!((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(324))
                    .read())
                        != 0))
                        && (!((IsSEPlaying()) != 0))
                    {
                        PlaySE(22u16);
                        StartDodrioMissedAnim(1u8);
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(324))
                        .write(1u8);
                    }
                }
            }
        }
        if (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340)).read())
            as i32)
            == 0i32)
            && (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                .read()) as i32)
                >= 10i32)
        {
            StopMapMusic();
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340))
                .write(1u8);
        } else {
            if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340))
                .read()) as i32)
                == 1i32
            {
                PlayFanfareByFanfareNum(11u8);
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340))
                    .write(2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleSound_Member() {
    unsafe {
        let mut berryStart: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68)).read();
        let mut berryEnd: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72)).read();
        let mut i: u8 = 0u8;
        if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40)).read())
                as i32) as isize
                * 60,
        ))
        .wrapping_add(44))
        .read()) as i32)
            == 0i32
        {
            if (((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12704))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                    .read()) as i32) as isize
                    * 60,
            ))
            .wrapping_add(44))
            .wrapping_add(4))
            .read()) as i32)
                != 1i32)
                && (((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize
                        * 60,
                ))
                .wrapping_add(44))
                .wrapping_add(8))
                .read()) as i32)
                    != 1i32)
            {
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(324))
                    .write(0u8);
            }
        } else {
            if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12704))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                    .read()) as i32) as isize
                    * 60,
            ))
            .wrapping_add(44))
            .wrapping_add(4))
            .read()) as i32)
                == 1i32
            {
                if !((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(324))
                .read())
                    != 0)
                {
                    m4aSongNumStop(31u16);
                    PlaySE(31u16);
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(324))
                        .write(1u8);
                }
            } else {
                if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize
                        * 60,
                ))
                .wrapping_add(44))
                .wrapping_add(8))
                .read()) as i32)
                    == 1i32
                {
                    if (!((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(324))
                    .read())
                        != 0))
                        && (!((IsSEPlaying()) != 0))
                    {
                        PlaySE(22u16);
                        StartDodrioMissedAnim(1u8);
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(324))
                        .write(1u8);
                    }
                }
            }
        }
        {
            i = berryStart;
            'l1: loop {
                if !(((i) as i32) < ((berryEnd) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut berries: *mut u8 =
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 60,
                        ))
                        .wrapping_add(20);
                    if ((((((berries).wrapping_add(11)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        >= 10i32
                    {
                        if !((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(328))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0)
                        {
                            PlaySE(
                                (((74i32).wrapping_add(
                                    (((((berries).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                )) as u16),
                            );
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(328))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(1u8);
                        }
                    } else {
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(328))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340)).read())
            as i32)
            == 0i32)
            && (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                .read()) as i32)
                >= 10i32)
        {
            StopMapMusic();
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340))
                .write(1u8);
        } else {
            if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340))
                .read()) as i32)
                == 1i32
            {
                PlayFanfareByFanfareNum(11u8);
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340))
                    .write(2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_DodrioGame() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_DodrioGame() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
    }
}
pub(crate) unsafe extern "C" fn InitMonInfo(monInfo: *mut u8, mon: *mut u8) {
    unsafe {
        let mut monInfo = monInfo;
        let mut mon = mon;
        (monInfo).write(IsMonShiny(mon));
    }
}
pub(crate) unsafe extern "C" fn CreateTask_(func: Option<unsafe extern "C" fn(u8)>, priority: u8) {
    unsafe {
        let mut func = func;
        let mut priority = priority;
        CreateTask(func, priority);
    }
}
pub(crate) unsafe extern "C" fn CreateDodrioGameTask(func: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut func = func;
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(CreateTask(func, 1u8));
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).write(0u8);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12)).write(0u8);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetGameFunc(funcId: u8) {
    unsafe {
        let mut funcId = funcId;
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(28)).write(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24)).read(),
        );
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24)).write(funcId);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).write(0u8);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SlideTreeBordersOut() -> u32 {
    unsafe {
        let mut x: u8 = ((crate::c::div_i32(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20)).read())
                as i32),
            4i32,
        )) as u8);
        let __p1 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((x) as i32) != 0i32)
            && (crate::c::rem_i32(
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20))
                    .read()) as i32),
                4i32,
            ) == 0i32)
        {
            if ((x) as i32)
                < ((((((&raw const sTreeBorderXPos).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as isize,
                    ))
                .read()) as i32)
            {
                SetGpuReg(20u8, ((((x) as i32).wrapping_mul(8i32)) as u16));
                SetGpuReg(
                    24u8,
                    (((((x) as i32).wrapping_mul(8i32)).wrapping_neg()) as u16),
                );
                return 0u32;
            } else {
                return 1u32;
            }
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn InitFirstWaveOfBerries() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut berryStart: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68)).read();
        let mut berryEnd: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72)).read();
        {
            i = berryStart;
            'l1: loop {
                if !(((i) as i32) < ((berryEnd) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut berries: *mut u8 =
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13004))
                        .wrapping_add(20);
                    ((((berries).wrapping_add(11)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((if crate::c::rem_i32(((i) as i32), 2i32) == 0i32 {
                            1i32
                        } else {
                            0i32
                        }) as u8),
                    );
                    (((berries).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandlePickBerries() {
    unsafe {
        let mut berryStart: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68)).read();
        let mut berryEnd: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72)).read();
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u8 = 0u8;
        let mut column: u8 = 0u8;
        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read())
            as i32)
            >= 10i32
        {
            return;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut pickState: *mut u8 =
                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 60))
                        .wrapping_add(44));
                    if ((((pickState).read()) as i32) != 0i32)
                        && (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(168))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 1i32)
                    {
                        {
                            j = berryStart;
                            'l3: loop {
                                if !(((j) as i32) < ((berryEnd) as i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    column = ((((((&raw const sActiveColumnMap)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .cast::<u8>())
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read();
                                    if ((((((((((&raw mut sGame)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(244))
                                    .cast::<u8>())
                                    .wrapping_offset(((column) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .read()) as i32)
                                        == ((i) as i32))
                                        || (((((((((((&raw mut sGame)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(244))
                                        .cast::<u8>())
                                        .wrapping_offset(((column) as i32) as isize * 2))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read())
                                            as i32)
                                            == ((i) as i32))
                                    {
                                        break 'l3;
                                    }
                                    if TryPickBerry(i, (pickState).read(), column) == 1u32 {
                                        {
                                            k = 0u8;
                                            'l5: loop {
                                                if !(((k) as u32) < crate::c::div_u32(2u32, 1u32)) {
                                                    break 'l5;
                                                }
                                                'l6: {
                                                    if ((((((((((&raw mut sGame)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(244))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((column) as i32) as isize * 2,
                                                    ))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((k) as i32) as isize))
                                                    .read())
                                                        as i32)
                                                        == 255i32
                                                    {
                                                        ((((((((&raw mut sGame)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(244))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((column) as i32) as isize * 2,
                                                        ))
                                                        .cast::<u8>())
                                                        .wrapping_offset(((k) as i32) as isize))
                                                        .write(i);
                                                        ((((((&raw mut sGame)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(168))
                                                        .cast::<u8>())
                                                        .wrapping_offset(((i) as i32) as isize))
                                                        .write(2u8);
                                                        ((((((&raw mut sGame)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(196))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((column) as i32) as isize,
                                                        ))
                                                        .write(1u8);
                                                        break 'l5;
                                                    }
                                                }
                                                k = (k).wrapping_add(1);
                                            }
                                        }
                                        break 'l3;
                                    }
                                    if ((((((((((&raw mut sGame)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12704))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 60))
                                    .wrapping_add(44))
                                    .wrapping_add(8))
                                    .read()) as i32)
                                        == 1i32
                                    {
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = berryStart;
            'l7: loop {
                if !(((j) as i32) < ((berryEnd) as i32)) {
                    break 'l7;
                }
                'l8: {
                    let mut playerIdMissed: u8 = 255u8;
                    column = ((((((&raw const sActiveColumnMap).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .cast::<u8>())
                    .cast::<u8>())
                    .wrapping_offset(((j) as i32) as isize))
                    .read();
                    if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(196))
                    .cast::<u8>())
                    .wrapping_offset(((column) as i32) as isize))
                    .read()) as i32)
                        == 1i32
                    {
                        let mut delayRemaining: i32 = 0i32;
                        let mut playerIdPicked: u8 = 0u8;
                        let mut delayStage: u8 = ((crate::c::div_i32(
                            ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .cast::<u8>())
                            .wrapping_offset(((GetPlayerIdAtColumn(column)) as i32) as isize))
                            .read()) as i32),
                            7i32,
                        )) as u8);
                        if ((delayStage) as u32)
                            >= (crate::c::div_u32(9u32, 3u32)).wrapping_sub(1u32)
                        {
                            delayStage =
                                (((crate::c::div_u32(9u32, 3u32)).wrapping_sub(1u32)) as u8);
                        }
                        delayRemaining = ((((((((&raw const sBerryFallDelays)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((delayStage) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12704))
                            .cast::<u8>())
                            .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(208))
                                .cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                                .read()) as i32),
                            );
                        if delayRemaining < 6i32 {
                            let __p1 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(156))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize);
                            (__p1).write(
                                (((((__p1).read()) as i32).wrapping_add(delayRemaining)) as u8),
                            );
                        }
                        if (({
                            let __p2 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(156))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize);
                            let __t3 = ((__p2).read()).wrapping_add(1);
                            (__p2).write(__t3);
                            __t3
                        }) as i32)
                            >= 6i32
                        {
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(156))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .write(0u8);
                            if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(244))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize * 2))
                            .cast::<u8>())
                            .read()) as i32)
                                == 255i32)
                                && (((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(244))
                                .cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    == 255i32)
                            {
                                break 'l8;
                            } else {
                                if ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(244))
                                .cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize * 2))
                                .cast::<u8>())
                                .read()) as i32)
                                    != 255i32)
                                    && (((((((((((&raw mut sGame)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(244))
                                    .cast::<u8>())
                                    .wrapping_offset(((column) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        == 255i32)
                                {
                                    playerIdPicked =
                                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(244))
                                        .cast::<u8>())
                                        .wrapping_offset(((column) as i32) as isize * 2))
                                        .cast::<u8>())
                                        .read();
                                } else {
                                    let mut playerId1: u8 =
                                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(244))
                                        .cast::<u8>())
                                        .wrapping_offset(((column) as i32) as isize * 2))
                                        .cast::<u8>())
                                        .read();
                                    i = ((((((((&raw mut sGame)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(244))
                                    .cast::<u8>())
                                    .wrapping_offset(((column) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read();
                                    if !((((Random()) as i32) & 1i32) != 0) {
                                        playerIdPicked = playerId1;
                                        playerIdMissed = i;
                                    } else {
                                        playerIdPicked = i;
                                        playerIdMissed = playerId1;
                                    }
                                }
                            }
                            ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(13004))
                            .wrapping_add(20))
                            .wrapping_add(11))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .write(7u8);
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(196))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .write(2u8);
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(168))
                            .cast::<u8>())
                            .wrapping_offset(((playerIdPicked) as i32) as isize))
                            .write(3u8);
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(184))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .write(playerIdPicked);
                            ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12704))
                            .cast::<u8>())
                            .wrapping_offset(((playerIdPicked) as i32) as isize * 60))
                            .wrapping_add(44))
                            .wrapping_add(4))
                            .write(1u8);
                            if ((playerIdMissed) as i32) != 255i32 {
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((playerIdMissed) as i32) as isize * 60))
                                .wrapping_add(44))
                                .wrapping_add(8))
                                .write(1u8);
                            }
                            let __p4 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(134))
                            .cast::<u16>())
                            .wrapping_offset(((playerIdPicked) as i32) as isize);
                            (__p4).write(((__p4).read()).wrapping_add(1));
                            IncrementBerryResult(0u8, column, playerIdPicked);
                            UpdateBerriesPickedInRow(1u32);
                            TryIncrementDifficulty(playerIdPicked);
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(232))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .write(
                                (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(13004))
                                .wrapping_add(20))
                                .cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                                .read(),
                            );
                            (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(13004))
                            .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                            .write(3u8);
                            (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(244))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize * 2))
                            .cast::<u8>())
                            .write(255u8);
                            ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(244))
                            .cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .write(255u8);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryPickBerry(playerId: u8, pickState: u8, column: u8) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut pickState = pickState;
        let mut column = column;
        let mut pick: i32 = 0i32;
        let mut numPlayersIdx: u8 =
            ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .read()) as i32)
                .wrapping_sub(1i32)) as u8);
        let mut berries: *mut u8 = ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(13004))
        .wrapping_add(20);
        'l1: {
            let __sw1 = ((pickState) as i32);
            let __matched = __sw1 == 3i32 || __sw1 == 2i32 || __sw1 == 1i32;
            if __sw1 == 3i32 || !__matched {
                pick = 0i32;
                break 'l1;
            }
            if __sw1 == 2i32 {
                pick = 1i32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                pick = 2i32;
                break 'l1;
            }
        }
        if (((((((berries).wrapping_add(11)).cast::<u8>())
            .wrapping_offset(((column) as i32) as isize))
        .read()) as i32)
            == 6i32)
            || (((((((berries).wrapping_add(11)).cast::<u8>())
                .wrapping_offset(((column) as i32) as isize))
            .read()) as i32)
                == 7i32)
        {
            if ((column) as i32)
                == ((((((((((&raw const sDodrioHeadToColumnMap).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((numPlayersIdx) as i32) as isize * 15))
                .cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 3))
                .cast::<u8>())
                .wrapping_offset((pick) as isize))
                .read()) as i32)
            {
                if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(196))
                .cast::<u8>())
                .wrapping_offset(((column) as i32) as isize))
                .read()) as i32)
                    == 1i32)
                    || (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(196))
                    .cast::<u8>())
                    .wrapping_offset(((column) as i32) as isize))
                    .read()) as i32)
                        == 2i32)
                {
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize * 60))
                    .wrapping_add(44))
                    .wrapping_add(8))
                    .write(1u8);
                    return 0u32;
                } else {
                    return 1u32;
                }
            }
        } else {
            if ((column) as i32)
                == ((((((((((&raw const sDodrioHeadToColumnMap).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((numPlayersIdx) as i32) as isize * 15))
                .cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 3))
                .cast::<u8>())
                .wrapping_offset((pick) as isize))
                .read()) as i32)
            {
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                    .cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize))
                .write(4u8);
                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 60))
                .wrapping_add(44))
                .wrapping_add(8))
                .write(1u8);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateFallingBerries() {
    unsafe {
        let mut berryStart: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68)).read();
        let mut berryEnd: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72)).read();
        let mut delayStage: u8 = 0u8;
        let mut otherBerryMissed: u8 = 0u8;
        let mut i: u8 = 0u8;
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(288)
            .cast::<u32>())
        .write(0u32);
        {
            i = berryStart;
            'l1: loop {
                if !(((i) as i32) < ((berryEnd) as i32).wrapping_sub(1i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut game: *mut u8 =
                        ((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read();
                    if (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(196))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32)
                        || (((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(196))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 1i32)
                    {
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(288)
                            .cast::<u32>())
                        .write(1u32);
                        if ((((((((game).wrapping_add(13004)).wrapping_add(20)).wrapping_add(11))
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            >= 10i32
                        {
                            ((((((game).wrapping_add(13004)).wrapping_add(20)).wrapping_add(11))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(10u8);
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(196))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(3u8);
                            if !((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(328))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                                != 0)
                            {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(328))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(1u8);
                                PlaySE(
                                    (((74i32).wrapping_add(
                                        (((((((game).wrapping_add(13004)).wrapping_add(20))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                                );
                            }
                            if (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(64))
                            .read()) as i32)
                                < 10i32)
                                || (((otherBerryMissed) as i32) == 1i32)
                            {
                                otherBerryMissed = 1u8;
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(328))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(64))
                                .read()) as i32)
                                    < 10i32
                                {
                                    let __p1 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(64);
                                    (__p1).write(((__p1).read()).wrapping_add(1));
                                }
                                IncrementBerryResult(3u8, i, 0u8);
                                UpdateBerriesPickedInRow(0u32);
                            }
                        } else {
                            let mut delay: u8 = 0u8;
                            delayStage = ((crate::c::div_i32(
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(144))
                                .cast::<u8>())
                                .wrapping_offset(((GetPlayerIdAtColumn(i)) as i32) as isize))
                                .read()) as i32),
                                7i32,
                            )) as u8);
                            if ((delayStage) as u32)
                                >= (crate::c::div_u32(9u32, 3u32)).wrapping_sub(1u32)
                            {
                                delayStage =
                                    (((crate::c::div_u32(9u32, 3u32)).wrapping_sub(1u32)) as u8);
                            }
                            delay = ((((((&raw const sBerryFallDelays).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((delayStage) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((game).wrapping_add(13004)).wrapping_add(20)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize,
                            ))
                            .read();
                            if (({
                                let __p2 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(208))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize);
                                let __t3 = ((__p2).read()).wrapping_add(1);
                                (__p2).write(__t3);
                                __t3
                            }) as i32)
                                >= ((delay) as i32)
                            {
                                let __p4 = (((((game).wrapping_add(13004)).wrapping_add(20))
                                    .wrapping_add(11))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(208))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                            }
                            HandlePickBerries();
                        }
                    } else {
                        if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(196))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 2i32
                        {
                            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(288)
                                .cast::<u32>())
                            .write(1u32);
                            if (({
                                let __p5 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize);
                                let __t6 = ((__p5).read()).wrapping_add(1);
                                (__p5).write(__t6);
                                __t6
                            }) as i32)
                                >= 20i32
                            {
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(184))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 60,
                                ))
                                .wrapping_add(44))
                                .wrapping_add(4))
                                .write(0u8);
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(208))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(196))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                ((((((game).wrapping_add(13004)).wrapping_add(20))
                                    .wrapping_add(11))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(1u8);
                                (((((game).wrapping_add(13004)).wrapping_add(20)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(GetNewBerryId(GetPlayerIdAtColumn(i), i));
                            }
                        } else {
                            if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(196))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                == 3i32
                            {
                                if (({
                                    let __p7 =
                                        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize);
                                    let __t8 = ((__p7).read()).wrapping_add(1);
                                    (__p7).write(__t8);
                                    __t8
                                }) as i32)
                                    >= 20i32
                                {
                                    if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(64))
                                    .read()) as i32)
                                        < 10i32
                                    {
                                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(0u8);
                                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(208))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(0u8);
                                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(196))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(0u8);
                                        ((((((game).wrapping_add(13004)).wrapping_add(20))
                                            .wrapping_add(11))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(1u8);
                                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(232))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            (((((game).wrapping_add(13004)).wrapping_add(20))
                                                .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read(),
                                        );
                                        (((((game).wrapping_add(13004)).wrapping_add(20))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(GetNewBerryId(GetPlayerIdAtColumn(i), i));
                                    }
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBerrySprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut berryStart: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68)).read();
        let mut berryEnd: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72)).read();
        {
            i = berryStart;
            'l1: loop {
                if !(((i) as i32) < ((berryEnd) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut player: *mut u8 =
                        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 60,
                        );
                    let mut column: u8 =
                        ((((((((&raw const sActiveColumnMap).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(36))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 55,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 11,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                    if (((((((player).wrapping_add(20)).wrapping_add(11)).cast::<u8>())
                        .wrapping_offset(((column) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        SetBerryInvisibility(i, 0u8);
                    } else {
                        SetBerryInvisibility(i, 1u8);
                    }
                    if (((((((player).wrapping_add(20)).wrapping_add(11)).cast::<u8>())
                        .wrapping_offset(((column) as i32) as isize))
                    .read()) as i32)
                        >= 10i32
                    {
                        SetBerryAnim(
                            ((i) as u16),
                            ((((((((player).wrapping_add(20)).cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                            .read()) as i32)
                                .wrapping_add(3i32)) as u8),
                        );
                        SetBerryYPos(
                            i,
                            ((((((((((player).wrapping_add(20)).wrapping_add(11)).cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                            .read()) as i32)
                                .wrapping_mul(2i32))
                            .wrapping_sub(1i32)) as u8),
                        );
                    } else {
                        if ((((((player).wrapping_add(20)).cast::<u8>())
                            .wrapping_offset(((column) as i32) as isize))
                        .read()) as i32)
                            == 3i32
                        {
                            (((((player).wrapping_add(20)).wrapping_add(11)).cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                            .write(7u8);
                            SetBerryAnim(((i) as u16), 6u8);
                            SetBerryYPos(
                                i,
                                ((((((((((player).wrapping_add(20)).wrapping_add(11))
                                    .cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_mul(2i32))
                                .wrapping_sub(1i32)) as u8),
                            );
                        } else {
                            SetBerryAnim(
                                ((i) as u16),
                                ((((player).wrapping_add(20)).cast::<u8>())
                                    .wrapping_offset(((column) as i32) as isize))
                                .read(),
                            );
                            SetBerryYPos(
                                i,
                                (((((((((player).wrapping_add(20)).wrapping_add(11)).cast::<u8>())
                                    .wrapping_offset(((column) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_mul(2i32)) as u8),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateAllDodrioAnims() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 = 0u8;
        numPlayers =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut player: *mut u8 =
                        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 60);
                    SetDodrioAnim(i, ((player).wrapping_add(44)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetAllDodrioDisabled() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 = 0u8;
        numPlayers =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    SetDodrioAnim(i, 4u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateGame_Leader() {
    unsafe {
        UpdateBerrySprites();
        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read())
            as i32)
            >= 10i32
        {
            SetAllDodrioDisabled();
        } else {
            UpdateAllDodrioAnims();
        }
        UpdateStatusBarAnim(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateGame_Member() {
    unsafe {
        UpdateBerrySprites();
        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read())
            as i32)
            >= 10i32
        {
            SetAllDodrioDisabled();
        } else {
            UpdateAllDodrioAnims();
        }
        UpdateStatusBarAnim(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetActiveBerryColumns(
    numPlayers: u8,
    start: *mut u8,
    end: *mut u8,
) {
    unsafe {
        let mut numPlayers = numPlayers;
        let mut start = start;
        let mut end = end;
        'l1: {
            let __sw1 = ((numPlayers) as i32);
            if __sw1 == 1i32 {
                (start).write(4u8);
                (end).write(7u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                (start).write(3u8);
                (end).write(8u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                (start).write(2u8);
                (end).write(9u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                (start).write(1u8);
                (end).write(10u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                (start).write(0u8);
                (end).write(11u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AllPlayersReadyToStart() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 = 0u8;
        numPlayers =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(344))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(((RecvPacket_ReadyToStart(((i) as u32))) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        numPlayers = numPlayers;
        {
            'l3: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l3;
                }
                'l4: {
                    if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ResetReadyToStart() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReadyToEndGame_Leader() -> u32 {
    unsafe {
        if (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read())
            as i32)
            >= 10i32)
            && (!((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(288)
                .cast::<u32>())
            .read())
                != 0))
        {
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                .write(10u8);
            if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(300)
                .cast::<u32>())
            .read())
                != 0
            {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ReadyToEndGame_Member() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut berryStart: u8 = 0u8;
        let mut berryEnd: u8 = 0u8;
        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).read())
            as i32)
            >= 10i32
        {
            berryStart = ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(68))
            .read();
            berryEnd = ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(72))
            .read();
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
                .write(10u8);
            if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(300)
                .cast::<u32>())
            .read())
                != 0
            {
                {
                    i = berryStart;
                    'l1: loop {
                        if !(((i) as i32) < ((berryEnd) as i32)) {
                            break 'l1;
                        }
                        'l2: {
                            let mut player: *mut u8 =
                                (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(40))
                                    .read()) as i32) as isize
                                        * 60,
                                );
                            let mut column: u8 = ((((((((&raw const sActiveColumnMap)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 55,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(40))
                                .read()) as i32) as isize
                                    * 11,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                            if (((((((player).wrapping_add(20)).wrapping_add(11)).cast::<u8>())
                                .wrapping_offset(((column) as i32) as isize))
                            .read()) as i32)
                                != 10i32
                            {
                                return 0u32;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TryIncrementDifficulty(playerId: u8) {
    unsafe {
        let mut playerId = playerId;
        let mut threshold: u8 =
            ((((((((&raw const sDifficultyThresholds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (crate::c::rem_i32(
                        ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize))
                        .read()) as i32),
                        7i32,
                    )) as isize,
                ))
            .read()) as i32)
                .wrapping_add(
                    (crate::c::div_i32(
                        ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize))
                        .read()) as i32),
                        7i32,
                    ))
                    .wrapping_mul(100i32),
                )) as u8);
        if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(134))
            .cast::<u16>())
        .wrapping_offset(((playerId) as i32) as isize))
        .read()) as i32)
            >= ((threshold) as i32)
        {
            let __p1 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(144))
            .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn GetPlayerIdAtColumn(column: u8) -> u8 {
    unsafe {
        let mut column = column;
        return ((((((&raw const sPlayerIdAtColumn).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read()) as i32))).wrapping_sub(1i32)) as isize * 11)).cast::<u8>()).wrapping_offset((((column) as i32)) as isize)).read();
    }
}
pub(crate) unsafe extern "C" fn GetNewBerryId(playerId: u8, column: u8) -> u8 {
    unsafe {
        let mut playerId = playerId;
        let mut column = column;
        let mut i: u8 = 0u8;
        let mut highestDifficulty: u8 = 0u8;
        let mut numPlayersIdx: u8 =
            ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .read()) as i32)
                .wrapping_sub(1i32)) as u8);
        let mut leftPlayer: u8 =
            (((((((&raw const sDodrioNeighborMap).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((numPlayersIdx) as i32) as isize * 15))
            .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 3))
            .cast::<u8>())
            .read();
        let mut middlePlayer: u8 =
            ((((((((&raw const sDodrioNeighborMap).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((numPlayersIdx) as i32) as isize * 15))
            .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 3))
            .cast::<u8>())
            .wrapping_offset(1))
            .read();
        let mut rightPlayer: u8 =
            ((((((((&raw const sDodrioNeighborMap).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((numPlayersIdx) as i32) as isize * 15))
            .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 3))
            .cast::<u8>())
            .wrapping_offset(2))
            .read();
        {
            i = 0u8;
            'l1: loop {
                if !(((((((((&raw const sUnsharedColumns).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((numPlayersIdx) as i32) as isize * 5))
                .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize))
                .read()) as i32)
                    != 0i32)
                {
                    break 'l1;
                }
                'l2: {
                    if ((column) as i32)
                        == ((((((((&raw const sUnsharedColumns).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((numPlayersIdx) as i32) as isize * 5))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return GetNewBerryIdByDifficulty(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .cast::<u8>())
                            .wrapping_offset(((middlePlayer) as i32) as isize))
                            .read(),
                            column,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
            .cast::<u8>())
        .wrapping_offset(((leftPlayer) as i32) as isize))
        .read()) as i32)
            > ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
                .cast::<u8>())
            .wrapping_offset(((middlePlayer) as i32) as isize))
            .read()) as i32)
        {
            highestDifficulty = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(144))
            .cast::<u8>())
            .wrapping_offset(((leftPlayer) as i32) as isize))
            .read();
        } else {
            highestDifficulty = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(144))
            .cast::<u8>())
            .wrapping_offset(((middlePlayer) as i32) as isize))
            .read();
        }
        if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
            .cast::<u8>())
        .wrapping_offset(((rightPlayer) as i32) as isize))
        .read()) as i32)
            > ((highestDifficulty) as i32)
        {
            highestDifficulty = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(144))
            .cast::<u8>())
            .wrapping_offset(((rightPlayer) as i32) as isize))
            .read();
        }
        return GetNewBerryIdByDifficulty(highestDifficulty, column);
    }
}
pub(crate) unsafe extern "C" fn GetNewBerryIdByDifficulty(difficulty: u8, column: u8) -> u8 {
    unsafe {
        let mut difficulty = difficulty;
        let mut column = column;
        let mut prevBerryId: u8 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(232))
        .cast::<u8>())
        .wrapping_offset(((column) as i32) as isize))
        .read();
        'l1: {
            let __sw1 = crate::c::rem_i32(((difficulty) as i32), 7i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            let mut __fall = false;
            if !__matched {
                __fall = true;
                return 0u8;
            }
            if __sw1 == 0i32 {
                __fall = true;
                return 0u8;
            }
            if __sw1 == 1i32 {
                __fall = true;
                return 1u8;
            }
            if __sw1 == 2i32 {
                __fall = true;
                return 2u8;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if ((prevBerryId) as i32) == 0i32 {
                    return 1u8;
                } else {
                    return 0u8;
                }
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                if ((prevBerryId) as i32) == 0i32 {
                    return 2u8;
                } else {
                    return 0u8;
                }
            }
            if __fall || __sw1 == 5i32 {
                __fall = true;
                if ((prevBerryId) as i32) == 2i32 {
                    return 1u8;
                } else {
                    return 2u8;
                }
            }
            if __fall || __sw1 == 6i32 {
                __fall = true;
                if ((prevBerryId) as i32) == 0i32 {
                    return 1u8;
                } else {
                    if ((prevBerryId) as i32) == 1i32 {
                        return 2u8;
                    } else {
                        return 0u8;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsTotalBerriesMissedOver10(berryResults: *mut u8) -> u32 {
    unsafe {
        let mut berryResults = berryResults;
        let mut missed: i32 = 0i32;
        let mut i: i32 = 0i32;
        {
            'l1: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {}
                missed = (missed).wrapping_add(
                    ((((((berryResults).wrapping_offset((i) as isize * 12)).cast::<u16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                );
                i = (i).wrapping_add(1);
            }
        }
        if missed > 10i32 {
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
pub(crate) unsafe extern "C" fn IncrementBerryResult(berryIdArg: u8, column: u8, playerId: u8) {
    unsafe {
        let mut berryIdArg = berryIdArg;
        let mut column = column;
        let mut playerId = playerId;
        let mut berryId: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        'l1: {
            let __sw1 = ((berryIdArg) as i32);
            if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 {
                berryId = ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_add(20))
                .cast::<u8>())
                .wrapping_offset(((column) as i32) as isize))
                .read();
                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74))
                .cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 12))
                .cast::<u16>())
                .wrapping_offset(((berryId) as i32) as isize))
                .write(
                    ((IncrementWithLimit(
                        ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74))
                        .cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 12))
                        .cast::<u16>())
                        .wrapping_offset(((berryId) as i32) as isize))
                        .read()) as u32),
                        20000u32,
                    )) as u16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsTotalBerriesMissedOver10(
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(74))
                        .cast::<u8>(),
                )) != 0
                {
                    break 'l1;
                }
                'l2: {
                    let __sw2 = ((numPlayers) as i32);
                    if __sw2 == 5i32 {
                        'l3: {
                            let __sw3 = ((column) as i32);
                            if __sw3 == 0i32 {
                                let __p4 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                                let __p5 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(36))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p5).write(((__p5).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 1i32 {
                                let __p6 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(36))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p6).write(((__p6).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 2i32 {
                                let __p7 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(36))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p7).write(((__p7).read()).wrapping_add(1));
                                let __p8 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(48))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p8).write(((__p8).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 3i32 {
                                let __p9 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(48))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p9).write(((__p9).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 4i32 {
                                let __p10 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(48))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p10).write(((__p10).read()).wrapping_add(1));
                                let __p11 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p11).write(((__p11).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 5i32 {
                                let __p12 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p12).write(((__p12).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 6i32 {
                                let __p13 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p13).write(((__p13).read()).wrapping_add(1));
                                let __p14 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p14).write(((__p14).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 7i32 {
                                let __p15 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p15).write(((__p15).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 8i32 {
                                let __p16 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p16).write(((__p16).read()).wrapping_add(1));
                                let __p17 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p17).write(((__p17).read()).wrapping_add(1));
                                break 'l3;
                            }
                            if __sw3 == 9i32 {
                                let __p18 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p18).write(((__p18).read()).wrapping_add(1));
                                break 'l3;
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        'l4: {
                            let __sw19 = ((column) as i32);
                            if __sw19 == 1i32 {
                                let __p20 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p20).write(((__p20).read()).wrapping_add(1));
                                let __p21 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(36))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p21).write(((__p21).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 2i32 {
                                let __p22 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(36))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p22).write(((__p22).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 3i32 {
                                let __p23 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(36))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p23).write(((__p23).read()).wrapping_add(1));
                                let __p24 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p24).write(((__p24).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 4i32 {
                                let __p25 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p25).write(((__p25).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 5i32 {
                                let __p26 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p26).write(((__p26).read()).wrapping_add(1));
                                let __p27 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p27).write(((__p27).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 6i32 {
                                let __p28 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p28).write(((__p28).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 7i32 {
                                let __p29 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p29).write(((__p29).read()).wrapping_add(1));
                                let __p30 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p30).write(((__p30).read()).wrapping_add(1));
                                break 'l4;
                            }
                            if __sw19 == 8i32 {
                                let __p31 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p31).write(((__p31).read()).wrapping_add(1));
                                break 'l4;
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        'l5: {
                            let __sw32 = ((column) as i32);
                            if __sw32 == 2i32 {
                                let __p33 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p33).write(((__p33).read()).wrapping_add(1));
                                let __p34 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p34).write(((__p34).read()).wrapping_add(1));
                                break 'l5;
                            }
                            if __sw32 == 3i32 {
                                let __p35 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p35).write(((__p35).read()).wrapping_add(1));
                                break 'l5;
                            }
                            if __sw32 == 4i32 {
                                let __p36 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(24))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p36).write(((__p36).read()).wrapping_add(1));
                                let __p37 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p37).write(((__p37).read()).wrapping_add(1));
                                break 'l5;
                            }
                            if __sw32 == 5i32 {
                                let __p38 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p38).write(((__p38).read()).wrapping_add(1));
                                break 'l5;
                            }
                            if __sw32 == 6i32 {
                                let __p39 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p39).write(((__p39).read()).wrapping_add(1));
                                let __p40 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p40).write(((__p40).read()).wrapping_add(1));
                                break 'l5;
                            }
                            if __sw32 == 7i32 {
                                let __p41 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p41).write(((__p41).read()).wrapping_add(1));
                                break 'l5;
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        'l6: {
                            let __sw42 = ((column) as i32);
                            if __sw42 == 3i32 {
                                let __p43 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p43).write(((__p43).read()).wrapping_add(1));
                                let __p44 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p44).write(((__p44).read()).wrapping_add(1));
                                break 'l6;
                            }
                            if __sw42 == 4i32 {
                                let __p45 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p45).write(((__p45).read()).wrapping_add(1));
                                break 'l6;
                            }
                            if __sw42 == 5i32 {
                                let __p46 =
                                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p46).write(((__p46).read()).wrapping_add(1));
                                let __p47 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p47).write(((__p47).read()).wrapping_add(1));
                                break 'l6;
                            }
                            if __sw42 == 6i32 {
                                let __p48 =
                                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(74))
                                    .cast::<u8>())
                                    .wrapping_offset(12))
                                    .cast::<u16>())
                                    .wrapping_offset(3);
                                (__p48).write(((__p48).read()).wrapping_add(1));
                                break 'l6;
                            }
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBerriesPickedInRow(picked: u32) {
    unsafe {
        let mut picked = picked;
        if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read())
            as i32)
            != 5i32
        {
            return;
        }
        if picked == 1u32 {
            if (({
                let __p1 = (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(274)
                    .cast::<u16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276)
                    .cast::<u16>())
                .read()) as i32)
            {
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(274)
                        .cast::<u16>())
                    .read(),
                );
            }
            if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(274)
                .cast::<u16>())
            .read()) as i32)
                > 9999i32
            {
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(274)
                    .cast::<u16>())
                .write(9999u16);
            }
        } else {
            if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(274)
                .cast::<u16>())
            .read()) as i32)
                > ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276)
                    .cast::<u16>())
                .read()) as i32)
            {
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(274)
                        .cast::<u16>())
                    .read(),
                );
            }
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(274)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SetMaxBerriesPickedInRow() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(5))
                    .write(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(276)
                            .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetForPlayAgainPrompt() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 11i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12704))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 60))
                                .wrapping_add(20))
                                .wrapping_add(11))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 60))
                    .wrapping_add(44))
                    .write(0u8);
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 60))
                    .wrapping_add(44))
                    .wrapping_add(4))
                    .write(0u8);
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(134))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13064))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .write(0u8);
                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13064))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .write(0u32);
                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .write(0u16);
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(2))
                    .write(0u16);
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(3))
                    .write(0u16);
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(4))
                    .write(0u16);
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(5))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(340)).write(0u8);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(274)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64)).write(0u8);
        UpdateAllDodrioAnims();
        UpdateBerrySprites();
    }
}
pub(crate) unsafe extern "C" fn SetRandomPrize() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut prizeSet: u8 = 0u8;
        let mut prizeIdx: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .read()) as i32);
            if __sw1 == 4i32 {
                prizeSet = 1u8;
                break 'l1;
            }
            if __sw1 == 5i32 {
                prizeSet = 2u8;
                break 'l1;
            }
        }
        prizeIdx = ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(10u32, 1u32))) as u8);
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l2;
                }
                'l3: {
                    ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .wrapping_offset(4))
                    .write(
                        ((((((((&raw const sPrizeBerryIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((prizeSet) as i32) as isize * 10))
                        .cast::<u8>())
                        .wrapping_offset(((prizeIdx) as i32) as isize))
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetBerriesPicked(playerId: u8) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut sum: u32 = ((((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74))
        .cast::<u8>())
        .wrapping_offset(((playerId) as i32) as isize * 12))
        .cast::<u16>())
        .read()) as i32)
            .wrapping_add(
                ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74))
                .cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 12))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32),
            ))
        .wrapping_add(
            ((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(74))
                .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 12))
            .cast::<u16>())
            .wrapping_offset(2))
            .read()) as i32),
        )) as u32);
        return (if sum < 9999u32 { sum } else { 9999u32 });
    }
}
pub(crate) unsafe extern "C" fn TryUpdateRecords() {
    unsafe {
        let mut berriesPicked: u32 = Min(
            GetBerriesPicked(
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                    .read(),
            ),
            9999u32,
        );
        let mut score: u32 = Min(
            GetScore(
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                    .read(),
            ),
            999990u32,
        );
        if (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
            .cast::<u32>())
        .read()
            < score
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
                .cast::<u32>())
            .write(score);
        }
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as u32)
            < berriesPicked
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
                .wrapping_add(4)
                .cast::<u16>())
            .write(((berriesPicked) as u16));
        }
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(276)
                .cast::<u16>())
            .read()) as i32)
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
                .wrapping_add(6)
                .cast::<u16>())
            .write(
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePickStateQueue(pickState: u8) -> u8 {
    unsafe {
        let mut pickState = pickState;
        let mut i: u8 = 0u8;
        let mut nextState: u8 = 0u8;
        nextState = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152))
        .cast::<u8>())
        .wrapping_offset((((crate::c::div_u32(4u32, 1u32)).wrapping_sub(1u32)) as i32) as isize))
        .read();
        {
            i = (((crate::c::div_u32(4u32, 1u32)).wrapping_sub(1u32)) as u8);
            'l1: loop {
                if !(((i) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(152))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(152))
                        .cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_sub(1);
            }
        }
        (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(152))
            .cast::<u8>())
        .write(pickState);
        return nextState;
    }
}
pub(crate) unsafe extern "C" fn HandleWaitPlayAgainInput() {
    unsafe {
        if ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(176))
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40)).read())
                as i32) as isize,
        ))
        .read()) as i32)
            == 0i32
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12704))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize
                        * 60,
                ))
                .wrapping_add(44))
                .write(2u8);
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(176))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                        .read()) as i32) as isize,
                ))
                .write(6u8);
                PlaySE(212u16);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12704))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize
                            * 60,
                    ))
                    .wrapping_add(44))
                    .write(3u8);
                    ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .read()) as i32) as isize,
                    ))
                    .write(6u8);
                    PlaySE(212u16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0
                    {
                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 60,
                        ))
                        .wrapping_add(44))
                        .write(1u8);
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize,
                        ))
                        .write(6u8);
                        PlaySE(212u16);
                    } else {
                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12704))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .read()) as i32) as isize
                                * 60,
                        ))
                        .wrapping_add(44))
                        .write(0u8);
                    }
                }
            }
        } else {
            let __p1 = (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(176))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
                    .read()) as i32) as isize,
            );
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ResetPickState() {
    unsafe {
        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12704))
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40)).read())
                as i32) as isize
                * 60,
        ))
        .wrapping_add(44))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetPrizeItemId() -> u16 {
    unsafe {
        return ((((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40)).read())
                as i32) as isize
                * 12,
        ))
        .cast::<u16>())
        .wrapping_offset(4))
        .read()) as i32)
            .wrapping_add(133i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetNumPlayers() -> u8 {
    unsafe {
        return ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
            .read();
    }
}
pub(crate) unsafe extern "C" fn GetPlayerName(id: u8) -> *mut u8 {
    unsafe {
        let mut id = id;
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            return ((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 28))
            .wrapping_add(8))
            .cast::<u8>();
        } else {
            return ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12704))
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 60))
            .cast::<u8>();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn GetBerryResult(playerId: u8, berryId: u8) -> u16 {
    unsafe {
        let mut playerId = playerId;
        let mut berryId = berryId;
        return ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74))
        .cast::<u8>())
        .wrapping_offset(((playerId) as i32) as isize * 12))
        .cast::<u16>())
        .wrapping_offset(((berryId) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetScore(playerId: u8) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut i: u8 = 0u8;
        let mut scoreLost: u32 = 0u32;
        let mut score: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    score = (score).wrapping_add(
                        ((((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74))
                        .cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 12))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_mul(
                                ((((((&raw const sBerryScoreMultipliers)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32),
                            )) as u32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        scoreLost = ((((((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74))
        .cast::<u8>())
        .wrapping_offset(((playerId) as i32) as isize * 12))
        .cast::<u16>())
        .wrapping_offset(3))
        .read()) as i32)
            .wrapping_mul(
                ((((((&raw const sBerryScoreMultipliers)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32),
            )) as u32);
        if score <= scoreLost {
            return 0u32;
        } else {
            return (score).wrapping_sub(scoreLost);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetHighestScore() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        let mut maxScore: u32 = GetScore(0u8);
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut score: u32 = GetScore(i);
                    if score > maxScore {
                        maxScore = score;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return Min(maxScore, 999990u32);
    }
}
pub(crate) unsafe extern "C" fn GetHighestBerryResult(berryId: u8) -> u32 {
    unsafe {
        let mut berryId = berryId;
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        let mut maxScore: u16 = (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74))
        .cast::<u8>())
        .cast::<u16>())
        .wrapping_offset(((berryId) as i32) as isize))
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut score: u16 =
                        ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                        .cast::<u16>())
                        .wrapping_offset(((berryId) as i32) as isize))
                        .read();
                    if ((score) as i32) > ((maxScore) as i32) {
                        maxScore = score;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((maxScore) as u32);
    }
}
pub(crate) unsafe extern "C" fn GetScoreByRanking(ranking: u8) -> u32 {
    unsafe {
        let mut ranking = ranking;
        let mut scores = crate::ffi::Align4([0u8; 20]);
        let mut temp: u32 = 0u32;
        let mut unsorted: i16 = 1i16;
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut scores).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write({
                            let __v1 = GetScore(i);
                            temp = __v1;
                            __v1
                        });
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: loop {
            if !((unsorted) != 0) {
                break 'l3;
            }
            unsorted = 0i16;
            {
                i = 0u8;
                'l4: loop {
                    if !(((i) as i32) < ((numPlayers) as i32).wrapping_sub(1i32)) {
                        break 'l4;
                    }
                    'l5: {
                        if (((&raw mut scores).cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()
                            < (((&raw mut scores).cast::<u32>())
                                .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                            .read()
                        {
                            {
                                temp = (((&raw mut scores).cast::<u32>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read();
                                (((&raw mut scores).cast::<u32>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    (((&raw mut scores).cast::<u32>()).wrapping_offset(
                                        (((i) as i32).wrapping_add(1i32)) as isize,
                                    ))
                                    .read(),
                                );
                                (((&raw mut scores).cast::<u32>())
                                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                                .write(temp);
                            }
                            unsorted = 1i16;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return (((&raw mut scores).cast::<u32>()).wrapping_offset(((ranking) as i32) as isize))
            .read();
    }
}
pub(crate) unsafe extern "C" fn SetScoreResults() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut ranking: u8 = 0u8;
        let mut nextRanking: u8 = 0u8;
        let mut playersRanked: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        GetHighestScore();
        if GetHighestScore() == 0u32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((numPlayers) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13064))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .write(4u8);
                        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13064))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .write(0u32);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13064))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .write(Min(GetScore(i), 999990u32));
                }
                i = (i).wrapping_add(1);
            }
        }
        'l5: loop {
            'l6: {
                let mut score: u32 = GetScoreByRanking(ranking);
                let mut curRanking: u8 = nextRanking;
                {
                    i = 0u8;
                    'l7: loop {
                        if !(((i) as i32) < ((numPlayers) as i32)) {
                            break 'l7;
                        }
                        'l8: {
                            if score
                                == (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(13064))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .wrapping_add(4)
                                .cast::<u32>())
                                .read()
                            {
                                ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(13064))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .write(curRanking);
                                nextRanking = (nextRanking).wrapping_add(1);
                                playersRanked = (playersRanked).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ranking = nextRanking;
            }
            if !(((playersRanked) as i32) < ((numPlayers) as i32)) {
                break 'l5;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetScoreResults(dst: *mut u8, playerId: u8) {
    unsafe {
        let mut dst = dst;
        let mut playerId = playerId;
        dst.cast::<crate::c::Rec4<8>>().write_unaligned(
            (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13064))
                .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 8)
            .cast::<crate::c::Rec4<8>>()
            .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetScoreRanking(playerId: u8) -> u8 {
    unsafe {
        let mut playerId = playerId;
        let mut i: u8 = 0u8;
        let mut ranking: u8 = 0u8;
        let mut numPlayers: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).read();
        let mut playersScore: u32 = 0u32;
        let mut scores = crate::ffi::Align4([0u8; 20]);
        (&raw mut scores)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u32>()
            .write(0u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut scores).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write(GetScore(i));
                }
                i = (i).wrapping_add(1);
            }
        }
        playersScore = (((&raw mut scores).cast::<u32>())
            .wrapping_offset(((playerId) as i32) as isize))
        .read();
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l3;
                }
                'l4: {
                    if (((i) as i32) != ((playerId) as i32))
                        && (playersScore
                            < (((&raw mut scores).cast::<u32>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                    {
                        ranking = (ranking).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ranking;
    }
}
pub(crate) unsafe extern "C" fn TryGivePrize() -> u8 {
    unsafe {
        let mut multiplayerId: u8 =
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40)).read();
        let mut itemId: u16 = GetPrizeItemId();
        if GetScore(multiplayerId) != GetHighestScore() {
            return 3u8;
        }
        if !((CheckBagHasSpace(itemId, 1u16)) != 0) {
            return 2u8;
        }
        AddBagItem(itemId, 1u16);
        if !((CheckBagHasSpace(itemId, 1u16)) != 0) {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IncrementWithLimit(num: u32, max: u32) -> u32 {
    unsafe {
        let mut num = num;
        let mut max = max;
        if num < max {
            return (num).wrapping_add(1u32);
        } else {
            return max;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn Min(a: u32, b: u32) -> u32 {
    unsafe {
        let mut a = a;
        let mut b = b;
        if a < b {
            return a;
        } else {
            return b;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetPlayerIdByPos(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        return ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsDodrioInParty() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        5i32,
                    )) != 0)
                        && (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        ) == 85u32)
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDodrioBerryPickingRecords() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShowDodrioBerryPickingRecords), 0u8);
        Task_ShowDodrioBerryPickingRecords(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowDodrioBerryPickingRecords(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut window = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut width: i32 = 0i32;
        let mut widthCurr: i32 = 0i32;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                (&raw mut window)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplates_Records)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                width =
                    GetStringWidth(1u8, (&raw mut gText_BerryPickingRecords).cast::<u8>(), 0i16);
                {
                    i = 0i32;
                    'l2: loop {
                        if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                            break 'l2;
                        }
                        'l3: {
                            widthCurr = (GetStringWidth(
                                1u8,
                                ((((&raw const sRecordsTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                                0i16,
                            ))
                            .wrapping_add(50i32);
                            if widthCurr > width {
                                width = widthCurr;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                width = crate::c::div_i32((width).wrapping_add(7i32), 8i32);
                if (width & 1i32) != 0 {
                    width = (width).wrapping_add(1);
                }
                (((&raw mut window).cast::<u8>()).wrapping_add(1))
                    .write(((crate::c::div_i32((30i32).wrapping_sub(width), 2i32)) as u8));
                (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(((width) as u8));
                ((data).wrapping_offset(1))
                    .write(((AddWindow((&raw mut window).cast::<u8>())) as i16));
                PrintRecordsText(((((data).wrapping_offset(1)).read()) as u8), width);
                CopyWindowToVram(((((data).wrapping_offset(1)).read()) as u8), 3u8);
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    rbox_fill_rectangle(((((data).wrapping_offset(1)).read()) as u8));
                    CopyWindowToVram(((((data).wrapping_offset(1)).read()) as u8), 1u8);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    RemoveWindow(((((data).wrapping_offset(1)).read()) as u8));
                    DestroyTask(taskId);
                    ScriptContext_Enable();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintRecordsText(windowId: u8, width: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut width = width;
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut numWidth: i32 = 0i32;
        let mut recordNums = crate::ffi::Align4([0u8; 12]);
        ((&raw mut recordNums).cast::<i32>()).write(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32),
        );
        (((&raw mut recordNums).cast::<i32>()).wrapping_offset(1)).write(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
                .cast::<u32>())
            .read()) as i32),
        );
        (((&raw mut recordNums).cast::<i32>()).wrapping_offset(2)).write(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(524))
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32),
        );
        LoadUserWindowBorderGfx_(windowId, 541u16, 208u8);
        DrawTextBorderOuter(windowId, 541u16, 13u8);
        FillWindowPixelBuffer(windowId, 17u8);
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_BerryPickingRecords).cast::<u8>(),
            ((GetStringCenterAlignXOffset(
                1i32,
                (&raw mut gText_BerryPickingRecords).cast::<u8>(),
                (width).wrapping_mul(8i32),
            )) as u8),
            1u8,
            255u8,
            None,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((&raw mut recordNums).cast::<i32>()).wrapping_offset((i) as isize))
                            .read(),
                        0i32,
                        ((((&raw const sRecordNumMaxDigits).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                    numWidth = GetStringWidth(1u8, (&raw mut gStringVar1).cast::<u8>(), (-1i16));
                    AddTextPrinterParameterized(
                        windowId,
                        1u8,
                        ((((&raw const sRecordsTexts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        0u8,
                        (((((&raw const sRecordTextYCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .read(),
                        255u8,
                        None,
                    );
                    x = ((width).wrapping_mul(8i32)).wrapping_sub(numWidth);
                    AddTextPrinterParameterized(
                        windowId,
                        1u8,
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((x) as u8),
                        (((((&raw const sRecordNumYCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .read(),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(windowId);
    }
}
pub(crate) unsafe extern "C" fn Debug_UpdateNumPlayers() {
    unsafe {
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
            .write(GetLinkPlayerCount());
    }
}
pub(crate) unsafe extern "C" fn Debug_SetPlayerNamesAndResults() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut playerId: u8 = 0u8;
        {
            playerId = ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .read();
            'l1: loop {
                if !(((playerId) as u32) < crate::c::div_u32(20u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    StringCopy(
                        ((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize * 28))
                        .wrapping_add(8))
                        .cast::<u8>(),
                        ((((&raw const sDebug_PlayerNames)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((playerId) as i32) as isize))
                        .read(),
                    );
                }
                playerId = (playerId).wrapping_add(1);
            }
        }
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36)).write(5u8);
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        playerId = 0u8;
                        'l5: loop {
                            if !(((playerId) as i32)
                                < ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .read()) as i32))
                            {
                                break 'l5;
                            }
                            'l6: {
                                ((((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(74))
                                .cast::<u8>())
                                .wrapping_offset(((playerId) as i32) as isize * 12))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((((&raw const sDebug_BerryResults)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((playerId) as i32) as isize * 8))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                            }
                            playerId = (playerId).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SendPacket_ReadyToStart(ready: u32) {
    unsafe {
        let mut ready = ready;
        let mut packet = crate::ffi::Align4([0u8; 8]);
        ((&raw mut packet).cast::<u8>()).write(1u8);
        (((&raw mut packet).cast::<u8>()).wrapping_add(4)).write(((ready) as u8));
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_ReadyToStart(playerId: u32) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut packet: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32) & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        packet = (((((&raw mut gRecvCmds).cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 16))
        .cast::<u16>())
        .wrapping_offset(1))
        .cast::<u8>();
        if (((packet).read()) as i32) == 1i32 {
            return ((((packet).wrapping_add(4)).read()) as u32);
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SendPacket_GameState(
    player: *mut u8,
    player1: *mut u8,
    player2: *mut u8,
    player3: *mut u8,
    player4: *mut u8,
    player5: *mut u8,
    numGraySquares: u8,
    berriesFalling: u32,
    allReadyToEnd: u32,
) {
    unsafe {
        let mut player = player;
        let mut player1 = player1;
        let mut player2 = player2;
        let mut player3 = player3;
        let mut player4 = player4;
        let mut player5 = player5;
        let mut numGraySquares = numGraySquares;
        let mut berriesFalling = berriesFalling;
        let mut allReadyToEnd = allReadyToEnd;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        let mut berries: *mut u8 = (player).wrapping_add(20);
        ((&raw mut packet).cast::<u8>()).write(2u8);
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(1),
            0,
            4,
            ((((berries).wrapping_add(11)).cast::<u8>()).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(1),
            4,
            4,
            (((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(1)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(2),
            0,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(2)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(2),
            4,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(3)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(3),
            0,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(4)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(3),
            4,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(5)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(4),
            0,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(6)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(4),
            4,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(7)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(5),
            0,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(8)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(5),
            4,
            4,
            ((((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(9)).read()) as u16)
                as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(6),
            0,
            2,
            ((((berries).cast::<u8>()).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(6),
            2,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(1)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(6),
            4,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(2)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(6),
            6,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(3)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(7),
            0,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(4)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(7),
            2,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(5)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(7),
            4,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(6)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(7),
            6,
            2,
            (((((berries).cast::<u8>()).wrapping_offset(7)).read()) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(8),
            0,
            2,
            ((((berries).cast::<u8>()).wrapping_offset(8)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(8),
            2,
            2,
            ((((berries).cast::<u8>()).wrapping_offset(9)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(8),
            4,
            2,
            ((player1).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(8),
            6,
            2,
            ((player2).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(9),
            0,
            2,
            ((player3).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(9),
            2,
            2,
            ((player4).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(9),
            4,
            2,
            ((player5).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(9),
            6,
            1,
            (((player1).wrapping_add(4)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(9),
            7,
            1,
            (((player2).wrapping_add(4)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(10),
            0,
            1,
            (((player3).wrapping_add(4)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(10),
            1,
            1,
            (((player4).wrapping_add(4)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(10),
            2,
            1,
            (((player5).wrapping_add(4)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            2,
            1,
            (((player1).wrapping_add(8)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            3,
            1,
            (((player2).wrapping_add(8)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            4,
            1,
            (((player3).wrapping_add(8)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            5,
            1,
            (((player4).wrapping_add(8)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            6,
            1,
            (((player5).wrapping_add(8)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(10),
            3,
            5,
            (numGraySquares) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            1,
            1,
            ((berriesFalling) as u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(11),
            0,
            1,
            ((allReadyToEnd) as u8) as i32,
        );
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_GameState(
    playerId: u32,
    player: *mut u8,
    player1: *mut u8,
    player2: *mut u8,
    player3: *mut u8,
    player4: *mut u8,
    player5: *mut u8,
    numGraySquares: *mut u8,
    berriesFalling: *mut u32,
    allReadyToEnd: *mut u32,
) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut player = player;
        let mut player1 = player1;
        let mut player2 = player2;
        let mut player3 = player3;
        let mut player4 = player4;
        let mut player5 = player5;
        let mut numGraySquares = numGraySquares;
        let mut berriesFalling = berriesFalling;
        let mut allReadyToEnd = allReadyToEnd;
        let mut packet: *mut u8 = core::ptr::null_mut();
        let mut berries: *mut u8 = (player).wrapping_add(20);
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32) & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        packet =
            ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1)).cast::<u8>();
        if (((packet).read()) as i32) == 2i32 {
            (((berries).wrapping_add(11)).cast::<u8>())
                .write((crate::c::bf_read((packet).wrapping_add(1), 0, 4, false) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(1))
                .write((crate::c::bf_read((packet).wrapping_add(1), 4, 4, false) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(2))
                .write(((crate::c::bf_read((packet).wrapping_add(2), 0, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(3))
                .write(((crate::c::bf_read((packet).wrapping_add(2), 4, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(4))
                .write(((crate::c::bf_read((packet).wrapping_add(3), 0, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(5))
                .write(((crate::c::bf_read((packet).wrapping_add(3), 4, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(6))
                .write(((crate::c::bf_read((packet).wrapping_add(4), 0, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(7))
                .write(((crate::c::bf_read((packet).wrapping_add(4), 4, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(8))
                .write(((crate::c::bf_read((packet).wrapping_add(5), 0, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(9))
                .write(((crate::c::bf_read((packet).wrapping_add(5), 4, 4, false) as u16) as u8));
            ((((berries).wrapping_add(11)).cast::<u8>()).wrapping_offset(10))
                .write((crate::c::bf_read((packet).wrapping_add(1), 0, 4, false) as u8));
            ((berries).cast::<u8>())
                .write(((crate::c::bf_read((packet).wrapping_add(6), 0, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(1))
                .write(((crate::c::bf_read((packet).wrapping_add(6), 2, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(2))
                .write(((crate::c::bf_read((packet).wrapping_add(6), 4, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(3))
                .write(((crate::c::bf_read((packet).wrapping_add(6), 6, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(4))
                .write(((crate::c::bf_read((packet).wrapping_add(7), 0, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(5))
                .write(((crate::c::bf_read((packet).wrapping_add(7), 2, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(6))
                .write(((crate::c::bf_read((packet).wrapping_add(7), 4, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(7))
                .write(((crate::c::bf_read((packet).wrapping_add(7), 6, 2, false) as u16) as u8));
            (((berries).cast::<u8>()).wrapping_offset(8))
                .write((crate::c::bf_read((packet).wrapping_add(8), 0, 2, false) as u8));
            (((berries).cast::<u8>()).wrapping_offset(9))
                .write((crate::c::bf_read((packet).wrapping_add(8), 2, 2, false) as u8));
            (((berries).cast::<u8>()).wrapping_offset(10))
                .write(((crate::c::bf_read((packet).wrapping_add(6), 0, 2, false) as u16) as u8));
            (player1).write((crate::c::bf_read((packet).wrapping_add(8), 4, 2, false) as u8));
            ((player1).wrapping_add(4))
                .write((crate::c::bf_read((packet).wrapping_add(9), 6, 1, false) as u8));
            ((player1).wrapping_add(8))
                .write((crate::c::bf_read((packet).wrapping_add(11), 2, 1, false) as u8));
            (player2).write((crate::c::bf_read((packet).wrapping_add(8), 6, 2, false) as u8));
            ((player2).wrapping_add(4))
                .write((crate::c::bf_read((packet).wrapping_add(9), 7, 1, false) as u8));
            ((player2).wrapping_add(8))
                .write((crate::c::bf_read((packet).wrapping_add(11), 3, 1, false) as u8));
            (player3).write((crate::c::bf_read((packet).wrapping_add(9), 0, 2, false) as u8));
            ((player3).wrapping_add(4))
                .write((crate::c::bf_read((packet).wrapping_add(10), 0, 1, false) as u8));
            ((player3).wrapping_add(8))
                .write((crate::c::bf_read((packet).wrapping_add(11), 4, 1, false) as u8));
            (player4).write((crate::c::bf_read((packet).wrapping_add(9), 2, 2, false) as u8));
            ((player4).wrapping_add(4))
                .write((crate::c::bf_read((packet).wrapping_add(10), 1, 1, false) as u8));
            ((player4).wrapping_add(8))
                .write((crate::c::bf_read((packet).wrapping_add(11), 5, 1, false) as u8));
            (player5).write((crate::c::bf_read((packet).wrapping_add(9), 4, 2, false) as u8));
            ((player5).wrapping_add(4))
                .write((crate::c::bf_read((packet).wrapping_add(10), 2, 1, false) as u8));
            ((player5).wrapping_add(8))
                .write((crate::c::bf_read((packet).wrapping_add(11), 6, 1, false) as u8));
            (numGraySquares)
                .write((crate::c::bf_read((packet).wrapping_add(10), 3, 5, false) as u8));
            (berriesFalling)
                .write(((crate::c::bf_read((packet).wrapping_add(11), 1, 1, false) as u8) as u32));
            (allReadyToEnd)
                .write(((crate::c::bf_read((packet).wrapping_add(11), 0, 1, false) as u8) as u32));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SendPacket_PickState(pickState: u8) {
    unsafe {
        let mut pickState = pickState;
        let mut packet = crate::ffi::Align4([0u8; 8]);
        ((&raw mut packet).cast::<u8>()).write(3u8);
        (((&raw mut packet).cast::<u8>()).wrapping_add(4)).write(pickState);
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_PickState(playerId: u32, pickState: *mut u8) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut pickState = pickState;
        let mut packet: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32) & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        packet = (((((&raw mut gRecvCmds).cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 16))
        .cast::<u16>())
        .wrapping_offset(1))
        .cast::<u8>();
        if (((packet).read()) as i32) == 3i32 {
            (pickState).write(((packet).wrapping_add(4)).read());
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SendPacket_ReadyToEnd(ready: u32) {
    unsafe {
        let mut ready = ready;
        let mut packet = crate::ffi::Align4([0u8; 8]);
        ((&raw mut packet).cast::<u8>()).write(4u8);
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(ready);
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_ReadyToEnd(playerId: u32) -> u32 {
    unsafe {
        let mut playerId = playerId;
        let mut packet: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32) & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        packet = (((((&raw mut gRecvCmds).cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 16))
        .cast::<u16>())
        .wrapping_offset(1))
        .cast::<u8>();
        if (((packet).read()) as i32) == 4i32 {
            return ((packet).wrapping_add(4).cast::<u32>()).read();
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn LoadDodrioGfx() {
    unsafe {
        let mut ptr: *mut u8 = AllocZeroed(12288u32);
        let mut normal = crate::ffi::Align4([0u8; 8]);
        (&raw mut normal)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sDodrioNormal_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut normal)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(0u16);
        let mut shiny = crate::ffi::Align4([0u8; 8]);
        (&raw mut shiny)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sDodrioShiny_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut shiny)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(1u16);
        LZ77UnCompWram(
            ((&raw const sDodrio_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ptr,
        );
        if !(ptr).is_null() {
            let mut sheet = crate::ffi::Align4([0u8; 8]);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<*mut u8>()
                .write(ptr);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<u16>()
                .write(12288u16);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(6)
                .cast::<u16>()
                .write(0u16);
            LoadSpriteSheet((&raw mut sheet).cast::<u8>());
            Free(ptr);
        }
        LoadSpritePalette((&raw mut normal).cast::<u8>());
        LoadSpritePalette((&raw mut shiny).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreateDodrioSprite(
    monInfo: *mut u8,
    playerId: u8,
    id: u8,
    numPlayers: u8,
) {
    unsafe {
        let mut monInfo = monInfo;
        let mut playerId = playerId;
        let mut id = id;
        let mut numPlayers = numPlayers;
        let mut template = crate::ffi::Align4([0u8; 24]);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write((((monInfo).read()) as u16));
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sOamData_Dodrio).cast::<u8>().cast_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnims_Dodrio)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCB_Dodrio));
        ((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>()).cast::<*mut u16>())
            .wrapping_offset(((id) as i32) as isize))
        .write((AllocZeroed(4u32)).cast::<u16>());
        (((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>()).cast::<*mut u16>())
            .wrapping_offset(((id) as i32) as isize))
        .read())
        .write(
            ((CreateSprite(
                (&raw mut template).cast::<u8>(),
                GetDodrioXPos(playerId, numPlayers),
                136i16,
                3u8,
            )) as u16),
        );
        SetDodrioInvisibility(1u8, id);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Dodrio(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                DoDodrioMissedAnim(sprite);
                break 'l1;
            }
            if __sw1 == 2i32 {
                DoDodrioIntroAnim(sprite);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartDodrioMissedAnim(unused: u8) {
    unsafe {
        let mut unused = unused;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>())
                .cast::<*mut u16>())
            .wrapping_offset(((GetMultiplayerId()) as i32) as isize))
            .read())
            .read()) as i32) as isize
                * 68,
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn StartDodrioIntroAnim(unused: u8) {
    unsafe {
        let mut unused = unused;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>())
                .cast::<*mut u16>())
            .wrapping_offset(((GetMultiplayerId()) as i32) as isize))
            .read())
            .read()) as i32) as isize
                * 68,
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn DoDodrioMissedAnim(sprite: *mut u8) -> u32 {
    unsafe {
        let mut sprite = sprite;
        let mut x: i8 = 0i8;
        let mut state: u8 = ((crate::c::rem_i32(
            crate::c::div_i32(
                (({
                    let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t2 = ((__p1).read()).wrapping_add(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32),
                2i32,
            ),
            4i32,
        )) as u8);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= 3i32
        {
            'l1: {
                let __sw3 = ((state) as i32);
                let __matched = __sw3 == 1i32 || __sw3 == 2i32;
                if !__matched {
                    x = 1i8;
                    break 'l1;
                }
                if __sw3 == 1i32 || __sw3 == 2i32 {
                    x = (-1i8);
                    break 'l1;
                }
            }
            let __p4 = (sprite).wrapping_add(32).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(((x) as i32))) as i16));
            if (({
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as i32)
                >= 40i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(32).cast::<i16>())
                    .write(GetDodrioXPos(0u8, GetNumPlayers()));
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn DoDodrioIntroAnim(sprite: *mut u8) -> u32 {
    unsafe {
        let mut sprite = sprite;
        let mut pickState: u8 = ((crate::c::rem_i32(
            crate::c::div_i32(
                (({
                    let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t2 = ((__p1).read()).wrapping_add(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32),
                13i32,
            ),
            4i32,
        )) as u8);
        if (crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            13i32,
        ) == 0i32)
            && (((pickState) as i32) != 0i32)
        {
            PlaySE(212u16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= 104i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            pickState = 0u8;
        }
        SetDodrioAnim(GetMultiplayerId(), pickState);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn FreeDodrioSprites(numPlayers: u8) {
    unsafe {
        let mut numPlayers = numPlayers;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .read()) as i32) as isize
                            * 68,
                    );
                    if !(sprite).is_null() {
                        DestroySpriteAndFreeResources(sprite);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDodrioInvisibility(invisible: u8, id: u8) {
    unsafe {
        let mut invisible = invisible;
        let mut id = id;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>())
                    .cast::<*mut u16>())
                .wrapping_offset(((id) as i32) as isize))
                .read())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SetAllDodrioInvisibility(invisible: u8, count: u8) {
    unsafe {
        let mut invisible = invisible;
        let mut count = count;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    SetDodrioInvisibility(invisible, i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDodrioAnim(id: u8, pickState: u8) {
    unsafe {
        let mut id = id;
        let mut pickState = pickState;
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sDodrioSpriteIds).cast::<u8>().cast::<*mut u16>())
                    .cast::<*mut u16>())
                .wrapping_offset(((id) as i32) as isize))
                .read())
                .read()) as i32) as isize
                    * 68,
            ),
            pickState,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Status(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn InitStatusBarPos() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(42))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    );
                    ((sprite).wrapping_add(32).cast::<i16>())
                        .write((((((i) as i32).wrapping_mul(16i32)).wrapping_add(48i32)) as i16));
                    ((sprite).wrapping_add(34).cast::<i16>())
                        .write((((-8i32).wrapping_sub(((i) as i32).wrapping_mul(8i32))) as i16));
                    ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateStatusBarSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut ptr: *mut u8 = AllocZeroed(384u32);
        let mut pal = crate::ffi::Align4([0u8; 8]);
        (&raw mut pal)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sStatus_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut pal)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(2u16);
        LZ77UnCompWram(
            ((&raw const sStatus_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ptr,
        );
        if !(ptr).is_null() {
            let mut sheet = crate::ffi::Align4([0u8; 8]);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<*mut u8>()
                .write(ptr);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<u16>()
                .write(384u16);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(6)
                .cast::<u16>()
                .write(1u16);
            let mut template = crate::ffi::Align4([0u8; 24]);
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<u16>()
                .write(1u16);
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(2)
                .cast::<u16>()
                .write(2u16);
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<*mut u8>()
                .write(
                    (&raw const sOamData_16x16_Priority0)
                        .cast::<u8>()
                        .cast_mut(),
                );
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(8)
                .cast::<*mut *mut u8>()
                .write(
                    ((&raw const sAnims_StatusBar)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>(),
                );
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(12)
                .cast::<*mut u8>()
                .write(core::ptr::null_mut());
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(16)
                .cast::<*mut *mut u8>()
                .write(
                    ((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>(),
                );
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
                .write(Some(SpriteCB_Status));
            ((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(64u32));
            LoadSpriteSheet((&raw mut sheet).cast::<u8>());
            LoadSpritePalette((&raw mut pal).cast::<u8>());
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(42))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((CreateSprite(
                                (&raw mut template).cast::<u8>(),
                                (((((i) as i32).wrapping_mul(16i32)).wrapping_add(48i32)) as i16),
                                (((-8i32).wrapping_sub(((i) as i32).wrapping_mul(8i32))) as i16),
                                0u8,
                            )) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        Free(ptr);
    }
}
pub(crate) unsafe extern "C" fn FreeStatusBar() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(42))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    );
                    if !(sprite).is_null() {
                        DestroySpriteAndFreeResources(sprite);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            Free(((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn DoStatusBarIntro() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut animActive: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(42))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    );
                    ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .cast::<i16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(2i16);
                    if ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                        != 0)
                        && (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) == 8i32)
                    {
                        break 'l2;
                    }
                    animActive = 1u32;
                    if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) == 8i32 {
                        if (((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0
                        {
                            break 'l2;
                        }
                        ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(1u8);
                        ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22))
                        .cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write((-16i16));
                        PlaySE(36u16);
                    }
                    let __p1 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_add(
                            ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(22))
                            .cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if (animActive) != 0 {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateStatusBarAnim(numEmpty: u8) {
    unsafe {
        let mut numEmpty = numEmpty;
        let mut i: u8 = 0u8;
        if ((numEmpty) as i32) > 10i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(42))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                            1u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < (10i32).wrapping_sub(((numEmpty) as i32))) {
                        break 'l3;
                    }
                    'l4: {
                        if ((numEmpty) as i32) > 6i32 {
                            let __p1 = (((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(62)
                            .cast::<u16>();
                            (__p1).write(
                                (((((__p1).read()) as i32)
                                    .wrapping_add(((numEmpty) as i32).wrapping_sub(6i32)))
                                    as u16),
                            );
                            if ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(62)
                                .cast::<u16>())
                            .read()) as i32)
                                > 30i32
                            {
                                ((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(62)
                                    .cast::<u16>())
                                .write(0u16);
                            } else {
                                if ((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(62)
                                .cast::<u16>())
                                .read()) as i32)
                                    > 10i32
                                {
                                    StartSpriteAnim(
                                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sStatusBar)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(42))
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ),
                                        2u8,
                                    );
                                } else {
                                    StartSpriteAnim(
                                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sStatusBar)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(42))
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ),
                                        0u8,
                                    );
                                }
                            }
                        } else {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sStatusBar)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(42))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                0u8,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                'l5: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l5;
                    }
                    'l6: {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(42))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                            1u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetStatusBarInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sStatusBar).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(42))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        ((invisible) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadBerryGfx() {
    unsafe {
        let mut ptr: *mut u8 = AllocZeroed(1152u32);
        let mut pal = crate::ffi::Align4([0u8; 8]);
        (&raw mut pal)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sBerries_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut pal)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(3u16);
        LZ77UnCompWram(
            ((&raw const sBerries_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ptr,
        );
        if !(ptr).is_null() {
            let mut sheet = crate::ffi::Align4([0u8; 8]);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<*mut u8>()
                .write(ptr);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<u16>()
                .write(1152u16);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(6)
                .cast::<u16>()
                .write(2u16);
            LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        }
        LoadSpritePalette((&raw mut pal).cast::<u8>());
        Free(ptr);
    }
}
pub(crate) unsafe extern "C" fn CreateBerrySprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut berry = crate::ffi::Align4([0u8; 24]);
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(2u16);
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(3u16);
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sOamData_Berry).cast::<u8>().cast_mut());
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnims_Berry)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut berry)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        let mut berryIcon = crate::ffi::Align4([0u8; 24]);
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(2u16);
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(3u16);
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write(
                (&raw const sOamData_16x16_Priority0)
                    .cast::<u8>()
                    .cast_mut(),
            );
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnims_Berry)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut berryIcon)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 11i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                        .cast::<*mut u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write((AllocZeroed(4u32)).cast::<u16>());
                    x = ((((i) as i32).wrapping_mul(16i32)) as i16);
                    (((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                        .cast::<*mut u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .write(
                        ((CreateSprite(
                            (&raw mut berry).cast::<u8>(),
                            ((((x) as i32).wrapping_add(((i) as i32).wrapping_mul(8i32))) as i16),
                            8i16,
                            1u8,
                        )) as u16),
                    );
                    SetBerryInvisibility(i, 1u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut sBerryIconSpriteIds)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write((AllocZeroed(4u32)).cast::<u16>());
                    if ((i) as i32) == 3i32 {
                        (((((&raw mut sBerryIconSpriteIds)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .write(
                            ((CreateSprite(
                                (&raw mut berryIcon).cast::<u8>(),
                                ((((&raw const sBerryIconXCoords)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                49i16,
                                0u8,
                            )) as u16),
                        );
                    } else {
                        (((((&raw mut sBerryIconSpriteIds)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .write(
                            ((CreateSprite(
                                (&raw mut berryIcon).cast::<u8>(),
                                ((((&raw const sBerryIconXCoords)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                52i16,
                                0u8,
                            )) as u16),
                        );
                    }
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sBerryIconSpriteIds)
                                .cast::<u8>()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        i,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetBerryIconsInvisibility(1u8);
    }
}
pub(crate) unsafe extern "C" fn FreeBerrySprites() {
    unsafe {
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 11i32) {
                    break 'l1;
                }
                'l2: {
                    sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .read()) as i32) as isize
                            * 68,
                    );
                    if !(sprite).is_null() {
                        DestroySprite(sprite);
                    }
                    {
                        Free(
                            (((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                                .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .cast::<u8>(),
                        );
                        ((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sBerryIconSpriteIds)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .read()) as i32) as isize
                            * 68,
                    );
                    if !(sprite).is_null() {
                        DestroySprite(sprite);
                    }
                    {
                        Free(
                            (((((&raw mut sBerryIconSpriteIds)
                                .cast::<u8>()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .cast::<u8>(),
                        );
                        ((((&raw mut sBerryIconSpriteIds)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBerryInvisibility(id: u8, invisible: u8) {
    unsafe {
        let mut id = id;
        let mut invisible = invisible;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                    .cast::<*mut u16>())
                .wrapping_offset(((id) as i32) as isize))
                .read())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SetBerryIconsInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sBerryIconSpriteIds)
                                .cast::<u8>()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        ((invisible) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBerryYPos(id: u8, y: u8) {
    unsafe {
        let mut id = id;
        let mut y = y;
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>()).cast::<*mut u16>())
                .wrapping_offset(((id) as i32) as isize))
            .read())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(((((y) as i32).wrapping_mul(8i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn SetBerryAnim(id: u16, animNum: u8) {
    unsafe {
        let mut id = id;
        let mut animNum = animNum;
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sBerrySpriteIds).cast::<u8>().cast::<*mut u16>())
                    .cast::<*mut u16>())
                .wrapping_offset(((id) as i32) as isize))
                .read())
                .read()) as i32) as isize
                    * 68,
            ),
            animNum,
        );
    }
}
pub(crate) unsafe extern "C" fn UnusedSetSpritePos(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
        .write(((((20i32).wrapping_mul(((spriteId) as i32))).wrapping_add(50i32)) as i16));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
        .write(50i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cloud(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 1i32
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 2i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (({
                            let __p1 =
                                (((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                                    .cast::<*mut u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_offset(1);
                            let __t2 = ((__p1).read()).wrapping_add(1);
                            (__p1).write(__t2);
                            __t2
                        }) as i32)
                            > ((((((&raw const moveDelays_0).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                        {
                            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                            (__p3).write(((__p3).read()).wrapping_sub(1));
                            ((((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                                .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_offset(1))
                            .write(0u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCloudSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut ptr: *mut u8 = AllocZeroed(1024u32);
        let mut pal = crate::ffi::Align4([0u8; 8]);
        (&raw mut pal)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sCloud_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut pal)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(6u16);
        LZ77UnCompWram(
            ((&raw const sCloud_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ptr,
        );
        if !(ptr).is_null() {
            let mut sheet = crate::ffi::Align4([0u8; 8]);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<*mut u8>()
                .write(ptr);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<u16>()
                .write(1024u16);
            (&raw mut sheet)
                .cast::<u8>()
                .wrapping_add(6)
                .cast::<u16>()
                .write(5u16);
            let mut template = crate::ffi::Align4([0u8; 24]);
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<u16>()
                .write(5u16);
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(2)
                .cast::<u16>()
                .write(6u16);
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(4)
                .cast::<*mut u8>()
                .write((&raw const sOamData_Cloud).cast::<u8>().cast_mut());
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(8)
                .cast::<*mut *mut u8>()
                .write(
                    ((&raw const sAnims_Cloud)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>(),
                );
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(12)
                .cast::<*mut u8>()
                .write(core::ptr::null_mut());
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(16)
                .cast::<*mut *mut u8>()
                .write(
                    ((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>(),
                );
            (&raw mut template)
                .cast::<u8>()
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
                .write(Some(SpriteCB_Cloud));
            LoadSpriteSheet((&raw mut sheet).cast::<u8>());
            LoadSpritePalette((&raw mut pal).cast::<u8>());
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 2i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write((AllocZeroed(4u32)).cast::<u16>());
                        (((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .write(
                            ((CreateSprite(
                                (&raw mut template).cast::<u8>(),
                                (((((&raw const sCloudStartCoords).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<i16>())
                                .read(),
                                ((((((&raw const sCloudStartCoords).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read(),
                                4u8,
                            )) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        Free(ptr);
    }
}
pub(crate) unsafe extern "C" fn ResetCloudPos() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .read()) as i32) as isize
                            * 68,
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        (((((&raw const sCloudStartCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<i16>())
                        .read(),
                    );
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((((((&raw const sCloudStartCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartCloudMovement() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .read()) as i32) as isize
                            * 68,
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeCloudSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .read()) as i32) as isize
                            * 68,
                    );
                    if !(sprite).is_null() {
                        DestroySprite(sprite);
                    }
                    {
                        Free(
                            (((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                                .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .cast::<u8>(),
                        );
                        ((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                            .cast::<*mut u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCloudInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sCloudSpriteIds).cast::<u8>().cast::<*mut u16>())
                                .cast::<*mut u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        ((invisible) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetDodrioXPos(playerId: u8, numPlayers: u8) -> i16 {
    unsafe {
        let mut playerId = playerId;
        let mut numPlayers = numPlayers;
        let mut x: i16 = 0i16;
        'l1: {
            let __sw1 = ((numPlayers) as i32);
            if __sw1 == 1i32 {
                x = 15i16;
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l2: {
                    let __sw2 = ((playerId) as i32);
                    if __sw2 == 0i32 {
                        x = 12i16;
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        x = 18i16;
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                'l3: {
                    let __sw3 = ((playerId) as i32);
                    if __sw3 == 0i32 {
                        x = 15i16;
                        break 'l3;
                    }
                    if __sw3 == 1i32 {
                        x = 21i16;
                        break 'l3;
                    }
                    if __sw3 == 2i32 {
                        x = 9i16;
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                'l4: {
                    let __sw4 = ((playerId) as i32);
                    if __sw4 == 0i32 {
                        x = 12i16;
                        break 'l4;
                    }
                    if __sw4 == 1i32 {
                        x = 18i16;
                        break 'l4;
                    }
                    if __sw4 == 2i32 {
                        x = 24i16;
                        break 'l4;
                    }
                    if __sw4 == 3i32 {
                        x = 6i16;
                        break 'l4;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                'l5: {
                    let __sw5 = ((playerId) as i32);
                    if __sw5 == 0i32 {
                        x = 15i16;
                        break 'l5;
                    }
                    if __sw5 == 1i32 {
                        x = 21i16;
                        break 'l5;
                    }
                    if __sw5 == 2i32 {
                        x = 27i16;
                        break 'l5;
                    }
                    if __sw5 == 3i32 {
                        x = 3i16;
                        break 'l5;
                    }
                    if __sw5 == 4i32 {
                        x = 9i16;
                        break 'l5;
                    }
                }
                break 'l1;
            }
        }
        return ((((x) as i32).wrapping_mul(8i32)) as i16);
    }
}
pub(crate) unsafe extern "C" fn ResetBerryAndStatusBarSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 11i32) {
                    break 'l1;
                }
                'l2: {
                    SetBerryInvisibility(i, 1u8);
                    SetBerryYPos(i, 1u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetStatusBarInvisibility(0u8);
    }
}
pub(crate) unsafe extern "C" fn LoadWindowFrameGfx(frameId: u8) {
    unsafe {
        let mut frameId = frameId;
        LoadBgTiles(
            0u8,
            ((GetWindowFrameTilesPal(frameId)).cast::<*mut u8>()).read(),
            288u16,
            1u16,
        );
        LoadPalette(
            (((GetWindowFrameTilesPal(frameId))
                .wrapping_add(4)
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
            160u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadUserWindowFrameGfx() {
    unsafe {
        LoadUserWindowBorderGfx_(0u8, 10u16, 176u8);
    }
}
pub(crate) unsafe extern "C" fn ResetGfxState() {
    unsafe {
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .write(0u32);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12312)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12320)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DrawYesNoMessageWindow(template: *mut u8) {
    unsafe {
        let mut template = template;
        let mut pal: u8 = 10u8;
        FillBgTilemapBufferRect(
            0u8,
            1u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            2u16,
            ((template).wrapping_add(1)).read(),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((template).wrapping_add(3)).read(),
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            3u16,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            4u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((template).wrapping_add(2)).read(),
            1u8,
            ((template).wrapping_add(4)).read(),
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            6u16,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((template).wrapping_add(2)).read(),
            1u8,
            ((template).wrapping_add(4)).read(),
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            7u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            1u8,
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            8u16,
            ((template).wrapping_add(1)).read(),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            ((template).wrapping_add(3)).read(),
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            9u16,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            1u8,
            1u8,
            pal,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawMessageWindow(template: *mut u8) {
    unsafe {
        let mut template = template;
        let mut pal: u8 = 11u8;
        FillBgTilemapBufferRect(
            0u8,
            10u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            11u16,
            ((template).wrapping_add(1)).read(),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((template).wrapping_add(3)).read(),
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            12u16,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            13u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((template).wrapping_add(2)).read(),
            1u8,
            ((template).wrapping_add(4)).read(),
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            15u16,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((template).wrapping_add(2)).read(),
            1u8,
            ((template).wrapping_add(4)).read(),
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            16u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            1u8,
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            17u16,
            ((template).wrapping_add(1)).read(),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            ((template).wrapping_add(3)).read(),
            1u8,
            pal,
        );
        FillBgTilemapBufferRect(
            0u8,
            18u16,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            1u8,
            1u8,
            pal,
        );
    }
}
pub(crate) unsafe extern "C" fn InitGameGfx(ptr: *mut u8) {
    unsafe {
        let mut ptr = ptr;
        ((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).write(ptr);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .write(0u32);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12312)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12320)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12292))
            .write(CreateTask(Some(Task_TryRunGfxFunc), 3u8));
        SetGfxFunc(Some(LoadGfx));
    }
}
pub(crate) unsafe extern "C" fn FreeAllWindowBuffers_() {
    unsafe {
        FreeAllWindowBuffers();
    }
}
pub(crate) unsafe extern "C" fn SetGfxFuncById(funcId: u8) {
    unsafe {
        let mut funcId = funcId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(80u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sGfxFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .read()) as i32)
                        == ((funcId) as i32)
                    {
                        SetGfxFunc(
                            (((((&raw const sGfxFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryRunGfxFunc(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .read())
            != 0)
        {
            (GetGfxFunc()).unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe extern "C" fn LoadGfx() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                InitBgs();
                let __p2 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if LoadBgGfx() == 1u32 {
                    let __p3 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw const sBg_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyToBgTilemapBuffer(
                    1u8,
                    (((&raw const sTreeBorderLeft_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyToBgTilemapBuffer(
                    2u8,
                    (((&raw const sTreeBorderRight_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(3u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                let __p4 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ShowBg(0u8);
                ShowBg(3u8);
                ShowBg(1u8);
                ShowBg(2u8);
                let __p5 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadWindowFrameGfx(
                    ((crate::c::bf_read(
                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                        3,
                        5,
                        false,
                    ) as u16) as u8),
                );
                LoadUserWindowFrameGfx();
                let __p6 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12288)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowNames() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numPlayers: u8 = 0u8;
        let mut playerId: u8 = 0u8;
        let mut colorsId: u8 = 0u8;
        let mut name: *mut u8 = core::ptr::null_mut();
        let mut left: u32 = 0u32;
        let mut window = crate::ffi::Align4([0u8; 8]);
        let mut coords: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                numPlayers = GetNumPlayers();
                coords = ((((&raw const sNameWindowCoords)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset((((numPlayers) as i32).wrapping_sub(1i32)) as isize))
                .read();
                ((&raw mut window).cast::<u8>()).write(0u8);
                (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(7u8);
                (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(2u8);
                (((&raw mut window).cast::<u8>()).wrapping_add(5)).write(13u8);
                (((&raw mut window).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(19u16);
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((numPlayers) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            colorsId = 0u8;
                            playerId = GetPlayerIdByPos(i);
                            left = crate::c::div_u32(
                                (((56i32).wrapping_sub(GetStringWidth(
                                    1u8,
                                    GetPlayerName(playerId),
                                    (-1i16),
                                ))) as u32),
                                2u32,
                            );
                            (((&raw mut window).cast::<u8>()).wrapping_add(1))
                                .write((coords).read());
                            (((&raw mut window).cast::<u8>()).wrapping_add(2))
                                .write(((coords).wrapping_add(1)).read());
                            ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12296))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(((AddWindow((&raw mut window).cast::<u8>())) as u8));
                            ClearWindowTilemap(
                                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12296))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            FillWindowPixelBuffer(
                                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12296))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                17u8,
                            );
                            if ((playerId) as i32) == ((GetMultiplayerId()) as i32) {
                                colorsId = 2u8;
                            }
                            name = GetPlayerName(playerId);
                            AddTextPrinterParameterized3(
                                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12296))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                1u8,
                                ((left) as u8),
                                1u8,
                                ((((&raw const sTextColorTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((colorsId) as i32) as isize * 3))
                                .cast::<u8>(),
                                (-1i8),
                                name,
                            );
                            CopyWindowToVram(
                                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12296))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                2u8,
                            );
                            let __p2 = ((&raw mut window).cast::<u8>())
                                .wrapping_add(6)
                                .cast::<u16>();
                            (__p2).write((((((__p2).read()) as i32).wrapping_add(14i32)) as u16));
                            DrawMessageWindow((&raw mut window).cast::<u8>());
                        }
                        coords = (coords).wrapping_offset(4);
                        i = (i).wrapping_add(1);
                    }
                }
                let __p3 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    numPlayers = GetNumPlayers();
                    {
                        i = 0u8;
                        'l4: loop {
                            if !(((i) as i32) < ((numPlayers) as i32)) {
                                break 'l4;
                            }
                            'l5: {
                                PutWindowTilemap(
                                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(12296))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    CopyBgTilemapBufferToVram(0u8);
                    let __p4 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                if (({
                    let __p5 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 180i32
                {
                    numPlayers = GetNumPlayers();
                    {
                        i = 0u8;
                        'l6: loop {
                            if !(((i) as i32) < ((numPlayers) as i32)) {
                                break 'l6;
                            }
                            'l7: {
                                ClearWindowTilemap(
                                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(12296))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                                RemoveWindow(
                                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(12296))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    FillBgTilemapBufferRect_Palette0(
                        0u8,
                        0u16,
                        0u8,
                        0u8,
                        ((crate::c::div_i32(240i32, 8i32)) as u8),
                        ((crate::c::div_i32(160i32, 8i32)) as u8),
                    );
                    CopyBgTilemapBufferToVram(0u8);
                    ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12288)
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintRankedScores(numPlayers_: u8) {
    unsafe {
        let mut numPlayers_ = numPlayers_;
        let mut i: u8 = 0u8;
        let mut ranking: u8 = 0u8;
        let mut rankedPlayers: u8 = 0u8;
        let mut numPlayers: u8 = numPlayers_;
        let mut name: *mut u8 = core::ptr::null_mut();
        let mut x: u32 = 0u32;
        let mut numWidth: u32 = 0u32;
        let mut numString = crate::ffi::Align4([0u8; 32]);
        let mut playersByRanking = crate::ffi::Align4([0u8; 5]);
        (&raw mut playersByRanking)
            .cast::<u8>()
            .wrapping_add(0)
            .write(0u8);
        (&raw mut playersByRanking)
            .cast::<u8>()
            .wrapping_add(1)
            .write(1u8);
        (&raw mut playersByRanking)
            .cast::<u8>()
            .wrapping_add(2)
            .write(2u8);
        (&raw mut playersByRanking)
            .cast::<u8>()
            .wrapping_add(3)
            .write(3u8);
        (&raw mut playersByRanking)
            .cast::<u8>()
            .wrapping_add(4)
            .write(4u8);
        let mut temp = crate::ffi::Align4([0u8; 8]);
        let mut scoreResults = crate::ffi::Align4([0u8; 40]);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut playersByRanking).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(i);
                    GetScoreResults((&raw mut temp).cast::<u8>(), i);
                    ((&raw mut scoreResults).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8)
                        .cast::<crate::c::Rec4<8>>()
                        .write_unaligned(
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<8>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        if GetHighestScore() != 0u32 {
            'l3: loop {
                'l4: {
                    {
                        i = 0u8;
                        'l5: loop {
                            if !(((i) as i32) < ((numPlayers) as i32)) {
                                break 'l5;
                            }
                            'l6: {
                                if (((((&raw mut scoreResults).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 8))
                                .read()) as i32)
                                    == ((ranking) as i32)
                                {
                                    (((&raw mut playersByRanking).cast::<u8>())
                                        .wrapping_offset(((rankedPlayers) as i32) as isize))
                                    .write(i);
                                    rankedPlayers = (rankedPlayers).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ranking = rankedPlayers;
                }
                if !(((rankedPlayers) as i32) < ((numPlayers) as i32)) {
                    break 'l3;
                }
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l7;
                }
                'l8: {
                    if ((((&raw mut scoreResults).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read()
                        == 0u32
                    {
                        (((&raw mut scoreResults).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .write(((((numPlayers) as i32).wrapping_sub(1i32)) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        x = (((216i32).wrapping_sub(GetStringWidth(
            1u8,
            (&raw mut gText_SpacePoints).cast::<u8>(),
            0i16,
        ))) as u32);
        {
            i = 0u8;
            'l9: loop {
                if !(((i) as i32) < ((numPlayers) as i32)) {
                    break 'l9;
                }
                'l10: {
                    let mut colorsId: u8 = 0u8;
                    let mut playerId: u8 = (((&raw mut playersByRanking).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read();
                    let mut points: u32 = ((((&raw mut scoreResults).cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read();
                    AddTextPrinterParameterized(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        1u8,
                        ((((&raw const sRankingTexts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            (((((&raw mut scoreResults).cast::<u8>())
                                .wrapping_offset(((playerId) as i32) as isize * 8))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                        8u8,
                        ((((((&raw const sRankingYCoords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u8),
                        255u8,
                        None,
                    );
                    if ((playerId) as i32) == ((GetMultiplayerId()) as i32) {
                        colorsId = 2u8;
                    }
                    name = GetPlayerName(playerId);
                    AddTextPrinterParameterized3(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        1u8,
                        28u8,
                        ((((((&raw const sRankingYCoords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u8),
                        ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((colorsId) as i32) as isize * 3))
                        .cast::<u8>(),
                        (-1i8),
                        name,
                    );
                    ConvertIntToDecimalStringN(
                        (&raw mut numString).cast::<u8>(),
                        ((points) as i32),
                        0i32,
                        7u8,
                    );
                    numWidth =
                        ((GetStringWidth(1u8, (&raw mut numString).cast::<u8>(), (-1i16))) as u32);
                    AddTextPrinterParameterized(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        1u8,
                        (&raw mut numString).cast::<u8>(),
                        (((x).wrapping_sub(numWidth)) as u8),
                        ((((((&raw const sRankingYCoords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u8),
                        255u8,
                        None,
                    );
                    AddTextPrinterParameterized(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        1u8,
                        (&raw mut gText_SpacePoints).cast::<u8>(),
                        ((x) as u8),
                        ((((((&raw const sRankingYCoords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowResults() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut prizeState: u8 = 0u8;
        let mut numPlayers: u8 = GetNumPlayers();
        let mut name: *mut u8 = core::ptr::null_mut();
        let mut strWidth: u32 = 0u32;
        let mut x: u32 = 0u32;
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32;
            if __sw1 == 0i32 {
                SetScoreResults();
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12316)
                    .cast::<u16>())
                .write(0u16);
                let __p2 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                    .cast::<u8>())
                .write(
                    ((AddWindow(
                        ((&raw const sWindowTemplates_Results)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    )) as u8),
                );
                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                    .cast::<u8>())
                .wrapping_offset(1))
                .write(
                    ((AddWindow(
                        (((&raw const sWindowTemplates_Results)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(8),
                    )) as u8),
                );
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                ClearWindowTilemap(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                DrawMessageWindow(
                    ((&raw const sWindowTemplates_Results)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DrawMessageWindow(
                    (((&raw const sWindowTemplates_Results)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(8),
                );
                let __p3 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                FillWindowPixelBuffer(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    17u8,
                );
                FillWindowPixelBuffer(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    17u8,
                );
                strWidth = ((GetStringWidth(
                    1u8,
                    (&raw mut gText_BerryPickingResults).cast::<u8>(),
                    (-1i16),
                )) as u32);
                x = crate::c::div_u32((224u32).wrapping_sub(strWidth), 2u32);
                AddTextPrinterParameterized(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    1u8,
                    (&raw mut gText_BerryPickingResults).cast::<u8>(),
                    ((x) as u8),
                    1u8,
                    255u8,
                    None,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_10P30P50P50P).cast::<u8>(),
                    68u8,
                    17u8,
                    255u8,
                    None,
                );
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((numPlayers) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            let mut colorsId: u8 = 0u8;
                            if ((i) as i32) == ((GetMultiplayerId()) as i32) {
                                colorsId = 2u8;
                            }
                            name = GetPlayerName(i);
                            AddTextPrinterParameterized3(
                                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12296))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                                1u8,
                                0u8,
                                ((((((&raw const sResultsYCoords)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as u8),
                                ((((&raw const sTextColorTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((colorsId) as i32) as isize * 3))
                                .cast::<u8>(),
                                (-1i8),
                                name,
                            );
                            {
                                j = 0u8;
                                'l4: loop {
                                    if !(((j) as i32) < 4i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        let mut width: u32 = 0u32;
                                        let mut berriesPicked: u16 =
                                            ((Min(((GetBerryResult(i, j)) as u32), 9999u32))
                                                as u16);
                                        let mut maxBerriesPicked: u16 =
                                            ((Min(GetHighestBerryResult(j), 9999u32)) as u16);
                                        ConvertIntToDecimalStringN(
                                            (&raw mut gStringVar4).cast::<u8>(),
                                            ((berriesPicked) as i32),
                                            0i32,
                                            4u8,
                                        );
                                        width = ((GetStringWidth(
                                            1u8,
                                            (&raw mut gStringVar4).cast::<u8>(),
                                            (-1i16),
                                        )) as u32);
                                        if (((maxBerriesPicked) as i32) == ((berriesPicked) as i32))
                                            && (((maxBerriesPicked) as i32) != 0i32)
                                        {
                                            AddTextPrinterParameterized3(
                                                ((((((&raw mut sGfx)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(12296))
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                                1u8,
                                                ((((((((&raw const sResultsXCoords)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read())
                                                    as u32)
                                                    .wrapping_sub(width))
                                                    as u8),
                                                ((((((&raw const sResultsYCoords)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read())
                                                    as u8),
                                                ((((&raw const sTextColorTable)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(3))
                                                .cast::<u8>(),
                                                (-1i8),
                                                (&raw mut gStringVar4).cast::<u8>(),
                                            );
                                        } else {
                                            AddTextPrinterParameterized(
                                                ((((((&raw mut sGfx)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(12296))
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                                1u8,
                                                (&raw mut gStringVar4).cast::<u8>(),
                                                ((((((((&raw const sResultsXCoords)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read())
                                                    as u32)
                                                    .wrapping_sub(width))
                                                    as u8),
                                                ((((((&raw const sResultsYCoords)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read())
                                                    as u8),
                                                255u8,
                                                None,
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
                CopyWindowToVram(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    2u8,
                );
                CopyWindowToVram(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    2u8,
                );
                let __p4 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .read(),
                    );
                    PutWindowTilemap(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                }
                CopyBgTilemapBufferToVram(0u8);
                SetBerryIconsInvisibility(0u8);
                let __p5 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((({
                    let __p6 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>();
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    >= 30i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>())
                    .write(0u16);
                    PlaySE(5u16);
                    SetBerryIconsInvisibility(1u8);
                    let __p8 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                FillWindowPixelBuffer(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    17u8,
                );
                FillWindowPixelBuffer(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    17u8,
                );
                strWidth = ((GetStringWidth(
                    1u8,
                    (&raw mut gText_AnnouncingRankings).cast::<u8>(),
                    (-1i16),
                )) as u32);
                x = crate::c::div_u32((224u32).wrapping_sub(strWidth), 2u32);
                AddTextPrinterParameterized(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    1u8,
                    (&raw mut gText_AnnouncingRankings).cast::<u8>(),
                    ((x) as u8),
                    1u8,
                    255u8,
                    None,
                );
                let __p9 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                PrintRankedScores(numPlayers);
                CopyWindowToVram(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    2u8,
                );
                CopyWindowToVram(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    2u8,
                );
                let __p10 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .read(),
                    );
                    PutWindowTilemap(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                }
                CopyBgTilemapBufferToVram(0u8);
                let __p11 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((({
                    let __p12 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>();
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    >= 30i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>())
                    .write(0u16);
                    PlaySE(5u16);
                    if GetHighestScore() < 3000u32 {
                        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12308))
                        .write(127u8);
                    } else {
                        StopMapMusic();
                        let __p14 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12308);
                        (__p14).write(((__p14).read()).wrapping_add(1));
                    }
                    FillBgTilemapBufferRect_Palette0(
                        0u8,
                        0u16,
                        0u8,
                        5u8,
                        ((crate::c::div_i32(240i32, 8i32)) as u8),
                        (((crate::c::div_i32(160i32, 8i32)).wrapping_sub(5i32)) as u8),
                    );
                    RemoveWindow(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .write(
                        ((AddWindow((&raw const sWindowTemplate_Prize).cast::<u8>().cast_mut()))
                            as u8),
                    );
                    ClearWindowTilemap(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    DrawMessageWindow((&raw const sWindowTemplate_Prize).cast::<u8>().cast_mut());
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                PlayNewMapMusic(367u16);
                FillWindowPixelBuffer(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    17u8,
                );
                FillWindowPixelBuffer(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    17u8,
                );
                strWidth =
                    ((GetStringWidth(1u8, (&raw mut gText_AnnouncingPrizes).cast::<u8>(), (-1i16)))
                        as u32);
                x = crate::c::div_u32((224u32).wrapping_sub(strWidth), 2u32);
                AddTextPrinterParameterized(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    1u8,
                    (&raw mut gText_AnnouncingPrizes).cast::<u8>(),
                    ((x) as u8),
                    1u8,
                    255u8,
                    None,
                );
                DynamicPlaceholderTextUtil_Reset();
                CopyItemName(GetPrizeItemId(), (&raw mut gStringVar1).cast::<u8>());
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                    0u8,
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                DynamicPlaceholderTextUtil_ExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_FirstPlacePrize).cast::<u8>(),
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                prizeState = TryGivePrize();
                if (((prizeState) as i32) != 0i32) && (((prizeState) as i32) != 3i32) {
                    DynamicPlaceholderTextUtil_Reset();
                    CopyItemName(GetPrizeItemId(), (&raw mut gStringVar1).cast::<u8>());
                    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                        0u8,
                        (&raw mut gStringVar1).cast::<u8>(),
                    );
                    if ((prizeState) as i32) == 2i32 {
                        DynamicPlaceholderTextUtil_ExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_CantHoldAnyMore).cast::<u8>(),
                        );
                    } else {
                        if ((prizeState) as i32) == 1i32 {
                            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (&raw mut gText_FilledStorageSpace).cast::<u8>(),
                            );
                        }
                    }
                    AddTextPrinterParameterized(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        1u8,
                        (&raw mut gStringVar4).cast::<u8>(),
                        0u8,
                        41u8,
                        255u8,
                        None,
                    );
                }
                CopyWindowToVram(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    2u8,
                );
                CopyWindowToVram(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    2u8,
                );
                let __p15 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .read(),
                    );
                    PutWindowTilemap(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                }
                CopyBgTilemapBufferToVram(0u8);
                FadeOutAndFadeInNewMapMusic(523u16, 20u8, 10u8);
                let __p16 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((({
                    let __p17 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>();
                    let __t18 = ((__p17).read()).wrapping_add(1);
                    (__p17).write(__t18);
                    __t18
                }) as i32)
                    >= 30i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>())
                    .write(0u16);
                    PlaySE(5u16);
                    let __p19 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p19).write(((__p19).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                ClearWindowTilemap(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                RemoveWindow(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                RemoveWindow(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12288)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_WantToPlayAgain() {
    unsafe {
        let mut y: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                    .cast::<u8>())
                .write(
                    ((AddWindow(
                        ((&raw const sWindowTemplates_PlayAgain)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    )) as u8),
                );
                ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                    .cast::<u8>())
                .wrapping_offset(1))
                .write(
                    ((AddWindow(
                        (((&raw const sWindowTemplates_PlayAgain)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(8),
                    )) as u8),
                );
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                ClearWindowTilemap(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                DrawMessageWindow(
                    ((&raw const sWindowTemplates_PlayAgain)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DrawYesNoMessageWindow(
                    (((&raw const sWindowTemplates_PlayAgain)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(8),
                );
                let __p2 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12320))
                    .write(0u8);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                FillWindowPixelBuffer(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    17u8,
                );
                FillWindowPixelBuffer(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    17u8,
                );
                AddTextPrinterParameterized(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    1u8,
                    (&raw mut gText_WantToPlayAgain).cast::<u8>(),
                    0u8,
                    5u8,
                    255u8,
                    None,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_Yes).cast::<u8>(),
                    8u8,
                    1u8,
                    255u8,
                    None,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_No).cast::<u8>(),
                    8u8,
                    17u8,
                    255u8,
                    None,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_SelectorArrow2).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    2u8,
                );
                CopyWindowToVram(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    2u8,
                );
                let __p3 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .read(),
                    );
                    PutWindowTilemap(
                        ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                }
                CopyBgTilemapBufferToVram(0u8);
                let __p4 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                y = ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12320))
                    .read();
                if ((y) as i32) == 0i32 {
                    y = 1u8;
                }
                FillWindowPixelBuffer(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    17u8,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_Yes).cast::<u8>(),
                    8u8,
                    1u8,
                    255u8,
                    None,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_No).cast::<u8>(),
                    8u8,
                    17u8,
                    255u8,
                    None,
                );
                AddTextPrinterParameterized(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    1u8,
                    (&raw mut gText_SelectorArrow2).cast::<u8>(),
                    0u8,
                    ((((((y) as i32).wrapping_sub(1i32)).wrapping_mul(16i32)).wrapping_add(1i32))
                        as u8),
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    3u8,
                );
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    if ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12320))
                    .read()) as i32)
                        == 0i32
                    {
                        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12320))
                        .write(1u8);
                    }
                    let __p5 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 192i32)
                        != 0
                    {
                        PlaySE(5u16);
                        'l2: {
                            let __sw6 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12320))
                            .read()) as i32);
                            if __sw6 == 0i32 {
                                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12320))
                                .write(2u8);
                                break 'l2;
                            }
                            if __sw6 == 1i32 {
                                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12320))
                                .write(2u8);
                                break 'l2;
                            }
                            if __sw6 == 2i32 {
                                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(12320))
                                .write(1u8);
                                break 'l2;
                            }
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            PlaySE(5u16);
                            ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12320))
                            .write(2u8);
                            let __p7 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(12308);
                            (__p7).write(((__p7).read()).wrapping_add(1));
                        }
                    }
                }
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324))
                    .write(
                        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12320))
                        .read(),
                    );
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                ClearWindowTilemap(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                RemoveWindow(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                RemoveWindow(
                    ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12288)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_SavingDontTurnOff() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                DrawDialogueFrame(0u8, 0u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                let __p2 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                CopyWindowToVram(0u8, 3u8);
                let __p3 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CreateTask(Some(Task_LinkFullSave), 0u8);
                    let __p4 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((FuncIsActiveTask(Some(Task_LinkFullSave))) != 0) {
                    let __p5 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12288)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_CommunicationStandby() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                    .cast::<u8>())
                .write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_CommStandby)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                DrawMessageWindow(
                    (&raw const sWindowTemplate_CommStandby)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p2 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                FillWindowPixelBuffer(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    17u8,
                );
                AddTextPrinterParameterized(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    1u8,
                    (&raw mut gText_CommunicationStandby3).cast::<u8>(),
                    0u8,
                    5u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    2u8,
                );
                let __p3 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .read(),
                    );
                }
                CopyBgTilemapBufferToVram(0u8);
                let __p4 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12288)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EraseMessage() {
    unsafe {
        ClearWindowTilemap(
            (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                .cast::<u8>())
            .read(),
        );
        RemoveWindow(
            (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                .cast::<u8>())
            .read(),
        );
        FillBgTilemapBufferRect_Palette0(
            0u8,
            0u16,
            0u8,
            0u8,
            ((crate::c::div_i32(240i32, 8i32)) as u8),
            ((crate::c::div_i32(160i32, 8i32)) as u8),
        );
        CopyBgTilemapBufferToVram(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .write(1u32);
    }
}
pub(crate) unsafe extern "C" fn Msg_SomeoneDroppedOut() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12308))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12296))
                    .cast::<u8>())
                .write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_DroppedOut)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                DrawMessageWindow(
                    (&raw const sWindowTemplate_DroppedOut)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p2 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12316)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12320))
                    .write(0u8);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                FillWindowPixelBuffer(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    17u8,
                );
                AddTextPrinterParameterized(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    1u8,
                    (&raw mut gText_SomeoneDroppedOut).cast::<u8>(),
                    0u8,
                    5u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                    2u8,
                );
                let __p3 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12296))
                        .cast::<u8>())
                        .read(),
                    );
                }
                CopyBgTilemapBufferToVram(0u8);
                let __p4 =
                    (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p5 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12316)
                        .cast::<u16>();
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    >= 120i32
                {
                    let __p7 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12308);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324))
                    .write(5u8);
                ClearWindowTilemap(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                RemoveWindow(
                    (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12296))
                    .cast::<u8>())
                    .read(),
                );
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                CopyBgTilemapBufferToVram(0u8);
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12288)
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StopGfxFuncs() {
    unsafe {
        DestroyTask(
            ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12292)).read(),
        );
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .write(1u32);
    }
}
pub(crate) unsafe extern "C" fn GfxIdle() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn SetGfxFunc(func: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut func = func;
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12308)).write(0u8);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .write(0u32);
        ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12328)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(func);
    }
}
pub(crate) unsafe extern "C" fn GetGfxFunc() -> Option<unsafe extern "C" fn()> {
    unsafe {
        return ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12328)
            .cast::<Option<unsafe extern "C" fn()>>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn IsGfxFuncActive() -> u32 {
    unsafe {
        if ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12288)
            .cast::<u32>())
        .read()
            == 1u32
        {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetPlayAgainState() -> u8 {
    unsafe {
        return ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12324))
            .read();
    }
}
pub(crate) unsafe extern "C" fn InitBgs() {
    unsafe {
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    4096i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
        }
        'l10: loop {
            'l11: {
                {
                    let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u8).cast::<u32>();
                    let mut _size: u32 = 1024u32;
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l14: loop {
                                    'l15: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2231369728u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l14;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l12;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l10;
            }
        }
        'l16: loop {
            'l17: {
                {
                    let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u8).cast::<u16>();
                    let mut _size: u32 = 1024u32;
                    'l18: loop {
                        'l19: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l20: loop {
                                    'l21: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l18;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l16;
            }
        }
        SetGpuReg(0u8, 0u16);
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
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
        InitStandardTextBoxWindows();
        InitTextBoxGfxAndPrinters();
        SetGpuReg(0u8, 4160u16);
        SetBgTilemapBuffer(
            3u8,
            (((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(4096))
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(8192))
            .cast::<u16>())
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadBgGfx() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12312))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                LoadPalette(
                    (((&raw const sBg_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    0u16,
                    64u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetTempTileDataBuffers();
                break 'l1;
            }
            if __sw1 == 2i32 {
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw const sBg_Gfx).cast::<u8>().cast_mut().cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw const sTreeBorder_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) == 1i32 {
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadPalette((GetTextWindowPalette(3u8)).cast::<u8>(), 208u16, 32u16);
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12312))
                    .write(0u8);
                return 1u32;
            }
        }
        let __p2 = (((&raw mut sGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12312);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
