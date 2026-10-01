//! Translated from `src/menu_helpers.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sOamData_SwapLine sAnim_SwapLine_RightArrow sAnim_SwapLine_Line sAnim_SwapLine_LeftArrow sAnims_SwapLine sSpriteSheet_SwapLine sSpritePalette_SwapLine sSpriteTemplate_SwapLine

static sSpritePalette_SwapLine: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::menu_helpers::sSpritePalette_SwapLine).cast());
static sSpriteSheet_SwapLine: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::menu_helpers::sSpriteSheet_SwapLine).cast());
static sSpriteTemplate_SwapLine: Table<SpriteTemplate> =
    Table((&raw const crate::data::menu_helpers::sSpriteTemplate_SwapLine).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sYesNo: YesNoFuncTable = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMessageWindowId: u8 = 0;
pub(crate) static mut sMessageNextTask: Option<unsafe extern "C" fn(u8)> = None;

unsafe extern "C" {
    static mut gMain: Main;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndFreeResources(a0: *mut Sprite);
    fn DrawDialogFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn InUnionRoom() -> u32;
    fn IsLinkRecvQueueAtOverworldMax() -> u32;
    fn IsOverworldLinkActive() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn ItemIsMail(a0: u16) -> u8;
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn Overworld_IsRecvQueueAtMax() -> u32;
    fn PlaySE(a0: u16);
    fn RunTextPrinters();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetVramOamAndBgCntRegs() {
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                VRAM as usize as *mut c_void,
                0x100c000,
            );
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                OAM as i32 as usize as *mut c_void,
                0x5000100,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                PLTT as i32 as usize as *mut c_void,
                0x1000200,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllBgsCoordinates() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetVBlankHBlankCallbacksToNull() {
    SetVBlankCallback(None);
    SetHBlankCallback(None);
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
    taskFunc: *mut c_void,
) {
    sMessageWindowId = windowId;
    DrawDialogFrameWithCustomTileAndPalette(windowId, TRUE, tileNum, paletteNum);
    if string != gStringVar4.as_mut_ptr() {
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), string);
    }
    gTextFlags.set_canABSpeedUpPrint(1);
    AddTextPrinterParameterized2(
        windowId,
        fontId,
        gStringVar4.as_mut_ptr(),
        textSpeed,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    sMessageNextTask = core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(taskFunc);
    gTasks[taskId].func = Some(Task_ContinueTaskAfterMessagePrints);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunTextPrintersRetIsActive(textPrinterId: u8) -> u16 {
    RunTextPrinters();
    return IsTextPrinterActive(textPrinterId);
}
pub(crate) unsafe extern "C" fn Task_ContinueTaskAfterMessagePrints(taskId: u8) {
    if RunTextPrintersRetIsActive(sMessageWindowId) == 0 {
        sMessageNextTask.unwrap_unchecked()(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoYesNoFuncWithChoice(taskId: u8, data: *mut YesNoFuncTable) {
    sYesNo = *data;
    gTasks[taskId].func = Some(Task_CallYesOrNoCallback);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateYesNoMenuWithCallbacks(
    taskId: u8,
    template: *mut WindowTemplate,
    unused1: u8,
    unused2: u8,
    unused3: u8,
    tileStart: u16,
    palette: u8,
    yesNo: *mut YesNoFuncTable,
) {
    CreateYesNoMenu(template, tileStart, palette, 0);
    sYesNo = *yesNo;
    gTasks[taskId].func = Some(Task_CallYesOrNoCallback);
}
pub(crate) unsafe extern "C" fn Task_CallYesOrNoCallback(taskId: u8) {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PlaySE(SE_SELECT);
            sYesNo.yesFunc.unwrap_unchecked()(taskId);
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            sYesNo.noFunc.unwrap_unchecked()(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdjustQuantityAccordingToDPadInput(quantity: *mut i16, max: u16) -> u8 {
    let mut valBefore: i16 = *quantity;
    if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_UP {
        *quantity += 1;
        if *quantity as i32 > max as i32 {
            *quantity = 1;
        }
        if *quantity == valBefore {
            return FALSE;
        } else {
            PlaySE(SE_SELECT);
            return TRUE;
        }
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_DOWN {
        *quantity -= 1;
        if *quantity <= 0 {
            *quantity = max as i16;
        }
        if *quantity == valBefore {
            return FALSE;
        } else {
            PlaySE(SE_SELECT);
            return TRUE;
        }
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_RIGHT {
        *quantity += 10;
        if *quantity as i32 > max as i32 {
            *quantity = max as i16;
        }
        if *quantity == valBefore {
            return FALSE;
        } else {
            PlaySE(SE_SELECT);
            return TRUE;
        }
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_LEFT {
        *quantity -= 10;
        if *quantity <= 0 {
            *quantity = 1;
        }
        if *quantity == valBefore {
            return FALSE;
        } else {
            PlaySE(SE_SELECT);
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLRKeysPressed() -> u8 {
    if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
        if gMain.newKeys as i32 & L_BUTTON != 0 {
            return MENU_L_PRESSED;
        }
        if gMain.newKeys as i32 & R_BUTTON != 0 {
            return MENU_R_PRESSED;
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLRKeysPressedAndHeld() -> u8 {
    if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
        if gMain.newAndRepeatedKeys as i32 & L_BUTTON != 0 {
            return MENU_L_PRESSED;
        }
        if gMain.newAndRepeatedKeys as i32 & R_BUTTON != 0 {
            return MENU_R_PRESSED;
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsHoldingItemAllowed(itemId: u16) -> u8 {
    if itemId == ITEM_ENIGMA_BERRY
        && ((*gSaveBlock1Ptr).location.mapGroup == 25 && (*gSaveBlock1Ptr).location.mapNum == 25
            || InUnionRoom() == TRUE as u32)
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWritingMailAllowed(itemId: u16) -> u8 {
    if (IsOverworldLinkActive() == TRUE as u32 || InUnionRoom() == TRUE as u32)
        && ItemIsMail(itemId) == TRUE
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MenuHelpers_IsLinkActive() -> u8 {
    if IsOverworldLinkActive() == 1 || gReceivedRemoteLinkPlayers == 1 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsActiveOverworldLinkBusy() -> u8 {
    if MenuHelpers_IsLinkActive() == 0 {
        return FALSE;
    } else {
        return Overworld_IsRecvQueueAtMax() as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MenuHelpers_ShouldWaitForLinkRecv() -> u8 {
    if IsActiveOverworldLinkBusy() == TRUE || IsLinkRecvQueueAtOverworldMax() == TRUE as u32 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetItemListPerPageCount(
    slots: *mut ItemSlot,
    slotsCount: u8,
    pageItems: *mut u8,
    totalItems: *mut u8,
    maxPerPage: u8,
) {
    let mut i: u16 = 0;
    let mut slots_: *mut ItemSlot = slots;
    *totalItems = 0;
    i = 0;
    while i < slotsCount as u16 {
        if (*slots_.at(i)).itemId != ITEM_NONE {
            *totalItems += 1;
        }
        i += 1;
    }
    *totalItems += 1;
    if *totalItems > maxPerPage {
        *pageItems = maxPerPage;
    } else {
        *pageItems = *totalItems;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCursorWithinListBounds(
    scrollOffset: *mut u16,
    cursorPos: *mut u16,
    maxShownItems: u8,
    totalItems: u8,
) {
    if *scrollOffset != 0 && *scrollOffset as i32 + maxShownItems as i32 > totalItems as i32 {
        *scrollOffset = totalItems as u16 - maxShownItems as u16;
    }
    if *scrollOffset as i32 + *cursorPos as i32 >= totalItems as i32 {
        if totalItems == 0 {
            *cursorPos = 0;
        } else {
            *cursorPos = totalItems as u16 - 1;
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
    let mut i: u8 = 0;
    if maxShownItems as i32 % 2 != 0 {
        if *cursorPos as i32 >= maxShownItems as i32 / 2 {
            i = 0;
            while (i as i32) < *cursorPos as i32 - maxShownItems as i32 / 2 {
                if *scrollOffset as i32 + shownItems as i32 == totalItems as i32 {
                    break;
                }
                *cursorPos -= 1;
                *scrollOffset += 1;
                i += 1;
            }
        }
    } else {
        if *cursorPos as i32 >= maxShownItems as i32 / 2 + 1 {
            i = 0;
            while i as i32 <= *cursorPos as i32 - maxShownItems as i32 / 2 {
                if *scrollOffset as i32 + shownItems as i32 == totalItems as i32 {
                    break;
                }
                *cursorPos -= 1;
                *scrollOffset += 1;
                i += 1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadListMenuSwapLineGfx() {
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_SwapLine).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_SwapLine).cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSwapLineSprites(mut spriteIds: *mut u8, count: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < count {
        *spriteIds.at(i) = CreateSprite(
            (&raw const *sSpriteTemplate_SwapLine).cast_mut(),
            i as i16 * 16,
            0,
            0,
        );
        if i != 0 {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], 1);
        }
        gSprites[*spriteIds.at(i)].set_invisible(TRUE as u16);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroySwapLineSprites(spriteIds: *mut u8, count: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < count {
        if i as i32 == count as i32 - 1 {
            DestroySpriteAndFreeResources(&raw mut gSprites[*spriteIds.at(i)]);
        } else {
            DestroySprite(&raw mut gSprites[*spriteIds.at(i)]);
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSwapLineSpritesInvisibility(
    spriteIds: *mut u8,
    count: u8,
    invisible: u8,
) {
    let mut i: u8 = 0;
    i = 0;
    while i < count {
        gSprites[*spriteIds.at(i)].set_invisible(invisible as u16);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSwapLineSpritesPos(
    spriteIds: *mut u8,
    mut count: u8,
    x: i16,
    y: u16,
) {
    let mut i: u8 = 0;
    let mut hasMargin: u8 = count & SWAP_LINE_HAS_MARGIN;
    count &= 127;
    i = 0;
    while i < count {
        if i as i32 == count as i32 - 1 && hasMargin != 0 {
            gSprites[*spriteIds.at(i)].x2 = x - 8;
        } else {
            gSprites[*spriteIds.at(i)].x2 = x;
        }
        gSprites[*spriteIds.at(i)].y = 1 + y as i16;
        i += 1;
    }
}
