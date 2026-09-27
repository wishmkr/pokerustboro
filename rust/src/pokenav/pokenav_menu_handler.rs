//! Translated from `src/pokenav_menu_handler.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sLastCursorPositions sMenuItems
#[allow(unused_imports)]
use crate::data::pokenav_menu_handler::*;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gSaveBlock2Ptr: u8;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn CanViewRibbonsMenu() -> u32;
    fn FlagGet(a0: u16) -> u8;
    fn FreePokenavSubstruct(a0: u32);
    fn GetPokenavMode() -> u32;
    fn GetSelectedConditionSearch() -> u32;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn PlaySE(a0: u16);
    fn SetPokenavMode(a0: u16);
    fn SetSelectedConditionSearch(a0: u32);
}

pub(crate) unsafe extern "C" fn GetPokenavMainMenuType() -> u8 {
    unsafe {
        let mut menuType: u8 = 0u8;
        if (FlagGet(304u16)) != 0 {
            menuType = 1u8;
            if (FlagGet(2203u16)) != 0 {
                menuType = 2u8;
            }
        }
        return menuType;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MainMenuCursorOnMap() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(1u32, 16u32);
        if !(!(menu).is_null()) {
            return 0u32;
        }
        ((menu).cast::<u16>()).write(((GetPokenavMainMenuType()) as u16));
        ((menu).wrapping_add(2).cast::<i16>()).write(0i16);
        ((menu).wrapping_add(4).cast::<u16>()).write(0u16);
        ((menu).wrapping_add(6).cast::<u16>()).write(0u16);
        SetMenuInputHandler(menu);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MainMenuCursorOnMatchCall() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(1u32, 16u32);
        if !(!(menu).is_null()) {
            return 0u32;
        }
        ((menu).cast::<u16>()).write(((GetPokenavMainMenuType()) as u16));
        ((menu).wrapping_add(2).cast::<i16>()).write(2i16);
        ((menu).wrapping_add(4).cast::<u16>()).write(2u16);
        ((menu).wrapping_add(6).cast::<u16>()).write(0u16);
        SetMenuInputHandler(menu);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MainMenuCursorOnRibbons() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(1u32, 16u32);
        if !(!(menu).is_null()) {
            return 0u32;
        }
        ((menu).cast::<u16>()).write(((GetPokenavMainMenuType()) as u16));
        ((menu).wrapping_add(2).cast::<i16>()).write(3i16);
        ((menu).wrapping_add(4).cast::<u16>()).write(3u16);
        SetMenuInputHandler(menu);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionMenu() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(1u32, 16u32);
        if !(!(menu).is_null()) {
            return 0u32;
        }
        ((menu).cast::<u16>()).write(3u16);
        ((menu).wrapping_add(2).cast::<i16>()).write(0i16);
        ((menu).wrapping_add(4).cast::<u16>()).write(5u16);
        ((menu).wrapping_add(6).cast::<u16>()).write(0u16);
        SetMenuInputHandler(menu);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionSearchMenu() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(1u32, 16u32);
        if !(!(menu).is_null()) {
            return 0u32;
        }
        ((menu).cast::<u16>()).write(4u16);
        ((menu).wrapping_add(2).cast::<i16>()).write(((GetSelectedConditionSearch()) as i16));
        ((menu).wrapping_add(4).cast::<u16>()).write(
            ((((((menu).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_add(8i32)) as u16),
        );
        ((menu).wrapping_add(6).cast::<u16>()).write(0u16);
        SetMenuInputHandler(menu);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn SetMenuInputHandler(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        'l1: {
            let __sw1 = ((((menu).cast::<u16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetPokenavMode(0u16);
            }
            if __fall || __sw1 == 1i32 || __sw1 == 2i32 {
                __fall = true;
                ((menu)
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(GetMainMenuInputHandler());
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                ((menu)
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(HandleConditionMenuInput));
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                ((menu)
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(HandleConditionSearchMenuInput));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMainMenuInputHandler()
-> Option<unsafe extern "C" fn(*mut u8) -> u32> {
    unsafe {
        'l1: {
            let __sw1 = GetPokenavMode();
            let __matched = __sw1 == 0u32 || __sw1 == 1u32 || __sw1 == 2u32;
            if __sw1 == 0u32 || !__matched {
                return Some(HandleMainMenuInput);
            }
            if __sw1 == 1u32 {
                return Some(HandleMainMenuInputTutorial);
            }
            if __sw1 == 2u32 {
                return Some(HandleMainMenuInputEndTutorial);
            }
        }
        #[allow(unreachable_code)]
        {
            return None;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMenuHandlerCallback() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(1u32);
        return (((menu)
            .wrapping_add(12)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .read())
        .unwrap_unchecked()(menu);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMenuHandlerSubstruct1() {
    unsafe {
        FreePokenavSubstruct(1u32);
    }
}
pub(crate) unsafe extern "C" fn HandleMainMenuInput(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if (UpdateMenuCursorPos(menu)) != 0 {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            'l1: {
                let __sw1 = ((((((((&raw const sMenuItems).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
                .cast::<u8>())
                .wrapping_offset(
                    ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize,
                ))
                .read()) as i32);
                let mut __fall = false;
                if __sw1 == 0i32 {
                    __fall = true;
                    ((menu).wrapping_add(6).cast::<u16>()).write(
                        ((if (crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                            3,
                            1,
                            false,
                        ) as u16)
                            != 0
                        {
                            2i32
                        } else {
                            1i32
                        }) as u16),
                    );
                    SetMenuIdAndCB(menu, 100006u32);
                    return 8u32;
                }
                if __sw1 == 1i32 {
                    __fall = true;
                    ((menu).cast::<u16>()).write(3u16);
                    ((menu).wrapping_add(2).cast::<i16>()).write(0i16);
                    ((menu).wrapping_add(4).cast::<u16>()).write(
                        (((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(18))
                        .cast::<u8>())
                        .read()) as u16),
                    );
                    ((menu)
                        .wrapping_add(12)
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                    .write(Some(HandleConditionMenuInput));
                    return 2u32;
                }
                if __sw1 == 2i32 {
                    __fall = true;
                    ((menu).wrapping_add(6).cast::<u16>()).write(6u16);
                    SetMenuIdAndCB(menu, 100011u32);
                    return 8u32;
                }
                if __sw1 == 3i32 {
                    __fall = true;
                    if (CanViewRibbonsMenu()) != 0 {
                        ((menu).wrapping_add(6).cast::<u16>()).write(9u16);
                        SetMenuIdAndCB(menu, 100012u32);
                        return 8u32;
                    } else {
                        ((menu)
                            .wrapping_add(12)
                            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                        .write(Some(HandleCantOpenRibbonsInput));
                        return 6u32;
                    }
                }
                if __fall || __sw1 == 4i32 {
                    __fall = true;
                    return 4294967295u32;
                }
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            return 4294967295u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleMainMenuInputTutorial(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if (UpdateMenuCursorPos(menu)) != 0 {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if ((((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
            .cast::<u8>())
            .wrapping_offset(((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize))
            .read()) as i32)
                == 2i32
            {
                ((menu).wrapping_add(6).cast::<u16>()).write(6u16);
                SetMenuIdAndCB(menu, 100011u32);
                return 8u32;
            } else {
                PlaySE(32u16);
                return 0u32;
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(32u16);
            return 0u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleMainMenuInputEndTutorial(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if (UpdateMenuCursorPos(menu)) != 0 {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            let mut menuItem: u32 = ((((((((&raw const sMenuItems).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
            .cast::<u8>())
            .wrapping_offset(((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize))
            .read()) as u32);
            if (menuItem != 2u32) && (menuItem != 4u32) {
                PlaySE(32u16);
                return 0u32;
            } else {
                if menuItem == 2u32 {
                    ((menu).wrapping_add(6).cast::<u16>()).write(6u16);
                    SetMenuIdAndCB(menu, 100011u32);
                    return 8u32;
                } else {
                    return 4294967295u32;
                }
            }
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return 4294967295u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleCantOpenRibbonsInput(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if (UpdateMenuCursorPos(menu)) != 0 {
            ((menu)
                .wrapping_add(12)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(GetMainMenuInputHandler());
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ((menu)
                .wrapping_add(12)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(GetMainMenuInputHandler());
            return 7u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleConditionMenuInput(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if (UpdateMenuCursorPos(menu)) != 0 {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            'l1: {
                let __sw1 = ((((((((&raw const sMenuItems).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
                .cast::<u8>())
                .wrapping_offset(
                    ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize,
                ))
                .read()) as i32);
                if __sw1 == 6i32 {
                    ((menu).cast::<u16>()).write(4u16);
                    ((menu).wrapping_add(2).cast::<i16>()).write(0i16);
                    ((menu).wrapping_add(4).cast::<u16>()).write(
                        (((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(24))
                        .cast::<u8>())
                        .read()) as u16),
                    );
                    ((menu)
                        .wrapping_add(12)
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                    .write(Some(HandleConditionSearchMenuInput));
                    return 4u32;
                }
                if __sw1 == 5i32 {
                    ((menu).wrapping_add(6).cast::<u16>()).write(0u16);
                    SetMenuIdAndCB(menu, 100007u32);
                    return 8u32;
                }
                if __sw1 == 7i32 {
                    PlaySE(5u16);
                    ReturnToMainMenu(menu);
                    return 3u32;
                }
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            if ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32)
                != ((((((&raw const sLastCursorPositions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize))
                .read()) as i32)
            {
                ((menu).wrapping_add(2).cast::<i16>()).write(
                    ((((((&raw const sLastCursorPositions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize))
                    .read()) as i16),
                );
                ((menu)
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(CB2_ReturnToMainMenu));
                return 1u32;
            } else {
                PlaySE(5u16);
                ReturnToMainMenu(menu);
                return 3u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleConditionSearchMenuInput(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if (UpdateMenuCursorPos(menu)) != 0 {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            let mut menuItem: u8 = ((((((&raw const sMenuItems).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
            .cast::<u8>())
            .wrapping_offset(((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize))
            .read();
            if ((menuItem) as i32) != 13i32 {
                SetSelectedConditionSearch(((((menuItem) as i32).wrapping_sub(8i32)) as u32));
                SetMenuIdAndCB(menu, 100008u32);
                ((menu).wrapping_add(6).cast::<u16>()).write(3u16);
                return 8u32;
            } else {
                PlaySE(5u16);
                ReturnToConditionMenu(menu);
                return 5u32;
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            if ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32)
                != ((((((&raw const sLastCursorPositions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize))
                .read()) as i32)
            {
                ((menu).wrapping_add(2).cast::<i16>()).write(
                    ((((((&raw const sLastCursorPositions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize))
                    .read()) as i16),
                );
                ((menu)
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(CB2_ReturnToConditionMenu));
                return 1u32;
            } else {
                PlaySE(5u16);
                ReturnToConditionMenu(menu);
                return 5u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToMainMenu(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        ReturnToMainMenu(menu);
        return 3u32;
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToConditionMenu(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        ReturnToConditionMenu(menu);
        return 5u32;
    }
}
pub(crate) unsafe extern "C" fn SetMenuIdAndCB(menu: *mut u8, menuId: u32) {
    unsafe {
        let mut menu = menu;
        let mut menuId = menuId;
        ((menu).wrapping_add(8).cast::<u32>()).write(menuId);
        ((menu)
            .wrapping_add(12)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(GetMenuId));
    }
}
pub(crate) unsafe extern "C" fn GetMenuId(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        return ((menu).wrapping_add(8).cast::<u32>()).read();
    }
}
pub(crate) unsafe extern "C" fn ReturnToMainMenu(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        ((menu).cast::<u16>()).write(((GetPokenavMainMenuType()) as u16));
        ((menu).wrapping_add(2).cast::<i16>()).write(1i16);
        ((menu).wrapping_add(4).cast::<u16>()).write(
            ((((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
            .cast::<u8>())
            .wrapping_offset(((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize))
            .read()) as u16),
        );
        ((menu)
            .wrapping_add(12)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(HandleMainMenuInput));
    }
}
pub(crate) unsafe extern "C" fn ReturnToConditionMenu(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        ((menu).cast::<u16>()).write(3u16);
        ((menu).wrapping_add(2).cast::<i16>()).write(1i16);
        ((menu).wrapping_add(4).cast::<u16>()).write(
            ((((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(18))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as u16),
        );
        ((menu)
            .wrapping_add(12)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(HandleConditionMenuInput));
    }
}
pub(crate) unsafe extern "C" fn UpdateMenuCursorPos(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            if (({
                let __p1 = (menu).wrapping_add(2).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                < 0i32
            {
                ((menu).wrapping_add(2).cast::<i16>()).write(
                    ((((((&raw const sLastCursorPositions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize))
                    .read()) as i16),
                );
            }
            ((menu).wrapping_add(4).cast::<u16>()).write(
                ((((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
                .cast::<u8>())
                .wrapping_offset(
                    ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize,
                ))
                .read()) as u16),
            );
            return 1u32;
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0
            {
                let __p3 = (menu).wrapping_add(2).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                if ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32)
                    > ((((((&raw const sLastCursorPositions).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize))
                    .read()) as i32)
                {
                    ((menu).wrapping_add(2).cast::<i16>()).write(0i16);
                }
                ((menu).wrapping_add(4).cast::<u16>()).write(
                    ((((((((&raw const sMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((((menu).cast::<u16>()).read()) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as u16),
                );
                return 1u32;
            } else {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokenavMenuType() -> i32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(1u32);
        return ((((menu).cast::<u16>()).read()) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokenavCursorPos() -> i32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(1u32);
        return ((((menu).wrapping_add(2).cast::<i16>()).read()) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMenuItemId() -> i32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(1u32);
        return ((((menu).wrapping_add(4).cast::<u16>()).read()) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHelpBarTextId() -> u16 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(1u32);
        return ((menu).wrapping_add(6).cast::<u16>()).read();
    }
}
