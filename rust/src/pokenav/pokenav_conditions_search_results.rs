//! Translated from `src/pokenav_conditions_search_results.c` by tools/rustport/c2rs.py.
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
use crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_Reset;
use crate::dynamic_placeholder_text_util::{
    DynamicPlaceholderTextUtil_ExpandPlaceholders, DynamicPlaceholderTextUtil_SetPlaceholderPtr,
};
use crate::international_string_util::GetStringClearToWidth;
use crate::menu::{DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible};
use crate::pokemon::{
    GetBoxMonGender, GetLevelFromBoxMonExp, GetLevelFromMonExp, GetMonData2, GetMonData3,
    GetMonGender, gPlayerParty,
};
use crate::pokemon_storage_system::{CheckBoxMonSanityAt, GetBoxMonDataAt, GetBoxedMonPtr};
use crate::pokenav::CreateLoopedTask;
use crate::pokenav::{
    AllocSubstruct, FreePokenavSubstruct, GetSelectedConditionSearch, GetSubstructPtr,
    IsLoopedTaskActive,
};
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
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3};
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
// Data tables (translate with cdata.py): sSearchMonDataIds sConditionSearchLoopedTaskFuncs sConditionSearchResultFramePal sConditionSearchResultTiles sConditionSearchResultTilemap sListBg_Pal sConditionSearchResultBgTemplates sSearchResultLoopTaskFuncs sSearchResultListMenuWindowTemplate sText_MaleSymbol sText_FemaleSymbol sText_NoGenderSymbol

/// `struct Pokenav_SearchResults`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_SearchResults {
    pub callback: Option<unsafe fn(*mut Pokenav_SearchResults) -> u32>,
    pub loopedTaskId: u32,
    pub fill1: CArray<u8, 4>,
    pub boxId: i32,
    pub monId: i32,
    pub conditionDataId: u32,
    pub returnFromGraph: u32,
    pub saveResultsList: u32,
    pub monList: *mut PokenavMonList,
}

unsafe impl Sync for Pokenav_SearchResults {}

/// `struct Pokenav_SearchResultsGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_SearchResultsGfx {
    pub callback: Option<unsafe fn() -> u32>,
    pub loopedTaskId: u32,
    pub winid: u16,
    pub fromGraph: u32,
    pub buff: CArray<u8, 2048>,
}

unsafe impl Sync for Pokenav_SearchResultsGfx {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_SearchResults>() == 36);
    assert!(offset_of!(Pokenav_SearchResults, callback) == 0);
    assert!(offset_of!(Pokenav_SearchResults, loopedTaskId) == 4);
    assert!(offset_of!(Pokenav_SearchResults, fill1) == 8);
    assert!(offset_of!(Pokenav_SearchResults, boxId) == 12);
    assert!(offset_of!(Pokenav_SearchResults, monId) == 16);
    assert!(offset_of!(Pokenav_SearchResults, conditionDataId) == 20);
    assert!(offset_of!(Pokenav_SearchResults, returnFromGraph) == 24);
    assert!(offset_of!(Pokenav_SearchResults, saveResultsList) == 28);
    assert!(offset_of!(Pokenav_SearchResults, monList) == 32);
    assert!(size_of::<Pokenav_SearchResultsGfx>() == 2064);
    assert!(offset_of!(Pokenav_SearchResultsGfx, callback) == 0);
    assert!(offset_of!(Pokenav_SearchResultsGfx, loopedTaskId) == 4);
    assert!(offset_of!(Pokenav_SearchResultsGfx, winid) == 8);
    assert!(offset_of!(Pokenav_SearchResultsGfx, fromGraph) == 12);
    assert!(offset_of!(Pokenav_SearchResultsGfx, buff) == 16);
};

const CONDITION_SEARCH_FUNC_EXIT: u32 = 5;
const CONDITION_SEARCH_FUNC_MOVE_DOWN: u32 = 2;
const CONDITION_SEARCH_FUNC_MOVE_UP: u32 = 1;
const CONDITION_SEARCH_FUNC_NONE: u32 = 0;
const CONDITION_SEARCH_FUNC_PAGE_DOWN: u32 = 4;
const CONDITION_SEARCH_FUNC_PAGE_UP: u32 = 3;
const CONDITION_SEARCH_FUNC_SELECT_MON: u32 = 6;

static sConditionSearchLoopedTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 4>> = Table(
    (&raw const crate::data::pokenav_conditions_search_results::sConditionSearchLoopedTaskFuncs)
        .cast(),
);
static sConditionSearchResultBgTemplates: Table<CArray<BgTemplate, 2>> = Table(
    (&raw const crate::data::pokenav_conditions_search_results::sConditionSearchResultBgTemplates)
        .cast(),
);
static sConditionSearchResultFramePal: Table<CArray<u16, 16>> = Table(
    (&raw const crate::data::pokenav_conditions_search_results::sConditionSearchResultFramePal)
        .cast(),
);
static sConditionSearchResultTilemap: Table<CArray<u32, 49>> = Table(
    (&raw const crate::data::pokenav_conditions_search_results::sConditionSearchResultTilemap)
        .cast(),
);
static sConditionSearchResultTiles: Table<CArray<u32, 50>> = Table(
    (&raw const crate::data::pokenav_conditions_search_results::sConditionSearchResultTiles).cast(),
);
static sListBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_conditions_search_results::sListBg_Pal).cast());
static sSearchMonDataIds: Table<CArray<u32, 5>> =
    Table((&raw const crate::data::pokenav_conditions_search_results::sSearchMonDataIds).cast());
static sSearchResultListMenuWindowTemplate: Table<WindowTemplate> = Table((&raw const crate::data::pokenav_conditions_search_results::sSearchResultListMenuWindowTemplate).cast());
static sSearchResultLoopTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 7>> = Table(
    (&raw const crate::data::pokenav_conditions_search_results::sSearchResultLoopTaskFuncs).cast(),
);
static sText_FemaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_conditions_search_results::sText_FemaleSymbol).cast());
static sText_MaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_conditions_search_results::sText_MaleSymbol).cast());
static sText_NoGenderSymbol: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokenav_conditions_search_results::sText_NoGenderSymbol).cast());

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

