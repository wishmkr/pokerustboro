//! Translated from `src/pokenav_menu_handler.c` by tools/rustport/c2rs.py.
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
    unused_labels
)]

use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::FlagGet;
use crate::load_save::gSaveBlock2Ptr;
use crate::pokenav::{
    AllocSubstruct, CanViewRibbonsMenu, FreePokenavSubstruct, GetPokenavMode,
    GetSelectedConditionSearch, GetSubstructPtr, SetPokenavMode, SetSelectedConditionSearch,
};
use crate::sound::PlaySE;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sLastCursorPositions sMenuItems

/// `struct Pokenav_Menu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_Menu {
    pub menuType: u16,
    pub cursorPos: i16,
    pub currMenuItem: u16,
    pub helpBarIndex: u16,
    pub menuId: u32,
    pub callback: Option<unsafe fn(*mut Pokenav_Menu) -> u32>,
}

unsafe impl Sync for Pokenav_Menu {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_Menu>() == 16);
    assert!(offset_of!(Pokenav_Menu, menuType) == 0);
    assert!(offset_of!(Pokenav_Menu, cursorPos) == 2);
    assert!(offset_of!(Pokenav_Menu, currMenuItem) == 4);
    assert!(offset_of!(Pokenav_Menu, helpBarIndex) == 6);
    assert!(offset_of!(Pokenav_Menu, menuId) == 8);
    assert!(offset_of!(Pokenav_Menu, callback) == 12);
};

static sLastCursorPositions: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::pokenav_menu_handler::sLastCursorPositions).cast());
static sMenuItems: Table<CArray<CArray<u8, 6>, 5>> =
    Table((&raw const crate::data::pokenav_menu_handler::sMenuItems).cast());

