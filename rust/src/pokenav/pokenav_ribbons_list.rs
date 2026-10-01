//! Translated from `src/pokenav_ribbons_list.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::bg::CopyToBgTilemapBuffer;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, HideBg, IsDma3ManagerBusyWithBgCopy, ShowBg,
};
use crate::box_mon::GetBoxMonData3;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::international_string_util::{GetStringCenterAlignXOffset, GetStringClearToWidth};
use crate::menu::{DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible};
use crate::pokemon::{
    GetBoxMonGender, GetLevelFromBoxMonExp, GetLevelFromMonExp, GetMonData2, GetMonData3,
    GetMonGender, gPlayerParty,
};
use crate::pokemon_storage_system::{CheckBoxMonSanityAt, GetBoxMonDataAt, GetBoxedMonPtr};
use crate::pokenav::CreateLoopedTask;
use crate::pokenav::{AllocSubstruct, FreePokenavSubstruct, GetSubstructPtr, IsLoopedTaskActive};
use crate::pokenav_list::{
    CreatePokenavList, DestroyPokenavList, IsCreatePokenavListTaskActive,
    PokenavList_GetSelectedIndex, PokenavList_IsMoveWindowTaskActive, PokenavList_MoveCursorDown,
    PokenavList_MoveCursorUp, PokenavList_PageDown, PokenavList_PageUp,
};
use crate::pokenav_main_menu::{
    AreLeftHeaderSpritesMoving, CopyPaletteIntoBufferUnfaded, InitBgTemplates, IsPaletteFadeActive,
    LoadLeftHeaderGfxForIndex, MainMenuLoopedTaskIsBusy, PokenavFadeScreen, PrintHelpBarText,
    SetLeftHeaderSpritesInvisibility, ShowLeftHeaderGfx, SlideMenuHeaderDown,
};
use crate::sound::PlaySE;
use crate::string_util::StringGet_Nickname;
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy};
use crate::string_util::{gStringVar1, gStringVar3};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, PutWindowTilemap, RemoveWindow};
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
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
// Data tables (translate with cdata.py): sMonRibbonListLoopTaskFuncs sMonRibbonListFramePal sMonRibbonListFrameTiles sMonRibbonListFrameTilemap sMonRibbonListUi_Pal sMonRibbonListBgTemplates sRibbonsMonMenuLoopTaskFuncs sRibbonsMonListWindowTemplate sText_MaleSymbol sText_FemaleSymbol sText_NoGenderSymbol

/// `struct Pokenav_RibbonsMonList`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RibbonsMonList {
    pub callback: Option<unsafe fn(*mut Pokenav_RibbonsMonList) -> u32>,
    pub loopedTaskId: u32,
    pub winid: u16,
    pub boxId: i32,
    pub monId: i32,
    pub changeBgs: u32,
    pub saveMonList: u32,
    pub monList: *mut PokenavMonList,
}

unsafe impl Sync for Pokenav_RibbonsMonList {}

/// `struct Pokenav_RibbonsMonMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RibbonsMonMenu {
    pub callback: Option<unsafe fn() -> u32>,
    pub loopedTaskId: u32,
    pub winid: u16,
    pub fromSummary: u32,
    pub buff: CArray<u8, 2048>,
}

unsafe impl Sync for Pokenav_RibbonsMonMenu {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_RibbonsMonList>() == 32);
    assert!(offset_of!(Pokenav_RibbonsMonList, callback) == 0);
    assert!(offset_of!(Pokenav_RibbonsMonList, loopedTaskId) == 4);
    assert!(offset_of!(Pokenav_RibbonsMonList, winid) == 8);
    assert!(offset_of!(Pokenav_RibbonsMonList, boxId) == 12);
    assert!(offset_of!(Pokenav_RibbonsMonList, monId) == 16);
    assert!(offset_of!(Pokenav_RibbonsMonList, changeBgs) == 20);
    assert!(offset_of!(Pokenav_RibbonsMonList, saveMonList) == 24);
    assert!(offset_of!(Pokenav_RibbonsMonList, monList) == 28);
    assert!(size_of::<Pokenav_RibbonsMonMenu>() == 2064);
    assert!(offset_of!(Pokenav_RibbonsMonMenu, callback) == 0);
    assert!(offset_of!(Pokenav_RibbonsMonMenu, loopedTaskId) == 4);
    assert!(offset_of!(Pokenav_RibbonsMonMenu, winid) == 8);
    assert!(offset_of!(Pokenav_RibbonsMonMenu, fromSummary) == 12);
    assert!(offset_of!(Pokenav_RibbonsMonMenu, buff) == 16);
};

