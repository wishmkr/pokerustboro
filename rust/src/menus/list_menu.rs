//! Translated from `src/list_menu.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sScrollIndicatorTemplates sOamData_ScrollArrowIndicator sSpriteAnim_ScrollArrowIndicator0 sSpriteAnim_ScrollArrowIndicator1 sSpriteAnim_ScrollArrowIndicator2 sSpriteAnim_ScrollArrowIndicator3 sSpriteAnimTable_ScrollArrowIndicator sSpriteTemplate_ScrollArrowIndicator sSubsprite_RedOutline1 sSubsprite_RedOutline2 sSubsprite_RedOutline3 sSubsprite_RedOutline4 sSubsprite_RedOutline5 sSubsprite_RedOutline6 sSubsprite_RedOutline7 sSubsprite_RedOutline8 sOamData_RedArrowCursor sSpriteAnim_RedArrowCursor sSpriteAnimTable_RedArrowCursor sSpriteTemplate_RedArrowCursor sRedInterface_Pal sScrollIndicator_Gfx sOutlineCursor_Gfx sArrowCursor_Gfx

/// `__typeof__(sMysteryGiftLinkMenu)`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sMysteryGiftLinkMenu_t {
    pub currItemId: i32,
    pub state: u8,
    pub windowId: u8,
    pub listTaskId: u8,
}

unsafe impl Sync for sMysteryGiftLinkMenu_t {}

/// `__typeof__(gListMenuOverride)`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct gListMenuOverride_t {
    bits_0: u8,
    bits_1: u8,
    bits_2: u8,
    bits_3: u8,
    bits_4: u8,
}

impl gListMenuOverride_t {
    #[inline(always)]
    pub fn cursorPal(&self) -> u8 {
        ((self.bits_0 as u32 >> 0) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_cursorPal(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0xf << 0)) | ((v as u8 & 0xf) << 0);
    }
    #[inline(always)]
    pub fn fillValue(&self) -> u8 {
        ((self.bits_0 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_fillValue(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
    #[inline(always)]
    pub fn cursorShadowPal(&self) -> u8 {
        ((self.bits_1 as u32 >> 0) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_cursorShadowPal(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !(0xf << 0)) | ((v as u8 & 0xf) << 0);
    }
    #[inline(always)]
    pub fn lettersSpacing(&self) -> u8 {
        ((self.bits_2 as u32 >> 0) & 0x3f) as u8
    }
    #[inline(always)]
    pub fn set_lettersSpacing(&mut self, v: u8) {
        self.bits_2 = (self.bits_2 & !(0x3f << 0)) | ((v as u8 & 0x3f) << 0);
    }
    #[inline(always)]
    pub fn field_2_2(&self) -> u8 {
        ((self.bits_3 as u32 >> 0) & 0x3f) as u8
    }
    #[inline(always)]
    pub fn set_field_2_2(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x3f << 0)) | ((v as u8 & 0x3f) << 0);
    }
    #[inline(always)]
    pub fn fontId(&self) -> u8 {
        ((self.bits_4 as u32 >> 0) & 0x7f) as u8
    }
    #[inline(always)]
    pub fn set_fontId(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x7f << 0)) | ((v as u8 & 0x7f) << 0);
    }
    #[inline(always)]
    pub fn enabled(&self) -> u8 {
        ((self.bits_4 as u32 >> 7) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_enabled(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
}

unsafe impl Sync for gListMenuOverride_t {}

/// `struct ScrollIndicatorPair`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScrollIndicatorPair {
    pub field_0: u8,
    pub scrollOffset: *mut u16,
    pub fullyUpThreshold: u16,
    pub fullyDownThreshold: u16,
    pub topSpriteId: u8,
    pub bottomSpriteId: u8,
    pub tileTag: u16,
    pub palTag: u16,
}

unsafe impl Sync for ScrollIndicatorPair {}

/// `struct RedOutlineCursor`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RedOutlineCursor {
    pub subspriteTable: SubspriteTable,
    pub subspritesPtr: *mut Subsprite,
    pub spriteId: u8,
    pub tileTag: u16,
    pub palTag: u16,
}

unsafe impl Sync for RedOutlineCursor {}

/// `struct RedArrowCursor`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RedArrowCursor {
    pub spriteId: u8,
    pub tileTag: u16,
    pub palTag: u16,
}

unsafe impl Sync for RedArrowCursor {}

/// `__typeof__(sScrollIndicatorTemplates[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sScrollIndicatorTemplates_0_t {
    bits_0: u8,
    pub multiplier: u8,
    pub frequency: u16,
}

impl sScrollIndicatorTemplates_0_t {
    #[inline(always)]
    pub fn animNum(&self) -> u8 {
        ((self.bits_0 as u32 >> 0) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_animNum(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0xf << 0)) | ((v as u8 & 0xf) << 0);
    }
    #[inline(always)]
    pub fn bounceDir(&self) -> u8 {
        ((self.bits_0 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_bounceDir(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
}

unsafe impl Sync for sScrollIndicatorTemplates_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sMysteryGiftLinkMenu_t>() == 8);
    assert!(offset_of!(sMysteryGiftLinkMenu_t, currItemId) == 0);
    assert!(offset_of!(sMysteryGiftLinkMenu_t, state) == 4);
    assert!(offset_of!(sMysteryGiftLinkMenu_t, windowId) == 5);
    assert!(offset_of!(sMysteryGiftLinkMenu_t, listTaskId) == 6);
    assert!(size_of::<gListMenuOverride_t>() == 8);
    assert!(offset_of!(gListMenuOverride_t, bits_0) == 0);
    assert!(offset_of!(gListMenuOverride_t, bits_1) == 1);
    assert!(offset_of!(gListMenuOverride_t, bits_2) == 2);
    assert!(offset_of!(gListMenuOverride_t, bits_3) == 3);
    assert!(offset_of!(gListMenuOverride_t, bits_4) == 4);
    assert!(size_of::<ScrollIndicatorPair>() == 20);
    assert!(offset_of!(ScrollIndicatorPair, field_0) == 0);
    assert!(offset_of!(ScrollIndicatorPair, scrollOffset) == 4);
    assert!(offset_of!(ScrollIndicatorPair, fullyUpThreshold) == 8);
    assert!(offset_of!(ScrollIndicatorPair, fullyDownThreshold) == 10);
    assert!(offset_of!(ScrollIndicatorPair, topSpriteId) == 12);
    assert!(offset_of!(ScrollIndicatorPair, bottomSpriteId) == 13);
    assert!(offset_of!(ScrollIndicatorPair, tileTag) == 14);
    assert!(offset_of!(ScrollIndicatorPair, palTag) == 16);
    assert!(size_of::<RedOutlineCursor>() == 20);
    assert!(offset_of!(RedOutlineCursor, subspriteTable) == 0);
    assert!(offset_of!(RedOutlineCursor, subspritesPtr) == 8);
    assert!(offset_of!(RedOutlineCursor, spriteId) == 12);
    assert!(offset_of!(RedOutlineCursor, tileTag) == 14);
    assert!(offset_of!(RedOutlineCursor, palTag) == 16);
    assert!(size_of::<RedArrowCursor>() == 8);
    assert!(offset_of!(RedArrowCursor, spriteId) == 0);
    assert!(offset_of!(RedArrowCursor, tileTag) == 2);
    assert!(offset_of!(RedArrowCursor, palTag) == 4);
    assert!(size_of::<sScrollIndicatorTemplates_0_t>() == 4);
    assert!(offset_of!(sScrollIndicatorTemplates_0_t, bits_0) == 0);
    assert!(offset_of!(sScrollIndicatorTemplates_0_t, multiplier) == 1);
    assert!(offset_of!(sScrollIndicatorTemplates_0_t, frequency) == 2);
};

static sArrowCursor_Gfx: Table<CArray<u32, 17>> =
    Table((&raw const crate::data::list_menu::sArrowCursor_Gfx).cast());
static sOutlineCursor_Gfx: Table<CArray<u32, 16>> =
    Table((&raw const crate::data::list_menu::sOutlineCursor_Gfx).cast());
static sRedInterface_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::list_menu::sRedInterface_Pal).cast());
static sScrollIndicatorTemplates: Table<CArray<sScrollIndicatorTemplates_0_t, 4>> =
    Table((&raw const crate::data::list_menu::sScrollIndicatorTemplates).cast());
