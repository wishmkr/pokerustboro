//! Translated from `src/pokenav_list.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sListArrow_Pal sListArrow_Gfx sListArrowSpriteSheets sListArrowPalettes sOamData_RightArrow sSpriteTemplate_RightArrow sOamData_UpDownArrow sSpriteTemplate_UpDownArrow lineOffsets.0
#[allow(unused_imports)]
use crate::data::pokenav_list::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMoveWindowDownIndex: u32 = 0u32;

unsafe extern "C" {
    static mut gSprites: u8;
    static mut gText_PokenavMatchCall_SelfIntroduction: u8;
    static mut gText_PokenavMatchCall_Strategy: u8;
    static mut gText_PokenavMatchCall_TrainerPokemon: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearRematchPokeballIcon(a0: u16, a1: u32);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowRectToVram(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FillWindowTilesByRow(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32);
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>) -> u32;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetBgY(a0: u8) -> i32;
    fn GetMatchCallFlavorText(a0: i32, a1: i32) -> *mut u8;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn Pokenav_AllocAndLoadPalettes(a0: *mut u8);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePokenavList(
    bgTemplate: *mut u8,
    listTemplate: *mut u8,
    tileOffset: u32,
) -> u32 {
    unsafe {
        let mut bgTemplate = bgTemplate;
        let mut listTemplate = listTemplate;
        let mut tileOffset = tileOffset;
        let mut list: *mut u8 = AllocSubstruct(17u32, 2212u32);
        if ((list) as usize) == 0usize {
            return 0u32;
        }
        InitPokenavListWindowState((list).wrapping_add(2184), listTemplate);
        if !((CopyPokenavListMenuTemplate(list, bgTemplate, listTemplate, tileOffset)) != 0) {
            return 0u32;
        }
        CreateLoopedTask(Some(LoopedTask_CreatePokenavList), 6u32);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCreatePokenavListTaskActive() -> u32 {
    unsafe {
        return FuncIsActiveLoopedTask(Some(LoopedTask_CreatePokenavList));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyPokenavList() {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        DestroyListArrows(list);
        RemoveWindow(((((list).wrapping_add(8).cast::<u16>()).read()) as u8));
        FreePokenavSubstruct(17u32);
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_CreatePokenavList(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut list: *mut u8 = core::ptr::null_mut();
        if (IsDma3ManagerBusyWithBgCopy()) != 0 {
            return 2u32;
        }
        list = GetSubstructPtr(17u32);
        'l1: {
            let __sw1 = state;
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                InitPokenavListBg(list);
                return 0u32;
            }
            if __sw1 == 1i32 {
                __fall = true;
                InitPokenavListWindow((list));
                return 0u32;
            }
            if __sw1 == 2i32 {
                __fall = true;
                InitListItems((list).wrapping_add(2184), list);
                return 0u32;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsPrintListItemsTaskActive()) != 0 {
                    return 2u32;
                } else {
                    LoadListArrowGfx();
                    return 1u32;
                }
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                CreateListArrowSprites((list).wrapping_add(2184), list);
                return 4u32;
            }
            if !__matched {
                __fall = true;
                return 4u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn InitPokenavListBg(list: *mut u8) {
    unsafe {
        let mut list = list;
        let mut tileNum: u16 = (((((((list).wrapping_add(1)).read()) as i32) << 12)
            | ((((list).wrapping_add(6).cast::<u16>()).read()) as i32))
            as u16);
        BgDmaFill(
            (((list).read()) as u32),
            17u8,
            ((((list).wrapping_add(6).cast::<u16>()).read()) as i32),
            1i32,
        );
        BgDmaFill(
            (((list).read()) as u32),
            68u8,
            ((((list).wrapping_add(6).cast::<u16>()).read()) as i32).wrapping_add(1i32),
            1i32,
        );
        SetBgTilemapBuffer((list).read(), ((list).wrapping_add(136)).cast::<u8>());
        FillBgTilemapBufferRect_Palette0((list).read(), tileNum, 0u8, 0u8, 32u8, 32u8);
        ChangeBgY((list).read(), 0i32, 0u8);
        ChangeBgX((list).read(), 0i32, 0u8);
        ChangeBgY(
            (list).read(),
            (((((list).wrapping_add(3)).read()) as i32) << 11),
            2u8,
        );
        CopyBgTilemapBufferToVram((list).read());
    }
}
pub(crate) unsafe extern "C" fn InitPokenavListWindow(listWindow: *mut u8) {
    unsafe {
        let mut listWindow = listWindow;
        FillWindowPixelBuffer(
            ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8));
        CopyWindowToVram(
            ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn InitListItems(windowState: *mut u8, list: *mut u8) {
    unsafe {
        let mut windowState = windowState;
        let mut list = list;
        let mut numToPrint: i32 = ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32)
            .wrapping_sub(((((windowState).cast::<u16>()).read()) as i32));
        if numToPrint > ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32) {
            numToPrint = ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32);
        }
        PrintListItems(
            ((windowState).wrapping_add(16).cast::<*mut u8>()).read(),
            ((((windowState).cast::<u16>()).read()) as u32),
            ((numToPrint) as u32),
            ((windowState).wrapping_add(12).cast::<u32>()).read(),
            0u32,
            list,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintListItems(
    listPtr: *mut u8,
    topIndex: u32,
    numItems: u32,
    itemSize: u32,
    printStart: u32,
    list: *mut u8,
) {
    unsafe {
        let mut listPtr = listPtr;
        let mut topIndex = topIndex;
        let mut numItems = numItems;
        let mut itemSize = itemSize;
        let mut printStart = printStart;
        let mut list = list;
        if numItems == 0u32 {
            return;
        }
        ((list).wrapping_add(28).cast::<*mut u8>()).write(
            (listPtr).wrapping_offset((((topIndex).wrapping_mul(itemSize)) as i32) as isize * 1),
        );
        ((list).wrapping_add(24).cast::<u32>()).write(itemSize);
        ((list).wrapping_add(12).cast::<u16>()).write(0u16);
        ((list).wrapping_add(14).cast::<u16>()).write(((numItems) as u16));
        ((list).wrapping_add(20).cast::<u32>()).write(topIndex);
        ((list).wrapping_add(16).cast::<u32>()).write(printStart);
        CreateLoopedTask(Some(LoopedTask_PrintListItems), 5u32);
    }
}
pub(crate) unsafe extern "C" fn IsPrintListItemsTaskActive() -> u32 {
    unsafe {
        return FuncIsActiveLoopedTask(Some(LoopedTask_PrintListItems));
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_PrintListItems(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut row: u32 = 0u32;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        'l1: {
            let __sw1 = state;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                row = (((((((list).wrapping_add(10).cast::<u16>()).read()) as i32)
                    .wrapping_add(((((list).wrapping_add(12).cast::<u16>()).read()) as i32)))
                    as u32)
                    .wrapping_add(((list).wrapping_add(16).cast::<u32>()).read())
                    & 15u32);
                (((list)
                    .wrapping_add(52)
                    .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
                .read())
                .unwrap_unchecked()(
                    ((list).wrapping_add(28).cast::<*mut u8>()).read(),
                    ((list).wrapping_add(72)).cast::<u8>(),
                );
                if core::mem::transmute::<_, usize>(
                    ((list)
                        .wrapping_add(56)
                        .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
                    .read(),
                ) != 0usize
                {
                    (((list)
                        .wrapping_add(56)
                        .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
                    .read())
                    .unwrap_unchecked()(
                        ((list).wrapping_add(8).cast::<u16>()).read(),
                        ((list).wrapping_add(20).cast::<u32>()).read(),
                        row,
                    );
                }
                AddTextPrinterParameterized(
                    ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
                    ((list).wrapping_add(5)).read(),
                    ((list).wrapping_add(72)).cast::<u8>(),
                    8u8,
                    (((row << 4).wrapping_add(1u32)) as u8),
                    255u8,
                    None,
                );
                if (({
                    let __p2 = (list).wrapping_add(12).cast::<u16>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    >= ((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
                {
                    if core::mem::transmute::<_, usize>(
                        ((list)
                            .wrapping_add(56)
                            .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
                        .read(),
                    ) != 0usize
                    {
                        CopyWindowToVram(
                            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
                            3u8,
                        );
                    } else {
                        CopyWindowToVram(
                            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
                            2u8,
                        );
                    }
                    return 0u32;
                } else {
                    let __p4 = (list).wrapping_add(28).cast::<*mut u8>();
                    (__p4).write(((__p4).read()).wrapping_offset(
                        ((((list).wrapping_add(24).cast::<u32>()).read()) as i32) as isize * 1,
                    ));
                    let __p5 = (list).wrapping_add(20).cast::<u32>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    return 3u32;
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                return 4u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn ShouldShowUpArrow() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        return (((((((list).wrapping_add(2184)).cast::<u16>()).read()) as i32) != 0i32) as u32);
    }
}
pub(crate) unsafe extern "C" fn ShouldShowDownArrow() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        let mut windowState: *mut u8 = (list).wrapping_add(2184);
        return ((((((windowState).cast::<u16>()).read()) as i32)
            .wrapping_add(((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32))
            < ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32))
            as u32);
    }
}
pub(crate) unsafe extern "C" fn MoveListWindow(delta: i32, printItems: u32) {
    unsafe {
        let mut delta = delta;
        let mut printItems = printItems;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        let mut windowState: *mut u8 = (list).wrapping_add(2184);
        if delta < 0i32 {
            if ((((windowState).cast::<u16>()).read()) as i32).wrapping_add(delta) < 0i32 {
                delta = (-1i32).wrapping_mul(((((windowState).cast::<u16>()).read()) as i32));
            }
            if (printItems) != 0 {
                PrintListItems(
                    ((windowState).wrapping_add(16).cast::<*mut u8>()).read(),
                    ((((((windowState).cast::<u16>()).read()) as i32).wrapping_add(delta)) as u32),
                    (((delta).wrapping_mul((-1i32))) as u32),
                    ((windowState).wrapping_add(12).cast::<u32>()).read(),
                    ((delta) as u32),
                    list,
                );
            }
        } else {
            if (printItems) != 0 {
                let mut index: i32 = (({
                    let __v1 = ((((((windowState).cast::<u16>()).read()) as i32).wrapping_add(
                        ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32),
                    )) as u32);
                    ((&raw mut sMoveWindowDownIndex).cast::<u8>().cast::<u32>()).write(__v1);
                    __v1
                }) as i32);
                if (index).wrapping_add(delta)
                    >= ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32)
                {
                    delta = ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_sub(index);
                }
                PrintListItems(
                    ((windowState).wrapping_add(16).cast::<*mut u8>()).read(),
                    ((index) as u32),
                    ((delta) as u32),
                    ((windowState).wrapping_add(12).cast::<u32>()).read(),
                    ((((windowState).wrapping_add(8).cast::<u16>()).read()) as u32),
                    list,
                );
            }
        }
        CreateMoveListWindowTask(delta, list);
        let __p2 = (windowState).cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(delta)) as u16));
    }
}
pub(crate) unsafe extern "C" fn CreateMoveListWindowTask(delta: i32, list: *mut u8) {
    unsafe {
        let mut delta = delta;
        let mut list = list;
        ((list).wrapping_add(32).cast::<i32>()).write(GetBgY((list).read()));
        ((list).wrapping_add(36).cast::<i32>())
            .write((((list).wrapping_add(32).cast::<i32>()).read()).wrapping_add((delta << 12)));
        if delta > 0i32 {
            ((list).wrapping_add(48).cast::<u32>()).write(1u32);
        } else {
            ((list).wrapping_add(48).cast::<u32>()).write(2u32);
        }
        ((list).wrapping_add(44).cast::<i32>()).write(delta);
        ((list).wrapping_add(40).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_MoveListWindow), 6u32));
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveListWindow(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut oldY: i32 = 0i32;
        let mut newY: i32 = 0i32;
        let mut finished: u32 = 0u32;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                if !((IsPrintListItemsTaskActive()) != 0) {
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 1i32 {
                finished = 0u32;
                oldY = GetBgY((list).read());
                newY = ChangeBgY(
                    (list).read(),
                    4096i32,
                    ((((list).wrapping_add(48).cast::<u32>()).read()) as u8),
                );
                if ((list).wrapping_add(48).cast::<u32>()).read() == 2u32 {
                    if ((oldY > ((list).wrapping_add(36).cast::<i32>()).read())
                        || (oldY <= ((list).wrapping_add(32).cast::<i32>()).read()))
                        && (newY <= ((list).wrapping_add(36).cast::<i32>()).read())
                    {
                        finished = 1u32;
                    }
                } else {
                    if ((oldY < ((list).wrapping_add(36).cast::<i32>()).read())
                        || (oldY >= ((list).wrapping_add(32).cast::<i32>()).read()))
                        && (newY >= ((list).wrapping_add(36).cast::<i32>()).read())
                    {
                        finished = 1u32;
                    }
                }
                if (finished) != 0 {
                    ((list).wrapping_add(10).cast::<u16>()).write(
                        ((((((list).wrapping_add(10).cast::<u16>()).read()) as i32)
                            .wrapping_add(((list).wrapping_add(44).cast::<i32>()).read())
                            & 15i32) as u16),
                    );
                    ChangeBgY(
                        (list).read(),
                        ((list).wrapping_add(36).cast::<i32>()).read(),
                        0u8,
                    );
                    return 4u32;
                }
                return 2u32;
            }
        }
        return 4u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_IsMoveWindowTaskActive() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        return IsLoopedTaskActive(((list).wrapping_add(40).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn GetPokenavListWindowState() -> *mut u8 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        return (list).wrapping_add(2184);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_MoveCursorUp() -> i32 {
    unsafe {
        let mut windowState: *mut u8 = GetPokenavListWindowState();
        if ((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32) != 0i32 {
            let __p1 = (windowState).wrapping_add(6).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return 1i32;
        }
        if (ShouldShowUpArrow()) != 0 {
            MoveListWindow((-1i32), 1u32);
            return 2i32;
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_MoveCursorDown() -> i32 {
    unsafe {
        let mut windowState: *mut u8 = GetPokenavListWindowState();
        if ((((windowState).cast::<u16>()).read()) as i32)
            .wrapping_add(((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32))
            >= ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
        {
            return 0i32;
        }
        if ((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32)
            < ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
        {
            let __p1 = (windowState).wrapping_add(6).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 1i32;
        }
        if (ShouldShowDownArrow()) != 0 {
            MoveListWindow(1i32, 1u32);
            return 2i32;
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_PageUp() -> i32 {
    unsafe {
        let mut scroll: i32 = 0i32;
        let mut windowState: *mut u8 = GetPokenavListWindowState();
        if (ShouldShowUpArrow()) != 0 {
            if ((((windowState).cast::<u16>()).read()) as i32)
                >= ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32)
            {
                scroll = ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32);
            } else {
                scroll = ((((windowState).cast::<u16>()).read()) as i32);
            }
            MoveListWindow((scroll).wrapping_mul((-1i32)), 1u32);
            return 2i32;
        } else {
            if ((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32) != 0i32 {
                ((windowState).wrapping_add(6).cast::<u16>()).write(0u16);
                return 1i32;
            }
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_PageDown() -> i32 {
    unsafe {
        let mut windowState: *mut u8 = GetPokenavListWindowState();
        if (ShouldShowDownArrow()) != 0 {
            let mut windowBottomIndex: i32 = ((((windowState).cast::<u16>()).read()) as i32)
                .wrapping_add(((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32));
            let mut scroll: i32 = ((((windowState).wrapping_add(4).cast::<u16>()).read()) as i32)
                .wrapping_sub(((((windowState).cast::<u16>()).read()) as i32));
            if windowBottomIndex <= ((((windowState).wrapping_add(4).cast::<u16>()).read()) as i32)
            {
                scroll = ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32);
            }
            MoveListWindow(scroll, 1u32);
            return 2i32;
        } else {
            let mut cursor: i32 = 0i32;
            let mut lastVisibleIndex: i32 = 0i32;
            if ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32)
                >= ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32)
            {
                cursor = ((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32);
                lastVisibleIndex = ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32);
            } else {
                cursor = ((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32);
                lastVisibleIndex = ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32);
            }
            lastVisibleIndex = (lastVisibleIndex).wrapping_sub(1i32);
            if cursor >= lastVisibleIndex {
                return 0i32;
            }
            ((windowState).wrapping_add(6).cast::<u16>()).write(((lastVisibleIndex) as u16));
            return 1i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_GetSelectedIndex() -> u32 {
    unsafe {
        let mut windowState: *mut u8 = GetPokenavListWindowState();
        return ((((((windowState).cast::<u16>()).read()) as i32)
            .wrapping_add(((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32)))
            as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_GetTopIndex() -> u32 {
    unsafe {
        let mut windowState: *mut u8 = GetPokenavListWindowState();
        return ((((windowState).cast::<u16>()).read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_EraseListForCheckPage() {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        ((list).wrapping_add(2204).cast::<i32>()).write(0i32);
        ((list).wrapping_add(2208).cast::<u32>()).write(CreateLoopedTask(
            Some(LoopedTask_EraseListForCheckPage),
            6u32,
        ));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintCheckPageInfo(delta: i16) {
    unsafe {
        let mut delta = delta;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        let __p1 = ((list).wrapping_add(2184)).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((delta) as i32))) as u16));
        ((list).wrapping_add(2204).cast::<i32>()).write(0i32);
        ((list).wrapping_add(2208).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_PrintCheckPageInfo), 6u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_ReshowListFromCheckPage() {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        ((list).wrapping_add(2204).cast::<i32>()).write(0i32);
        ((list).wrapping_add(2208).cast::<u32>()).write(CreateLoopedTask(
            Some(LoopedTask_ReshowListFromCheckPage),
            6u32,
        ));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_IsTaskActive() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        return IsLoopedTaskActive(((list).wrapping_add(2208).cast::<u32>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_DrawCurrentItemIcon() {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        let mut windowState: *mut u8 = (list).wrapping_add(2184);
        (((list)
            .wrapping_add(56)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
        .read())
        .unwrap_unchecked()(
            ((list).wrapping_add(8).cast::<u16>()).read(),
            ((((((windowState).cast::<u16>()).read()) as i32)
                .wrapping_add(((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32)))
                as u32),
            ((((((list).wrapping_add(10).cast::<u16>()).read()) as i32)
                .wrapping_add(((((windowState).wrapping_add(6).cast::<u16>()).read()) as i32))
                & 15i32) as u32),
        );
        CopyWindowToVram(((((list).wrapping_add(8).cast::<u16>()).read()) as u8), 1u8);
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_EraseListForCheckPage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        'l1: {
            let __sw1 = state;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ToggleListArrows(list, 1u32);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((list).wrapping_add(2204).cast::<i32>()).read()
                    != (((((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).read()) as i32)
                {
                    EraseListEntry(
                        (list),
                        ((list).wrapping_add(2204).cast::<i32>()).read(),
                        1i32,
                    );
                }
                let __p2 = (list).wrapping_add(2204).cast::<i32>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                return 0u32;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    if ((list).wrapping_add(2204).cast::<i32>()).read()
                        != (((((list).wrapping_add(2184)).wrapping_add(8).cast::<u16>()).read())
                            as i32)
                    {
                        return 6u32;
                    }
                    if (((((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).read()) as i32)
                        != 0i32
                    {
                        EraseListEntry(
                            (list),
                            ((list).wrapping_add(2204).cast::<i32>()).read(),
                            (((((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).read())
                                as i32),
                        );
                    }
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    if (((((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).read()) as i32)
                        != 0i32
                    {
                        MoveListWindow(
                            (((((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).read())
                                as i32),
                            0u32,
                        );
                        return 0u32;
                    }
                    return 4u32;
                }
                return 2u32;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
                (((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).write(0u16);
                return 4u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_PrintCheckPageInfo(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        if (IsDma3ManagerBusyWithBgCopy()) != 0 {
            return 2u32;
        }
        'l1: {
            let __sw1 = state;
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                PrintCheckPageTrainerName((list).wrapping_add(2184), list);
                break 'l1;
            }
            if __sw1 == 1i32 {
                PrintMatchCallFieldNames(list, 0u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintMatchCallFlavorText((list).wrapping_add(2184), list, 0u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintMatchCallFieldNames(list, 1u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintMatchCallFlavorText((list).wrapping_add(2184), list, 1u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintMatchCallFieldNames(list, 2u32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                PrintMatchCallFlavorText((list).wrapping_add(2184), list, 2u32);
                break 'l1;
            }
            if __sw1 == 7i32 {
                PrintMatchCallFlavorText((list).wrapping_add(2184), list, 3u32);
                break 'l1;
            }
            if !__matched {
                return 4u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ReshowListFromCheckPage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut list: *mut u8 = core::ptr::null_mut();
        let mut listAlias: *mut u8 = core::ptr::null_mut();
        let mut windowState: *mut u8 = core::ptr::null_mut();
        if (IsDma3ManagerBusyWithBgCopy()) != 0 {
            return 2u32;
        }
        list = GetSubstructPtr(17u32);
        windowState = (list).wrapping_add(2184);
        listAlias = list;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PrintMatchCallListTrainerName(windowState, listAlias);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if {
                    let __p2 = (list).wrapping_add(2204).cast::<i32>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                } < (((((list).wrapping_add(2184)).wrapping_add(8).cast::<u16>()).read()) as i32)
                {
                    EraseListEntry(
                        (listAlias),
                        ((list).wrapping_add(2204).cast::<i32>()).read(),
                        1i32,
                    );
                    return 2u32;
                }
                ((list).wrapping_add(2204).cast::<i32>()).write(0i32);
                if ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32)
                    <= ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32)
                {
                    if ((((windowState).cast::<u16>()).read()) as i32) != 0i32 {
                        let mut entries: i32 = ((((windowState).cast::<u16>()).read()) as i32);
                        EraseListEntry((listAlias), (entries).wrapping_neg(), entries);
                        ((windowState).wrapping_add(6).cast::<u16>()).write(((entries) as u16));
                        ((list).wrapping_add(2204).cast::<i32>()).write((entries).wrapping_neg());
                        return 0u32;
                    }
                } else {
                    if ((((windowState).cast::<u16>()).read()) as i32).wrapping_add(
                        ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32),
                    ) > ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32)
                    {
                        let mut entries: i32 = (((((windowState).cast::<u16>()).read()) as i32)
                            .wrapping_add(
                                ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32),
                            ))
                        .wrapping_sub(
                            ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32),
                        );
                        EraseListEntry((listAlias), (entries).wrapping_neg(), entries);
                        ((windowState).wrapping_add(6).cast::<u16>()).write(((entries) as u16));
                        ((list).wrapping_add(2204).cast::<i32>()).write((entries).wrapping_neg());
                        return 0u32;
                    }
                }
                return 9u32;
            }
            if __sw1 == 2i32 {
                MoveListWindow(((list).wrapping_add(2204).cast::<i32>()).read(), 0u32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if !((PokenavList_IsMoveWindowTaskActive()) != 0) {
                    ((list).wrapping_add(2204).cast::<i32>()).write(0i32);
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 4i32 {
                PrintListItems(
                    ((windowState).wrapping_add(16).cast::<*mut u8>()).read(),
                    ((((((windowState).cast::<u16>()).read()) as i32)
                        .wrapping_add(((list).wrapping_add(2204).cast::<i32>()).read()))
                        as u32),
                    1u32,
                    ((windowState).wrapping_add(12).cast::<u32>()).read(),
                    ((((list).wrapping_add(2204).cast::<i32>()).read()) as u32),
                    list,
                );
                return 0u32;
            }
            if __sw1 == 5i32 {
                if (IsPrintListItemsTaskActive()) != 0 {
                    return 2u32;
                }
                if ({
                    let __p4 = (list).wrapping_add(2204).cast::<i32>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                } >= ((((windowState).wrapping_add(2).cast::<u16>()).read()) as i32))
                    || (((list).wrapping_add(2204).cast::<i32>()).read()
                        >= ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32))
                {
                    return 1u32;
                }
                return 9u32;
            }
            if __sw1 == 6i32 {
                ToggleListArrows(listAlias, 0u32);
                return 4u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn EraseListEntry(listWindow: *mut u8, offset: i32, entries: i32) {
    unsafe {
        let mut listWindow = listWindow;
        let mut offset = offset;
        let mut entries = entries;
        let mut tileData: *mut u8 = ((GetWindowAttribute(
            ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
            7u8,
        )) as usize as *mut u8);
        let mut width: u32 =
            ((((((listWindow).wrapping_add(4)).read()) as i32).wrapping_mul(64i32)) as u32);
        offset = (((((listWindow).wrapping_add(10).cast::<u16>()).read()) as i32)
            .wrapping_add(offset)
            & 15i32);
        if (offset).wrapping_add(entries) <= 16i32 {
            {
                let mut tmp: u32 = 0u32;
                (&raw mut tmp).write_volatile(286331153u32);
                'l1: loop {
                    'l2: {
                        CpuFastSet(
                            (&raw mut tmp).cast::<u8>(),
                            (tileData).wrapping_offset(
                                ((((offset) as u32).wrapping_mul(width)) as i32) as isize,
                            ),
                            (16777216u32
                                | (crate::c::div_u32(
                                    ((entries) as u32).wrapping_mul(width),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l1;
                    }
                }
            }
            CopyWindowToVram(
                ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
                2u8,
            );
        } else {
            let mut v3: u32 = (((16i32).wrapping_sub(offset)) as u32);
            let mut v4: u32 = ((entries) as u32).wrapping_sub(v3);
            {
                let mut tmp: u32 = 0u32;
                (&raw mut tmp).write_volatile(286331153u32);
                'l3: loop {
                    'l4: {
                        CpuFastSet(
                            (&raw mut tmp).cast::<u8>(),
                            (tileData).wrapping_offset(
                                ((((offset) as u32).wrapping_mul(width)) as i32) as isize,
                            ),
                            (16777216u32
                                | (crate::c::div_u32(
                                    (v3).wrapping_mul(width),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            {
                let mut tmp: u32 = 0u32;
                (&raw mut tmp).write_volatile(286331153u32);
                'l5: loop {
                    'l6: {
                        CpuFastSet(
                            (&raw mut tmp).cast::<u8>(),
                            tileData,
                            (16777216u32
                                | (crate::c::div_u32(
                                    (v4).wrapping_mul(width),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l5;
                    }
                }
            }
            CopyWindowToVram(
                ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
                2u8,
            );
        }
        {
            entries = (entries).wrapping_sub(1);
            'l7: loop {
                if !(entries != (-1i32)) {
                    break 'l7;
                }
                'l8: {
                    ClearRematchPokeballIcon(
                        ((listWindow).wrapping_add(8).cast::<u16>()).read(),
                        ((offset) as u32),
                    );
                }
                offset = ((offset).wrapping_add(1i32) & 15i32);
                entries = (entries).wrapping_sub(1);
            }
        }
        CopyWindowToVram(
            ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetListMarginTile(listWindow: *mut u8, draw: u32) {
    unsafe {
        let mut listWindow = listWindow;
        let mut draw = draw;
        let mut var: u16 = 0u16;
        let mut tilemapBuffer: *mut u16 = (GetBgTilemapBuffer(
            ((GetWindowAttribute(
                ((((listWindow).wrapping_add(8).cast::<u16>()).read()) as u8),
                0u8,
            )) as u8),
        ))
        .cast::<u16>();
        tilemapBuffer = (tilemapBuffer).wrapping_offset(
            (((((((listWindow).wrapping_add(10).cast::<u16>()).read()) as i32) << 6)
                .wrapping_add(((((listWindow).wrapping_add(2)).read()) as i32)))
            .wrapping_sub(1i32)) as isize,
        );
        if (draw) != 0 {
            var = (((((((listWindow).wrapping_add(1)).read()) as i32) << 12)
                | ((((listWindow).wrapping_add(6).cast::<u16>()).read()) as i32).wrapping_add(1i32))
                as u16);
        } else {
            var = (((((((listWindow).wrapping_add(1)).read()) as i32) << 12)
                | ((((listWindow).wrapping_add(6).cast::<u16>()).read()) as i32))
                as u16);
        }
        (tilemapBuffer).write(var);
        ((tilemapBuffer).wrapping_offset(32)).write(var);
    }
}
pub(crate) unsafe extern "C" fn PrintCheckPageTrainerName(state: *mut u8, list: *mut u8) {
    unsafe {
        let mut state = state;
        let mut list = list;
        let mut colors = crate::ffi::Align4([0u8; 3]);
        (&raw mut colors).cast::<u8>().wrapping_add(0).write(0u8);
        (&raw mut colors).cast::<u8>().wrapping_add(1).write(2u8);
        (&raw mut colors).cast::<u8>().wrapping_add(2).write(5u8);
        (((list)
            .wrapping_add(52)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .read())
        .unwrap_unchecked()(
            (((state).wrapping_add(16).cast::<*mut u8>()).read()).wrapping_offset(
                (((((state).wrapping_add(12).cast::<u32>()).read())
                    .wrapping_mul(((((state).cast::<u16>()).read()) as u32)))
                    as i32) as isize
                    * 1,
            ),
            ((list).wrapping_add(72)).cast::<u8>(),
        );
        (((list)
            .wrapping_add(56)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
        .read())
        .unwrap_unchecked()(
            ((list).wrapping_add(8).cast::<u16>()).read(),
            ((((state).cast::<u16>()).read()) as u32),
            ((((list).wrapping_add(10).cast::<u16>()).read()) as u32),
        );
        FillWindowPixelRect(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
            68u8,
            0u16,
            ((((((list).wrapping_add(10).cast::<u16>()).read()) as i32).wrapping_mul(16i32))
                as u16),
            ((((((list).wrapping_add(4)).read()) as i32).wrapping_mul(8i32)) as u16),
            16u16,
        );
        AddTextPrinterParameterized3(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
            ((list).wrapping_add(5)).read(),
            8u8,
            (((((((list).wrapping_add(10).cast::<u16>()).read()) as i32).wrapping_mul(16i32))
                .wrapping_add(1i32)) as u8),
            (&raw mut colors).cast::<u8>(),
            (-1i8),
            ((list).wrapping_add(72)).cast::<u8>(),
        );
        SetListMarginTile((list), 1u32);
        CopyWindowRectToVram(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u32),
            3u32,
            0u32,
            ((((((list).wrapping_add(10).cast::<u16>()).read()) as i32).wrapping_mul(2i32)) as u32),
            ((((list).wrapping_add(4)).read()) as u32),
            2u32,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallListTrainerName(state: *mut u8, list: *mut u8) {
    unsafe {
        let mut state = state;
        let mut list = list;
        (((list)
            .wrapping_add(52)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .read())
        .unwrap_unchecked()(
            (((state).wrapping_add(16).cast::<*mut u8>()).read()).wrapping_offset(
                (((((state).wrapping_add(12).cast::<u32>()).read())
                    .wrapping_mul(((((state).cast::<u16>()).read()) as u32)))
                    as i32) as isize
                    * 1,
            ),
            ((list).wrapping_add(72)).cast::<u8>(),
        );
        FillWindowPixelRect(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
            17u8,
            0u16,
            ((((((list).wrapping_add(10).cast::<u16>()).read()) as i32).wrapping_mul(16i32))
                as u16),
            ((((((list).wrapping_add(4)).read()) as i32).wrapping_mul(8i32)) as u16),
            16u16,
        );
        AddTextPrinterParameterized(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
            ((list).wrapping_add(5)).read(),
            ((list).wrapping_add(72)).cast::<u8>(),
            8u8,
            (((((((list).wrapping_add(10).cast::<u16>()).read()) as i32).wrapping_mul(16i32))
                .wrapping_add(1i32)) as u8),
            255u8,
            None,
        );
        SetListMarginTile((list), 0u32);
        CopyWindowToVram(((((list).wrapping_add(8).cast::<u16>()).read()) as u8), 3u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallFieldNames(list: *mut u8, fieldId: u32) {
    unsafe {
        let mut list = list;
        let mut fieldId = fieldId;
        let mut fieldNames = crate::ffi::Align4([0u8; 12]);
        (&raw mut fieldNames)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write((&raw mut gText_PokenavMatchCall_Strategy).cast::<u8>());
        (&raw mut fieldNames)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw mut gText_PokenavMatchCall_TrainerPokemon).cast::<u8>());
        (&raw mut fieldNames)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut u8>()
            .write((&raw mut gText_PokenavMatchCall_SelfIntroduction).cast::<u8>());
        let mut colors = crate::ffi::Align4([0u8; 3]);
        (&raw mut colors).cast::<u8>().wrapping_add(0).write(1u8);
        (&raw mut colors).cast::<u8>().wrapping_add(1).write(4u8);
        (&raw mut colors).cast::<u8>().wrapping_add(2).write(5u8);
        let mut top: u32 = (((((((list).wrapping_add(10).cast::<u16>()).read()) as i32)
            .wrapping_add(1i32)) as u32)
            .wrapping_add((fieldId).wrapping_mul(2u32))
            & 15u32);
        FillWindowPixelRect(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
            17u8,
            0u16,
            ((top << 4) as u16),
            ((((list).wrapping_add(4)).read()) as u16),
            16u16,
        );
        AddTextPrinterParameterized3(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
            7u8,
            2u8,
            (((top << 4).wrapping_add(1u32)) as u8),
            (&raw mut colors).cast::<u8>(),
            (-1i8),
            (((&raw mut fieldNames).cast::<*mut u8>())
                .wrapping_offset(((fieldId) as i32) as isize))
            .read(),
        );
        CopyWindowRectToVram(
            ((((list).wrapping_add(8).cast::<u16>()).read()) as u32),
            2u32,
            0u32,
            (top << 1),
            ((((list).wrapping_add(4)).read()) as u32),
            2u32,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallFlavorText(
    windowState: *mut u8,
    list: *mut u8,
    checkPageEntry: u32,
) {
    unsafe {
        let mut windowState = windowState;
        let mut list = list;
        let mut checkPageEntry = checkPageEntry;
        let mut r6: u32 = ((((((list).wrapping_add(10).cast::<u16>()).read()) as i32).wrapping_add(
            ((((((&raw const lineOffsets_0).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((checkPageEntry) as i32) as isize))
            .read()) as i32),
        ) & 15i32) as u32);
        let mut str: *mut u8 = GetMatchCallFlavorText(
            ((((windowState).cast::<u16>()).read()) as i32),
            ((checkPageEntry) as i32),
        );
        if ((str) as usize) != 0usize {
            FillWindowTilesByRow(
                ((((list).wrapping_add(8).cast::<u16>()).read()) as i32),
                1i32,
                (((r6).wrapping_mul(2u32)) as i32),
                ((((list).wrapping_add(4)).read()) as i32).wrapping_sub(1i32),
                2i32,
            );
            AddTextPrinterParameterized(
                ((((list).wrapping_add(8).cast::<u16>()).read()) as u8),
                7u8,
                str,
                2u8,
                (((r6 << 4).wrapping_add(1u32)) as u8),
                255u8,
                None,
            );
            CopyWindowRectToVram(
                ((((list).wrapping_add(8).cast::<u16>()).read()) as u32),
                2u32,
                0u32,
                (r6).wrapping_mul(2u32),
                ((((list).wrapping_add(4)).read()) as u32),
                2u32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadListArrowGfx() {
    unsafe {
        let mut i: u32 = 0u32;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        {
            i = 0u32;
            ptr = ((&raw const sListArrowSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>();
            'l1: loop {
                if !(i < crate::c::div_u32(8u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(ptr);
                }
                ptr = (ptr).wrapping_offset(8);
                i = (i).wrapping_add(1);
            }
        }
        Pokenav_AllocAndLoadPalettes(
            ((&raw const sListArrowPalettes).cast::<u8>().cast_mut()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateListArrowSprites(windowState: *mut u8, list: *mut u8) {
    unsafe {
        let mut windowState = windowState;
        let mut list = list;
        let mut spriteId: u32 = 0u32;
        let mut x: i16 = 0i16;
        spriteId = ((CreateSprite(
            (&raw const sSpriteTemplate_RightArrow)
                .cast::<u8>()
                .cast_mut(),
            (((((((list).wrapping_add(2)).read()) as i32).wrapping_mul(8i32)).wrapping_add(3i32))
                as i16),
            (((((((list).wrapping_add(3)).read()) as i32).wrapping_add(1i32)).wrapping_mul(8i32))
                as i16),
            7u8,
        )) as u32);
        ((list).wrapping_add(60).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        x = (((((((list).wrapping_add(2)).read()) as i32).wrapping_mul(8i32)).wrapping_add(
            (((((list).wrapping_add(4)).read()) as i32).wrapping_sub(1i32)).wrapping_mul(4i32),
        )) as i16);
        spriteId = ((CreateSprite(
            (&raw const sSpriteTemplate_UpDownArrow)
                .cast::<u8>()
                .cast_mut(),
            x,
            (((((((list).wrapping_add(3)).read()) as i32).wrapping_mul(8i32)).wrapping_add(
                ((((windowState).wrapping_add(8).cast::<u16>()).read()) as i32).wrapping_mul(16i32),
            )) as i16),
            7u8,
        )) as u32);
        ((list).wrapping_add(68).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((list).wrapping_add(68).cast::<*mut u8>()).read()).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read(
                (((list).wrapping_add(68).cast::<*mut u8>()).read()).wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32)
                .wrapping_add(2i32)) as u16) as i32,
        );
        ((((list).wrapping_add(68).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_DownArrow));
        spriteId = ((CreateSprite(
            (&raw const sSpriteTemplate_UpDownArrow)
                .cast::<u8>()
                .cast_mut(),
            x,
            ((((((list).wrapping_add(3)).read()) as i32).wrapping_mul(8i32)) as i16),
            7u8,
        )) as u32);
        ((list).wrapping_add(64).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((list).wrapping_add(64).cast::<*mut u8>()).read()).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read(
                (((list).wrapping_add(64).cast::<*mut u8>()).read()).wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32)
                .wrapping_add(4i32)) as u16) as i32,
        );
        ((((list).wrapping_add(64).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_UpArrow));
    }
}
pub(crate) unsafe extern "C" fn DestroyListArrows(list: *mut u8) {
    unsafe {
        let mut list = list;
        DestroySprite(((list).wrapping_add(60).cast::<*mut u8>()).read());
        DestroySprite(((list).wrapping_add(64).cast::<*mut u8>()).read());
        DestroySprite(((list).wrapping_add(68).cast::<*mut u8>()).read());
        FreeSpriteTilesByTag(10u16);
        FreeSpritePaletteByTag(20u16);
    }
}
pub(crate) unsafe extern "C" fn ToggleListArrows(list: *mut u8, invisible: u32) {
    unsafe {
        let mut list = list;
        let mut invisible = invisible;
        if (invisible) != 0 {
            ((((list).wrapping_add(60).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            ((((list).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            ((((list).wrapping_add(68).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        } else {
            ((((list).wrapping_add(60).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_RightArrow));
            ((((list).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_UpArrow));
            ((((list).wrapping_add(68).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_DownArrow));
        }
        crate::c::bf_write(
            (((list).wrapping_add(60).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
        crate::c::bf_write(
            (((list).wrapping_add(64).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
        crate::c::bf_write(
            (((list).wrapping_add(68).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RightArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((list).wrapping_add(2184)).wrapping_add(6).cast::<u16>()).read()) as i32) << 4)
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DownArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (!((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0))
            && ((ShouldShowDownArrow()) != 0)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            let mut offset: i16 = 0i16;
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            offset = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                as i32)
                .wrapping_add(1i32)
                & 7i32) as i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(offset);
            ((sprite).wrapping_add(38).cast::<i16>()).write(offset);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (!((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0))
            && ((ShouldShowUpArrow()) != 0)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            let mut offset: i16 = 0i16;
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            offset = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                as i32)
                .wrapping_add(1i32)
                & 7i32) as i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(offset);
            ((sprite).wrapping_add(38).cast::<i16>())
                .write((((-1i32).wrapping_mul(((offset) as i32))) as i16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavList_ToggleVerticalArrows(invisible: u32) {
    unsafe {
        let mut invisible = invisible;
        let mut list: *mut u8 = GetSubstructPtr(17u32);
        ((((((list).wrapping_add(64).cast::<*mut u8>()).read()).wrapping_add(46)).cast::<i16>())
            .wrapping_offset(7))
        .write(((invisible) as i16));
        ((((((list).wrapping_add(68).cast::<*mut u8>()).read()).wrapping_add(46)).cast::<i16>())
            .wrapping_offset(7))
        .write(((invisible) as i16));
    }
}
pub(crate) unsafe extern "C" fn InitPokenavListWindowState(dst: *mut u8, template: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut template = template;
        ((dst).wrapping_add(16).cast::<*mut u8>()).write(((template).cast::<*mut u8>()).read());
        ((dst).cast::<u16>()).write(((template).wrapping_add(6).cast::<u16>()).read());
        ((dst).wrapping_add(2).cast::<u16>())
            .write(((template).wrapping_add(4).cast::<u16>()).read());
        ((dst).wrapping_add(12).cast::<u32>())
            .write(((((template).wrapping_add(8)).read()) as u32));
        ((dst).wrapping_add(8).cast::<u16>())
            .write(((((template).wrapping_add(12)).read()) as u16));
        if ((((dst).wrapping_add(8).cast::<u16>()).read()) as i32)
            >= ((((dst).wrapping_add(2).cast::<u16>()).read()) as i32)
        {
            ((dst).cast::<u16>()).write(0u16);
            ((dst).wrapping_add(4).cast::<u16>()).write(0u16);
            ((dst).wrapping_add(6).cast::<u16>())
                .write(((template).wrapping_add(6).cast::<u16>()).read());
        } else {
            ((dst).wrapping_add(4).cast::<u16>()).write(
                ((((((dst).wrapping_add(2).cast::<u16>()).read()) as i32)
                    .wrapping_sub(((((dst).wrapping_add(8).cast::<u16>()).read()) as i32)))
                    as u16),
            );
            if ((((dst).cast::<u16>()).read()) as i32)
                .wrapping_add(((((dst).wrapping_add(8).cast::<u16>()).read()) as i32))
                > ((((dst).wrapping_add(2).cast::<u16>()).read()) as i32)
            {
                ((dst).wrapping_add(6).cast::<u16>()).write(
                    (((((((dst).cast::<u16>()).read()) as i32)
                        .wrapping_add(((((dst).wrapping_add(8).cast::<u16>()).read()) as i32)))
                    .wrapping_sub(((((dst).wrapping_add(2).cast::<u16>()).read()) as i32)))
                        as u16),
                );
                ((dst).cast::<u16>()).write(
                    ((((((template).wrapping_add(6).cast::<u16>()).read()) as i32)
                        .wrapping_sub(((((dst).wrapping_add(6).cast::<u16>()).read()) as i32)))
                        as u16),
                );
            } else {
                ((dst).wrapping_add(6).cast::<u16>()).write(0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CopyPokenavListMenuTemplate(
    dest: *mut u8,
    bgTemplate: *mut u8,
    template: *mut u8,
    tileOffset: u32,
) -> u32 {
    unsafe {
        let mut dest = dest;
        let mut bgTemplate = bgTemplate;
        let mut template = template;
        let mut tileOffset = tileOffset;
        let mut window = crate::ffi::Align4([0u8; 8]);
        (dest).write(((crate::c::bf_read((bgTemplate).wrapping_add(0), 0, 2, false) as u16) as u8));
        ((dest).wrapping_add(6).cast::<u16>()).write(((tileOffset) as u16));
        ((dest)
            .wrapping_add(52)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .write(
            ((template)
                .wrapping_add(16)
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
            .read(),
        );
        ((dest)
            .wrapping_add(56)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
        .write(
            ((template)
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
            .read(),
        );
        ((dest).wrapping_add(1)).write(((template).wrapping_add(13)).read());
        ((dest).wrapping_add(2)).write(((template).wrapping_add(9)).read());
        ((dest).wrapping_add(3)).write(((template).wrapping_add(11)).read());
        ((dest).wrapping_add(4)).write(((template).wrapping_add(10)).read());
        ((dest).wrapping_add(5)).write(((template).wrapping_add(14)).read());
        ((&raw mut window).cast::<u8>())
            .write(((crate::c::bf_read((bgTemplate).wrapping_add(0), 0, 2, false) as u16) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(1))
            .write(((template).wrapping_add(9)).read());
        (((&raw mut window).cast::<u8>()).wrapping_add(2)).write(0u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(3))
            .write(((template).wrapping_add(10)).read());
        (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(32u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(5))
            .write(((template).wrapping_add(13)).read());
        (((&raw mut window).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write((((tileOffset).wrapping_add(2u32)) as u16));
        ((dest).wrapping_add(8).cast::<u16>()).write(AddWindow((&raw mut window).cast::<u8>()));
        if ((((dest).wrapping_add(8).cast::<u16>()).read()) as i32) == 255i32 {
            return 0u32;
        }
        ((dest).wrapping_add(10).cast::<u16>()).write(0u16);
        ((dest).wrapping_add(60).cast::<*mut u8>()).write(core::ptr::null_mut());
        ((dest).wrapping_add(64).cast::<*mut u8>()).write(core::ptr::null_mut());
        ((dest).wrapping_add(68).cast::<*mut u8>()).write(core::ptr::null_mut());
        return 1u32;
    }
}
