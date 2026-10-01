//! Translated from `src/menu_specialized.c` by tools/rustport/c2rs.py.
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
    clippy::int_plus_one,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    dead_code,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::ConvertInternationalPlayerName;
use crate::list_menu::{
    AddScrollIndicatorArrowPairParameterized, ListMenuInit, gMultiuseListMenuTemplate,
};
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::{
    AddTextPrinterParameterized2, AddTextPrinterParameterized3, AddTextPrinterParameterized4,
    ClearStdWindowAndFrameToTransparent, CreateYesNoMenu, DrawStdFrameWithCustomTileAndPalette,
    GetPlayerTextSpeedDelay, ScheduleBgCopyTilemapToVram, SetStandardWindowBorderStyle,
};
use crate::move_relearner::MoveRelearnerShowHideHearts;
use crate::palette::LoadPalette;
use crate::pokemon::{
    GetBoxMonGender, GetLevelFromBoxMonExp, GetMonData2, GetMonData3, GetMonGender,
    GetMonSpritePalFromSpeciesAndPersonality, gPlayerParty,
};
use crate::pokemon_storage_system::{
    GetAndCopyBoxMonDataAt, GetBoxMonDataAt, GetBoxNamePtr, GetBoxedMonPtr,
};
use crate::scanline_effect::ScanlineEffect_Clear;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::string_util::{ConvertInternationalString, StringGet_Nickname};
use crate::text::{DeactivateAllTextPrinters, IsTextPrinterActive, RunTextPrinters};
use crate::text_window::LoadUserWindowBorderGfx;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow,
};
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
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `GetMaxWidthInMenuTable` with this module's view of its types.
#[inline]
unsafe fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32 {
    unsafe { crate::international_string_util::GetMaxWidthInMenuTable(a0 as _, a1) }
}
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LoadSpecialPokePic` with this module's view of its types.
#[inline]
unsafe fn LoadSpecialPokePic(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
    a4: u8,
) {
    unsafe {
        crate::decompress::LoadSpecialPokePic(a0 as _, a1 as _, a2, a3, a4);
    }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
/// `SeekSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn SeekSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::SeekSpriteAnim(a0 as _, a1);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StringCompare` with this module's view of its types.
#[inline]
unsafe fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompare(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopyPadded` with this module's view of its types.
#[inline]
unsafe fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8 {
    unsafe { crate::string_util::StringCopyPadded(a0 as _, a1 as _, a2, a3) as *mut u8 }
}
/// `StringLength` with this module's view of its types.
#[inline]
unsafe fn StringLength(a0: *mut u8) -> u16 {
    unsafe { crate::string_util::StringLength(a0 as _) }
}
// The C's names for task and sprite data slots.
const sSparkleId: usize = 0;
const sDelayTimer: usize = 1;
const sNumExtraSparkles: usize = 2;
const sCurSparkleId: usize = 3;
const sMonSpriteId: usize = 4;
const sNextSparkleSpriteId: usize = 5;
// Data tables (translate with cdata.py): sWindowTemplates_MailboxMenu sPlayerNameTextColors sEmptyItemName sConditionGraphScanline sConditionToLineLength sMoveRelearnerWindowTemplates sMoveRelearnerYesNoMenuTemplate sMoveRelearnerMovesListTemplate sConditionPokeball_Gfx sConditionPokeballPlaceholder_Gfx sConditionSparkle_Gfx sConditionSparkle_Pal sOam_ConditionMonPic sOam_ConditionSelectionIcon sAnim_ConditionSelectionIcon_Selected sAnim_ConditionSelectionIcon_Unselected sAnims_ConditionSelectionIcon sOam_ConditionSparkle sAnim_ConditionSparkle sAnims_ConditionSparkle sSpriteTemplate_ConditionSparkle sConditionSparkleCoords sLvlUpStatStrings

static sAnims_ConditionSelectionIcon: Table<CArray<*mut AnimCmd, 2>> =
    Table((&raw const crate::data::menu_specialized::sAnims_ConditionSelectionIcon).cast());
static sConditionGraphScanline: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::menu_specialized::sConditionGraphScanline).cast());
static sConditionPokeballPlaceholder_Gfx: Table<CArray<u32, 8>> =
    Table((&raw const crate::data::menu_specialized::sConditionPokeballPlaceholder_Gfx).cast());
static sConditionPokeball_Gfx: Table<CArray<u32, 64>> =
    Table((&raw const crate::data::menu_specialized::sConditionPokeball_Gfx).cast());
static sConditionSparkleCoords: Table<CArray<CArray<i16, 2>, 10>> =
    Table((&raw const crate::data::menu_specialized::sConditionSparkleCoords).cast());
static sConditionSparkle_Gfx: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::menu_specialized::sConditionSparkle_Gfx).cast());
static sConditionSparkle_Pal: Table<CArray<u32, 224>> =
    Table((&raw const crate::data::menu_specialized::sConditionSparkle_Pal).cast());
static sConditionToLineLength: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::menu_specialized::sConditionToLineLength).cast());
static sEmptyItemName: Table<CArray<u8, 1>> =
    Table((&raw const crate::data::menu_specialized::sEmptyItemName).cast());
static sLvlUpStatStrings: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::menu_specialized::sLvlUpStatStrings).cast());
static sMoveRelearnerMovesListTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::menu_specialized::sMoveRelearnerMovesListTemplate).cast());
static sMoveRelearnerWindowTemplates: Table<CArray<WindowTemplate, 6>> =
    Table((&raw const crate::data::menu_specialized::sMoveRelearnerWindowTemplates).cast());
static sMoveRelearnerYesNoMenuTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::menu_specialized::sMoveRelearnerYesNoMenuTemplate).cast());
static sOam_ConditionMonPic: Table<OamData> =
    Table((&raw const crate::data::menu_specialized::sOam_ConditionMonPic).cast());
static sOam_ConditionSelectionIcon: Table<OamData> =
    Table((&raw const crate::data::menu_specialized::sOam_ConditionSelectionIcon).cast());
static sPlayerNameTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::menu_specialized::sPlayerNameTextColors).cast());
static sSpriteTemplate_ConditionSparkle: Table<SpriteTemplate> =
    Table((&raw const crate::data::menu_specialized::sSpriteTemplate_ConditionSparkle).cast());
static sWindowTemplates_MailboxMenu: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::menu_specialized::sWindowTemplates_MailboxMenu).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMailboxWindowIds: Aligned<CArray<u8, 3>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMailboxList: *mut ListMenuItem = null_mut();

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
/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