static sScrollIndicator_Gfx: Table<CArray<u32, 28>> =
    Table((&raw const crate::data::list_menu::sScrollIndicator_Gfx).cast());
static sSpriteTemplate_RedArrowCursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::list_menu::sSpriteTemplate_RedArrowCursor).cast());
static sSpriteTemplate_ScrollArrowIndicator: Table<SpriteTemplate> =
    Table((&raw const crate::data::list_menu::sSpriteTemplate_ScrollArrowIndicator).cast());
static sSubsprite_RedOutline1: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline1).cast());
static sSubsprite_RedOutline2: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline2).cast());
static sSubsprite_RedOutline3: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline3).cast());
static sSubsprite_RedOutline4: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline4).cast());
static sSubsprite_RedOutline5: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline5).cast());
static sSubsprite_RedOutline6: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline6).cast());
static sSubsprite_RedOutline7: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline7).cast());
static sSubsprite_RedOutline8: Table<Subsprite> =
    Table((&raw const crate::data::list_menu::sSubsprite_RedOutline8).cast());

pub(crate) static mut sMysteryGiftLinkMenu: sMysteryGiftLinkMenu_t = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTempScrollArrowTemplate: ScrollArrowsTemplate = unsafe { zeroed() };
#[unsafe(no_mangle)]
pub static mut gListMenuOverride: gListMenuOverride_t = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMultiuseListMenuTemplate: ListMenuTemplate = unsafe { zeroed() };

unsafe extern "C" {
    static gDummySpriteTemplate: SpriteTemplate;
    static mut gMain: Main;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_SelectorArrow2: CArray<u8, 0>;
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut c_void);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn PlaySE(a0: u16);
    fn PutWindowRectTilemapOverridePalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScrollWindow(a0: u8, a1: u8, a2: u8, a3: u8);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn SetWindowAttribute(a0: u8, a1: u8, a2: u32) -> u8;
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
}