pub unsafe fn PokenavCallback_Init_ConditionSearch() -> u32 {
    let menu: *mut Pokenav_SearchResults =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS, 36)
            as *mut Pokenav_SearchResults;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).monList = AllocSubstruct(POKENAV_SUBSTRUCT_MON_LIST, 1708) as *mut PokenavMonList;
    if (*menu).monList.is_null() {
        return FALSE as u32;
    }
    (*menu).callback = Some(HandleConditionSearchInput_WaitSetup);
    (*menu).loopedTaskId = CreateLoopedTask(Some(GetConditionSearchLoopedTask), 1);
    (*menu).returnFromGraph = FALSE as u32;
    (*menu).conditionDataId = sSearchMonDataIds[GetSelectedConditionSearch()];
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_ReturnToMonSearchList() -> u32 {
    let menu: *mut Pokenav_SearchResults =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS, 36)
            as *mut Pokenav_SearchResults;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).monList = GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    (*menu).callback = Some(HandleConditionSearchInput);
    (*menu).returnFromGraph = TRUE as u32;
    (*menu).conditionDataId = sSearchMonDataIds[GetSelectedConditionSearch()];
    TRUE as u32
}
pub unsafe fn GetConditionSearchResultsCallback() -> u32 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*menu).callback.unwrap_unchecked()(menu)
}
pub unsafe fn FreeSearchResultSubstruct1() {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    if (*menu).saveResultsList == 0 {
        FreePokenavSubstruct(POKENAV_SUBSTRUCT_MON_LIST);
    }
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS);
}
pub(crate) unsafe fn HandleConditionSearchInput_WaitSetup(menu: *mut Pokenav_SearchResults) -> u32 {
    if IsLoopedTaskActive((*menu).loopedTaskId) == 0 {
        (*menu).callback = Some(HandleConditionSearchInput);
    }
    FALSE as u32
}
pub(crate) unsafe fn HandleConditionSearchInput(menu: *mut Pokenav_SearchResults) -> u32 {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        return CONDITION_SEARCH_FUNC_MOVE_UP;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        return CONDITION_SEARCH_FUNC_MOVE_DOWN;
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        return CONDITION_SEARCH_FUNC_PAGE_UP;
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        return CONDITION_SEARCH_FUNC_PAGE_DOWN;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*menu).saveResultsList = FALSE as u32;
        (*menu).callback = Some(ReturnToConditionSearchList);
        return CONDITION_SEARCH_FUNC_EXIT;
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*(*menu).monList).currIndex = PokenavList_GetSelectedIndex() as u16;
        (*menu).saveResultsList = TRUE as u32;
        (*menu).callback = Some(OpenConditionGraphFromSearchList);
        return CONDITION_SEARCH_FUNC_SELECT_MON;
    } else {
        return CONDITION_SEARCH_FUNC_NONE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn ReturnToConditionSearchList(menu: *mut Pokenav_SearchResults) -> u32 {
    POKENAV_CONDITION_SEARCH_MENU
}
pub(crate) unsafe fn OpenConditionGraphFromSearchList(menu: *mut Pokenav_SearchResults) -> u32 {
    POKENAV_CONDITION_GRAPH_SEARCH
}
unsafe fn GetReturningFromGraph() -> u32 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*menu).returnFromGraph
}
unsafe fn GetSearchResultsMonDataList() -> *mut PokenavMonListItem {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*(*menu).monList).monData.as_mut_ptr()
}
unsafe fn GetSearchResultsMonListCount() -> u16 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*(*menu).monList).listCount
}
unsafe fn GetSearchResultsSelectedMonRank() -> i32 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    let i: i32 = PokenavList_GetSelectedIndex() as i32;
    (*(*menu).monList).monData[i].data as i32
}
unsafe fn GetSearchResultsCurrentListIndex() -> u16 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*(*menu).monList).currIndex
}
pub(crate) unsafe fn GetConditionSearchLoopedTask(state: i32) -> u32 {
    sConditionSearchLoopedTaskFuncs[state].unwrap_unchecked()(state)
}
pub(crate) unsafe fn BuildPartyMonSearchResults(state: i32) -> u32 {
    let mut item: PokenavMonListItem = zeroed();
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*(*menu).monList).listCount = 0;
    (*(*menu).monList).currIndex = 0;
    item.boxId = TOTAL_BOXES_COUNT;
    for i in 0..PARTY_SIZE {
        let pokemon: *mut Pokemon = &raw mut gPlayerParty[i];
        if GetMonData2(pokemon, MON_DATA_SANITY_HAS_SPECIES) == 0 {
            return LT_INC_AND_CONTINUE;
        }
        if GetMonData2(pokemon, MON_DATA_SANITY_IS_EGG) == 0 {
            item.monId = i as u8;
            item.data = GetMonData2(pokemon, (*menu).conditionDataId as i32) as u16;
            InsertMonListItem(menu, &raw mut item);
        }
    }
    LT_INC_AND_CONTINUE
}
pub(crate) unsafe fn InitBoxMonSearchResults(state: i32) -> u32 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    (*menu).monId = 0;
    (*menu).boxId = 0;
    LT_INC_AND_CONTINUE
}
pub(crate) unsafe fn BuildBoxMonSearchResults(state: i32) -> u32 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    let mut boxId: i32 = (*menu).boxId;
    let mut monId: i32 = (*menu).monId;
    let mut boxCount: i32 = 0;
    let mut item: PokenavMonListItem = zeroed();
    while boxId < TOTAL_BOXES_COUNT as i32 {
        while monId < IN_BOX_COUNT {
            if CheckBoxMonSanityAt(boxId as u32, monId as u32) != 0 {
                item.boxId = boxId as u8;
                item.monId = monId as u8;
                item.data =
                    GetBoxMonDataAt(boxId as u8, monId as u8, (*menu).conditionDataId as i32)
                        as u16;
                InsertMonListItem(menu, &raw mut item);
            }
            boxCount += 1;
            monId += 1;
            if boxCount > TOTAL_BOXES_COUNT as i32 {
                (*menu).boxId = boxId;
                (*menu).monId = monId;
                return LT_CONTINUE;
            }
        }
        monId = 0;
        boxId += 1;
    }
    LT_INC_AND_CONTINUE
}
pub(crate) unsafe fn ConvertConditionsToListRanks(state: i32) -> u32 {
    let menu: *mut Pokenav_SearchResults =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS) as *mut Pokenav_SearchResults;
    let listCount: i32 = (*(*menu).monList).listCount as i32;
    let mut prevCondition: i32 = (*(*menu).monList).monData[0].data as i32;
    (*(*menu).monList).monData[0].data = 1;
    for i in 1..listCount {
        if (*(*menu).monList).monData[i].data as i32 == prevCondition {
            (*(*menu).monList).monData[i].data = (*(*menu).monList).monData[i - 1].data;
        } else {
            prevCondition = (*(*menu).monList).monData[i].data as i32;
            (*(*menu).monList).monData[i].data = i as u16 + 1;
        }
    }
    (*menu).returnFromGraph = TRUE as u32;
    LT_FINISH
}
pub(crate) unsafe fn InsertMonListItem(
    menu: *mut Pokenav_SearchResults,
    item: *mut PokenavMonListItem,
) {
    let mut left: u32 = 0;
    let mut right: u32 = (*(*menu).monList).listCount as u32;
    let mut insertionIdx: u32 = left + (right - left) / 2;
    while right != insertionIdx {
        if (*item).data > (*(*menu).monList).monData[insertionIdx].data {
            right = insertionIdx;
        } else {
            left = insertionIdx + 1;
        }
        insertionIdx = left + (right - left) / 2;
    }
    right = (*(*menu).monList).listCount as u32;
    while right > insertionIdx {
        (*(*menu).monList).monData[right] = (*(*menu).monList).monData[right - 1];
        right -= 1;
    }
    (*(*menu).monList).monData[insertionIdx] = *item;
    (*(*menu).monList).listCount += 1;
}
pub unsafe fn OpenConditionSearchResults() -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX, 2064)
            as *mut Pokenav_SearchResultsGfx;
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenConditionSearchResults), 1);
    (*gfx).callback = Some(GetSearchResultCurrentLoopedTaskActive);
    (*gfx).fromGraph = FALSE as u32;
    TRUE as u32
}
pub unsafe fn OpenConditionSearchListFromGraph() -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX, 2064)
            as *mut Pokenav_SearchResultsGfx;
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenConditionSearchResults), 1);
    (*gfx).callback = Some(GetSearchResultCurrentLoopedTaskActive);
    (*gfx).fromGraph = TRUE as u32;
    TRUE as u32
}
pub unsafe fn CreateSearchResultsLoopedTask(idx: i32) {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
    (*gfx).loopedTaskId = CreateLoopedTask(sSearchResultLoopTaskFuncs[idx], 1);
    (*gfx).callback = Some(GetSearchResultCurrentLoopedTaskActive);
}
pub unsafe fn IsSearchResultLoopedTaskActive() -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
    (*gfx).callback.unwrap_unchecked()()
}
pub unsafe fn GetSearchResultCurrentLoopedTaskActive() -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
    IsLoopedTaskActive((*gfx).loopedTaskId)
}
pub unsafe fn FreeSearchResultSubstruct2() {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
    DestroyPokenavList();
    RemoveWindow((*gfx).winid as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX);
}
pub(crate) unsafe fn LoopedTask_OpenConditionSearchResults(state: i32) -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
    match state {
        0 => {
            InitBgTemplates(sConditionSearchResultBgTemplates.as_ptr().cast_mut(), 2);
            DecompressAndCopyTileDataToVram(
                1,
                sConditionSearchResultTiles.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(1, (*gfx).buff.as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                1,
                sConditionSearchResultTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyPaletteIntoBufferUnfaded(
                sConditionSearchResultFramePal.as_ptr().cast_mut(),
                16,
                32,
            );
            CopyBgTilemapBufferToVram(1);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            if GetReturningFromGraph() == 0 {
                return LT_PAUSE;
            }
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            CopyPaletteIntoBufferUnfaded(sListBg_Pal.as_ptr().cast_mut(), 32, 32);
            CreateSearchResultsList();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsCreatePokenavListTaskActive() != 0 {
                return LT_PAUSE;
            }
            AddSearchResultListMenuWindow(gfx);
            PrintHelpBarText(HELPBAR_CONDITION_MON_LIST);
            return LT_INC_AND_PAUSE;
        }
        4 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ShowBg(1);
            ShowBg(2);
            HideBg(3);
            if (*gfx).fromGraph == 0 {
                let searchGfxId: u8 =
                    GetSelectedConditionSearch() as u8 + POKENAV_MENUITEM_CONDITION_SEARCH_COOL;
                LoadLeftHeaderGfxForIndex(searchGfxId as u32);
                ShowLeftHeaderGfx(searchGfxId as u32, TRUE as u32, FALSE as u32);
                ShowLeftHeaderGfx(POKENAV_GFX_CONDITION_MENU, TRUE as u32, FALSE as u32);
            }
            PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
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
pub(crate) unsafe fn LoopedTask_MoveSearchListCursorUp(state: i32) -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
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
            PrintSearchResultListMenuItems(gfx);
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
pub(crate) unsafe fn LoopedTask_MoveSearchListCursorDown(state: i32) -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
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
            PrintSearchResultListMenuItems(gfx);
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
pub(crate) unsafe fn LoopedTask_MoveSearchListPageUp(state: i32) -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
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
            PrintSearchResultListMenuItems(gfx);
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
pub(crate) unsafe fn LoopedTask_MoveSearchListPageDown(state: i32) -> u32 {
    let gfx: *mut Pokenav_SearchResultsGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_SEARCH_RESULTS_GFX)
            as *mut Pokenav_SearchResultsGfx;
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
            PrintSearchResultListMenuItems(gfx);
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
pub(crate) unsafe fn LoopedTask_ExitConditionSearchMenu(state: i32) -> u32 {
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
pub(crate) unsafe fn LoopedTask_SelectSearchResult(state: i32) -> u32 {
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
unsafe fn AddSearchResultListMenuWindow(gfx: *mut Pokenav_SearchResultsGfx) {
    (*gfx).winid = AddWindow((&raw const *sSearchResultListMenuWindowTemplate).cast_mut());
    PutWindowTilemap((*gfx).winid as u8);
    CopyWindowToVram((*gfx).winid as u8, COPYWIN_MAP);
    PrintSearchResultListMenuItems(gfx);
}
unsafe fn PrintSearchResultListMenuItems(gfx: *mut Pokenav_SearchResultsGfx) {
    let rank: i32 = GetSearchResultsSelectedMonRank();
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
    *gStringVar1.as_mut_ptr() = EOS;
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        gStringVar2.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_NumberIndex).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        (*gfx).winid as u8,
        FONT_NORMAL,
        gStringVar2.as_mut_ptr(),
        4,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    ConvertIntToDecimalStringN(gStringVar1.as_mut_ptr(), rank, STR_CONV_MODE_RIGHT_ALIGN, 3);
    AddTextPrinterParameterized(
        (*gfx).winid as u8,
        FONT_NORMAL,
        gStringVar1.as_mut_ptr(),
        34,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram((*gfx).winid as u8, COPYWIN_GFX);
}
unsafe fn CreateSearchResultsList() {
    let mut template: PokenavListTemplate = zeroed();
    template.list = GetSearchResultsMonDataList() as *mut PokenavListItem;
    template.count = GetSearchResultsMonListCount();
    template.itemSize = 4;
    template.startIndex = GetSearchResultsCurrentListIndex();
    template.item_X = 13;
    template.windowWidth = 17;
    template.listTop = 1;
    template.maxShowed = 8;
    template.fillValue = 2;
    template.fontId = FONT_NORMAL;
    template.bufferItemFunc = core::mem::transmute::<
        Option<unsafe fn(*mut PokenavMonListItem, *mut u8)>,
        Option<unsafe fn(*mut PokenavListItem, *mut u8)>,
    >(Some(BufferSearchMonListItem));
    template.iconDrawFunc = None;
    CreatePokenavList(
        (&raw const sConditionSearchResultBgTemplates[1]).cast_mut(),
        &raw mut template,
        0,
    );
}
pub(crate) unsafe fn BufferSearchMonListItem(item: *mut PokenavMonListItem, mut dest: *mut u8) {
    let mut gender: u8 = 0;
    let mut level: u8 = 0;
    let mut genderStr: *mut u8 = null_mut();
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
    GetStringClearToWidth(dest, FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 40);
}
