//! Translated from `src/pokenav_list.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    unused_assignments,
    unused_labels,
    unused_variables
)]

use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, GetBgY,
    IsDma3ManagerBusyWithBgCopy,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::menu::{AddTextPrinterParameterized3, BgDmaFill};
use crate::pokenav::{AllocSubstruct, FreePokenavSubstruct, GetSubstructPtr, IsLoopedTaskActive};
use crate::pokenav_main_menu::Pokenav_AllocAndLoadPalettes;
use crate::pokenav_match_call_gfx::ClearRematchPokeballIcon;
use crate::pokenav_match_call_list::GetMatchCallFlavorText;
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowRectToVram, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    GetWindowAttribute, PutWindowTilemap, RemoveWindow,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CreateLoopedTask` with this module's view of its types.
#[inline]
unsafe fn CreateLoopedTask(a0: Option<unsafe fn(i32) -> u32>, a1: u32) -> u32 {
    unsafe { crate::pokenav::CreateLoopedTask(a0, a1) }
}
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
/// `FillWindowTilesByRow` with this module's view of its types.
#[inline]
unsafe fn FillWindowTilesByRow(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) {
    unsafe {
        crate::international_string_util::FillWindowTilesByRow(a0, a1, a2, a3, a4);
    }
}
/// `FuncIsActiveLoopedTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveLoopedTask(a0: Option<unsafe fn(i32) -> u32>) -> u32 {
    unsafe { crate::pokenav::FuncIsActiveLoopedTask(a0) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const sTimer: usize = 0;
const sOffset: usize = 1;
const sInvisible: usize = 7;
// Data tables (translate with cdata.py): sListArrow_Pal sListArrow_Gfx sListArrowSpriteSheets sListArrowPalettes sOamData_RightArrow sSpriteTemplate_RightArrow sOamData_UpDownArrow sSpriteTemplate_UpDownArrow lineOffsets.0

/// `struct PokenavList`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokenavList {
    pub listWindow: PokenavListMenuWindow,
    pub printStart: u32,
    pub printIndex: u32,
    pub itemSize: u32,
    pub listPtr: *mut core::ffi::c_void,
    pub startBgY: i32,
    pub endBgY: i32,
    pub moveListWindowLoopedTaskId: u32,
    pub moveDelta: i32,
    pub bgMoveType: u32,
    pub bufferItemFunc: Option<unsafe fn(*mut PokenavListItem, *mut u8)>,
    pub iconDrawFunc: Option<unsafe fn(u16, u32, u32)>,
    pub rightArrow: *mut Sprite,
    pub upArrow: *mut Sprite,
    pub downArrow: *mut Sprite,
    pub itemTextBuffer: CArray<u8, 64>,
    pub tilemapBuffer: CArray<u8, 2048>,
    pub windowState: PokenavListWindowState,
    pub eraseIndex: i32,
    pub loopedTaskId: u32,
}

unsafe impl Sync for PokenavList {}

/// `struct PokenavListWindowState`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokenavListWindowState {
    pub windowTopIndex: u16,
    pub listLength: u16,
    pub entriesOffscreen: u16,
    pub selectedIndexOffset: u16,
    pub entriesOnscreen: u16,
    pub listItemSize: u32,
    pub listPtr: *mut core::ffi::c_void,
}

unsafe impl Sync for PokenavListWindowState {}

/// `struct PokenavListMenuWindow`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokenavListMenuWindow {
    pub bg: u8,
    pub fillValue: u8,
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub fontId: u8,
    pub tileOffset: u16,
    pub windowId: u16,
    pub unkA: u16,
    pub numPrinted: u16,
    pub numToPrint: u16,
}

unsafe impl Sync for PokenavListMenuWindow {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokenavList>() == 2212);
    assert!(offset_of!(PokenavList, listWindow) == 0);
    assert!(offset_of!(PokenavList, printStart) == 16);
    assert!(offset_of!(PokenavList, printIndex) == 20);
    assert!(offset_of!(PokenavList, itemSize) == 24);
    assert!(offset_of!(PokenavList, listPtr) == 28);
    assert!(offset_of!(PokenavList, startBgY) == 32);
    assert!(offset_of!(PokenavList, endBgY) == 36);
    assert!(offset_of!(PokenavList, moveListWindowLoopedTaskId) == 40);
    assert!(offset_of!(PokenavList, moveDelta) == 44);
    assert!(offset_of!(PokenavList, bgMoveType) == 48);
    assert!(offset_of!(PokenavList, bufferItemFunc) == 52);
    assert!(offset_of!(PokenavList, iconDrawFunc) == 56);
    assert!(offset_of!(PokenavList, rightArrow) == 60);
    assert!(offset_of!(PokenavList, upArrow) == 64);
    assert!(offset_of!(PokenavList, downArrow) == 68);
    assert!(offset_of!(PokenavList, itemTextBuffer) == 72);
    assert!(offset_of!(PokenavList, tilemapBuffer) == 136);
    assert!(offset_of!(PokenavList, windowState) == 2184);
    assert!(offset_of!(PokenavList, eraseIndex) == 2204);
    assert!(offset_of!(PokenavList, loopedTaskId) == 2208);
    assert!(size_of::<PokenavListWindowState>() == 20);
    assert!(offset_of!(PokenavListWindowState, windowTopIndex) == 0);
    assert!(offset_of!(PokenavListWindowState, listLength) == 2);
    assert!(offset_of!(PokenavListWindowState, entriesOffscreen) == 4);
    assert!(offset_of!(PokenavListWindowState, selectedIndexOffset) == 6);
    assert!(offset_of!(PokenavListWindowState, entriesOnscreen) == 8);
    assert!(offset_of!(PokenavListWindowState, listItemSize) == 12);
    assert!(offset_of!(PokenavListWindowState, listPtr) == 16);
    assert!(size_of::<PokenavListMenuWindow>() == 16);
    assert!(offset_of!(PokenavListMenuWindow, bg) == 0);
    assert!(offset_of!(PokenavListMenuWindow, fillValue) == 1);
    assert!(offset_of!(PokenavListMenuWindow, x) == 2);
    assert!(offset_of!(PokenavListMenuWindow, y) == 3);
    assert!(offset_of!(PokenavListMenuWindow, width) == 4);
    assert!(offset_of!(PokenavListMenuWindow, fontId) == 5);
    assert!(offset_of!(PokenavListMenuWindow, tileOffset) == 6);
    assert!(offset_of!(PokenavListMenuWindow, windowId) == 8);
    assert!(offset_of!(PokenavListMenuWindow, unkA) == 10);
    assert!(offset_of!(PokenavListMenuWindow, numPrinted) == 12);
    assert!(offset_of!(PokenavListMenuWindow, numToPrint) == 14);
};

