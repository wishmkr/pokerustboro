//! Translated from `src/menu_helpers.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sOamData_SwapLine sAnim_SwapLine_RightArrow sAnim_SwapLine_Line sAnim_SwapLine_LeftArrow sAnims_SwapLine sSpriteSheet_SwapLine sSpritePalette_SwapLine sSpriteTemplate_SwapLine
#[allow(unused_imports)]
use crate::data::menu_helpers::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sYesNo: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMessageWindowId: u8 = 0u8;
pub(crate) static mut sMessageNextTask: Option<unsafe extern "C" fn(u8)> = None;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
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
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DrawDialogFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn InUnionRoom() -> u32;
    fn IsLinkRecvQueueAtOverworldMax() -> u32;
    fn IsOverworldLinkActive() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn ItemIsMail(a0: u16) -> u8;
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn Overworld_IsRecvQueueAtMax() -> u32;
    fn PlaySE(a0: u16);
    fn RunTextPrinters();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetVramOamAndBgCntRegs() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        SetGpuReg(14u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(98304i32, crate::c::div_i32(16i32, 8i32))
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
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((117440512i32) as usize as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
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
                    (&raw mut tmp).write_volatile(0u16);
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((83886080i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(1024i32, crate::c::div_i32(16i32, 8i32))
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
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllBgsCoordinates() {
    unsafe {
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetVBlankHBlankCallbacksToNull() {
    unsafe {
        SetVBlankCallback(None);
        SetHBlankCallback(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayMessageAndContinueTask(
    taskId: u8,
    windowId: u8,
    tileNum: u16,
    paletteNum: u8,
    fontId: u8,
    textSpeed: u8,
    string: *mut u8,
    taskFunc: *mut u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut windowId = windowId;
        let mut tileNum = tileNum;
        let mut paletteNum = paletteNum;
        let mut fontId = fontId;
        let mut textSpeed = textSpeed;
        let mut string = string;
        let mut taskFunc = taskFunc;
        ((&raw mut sMessageWindowId).cast::<u8>().cast::<u8>()).write(windowId);
        DrawDialogFrameWithCustomTileAndPalette(windowId, 1u8, tileNum, paletteNum);
        if ((string) as usize) != (((&raw mut gStringVar4).cast::<u8>()) as usize) {
            StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), string);
        }
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (1u8) as i32,
        );
        AddTextPrinterParameterized2(
            windowId,
            fontId,
            (&raw mut gStringVar4).cast::<u8>(),
            textSpeed,
            None,
            2u8,
            1u8,
            3u8,
        );
        ((&raw mut sMessageNextTask)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
            taskFunc,
        ));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ContinueTaskAfterMessagePrints));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunTextPrintersRetIsActive(textPrinterId: u8) -> u16 {
    unsafe {
        let mut textPrinterId = textPrinterId;
        RunTextPrinters();
        return IsTextPrinterActive(textPrinterId);
    }
}
pub(crate) unsafe extern "C" fn Task_ContinueTaskAfterMessagePrints(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersRetIsActive(
            ((&raw mut sMessageWindowId).cast::<u8>().cast::<u8>()).read(),
        )) != 0)
        {
            (((&raw mut sMessageNextTask)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read())
            .unwrap_unchecked()(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoYesNoFuncWithChoice(taskId: u8, data: *mut u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data = data;
        (&raw mut sYesNo)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(data.cast::<crate::c::Rec4<8>>().read_unaligned());
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CallYesOrNoCallback));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateYesNoMenuWithCallbacks(
    taskId: u8,
    template: *mut u8,
    unused1: u8,
    unused2: u8,
    unused3: u8,
    tileStart: u16,
    palette: u8,
    yesNo: *mut u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut template = template;
        let mut unused1 = unused1;
        let mut unused2 = unused2;
        let mut unused3 = unused3;
        let mut tileStart = tileStart;
        let mut palette = palette;
        let mut yesNo = yesNo;
        CreateYesNoMenu(template, tileStart, palette, 0u8);
        (&raw mut sYesNo)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(yesNo.cast::<crate::c::Rec4<8>>().read_unaligned());
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CallYesOrNoCallback));
    }
}
pub(crate) unsafe extern "C" fn Task_CallYesOrNoCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                PlaySE(5u16);
                ((((&raw mut sYesNo).cast::<u8>()).cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == (-1i32) {
                PlaySE(5u16);
                ((((&raw mut sYesNo).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdjustQuantityAccordingToDPadInput(quantity: *mut i16, max: u16) -> u8 {
    unsafe {
        let mut quantity = quantity;
        let mut max = max;
        let mut valBefore: i16 = (quantity).read();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 240i32)
            == 64i32
        {
            (quantity).write(((quantity).read()).wrapping_add(1));
            if (((quantity).read()) as i32) > ((max) as i32) {
                (quantity).write(1i16);
            }
            if (((quantity).read()) as i32) == ((valBefore) as i32) {
                return 0u8;
            } else {
                PlaySE(5u16);
                return 1u8;
            }
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 240i32)
                == 128i32
            {
                (quantity).write(((quantity).read()).wrapping_sub(1));
                if (((quantity).read()) as i32) <= 0i32 {
                    (quantity).write(((max) as i16));
                }
                if (((quantity).read()) as i32) == ((valBefore) as i32) {
                    return 0u8;
                } else {
                    PlaySE(5u16);
                    return 1u8;
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 240i32)
                    == 16i32
                {
                    (quantity).write((((((quantity).read()) as i32).wrapping_add(10i32)) as i16));
                    if (((quantity).read()) as i32) > ((max) as i32) {
                        (quantity).write(((max) as i16));
                    }
                    if (((quantity).read()) as i32) == ((valBefore) as i32) {
                        return 0u8;
                    } else {
                        PlaySE(5u16);
                        return 1u8;
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 240i32)
                        == 32i32
                    {
                        (quantity)
                            .write((((((quantity).read()) as i32).wrapping_sub(10i32)) as i16));
                        if (((quantity).read()) as i32) <= 0i32 {
                            (quantity).write(1i16);
                        }
                        if (((quantity).read()) as i32) == ((valBefore) as i32) {
                            return 0u8;
                        } else {
                            PlaySE(5u16);
                            return 1u8;
                        }
                    }
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLRKeysPressed() -> u8 {
    unsafe {
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).read())
            as i32)
            == 1i32
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 512i32)
                != 0
            {
                return 1u8;
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 256i32)
                != 0
            {
                return 2u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLRKeysPressedAndHeld() -> u8 {
    unsafe {
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).read())
            as i32)
            == 1i32
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 512i32)
                != 0
            {
                return 1u8;
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 256i32)
                != 0
            {
                return 2u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsHoldingItemAllowed(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        if (((itemId) as i32) == 175i32)
            && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 25i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 25i32))
                || (InUnionRoom() == 1u32))
        {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWritingMailAllowed(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        if ((IsOverworldLinkActive() == 1u32) || (InUnionRoom() == 1u32))
            && (((ItemIsMail(itemId)) as i32) == 1i32)
        {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MenuHelpers_IsLinkActive() -> u8 {
    unsafe {
        if (IsOverworldLinkActive() == 1u32)
            || (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32)
        {
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
pub(crate) unsafe extern "C" fn IsActiveOverworldLinkBusy() -> u8 {
    unsafe {
        if !((MenuHelpers_IsLinkActive()) != 0) {
            return 0u8;
        } else {
            return ((Overworld_IsRecvQueueAtMax()) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MenuHelpers_ShouldWaitForLinkRecv() -> u8 {
    unsafe {
        if (((IsActiveOverworldLinkBusy()) as i32) == 1i32)
            || (IsLinkRecvQueueAtOverworldMax() == 1u32)
        {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetItemListPerPageCount(
    slots: *mut u8,
    slotsCount: u8,
    pageItems: *mut u8,
    totalItems: *mut u8,
    maxPerPage: u8,
) {
    unsafe {
        let mut slots = slots;
        let mut slotsCount = slotsCount;
        let mut pageItems = pageItems;
        let mut totalItems = totalItems;
        let mut maxPerPage = maxPerPage;
        let mut i: u16 = 0u16;
        let mut slots_: *mut u8 = slots;
        (totalItems).write(0u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((slotsCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((slots_).wrapping_offset(((i) as i32) as isize * 4)).cast::<u16>())
                        .read()) as i32)
                        != 0i32
                    {
                        (totalItems).write(((totalItems).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (totalItems).write(((totalItems).read()).wrapping_add(1));
        if (((totalItems).read()) as i32) > ((maxPerPage) as i32) {
            (pageItems).write(maxPerPage);
        } else {
            (pageItems).write((totalItems).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCursorWithinListBounds(
    scrollOffset: *mut u16,
    cursorPos: *mut u16,
    maxShownItems: u8,
    totalItems: u8,
) {
    unsafe {
        let mut scrollOffset = scrollOffset;
        let mut cursorPos = cursorPos;
        let mut maxShownItems = maxShownItems;
        let mut totalItems = totalItems;
        if ((((scrollOffset).read()) as i32) != 0i32)
            && ((((scrollOffset).read()) as i32).wrapping_add(((maxShownItems) as i32))
                > ((totalItems) as i32))
        {
            (scrollOffset)
                .write(((((totalItems) as i32).wrapping_sub(((maxShownItems) as i32))) as u16));
        }
        if (((scrollOffset).read()) as i32).wrapping_add((((cursorPos).read()) as i32))
            >= ((totalItems) as i32)
        {
            if ((totalItems) as i32) == 0i32 {
                (cursorPos).write(0u16);
            } else {
                (cursorPos).write(((((totalItems) as i32).wrapping_sub(1i32)) as u16));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCursorScrollWithinListBounds(
    scrollOffset: *mut u16,
    cursorPos: *mut u16,
    shownItems: u8,
    totalItems: u8,
    maxShownItems: u8,
) {
    unsafe {
        let mut scrollOffset = scrollOffset;
        let mut cursorPos = cursorPos;
        let mut shownItems = shownItems;
        let mut totalItems = totalItems;
        let mut maxShownItems = maxShownItems;
        let mut i: u8 = 0u8;
        if crate::c::rem_i32(((maxShownItems) as i32), 2i32) != 0i32 {
            if (((cursorPos).read()) as i32) >= crate::c::div_i32(((maxShownItems) as i32), 2i32) {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32)
                            < (((cursorPos).read()) as i32)
                                .wrapping_sub(crate::c::div_i32(((maxShownItems) as i32), 2i32)))
                        {
                            break 'l1;
                        }
                        'l2: {
                            if (((scrollOffset).read()) as i32).wrapping_add(((shownItems) as i32))
                                == ((totalItems) as i32)
                            {
                                break 'l1;
                            }
                            (cursorPos).write(((cursorPos).read()).wrapping_sub(1));
                            (scrollOffset).write(((scrollOffset).read()).wrapping_add(1));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        } else {
            if (((cursorPos).read()) as i32)
                >= (crate::c::div_i32(((maxShownItems) as i32), 2i32)).wrapping_add(1i32)
            {
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32)
                            <= (((cursorPos).read()) as i32)
                                .wrapping_sub(crate::c::div_i32(((maxShownItems) as i32), 2i32)))
                        {
                            break 'l3;
                        }
                        'l4: {
                            if (((scrollOffset).read()) as i32).wrapping_add(((shownItems) as i32))
                                == ((totalItems) as i32)
                            {
                                break 'l3;
                            }
                            (cursorPos).write(((cursorPos).read()).wrapping_sub(1));
                            (scrollOffset).write(((scrollOffset).read()).wrapping_add(1));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadListMenuSwapLineGfx() {
    unsafe {
        LoadCompressedSpriteSheet((&raw const sSpriteSheet_SwapLine).cast::<u8>().cast_mut());
        LoadCompressedSpritePalette((&raw const sSpritePalette_SwapLine).cast::<u8>().cast_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSwapLineSprites(spriteIds: *mut u8, count: u8) {
    unsafe {
        let mut spriteIds = spriteIds;
        let mut count = count;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((spriteIds).wrapping_offset(((i) as i32) as isize)).write(CreateSprite(
                        (&raw const sSpriteTemplate_SwapLine)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((i) as i32).wrapping_mul(16i32)) as i16),
                        0i16,
                        0u8,
                    ));
                    if ((i) as i32) != 0i32 {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read())
                                    as i32) as isize
                                    * 68,
                            ),
                            1u8,
                        );
                    }
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroySwapLineSprites(spriteIds: *mut u8, count: u8) {
    unsafe {
        let mut spriteIds = spriteIds;
        let mut count = count;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32) == ((count) as i32).wrapping_sub(1i32) {
                        DestroySpriteAndFreeResources(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read())
                                    as i32) as isize
                                    * 68,
                            ),
                        );
                    } else {
                        DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSwapLineSpritesInvisibility(
    spriteIds: *mut u8,
    count: u8,
    invisible: u8,
) {
    unsafe {
        let mut spriteIds = spriteIds;
        let mut count = count;
        let mut invisible = invisible;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSwapLineSpritesPos(spriteIds: *mut u8, count: u8, x: i16, y: u16) {
    unsafe {
        let mut spriteIds = spriteIds;
        let mut count = count;
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        let mut hasMargin: u8 = ((((count) as i32) & 128i32) as u8);
        count = ((((count) as i32) & (-129i32)) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((i) as i32) == ((count) as i32).wrapping_sub(1i32)) && ((hasMargin) != 0) {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(((((x) as i32).wrapping_sub(8i32)) as i16));
                    } else {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(x);
                    }
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write((((1i32).wrapping_add(((y) as i32))) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
