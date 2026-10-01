//! Translated from `src/pokenav_menu_handler.c` by tools/rustport/c2rs.py.
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
    pub callback: Option<unsafe extern "C" fn(*mut Pokenav_Menu) -> u32>,
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

unsafe extern "C" {
    static mut gMain: Main;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn CanViewRibbonsMenu() -> u32;
    fn FlagGet(a0: u16) -> u8;
    fn FreePokenavSubstruct(a0: u32);
    fn GetPokenavMode() -> u32;
    fn GetSelectedConditionSearch() -> u32;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn PlaySE(a0: u16);
    fn SetPokenavMode(a0: u16);
    fn SetSelectedConditionSearch(a0: u32);
}

pub(crate) unsafe extern "C" fn GetPokenavMainMenuType() -> u8 {
    let mut menuType: u8 = POKENAV_MENU_TYPE_DEFAULT;
    if FlagGet(FLAG_ADDED_MATCH_CALL_TO_POKENAV) != 0 {
        menuType = POKENAV_MENU_TYPE_UNLOCK_MC;
        if FlagGet(FLAG_SYS_RIBBON_GET) != 0 {
            menuType = POKENAV_MENU_TYPE_UNLOCK_MC_RIBBONS;
        }
    }
    return menuType;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MainMenuCursorOnMap() -> u32 {
    let mut menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = POKENAV_MENUITEM_MAP;
    (*menu).currMenuItem = POKENAV_MENUITEM_MAP as u16;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MainMenuCursorOnMatchCall() -> u32 {
    let mut menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = POKENAV_MENUITEM_MATCH_CALL as i16;
    (*menu).currMenuItem = POKENAV_MENUITEM_MATCH_CALL as u16;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MainMenuCursorOnRibbons() -> u32 {
    let mut menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = POKENAV_MENUITEM_RIBBONS;
    (*menu).currMenuItem = POKENAV_MENUITEM_RIBBONS as u16;
    SetMenuInputHandler(menu);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionMenu() -> u32 {
    let mut menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = POKENAV_MENU_TYPE_CONDITION;
    (*menu).cursorPos = 0;
    (*menu).currMenuItem = POKENAV_MENUITEM_CONDITION_PARTY;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionSearchMenu() -> u32 {
    let mut menu: *mut Pokenav_Menu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER, 16) as *mut Pokenav_Menu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).menuType = POKENAV_MENU_TYPE_CONDITION_SEARCH as u16;
    (*menu).cursorPos = GetSelectedConditionSearch() as i16;
    (*menu).currMenuItem = (*menu).cursorPos as u16 + POKENAV_MENUITEM_CONDITION_SEARCH_COOL as u16;
    (*menu).helpBarIndex = HELPBAR_NONE;
    SetMenuInputHandler(menu);
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn SetMenuInputHandler(menu: *mut Pokenav_Menu) {
    'l1: {
        let sw1: u16 = (*menu).menuType;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetPokenavMode(POKENAV_MODE_NORMAL);
        }
        if fall || sw1 == 1 || sw1 == 2 {
            fall = true;
            (*menu).callback = GetMainMenuInputHandler();
            break 'l1;
        }
        if sw1 == POKENAV_MENU_TYPE_CONDITION {
            fall = true;
            (*menu).callback = Some(HandleConditionMenuInput);
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            (*menu).callback = Some(HandleConditionSearchMenuInput);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn GetMainMenuInputHandler()
-> Option<unsafe extern "C" fn(*mut Pokenav_Menu) -> u32> {
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
        return None;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMenuHandlerCallback() -> u32 {
    let mut menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    return (*menu).callback.unwrap_unchecked()(menu);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMenuHandlerSubstruct1() {
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER);
}
pub(crate) unsafe extern "C" fn HandleMainMenuInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        'l1: {
            let sw1: u8 = sMenuItems[(*menu).menuType][(*menu).cursorPos];
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
                (*menu).helpBarIndex = (if (*gSaveBlock2Ptr).regionMapZoom() != 0 {
                    HELPBAR_MAP_ZOOMED_IN
                } else {
                    HELPBAR_MAP_ZOOMED_OUT
                }) as u16;
                SetMenuIdAndCB(menu, POKENAV_REGION_MAP);
                return POKENAV_MENU_FUNC_OPEN_FEATURE;
            }
            if sw1 == POKENAV_MENUITEM_CONDITION {
                fall = true;
                (*menu).menuType = POKENAV_MENU_TYPE_CONDITION;
                (*menu).cursorPos = 0;
                (*menu).currMenuItem = sMenuItems[3][0] as u16;
                (*menu).callback = Some(HandleConditionMenuInput);
                return POKENAV_MENU_FUNC_OPEN_CONDITION;
            }
            if sw1 == POKENAV_MENUITEM_MATCH_CALL {
                fall = true;
                (*menu).helpBarIndex = HELPBAR_MC_TRAINER_LIST as u16;
                SetMenuIdAndCB(menu, POKENAV_MATCH_CALL);
                return POKENAV_MENU_FUNC_OPEN_FEATURE;
            }
            if sw1 == 3 {
                fall = true;
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
                fall = true;
                return POKENAV_MENU_FUNC_EXIT as u32;
            }
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        return POKENAV_MENU_FUNC_EXIT as u32;
    }
    return POKENAV_MENU_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleMainMenuInputTutorial(menu: *mut Pokenav_Menu) -> u32 {
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
    return POKENAV_MENU_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleMainMenuInputEndTutorial(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        let mut menuItem: u32 = sMenuItems[(*menu).menuType][(*menu).cursorPos] as u32;
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
    return POKENAV_MENU_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleCantOpenRibbonsInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        (*menu).callback = GetMainMenuInputHandler();
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & 3 != 0 {
        (*menu).callback = GetMainMenuInputHandler();
        return POKENAV_MENU_FUNC_RESHOW_DESCRIPTION;
    }
    return POKENAV_MENU_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleConditionMenuInput(menu: *mut Pokenav_Menu) -> u32 {
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
    return POKENAV_MENU_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleConditionSearchMenuInput(menu: *mut Pokenav_Menu) -> u32 {
    if UpdateMenuCursorPos(menu) != 0 {
        return POKENAV_MENU_FUNC_MOVE_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        let mut menuItem: u8 = sMenuItems[(*menu).menuType][(*menu).cursorPos];
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
    return POKENAV_MENU_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn CB2_ReturnToMainMenu(menu: *mut Pokenav_Menu) -> u32 {
    ReturnToMainMenu(menu);
    return POKENAV_MENU_FUNC_RETURN_TO_MAIN;
}
pub(crate) unsafe extern "C" fn CB2_ReturnToConditionMenu(menu: *mut Pokenav_Menu) -> u32 {
    ReturnToConditionMenu(menu);
    return POKENAV_MENU_FUNC_RETURN_TO_CONDITION;
}
pub(crate) unsafe extern "C" fn SetMenuIdAndCB(menu: *mut Pokenav_Menu, menuId: u32) {
    (*menu).menuId = menuId;
    (*menu).callback = Some(GetMenuId);
}
pub(crate) unsafe extern "C" fn GetMenuId(menu: *mut Pokenav_Menu) -> u32 {
    return (*menu).menuId;
}
pub(crate) unsafe extern "C" fn ReturnToMainMenu(menu: *mut Pokenav_Menu) {
    (*menu).menuType = GetPokenavMainMenuType() as u16;
    (*menu).cursorPos = 1;
    (*menu).currMenuItem = sMenuItems[(*menu).menuType][(*menu).cursorPos] as u16;
    (*menu).callback = Some(HandleMainMenuInput);
}
pub(crate) unsafe extern "C" fn ReturnToConditionMenu(menu: *mut Pokenav_Menu) {
    (*menu).menuType = POKENAV_MENU_TYPE_CONDITION;
    (*menu).cursorPos = 1;
    (*menu).currMenuItem = sMenuItems[3][1] as u16;
    (*menu).callback = Some(HandleConditionMenuInput);
}
pub(crate) unsafe extern "C" fn UpdateMenuCursorPos(menu: *mut Pokenav_Menu) -> u32 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokenavMenuType() -> i32 {
    let mut menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    return (*menu).menuType as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokenavCursorPos() -> i32 {
    let mut menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    return (*menu).cursorPos as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMenuItemId() -> i32 {
    let mut menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    return (*menu).currMenuItem as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHelpBarTextId() -> u16 {
    let mut menu: *mut Pokenav_Menu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU_HANDLER) as *mut Pokenav_Menu;
    return (*menu).helpBarIndex;
}
