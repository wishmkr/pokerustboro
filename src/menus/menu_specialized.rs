//! Translated from `src/menu_specialized.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWindowTemplates_MailboxMenu sPlayerNameTextColors sEmptyItemName sConditionGraphScanline sConditionToLineLength sMoveRelearnerWindowTemplates sMoveRelearnerYesNoMenuTemplate sMoveRelearnerMovesListTemplate sConditionPokeball_Gfx sConditionPokeballPlaceholder_Gfx sConditionSparkle_Gfx sConditionSparkle_Pal sOam_ConditionMonPic sOam_ConditionSelectionIcon sAnim_ConditionSelectionIcon_Selected sAnim_ConditionSelectionIcon_Unselected sAnims_ConditionSelectionIcon sOam_ConditionSparkle sAnim_ConditionSparkle sAnims_ConditionSparkle sSpriteTemplate_ConditionSparkle sConditionSparkleCoords sLvlUpStatStrings
#[allow(unused_imports)]
use crate::data::menu_specialized::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMailboxWindowIds: crate::ffi::Align4<[u8; 3]> = crate::ffi::Align4([0; 3]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMailboxList: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleMoves: u8;
    static mut gContestEffectDescriptionPointers: u8;
    static mut gContestMoveTypeTextPointers: u8;
    static mut gContestMoves: u8;
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gDummySpriteAnimTable: u8;
    static mut gMailboxMailOptions: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMoveDescriptionPointers: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPlayerParty: u8;
    static mut gPokenavConditionCancel_Gfx: u8;
    static mut gPokenavConditionCancel_Pal: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gTextFlags: u8;
    static mut gText_Cancel2: u8;
    static mut gText_Dash: u8;
    static mut gText_EggNickname: u8;
    static mut gText_InParty: u8;
    static mut gText_MoveRelearnerAccuracy: u8;
    static mut gText_MoveRelearnerAppeal: u8;
    static mut gText_MoveRelearnerBattleMoves: u8;
    static mut gText_MoveRelearnerContestMovesTitle: u8;
    static mut gText_MoveRelearnerJam: u8;
    static mut gText_MoveRelearnerPP: u8;
    static mut gText_MoveRelearnerPower: u8;
    static mut gText_Plus: u8;
    static mut gText_ThreeDashes: u8;
    static mut gTypeNames: u8;
    fn AddScrollIndicatorArrowPairParameterized(
        a0: u32,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: *mut u16,
    ) -> u8;
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
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalPlayerName(a0: *mut u8);
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetAndCopyBoxMonDataAt(a0: u8, a1: u8, a2: i32, a3: *mut u8) -> u32;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxMonGender(a0: *mut u8) -> u8;
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetLevelFromBoxMonExp(a0: *mut u8) -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut u8, a1: i32) -> i32;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpecialPokePic(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32, a4: u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MoveRelearnerShowHideHearts(a0: i32);
    fn PlaySE(a0: u16);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RunTextPrinters();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SeekSpriteAnim(a0: *mut u8, a1: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MailboxMenu_Alloc(count: u8) -> u8 {
    unsafe {
        let mut count = count;
        let mut i: u8 = 0u8;
        ((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).write(Alloc(
            ((((count) as i32).wrapping_add(1i32)) as u32).wrapping_mul(8u32),
        ));
        if ((((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 0u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MailboxMenu_AddWindow(windowIdx: u8) -> u8 {
    unsafe {
        let mut windowIdx = windowIdx;
        if ((((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((windowIdx) as i32) as isize))
        .read()) as i32)
            == 255i32
        {
            if ((windowIdx) as i32) == 2i32 {
                let mut template = crate::ffi::Align4([0u8; 8]);
                (&raw mut template)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (((&raw const sWindowTemplates_MailboxMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((windowIdx) as i32) as isize * 8)
                        .cast::<crate::c::Rec4<8>>()
                        .read_unaligned(),
                    );
                (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(
                    ((GetMaxWidthInMenuTable((&raw mut gMailboxMailOptions).cast::<u8>(), 4i32))
                        as u8),
                );
                ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((windowIdx) as i32) as isize))
                .write(((AddWindow((&raw mut template).cast::<u8>())) as u8));
            } else {
                ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((windowIdx) as i32) as isize))
                .write(
                    ((AddWindow(
                        (((&raw const sWindowTemplates_MailboxMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((windowIdx) as i32) as isize * 8),
                    )) as u8),
                );
            }
            SetStandardWindowBorderStyle(
                ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((windowIdx) as i32) as isize))
                .read(),
                0u8,
            );
        }
        return ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((windowIdx) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MailboxMenu_RemoveWindow(windowIdx: u8) {
    unsafe {
        let mut windowIdx = windowIdx;
        ClearStdWindowAndFrameToTransparent(
            ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((windowIdx) as i32) as isize))
            .read(),
            0u8,
        );
        ClearWindowTilemap(
            ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((windowIdx) as i32) as isize))
            .read(),
        );
        RemoveWindow(
            ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((windowIdx) as i32) as isize))
            .read(),
        );
        ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((windowIdx) as i32) as isize))
        .write(255u8);
    }
}
pub(crate) unsafe extern "C" fn MailboxMenu_GetWindowId(windowIdx: u8) -> u8 {
    unsafe {
        let mut windowIdx = windowIdx;
        return ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((windowIdx) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn MailboxMenu_ItemPrintFunc(windowId: u8, itemId: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemId = itemId;
        let mut y = y;
        let mut buffer = crate::ffi::Align4([0u8; 30]);
        let mut length: u16 = 0u16;
        if itemId == 4294967294u32 {
            return;
        }
        StringCopy(
            (&raw mut buffer).cast::<u8>(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                .cast::<u8>())
            .wrapping_offset((((6u32).wrapping_add(itemId)) as i32) as isize * 36))
            .wrapping_add(18))
            .cast::<u8>(),
        );
        ConvertInternationalPlayerName((&raw mut buffer).cast::<u8>());
        length = StringLength((&raw mut buffer).cast::<u8>());
        if ((length) as i32) < 6i32 {
            ConvertInternationalString((&raw mut buffer).cast::<u8>(), 1u8);
        }
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            8u8,
            y,
            0u8,
            0u8,
            ((&raw const sPlayerNameTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut buffer).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MailboxMenu_CreateList(page: *mut u8) -> u8 {
    unsafe {
        let mut page = page;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((((page).wrapping_add(5)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(((&raw const sEmptyItemName).cast::<u8>().cast_mut()).cast::<u8>());
                    (((((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(((i) as i32));
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((i) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write((&raw mut gText_Cancel2).cast::<u8>());
        (((((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((i) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>())
            .write(((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read());
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(((((((page).wrapping_add(5)).read()) as i32).wrapping_add(1i32)) as u16));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16)).write(
            ((((&raw mut sMailboxWindowIds).cast::<u8>()).cast::<u8>()).wrapping_offset(1)).read(),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(17)).write(0u8);
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(18)).write(8u8);
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(19)).write(0u8);
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(8u16);
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(20),
            0,
            4,
            (9u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(20),
            4,
            4,
            (2u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(21),
            0,
            4,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(21),
            4,
            4,
            (3u8) as i32,
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(i32, u8, *mut u8)>>())
        .write(Some(MailboxMenu_MoveCursorFunc));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn(u8, u32, u8)>>())
        .write(Some(MailboxMenu_ItemPrintFunc));
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(23),
            0,
            6,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(23),
            6,
            2,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(22),
            0,
            3,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(22),
            3,
            3,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(22),
            6,
            2,
            (0u8) as i32,
        );
        return ListMenuInit(
            (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
            ((page).wrapping_add(2).cast::<u16>()).read(),
            ((page).cast::<u16>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MailboxMenu_MoveCursorFunc(
    itemIndex: i32,
    onInit: u8,
    list: *mut u8,
) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut onInit = onInit;
        let mut list = list;
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MailboxMenu_AddScrollArrows(page: *mut u8) {
    unsafe {
        let mut page = page;
        ((page).wrapping_add(9)).write(AddScrollIndicatorArrowPairParameterized(
            2u32,
            200i32,
            12i32,
            148i32,
            (((((page).wrapping_add(5)).read()) as i32)
                .wrapping_sub(((((page).wrapping_add(4)).read()) as i32)))
            .wrapping_add(1i32),
            110i32,
            110i32,
            (page).wrapping_add(2).cast::<u16>(),
        ));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MailboxMenu_Free() {
    unsafe {
        Free(((&raw mut sMailboxList).cast::<u8>().cast::<*mut u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_Init(graph: *mut u8) {
    unsafe {
        let mut graph = graph;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32) < 10i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((graph).wrapping_add(100)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 4))
                                .cast::<u16>())
                                .write(0u16);
                                (((((((graph).wrapping_add(100)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .write(0u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    {
                        i = 0u8;
                        'l5: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                (((((graph).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 5))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(0u8);
                                (((((((graph).wrapping_add(20)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 4))
                                .cast::<u16>())
                                .write(155u16);
                                (((((((graph).wrapping_add(20)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .write(
                                    (((crate::c::div_i32(177i32, 2i32)).wrapping_add(3i32)) as u16),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    (((((graph).wrapping_add(300)).cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize * 4))
                    .cast::<u16>())
                    .write(0u16);
                    (((((graph).wrapping_add(300)).cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(0u16);
                }
                j = (j).wrapping_add(1);
            }
        }
        ((graph).wrapping_add(852)).write(0u8);
        ((graph).wrapping_add(850).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_SetNewPositions(
    graph: *mut u8,
    old: *mut u8,
    new: *mut u8,
) {
    unsafe {
        let mut graph = graph;
        let mut old = old;
        let mut new = new;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut coord: i32 = 0i32;
        let mut increment: i32 = 0i32;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    coord = ((((((old).wrapping_offset(((i) as i32) as isize * 4)).cast::<u16>())
                        .read()) as i32)
                        << 8);
                    increment = crate::c::div_i32(
                        ((((((new).wrapping_offset(((i) as i32) as isize * 4)).cast::<u16>())
                            .read()) as i32)
                            .wrapping_sub(
                                (((((old).wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                .read()) as i32),
                            )
                            << 8),
                        10i32,
                    );
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < 9i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((graph).wrapping_add(100)).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<u16>())
                                .write((((coord >> 8).wrapping_add(((coord >> 7) & 1i32))) as u16));
                                coord = (coord).wrapping_add(increment);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((((graph).wrapping_add(100)).cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize * 20))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .write(
                        (((new).wrapping_offset(((i) as i32) as isize * 4)).cast::<u16>()).read(),
                    );
                    coord = ((((((old).wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32)
                        << 8);
                    increment = crate::c::div_i32(
                        ((((((new).wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(
                                (((((old).wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                .read()) as i32),
                            )
                            << 8),
                        10i32,
                    );
                    {
                        j = 0u16;
                        'l5: loop {
                            if !(((j) as i32) < 9i32) {
                                break 'l5;
                            }
                            'l6: {
                                (((((((graph).wrapping_add(100)).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .write((((coord >> 8).wrapping_add(((coord >> 7) & 1i32))) as u16));
                                coord = (coord).wrapping_add(increment);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((((graph).wrapping_add(100)).cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize * 20))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(
                        (((new).wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((graph).wrapping_add(850).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_TryUpdate(graph: *mut u8) -> u8 {
    unsafe {
        let mut graph = graph;
        if ((((graph).wrapping_add(850).cast::<u16>()).read()) as i32) < 10i32 {
            ConditionGraph_Update(graph);
            return (((({
                let __p1 = (graph).wrapping_add(850).cast::<u16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                != 10i32) as u8);
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_InitResetScanline(graph: *mut u8) {
    unsafe {
        let mut graph = graph;
        ((graph).wrapping_add(853)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_ResetScanline(graph: *mut u8) -> u8 {
    unsafe {
        let mut graph = graph;
        let mut params = crate::ffi::Align4([0u8; 12]);
        'l1: {
            let __sw1 = ((((graph).wrapping_add(853)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                ScanlineEffect_Clear();
                let __p2 = (graph).wrapping_add(853);
                (__p2).write(((__p2).read()).wrapping_add(1));
                return 1u8;
            }
            if __sw1 == 1i32 {
                (&raw mut params)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<12>>()
                    .write_unaligned(
                        (&raw const sConditionGraphScanline)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<12>>()
                            .read_unaligned(),
                    );
                ScanlineEffect_SetParams(
                    (&raw mut params)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<12>>()
                        .read_unaligned(),
                );
                let __p3 = (graph).wrapping_add(853);
                (__p3).write(((__p3).read()).wrapping_add(1));
                return 0u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_Draw(graph: *mut u8) {
    unsafe {
        let mut graph = graph;
        let mut i: u16 = 0u16;
        if !((((graph).wrapping_add(852)).read()) != 0) {
            return;
        }
        ConditionGraph_CalcRightHalf(graph);
        ConditionGraph_CalcLeftHalf(graph);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 66i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((i) as i32).wrapping_add(56i32)).wrapping_sub(1i32))
                            .wrapping_mul(2i32))
                        .wrapping_add(0i32)) as isize,
                    ))
                    .write({
                        let __v1 = ((((((((((graph).wrapping_add(320)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32)
                            << 8)
                            | ((((((((graph).wrapping_add(320)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .wrapping_offset(1))
                            .read()) as i32)) as u16);
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(
                                ((((((i) as i32).wrapping_add(56i32)).wrapping_sub(1i32))
                                    .wrapping_mul(2i32))
                                .wrapping_add(0i32)) as isize,
                            ))
                        .write(__v1);
                        __v1
                    });
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((i) as i32).wrapping_add(56i32)).wrapping_sub(1i32))
                            .wrapping_mul(2i32))
                        .wrapping_add(1i32)) as isize,
                    ))
                    .write({
                        let __v2 = ((((((((((graph).wrapping_add(584)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32)
                            << 8)
                            | ((((((((graph).wrapping_add(584)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .wrapping_offset(1))
                            .read()) as i32)) as u16);
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(
                                ((((((i) as i32).wrapping_add(56i32)).wrapping_sub(1i32))
                                    .wrapping_mul(2i32))
                                .wrapping_add(1i32)) as isize,
                            ))
                        .write(__v2);
                        __v2
                    });
                }
                i = (i).wrapping_add(1);
            }
        }
        ((graph).wrapping_add(852)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_InitWindow(bg: u8) {
    unsafe {
        let mut bg = bg;
        let mut flags: u32 = 0u32;
        if ((bg) as i32) >= 4i32 {
            bg = 0u8;
        }
        flags = ((31i32 & !(crate::c::shl_i32(1i32, ((bg) as u32)))) as u32);
        SetGpuReg(64u8, 240u16);
        SetGpuReg(66u8, 155u16);
        SetGpuReg(68u8, 14457u16);
        SetGpuReg(70u8, 14457u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, ((flags) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_Update(graph: *mut u8) {
    unsafe {
        let mut graph = graph;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((graph).wrapping_add(300)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4)
                        .cast::<crate::c::Rec4<4>>()
                        .write_unaligned(
                            (((((graph).wrapping_add(100)).cast::<u8>()).wrapping_offset(
                                ((((graph).wrapping_add(850).cast::<u16>()).read()) as i32)
                                    as isize
                                    * 20,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((graph).wrapping_add(852)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn ConditionGraph_CalcLine(
    graph: *mut u8,
    scanline: *mut u16,
    pos1: *mut u8,
    pos2: *mut u8,
    dir: u8,
    overflowScanline: *mut u16,
) {
    unsafe {
        let mut graph = graph;
        let mut scanline = scanline;
        let mut pos1 = pos1;
        let mut pos2 = pos2;
        let mut dir = dir;
        let mut overflowScanline = overflowScanline;
        let mut i: u16 = 0u16;
        let mut height: u16 = 0u16;
        let mut top: u16 = 0u16;
        let mut bottom: u16 = 0u16;
        let mut x2: u16 = 0u16;
        let mut ptr: *mut u16 = core::ptr::null_mut();
        let mut x: i32 = 0i32;
        let mut xIncrement: i32 = 0i32;
        if ((((pos1).wrapping_add(2).cast::<u16>()).read()) as i32)
            < ((((pos2).wrapping_add(2).cast::<u16>()).read()) as i32)
        {
            top = ((pos1).wrapping_add(2).cast::<u16>()).read();
            bottom = ((pos2).wrapping_add(2).cast::<u16>()).read();
            x = (((((pos1).cast::<u16>()).read()) as i32) << 10);
            x2 = ((pos2).cast::<u16>()).read();
            height = ((((bottom) as i32).wrapping_sub(((top) as i32))) as u16);
            if ((height) as i32) != 0i32 {
                xIncrement = crate::c::div_i32(
                    (((x2) as i32).wrapping_sub(((((pos1).cast::<u16>()).read()) as i32)) << 10),
                    ((height) as i32),
                );
            }
        } else {
            bottom = ((pos1).wrapping_add(2).cast::<u16>()).read();
            top = ((pos2).wrapping_add(2).cast::<u16>()).read();
            x = (((((pos2).cast::<u16>()).read()) as i32) << 10);
            x2 = ((pos1).cast::<u16>()).read();
            height = ((((bottom) as i32).wrapping_sub(((top) as i32))) as u16);
            if ((height) as i32) != 0i32 {
                xIncrement = crate::c::div_i32(
                    (((x2) as i32).wrapping_sub(((((pos2).cast::<u16>()).read()) as i32)) << 10),
                    ((height) as i32),
                );
            }
        }
        height = (height).wrapping_add(1);
        if ((overflowScanline) as usize) == 0usize {
            scanline = (scanline).wrapping_offset(
                ((((top) as i32).wrapping_sub(56i32)).wrapping_mul(2i32)) as isize,
            );
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < ((height) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((scanline).wrapping_offset(((dir) as i32) as isize)).write(
                            ((((x >> 10).wrapping_add(((x >> 9) & 1i32)))
                                .wrapping_add(((dir) as i32))) as u16),
                        );
                        x = (x).wrapping_add(xIncrement);
                        scanline = (scanline).wrapping_offset(2);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ptr = (scanline).wrapping_offset(-2);
        } else {
            if xIncrement > 0i32 {
                overflowScanline = (overflowScanline).wrapping_offset(
                    ((((top) as i32).wrapping_sub(56i32)).wrapping_mul(2i32)) as isize,
                );
                {
                    i = 0u16;
                    'l3: loop {
                        if !(((i) as i32) < ((height) as i32)) {
                            break 'l3;
                        }
                        'l4: {
                            if x >= 158720i32 {
                                break 'l3;
                            }
                        }
                        ((overflowScanline).wrapping_offset(((dir) as i32) as isize)).write(
                            ((((x >> 10).wrapping_add(((x >> 9) & 1i32)))
                                .wrapping_add(((dir) as i32))) as u16),
                        );
                        x = (x).wrapping_add(xIncrement);
                        overflowScanline = (overflowScanline).wrapping_offset(2);
                        i = (i).wrapping_add(1);
                    }
                }
                ((graph).wrapping_add(848).cast::<u16>())
                    .write(((((top) as i32).wrapping_add(((i) as i32))) as u16));
                scanline = (scanline).wrapping_offset(
                    ((((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)
                        .wrapping_sub(56i32))
                    .wrapping_mul(2i32)) as isize,
                );
                {
                    'l5: loop {
                        if !(((i) as i32) < ((height) as i32)) {
                            break 'l5;
                        }
                        'l6: {
                            ((scanline).wrapping_offset(((dir) as i32) as isize)).write(
                                ((((x >> 10).wrapping_add(((x >> 9) & 1i32)))
                                    .wrapping_add(((dir) as i32)))
                                    as u16),
                            );
                            x = (x).wrapping_add(xIncrement);
                            scanline = (scanline).wrapping_offset(2);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ptr = (scanline).wrapping_offset(-2);
            } else {
                if xIncrement < 0i32 {
                    scanline = (scanline).wrapping_offset(
                        ((((top) as i32).wrapping_sub(56i32)).wrapping_mul(2i32)) as isize,
                    );
                    {
                        i = 0u16;
                        'l7: loop {
                            if !(((i) as i32) < ((height) as i32)) {
                                break 'l7;
                            }
                            'l8: {
                                ((scanline).wrapping_offset(((dir) as i32) as isize)).write(
                                    ((((x >> 10).wrapping_add(((x >> 9) & 1i32)))
                                        .wrapping_add(((dir) as i32)))
                                        as u16),
                                );
                                if x < 158720i32 {
                                    ((scanline).wrapping_offset(((dir) as i32) as isize))
                                        .write(155u16);
                                    break 'l7;
                                }
                                x = (x).wrapping_add(xIncrement);
                                scanline = (scanline).wrapping_offset(2);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((graph).wrapping_add(848).cast::<u16>())
                        .write(((((top) as i32).wrapping_add(((i) as i32))) as u16));
                    overflowScanline = (overflowScanline).wrapping_offset(
                        ((((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)
                            .wrapping_sub(56i32))
                        .wrapping_mul(2i32)) as isize,
                    );
                    {
                        'l9: loop {
                            if !(((i) as i32) < ((height) as i32)) {
                                break 'l9;
                            }
                            'l10: {
                                ((overflowScanline).wrapping_offset(((dir) as i32) as isize))
                                    .write(
                                        ((((x >> 10).wrapping_add(((x >> 9) & 1i32)))
                                            .wrapping_add(((dir) as i32)))
                                            as u16),
                                    );
                                x = (x).wrapping_add(xIncrement);
                                overflowScanline = (overflowScanline).wrapping_offset(2);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ptr = (overflowScanline).wrapping_offset(-2);
                } else {
                    ((graph).wrapping_add(848).cast::<u16>()).write(top);
                    scanline = (scanline).wrapping_offset(
                        ((((top) as i32).wrapping_sub(56i32)).wrapping_mul(2i32)) as isize,
                    );
                    overflowScanline = (overflowScanline).wrapping_offset(
                        ((((top) as i32).wrapping_sub(56i32)).wrapping_mul(2i32)) as isize,
                    );
                    ((scanline).wrapping_offset(1)).write(
                        ((((((pos1).cast::<u16>()).read()) as i32).wrapping_add(1i32)) as u16),
                    );
                    (overflowScanline).write(((pos2).cast::<u16>()).read());
                    ((overflowScanline).wrapping_offset(1)).write(155u16);
                    return;
                }
            }
        }
        ((ptr).wrapping_offset(((dir) as i32) as isize))
            .write(((((dir) as i32).wrapping_add(((x2) as i32))) as u16));
    }
}
pub(crate) unsafe extern "C" fn ConditionGraph_CalcRightHalf(graph: *mut u8) {
    unsafe {
        let mut graph = graph;
        let mut i: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut bottom: u16 = 0u16;
        if ((((((graph).wrapping_add(300)).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            < (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(4))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
        {
            y = ((((graph).wrapping_add(300)).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read();
            ConditionGraph_CalcLine(
                graph,
                (((graph).wrapping_add(320)).cast::<u8>()).cast::<u16>(),
                ((graph).wrapping_add(300)).cast::<u8>(),
                (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(4),
                1u8,
                core::ptr::null_mut(),
            );
        } else {
            y = (((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(4))
                .wrapping_add(2)
                .cast::<u16>())
            .read();
            ConditionGraph_CalcLine(
                graph,
                (((graph).wrapping_add(320)).cast::<u8>()).cast::<u16>(),
                (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(4),
                ((graph).wrapping_add(300)).cast::<u8>(),
                0u8,
                core::ptr::null_mut(),
            );
        }
        ConditionGraph_CalcLine(
            graph,
            (((graph).wrapping_add(320)).cast::<u8>()).cast::<u16>(),
            (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(4),
            (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(8),
            1u8,
            core::ptr::null_mut(),
        );
        i = (((((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(8))
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            <= (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(12))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)) as u16);
        ConditionGraph_CalcLine(
            graph,
            (((graph).wrapping_add(320)).cast::<u8>()).cast::<u16>(),
            (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(8),
            (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(12),
            ((i) as u8),
            (((graph).wrapping_add(584)).cast::<u8>()).cast::<u16>(),
        );
        {
            i = 56u16;
            'l1: loop {
                if !(((i) as i32) < ((y) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((graph).wrapping_add(320)).cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .write(0u16);
                    ((((((graph).wrapping_add(320)).cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = ((((graph).wrapping_add(300)).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read();
            'l3: loop {
                if !(((i) as i32) <= ((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((((graph).wrapping_add(320)).cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .write(155u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        bottom = ((if ((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)
            >= (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(8))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
        {
            ((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)
        } else {
            (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(8))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
        }) as u16);
        {
            i = ((((bottom) as i32).wrapping_add(1i32)) as u16);
            'l5: loop {
                if !(((i) as i32) <= 121i32) {
                    break 'l5;
                }
                'l6: {
                    (((((graph).wrapping_add(320)).cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .write(0u16);
                    ((((((graph).wrapping_add(320)).cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 56u16;
            'l7: loop {
                if !(((i) as i32) <= 121i32) {
                    break 'l7;
                }
                'l8: {
                    if ((((((((graph).wrapping_add(320)).cast::<u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == 0i32)
                        && (((((((((graph).wrapping_add(320)).cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != 0i32)
                    {
                        (((((graph).wrapping_add(320)).cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_sub(56i32)) as isize * 4))
                        .cast::<u16>())
                        .write(155u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ConditionGraph_CalcLeftHalf(graph: *mut u8) {
    unsafe {
        let mut graph = graph;
        let mut i: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut bottom: i32 = 0i32;
        if ((((((graph).wrapping_add(300)).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            < (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(16))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
        {
            y = ((((((graph).wrapping_add(300)).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32);
            ConditionGraph_CalcLine(
                graph,
                (((graph).wrapping_add(584)).cast::<u8>()).cast::<u16>(),
                ((graph).wrapping_add(300)).cast::<u8>(),
                (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(16),
                0u8,
                core::ptr::null_mut(),
            );
        } else {
            y = (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(16))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32);
            ConditionGraph_CalcLine(
                graph,
                (((graph).wrapping_add(584)).cast::<u8>()).cast::<u16>(),
                (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(16),
                ((graph).wrapping_add(300)).cast::<u8>(),
                1u8,
                core::ptr::null_mut(),
            );
        }
        ConditionGraph_CalcLine(
            graph,
            (((graph).wrapping_add(584)).cast::<u8>()).cast::<u16>(),
            (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(16),
            (((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(12),
            0u8,
            core::ptr::null_mut(),
        );
        {
            i = 56i32;
            'l1: loop {
                if !(i < y) {
                    break 'l1;
                }
                'l2: {
                    (((((graph).wrapping_add(584)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .write(0u16);
                    ((((((graph).wrapping_add(584)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = ((((((graph).wrapping_add(300)).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32);
            'l3: loop {
                if !(i <= ((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    ((((((graph).wrapping_add(584)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(155u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        bottom = (if ((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)
            >= (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(12))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)
        {
            ((((graph).wrapping_add(848).cast::<u16>()).read()) as i32)
        } else {
            (((((((graph).wrapping_add(300)).cast::<u8>()).wrapping_offset(12))
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)
        });
        {
            i = bottom;
            'l5: loop {
                if !(i <= 121i32) {
                    break 'l5;
                }
                'l6: {
                    (((((graph).wrapping_add(584)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .write(0u16);
                    ((((((graph).wrapping_add(584)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(56i32)) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 66i32) {
                    break 'l7;
                }
                'l8: {
                    if (((((((graph).wrapping_add(584)).cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        >= ((((((((graph).wrapping_add(584)).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        ((((((graph).wrapping_add(584)).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .write(0u16);
                        (((((graph).wrapping_add(584)).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .cast::<u16>())
                        .write(0u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionGraph_CalcPositions(conditions: *mut u8, positions: *mut u8) {
    unsafe {
        let mut conditions = conditions;
        let mut positions = positions;
        let mut lineLength: u8 = 0u8;
        let mut sinIdx: u8 = 0u8;
        let mut posIdx: i8 = 0i8;
        let mut i: u16 = 0u16;
        lineLength = ((((&raw const sConditionToLineLength).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((({
                    let __t2 = conditions;
                    conditions = (conditions).wrapping_offset(1);
                    __t2
                })
                .read()) as i32) as isize,
            ))
        .read();
        ((positions).cast::<u16>()).write(155u16);
        ((positions).wrapping_add(2).cast::<u16>()).write(
            ((((crate::c::div_i32(177i32, 2i32)).wrapping_add(3i32))
                .wrapping_sub(((lineLength) as i32))) as u16),
        );
        sinIdx = 64u8;
        posIdx = 0i8;
        {
            i = 1u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    sinIdx = ((((sinIdx) as i32).wrapping_add(51i32)) as u8);
                    if (({
                        let __t3 = (posIdx).wrapping_sub(1);
                        posIdx = __t3;
                        __t3
                    }) as i32)
                        < 0i32
                    {
                        posIdx = 4i8;
                    }
                    if ((posIdx) as i32) == 2i32 {
                        sinIdx = (sinIdx).wrapping_add(1);
                    }
                    lineLength = ((((&raw const sConditionToLineLength).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((({
                            let __t5 = conditions;
                            conditions = (conditions).wrapping_offset(1);
                            __t5
                        })
                        .read()) as i32) as isize,
                    ))
                    .read();
                    (((positions).wrapping_offset(((posIdx) as i32) as isize * 4)).cast::<u16>())
                        .write(
                            (((155i32).wrapping_add(
                                (((lineLength) as i32).wrapping_mul(
                                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                        .wrapping_offset(
                                            ((64i32).wrapping_add(((sinIdx) as i32))) as isize,
                                        ))
                                    .read()) as i32),
                                ) >> 8),
                            )) as u16),
                        );
                    (((positions).wrapping_offset(((posIdx) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .write(
                        ((((crate::c::div_i32(177i32, 2i32)).wrapping_add(3i32)).wrapping_sub(
                            (((lineLength) as i32).wrapping_mul(
                                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                    .wrapping_offset(((sinIdx) as i32) as isize))
                                .read()) as i32),
                            ) >> 8),
                        )) as u16),
                    );
                    if (((posIdx) as i32) <= 2i32)
                        && ((((lineLength) as i32) != 32i32) || (((posIdx) as i32) != 2i32))
                    {
                        let __p6 = ((positions).wrapping_offset(((posIdx) as i32) as isize * 4))
                            .cast::<u16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMoveRelearnerWindows(useContestWindow: u8) {
    unsafe {
        let mut useContestWindow = useContestWindow;
        let mut i: u8 = 0u8;
        InitWindows(
            ((&raw const sMoveRelearnerWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 1u16, 224u8);
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            240u16,
            32u16,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(48u32, 8u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(i, 17u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((useContestWindow) != 0) {
            PutWindowTilemap(0u8);
            DrawStdFrameWithCustomTileAndPalette(0u8, 0u8, 1u16, 14u8);
        } else {
            PutWindowTilemap(1u8);
            DrawStdFrameWithCustomTileAndPalette(1u8, 0u8, 1u16, 14u8);
        }
        PutWindowTilemap(2u8);
        PutWindowTilemap(3u8);
        DrawStdFrameWithCustomTileAndPalette(2u8, 0u8, 1u16, 14u8);
        DrawStdFrameWithCustomTileAndPalette(3u8, 0u8, 1u16, 14u8);
        MoveRelearnerDummy();
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn MoveRelearnerDummy() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMoveRelearnerMovesList(items: *mut u8, numChoices: u16) -> u8 {
    unsafe {
        let mut items = items;
        let mut numChoices = numChoices;
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sMoveRelearnerMovesListTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(numChoices);
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(items);
        if ((numChoices) as i32) < 6i32 {
            (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
            .write(numChoices);
        } else {
            (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
            .write(6u16);
        }
        return (((((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn MoveRelearnerLoadBattleMoveDescription(chosenMove: u32) {
    unsafe {
        let mut chosenMove = chosenMove;
        let mut x: i32 = 0i32;
        let mut r#move: *mut u8 = core::ptr::null_mut();
        let mut buffer = crate::ffi::Align4([0u8; 32]);
        let mut str: *mut u8 = core::ptr::null_mut();
        FillWindowPixelBuffer(0u8, 17u8);
        str = (&raw mut gText_MoveRelearnerBattleMoves).cast::<u8>();
        x = GetStringCenterAlignXOffset(1i32, str, 128i32);
        AddTextPrinterParameterized(0u8, 1u8, str, ((x) as u8), 1u8, 255u8, None);
        str = (&raw mut gText_MoveRelearnerPP).cast::<u8>();
        AddTextPrinterParameterized(0u8, 1u8, str, 4u8, 41u8, 255u8, None);
        str = (&raw mut gText_MoveRelearnerPower).cast::<u8>();
        x = GetStringRightAlignXOffset(1i32, str, 106i32);
        AddTextPrinterParameterized(0u8, 1u8, str, ((x) as u8), 25u8, 255u8, None);
        str = (&raw mut gText_MoveRelearnerAccuracy).cast::<u8>();
        x = GetStringRightAlignXOffset(1i32, str, 106i32);
        AddTextPrinterParameterized(0u8, 1u8, str, ((x) as u8), 41u8, 255u8, None);
        if chosenMove == 4294967294u32 {
            CopyWindowToVram(0u8, 2u8);
            return;
        }
        r#move = ((&raw mut gBattleMoves).cast::<u8>())
            .wrapping_offset(((chosenMove) as i32) as isize * 12);
        str = (((&raw mut gTypeNames).cast::<u8>())
            .wrapping_offset(((((r#move).wrapping_add(2)).read()) as i32) as isize * 7))
        .cast::<u8>();
        AddTextPrinterParameterized(0u8, 1u8, str, 4u8, 25u8, 255u8, None);
        x = (4i32).wrapping_add(GetStringWidth(
            1u8,
            (&raw mut gText_MoveRelearnerPP).cast::<u8>(),
            0i16,
        ));
        ConvertIntToDecimalStringN(
            (&raw mut buffer).cast::<u8>(),
            ((((r#move).wrapping_add(4)).read()) as i32),
            0i32,
            2u8,
        );
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut buffer).cast::<u8>(),
            ((x) as u8),
            41u8,
            255u8,
            None,
        );
        if ((((r#move).wrapping_add(1)).read()) as i32) < 2i32 {
            str = (&raw mut gText_ThreeDashes).cast::<u8>();
        } else {
            ConvertIntToDecimalStringN(
                (&raw mut buffer).cast::<u8>(),
                ((((r#move).wrapping_add(1)).read()) as i32),
                0i32,
                3u8,
            );
            str = (&raw mut buffer).cast::<u8>();
        }
        AddTextPrinterParameterized(0u8, 1u8, str, 106u8, 25u8, 255u8, None);
        if ((((r#move).wrapping_add(3)).read()) as i32) == 0i32 {
            str = (&raw mut gText_ThreeDashes).cast::<u8>();
        } else {
            ConvertIntToDecimalStringN(
                (&raw mut buffer).cast::<u8>(),
                ((((r#move).wrapping_add(3)).read()) as i32),
                0i32,
                3u8,
            );
            str = (&raw mut buffer).cast::<u8>();
        }
        AddTextPrinterParameterized(0u8, 1u8, str, 106u8, 41u8, 255u8, None);
        str = ((((&raw mut gMoveDescriptionPointers).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset((((chosenMove).wrapping_sub(1u32)) as i32) as isize))
        .read();
        AddTextPrinterParameterized(0u8, 7u8, str, 0u8, 65u8, 0u8, None);
    }
}
pub(crate) unsafe extern "C" fn MoveRelearnerMenuLoadContestMoveDescription(chosenMove: u32) {
    unsafe {
        let mut chosenMove = chosenMove;
        let mut x: i32 = 0i32;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut r#move: *mut u8 = core::ptr::null_mut();
        MoveRelearnerShowHideHearts(((chosenMove) as i32));
        FillWindowPixelBuffer(1u8, 17u8);
        str = (&raw mut gText_MoveRelearnerContestMovesTitle).cast::<u8>();
        x = GetStringCenterAlignXOffset(1i32, str, 128i32);
        AddTextPrinterParameterized(1u8, 1u8, str, ((x) as u8), 1u8, 255u8, None);
        str = (&raw mut gText_MoveRelearnerAppeal).cast::<u8>();
        x = GetStringRightAlignXOffset(1i32, str, 92i32);
        AddTextPrinterParameterized(1u8, 1u8, str, ((x) as u8), 25u8, 255u8, None);
        str = (&raw mut gText_MoveRelearnerJam).cast::<u8>();
        x = GetStringRightAlignXOffset(1i32, str, 92i32);
        AddTextPrinterParameterized(1u8, 1u8, str, ((x) as u8), 41u8, 255u8, None);
        if chosenMove == 4294967294u32 {
            CopyWindowToVram(1u8, 2u8);
            return;
        }
        r#move = ((&raw mut gContestMoves).cast::<u8>())
            .wrapping_offset(((chosenMove) as i32) as isize * 8);
        str = ((((&raw mut gContestMoveTypeTextPointers).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(
                ((crate::c::bf_read((r#move).wrapping_add(1), 0, 3, false) as u8) as i32) as isize,
            ))
        .read();
        AddTextPrinterParameterized(1u8, 1u8, str, 4u8, 25u8, 255u8, None);
        str = ((((&raw mut gContestEffectDescriptionPointers).cast::<*mut u8>())
            .cast::<*mut u8>())
        .wrapping_offset((((r#move).read()) as i32) as isize))
        .read();
        AddTextPrinterParameterized(1u8, 7u8, str, 0u8, 65u8, 255u8, None);
        CopyWindowToVram(1u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn MoveRelearnerCursorCallback(
    itemIndex: i32,
    onInit: u8,
    list: *mut u8,
) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut onInit = onInit;
        let mut list = list;
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
        }
        MoveRelearnerLoadBattleMoveDescription(((itemIndex) as u32));
        MoveRelearnerMenuLoadContestMoveDescription(((itemIndex) as u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRelearnerPrintMessage(str: *mut u8) {
    unsafe {
        let mut str = str;
        let mut speed: u8 = 0u8;
        FillWindowPixelBuffer(3u8, 17u8);
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (1u8) as i32,
        );
        speed = GetPlayerTextSpeedDelay();
        AddTextPrinterParameterized2(3u8, 1u8, str, speed, None, 2u8, 1u8, 3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRelearnerRunTextPrinters() -> u16 {
    unsafe {
        RunTextPrinters();
        return IsTextPrinterActive(3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRelearnerCreateYesNoMenu() {
    unsafe {
        CreateYesNoMenu(
            (&raw const sMoveRelearnerYesNoMenuTemplate)
                .cast::<u8>()
                .cast_mut(),
            1u16,
            14u8,
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxOrPartyMonData(
    boxId: u16,
    monId: u16,
    request: i32,
    dst: *mut u8,
) -> i32 {
    unsafe {
        let mut boxId = boxId;
        let mut monId = monId;
        let mut request = request;
        let mut dst = dst;
        let mut ret: i32 = 0i32;
        if ((boxId) as i32) == 14i32 {
            if (request == 2i32) || (request == 7i32) {
                ret = ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    request,
                    dst,
                )) as i32);
            } else {
                ret = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    request,
                )) as i32);
            }
        } else {
            if (request == 2i32) || (request == 7i32) {
                ret = ((GetAndCopyBoxMonDataAt(((boxId) as u8), ((monId) as u8), request, dst))
                    as i32);
            } else {
                ret = ((GetBoxMonDataAt(((boxId) as u8), ((monId) as u8), request)) as i32);
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn GetConditionMenuMonString(
    dst: *mut u8,
    boxId: u16,
    monId: u16,
) -> *mut u8 {
    unsafe {
        let mut dst = dst;
        let mut boxId = boxId;
        let mut monId = monId;
        let mut r#box: u16 = 0u16;
        let mut mon: u16 = 0u16;
        let mut species: u16 = 0u16;
        let mut level: u16 = 0u16;
        let mut gender: u16 = 0u16;
        let mut boxMon: *mut u8 = core::ptr::null_mut();
        let mut str: *mut u8 = core::ptr::null_mut();
        r#box = boxId;
        mon = monId;
        ({
            let __t1 = dst;
            dst = (dst).wrapping_offset(1);
            __t1
        })
        .write(252u8);
        ({
            let __t2 = dst;
            dst = (dst).wrapping_offset(1);
            __t2
        })
        .write(4u8);
        ({
            let __t3 = dst;
            dst = (dst).wrapping_offset(1);
            __t3
        })
        .write(8u8);
        ({
            let __t4 = dst;
            dst = (dst).wrapping_offset(1);
            __t4
        })
        .write(0u8);
        ({
            let __t5 = dst;
            dst = (dst).wrapping_offset(1);
            __t5
        })
        .write(9u8);
        if (GetBoxOrPartyMonData(r#box, mon, 45i32, core::ptr::null_mut())) != 0 {
            return StringCopyPadded(dst, (&raw mut gText_EggNickname).cast::<u8>(), 0u8, 12u16);
        }
        GetBoxOrPartyMonData(r#box, mon, 2i32, dst);
        StringGet_Nickname(dst);
        species = ((GetBoxOrPartyMonData(r#box, mon, 11i32, core::ptr::null_mut())) as u16);
        if ((r#box) as i32) == 14i32 {
            level = ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((mon) as i32) as isize * 100),
                56i32,
            )) as u16);
            gender = ((GetMonGender(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((mon) as i32) as isize * 100),
            )) as u16);
        } else {
            boxMon = GetBoxedMonPtr(((r#box) as u8), ((mon) as u8));
            gender = ((GetBoxMonGender(boxMon)) as u16);
            level = ((GetLevelFromBoxMonExp(boxMon)) as u16);
        }
        if ((((species) as i32) == 29i32) || (((species) as i32) == 32i32))
            && (!((StringCompare(
                dst,
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            )) != 0))
        {
            gender = 255u16;
        }
        {
            str = dst;
            'l1: loop {
                if !((((str).read()) as i32) != 255i32) {
                    break 'l1;
                }
                'l2: {}
                str = (str).wrapping_offset(1);
            }
        }
        ({
            let __t6 = str;
            str = (str).wrapping_offset(1);
            __t6
        })
        .write(252u8);
        ({
            let __t7 = str;
            str = (str).wrapping_offset(1);
            __t7
        })
        .write(18u8);
        ({
            let __t8 = str;
            str = (str).wrapping_offset(1);
            __t8
        })
        .write(60u8);
        'l3: {
            let __sw9 = ((gender) as i32);
            let __matched = __sw9 == 0i32 || __sw9 == 254i32;
            if !__matched {
                ({
                    let __t10 = str;
                    str = (str).wrapping_offset(1);
                    __t10
                })
                .write(0u8);
                break 'l3;
            }
            if __sw9 == 0i32 {
                ({
                    let __t11 = str;
                    str = (str).wrapping_offset(1);
                    __t11
                })
                .write(252u8);
                ({
                    let __t12 = str;
                    str = (str).wrapping_offset(1);
                    __t12
                })
                .write(1u8);
                ({
                    let __t13 = str;
                    str = (str).wrapping_offset(1);
                    __t13
                })
                .write(4u8);
                ({
                    let __t14 = str;
                    str = (str).wrapping_offset(1);
                    __t14
                })
                .write(252u8);
                ({
                    let __t15 = str;
                    str = (str).wrapping_offset(1);
                    __t15
                })
                .write(3u8);
                ({
                    let __t16 = str;
                    str = (str).wrapping_offset(1);
                    __t16
                })
                .write(5u8);
                ({
                    let __t17 = str;
                    str = (str).wrapping_offset(1);
                    __t17
                })
                .write(181u8);
                break 'l3;
            }
            if __sw9 == 254i32 {
                ({
                    let __t18 = str;
                    str = (str).wrapping_offset(1);
                    __t18
                })
                .write(252u8);
                ({
                    let __t19 = str;
                    str = (str).wrapping_offset(1);
                    __t19
                })
                .write(1u8);
                ({
                    let __t20 = str;
                    str = (str).wrapping_offset(1);
                    __t20
                })
                .write(6u8);
                ({
                    let __t21 = str;
                    str = (str).wrapping_offset(1);
                    __t21
                })
                .write(252u8);
                ({
                    let __t22 = str;
                    str = (str).wrapping_offset(1);
                    __t22
                })
                .write(3u8);
                ({
                    let __t23 = str;
                    str = (str).wrapping_offset(1);
                    __t23
                })
                .write(7u8);
                ({
                    let __t24 = str;
                    str = (str).wrapping_offset(1);
                    __t24
                })
                .write(182u8);
                break 'l3;
            }
        }
        ({
            let __t25 = str;
            str = (str).wrapping_offset(1);
            __t25
        })
        .write(252u8);
        ({
            let __t26 = str;
            str = (str).wrapping_offset(1);
            __t26
        })
        .write(4u8);
        ({
            let __t27 = str;
            str = (str).wrapping_offset(1);
            __t27
        })
        .write(8u8);
        ({
            let __t28 = str;
            str = (str).wrapping_offset(1);
            __t28
        })
        .write(0u8);
        ({
            let __t29 = str;
            str = (str).wrapping_offset(1);
            __t29
        })
        .write(9u8);
        ({
            let __t30 = str;
            str = (str).wrapping_offset(1);
            __t30
        })
        .write(186u8);
        ({
            let __t31 = str;
            str = (str).wrapping_offset(1);
            __t31
        })
        .write(249u8);
        ({
            let __t32 = str;
            str = (str).wrapping_offset(1);
            __t32
        })
        .write(5u8);
        str = ConvertIntToDecimalStringN(str, ((level) as i32), 0i32, 3u8);
        ({
            let __t33 = str;
            str = (str).wrapping_offset(1);
            __t33
        })
        .write(0u8);
        (str).write(255u8);
        return str;
    }
}
pub(crate) unsafe extern "C" fn BufferConditionMenuSpacedStringN(
    dst: *mut u8,
    src: *mut u8,
    n: i16,
) -> *mut u8 {
    unsafe {
        let mut dst = dst;
        let mut src = src;
        let mut n = n;
        'l1: loop {
            if !((((src).read()) as i32) != 255i32) {
                break 'l1;
            }
            ({
                let __t1 = dst;
                dst = (dst).wrapping_offset(1);
                __t1
            })
            .write(
                ({
                    let __t3 = src;
                    src = (src).wrapping_offset(1);
                    __t3
                })
                .read(),
            );
            n = (n).wrapping_sub(1);
        }
        'l2: loop {
            if !((({
                let __t4 = n;
                n = (n).wrapping_sub(1);
                __t4
            }) as i32)
                > 0i32)
            {
                break 'l2;
            }
            ({
                let __t5 = dst;
                dst = (dst).wrapping_offset(1);
                __t5
            })
            .write(0u8);
        }
        (dst).write(255u8);
        return dst;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMenuMonNameAndLocString(
    locationDst: *mut u8,
    nameDst: *mut u8,
    boxId: u16,
    monId: u16,
    partyId: u16,
    numMons: u16,
    excludesCancel: u8,
) {
    unsafe {
        let mut locationDst = locationDst;
        let mut nameDst = nameDst;
        let mut boxId = boxId;
        let mut monId = monId;
        let mut partyId = partyId;
        let mut numMons = numMons;
        let mut excludesCancel = excludesCancel;
        let mut i: u16 = 0u16;
        let mut r#box: u16 = boxId;
        let mut mon: u16 = monId;
        if !((excludesCancel) != 0) {
            numMons = (numMons).wrapping_sub(1);
        }
        if ((partyId) as i32) != ((numMons) as i32) {
            GetConditionMenuMonString(nameDst, r#box, mon);
            (locationDst).write(252u8);
            ((locationDst).wrapping_offset(1)).write(4u8);
            ((locationDst).wrapping_offset(2)).write(8u8);
            ((locationDst).wrapping_offset(3)).write(0u8);
            ((locationDst).wrapping_offset(4)).write(9u8);
            if ((r#box) as i32) == 14i32 {
                BufferConditionMenuSpacedStringN(
                    (locationDst).wrapping_offset(5),
                    (&raw mut gText_InParty).cast::<u8>(),
                    8i16,
                );
            } else {
                BufferConditionMenuSpacedStringN(
                    (locationDst).wrapping_offset(5),
                    GetBoxNamePtr(((r#box) as u8)),
                    8i16,
                );
            }
        } else {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 12i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((nameDst).wrapping_offset(((i) as i32) as isize)).write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((nameDst).wrapping_offset(((i) as i32) as isize)).write(255u8);
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((locationDst).wrapping_offset(((i) as i32) as isize)).write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((locationDst).wrapping_offset(((i) as i32) as isize)).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMenuMonConditions(
    graph: *mut u8,
    numSparkles: *mut u8,
    boxId: u16,
    monId: u16,
    partyId: u16,
    id: u16,
    numMons: u16,
    excludesCancel: u8,
) {
    unsafe {
        let mut graph = graph;
        let mut numSparkles = numSparkles;
        let mut boxId = boxId;
        let mut monId = monId;
        let mut partyId = partyId;
        let mut id = id;
        let mut numMons = numMons;
        let mut excludesCancel = excludesCancel;
        let mut i: u16 = 0u16;
        if !((excludesCancel) != 0) {
            numMons = (numMons).wrapping_sub(1);
        }
        if ((partyId) as i32) != ((numMons) as i32) {
            ((((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5)).cast::<u8>())
                .write(((GetBoxOrPartyMonData(boxId, monId, 22i32, core::ptr::null_mut())) as u8));
            (((((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5)).cast::<u8>())
                .wrapping_offset(1))
            .write(((GetBoxOrPartyMonData(boxId, monId, 47i32, core::ptr::null_mut())) as u8));
            (((((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5)).cast::<u8>())
                .wrapping_offset(2))
            .write(((GetBoxOrPartyMonData(boxId, monId, 33i32, core::ptr::null_mut())) as u8));
            (((((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5)).cast::<u8>())
                .wrapping_offset(3))
            .write(((GetBoxOrPartyMonData(boxId, monId, 24i32, core::ptr::null_mut())) as u8));
            (((((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5)).cast::<u8>())
                .wrapping_offset(4))
            .write(((GetBoxOrPartyMonData(boxId, monId, 23i32, core::ptr::null_mut())) as u8));
            ((numSparkles).wrapping_offset(((id) as i32) as isize)).write(
                ((if GetBoxOrPartyMonData(boxId, monId, 48i32, core::ptr::null_mut()) != 255i32 {
                    crate::c::div_u32(
                        ((GetBoxOrPartyMonData(boxId, monId, 48i32, core::ptr::null_mut())) as u32),
                        (crate::c::div_u32(255u32, 9u32)).wrapping_add(1u32),
                    )
                } else {
                    9u32
                }) as u8),
            );
            ConditionGraph_CalcPositions(
                (((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5)).cast::<u8>(),
                ((((graph).wrapping_add(20)).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 20))
                .cast::<u8>(),
            );
        } else {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((graph).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 5))
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                        (((((((graph).wrapping_add(20)).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .write(155u16);
                        (((((((graph).wrapping_add(20)).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write((((crate::c::div_i32(177i32, 2i32)).wrapping_add(3i32)) as u16));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMenuMonGfx(
    tilesDst: *mut u8,
    palDst: *mut u8,
    boxId: u16,
    monId: u16,
    partyId: u16,
    numMons: u16,
    excludesCancel: u8,
) {
    unsafe {
        let mut tilesDst = tilesDst;
        let mut palDst = palDst;
        let mut boxId = boxId;
        let mut monId = monId;
        let mut partyId = partyId;
        let mut numMons = numMons;
        let mut excludesCancel = excludesCancel;
        if !((excludesCancel) != 0) {
            numMons = (numMons).wrapping_sub(1);
        }
        if ((partyId) as i32) != ((numMons) as i32) {
            let mut species: u16 =
                ((GetBoxOrPartyMonData(boxId, monId, 65i32, core::ptr::null_mut())) as u16);
            let mut trainerId: u32 =
                ((GetBoxOrPartyMonData(boxId, monId, 1i32, core::ptr::null_mut())) as u32);
            let mut personality: u32 =
                ((GetBoxOrPartyMonData(boxId, monId, 0i32, core::ptr::null_mut())) as u32);
            LoadSpecialPokePic(
                ((&raw mut gMonFrontPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                tilesDst,
                ((species) as i32),
                personality,
                1u8,
            );
            LZ77UnCompWram(
                GetMonSpritePalFromSpeciesAndPersonality(species, trainerId, personality),
                palDst,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveConditionMonOnscreen(x: *mut i16) -> u8 {
    unsafe {
        let mut x = x;
        (x).write((((((x).read()) as i32).wrapping_add(24i32)) as i16));
        if (((x).read()) as i32) > 0i32 {
            (x).write(0i16);
        }
        return (((((x).read()) as i32) != 0i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveConditionMonOffscreen(x: *mut i16) -> u8 {
    unsafe {
        let mut x = x;
        (x).write((((((x).read()) as i32).wrapping_sub(24i32)) as i16));
        if (((x).read()) as i32) < (-80i32) {
            (x).write((-80i16));
        }
        return (((((x).read()) as i32) != (-80i32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionMenu_UpdateMonEnter(graph: *mut u8, x: *mut i16) -> u8 {
    unsafe {
        let mut graph = graph;
        let mut x = x;
        let mut graphUpdating: u8 = ConditionGraph_TryUpdate(graph);
        let mut monUpdating: u8 = MoveConditionMonOnscreen(x);
        return ((((graphUpdating) != 0) || ((monUpdating) != 0)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConditionMenu_UpdateMonExit(graph: *mut u8, x: *mut i16) -> u8 {
    unsafe {
        let mut graph = graph;
        let mut x = x;
        let mut graphUpdating: u8 = ConditionGraph_TryUpdate(graph);
        let mut monUpdating: u8 = MoveConditionMonOffscreen(x);
        return ((((graphUpdating) != 0) || ((monUpdating) != 0)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadConditionMonPicTemplate(
    sheet: *mut u8,
    template: *mut u8,
    pal: *mut u8,
) {
    unsafe {
        let mut sheet = sheet;
        let mut template = template;
        let mut pal = pal;
        let mut dataSheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut dataSheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut dataSheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(((crate::c::div_i32(4096i32, 2i32)) as u16));
        (&raw mut dataSheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(100u16);
        let mut dataTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(100u16);
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(100u16);
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sOam_ConditionMonPic).cast::<u8>().cast_mut());
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        let mut dataPal = crate::ffi::Align4([0u8; 8]);
        (&raw mut dataPal)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(core::ptr::null_mut());
        (&raw mut dataPal)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(100u16);
        sheet.cast::<crate::c::Rec4<8>>().write_unaligned(
            (&raw mut dataSheet)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
        );
        template.cast::<crate::c::Rec4<24>>().write_unaligned(
            (&raw mut dataTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .read_unaligned(),
        );
        pal.cast::<crate::c::Rec4<8>>().write_unaligned(
            (&raw mut dataPal)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadConditionSelectionIcons(
    sheets: *mut u8,
    template: *mut u8,
    pals: *mut u8,
) {
    unsafe {
        let mut sheets = sheets;
        let mut template = template;
        let mut pals = pals;
        let mut i: u8 = 0u8;
        let mut dataSheets = crate::ffi::Align4([0u8; 32]);
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                (((&raw const sConditionPokeball_Gfx)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u32>())
                .cast::<u32>())
                .cast::<u8>(),
            );
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(256u16);
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(6)
            .cast::<u16>()
            .write(101u16);
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                (((&raw const sConditionPokeballPlaceholder_Gfx)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u32>())
                .cast::<u32>())
                .cast::<u8>(),
            );
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(32u16);
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(6)
            .cast::<u16>()
            .write(103u16);
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write((&raw mut gPokenavConditionCancel_Gfx).cast::<u8>());
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(4)
            .cast::<u16>()
            .write(256u16);
        (&raw mut dataSheets)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(6)
            .cast::<u16>()
            .write(102u16);
        let mut dataPals = crate::ffi::Align4([0u8; 24]);
        (&raw mut dataPals)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(((&raw mut gPokenavConditionCancel_Pal).cast::<u16>()).cast::<u16>());
        (&raw mut dataPals)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(101u16);
        (&raw mut dataPals)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                (((&raw mut gPokenavConditionCancel_Pal).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(16),
            );
        (&raw mut dataPals)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(102u16);
        let mut dataTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(101u16);
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(101u16);
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write(
                (&raw const sOam_ConditionSelectionIcon)
                    .cast::<u8>()
                    .cast_mut(),
            );
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnims_ConditionSelectionIcon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut dataTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        let __t1 = sheets;
                        sheets = (sheets).wrapping_offset(8);
                        __t1
                    }
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        ((&raw mut dataSheets).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8)
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        template.cast::<crate::c::Rec4<24>>().write_unaligned(
            (&raw mut dataTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .read_unaligned(),
        );
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(24u32, 8u32)) {
                    break 'l3;
                }
                'l4: {
                    {
                        let __t2 = pals;
                        pals = (pals).wrapping_offset(8);
                        __t2
                    }
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        ((&raw mut dataPals).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8)
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadConditionSparkle(sheet: *mut u8, pal: *mut u8) {
    unsafe {
        let mut sheet = sheet;
        let mut pal = pal;
        let mut dataSheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut dataSheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                (((&raw const sConditionSparkle_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u32>())
                .cast::<u32>())
                .cast::<u8>(),
            );
        (&raw mut dataSheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(896u16);
        (&raw mut dataSheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(104u16);
        let mut dataPal = crate::ffi::Align4([0u8; 8]);
        (&raw mut dataPal)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sConditionSparkle_Gfx)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut dataPal)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(104u16);
        sheet.cast::<crate::c::Rec4<8>>().write_unaligned(
            (&raw mut dataSheet)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
        );
        pal.cast::<crate::c::Rec4<8>>().write_unaligned(
            (&raw mut dataPal)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ConditionSparkle_DoNextAfterDelay(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 60i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            SetNextConditionSparkle(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ConditionSparkle_WaitForAllAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ConditionSparkle_DoNextAfterDelay));
        }
    }
}
pub(crate) unsafe extern "C" fn SetConditionSparklePosition(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut mon: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 68,
        );
        if ((mon) as usize) != 0usize {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((mon).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((mon).wrapping_add(36).cast::<i16>()).read()) as i32)))
                .wrapping_add(
                    (((((((&raw const sConditionSparkleCoords).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
                    ))
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((((((mon).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((mon).wrapping_add(38).cast::<i16>()).read()) as i32)))
                .wrapping_add(
                    ((((((((&raw const sConditionSparkleCoords).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
                    ))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((((&raw const sConditionSparkleCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
                ))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(40i32)) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((((&raw const sConditionSparkleCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
                ))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_add(104i32)) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InitConditionSparkles(
    count: u8,
    allowFirstShowAll: u8,
    sprites: *mut *mut u8,
) {
    unsafe {
        let mut count = count;
        let mut allowFirstShowAll = allowFirstShowAll;
        let mut sprites = sprites;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((sprites).wrapping_offset(((i) as i32) as isize)).read()) as usize)
                        != 0usize
                    {
                        (((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .write(((i) as i16));
                        ((((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write((((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as i16));
                        ((((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(((count) as i16));
                        ((((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(((i) as i16));
                        if (!((allowFirstShowAll) != 0)) || (((count) as i32) != 9i32) {
                            ((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_ConditionSparkle));
                        } else {
                            SetConditionSparklePosition(
                                ((sprites).wrapping_offset(((i) as i32) as isize)).read(),
                            );
                            ShowAllConditionSparkles(
                                ((sprites).wrapping_offset(((i) as i32) as isize)).read(),
                            );
                            ((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_ConditionSparkle_WaitForAllAnim));
                            crate::c::bf_write(
                                (((sprites).wrapping_offset(((i) as i32) as isize)).read())
                                    .wrapping_add(62),
                                2,
                                1,
                                (0u16) as i32,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetNextConditionSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u16 = 0u16;
        let mut id: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((((((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_mul(16i32))
                        .wrapping_add(1i32)) as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ConditionSparkle));
                    id = ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetConditionSparkleSprites(sprites: *mut *mut u8) {
    unsafe {
        let mut sprites = sprites;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    ((sprites).wrapping_offset(((i) as i32) as isize)).write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateConditionSparkleSprites(
    sprites: *mut *mut u8,
    monSpriteId: u8,
    _count: u8,
) {
    unsafe {
        let mut sprites = sprites;
        let mut monSpriteId = monSpriteId;
        let mut _count = _count;
        let mut i: u16 = 0u16;
        let mut spriteId: u16 = 0u16;
        let mut firstSpriteId: u16 = 0u16;
        let mut count: u8 = _count;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32).wrapping_add(1i32)) {
                    break 'l1;
                }
                'l2: {
                    spriteId = ((CreateSprite(
                        (&raw const sSpriteTemplate_ConditionSparkle)
                            .cast::<u8>()
                            .cast_mut(),
                        0i16,
                        0i16,
                        0u8,
                    )) as u16);
                    if ((spriteId) as i32) != 64i32 {
                        ((sprites).wrapping_offset(((i) as i32) as isize)).write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        crate::c::bf_write(
                            (((sprites).wrapping_offset(((i) as i32) as isize)).read())
                                .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        ((((((sprites).wrapping_offset(((i) as i32) as isize)).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(((monSpriteId) as i16));
                        if ((i) as i32) != 0i32 {
                            ((((((sprites)
                                .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                            .read())
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(5))
                            .write(((spriteId) as i16));
                        } else {
                            firstSpriteId = spriteId;
                        }
                    } else {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((sprites).wrapping_offset(((count) as i32) as isize)).read()).wrapping_add(46))
            .cast::<i16>())
        .wrapping_offset(5))
        .write(((firstSpriteId) as i16));
        InitConditionSparkles(count, 1u8, sprites);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyConditionSparkleSprites(sprites: *mut *mut u8) {
    unsafe {
        let mut sprites = sprites;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((sprites).wrapping_offset(((i) as i32) as isize)).read()) as usize)
                        != 0usize
                    {
                        DestroySprite(((sprites).wrapping_offset(((i) as i32) as isize)).read());
                        ((sprites).wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                    } else {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeConditionSparkles(sprites: *mut *mut u8) {
    unsafe {
        let mut sprites = sprites;
        DestroyConditionSparkleSprites(sprites);
        FreeSpriteTilesByTag(104u16);
        FreeSpritePaletteByTag(104u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ConditionSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 0i32
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                != 0i32
            {
                return;
            }
            SeekSpriteAnim(sprite, 0u8);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
        SetConditionSparklePosition(sprite);
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
            {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    == 9i32
                {
                    ShowAllConditionSparkles(sprite);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ConditionSparkle_WaitForAllAnim));
                } else {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ConditionSparkle_DoNextAfterDelay));
                }
            } else {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowAllConditionSparkles(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        let mut id: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    SeekSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 68),
                        0u8,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    id = ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawLevelUpWindowPg1(
    windowId: u16,
    statsBefore: *mut u16,
    statsAfter: *mut u16,
    bgClr: u8,
    fgClr: u8,
    shadowClr: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut statsBefore = statsBefore;
        let mut statsAfter = statsAfter;
        let mut bgClr = bgClr;
        let mut fgClr = fgClr;
        let mut shadowClr = shadowClr;
        let mut i: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut statsDiff = crate::ffi::Align4([0u8; 12]);
        let mut text = crate::ffi::Align4([0u8; 12]);
        let mut color = crate::ffi::Align4([0u8; 3]);
        FillWindowPixelBuffer(
            ((windowId) as u8),
            ((((bgClr) as i32) | (((bgClr) as i32) << 4)) as u8),
        );
        ((&raw mut statsDiff).cast::<i16>()).write(
            (((((statsAfter).read()) as i32).wrapping_sub((((statsBefore).read()) as i32))) as i16),
        );
        (((&raw mut statsDiff).cast::<i16>()).wrapping_offset(1)).write(
            ((((((statsAfter).wrapping_offset(1)).read()) as i32)
                .wrapping_sub(((((statsBefore).wrapping_offset(1)).read()) as i32)))
                as i16),
        );
        (((&raw mut statsDiff).cast::<i16>()).wrapping_offset(2)).write(
            ((((((statsAfter).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(((((statsBefore).wrapping_offset(2)).read()) as i32)))
                as i16),
        );
        (((&raw mut statsDiff).cast::<i16>()).wrapping_offset(3)).write(
            ((((((statsAfter).wrapping_offset(4)).read()) as i32)
                .wrapping_sub(((((statsBefore).wrapping_offset(4)).read()) as i32)))
                as i16),
        );
        (((&raw mut statsDiff).cast::<i16>()).wrapping_offset(4)).write(
            ((((((statsAfter).wrapping_offset(5)).read()) as i32)
                .wrapping_sub(((((statsBefore).wrapping_offset(5)).read()) as i32)))
                as i16),
        );
        (((&raw mut statsDiff).cast::<i16>()).wrapping_offset(5)).write(
            ((((((statsAfter).wrapping_offset(3)).read()) as i32)
                .wrapping_sub(((((statsBefore).wrapping_offset(3)).read()) as i32)))
                as i16),
        );
        ((&raw mut color).cast::<u8>()).write(bgClr);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(fgClr);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(shadowClr);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        1u8,
                        0u8,
                        (((15i32).wrapping_mul(((i) as i32))) as u8),
                        (&raw mut color).cast::<u8>(),
                        (-1i8),
                        ((((&raw const sLvlUpStatStrings)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    StringCopy(
                        (&raw mut text).cast::<u8>(),
                        (if (((((&raw mut statsDiff).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            >= 0i32
                        {
                            (&raw mut gText_Plus).cast::<u8>()
                        } else {
                            (&raw mut gText_Dash).cast::<u8>()
                        }),
                    );
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        1u8,
                        56u8,
                        (((15i32).wrapping_mul(((i) as i32))) as u8),
                        (&raw mut color).cast::<u8>(),
                        (-1i8),
                        (&raw mut text).cast::<u8>(),
                    );
                    if (if (((((&raw mut statsDiff).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        < 0i32
                    {
                        (((((&raw mut statsDiff).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_neg()
                    } else {
                        (((((&raw mut statsDiff).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    }) <= 9i32
                    {
                        x = 18u16;
                    } else {
                        x = 12u16;
                    }
                    ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        (if (((((&raw mut statsDiff).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            < 0i32
                        {
                            (((((&raw mut statsDiff).cast::<i16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_neg()
                        } else {
                            (((((&raw mut statsDiff).cast::<i16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                        }),
                        0i32,
                        2u8,
                    );
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        1u8,
                        (((56i32).wrapping_add(((x) as i32))) as u8),
                        (((15i32).wrapping_mul(((i) as i32))) as u8),
                        (&raw mut color).cast::<u8>(),
                        (-1i8),
                        (&raw mut text).cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawLevelUpWindowPg2(
    windowId: u16,
    currStats: *mut u16,
    bgClr: u8,
    fgClr: u8,
    shadowClr: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut currStats = currStats;
        let mut bgClr = bgClr;
        let mut fgClr = fgClr;
        let mut shadowClr = shadowClr;
        let mut i: u16 = 0u16;
        let mut numDigits: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut stats = crate::ffi::Align4([0u8; 12]);
        let mut text = crate::ffi::Align4([0u8; 12]);
        let mut color = crate::ffi::Align4([0u8; 3]);
        FillWindowPixelBuffer(
            ((windowId) as u8),
            ((((bgClr) as i32) | (((bgClr) as i32) << 4)) as u8),
        );
        ((&raw mut stats).cast::<i16>()).write((((currStats).read()) as i16));
        (((&raw mut stats).cast::<i16>()).wrapping_offset(1))
            .write(((((currStats).wrapping_offset(1)).read()) as i16));
        (((&raw mut stats).cast::<i16>()).wrapping_offset(2))
            .write(((((currStats).wrapping_offset(2)).read()) as i16));
        (((&raw mut stats).cast::<i16>()).wrapping_offset(3))
            .write(((((currStats).wrapping_offset(4)).read()) as i16));
        (((&raw mut stats).cast::<i16>()).wrapping_offset(4))
            .write(((((currStats).wrapping_offset(5)).read()) as i16));
        (((&raw mut stats).cast::<i16>()).wrapping_offset(5))
            .write(((((currStats).wrapping_offset(3)).read()) as i16));
        ((&raw mut color).cast::<u8>()).write(bgClr);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(fgClr);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(shadowClr);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut stats).cast::<i16>()).wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                        > 99i32
                    {
                        numDigits = 3u16;
                    } else {
                        if (((((&raw mut stats).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            > 9i32
                        {
                            numDigits = 2u16;
                        } else {
                            numDigits = 1u16;
                        }
                    }
                    ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        (((((&raw mut stats).cast::<i16>()).wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        0i32,
                        ((numDigits) as u8),
                    );
                    x = (((6i32).wrapping_mul((4i32).wrapping_sub(((numDigits) as i32)))) as u16);
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        1u8,
                        0u8,
                        (((15i32).wrapping_mul(((i) as i32))) as u8),
                        (&raw mut color).cast::<u8>(),
                        (-1i8),
                        ((((&raw const sLvlUpStatStrings)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    AddTextPrinterParameterized3(
                        ((windowId) as u8),
                        1u8,
                        (((56i32).wrapping_add(((x) as i32))) as u8),
                        (((15i32).wrapping_mul(((i) as i32))) as u8),
                        (&raw mut color).cast::<u8>(),
                        (-1i8),
                        (&raw mut text).cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonLevelUpWindowStats(mon: *mut u8, currStats: *mut u16) {
    unsafe {
        let mut mon = mon;
        let mut currStats = currStats;
        (currStats).write(((GetMonData2(mon, 58i32)) as u16));
        ((currStats).wrapping_offset(1)).write(((GetMonData2(mon, 59i32)) as u16));
        ((currStats).wrapping_offset(2)).write(((GetMonData2(mon, 60i32)) as u16));
        ((currStats).wrapping_offset(3)).write(((GetMonData2(mon, 61i32)) as u16));
        ((currStats).wrapping_offset(4)).write(((GetMonData2(mon, 62i32)) as u16));
        ((currStats).wrapping_offset(5)).write(((GetMonData2(mon, 63i32)) as u16));
    }
}
