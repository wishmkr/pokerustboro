//! Translated from `src/pokeblock.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gPokeblockFlavorCompatibilityTable sBgTemplatesForPokeblockMenu gPokeblockNames sPokeblockMenuActions sActionsOnField sActionsInBattle sActionsOnPokeblockFeeder sActionsWhenGivingToLady sTossYesNoFuncTable sContestStatsMonData sOamData_PokeblockCase sSpriteAnim_PokeblockCase sSpriteAnimTable_PokeblockCase sAffineAnim_PokeblockCaseShake sAffineAnims_PokeblockCaseShake gPokeblockCase_SpriteSheet gPokeblockCase_SpritePal sSpriteTemplate_PokeblockCase sTextColor sFavoritePokeblocksTable sWindowTemplates sTossPkblockWindowTemplate sPokeblockListMenuTemplate

/// `struct PokeblockSavedData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokeblockSavedData {
    pub callback: Option<unsafe extern "C" fn()>,
    pub selectedRow: u16,
    pub scrollOffset: u16,
}

unsafe impl Sync for PokeblockSavedData {}

/// `struct PokeblockMenuStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokeblockMenuStruct {
    pub tilemap: CArray<u8, 2048>,
    pub callbackOnUse: Option<unsafe extern "C" fn()>,
    pub pokeblockActionIds: *mut u8,
    pub numActions: u8,
    pub caseId: u8,
    pub itemsNo: u8,
    pub maxShowed: u8,
    pub items: CArray<ListMenuItem, 41>,
    pub menuItemsStrings: CArray<CArray<u8, 32>, 41>,
    pub pokeblockCaseSpriteId: u8,
    pub swapLineSpriteIds: CArray<u8, 7>,
    pub arrowTaskId: u8,
    pub isSwapping: u8,
    pub gfxState: i16,
    pub unused: CArray<u8, 8>,
}

unsafe impl Sync for PokeblockMenuStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokeblockSavedData>() == 8);
    assert!(offset_of!(PokeblockSavedData, callback) == 0);
    assert!(offset_of!(PokeblockSavedData, selectedRow) == 4);
    assert!(offset_of!(PokeblockSavedData, scrollOffset) == 6);
    assert!(size_of::<PokeblockMenuStruct>() == 3720);
    assert!(offset_of!(PokeblockMenuStruct, tilemap) == 0);
    assert!(offset_of!(PokeblockMenuStruct, callbackOnUse) == 2048);
    assert!(offset_of!(PokeblockMenuStruct, pokeblockActionIds) == 2052);
    assert!(offset_of!(PokeblockMenuStruct, numActions) == 2056);
    assert!(offset_of!(PokeblockMenuStruct, caseId) == 2057);
    assert!(offset_of!(PokeblockMenuStruct, itemsNo) == 2058);
    assert!(offset_of!(PokeblockMenuStruct, maxShowed) == 2059);
    assert!(offset_of!(PokeblockMenuStruct, items) == 2060);
    assert!(offset_of!(PokeblockMenuStruct, menuItemsStrings) == 2388);
    assert!(offset_of!(PokeblockMenuStruct, pokeblockCaseSpriteId) == 3700);
    assert!(offset_of!(PokeblockMenuStruct, swapLineSpriteIds) == 3701);
    assert!(offset_of!(PokeblockMenuStruct, arrowTaskId) == 3708);
    assert!(offset_of!(PokeblockMenuStruct, isSwapping) == 3709);
    assert!(offset_of!(PokeblockMenuStruct, gfxState) == 3710);
    assert!(offset_of!(PokeblockMenuStruct, unused) == 3712);
};

const MAX_MENU_ITEMS: u8 = 9;
const MENU_MIDPOINT: u16 = 4;
const POKEBLOCK_MAX_FEEL: u8 = 99;
const TAG_SCROLL_ARROW: i32 = 1110;
const TILE_HIGHLIGHT_BLUE: u16 = 4101;
const TILE_HIGHLIGHT_NONE: u16 = 5;
const TILE_HIGHLIGHT_RED: u16 = 8197;
const WIN_ACTIONS: i16 = 9;
const WIN_ACTIONS_TALL: u8 = 8;
const WIN_BITTER: u8 = 5;
const WIN_DRY: u8 = 3;
const WIN_FEEL: u8 = 7;
const WIN_SOUR: u8 = 6;
const WIN_SPICY: u8 = 2;
const WIN_SWEET: u8 = 4;
const WIN_TITLE: u8 = 0;
const WIN_TOSS_MSG: u8 = 10;

static gPokeblockCase_SpritePal: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::pokeblock::gPokeblockCase_SpritePal).cast());
static gPokeblockCase_SpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokeblock::gPokeblockCase_SpriteSheet).cast());
static gPokeblockFlavorCompatibilityTable: Table<CArray<i8, 125>> =
    Table((&raw const crate::data::pokeblock::gPokeblockFlavorCompatibilityTable).cast());
static gPokeblockNames: Table<CArray<*mut u8, 15>> =
    Table((&raw const crate::data::pokeblock::gPokeblockNames).cast());
static sActionsInBattle: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokeblock::sActionsInBattle).cast());
static sActionsOnField: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokeblock::sActionsOnField).cast());
static sActionsOnPokeblockFeeder: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokeblock::sActionsOnPokeblockFeeder).cast());
static sActionsWhenGivingToLady: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokeblock::sActionsWhenGivingToLady).cast());
static sAffineAnims_PokeblockCaseShake: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::pokeblock::sAffineAnims_PokeblockCaseShake).cast());
static sBgTemplatesForPokeblockMenu: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::pokeblock::sBgTemplatesForPokeblockMenu).cast());
static sFavoritePokeblocksTable: Table<CArray<Pokeblock, 5>> =
    Table((&raw const crate::data::pokeblock::sFavoritePokeblocksTable).cast());
static sPokeblockListMenuTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::pokeblock::sPokeblockListMenuTemplate).cast());
static sPokeblockMenuActions: Table<CArray<MenuAction, 6>> =
    Table((&raw const crate::data::pokeblock::sPokeblockMenuActions).cast());
static sSpriteTemplate_PokeblockCase: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokeblock::sSpriteTemplate_PokeblockCase).cast());
static sTextColor: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokeblock::sTextColor).cast());
static sTossPkblockWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokeblock::sTossPkblockWindowTemplate).cast());
static sTossYesNoFuncTable: Table<YesNoFuncTable> =
    Table((&raw const crate::data::pokeblock::sTossYesNoFuncTable).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 12>> =
    Table((&raw const crate::data::pokeblock::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedPokeblockData: PokeblockSavedData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeblockMenu: *mut PokeblockMenuStruct = null_mut();

unsafe extern "C" {
    static mut gBattleTextBuff1: CArray<u8, 16>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gMain: Main;
    static gMenuPokeblock_Gfx: CArray<u32, 0>;
    static gMenuPokeblock_Pal: CArray<u32, 0>;
    static gMenuPokeblock_Tilemap: CArray<u32, 0>;
    static mut gMultiuseListMenuTemplate: ListMenuTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_ItemId: u16;
    static mut gSpecialVar_Result: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static gStandardMenuPalette: CArray<u16, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Bitter: CArray<u8, 0>;
    static gText_Dry: CArray<u8, 0>;
    static gText_LvVar1: CArray<u8, 0>;
    static gText_Sour: CArray<u8, 0>;
    static gText_Spicy: CArray<u8, 0>;
    static gText_StowCase: CArray<u8, 0>;
    static gText_Sweet: CArray<u8, 0>;
    static gText_ThrowAwayVar1: CArray<u8, 0>;
    static gText_Var1ThrownAway: CArray<u8, 0>;
    fn AddScrollIndicatorArrowPairParameterized(
        a0: u32,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: *mut u16,
    ) -> u8;
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
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn CB2_SetUpReshowBattleScreenAfterMenu2();
    fn ChooseMonToGivePokeblock(a0: *mut Pokeblock, a1: Option<unsafe extern "C" fn()>);
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSwapLineSprites(a0: *mut u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenuWithCallbacks(
        a0: u8,
        a1: *mut WindowTemplate,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u16,
        a6: u8,
        a7: *mut YesNoFuncTable,
    );
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DisplayMessageAndContinueTask(
        a0: u8,
        a1: u8,
        a2: u16,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: *mut c_void,
    );
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeOamMatrix(a0: u8);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetNature(a0: *mut Pokemon) -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GivePokeblockToContestLady(a0: *mut Pokeblock) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadListMenuSwapLineGfx();
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuActionTextsInUpperLeftCorner(a0: u8, a1: u8, a2: *mut MenuAction, a3: *mut u8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn SafariZoneActivatePokeblockFeeder(a0: u8);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSwapLineSpritesInvisibility(a0: *mut u8, a1: u8, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn UpdateSwapLineSpritesPos(a0: *mut u8, a1: u8, a2: i16, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCase(caseId: u8, callback: Option<unsafe extern "C" fn()>) {
    sPokeblockMenu = Alloc(3720) as *mut PokeblockMenuStruct;
    (*sPokeblockMenu).caseId = caseId;
    (*sPokeblockMenu).callbackOnUse = None;
    (*sPokeblockMenu).arrowTaskId = TASK_NONE;
    (*sPokeblockMenu).isSwapping = FALSE;
    sSavedPokeblockData.callback = callback;
    match (*sPokeblockMenu).caseId {
        PBLOCK_CASE_BATTLE => {
            (*sPokeblockMenu).pokeblockActionIds = sActionsInBattle.as_ptr().cast_mut();
            (*sPokeblockMenu).numActions = 2;
        }
        PBLOCK_CASE_FEEDER => {
            (*sPokeblockMenu).pokeblockActionIds = sActionsOnPokeblockFeeder.as_ptr().cast_mut();
            (*sPokeblockMenu).numActions = 2;
        }
        PBLOCK_CASE_GIVE => {
            (*sPokeblockMenu).pokeblockActionIds = sActionsWhenGivingToLady.as_ptr().cast_mut();
            (*sPokeblockMenu).numActions = 2;
        }
        _ => {
            (*sPokeblockMenu).pokeblockActionIds = sActionsOnField.as_ptr().cast_mut();
            (*sPokeblockMenu).numActions = 3;
        }
    }
    SetMainCallback2(Some(CB2_InitPokeblockMenu));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCaseInBattle() {
    OpenPokeblockCase(
        PBLOCK_CASE_BATTLE,
        Some(CB2_SetUpReshowBattleScreenAfterMenu2),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCaseOnFeeder() {
    OpenPokeblockCase(PBLOCK_CASE_FEEDER, Some(CB2_ReturnToField));
}
pub(crate) unsafe extern "C" fn CB2_PokeblockMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_PokeblockMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_InitPokeblockMenu() {
    loop {
        if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
            break;
        }
        if InitPokeblockMenu() == TRUE {
            break;
        }
        if MenuHelpers_IsLinkActive() == TRUE {
            break;
        }
    }
}
pub(crate) unsafe extern "C" fn InitPokeblockMenu() -> u8 {
    let mut taskId: u8 = 0;
    match gMain.state {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            ClearScheduledBgCopiesToVram();
            gMain.state += 1;
        }
        1 => {
            ScanlineEffect_Stop();
            gMain.state += 1;
        }
        2 => {
            FreeAllSpritePalettes();
            gMain.state += 1;
        }
        3 => {
            ResetPaletteFade();
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            gMain.state += 1;
        }
        4 => {
            ResetSpriteData();
            gMain.state += 1;
        }
        5 => {
            if (*sPokeblockMenu).caseId != PBLOCK_CASE_BATTLE {
                ResetTasks();
            }
            gMain.state += 1;
        }
        6 => {
            HandleInitBackgrounds();
            (*sPokeblockMenu).gfxState = 0;
            gMain.state += 1;
        }
        7 => {
            if LoadPokeblockMenuGfx() == 0 {
                return FALSE;
            }
            gMain.state += 1;
        }
        8 => {
            SetMenuItemsCountAndMaxShowed();
            LimitMenuScrollAndRow();
            SetInitialScroll();
            gMain.state += 1;
        }
        9 => {
            (*sPokeblockMenu).pokeblockCaseSpriteId = CreatePokeblockCaseSprite(56, 64, 0);
            gMain.state += 1;
        }
        10 => {
            CreateSwapLineSprites((*sPokeblockMenu).swapLineSpriteIds.as_mut_ptr(), 7);
            gMain.state += 1;
        }
        11 => {
            DrawPokeblockMenuHighlight(sSavedPokeblockData.selectedRow, TILE_HIGHLIGHT_BLUE);
            gMain.state += 1;
        }
        12 => {
            HandleInitWindows();
            gMain.state += 1;
        }
        13 => {
            UpdatePokeblockList();
            gMain.state += 1;
        }
        14 => {
            CreateScrollArrows();
            gMain.state += 1;
        }
        15 => {
            taskId = CreateTask(Some(Task_HandlePokeblockMenuInput), 0);
            gTasks[taskId].data[0] = ListMenuInit(
                &raw mut gMultiuseListMenuTemplate,
                sSavedPokeblockData.scrollOffset,
                sSavedPokeblockData.selectedRow,
            ) as i16;
            gMain.state += 1;
        }
        16 => {
            DrawPokeblockMenuTitleText();
            gMain.state += 1;
        }
        17 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            gMain.state += 1;
        }
        18 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            gMain.state += 1;
        }
        _ => {
            SetVBlankCallback(Some(VBlankCB_PokeblockMenu));
            SetMainCallback2(Some(CB2_PokeblockMenu));
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn HandleInitBackgrounds() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplatesForPokeblockMenu.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(2, (*sPokeblockMenu).tilemap.as_mut_ptr() as *mut c_void);
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
pub(crate) unsafe extern "C" fn LoadPokeblockMenuGfx() -> u8 {
    match (*sPokeblockMenu).gfxState {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                2,
                gMenuPokeblock_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*sPokeblockMenu).gfxState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    gMenuPokeblock_Tilemap.as_ptr().cast_mut(),
                    (*sPokeblockMenu).tilemap.as_mut_ptr() as *mut c_void,
                );
                (*sPokeblockMenu).gfxState += 1;
            }
        }
        2 => {
            LoadCompressedPalette(gMenuPokeblock_Pal.as_ptr().cast_mut(), 0, 192);
            (*sPokeblockMenu).gfxState += 1;
        }
        3 => {
            LoadCompressedSpriteSheet((&raw const *gPokeblockCase_SpriteSheet).cast_mut());
            (*sPokeblockMenu).gfxState += 1;
        }
        4 => {
            LoadCompressedSpritePalette((&raw const *gPokeblockCase_SpritePal).cast_mut());
            (*sPokeblockMenu).gfxState += 1;
        }
        5 => {
            LoadListMenuSwapLineGfx();
            (*sPokeblockMenu).gfxState = 0;
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn HandleInitWindows() {
    let mut i: u8 = 0;
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 1, 224);
    LoadMessageBoxGfx(0, 0xA, 208);
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    i = 0;
    while i < 11 {
        FillWindowPixelBuffer(i, 0);
        i += 1;
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn PrintOnPokeblockWindow(windowId: u8, string: *mut u8, x: i32) {
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x as u8,
        1,
        0,
        0,
        sTextColor.as_ptr().cast_mut(),
        0,
        string,
    );
}
pub(crate) unsafe extern "C" fn DrawPokeblockMenuTitleText() {
    let mut i: u8 = 0;
    let mut itemName: *mut u8 = GetItemName(ITEM_POKEBLOCK_CASE);
    PrintOnPokeblockWindow(
        WIN_TITLE,
        itemName,
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, itemName, 0x48),
    );
    PrintOnPokeblockWindow(WIN_SPICY, gText_Spicy.as_ptr().cast_mut(), 0);
    PrintOnPokeblockWindow(WIN_DRY, gText_Dry.as_ptr().cast_mut(), 0);
    PrintOnPokeblockWindow(WIN_SWEET, gText_Sweet.as_ptr().cast_mut(), 0);
    PrintOnPokeblockWindow(WIN_BITTER, gText_Bitter.as_ptr().cast_mut(), 0);
    PrintOnPokeblockWindow(WIN_SOUR, gText_Sour.as_ptr().cast_mut(), 0);
    i = 0;
    while i < WIN_ACTIONS_TALL {
        PutWindowTilemap(i);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdatePokeblockList() {
    let mut i: u16 = 0;
    i = 0;
    while (i as i32) < (*sPokeblockMenu).itemsNo as i32 - 1 {
        PutPokeblockListMenuString((*sPokeblockMenu).menuItemsStrings[i].as_mut_ptr(), i);
        (*sPokeblockMenu).items[i].name = (*sPokeblockMenu).menuItemsStrings[i].as_mut_ptr();
        (*sPokeblockMenu).items[i].id = i as i32;
        i += 1;
    }
    StringCopy(
        (*sPokeblockMenu).menuItemsStrings[i].as_mut_ptr(),
        gText_StowCase.as_ptr().cast_mut(),
    );
    (*sPokeblockMenu).items[i].name = (*sPokeblockMenu).menuItemsStrings[i].as_mut_ptr();
    (*sPokeblockMenu).items[i].id = LIST_CANCEL;
    gMultiuseListMenuTemplate = *sPokeblockListMenuTemplate;
    gMultiuseListMenuTemplate.set_fontId(FONT_NARROW);
    gMultiuseListMenuTemplate.totalItems = (*sPokeblockMenu).itemsNo as u16;
    gMultiuseListMenuTemplate.items = (*sPokeblockMenu).items.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = (*sPokeblockMenu).maxShowed as u16;
}
pub(crate) unsafe extern "C" fn PutPokeblockListMenuString(dst: *mut u8, pkblId: u16) {
    let mut pkblock: *mut Pokeblock = &raw mut (*gSaveBlock1Ptr).pokeblocks[pkblId];
    let mut txtPtr: *mut u8 = StringCopy(dst, gPokeblockNames[(*pkblock).color]);
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t2 = txtPtr;
        txtPtr = txtPtr.at(1);
        t2
    }) = EXT_CTRL_CODE_SKIP_TO;
    *({
        let t3 = txtPtr;
        txtPtr = txtPtr.at(1);
        t3
    }) = CHAR_BLOCK_1;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetHighestPokeblocksFlavorLevel(pkblock) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringExpandPlaceholders(txtPtr, gText_LvVar1.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn MovePokeblockMenuCursor(
    pkblId: i32,
    onInit: u8,
    list: *mut ListMenu,
) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
        gSprites[(*sPokeblockMenu).pokeblockCaseSpriteId].callback =
            Some(SpriteCB_ShakePokeblockCase);
    }
    if (*sPokeblockMenu).isSwapping == 0 {
        DrawPokeblockInfo(pkblId);
    }
}
pub(crate) unsafe extern "C" fn DrawPokeblockInfo(pkblId: i32) {
    let mut i: u8 = 0;
    let mut pokeblock: *mut Pokeblock = null_mut();
    let mut rectTilemapSrc: CArray<u16, 2> = zeroed();
    FillWindowPixelBuffer(WIN_FEEL, 0);
    if pkblId != LIST_CANCEL {
        pokeblock = &raw mut (*gSaveBlock1Ptr).pokeblocks[pkblId];
        rectTilemapSrc[0] = 0x17;
        rectTilemapSrc[1] = 0x18;
        i = 0;
        while i < FLAVOR_COUNT as u8 {
            if GetPokeblockData(pokeblock, PBLOCK_SPICY + i) > 0 {
                rectTilemapSrc[0] = ((i as u16) << 12) + 0x17;
                rectTilemapSrc[1] = ((i as u16) << 12) + 0x18;
            } else {
                rectTilemapSrc[0] = 0xF;
                rectTilemapSrc[1] = 0xF;
            }
            CopyToBgTilemapBufferRect(
                2,
                rectTilemapSrc.as_mut_ptr() as *mut c_void,
                (i as i32 / 3) as u8 * 6 + 1,
                (i as i32 % 3) as u8 * 2 + 13,
                1,
                2,
            );
            i += 1;
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            GetPokeblocksFeel(pokeblock) as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        PrintOnPokeblockWindow(WIN_FEEL, gStringVar1.as_mut_ptr(), 4);
    } else {
        rectTilemapSrc[0] = 0xF;
        rectTilemapSrc[1] = 0xF;
        i = 0;
        while i < FLAVOR_COUNT as u8 {
            CopyToBgTilemapBufferRect(
                2,
                rectTilemapSrc.as_mut_ptr() as *mut c_void,
                (i as i32 / 3) as u8 * 6 + 1,
                (i as i32 % 3) as u8 * 2 + 13,
                1,
                2,
            );
            i += 1;
        }
        CopyWindowToVram(WIN_FEEL, COPYWIN_GFX);
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn DrawPokeblockMenuHighlight(cursorPos: u16, tileNum: u16) {
    FillBgTilemapBufferRect_Palette0(2, tileNum, 0xF, cursorPos as u8 * 2 + 1, 0xE, 2);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn CompactPokeblockSlots() {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while i < 39 {
        j = i + 1;
        while j < POKEBLOCKS_COUNT {
            if (*gSaveBlock1Ptr).pokeblocks[i].color == PBLOCK_CLR_NONE {
                let mut temp: Pokeblock = zeroed();
                temp = (*gSaveBlock1Ptr).pokeblocks[i];
                (*gSaveBlock1Ptr).pokeblocks[i] = (*gSaveBlock1Ptr).pokeblocks[j];
                (*gSaveBlock1Ptr).pokeblocks[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SwapPokeblockMenuItems(id1: u32, mut id2: u32) {
    let mut i: i16 = 0;
    let mut count: i16 = 0;
    let mut pokeblocks: *mut Pokeblock = (*gSaveBlock1Ptr).pokeblocks.as_mut_ptr();
    let mut copyPokeblock1: *mut Pokeblock = null_mut();
    if id1 == id2 {
        return;
    }
    copyPokeblock1 = Alloc(8) as *mut Pokeblock;
    *copyPokeblock1 = *pokeblocks.at(id1);
    if id2 > id1 {
        id2 -= 1;
        count = id2 as i16;
        i = id1 as i16;
        while i < count {
            *pokeblocks.at(i) = *pokeblocks.at(i as i32 + 1);
            i += 1;
        }
    } else {
        count = id2 as i16;
        i = id1 as i16;
        while i > count {
            *pokeblocks.at(i) = *pokeblocks.at(i as i32 - 1);
            i -= 1;
        }
    }
    *pokeblocks.at(id2) = *copyPokeblock1;
    Free(copyPokeblock1 as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokeblockScrollPositions() {
    sSavedPokeblockData.selectedRow = 0;
    sSavedPokeblockData.scrollOffset = 0;
}
pub(crate) unsafe extern "C" fn SetMenuItemsCountAndMaxShowed() {
    let mut i: u16 = 0;
    CompactPokeblockSlots();
    (*sPokeblockMenu).itemsNo = 0;
    i = 0;
    while i < POKEBLOCKS_COUNT {
        if (*gSaveBlock1Ptr).pokeblocks[i].color != PBLOCK_CLR_NONE {
            (*sPokeblockMenu).itemsNo += 1;
        }
        i += 1;
    }
    (*sPokeblockMenu).itemsNo += 1;
    if (*sPokeblockMenu).itemsNo > MAX_MENU_ITEMS {
        (*sPokeblockMenu).maxShowed = MAX_MENU_ITEMS;
    } else {
        (*sPokeblockMenu).maxShowed = (*sPokeblockMenu).itemsNo;
    }
}
pub(crate) unsafe extern "C" fn LimitMenuScrollAndRow() {
    if sSavedPokeblockData.scrollOffset != 0 {
        if sSavedPokeblockData.scrollOffset as i32 + (*sPokeblockMenu).maxShowed as i32
            > (*sPokeblockMenu).itemsNo as i32
        {
            sSavedPokeblockData.scrollOffset =
                (*sPokeblockMenu).itemsNo as u16 - (*sPokeblockMenu).maxShowed as u16;
        }
    }
    if sSavedPokeblockData.scrollOffset as i32 + sSavedPokeblockData.selectedRow as i32
        >= (*sPokeblockMenu).itemsNo as i32
    {
        if (*sPokeblockMenu).itemsNo == 0 {
            sSavedPokeblockData.selectedRow = 0;
        } else {
            sSavedPokeblockData.selectedRow = (*sPokeblockMenu).itemsNo as u16 - 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetInitialScroll() {
    if sSavedPokeblockData.selectedRow > MENU_MIDPOINT {
        let mut i: u8 = 0;
        i = 0;
        while (i as i32) < sSavedPokeblockData.selectedRow as i32 - MENU_MIDPOINT as i32
            && sSavedPokeblockData.scrollOffset as i32 + (*sPokeblockMenu).maxShowed as i32
                != (*sPokeblockMenu).itemsNo as i32
        {
            sSavedPokeblockData.selectedRow -= 1;
            sSavedPokeblockData.scrollOffset += 1;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateScrollArrows() {
    if (*sPokeblockMenu).arrowTaskId == TASK_NONE {
        (*sPokeblockMenu).arrowTaskId = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            0xB0,
            8,
            0x98,
            (*sPokeblockMenu).itemsNo as i32 - (*sPokeblockMenu).maxShowed as i32,
            TAG_SCROLL_ARROW,
            TAG_SCROLL_ARROW,
            &raw mut sSavedPokeblockData.scrollOffset,
        );
    }
}
pub(crate) unsafe extern "C" fn DestroyScrollArrows() {
    if (*sPokeblockMenu).arrowTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sPokeblockMenu).arrowTaskId);
        (*sPokeblockMenu).arrowTaskId = TASK_NONE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePokeblockCaseSprite(x: i16, y: i16, subpriority: u8) -> u8 {
    return CreateSprite(
        (&raw const *sSpriteTemplate_PokeblockCase).cast_mut(),
        x,
        y,
        subpriority,
    );
}
pub(crate) unsafe extern "C" fn SpriteCB_ShakePokeblockCase(sprite: *mut Sprite) {
    if (*sprite).data[0] > 1 {
        (*sprite).data[0] = 0;
    }
    match (*sprite).data[0] {
        0 => {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            (*sprite).affineAnims = sAffineAnims_PokeblockCaseShake.as_ptr().cast_mut();
            InitSpriteAffineAnim(sprite);
            (*sprite).data[0] = 1;
            (*sprite).data[1] = 0;
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 11
            {
                (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
                (*sprite).data[0] = 0;
                (*sprite).data[1] = 0;
                FreeOamMatrix((*sprite).oam.matrixNum() as u8);
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn FadePaletteAndSetTaskToClosePokeblockCase(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_FreeDataAndExitPokeblockCase);
}
pub(crate) unsafe extern "C" fn Task_FreeDataAndExitPokeblockCase(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        if (*sPokeblockMenu).caseId == PBLOCK_CASE_FEEDER
            || (*sPokeblockMenu).caseId == PBLOCK_CASE_GIVE
        {
            gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        }
        DestroyListMenuTask(
            *data as u8,
            &raw mut sSavedPokeblockData.scrollOffset,
            &raw mut sSavedPokeblockData.selectedRow,
        );
        DestroyScrollArrows();
        ResetSpriteData();
        FreeAllSpritePalettes();
        if (*sPokeblockMenu).callbackOnUse.is_some() {
            SetMainCallback2((*sPokeblockMenu).callbackOnUse);
        } else {
            SetMainCallback2(sSavedPokeblockData.callback);
        }
        FreeAllWindowBuffers();
        Free(sPokeblockMenu as *mut c_void);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblockMenuInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 && MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut sSavedPokeblockData.scrollOffset,
                &raw mut sSavedPokeblockData.selectedRow,
            );
            if sSavedPokeblockData.scrollOffset as i32 + sSavedPokeblockData.selectedRow as i32
                != (*sPokeblockMenu).itemsNo as i32 - 1
            {
                PlaySE(SE_SELECT);
                DrawPokeblockMenuHighlight(sSavedPokeblockData.selectedRow, TILE_HIGHLIGHT_RED);
                *data.at(2) = sSavedPokeblockData.scrollOffset as i16
                    + sSavedPokeblockData.selectedRow as i16;
                (*sPokeblockMenu).isSwapping = TRUE;
                gTasks[taskId].func = Some(Task_HandlePokeblocksSwapInput);
            }
        } else {
            let mut oldPosition: u16 = sSavedPokeblockData.selectedRow;
            let mut input: i32 = ListMenu_ProcessInput(*data as u8);
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut sSavedPokeblockData.scrollOffset,
                &raw mut sSavedPokeblockData.selectedRow,
            );
            if oldPosition != sSavedPokeblockData.selectedRow {
                DrawPokeblockMenuHighlight(oldPosition, TILE_HIGHLIGHT_NONE);
                DrawPokeblockMenuHighlight(sSavedPokeblockData.selectedRow, TILE_HIGHLIGHT_BLUE);
            }
            match input {
                LIST_NOTHING_CHOSEN => {}
                LIST_CANCEL => {
                    PlaySE(SE_SELECT);
                    gSpecialVar_Result = 0xFFFF;
                    gSpecialVar_ItemId = 0;
                    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
                }
                _ => {
                    PlaySE(SE_SELECT);
                    gSpecialVar_ItemId = input as u16;
                    ShowPokeblockActionsWindow(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblocksSwapInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
        return;
    }
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        PlaySE(SE_SELECT);
        ListMenuGetScrollAndRow(
            *data as u8,
            &raw mut sSavedPokeblockData.scrollOffset,
            &raw mut sSavedPokeblockData.selectedRow,
        );
        UpdatePokeblockSwapMenu(taskId, FALSE);
    } else {
        let mut i: u16 = sSavedPokeblockData.scrollOffset;
        let mut row: u16 = sSavedPokeblockData.selectedRow;
        let mut input: i32 = ListMenu_ProcessInput(*data as u8);
        ListMenuGetScrollAndRow(
            *data as u8,
            &raw mut sSavedPokeblockData.scrollOffset,
            &raw mut sSavedPokeblockData.selectedRow,
        );
        if i != sSavedPokeblockData.scrollOffset || row != sSavedPokeblockData.selectedRow {
            i = 0;
            while i < MAX_MENU_ITEMS as u16 {
                row = i + sSavedPokeblockData.scrollOffset;
                if row as i32 == *data.at(2) as i32 {
                    DrawPokeblockMenuHighlight(i, TILE_HIGHLIGHT_RED);
                } else {
                    DrawPokeblockMenuHighlight(i, TILE_HIGHLIGHT_NONE);
                }
                i += 1;
            }
        }
        SetSwapLineSpritesInvisibility((*sPokeblockMenu).swapLineSpriteIds.as_mut_ptr(), 7, FALSE);
        UpdateSwapLineSpritesPos(
            (*sPokeblockMenu).swapLineSpriteIds.as_mut_ptr(),
            7,
            128,
            sSavedPokeblockData.selectedRow * 16 + 8,
        );
        match input {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                if gMain.newKeys as i32 & A_BUTTON != 0 {
                    UpdatePokeblockSwapMenu(taskId, FALSE);
                } else {
                    UpdatePokeblockSwapMenu(taskId, TRUE);
                }
            }
            _ => {
                PlaySE(SE_SELECT);
                UpdatePokeblockSwapMenu(taskId, FALSE);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePokeblockSwapMenu(taskId: u8, noSwap: u8) {
    let mut i: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut swappedFromId: u16 = sSavedPokeblockData.scrollOffset + sSavedPokeblockData.selectedRow;
    (*sPokeblockMenu).isSwapping = FALSE;
    DestroyListMenuTask(
        *data as u8,
        &raw mut sSavedPokeblockData.scrollOffset,
        &raw mut sSavedPokeblockData.selectedRow,
    );
    if noSwap == 0
        && *data.at(2) as i32 != swappedFromId as i32
        && *data.at(2) as i32 != swappedFromId as i32 - 1
    {
        SwapPokeblockMenuItems(*data.at(2) as u32, swappedFromId as u32);
        UpdatePokeblockList();
    }
    if (*data.at(2) as i32) < swappedFromId as i32 {
        sSavedPokeblockData.selectedRow -= 1;
    }
    *data = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        sSavedPokeblockData.scrollOffset,
        sSavedPokeblockData.selectedRow,
    ) as i16;
    ScheduleBgCopyTilemapToVram(0);
    SetSwapLineSpritesInvisibility((*sPokeblockMenu).swapLineSpriteIds.as_mut_ptr(), 7, TRUE);
    i = 0;
    while i < MAX_MENU_ITEMS {
        DrawPokeblockMenuHighlight(i as u16, TILE_HIGHLIGHT_NONE);
        i += 1;
    }
    DrawPokeblockMenuHighlight(sSavedPokeblockData.selectedRow, TILE_HIGHLIGHT_BLUE);
    gTasks[taskId].func = Some(Task_HandlePokeblockMenuInput);
}
pub(crate) unsafe extern "C" fn ShowPokeblockActionsWindow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if (*sPokeblockMenu).numActions == 3 {
        *data.at(1) = WIN_ACTIONS_TALL as i16;
    } else {
        *data.at(1) = WIN_ACTIONS;
    }
    DestroyScrollArrows();
    DrawStdFrameWithCustomTileAndPalette(*data.at(1) as u8, FALSE, 1, 0xE);
    PrintMenuActionTextsInUpperLeftCorner(
        *data.at(1) as u8,
        (*sPokeblockMenu).numActions,
        sPokeblockMenuActions.as_ptr().cast_mut(),
        (*sPokeblockMenu).pokeblockActionIds,
    );
    InitMenuInUpperLeftCornerNormal(*data.at(1) as u8, (*sPokeblockMenu).numActions, 0);
    PutWindowTilemap(*data.at(1) as u8);
    ScheduleBgCopyTilemapToVram(1);
    gTasks[taskId].func = Some(Task_HandlePokeblockActionsInput);
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblockActionsInput(taskId: u8) {
    let mut itemId: i8 = 0;
    if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
        return;
    }
    itemId = Menu_ProcessInputNoWrap();
    if itemId == MENU_NOTHING_CHOSEN {
        return;
    } else if itemId == MENU_B_PRESSED {
        PlaySE(SE_SELECT);
        PokeblockAction_Cancel(taskId);
    } else {
        PlaySE(SE_SELECT);
        sPokeblockMenuActions[*(*sPokeblockMenu).pokeblockActionIds.at(itemId)]
            .func
            .void_u8
            .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_UseOnField(taskId: u8) {
    (*sPokeblockMenu).callbackOnUse = Some(UsePokeblockOnField);
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe extern "C" fn UsePokeblockOnField() {
    ChooseMonToGivePokeblock(
        &raw mut (*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId],
        Some(ReturnToPokeblockCaseOnField),
    );
}
pub(crate) unsafe extern "C" fn ReturnToPokeblockCaseOnField() {
    OpenPokeblockCase(PBLOCK_CASE_FIELD, sSavedPokeblockData.callback);
}
pub(crate) unsafe extern "C" fn PokeblockAction_Toss(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    ClearStdWindowAndFrameToTransparent(*data.at(1) as u8, FALSE);
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gPokeblockNames[(*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId].color],
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_ThrowAwayVar1.as_ptr().cast_mut(),
    );
    DisplayMessageAndContinueTask(
        taskId,
        WIN_TOSS_MSG,
        10,
        13,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        gStringVar4.as_mut_ptr(),
        core::mem::transmute::<Option<unsafe extern "C" fn(u8)>, *mut c_void>(Some(
            CreateTossPokeblockYesNoMenu,
        )),
    );
}
pub(crate) unsafe extern "C" fn CreateTossPokeblockYesNoMenu(taskId: u8) {
    CreateYesNoMenuWithCallbacks(
        taskId,
        (&raw const *sTossPkblockWindowTemplate).cast_mut(),
        1,
        0,
        2,
        1,
        0xE,
        (&raw const *sTossYesNoFuncTable).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn TossedPokeblockMessage(taskId: u8) {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_Var1ThrownAway.as_ptr().cast_mut(),
    );
    DisplayMessageAndContinueTask(
        taskId,
        WIN_TOSS_MSG,
        10,
        13,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        gStringVar4.as_mut_ptr(),
        core::mem::transmute::<Option<unsafe extern "C" fn(u8)>, *mut c_void>(Some(TossPokeblock)),
    );
}
pub(crate) unsafe extern "C" fn TossPokeblock(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        let mut data: *mut i16 = null_mut();
        let mut scrollOffset: *mut u16 = null_mut();
        let mut selectedRow: *mut u16 = null_mut();
        TryClearPokeblock(gSpecialVar_ItemId as u8);
        PlaySE(SE_SELECT);
        scrollOffset = &raw mut sSavedPokeblockData.scrollOffset;
        selectedRow = &raw mut sSavedPokeblockData.selectedRow;
        data = gTasks[taskId].data.as_mut_ptr();
        DestroyListMenuTask(*data as u8, scrollOffset, selectedRow);
        DrawPokeblockMenuHighlight(*selectedRow, TILE_HIGHLIGHT_NONE);
        SetMenuItemsCountAndMaxShowed();
        LimitMenuScrollAndRow();
        UpdatePokeblockList();
        *data = ListMenuInit(
            &raw mut gMultiuseListMenuTemplate,
            *scrollOffset,
            *selectedRow,
        ) as i16;
        DrawPokeblockMenuHighlight(*selectedRow, TILE_HIGHLIGHT_BLUE);
        ScheduleBgCopyTilemapToVram(0);
        ScheduleBgCopyTilemapToVram(1);
        CloseTossPokeblockWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn CloseTossPokeblockWindow(taskId: u8) {
    ClearDialogWindowAndFrameToTransparent(WIN_TOSS_MSG, FALSE);
    ScheduleBgCopyTilemapToVram(1);
    CreateScrollArrows();
    gTasks[taskId].func = Some(Task_HandlePokeblockMenuInput);
}
pub(crate) unsafe extern "C" fn PokeblockAction_UseInBattle(taskId: u8) {
    let mut nature: u8 = GetNature(&raw mut gEnemyParty[0]);
    let mut gain: i16 = PokeblockGetGain(
        nature,
        &raw mut (*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId],
    );
    StringCopy(
        gBattleTextBuff1.as_mut_ptr(),
        gPokeblockNames[(*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId].color],
    );
    TryClearPokeblock(gSpecialVar_ItemId as u8);
    gSpecialVar_ItemId = ((*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId].color as u16) << 8;
    if gain == 0 {
        gSpecialVar_ItemId += 1;
    } else if gain > 0 {
        gSpecialVar_ItemId += 2;
    } else {
        gSpecialVar_ItemId += 3;
    }
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe extern "C" fn PokeblockAction_UseOnPokeblockFeeder(taskId: u8) {
    SafariZoneActivatePokeblockFeeder(gSpecialVar_ItemId as u8);
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gPokeblockNames[(*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId].color],
    );
    gSpecialVar_Result = gSpecialVar_ItemId;
    TryClearPokeblock(gSpecialVar_ItemId as u8);
    gSpecialVar_ItemId = 0;
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe extern "C" fn PokeblockAction_GiveToContestLady(taskId: u8) {
    gSpecialVar_0x8004 =
        GivePokeblockToContestLady(&raw mut (*gSaveBlock1Ptr).pokeblocks[gSpecialVar_ItemId])
            as u16;
    gSpecialVar_Result = gSpecialVar_ItemId;
    TryClearPokeblock(gSpecialVar_ItemId as u8);
    gSpecialVar_ItemId = 0;
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe extern "C" fn PokeblockAction_Cancel(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    ClearStdWindowAndFrameToTransparent(*data.at(1) as u8, FALSE);
    ScheduleBgCopyTilemapToVram(1);
    CreateScrollArrows();
    gTasks[taskId].func = Some(Task_HandlePokeblockMenuInput);
}
pub(crate) unsafe extern "C" fn ClearPokeblock(pkblId: u8) {
    (*gSaveBlock1Ptr).pokeblocks[pkblId].color = 0;
    (*gSaveBlock1Ptr).pokeblocks[pkblId].spicy = 0;
    (*gSaveBlock1Ptr).pokeblocks[pkblId].dry = 0;
    (*gSaveBlock1Ptr).pokeblocks[pkblId].sweet = 0;
    (*gSaveBlock1Ptr).pokeblocks[pkblId].bitter = 0;
    (*gSaveBlock1Ptr).pokeblocks[pkblId].sour = 0;
    (*gSaveBlock1Ptr).pokeblocks[pkblId].feel = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPokeblocks() {
    let mut i: u8 = 0;
    i = 0;
    while i < POKEBLOCKS_COUNT as u8 {
        ClearPokeblock(i);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHighestPokeblocksFlavorLevel(pokeblock: *mut Pokeblock) -> u8 {
    let mut i: u8 = 0;
    let mut maxFlavor: u8 = GetPokeblockData(pokeblock, PBLOCK_SPICY) as u8;
    i = PBLOCK_SPICY;
    while i < FLAVOR_COUNT as u8 {
        let mut currFlavor: u8 = GetPokeblockData(pokeblock, PBLOCK_SPICY + i) as u8;
        if maxFlavor < currFlavor {
            maxFlavor = currFlavor;
        }
        i += 1;
    }
    return maxFlavor;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblocksFeel(pokeblock: *mut Pokeblock) -> u8 {
    let mut feel: u8 = GetPokeblockData(pokeblock, PBLOCK_FEEL) as u8;
    if feel > POKEBLOCK_MAX_FEEL {
        feel = POKEBLOCK_MAX_FEEL;
    }
    return feel;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFirstFreePokeblockSlot() -> i8 {
    let mut i: u8 = 0;
    i = 0;
    while i < POKEBLOCKS_COUNT as u8 {
        if (*gSaveBlock1Ptr).pokeblocks[i].color == PBLOCK_CLR_NONE {
            return i as i8;
        }
        i += 1;
    }
    return -1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddPokeblock(pokeblock: *mut Pokeblock) -> u32 {
    let mut slot: i8 = GetFirstFreePokeblockSlot();
    if slot == -1 {
        return FALSE as u32;
    } else {
        (*gSaveBlock1Ptr).pokeblocks[slot] = *pokeblock;
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryClearPokeblock(pkblId: u8) -> u32 {
    if (*gSaveBlock1Ptr).pokeblocks[pkblId].color == PBLOCK_CLR_NONE {
        return FALSE as u32;
    } else {
        ClearPokeblock(pkblId);
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblockData(pokeblock: *mut Pokeblock, field: u8) -> i16 {
    if field == PBLOCK_COLOR {
        return (*pokeblock).color as i16;
    }
    if field == PBLOCK_SPICY {
        return (*pokeblock).spicy as i16;
    }
    if field == PBLOCK_DRY {
        return (*pokeblock).dry as i16;
    }
    if field == PBLOCK_SWEET {
        return (*pokeblock).sweet as i16;
    }
    if field == PBLOCK_BITTER {
        return (*pokeblock).bitter as i16;
    }
    if field == PBLOCK_SOUR {
        return (*pokeblock).sour as i16;
    }
    if field == PBLOCK_FEEL {
        return (*pokeblock).feel as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokeblockGetGain(nature: u8, pokeblock: *mut Pokeblock) -> i16 {
    let mut flavor: u8 = 0;
    let mut curGain: i16 = 0;
    let mut totalGain: i16 = 0;
    flavor = 0;
    while flavor < FLAVOR_COUNT as u8 {
        curGain = GetPokeblockData(pokeblock, flavor + PBLOCK_SPICY);
        if curGain > 0 {
            totalGain += curGain
                * gPokeblockFlavorCompatibilityTable[FLAVOR_COUNT * nature as i32 + flavor as i32]
                    as i16;
        }
        flavor += 1;
    }
    return totalGain;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokeblockCopyName(pokeblock: *mut Pokeblock, dest: *mut u8) {
    let mut color: u8 = GetPokeblockData(pokeblock, PBLOCK_COLOR) as u8;
    StringCopy(dest, gPokeblockNames[color]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMonFavoritePokeblockName(nature: u8, dest: *mut u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < FLAVOR_COUNT as u8 {
        if PokeblockGetGain(nature, (&raw const sFavoritePokeblocksTable[i]).cast_mut()) > 0 {
            StringCopy(dest, gPokeblockNames[i as i32 + 1]);
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblocksFlavor(pokeblock: *mut Pokeblock) -> u8 {
    let mut bestFlavor: i16 = 0;
    let mut i: i16 = 0;
    i = 0;
    while i < FLAVOR_COUNT as i16 {
        if GetPokeblockData(pokeblock, bestFlavor as u8 + 1)
            < GetPokeblockData(pokeblock, i as u8 + 1)
        {
            bestFlavor = i;
        }
        i += 1;
    }
    return bestFlavor as u8;
}
