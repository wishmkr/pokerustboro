//! The Mauville trader: four decorations offered for swap, each remembering
//! who last traded it in.

use crate::decoration::gCurDecorationItems;
use crate::decoration_inventory::{
    DecorationAdd, DecorationRemove, GetFirstEmptyDecorSlot, GetNumOwnedDecorationsInCategory,
};
use crate::ffi::{
    AddTextPrinterParameterized, AddWindow, ClearStdWindowAndFrameToTransparent,
    ClearWindowTilemap, ConvertInternationalString, CreateTask, DestroyTask,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, PlaySE, RemoveWindow, StringCopy, TaskFunc,
    WindowTemplate, gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result,
    gStringVar1, gStringVar2, gStringVar3, set_task_data, task_data,
};
use core::ffi::c_int;

const NUM_TRADER_ITEMS: usize = 4;
const MAUVILLE_MAN_TRADER: u8 = 2;
const NUM_DECORATIONS: u16 = 120;
const DECORCAT_COUNT: u8 = 8;
const GAME_LANGUAGE: u8 = 2;

const DECOR_DUSKULL_DOLL: u8 = 91;
const DECOR_BALL_CUSHION: u8 = 107;
const DECOR_TIRE: u8 = 37;
const DECOR_PRETTY_FLOWERS: u8 = 21;

const MENU_NOTHING_CHOSEN: i8 = -2;
const MENU_B_PRESSED: i8 = -1;
const SE_SELECT: u16 = 5;
const TEXT_SKIP_DRAW: u8 = 0xff;

/// `offsetof(struct SaveBlock1, oldMan)`. `oldMan` is a union and `trader`
/// is its first member, so this is the trader's address too.
const SAVE1_TRADER_OFFSET: usize = 0x2e28;

/// `struct MauvilleOldManTrader`, 56 bytes.
const TRADER_ID: usize = 0;
const TRADER_DECORATIONS: usize = 1;
const TRADER_PLAYER_NAMES: usize = 5;
const TRADER_PLAYER_NAME_STRIDE: usize = 11;
const TRADER_ALREADY_TRADED: usize = 49;
const TRADER_LANGUAGE: usize = 50;

/// `struct Decoration`, 32 bytes.
const DECORATION_STRIDE: usize = 32;
const DECORATION_NAME: usize = 1;
const DECORATION_CATEGORY: usize = 19;

/// The window id lives in task data slot 3.
const T_WINDOW_ID: usize = 3;

/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *const u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `ConvertPixelWidthToTileWidth` with this module's view of its types.
#[inline]
unsafe fn ConvertPixelWidthToTileWidth(a0: c_int) -> c_int {
    unsafe { crate::script_menu::ConvertPixelWidthToTileWidth(a0) }
}
/// `InitMenuInUpperLeftCornerNormal` with this module's view of its types.
#[inline]
unsafe fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8 {
    unsafe { crate::menu::InitMenuInUpperLeftCornerNormal(a0, a1, a2) }
}
/// `Menu_ProcessInput` with this module's view of its types.
#[inline]
unsafe fn Menu_ProcessInput() -> i8 {
    unsafe { crate::menu::Menu_ProcessInput() }
}
/// `ScheduleBgCopyTilemapToVram` with this module's view of its types.
#[inline]
unsafe fn ScheduleBgCopyTilemapToVram(a0: u8) {
    unsafe {
        crate::menu::ScheduleBgCopyTilemapToVram(a0);
    }
}
/// `ScriptContext_Enable` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_Enable() {
    unsafe {
        crate::script::ScriptContext_Enable();
    }
}
/// `CopyDecorationCategoryName` with this module's view of its types.
#[inline]
unsafe fn CopyDecorationCategoryName(a0: *mut u8, a1: u8) {
    unsafe {
        crate::decoration::CopyDecorationCategoryName(a0 as _, a1);
    }
}
/// `ShowDecorationCategoriesWindow` with this module's view of its types.
#[inline]
unsafe fn ShowDecorationCategoriesWindow(a0: u8) {
    unsafe {
        crate::decoration::ShowDecorationCategoriesWindow(a0);
    }
}
/// `IsSelectedDecorInThePC` with this module's view of its types.
#[inline]
unsafe fn IsSelectedDecorInThePC() -> u8 {
    unsafe { crate::decoration::IsSelectedDecorInThePC() }
}