pub unsafe fn MailboxMenu_Alloc(count: u8) -> u8 {
    sMailboxList = Alloc((count as u32 + 1) * 8) as *mut ListMenuItem;
    if sMailboxList.is_null() {
        return FALSE;
    }
    for i in 0..3u8 {
        sMailboxWindowIds[i] = WINDOW_NONE;
    }
    TRUE
}
pub unsafe fn MailboxMenu_AddWindow(windowIdx: u8) -> u8 {
    if sMailboxWindowIds[windowIdx] == WINDOW_NONE {
        if windowIdx == MAILBOXWIN_OPTIONS {
            let mut template: WindowTemplate = sWindowTemplates_MailboxMenu[windowIdx];
            template.width = GetMaxWidthInMenuTable(
                (&raw const (*(&raw const crate::data::player_pc::gMailboxMailOptions)
                    .cast::<CArray<MenuAction, 0>>())[0])
                    .cast_mut(),
                4,
            ) as u8;
            sMailboxWindowIds[windowIdx] = AddWindow(&raw mut template) as u8;
        } else {
            sMailboxWindowIds[windowIdx] =
                AddWindow((&raw const sWindowTemplates_MailboxMenu[windowIdx]).cast_mut()) as u8;
        }
        SetStandardWindowBorderStyle(sMailboxWindowIds[windowIdx], FALSE);
    }
    sMailboxWindowIds[windowIdx]
}
pub unsafe fn MailboxMenu_RemoveWindow(windowIdx: u8) {
    ClearStdWindowAndFrameToTransparent(sMailboxWindowIds[windowIdx], FALSE);
    ClearWindowTilemap(sMailboxWindowIds[windowIdx]);
    RemoveWindow(sMailboxWindowIds[windowIdx]);
    sMailboxWindowIds[windowIdx] = WINDOW_NONE;
}
unsafe fn MailboxMenu_GetWindowId(windowIdx: u8) -> u8 {
    sMailboxWindowIds[windowIdx]
}
pub(crate) unsafe fn MailboxMenu_ItemPrintFunc(windowId: u8, itemId: u32, y: u8) {
    let mut buffer: CArray<u8, 30> = zeroed();
    if itemId == LIST_CANCEL as u32 {
        return;
    }
    StringCopy(
        buffer.as_mut_ptr(),
        (*gSaveBlock1Ptr).mail[PARTY_SIZE as u32 + itemId]
            .playerName
            .as_mut_ptr(),
    );
    ConvertInternationalPlayerName(buffer.as_mut_ptr());
    let length: u16 = StringLength(buffer.as_mut_ptr());
    if length < 6 {
        ConvertInternationalString(buffer.as_mut_ptr(), LANGUAGE_JAPANESE);
    }
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        8,
        y,
        0,
        0,
        sPlayerNameTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        buffer.as_mut_ptr(),
    );
}
pub unsafe fn MailboxMenu_CreateList(page: *mut PlayerPCItemPageStruct) -> u8 {
    let mut i: u16 = 0;
    while i < (*page).count as u16 {
        (*sMailboxList.at(i)).name = sEmptyItemName.as_ptr().cast_mut();
        (*sMailboxList.at(i)).id = i as i32;
        i += 1;
    }
    (*sMailboxList.at(i)).name = (*(&raw const crate::data::strings::gText_Cancel2)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    (*sMailboxList.at(i)).id = LIST_CANCEL;
    gMultiuseListMenuTemplate.items = sMailboxList;
    gMultiuseListMenuTemplate.totalItems = (*page).count as u16 + 1;
    gMultiuseListMenuTemplate.windowId = sMailboxWindowIds[1];
    gMultiuseListMenuTemplate.header_X = 0;
    gMultiuseListMenuTemplate.item_X = 8;
    gMultiuseListMenuTemplate.cursor_X = 0;
    gMultiuseListMenuTemplate.maxShowed = 8;
    gMultiuseListMenuTemplate.set_upText_Y(9);
    gMultiuseListMenuTemplate.set_cursorPal(2);
    gMultiuseListMenuTemplate.set_fillValue(1);
    gMultiuseListMenuTemplate.set_cursorShadowPal(3);
    gMultiuseListMenuTemplate.moveCursorFunc = Some(MailboxMenu_MoveCursorFunc);
    gMultiuseListMenuTemplate.itemPrintFunc = Some(MailboxMenu_ItemPrintFunc);
    gMultiuseListMenuTemplate.set_fontId(FONT_NORMAL);
    gMultiuseListMenuTemplate.set_cursorKind(CURSOR_BLACK_ARROW);
    gMultiuseListMenuTemplate.set_lettersSpacing(0);
    gMultiuseListMenuTemplate.set_itemVerticalPadding(0);
    gMultiuseListMenuTemplate.set_scrollMultiple(LIST_NO_MULTIPLE_SCROLL);
    ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        (*page).itemsAbove,
        (*page).cursorPos,
    )
}
pub(crate) unsafe fn MailboxMenu_MoveCursorFunc(itemIndex: i32, onInit: u8, list: *mut ListMenu) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
    }
}
pub unsafe fn MailboxMenu_AddScrollArrows(page: *mut PlayerPCItemPageStruct) {
    (*page).scrollIndicatorTaskId = AddScrollIndicatorArrowPairParameterized(
        2,
        0xC8,
        12,
        0x94,
        (*page).count as i32 - (*page).pageItems as i32 + 1,
        0x6E,
        0x6E,
        &raw mut (*page).itemsAbove,
    );
}
pub unsafe fn MailboxMenu_Free() {
    Free(sMailboxList as *mut c_void);
}
pub unsafe fn ConditionGraph_Init(graph: *mut ConditionGraph) {
    let mut i: u8 = 0;
    for j in 0..(CONDITION_COUNT as u8) {
        i = 0;
        while i < CONDITION_GRAPH_UPDATE_STEPS as u8 {
            (*graph).newPositions[i][j].x = 0;
            (*graph).newPositions[i][j].y = 0;
            i += 1;
        }
        for i in 0..CONDITION_GRAPH_LOAD_MAX {
            (*graph).conditions[i][j] = 0;
            (*graph).savedPositions[i][j].x = CONDITION_GRAPH_CENTER_X;
            (*graph).savedPositions[i][j].y = CONDITION_GRAPH_CENTER_Y;
        }
        (*graph).curPositions[j].x = 0;
        (*graph).curPositions[j].y = 0;
    }
    (*graph).needsDraw = FALSE;
    (*graph).updateCounter = 0;
}
pub unsafe fn ConditionGraph_SetNewPositions(
    graph: *mut ConditionGraph,
    old: *mut UCoords16,
    new: *mut UCoords16,
) {
    let mut j: u16 = 0;
    let mut coord: i32 = 0;
    let mut increment: i32 = 0;
    for i in 0..CONDITION_COUNT {
        coord = ((*old.at(i)).x as i32) << 8;
        increment = (((*new.at(i)).x as i32 - (*old.at(i)).x as i32) << 8) / 10;
        j = 0;
        while j < 9 {
            (*graph).newPositions[j][i].x = (coord >> 8) as u16 + ((coord >> 7) as u16 & 1);
            coord += increment;
            j += 1;
        }
        (*graph).newPositions[j][i].x = (*new.at(i)).x;
        coord = ((*old.at(i)).y as i32) << 8;
        increment = (((*new.at(i)).y as i32 - (*old.at(i)).y as i32) << 8) / 10;
        j = 0;
        while j < 9 {
            (*graph).newPositions[j][i].y = (coord >> 8) as u16 + ((coord >> 7) as u16 & 1);
            coord += increment;
            j += 1;
        }
        (*graph).newPositions[j][i].y = (*new.at(i)).y;
    }
    (*graph).updateCounter = 0;
}
pub unsafe fn ConditionGraph_TryUpdate(graph: *mut ConditionGraph) -> u8 {
    if (*graph).updateCounter < CONDITION_GRAPH_UPDATE_STEPS {
        ConditionGraph_Update(graph);
        return (({
            (*graph).updateCounter += 1;
            (*graph).updateCounter
        }) != CONDITION_GRAPH_UPDATE_STEPS) as u8;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ConditionGraph_InitResetScanline(graph: *mut ConditionGraph) {
    (*graph).scanlineResetState = 0;
}
pub unsafe fn ConditionGraph_ResetScanline(graph: *mut ConditionGraph) -> u8 {
    let mut params: ScanlineEffectParams = zeroed();
    match (*graph).scanlineResetState {
        0 => {
            ScanlineEffect_Clear();
            (*graph).scanlineResetState += 1;
            return TRUE;
        }
        1 => {
            params = *sConditionGraphScanline;
            ScanlineEffect_SetParams(params);
            (*graph).scanlineResetState += 1;
            return FALSE;
        }
        _ => {
            return FALSE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ConditionGraph_Draw(graph: *mut ConditionGraph) {
    if (*graph).needsDraw == 0 {
        return;
    }
    ConditionGraph_CalcRightHalf(graph);
    ConditionGraph_CalcLeftHalf(graph);
    for i in 0..CONDITION_GRAPH_HEIGHT {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][(i as i32 + CONDITION_GRAPH_TOP_Y - 1) * 2] = {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][(i as i32 + CONDITION_GRAPH_TOP_Y - 1) * 2] =
                (*graph).scanlineRight[i][0] << 8 | (*graph).scanlineRight[i][1];
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][(i as i32 + CONDITION_GRAPH_TOP_Y - 1) * 2]
        };
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][(i as i32 + CONDITION_GRAPH_TOP_Y - 1) * 2 + 1] = {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][(i as i32 + CONDITION_GRAPH_TOP_Y - 1) * 2 + 1] =
                (*graph).scanlineLeft[i][0] << 8 | (*graph).scanlineLeft[i][1];
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][(i as i32 + CONDITION_GRAPH_TOP_Y - 1) * 2 + 1]
        };
    }
    (*graph).needsDraw = FALSE;
}
pub unsafe fn ConditionGraph_InitWindow(mut bg: u8) {
    if bg >= NUM_BACKGROUNDS {
        bg = 0;
    }
    let flags: u32 = 31 & !(shl_i32(1, bg as u32) as u32);
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    SetGpuReg(REG_OFFSET_WIN1H, CONDITION_GRAPH_CENTER_X);
    SetGpuReg(REG_OFFSET_WIN0V, 14457);
    SetGpuReg(REG_OFFSET_WIN1V, 14457);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, flags as u16);
}
pub unsafe fn ConditionGraph_Update(graph: *mut ConditionGraph) {
    for i in 0..CONDITION_COUNT {
        (*graph).curPositions[i] = (*graph).newPositions[(*graph).updateCounter][i];
    }
    (*graph).needsDraw = TRUE;
}
unsafe fn ConditionGraph_CalcLine(
    graph: *mut ConditionGraph,
    mut scanline: *mut u16,
    pos1: *mut UCoords16,
    pos2: *mut UCoords16,
    dir: u8,
    mut overflowScanline: *mut u16,
) {
    let mut i: u16 = 0;
    let mut height: u16 = 0;
    let mut top: u16 = 0;
    let mut bottom: u16 = 0;
    let mut x2: u16 = 0;
    let mut ptr: *mut u16 = null_mut();
    let mut x: i32 = 0;
    let mut xIncrement: i32 = 0;
    if (*pos1).y < (*pos2).y {
        top = (*pos1).y;
        bottom = (*pos2).y;
        x = ((*pos1).x as i32) << 10;
        x2 = (*pos2).x;
        height = bottom - top;
        if height != 0 {
            xIncrement = div_i32((x2 as i32 - (*pos1).x as i32) << 10, height as i32);
        }
    } else {
        bottom = (*pos1).y;
        top = (*pos2).y;
        x = ((*pos2).x as i32) << 10;
        x2 = (*pos1).x;
        height = bottom - top;
        if height != 0 {
            xIncrement = div_i32((x2 as i32 - (*pos2).x as i32) << 10, height as i32);
        }
    }
    height += 1;
    if overflowScanline.is_null() {
        scanline = scanline.at((top as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        for i in 0..height {
            *scanline.at(dir) = (x >> 10) as u16 + ((x >> 9) as u16 & 1) + dir as u16;
            x += xIncrement;
            scanline = scanline.at(2);
        }
        ptr = scanline.at(-2);
    } else if xIncrement > 0 {
        overflowScanline = overflowScanline.at((top as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        i = 0;
        while i < height {
            if x >= 0x26c00 {
                break;
            }
            *overflowScanline.at(dir) = (x >> 10) as u16 + ((x >> 9) as u16 & 1) + dir as u16;
            x += xIncrement;
            overflowScanline = overflowScanline.at(2);
            i += 1;
        }
        (*graph).bottom = top + i;
        scanline = scanline.at(((*graph).bottom as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        while i < height {
            *scanline.at(dir) = (x >> 10) as u16 + ((x >> 9) as u16 & 1) + dir as u16;
            x += xIncrement;
            scanline = scanline.at(2);
            i += 1;
        }
        ptr = scanline.at(-2);
    } else if xIncrement < 0 {
        scanline = scanline.at((top as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        i = 0;
        while i < height {
            *scanline.at(dir) = (x >> 10) as u16 + ((x >> 9) as u16 & 1) + dir as u16;
            if x < 0x26c00 {
                *scanline.at(dir) = CONDITION_GRAPH_CENTER_X;
                break;
            }
            x += xIncrement;
            scanline = scanline.at(2);
            i += 1;
        }
        (*graph).bottom = top + i;
        overflowScanline =
            overflowScanline.at(((*graph).bottom as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        while i < height {
            *overflowScanline.at(dir) = (x >> 10) as u16 + ((x >> 9) as u16 & 1) + dir as u16;
            x += xIncrement;
            overflowScanline = overflowScanline.at(2);
            i += 1;
        }
        ptr = overflowScanline.at(-2);
    } else {
        (*graph).bottom = top;
        scanline = scanline.at((top as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        overflowScanline = overflowScanline.at((top as i32 - CONDITION_GRAPH_TOP_Y) * 2);
        *scanline.at(1) = (*pos1).x + 1;
        *overflowScanline = (*pos2).x;
        *overflowScanline.at(1) = CONDITION_GRAPH_CENTER_X;
        return;
    }
    *ptr.at(dir) = dir as u16 + x2;
}
unsafe fn ConditionGraph_CalcRightHalf(graph: *mut ConditionGraph) {
    let mut y: u16 = 0;
    let mut bottom: u16 = 0;
    if (*graph).curPositions[0].y < (*graph).curPositions[1].y {
        y = (*graph).curPositions[0].y;
        ConditionGraph_CalcLine(
            graph,
            (*graph).scanlineRight[0].as_mut_ptr(),
            &raw mut (*graph).curPositions[0],
            &raw mut (*graph).curPositions[1],
            TRUE,
            null_mut(),
        );
    } else {
        y = (*graph).curPositions[1].y;
        ConditionGraph_CalcLine(
            graph,
            (*graph).scanlineRight[0].as_mut_ptr(),
            &raw mut (*graph).curPositions[1],
            &raw mut (*graph).curPositions[0],
            0,
            null_mut(),
        );
    }
    ConditionGraph_CalcLine(
        graph,
        (*graph).scanlineRight[0].as_mut_ptr(),
        &raw mut (*graph).curPositions[1],
        &raw mut (*graph).curPositions[2],
        TRUE,
        null_mut(),
    );
    let mut i: u16 = ((*graph).curPositions[2].y <= (*graph).curPositions[3].y) as u16;
    ConditionGraph_CalcLine(
        graph,
        (*graph).scanlineRight[0].as_mut_ptr(),
        &raw mut (*graph).curPositions[2],
        &raw mut (*graph).curPositions[3],
        i as u8,
        (*graph).scanlineLeft[0].as_mut_ptr(),
    );
    for i in (CONDITION_GRAPH_TOP_Y as u16)..y {
        (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][0] = 0;
        (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][1] = 0;
    }
    i = (*graph).curPositions[0].y;
    while i <= (*graph).bottom {
        (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][0] = CONDITION_GRAPH_CENTER_X;
        i += 1;
    }
    bottom = if (*graph).bottom >= (*graph).curPositions[2].y {
        (*graph).bottom
    } else {
        (*graph).curPositions[2].y
    };
    i = bottom + 1;
    while i <= CONDITION_GRAPH_BOTTOM_Y {
        (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][0] = 0;
        (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][1] = 0;
        i += 1;
    }
    i = CONDITION_GRAPH_TOP_Y as u16;
    while i <= CONDITION_GRAPH_BOTTOM_Y {
        if (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][0] == 0
            && (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][1] != 0
        {
            (*graph).scanlineRight[i as i32 - CONDITION_GRAPH_TOP_Y][0] = CONDITION_GRAPH_CENTER_X;
        }
        i += 1;
    }
}
unsafe fn ConditionGraph_CalcLeftHalf(graph: *mut ConditionGraph) {
    let mut y: i32 = 0;
    let mut bottom: i32 = 0;
    if (*graph).curPositions[0].y < (*graph).curPositions[4].y {
        y = (*graph).curPositions[0].y as i32;
        ConditionGraph_CalcLine(
            graph,
            (*graph).scanlineLeft[0].as_mut_ptr(),
            &raw mut (*graph).curPositions[0],
            &raw mut (*graph).curPositions[4],
            0,
            null_mut(),
        );
    } else {
        y = (*graph).curPositions[4].y as i32;
        ConditionGraph_CalcLine(
            graph,
            (*graph).scanlineLeft[0].as_mut_ptr(),
            &raw mut (*graph).curPositions[4],
            &raw mut (*graph).curPositions[0],
            TRUE,
            null_mut(),
        );
    }
    ConditionGraph_CalcLine(
        graph,
        (*graph).scanlineLeft[0].as_mut_ptr(),
        &raw mut (*graph).curPositions[4],
        &raw mut (*graph).curPositions[3],
        0,
        null_mut(),
    );
    for i in CONDITION_GRAPH_TOP_Y..y {
        (*graph).scanlineLeft[i - CONDITION_GRAPH_TOP_Y][0] = 0;
        (*graph).scanlineLeft[i - CONDITION_GRAPH_TOP_Y][1] = 0;
    }
    let mut i: i32 = (*graph).curPositions[0].y as i32;
    while i <= (*graph).bottom as i32 {
        (*graph).scanlineLeft[i - CONDITION_GRAPH_TOP_Y][1] = CONDITION_GRAPH_CENTER_X;
        i += 1;
    }
    bottom = if (*graph).bottom as i32 >= (*graph).curPositions[3].y as i32 + 1 {
        (*graph).bottom as i32
    } else {
        (*graph).curPositions[3].y as i32 + 1
    };
    i = bottom;
    while i <= CONDITION_GRAPH_BOTTOM_Y as i32 {
        (*graph).scanlineLeft[i - CONDITION_GRAPH_TOP_Y][0] = 0;
        (*graph).scanlineLeft[i - CONDITION_GRAPH_TOP_Y][1] = 0;
        i += 1;
    }
    for i in 0..(CONDITION_GRAPH_HEIGHT as i32) {
        if (*graph).scanlineLeft[i][0] >= (*graph).scanlineLeft[i][1] {
            (*graph).scanlineLeft[i][1] = 0;
            (*graph).scanlineLeft[i][0] = 0;
        }
    }
}
pub unsafe fn ConditionGraph_CalcPositions(mut conditions: *mut u8, positions: *mut UCoords16) {
    let mut lineLength: u8 = sConditionToLineLength[*({
        let t2 = conditions;
        conditions = conditions.at(1);
        t2
    })];
    (*positions).x = CONDITION_GRAPH_CENTER_X;
    (*positions).y = CONDITION_GRAPH_CENTER_Y - lineLength as u16;
    let mut sinIdx: u8 = 64;
    let mut posIdx: i8 = GRAPH_COOL;
    for i in 1..CONDITION_COUNT {
        sinIdx += 51;
        if ({
            posIdx -= 1;
            posIdx
        }) < 0
        {
            posIdx = 4;
        }
        if posIdx == GRAPH_CUTE {
            sinIdx += 1;
        }
        lineLength = sConditionToLineLength[*({
            let t5 = conditions;
            conditions = conditions.at(1);
            t5
        })];
        (*positions.at(posIdx)).x = CONDITION_GRAPH_CENTER_X
            + ((lineLength as i32
                * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                    [64 + sinIdx as i32] as i32)
                >> 8) as u16;
        (*positions.at(posIdx)).y = CONDITION_GRAPH_CENTER_Y
            - ((lineLength as i32
                * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[sinIdx] as i32)
                >> 8) as u16;
        if posIdx <= GRAPH_CUTE && (lineLength != 32 || posIdx != GRAPH_CUTE) {
            (*positions.at(posIdx)).x += 1;
        }
    }
}
pub unsafe fn InitMoveRelearnerWindows(useContestWindow: u8) {
    InitWindows(sMoveRelearnerWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 1, 224);
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        240,
        32,
    );
    for i in 0..5u8 {
        FillWindowPixelBuffer(i, 17);
    }
    if useContestWindow == 0 {
        PutWindowTilemap(RELEARNERWIN_DESC_BATTLE);
        DrawStdFrameWithCustomTileAndPalette(RELEARNERWIN_DESC_BATTLE, FALSE, 0x1, 0xE);
    } else {
        PutWindowTilemap(RELEARNERWIN_DESC_CONTEST);
        DrawStdFrameWithCustomTileAndPalette(RELEARNERWIN_DESC_CONTEST, FALSE, 1, 0xE);
    }
    PutWindowTilemap(RELEARNERWIN_MOVE_LIST);
    PutWindowTilemap(RELEARNERWIN_MSG);
    DrawStdFrameWithCustomTileAndPalette(RELEARNERWIN_MOVE_LIST, FALSE, 1, 0xE);
    DrawStdFrameWithCustomTileAndPalette(RELEARNERWIN_MSG, FALSE, 1, 0xE);
    MoveRelearnerDummy();
    ScheduleBgCopyTilemapToVram(1);
}
fn MoveRelearnerDummy() {}
pub unsafe fn LoadMoveRelearnerMovesList(items: *mut ListMenuItem, numChoices: u16) -> u8 {
    gMultiuseListMenuTemplate = *sMoveRelearnerMovesListTemplate;
    gMultiuseListMenuTemplate.totalItems = numChoices;
    gMultiuseListMenuTemplate.items = items;
    if numChoices < 6 {
        gMultiuseListMenuTemplate.maxShowed = numChoices;
    } else {
        gMultiuseListMenuTemplate.maxShowed = 6;
    }
    gMultiuseListMenuTemplate.maxShowed as u8
}
unsafe fn MoveRelearnerLoadBattleMoveDescription(chosenMove: u32) {
    let mut buffer: CArray<u8, 32> = zeroed();
    FillWindowPixelBuffer(RELEARNERWIN_DESC_BATTLE, 17);
    let mut str: *mut u8 = (*(&raw const crate::data::strings::gText_MoveRelearnerBattleMoves)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    let mut x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, str, 128);
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_MoveRelearnerPP).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        4,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_MoveRelearnerPower).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 106);
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_MoveRelearnerAccuracy).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 106);
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        x as u8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    if chosenMove == LIST_CANCEL as u32 {
        CopyWindowToVram(RELEARNERWIN_DESC_BATTLE, COPYWIN_GFX);
        return;
    }
    let r#move: *mut BattleMove = (&raw const (*(&raw const crate::data::pokemon::gBattleMoves)
        .cast::<CArray<BattleMove, 0>>())[chosenMove])
        .cast_mut();
    str = (*(&raw const crate::data::battle_main::gTypeNames).cast::<CArray<CArray<u8, 7>, 18>>())
        [(*r#move).r#type]
        .as_ptr()
        .cast_mut();
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        4,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    x = 4 + GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MoveRelearnerPP).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    ConvertIntToDecimalStringN(
        buffer.as_mut_ptr(),
        (*r#move).pp as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        buffer.as_mut_ptr(),
        x as u8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    if (*r#move).power < 2 {
        str = (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        ConvertIntToDecimalStringN(
            buffer.as_mut_ptr(),
            (*r#move).power as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        str = buffer.as_mut_ptr();
    }
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        106,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    if (*r#move).accuracy == 0 {
        str = (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        ConvertIntToDecimalStringN(
            buffer.as_mut_ptr(),
            (*r#move).accuracy as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        str = buffer.as_mut_ptr();
    }
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_BATTLE,
        FONT_NORMAL,
        str,
        106,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::pokemon_summary_screen::gMoveDescriptionPointers)
        .cast::<CArray<*mut u8, 0>>())[chosenMove - 1];
    AddTextPrinterParameterized(RELEARNERWIN_DESC_BATTLE, FONT_NARROW, str, 0, 65, 0, None);
}
unsafe fn MoveRelearnerMenuLoadContestMoveDescription(chosenMove: u32) {
    MoveRelearnerShowHideHearts(chosenMove as i32);
    FillWindowPixelBuffer(RELEARNERWIN_DESC_CONTEST, 17);
    let mut str: *mut u8 =
        (*(&raw const crate::data::strings::gText_MoveRelearnerContestMovesTitle)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    let mut x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, str, 128);
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_CONTEST,
        FONT_NORMAL,
        str,
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_MoveRelearnerAppeal).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 92);
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_CONTEST,
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_MoveRelearnerJam).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 92);
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_CONTEST,
        FONT_NORMAL,
        str,
        x as u8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    if chosenMove == MENU_NOTHING_CHOSEN as u32 {
        CopyWindowToVram(RELEARNERWIN_DESC_CONTEST, COPYWIN_GFX);
        return;
    }
    let r#move: *mut ContestMove =
        (&raw const (*(&raw const crate::data::contest_effect::gContestMoves)
            .cast::<CArray<ContestMove, 0>>())[chosenMove])
            .cast_mut();
    str = (*(&raw const crate::data::contest::gContestMoveTypeTextPointers)
        .cast::<CArray<*mut u8, 0>>())[(*r#move).contestCategory()];
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_CONTEST,
        FONT_NORMAL,
        str,
        4,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::contest::gContestEffectDescriptionPointers)
        .cast::<CArray<*mut u8, 0>>())[(*r#move).effect];
    AddTextPrinterParameterized(
        RELEARNERWIN_DESC_CONTEST,
        FONT_NARROW,
        str,
        0,
        65,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(RELEARNERWIN_DESC_CONTEST, COPYWIN_GFX);
}
pub(crate) unsafe fn MoveRelearnerCursorCallback(itemIndex: i32, onInit: u8, list: *mut ListMenu) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
    }
    MoveRelearnerLoadBattleMoveDescription(itemIndex as u32);
    MoveRelearnerMenuLoadContestMoveDescription(itemIndex as u32);
}
pub unsafe fn MoveRelearnerPrintMessage(str: *mut u8) {
    FillWindowPixelBuffer(RELEARNERWIN_MSG, 17);
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(TRUE);
    let speed: u8 = GetPlayerTextSpeedDelay();
    AddTextPrinterParameterized2(
        RELEARNERWIN_MSG,
        FONT_NORMAL,
        str,
        speed,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        3,
    );
}
pub unsafe fn MoveRelearnerRunTextPrinters() -> u16 {
    RunTextPrinters();
    IsTextPrinterActive(RELEARNERWIN_MSG)
}
pub unsafe fn MoveRelearnerCreateYesNoMenu() {
    CreateYesNoMenu(
        (&raw const *sMoveRelearnerYesNoMenuTemplate).cast_mut(),
        1,
        0xE,
        0,
    );
}
pub unsafe fn GetBoxOrPartyMonData(boxId: u16, monId: u16, request: i32, dst: *mut u8) -> i32 {
    let mut ret: i32 = 0;
    if boxId == TOTAL_BOXES_COUNT as u16 {
        if request == MON_DATA_NICKNAME || request == MON_DATA_OT_NAME {
            ret = GetMonData3(&raw mut gPlayerParty[monId], request, dst) as i32;
        } else {
            ret = GetMonData2(&raw mut gPlayerParty[monId], request) as i32;
        }
    } else {
        if request == MON_DATA_NICKNAME || request == MON_DATA_OT_NAME {
            ret = GetAndCopyBoxMonDataAt(boxId as u8, monId as u8, request, dst as *mut c_void)
                as i32;
        } else {
            ret = GetBoxMonDataAt(boxId as u8, monId as u8, request) as i32;
        }
    }
    ret
}
unsafe fn GetConditionMenuMonString(mut dst: *mut u8, boxId: u16, monId: u16) -> *mut u8 {
    let mut level: u16 = 0;
    let mut gender: u16 = 0;
    let mut boxMon: *mut BoxPokemon = null_mut();
    let r#box: u16 = boxId;
    let mon: u16 = monId;
    *({
        let t1 = dst;
        dst = dst.at(1);
        t1
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t2 = dst;
        dst = dst.at(1);
        t2
    }) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
    *({
        let t3 = dst;
        dst = dst.at(1);
        t3
    }) = TEXT_COLOR_BLUE;
    *({
        let t4 = dst;
        dst = dst.at(1);
        t4
    }) = TEXT_COLOR_TRANSPARENT;
    *({
        let t5 = dst;
        dst = dst.at(1);
        t5
    }) = TEXT_COLOR_LIGHT_BLUE;
    if GetBoxOrPartyMonData(r#box, mon, MON_DATA_IS_EGG, null_mut()) != 0 {
        return StringCopyPadded(
            dst,
            (*(&raw const crate::data::strings::gText_EggNickname).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            12,
        );
    }
    GetBoxOrPartyMonData(r#box, mon, MON_DATA_NICKNAME, dst);
    StringGet_Nickname(dst);
    let species: u16 = GetBoxOrPartyMonData(r#box, mon, MON_DATA_SPECIES, null_mut()) as u16;
    if r#box == TOTAL_BOXES_COUNT as u16 {
        level = GetMonData2(&raw mut gPlayerParty[mon], MON_DATA_LEVEL) as u16;
        gender = GetMonGender(&raw mut gPlayerParty[mon]) as u16;
    } else {
        boxMon = GetBoxedMonPtr(r#box as u8, mon as u8);
        gender = GetBoxMonGender(boxMon) as u16;
        level = GetLevelFromBoxMonExp(boxMon) as u16;
    }
    if (species == SPECIES_NIDORAN_F || species == SPECIES_NIDORAN_M)
        && StringCompare(
            dst,
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        ) == 0
    {
        gender = MON_GENDERLESS as u16;
    }
    let mut str: *mut u8 = dst;
    while *str != EOS {
        str = str.at(1);
    }
    *({
        let t6 = str;
        str = str.at(1);
        t6
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t7 = str;
        str = str.at(1);
        t7
    }) = EXT_CTRL_CODE_SKIP_TO;
    *({
        let t8 = str;
        str = str.at(1);
        t8
    }) = 60;
    match gender {
        0 => {
            *({
                let t10 = str;
                str = str.at(1);
                t10
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t11 = str;
                str = str.at(1);
                t11
            }) = EXT_CTRL_CODE_COLOR;
            *({
                let t12 = str;
                str = str.at(1);
                t12
            }) = TEXT_COLOR_RED;
            *({
                let t13 = str;
                str = str.at(1);
                t13
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t14 = str;
                str = str.at(1);
                t14
            }) = EXT_CTRL_CODE_SHADOW;
            *({
                let t15 = str;
                str = str.at(1);
                t15
            }) = TEXT_COLOR_LIGHT_RED;
            *({
                let t16 = str;
                str = str.at(1);
                t16
            }) = CHAR_MALE;
        }
        254 => {
            *({
                let t17 = str;
                str = str.at(1);
                t17
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t18 = str;
                str = str.at(1);
                t18
            }) = EXT_CTRL_CODE_COLOR;
            *({
                let t19 = str;
                str = str.at(1);
                t19
            }) = TEXT_COLOR_GREEN;
            *({
                let t20 = str;
                str = str.at(1);
                t20
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t21 = str;
                str = str.at(1);
                t21
            }) = EXT_CTRL_CODE_SHADOW;
            *({
                let t22 = str;
                str = str.at(1);
                t22
            }) = TEXT_COLOR_LIGHT_GREEN;
            *({
                let t23 = str;
                str = str.at(1);
                t23
            }) = CHAR_FEMALE;
        }
        _ => {
            *({
                let t9 = str;
                str = str.at(1);
                t9
            }) = CHAR_SPACE;
        }
    }
    *({
        let t24 = str;
        str = str.at(1);
        t24
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t25 = str;
        str = str.at(1);
        t25
    }) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
    *({
        let t26 = str;
        str = str.at(1);
        t26
    }) = TEXT_COLOR_BLUE;
    *({
        let t27 = str;
        str = str.at(1);
        t27
    }) = TEXT_COLOR_TRANSPARENT;
    *({
        let t28 = str;
        str = str.at(1);
        t28
    }) = TEXT_COLOR_LIGHT_BLUE;
    *({
        let t29 = str;
        str = str.at(1);
        t29
    }) = CHAR_SLASH;
    *({
        let t30 = str;
        str = str.at(1);
        t30
    }) = CHAR_EXTRA_SYMBOL;
    *({
        let t31 = str;
        str = str.at(1);
        t31
    }) = CHAR_LV_2;
    str = ConvertIntToDecimalStringN(str, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    *({
        let t32 = str;
        str = str.at(1);
        t32
    }) = CHAR_SPACE;
    *str = EOS;
    str
}
unsafe fn BufferConditionMenuSpacedStringN(
    mut dst: *mut u8,
    mut src: *mut u8,
    mut n: i16,
) -> *mut u8 {
    while *src != EOS {
        *({
            let t1 = dst;
            dst = dst.at(1);
            t1
        }) = *({
            let t3 = src;
            src = src.at(1);
            t3
        });
        n -= 1;
    }
    while ({
        let t4 = n;
        n -= 1;
        t4
    }) > 0
    {
        *({
            let t5 = dst;
            dst = dst.at(1);
            t5
        }) = CHAR_SPACE;
    }
    *dst = EOS;
    dst
}
pub unsafe fn GetConditionMenuMonNameAndLocString(
    locationDst: *mut u8,
    nameDst: *mut u8,
    boxId: u16,
    monId: u16,
    partyId: u16,
    mut numMons: u16,
    excludesCancel: u8,
) {
    let mut i: u16 = 0;
    let r#box: u16 = boxId;
    let mon: u16 = monId;
    if excludesCancel == 0 {
        numMons -= 1;
    }
    if partyId != numMons {
        GetConditionMenuMonString(nameDst, r#box, mon);
        *locationDst = EXT_CTRL_CODE_BEGIN;
        *locationDst.at(1) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
        *locationDst.at(2) = TEXT_COLOR_BLUE;
        *locationDst.at(3) = TEXT_COLOR_TRANSPARENT;
        *locationDst.at(4) = TEXT_COLOR_LIGHT_BLUE;
        if r#box == TOTAL_BOXES_COUNT as u16 {
            BufferConditionMenuSpacedStringN(
                locationDst.at(5),
                (*(&raw const crate::data::strings::gText_InParty).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                BOX_NAME_LENGTH,
            );
        } else {
            BufferConditionMenuSpacedStringN(
                locationDst.at(5),
                GetBoxNamePtr(r#box as u8),
                BOX_NAME_LENGTH,
            );
        }
    } else {
        i = 0;
        while i < 12 {
            *nameDst.at(i) = CHAR_SPACE;
            i += 1;
        }
        *nameDst.at(i) = EOS;
        i = 0;
        while i < BOX_NAME_LENGTH as u16 {
            *locationDst.at(i) = CHAR_SPACE;
            i += 1;
        }
        *locationDst.at(i) = EOS;
    }
}
pub unsafe fn GetConditionMenuMonConditions(
    graph: *mut ConditionGraph,
    numSparkles: *mut u8,
    boxId: u16,
    monId: u16,
    partyId: u16,
    id: u16,
    mut numMons: u16,
    excludesCancel: u8,
) {
    if excludesCancel == 0 {
        numMons -= 1;
    }
    if partyId != numMons {
        (*graph).conditions[id][0] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_COOL, null_mut()) as u8;
        (*graph).conditions[id][1] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_TOUGH, null_mut()) as u8;
        (*graph).conditions[id][2] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_SMART, null_mut()) as u8;
        (*graph).conditions[id][3] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_CUTE, null_mut()) as u8;
        (*graph).conditions[id][4] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_BEAUTY, null_mut()) as u8;
        *numSparkles.at(id) =
            (if GetBoxOrPartyMonData(boxId, monId, MON_DATA_SHEEN, null_mut()) != 255 {
                GetBoxOrPartyMonData(boxId, monId, MON_DATA_SHEEN, null_mut()) / 29
            } else {
                9
            }) as u8;
        ConditionGraph_CalcPositions(
            (*graph).conditions[id].as_mut_ptr(),
            (*graph).savedPositions[id].as_mut_ptr(),
        );
    } else {
        for i in 0..CONDITION_COUNT {
            (*graph).conditions[id][i] = 0;
            (*graph).savedPositions[id][i].x = CONDITION_GRAPH_CENTER_X;
            (*graph).savedPositions[id][i].y = CONDITION_GRAPH_CENTER_Y;
        }
    }
}
pub unsafe fn GetConditionMenuMonGfx(
    tilesDst: *mut c_void,
    palDst: *mut c_void,
    boxId: u16,
    monId: u16,
    partyId: u16,
    mut numMons: u16,
    excludesCancel: u8,
) {
    if excludesCancel == 0 {
        numMons -= 1;
    }
    if partyId != numMons {
        let species: u16 =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
        let trainerId: u32 = GetBoxOrPartyMonData(boxId, monId, MON_DATA_OT_ID, null_mut()) as u32;
        let personality: u32 =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_PERSONALITY, null_mut()) as u32;
        LoadSpecialPokePic(
            (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable).cast::<CArray<
                CompressedSpriteSheet,
                0,
            >>(
            ))[species])
                .cast_mut(),
            tilesDst,
            species as i32,
            personality,
            TRUE,
        );
        LZ77UnCompWram(
            GetMonSpritePalFromSpeciesAndPersonality(species, trainerId, personality),
            palDst,
        );
    }
}
pub unsafe fn MoveConditionMonOnscreen(x: *mut i16) -> u8 {
    *x += 24;
    if *x > 0 {
        *x = 0;
    }
    (*x != 0) as u8
}
pub unsafe fn MoveConditionMonOffscreen(x: *mut i16) -> u8 {
    *x -= 24;
    if *x < -80 {
        *x = -80;
    }
    (*x != -80) as u8
}
pub unsafe fn ConditionMenu_UpdateMonEnter(graph: *mut ConditionGraph, x: *mut i16) -> u8 {
    let graphUpdating: u8 = ConditionGraph_TryUpdate(graph);
    let monUpdating: u8 = MoveConditionMonOnscreen(x);
    (graphUpdating != 0 || monUpdating != 0) as u8
}
pub unsafe fn ConditionMenu_UpdateMonExit(graph: *mut ConditionGraph, x: *mut i16) -> u8 {
    let graphUpdating: u8 = ConditionGraph_TryUpdate(graph);
    let monUpdating: u8 = MoveConditionMonOffscreen(x);
    (graphUpdating != 0 || monUpdating != 0) as u8
}
pub unsafe fn LoadConditionMonPicTemplate(
    sheet: *mut SpriteSheet,
    template: *mut SpriteTemplate,
    pal: *mut SpritePalette,
) {
    let mut dataSheet: SpriteSheet = zeroed();
    dataSheet.data = null_mut();
    dataSheet.size = MON_PIC_SIZE;
    dataSheet.tag = TAG_CONDITION_MON;
    let mut dataTemplate: SpriteTemplate = zeroed();
    dataTemplate.tileTag = TAG_CONDITION_MON;
    dataTemplate.paletteTag = TAG_CONDITION_MON;
    dataTemplate.oam = (&raw const *sOam_ConditionMonPic).cast_mut();
    dataTemplate.anims = (*(&raw const crate::sprite::gDummySpriteAnimTable)
        .cast::<CArray<*mut AnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    dataTemplate.images = null_mut();
    dataTemplate.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    dataTemplate.callback = Some(SpriteCallbackDummy);
    let mut dataPal: SpritePalette = zeroed();
    dataPal.data = null_mut();
    dataPal.tag = TAG_CONDITION_MON;
    *sheet = dataSheet;
    *template = dataTemplate;
    *pal = dataPal;
}
pub unsafe fn LoadConditionSelectionIcons(
    mut sheets: *mut SpriteSheet,
    template: *mut SpriteTemplate,
    mut pals: *mut SpritePalette,
) {
    let mut dataSheets: CArray<SpriteSheet, 4> = zeroed();
    dataSheets[0].data = sConditionPokeball_Gfx.as_ptr().cast_mut() as *mut c_void;
    dataSheets[0].size = 0x100;
    dataSheets[0].tag = TAG_CONDITION_BALL;
    dataSheets[1].data = sConditionPokeballPlaceholder_Gfx.as_ptr().cast_mut() as *mut c_void;
    dataSheets[1].size = 0x20;
    dataSheets[1].tag = TAG_CONDITION_BALL_PLACEHOLDER;
    dataSheets[2].data = (*(&raw const crate::data::graphics::gPokenavConditionCancel_Gfx)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut() as *mut c_void;
    dataSheets[2].size = 0x100;
    dataSheets[2].tag = TAG_CONDITION_CANCEL;
    let mut dataPals: CArray<SpritePalette, 3> = zeroed();
    dataPals[0].data = (*(&raw const crate::data::graphics::gPokenavConditionCancel_Pal)
        .cast::<CArray<u16, 0>>())
    .as_ptr()
    .cast_mut();
    dataPals[0].tag = TAG_CONDITION_BALL;
    dataPals[1].data = (*(&raw const crate::data::graphics::gPokenavConditionCancel_Pal)
        .cast::<CArray<u16, 0>>())
    .as_ptr()
    .cast_mut()
    .at(16);
    dataPals[1].tag = TAG_CONDITION_CANCEL;
    let mut dataTemplate: SpriteTemplate = zeroed();
    dataTemplate.tileTag = TAG_CONDITION_BALL;
    dataTemplate.paletteTag = TAG_CONDITION_BALL;
    dataTemplate.oam = (&raw const *sOam_ConditionSelectionIcon).cast_mut();
    dataTemplate.anims = sAnims_ConditionSelectionIcon.as_ptr().cast_mut();
    dataTemplate.images = null_mut();
    dataTemplate.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    dataTemplate.callback = Some(SpriteCallbackDummy);
    for i in 0..4u8 {
        *({
            let t1 = sheets;
            sheets = sheets.at(1);
            t1
        }) = dataSheets[i];
    }
    *template = dataTemplate;
    for i in 0..3u8 {
        *({
            let t2 = pals;
            pals = pals.at(1);
            t2
        }) = dataPals[i];
    }
}
pub unsafe fn LoadConditionSparkle(sheet: *mut SpriteSheet, pal: *mut SpritePalette) {
    let mut dataSheet: SpriteSheet = zeroed();
    dataSheet.data = sConditionSparkle_Pal.as_ptr().cast_mut() as *mut c_void;
    dataSheet.size = 0x380;
    dataSheet.tag = TAG_CONDITION_SPARKLE;
    let mut dataPal: SpritePalette = zeroed();
    dataPal.data = sConditionSparkle_Gfx.as_ptr().cast_mut();
    dataPal.tag = TAG_CONDITION_SPARKLE;
    *sheet = dataSheet;
    *pal = dataPal;
}
pub(crate) unsafe fn SpriteCB_ConditionSparkle_DoNextAfterDelay(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sDelayTimer] += 1;
        (*sprite).data[sDelayTimer]
    }) > 60
    {
        (*sprite).data[sDelayTimer] = 0;
        SetNextConditionSparkle(sprite);
    }
}
pub(crate) unsafe fn SpriteCB_ConditionSparkle_WaitForAllAnim(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).data[sDelayTimer] = 0;
        (*sprite).callback = Some(SpriteCB_ConditionSparkle_DoNextAfterDelay);
    }
}
unsafe fn SetConditionSparklePosition(sprite: *mut Sprite) {
    let mon: *mut Sprite = &raw mut gSprites[(*sprite).data[sMonSpriteId]];
    if !mon.is_null() {
        (*sprite).x = (*mon).x + (*mon).x2 + sConditionSparkleCoords[(*sprite).data[sSparkleId]][0];
        (*sprite).y = (*mon).y + (*mon).y2 + sConditionSparkleCoords[(*sprite).data[sSparkleId]][1];
    } else {
        (*sprite).x = sConditionSparkleCoords[(*sprite).data[sSparkleId]][0] + 40;
        (*sprite).y = sConditionSparkleCoords[(*sprite).data[sSparkleId]][1] + 104;
    }
}
unsafe fn InitConditionSparkles(count: u8, allowFirstShowAll: u8, sprites: *mut *mut Sprite) {
    for i in 0..MAX_CONDITION_SPARKLES {
        if !(*sprites.at(i)).is_null() {
            (*(*sprites.at(i))).data[sSparkleId] = i as i16;
            (*(*sprites.at(i))).data[sDelayTimer] = i as i16 * 16 + 1;
            (*(*sprites.at(i))).data[sNumExtraSparkles] = count as i16;
            (*(*sprites.at(i))).data[sCurSparkleId] = i as i16;
            if allowFirstShowAll == 0 || count != 9 {
                (*(*sprites.at(i))).callback = Some(SpriteCB_ConditionSparkle);
            } else {
                SetConditionSparklePosition(*sprites.at(i));
                ShowAllConditionSparkles(*sprites.at(i));
                (*(*sprites.at(i))).callback = Some(SpriteCB_ConditionSparkle_WaitForAllAnim);
                (*(*sprites.at(i))).set_invisible(FALSE as u16);
            }
        }
    }
}
unsafe fn SetNextConditionSparkle(sprite: *mut Sprite) {
    let mut id: u8 = (*sprite).data[sNextSparkleSpriteId] as u8;
    let mut i: u16 = 0;
    while (i as i32) < (*sprite).data[sNumExtraSparkles] as i32 + 1 {
        gSprites[id].data[sDelayTimer] = gSprites[id].data[sSparkleId] * 16 + 1;
        gSprites[id].callback = Some(SpriteCB_ConditionSparkle);
        id = gSprites[id].data[sNextSparkleSpriteId] as u8;
        i += 1;
    }
}
pub unsafe fn ResetConditionSparkleSprites(sprites: *mut *mut Sprite) {
    for i in 0..(MAX_CONDITION_SPARKLES as u8) {
        *sprites.at(i) = null_mut();
    }
}
pub unsafe fn CreateConditionSparkleSprites(
    sprites: *mut *mut Sprite,
    monSpriteId: u8,
    _count: u8,
) {
    let mut spriteId: u16 = 0;
    let mut firstSpriteId: u16 = 0;
    let count: u8 = _count;
    let mut i: u16 = 0;
    while (i as i32) < count as i32 + 1 {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_ConditionSparkle).cast_mut(),
            0,
            0,
            0,
        ) as u16;
        if spriteId != MAX_SPRITES as u16 {
            *sprites.at(i) = &raw mut gSprites[spriteId];
            (*(*sprites.at(i))).set_invisible(TRUE as u16);
            (*(*sprites.at(i))).data[sMonSpriteId] = monSpriteId as i16;
            if i != 0 {
                (*(*sprites.at(i as i32 - 1))).data[sNextSparkleSpriteId] = spriteId as i16;
            } else {
                firstSpriteId = spriteId;
            }
        } else {
            break;
        }
        i += 1;
    }
    (*(*sprites.at(count))).data[sNextSparkleSpriteId] = firstSpriteId as i16;
    InitConditionSparkles(count, TRUE, sprites);
}
pub unsafe fn DestroyConditionSparkleSprites(sprites: *mut *mut Sprite) {
    for i in 0..MAX_CONDITION_SPARKLES {
        if !(*sprites.at(i)).is_null() {
            DestroySprite(*sprites.at(i));
            *sprites.at(i) = null_mut();
        } else {
            break;
        }
    }
}
pub unsafe fn FreeConditionSparkles(sprites: *mut *mut Sprite) {
    DestroyConditionSparkleSprites(sprites);
    FreeSpriteTilesByTag(TAG_CONDITION_SPARKLE);
    FreeSpritePaletteByTag(TAG_CONDITION_SPARKLE);
}
pub(crate) unsafe fn SpriteCB_ConditionSparkle(sprite: *mut Sprite) {
    if (*sprite).data[sDelayTimer] != 0 {
        if ({
            (*sprite).data[sDelayTimer] -= 1;
            (*sprite).data[sDelayTimer]
        }) != 0
        {
            return;
        }
        SeekSpriteAnim(sprite, 0);
        (*sprite).set_invisible(FALSE as u16);
    }
    SetConditionSparklePosition(sprite);
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
        if (*sprite).data[sCurSparkleId] == (*sprite).data[sNumExtraSparkles] {
            if (*sprite).data[sCurSparkleId] == 9 {
                ShowAllConditionSparkles(sprite);
                (*sprite).callback = Some(SpriteCB_ConditionSparkle_WaitForAllAnim);
            } else {
                (*sprite).callback = Some(SpriteCB_ConditionSparkle_DoNextAfterDelay);
            }
        } else {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
unsafe fn ShowAllConditionSparkles(sprite: *mut Sprite) {
    let mut id: u8 = (*sprite).data[sNextSparkleSpriteId] as u8;
    let mut i: u8 = 0;
    while (i as i32) < (*sprite).data[sNumExtraSparkles] as i32 + 1 {
        SeekSpriteAnim(&raw mut gSprites[id], 0);
        gSprites[id].set_invisible(FALSE as u16);
        id = gSprites[id].data[sNextSparkleSpriteId] as u8;
        i += 1;
    }
}
pub unsafe fn DrawLevelUpWindowPg1(
    windowId: u16,
    statsBefore: *mut u16,
    statsAfter: *mut u16,
    bgClr: u8,
    fgClr: u8,
    shadowClr: u8,
) {
    let mut x: u16 = 0;
    let mut statsDiff: CArray<i16, 6> = zeroed();
    let mut text: CArray<u8, 12> = zeroed();
    let mut color: CArray<u8, 3> = zeroed();
    FillWindowPixelBuffer(windowId as u8, bgClr | bgClr << 4);
    statsDiff[0] = *statsAfter as i16 - *statsBefore as i16;
    statsDiff[1] = *statsAfter.at(1) as i16 - *statsBefore.at(1) as i16;
    statsDiff[2] = *statsAfter.at(2) as i16 - *statsBefore.at(2) as i16;
    statsDiff[3] = *statsAfter.at(4) as i16 - *statsBefore.at(4) as i16;
    statsDiff[4] = *statsAfter.at(5) as i16 - *statsBefore.at(5) as i16;
    statsDiff[5] = *statsAfter.at(3) as i16 - *statsBefore.at(3) as i16;
    color[0] = bgClr;
    color[1] = fgClr;
    color[2] = shadowClr;
    for i in 0..(NUM_STATS as u16) {
        AddTextPrinterParameterized3(
            windowId as u8,
            FONT_NORMAL,
            0,
            15 * i as u8,
            color.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            sLvlUpStatStrings[i],
        );
        StringCopy(
            text.as_mut_ptr(),
            if statsDiff[i] >= 0 {
                (*(&raw const crate::data::strings::gText_Plus).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
            } else {
                (*(&raw const crate::data::strings::gText_Dash).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
            },
        );
        AddTextPrinterParameterized3(
            windowId as u8,
            FONT_NORMAL,
            56,
            15 * i as u8,
            color.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
        if (if statsDiff[i] < 0 {
            -(statsDiff[i] as i32)
        } else {
            statsDiff[i] as i32
        }) <= 9
        {
            x = 18;
        } else {
            x = 12;
        }
        ConvertIntToDecimalStringN(
            text.as_mut_ptr(),
            if statsDiff[i] < 0 {
                -(statsDiff[i] as i32)
            } else {
                statsDiff[i] as i32
            },
            STR_CONV_MODE_LEFT_ALIGN,
            2,
        );
        AddTextPrinterParameterized3(
            windowId as u8,
            FONT_NORMAL,
            56 + x as u8,
            15 * i as u8,
            color.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
    }
}
pub unsafe fn DrawLevelUpWindowPg2(
    windowId: u16,
    currStats: *mut u16,
    bgClr: u8,
    fgClr: u8,
    shadowClr: u8,
) {
    let mut numDigits: u16 = 0;
    let mut x: u16 = 0;
    let mut stats: CArray<i16, 6> = zeroed();
    let mut text: CArray<u8, 12> = zeroed();
    let mut color: CArray<u8, 3> = zeroed();
    FillWindowPixelBuffer(windowId as u8, bgClr | bgClr << 4);
    stats[0] = *currStats as i16;
    stats[1] = *currStats.at(1) as i16;
    stats[2] = *currStats.at(2) as i16;
    stats[3] = *currStats.at(4) as i16;
    stats[4] = *currStats.at(5) as i16;
    stats[5] = *currStats.at(3) as i16;
    color[0] = bgClr;
    color[1] = fgClr;
    color[2] = shadowClr;
    for i in 0..(NUM_STATS as u16) {
        if stats[i] > 99 {
            numDigits = 3;
        } else if stats[i] > 9 {
            numDigits = 2;
        } else {
            numDigits = 1;
        }
        ConvertIntToDecimalStringN(
            text.as_mut_ptr(),
            stats[i] as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            numDigits as u8,
        );
        x = 6 * (4 - numDigits);
        AddTextPrinterParameterized3(
            windowId as u8,
            FONT_NORMAL,
            0,
            15 * i as u8,
            color.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            sLvlUpStatStrings[i],
        );
        AddTextPrinterParameterized3(
            windowId as u8,
            FONT_NORMAL,
            56 + x as u8,
            15 * i as u8,
            color.as_mut_ptr(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
    }
}
pub unsafe fn GetMonLevelUpWindowStats(mon: *mut Pokemon, currStats: *mut u16) {
    *currStats = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
    *currStats.at(1) = GetMonData2(mon, MON_DATA_ATK) as u16;
    *currStats.at(2) = GetMonData2(mon, MON_DATA_DEF) as u16;
    *currStats.at(3) = GetMonData2(mon, MON_DATA_SPEED) as u16;
    *currStats.at(4) = GetMonData2(mon, MON_DATA_SPATK) as u16;
    *currStats.at(5) = GetMonData2(mon, MON_DATA_SPDEF) as u16;
}
