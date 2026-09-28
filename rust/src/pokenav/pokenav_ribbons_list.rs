//! Translated from `src/pokenav_ribbons_list.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMonRibbonListLoopTaskFuncs sMonRibbonListFramePal sMonRibbonListFrameTiles sMonRibbonListFrameTilemap sMonRibbonListUi_Pal sMonRibbonListBgTemplates sRibbonsMonMenuLoopTaskFuncs sRibbonsMonListWindowTemplate sText_MaleSymbol sText_FemaleSymbol sText_NoGenderSymbol

/// `struct Pokenav_RibbonsMonList`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RibbonsMonList {
    pub callback: Option<unsafe extern "C" fn(*mut Pokenav_RibbonsMonList) -> u32>,
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
    pub callback: Option<unsafe extern "C" fn() -> u32>,
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
static sMonRibbonListLoopTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(i32) -> u32>, 3>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListLoopTaskFuncs).cast());
static sMonRibbonListUi_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sMonRibbonListUi_Pal).cast());
static sRibbonsMonListWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_ribbons_list::sRibbonsMonListWindowTemplate).cast());
static sRibbonsMonMenuLoopTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(i32) -> u32>, 7>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sRibbonsMonMenuLoopTaskFuncs).cast());
static sText_FemaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sText_FemaleSymbol).cast());
static sText_MaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sText_MaleSymbol).cast());
static sText_NoGenderSymbol: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokenav_ribbons_list::sText_NoGenderSymbol).cast());

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckBoxMonSanityAt(a0: u32, a1: u32) -> u32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreatePokenavList(a0: *mut BgTemplate, a1: *mut PokenavListTemplate, a2: u32) -> u32;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyPokenavList();
    fn FreePokenavSubstruct(a0: u32);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxMonGender(a0: *mut BoxPokemon) -> u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut BoxPokemon;
    fn GetLevelFromBoxMonExp(a0: *mut BoxPokemon) -> u8;
    fn GetLevelFromMonExp(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringClearToWidth(a0: *mut u8, a1: i32, a2: *mut u8, a3: i32) -> *mut u8;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn HideBg(a0: u8);
    fn InitBgTemplates(a0: *mut BgTemplate, a1: i32);
    fn IsCreatePokenavListTaskActive() -> u32;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn PlaySE(a0: u16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavList_GetSelectedIndex() -> u32;
    fn PokenavList_IsMoveWindowTaskActive() -> u32;
    fn PokenavList_MoveCursorDown() -> i32;
    fn PokenavList_MoveCursorUp() -> i32;
    fn PokenavList_PageDown() -> i32;
    fn PokenavList_PageUp() -> i32;
    fn PrintHelpBarText(a0: u32);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetLeftHeaderSpritesInvisibility();
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MonRibbonList() -> u32 {
    let mut list: *mut Pokenav_RibbonsMonList =
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
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_RibbonsMonListFromSummary() -> u32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST, 32) as *mut Pokenav_RibbonsMonList;
    if list.is_null() {
        return FALSE as u32;
    }
    (*list).monList = GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    (*list).callback = Some(HandleRibbonsMonListInput);
    (*list).changeBgs = 1;
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonsMonListCallback() -> u32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    return (*list).callback.unwrap_unchecked()(list);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsMonList() {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    if (*list).saveMonList == 0 {
        FreePokenavSubstruct(POKENAV_SUBSTRUCT_MON_LIST);
    }
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST);
}
pub(crate) unsafe extern "C" fn HandleRibbonsMonListInput_WaitListInit(
    list: *mut Pokenav_RibbonsMonList,
) -> u32 {
    if IsLoopedTaskActive((*list).loopedTaskId) == 0 {
        (*list).callback = Some(HandleRibbonsMonListInput);
    }
    return 0;
}
pub(crate) unsafe extern "C" fn HandleRibbonsMonListInput(
    list: *mut Pokenav_RibbonsMonList,
) -> u32 {
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
    return RIBBONS_MON_LIST_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn RibbonsMonMenu_ReturnToMainMenu(
    list: *mut Pokenav_RibbonsMonList,
) -> u32 {
    return POKENAV_MAIN_MENU_CURSOR_ON_RIBBONS;
}
pub(crate) unsafe extern "C" fn RibbonsMonMenu_ToSummaryScreen(
    list: *mut Pokenav_RibbonsMonList,
) -> u32 {
    return POKENAV_RIBBONS_SUMMARY_SCREEN;
}
pub(crate) unsafe extern "C" fn UpdateMonListBgs() -> u32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    return (*list).changeBgs;
}
pub(crate) unsafe extern "C" fn GetMonRibbonMonListData() -> *mut PokenavMonListItem {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    return (*(*list).monList).monData.as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetRibbonsMonListCount() -> i32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    return (*(*list).monList).listCount as i32;
}
pub(crate) unsafe extern "C" fn GetMonRibbonSelectedMonData() -> i32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    let mut idx: i32 = PokenavList_GetSelectedIndex() as i32;
    return (*(*list).monList).monData[idx].data as i32;
}
pub(crate) unsafe extern "C" fn GetRibbonListMenuCurrIndex() -> i32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    return (*(*list).monList).currIndex as i32;
}
pub(crate) unsafe extern "C" fn GetMonRibbonListLoopTaskFunc(state: i32) -> u32 {
    return sMonRibbonListLoopTaskFuncs[state].unwrap_unchecked()(state);
}
pub(crate) unsafe extern "C" fn BuildPartyMonRibbonList(state: i32) -> u32 {
    let mut i: i32 = 0;
    let mut item: PokenavMonListItem = zeroed();
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*(*list).monList).listCount = 0;
    (*(*list).monList).currIndex = 0;
    item.boxId = TOTAL_BOXES_COUNT;
    i = 0;
    while i < PARTY_SIZE {
        let mut pokemon: *mut Pokemon = &raw mut gPlayerParty[i];
        if GetMonData2(pokemon, MON_DATA_SANITY_HAS_SPECIES) == 0 {
            return LT_INC_AND_CONTINUE;
        }
        if GetMonData2(pokemon, MON_DATA_SANITY_IS_EGG) == 0
            && GetMonData2(pokemon, MON_DATA_SANITY_IS_BAD_EGG) == 0
        {
            let mut ribbonCount: u32 = GetMonData2(pokemon, MON_DATA_RIBBON_COUNT);
            if ribbonCount != 0 {
                item.monId = i as u8;
                item.data = ribbonCount as u16;
                InsertMonListItem(list, &raw mut item);
            }
        }
        i += 1;
    }
    return LT_INC_AND_CONTINUE;
}
pub(crate) unsafe extern "C" fn InitBoxMonRibbonList(state: i32) -> u32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    (*list).monId = 0;
    (*list).boxId = 0;
    return LT_INC_AND_CONTINUE;
}
pub(crate) unsafe extern "C" fn BuildBoxMonRibbonList(state: i32) -> u32 {
    let mut list: *mut Pokenav_RibbonsMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_LIST) as *mut Pokenav_RibbonsMonList;
    let mut boxId: i32 = (*list).boxId;
    let mut monId: i32 = (*list).monId;
    let mut boxCount: i32 = 0;
    let mut item: PokenavMonListItem = zeroed();
    while boxId < TOTAL_BOXES_COUNT as i32 {
        while monId < IN_BOX_COUNT {
            if CheckBoxMonSanityAt(boxId as u32, monId as u32) != 0 {
                let mut ribbonCount: u32 =
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn InsertMonListItem(
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
pub(crate) unsafe extern "C" fn PlayerHasRibbonsMon() -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        'l1: {
            let mut mon: *mut Pokemon = &raw mut gPlayerParty[i];
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
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            'l4: {
                if CheckBoxMonSanityAt(i as u32, j as u32) == 0 {
                    break 'l4;
                }
                if GetBoxMonDataAt(i as u8, j as u8, MON_DATA_RIBBONS) != 0 {
                    return TRUE as u32;
                }
            }
            j += 1;
        }
        i += 1;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenRibbonsMonList() -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU, 2064) as *mut Pokenav_RibbonsMonMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenRibbonsMonList), 1);
    (*menu).callback = Some(GetRibbonsMonCurrentLoopedTaskActive);
    (*menu).fromSummary = FALSE as u32;
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenRibbonsMonListFromRibbonsSummary() -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU, 2064) as *mut Pokenav_RibbonsMonMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenRibbonsMonList), 1);
    (*menu).callback = Some(GetRibbonsMonCurrentLoopedTaskActive);
    (*menu).fromSummary = TRUE as u32;
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRibbonsMonListLoopedTask(idx: i32) {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    (*menu).loopedTaskId = CreateLoopedTask(sRibbonsMonMenuLoopTaskFuncs[idx], 1);
    (*menu).callback = Some(GetRibbonsMonCurrentLoopedTaskActive);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRibbonsMonListLoopedTaskActive() -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    return (*menu).callback.unwrap_unchecked()();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonsMonCurrentLoopedTaskActive() -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    return IsLoopedTaskActive((*menu).loopedTaskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsMonMenu() {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    DestroyPokenavList();
    RemoveWindow((*menu).winid as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU);
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenRibbonsMonList(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMoveCursorUp(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
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
            fall = true;
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMoveCursorDown(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
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
            fall = true;
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMovePageUp(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
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
            fall = true;
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMovePageDown(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsMonMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_MON_MENU) as *mut Pokenav_RibbonsMonMenu;
    'l1: {
        let sw1: i32 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
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
            fall = true;
            UpdateIndexNumberDisplay(menu);
            return LT_INC_AND_PAUSE;
        }
        if sw1 == 3 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            break 'l1;
        }
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListReturnToMainMenu(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListOpenSummary(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn AddRibbonsMonListWindow(menu: *mut Pokenav_RibbonsMonMenu) {
    let mut listCount: i32 = 0;
    (*menu).winid = AddWindow((&raw const *sRibbonsMonListWindowTemplate).cast_mut());
    PutWindowTilemap((*menu).winid as u8);
    listCount = GetRibbonsMonListCount();
    DrawListIndexNumber((*menu).winid as i32, 0, listCount);
    CopyWindowToVram((*menu).winid as u8, COPYWIN_MAP);
    UpdateIndexNumberDisplay(menu);
}
pub(crate) unsafe extern "C" fn UpdateIndexNumberDisplay(menu: *mut Pokenav_RibbonsMonMenu) {
    let mut listIndex: i32 = PokenavList_GetSelectedIndex() as i32;
    let mut listCount: i32 = GetRibbonsMonListCount();
    DrawListIndexNumber((*menu).winid as i32, listIndex + 1, listCount);
    CopyWindowToVram((*menu).winid as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn DrawListIndexNumber(windowId: i32, index: i32, max: i32) {
    let mut strbuf: CArray<u8, 16> = zeroed();
    let mut x: u32 = 0;
    let mut ptr: *mut u8 = strbuf.as_mut_ptr();
    ptr = ConvertIntToDecimalStringN(ptr, index, STR_CONV_MODE_RIGHT_ALIGN, 3);
    *({
        let t1 = ptr;
        ptr = ptr.at(1);
        t1
    }) = CHAR_SLASH;
    ConvertIntToDecimalStringN(ptr, max, STR_CONV_MODE_RIGHT_ALIGN, 3);
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, strbuf.as_mut_ptr(), 56) as u32;
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
pub(crate) unsafe extern "C" fn CreateRibbonMonsList() {
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
pub(crate) unsafe extern "C" fn BufferRibbonMonInfoText(
    listItem: *mut PokenavListItem,
    mut dest: *mut u8,
) {
    let mut gender: u8 = 0;
    let mut level: u8 = 0;
    let mut s: *mut u8 = null_mut();
    let mut genderStr: *mut u8 = null_mut();
    let mut item: *mut PokenavMonListItem = listItem as *mut PokenavMonListItem;
    if (*item).boxId == TOTAL_BOXES_COUNT {
        let mut mon: *mut Pokemon = &raw mut gPlayerParty[(*item).monId];
        gender = GetMonGender(mon);
        level = GetLevelFromMonExp(mon);
        GetMonData3(mon, MON_DATA_NICKNAME, gStringVar3.as_mut_ptr());
    } else {
        let mut mon: *mut BoxPokemon = GetBoxedMonPtr((*item).boxId, (*item).monId);
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
    s = StringCopy(gStringVar1.as_mut_ptr(), genderStr);
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
