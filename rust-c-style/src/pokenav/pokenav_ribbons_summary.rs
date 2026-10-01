//! Translated from `src/pokenav_ribbons_summary.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sRibbonData gRibbonDescriptionPart1_Champion gRibbonDescriptionPart2_Champion gRibbonDescriptionPart1_CoolContest gRibbonDescriptionPart1_BeautyContest gRibbonDescriptionPart1_CuteContest gRibbonDescriptionPart1_SmartContest gRibbonDescriptionPart1_ToughContest gRibbonDescriptionPart2_NormalRank gRibbonDescriptionPart2_SuperRank gRibbonDescriptionPart2_HyperRank gRibbonDescriptionPart2_MasterRank gRibbonDescriptionPart1_Winning gRibbonDescriptionPart2_Winning gRibbonDescriptionPart1_Victory gRibbonDescriptionPart2_Victory gRibbonDescriptionPart1_Artist gRibbonDescriptionPart2_Artist gRibbonDescriptionPart1_Effort gRibbonDescriptionPart2_Effort gRibbonDescriptionPointers gGiftRibbonDescriptionPart1_2003RegionalTourney gGiftRibbonDescriptionPart2_Champion gGiftRibbonDescriptionPart1_2003NationalTourney gGiftRibbonDescriptionPart1_2003GlobalCup gGiftRibbonDescriptionPart2_RunnerUp gGiftRibbonDescriptionPart2_Semifinalist gGiftRibbonDescriptionPart1_2004RegionalTourney gGiftRibbonDescriptionPart1_2004NationalTourney gGiftRibbonDescriptionPart1_2004GlobalCup gGiftRibbonDescriptionPart1_2005RegionalTourney gGiftRibbonDescriptionPart1_2005NationalTourney gGiftRibbonDescriptionPart1_2005GlobalCup gGiftRibbonDescriptionPart1_PokemonBattleCup gGiftRibbonDescriptionPart2_Participation gGiftRibbonDescriptionPart1_PokemonLeague gGiftRibbonDescriptionPart1_AdvanceCup gGiftRibbonDescriptionPart1_PokemonTournament gGiftRibbonDescriptionPart2_Participation2 gGiftRibbonDescriptionPart1_PokemonEvent gGiftRibbonDescriptionPart1_PokemonFestival gGiftRibbonDescriptionPart1_DifficultyClearing gGiftRibbonDescriptionPart2_Commemorative gGiftRibbonDescriptionPart1_ClearingAllChallenges gGiftRibbonDescriptionPart2_ClearingAllChallenges gGiftRibbonDescriptionPart1_100StraightWin gGiftRibbonDescriptionPart1_DarknessTower gGiftRibbonDescriptionPart1_RedTower gGiftRibbonDescriptionPart1_BlackironTower gGiftRibbonDescriptionPart1_FinalTower gGiftRibbonDescriptionPart1_LegendMaking gGiftRibbonDescriptionPart1_PokemonCenterTokyo gGiftRibbonDescriptionPart1_PokemonCenterOsaka gGiftRibbonDescriptionPart1_PokemonCenterNagoya gGiftRibbonDescriptionPart1_PokemonCenterNY gGiftRibbonDescriptionPart1_SummerHolidays gGiftRibbonDescriptionPart2_EmptyString gGiftRibbonDescriptionPart1_WinterHolidays gGiftRibbonDescriptionPart1_SpringHolidays gGiftRibbonDescriptionPart1_Evergreen gGiftRibbonDescriptionPart1_SpecialHoliday gGiftRibbonDescriptionPart1_HardWorker gGiftRibbonDescriptionPart1_LotsOfFriends gGiftRibbonDescriptionPart1_FullOfEnergy gGiftRibbonDescriptionPart1_LovedPokemon gGiftRibbonDescriptionPart2_LovedPokemon gGiftRibbonDescriptionPart1_LoveForPokemon gGiftRibbonDescriptionPart2_LoveForPokemon gGiftRibbonDescriptionPointers sRibbonIcons1_Pal sRibbonIcons2_Pal sRibbonIcons3_Pal sRibbonIcons4_Pal sRibbonIcons5_Pal sMonInfo_Pal sRibbonIconsSmall_Gfx sRibbonIconsBig_Gfx sBgTemplates sRibbonsSummaryMenuLoopTaskFuncs sRibbonCountWindowTemplate sRibbonSummaryMonNameWindowTemplate sText_MaleSymbol sText_FemaleSymbol sGenderlessIconString sRibbonMonListIndexWindowTemplate sRibbonGfxData sSpriteSheet_RibbonIconsBig sSpritePalettes_RibbonIcons sOamData_RibbonIconBig sAffineAnim_RibbonIconBig_Normal sAffineAnim_RibbonIconBig_ZoomIn sAffineAnim_RibbonIconBig_ZoomOut sAffineAnims_RibbonIconBig sSpriteTemplate_RibbonIconBig

/// `struct Pokenav_RibbonsSummaryList`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RibbonsSummaryList {
    pub unused1: CArray<u8, 8>,
    pub monList: *mut PokenavMonList,
    pub selectedPos: u16,
    pub normalRibbonLastRowStart: u16,
    pub numNormalRibbons: u16,
    pub numGiftRibbons: u16,
    pub ribbonIds: CArray<u32, 25>,
    pub giftRibbonIds: CArray<u32, 7>,
    pub unused2: u32,
    pub callback: Option<unsafe extern "C" fn(*mut Pokenav_RibbonsSummaryList) -> u32>,
}

unsafe impl Sync for Pokenav_RibbonsSummaryList {}

