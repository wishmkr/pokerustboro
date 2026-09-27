//! Translated from `src/move_relearner.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUI_Pal sUI_Tiles sHeartSpriteOamData sUnusedOam1 sUnusedOam2 sMoveRelearnerSpriteSheet sMoveRelearnerPalette sDisplayModeArrowsTemplate sMoveListScrollArrowsTemplate sHeartSprite_AppealEmptyFrame sHeartSprite_AppealFullFrame sHeartSprite_JamEmptyFrame sHeartSprite_JamFullFrame sHeartSpriteAnimationCommands sConstestMoveHeartSprite sMoveRelearnerMenuBackgroundTemplates
#[allow(unused_imports)]
use crate::data::move_relearner::*;

pub(crate) static mut sMoveRelearnerStruct: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sMoveRelearnerMenuState: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut gContestEffects: u8;
    static mut gContestMoves: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTempScrollArrowTemplate: u8;
    static mut gText_Cancel: u8;
    static mut gText_MoveRelearnerAndPoof: u8;
    static mut gText_MoveRelearnerGiveUp: u8;
    static mut gText_MoveRelearnerPkmnForgotMoveAndLearnedNew: u8;
    static mut gText_MoveRelearnerPkmnLearnedMove: u8;
    static mut gText_MoveRelearnerPkmnTryingToLearnMove: u8;
    static mut gText_MoveRelearnerStopTryingToTeachMove: u8;
    static mut gText_MoveRelearnerTeachMoveConfirm: u8;
    static mut gText_MoveRelearnerWhichMoveToForget: u8;
    static mut gText_TeachWhichMoveToPkmn: u8;
    fn AddScrollIndicatorArrowPair(a0: *mut u8, a1: *mut u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn ClearScheduledBgCopiesToVram();
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetLRKeysPressed() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMoveRelearnerMoves(a0: *mut u8, a1: *mut u16) -> u8;
    fn GiveMoveToMon(a0: *mut u8, a1: u16) -> u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMoveRelearnerWindows(a0: u8);
    fn IsFanfareTaskInactive() -> u8;
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadMoveRelearnerMovesList(a0: *mut u8, a1: u16) -> u8;
    fn LoadOam();
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LockPlayerFieldControls();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MoveRelearnerCreateYesNoMenu();
    fn MoveRelearnerPrintMessage(a0: *mut u8);
    fn MoveRelearnerRunTextPrinters() -> u16;
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveMonPPBonus(a0: *mut u8, a1: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut u8,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn VBlankCB_MoveRelearner() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TeachMoveRelearnerMove() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_WaitForFadeOut), 10u8);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForFadeOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetMainCallback2(Some(CB2_InitLearnMove));
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_ContinueScriptHandleMusic));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_InitLearnMove() {
    unsafe {
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetTasks();
        ClearScheduledBgCopiesToVram();
        ((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(280u32));
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(68))
        .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        SetVBlankCallback(Some(VBlankCB_MoveRelearner));
        InitMoveRelearnerBackgroundLayers();
        InitMoveRelearnerWindows(0u8);
        (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>()).write(0u16);
        (((&raw mut sMoveRelearnerMenuState).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).write(0u8);
        CreateLearnableMovesList();
        LoadSpriteSheet(
            (&raw const sMoveRelearnerSpriteSheet)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpritePalette((&raw const sMoveRelearnerPalette).cast::<u8>().cast_mut());
        CreateUISprites();
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(274))
        .write(ListMenuInit(
            (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
            (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>()).read(),
            (((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        ));
        SetBackdropFromColor(0u16);
        SetMainCallback2(Some(CB2_MoveRelearnerMain));
    }
}
pub(crate) unsafe extern "C" fn CB2_InitLearnMoveReturnFromSelectMove() {
    unsafe {
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetTasks();
        ClearScheduledBgCopiesToVram();
        ((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(280u32));
        (((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .write(28u8);
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(68))
        .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(69))
        .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
        SetVBlankCallback(Some(VBlankCB_MoveRelearner));
        InitMoveRelearnerBackgroundLayers();
        InitMoveRelearnerWindows(
            (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).read(),
        );
        CreateLearnableMovesList();
        LoadSpriteSheet(
            (&raw const sMoveRelearnerSpriteSheet)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpritePalette((&raw const sMoveRelearnerPalette).cast::<u8>().cast_mut());
        CreateUISprites();
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(274))
        .write(ListMenuInit(
            (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
            (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>()).read(),
            (((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        ));
        SetBackdropFromColor(0u16);
        SetMainCallback2(Some(CB2_MoveRelearnerMain));
    }
}
pub(crate) unsafe extern "C" fn InitMoveRelearnerBackgroundLayers() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sMoveRelearnerMenuBackgroundTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        ResetAllBgsCoordinates();
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn CB2_MoveRelearnerMain() {
    unsafe {
        DoMoveRelearnerMain();
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn PrintMessageWithPlaceholders(src: *mut u8) {
    unsafe {
        let mut src = src;
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), src);
        MoveRelearnerPrintMessage((&raw mut gStringVar4).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn DoMoveRelearnerMain() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                HideHeartSpritesAndShowTeachMoveText(0u8);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                HideHeartSpritesAndShowTeachMoveText(0u8);
                let __p4 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                AddScrollArrows();
                break 'l1;
            }
            if __sw1 == 4i32 {
                HandleInput(0u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ShowTeachMoveText(0u8);
                let __p5 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p5).write(((__p5).read()).wrapping_add(1));
                AddScrollArrows();
                break 'l1;
            }
            if __sw1 == 6i32 {
                HandleInput(1u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    MoveRelearnerCreateYesNoMenu();
                    let __p6 = (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read());
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                {
                    let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
                    if ((selection) as i32) == 0i32 {
                        if ((GiveMoveToMon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(68))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            ((GetCurrentSelectedMove()) as u16),
                        )) as i32)
                            != 65535i32
                        {
                            PrintMessageWithPlaceholders(
                                (&raw mut gText_MoveRelearnerPkmnLearnedMove).cast::<u8>(),
                            );
                            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
                            (((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .write(31u8);
                        } else {
                            (((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .write(16u8);
                        }
                    } else {
                        if (((selection) as i32) == (-1i32)) || (((selection) as i32) == 1i32) {
                            if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                                .wrapping_add(4))
                            .read()) as i32)
                                == 0i32
                            {
                                (((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .write(3u8);
                            } else {
                                if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                                    .wrapping_add(4))
                                .read()) as i32)
                                    == 1i32
                                {
                                    (((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .write(5u8);
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    MoveRelearnerCreateYesNoMenu();
                    let __p7 = (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                {
                    let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
                    if ((selection) as i32) == 0i32 {
                        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
                        (((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .write(14u8);
                    } else {
                        if (((selection) as i32) == (-1i32)) || (((selection) as i32) == 1i32) {
                            if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                                .wrapping_add(4))
                            .read()) as i32)
                                == 0i32
                            {
                                (((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .write(3u8);
                            } else {
                                if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                                    .wrapping_add(4))
                                .read()) as i32)
                                    == 1i32
                                {
                                    (((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .write(5u8);
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                PrintMessageWithPlaceholders(
                    (&raw mut gText_MoveRelearnerPkmnTryingToLearnMove).cast::<u8>(),
                );
                let __p8 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    MoveRelearnerCreateYesNoMenu();
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(18u8);
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                {
                    let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
                    if ((selection) as i32) == 0i32 {
                        PrintMessageWithPlaceholders(
                            (&raw mut gText_MoveRelearnerWhichMoveToForget).cast::<u8>(),
                        );
                        (((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .write(19u8);
                    } else {
                        if (((selection) as i32) == (-1i32)) || (((selection) as i32) == 1i32) {
                            (((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .write(24u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 24i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>())
                        .wrapping_offset((GetCurrentSelectedMove()) as isize * 13))
                    .cast::<u8>(),
                );
                PrintMessageWithPlaceholders(
                    (&raw mut gText_MoveRelearnerStopTryingToTeachMove).cast::<u8>(),
                );
                let __p9 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 25i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    MoveRelearnerCreateYesNoMenu();
                    let __p10 = (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read());
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 26i32 {
                {
                    let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
                    if ((selection) as i32) == 0i32 {
                        (((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .write(27u8);
                    } else {
                        if (((selection) as i32) == (-1i32)) || (((selection) as i32) == 1i32) {
                            if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                                .wrapping_add(4))
                            .read()) as i32)
                                == 0i32
                            {
                                (((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .write(3u8);
                            } else {
                                if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                                    .wrapping_add(4))
                                .read()) as i32)
                                    == 1i32
                                {
                                    (((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .write(5u8);
                                }
                            }
                            (((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .write(16u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 27i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    FillWindowPixelBuffer(3u8, 17u8);
                    if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).read())
                        as i32)
                        == 0i32
                    {
                        (((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .write(3u8);
                    } else {
                        if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4))
                            .read()) as i32)
                            == 1i32
                        {
                            (((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .write(5u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(20u8);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ShowSelectMovePokemonSummaryScreen(
                        (&raw mut gPlayerParty).cast::<u8>(),
                        ((((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(68))
                        .read(),
                        ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                            .wrapping_sub(1i32)) as u8),
                        Some(CB2_InitLearnMoveReturnFromSelectMove),
                        ((GetCurrentSelectedMove()) as u16),
                    );
                    FreeMoveRelearnerResources();
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(14u8);
                }
                break 'l1;
            }
            if __sw1 == 22i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                break 'l1;
            }
            if __sw1 == 14i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p11 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeMoveRelearnerResources();
                    SetMainCallback2(Some(CB2_ReturnToField));
                }
                break 'l1;
            }
            if __sw1 == 28i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p12 = (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p12).write(((__p12).read()).wrapping_add(1));
                if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).read())
                    as i32)
                    == 0i32
                {
                    HideHeartSpritesAndShowTeachMoveText(1u8);
                } else {
                    if (((((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).read())
                        as i32)
                        == 1i32
                    {
                        ShowTeachMoveText(1u8);
                    }
                }
                RemoveScrollArrows();
                CopyWindowToVram(3u8, 2u8);
                break 'l1;
            }
            if __sw1 == 29i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if ((((((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(69))
                    .read()) as i32)
                        == 4i32
                    {
                        (((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .write(24u8);
                    } else {
                        let mut r#move: u16 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(68))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            (13i32).wrapping_add(
                                ((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(69))
                                .read()) as i32),
                            ),
                        )) as u16);
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            (((&raw mut gMoveNames).cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 13))
                            .cast::<u8>(),
                        );
                        RemoveMonPPBonus(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(68))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            ((((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(69))
                            .read(),
                        );
                        SetMonMoveSlot(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(68))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            ((GetCurrentSelectedMove()) as u16),
                            ((((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(69))
                            .read(),
                        );
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (((&raw mut gMoveNames).cast::<u8>())
                                .wrapping_offset((GetCurrentSelectedMove()) as isize * 13))
                            .cast::<u8>(),
                        );
                        PrintMessageWithPlaceholders(
                            (&raw mut gText_MoveRelearnerAndPoof).cast::<u8>(),
                        );
                        (((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .write(30u8);
                        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 30i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    PrintMessageWithPlaceholders(
                        (&raw mut gText_MoveRelearnerPkmnForgotMoveAndLearnedNew).cast::<u8>(),
                    );
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(31u8);
                    PlayFanfare(367u16);
                }
                break 'l1;
            }
            if __sw1 == 31i32 {
                if !((MoveRelearnerRunTextPrinters()) != 0) {
                    PlayFanfare(367u16);
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(32u8);
                }
                break 'l1;
            }
            if __sw1 == 32i32 {
                if (IsFanfareTaskInactive()) != 0 {
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(33u8);
                }
                break 'l1;
            }
            if __sw1 == 33i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(14u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeMoveRelearnerResources() {
    unsafe {
        RemoveScrollArrows();
        DestroyListMenuTask(
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(274))
            .read(),
            ((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>(),
            ((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>(),
        );
        FreeAllWindowBuffers();
        {
            Free(
                ((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        ResetSpriteData();
        FreeAllSpritePalettes();
    }
}
pub(crate) unsafe extern "C" fn HideHeartSpritesAndShowTeachMoveText(onlyHideSprites: u8) {
    unsafe {
        let mut onlyHideSprites = onlyHideSprites;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((onlyHideSprites) != 0) {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_TeachWhichMoveToPkmn).cast::<u8>(),
            );
            FillWindowPixelBuffer(3u8, 17u8);
            AddTextPrinterParameterized(
                3u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                0u8,
                1u8,
                0u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInput(showContest: u8) {
    unsafe {
        let mut showContest = showContest;
        let mut itemId: i32 = ListMenu_ProcessInput(
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(274))
            .read(),
        );
        ListMenuGetScrollAndRow(
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(274))
            .read(),
            ((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>(),
            ((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>(),
        );
        'l1: {
            let __sw1 = itemId;
            let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
            if __sw1 == (-1i32) {
                if (!(((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 48i32)
                    != 0))
                    && (!((GetLRKeysPressed()) != 0))
                {
                    break 'l1;
                }
                PlaySE(5u16);
                if ((showContest) as i32) == 0i32 {
                    PutWindowTilemap(1u8);
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(5u8);
                    (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).write(1u8);
                } else {
                    PutWindowTilemap(0u8);
                    (((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .write(3u8);
                    (((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).write(0u8);
                }
                ScheduleBgCopyTilemapToVram(1u8);
                MoveRelearnerShowHideHearts(GetCurrentSelectedMove());
                break 'l1;
            }
            if __sw1 == (-2i32) {
                PlaySE(5u16);
                RemoveScrollArrows();
                (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .write(12u8);
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_MoveRelearnerGiveUp).cast::<u8>(),
                );
                MoveRelearnerPrintMessage((&raw mut gStringVar4).cast::<u8>());
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                RemoveScrollArrows();
                (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .write(8u8);
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset((itemId) as isize * 13))
                        .cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_MoveRelearnerTeachMoveConfirm).cast::<u8>(),
                );
                MoveRelearnerPrintMessage((&raw mut gStringVar4).cast::<u8>());
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrentSelectedMove() -> i32 {
    unsafe {
        return (((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(72))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sMoveRelearnerMenuState).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(
                    (((((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>()).read())
                        as i32),
                )) as isize
                * 8,
        ))
        .wrapping_add(4)
        .cast::<i32>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn ShowTeachMoveText(shouldDoNothingInstead: u8) {
    unsafe {
        let mut shouldDoNothingInstead = shouldDoNothingInstead;
        if ((shouldDoNothingInstead) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_TeachWhichMoveToPkmn).cast::<u8>(),
            );
            FillWindowPixelBuffer(3u8, 17u8);
            AddTextPrinterParameterized(
                3u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                0u8,
                1u8,
                0u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreateUISprites() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(276))
        .write(255u8);
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(275))
        .write(255u8);
        AddScrollArrows();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(CreateSprite(
                        (&raw const sConstestMoveHeartSprite)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((i).wrapping_sub((crate::c::div_i32(i, 4i32)).wrapping_mul(4i32)))
                            .wrapping_mul(8i32))
                        .wrapping_add(104i32)) as i16),
                        ((((crate::c::div_i32(i, 4i32)).wrapping_mul(8i32)).wrapping_add(36i32))
                            as i16),
                        0u8,
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 8i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(8i32)) as isize))
                    .write(CreateSprite(
                        (&raw const sConstestMoveHeartSprite)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((i).wrapping_sub((crate::c::div_i32(i, 4i32)).wrapping_mul(4i32)))
                            .wrapping_mul(8i32))
                        .wrapping_add(104i32)) as i16),
                        ((((crate::c::div_i32(i, 4i32)).wrapping_mul(8i32)).wrapping_add(52i32))
                            as i16),
                        0u8,
                    ));
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .cast::<u8>())
                            .wrapping_offset(((i).wrapping_add(8i32)) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        2u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 16i32) {
                    break 'l5;
                }
                'l6: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddScrollArrows() {
    unsafe {
        if ((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(276))
        .read()) as i32)
            == 255i32
        {
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(276))
            .write(AddScrollIndicatorArrowPair(
                (&raw const sDisplayModeArrowsTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                (((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(278)
                .cast::<u16>(),
            ));
        }
        if ((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(275))
        .read()) as i32)
            == 255i32
        {
            (&raw mut gTempScrollArrowTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<16>>()
                .write_unaligned(
                    (&raw const sMoveListScrollArrowsTemplate)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<16>>()
                        .read_unaligned(),
                );
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .write(
                ((((((((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(272))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(273))
                        .read()) as i32),
                    )) as u16),
            );
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(275))
            .write(AddScrollIndicatorArrowPair(
                (&raw mut gTempScrollArrowTemplate).cast::<u8>(),
                ((&raw mut sMoveRelearnerMenuState).cast::<u8>()).cast::<u16>(),
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn RemoveScrollArrows() {
    unsafe {
        if ((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(276))
        .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(276))
                .read(),
            );
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(276))
            .write(255u8);
        }
        if ((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(275))
        .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(275))
                .read(),
            );
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(275))
            .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateLearnableMovesList() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(272))
        .write(GetMoveRelearnerMoves(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(68))
                .read()) as i32) as isize
                    * 100,
            ),
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(18))
            .cast::<u16>(),
        ));
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(272))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(72))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sMoveRelearnerStruct)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(18))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>(),
                    );
                    (((((((&raw mut sMoveRelearnerStruct)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(72))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(
                        ((((((((&raw mut sMoveRelearnerStruct)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(18))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sMoveRelearnerStruct)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(68))
                .read()) as i32) as isize
                    * 100,
            ),
            2i32,
            (&raw mut nickname).cast::<u8>(),
        );
        StringCopy_Nickname(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut nickname).cast::<u8>(),
        );
        (((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(72))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(272))
            .read()) as i32) as isize
                * 8,
        ))
        .cast::<*mut u8>())
        .write((&raw mut gText_Cancel).cast::<u8>());
        (((((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(72))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(272))
            .read()) as i32) as isize
                * 8,
        ))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        let __p1 = (((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(272);
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((&raw mut sMoveRelearnerStruct)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(273))
        .write(LoadMoveRelearnerMovesList(
            ((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(72))
            .cast::<u8>(),
            ((((((&raw mut sMoveRelearnerStruct)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(272))
            .read()) as u16),
        ));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRelearnerShowHideHearts(r#move: i32) {
    unsafe {
        let mut r#move = r#move;
        let mut numHearts: u16 = 0u16;
        let mut i: u16 = 0u16;
        if (!(((((&raw mut sMoveRelearnerMenuState).cast::<u8>()).wrapping_add(4)).read()) != 0))
            || (r#move == (-2i32))
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            numHearts = (((crate::c::div_i32(
                ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset((r#move) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(1))
                .read()) as i32),
                10i32,
            )) as u8) as u16);
            if ((numHearts) as i32) == 255i32 {
                numHearts = 0u16;
            }
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((i) as i32) < ((numHearts) as i32) {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                1u8,
                            );
                        } else {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                0u8,
                            );
                        }
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            numHearts = (((crate::c::div_i32(
                ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset((r#move) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(2))
                .read()) as i32),
                10i32,
            )) as u8) as u16);
            if ((numHearts) as i32) == 255i32 {
                numHearts = 0u16;
            }
            {
                i = 0u16;
                'l5: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((i) as i32) < ((numHearts) as i32) {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(8i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                3u8,
                            );
                        } else {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sMoveRelearnerStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1))
                                    .cast::<u8>())
                                    .wrapping_offset((((i) as i32).wrapping_add(8i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                2u8,
                            );
                        }
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sMoveRelearnerStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1))
                                .cast::<u8>())
                                .wrapping_offset((((i) as i32).wrapping_add(8i32)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromColor(color: u16) {
    unsafe {
        let mut color = color;
        FillPalette(color, 0u16, 2u16);
    }
}