pub(crate) unsafe extern "C" fn ListMenuDummyTask(taskId: u8) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMysteryGiftListMenu(
    windowTemplate: *mut WindowTemplate,
    listMenuTemplate: *mut ListMenuTemplate,
    drawMode: u8,
    tileNum: u16,
    palOffset: u16,
) -> i32 {
    match sMysteryGiftLinkMenu.state {
        1 => {
            sMysteryGiftLinkMenu.currItemId =
                ListMenu_ProcessInput(sMysteryGiftLinkMenu.listTaskId);
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                sMysteryGiftLinkMenu.state = 2;
            }
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                sMysteryGiftLinkMenu.currItemId = LIST_CANCEL;
                sMysteryGiftLinkMenu.state = 2;
            }
            if sMysteryGiftLinkMenu.state == 2 {
                if drawMode == 0 {
                    ClearWindowTilemap(sMysteryGiftLinkMenu.windowId);
                } else {
                    match drawMode {
                        0 => {
                            ClearStdWindowAndFrame(sMysteryGiftLinkMenu.windowId, FALSE);
                        }
                        2 | 1 => {
                            ClearStdWindowAndFrame(sMysteryGiftLinkMenu.windowId, FALSE);
                        }
                        _ => {}
                    }
                }
                CopyWindowToVram(sMysteryGiftLinkMenu.windowId, COPYWIN_MAP);
            }
        }
        2 => {
            DestroyListMenuTask(sMysteryGiftLinkMenu.listTaskId, null_mut(), null_mut());
            RemoveWindow(sMysteryGiftLinkMenu.windowId);
            sMysteryGiftLinkMenu.state = 0;
            return sMysteryGiftLinkMenu.currItemId;
        }
        _ => {
            sMysteryGiftLinkMenu.windowId = AddWindow(windowTemplate) as u8;
            'l2: {
                let sw1: u8 = drawMode;
                let mut fall = false;
                if sw1 == 2 {
                    fall = true;
                    LoadUserWindowBorderGfx(
                        sMysteryGiftLinkMenu.windowId,
                        tileNum,
                        palOffset as u8,
                    );
                }
                if fall || sw1 == 1 {
                    fall = true;
                    DrawTextBorderOuter(
                        sMysteryGiftLinkMenu.windowId,
                        tileNum,
                        (palOffset as i32 / 16) as u8,
                    );
                    break 'l2;
                }
            }
            gMultiuseListMenuTemplate = *listMenuTemplate;
            gMultiuseListMenuTemplate.windowId = sMysteryGiftLinkMenu.windowId;
            sMysteryGiftLinkMenu.listTaskId =
                ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
            CopyWindowToVram(sMysteryGiftLinkMenu.windowId, COPYWIN_MAP);
            sMysteryGiftLinkMenu.state = 1;
        }
    }
    return LIST_NOTHING_CHOSEN;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuInit(
    listMenuTemplate: *mut ListMenuTemplate,
    scrollOffset: u16,
    selectedRow: u16,
) -> u8 {
    let mut taskId: u8 = ListMenuInitInternal(listMenuTemplate, scrollOffset, selectedRow);
    PutWindowTilemap((*listMenuTemplate).windowId);
    CopyWindowToVram((*listMenuTemplate).windowId, COPYWIN_GFX);
    return taskId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuInitInRect(
    listMenuTemplate: *mut ListMenuTemplate,
    rect: *mut ListMenuWindowRect,
    scrollOffset: u16,
    selectedRow: u16,
) -> u8 {
    let mut i: i32 = 0;
    let mut taskId: u8 = ListMenuInitInternal(listMenuTemplate, scrollOffset, selectedRow);
    i = 0;
    while (*rect.at(i)).palNum != 0xFF {
        PutWindowRectTilemapOverridePalette(
            (*listMenuTemplate).windowId,
            (*rect.at(i)).x,
            (*rect.at(i)).y,
            (*rect.at(i)).width,
            (*rect.at(i)).height,
            (*rect.at(i)).palNum,
        );
        i += 1;
    }
    CopyWindowToVram((*listMenuTemplate).windowId, COPYWIN_GFX);
    return taskId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenu_ProcessInput(listTaskId: u8) -> i32 {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        return (*(*list)
            .template
            .items
            .at((*list).scrollOffset as i32 + (*list).selectedRow as i32))
        .id;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return LIST_CANCEL;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        ListMenuChangeSelection(list, 1, 1, FALSE);
        return LIST_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        ListMenuChangeSelection(list, 1, 1, 1);
        return LIST_NOTHING_CHOSEN;
    } else {
        let mut rightButton: u16 = 0;
        let mut leftButton: u16 = 0;
        match (*list).template.scrollMultiple() {
            LIST_MULTIPLE_SCROLL_DPAD => {
                leftButton = gMain.newAndRepeatedKeys & DPAD_LEFT as u16;
                rightButton = gMain.newAndRepeatedKeys & DPAD_RIGHT as u16;
            }
            LIST_MULTIPLE_SCROLL_L_R => {
                leftButton = gMain.newAndRepeatedKeys & L_BUTTON as u16;
                rightButton = gMain.newAndRepeatedKeys & R_BUTTON as u16;
            }
            _ => {
                leftButton = FALSE as u16;
                rightButton = FALSE as u16;
            }
        }
        if leftButton != 0 {
            ListMenuChangeSelection(list, TRUE, (*list).template.maxShowed as u8, FALSE);
            return LIST_NOTHING_CHOSEN;
        } else if rightButton != 0 {
            ListMenuChangeSelection(list, TRUE, (*list).template.maxShowed as u8, TRUE);
            return LIST_NOTHING_CHOSEN;
        } else {
            return LIST_NOTHING_CHOSEN;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyListMenuTask(
    listTaskId: u8,
    scrollOffset: *mut u16,
    selectedRow: *mut u16,
) {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    if !scrollOffset.is_null() {
        *scrollOffset = (*list).scrollOffset;
    }
    if !selectedRow.is_null() {
        *selectedRow = (*list).selectedRow;
    }
    if (*list).taskId != TASK_NONE {
        ListMenuRemoveCursorObject(
            (*list).taskId,
            (*list).template.cursorKind() as u32 - CURSOR_RED_OUTLINE,
        );
    }
    DestroyTask(listTaskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RedrawListMenu(listTaskId: u8) {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    FillWindowPixelBuffer(
        (*list).template.windowId,
        (*list).template.fillValue() | (*list).template.fillValue() << 4,
    );
    ListMenuPrintEntries(list, (*list).scrollOffset, 0, (*list).template.maxShowed);
    ListMenuDrawCursor(list);
    CopyWindowToVram((*list).template.windowId, COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeListMenuPals(
    listTaskId: u8,
    cursorPal: u8,
    fillValue: u8,
    cursorShadowPal: u8,
) {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    (*list).template.set_cursorPal(cursorPal);
    (*list).template.set_fillValue(fillValue);
    (*list).template.set_cursorShadowPal(cursorShadowPal);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeListMenuCoords(listTaskId: u8, x: u8, y: u8) {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    SetWindowAttribute((*list).template.windowId, WINDOW_TILEMAP_LEFT, x as u32);
    SetWindowAttribute((*list).template.windowId, WINDOW_TILEMAP_TOP, y as u32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuTestInput(
    template: *mut ListMenuTemplate,
    scrollOffset: u32,
    selectedRow: u32,
    keys: u16,
    newScrollOffset: *mut u16,
    newSelectedRow: *mut u16,
) -> i32 {
    let mut list: ListMenu = zeroed();
    list.template = *template;
    list.scrollOffset = scrollOffset as u16;
    list.selectedRow = selectedRow as u16;
    list.unk_1C = 0;
    list.unk_1D = 0;
    if keys == DPAD_UP as u16 {
        ListMenuChangeSelection(&raw mut list, FALSE, 1, FALSE);
    }
    if keys == DPAD_DOWN as u16 {
        ListMenuChangeSelection(&raw mut list, FALSE, 1, 1);
    }
    if !newScrollOffset.is_null() {
        *newScrollOffset = list.scrollOffset;
    }
    if !newSelectedRow.is_null() {
        *newSelectedRow = list.selectedRow;
    }
    return LIST_NOTHING_CHOSEN;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetCurrentItemArrayId(listTaskId: u8, arrayId: *mut u16) {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    if !arrayId.is_null() {
        *arrayId = (*list).scrollOffset + (*list).selectedRow;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetScrollAndRow(
    listTaskId: u8,
    scrollOffset: *mut u16,
    selectedRow: *mut u16,
) {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    if !scrollOffset.is_null() {
        *scrollOffset = (*list).scrollOffset;
    }
    if !selectedRow.is_null() {
        *selectedRow = (*list).selectedRow;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetYCoordForPrintingArrowCursor(listTaskId: u8) -> u16 {
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    let mut yMultiplier: u8 =
        GetFontAttribute((*list).template.fontId(), FONTATTR_MAX_LETTER_HEIGHT)
            + (*list).template.itemVerticalPadding();
    return (*list).selectedRow * yMultiplier as u16 + (*list).template.upText_Y() as u16;
}
pub(crate) unsafe extern "C" fn ListMenuInitInternal(
    listMenuTemplate: *mut ListMenuTemplate,
    scrollOffset: u16,
    selectedRow: u16,
) -> u8 {
    let mut listTaskId: u8 = CreateTask(Some(ListMenuDummyTask), 0);
    let mut list: *mut ListMenu =
        gTasks[listTaskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    (*list).template = *listMenuTemplate;
    (*list).scrollOffset = scrollOffset;
    (*list).selectedRow = selectedRow;
    (*list).unk_1C = 0;
    (*list).unk_1D = 0;
    (*list).taskId = TASK_NONE;
    (*list).unk_1F = 0;
    gListMenuOverride.set_cursorPal((*list).template.cursorPal());
    gListMenuOverride.set_fillValue((*list).template.fillValue());
    gListMenuOverride.set_cursorShadowPal((*list).template.cursorShadowPal());
    gListMenuOverride.set_lettersSpacing((*list).template.lettersSpacing());
    gListMenuOverride.set_fontId((*list).template.fontId());
    gListMenuOverride.set_enabled(FALSE);
    if (*list).template.totalItems < (*list).template.maxShowed {
        (*list).template.maxShowed = (*list).template.totalItems;
    }
    FillWindowPixelBuffer(
        (*list).template.windowId,
        (*list).template.fillValue() | (*list).template.fillValue() << 4,
    );
    ListMenuPrintEntries(list, (*list).scrollOffset, 0, (*list).template.maxShowed);
    ListMenuDrawCursor(list);
    ListMenuCallSelectionChangedCallback(list, TRUE);
    return listTaskId;
}
pub(crate) unsafe extern "C" fn ListMenuPrint(list: *mut ListMenu, str: *mut u8, x: u8, y: u8) {
    let mut colors: CArray<u8, 3> = zeroed();
    if gListMenuOverride.enabled() != 0 {
        colors[0] = gListMenuOverride.fillValue();
        colors[1] = gListMenuOverride.cursorPal();
        colors[2] = gListMenuOverride.cursorShadowPal();
        AddTextPrinterParameterized4(
            (*list).template.windowId,
            gListMenuOverride.fontId(),
            x,
            y,
            gListMenuOverride.lettersSpacing(),
            0,
            colors.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            str,
        );
        gListMenuOverride.set_enabled(FALSE);
    } else {
        colors[0] = (*list).template.fillValue();
        colors[1] = (*list).template.cursorPal();
        colors[2] = (*list).template.cursorShadowPal();
        AddTextPrinterParameterized4(
            (*list).template.windowId,
            (*list).template.fontId(),
            x,
            y,
            (*list).template.lettersSpacing(),
            0,
            colors.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn ListMenuPrintEntries(
    list: *mut ListMenu,
    mut startIndex: u16,
    yOffset: u16,
    count: u16,
) {
    let mut i: i32 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut yMultiplier: u8 =
        GetFontAttribute((*list).template.fontId(), FONTATTR_MAX_LETTER_HEIGHT)
            + (*list).template.itemVerticalPadding();
    i = 0;
    while i < count as i32 {
        if (*(*list).template.items.at(startIndex)).id != LIST_HEADER {
            x = (*list).template.item_X;
        } else {
            x = (*list).template.header_X;
        }
        y = (yOffset as u8 + i as u8) * yMultiplier + (*list).template.upText_Y();
        if (*list).template.itemPrintFunc.is_some() {
            (*list).template.itemPrintFunc.unwrap_unchecked()(
                (*list).template.windowId,
                (*(*list).template.items.at(startIndex)).id as u32,
                y,
            );
        }
        ListMenuPrint(list, (*(*list).template.items.at(startIndex)).name, x, y);
        startIndex += 1;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ListMenuDrawCursor(list: *mut ListMenu) {
    let mut yMultiplier: u8 =
        GetFontAttribute((*list).template.fontId(), FONTATTR_MAX_LETTER_HEIGHT)
            + (*list).template.itemVerticalPadding();
    let mut x: u8 = (*list).template.cursor_X;
    let mut y: u8 = (*list).selectedRow as u8 * yMultiplier + (*list).template.upText_Y();
    match (*list).template.cursorKind() {
        CURSOR_BLACK_ARROW => {
            ListMenuPrint(list, gText_SelectorArrow2.as_ptr().cast_mut(), x, y);
        }
        1 => {}
        2 => {
            if (*list).taskId == TASK_NONE {
                (*list).taskId = ListMenuAddCursorObject(list, 0);
            }
            ListMenuUpdateCursorObject(
                (*list).taskId,
                GetWindowAttribute((*list).template.windowId, WINDOW_TILEMAP_LEFT) as u16 * 8 - 1,
                GetWindowAttribute((*list).template.windowId, WINDOW_TILEMAP_TOP) as u16 * 8
                    + y as u16
                    - 1,
                0,
            );
        }
        CURSOR_RED_ARROW => {
            if (*list).taskId == TASK_NONE {
                (*list).taskId = ListMenuAddCursorObject(list, 1);
            }
            ListMenuUpdateCursorObject(
                (*list).taskId,
                GetWindowAttribute((*list).template.windowId, WINDOW_TILEMAP_LEFT) as u16 * 8
                    + x as u16,
                GetWindowAttribute((*list).template.windowId, WINDOW_TILEMAP_TOP) as u16 * 8
                    + y as u16,
                1,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ListMenuAddCursorObject(
    list: *mut ListMenu,
    cursorObjId: u32,
) -> u8 {
    let mut cursor: CursorStruct = zeroed();
    cursor.left = 0;
    cursor.top = DISPLAY_HEIGHT as u8;
    cursor.rowWidth = GetWindowAttribute((*list).template.windowId, WINDOW_WIDTH) as u16 * 8 + 2;
    cursor.rowHeight =
        GetFontAttribute((*list).template.fontId(), FONTATTR_MAX_LETTER_HEIGHT) as u16 + 2;
    cursor.tileTag = 0x4000;
    cursor.palTag = TAG_NONE;
    cursor.palNum = 15;
    return ListMenuAddCursorObjectInternal(&raw mut cursor, cursorObjId);
}
pub(crate) unsafe extern "C" fn ListMenuErasePrintedCursor(list: *mut ListMenu, selectedRow: u16) {
    let mut cursorKind: u8 = (*list).template.cursorKind();
    if cursorKind == CURSOR_BLACK_ARROW {
        let mut yMultiplier: u8 =
            GetFontAttribute((*list).template.fontId(), FONTATTR_MAX_LETTER_HEIGHT)
                + (*list).template.itemVerticalPadding();
        let mut width: u8 = GetMenuCursorDimensionByFont((*list).template.fontId(), 0);
        let mut height: u8 = GetMenuCursorDimensionByFont((*list).template.fontId(), 1);
        FillWindowPixelRect(
            (*list).template.windowId,
            (*list).template.fillValue() | (*list).template.fillValue() << 4,
            (*list).template.cursor_X as u16,
            selectedRow * yMultiplier as u16 + (*list).template.upText_Y() as u16,
            width as u16,
            height as u16,
        );
    }
}
pub(crate) unsafe extern "C" fn ListMenuUpdateSelectedRowIndexAndScrollOffset(
    list: *mut ListMenu,
    movingDown: u8,
) -> u8 {
    let mut selectedRow: u16 = (*list).selectedRow;
    let mut scrollOffset: u16 = (*list).scrollOffset;
    let mut newRow: u16 = 0;
    let mut newScroll: u32 = 0;
    if movingDown == 0 {
        if (*list).template.maxShowed == 1 {
            newRow = 0;
        } else {
            newRow = (*list).template.maxShowed
                - (((*list).template.maxShowed as i32 / 2) as u16
                    + ((*list).template.maxShowed as i32 % 2) as u16)
                - 1;
        }
        if scrollOffset == 0 {
            while selectedRow != 0 {
                selectedRow -= 1;
                if (*(*list)
                    .template
                    .items
                    .at(scrollOffset as i32 + selectedRow as i32))
                .id != LIST_HEADER
                {
                    (*list).selectedRow = selectedRow;
                    return 1;
                }
            }
            return 0;
        } else {
            while selectedRow > newRow {
                selectedRow -= 1;
                if (*(*list)
                    .template
                    .items
                    .at(scrollOffset as i32 + selectedRow as i32))
                .id != LIST_HEADER
                {
                    (*list).selectedRow = selectedRow;
                    return 1;
                }
            }
            newScroll = scrollOffset as u32 - 1;
        }
    } else {
        if (*list).template.maxShowed == 1 {
            newRow = 0;
        } else {
            newRow = ((*list).template.maxShowed as i32 / 2) as u16
                + ((*list).template.maxShowed as i32 % 2) as u16;
        }
        if scrollOffset as i32
            == (*list).template.totalItems as i32 - (*list).template.maxShowed as i32
        {
            while (selectedRow as i32) < (*list).template.maxShowed as i32 - 1 {
                selectedRow += 1;
                if (*(*list)
                    .template
                    .items
                    .at(scrollOffset as i32 + selectedRow as i32))
                .id != LIST_HEADER
                {
                    (*list).selectedRow = selectedRow;
                    return 1;
                }
            }
            return 0;
        } else {
            while selectedRow < newRow {
                selectedRow += 1;
                if (*(*list)
                    .template
                    .items
                    .at(scrollOffset as i32 + selectedRow as i32))
                .id != LIST_HEADER
                {
                    (*list).selectedRow = selectedRow;
                    return 1;
                }
            }
            newScroll = scrollOffset as u32 + 1;
        }
    }
    (*list).selectedRow = newRow;
    (*list).scrollOffset = newScroll as u16;
    return 2;
}
pub(crate) unsafe extern "C" fn ListMenuScroll(list: *mut ListMenu, count: u8, movingDown: u8) {
    if count as u16 >= (*list).template.maxShowed {
        FillWindowPixelBuffer(
            (*list).template.windowId,
            (*list).template.fillValue() | (*list).template.fillValue() << 4,
        );
        ListMenuPrintEntries(list, (*list).scrollOffset, 0, (*list).template.maxShowed);
    } else {
        let mut yMultiplier: u8 =
            GetFontAttribute((*list).template.fontId(), FONTATTR_MAX_LETTER_HEIGHT)
                + (*list).template.itemVerticalPadding();
        if movingDown == 0 {
            let mut y: u16 = 0;
            let mut width: u16 = 0;
            let mut height: u16 = 0;
            ScrollWindow(
                (*list).template.windowId,
                1,
                count * yMultiplier,
                (*list).template.fillValue() | (*list).template.fillValue() << 4,
            );
            ListMenuPrintEntries(list, (*list).scrollOffset, 0, count as u16);
            y = (*list).template.maxShowed * yMultiplier as u16
                + (*list).template.upText_Y() as u16;
            width = GetWindowAttribute((*list).template.windowId, WINDOW_WIDTH) as u16 * 8;
            height = GetWindowAttribute((*list).template.windowId, WINDOW_HEIGHT) as u16 * 8 - y;
            FillWindowPixelRect(
                (*list).template.windowId,
                (*list).template.fillValue() | (*list).template.fillValue() << 4,
                0,
                y,
                width,
                height,
            );
        } else {
            let mut width: u16 = 0;
            ScrollWindow(
                (*list).template.windowId,
                0,
                count * yMultiplier,
                (*list).template.fillValue() | (*list).template.fillValue() << 4,
            );
            ListMenuPrintEntries(
                list,
                (*list).scrollOffset + ((*list).template.maxShowed - count as u16),
                (*list).template.maxShowed - count as u16,
                count as u16,
            );
            width = GetWindowAttribute((*list).template.windowId, WINDOW_WIDTH) as u16 * 8;
            FillWindowPixelRect(
                (*list).template.windowId,
                (*list).template.fillValue() | (*list).template.fillValue() << 4,
                0,
                0,
                width,
                (*list).template.upText_Y() as u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuChangeSelection(
    list: *mut ListMenu,
    updateCursorAndCallCallback: u8,
    count: u8,
    movingDown: u8,
) -> u8 {
    let mut oldSelectedRow: u16 = 0;
    let mut selectionChange: u8 = 0;
    let mut i: u8 = 0;
    let mut cursorCount: u8 = 0;
    oldSelectedRow = (*list).selectedRow;
    cursorCount = 0;
    selectionChange = 0;
    i = 0;
    while i < count {
        loop {
            let mut ret: u8 = ListMenuUpdateSelectedRowIndexAndScrollOffset(list, movingDown);
            selectionChange |= ret;
            if ret != 2 {
                break;
            }
            cursorCount += 1;
            if (*(*list)
                .template
                .items
                .at((*list).scrollOffset as i32 + (*list).selectedRow as i32))
            .id != LIST_HEADER
            {
                break;
            }
        }
        i += 1;
    }
    if updateCursorAndCallCallback != 0 {
        match selectionChange {
            1 => {
                ListMenuErasePrintedCursor(list, oldSelectedRow);
                ListMenuDrawCursor(list);
                ListMenuCallSelectionChangedCallback(list, FALSE);
                CopyWindowToVram((*list).template.windowId, COPYWIN_GFX);
            }
            2 | 3 => {
                ListMenuErasePrintedCursor(list, oldSelectedRow);
                ListMenuScroll(list, cursorCount, movingDown);
                ListMenuDrawCursor(list);
                ListMenuCallSelectionChangedCallback(list, FALSE);
                CopyWindowToVram((*list).template.windowId, COPYWIN_GFX);
            }
            _ => {
                return TRUE;
            }
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ListMenuCallSelectionChangedCallback(
    list: *mut ListMenu,
    onInit: u8,
) {
    if (*list).template.moveCursorFunc.is_some() {
        (*list).template.moveCursorFunc.unwrap_unchecked()(
            (*(*list)
                .template
                .items
                .at((*list).scrollOffset as i32 + (*list).selectedRow as i32))
            .id,
            onInit,
            list,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuOverrideSetColors(
    cursorPal: u8,
    fillValue: u8,
    cursorShadowPal: u8,
) {
    gListMenuOverride.set_cursorPal(cursorPal);
    gListMenuOverride.set_fillValue(fillValue);
    gListMenuOverride.set_cursorShadowPal(cursorShadowPal);
    gListMenuOverride.set_enabled(TRUE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuDefaultCursorMoveFunc(
    itemIndex: i32,
    onInit: u8,
    list: *mut ListMenu,
) {
    if onInit == 0 {
        PlaySE(SE_SELECT);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetTemplateField(taskId: u8, field: u8) -> i32 {
    let mut data: *mut ListMenu = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut ListMenu;
    match field {
        LISTFIELD_MOVECURSORFUNC | LISTFIELD_MOVECURSORFUNC2 => {
            return core::mem::transmute::<_, usize>((*data).template.moveCursorFunc) as i32;
        }
        LISTFIELD_TOTALITEMS => {
            return (*data).template.totalItems as i32;
        }
        LISTFIELD_MAXSHOWED => {
            return (*data).template.maxShowed as i32;
        }
        LISTFIELD_WINDOWID => {
            return (*data).template.windowId as i32;
        }
        LISTFIELD_HEADERX => {
            return (*data).template.header_X as i32;
        }
        LISTFIELD_ITEMX => {
            return (*data).template.item_X as i32;
        }
        LISTFIELD_CURSORX => {
            return (*data).template.cursor_X as i32;
        }
        LISTFIELD_UPTEXTY => {
            return (*data).template.upText_Y() as i32;
        }
        LISTFIELD_CURSORPAL => {
            return (*data).template.cursorPal() as i32;
        }
        LISTFIELD_FILLVALUE => {
            return (*data).template.fillValue() as i32;
        }
        LISTFIELD_CURSORSHADOWPAL => {
            return (*data).template.cursorShadowPal() as i32;
        }
        LISTFIELD_LETTERSPACING => {
            return (*data).template.lettersSpacing() as i32;
        }
        LISTFIELD_ITEMVERTICALPADDING => {
            return (*data).template.itemVerticalPadding() as i32;
        }
        LISTFIELD_SCROLLMULTIPLE => {
            return (*data).template.scrollMultiple() as i32;
        }
        LISTFIELD_FONTID => {
            return (*data).template.fontId() as i32;
        }
        LISTFIELD_CURSORKIND => {
            return (*data).template.cursorKind() as i32;
        }
        _ => {
            return -1;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuSetTemplateField(taskId: u8, field: u8, value: i32) {
    let mut data: *mut ListMenu = &raw mut gTasks[taskId].data as *mut c_void as *mut ListMenu;
    match field {
        LISTFIELD_MOVECURSORFUNC | LISTFIELD_MOVECURSORFUNC2 => {
            (*data).template.moveCursorFunc = core::mem::transmute::<
                _,
                Option<unsafe extern "C" fn(i32, u8, *mut ListMenu)>,
            >(value as usize as *mut c_void);
        }
        LISTFIELD_TOTALITEMS => {
            (*data).template.totalItems = value as u16;
        }
        LISTFIELD_MAXSHOWED => {
            (*data).template.maxShowed = value as u16;
        }
        LISTFIELD_WINDOWID => {
            (*data).template.windowId = value as u8;
        }
        LISTFIELD_HEADERX => {
            (*data).template.header_X = value as u8;
        }
        LISTFIELD_ITEMX => {
            (*data).template.item_X = value as u8;
        }
        LISTFIELD_CURSORX => {
            (*data).template.cursor_X = value as u8;
        }
        LISTFIELD_UPTEXTY => {
            (*data).template.set_upText_Y(value as u8);
        }
        LISTFIELD_CURSORPAL => {
            (*data).template.set_cursorPal(value as u8);
        }
        LISTFIELD_FILLVALUE => {
            (*data).template.set_fillValue(value as u8);
        }
        LISTFIELD_CURSORSHADOWPAL => {
            (*data).template.set_cursorShadowPal(value as u8);
        }
        LISTFIELD_LETTERSPACING => {
            (*data).template.set_lettersSpacing(value as u8);
        }
        LISTFIELD_ITEMVERTICALPADDING => {
            (*data).template.set_itemVerticalPadding(value as u8);
        }
        LISTFIELD_SCROLLMULTIPLE => {
            (*data).template.set_scrollMultiple(value as u8);
        }
        LISTFIELD_FONTID => {
            (*data).template.set_fontId(value as u8);
        }
        LISTFIELD_CURSORKIND => {
            (*data).template.set_cursorKind(value as u8);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCallback_ScrollIndicatorArrow(sprite: *mut Sprite) {
    let mut multiplier: i32 = 0;
    match (*sprite).data[0] {
        0 => {
            StartSpriteAnim(sprite, (*sprite).data[1] as u8);
            (*sprite).data[0] += 1;
        }
        1 => {
            match (*sprite).data[2] {
                0 => {
                    multiplier = (*sprite).data[3] as i32;
                    (*sprite).x2 =
                        (gSineTable[(*sprite).data[5] as u8] as i32 * multiplier / 256) as i16;
                }
                1 => {
                    multiplier = (*sprite).data[3] as i32;
                    (*sprite).y2 =
                        (gSineTable[(*sprite).data[5] as u8] as i32 * multiplier / 256) as i16;
                }
                _ => {}
            }
            (*sprite).data[5] += (*sprite).data[4];
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AddScrollIndicatorArrowObject(
    arrowDir: u8,
    x: u8,
    y: u8,
    tileTag: u16,
    palTag: u16,
) -> u8 {
    let mut spriteId: u8 = 0;
    let mut spriteTemplate: SpriteTemplate = zeroed();
    spriteTemplate = *sSpriteTemplate_ScrollArrowIndicator;
    spriteTemplate.tileTag = tileTag;
    spriteTemplate.paletteTag = palTag;
    spriteId = CreateSprite(&raw mut spriteTemplate, x as i16, y as i16, 0);
    gSprites[spriteId].set_invisible(TRUE as u16);
    gSprites[spriteId].data[0] = 0;
    gSprites[spriteId].data[1] = sScrollIndicatorTemplates[arrowDir].animNum() as i16;
    gSprites[spriteId].data[2] = sScrollIndicatorTemplates[arrowDir].bounceDir() as i16;
    gSprites[spriteId].data[3] = sScrollIndicatorTemplates[arrowDir].multiplier as i16;
    gSprites[spriteId].data[4] = sScrollIndicatorTemplates[arrowDir].frequency as i16;
    gSprites[spriteId].data[5] = 0;
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddScrollIndicatorArrowPair(
    arrowInfo: *mut ScrollArrowsTemplate,
    scrollOffset: *mut u16,
) -> u8 {
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    let mut spritePal: SpritePalette = zeroed();
    let mut data: *mut ScrollIndicatorPair = null_mut();
    let mut taskId: u8 = 0;
    spriteSheet.data = sScrollIndicator_Gfx.as_ptr().cast_mut();
    spriteSheet.size = 0x100;
    spriteSheet.tag = (*arrowInfo).tileTag;
    LoadCompressedSpriteSheet(&raw mut spriteSheet);
    if (*arrowInfo).palTag == TAG_NONE {
        LoadPalette(
            sRedInterface_Pal.as_ptr().cast_mut() as *mut c_void,
            0x100 + (*arrowInfo).palNum as u16 * 16,
            32,
        );
    } else {
        spritePal.data = sRedInterface_Pal.as_ptr().cast_mut();
        spritePal.tag = (*arrowInfo).palTag;
        LoadSpritePalette(&raw mut spritePal);
    }
    taskId = CreateTask(Some(Task_ScrollIndicatorArrowPair), 0);
    data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut ScrollIndicatorPair;
    (*data).field_0 = 0;
    (*data).scrollOffset = scrollOffset;
    (*data).fullyUpThreshold = (*arrowInfo).fullyUpThreshold;
    (*data).fullyDownThreshold = (*arrowInfo).fullyDownThreshold;
    (*data).tileTag = (*arrowInfo).tileTag;
    (*data).palTag = (*arrowInfo).palTag;
    (*data).topSpriteId = AddScrollIndicatorArrowObject(
        (*arrowInfo).firstArrowType,
        (*arrowInfo).firstX,
        (*arrowInfo).firstY,
        (*arrowInfo).tileTag,
        (*arrowInfo).palTag,
    );
    (*data).bottomSpriteId = AddScrollIndicatorArrowObject(
        (*arrowInfo).secondArrowType,
        (*arrowInfo).secondX,
        (*arrowInfo).secondY,
        (*arrowInfo).tileTag,
        (*arrowInfo).palTag,
    );
    if (*arrowInfo).palTag == TAG_NONE {
        gSprites[(*data).topSpriteId]
            .oam
            .set_paletteNum((*arrowInfo).palNum as u16);
        gSprites[(*data).bottomSpriteId]
            .oam
            .set_paletteNum((*arrowInfo).palNum as u16);
    }
    return taskId;
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
    if arrowType == SCROLL_ARROW_UP || arrowType == SCROLL_ARROW_DOWN {
        gTempScrollArrowTemplate.firstArrowType = SCROLL_ARROW_UP as u8;
        gTempScrollArrowTemplate.firstX = commonPos as u8;
        gTempScrollArrowTemplate.firstY = firstPos as u8;
        gTempScrollArrowTemplate.secondArrowType = SCROLL_ARROW_DOWN as u8;
        gTempScrollArrowTemplate.secondX = commonPos as u8;
        gTempScrollArrowTemplate.secondY = secondPos as u8;
    } else {
        gTempScrollArrowTemplate.firstArrowType = SCROLL_ARROW_LEFT;
        gTempScrollArrowTemplate.firstX = firstPos as u8;
        gTempScrollArrowTemplate.firstY = commonPos as u8;
        gTempScrollArrowTemplate.secondArrowType = SCROLL_ARROW_RIGHT;
        gTempScrollArrowTemplate.secondX = secondPos as u8;
        gTempScrollArrowTemplate.secondY = commonPos as u8;
    }
    gTempScrollArrowTemplate.fullyUpThreshold = 0;
    gTempScrollArrowTemplate.fullyDownThreshold = fullyDownThreshold as u16;
    gTempScrollArrowTemplate.tileTag = tileTag as u16;
    gTempScrollArrowTemplate.palTag = palTag as u16;
    gTempScrollArrowTemplate.palNum = 0;
    return AddScrollIndicatorArrowPair(&raw mut gTempScrollArrowTemplate, scrollOffset);
}
pub(crate) unsafe extern "C" fn Task_ScrollIndicatorArrowPair(taskId: u8) {
    let mut data: *mut ScrollIndicatorPair =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut ScrollIndicatorPair;
    let mut currItem: u16 = *(*data).scrollOffset;
    if currItem == (*data).fullyUpThreshold && currItem != 0xFFFF {
        gSprites[(*data).topSpriteId].set_invisible(TRUE as u16);
    } else {
        gSprites[(*data).topSpriteId].set_invisible(FALSE as u16);
    }
    if currItem == (*data).fullyDownThreshold {
        gSprites[(*data).bottomSpriteId].set_invisible(TRUE as u16);
    } else {
        gSprites[(*data).bottomSpriteId].set_invisible(FALSE as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ScrollIndicatorArrowPairOnMainMenu(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut scrollData: *mut ScrollIndicatorPair = data as *mut c_void as *mut ScrollIndicatorPair;
    if *data.at(15) != 0 {
        gSprites[(*scrollData).topSpriteId].set_invisible(FALSE as u16);
        gSprites[(*scrollData).bottomSpriteId].set_invisible(TRUE as u16);
    } else {
        gSprites[(*scrollData).topSpriteId].set_invisible(TRUE as u16);
        gSprites[(*scrollData).bottomSpriteId].set_invisible(FALSE as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveScrollIndicatorArrowPair(taskId: u8) {
    let mut data: *mut ScrollIndicatorPair =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut ScrollIndicatorPair;
    if (*data).tileTag != TAG_NONE {
        FreeSpriteTilesByTag((*data).tileTag);
    }
    if (*data).palTag != TAG_NONE {
        FreeSpritePaletteByTag((*data).palTag);
    }
    DestroySprite(&raw mut gSprites[(*data).topSpriteId]);
    DestroySprite(&raw mut gSprites[(*data).bottomSpriteId]);
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn ListMenuAddCursorObjectInternal(
    cursor: *mut CursorStruct,
    cursorObjId: u32,
) -> u8 {
    match cursorObjId {
        1 => {
            return ListMenuAddRedArrowCursorObject(cursor);
        }
        _ => {
            return ListMenuAddRedOutlineCursorObject(cursor);
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ListMenuUpdateCursorObject(
    taskId: u8,
    x: u16,
    y: u16,
    cursorObjId: u32,
) {
    match cursorObjId {
        0 => {
            ListMenuUpdateRedOutlineCursorObject(taskId, x, y);
        }
        1 => {
            ListMenuUpdateRedArrowCursorObject(taskId, x, y);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ListMenuRemoveCursorObject(taskId: u8, cursorObjId: u32) {
    match cursorObjId {
        0 => {
            ListMenuRemoveRedOutlineCursorObject(taskId);
        }
        1 => {
            ListMenuRemoveRedArrowCursorObject(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_RedOutlineCursor(taskId: u8) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuGetRedOutlineCursorSpriteCount(
    rowWidth: u16,
    rowHeight: u16,
) -> u8 {
    let mut i: i32 = 0;
    let mut count: i32 = 4;
    if rowWidth > 16 {
        i = 8;
        while i < rowWidth as i32 - 8 {
            count += 2;
            i += 8;
        }
    }
    if rowHeight > 16 {
        i = 8;
        while i < rowHeight as i32 - 8 {
            count += 2;
            i += 8;
        }
    }
    return count as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuSetUpRedOutlineCursorSpriteOamTable(
    rowWidth: u16,
    rowHeight: u16,
    mut subsprites: *mut Subsprite,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut id: i32 = 0;
    *subsprites.at(id) = *sSubsprite_RedOutline1;
    (*subsprites.at(id)).x = -120;
    (*subsprites.at(id)).y = -120;
    id += 1;
    *subsprites.at(id) = *sSubsprite_RedOutline2;
    (*subsprites.at(id)).x = rowWidth as i8 + -128;
    (*subsprites.at(id)).y = -120;
    id += 1;
    *subsprites.at(id) = *sSubsprite_RedOutline7;
    (*subsprites.at(id)).x = -120;
    (*subsprites.at(id)).y = rowHeight as i8 + -128;
    id += 1;
    *subsprites.at(id) = *sSubsprite_RedOutline8;
    (*subsprites.at(id)).x = rowWidth as i8 + -128;
    (*subsprites.at(id)).y = rowHeight as i8 + -128;
    id += 1;
    if rowWidth > 16 {
        i = 8;
        while i < rowWidth as i32 - 8 {
            *subsprites.at(id) = *sSubsprite_RedOutline3;
            (*subsprites.at(id)).x = i as i8 - 120;
            (*subsprites.at(id)).y = -120;
            id += 1;
            *subsprites.at(id) = *sSubsprite_RedOutline6;
            (*subsprites.at(id)).x = i as i8 - 120;
            (*subsprites.at(id)).y = rowHeight as i8 + -128;
            id += 1;
            i += 8;
        }
    }
    if rowHeight > 16 {
        j = 8;
        while j < rowHeight as i32 - 8 {
            *subsprites.at(id) = *sSubsprite_RedOutline4;
            (*subsprites.at(id)).x = -120;
            (*subsprites.at(id)).y = j as i8 - 120;
            id += 1;
            *subsprites.at(id) = *sSubsprite_RedOutline5;
            (*subsprites.at(id)).x = rowWidth as i8 + -128;
            (*subsprites.at(id)).y = j as i8 - 120;
            id += 1;
            j += 8;
        }
    }
}
pub(crate) unsafe extern "C" fn ListMenuAddRedOutlineCursorObject(cursor: *mut CursorStruct) -> u8 {
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    let mut spritePal: SpritePalette = zeroed();
    let mut data: *mut RedOutlineCursor = null_mut();
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut taskId: u8 = 0;
    spriteSheet.data = sOutlineCursor_Gfx.as_ptr().cast_mut();
    spriteSheet.size = 0x100;
    spriteSheet.tag = (*cursor).tileTag;
    LoadCompressedSpriteSheet(&raw mut spriteSheet);
    if (*cursor).palTag == TAG_NONE {
        LoadPalette(
            sRedInterface_Pal.as_ptr().cast_mut() as *mut c_void,
            0x100 + (*cursor).palNum as u16 * 16,
            32,
        );
    } else {
        spritePal.data = sRedInterface_Pal.as_ptr().cast_mut();
        spritePal.tag = (*cursor).palTag;
        LoadSpritePalette(&raw mut spritePal);
    }
    taskId = CreateTask(Some(Task_RedOutlineCursor), 0);
    data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut RedOutlineCursor;
    (*data).tileTag = (*cursor).tileTag;
    (*data).palTag = (*cursor).palTag;
    (*data).subspriteTable.subspriteCount =
        ListMenuGetRedOutlineCursorSpriteCount((*cursor).rowWidth, (*cursor).rowHeight);
    (*data).subspriteTable.subsprites = {
        (*data).subspritesPtr =
            Alloc((*data).subspriteTable.subspriteCount as u32 * 4) as *mut Subsprite;
        (*data).subspritesPtr
    };
    ListMenuSetUpRedOutlineCursorSpriteOamTable(
        (*cursor).rowWidth,
        (*cursor).rowHeight,
        (*data).subspritesPtr,
    );
    spriteTemplate = gDummySpriteTemplate;
    spriteTemplate.tileTag = (*cursor).tileTag;
    spriteTemplate.paletteTag = (*cursor).palTag;
    (*data).spriteId = CreateSprite(
        &raw mut spriteTemplate,
        (*cursor).left as i16 + 120,
        (*cursor).top as i16 + 120,
        0,
    );
    SetSubspriteTables(
        &raw mut gSprites[(*data).spriteId],
        &raw mut (*data).subspriteTable,
    );
    gSprites[(*data).spriteId].oam.set_priority(0);
    gSprites[(*data).spriteId].subpriority = 0;
    gSprites[(*data).spriteId].set_subspriteTableNum(0);
    if (*cursor).palTag == TAG_NONE {
        gSprites[(*data).spriteId]
            .oam
            .set_paletteNum((*cursor).palNum as u16);
    }
    return taskId;
}
pub(crate) unsafe extern "C" fn ListMenuUpdateRedOutlineCursorObject(taskId: u8, x: u16, y: u16) {
    let mut data: *mut RedOutlineCursor =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut RedOutlineCursor;
    gSprites[(*data).spriteId].x = x as i16 + 120;
    gSprites[(*data).spriteId].y = y as i16 + 120;
}
pub(crate) unsafe extern "C" fn ListMenuRemoveRedOutlineCursorObject(taskId: u8) {
    let mut data: *mut RedOutlineCursor =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut RedOutlineCursor;
    Free((*data).subspritesPtr as *mut c_void);
    if (*data).tileTag != TAG_NONE {
        FreeSpriteTilesByTag((*data).tileTag);
    }
    if (*data).palTag != TAG_NONE {
        FreeSpritePaletteByTag((*data).palTag);
    }
    DestroySprite(&raw mut gSprites[(*data).spriteId]);
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn SpriteCallback_RedArrowCursor(sprite: *mut Sprite) {
    (*sprite).x2 = gSineTable[(*sprite).data[0] as u8] / 64;
    (*sprite).data[0] += 8;
}
pub(crate) unsafe extern "C" fn Task_RedArrowCursor(taskId: u8) {}
pub(crate) unsafe extern "C" fn ListMenuAddRedArrowCursorObject(cursor: *mut CursorStruct) -> u8 {
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    let mut spritePal: SpritePalette = zeroed();
    let mut data: *mut RedArrowCursor = null_mut();
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut taskId: u8 = 0;
    spriteSheet.data = sArrowCursor_Gfx.as_ptr().cast_mut();
    spriteSheet.size = 0x80;
    spriteSheet.tag = (*cursor).tileTag;
    LoadCompressedSpriteSheet(&raw mut spriteSheet);
    if (*cursor).palTag == TAG_NONE {
        LoadPalette(
            sRedInterface_Pal.as_ptr().cast_mut() as *mut c_void,
            0x100 + (*cursor).palNum as u16 * 16,
            32,
        );
    } else {
        spritePal.data = sRedInterface_Pal.as_ptr().cast_mut();
        spritePal.tag = (*cursor).palTag;
        LoadSpritePalette(&raw mut spritePal);
    }
    taskId = CreateTask(Some(Task_RedArrowCursor), 0);
    data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut RedArrowCursor;
    (*data).tileTag = (*cursor).tileTag;
    (*data).palTag = (*cursor).palTag;
    spriteTemplate = *sSpriteTemplate_RedArrowCursor;
    spriteTemplate.tileTag = (*cursor).tileTag;
    spriteTemplate.paletteTag = (*cursor).palTag;
    (*data).spriteId = CreateSprite(
        &raw mut spriteTemplate,
        (*cursor).left as i16,
        (*cursor).top as i16,
        0,
    );
    gSprites[(*data).spriteId].x2 = 8;
    gSprites[(*data).spriteId].y2 = 8;
    if (*cursor).palTag == TAG_NONE {
        gSprites[(*data).spriteId]
            .oam
            .set_paletteNum((*cursor).palNum as u16);
    }
    return taskId;
}
pub(crate) unsafe extern "C" fn ListMenuUpdateRedArrowCursorObject(taskId: u8, x: u16, y: u16) {
    let mut data: *mut RedArrowCursor =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut RedArrowCursor;
    gSprites[(*data).spriteId].x = x as i16;
    gSprites[(*data).spriteId].y = y as i16;
}
pub(crate) unsafe extern "C" fn ListMenuRemoveRedArrowCursorObject(taskId: u8) {
    let mut data: *mut RedArrowCursor =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut RedArrowCursor;
    if (*data).tileTag != TAG_NONE {
        FreeSpriteTilesByTag((*data).tileTag);
    }
    if (*data).palTag != TAG_NONE {
        FreeSpritePaletteByTag((*data).palTag);
    }
    DestroySprite(&raw mut gSprites[(*data).spriteId]);
    DestroyTask(taskId);
}