/// `struct Pokenav_RibbonsSummaryMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RibbonsSummaryMenu {
    pub callback: Option<unsafe extern "C" fn() -> u32>,
    pub loopedTaskId: u32,
    pub nameWindowId: u16,
    pub ribbonCountWindowId: u16,
    pub listIdxWindowId: u16,
    pub unusedWindowId: u16,
    pub monSpriteId: u16,
    pub bigRibbonSprite: *mut Sprite,
    pub unused: u32,
    pub tilemapBuffers: CArray<CArray<u8, 2048>, 2>,
}

unsafe impl Sync for Pokenav_RibbonsSummaryMenu {}

/// `__typeof__(sRibbonData[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sRibbonData_0_t {
    pub numBits: u8,
    pub numRibbons: u8,
    pub ribbonId: u8,
    pub isGiftRibbon: u8,
}

unsafe impl Sync for sRibbonData_0_t {}

/// `__typeof__(sRibbonGfxData[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sRibbonGfxData_0_t {
    pub tileNumOffset: u16,
    pub palNumOffset: u16,
}

unsafe impl Sync for sRibbonGfxData_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_RibbonsSummaryList>() == 156);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, unused1) == 0);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, monList) == 8);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, selectedPos) == 12);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, normalRibbonLastRowStart) == 14);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, numNormalRibbons) == 16);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, numGiftRibbons) == 18);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, ribbonIds) == 20);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, giftRibbonIds) == 120);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, unused2) == 148);
    assert!(offset_of!(Pokenav_RibbonsSummaryList, callback) == 152);
    assert!(size_of::<Pokenav_RibbonsSummaryMenu>() == 4124);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, callback) == 0);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, loopedTaskId) == 4);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, nameWindowId) == 8);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, ribbonCountWindowId) == 10);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, listIdxWindowId) == 12);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, unusedWindowId) == 14);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, monSpriteId) == 16);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, bigRibbonSprite) == 20);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, unused) == 24);
    assert!(offset_of!(Pokenav_RibbonsSummaryMenu, tilemapBuffers) == 28);
    assert!(size_of::<sRibbonData_0_t>() == 4);
    assert!(offset_of!(sRibbonData_0_t, numBits) == 0);
    assert!(offset_of!(sRibbonData_0_t, numRibbons) == 1);
    assert!(offset_of!(sRibbonData_0_t, ribbonId) == 2);
    assert!(offset_of!(sRibbonData_0_t, isGiftRibbon) == 3);
    assert!(size_of::<sRibbonGfxData_0_t>() == 4);
    assert!(offset_of!(sRibbonGfxData_0_t, tileNumOffset) == 0);
    assert!(offset_of!(sRibbonGfxData_0_t, palNumOffset) == 2);
};

const GFXTAG_RIBBON_ICONS_BIG: u16 = 9;
const GIFT_RIBBON_START_POS: u16 = 27;
const MON_SPRITE_X_OFF: i32 = -32;
const MON_SPRITE_X_ON: i32 = 40;
const MON_SPRITE_Y: i32 = 104;
const PALTAG_RIBBON_ICONS_1: u16 = 15;
const PALTAG_RIBBON_ICONS_2: u16 = 16;
const PALTAG_RIBBON_ICONS_3: u16 = 17;
const PALTAG_RIBBON_ICONS_4: u16 = 18;
const PALTAG_RIBBON_ICONS_5: u16 = 19;
const RIBBONANIM_ZOOM_IN: u8 = 1;
const RIBBONANIM_ZOOM_OUT: u8 = 2;
const RIBBONS_PER_ROW: u16 = 9;
const RIBBONS_SUMMARY_FUNC_EXIT: u32 = 5;
const RIBBONS_SUMMARY_FUNC_EXPANDED_CANCEL: u32 = 4;
const RIBBONS_SUMMARY_FUNC_EXPANDED_CURSOR_MOVE: u32 = 3;
const RIBBONS_SUMMARY_FUNC_NONE: u32 = 0;
const RIBBONS_SUMMARY_FUNC_SELECT_RIBBON: u32 = 2;
const RIBBONS_SUMMARY_FUNC_SWITCH_MONS: u32 = 1;

static gGiftRibbonDescriptionPointers: Table<CArray<CArray<*mut u8, 2>, 64>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::gGiftRibbonDescriptionPointers).cast());
static gRibbonDescriptionPointers: Table<CArray<CArray<*mut u8, 2>, 25>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::gRibbonDescriptionPointers).cast());
static sBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sBgTemplates).cast());
static sGenderlessIconString: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sGenderlessIconString).cast());
static sMonInfo_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sMonInfo_Pal).cast());
static sRibbonCountWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sRibbonCountWindowTemplate).cast());
static sRibbonData: Table<CArray<sRibbonData_0_t, 17>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sRibbonData).cast());
static sRibbonGfxData: Table<CArray<sRibbonGfxData_0_t, 32>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sRibbonGfxData).cast());
static sRibbonIcons1_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sRibbonIcons1_Pal).cast());
static sRibbonIconsSmall_Gfx: Table<CArray<u32, 114>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sRibbonIconsSmall_Gfx).cast());
static sRibbonMonListIndexWindowTemplate: Table<CArray<WindowTemplate, 2>> = Table(
    (&raw const crate::data::pokenav_ribbons_summary::sRibbonMonListIndexWindowTemplate).cast(),
);
static sRibbonSummaryMonNameWindowTemplate: Table<WindowTemplate> = Table(
    (&raw const crate::data::pokenav_ribbons_summary::sRibbonSummaryMonNameWindowTemplate).cast(),
);
static sRibbonsSummaryMenuLoopTaskFuncs: Table<
    CArray<Option<unsafe extern "C" fn(i32) -> u32>, 6>,