const RIBBONS_MON_LIST_FUNC_EXIT: u32 = 5;
const RIBBONS_MON_LIST_FUNC_MOVE_DOWN: u32 = 2;
const RIBBONS_MON_LIST_FUNC_MOVE_UP: u32 = 1;
const RIBBONS_MON_LIST_FUNC_NONE: u32 = 0;
const RIBBONS_MON_LIST_FUNC_OPEN_RIBBONS_SUMMARY: u32 = 6;
const RIBBONS_MON_LIST_FUNC_PAGE_DOWN: u32 = 4;
const RIBBONS_MON_LIST_FUNC_PAGE_UP: u32 = 3;

static sMonRibbonListBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListBgTemplates).cast());
static sMonRibbonListFramePal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListFramePal).cast());
static sMonRibbonListFrameTilemap: Table<CArray<u32, 49>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListFrameTilemap).cast());
static sMonRibbonListFrameTiles: Table<CArray<u32, 50>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListFrameTiles).cast());
static sMonRibbonListLoopTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 3>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListLoopTaskFuncs).cast());
static sMonRibbonListUi_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListUi_Pal).cast());
static sRibbonsMonListWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_ribbons_list::sRibbonsMonListWindowTemplate).cast());
static sRibbonsMonMenuLoopTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 7>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sRibbonsMonMenuLoopTaskFuncs).cast());
static sText_FemaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sText_FemaleSymbol).cast());
static sText_MaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sText_MaleSymbol).cast());
static sText_NoGenderSymbol: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sText_NoGenderSymbol).cast());

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