unsafe fn GetPokenavMainMenuType() -> u8 {
    let mut menuType: u8 = POKENAV_MENU_TYPE_DEFAULT;
    if FlagGet(FLAG_ADDED_MATCH_CALL_TO_POKENAV) != 0 {
        menuType = POKENAV_MENU_TYPE_UNLOCK_MC;
        if FlagGet(FLAG_SYS_RIBBON_GET) != 0 {
            menuType = POKENAV_MENU_TYPE_UNLOCK_MC_RIBBONS;
        }
    }
    menuType
}
pub unsafe fn PokenavCallback_Init_MainMenuCursorOnMap() -> u32 {
    let menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = POKENAV_MENUITEM_MAP;
    (*menu).currMenuItem = POKENAV_MENUITEM_MAP as u16;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_MainMenuCursorOnMatchCall() -> u32 {
    let menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = POKENAV_MENUITEM_MATCH_CALL as i16;
    (*menu).currMenuItem = POKENAV_MENUITEM_MATCH_CALL as u16;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_MainMenuCursorOnRibbons() -> u32 {
    let menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = POKENAV_MENUITEM_RIBBONS;
    (*menu).currMenuItem = POKENAV_MENUITEM_RIBBONS as u16;
    SetMenuInputHandler(menu);
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_ConditionMenu() -> u32 {
    let menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = POKENAV_MENU_TYPE_CONDITION;
    (*menu).cursorPos = 0;
    (*menu).currMenuItem = POKENAV_MENUITEM_CONDITION_PARTY;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_ConditionSearchMenu() -> u32 {
    let menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = POKENAV_MENU_TYPE_CONDITION_SEARCH as u16;
    (*menu).cursorPos = GetSelectedConditionSearch() as i16;
    (*menu).currMenuItem = (*menu).cursorPos as u16 + POKENAV_MENUITEM_CONDITION_SEARCH_COOL as u16;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    TRUE as u32
}
unsafe fn SetMenuInputHandler(menu: *mut Pokenav_Menu) {
    'l1: {
        let sw1: u16 = (*menu).menuType;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetPokenavMode(POKENAV_MODE_NORMAL);
        }
        if fall || sw1 == 1 || sw1 == 2 {
            (*menu).callback = GetMainMenuInputHandler();
            break 'l1;
        }
        if sw1 == POKENAV_MENU_TYPE_CONDITION {
            (*menu).callback = Some(HandleConditionMenuInput);
            break 'l1;
        }
        if sw1 == 4 {
            (*menu).callback = Some(HandleConditionSearchMenuInput);
            break 'l1;
        }
    }
}
unsafe fn GetMainMenuInputHandler() -> Option<unsafe fn(*mut Pokenav_Menu) -> u32> {
    match GetPokenavMode() {
        POKENAV_MODE_FORCE_CALL_READY => {
            return Some(HandleMainMenuInputTutorial);
        }
        2 => {
            return Some(HandleMainMenuInputEndTutorial);
        }
        _ => {
            return Some(HandleMainMenuInput);
        }
    }
    #[allow(unreachable_code)]
    {
        None
    }
}
pub unsafe fn GetMenuHandlerCallback() -> u32 {
    let menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    (*menu).callback.unwrap_unchecked()(menu)
}
pub unsafe fn FreeMenuHandlerSubstruct1() {
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER);
}
pub(crate) unsafe fn HandleMainMenuInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        'l1: {
            let sw1: u8 = sMenuItems[(*menu).menuType][(*menu).cursorPos];
            let fall = false;
            if sw1 == 0 {
                (*menu).helpBarIndex = (if (*gSaveBlock2Ptr).regionMapZoom() != 0 {
                    HELPBAR_MAP_ZOOMED_IN
                } else {
                    HELPBAR_MAP_ZOOMED_OUT
                }) as u16;
                SetMenuIdAndCB(menu, POKENAV_REGION_MAP);
                return POKENAV_MENU_FUNC_OPEN_FEATURE;
            }
            if sw1 == POKENAV_MENUITEM_CONDITION {
                (*menu).menuType = POKENAV_MENU_TYPE_CONDITION;
                (*menu).cursorPos = 0;
                (*menu).currMenuItem = sMenuItems[3][0] as u16;
                (*menu).callback = Some(HandleConditionMenuInput);
                return POKENAV_MENU_FUNC_OPEN_CONDITION;
            }
            if sw1 == POKENAV_MENUITEM_MATCH_CALL {
                (*menu).helpBarIndex = HELPBAR_MC_TRAINER_LIST as u16;
                SetMenuIdAndCB(menu, POKENAV_MATCH_CALL);
                return POKENAV_MENU_FUNC_OPEN_FEATURE;
            }
            if sw1 == 3 {
                if CanViewRibbonsMenu() != 0 {
                    (*menu).helpBarIndex = HELPBAR_RIBBONS_MON_LIST;
                    SetMenuIdAndCB(menu, POKENAV_RIBBONS_MON_LIST);
                    return POKENAV_MENU_FUNC_OPEN_FEATURE;
                } else {
                    (*menu).callback = Some(HandleCantOpenRibbonsInput);
                    return POKENAV_MENU_FUNC_NO_RIBBON_WINNERS;
                }
            }
            if fall || sw1 == POKENAV_MENUITEM_SWITCH_OFF {
                return POKENAV_MENU_FUNC_EXIT as u32;
            }
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        return POKENAV_MENU_FUNC_EXIT as u32;
    }
    POKENAV_MENU_FUNC_NONE
}
pub(crate) unsafe fn HandleMainMenuInputTutorial(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if sMenuItems[(*menu).menuType][(*menu).cursorPos] == POKENAV_MENUITEM_MATCH_CALL {
            (*menu).helpBarIndex = HELPBAR_MC_TRAINER_LIST as u16;
            SetMenuIdAndCB(menu, POKENAV_MATCH_CALL);
            return POKENAV_MENU_FUNC_OPEN_FEATURE;
        } else {
            PlaySE(SE_FAILURE);
            return POKENAV_MENU_FUNC_NONE;
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_FAILURE);
        return POKENAV_MENU_FUNC_NONE;
    }
    POKENAV_MENU_FUNC_NONE
}
pub(crate) unsafe fn HandleMainMenuInputEndTutorial(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        let menuItem: u32 = sMenuItems[(*menu).menuType][(*menu).cursorPos] as u32;
        if menuItem != POKENAV_MENUITEM_MATCH_CALL as u32
            && menuItem != POKENAV_MENUITEM_SWITCH_OFF as u32
        {
            PlaySE(SE_FAILURE);
            return POKENAV_MENU_FUNC_NONE;
        } else if menuItem == POKENAV_MENUITEM_MATCH_CALL as u32 {
            (*menu).helpBarIndex = HELPBAR_MC_TRAINER_LIST as u16;
            SetMenuIdAndCB(menu, POKENAV_MATCH_CALL);
            return POKENAV_MENU_FUNC_OPEN_FEATURE;
        } else {
            return 0xffffffff;
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return 0xffffffff;
    }
    POKENAV_MENU_FUNC_NONE
}
pub(crate) unsafe fn HandleCantOpenRibbonsInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        (*menu).callback = GetMainMenuInputHandler();
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & 3 != 0 {
        (*menu).callback = GetMainMenuInputHandler();
        return POKENAV_MENU_FUNC_RESHOW_DESCRIPTION;
    }
    POKENAV_MENU_FUNC_NONE
}
pub(crate) unsafe fn HandleConditionMenuInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        match sMenuItems[(*menu).menuType][(*menu).cursorPos] {
            POKENAV_MENUITEM_CONDITION_SEARCH => {
                (*menu).menuType = POKENAV_MENU_TYPE_CONDITION_SEARCH as u16;
                (*menu).cursorPos = 0;
                (*menu).currMenuItem = sMenuItems[4][0] as u16;
                (*menu).callback = Some(HandleConditionSearchMenuInput);
                return POKENAV_MENU_FUNC_OPEN_CONDITION_SEARCH;
            }
            5 => {
                (*menu).helpBarIndex = 0;
                SetMenuIdAndCB(menu, POKENAV_CONDITION_GRAPH_PARTY);
                return POKENAV_MENU_FUNC_OPEN_FEATURE;
            }
            POKENAV_MENUITEM_CONDITION_CANCEL => {
                PlaySE(SE_SELECT);
                ReturnToMainMenu(menu);
                return POKENAV_MENU_FUNC_RETURN_TO_MAIN;
            }
            _ => {}
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        if (*menu).cursorPos != sLastCursorPositions[(*menu).menuType] as i16 {
            (*menu).cursorPos = sLastCursorPositions[(*menu).menuType] as i16;
            (*menu).callback = Some(CB2_ReturnToMainMenu);
            return POKENAV_MENU_FUNC_MOVE_CURSOR;
        } else {
            PlaySE(SE_SELECT);
            ReturnToMainMenu(menu);
            return POKENAV_MENU_FUNC_RETURN_TO_MAIN;
        }
    }
    POKENAV_MENU_FUNC_NONE
}
pub(crate) unsafe fn HandleConditionSearchMenuInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        let menuItem: u8 = sMenuItems[(*menu).menuType][(*menu).cursorPos];
        if menuItem != POKENAV_MENUITEM_CONDITION_SEARCH_CANCEL {
            SetSelectedConditionSearch(
                menuItem as u32 - POKENAV_MENUITEM_CONDITION_SEARCH_COOL as u32,
            );
            SetMenuIdAndCB(menu, POKENAV_CONDITION_SEARCH_RESULTS);
            (*menu).helpBarIndex = HELPBAR_CONDITION_MON_LIST as u16;
            return POKENAV_MENU_FUNC_OPEN_FEATURE;
        } else {
            PlaySE(SE_SELECT);
            ReturnToConditionMenu(menu);
            return POKENAV_MENU_FUNC_RETURN_TO_CONDITION;
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        if (*menu).cursorPos != sLastCursorPositions[(*menu).menuType] as i16 {
            (*menu).cursorPos = sLastCursorPositions[(*menu).menuType] as i16;
            (*menu).callback = Some(CB2_ReturnToConditionMenu);
            return POKENAV_MENU_FUNC_MOVE_CURSOR;
        } else {
            PlaySE(SE_SELECT);
            ReturnToConditionMenu(menu);
            return POKENAV_MENU_FUNC_RETURN_TO_CONDITION;
        }
    }
    POKENAV_MENU_FUNC_NONE
}
pub(crate) unsafe fn CB2_ReturnToMainMenu(menu: *mut Pokenav_Menu) -> u32 {
    ReturnToMainMenu(menu);
    POKENAV_MENU_FUNC_RETURN_TO_MAIN
}
pub(crate) unsafe fn CB2_ReturnToConditionMenu(menu: *mut Pokenav_Menu) -> u32 {
    ReturnToConditionMenu(menu);
    POKENAV_MENU_FUNC_RETURN_TO_CONDITION
}
unsafe fn SetMenuIdAndCB(menu: *mut Pokenav_Menu, menuId: u32) {
    (*menu).menuId = menuId;
    (*menu).callback = Some(GetMenuId);
}
pub(crate) unsafe fn GetMenuId(menu: *mut Pokenav_Menu) -> u32 {
    (*menu).menuId
}
unsafe fn ReturnToMainMenu(menu: *mut Pokenav_Menu) {
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = 1;
    (*menu).currMenuItem = sMenuItems[(*menu).menuType][(*menu).cursorPos] as u16;
    (*menu).callback = Some(HandleMainMenuInput);
}
unsafe fn ReturnToConditionMenu(menu: *mut Pokenav_Menu) {
    (*menu).menuType = POKENAV_MENU_TYPE_CONDITION;
    (*menu).cursorPos = 1;
    (*menu).currMenuItem = sMenuItems[3][1] as u16;
    (*menu).callback = Some(HandleConditionMenuInput);
}
unsafe fn UpdateMenuCursorPos(menu: *mut Pokenav_Menu) -> u32 {
    if gMain.newKeys as i32 & DPAD_UP != 0 {
        if ({
            (*menu).cursorPos -= 1;
            (*menu).cursorPos
        }) < 0
        {
            (*menu).cursorPos = sLastCursorPositions[(*menu).menuType] as i16;
        }
        (*menu).currMenuItem = sMenuItems[(*menu).menuType][(*menu).cursorPos] as u16;
        return TRUE as u32;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        (*menu).cursorPos += 1;
        if (*menu).cursorPos > sLastCursorPositions[(*menu).menuType] as i16 {
            (*menu).cursorPos = 0;
        }
        (*menu).currMenuItem = sMenuItems[(*menu).menuType][(*menu).cursorPos] as u16;
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetPokenavMenuType() -> i32 {
    let menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    (*menu).menuType as i32
}
pub unsafe fn GetPokenavCursorPos() -> i32 {
    let menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    (*menu).cursorPos as i32
}
pub unsafe fn GetCurrentMenuItemId() -> i32 {
    let menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    (*menu).currMenuItem as i32
}
pub unsafe fn GetHelpBarTextId() -> u16 {
    let menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    (*menu).helpBarIndex
}
