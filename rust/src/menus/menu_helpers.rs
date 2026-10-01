//! Translated from `src/menu_helpers.c` by tools/rustport/c2rs.py.
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
    clippy::int_plus_one,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::bg::{ChangeBgX, ChangeBgY};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::link::{IsLinkRecvQueueAtOverworldMax, gReceivedRemoteLinkPlayers};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::mail_data::ItemIsMail;
use crate::menu::{
    AddTextPrinterParameterized2, CreateYesNoMenu, DrawDialogFrameWithCustomTileAndPalette,
    Menu_ProcessInputNoWrapClearOnChoose,
};
use crate::overworld::{IsOverworldLinkActive, Overworld_IsRecvQueueAtMax};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::string_util::gStringVar4;
use crate::task::task_set_func;
use crate::text::{IsTextPrinterActive, RunTextPrinters};
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::InUnionRoom;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `DestroySpriteAndFreeResources` with this module's view of its types.
#[inline]
unsafe fn DestroySpriteAndFreeResources(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySpriteAndFreeResources(a0 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
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
pub(crate) static sMessageWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static mut sMessageNextTask: Option<unsafe fn(u8)> = None;

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn ResetVramOamAndBgCntRegs() {
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
pub unsafe fn ResetAllBgsCoordinates() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
}
pub unsafe fn SetVBlankHBlankCallbacksToNull() {
    SetVBlankCallback(None);
    SetHBlankCallback(None);
}
pub unsafe fn DisplayMessageAndContinueTask(
    taskId: u8,
    windowId: u8,
    tileNum: u16,
    paletteNum: u8,
    fontId: u8,
    textSpeed: u8,
    string: *mut u8,
    taskFunc: *mut c_void,
) {
    sMessageWindowId.set(windowId);
    DrawDialogFrameWithCustomTileAndPalette(windowId, TRUE, tileNum, paletteNum);
    if string != gStringVar4.as_mut_ptr() {
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), string);
    }
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(1);
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
    sMessageNextTask = core::mem::transmute::<_, Option<unsafe fn(u8)>>(taskFunc);
    task_set_func(taskId, Some(Task_ContinueTaskAfterMessagePrints));
}
pub unsafe fn RunTextPrintersRetIsActive(textPrinterId: u8) -> u16 {
    RunTextPrinters();
    IsTextPrinterActive(textPrinterId)
}
pub(crate) unsafe fn Task_ContinueTaskAfterMessagePrints(taskId: u8) {
    if RunTextPrintersRetIsActive(sMessageWindowId.get()) == 0 {
        sMessageNextTask.unwrap_unchecked()(taskId);
    }
}
pub unsafe fn DoYesNoFuncWithChoice(taskId: u8, data: *mut YesNoFuncTable) {
    sYesNo = *data;
    task_set_func(taskId, Some(Task_CallYesOrNoCallback));
}
pub unsafe fn CreateYesNoMenuWithCallbacks(
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
    task_set_func(taskId, Some(Task_CallYesOrNoCallback));
}
pub(crate) unsafe fn Task_CallYesOrNoCallback(taskId: u8) {
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
pub unsafe fn AdjustQuantityAccordingToDPadInput(quantity: *mut i16, max: u16) -> u8 {
    let valBefore: i16 = *quantity;
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
    FALSE
}
pub unsafe fn GetLRKeysPressed() -> u8 {
    if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
        if gMain.newKeys as i32 & L_BUTTON != 0 {
            return MENU_L_PRESSED;
        }
        if gMain.newKeys as i32 & R_BUTTON != 0 {
            return MENU_R_PRESSED;
        }
    }
    0
}
pub unsafe fn GetLRKeysPressedAndHeld() -> u8 {
    if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
        if gMain.newAndRepeatedKeys as i32 & L_BUTTON != 0 {
            return MENU_L_PRESSED;
        }
        if gMain.newAndRepeatedKeys as i32 & R_BUTTON != 0 {
            return MENU_R_PRESSED;
        }
    }
    0
}
pub unsafe fn IsHoldingItemAllowed(itemId: u16) -> u8 {
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
        0
    }
}
pub unsafe fn IsWritingMailAllowed(itemId: u16) -> u8 {
    if (IsOverworldLinkActive() == TRUE as u32 || InUnionRoom() == TRUE as u32)
        && ItemIsMail(itemId) == TRUE
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn MenuHelpers_IsLinkActive() -> u8 {
    if IsOverworldLinkActive() == 1 || gReceivedRemoteLinkPlayers == 1 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsActiveOverworldLinkBusy() -> u8 {
    if MenuHelpers_IsLinkActive() == 0 {
        return FALSE;
    } else {
        return Overworld_IsRecvQueueAtMax() as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn MenuHelpers_ShouldWaitForLinkRecv() -> u8 {
    if IsActiveOverworldLinkBusy() == TRUE || IsLinkRecvQueueAtOverworldMax() == TRUE as u32 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn SetItemListPerPageCount(
    slots: *mut ItemSlot,
    slotsCount: u8,
    pageItems: *mut u8,
    totalItems: *mut u8,
    maxPerPage: u8,
) {
    let slots_: *mut ItemSlot = slots;
    *totalItems = 0;
    for i in 0..(slotsCount as u16) {
        if (*slots_.at(i)).itemId != ITEM_NONE {
            *totalItems += 1;
        }
    }
    *totalItems += 1;
    if *totalItems > maxPerPage {
        *pageItems = maxPerPage;
    } else {
        *pageItems = *totalItems;
    }
}
pub unsafe fn SetCursorWithinListBounds(
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
pub unsafe fn SetCursorScrollWithinListBounds(
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
pub unsafe fn LoadListMenuSwapLineGfx() {
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_SwapLine).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_SwapLine).cast_mut());
}
pub unsafe fn CreateSwapLineSprites(spriteIds: *mut u8, count: u8) {
    for i in 0..count {
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
    }
}
pub unsafe fn DestroySwapLineSprites(spriteIds: *mut u8, count: u8) {
    for i in 0..count {
        if i as i32 == count as i32 - 1 {
            DestroySpriteAndFreeResources(&raw mut gSprites[*spriteIds.at(i)]);
        } else {
            DestroySprite(&raw mut gSprites[*spriteIds.at(i)]);
        }
    }
}
pub unsafe fn SetSwapLineSpritesInvisibility(spriteIds: *mut u8, count: u8, invisible: u8) {
    for i in 0..count {
        gSprites[*spriteIds.at(i)].set_invisible(invisible as u16);
    }
}
pub unsafe fn UpdateSwapLineSpritesPos(spriteIds: *mut u8, mut count: u8, x: i16, y: u16) {
    let hasMargin: u8 = count & SWAP_LINE_HAS_MARGIN;
    count &= 127;
    for i in 0..count {
        if i as i32 == count as i32 - 1 && hasMargin != 0 {
            gSprites[*spriteIds.at(i)].x2 = x - 8;
        } else {
            gSprites[*spriteIds.at(i)].x2 = x;
        }
        gSprites[*spriteIds.at(i)].y = 1 + y as i16;
    }
}