pub unsafe fn PokenavCallback_Init_MonRibbonList() -> u32 {
    let list: *mut Pokenav_RibbonsMonList =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST, 32) as *mut Pokenav_RibbonsMonList;
    if list.is_null() {
        return FALSE as u32;
    }
    (*list).monList = AllocSubstruct(POKENAV_SUBSTRUCT_MON_LIST, 1708) as *mut PokenavMonList;
    if (*list).monList.is_null() {
        return FALSE as u32;
    }
    (*list).callback = Some(HandleRibbonsMonListInput_WaitListInit);
    (*list).loopedTaskId = CreateLoopedTask(Some(GetMonRibbonListLoopTaskFunc), 1);
    (*list).changeBgs = 0;
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_RibbonsMonListFromSummary() -> u32 {
    let list: *mut Pokenav_RibbonsMonList =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST, 32) as *mut Pokenav_RibbonsMonList;
    if list.is_null() {
        return FALSE as u32;
    }
    (*list).monList = GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    (*list).callback = Some(HandleRibbonsMonListInput);
    (*list).changeBgs = 1;
    TRUE as u32
}
pub unsafe fn GetRibbonsMonListCallback() -> u32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*list).callback.unwrap_unchecked()(list)
}
pub unsafe fn FreeRibbonsMonList() {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    if (*list).saveMonList == 0 {
        FreePokenavSubstruct(POKENAV_SUBSTRUCT_MON_LIST);
    }
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST);
}
pub(crate) unsafe fn HandleRibbonsMonListInput_WaitListInit(
    list: *mut Pokenav_RibbonsMonList,
) -> u32 {
    if IsLoopedTaskActive((*list).loopedTaskId) == 0 {
        (*list).callback = Some(HandleRibbonsMonListInput);
    }
    0
}
pub(crate) unsafe fn HandleRibbonsMonListInput(list: *mut Pokenav_RibbonsMonList) -> u32 {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        return RIBBONS_MON_LIST_FUNC_MOVE_UP;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        return RIBBONS_MON_LIST_FUNC_MOVE_DOWN;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        return RIBBONS_MON_LIST_FUNC_PAGE_UP;
    }
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        return RIBBONS_MON_LIST_FUNC_PAGE_DOWN;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*list).saveMonList = 0;
        (*list).callback = Some(RibbonsMonMenu_ReturnToMainMenu);
        return RIBBONS_MON_LIST_FUNC_EXIT;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*(*list).monList).currIndex = PokenavList_GetSelectedIndex() as u16;
        (*list).saveMonList = 1;
        (*list).callback = Some(RibbonsMonMenu_ToSummaryScreen);
        return RIBBONS_MON_LIST_FUNC_OPEN_RIBBONS_SUMMARY;
    }
    RIBBONS_MON_LIST_FUNC_NONE
}
pub(crate) unsafe fn RibbonsMonMenu_ReturnToMainMenu(list: *mut Pokenav_RibbonsMonList) -> u32 {
    POKENAV_MAIN_MENU_CURSOR_ON_RIBBONS
}
pub(crate) unsafe fn RibbonsMonMenu_ToSummaryScreen(list: *mut Pokenav_RibbonsMonList) -> u32 {
    POKENAV_RIBBONS_SUMMARY_SCREEN
}
unsafe fn UpdateMonListBgs() -> u32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*list).changeBgs
}
unsafe fn GetMonRibbonMonListData() -> *mut PokenavMonListItem {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*(*list).monList).monData.as_mut_ptr()
}
unsafe fn GetRibbonsMonListCount() -> i32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*(*list).monList).listCount as i32
}
unsafe fn GetMonRibbonSelectedMonData() -> i32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    let idx: i32 = PokenavList_GetSelectedIndex() as i32;
    (*(*list).monList).monData[idx].data as i32
}
unsafe fn GetRibbonListMenuCurrIndex() -> i32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*(*list).monList).currIndex as i32
}
pub(crate) unsafe fn GetMonRibbonListLoopTaskFunc(state: i32) -> u32 {
    sMonRibbonListLoopTaskFuncs[state].unwrap_unchecked()(state)
}
pub(crate) unsafe fn BuildPartyMonRibbonList(state: i32) -> u32 {
    let mut item: PokenavMonListItem = zeroed();
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*(*list).monList).listCount = 0;
    (*(*list).monList).currIndex = 0;
    item.boxId = TOTAL_BOXES_COUNT;
    for i in 0..PARTY_SIZE {
        let pokemon: *mut Pokemon = &raw mut gPlayerParty[i];
        if GetMonData2(pokemon, MON_DATA_SANITY_HAS_SPECIES) == 0 {
            return LT_INC_AND_CONTINUE;
        }
        if GetMonData2(pokemon, MON_DATA_SANITY_IS_EGG) == 0
            && GetMonData2(pokemon, MON_DATA_SANITY_IS_BAD_EGG) == 0
        {
            let ribbonCount: u32 = GetMonData2(pokemon, MON_DATA_RIBBON_COUNT);
            if ribbonCount != 0 {
                item.monId = i as u8;
                item.data = ribbonCount as u16;
                InsertMonListItem(list, &raw mut item);
            }
        }
    }
    LT_INC_AND_CONTINUE
}
pub(crate) unsafe fn InitBoxMonRibbonList(state: i32) -> u32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*list).monId = 0;
    (*list).boxId = 0;
    LT_INC_AND_CONTINUE
}
pub(crate) unsafe fn BuildBoxMonRibbonList(state: i32) -> u32 {
    let list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    let mut boxId: i32 = (*list).boxId;
    let mut monId: i32 = (*list).monId;
    let mut boxCount: i32 = 0;
    let mut item: PokenavMonListItem = zeroed();
    while boxId < TOTAL_BOXES_COUNT as i32 {
        while monId < IN_BOX_COUNT {
            if CheckBoxMonSanityAt(boxId as u32, monId as u32) != 0 {
                let ribbonCount: u32 =
                    GetBoxMonDataAt(boxId as u8, monId as u8, MON_DATA_RIBBON_COUNT);
                if ribbonCount != 0 {
                    item.boxId = boxId as u8;
                    item.monId = monId as u8;
                    item.data = ribbonCount as u16;
                    InsertMonListItem(list, &raw mut item);
                }
            }
            boxCount += 1;
            monId += 1;
            if boxCount > TOTAL_BOXES_COUNT as i32 {
                (*list).boxId = boxId;
                (*list).monId = monId;
                return LT_CONTINUE;
            }
        }
        monId = 0;
        boxId += 1;
    }
    (*list).changeBgs = 1;
    LT_FINISH
}
pub(crate) unsafe fn InsertMonListItem(
    list: *mut Pokenav_RibbonsMonList,
    item: *mut PokenavMonListItem,
) {
    let mut left: u32 = 0;
    let mut right: u32 = (*(*list).monList).listCount as u32;
    let mut insertionIdx: u32 = left + (right - left) / 2;
    while right != insertionIdx {
        if (*item).data > (*(*list).monList).monData[insertionIdx].data {
            right = insertionIdx;
        } else {
            left = insertionIdx + 1;
        }
        insertionIdx = left + (right - left) / 2;
    }
    right = (*(*list).monList).listCount as u32;
    while right > insertionIdx {
        (*(*list).monList).monData[right] = (*(*list).monList).monData[right - 1];
        right -= 1;
    }
    (*(*list).monList).monData[insertionIdx] = *item;
    (*(*list).monList).listCount += 1;
}
unsafe fn PlayerHasRibbonsMon() -> u32 {
    let mut i: i32 = 0;
    while i < PARTY_SIZE {
        'l1: {
            let mon: *mut Pokemon = &raw mut gPlayerParty[i];
            if GetMonData2(mon, MON_DATA_SANITY_HAS_SPECIES) == 0 {
                break 'l1;
            }
            if GetMonData2(mon, MON_DATA_SANITY_IS_EGG) != 0 {
                break 'l1;
            }
            if GetMonData2(mon, MON_DATA_RIBBONS) != 0 {
                return TRUE as u32;
            }
        }
        i += 1;
    }
    for i in 0..(TOTAL_BOXES_COUNT as i32) {
        for j in 0..IN_BOX_COUNT {
            'l4: {
                if CheckBoxMonSanityAt(i as u32, j as u32) == 0 {
                    break 'l4;
                }
                if GetBoxMonDataAt(i as u8, j as u8, MON_DATA_RIBBONS) != 0 {
                    return TRUE as u32;
                }
            }
        }
    }
    FALSE as u32
}
pub unsafe fn OpenRibbonsMonList() -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU, 2064) as *mut Pokenav_RibbonsMonMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenRibbonsMonList), 1);
    (*menu).callback = Some(GetRibbonsMonCurrentLoopedTaskActive);
    (*menu).fromSummary = FALSE as u32;
    TRUE as u32
}
pub unsafe fn OpenRibbonsMonListFromRibbonsSummary() -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU, 2064) as *mut Pokenav_RibbonsMonMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenRibbonsMonList), 1);
    (*menu).callback = Some(GetRibbonsMonCurrentLoopedTaskActive);
    (*menu).fromSummary = TRUE as u32;
    TRUE as u32
}
pub unsafe fn CreateRibbonsMonListLoopedTask(idx: i32) {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    (*menu).loopedTaskId = CreateLoopedTask(sRibbonsMonMenuLoopTaskFuncs[idx], 1);
    (*menu).callback = Some(GetRibbonsMonCurrentLoopedTaskActive);
}
pub unsafe fn IsRibbonsMonListLoopedTaskActive() -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    (*menu).callback.unwrap_unchecked()()
}
pub unsafe fn GetRibbonsMonCurrentLoopedTaskActive() -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    IsLoopedTaskActive((*menu).loopedTaskId)
}
pub unsafe fn FreeRibbonsMonMenu() {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    DestroyPokenavList();
    RemoveWindow((*menu).winid as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU);
}
pub(crate) unsafe fn LoopedTask_OpenRibbonsMonList(state: i32) -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    match state {
        0 => {
            InitBgTemplates(sMonRibbonListBgTemplates.as_ptr().cast_mut(), 2);
            DecompressAndCopyTileDataToVram(
                1,
                sMonRibbonListFrameTiles.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(1, (*menu).buff.as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                1,
                sMonRibbonListFrameTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyPaletteIntoBufferUnfaded(sMonRibbonListFramePal.as_ptr().cast_mut(), 16, 32);
            CopyBgTilemapBufferToVram(1);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            if UpdateMonListBgs() == 0 {
                return LT_PAUSE;
            }
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ShowBg(1);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            CopyPaletteIntoBufferUnfaded(sMonRibbonListUi_Pal.as_ptr().cast_mut(), 32, 32);
            CreateRibbonMonsList();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsCreatePokenavListTaskActive() != 0 {
                return LT_PAUSE;
            }
            AddRibbonsMonListWindow(menu);
            return LT_INC_AND_PAUSE;
        }
        4 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            ShowBg(2);
            HideBg(3);
            PrintHelpBarText(HELPBAR_RIBBONS_MON_LIST as u32);
            PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
            if (*menu).fromSummary == 0 {
                LoadLeftHeaderGfxForIndex(POKENAV_GFX_RIBBONS_MENU);
                ShowLeftHeaderGfx(POKENAV_GFX_RIBBONS_MENU, TRUE as u32, FALSE as u32);
            }
            return LT_INC_AND_PAUSE;
        }
        5 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RibbonsListMoveCursorUp(state: i32) -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            match PokenavList_MoveCursorUp() {
                0 => {
                    return LT_FINISH;
                }
                1 => {
                    PlaySE(SE_SELECT);
                    return 7;
                }
                2 => {
                    PlaySE(SE_SELECT);
                }
                _ => {}
            }
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 1 {
            fall = true;
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
        }
        if fall || sw1 == 2 {
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RibbonsListMoveCursorDown(state: i32) -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            match PokenavList_MoveCursorDown() {
                0 => {
                    return LT_FINISH;
                }
                1 => {
                    PlaySE(SE_SELECT);
                    return 7;
                }
                2 => {
                    PlaySE(SE_SELECT);
                }
                _ => {}
            }
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 1 {
            fall = true;
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
        }
        if fall || sw1 == 2 {
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RibbonsListMovePageUp(state: i32) -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            match PokenavList_PageUp() {
                0 => {
                    return LT_FINISH;
                }
                1 => {
                    PlaySE(SE_SELECT);
                    return 7;
                }
                2 => {
                    PlaySE(SE_SELECT);
                }
                _ => {}
            }
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 1 {
            fall = true;
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
        }
        if fall || sw1 == 2 {
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RibbonsListMovePageDown(state: i32) -> u32 {
    let menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            match PokenavList_PageDown() {
                0 => {
                    return LT_FINISH;
                }
                1 => {
                    PlaySE(SE_SELECT);
                    return 7;
                }
                2 => {
                    PlaySE(SE_SELECT);
                }
                _ => {}
            }
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 1 {
            fall = true;
            if PokenavList_IsMoveWindowTaskActive() != 0 {
                return LT_PAUSE;
            }
        }
        if fall || sw1 == 2 {
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RibbonsListReturnToMainMenu(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            SlideMenuHeaderDown();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
            if MainMenuLoopedTaskIsBusy() != 0 {
                return LT_PAUSE;
            }
            SetLeftHeaderSpritesInvisibility();
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RibbonsListOpenSummary(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            return LT_INC_AND_PAUSE;
        }
        1 if IsPaletteFadeActive() != 0 => {
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
unsafe fn AddRibbonsMonListWindow(menu: *mut Pokenav_RibbonsMonMenu) {
    (*menu).winid = AddWindow((&raw const *sRibbonsMonListWindowTemplate).cast_mut());
    PutWindowTilemap((*menu).winid as u8);
    let listCount: i32 = GetRibbonsMonListCount();
    DrawListIndexNumber((*menu).winid as i32, 0, listCount);
    CopyWindowToVram((*menu).winid as u8, COPYWIN_MAP);
    UpdateIndexNumberDisplay(menu);
}
unsafe fn UpdateIndexNumberDisplay(menu: *mut Pokenav_RibbonsMonMenu) {
    let listIndex: i32 = PokenavList_GetSelectedIndex() as i32;
    let listCount: i32 = GetRibbonsMonListCount();
    DrawListIndexNumber((*menu).winid as i32, listIndex + 1, listCount);
    CopyWindowToVram((*menu).winid as u8, COPYWIN_GFX);
}
unsafe fn DrawListIndexNumber(windowId: i32, index: i32, max: i32) {
    let mut strbuf: CArray<u8, 16> = zeroed();
    let mut ptr: *mut u8 = strbuf.as_mut_ptr();
    ptr = ConvertIntToDecimalStringN(ptr, index, STR_CONV_MODE_RIGHT_ALIGN, 3);
    *({
        let t1 = ptr;
        ptr = ptr.at(1);
        t1
    }) = CHAR_SLASH;
    ConvertIntToDecimalStringN(ptr, max, STR_CONV_MODE_RIGHT_ALIGN, 3);
    let x: u32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, strbuf.as_mut_ptr(), 56) as u32;
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        strbuf.as_mut_ptr(),
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn CreateRibbonMonsList() {
    let mut template: PokenavListTemplate = zeroed();
    template.list = GetMonRibbonMonListData() as *mut PokenavListItem;
    template.count = GetRibbonsMonListCount() as u16;
    template.itemSize = 4;
    template.startIndex = GetRibbonListMenuCurrIndex() as u16;
    template.item_X = 13;
    template.windowWidth = 17;
    template.listTop = 1;
    template.maxShowed = 8;
    template.fillValue = 2;
    template.fontId = FONT_NORMAL;
    template.bufferItemFunc = Some(BufferRibbonMonInfoText);
    template.iconDrawFunc = None;
    CreatePokenavList(
        (&raw const sMonRibbonListBgTemplates[1]).cast_mut(),
        &raw mut template,
        0,
    );
}
pub(crate) unsafe fn BufferRibbonMonInfoText(listItem: *mut PokenavListItem, mut dest: *mut u8) {
    let mut gender: u8 = 0;
    let mut level: u8 = 0;
    let mut genderStr: *mut u8 = null_mut();
    let item: *mut PokenavMonListItem = listItem as *mut PokenavMonListItem;
    if (*item).boxId == TOTAL_BOXES_COUNT {
        let mon: *mut Pokemon = &raw mut gPlayerParty[(*item).monId];
        gender = GetMonGender(mon);
        level = GetLevelFromMonExp(mon);
        GetMonData3(mon, MON_DATA_NICKNAME, gStringVar3.as_mut_ptr());
    } else {
        let mon: *mut BoxPokemon = GetBoxedMonPtr((*item).boxId, (*item).monId);
        gender = GetBoxMonGender(mon);
        level = GetLevelFromBoxMonExp(mon);
        GetBoxMonData3(mon, MON_DATA_NICKNAME, gStringVar3.as_mut_ptr());
    }
    StringGet_Nickname(gStringVar3.as_mut_ptr());
    dest = GetStringClearToWidth(dest, FONT_NORMAL as i32, gStringVar3.as_mut_ptr(), 60);
    match gender {
        MON_MALE => {
            genderStr = sText_MaleSymbol.as_ptr().cast_mut();
        }
        MON_FEMALE => {
            genderStr = sText_FemaleSymbol.as_ptr().cast_mut();
        }
        _ => {
            genderStr = sText_NoGenderSymbol.as_ptr().cast_mut();
        }
    }
    let mut s: *mut u8 = StringCopy(gStringVar1.as_mut_ptr(), genderStr);
    *({
        let t1 = s;
        s = s.at(1);
        t1
    }) = CHAR_SLASH;
    *({
        let t2 = s;
        s = s.at(1);
        t2
    }) = CHAR_EXTRA_SYMBOL;
    *({
        let t3 = s;
        s = s.at(1);
        t3
    }) = CHAR_LV_2;
    ConvertIntToDecimalStringN(s, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    dest = GetStringClearToWidth(dest, FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 54);
    ConvertIntToDecimalStringN(dest, (*item).data as i32, STR_CONV_MODE_RIGHT_ALIGN, 2);
}