const GFXTAG_ARROW: u16 = 10;
const PALTAG_ARROW: u16 = 20;

static lineOffsets_0: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::pokenav_list::lineOffsets_0).cast());
static sListArrowPalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokenav_list::sListArrowPalettes).cast());
static sListArrowSpriteSheets: Table<CArray<CompressedSpriteSheet, 1>> =
    Table((&raw const crate::data::pokenav_list::sListArrowSpriteSheets).cast());
static sSpriteTemplate_RightArrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_list::sSpriteTemplate_RightArrow).cast());
static sSpriteTemplate_UpDownArrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_list::sSpriteTemplate_UpDownArrow).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sMoveWindowDownIndex: crate::global::Global<u32> = crate::global::Global::new(0);

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}

pub unsafe fn CreatePokenavList(
    bgTemplate: *mut BgTemplate,
    listTemplate: *mut PokenavListTemplate,
    tileOffset: u32,
) -> u32 {
    let list: *mut PokenavList = AllocSubstruct(POKENAV_SUBSTRUCT_LIST, 2212) as *mut PokenavList;
    if list.is_null() {
        return FALSE as u32;
    }
    InitPokenavListWindowState(&raw mut (*list).windowState, listTemplate);
    if CopyPokenavListMenuTemplate(list, bgTemplate, listTemplate, tileOffset) == 0 {
        return FALSE as u32;
    }
    CreateLoopedTask(Some(LoopedTask_CreatePokenavList), 6);
    TRUE as u32
}
pub unsafe fn IsCreatePokenavListTaskActive() -> u32 {
    FuncIsActiveLoopedTask(Some(LoopedTask_CreatePokenavList))
}
pub unsafe fn DestroyPokenavList() {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    DestroyListArrows(list);
    RemoveWindow((*list).listWindow.windowId as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_LIST);
}
pub(crate) unsafe fn LoopedTask_CreatePokenavList(state: i32) -> u32 {
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return LT_PAUSE;
    }
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    'l1: {
        let sw1: i32 = state;
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4;
        let fall = false;
        if sw1 == 0 {
            InitPokenavListBg(list);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 1 {
            InitPokenavListWindow(&raw mut (*list).listWindow);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 2 {
            InitListItems(&raw mut (*list).windowState, list);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            if IsPrintListItemsTaskActive() != 0 {
                return LT_PAUSE;
            } else {
                LoadListArrowGfx();
                return LT_INC_AND_CONTINUE;
            }
        }
        if fall || sw1 == 4 {
            CreateListArrowSprites(&raw mut (*list).windowState, list);
            return LT_FINISH;
        }
        if !matched {
            return LT_FINISH;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn InitPokenavListBg(list: *mut PokenavList) {
    let tileNum: u16 = ((*list).listWindow.fillValue as u16) << 12 | (*list).listWindow.tileOffset;
    BgDmaFill(
        (*list).listWindow.bg as u32,
        17,
        (*list).listWindow.tileOffset as i32,
        1,
    );
    BgDmaFill(
        (*list).listWindow.bg as u32,
        68,
        (*list).listWindow.tileOffset as i32 + 1,
        1,
    );
    SetBgTilemapBuffer(
        (*list).listWindow.bg,
        (*list).tilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    FillBgTilemapBufferRect_Palette0((*list).listWindow.bg, tileNum, 0, 0, 32, 32);
    ChangeBgY((*list).listWindow.bg, 0, BG_COORD_SET);
    ChangeBgX((*list).listWindow.bg, 0, BG_COORD_SET);
    ChangeBgY(
        (*list).listWindow.bg,
        ((*list).listWindow.y as i32) << 11,
        BG_COORD_SUB,
    );
    CopyBgTilemapBufferToVram((*list).listWindow.bg);
}
unsafe fn InitPokenavListWindow(listWindow: *mut PokenavListMenuWindow) {
    FillWindowPixelBuffer((*listWindow).windowId as u8, 17);
    PutWindowTilemap((*listWindow).windowId as u8);
    CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_MAP);
}
unsafe fn InitListItems(windowState: *mut PokenavListWindowState, list: *mut PokenavList) {
    let mut numToPrint: i32 =
        (*windowState).listLength as i32 - (*windowState).windowTopIndex as i32;
    if numToPrint > (*windowState).entriesOnscreen as i32 {
        numToPrint = (*windowState).entriesOnscreen as i32;
    }
    PrintListItems(
        (*windowState).listPtr,
        (*windowState).windowTopIndex as u32,
        numToPrint as u32,
        (*windowState).listItemSize,
        0,
        list,
    );
}
unsafe fn PrintListItems(
    listPtr: *mut c_void,
    topIndex: u32,
    numItems: u32,
    itemSize: u32,
    printStart: u32,
    list: *mut PokenavList,
) {
    if numItems == 0 {
        return;
    }
    (*list).listPtr = (listPtr as *mut u8).at(topIndex * itemSize) as *mut c_void;
    (*list).itemSize = itemSize;
    (*list).listWindow.numPrinted = 0;
    (*list).listWindow.numToPrint = numItems as u16;
    (*list).printIndex = topIndex;
    (*list).printStart = printStart;
    CreateLoopedTask(Some(LoopedTask_PrintListItems), 5);
}
unsafe fn IsPrintListItemsTaskActive() -> u32 {
    FuncIsActiveLoopedTask(Some(LoopedTask_PrintListItems))
}
pub(crate) unsafe fn LoopedTask_PrintListItems(state: i32) -> u32 {
    let mut row: u32 = 0;
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    'l1: {
        let sw1: i32 = state;
        let fall = false;
        if sw1 == 0 {
            row = ((*list).listWindow.unkA as u32
                + (*list).listWindow.numPrinted as u32
                + (*list).printStart)
                & 0xF;
            (*list).bufferItemFunc.unwrap_unchecked()(
                (*list).listPtr as *mut PokenavListItem,
                (*list).itemTextBuffer.as_mut_ptr(),
            );
            if (*list).iconDrawFunc.is_some() {
                (*list).iconDrawFunc.unwrap_unchecked()(
                    (*list).listWindow.windowId,
                    (*list).printIndex,
                    row,
                );
            }
            AddTextPrinterParameterized(
                (*list).listWindow.windowId as u8,
                (*list).listWindow.fontId,
                (*list).itemTextBuffer.as_mut_ptr(),
                8,
                ((row as u8) << 4) + 1,
                TEXT_SKIP_DRAW,
                None,
            );
            if ({
                (*list).listWindow.numPrinted += 1;
                (*list).listWindow.numPrinted
            }) >= (*list).listWindow.numToPrint
            {
                if (*list).iconDrawFunc.is_some() {
                    CopyWindowToVram((*list).listWindow.windowId as u8, COPYWIN_FULL);
                } else {
                    CopyWindowToVram((*list).listWindow.windowId as u8, COPYWIN_GFX);
                }
                return LT_INC_AND_PAUSE;
            } else {
                (*list).listPtr = ((*list).listPtr as *mut u8).at((*list).itemSize) as *mut c_void;
                (*list).printIndex += 1;
                return LT_CONTINUE;
            }
        }
        if fall || sw1 == 1 {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            return LT_FINISH;
        }
    }
    LT_FINISH
}
unsafe fn ShouldShowUpArrow() -> u32 {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    ((*list).windowState.windowTopIndex != 0) as u32
}
unsafe fn ShouldShowDownArrow() -> u32 {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    let windowState: *mut PokenavListWindowState = &raw mut (*list).windowState;
    (((*windowState).windowTopIndex as i32 + (*windowState).entriesOnscreen as i32)
        < (*windowState).listLength as i32) as u32
}
unsafe fn MoveListWindow(mut delta: i32, printItems: u32) {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    let windowState: *mut PokenavListWindowState = &raw mut (*list).windowState;
    if delta < 0 {
        if (*windowState).windowTopIndex as i32 + delta < 0 {
            delta = -((*windowState).windowTopIndex as i32);
        }
        if printItems != 0 {
            PrintListItems(
                (*windowState).listPtr,
                (*windowState).windowTopIndex as u32 + delta as u32,
                delta as u32 * 0xffffffff,
                (*windowState).listItemSize,
                delta as u32,
                list,
            );
        }
    } else if printItems != 0 {
        let index: i32 = ({
            sMoveWindowDownIndex
                .set((*windowState).windowTopIndex as u32 + (*windowState).entriesOnscreen as u32);
            sMoveWindowDownIndex.get()
        }) as i32;
        if index + delta >= (*windowState).listLength as i32 {
            delta = (*windowState).listLength as i32 - index;
        }
        PrintListItems(
            (*windowState).listPtr,
            index as u32,
            delta as u32,
            (*windowState).listItemSize,
            (*windowState).entriesOnscreen as u32,
            list,
        );
    }
    CreateMoveListWindowTask(delta, list);
    (*windowState).windowTopIndex += delta as u16;
}
unsafe fn CreateMoveListWindowTask(delta: i32, list: *mut PokenavList) {
    (*list).startBgY = GetBgY((*list).listWindow.bg);
    (*list).endBgY = (*list).startBgY + (delta << 12);
    if delta > 0 {
        (*list).bgMoveType = BG_COORD_ADD as u32;
    } else {
        (*list).bgMoveType = BG_COORD_SUB as u32;
    }
    (*list).moveDelta = delta;
    (*list).moveListWindowLoopedTaskId = CreateLoopedTask(Some(LoopedTask_MoveListWindow), 6);
}
pub(crate) unsafe fn LoopedTask_MoveListWindow(state: i32) -> u32 {
    let mut oldY: i32 = 0;
    let mut newY: i32 = 0;
    let mut finished: u32 = 0;
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    match state {
        0 => {
            if IsPrintListItemsTaskActive() == 0 {
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        1 => {
            finished = FALSE as u32;
            oldY = GetBgY((*list).listWindow.bg);
            newY = ChangeBgY((*list).listWindow.bg, 0x1000, (*list).bgMoveType as u8);
            if (*list).bgMoveType == BG_COORD_SUB as u32 {
                if (oldY > (*list).endBgY || oldY <= (*list).startBgY) && newY <= (*list).endBgY {
                    finished = TRUE as u32;
                }
            } else {
                if (oldY < (*list).endBgY || oldY >= (*list).startBgY) && newY >= (*list).endBgY {
                    finished = TRUE as u32;
                }
            }
            if finished != 0 {
                (*list).listWindow.unkA =
                    ((*list).listWindow.unkA + (*list).moveDelta as u16) & 0xF;
                ChangeBgY((*list).listWindow.bg, (*list).endBgY, BG_COORD_SET);
                return LT_FINISH;
            }
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub unsafe fn PokenavList_IsMoveWindowTaskActive() -> u32 {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    IsLoopedTaskActive((*list).moveListWindowLoopedTaskId)
}
unsafe fn GetPokenavListWindowState() -> *mut PokenavListWindowState {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    &raw mut (*list).windowState
}
pub unsafe fn PokenavList_MoveCursorUp() -> i32 {
    let windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if (*windowState).selectedIndexOffset != 0 {
        (*windowState).selectedIndexOffset -= 1;
        return 1;
    }
    if ShouldShowUpArrow() != 0 {
        MoveListWindow(-1, 1);
        return 2;
    }
    0
}
pub unsafe fn PokenavList_MoveCursorDown() -> i32 {
    let windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if (*windowState).windowTopIndex as i32 + (*windowState).selectedIndexOffset as i32
        >= (*windowState).listLength as i32 - 1
    {
        return 0;
    }
    if ((*windowState).selectedIndexOffset as i32) < (*windowState).entriesOnscreen as i32 - 1 {
        (*windowState).selectedIndexOffset += 1;
        return 1;
    }
    if ShouldShowDownArrow() != 0 {
        MoveListWindow(1, 1);
        return 2;
    }
    0
}
pub unsafe fn PokenavList_PageUp() -> i32 {
    let mut scroll: i32 = 0;
    let windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if ShouldShowUpArrow() != 0 {
        if (*windowState).windowTopIndex >= (*windowState).entriesOnscreen {
            scroll = (*windowState).entriesOnscreen as i32;
        } else {
            scroll = (*windowState).windowTopIndex as i32;
        }
        MoveListWindow(-scroll, 1);
        return 2;
    } else if (*windowState).selectedIndexOffset != 0 {
        (*windowState).selectedIndexOffset = 0;
        return 1;
    }
    0
}
pub unsafe fn PokenavList_PageDown() -> i32 {
    let windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if ShouldShowDownArrow() != 0 {
        let windowBottomIndex: i32 =
            (*windowState).windowTopIndex as i32 + (*windowState).entriesOnscreen as i32;
        let mut scroll: i32 =
            (*windowState).entriesOffscreen as i32 - (*windowState).windowTopIndex as i32;
        if windowBottomIndex <= (*windowState).entriesOffscreen as i32 {
            scroll = (*windowState).entriesOnscreen as i32;
        }
        MoveListWindow(scroll, TRUE as u32);
        return 2;
    } else {
        let mut cursor: i32 = 0;
        let mut lastVisibleIndex: i32 = 0;
        if (*windowState).listLength >= (*windowState).entriesOnscreen {
            cursor = (*windowState).selectedIndexOffset as i32;
            lastVisibleIndex = (*windowState).entriesOnscreen as i32;
        } else {
            cursor = (*windowState).selectedIndexOffset as i32;
            lastVisibleIndex = (*windowState).listLength as i32;
        }
        lastVisibleIndex -= 1;
        if cursor >= lastVisibleIndex {
            return 0;
        }
        (*windowState).selectedIndexOffset = lastVisibleIndex as u16;
        return 1;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn PokenavList_GetSelectedIndex() -> u32 {
    let windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    (*windowState).windowTopIndex as u32 + (*windowState).selectedIndexOffset as u32
}
pub unsafe fn PokenavList_GetTopIndex() -> u32 {
    let windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    (*windowState).windowTopIndex as u32
}
pub unsafe fn PokenavList_EraseListForCheckPage() {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*list).eraseIndex = 0;
    (*list).loopedTaskId = CreateLoopedTask(Some(LoopedTask_EraseListForCheckPage), 6);
}
pub unsafe fn PrintCheckPageInfo(delta: i16) {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*list).windowState.windowTopIndex += delta as u16;
    (*list).eraseIndex = 0;
    (*list).loopedTaskId = CreateLoopedTask(Some(LoopedTask_PrintCheckPageInfo), 6);
}
pub unsafe fn PokenavList_ReshowListFromCheckPage() {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*list).eraseIndex = 0;
    (*list).loopedTaskId = CreateLoopedTask(Some(LoopedTask_ReshowListFromCheckPage), 6);
}
pub unsafe fn PokenavList_IsTaskActive() -> u32 {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    IsLoopedTaskActive((*list).loopedTaskId)
}
pub unsafe fn PokenavList_DrawCurrentItemIcon() {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    let windowState: *mut PokenavListWindowState = &raw mut (*list).windowState;
    (*list).iconDrawFunc.unwrap_unchecked()(
        (*list).listWindow.windowId,
        (*windowState).windowTopIndex as u32 + (*windowState).selectedIndexOffset as u32,
        ((*list).listWindow.unkA as u32 + (*windowState).selectedIndexOffset as u32) & 0xF,
    );
    CopyWindowToVram((*list).listWindow.windowId as u8, COPYWIN_MAP);
}
pub(crate) unsafe fn LoopedTask_EraseListForCheckPage(state: i32) -> u32 {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            ToggleListArrows(list, TRUE as u32);
        }
        if fall || sw1 == 1 {
            if (*list).eraseIndex != (*list).windowState.selectedIndexOffset as i32 {
                EraseListEntry(&raw mut (*list).listWindow, (*list).eraseIndex, 1);
            }
            (*list).eraseIndex += 1;
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 2 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                if (*list).eraseIndex != (*list).windowState.entriesOnscreen as i32 {
                    return 6;
                }
                if (*list).windowState.selectedIndexOffset != 0 {
                    EraseListEntry(
                        &raw mut (*list).listWindow,
                        (*list).eraseIndex,
                        (*list).windowState.selectedIndexOffset as i32,
                    );
                }
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        if sw1 == 3 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                if (*list).windowState.selectedIndexOffset != 0 {
                    MoveListWindow((*list).windowState.selectedIndexOffset as i32, FALSE as u32);
                    return LT_INC_AND_PAUSE;
                }
                return LT_FINISH;
            }
            return LT_PAUSE;
        }
        if sw1 == 4 {
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
            (*list).windowState.selectedIndexOffset = 0;
            return LT_FINISH;
        }
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_PrintCheckPageInfo(state: i32) -> u32 {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return LT_PAUSE;
    }
    match state {
        0 => {
            PrintCheckPageTrainerName(&raw mut (*list).windowState, list);
        }
        1 => {
            PrintMatchCallFieldNames(list, 0);
        }
        2 => {
            PrintMatchCallFlavorText(&raw mut (*list).windowState, list, CHECK_PAGE_STRATEGY);
        }
        3 => {
            PrintMatchCallFieldNames(list, 1);
        }
        4 => {
            PrintMatchCallFlavorText(&raw mut (*list).windowState, list, CHECK_PAGE_POKEMON);
        }
        5 => {
            PrintMatchCallFieldNames(list, 2);
        }
        6 => {
            PrintMatchCallFlavorText(&raw mut (*list).windowState, list, CHECK_PAGE_INTRO_1);
        }
        7 => {
            PrintMatchCallFlavorText(&raw mut (*list).windowState, list, CHECK_PAGE_INTRO_2);
        }
        _ => {
            return LT_FINISH;
        }
    }
    LT_INC_AND_PAUSE
}
pub(crate) unsafe fn LoopedTask_ReshowListFromCheckPage(state: i32) -> u32 {
    let mut windowState: *mut PokenavListWindowState = null_mut();
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return LT_PAUSE;
    }
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    windowState = &raw mut (*list).windowState;
    let listAlias: *mut PokenavList = list;
    match state {
        0 => {
            PrintMatchCallListTrainerName(windowState, listAlias);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if ({
                (*list).eraseIndex += 1;
                (*list).eraseIndex
            }) < (*list).windowState.entriesOnscreen as i32
            {
                EraseListEntry(&raw mut (*listAlias).listWindow, (*list).eraseIndex, 1);
                return LT_PAUSE;
            }
            (*list).eraseIndex = 0;
            if (*windowState).listLength <= (*windowState).entriesOnscreen {
                if (*windowState).windowTopIndex != 0 {
                    let entries: i32 = (*windowState).windowTopIndex as i32;
                    EraseListEntry(&raw mut (*listAlias).listWindow, -entries, entries);
                    (*windowState).selectedIndexOffset = entries as u16;
                    (*list).eraseIndex = -entries;
                    return LT_INC_AND_PAUSE;
                }
            } else {
                if (*windowState).windowTopIndex as i32 + (*windowState).entriesOnscreen as i32
                    > (*windowState).listLength as i32
                {
                    let entries: i32 = (*windowState).windowTopIndex as i32
                        + (*windowState).entriesOnscreen as i32
                        - (*windowState).listLength as i32;
                    EraseListEntry(&raw mut (*listAlias).listWindow, -entries, entries);
                    (*windowState).selectedIndexOffset = entries as u16;
                    (*list).eraseIndex = -entries;
                    return LT_INC_AND_PAUSE;
                }
            }
            return 9;
        }
        2 => {
            MoveListWindow((*list).eraseIndex, FALSE as u32);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if PokenavList_IsMoveWindowTaskActive() == 0 {
                (*list).eraseIndex = 0;
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        4 => {
            PrintListItems(
                (*windowState).listPtr,
                (*windowState).windowTopIndex as u32 + (*list).eraseIndex as u32,
                1,
                (*windowState).listItemSize,
                (*list).eraseIndex as u32,
                list,
            );
            return LT_INC_AND_PAUSE;
        }
        5 => {
            if IsPrintListItemsTaskActive() != 0 {
                return LT_PAUSE;
            }
            if ({
                (*list).eraseIndex += 1;
                (*list).eraseIndex
            }) >= (*windowState).listLength as i32
                || (*list).eraseIndex >= (*windowState).entriesOnscreen as i32
            {
                return LT_INC_AND_CONTINUE;
            }
            return 9;
        }
        6 => {
            ToggleListArrows(listAlias, FALSE as u32);
            return LT_FINISH;
        }
        _ => {}
    }
    LT_FINISH
}
unsafe fn EraseListEntry(
    listWindow: *mut PokenavListMenuWindow,
    mut offset: i32,
    mut entries: i32,
) {
    let tileData: *mut u8 =
        GetWindowAttribute((*listWindow).windowId as u8, WINDOW_TILE_DATA) as usize as *mut u8;
    let width: u32 = (*listWindow).width as u32 * 64;
    offset = ((*listWindow).unkA as i32 + offset) & 0xF;
    if offset + entries <= 16 {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111);
            CpuFastSet(
                &raw mut tmp as *mut c_void,
                tileData.at(offset as u32 * width) as *mut c_void,
                0x01000000 | (entries as u32 * width / 4) & 0x1FFFFF,
            );
        }
        CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_GFX);
    } else {
        let v3: u32 = 16 - offset as u32;
        let v4: u32 = entries as u32 - v3;
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111);
            CpuFastSet(
                &raw mut tmp as *mut c_void,
                tileData.at(offset as u32 * width) as *mut c_void,
                0x01000000 | (v3 * width / 4) & 0x1FFFFF,
            );
        }
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111);
            CpuFastSet(
                &raw mut tmp as *mut c_void,
                tileData as *mut c_void,
                0x01000000 | (v4 * width / 4) & 0x1FFFFF,
            );
        }
        CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_GFX);
    }
    entries -= 1;
    while entries != -1 {
        ClearRematchPokeballIcon((*listWindow).windowId, offset as u32);
        offset = (offset + 1) & 0xF;
        entries -= 1;
    }
    CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_MAP);
}
unsafe fn SetListMarginTile(listWindow: *mut PokenavListMenuWindow, draw: u32) {
    let mut var: u16 = 0;
    let mut tilemapBuffer: *mut u16 =
        GetBgTilemapBuffer(GetWindowAttribute((*listWindow).windowId as u8, WINDOW_BG) as u8)
            as *mut u16;
    tilemapBuffer =
        tilemapBuffer.at((((*listWindow).unkA as i32) << 6) + (*listWindow).x as i32 - 1);
    if draw != 0 {
        var = (((*listWindow).fillValue as u16) << 12) | ((*listWindow).tileOffset + 1);
    } else {
        var = ((*listWindow).fillValue as u16) << 12 | (*listWindow).tileOffset;
    }
    *tilemapBuffer = var;
    *tilemapBuffer.at(32) = var;
}
unsafe fn PrintCheckPageTrainerName(state: *mut PokenavListWindowState, list: *mut PokenavList) {
    let mut colors: CArray<u8, 3> = CArray([0, 2, 5]);
    (*list).bufferItemFunc.unwrap_unchecked()(
        ((*state).listPtr as *mut u8).at((*state).listItemSize * (*state).windowTopIndex as u32)
            as *mut c_void as *mut PokenavListItem,
        (*list).itemTextBuffer.as_mut_ptr(),
    );
    (*list).iconDrawFunc.unwrap_unchecked()(
        (*list).listWindow.windowId,
        (*state).windowTopIndex as u32,
        (*list).listWindow.unkA as u32,
    );
    FillWindowPixelRect(
        (*list).listWindow.windowId as u8,
        68,
        0,
        (*list).listWindow.unkA * 16,
        (*list).listWindow.width as u16 * 8,
        16,
    );
    AddTextPrinterParameterized3(
        (*list).listWindow.windowId as u8,
        (*list).listWindow.fontId,
        8,
        (*list).listWindow.unkA as u8 * 16 + 1,
        colors.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        (*list).itemTextBuffer.as_mut_ptr(),
    );
    SetListMarginTile(&raw mut (*list).listWindow, TRUE as u32);
    CopyWindowRectToVram(
        (*list).listWindow.windowId as u32,
        COPYWIN_FULL as u32,
        0,
        (*list).listWindow.unkA as u32 * 2,
        (*list).listWindow.width as u32,
        2,
    );
}
unsafe fn PrintMatchCallListTrainerName(
    state: *mut PokenavListWindowState,
    list: *mut PokenavList,
) {
    (*list).bufferItemFunc.unwrap_unchecked()(
        ((*state).listPtr as *mut u8).at((*state).listItemSize * (*state).windowTopIndex as u32)
            as *mut c_void as *mut PokenavListItem,
        (*list).itemTextBuffer.as_mut_ptr(),
    );
    FillWindowPixelRect(
        (*list).listWindow.windowId as u8,
        17,
        0,
        (*list).listWindow.unkA * 16,
        (*list).listWindow.width as u16 * 8,
        16,
    );
    AddTextPrinterParameterized(
        (*list).listWindow.windowId as u8,
        (*list).listWindow.fontId,
        (*list).itemTextBuffer.as_mut_ptr(),
        8,
        (*list).listWindow.unkA as u8 * 16 + 1,
        TEXT_SKIP_DRAW,
        None,
    );
    SetListMarginTile(&raw mut (*list).listWindow, FALSE as u32);
    CopyWindowToVram((*list).listWindow.windowId as u8, COPYWIN_FULL);
}
unsafe fn PrintMatchCallFieldNames(list: *mut PokenavList, fieldId: u32) {
    let mut fieldNames: CArray<*mut u8, 3> = zeroed();
    fieldNames[0] = (*(&raw const crate::data::strings::gText_PokenavMatchCall_Strategy)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    fieldNames[1] = (*(&raw const crate::data::strings::gText_PokenavMatchCall_TrainerPokemon)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    fieldNames[2] = (*(&raw const crate::data::strings::gText_PokenavMatchCall_SelfIntroduction)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    let mut colors: CArray<u8, 3> = CArray([1, 4, 5]);
    let top: u32 = ((*list).listWindow.unkA as u32 + 1 + fieldId * 2) & 0xF;
    FillWindowPixelRect(
        (*list).listWindow.windowId as u8,
        17,
        0,
        (top as u16) << 4,
        (*list).listWindow.width as u16,
        16,
    );
    AddTextPrinterParameterized3(
        (*list).listWindow.windowId as u8,
        FONT_NARROW,
        2,
        ((top as u8) << 4) + 1,
        colors.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        fieldNames[fieldId],
    );
    CopyWindowRectToVram(
        (*list).listWindow.windowId as u32,
        COPYWIN_GFX as u32,
        0,
        top << 1,
        (*list).listWindow.width as u32,
        2,
    );
}
unsafe fn PrintMatchCallFlavorText(
    windowState: *mut PokenavListWindowState,
    list: *mut PokenavList,
    checkPageEntry: u32,
) {
    let r6: u32 = ((*list).listWindow.unkA as u32 + lineOffsets_0[checkPageEntry] as u32) & 0xF;
    let str: *mut u8 =
        GetMatchCallFlavorText((*windowState).windowTopIndex as i32, checkPageEntry as i32);
    if !str.is_null() {
        FillWindowTilesByRow(
            (*list).listWindow.windowId as i32,
            1,
            r6 as i32 * 2,
            (*list).listWindow.width as i32 - 1,
            2,
        );
        AddTextPrinterParameterized(
            (*list).listWindow.windowId as u8,
            FONT_NARROW,
            str,
            2,
            ((r6 as u8) << 4) + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        CopyWindowRectToVram(
            (*list).listWindow.windowId as u32,
            COPYWIN_GFX as u32,
            0,
            r6 * 2,
            (*list).listWindow.width as u32,
            2,
        );
    }
}
unsafe fn LoadListArrowGfx() {
    let mut ptr: *mut CompressedSpriteSheet = sListArrowSpriteSheets.as_ptr().cast_mut();
    for i in 0..1u32 {
        LoadCompressedSpriteSheet(ptr);
        ptr = ptr.at(1);
    }
    Pokenav_AllocAndLoadPalettes(sListArrowPalettes.as_ptr().cast_mut());
}
unsafe fn CreateListArrowSprites(windowState: *mut PokenavListWindowState, list: *mut PokenavList) {
    let mut x: i16 = 0;
    let mut spriteId: u32 = CreateSprite(
        (&raw const *sSpriteTemplate_RightArrow).cast_mut(),
        (*list).listWindow.x as i16 * 8 + 3,
        ((*list).listWindow.y as i16 + 1) * 8,
        7,
    ) as u32;
    (*list).rightArrow = &raw mut gSprites[spriteId];
    x = (*list).listWindow.x as i16 * 8 + ((*list).listWindow.width as i16 - 1) * 4;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_UpDownArrow).cast_mut(),
        x,
        (*list).listWindow.y as i16 * 8 + (*windowState).entriesOnscreen as i16 * 16,
        7,
    ) as u32;
    (*list).downArrow = &raw mut gSprites[spriteId];
    (*(*list).downArrow)
        .oam
        .set_tileNum((*(*list).downArrow).oam.tileNum() + 2);
    (*(*list).downArrow).callback = Some(SpriteCB_DownArrow);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_UpDownArrow).cast_mut(),
        x,
        (*list).listWindow.y as i16 * 8,
        7,
    ) as u32;
    (*list).upArrow = &raw mut gSprites[spriteId];
    (*(*list).upArrow)
        .oam
        .set_tileNum((*(*list).upArrow).oam.tileNum() + 4);
    (*(*list).upArrow).callback = Some(SpriteCB_UpArrow);
}
unsafe fn DestroyListArrows(list: *mut PokenavList) {
    DestroySprite((*list).rightArrow);
    DestroySprite((*list).upArrow);
    DestroySprite((*list).downArrow);
    FreeSpriteTilesByTag(GFXTAG_ARROW);
    FreeSpritePaletteByTag(PALTAG_ARROW);
}
unsafe fn ToggleListArrows(list: *mut PokenavList, invisible: u32) {
    if invisible != 0 {
        (*(*list).rightArrow).callback = Some(SpriteCallbackDummy);
        (*(*list).upArrow).callback = Some(SpriteCallbackDummy);
        (*(*list).downArrow).callback = Some(SpriteCallbackDummy);
    } else {
        (*(*list).rightArrow).callback = Some(SpriteCB_RightArrow);
        (*(*list).upArrow).callback = Some(SpriteCB_UpArrow);
        (*(*list).downArrow).callback = Some(SpriteCB_DownArrow);
    }
    (*(*list).rightArrow).set_invisible(invisible as u16);
    (*(*list).upArrow).set_invisible(invisible as u16);
    (*(*list).downArrow).set_invisible(invisible as u16);
}
pub(crate) unsafe fn SpriteCB_RightArrow(sprite: *mut Sprite) {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*sprite).y2 = ((*list).windowState.selectedIndexOffset as i16) << 4;
}
pub(crate) unsafe fn SpriteCB_DownArrow(sprite: *mut Sprite) {
    if (*sprite).data[sInvisible] == 0 && ShouldShowDownArrow() != 0 {
        (*sprite).set_invisible(FALSE as u16);
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
    if ({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) > 3
    {
        (*sprite).data[sTimer] = 0;
        let offset: i16 = ((*sprite).data[sOffset] + 1) & 7;
        (*sprite).data[sOffset] = offset;
        (*sprite).y2 = offset;
    }
}
pub(crate) unsafe fn SpriteCB_UpArrow(sprite: *mut Sprite) {
    if (*sprite).data[sInvisible] == 0 && ShouldShowUpArrow() != 0 {
        (*sprite).set_invisible(FALSE as u16);
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
    if ({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) > 3
    {
        (*sprite).data[sTimer] = 0;
        let offset: i16 = ((*sprite).data[sOffset] + 1) & 7;
        (*sprite).data[sOffset] = offset;
        (*sprite).y2 = -offset;
    }
}
pub unsafe fn PokenavList_ToggleVerticalArrows(invisible: u32) {
    let list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*(*list).upArrow).data[sInvisible] = invisible as i16;
    (*(*list).downArrow).data[sInvisible] = invisible as i16;
}
unsafe fn InitPokenavListWindowState(
    dst: *mut PokenavListWindowState,
    template: *mut PokenavListTemplate,
) {
    (*dst).listPtr = (*template).list as *mut c_void;
    (*dst).windowTopIndex = (*template).startIndex;
    (*dst).listLength = (*template).count;
    (*dst).listItemSize = (*template).itemSize as u32;
    (*dst).entriesOnscreen = (*template).maxShowed as u16;
    if (*dst).entriesOnscreen >= (*dst).listLength {
        (*dst).windowTopIndex = 0;
        (*dst).entriesOffscreen = 0;
        (*dst).selectedIndexOffset = (*template).startIndex;
    } else {
        (*dst).entriesOffscreen = (*dst).listLength - (*dst).entriesOnscreen;
        if (*dst).windowTopIndex as i32 + (*dst).entriesOnscreen as i32 > (*dst).listLength as i32 {
            (*dst).selectedIndexOffset =
                (*dst).windowTopIndex + (*dst).entriesOnscreen - (*dst).listLength;
            (*dst).windowTopIndex = (*template).startIndex - (*dst).selectedIndexOffset;
        } else {
            (*dst).selectedIndexOffset = 0;
        }
    }
}
unsafe fn CopyPokenavListMenuTemplate(
    dest: *mut PokenavList,
    bgTemplate: *mut BgTemplate,
    template: *mut PokenavListTemplate,
    tileOffset: u32,
) -> u32 {
    let mut window: WindowTemplate = zeroed();
    (*dest).listWindow.bg = (*bgTemplate).bg() as u8;
    (*dest).listWindow.tileOffset = tileOffset as u16;
    (*dest).bufferItemFunc = (*template).bufferItemFunc;
    (*dest).iconDrawFunc = (*template).iconDrawFunc;
    (*dest).listWindow.fillValue = (*template).fillValue;
    (*dest).listWindow.x = (*template).item_X;
    (*dest).listWindow.y = (*template).listTop;
    (*dest).listWindow.width = (*template).windowWidth;
    (*dest).listWindow.fontId = (*template).fontId;
    window.bg = (*bgTemplate).bg() as u8;
    window.tilemapLeft = (*template).item_X;
    window.tilemapTop = 0;
    window.width = (*template).windowWidth;
    window.height = 32;
    window.paletteNum = (*template).fillValue;
    window.baseBlock = tileOffset as u16 + 2;
    (*dest).listWindow.windowId = AddWindow(&raw mut window);
    if (*dest).listWindow.windowId == WINDOW_NONE as u16 {
        return FALSE as u32;
    }
    (*dest).listWindow.unkA = 0;
    (*dest).rightArrow = null_mut();
    (*dest).upArrow = null_mut();
    (*dest).downArrow = null_mut();
    1
}
