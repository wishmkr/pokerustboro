//! Translated from `src/list_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sScrollIndicatorTemplates sOamData_ScrollArrowIndicator sSpriteAnim_ScrollArrowIndicator0 sSpriteAnim_ScrollArrowIndicator1 sSpriteAnim_ScrollArrowIndicator2 sSpriteAnim_ScrollArrowIndicator3 sSpriteAnimTable_ScrollArrowIndicator sSpriteTemplate_ScrollArrowIndicator sSubsprite_RedOutline1 sSubsprite_RedOutline2 sSubsprite_RedOutline3 sSubsprite_RedOutline4 sSubsprite_RedOutline5 sSubsprite_RedOutline6 sSubsprite_RedOutline7 sSubsprite_RedOutline8 sOamData_RedArrowCursor sSpriteAnim_RedArrowCursor sSpriteAnimTable_RedArrowCursor sSpriteTemplate_RedArrowCursor sRedInterface_Pal sScrollIndicator_Gfx sOutlineCursor_Gfx sArrowCursor_Gfx
#[allow(unused_imports)]
use crate::data::list_menu::*;

pub(crate) static mut sMysteryGiftLinkMenu: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTempScrollArrowTemplate: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
pub static mut gListMenuOverride: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMultiuseListMenuTemplate: crate::ffi::Align4<[u8; 24]> =
    crate::ffi::Align4([0; 24]);

unsafe extern "C" {
    static mut gDummySpriteTemplate: u8;
    static mut gMain: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_SelectorArrow2: u8;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn PlaySE(a0: u16);
    fn PutWindowRectTilemapOverridePalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScrollWindow(a0: u8, a1: u8, a2: u8, a3: u8);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn SetWindowAttribute(a0: u8, a1: u8, a2: u32) -> u8;
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
}