/// `&gSaveBlock1Ptr->oldMan.trader`
#[inline]
unsafe fn trader() -> *mut u8 {
    unsafe {
        (*(&raw const crate::load_save::gSaveBlock1Ptr)
            .cast::<*mut u8>()
            .cast_mut())
        .add(SAVE1_TRADER_OFFSET)
    }
}

#[inline]
unsafe fn trader_decoration(index: usize) -> u8 {
    unsafe { trader().add(TRADER_DECORATIONS + index).read_volatile() }
}

#[inline]
unsafe fn player_name(index: usize) -> *mut u8 {
    unsafe { trader().add(TRADER_PLAYER_NAMES + index * TRADER_PLAYER_NAME_STRIDE) }
}

/// `&gDecorations[id]`
#[inline]
unsafe fn decoration(id: u16) -> *const u8 {
    unsafe {
        (&raw const (*(&raw const crate::data::decoration::gDecorations).cast::<u8>()))
            .add(id as usize * DECORATION_STRIDE)
    }
}

#[inline]
unsafe fn decoration_name(id: u16) -> *const u8 {
    unsafe { decoration(id).add(DECORATION_NAME) }
}

#[inline]
unsafe fn decoration_category(id: u16) -> u8 {
    unsafe { decoration(id).add(DECORATION_CATEGORY).read_volatile() }
}

#[inline]
unsafe fn window_id(task_id: u8) -> u8 {
    let stored = unsafe { task_data(task_id, T_WINDOW_ID) };
    stored as u8
}

