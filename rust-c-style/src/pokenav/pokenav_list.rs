//! Translated from `src/pokenav_list.c` by tools/rustport/c2rs.py.
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
    pub bufferItemFunc: Option<unsafe extern "C" fn(*mut PokenavListItem, *mut u8)>,
    pub iconDrawFunc: Option<unsafe extern "C" fn(u16, u32, u32)>,
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
pub(crate) static mut sMoveWindowDownIndex: u32 = 0;

unsafe extern "C" {
    static mut gSprites: CArray<Sprite, 65>;
    static gText_PokenavMatchCall_SelfIntroduction: CArray<u8, 0>;
    static gText_PokenavMatchCall_Strategy: CArray<u8, 0>;
    static gText_PokenavMatchCall_TrainerPokemon: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearRematchPokeballIcon(a0: u16, a1: u32);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowRectToVram(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FillWindowTilesByRow(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32);
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>) -> u32;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetBgY(a0: u8) -> i32;
    fn GetMatchCallFlavorText(a0: i32, a1: i32) -> *mut u8;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn Pokenav_AllocAndLoadPalettes(a0: *mut SpritePalette);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SpriteCallbackDummy(a0: *mut Sprite);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePokenavList(
    bgTemplate: *mut BgTemplate,
    listTemplate: *mut PokenavListTemplate,
    tileOffset: u32,
) -> u32 {
    let mut list: *mut PokenavList =
        AllocSubstruct(POKENAV_SUBSTRUCT_LIST, 2212) as *mut PokenavList;
    if list.is_null() {
        return FALSE as u32;
    }
    InitPokenavListWindowState(&raw mut (*list).windowState, listTemplate);
    if CopyPokenavListMenuTemplate(list, bgTemplate, listTemplate, tileOffset) == 0 {
        return FALSE as u32;
    }
    CreateLoopedTask(Some(LoopedTask_CreatePokenavList), 6);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCreatePokenavListTaskActive() -> u32 {
    return FuncIsActiveLoopedTask(Some(LoopedTask_CreatePokenavList));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyPokenavList() {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    DestroyListArrows(list);
    RemoveWindow((*list).listWindow.windowId as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_LIST);
}
pub(crate) unsafe extern "C" fn LoopedTask_CreatePokenavList(state: i32) -> u32 {
    let mut list: *mut PokenavList = null_mut();
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return LT_PAUSE;
    }
    list = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    'l1: {
        let sw1: i32 = state;
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            InitPokenavListBg(list);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 1 {
            fall = true;
            InitPokenavListWindow(&raw mut (*list).listWindow);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 2 {
            fall = true;
            InitListItems(&raw mut (*list).windowState, list);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            fall = true;
            if IsPrintListItemsTaskActive() != 0 {
                return LT_PAUSE;
            } else {
                LoadListArrowGfx();
                return LT_INC_AND_CONTINUE;
            }
        }
        if fall || sw1 == 4 {
            fall = true;
            CreateListArrowSprites(&raw mut (*list).windowState, list);
            return LT_FINISH;
        }
        if !matched {
            fall = true;
            return LT_FINISH;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InitPokenavListBg(list: *mut PokenavList) {
    let mut tileNum: u16 =
        ((*list).listWindow.fillValue as u16) << 12 | (*list).listWindow.tileOffset;
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
pub(crate) unsafe extern "C" fn InitPokenavListWindow(listWindow: *mut PokenavListMenuWindow) {
    FillWindowPixelBuffer((*listWindow).windowId as u8, 17);
    PutWindowTilemap((*listWindow).windowId as u8);
    CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_MAP);
}
pub(crate) unsafe extern "C" fn InitListItems(
    windowState: *mut PokenavListWindowState,
    list: *mut PokenavList,
) {
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
pub(crate) unsafe extern "C" fn PrintListItems(
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
pub(crate) unsafe extern "C" fn IsPrintListItemsTaskActive() -> u32 {
    return FuncIsActiveLoopedTask(Some(LoopedTask_PrintListItems));
}
pub(crate) unsafe extern "C" fn LoopedTask_PrintListItems(state: i32) -> u32 {
    let mut row: u32 = 0;
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            row = (*list).listWindow.unkA as u32
                + (*list).listWindow.numPrinted as u32
                + (*list).printStart
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
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            return LT_FINISH;
        }
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn ShouldShowUpArrow() -> u32 {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    return ((*list).windowState.windowTopIndex != 0) as u32;
}
pub(crate) unsafe extern "C" fn ShouldShowDownArrow() -> u32 {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    let mut windowState: *mut PokenavListWindowState = &raw mut (*list).windowState;
    return (((*windowState).windowTopIndex as i32 + (*windowState).entriesOnscreen as i32)
        < (*windowState).listLength as i32) as u32;
}
pub(crate) unsafe extern "C" fn MoveListWindow(mut delta: i32, printItems: u32) {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    let mut windowState: *mut PokenavListWindowState = &raw mut (*list).windowState;
    if delta < 0 {
        if (*windowState).windowTopIndex as i32 + delta < 0 {
            delta = -1 * (*windowState).windowTopIndex as i32;
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
        let mut index: i32 = ({
            sMoveWindowDownIndex =
                (*windowState).windowTopIndex as u32 + (*windowState).entriesOnscreen as u32;
            sMoveWindowDownIndex
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
pub(crate) unsafe extern "C" fn CreateMoveListWindowTask(delta: i32, list: *mut PokenavList) {
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
pub(crate) unsafe extern "C" fn LoopedTask_MoveListWindow(state: i32) -> u32 {
    let mut oldY: i32 = 0;
    let mut newY: i32 = 0;
    let mut finished: u32 = 0;
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
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
                (*list).listWindow.unkA = (*list).listWindow.unkA + (*list).moveDelta as u16 & 0xF;
                ChangeBgY((*list).listWindow.bg, (*list).endBgY, BG_COORD_SET);
                return LT_FINISH;
            }
            return LT_PAUSE;
        }
        _ => {}
    }
    return LT_FINISH;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_IsMoveWindowTaskActive() -> u32 {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    return IsLoopedTaskActive((*list).moveListWindowLoopedTaskId);
}
pub(crate) unsafe extern "C" fn GetPokenavListWindowState() -> *mut PokenavListWindowState {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    return &raw mut (*list).windowState;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_MoveCursorUp() -> i32 {
    let mut windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if (*windowState).selectedIndexOffset != 0 {
        (*windowState).selectedIndexOffset -= 1;
        return 1;
    }
    if ShouldShowUpArrow() != 0 {
        MoveListWindow(-1, 1);
        return 2;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_MoveCursorDown() -> i32 {
    let mut windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
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
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_PageUp() -> i32 {
    let mut scroll: i32 = 0;
    let mut windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if ShouldShowUpArrow() != 0 {
        if (*windowState).windowTopIndex >= (*windowState).entriesOnscreen {
            scroll = (*windowState).entriesOnscreen as i32;
        } else {
            scroll = (*windowState).windowTopIndex as i32;
        }
        MoveListWindow(scroll * -1, 1);
        return 2;
    } else if (*windowState).selectedIndexOffset != 0 {
        (*windowState).selectedIndexOffset = 0;
        return 1;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_PageDown() -> i32 {
    let mut windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    if ShouldShowDownArrow() != 0 {
        let mut windowBottomIndex: i32 =
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_GetSelectedIndex() -> u32 {
    let mut windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    return (*windowState).windowTopIndex as u32 + (*windowState).selectedIndexOffset as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_GetTopIndex() -> u32 {
    let mut windowState: *mut PokenavListWindowState = GetPokenavListWindowState();
    return (*windowState).windowTopIndex as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_EraseListForCheckPage() {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*list).eraseIndex = 0;
    (*list).loopedTaskId = CreateLoopedTask(Some(LoopedTask_EraseListForCheckPage), 6);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintCheckPageInfo(delta: i16) {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*list).windowState.windowTopIndex += delta as u16;
    (*list).eraseIndex = 0;
    (*list).loopedTaskId = CreateLoopedTask(Some(LoopedTask_PrintCheckPageInfo), 6);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_ReshowListFromCheckPage() {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*list).eraseIndex = 0;
    (*list).loopedTaskId = CreateLoopedTask(Some(LoopedTask_ReshowListFromCheckPage), 6);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_IsTaskActive() -> u32 {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    return IsLoopedTaskActive((*list).loopedTaskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_DrawCurrentItemIcon() {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    let mut windowState: *mut PokenavListWindowState = &raw mut (*list).windowState;
    (*list).iconDrawFunc.unwrap_unchecked()(
        (*list).listWindow.windowId,
        (*windowState).windowTopIndex as u32 + (*windowState).selectedIndexOffset as u32,
        (*list).listWindow.unkA as u32 + (*windowState).selectedIndexOffset as u32 & 0xF,
    );
    CopyWindowToVram((*list).listWindow.windowId as u8, COPYWIN_MAP);
}
pub(crate) unsafe extern "C" fn LoopedTask_EraseListForCheckPage(state: i32) -> u32 {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            ToggleListArrows(list, TRUE as u32);
        }
        if fall || sw1 == 1 {
            fall = true;
            if (*list).eraseIndex != (*list).windowState.selectedIndexOffset as i32 {
                EraseListEntry(&raw mut (*list).listWindow, (*list).eraseIndex, 1);
            }
            (*list).eraseIndex += 1;
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 2 {
            fall = true;
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
            fall = true;
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
            fall = true;
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
            (*list).windowState.selectedIndexOffset = 0;
            return LT_FINISH;
        }
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_PrintCheckPageInfo(state: i32) -> u32 {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
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
    return LT_INC_AND_PAUSE;
}
pub(crate) unsafe extern "C" fn LoopedTask_ReshowListFromCheckPage(state: i32) -> u32 {
    let mut list: *mut PokenavList = null_mut();
    let mut listAlias: *mut PokenavList = null_mut();
    let mut windowState: *mut PokenavListWindowState = null_mut();
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return LT_PAUSE;
    }
    list = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    windowState = &raw mut (*list).windowState;
    listAlias = list;
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
                    let mut entries: i32 = (*windowState).windowTopIndex as i32;
                    EraseListEntry(&raw mut (*listAlias).listWindow, -entries, entries);
                    (*windowState).selectedIndexOffset = entries as u16;
                    (*list).eraseIndex = -entries;
                    return LT_INC_AND_PAUSE;
                }
            } else {
                if (*windowState).windowTopIndex as i32 + (*windowState).entriesOnscreen as i32
                    > (*windowState).listLength as i32
                {
                    let mut entries: i32 = (*windowState).windowTopIndex as i32
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn EraseListEntry(
    listWindow: *mut PokenavListMenuWindow,
    mut offset: i32,
    mut entries: i32,
) {
    let mut tileData: *mut u8 =
        GetWindowAttribute((*listWindow).windowId as u8, WINDOW_TILE_DATA) as usize as *mut u8;
    let mut width: u32 = (*listWindow).width as u32 * 64;
    offset = (*listWindow).unkA as i32 + offset & 0xF;
    if offset + entries <= 16 {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111);
            CpuFastSet(
                &raw mut tmp as *mut c_void,
                tileData.at(offset as u32 * width) as *mut c_void,
                0x01000000 | entries as u32 * width / 4 & 0x1FFFFF,
            );
        }
        CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_GFX);
    } else {
        let mut v3: u32 = 16 - offset as u32;
        let mut v4: u32 = entries as u32 - v3;
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111);
            CpuFastSet(
                &raw mut tmp as *mut c_void,
                tileData.at(offset as u32 * width) as *mut c_void,
                0x01000000 | v3 * width / 4 & 0x1FFFFF,
            );
        }
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111);
            CpuFastSet(
                &raw mut tmp as *mut c_void,
                tileData as *mut c_void,
                0x01000000 | v4 * width / 4 & 0x1FFFFF,
            );
        }
        CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_GFX);
    }
    entries -= 1;
    while entries != -1 {
        ClearRematchPokeballIcon((*listWindow).windowId, offset as u32);
        offset = offset + 1 & 0xF;
        entries -= 1;
    }
    CopyWindowToVram((*listWindow).windowId as u8, COPYWIN_MAP);
}
pub(crate) unsafe extern "C" fn SetListMarginTile(
    listWindow: *mut PokenavListMenuWindow,
    draw: u32,
) {
    let mut var: u16 = 0;
    let mut tilemapBuffer: *mut u16 =
        GetBgTilemapBuffer(GetWindowAttribute((*listWindow).windowId as u8, WINDOW_BG) as u8)
            as *mut u16;
    tilemapBuffer =
        tilemapBuffer.at((((*listWindow).unkA as i32) << 6) + (*listWindow).x as i32 - 1);
    if draw != 0 {
        var = ((*listWindow).fillValue as u16) << 12 | (*listWindow).tileOffset + 1;
    } else {
        var = ((*listWindow).fillValue as u16) << 12 | (*listWindow).tileOffset;
    }
    *tilemapBuffer = var;
    *tilemapBuffer.at(32) = var;
}
pub(crate) unsafe extern "C" fn PrintCheckPageTrainerName(
    state: *mut PokenavListWindowState,
    list: *mut PokenavList,
) {
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
pub(crate) unsafe extern "C" fn PrintMatchCallListTrainerName(
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
pub(crate) unsafe extern "C" fn PrintMatchCallFieldNames(list: *mut PokenavList, fieldId: u32) {
    let mut fieldNames: CArray<*mut u8, 3> = zeroed();
    fieldNames[0] = gText_PokenavMatchCall_Strategy.as_ptr().cast_mut();
    fieldNames[1] = gText_PokenavMatchCall_TrainerPokemon.as_ptr().cast_mut();
    fieldNames[2] = gText_PokenavMatchCall_SelfIntroduction.as_ptr().cast_mut();
    let mut colors: CArray<u8, 3> = CArray([1, 4, 5]);
    let mut top: u32 = (*list).listWindow.unkA as u32 + 1 + fieldId * 2 & 0xF;
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
pub(crate) unsafe extern "C" fn PrintMatchCallFlavorText(
    windowState: *mut PokenavListWindowState,
    list: *mut PokenavList,
    checkPageEntry: u32,
) {
    let mut r6: u32 = (*list).listWindow.unkA as u32 + lineOffsets_0[checkPageEntry] as u32 & 0xF;
    let mut str: *mut u8 =
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
pub(crate) unsafe extern "C" fn LoadListArrowGfx() {
    let mut i: u32 = 0;
    let mut ptr: *mut CompressedSpriteSheet = null_mut();
    i = 0;
    ptr = sListArrowSpriteSheets.as_ptr().cast_mut();
    while i < 1 {
        LoadCompressedSpriteSheet(ptr);
        ptr = ptr.at(1);
        i += 1;
    }
    Pokenav_AllocAndLoadPalettes(sListArrowPalettes.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn CreateListArrowSprites(
    windowState: *mut PokenavListWindowState,
    list: *mut PokenavList,
) {
    let mut spriteId: u32 = 0;
    let mut x: i16 = 0;
    spriteId = CreateSprite(
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
pub(crate) unsafe extern "C" fn DestroyListArrows(list: *mut PokenavList) {
    DestroySprite((*list).rightArrow);
    DestroySprite((*list).upArrow);
    DestroySprite((*list).downArrow);
    FreeSpriteTilesByTag(GFXTAG_ARROW);
    FreeSpritePaletteByTag(PALTAG_ARROW);
}
pub(crate) unsafe extern "C" fn ToggleListArrows(list: *mut PokenavList, invisible: u32) {
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
pub(crate) unsafe extern "C" fn SpriteCB_RightArrow(sprite: *mut Sprite) {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*sprite).y2 = ((*list).windowState.selectedIndexOffset as i16) << 4;
}
pub(crate) unsafe extern "C" fn SpriteCB_DownArrow(sprite: *mut Sprite) {
    if (*sprite).data[7] == 0 && ShouldShowDownArrow() != 0 {
        (*sprite).set_invisible(FALSE as u16);
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 3
    {
        let mut offset: i16 = 0;
        (*sprite).data[0] = 0;
        offset = (*sprite).data[1] + 1 & 7;
        (*sprite).data[1] = offset;
        (*sprite).y2 = offset;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpArrow(sprite: *mut Sprite) {
    if (*sprite).data[7] == 0 && ShouldShowUpArrow() != 0 {
        (*sprite).set_invisible(FALSE as u16);
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 3
    {
        let mut offset: i16 = 0;
        (*sprite).data[0] = 0;
        offset = (*sprite).data[1] + 1 & 7;
        (*sprite).data[1] = offset;
        (*sprite).y2 = -1 * offset;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_ToggleVerticalArrows(invisible: u32) {
    let mut list: *mut PokenavList = GetSubstructPtr(POKENAV_SUBSTRUCT_LIST) as *mut PokenavList;
    (*(*list).upArrow).data[7] = invisible as i16;
    (*(*list).downArrow).data[7] = invisible as i16;
}
pub(crate) unsafe extern "C" fn InitPokenavListWindowState(
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
pub(crate) unsafe extern "C" fn CopyPokenavListMenuTemplate(
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
    return 1;
}