> = Table(
    (&raw const crate::data::pokenav_ribbons_summary::sRibbonsSummaryMenuLoopTaskFuncs).cast(),
);
static sSpritePalettes_RibbonIcons: Table<CArray<SpritePalette, 6>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sSpritePalettes_RibbonIcons).cast());
static sSpriteSheet_RibbonIconsBig: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sSpriteSheet_RibbonIconsBig).cast());
static sSpriteTemplate_RibbonIconBig: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sSpriteTemplate_RibbonIconBig).cast());
static sText_FemaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sText_FemaleSymbol).cast());
static sText_MaleSymbol: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokenav_ribbons_summary::sText_MaleSymbol).cast());

pub(crate) static mut sRibbonDraw_Total: u32 = 0;
pub(crate) static mut sRibbonDraw_Current: u32 = 0;

unsafe extern "C" {
    static mut gKeyRepeatContinueDelay: u16;
    static mut gKeyRepeatStartDelay: u16;
    static mut gMain: Main;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static gPokenavRibbonsSummaryBg_Gfx: CArray<u32, 0>;
    static gPokenavRibbonsSummaryBg_Pal: CArray<u16, 0>;
    static gPokenavRibbonsSummaryBg_Tilemap: CArray<u32, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gText_RibbonsF700: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateMonPicSprite_HandleDeoxys(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBoxMonData2(a0: *mut BoxPokemon, a1: i32) -> u32;
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxMonGender(a0: *mut BoxPokemon) -> u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut BoxPokemon;
    fn GetLevelFromBoxMonExp(a0: *mut BoxPokemon) -> u8;
    fn GetLevelFromMonExp(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgTemplates(a0: *mut BgTemplate, a1: i32);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn PlaySE(a0: u16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavFillPalette(a0: u32, a1: u16);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut SpritePalette);
    fn PrintHelpBarText(a0: u32);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetAllPicSprites() -> u16;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_RibbonsSummaryMenu() -> u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST, 156)
            as *mut Pokenav_RibbonsSummaryList;
    if list.is_null() {
        return FALSE as u32;
    }
    (*list).monList = GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    if (*list).monList.is_null() {
        return FALSE as u32;
    }
    GetMonRibbons(list);
    (*list).callback = Some(RibbonsSummaryHandleInput);
    gKeyRepeatContinueDelay = 3;
    gKeyRepeatStartDelay = 10;
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonsSummaryMenuCallback() -> u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    return (*list).callback.unwrap_unchecked()(list);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsSummaryScreen1() {
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST);
}
pub(crate) unsafe extern "C" fn RibbonsSummaryHandleInput(
    list: *mut Pokenav_RibbonsSummaryList,
) -> u32 {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 && (*(*list).monList).currIndex != 0 {
        (*(*list).monList).currIndex -= 1;
        (*list).selectedPos = 0;
        GetMonRibbons(list);
        return RIBBONS_SUMMARY_FUNC_SWITCH_MONS;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0
        && ((*(*list).monList).currIndex as i32) < (*(*list).monList).listCount as i32 - 1
    {
        (*(*list).monList).currIndex += 1;
        (*list).selectedPos = 0;
        GetMonRibbons(list);
        return RIBBONS_SUMMARY_FUNC_SWITCH_MONS;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*list).callback = Some(HandleExpandedRibbonInput);
        return RIBBONS_SUMMARY_FUNC_SELECT_RIBBON;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*list).callback = Some(ReturnToRibbonsListFromSummary);
        return RIBBONS_SUMMARY_FUNC_EXIT;
    }
    return RIBBONS_SUMMARY_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleExpandedRibbonInput(
    list: *mut Pokenav_RibbonsSummaryList,
) -> u32 {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 && TrySelectRibbonUp(list) != 0 {
        return RIBBONS_SUMMARY_FUNC_EXPANDED_CURSOR_MOVE;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 && TrySelectRibbonDown(list) != 0 {
        return RIBBONS_SUMMARY_FUNC_EXPANDED_CURSOR_MOVE;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 && TrySelectRibbonLeft(list) != 0 {
        return RIBBONS_SUMMARY_FUNC_EXPANDED_CURSOR_MOVE;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 && TrySelectRibbonRight(list) != 0 {
        return RIBBONS_SUMMARY_FUNC_EXPANDED_CURSOR_MOVE;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*list).callback = Some(RibbonsSummaryHandleInput);
        return RIBBONS_SUMMARY_FUNC_EXPANDED_CANCEL;
    }
    return RIBBONS_SUMMARY_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn ReturnToRibbonsListFromSummary(
    list: *mut Pokenav_RibbonsSummaryList,
) -> u32 {
    return POKENAV_RIBBONS_RETURN_TO_MON_LIST;
}
pub(crate) unsafe extern "C" fn TrySelectRibbonUp(list: *mut Pokenav_RibbonsSummaryList) -> u32 {
    if (*list).selectedPos < FIRST_GIFT_RIBBON {
        if (*list).selectedPos < RIBBONS_PER_ROW {
            return FALSE as u32;
        }
        (*list).selectedPos -= RIBBONS_PER_ROW;
        return TRUE as u32;
    }
    if (*list).numNormalRibbons != 0 {
        let mut ribbonPos: u32 = (*list).selectedPos as u32 - GIFT_RIBBON_START_POS as u32;
        (*list).selectedPos = ribbonPos as u16 + (*list).normalRibbonLastRowStart;
        if (*list).selectedPos >= (*list).numNormalRibbons {
            (*list).selectedPos = (*list).numNormalRibbons - 1;
        }
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn TrySelectRibbonDown(list: *mut Pokenav_RibbonsSummaryList) -> u32 {
    if (*list).selectedPos >= FIRST_GIFT_RIBBON {
        return FALSE as u32;
    }
    if (*list).selectedPos < (*list).normalRibbonLastRowStart {
        (*list).selectedPos += RIBBONS_PER_ROW;
        if (*list).selectedPos >= (*list).numNormalRibbons {
            (*list).selectedPos = (*list).numNormalRibbons - 1;
        }
        return TRUE as u32;
    }
    if (*list).numGiftRibbons != 0 {
        let mut ribbonPos: i32 =
            (*list).selectedPos as i32 - (*list).normalRibbonLastRowStart as i32;
        if ribbonPos >= (*list).numGiftRibbons as i32 {
            ribbonPos = (*list).numGiftRibbons as i32 - 1;
        }
        (*list).selectedPos = ribbonPos as u16 + GIFT_RIBBON_START_POS;
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn TrySelectRibbonLeft(list: *mut Pokenav_RibbonsSummaryList) -> u32 {
    let mut column: u16 = ((*list).selectedPos as i32 % 9) as u16;
    if column != 0 {
        (*list).selectedPos -= 1;
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn TrySelectRibbonRight(list: *mut Pokenav_RibbonsSummaryList) -> u32 {
    let mut column: i32 = (*list).selectedPos as i32 % 9;
    if column >= 8 {
        return FALSE as u32;
    }
    if (*list).selectedPos < GIFT_RIBBON_START_POS {
        if ((*list).selectedPos as i32) < (*list).numNormalRibbons as i32 - 1 {
            (*list).selectedPos += 1;
            return TRUE as u32;
        }
    } else {
        if column < (*list).numGiftRibbons as i32 - 1 {
            (*list).selectedPos += 1;
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetRibbonsSummaryCurrentIndex() -> u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    return (*(*list).monList).currIndex as u32;
}
pub(crate) unsafe extern "C" fn GetRibbonsSummaryMonListCount() -> u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    return (*(*list).monList).listCount as u32;
}
pub(crate) unsafe extern "C" fn GetMonNicknameLevelGender(
    nick: *mut u8,
    level: *mut u8,
    gender: *mut u8,
) {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    let mut mons: *mut PokenavMonList = (*list).monList;
    let mut monInfo: *mut PokenavMonListItem = &raw mut (*mons).monData[(*mons).currIndex];
    if (*monInfo).boxId == TOTAL_BOXES_COUNT {
        let mut mon: *mut Pokemon = &raw mut gPlayerParty[(*monInfo).monId];
        GetMonData3(mon, MON_DATA_NICKNAME, nick);
        *level = GetLevelFromMonExp(mon);
        *gender = GetMonGender(mon);
    } else {
        let mut boxMon: *mut BoxPokemon = GetBoxedMonPtr((*monInfo).boxId, (*monInfo).monId);
        *gender = GetBoxMonGender(boxMon);
        *level = GetLevelFromBoxMonExp(boxMon);
        GetBoxMonData3(boxMon, MON_DATA_NICKNAME, nick);
    }
    StringGet_Nickname(nick);
}
pub(crate) unsafe extern "C" fn GetMonSpeciesPersonalityOtId(
    species: *mut u16,
    personality: *mut u32,
    otId: *mut u32,
) {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    let mut mons: *mut PokenavMonList = (*list).monList;
    let mut monInfo: *mut PokenavMonListItem = &raw mut (*mons).monData[(*mons).currIndex];
    if (*monInfo).boxId == TOTAL_BOXES_COUNT {
        let mut mon: *mut Pokemon = &raw mut gPlayerParty[(*monInfo).monId];
        *species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        *personality = GetMonData2(mon, MON_DATA_PERSONALITY);
        *otId = GetMonData2(mon, MON_DATA_OT_ID);
    } else {
        let mut boxMon: *mut BoxPokemon = GetBoxedMonPtr((*monInfo).boxId, (*monInfo).monId);
        *species = GetBoxMonData2(boxMon, MON_DATA_SPECIES) as u16;
        *personality = GetBoxMonData2(boxMon, MON_DATA_PERSONALITY);
        *otId = GetBoxMonData2(boxMon, MON_DATA_OT_ID);
    }
}
pub(crate) unsafe extern "C" fn GetCurrMonRibbonCount() -> u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    let mut mons: *mut PokenavMonList = (*list).monList;
    let mut monInfo: *mut PokenavMonListItem = &raw mut (*mons).monData[(*mons).currIndex];
    if (*monInfo).boxId == TOTAL_BOXES_COUNT {
        return GetMonData2(
            &raw mut gPlayerParty[(*monInfo).monId],
            MON_DATA_RIBBON_COUNT,
        );
    } else {
        return GetBoxMonDataAt((*monInfo).boxId, (*monInfo).monId, MON_DATA_RIBBON_COUNT);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetMonRibbons(list: *mut Pokenav_RibbonsSummaryList) {
    let mut ribbonFlags: u32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut mons: *mut PokenavMonList = (*list).monList;
    let mut monInfo: *mut PokenavMonListItem = &raw mut (*mons).monData[(*mons).currIndex];
    if (*monInfo).boxId == TOTAL_BOXES_COUNT {
        ribbonFlags = GetMonData2(&raw mut gPlayerParty[(*monInfo).monId], MON_DATA_RIBBONS);
    } else {
        ribbonFlags = GetBoxMonDataAt((*monInfo).boxId, (*monInfo).monId, MON_DATA_RIBBONS);
    }
    (*list).numNormalRibbons = 0;
    (*list).numGiftRibbons = 0;
    i = 0;
    while i < 17 {
        let mut numRibbons: i32 =
            shl_i32(1, sRibbonData[i].numBits as u32) - 1 & ribbonFlags as i32;
        if sRibbonData[i].isGiftRibbon == 0 {
            j = 0;
            while j < numRibbons {
                (*list).ribbonIds[{
                    let t1 = (*list).numNormalRibbons;
                    (*list).numNormalRibbons += 1;
                    t1
                }] = sRibbonData[i].ribbonId as u32 + j as u32;
                j += 1;
            }
        } else {
            j = 0;
            while j < numRibbons {
                (*list).giftRibbonIds[{
                    let t2 = (*list).numGiftRibbons;
                    (*list).numGiftRibbons += 1;
                    t2
                }] = sRibbonData[i].ribbonId as u32 + j as u32;
                j += 1;
            }
        }
        ribbonFlags = shr_u32(ribbonFlags, sRibbonData[i].numBits as u32);
        i += 1;
    }
    if (*list).numNormalRibbons != 0 {
        (*list).normalRibbonLastRowStart =
            (((*list).numNormalRibbons as i32 - 1) / 9) as u16 * RIBBONS_PER_ROW;
        (*list).selectedPos = 0;
    } else {
        (*list).normalRibbonLastRowStart = 0;
        (*list).selectedPos = GIFT_RIBBON_START_POS;
    }
}
pub(crate) unsafe extern "C" fn GetNormalRibbonIds(size: *mut u32) -> *mut u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    *size = (*list).numNormalRibbons as u32;
    return (*list).ribbonIds.as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetGiftRibbonIds(size: *mut u32) -> *mut u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    *size = (*list).numGiftRibbons as u32;
    return (*list).giftRibbonIds.as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetSelectedPosition() -> u16 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    return (*list).selectedPos;
}
pub(crate) unsafe extern "C" fn GetRibbonId() -> u32 {
    let mut list: *mut Pokenav_RibbonsSummaryList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_LIST) as *mut Pokenav_RibbonsSummaryList;
    let mut ribbonPos: i32 = (*list).selectedPos as i32;
    if ribbonPos < FIRST_GIFT_RIBBON as i32 {
        return (*list).ribbonIds[ribbonPos];
    } else {
        return (*list).giftRibbonIds[ribbonPos - GIFT_RIBBON_START_POS as i32];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenRibbonsSummaryMenu() -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU, 4124)
            as *mut Pokenav_RibbonsSummaryMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenRibbonsSummaryMenu), 1);
    (*menu).callback = Some(GetCurrentLoopedTaskActive);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRibbonsSummaryLoopedTask(id: i32) {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    (*menu).loopedTaskId = CreateLoopedTask(sRibbonsSummaryMenuLoopTaskFuncs[id], 1);
    (*menu).callback = Some(GetCurrentLoopedTaskActive);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRibbonsSummaryLoopedTaskActive() -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    return (*menu).callback.unwrap_unchecked()();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsSummaryScreen2() {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    RemoveWindow((*menu).ribbonCountWindowId as u8);
    RemoveWindow((*menu).nameWindowId as u8);
    RemoveWindow((*menu).listIdxWindowId as u8);
    RemoveWindow((*menu).unusedWindowId as u8);
    DestroyRibbonsMonFrontPic(menu);
    FreeSpriteTilesByTag(GFXTAG_RIBBON_ICONS_BIG);
    FreeSpritePaletteByTag(PALTAG_RIBBON_ICONS_1);
    FreeSpritePaletteByTag(PALTAG_RIBBON_ICONS_2);
    FreeSpritePaletteByTag(PALTAG_RIBBON_ICONS_3);
    FreeSpritePaletteByTag(PALTAG_RIBBON_ICONS_4);
    FreeSpritePaletteByTag(PALTAG_RIBBON_ICONS_5);
    FreeSpriteOamMatrix((*menu).bigRibbonSprite);
    DestroySprite((*menu).bigRibbonSprite);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU);
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    return IsLoopedTaskActive((*menu).loopedTaskId);
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenRibbonsSummaryMenu(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    match state {
        0 => {
            InitBgTemplates(sBgTemplates.as_ptr().cast_mut(), 2);
            DecompressAndCopyTileDataToVram(
                2,
                gPokenavRibbonsSummaryBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(2, (*menu).tilemapBuffers[0].as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                2,
                gPokenavRibbonsSummaryBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyPaletteIntoBufferUnfaded(gPokenavRibbonsSummaryBg_Pal.as_ptr().cast_mut(), 16, 32);
            CopyBgTilemapBufferToVram(2);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                BgDmaFill(1, 0, 0, 1);
                DecompressAndCopyTileDataToVram(
                    1,
                    sRibbonIconsSmall_Gfx.as_ptr().cast_mut() as *mut c_void,
                    0,
                    1,
                    0,
                );
                SetBgTilemapBuffer(1, (*menu).tilemapBuffers[1].as_mut_ptr() as *mut c_void);
                FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 32, 20);
                CopyPaletteIntoBufferUnfaded(sRibbonIcons1_Pal.as_ptr().cast_mut(), 32, 160);
                CopyPaletteIntoBufferUnfaded(sMonInfo_Pal.as_ptr().cast_mut(), 160, 32);
                CopyBgTilemapBufferToVram(1);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                AddRibbonCountWindow(menu);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                AddRibbonSummaryMonNameWindow(menu);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        4 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                AddRibbonListIndexWindow(menu);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        5 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CopyBgTilemapBufferToVram(2);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        6 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ResetSpritesAndDrawMonFrontPic(menu);
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        7 => {
            DrawAllRibbonsSmall(menu);
            PrintHelpBarText(HELPBAR_RIBBONS_LIST);
            return LT_INC_AND_PAUSE;
        }
        8 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CreateBigRibbonSprite(menu);
                ChangeBgX(1, 0, BG_COORD_SET);
                ChangeBgY(1, 0, BG_COORD_SET);
                ChangeBgX(2, 0, BG_COORD_SET);
                ChangeBgY(2, 0, BG_COORD_SET);
                ShowBg(1);
                ShowBg(2);
                HideBg(3);
                PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        9 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ExitRibbonsSummaryMenu(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
            return LT_FINISH;
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_SwitchRibbonsSummaryMon(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            SlideMonSpriteOff(menu);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsMonSpriteAnimating(menu) == 0 {
                PrintRibbbonsSummaryMonInfo(menu);
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        2 => {
            DrawAllRibbonsSmall(menu);
            return LT_INC_AND_CONTINUE;
        }
        3 => {
            PrintRibbonsMonListIndex(menu);
            return LT_INC_AND_CONTINUE;
        }
        4 => {
            PrintCurrentMonRibbonCount(menu);
            return LT_INC_AND_CONTINUE;
        }
        5 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SlideMonSpriteOn(menu);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        6 => {
            if IsMonSpriteAnimating(menu) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ExpandSelectedRibbon(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            UpdateAndZoomInSelectedRibbon(menu);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsRibbonAnimating(menu) == 0 {
                PrintRibbonNameAndDescription(menu);
                PrintHelpBarText(HELPBAR_RIBBONS_CHECK);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveRibbonsCursorExpanded(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            ZoomOutSelectedRibbon(menu);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsRibbonAnimating(menu) == 0 {
                UpdateAndZoomInSelectedRibbon(menu);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        2 => {
            if IsRibbonAnimating(menu) == 0 {
                PrintRibbonNameAndDescription(menu);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ShrinkExpandedRibbon(state: i32) -> u32 {
    let mut menu: *mut Pokenav_RibbonsSummaryMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_RIBBONS_SUMMARY_MENU) as *mut Pokenav_RibbonsSummaryMenu;
    match state {
        0 => {
            PlaySE(SE_SELECT);
            ZoomOutSelectedRibbon(menu);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsRibbonAnimating(menu) == 0 {
                PrintCurrentMonRibbonCount(menu);
                PrintHelpBarText(HELPBAR_RIBBONS_LIST);
                return LT_INC_AND_PAUSE;
            }
            return LT_PAUSE;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn AddRibbonCountWindow(menu: *mut Pokenav_RibbonsSummaryMenu) {
    (*menu).ribbonCountWindowId = AddWindow((&raw const *sRibbonCountWindowTemplate).cast_mut());
    PutWindowTilemap((*menu).ribbonCountWindowId as u8);
    PrintCurrentMonRibbonCount(menu);
}
pub(crate) unsafe extern "C" fn PrintCurrentMonRibbonCount(menu: *mut Pokenav_RibbonsSummaryMenu) {
    let mut color: CArray<u8, 3> = CArray([4, 2, 3]);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetCurrMonRibbonCount() as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_RibbonsF700.as_ptr().cast_mut(),
    );
    FillWindowPixelBuffer((*menu).ribbonCountWindowId as u8, 68);
    AddTextPrinterParameterized3(
        (*menu).ribbonCountWindowId as u8,
        FONT_NORMAL,
        0,
        1,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
    CopyWindowToVram((*menu).ribbonCountWindowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn PrintRibbonNameAndDescription(
    menu: *mut Pokenav_RibbonsSummaryMenu,
) {
    let mut i: i32 = 0;
    let mut ribbonId: u32 = GetRibbonId();
    let mut color: CArray<u8, 3> = CArray([4, 2, 3]);
    FillWindowPixelBuffer((*menu).ribbonCountWindowId as u8, 68);
    if ribbonId < FIRST_GIFT_RIBBON as u32 {
        i = 0;
        while i < 2 {
            AddTextPrinterParameterized3(
                (*menu).ribbonCountWindowId as u8,
                FONT_NORMAL,
                0,
                i as u8 * 16 + 1,
                color.as_mut_ptr(),
                TEXT_SKIP_DRAW as i8,
                gRibbonDescriptionPointers[ribbonId][i],
            );
            i += 1;
        }
    } else {
        ribbonId = (*gSaveBlock1Ptr).giftRibbons[ribbonId - FIRST_GIFT_RIBBON as u32] as u32;
        if ribbonId == 0 {
            return;
        }
        ribbonId -= 1;
        i = 0;
        while i < 2 {
            AddTextPrinterParameterized3(
                (*menu).ribbonCountWindowId as u8,
                FONT_NORMAL,
                0,
                i as u8 * 16 + 1,
                color.as_mut_ptr(),
                TEXT_SKIP_DRAW as i8,
                gGiftRibbonDescriptionPointers[ribbonId][i],
            );
            i += 1;
        }
    }
    CopyWindowToVram((*menu).ribbonCountWindowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn AddRibbonSummaryMonNameWindow(
    menu: *mut Pokenav_RibbonsSummaryMenu,
) {
    (*menu).nameWindowId = AddWindow((&raw const *sRibbonSummaryMonNameWindowTemplate).cast_mut());
    PutWindowTilemap((*menu).nameWindowId as u8);
    PrintRibbbonsSummaryMonInfo(menu);
}
pub(crate) unsafe extern "C" fn PrintRibbbonsSummaryMonInfo(menu: *mut Pokenav_RibbonsSummaryMenu) {
    let mut genderTxt: *mut u8 = null_mut();
    let mut txtPtr: *mut u8 = null_mut();
    let mut level: u8 = 0;
    let mut gender: u8 = 0;
    let mut windowId: u16 = (*menu).nameWindowId;
    FillWindowPixelBuffer(windowId as u8, 17);
    GetMonNicknameLevelGender(gStringVar3.as_mut_ptr(), &raw mut level, &raw mut gender);
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        gStringVar3.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    match gender {
        MON_MALE => {
            genderTxt = sText_MaleSymbol.as_ptr().cast_mut();
        }
        MON_FEMALE => {
            genderTxt = sText_FemaleSymbol.as_ptr().cast_mut();
        }
        _ => {
            genderTxt = sGenderlessIconString.as_ptr().cast_mut();
        }
    }
    txtPtr = StringCopy(gStringVar1.as_mut_ptr(), genderTxt);
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = CHAR_SLASH;
    *({
        let t2 = txtPtr;
        txtPtr = txtPtr.at(1);
        t2
    }) = CHAR_EXTRA_SYMBOL;
    *({
        let t3 = txtPtr;
        txtPtr = txtPtr.at(1);
        t3
    }) = CHAR_LV_2;
    ConvertIntToDecimalStringN(txtPtr, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        gStringVar1.as_mut_ptr(),
        60,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(windowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn AddRibbonListIndexWindow(menu: *mut Pokenav_RibbonsSummaryMenu) {
    (*menu).listIdxWindowId = AddWindow(sRibbonMonListIndexWindowTemplate.as_ptr().cast_mut());
    FillWindowPixelBuffer((*menu).listIdxWindowId as u8, 17);
    PutWindowTilemap((*menu).listIdxWindowId as u8);
    PrintRibbonsMonListIndex(menu);
}
pub(crate) unsafe extern "C" fn PrintRibbonsMonListIndex(menu: *mut Pokenav_RibbonsSummaryMenu) {
    let mut x: i32 = 0;
    let mut txtPtr: *mut u8 = null_mut();
    let mut id: u32 = GetRibbonsSummaryCurrentIndex() + 1;
    let mut count: u32 = GetRibbonsSummaryMonListCount();
    txtPtr = ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        id as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = CHAR_SLASH;
    ConvertIntToDecimalStringN(txtPtr, count as i32, STR_CONV_MODE_RIGHT_ALIGN, 3);
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 56);
    AddTextPrinterParameterized(
        (*menu).listIdxWindowId as u8,
        FONT_NORMAL,
        gStringVar1.as_mut_ptr(),
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram((*menu).listIdxWindowId as u8, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn ResetSpritesAndDrawMonFrontPic(
    menu: *mut Pokenav_RibbonsSummaryMenu,
) {
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    GetMonSpeciesPersonalityOtId(&raw mut species, &raw mut personality, &raw mut otId);
    ResetAllPicSprites();
    (*menu).monSpriteId = DrawRibbonsMonFrontPic(MON_SPRITE_X_ON, MON_SPRITE_Y);
    PokenavFillPalette(15, 0);
}
pub(crate) unsafe extern "C" fn DestroyRibbonsMonFrontPic(menu: *mut Pokenav_RibbonsSummaryMenu) {
    FreeAndDestroyMonPicSprite((*menu).monSpriteId);
}
pub(crate) unsafe extern "C" fn DrawRibbonsMonFrontPic(x: i32, y: i32) -> u16 {
    let mut species: u16 = 0;
    let mut spriteId: u16 = 0;
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    GetMonSpeciesPersonalityOtId(&raw mut species, &raw mut personality, &raw mut otId);
    spriteId = CreateMonPicSprite_HandleDeoxys(
        species,
        otId,
        personality,
        TRUE,
        MON_SPRITE_X_ON as i16,
        MON_SPRITE_Y as i16,
        15,
        TAG_NONE,
    );
    gSprites[spriteId].oam.set_priority(0);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SlideMonSpriteOff(menu: *mut Pokenav_RibbonsSummaryMenu) {
    StartMonSpriteSlide(
        &raw mut gSprites[(*menu).monSpriteId],
        MON_SPRITE_X_ON,
        MON_SPRITE_X_OFF,
        6,
    );
}
pub(crate) unsafe extern "C" fn SlideMonSpriteOn(menu: *mut Pokenav_RibbonsSummaryMenu) {
    FreeAndDestroyMonPicSprite((*menu).monSpriteId);
    (*menu).monSpriteId = DrawRibbonsMonFrontPic(MON_SPRITE_X_OFF, MON_SPRITE_Y);
    StartMonSpriteSlide(
        &raw mut gSprites[(*menu).monSpriteId],
        MON_SPRITE_X_OFF,
        MON_SPRITE_X_ON,
        6,
    );
}
pub(crate) unsafe extern "C" fn IsMonSpriteAnimating(menu: *mut Pokenav_RibbonsSummaryMenu) -> u32 {
    return (gSprites[(*menu).monSpriteId].callback
        != Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))) as u32;
}
pub(crate) unsafe extern "C" fn StartMonSpriteSlide(
    sprite: *mut Sprite,
    startX: i32,
    destX: i32,
    time: i32,
) {
    let mut delta: u32 = destX as u32 - startX as u32;
    (*sprite).x = startX as i16;
    (*sprite).data[0] = (startX as i16) << 4;
    (*sprite).data[1] = div_u32(delta << 4, time as u32) as i16;
    (*sprite).data[2] = time as i16;
    (*sprite).data[3] = destX as i16;
    (*sprite).callback = Some(SpriteCB_MonSpriteSlide);
}
pub(crate) unsafe extern "C" fn SpriteCB_MonSpriteSlide(sprite: *mut Sprite) {
    if (*sprite).data[2] != 0 {
        (*sprite).data[2] -= 1;
        (*sprite).data[0] += (*sprite).data[1];
        (*sprite).x = (*sprite).data[0] >> 4;
        if (*sprite).x <= MON_SPRITE_X_OFF as i16 {
            (*sprite).set_invisible(TRUE as u16);
        } else {
            (*sprite).set_invisible(FALSE as u16);
        }
    } else {
        (*sprite).x = (*sprite).data[3];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn DrawAllRibbonsSmall(menu: *mut Pokenav_RibbonsSummaryMenu) {
    let mut ribbonIds: *mut u32 = null_mut();
    ClearRibbonsSummaryBg();
    ribbonIds = GetNormalRibbonIds(&raw mut sRibbonDraw_Total);
    sRibbonDraw_Current = 0;
    while sRibbonDraw_Current < sRibbonDraw_Total {
        DrawRibbonSmall(
            sRibbonDraw_Current,
            *({
                let t2 = ribbonIds;
                ribbonIds = ribbonIds.at(1);
                t2
            }),
        );
        sRibbonDraw_Current += 1;
    }
    ribbonIds = GetGiftRibbonIds(&raw mut sRibbonDraw_Total);
    sRibbonDraw_Current = 0;
    while sRibbonDraw_Current < sRibbonDraw_Total {
        DrawRibbonSmall(
            sRibbonDraw_Current + GIFT_RIBBON_START_POS as u32,
            *({
                let t4 = ribbonIds;
                ribbonIds = ribbonIds.at(1);
                t4
            }),
        );
        sRibbonDraw_Current += 1;
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn ClearRibbonsSummaryBg() {
    FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 32, 20);
}
pub(crate) unsafe extern "C" fn DrawRibbonSmall(i: u32, ribbonId: u32) {
    let mut bgData: CArray<u16, 4> = zeroed();
    let mut destX: u32 = i % 9 * 2 + 11;
    let mut destY: u32 = i / 9 * 2 + 4;
    BufferSmallRibbonGfxData(bgData.as_mut_ptr(), ribbonId);
    CopyToBgTilemapBufferRect(
        1,
        bgData.as_mut_ptr() as *mut c_void,
        destX as u8,
        destY as u8,
        2,
        2,
    );
}
pub(crate) unsafe extern "C" fn BufferSmallRibbonGfxData(mut dst: *mut u16, ribbonId: u32) {
    let mut palNum: u16 = sRibbonGfxData[ribbonId].palNumOffset + 2;
    let mut tileNum: u16 = sRibbonGfxData[ribbonId].tileNumOffset * 2 + 1;
    *dst = tileNum | palNum << 12;
    *dst.at(1) = tileNum | palNum << 12 | 0x400;
    *dst.at(2) = tileNum + 1 | palNum << 12;
    *dst.at(3) = tileNum + 1 | palNum << 12 | 0x400;
}
pub(crate) unsafe extern "C" fn CreateBigRibbonSprite(menu: *mut Pokenav_RibbonsSummaryMenu) {
    let mut spriteId: u8 = 0;
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_RibbonIconsBig).cast_mut());
    Pokenav_AllocAndLoadPalettes(sSpritePalettes_RibbonIcons.as_ptr().cast_mut());
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_RibbonIconBig).cast_mut(),
        0,
        0,
        0,
    );
    (*menu).bigRibbonSprite = &raw mut gSprites[spriteId];
    (*(*menu).bigRibbonSprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn UpdateAndZoomInSelectedRibbon(
    menu: *mut Pokenav_RibbonsSummaryMenu,
) {
    let mut ribbonId: u32 = 0;
    let mut position: i32 = GetSelectedPosition() as i32;
    let mut x: i32 = position % 9 * 16 + 96;
    let mut y: i32 = position / 9 * 16 + 40;
    (*(*menu).bigRibbonSprite).x = x as i16;
    (*(*menu).bigRibbonSprite).y = y as i16;
    ribbonId = GetRibbonId();
    (*(*menu).bigRibbonSprite).oam.set_tileNum(
        sRibbonGfxData[ribbonId].tileNumOffset * 16
            + GetSpriteTileStartByTag(GFXTAG_RIBBON_ICONS_BIG),
    );
    (*(*menu).bigRibbonSprite)
        .oam
        .set_paletteNum(IndexOfSpritePaletteTag(
            sRibbonGfxData[ribbonId].palNumOffset + PALTAG_RIBBON_ICONS_1,
        ) as u16);
    StartSpriteAffineAnim((*menu).bigRibbonSprite, RIBBONANIM_ZOOM_IN);
    (*(*menu).bigRibbonSprite).set_invisible(FALSE as u16);
    (*(*menu).bigRibbonSprite).data[0] = FALSE as i16;
    (*(*menu).bigRibbonSprite).callback = Some(SpriteCB_WaitForRibbonAnimation);
}
pub(crate) unsafe extern "C" fn ZoomOutSelectedRibbon(menu: *mut Pokenav_RibbonsSummaryMenu) {
    (*(*menu).bigRibbonSprite).data[0] = TRUE as i16;
    StartSpriteAffineAnim((*menu).bigRibbonSprite, RIBBONANIM_ZOOM_OUT);
    (*(*menu).bigRibbonSprite).callback = Some(SpriteCB_WaitForRibbonAnimation);
}
pub(crate) unsafe extern "C" fn IsRibbonAnimating(menu: *mut Pokenav_RibbonsSummaryMenu) -> u32 {
    return ((*(*menu).bigRibbonSprite).callback
        != Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))) as u32;
}
pub(crate) unsafe extern "C" fn SpriteCB_WaitForRibbonAnimation(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        (*sprite).set_invisible((*sprite).data[0] as u16);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