pub(crate) unsafe extern "C" fn ListMenuDummyTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMysteryGiftListMenu(
    windowTemplate: *mut u8,
    listMenuTemplate: *mut u8,
    drawMode: u8,
    tileNum: u16,
    palOffset: u16,
) -> i32 {
    unsafe {
        let mut windowTemplate = windowTemplate;
        let mut listMenuTemplate = listMenuTemplate;
        let mut drawMode = drawMode;
        let mut tileNum = tileNum;
        let mut palOffset = palOffset;
        'l1: {
            let __sw1 =
                (((((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(4)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5))
                    .write(((AddWindow(windowTemplate)) as u8));
                'l2: {
                    let __sw2 = ((drawMode) as i32);
                    let mut __fall = false;
                    if __sw2 == 2i32 {
                        __fall = true;
                        LoadUserWindowBorderGfx(
                            (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read(),
                            tileNum,
                            ((palOffset) as u8),
                        );
                    }
                    if __fall || __sw2 == 1i32 {
                        __fall = true;
                        DrawTextBorderOuter(
                            (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read(),
                            tileNum,
                            ((crate::c::div_i32(((palOffset) as i32), 16i32)) as u8),
                        );
                        break 'l2;
                    }
                }
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        listMenuTemplate
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write((((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read());
                (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(6)).write(
                    ListMenuInit(
                        (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                        0u16,
                        0u16,
                    ),
                );
                CopyWindowToVram(
                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read(),
                    1u8,
                );
                (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(4)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).cast::<i32>()).write(
                    ListMenu_ProcessInput(
                        (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(6)).read(),
                    ),
                );
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(4)).write(2u8);
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).cast::<i32>()).write((-2i32));
                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(4)).write(2u8);
                }
                if (((((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(4)).read())
                    as i32)
                    == 2i32
                {
                    if ((drawMode) as i32) == 0i32 {
                        ClearWindowTilemap(
                            (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read(),
                        );
                    } else {
                        'l3: {
                            let __sw3 = ((drawMode) as i32);
                            if __sw3 == 0i32 {
                                ClearStdWindowAndFrame(
                                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>())
                                        .wrapping_add(5))
                                    .read(),
                                    0u8,
                                );
                                break 'l3;
                            }
                            if __sw3 == 2i32 || __sw3 == 1i32 {
                                ClearStdWindowAndFrame(
                                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>())
                                        .wrapping_add(5))
                                    .read(),
                                    0u8,
                                );
                                break 'l3;
                            }
                        }
                    }
                    CopyWindowToVram(
                        (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read(),
                        1u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroyListMenuTask(
                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(6)).read(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                RemoveWindow(
                    (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(5)).read(),
                );
                (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).wrapping_add(4)).write(0u8);
                return (((&raw mut sMysteryGiftLinkMenu).cast::<u8>()).cast::<i32>()).read();
            }
        }
        return (-1i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuInit(
    listMenuTemplate: *mut u8,
    scrollOffset: u16,
    selectedRow: u16,
) -> u8 {
    unsafe {
        let mut listMenuTemplate = listMenuTemplate;
        let mut scrollOffset = scrollOffset;
        let mut selectedRow = selectedRow;
        let mut taskId: u8 = ListMenuInitInternal(listMenuTemplate, scrollOffset, selectedRow);
        PutWindowTilemap(((listMenuTemplate).wrapping_add(16)).read());
        CopyWindowToVram(((listMenuTemplate).wrapping_add(16)).read(), 2u8);
        return taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuInitInRect(
    listMenuTemplate: *mut u8,
    rect: *mut u8,
    scrollOffset: u16,
    selectedRow: u16,
) -> u8 {
    unsafe {
        let mut listMenuTemplate = listMenuTemplate;
        let mut rect = rect;
        let mut scrollOffset = scrollOffset;
        let mut selectedRow = selectedRow;
        let mut i: i32 = 0i32;
        let mut taskId: u8 = ListMenuInitInternal(listMenuTemplate, scrollOffset, selectedRow);
        {
            i = 0i32;
            'l1: loop {
                if !((((((rect).wrapping_offset((i) as isize * 8)).wrapping_add(4)).read()) as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                'l2: {
                    PutWindowRectTilemapOverridePalette(
                        ((listMenuTemplate).wrapping_add(16)).read(),
                        ((rect).wrapping_offset((i) as isize * 8)).read(),
                        (((rect).wrapping_offset((i) as isize * 8)).wrapping_add(1)).read(),
                        (((rect).wrapping_offset((i) as isize * 8)).wrapping_add(2)).read(),
                        (((rect).wrapping_offset((i) as isize * 8)).wrapping_add(3)).read(),
                        (((rect).wrapping_offset((i) as isize * 8)).wrapping_add(4)).read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(((listMenuTemplate).wrapping_add(16)).read(), 2u8);
        return taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenu_ProcessInput(listTaskId: u8) -> i32 {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            return (((((list).cast::<*mut u8>()).read()).wrapping_offset(
                (((((list).wrapping_add(24).cast::<u16>()).read()) as i32)
                    .wrapping_add(((((list).wrapping_add(26).cast::<u16>()).read()) as i32)))
                    as isize
                    * 8,
            ))
            .wrapping_add(4)
            .cast::<i32>())
            .read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-2i32);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    ListMenuChangeSelection(list, 1u8, 1u8, 0u8);
                    return (-1i32);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        ListMenuChangeSelection(list, 1u8, 1u8, 1u8);
                        return (-1i32);
                    } else {
                        let mut rightButton: u16 = 0u16;
                        let mut leftButton: u16 = 0u16;
                        'l1: {
                            let __sw1 = ((crate::c::bf_read((list).wrapping_add(22), 6, 2, false)
                                as u8) as i32);
                            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
                            if __sw1 == 0i32 || !__matched {
                                leftButton = 0u16;
                                rightButton = 0u16;
                                break 'l1;
                            }
                            if __sw1 == 1i32 {
                                leftButton = (((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(48)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 32i32) as u16);
                                rightButton = (((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(48)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 16i32) as u16);
                                break 'l1;
                            }
                            if __sw1 == 2i32 {
                                leftButton = (((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(48)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 512i32) as u16);
                                rightButton = (((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(48)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 256i32)
                                    as u16);
                                break 'l1;
                            }
                        }
                        if (leftButton) != 0 {
                            ListMenuChangeSelection(
                                list,
                                1u8,
                                ((((list).wrapping_add(14).cast::<u16>()).read()) as u8),
                                0u8,
                            );
                            return (-1i32);
                        } else {
                            if (rightButton) != 0 {
                                ListMenuChangeSelection(
                                    list,
                                    1u8,
                                    ((((list).wrapping_add(14).cast::<u16>()).read()) as u8),
                                    1u8,
                                );
                                return (-1i32);
                            } else {
                                return (-1i32);
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyListMenuTask(
    listTaskId: u8,
    scrollOffset: *mut u16,
    selectedRow: *mut u16,
) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut scrollOffset = scrollOffset;
        let mut selectedRow = selectedRow;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        if ((scrollOffset) as usize) != 0usize {
            (scrollOffset).write(((list).wrapping_add(24).cast::<u16>()).read());
        }
        if ((selectedRow) as usize) != 0usize {
            (selectedRow).write(((list).wrapping_add(26).cast::<u16>()).read());
        }
        if ((((list).wrapping_add(30)).read()) as i32) != 255i32 {
            ListMenuRemoveCursorObject(
                ((list).wrapping_add(30)).read(),
                ((((crate::c::bf_read((list).wrapping_add(23), 6, 2, false) as u8) as i32)
                    .wrapping_sub(2i32)) as u32),
            );
        }
        DestroyTask(listTaskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RedrawListMenu(listTaskId: u8) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        FillWindowPixelBuffer(
            ((list).wrapping_add(16)).read(),
            ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32) << 4))
                as u8),
        );
        ListMenuPrintEntries(
            list,
            ((list).wrapping_add(24).cast::<u16>()).read(),
            0u16,
            ((list).wrapping_add(14).cast::<u16>()).read(),
        );
        ListMenuDrawCursor(list);
        CopyWindowToVram(((list).wrapping_add(16)).read(), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeListMenuPals(
    listTaskId: u8,
    cursorPal: u8,
    fillValue: u8,
    cursorShadowPal: u8,
) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut cursorPal = cursorPal;
        let mut fillValue = fillValue;
        let mut cursorShadowPal = cursorShadowPal;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        crate::c::bf_write((list).wrapping_add(20), 4, 4, (cursorPal) as i32);
        crate::c::bf_write((list).wrapping_add(21), 0, 4, (fillValue) as i32);
        crate::c::bf_write((list).wrapping_add(21), 4, 4, (cursorShadowPal) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeListMenuCoords(listTaskId: u8, x: u8, y: u8) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut x = x;
        let mut y = y;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        SetWindowAttribute(((list).wrapping_add(16)).read(), 1u8, ((x) as u32));
        SetWindowAttribute(((list).wrapping_add(16)).read(), 2u8, ((y) as u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuTestInput(
    template: *mut u8,
    scrollOffset: u32,
    selectedRow: u32,
    keys: u16,
    newScrollOffset: *mut u16,
    newSelectedRow: *mut u16,
) -> i32 {
    unsafe {
        let mut template = template;
        let mut scrollOffset = scrollOffset;
        let mut selectedRow = selectedRow;
        let mut keys = keys;
        let mut newScrollOffset = newScrollOffset;
        let mut newSelectedRow = newSelectedRow;
        let mut list = crate::ffi::Align4([0u8; 32]);
        ((&raw mut list).cast::<u8>())
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(template.cast::<crate::c::Rec4<24>>().read_unaligned());
        (((&raw mut list).cast::<u8>())
            .wrapping_add(24)
            .cast::<u16>())
        .write(((scrollOffset) as u16));
        (((&raw mut list).cast::<u8>())
            .wrapping_add(26)
            .cast::<u16>())
        .write(((selectedRow) as u16));
        (((&raw mut list).cast::<u8>()).wrapping_add(28)).write(0u8);
        (((&raw mut list).cast::<u8>()).wrapping_add(29)).write(0u8);
        if ((keys) as i32) == 64i32 {
            ListMenuChangeSelection((&raw mut list).cast::<u8>(), 0u8, 1u8, 0u8);
        }
        if ((keys) as i32) == 128i32 {
            ListMenuChangeSelection((&raw mut list).cast::<u8>(), 0u8, 1u8, 1u8);
        }
        if ((newScrollOffset) as usize) != 0usize {
            (newScrollOffset).write(
                (((&raw mut list).cast::<u8>())
                    .wrapping_add(24)
                    .cast::<u16>())
                .read(),
            );
        }
        if ((newSelectedRow) as usize) != 0usize {
            (newSelectedRow).write(
                (((&raw mut list).cast::<u8>())
                    .wrapping_add(26)
                    .cast::<u16>())
                .read(),
            );
        }
        return (-1i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetCurrentItemArrayId(listTaskId: u8, arrayId: *mut u16) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut arrayId = arrayId;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        if ((arrayId) as usize) != 0usize {
            (arrayId).write(
                ((((((list).wrapping_add(24).cast::<u16>()).read()) as i32)
                    .wrapping_add(((((list).wrapping_add(26).cast::<u16>()).read()) as i32)))
                    as u16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetScrollAndRow(
    listTaskId: u8,
    scrollOffset: *mut u16,
    selectedRow: *mut u16,
) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut scrollOffset = scrollOffset;
        let mut selectedRow = selectedRow;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        if ((scrollOffset) as usize) != 0usize {
            (scrollOffset).write(((list).wrapping_add(24).cast::<u16>()).read());
        }
        if ((selectedRow) as usize) != 0usize {
            (selectedRow).write(((list).wrapping_add(26).cast::<u16>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetYCoordForPrintingArrowCursor(listTaskId: u8) -> u16 {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        let mut yMultiplier: u8 = ((((GetFontAttribute(
            (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
            1u8,
        )) as i32)
            .wrapping_add(((crate::c::bf_read((list).wrapping_add(22), 3, 3, false) as u8) as i32)))
            as u8);
        return (((((((list).wrapping_add(26).cast::<u16>()).read()) as i32)
            .wrapping_mul(((yMultiplier) as i32)))
        .wrapping_add(((crate::c::bf_read((list).wrapping_add(20), 0, 4, false) as u8) as i32)))
            as u16);
    }
}
pub(crate) unsafe extern "C" fn ListMenuInitInternal(
    listMenuTemplate: *mut u8,
    scrollOffset: u16,
    selectedRow: u16,
) -> u8 {
    unsafe {
        let mut listMenuTemplate = listMenuTemplate;
        let mut scrollOffset = scrollOffset;
        let mut selectedRow = selectedRow;
        let mut listTaskId: u8 = CreateTask(Some(ListMenuDummyTask), 0u8);
        let mut list: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((listTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        (list).cast::<crate::c::Rec4<24>>().write_unaligned(
            listMenuTemplate
                .cast::<crate::c::Rec4<24>>()
                .read_unaligned(),
        );
        ((list).wrapping_add(24).cast::<u16>()).write(scrollOffset);
        ((list).wrapping_add(26).cast::<u16>()).write(selectedRow);
        ((list).wrapping_add(28)).write(0u8);
        ((list).wrapping_add(29)).write(0u8);
        ((list).wrapping_add(30)).write(255u8);
        ((list).wrapping_add(31)).write(0u8);
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(0),
            0,
            4,
            (crate::c::bf_read((list).wrapping_add(20), 4, 4, false) as u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(0),
            4,
            4,
            (crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(1),
            0,
            4,
            (crate::c::bf_read((list).wrapping_add(21), 4, 4, false) as u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(2),
            0,
            6,
            (crate::c::bf_read((list).wrapping_add(22), 0, 3, false) as u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(4),
            0,
            7,
            (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(4),
            7,
            1,
            (0u8) as i32,
        );
        if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
            < ((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
        {
            ((list).wrapping_add(14).cast::<u16>())
                .write(((list).wrapping_add(12).cast::<u16>()).read());
        }
        FillWindowPixelBuffer(
            ((list).wrapping_add(16)).read(),
            ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32) << 4))
                as u8),
        );
        ListMenuPrintEntries(
            list,
            ((list).wrapping_add(24).cast::<u16>()).read(),
            0u16,
            ((list).wrapping_add(14).cast::<u16>()).read(),
        );
        ListMenuDrawCursor(list);
        ListMenuCallSelectionChangedCallback(list, 1u8);
        return listTaskId;
    }
}
pub(crate) unsafe extern "C" fn ListMenuPrint(list: *mut u8, str: *mut u8, x: u8, y: u8) {
    unsafe {
        let mut list = list;
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut colors = crate::ffi::Align4([0u8; 3]);
        if (crate::c::bf_read(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(4),
            7,
            1,
            false,
        ) as u8)
            != 0
        {
            ((&raw mut colors).cast::<u8>()).write(
                (crate::c::bf_read(
                    ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(0),
                    4,
                    4,
                    false,
                ) as u8),
            );
            (((&raw mut colors).cast::<u8>()).wrapping_offset(1)).write(
                (crate::c::bf_read(
                    ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(0),
                    0,
                    4,
                    false,
                ) as u8),
            );
            (((&raw mut colors).cast::<u8>()).wrapping_offset(2)).write(
                (crate::c::bf_read(
                    ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(1),
                    0,
                    4,
                    false,
                ) as u8),
            );
            AddTextPrinterParameterized4(
                ((list).wrapping_add(16)).read(),
                (crate::c::bf_read(
                    ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(4),
                    0,
                    7,
                    false,
                ) as u8),
                x,
                y,
                (crate::c::bf_read(
                    ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(2),
                    0,
                    6,
                    false,
                ) as u8),
                0u8,
                (&raw mut colors).cast::<u8>(),
                (-1i8),
                str,
            );
            crate::c::bf_write(
                ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(4),
                7,
                1,
                (0u8) as i32,
            );
        } else {
            ((&raw mut colors).cast::<u8>())
                .write((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8));
            (((&raw mut colors).cast::<u8>()).wrapping_offset(1))
                .write((crate::c::bf_read((list).wrapping_add(20), 4, 4, false) as u8));
            (((&raw mut colors).cast::<u8>()).wrapping_offset(2))
                .write((crate::c::bf_read((list).wrapping_add(21), 4, 4, false) as u8));
            AddTextPrinterParameterized4(
                ((list).wrapping_add(16)).read(),
                (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
                x,
                y,
                (crate::c::bf_read((list).wrapping_add(22), 0, 3, false) as u8),
                0u8,
                (&raw mut colors).cast::<u8>(),
                (-1i8),
                str,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuPrintEntries(
    list: *mut u8,
    startIndex: u16,
    yOffset: u16,
    count: u16,
) {
    unsafe {
        let mut list = list;
        let mut startIndex = startIndex;
        let mut yOffset = yOffset;
        let mut count = count;
        let mut i: i32 = 0i32;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut yMultiplier: u8 = ((((GetFontAttribute(
            (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
            1u8,
        )) as i32)
            .wrapping_add(((crate::c::bf_read((list).wrapping_add(22), 3, 3, false) as u8) as i32)))
            as u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((list).cast::<*mut u8>()).read())
                        .wrapping_offset(((startIndex) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read()
                        != (-3i32)
                    {
                        x = ((list).wrapping_add(18)).read();
                    } else {
                        x = ((list).wrapping_add(17)).read();
                    }
                    y = ((((((yOffset) as i32).wrapping_add(i))
                        .wrapping_mul(((yMultiplier) as i32)))
                    .wrapping_add(
                        ((crate::c::bf_read((list).wrapping_add(20), 0, 4, false) as u8) as i32),
                    )) as u8);
                    if core::mem::transmute::<_, usize>(
                        ((list)
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn(u8, u32, u8)>>())
                        .read(),
                    ) != 0usize
                    {
                        (((list)
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn(u8, u32, u8)>>())
                        .read())
                        .unwrap_unchecked()(
                            ((list).wrapping_add(16)).read(),
                            (((((((list).cast::<*mut u8>()).read())
                                .wrapping_offset(((startIndex) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<i32>())
                            .read()) as u32),
                            y,
                        );
                    }
                    ListMenuPrint(
                        list,
                        (((((list).cast::<*mut u8>()).read())
                            .wrapping_offset(((startIndex) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .read(),
                        x,
                        y,
                    );
                    startIndex = (startIndex).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuDrawCursor(list: *mut u8) {
    unsafe {
        let mut list = list;
        let mut yMultiplier: u8 = ((((GetFontAttribute(
            (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
            1u8,
        )) as i32)
            .wrapping_add(((crate::c::bf_read((list).wrapping_add(22), 3, 3, false) as u8) as i32)))
            as u8);
        let mut x: u8 = ((list).wrapping_add(19)).read();
        let mut y: u8 = (((((((list).wrapping_add(26).cast::<u16>()).read()) as i32)
            .wrapping_mul(((yMultiplier) as i32)))
        .wrapping_add(((crate::c::bf_read((list).wrapping_add(20), 0, 4, false) as u8) as i32)))
            as u8);
        'l1: {
            let __sw1 = ((crate::c::bf_read((list).wrapping_add(23), 6, 2, false) as u8) as i32);
            if __sw1 == 0i32 {
                ListMenuPrint(list, (&raw mut gText_SelectorArrow2).cast::<u8>(), x, y);
                break 'l1;
            }
            if __sw1 == 1i32 {
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((list).wrapping_add(30)).read()) as i32) == 255i32 {
                    ((list).wrapping_add(30)).write(ListMenuAddCursorObject(list, 0u32));
                }
                ListMenuUpdateCursorObject(
                    ((list).wrapping_add(30)).read(),
                    ((((GetWindowAttribute(((list).wrapping_add(16)).read(), 1u8))
                        .wrapping_mul(8u32))
                    .wrapping_sub(1u32)) as u16),
                    (((((GetWindowAttribute(((list).wrapping_add(16)).read(), 2u8))
                        .wrapping_mul(8u32))
                    .wrapping_add(((y) as u32)))
                    .wrapping_sub(1u32)) as u16),
                    0u32,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((list).wrapping_add(30)).read()) as i32) == 255i32 {
                    ((list).wrapping_add(30)).write(ListMenuAddCursorObject(list, 1u32));
                }
                ListMenuUpdateCursorObject(
                    ((list).wrapping_add(30)).read(),
                    ((((GetWindowAttribute(((list).wrapping_add(16)).read(), 1u8))
                        .wrapping_mul(8u32))
                    .wrapping_add(((x) as u32))) as u16),
                    ((((GetWindowAttribute(((list).wrapping_add(16)).read(), 2u8))
                        .wrapping_mul(8u32))
                    .wrapping_add(((y) as u32))) as u16),
                    1u32,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuAddCursorObject(list: *mut u8, cursorObjId: u32) -> u8 {
    unsafe {
        let mut list = list;
        let mut cursorObjId = cursorObjId;
        let mut cursor = crate::ffi::Align4([0u8; 12]);
        ((&raw mut cursor).cast::<u8>()).write(0u8);
        (((&raw mut cursor).cast::<u8>()).wrapping_add(1)).write(160u8);
        (((&raw mut cursor).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            ((((GetWindowAttribute(((list).wrapping_add(16)).read(), 3u8)).wrapping_mul(8u32))
                .wrapping_add(2u32)) as u16),
        );
        (((&raw mut cursor).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(
            ((((GetFontAttribute(
                (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
                1u8,
            )) as i32)
                .wrapping_add(2i32)) as u16),
        );
        (((&raw mut cursor).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(16384u16);
        (((&raw mut cursor).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>())
        .write(65535u16);
        (((&raw mut cursor).cast::<u8>()).wrapping_add(10)).write(15u8);
        return ListMenuAddCursorObjectInternal((&raw mut cursor).cast::<u8>(), cursorObjId);
    }
}
pub(crate) unsafe extern "C" fn ListMenuErasePrintedCursor(list: *mut u8, selectedRow: u16) {
    unsafe {
        let mut list = list;
        let mut selectedRow = selectedRow;
        let mut cursorKind: u8 = (crate::c::bf_read((list).wrapping_add(23), 6, 2, false) as u8);
        if ((cursorKind) as i32) == 0i32 {
            let mut yMultiplier: u8 = ((((GetFontAttribute(
                (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
                1u8,
            )) as i32)
                .wrapping_add(
                    ((crate::c::bf_read((list).wrapping_add(22), 3, 3, false) as u8) as i32),
                )) as u8);
            let mut width: u8 = GetMenuCursorDimensionByFont(
                (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
                0u8,
            );
            let mut height: u8 = GetMenuCursorDimensionByFont(
                (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
                1u8,
            );
            FillWindowPixelRect(
                ((list).wrapping_add(16)).read(),
                ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                    | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                        << 4)) as u8),
                ((((list).wrapping_add(19)).read()) as u16),
                (((((selectedRow) as i32).wrapping_mul(((yMultiplier) as i32))).wrapping_add(
                    ((crate::c::bf_read((list).wrapping_add(20), 0, 4, false) as u8) as i32),
                )) as u16),
                ((width) as u16),
                ((height) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuUpdateSelectedRowIndexAndScrollOffset(
    list: *mut u8,
    movingDown: u8,
) -> u8 {
    unsafe {
        let mut list = list;
        let mut movingDown = movingDown;
        let mut selectedRow: u16 = ((list).wrapping_add(26).cast::<u16>()).read();
        let mut scrollOffset: u16 = ((list).wrapping_add(24).cast::<u16>()).read();
        let mut newRow: u16 = 0u16;
        let mut newScroll: u32 = 0u32;
        if !((movingDown) != 0) {
            if ((((list).wrapping_add(14).cast::<u16>()).read()) as i32) == 1i32 {
                newRow = 0u16;
            } else {
                newRow = (((((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
                    .wrapping_sub(
                        (crate::c::div_i32(
                            ((((list).wrapping_add(14).cast::<u16>()).read()) as i32),
                            2i32,
                        ))
                        .wrapping_add(crate::c::rem_i32(
                            ((((list).wrapping_add(14).cast::<u16>()).read()) as i32),
                            2i32,
                        )),
                    ))
                .wrapping_sub(1i32)) as u16);
            }
            if ((scrollOffset) as i32) == 0i32 {
                'l1: loop {
                    if !(((selectedRow) as i32) != 0i32) {
                        break 'l1;
                    }
                    selectedRow = (selectedRow).wrapping_sub(1);
                    if (((((list).cast::<*mut u8>()).read()).wrapping_offset(
                        (((scrollOffset) as i32).wrapping_add(((selectedRow) as i32))) as isize * 8,
                    ))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read()
                        != (-3i32)
                    {
                        ((list).wrapping_add(26).cast::<u16>()).write(selectedRow);
                        return 1u8;
                    }
                }
                return 0u8;
            } else {
                'l2: loop {
                    if !(((selectedRow) as i32) > ((newRow) as i32)) {
                        break 'l2;
                    }
                    selectedRow = (selectedRow).wrapping_sub(1);
                    if (((((list).cast::<*mut u8>()).read()).wrapping_offset(
                        (((scrollOffset) as i32).wrapping_add(((selectedRow) as i32))) as isize * 8,
                    ))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read()
                        != (-3i32)
                    {
                        ((list).wrapping_add(26).cast::<u16>()).write(selectedRow);
                        return 1u8;
                    }
                }
                newScroll = ((((scrollOffset) as i32).wrapping_sub(1i32)) as u32);
            }
        } else {
            if ((((list).wrapping_add(14).cast::<u16>()).read()) as i32) == 1i32 {
                newRow = 0u16;
            } else {
                newRow = (((crate::c::div_i32(
                    ((((list).wrapping_add(14).cast::<u16>()).read()) as i32),
                    2i32,
                ))
                .wrapping_add(crate::c::rem_i32(
                    ((((list).wrapping_add(14).cast::<u16>()).read()) as i32),
                    2i32,
                ))) as u16);
            }
            if ((scrollOffset) as i32)
                == ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
                    .wrapping_sub(((((list).wrapping_add(14).cast::<u16>()).read()) as i32))
            {
                'l3: loop {
                    if !(((selectedRow) as i32)
                        < ((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
                            .wrapping_sub(1i32))
                    {
                        break 'l3;
                    }
                    selectedRow = (selectedRow).wrapping_add(1);
                    if (((((list).cast::<*mut u8>()).read()).wrapping_offset(
                        (((scrollOffset) as i32).wrapping_add(((selectedRow) as i32))) as isize * 8,
                    ))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read()
                        != (-3i32)
                    {
                        ((list).wrapping_add(26).cast::<u16>()).write(selectedRow);
                        return 1u8;
                    }
                }
                return 0u8;
            } else {
                'l4: loop {
                    if !(((selectedRow) as i32) < ((newRow) as i32)) {
                        break 'l4;
                    }
                    selectedRow = (selectedRow).wrapping_add(1);
                    if (((((list).cast::<*mut u8>()).read()).wrapping_offset(
                        (((scrollOffset) as i32).wrapping_add(((selectedRow) as i32))) as isize * 8,
                    ))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read()
                        != (-3i32)
                    {
                        ((list).wrapping_add(26).cast::<u16>()).write(selectedRow);
                        return 1u8;
                    }
                }
                newScroll = ((((scrollOffset) as i32).wrapping_add(1i32)) as u32);
            }
        }
        ((list).wrapping_add(26).cast::<u16>()).write(newRow);
        ((list).wrapping_add(24).cast::<u16>()).write(((newScroll) as u16));
        return 2u8;
    }
}
pub(crate) unsafe extern "C" fn ListMenuScroll(list: *mut u8, count: u8, movingDown: u8) {
    unsafe {
        let mut list = list;
        let mut count = count;
        let mut movingDown = movingDown;
        if ((count) as i32) >= ((((list).wrapping_add(14).cast::<u16>()).read()) as i32) {
            FillWindowPixelBuffer(
                ((list).wrapping_add(16)).read(),
                ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                    | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                        << 4)) as u8),
            );
            ListMenuPrintEntries(
                list,
                ((list).wrapping_add(24).cast::<u16>()).read(),
                0u16,
                ((list).wrapping_add(14).cast::<u16>()).read(),
            );
        } else {
            let mut yMultiplier: u8 = ((((GetFontAttribute(
                (crate::c::bf_read((list).wrapping_add(23), 0, 6, false) as u8),
                1u8,
            )) as i32)
                .wrapping_add(
                    ((crate::c::bf_read((list).wrapping_add(22), 3, 3, false) as u8) as i32),
                )) as u8);
            if !((movingDown) != 0) {
                let mut y: u16 = 0u16;
                let mut width: u16 = 0u16;
                let mut height: u16 = 0u16;
                ScrollWindow(
                    ((list).wrapping_add(16)).read(),
                    1u8,
                    ((((count) as i32).wrapping_mul(((yMultiplier) as i32))) as u8),
                    ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                        | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8)
                            as i32)
                            << 4)) as u8),
                );
                ListMenuPrintEntries(
                    list,
                    ((list).wrapping_add(24).cast::<u16>()).read(),
                    0u16,
                    ((count) as u16),
                );
                y = (((((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
                    .wrapping_mul(((yMultiplier) as i32)))
                .wrapping_add(
                    ((crate::c::bf_read((list).wrapping_add(20), 0, 4, false) as u8) as i32),
                )) as u16);
                width = (((GetWindowAttribute(((list).wrapping_add(16)).read(), 3u8))
                    .wrapping_mul(8u32)) as u16);
                height = ((((GetWindowAttribute(((list).wrapping_add(16)).read(), 4u8))
                    .wrapping_mul(8u32))
                .wrapping_sub(((y) as u32))) as u16);
                FillWindowPixelRect(
                    ((list).wrapping_add(16)).read(),
                    ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                        | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8)
                            as i32)
                            << 4)) as u8),
                    0u16,
                    y,
                    width,
                    height,
                );
            } else {
                let mut width: u16 = 0u16;
                ScrollWindow(
                    ((list).wrapping_add(16)).read(),
                    0u8,
                    ((((count) as i32).wrapping_mul(((yMultiplier) as i32))) as u8),
                    ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                        | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8)
                            as i32)
                            << 4)) as u8),
                );
                ListMenuPrintEntries(
                    list,
                    ((((((list).wrapping_add(24).cast::<u16>()).read()) as i32).wrapping_add(
                        ((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
                            .wrapping_sub(((count) as i32)),
                    )) as u16),
                    ((((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
                        .wrapping_sub(((count) as i32))) as u16),
                    ((count) as u16),
                );
                width = (((GetWindowAttribute(((list).wrapping_add(16)).read(), 3u8))
                    .wrapping_mul(8u32)) as u16);
                FillWindowPixelRect(
                    ((list).wrapping_add(16)).read(),
                    ((((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8) as i32)
                        | (((crate::c::bf_read((list).wrapping_add(21), 0, 4, false) as u8)
                            as i32)
                            << 4)) as u8),
                    0u16,
                    0u16,
                    width,
                    ((crate::c::bf_read((list).wrapping_add(20), 0, 4, false) as u8) as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuChangeSelection(
    list: *mut u8,
    updateCursorAndCallCallback: u8,
    count: u8,
    movingDown: u8,
) -> u8 {
    unsafe {
        let mut list = list;
        let mut updateCursorAndCallCallback = updateCursorAndCallCallback;
        let mut count = count;
        let mut movingDown = movingDown;
        let mut oldSelectedRow: u16 = 0u16;
        let mut selectionChange: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut cursorCount: u8 = 0u8;
        oldSelectedRow = ((list).wrapping_add(26).cast::<u16>()).read();
        cursorCount = 0u8;
        selectionChange = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            let mut ret: u8 =
                                ListMenuUpdateSelectedRowIndexAndScrollOffset(list, movingDown);
                            selectionChange = ((((selectionChange) as i32) | ((ret) as i32)) as u8);
                            if ((ret) as i32) != 2i32 {
                                break 'l3;
                            }
                            cursorCount = (cursorCount).wrapping_add(1);
                        }
                        if !((((((list).cast::<*mut u8>()).read()).wrapping_offset(
                            (((((list).wrapping_add(24).cast::<u16>()).read()) as i32)
                                .wrapping_add(
                                    ((((list).wrapping_add(26).cast::<u16>()).read()) as i32),
                                )) as isize
                                * 8,
                        ))
                        .wrapping_add(4)
                        .cast::<i32>())
                        .read()
                            == (-3i32))
                        {
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (updateCursorAndCallCallback) != 0 {
            'l5: {
                let __sw1 = ((selectionChange) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                if __sw1 == 0i32 || !__matched {
                    return 1u8;
                }
                if __sw1 == 1i32 {
                    ListMenuErasePrintedCursor(list, oldSelectedRow);
                    ListMenuDrawCursor(list);
                    ListMenuCallSelectionChangedCallback(list, 0u8);
                    CopyWindowToVram(((list).wrapping_add(16)).read(), 2u8);
                    break 'l5;
                }
                if __sw1 == 2i32 || __sw1 == 3i32 {
                    ListMenuErasePrintedCursor(list, oldSelectedRow);
                    ListMenuScroll(list, cursorCount, movingDown);
                    ListMenuDrawCursor(list);
                    ListMenuCallSelectionChangedCallback(list, 0u8);
                    CopyWindowToVram(((list).wrapping_add(16)).read(), 2u8);
                    break 'l5;
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ListMenuCallSelectionChangedCallback(list: *mut u8, onInit: u8) {
    unsafe {
        let mut list = list;
        let mut onInit = onInit;
        if core::mem::transmute::<_, usize>(
            ((list)
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn(i32, u8, *mut u8)>>())
            .read(),
        ) != 0usize
        {
            (((list)
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn(i32, u8, *mut u8)>>())
            .read())
            .unwrap_unchecked()(
                (((((list).cast::<*mut u8>()).read()).wrapping_offset(
                    (((((list).wrapping_add(24).cast::<u16>()).read()) as i32)
                        .wrapping_add(((((list).wrapping_add(26).cast::<u16>()).read()) as i32)))
                        as isize
                        * 8,
                ))
                .wrapping_add(4)
                .cast::<i32>())
                .read(),
                onInit,
                list,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuOverrideSetColors(
    cursorPal: u8,
    fillValue: u8,
    cursorShadowPal: u8,
) {
    unsafe {
        let mut cursorPal = cursorPal;
        let mut fillValue = fillValue;
        let mut cursorShadowPal = cursorShadowPal;
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(0),
            0,
            4,
            (cursorPal) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(0),
            4,
            4,
            (fillValue) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(1),
            0,
            4,
            (cursorShadowPal) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gListMenuOverride).cast::<u8>()).wrapping_add(4),
            7,
            1,
            (1u8) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuDefaultCursorMoveFunc(itemIndex: i32, onInit: u8, list: *mut u8) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut onInit = onInit;
        let mut list = list;
        if !((onInit) != 0) {
            PlaySE(5u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetTemplateField(taskId: u8, field: u8) -> i32 {
    unsafe {
        let mut taskId = taskId;
        let mut field = field;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        'l1: {
            let __sw1 = ((field) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32;
            if __sw1 == 0i32 || __sw1 == 1i32 {
                return (core::mem::transmute::<_, usize>(
                    ((data)
                        .wrapping_add(4)
                        .cast::<Option<unsafe extern "C" fn(i32, u8, *mut u8)>>())
                    .read(),
                ) as i32);
            }
            if __sw1 == 2i32 {
                return ((((data).wrapping_add(12).cast::<u16>()).read()) as i32);
            }
            if __sw1 == 3i32 {
                return ((((data).wrapping_add(14).cast::<u16>()).read()) as i32);
            }
            if __sw1 == 4i32 {
                return ((((data).wrapping_add(16)).read()) as i32);
            }
            if __sw1 == 5i32 {
                return ((((data).wrapping_add(17)).read()) as i32);
            }
            if __sw1 == 6i32 {
                return ((((data).wrapping_add(18)).read()) as i32);
            }
            if __sw1 == 7i32 {
                return ((((data).wrapping_add(19)).read()) as i32);
            }
            if __sw1 == 8i32 {
                return ((crate::c::bf_read((data).wrapping_add(20), 0, 4, false) as u8) as i32);
            }
            if __sw1 == 9i32 {
                return ((crate::c::bf_read((data).wrapping_add(20), 4, 4, false) as u8) as i32);
            }
            if __sw1 == 10i32 {
                return ((crate::c::bf_read((data).wrapping_add(21), 0, 4, false) as u8) as i32);
            }
            if __sw1 == 11i32 {
                return ((crate::c::bf_read((data).wrapping_add(21), 4, 4, false) as u8) as i32);
            }
            if __sw1 == 12i32 {
                return ((crate::c::bf_read((data).wrapping_add(22), 0, 3, false) as u8) as i32);
            }
            if __sw1 == 13i32 {
                return ((crate::c::bf_read((data).wrapping_add(22), 3, 3, false) as u8) as i32);
            }
            if __sw1 == 14i32 {
                return ((crate::c::bf_read((data).wrapping_add(22), 6, 2, false) as u8) as i32);
            }
            if __sw1 == 15i32 {
                return ((crate::c::bf_read((data).wrapping_add(23), 0, 6, false) as u8) as i32);
            }
            if __sw1 == 16i32 {
                return ((crate::c::bf_read((data).wrapping_add(23), 6, 2, false) as u8) as i32);
            }
            if !__matched {
                return (-1i32);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuSetTemplateField(taskId: u8, field: u8, value: i32) {
    unsafe {
        let mut taskId = taskId;
        let mut field = field;
        let mut value = value;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        'l1: {
            let __sw1 = ((field) as i32);
            if __sw1 == 0i32 || __sw1 == 1i32 {
                ((data)
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(i32, u8, *mut u8)>>())
                .write(core::mem::transmute::<
                    _,
                    Option<unsafe extern "C" fn(i32, u8, *mut u8)>,
                >(((value) as usize as *mut u8)));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((data).wrapping_add(12).cast::<u16>()).write(((value) as u16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((data).wrapping_add(14).cast::<u16>()).write(((value) as u16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((data).wrapping_add(16)).write(((value) as u8));
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((data).wrapping_add(17)).write(((value) as u8));
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((data).wrapping_add(18)).write(((value) as u8));
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((data).wrapping_add(19)).write(((value) as u8));
                break 'l1;
            }
            if __sw1 == 8i32 {
                crate::c::bf_write((data).wrapping_add(20), 0, 4, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 9i32 {
                crate::c::bf_write((data).wrapping_add(20), 4, 4, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 10i32 {
                crate::c::bf_write((data).wrapping_add(21), 0, 4, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 11i32 {
                crate::c::bf_write((data).wrapping_add(21), 4, 4, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 12i32 {
                crate::c::bf_write((data).wrapping_add(22), 0, 3, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 13i32 {
                crate::c::bf_write((data).wrapping_add(22), 3, 3, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 14i32 {
                crate::c::bf_write((data).wrapping_add(22), 6, 2, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 15i32 {
                crate::c::bf_write((data).wrapping_add(23), 0, 6, ((value) as u8) as i32);
                break 'l1;
            }
            if __sw1 == 16i32 {
                crate::c::bf_write((data).wrapping_add(23), 6, 2, ((value) as u8) as i32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCallback_ScrollIndicatorArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut multiplier: i32 = 0i32;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                StartSpriteAnim(
                    sprite,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u8),
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: {
                    let __sw3 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                        .read()) as i32);
                    if __sw3 == 0i32 {
                        multiplier = ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32);
                        ((sprite).wrapping_add(36).cast::<i16>()).write(
                            ((crate::c::div_i32(
                                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                    .wrapping_offset(
                                        (((((((sprite).wrapping_add(46)).cast::<i16>())
                                            .wrapping_offset(5))
                                        .read()) as u8)
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32)
                                    .wrapping_mul(multiplier),
                                256i32,
                            )) as i16),
                        );
                        break 'l2;
                    }
                    if __sw3 == 1i32 {
                        multiplier = ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32);
                        ((sprite).wrapping_add(38).cast::<i16>()).write(
                            ((crate::c::div_i32(
                                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                    .wrapping_offset(
                                        (((((((sprite).wrapping_add(46)).cast::<i16>())
                                            .wrapping_offset(5))
                                        .read()) as u8)
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32)
                                    .wrapping_mul(multiplier),
                                256i32,
                            )) as i16),
                        );
                        break 'l2;
                    }
                }
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddScrollIndicatorArrowObject(
    arrowDir: u8,
    x: u8,
    y: u8,
    tileTag: u16,
    palTag: u16,
) -> u8 {
    unsafe {
        let mut arrowDir = arrowDir;
        let mut x = x;
        let mut y = y;
        let mut tileTag = tileTag;
        let mut palTag = palTag;
        let mut spriteId: u8 = 0u8;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_ScrollArrowIndicator)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(tileTag);
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(palTag);
        spriteId = CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            ((x) as i16),
            ((y) as i16),
            0u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((crate::c::bf_read(
                ((((&raw const sScrollIndicatorTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((arrowDir) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                4,
                false,
            ) as u8) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((crate::c::bf_read(
                ((((&raw const sScrollIndicatorTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((arrowDir) as i32) as isize * 4))
                .wrapping_add(0),
                4,
                4,
                false,
            ) as u8) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            (((((((&raw const sScrollIndicatorTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((arrowDir) as i32) as isize * 4))
            .wrapping_add(1))
            .read()) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            (((((((&raw const sScrollIndicatorTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((arrowDir) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddScrollIndicatorArrowPair(
    arrowInfo: *mut u8,
    scrollOffset: *mut u16,
) -> u8 {
    unsafe {
        let mut arrowInfo = arrowInfo;
        let mut scrollOffset = scrollOffset;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spritePal = crate::ffi::Align4([0u8; 8]);
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut taskId: u8 = 0u8;
        (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u32>()).write(
            ((&raw const sScrollIndicator_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
        );
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(256u16);
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(((arrowInfo).wrapping_add(10).cast::<u16>()).read());
        LoadCompressedSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        if ((((arrowInfo).wrapping_add(12).cast::<u16>()).read()) as i32) == 65535i32 {
            LoadPalette(
                (((&raw const sRedInterface_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                (((256i32).wrapping_add(
                    ((((arrowInfo).wrapping_add(14)).read()) as i32).wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
        } else {
            (((&raw mut spritePal).cast::<u8>()).cast::<*mut u16>()).write(
                ((&raw const sRedInterface_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
            (((&raw mut spritePal).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(((arrowInfo).wrapping_add(12).cast::<u16>()).read());
            LoadSpritePalette((&raw mut spritePal).cast::<u8>());
        }
        taskId = CreateTask(Some(Task_ScrollIndicatorArrowPair), 0u8);
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        (data).write(0u8);
        ((data).wrapping_add(4).cast::<*mut u16>()).write(scrollOffset);
        ((data).wrapping_add(8).cast::<u16>())
            .write(((arrowInfo).wrapping_add(6).cast::<u16>()).read());
        ((data).wrapping_add(10).cast::<u16>())
            .write(((arrowInfo).wrapping_add(8).cast::<u16>()).read());
        ((data).wrapping_add(14).cast::<u16>())
            .write(((arrowInfo).wrapping_add(10).cast::<u16>()).read());
        ((data).wrapping_add(16).cast::<u16>())
            .write(((arrowInfo).wrapping_add(12).cast::<u16>()).read());
        ((data).wrapping_add(12)).write(AddScrollIndicatorArrowObject(
            (arrowInfo).read(),
            ((arrowInfo).wrapping_add(1)).read(),
            ((arrowInfo).wrapping_add(2)).read(),
            ((arrowInfo).wrapping_add(10).cast::<u16>()).read(),
            ((arrowInfo).wrapping_add(12).cast::<u16>()).read(),
        ));
        ((data).wrapping_add(13)).write(AddScrollIndicatorArrowObject(
            ((arrowInfo).wrapping_add(3)).read(),
            ((arrowInfo).wrapping_add(4)).read(),
            ((arrowInfo).wrapping_add(5)).read(),
            ((arrowInfo).wrapping_add(10).cast::<u16>()).read(),
            ((arrowInfo).wrapping_add(12).cast::<u16>()).read(),
        ));
        if ((((arrowInfo).wrapping_add(12).cast::<u16>()).read()) as i32) == 65535i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                ((((arrowInfo).wrapping_add(14)).read()) as u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(13)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                ((((arrowInfo).wrapping_add(14)).read()) as u16) as i32,
            );
        }
        return taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddScrollIndicatorArrowPairParameterized(
    arrowType: u32,
    commonPos: i32,
    firstPos: i32,
    secondPos: i32,
    fullyDownThreshold: i32,
    tileTag: i32,
    palTag: i32,
    scrollOffset: *mut u16,
) -> u8 {
    unsafe {
        let mut arrowType = arrowType;
        let mut commonPos = commonPos;
        let mut firstPos = firstPos;
        let mut secondPos = secondPos;
        let mut fullyDownThreshold = fullyDownThreshold;
        let mut tileTag = tileTag;
        let mut palTag = palTag;
        let mut scrollOffset = scrollOffset;
        if (arrowType == 2u32) || (arrowType == 3u32) {
            ((&raw mut gTempScrollArrowTemplate).cast::<u8>()).write(2u8);
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(1))
                .write(((commonPos) as u8));
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(2))
                .write(((firstPos) as u8));
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(3)).write(3u8);
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(4))
                .write(((commonPos) as u8));
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(5))
                .write(((secondPos) as u8));
        } else {
            ((&raw mut gTempScrollArrowTemplate).cast::<u8>()).write(0u8);
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(1))
                .write(((firstPos) as u8));
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(2))
                .write(((commonPos) as u8));
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(3)).write(1u8);
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(4))
                .write(((secondPos) as u8));
            (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(5))
                .write(((commonPos) as u8));
        }
        (((&raw mut gTempScrollArrowTemplate).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut gTempScrollArrowTemplate).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>())
        .write(((fullyDownThreshold) as u16));
        (((&raw mut gTempScrollArrowTemplate).cast::<u8>())
            .wrapping_add(10)
            .cast::<u16>())
        .write(((tileTag) as u16));
        (((&raw mut gTempScrollArrowTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(((palTag) as u16));
        (((&raw mut gTempScrollArrowTemplate).cast::<u8>()).wrapping_add(14)).write(0u8);
        return AddScrollIndicatorArrowPair(
            (&raw mut gTempScrollArrowTemplate).cast::<u8>(),
            scrollOffset,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ScrollIndicatorArrowPair(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        let mut currItem: u16 = (((data).wrapping_add(4).cast::<*mut u16>()).read()).read();
        if (((currItem) as i32) == ((((data).wrapping_add(8).cast::<u16>()).read()) as i32))
            && (((currItem) as i32) != 65535i32)
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        if ((currItem) as i32) == ((((data).wrapping_add(10).cast::<u16>()).read()) as i32) {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(13)).read()) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(13)).read()) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ScrollIndicatorArrowPairOnMainMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollData: *mut u8 = (data).cast::<u8>();
        if (((data).wrapping_offset(15)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((scrollData).wrapping_add(12)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((scrollData).wrapping_add(13)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((scrollData).wrapping_add(12)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((scrollData).wrapping_add(13)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveScrollIndicatorArrowPair(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        if ((((data).wrapping_add(14).cast::<u16>()).read()) as i32) != 65535i32 {
            FreeSpriteTilesByTag(((data).wrapping_add(14).cast::<u16>()).read());
        }
        if ((((data).wrapping_add(16).cast::<u16>()).read()) as i32) != 65535i32 {
            FreeSpritePaletteByTag(((data).wrapping_add(16).cast::<u16>()).read());
        }
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(13)).read()) as i32) as isize * 68),
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ListMenuAddCursorObjectInternal(
    cursor: *mut u8,
    cursorObjId: u32,
) -> u8 {
    unsafe {
        let mut cursor = cursor;
        let mut cursorObjId = cursorObjId;
        'l1: {
            let __sw1 = cursorObjId;
            let __matched = __sw1 == 0u32 || __sw1 == 1u32;
            if __sw1 == 0u32 || !__matched {
                return ListMenuAddRedOutlineCursorObject(cursor);
            }
            if __sw1 == 1u32 {
                return ListMenuAddRedArrowCursorObject(cursor);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuUpdateCursorObject(
    taskId: u8,
    x: u16,
    y: u16,
    cursorObjId: u32,
) {
    unsafe {
        let mut taskId = taskId;
        let mut x = x;
        let mut y = y;
        let mut cursorObjId = cursorObjId;
        'l1: {
            let __sw1 = cursorObjId;
            if __sw1 == 0u32 {
                ListMenuUpdateRedOutlineCursorObject(taskId, x, y);
                break 'l1;
            }
            if __sw1 == 1u32 {
                ListMenuUpdateRedArrowCursorObject(taskId, x, y);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuRemoveCursorObject(taskId: u8, cursorObjId: u32) {
    unsafe {
        let mut taskId = taskId;
        let mut cursorObjId = cursorObjId;
        'l1: {
            let __sw1 = cursorObjId;
            if __sw1 == 0u32 {
                ListMenuRemoveRedOutlineCursorObject(taskId);
                break 'l1;
            }
            if __sw1 == 1u32 {
                ListMenuRemoveRedArrowCursorObject(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RedOutlineCursor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetRedOutlineCursorSpriteCount(
    rowWidth: u16,
    rowHeight: u16,
) -> u8 {
    unsafe {
        let mut rowWidth = rowWidth;
        let mut rowHeight = rowHeight;
        let mut i: i32 = 0i32;
        let mut count: i32 = 4i32;
        if ((rowWidth) as i32) > 16i32 {
            {
                i = 8i32;
                'l1: loop {
                    if !(i < ((rowWidth) as i32).wrapping_sub(8i32)) {
                        break 'l1;
                    }
                    'l2: {
                        count = (count).wrapping_add(2i32);
                    }
                    i = (i).wrapping_add(8i32);
                }
            }
        }
        if ((rowHeight) as i32) > 16i32 {
            {
                i = 8i32;
                'l3: loop {
                    if !(i < ((rowHeight) as i32).wrapping_sub(8i32)) {
                        break 'l3;
                    }
                    'l4: {
                        count = (count).wrapping_add(2i32);
                    }
                    i = (i).wrapping_add(8i32);
                }
            }
        }
        return ((count) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuSetUpRedOutlineCursorSpriteOamTable(
    rowWidth: u16,
    rowHeight: u16,
    subsprites: *mut u8,
) {
    unsafe {
        let mut rowWidth = rowWidth;
        let mut rowHeight = rowHeight;
        let mut subsprites = subsprites;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut id: i32 = 0i32;
        (subsprites)
            .wrapping_offset((id) as isize * 4)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw const sSubsprite_RedOutline1)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>()).write((-120i8));
        (((subsprites).wrapping_offset((id) as isize * 4))
            .wrapping_add(1)
            .cast::<i8>())
        .write((-120i8));
        id = (id).wrapping_add(1);
        (subsprites)
            .wrapping_offset((id) as isize * 4)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw const sSubsprite_RedOutline2)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>())
            .write(((((rowWidth) as i32).wrapping_add(128i32)) as i8));
        (((subsprites).wrapping_offset((id) as isize * 4))
            .wrapping_add(1)
            .cast::<i8>())
        .write((-120i8));
        id = (id).wrapping_add(1);
        (subsprites)
            .wrapping_offset((id) as isize * 4)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw const sSubsprite_RedOutline7)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>()).write((-120i8));
        (((subsprites).wrapping_offset((id) as isize * 4))
            .wrapping_add(1)
            .cast::<i8>())
        .write(((((rowHeight) as i32).wrapping_add(128i32)) as i8));
        id = (id).wrapping_add(1);
        (subsprites)
            .wrapping_offset((id) as isize * 4)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw const sSubsprite_RedOutline8)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>())
            .write(((((rowWidth) as i32).wrapping_add(128i32)) as i8));
        (((subsprites).wrapping_offset((id) as isize * 4))
            .wrapping_add(1)
            .cast::<i8>())
        .write(((((rowHeight) as i32).wrapping_add(128i32)) as i8));
        id = (id).wrapping_add(1);
        if ((rowWidth) as i32) > 16i32 {
            {
                i = 8i32;
                'l1: loop {
                    if !(i < ((rowWidth) as i32).wrapping_sub(8i32)) {
                        break 'l1;
                    }
                    'l2: {
                        (subsprites)
                            .wrapping_offset((id) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                (&raw const sSubsprite_RedOutline3)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                            );
                        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>())
                            .write((((i).wrapping_sub(120i32)) as i8));
                        (((subsprites).wrapping_offset((id) as isize * 4))
                            .wrapping_add(1)
                            .cast::<i8>())
                        .write((-120i8));
                        id = (id).wrapping_add(1);
                        (subsprites)
                            .wrapping_offset((id) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                (&raw const sSubsprite_RedOutline6)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                            );
                        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>())
                            .write((((i).wrapping_sub(120i32)) as i8));
                        (((subsprites).wrapping_offset((id) as isize * 4))
                            .wrapping_add(1)
                            .cast::<i8>())
                        .write(((((rowHeight) as i32).wrapping_add(128i32)) as i8));
                        id = (id).wrapping_add(1);
                    }
                    i = (i).wrapping_add(8i32);
                }
            }
        }
        if ((rowHeight) as i32) > 16i32 {
            {
                j = 8i32;
                'l3: loop {
                    if !(j < ((rowHeight) as i32).wrapping_sub(8i32)) {
                        break 'l3;
                    }
                    'l4: {
                        (subsprites)
                            .wrapping_offset((id) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                (&raw const sSubsprite_RedOutline4)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                            );
                        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>())
                            .write((-120i8));
                        (((subsprites).wrapping_offset((id) as isize * 4))
                            .wrapping_add(1)
                            .cast::<i8>())
                        .write((((j).wrapping_sub(120i32)) as i8));
                        id = (id).wrapping_add(1);
                        (subsprites)
                            .wrapping_offset((id) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                (&raw const sSubsprite_RedOutline5)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                            );
                        (((subsprites).wrapping_offset((id) as isize * 4)).cast::<i8>())
                            .write(((((rowWidth) as i32).wrapping_add(128i32)) as i8));
                        (((subsprites).wrapping_offset((id) as isize * 4))
                            .wrapping_add(1)
                            .cast::<i8>())
                        .write((((j).wrapping_sub(120i32)) as i8));
                        id = (id).wrapping_add(1);
                    }
                    j = (j).wrapping_add(8i32);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuAddRedOutlineCursorObject(cursor: *mut u8) -> u8 {
    unsafe {
        let mut cursor = cursor;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spritePal = crate::ffi::Align4([0u8; 8]);
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut taskId: u8 = 0u8;
        (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u32>()).write(
            ((&raw const sOutlineCursor_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
        );
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(256u16);
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(((cursor).wrapping_add(6).cast::<u16>()).read());
        LoadCompressedSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        if ((((cursor).wrapping_add(8).cast::<u16>()).read()) as i32) == 65535i32 {
            LoadPalette(
                (((&raw const sRedInterface_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                (((256i32).wrapping_add(
                    ((((cursor).wrapping_add(10)).read()) as i32).wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
        } else {
            (((&raw mut spritePal).cast::<u8>()).cast::<*mut u16>()).write(
                ((&raw const sRedInterface_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
            (((&raw mut spritePal).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(((cursor).wrapping_add(8).cast::<u16>()).read());
            LoadSpritePalette((&raw mut spritePal).cast::<u8>());
        }
        taskId = CreateTask(Some(Task_RedOutlineCursor), 0u8);
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        ((data).wrapping_add(14).cast::<u16>())
            .write(((cursor).wrapping_add(6).cast::<u16>()).read());
        ((data).wrapping_add(16).cast::<u16>())
            .write(((cursor).wrapping_add(8).cast::<u16>()).read());
        (data).write(ListMenuGetRedOutlineCursorSpriteCount(
            ((cursor).wrapping_add(2).cast::<u16>()).read(),
            ((cursor).wrapping_add(4).cast::<u16>()).read(),
        ));
        ((data).wrapping_add(4).cast::<*mut u8>()).write({
            let __v1 = Alloc((((((data).read()) as i32).wrapping_mul(4i32)) as u32));
            ((data).wrapping_add(8).cast::<*mut u8>()).write(__v1);
            __v1
        });
        ListMenuSetUpRedOutlineCursorSpriteOamTable(
            ((cursor).wrapping_add(2).cast::<u16>()).read(),
            ((cursor).wrapping_add(4).cast::<u16>()).read(),
            ((data).wrapping_add(8).cast::<*mut u8>()).read(),
        );
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw mut gDummySpriteTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>())
            .write(((cursor).wrapping_add(6).cast::<u16>()).read());
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(((cursor).wrapping_add(8).cast::<u16>()).read());
        ((data).wrapping_add(12)).write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            (((((cursor).read()) as i32).wrapping_add(120i32)) as i16),
            ((((((cursor).wrapping_add(1)).read()) as i32).wrapping_add(120i32)) as i16),
            0u8,
        ));
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68),
            (data),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
        .wrapping_add(67))
        .write(0u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
            .wrapping_add(66),
            0,
            6,
            (0u8) as i32,
        );
        if ((((cursor).wrapping_add(8).cast::<u16>()).read()) as i32) == 65535i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                ((((cursor).wrapping_add(10)).read()) as u16) as i32,
            );
        }
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn ListMenuUpdateRedOutlineCursorObject(taskId: u8, x: u16, y: u16) {
    unsafe {
        let mut taskId = taskId;
        let mut x = x;
        let mut y = y;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .write(((((x) as i32).wrapping_add(120i32)) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .write(((((y) as i32).wrapping_add(120i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn ListMenuRemoveRedOutlineCursorObject(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        Free(((data).wrapping_add(8).cast::<*mut u8>()).read());
        if ((((data).wrapping_add(14).cast::<u16>()).read()) as i32) != 65535i32 {
            FreeSpriteTilesByTag(((data).wrapping_add(14).cast::<u16>()).read());
        }
        if ((((data).wrapping_add(16).cast::<u16>()).read()) as i32) != 65535i32 {
            FreeSpritePaletteByTag(((data).wrapping_add(16).cast::<u16>()).read());
        }
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(12)).read()) as i32) as isize * 68),
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCallback_RedArrowCursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8) as i32) as isize,
                ))
                .read()) as i32),
                64i32,
            )) as i16),
        );
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_RedArrowCursor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
pub(crate) unsafe extern "C" fn ListMenuAddRedArrowCursorObject(cursor: *mut u8) -> u8 {
    unsafe {
        let mut cursor = cursor;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spritePal = crate::ffi::Align4([0u8; 8]);
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut taskId: u8 = 0u8;
        (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u32>()).write(
            ((&raw const sArrowCursor_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
        );
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(128u16);
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(((cursor).wrapping_add(6).cast::<u16>()).read());
        LoadCompressedSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        if ((((cursor).wrapping_add(8).cast::<u16>()).read()) as i32) == 65535i32 {
            LoadPalette(
                (((&raw const sRedInterface_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                (((256i32).wrapping_add(
                    ((((cursor).wrapping_add(10)).read()) as i32).wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
        } else {
            (((&raw mut spritePal).cast::<u8>()).cast::<*mut u16>()).write(
                ((&raw const sRedInterface_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
            (((&raw mut spritePal).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(((cursor).wrapping_add(8).cast::<u16>()).read());
            LoadSpritePalette((&raw mut spritePal).cast::<u8>());
        }
        taskId = CreateTask(Some(Task_RedArrowCursor), 0u8);
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        ((data).wrapping_add(2).cast::<u16>())
            .write(((cursor).wrapping_add(6).cast::<u16>()).read());
        ((data).wrapping_add(4).cast::<u16>())
            .write(((cursor).wrapping_add(8).cast::<u16>()).read());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_RedArrowCursor)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>())
            .write(((cursor).wrapping_add(6).cast::<u16>()).read());
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(((cursor).wrapping_add(8).cast::<u16>()).read());
        (data).write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            (((cursor).read()) as i16),
            ((((cursor).wrapping_add(1)).read()) as i16),
            0u8,
        ));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(8i16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(8i16);
        if ((((cursor).wrapping_add(8).cast::<u16>()).read()) as i32) == 65535i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((data).read()) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                ((((cursor).wrapping_add(10)).read()) as u16) as i32,
            );
        }
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn ListMenuUpdateRedArrowCursorObject(taskId: u8, x: u16, y: u16) {
    unsafe {
        let mut taskId = taskId;
        let mut x = x;
        let mut y = y;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .write(((x) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .write(((y) as i16));
    }
}
pub(crate) unsafe extern "C" fn ListMenuRemoveRedArrowCursorObject(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        if ((((data).wrapping_add(2).cast::<u16>()).read()) as i32) != 65535i32 {
            FreeSpriteTilesByTag(((data).wrapping_add(2).cast::<u16>()).read());
        }
        if ((((data).wrapping_add(4).cast::<u16>()).read()) as i32) != 65535i32 {
            FreeSpritePaletteByTag(((data).wrapping_add(4).cast::<u16>()).read());
        }
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 68),
        );
        DestroyTask(taskId);
    }
}