#[unsafe(no_mangle)]
pub unsafe fn TraderSetup() {
    let trader = unsafe { trader() };
    unsafe { trader.add(TRADER_ID).write_volatile(MAUVILLE_MAN_TRADER) };
    unsafe { trader.add(TRADER_ALREADY_TRADED).write_volatile(0) };

    let names = [
        &raw const (*(&raw const crate::data::strings::gText_Tristan).cast::<u8>()),
        &raw const (*(&raw const crate::data::strings::gText_Philip).cast::<u8>()),
        &raw const (*(&raw const crate::data::strings::gText_Dennis).cast::<u8>()),
        &raw const (*(&raw const crate::data::strings::gText_Roberto).cast::<u8>()),
    ];
    let decorations = [
        DECOR_DUSKULL_DOLL,
        DECOR_BALL_CUSHION,
        DECOR_TIRE,
        DECOR_PRETTY_FLOWERS,
    ];

    for i in 0..NUM_TRADER_ITEMS {
        let _ = unsafe { StringCopy(player_name(i), names[i]) };
        unsafe {
            trader
                .add(TRADER_DECORATIONS + i)
                .write_volatile(decorations[i])
        };
        unsafe {
            trader
                .add(TRADER_LANGUAGE + i)
                .write_volatile(GAME_LANGUAGE)
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn Trader_ResetFlag() {
    unsafe { trader().add(TRADER_ALREADY_TRADED).write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe fn CreateAvailableDecorationsMenu(task_id: u8) {
    let mut template = WindowTemplate {
        bg: 0,
        tilemap_left: 1,
        tilemap_top: 1,
        width: 10,
        height: 10,
        palette_num: 15,
        base_block: 1,
    };

    // The window is only as wide as its widest entry.
    let five_marks_width = unsafe {
        GetStringWidth(
            FONT_NORMAL,
            &raw const (*(&raw const crate::data::strings::gText_FiveMarks).cast::<u8>()),
            0,
        )
    };
    let mut widest = unsafe {
        GetStringWidth(
            FONT_NORMAL,
            &raw const (*(&raw const crate::data::strings::gText_Exit).cast::<u8>()),
            0,
        )
    };
    for i in 0..NUM_TRADER_ITEMS {
        let id = u16::from(unsafe { trader_decoration(i) });
        let width = if id > NUM_DECORATIONS {
            five_marks_width
        } else {
            unsafe { GetStringWidth(FONT_NORMAL, decoration_name(id), 0) }
        };
        if width > widest {
            widest = width;
        }
    }
    template.width = unsafe { ConvertPixelWidthToTileWidth(widest) } as u8;

    let window = unsafe { AddWindow(&raw const template) } as u8;
    unsafe { set_task_data(task_id, T_WINDOW_ID, i16::from(window)) };
    unsafe { DrawStdFrameWithCustomTileAndPalette(window, 0, 0x214, 14) };

    for i in 0..NUM_TRADER_ITEMS {
        let id = u16::from(unsafe { trader_decoration(i) });
        // An id past the table means the slot has never been traded into.
        let text = if id > NUM_DECORATIONS {
            &raw const (*(&raw const crate::data::strings::gText_FiveMarks).cast::<u8>())
        } else {
            unsafe { decoration_name(id) }
        };
        let _ = unsafe {
            AddTextPrinterParameterized(
                window,
                FONT_NORMAL,
                text,
                8,
                (16 * i + 1) as u8,
                TEXT_SKIP_DRAW,
                None,
            )
        };
    }
    let _ = unsafe {
        AddTextPrinterParameterized(
            window,
            FONT_NORMAL,
            &raw const (*(&raw const crate::data::strings::gText_Exit).cast::<u8>()),
            8,
            (16 * NUM_TRADER_ITEMS + 1) as u8,
            TEXT_SKIP_DRAW,
            None,
        )
    };

    unsafe { InitMenuInUpperLeftCornerNormal(window, NUM_TRADER_ITEMS as u8 + 1, 0) };
    unsafe { ScheduleBgCopyTilemapToVram(0) };
}

#[unsafe(no_mangle)]
pub unsafe fn Task_BufferDecorSelectionAndCloseWindow(task_id: u8, decoration_id: u8) {
    let selected = if u16::from(decoration_id) > NUM_DECORATIONS {
        0xffff
    } else {
        u16::from(decoration_id)
    };
    unsafe { (&raw mut gSpecialVar_0x8004).write_volatile(selected) };

    let window = unsafe { window_id(task_id) };
    unsafe { ClearStdWindowAndFrameToTransparent(window, 0) };
    unsafe { ClearWindowTilemap(window) };
    unsafe { RemoveWindow(window) };
    unsafe { ScheduleBgCopyTilemapToVram(0) };
    unsafe { DestroyTask(task_id) };
    unsafe { ScriptContext_Enable() };
}

#[unsafe(no_mangle)]
pub unsafe fn Task_HandleGetDecorationMenuInput(task_id: u8) {
    let input = unsafe { Menu_ProcessInput() };

    if input == MENU_NOTHING_CHOSEN {
        return;
    }

    // The entry past the four decorations is the Exit line.
    if input == MENU_B_PRESSED || input == NUM_TRADER_ITEMS as i8 {
        unsafe { PlaySE(SE_SELECT) };
        unsafe { Task_BufferDecorSelectionAndCloseWindow(task_id, 0) };
        return;
    }

    unsafe { PlaySE(SE_SELECT) };
    let slot = input as usize;
    unsafe { (&raw mut gSpecialVar_0x8005).write_volatile(input as u16) };
    let _ = unsafe { StringCopy((&raw mut gStringVar1).cast::<u8>(), player_name(slot)) };
    let language = unsafe { trader().add(TRADER_LANGUAGE + slot).read_volatile() };
    unsafe { ConvertInternationalString((&raw mut gStringVar1).cast::<u8>(), language) };
    unsafe { Task_BufferDecorSelectionAndCloseWindow(task_id, trader_decoration(slot)) };
}

#[unsafe(no_mangle)]
pub unsafe fn GetTraderTradedFlag() {
    let traded = unsafe { trader().add(TRADER_ALREADY_TRADED).read_volatile() };
    unsafe { (&raw mut gSpecialVar_Result).write_volatile(u16::from(traded)) };
}

#[unsafe(no_mangle)]
pub unsafe fn DoesPlayerHaveNoDecorations() {
    for category in 0..DECORCAT_COUNT {
        if GetNumOwnedDecorationsInCategory(category) != 0 {
            unsafe { (&raw mut gSpecialVar_Result).write_volatile(0) };
            return;
        }
    }
    unsafe { (&raw mut gSpecialVar_Result).write_volatile(1) };
}

#[unsafe(no_mangle)]
pub unsafe fn IsDecorationCategoryFull() {
    unsafe { (&raw mut gSpecialVar_Result).write_volatile(0) };

    let offered = unsafe { (&raw const gSpecialVar_0x8004).read_volatile() };
    let owned = unsafe { (&raw const gSpecialVar_0x8006).read_volatile() };
    let offered_category = unsafe { decoration_category(offered) };

    // Only a problem when the trade would move the decoration into a
    // different, already full category.
    if offered_category != unsafe { decoration_category(owned) }
        && GetFirstEmptyDecorSlot(offered_category) == -1
    {
        unsafe {
            CopyDecorationCategoryName((&raw mut gStringVar2).cast::<u8>(), offered_category)
        };
        unsafe { (&raw mut gSpecialVar_Result).write_volatile(1) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn TraderShowDecorationMenu() {
    let callback: TaskFunc = ShowDecorationCategoriesWindow;
    unsafe { CreateTask(callback, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn DecorationItemsMenuAction_Trade(task_id: u8) {
    if unsafe { IsSelectedDecorInThePC() } == 1 {
        let index = unsafe {
            (&raw const (*crate::decoration::gCurDecorationIndex.as_ptr().cast::<u8>()))
                .read_volatile()
        } as usize;
        let owned = unsafe { gCurDecorationItems.add(index).read_volatile() };
        unsafe { (&raw mut gSpecialVar_0x8006).write_volatile(u16::from(owned)) };

        let offered = unsafe { (&raw const gSpecialVar_0x8004).read_volatile() };
        let _ = unsafe {
            StringCopy(
                (&raw mut gStringVar3).cast::<u8>(),
                decoration_name(offered),
            )
        };
        let _ = unsafe {
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                decoration_name(u16::from(owned)),
            )
        };
    } else {
        unsafe { (&raw mut gSpecialVar_0x8006).write_volatile(0xffff) };
    }

    unsafe { DestroyTask(task_id) };
    unsafe { ScriptContext_Enable() };
}

#[unsafe(no_mangle)]
pub unsafe fn ExitTraderMenu(task_id: u8) {
    unsafe { (&raw mut gSpecialVar_0x8006).write_volatile(0) };
    unsafe { DestroyTask(task_id) };
    unsafe { ScriptContext_Enable() };
}

#[unsafe(no_mangle)]
pub unsafe fn TraderDoDecorationTrade() {
    let offered = unsafe { (&raw const gSpecialVar_0x8004).read_volatile() };
    let owned = unsafe { (&raw const gSpecialVar_0x8006).read_volatile() };
    let slot = unsafe { (&raw const gSpecialVar_0x8005).read_volatile() } as usize;

    let _ = DecorationRemove(owned as u8);
    let _ = DecorationAdd(offered as u8);

    // The trader now remembers the player as the previous owner.
    let trader = unsafe { trader() };
    let _ = unsafe {
        StringCopy(
            player_name(slot),
            *(&raw const crate::load_save::gSaveBlock2Ptr)
                .cast::<*mut u8>()
                .cast_mut(),
        )
    };
    unsafe {
        trader
            .add(TRADER_DECORATIONS + slot)
            .write_volatile(owned as u8)
    };
    unsafe {
        trader
            .add(TRADER_LANGUAGE + slot)
            .write_volatile(GAME_LANGUAGE)
    };
    unsafe { trader.add(TRADER_ALREADY_TRADED).write_volatile(1) };
}

#[unsafe(no_mangle)]
pub unsafe fn TraderMenuGetDecoration() {
    let task_id = unsafe { CreateTask(Task_HandleGetDecorationMenuInput, 0) };
    unsafe { CreateAvailableDecorationsMenu(task_id) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trader_field_offsets_match_the_arm_structure() {
        assert_eq!(TRADER_ID, 0);
        assert_eq!(TRADER_DECORATIONS, 1);
        assert_eq!(TRADER_DECORATIONS + NUM_TRADER_ITEMS, TRADER_PLAYER_NAMES);
        assert_eq!(
            TRADER_PLAYER_NAMES + NUM_TRADER_ITEMS * TRADER_PLAYER_NAME_STRIDE,
            TRADER_ALREADY_TRADED
        );
        assert_eq!(TRADER_ALREADY_TRADED + 1, TRADER_LANGUAGE);
        // 54 used bytes, rounded to 56 by the APCS structure-size boundary.
        assert_eq!(TRADER_LANGUAGE + NUM_TRADER_ITEMS, 54);
    }

    #[test]
    fn an_untraded_slot_reads_back_as_the_empty_marker() {
        // Ids above the decoration table print as five marks and buffer
        // 0xFFFF rather than a real decoration.
        let selected = |id: u8| {
            if u16::from(id) > NUM_DECORATIONS {
                0xffffu16
            } else {
                u16::from(id)
            }
        };
        assert_eq!(selected(0), 0);
        assert_eq!(selected(NUM_DECORATIONS as u8), NUM_DECORATIONS);
        assert_eq!(selected(121), 0xffff);
    }
}
