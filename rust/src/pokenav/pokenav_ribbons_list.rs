//! Translated from `src/pokenav_ribbons_list.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMonRibbonListLoopTaskFuncs sMonRibbonListFramePal sMonRibbonListFrameTiles sMonRibbonListFrameTilemap sMonRibbonListUi_Pal sMonRibbonListBgTemplates sRibbonsMonMenuLoopTaskFuncs sRibbonsMonListWindowTemplate sText_MaleSymbol sText_FemaleSymbol sText_NoGenderSymbol
#[allow(unused_imports)]
use crate::data::pokenav_ribbons_list::*;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPlayerParty: u8;
    static mut gStringVar1: u8;
    static mut gStringVar3: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckBoxMonSanityAt(a0: u32, a1: u32) -> u32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreatePokenavList(a0: *mut u8, a1: *mut u8, a2: u32) -> u32;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyPokenavList();
    fn FreePokenavSubstruct(a0: u32);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxMonGender(a0: *mut u8) -> u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetLevelFromBoxMonExp(a0: *mut u8) -> u8;
    fn GetLevelFromMonExp(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringClearToWidth(a0: *mut u8, a1: i32, a2: *mut u8, a3: i32) -> *mut u8;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn HideBg(a0: u8);
    fn InitBgTemplates(a0: *mut u8, a1: i32);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetLeftHeaderSpritesInvisibility();
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MonRibbonList() -> u32 {
    unsafe {
        let mut list: *mut u8 = AllocSubstruct(9u32, 32u32);
        if ((list) as usize) == 0usize {
            return 0u32;
        }
        ((list).wrapping_add(28).cast::<*mut u8>()).write(AllocSubstruct(18u32, 1708u32));
        if ((((list).wrapping_add(28).cast::<*mut u8>()).read()) as usize) == 0usize {
            return 0u32;
        }
        ((list).cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(HandleRibbonsMonListInput_WaitListInit));
        ((list).wrapping_add(4).cast::<u32>())
            .write(CreateLoopedTask(Some(GetMonRibbonListLoopTaskFunc), 1u32));
        ((list).wrapping_add(20).cast::<u32>()).write(0u32);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_RibbonsMonListFromSummary() -> u32 {
    unsafe {
        let mut list: *mut u8 = AllocSubstruct(9u32, 32u32);
        if ((list) as usize) == 0usize {
            return 0u32;
        }
        ((list).wrapping_add(28).cast::<*mut u8>()).write(GetSubstructPtr(18u32));
        ((list).cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(HandleRibbonsMonListInput));
        ((list).wrapping_add(20).cast::<u32>()).write(1u32);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonsMonListCallback() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        return (((list).cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>()).read())
            .unwrap_unchecked()(list);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsMonList() {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        if !((((list).wrapping_add(24).cast::<u32>()).read()) != 0) {
            FreePokenavSubstruct(18u32);
        }
        FreePokenavSubstruct(9u32);
    }
}
pub(crate) unsafe extern "C" fn HandleRibbonsMonListInput_WaitListInit(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        if !((IsLoopedTaskActive(((list).wrapping_add(4).cast::<u32>()).read())) != 0) {
            ((list).cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(HandleRibbonsMonListInput));
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleRibbonsMonListInput(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            return 2u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            return 3u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            return 4u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((list).wrapping_add(24).cast::<u32>()).write(0u32);
            ((list).cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(RibbonsMonMenu_ReturnToMainMenu));
            return 5u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            ((((list).wrapping_add(28).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .write(((PokenavList_GetSelectedIndex()) as u16));
            ((list).wrapping_add(24).cast::<u32>()).write(1u32);
            ((list).cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(RibbonsMonMenu_ToSummaryScreen));
            return 6u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn RibbonsMonMenu_ReturnToMainMenu(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        return 100005u32;
    }
}
pub(crate) unsafe extern "C" fn RibbonsMonMenu_ToSummaryScreen(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        return 100013u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateMonListBgs() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        return ((list).wrapping_add(20).cast::<u32>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetMonRibbonMonListData() -> *mut u8 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        return ((((list).wrapping_add(28).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn GetRibbonsMonListCount() -> i32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        return ((((((list).wrapping_add(28).cast::<*mut u8>()).read()).cast::<u16>()).read())
            as i32);
    }
}
pub(crate) unsafe extern "C" fn GetMonRibbonSelectedMonData() -> i32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        let mut idx: i32 = ((PokenavList_GetSelectedIndex()) as i32);
        return (((((((((list).wrapping_add(28).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<u8>())
        .wrapping_offset((idx) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetRibbonListMenuCurrIndex() -> i32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        return ((((((list).wrapping_add(28).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetMonRibbonListLoopTaskFunc(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        return (((((&raw const sMonRibbonListLoopTaskFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
        .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
        .wrapping_offset((state) as isize))
        .read())
        .unwrap_unchecked()(state);
    }
}
pub(crate) unsafe extern "C" fn BuildPartyMonRibbonList(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut i: i32 = 0i32;
        let mut item = crate::ffi::Align4([0u8; 4]);
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        ((((list).wrapping_add(28).cast::<*mut u8>()).read()).cast::<u16>()).write(0u16);
        ((((list).wrapping_add(28).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        ((&raw mut item).cast::<u8>()).write(14u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut pokemon: *mut u8 =
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100);
                    if !((GetMonData2(pokemon, 5i32)) != 0) {
                        return 1u32;
                    }
                    if (!((GetMonData2(pokemon, 6i32)) != 0))
                        && (!((GetMonData2(pokemon, 4i32)) != 0))
                    {
                        let mut ribbonCount: u32 = GetMonData2(pokemon, 82i32);
                        if ribbonCount != 0u32 {
                            (((&raw mut item).cast::<u8>()).wrapping_add(1)).write(((i) as u8));
                            (((&raw mut item).cast::<u8>()).wrapping_add(2).cast::<u16>())
                                .write(((ribbonCount) as u16));
                            InsertMonListItem(list, (&raw mut item).cast::<u8>());
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn InitBoxMonRibbonList(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        ((list).wrapping_add(16).cast::<i32>()).write(0i32);
        ((list).wrapping_add(12).cast::<i32>()).write(0i32);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn BuildBoxMonRibbonList(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut list: *mut u8 = GetSubstructPtr(9u32);
        let mut boxId: i32 = ((list).wrapping_add(12).cast::<i32>()).read();
        let mut monId: i32 = ((list).wrapping_add(16).cast::<i32>()).read();
        let mut boxCount: i32 = 0i32;
        let mut item = crate::ffi::Align4([0u8; 4]);
        'l1: loop {
            if !(boxId < 14i32) {
                break 'l1;
            }
            'l2: loop {
                if !(monId < 30i32) {
                    break 'l2;
                }
                if (CheckBoxMonSanityAt(((boxId) as u32), ((monId) as u32))) != 0 {
                    let mut ribbonCount: u32 =
                        GetBoxMonDataAt(((boxId) as u8), ((monId) as u8), 82i32);
                    if ribbonCount != 0u32 {
                        ((&raw mut item).cast::<u8>()).write(((boxId) as u8));
                        (((&raw mut item).cast::<u8>()).wrapping_add(1)).write(((monId) as u8));
                        (((&raw mut item).cast::<u8>()).wrapping_add(2).cast::<u16>())
                            .write(((ribbonCount) as u16));
                        InsertMonListItem(list, (&raw mut item).cast::<u8>());
                    }
                }
                boxCount = (boxCount).wrapping_add(1);
                monId = (monId).wrapping_add(1);
                if boxCount > 14i32 {
                    ((list).wrapping_add(12).cast::<i32>()).write(boxId);
                    ((list).wrapping_add(16).cast::<i32>()).write(monId);
                    return 3u32;
                }
            }
            monId = 0i32;
            boxId = (boxId).wrapping_add(1);
        }
        ((list).wrapping_add(20).cast::<u32>()).write(1u32);
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn InsertMonListItem(list: *mut u8, item: *mut u8) {
    unsafe {
        let mut list = list;
        let mut item = item;
        let mut left: u32 = 0u32;
        let mut right: u32 =
            ((((((list).wrapping_add(28).cast::<*mut u8>()).read()).cast::<u16>()).read()) as u32);
        let mut insertionIdx: u32 =
            (left).wrapping_add(crate::c::div_u32((right).wrapping_sub(left), 2u32));
        'l1: loop {
            if !(right != insertionIdx) {
                break 'l1;
            }
            if ((((item).wrapping_add(2).cast::<u16>()).read()) as i32)
                > (((((((((list).wrapping_add(28).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((insertionIdx) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
            {
                right = insertionIdx;
            } else {
                left = (insertionIdx).wrapping_add(1u32);
            }
            insertionIdx = (left).wrapping_add(crate::c::div_u32((right).wrapping_sub(left), 2u32));
        }
        {
            right = ((((((list).wrapping_add(28).cast::<*mut u8>()).read()).cast::<u16>()).read())
                as u32);
            'l2: loop {
                if !(right > insertionIdx) {
                    break 'l2;
                }
                'l3: {
                    (((((list).wrapping_add(28).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<u8>())
                    .wrapping_offset(((right) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .write_unaligned(
                        (((((list).wrapping_add(28).cast::<*mut u8>()).read()).wrapping_add(4))
                            .cast::<u8>())
                        .wrapping_offset((((right).wrapping_sub(1u32)) as i32) as isize * 4)
                        .cast::<crate::c::Rec4<4>>()
                        .read_unaligned(),
                    );
                }
                right = (right).wrapping_sub(1);
            }
        }
        (((((list).wrapping_add(28).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((insertionIdx) as i32) as isize * 4)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(item.cast::<crate::c::Rec4<4>>().read_unaligned());
        let __p1 = (((list).wrapping_add(28).cast::<*mut u8>()).read()).cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PlayerHasRibbonsMon() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut mon: *mut u8 =
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100);
                    if !((GetMonData2(mon, 5i32)) != 0) {
                        break 'l2;
                    }
                    if (GetMonData2(mon, 6i32)) != 0 {
                        break 'l2;
                    }
                    if (GetMonData2(mon, 83i32)) != 0 {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 14i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 30i32) {
                                break 'l5;
                            }
                            'l6: {
                                if !((CheckBoxMonSanityAt(((i) as u32), ((j) as u32))) != 0) {
                                    break 'l6;
                                }
                                if (GetBoxMonDataAt(((i) as u8), ((j) as u8), 83i32)) != 0 {
                                    return 1u32;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenRibbonsMonList() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(10u32, 2064u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ((menu).wrapping_add(4).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_OpenRibbonsMonList), 1u32));
        ((menu).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetRibbonsMonCurrentLoopedTaskActive));
        ((menu).wrapping_add(12).cast::<u32>()).write(0u32);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenRibbonsMonListFromRibbonsSummary() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(10u32, 2064u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ((menu).wrapping_add(4).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_OpenRibbonsMonList), 1u32));
        ((menu).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetRibbonsMonCurrentLoopedTaskActive));
        ((menu).wrapping_add(12).cast::<u32>()).write(1u32);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRibbonsMonListLoopedTask(idx: i32) {
    unsafe {
        let mut idx = idx;
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        ((menu).wrapping_add(4).cast::<u32>()).write(CreateLoopedTask(
            ((((&raw const sRibbonsMonMenuLoopTaskFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .wrapping_offset((idx) as isize))
            .read(),
            1u32,
        ));
        ((menu).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetRibbonsMonCurrentLoopedTaskActive));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRibbonsMonListLoopedTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        return (((menu).cast::<Option<unsafe extern "C" fn() -> u32>>()).read())
            .unwrap_unchecked()();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonsMonCurrentLoopedTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        return IsLoopedTaskActive(((menu).wrapping_add(4).cast::<u32>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsMonMenu() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        DestroyPokenavList();
        RemoveWindow(((((menu).wrapping_add(8).cast::<u16>()).read()) as u8));
        FreePokenavSubstruct(10u32);
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenRibbonsMonList(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                InitBgTemplates(
                    ((&raw const sMonRibbonListBgTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(8u32, 4u32)) as i32),
                );
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw const sMonRibbonListFrameTiles)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                SetBgTilemapBuffer(1u8, ((menu).wrapping_add(16)).cast::<u8>());
                CopyToBgTilemapBuffer(
                    1u8,
                    (((&raw const sMonRibbonListFrameTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sMonRibbonListFramePal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    16u32,
                    32u32,
                );
                CopyBgTilemapBufferToVram(1u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                if !((UpdateMonListBgs()) != 0) {
                    return 2u32;
                }
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ShowBg(1u8);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sMonRibbonListUi_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    32u32,
                    32u32,
                );
                CreateRibbonMonsList();
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsCreatePokenavListTaskActive()) != 0 {
                    return 2u32;
                }
                AddRibbonsMonListWindow(menu);
                return 0u32;
            }
            if __sw1 == 4i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                ShowBg(2u8);
                HideBg(3u8);
                PrintHelpBarText(9u32);
                PokenavFadeScreen(1i32);
                if !((((menu).wrapping_add(12).cast::<u32>()).read()) != 0) {
                    LoadLeftHeaderGfxForIndex(2u32);
                    ShowLeftHeaderGfx(2u32, 1u32, 0u32);
                }
                return 0u32;
            }
            if __sw1 == 5i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMoveCursorUp(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        'l1: {
            let __sw1 = state;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                'l2: {
                    let __sw2 = PokenavList_MoveCursorUp();
                    if __sw2 == 0i32 {
                        return 4u32;
                    }
                    if __sw2 == 1i32 {
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        PlaySE(5u16);
                        break 'l2;
                    }
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                UpdateIndexNumberDisplay(menu);
                return 0u32;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMoveCursorDown(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        'l1: {
            let __sw1 = state;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                'l2: {
                    let __sw2 = PokenavList_MoveCursorDown();
                    if __sw2 == 0i32 {
                        return 4u32;
                    }
                    if __sw2 == 1i32 {
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        PlaySE(5u16);
                        break 'l2;
                    }
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                UpdateIndexNumberDisplay(menu);
                return 0u32;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMovePageUp(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        'l1: {
            let __sw1 = state;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                'l2: {
                    let __sw2 = PokenavList_PageUp();
                    if __sw2 == 0i32 {
                        return 4u32;
                    }
                    if __sw2 == 1i32 {
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        PlaySE(5u16);
                        break 'l2;
                    }
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                UpdateIndexNumberDisplay(menu);
                return 0u32;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListMovePageDown(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(10u32);
        'l1: {
            let __sw1 = state;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                'l2: {
                    let __sw2 = PokenavList_PageDown();
                    if __sw2 == 0i32 {
                        return 4u32;
                    }
                    if __sw2 == 1i32 {
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        PlaySE(5u16);
                        break 'l2;
                    }
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                UpdateIndexNumberDisplay(menu);
                return 0u32;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListReturnToMainMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PokenavFadeScreen(0i32);
                SlideMenuHeaderDown();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                if (MainMenuLoopedTaskIsBusy()) != 0 {
                    return 2u32;
                }
                SetLeftHeaderSpritesInvisibility();
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RibbonsListOpenSummary(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PokenavFadeScreen(0i32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn AddRibbonsMonListWindow(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut listCount: i32 = 0i32;
        ((menu).wrapping_add(8).cast::<u16>()).write(AddWindow(
            (&raw const sRibbonsMonListWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        PutWindowTilemap(((((menu).wrapping_add(8).cast::<u16>()).read()) as u8));
        listCount = GetRibbonsMonListCount();
        DrawListIndexNumber(
            ((((menu).wrapping_add(8).cast::<u16>()).read()) as i32),
            0i32,
            listCount,
        );
        CopyWindowToVram(((((menu).wrapping_add(8).cast::<u16>()).read()) as u8), 1u8);
        UpdateIndexNumberDisplay(menu);
    }
}
pub(crate) unsafe extern "C" fn UpdateIndexNumberDisplay(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut listIndex: i32 = ((PokenavList_GetSelectedIndex()) as i32);
        let mut listCount: i32 = GetRibbonsMonListCount();
        DrawListIndexNumber(
            ((((menu).wrapping_add(8).cast::<u16>()).read()) as i32),
            (listIndex).wrapping_add(1i32),
            listCount,
        );
        CopyWindowToVram(((((menu).wrapping_add(8).cast::<u16>()).read()) as u8), 2u8);
    }
}
pub(crate) unsafe extern "C" fn DrawListIndexNumber(windowId: i32, index: i32, max: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut index = index;
        let mut max = max;
        let mut strbuf = crate::ffi::Align4([0u8; 16]);
        let mut x: u32 = 0u32;
        let mut ptr: *mut u8 = (&raw mut strbuf).cast::<u8>();
        ptr = ConvertIntToDecimalStringN(ptr, index, 1i32, 3u8);
        ({
            let __t1 = ptr;
            ptr = (ptr).wrapping_offset(1);
            __t1
        })
        .write(186u8);
        ConvertIntToDecimalStringN(ptr, max, 1i32, 3u8);
        x = ((GetStringCenterAlignXOffset(1i32, (&raw mut strbuf).cast::<u8>(), 56i32)) as u32);
        AddTextPrinterParameterized(
            ((windowId) as u8),
            1u8,
            (&raw mut strbuf).cast::<u8>(),
            ((x) as u8),
            1u8,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateRibbonMonsList() {
    unsafe {
        let mut template = crate::ffi::Align4([0u8; 24]);
        (((&raw mut template).cast::<u8>()).cast::<*mut u8>()).write(GetMonRibbonMonListData());
        (((&raw mut template).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(((GetRibbonsMonListCount()) as u16));
        (((&raw mut template).cast::<u8>()).wrapping_add(8)).write(4u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(((GetRibbonListMenuCurrIndex()) as u16));
        (((&raw mut template).cast::<u8>()).wrapping_add(9)).write(13u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(10)).write(17u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(11)).write(1u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(12)).write(8u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(13)).write(2u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(14)).write(1u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .write(Some(BufferRibbonMonInfoText));
        (((&raw mut template).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
        .write(None);
        CreatePokenavList(
            (((&raw const sMonRibbonListBgTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(4),
            (&raw mut template).cast::<u8>(),
            0u32,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferRibbonMonInfoText(listItem: *mut u8, dest: *mut u8) {
    unsafe {
        let mut listItem = listItem;
        let mut dest = dest;
        let mut gender: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut s: *mut u8 = core::ptr::null_mut();
        let mut genderStr: *mut u8 = core::ptr::null_mut();
        let mut item: *mut u8 = listItem;
        if (((item).read()) as i32) == 14i32 {
            let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((((item).wrapping_add(1)).read()) as i32) as isize * 100);
            gender = GetMonGender(mon);
            level = GetLevelFromMonExp(mon);
            GetMonData3(mon, 2i32, (&raw mut gStringVar3).cast::<u8>());
        } else {
            let mut mon: *mut u8 = GetBoxedMonPtr((item).read(), ((item).wrapping_add(1)).read());
            gender = GetBoxMonGender(mon);
            level = GetLevelFromBoxMonExp(mon);
            GetBoxMonData3(mon, 2i32, (&raw mut gStringVar3).cast::<u8>());
        }
        StringGet_Nickname((&raw mut gStringVar3).cast::<u8>());
        dest = GetStringClearToWidth(dest, 1i32, (&raw mut gStringVar3).cast::<u8>(), 60i32);
        'l1: {
            let __sw1 = ((gender) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 254i32;
            if !__matched {
                genderStr =
                    ((&raw const sText_NoGenderSymbol).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 0i32 {
                genderStr = ((&raw const sText_MaleSymbol).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 254i32 {
                genderStr = ((&raw const sText_FemaleSymbol).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l1;
            }
        }
        s = StringCopy((&raw mut gStringVar1).cast::<u8>(), genderStr);
        ({
            let __t2 = s;
            s = (s).wrapping_offset(1);
            __t2
        })
        .write(186u8);
        ({
            let __t3 = s;
            s = (s).wrapping_offset(1);
            __t3
        })
        .write(249u8);
        ({
            let __t4 = s;
            s = (s).wrapping_offset(1);
            __t4
        })
        .write(5u8);
        ConvertIntToDecimalStringN(s, ((level) as i32), 0i32, 3u8);
        dest = GetStringClearToWidth(dest, 1i32, (&raw mut gStringVar1).cast::<u8>(), 54i32);
        ConvertIntToDecimalStringN(
            dest,
            ((((item).wrapping_add(2).cast::<u16>()).read()) as i32),
            1i32,
            2u8,
        );
    }
}
