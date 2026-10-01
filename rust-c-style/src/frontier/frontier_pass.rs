//! Translated from `src/frontier_pass.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMaleHead_Pal sFemaleHead_Pal sMapScreen_Gfx sCursor_Gfx sHeads_Gfx sMapCursor_Gfx sMapScreen_Tilemap sMapAndCard_ZoomedOut_Tilemap sCardBall_Filled_Tilemap sBattleRecord_Tilemap sMapAndCard_Zooming_Tilemap sBgAffineCoords sPassBgTemplates sMapBgTemplates sPassWindowTemplates sMapWindowTemplates sTextColors sPassAreasLayout sCursorSpriteSheets sHeadsSpriteSheet sSpritePalettes sAnim_Frame1_Unused sAnim_Frame1 sAnim_Frame2 sAnim_Frame3 sAnim_Frame4 sAnim_Frame5 sAnim_Frame6 sAnim_Frame7 sAnim_MapIndicatorCursor_Rectangle sAnim_MapIndicatorCursor_Square sAnims_TwoFrame sAnims_Medal sAnims_MapIndicatorCursor sAffineAnim_Unused sAffineAnims_Unused sSpriteTemplates_Cursors sSpriteTemplate_Medal sSpriteTemplate_PlayerHead sPassAreaDescriptions sMapLandmarks

/// `struct FrontierPassData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FrontierPassData {
    pub callback: Option<unsafe extern "C" fn()>,
    pub state: u16,
    pub battlePoints: u16,
    pub cursorX: i16,
    pub cursorY: i16,
    pub cursorArea: u8,
    pub previousCursorArea: u8,
    bits_14: u8,
    pub facilitySymbols: CArray<u8, 7>,
}

impl FrontierPassData {
    #[inline(always)]
    pub fn hasBattleRecord(&self) -> u8 {
        ((self.bits_14 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_hasBattleRecord(&mut self, v: u8) {
        self.bits_14 = (self.bits_14 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn areaToShow(&self) -> u8 {
        ((self.bits_14 as u32 >> 1) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_areaToShow(&mut self, v: u8) {
        self.bits_14 = (self.bits_14 & !(0x7 << 1)) | ((v as u8 & 0x7) << 1);
    }
    #[inline(always)]
    pub fn trainerStars(&self) -> u8 {
        ((self.bits_14 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_trainerStars(&mut self, v: u8) {
        self.bits_14 = (self.bits_14 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
}

unsafe impl Sync for FrontierPassData {}

/// `struct FrontierPassGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FrontierPassGfx {
    pub cursorSprite: *mut Sprite,
    pub symbolSprites: CArray<*mut Sprite, 7>,
    pub mapAndCardZoomTilemap: *mut u8,
    pub mapAndCardTilemap: *mut u8,
    pub battleRecordTilemap: *mut u8,
    pub zooming: u8,
    pub scaleX: i16,
    pub scaleY: i16,
    pub tilemapBuff1: CArray<u8, 4096>,
    pub tilemapBuff2: CArray<u8, 4096>,
    pub tilemapBuff3: CArray<u8, 1024>,
}

unsafe impl Sync for FrontierPassGfx {}

/// `struct FrontierMapData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FrontierMapData {
    pub callback: Option<unsafe extern "C" fn()>,
    pub cursorSprite: *mut Sprite,
    pub playerHeadSprite: *mut Sprite,
    pub mapIndicatorSprite: *mut Sprite,
    pub cursorPos: u8,
    pub unused: u8,
    pub tilemapBuff0: CArray<u8, 4096>,
    pub tilemapBuff1: CArray<u8, 4096>,
    pub tilemapBuff2: CArray<u8, 4096>,
}

unsafe impl Sync for FrontierMapData {}

/// `struct FrontierPassSaved`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FrontierPassSaved {
    pub callback: Option<unsafe extern "C" fn()>,
    pub cursorX: i16,
    pub cursorY: i16,
}

unsafe impl Sync for FrontierPassSaved {}

/// `__typeof__(sMapLandmarks[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sMapLandmarks_0_t {
    pub name: *mut u8,
    pub description: *mut u8,
    pub x: i16,
    pub y: i16,
    pub animNum: u8,
}

unsafe impl Sync for sMapLandmarks_0_t {}

/// `__typeof__(sPassAreasLayout[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sPassAreasLayout_0_t {
    pub yStart: i16,
    pub yEnd: i16,
    pub xStart: i16,
    pub xEnd: i16,
}

unsafe impl Sync for sPassAreasLayout_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FrontierPassData>() == 24);
    assert!(offset_of!(FrontierPassData, callback) == 0);
    assert!(offset_of!(FrontierPassData, state) == 4);
    assert!(offset_of!(FrontierPassData, battlePoints) == 6);
    assert!(offset_of!(FrontierPassData, cursorX) == 8);
    assert!(offset_of!(FrontierPassData, cursorY) == 10);
    assert!(offset_of!(FrontierPassData, cursorArea) == 12);
    assert!(offset_of!(FrontierPassData, previousCursorArea) == 13);
    assert!(offset_of!(FrontierPassData, bits_14) == 14);
    assert!(offset_of!(FrontierPassData, facilitySymbols) == 15);
    assert!(size_of::<FrontierPassGfx>() == 9268);
    assert!(offset_of!(FrontierPassGfx, cursorSprite) == 0);
    assert!(offset_of!(FrontierPassGfx, symbolSprites) == 4);
    assert!(offset_of!(FrontierPassGfx, mapAndCardZoomTilemap) == 32);
    assert!(offset_of!(FrontierPassGfx, mapAndCardTilemap) == 36);
    assert!(offset_of!(FrontierPassGfx, battleRecordTilemap) == 40);
    assert!(offset_of!(FrontierPassGfx, zooming) == 44);
    assert!(offset_of!(FrontierPassGfx, scaleX) == 46);
    assert!(offset_of!(FrontierPassGfx, scaleY) == 48);
    assert!(offset_of!(FrontierPassGfx, tilemapBuff1) == 50);
    assert!(offset_of!(FrontierPassGfx, tilemapBuff2) == 4146);
    assert!(offset_of!(FrontierPassGfx, tilemapBuff3) == 8242);
    assert!(size_of::<FrontierMapData>() == 12308);
    assert!(offset_of!(FrontierMapData, callback) == 0);
    assert!(offset_of!(FrontierMapData, cursorSprite) == 4);
    assert!(offset_of!(FrontierMapData, playerHeadSprite) == 8);
    assert!(offset_of!(FrontierMapData, mapIndicatorSprite) == 12);
    assert!(offset_of!(FrontierMapData, cursorPos) == 16);
    assert!(offset_of!(FrontierMapData, unused) == 17);
    assert!(offset_of!(FrontierMapData, tilemapBuff0) == 18);
    assert!(offset_of!(FrontierMapData, tilemapBuff1) == 4114);
    assert!(offset_of!(FrontierMapData, tilemapBuff2) == 8210);
    assert!(size_of::<FrontierPassSaved>() == 8);
    assert!(offset_of!(FrontierPassSaved, callback) == 0);
    assert!(offset_of!(FrontierPassSaved, cursorX) == 4);
    assert!(offset_of!(FrontierPassSaved, cursorY) == 6);
    assert!(size_of::<sMapLandmarks_0_t>() == 16);
    assert!(offset_of!(sMapLandmarks_0_t, name) == 0);
    assert!(offset_of!(sMapLandmarks_0_t, description) == 4);
    assert!(offset_of!(sMapLandmarks_0_t, x) == 8);
    assert!(offset_of!(sMapLandmarks_0_t, y) == 10);
    assert!(offset_of!(sMapLandmarks_0_t, animNum) == 12);
    assert!(size_of::<sPassAreasLayout_0_t>() == 8);
    assert!(offset_of!(sPassAreasLayout_0_t, yStart) == 0);
    assert!(offset_of!(sPassAreasLayout_0_t, yEnd) == 2);
    assert!(offset_of!(sPassAreasLayout_0_t, xStart) == 4);
    assert!(offset_of!(sPassAreasLayout_0_t, xEnd) == 6);
};

const CURSOR_AREA_CANCEL: u8 = 4;
const CURSOR_AREA_CARD: u8 = 2;
const CURSOR_AREA_COUNT: i32 = 14;
const CURSOR_AREA_MAP: u8 = 1;
const CURSOR_AREA_NOTHING: u8 = 0;
const CURSOR_AREA_RECORD: u8 = 3;
const CURSOR_AREA_SYMBOL_TOWER: i32 = 7;
const ERR_ALLOC_FAILED: u32 = 2;
const ERR_ALREADY_DONE: u32 = 1;
const MAP_WINDOW_COUNT: u8 = 3;
const MAP_WINDOW_DESCRIPTION: u8 = 2;
const MAP_WINDOW_NAME: u8 = 1;
const NUM_BG_PAL_SLOTS: i32 = 13;
const SUCCESS: u32 = 0;
const TAG_CURSOR: u16 = 0;
const TAG_HEAD_MALE: u16 = 4;
const TAG_MAP_INDICATOR: u16 = 1;
const TAG_MEDAL_SILVER: u16 = 2;
const WINDOW_BATTLE_POINTS: u8 = 2;
const WINDOW_BATTLE_RECORD: u8 = 1;
const WINDOW_COUNT: u8 = 5;
const WINDOW_DESCRIPTION: u8 = 3;
const WINDOW_EARNED_SYMBOLS: u8 = 0;

static sBattleRecord_Tilemap: Table<CArray<u32, 14>> =
    Table((&raw const crate::data::frontier_pass::sBattleRecord_Tilemap).cast());
static sBgAffineCoords: Table<CArray<CArray<i16, 2>, 2>> =
    Table((&raw const crate::data::frontier_pass::sBgAffineCoords).cast());
static sCursorSpriteSheets: Table<CArray<CompressedSpriteSheet, 3>> =
    Table((&raw const crate::data::frontier_pass::sCursorSpriteSheets).cast());
static sHeadsSpriteSheet: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::frontier_pass::sHeadsSpriteSheet).cast());
static sMapAndCard_ZoomedOut_Tilemap: Table<CArray<u32, 142>> =
    Table((&raw const crate::data::frontier_pass::sMapAndCard_ZoomedOut_Tilemap).cast());
static sMapAndCard_Zooming_Tilemap: Table<CArray<u32, 58>> =
    Table((&raw const crate::data::frontier_pass::sMapAndCard_Zooming_Tilemap).cast());
static sMapBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::frontier_pass::sMapBgTemplates).cast());
static sMapLandmarks: Table<CArray<sMapLandmarks_0_t, 7>> =
    Table((&raw const crate::data::frontier_pass::sMapLandmarks).cast());
static sMapScreen_Gfx: Table<CArray<u32, 1019>> =
    Table((&raw const crate::data::frontier_pass::sMapScreen_Gfx).cast());
static sMapScreen_Tilemap: Table<CArray<u32, 152>> =
    Table((&raw const crate::data::frontier_pass::sMapScreen_Tilemap).cast());
static sMapWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::frontier_pass::sMapWindowTemplates).cast());
static sPassAreaDescriptions: Table<CArray<*mut u8, 15>> =
    Table((&raw const crate::data::frontier_pass::sPassAreaDescriptions).cast());
static sPassAreasLayout: Table<CArray<sPassAreasLayout_0_t, 13>> =
    Table((&raw const crate::data::frontier_pass::sPassAreasLayout).cast());
static sPassBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::frontier_pass::sPassBgTemplates).cast());
static sPassWindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::frontier_pass::sPassWindowTemplates).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 7>> =
    Table((&raw const crate::data::frontier_pass::sSpritePalettes).cast());
static sSpriteTemplate_Medal: Table<SpriteTemplate> =
    Table((&raw const crate::data::frontier_pass::sSpriteTemplate_Medal).cast());
static sSpriteTemplate_PlayerHead: Table<SpriteTemplate> =
    Table((&raw const crate::data::frontier_pass::sSpriteTemplate_PlayerHead).cast());
static sSpriteTemplates_Cursors: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::frontier_pass::sSpriteTemplates_Cursors).cast());
static sTextColors: Table<CArray<CArray<u8, 3>, 3>> =
    Table((&raw const crate::data::frontier_pass::sTextColors).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPassData: *mut FrontierPassData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPassGfx: *mut FrontierPassGfx = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMapData: *mut FrontierMapData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedPassData: FrontierPassSaved = unsafe { zeroed() };

unsafe extern "C" {
    static gFrontierPassBg_Gfx: CArray<u32, 0>;
    static gFrontierPassBg_Pal: CArray<CArray<u16, 16>, 0>;
    static gFrontierPassBg_Tilemap: CArray<u32, 0>;
    static gFrontierPassCancelButtonHighlighted_Tilemap: CArray<u32, 0>;
    static gFrontierPassCancelButton_Tilemap: CArray<u32, 0>;
    static gFrontierPassMapAndCard_Gfx: CArray<u32, 0>;
    static mut gMain: Main;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattlePoints: CArray<u8, 0>;
    static gText_BattleRecord: CArray<u8, 0>;
    static gText_SymbolsEarned: CArray<u8, 0>;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CanCopyRecordedBattleSaveData() -> u32;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountPlayerTrainerStars() -> u32;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DisableInterrupts(a0: u16);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetCurrentRegionMapSectionId() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn MathUtil_Inv16(a0: i16) -> i16;
    fn Overworld_PlaySpecialMapMusic();
    fn PlayBGM(a0: u16);
    fn PlayRecordedBattle(a0: Option<unsafe extern "C" fn()>);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetAffineAnimData();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgAffine(a0: u8, a1: i32, a2: i32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn ShowPlayerTrainerCard(a0: Option<unsafe extern "C" fn()>);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn malloc_and_decompress(a0: *mut c_void, a1: *mut u32) -> *mut c_void;
}

pub(crate) unsafe extern "C" fn ResetGpuRegsAndBgs() {
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                VRAM as usize as *mut c_void,
                0x100c000,
            );
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                OAM as i32 as usize as *mut c_void,
                0x5000100,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierPass(callback: Option<unsafe extern "C" fn()>) {
    AllocateFrontierPassData(callback);
    SetMainCallback2(Some(CB2_InitFrontierPass));
}
pub(crate) unsafe extern "C" fn LeaveFrontierPass() {
    SetMainCallback2((*sPassData).callback);
    FreeFrontierPassData();
}
pub(crate) unsafe extern "C" fn AllocateFrontierPassData(
    callback: Option<unsafe extern "C" fn()>,
) -> u32 {
    let mut i: u8 = 0;
    if !sPassData.is_null() {
        return ERR_ALREADY_DONE;
    }
    sPassData = AllocZeroed(24) as *mut FrontierPassData;
    if sPassData.is_null() {
        return ERR_ALLOC_FAILED;
    }
    (*sPassData).callback = callback;
    i = GetCurrentRegionMapSectionId();
    if i != MAPSEC_BATTLE_FRONTIER && i != MAPSEC_ARTISAN_CAVE {
        (*sPassData).cursorX = 176;
        (*sPassData).cursorY = 104;
    } else {
        (*sPassData).cursorX = 176;
        (*sPassData).cursorY = 48;
    }
    (*sPassData).battlePoints = (*gSaveBlock2Ptr).frontier.battlePoints;
    (*sPassData).set_hasBattleRecord(CanCopyRecordedBattleSaveData() as u8);
    (*sPassData).set_areaToShow(CURSOR_AREA_NOTHING);
    (*sPassData).set_trainerStars(CountPlayerTrainerStars() as u8);
    i = 0;
    while i < NUM_FRONTIER_FACILITIES {
        if FlagGet(FLAG_SYS_TOWER_SILVER + i as u16 * 2) != 0 {
            (*sPassData).facilitySymbols[i] += 1;
        }
        if FlagGet(FLAG_SYS_TOWER_GOLD + i as u16 * 2) != 0 {
            (*sPassData).facilitySymbols[i] += 1;
        }
        i += 1;
    }
    return SUCCESS;
}
pub(crate) unsafe extern "C" fn FreeFrontierPassData() -> u32 {
    if sPassData.is_null() {
        return ERR_ALREADY_DONE;
    }
    memset(sPassData as *mut u8, 0, 24);
    Free(sPassData as *mut c_void);
    sPassData = null_mut();
    return SUCCESS;
}
pub(crate) unsafe extern "C" fn AllocateFrontierPassGfx() -> u32 {
    if !sPassGfx.is_null() {
        return ERR_ALREADY_DONE;
    }
    sPassGfx = AllocZeroed(9268) as *mut FrontierPassGfx;
    if sPassGfx.is_null() {
        return ERR_ALLOC_FAILED;
    }
    return SUCCESS;
}
pub(crate) unsafe extern "C" fn FreeFrontierPassGfx() -> u32 {
    FreeAllWindowBuffers();
    if sPassGfx.is_null() {
        return ERR_ALREADY_DONE;
    }
    if !(*sPassGfx).battleRecordTilemap.is_null() {
        Free((*sPassGfx).battleRecordTilemap as *mut c_void);
        (*sPassGfx).battleRecordTilemap = null_mut();
    }
    if !(*sPassGfx).mapAndCardTilemap.is_null() {
        Free((*sPassGfx).mapAndCardTilemap as *mut c_void);
        (*sPassGfx).mapAndCardTilemap = null_mut();
    }
    if !(*sPassGfx).mapAndCardZoomTilemap.is_null() {
        Free((*sPassGfx).mapAndCardZoomTilemap as *mut c_void);
        (*sPassGfx).mapAndCardZoomTilemap = null_mut();
    }
    memset(sPassGfx as *mut u8, 0, 9268);
    Free(sPassGfx as *mut c_void);
    sPassGfx = null_mut();
    return SUCCESS;
}
pub(crate) unsafe extern "C" fn VBlankCB_FrontierPass() {
    if (*sPassGfx).zooming != 0 {
        SetBgAffine(
            2,
            (sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][0] as i32) << 8,
            (sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][1] as i32) << 8,
            sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][0],
            sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][1],
            (*sPassGfx).scaleX,
            (*sPassGfx).scaleY,
            0,
        );
    }
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_FrontierPass() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
}
pub(crate) unsafe extern "C" fn CB2_InitFrontierPass() {
    if InitFrontierPass() != 0 {
        CreateTask(Some(Task_HandleFrontierPassInput), 0);
        SetMainCallback2(Some(CB2_FrontierPass));
    }
}
pub(crate) unsafe extern "C" fn CB2_HideFrontierPass() {
    if HideFrontierPass() != 0 {
        LeaveFrontierPass();
    }
}
pub(crate) unsafe extern "C" fn InitFrontierPass() -> u32 {
    let mut sizeOut: u32 = 0;
    match (*sPassData).state {
        0 => {
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            SetVBlankHBlankCallbacksToNull();
            DisableInterrupts(INTR_FLAG_HBLANK);
        }
        1 => {
            ResetGpuRegsAndBgs();
        }
        2 => {
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            ResetPaletteFade();
            ResetTempTileDataBuffers();
        }
        3 => {
            AllocateFrontierPassGfx();
        }
        4 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(1, sPassBgTemplates.as_ptr().cast_mut(), 3);
            SetBgTilemapBuffer(1, (*sPassGfx).tilemapBuff1.as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(2, (*sPassGfx).tilemapBuff2.as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(3, (*sPassGfx).tilemapBuff3.as_mut_ptr() as *mut c_void);
            SetBgAttribute(2, BG_ATTR_WRAPAROUND, 1);
        }
        5 => {
            InitWindows(sPassWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
        }
        6 => {
            (*sPassGfx).mapAndCardZoomTilemap = malloc_and_decompress(
                sMapAndCard_Zooming_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u8;
            (*sPassGfx).mapAndCardTilemap = malloc_and_decompress(
                sMapAndCard_ZoomedOut_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u8;
            (*sPassGfx).battleRecordTilemap = malloc_and_decompress(
                sBattleRecord_Tilemap.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeOut,
            ) as *mut u8;
            DecompressAndCopyTileDataToVram(
                1,
                gFrontierPassBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                2,
                gFrontierPassMapAndCard_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        7 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return FALSE as u32;
            }
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
        }
        8 => {
            LoadPalette(
                gFrontierPassBg_Pal.as_ptr().cast_mut() as *mut c_void,
                0,
                416,
            );
            LoadPalette(
                gFrontierPassBg_Pal[1 + (*sPassData).trainerStars() as i32]
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                16,
                32,
            );
            LoadPalette(GetTextWindowPalette(0) as *mut c_void, 240, 32);
            DrawFrontierPassBg();
            UpdateAreaHighlight((*sPassData).cursorArea, (*sPassData).previousCursorArea);
            if (*sPassData).areaToShow() == CURSOR_AREA_MAP
                || (*sPassData).areaToShow() == CURSOR_AREA_CARD
            {
                (*sPassData).state = 0;
                return TRUE as u32;
            }
        }
        9 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            LoadCursorAndSymbolSprites();
            SetVBlankCallback(Some(VBlankCB_FrontierPass));
            BlendPalettes(PALETTES_ALL, 16, 0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        }
        10 => {
            AnimateSprites();
            BuildOamBuffer();
            if UpdatePaletteFade() != 0 {
                return FALSE as u32;
            }
            (*sPassData).state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    (*sPassData).state += 1;
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn HideFrontierPass() -> u32 {
    match (*sPassData).state {
        0 => {
            if (*sPassData).areaToShow() != CURSOR_AREA_MAP
                && (*sPassData).areaToShow() != CURSOR_AREA_CARD
            {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            } else {
                (*sPassData).state = 2;
                return FALSE as u32;
            }
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return FALSE as u32;
            }
        }
        2 => {
            SetGpuReg(0x0, 0);
            HideBg(0);
            HideBg(1);
            HideBg(2);
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            SetVBlankHBlankCallbacksToNull();
        }
        3 => {
            FreeCursorAndSymbolSprites();
        }
        4 => {
            ResetGpuRegsAndBgs();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
        }
        5 => {
            UnsetBgTilemapBuffer(0);
            UnsetBgTilemapBuffer(1);
            UnsetBgTilemapBuffer(2);
            FreeFrontierPassGfx();
            (*sPassData).state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    (*sPassData).state += 1;
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetCursorAreaFromCoords(x: i16, y: i16) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 13 {
        if sPassAreasLayout[i].yStart <= y
            && sPassAreasLayout[i].yEnd >= y
            && sPassAreasLayout[i].xStart <= x
            && sPassAreasLayout[i].xEnd >= x
        {
            if i >= 6 && (*sPassData).facilitySymbols[i as i32 - CURSOR_AREA_SYMBOL_TOWER + 1] == 0
            {
                break;
            }
            return i + 1;
        }
        i += 1;
    }
    return CURSOR_AREA_NOTHING;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReshowFrontierPass() {
    let mut taskId: u8 = 0;
    if InitFrontierPass() == 0 {
        return;
    }
    match (*sPassData).areaToShow() {
        CURSOR_AREA_MAP | CURSOR_AREA_CARD => {
            taskId = CreateTask(Some(Task_PassAreaZoom), 0);
            gTasks[taskId].data[0] = TRUE as i16;
        }
        _ => {
            (*sPassData).set_areaToShow(CURSOR_AREA_NOTHING);
            taskId = CreateTask(Some(Task_HandleFrontierPassInput), 0);
        }
    }
    SetMainCallback2(Some(CB2_FrontierPass));
}
pub(crate) unsafe extern "C" fn CB2_ReturnFromRecord() {
    AllocateFrontierPassData(sSavedPassData.callback);
    (*sPassData).cursorX = sSavedPassData.cursorX;
    (*sPassData).cursorY = sSavedPassData.cursorY;
    memset(&raw mut sSavedPassData as *mut u8, 0, 8);
    match CurrentBattlePyramidLocation() {
        PYRAMID_LOCATION_FLOOR => {
            PlayBGM(MUS_B_PYRAMID);
        }
        PYRAMID_LOCATION_TOP => {
            PlayBGM(MUS_B_PYRAMID_TOP);
        }
        _ => {
            Overworld_PlaySpecialMapMusic();
        }
    }
    SetMainCallback2(Some(CB2_ReshowFrontierPass));
}
pub(crate) unsafe extern "C" fn CB2_ShowFrontierPassFeature() {
    if HideFrontierPass() == 0 {
        return;
    }
    match (*sPassData).areaToShow() {
        CURSOR_AREA_MAP => {
            ShowFrontierMap(Some(CB2_ReshowFrontierPass));
        }
        CURSOR_AREA_RECORD => {
            sSavedPassData.callback = (*sPassData).callback;
            sSavedPassData.cursorX = (*sPassData).cursorX;
            sSavedPassData.cursorY = (*sPassData).cursorY;
            FreeFrontierPassData();
            PlayRecordedBattle(Some(CB2_ReturnFromRecord));
        }
        CURSOR_AREA_CARD => {
            ShowPlayerTrainerCard(Some(CB2_ReshowFrontierPass));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn TryCallPassAreaFunction(taskId: u8, cursorArea: u8) -> u32 {
    match cursorArea {
        CURSOR_AREA_RECORD => {
            if (*sPassData).hasBattleRecord() == 0 {
                return FALSE as u32;
            }
            (*sPassData).set_areaToShow(CURSOR_AREA_RECORD);
            DestroyTask(taskId);
            SetMainCallback2(Some(CB2_ShowFrontierPassFeature));
        }
        CURSOR_AREA_MAP | CURSOR_AREA_CARD => {
            (*sPassData).set_areaToShow(cursorArea);
            gTasks[taskId].func = Some(Task_PassAreaZoom);
            gTasks[taskId].data[0] = FALSE as i16;
        }
        _ => {
            return FALSE as u32;
        }
    }
    (*sPassData).cursorX = (*(*sPassGfx).cursorSprite).x;
    (*sPassData).cursorY = (*(*sPassGfx).cursorSprite).y;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Task_HandleFrontierPassInput(taskId: u8) {
    let mut var: u8 = FALSE;
    if gMain.heldKeys as i32 & DPAD_UP != 0 && (*(*sPassGfx).cursorSprite).y >= 9 {
        (*(*sPassGfx).cursorSprite).y -= 2;
        if (*(*sPassGfx).cursorSprite).y <= 7 {
            (*(*sPassGfx).cursorSprite).y = 2;
        }
        var = TRUE;
    }
    if gMain.heldKeys as i32 & DPAD_DOWN != 0 && (*(*sPassGfx).cursorSprite).y <= 135 {
        (*(*sPassGfx).cursorSprite).y += 2;
        if (*(*sPassGfx).cursorSprite).y >= 137 {
            (*(*sPassGfx).cursorSprite).y = 136;
        }
        var = TRUE;
    }
    if gMain.heldKeys as i32 & DPAD_LEFT != 0 && (*(*sPassGfx).cursorSprite).x >= 6 {
        (*(*sPassGfx).cursorSprite).x -= 2;
        if (*(*sPassGfx).cursorSprite).x <= 4 {
            (*(*sPassGfx).cursorSprite).x = 5;
        }
        var = TRUE;
    }
    if gMain.heldKeys as i32 & DPAD_RIGHT != 0 && (*(*sPassGfx).cursorSprite).x <= 231 {
        (*(*sPassGfx).cursorSprite).x += 2;
        if (*(*sPassGfx).cursorSprite).x >= 233 {
            (*(*sPassGfx).cursorSprite).x = 232;
        }
        var = TRUE;
    }
    if var == 0 {
        if (*sPassData).cursorArea != CURSOR_AREA_NOTHING && gMain.newKeys as i32 & A_BUTTON != 0 {
            if (*sPassData).cursorArea <= CURSOR_AREA_RECORD {
                PlaySE(SE_SELECT);
                if TryCallPassAreaFunction(taskId, (*sPassData).cursorArea) != 0 {
                    return;
                }
            } else if (*sPassData).cursorArea == CURSOR_AREA_CANCEL {
                PlaySE(SE_PC_OFF);
                SetMainCallback2(Some(CB2_HideFrontierPass));
                DestroyTask(taskId);
            }
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_PC_OFF);
            SetMainCallback2(Some(CB2_HideFrontierPass));
            DestroyTask(taskId);
        }
    } else {
        var = GetCursorAreaFromCoords(
            (*(*sPassGfx).cursorSprite).x - 5,
            (*(*sPassGfx).cursorSprite).y + 5,
        );
        if (*sPassData).cursorArea != var {
            PrintAreaDescription(var);
            (*sPassData).previousCursorArea = (*sPassData).cursorArea;
            (*sPassData).cursorArea = var;
            UpdateAreaHighlight((*sPassData).cursorArea, (*sPassData).previousCursorArea);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PassAreaZoom(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match (*sPassData).state {
        0 => {
            if *data == 0 {
                ShowHideZoomingArea(TRUE, FALSE);
                *data.at(1) = 256;
                *data.at(2) = 256;
                *data.at(3) = 0x15;
                *data.at(4) = 0x15;
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 32767);
            } else {
                *data.at(1) = (1.984375f32 as f32 * 256 as f32) as i16;
                *data.at(2) = (1.984375f32 as f32 * 256 as f32) as i16;
                *data.at(3) = -21;
                *data.at(4) = -21;
                SetGpuReg(REG_OFFSET_DISPCNT, 4160);
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                LoadCursorAndSymbolSprites();
                SetVBlankCallback(Some(VBlankCB_FrontierPass));
                BlendPalettes(PALETTES_ALL, 16, 32767);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 32767);
            }
            (*sPassGfx).zooming = TRUE;
            (*sPassGfx).scaleX = MathUtil_Inv16(*data.at(1));
            (*sPassGfx).scaleY = MathUtil_Inv16(*data.at(2));
        }
        1 => {
            UpdatePaletteFade();
            *data.at(1) += *data.at(3);
            *data.at(2) += *data.at(4);
            (*sPassGfx).scaleX = MathUtil_Inv16(*data.at(1));
            (*sPassGfx).scaleY = MathUtil_Inv16(*data.at(2));
            if *data == 0 {
                if *data.at(1) <= (1.984375f32 as f32 * 256 as f32) as i16 {
                    return;
                }
            } else {
                if *data.at(1) != 256 {
                    return;
                }
            }
        }
        2 => {
            if (*sPassGfx).zooming != 0 {
                (*sPassGfx).zooming = FALSE;
            }
            if UpdatePaletteFade() != 0 {
                return;
            }
            if *data == 0 {
                DestroyTask(taskId);
                SetMainCallback2(Some(CB2_ShowFrontierPassFeature));
            } else {
                ShowHideZoomingArea(FALSE, FALSE);
                (*sPassData).set_areaToShow(CURSOR_AREA_NOTHING);
                gTasks[taskId].func = Some(Task_HandleFrontierPassInput);
            }
            SetBgAttribute(2, BG_ATTR_WRAPAROUND, 0);
            (*sPassData).state = 0;
            return;
        }
        _ => {}
    }
    (*sPassData).state += 1;
}
pub(crate) unsafe extern "C" fn ShowAndPrintWindows() {
    let mut x: i32 = 0;
    let mut i: u8 = 0;
    i = 0;
    while i < WINDOW_COUNT {
        PutWindowTilemap(i);
        FillWindowPixelBuffer(i, 0);
        i += 1;
    }
    x = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        gText_SymbolsEarned.as_ptr().cast_mut(),
        96,
    );
    AddTextPrinterParameterized3(
        WINDOW_EARNED_SYMBOLS,
        FONT_NORMAL,
        x as u8,
        5,
        sTextColors[0].as_ptr().cast_mut(),
        0,
        gText_SymbolsEarned.as_ptr().cast_mut(),
    );
    x = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        gText_BattleRecord.as_ptr().cast_mut(),
        96,
    );
    AddTextPrinterParameterized3(
        WINDOW_BATTLE_RECORD,
        FONT_NORMAL,
        x as u8,
        5,
        sTextColors[0].as_ptr().cast_mut(),
        0,
        gText_BattleRecord.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        WINDOW_BATTLE_POINTS,
        FONT_SMALL_NARROW,
        5,
        4,
        sTextColors[0].as_ptr().cast_mut(),
        0,
        gText_BattlePoints.as_ptr().cast_mut(),
    );
    ConvertIntToDecimalStringN(
        gStringVar4.as_mut_ptr(),
        (*sPassData).battlePoints as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    x = GetStringRightAlignXOffset(FONT_SMALL_NARROW as i32, gStringVar4.as_mut_ptr(), 91);
    AddTextPrinterParameterized3(
        WINDOW_BATTLE_POINTS,
        FONT_SMALL_NARROW,
        x as u8,
        16,
        sTextColors[0].as_ptr().cast_mut(),
        0,
        gStringVar4.as_mut_ptr(),
    );
    (*sPassData).cursorArea =
        GetCursorAreaFromCoords((*sPassData).cursorX - 5, (*sPassData).cursorY + 5);
    (*sPassData).previousCursorArea = CURSOR_AREA_NOTHING;
    PrintAreaDescription((*sPassData).cursorArea);
    i = 0;
    while i < WINDOW_COUNT {
        CopyWindowToVram(i, COPYWIN_FULL);
        i += 1;
    }
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn PrintAreaDescription(cursorArea: u8) {
    FillWindowPixelBuffer(WINDOW_DESCRIPTION, 0);
    if cursorArea == CURSOR_AREA_RECORD && (*sPassData).hasBattleRecord() == 0 {
        AddTextPrinterParameterized3(
            WINDOW_DESCRIPTION,
            FONT_NORMAL,
            2,
            0,
            sTextColors[1].as_ptr().cast_mut(),
            0,
            sPassAreaDescriptions[0],
        );
    } else if cursorArea != CURSOR_AREA_NOTHING {
        AddTextPrinterParameterized3(
            WINDOW_DESCRIPTION,
            FONT_NORMAL,
            2,
            0,
            sTextColors[1].as_ptr().cast_mut(),
            0,
            sPassAreaDescriptions[cursorArea],
        );
    }
    CopyWindowToVram(WINDOW_DESCRIPTION, COPYWIN_FULL);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn ShowHideZoomingArea(show: u8, zoomedIn: u8) {
    match (*sPassData).areaToShow() {
        CURSOR_AREA_MAP => {
            if show != 0 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    2,
                    (*sPassGfx).mapAndCardZoomTilemap as *mut c_void,
                    16,
                    3,
                    12,
                    7,
                    16,
                );
            } else {
                FillBgTilemapBufferRect(2, 0, 16, 3, 12, 7, 16);
            }
        }
        CURSOR_AREA_CARD => {
            if show != 0 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    2,
                    (*sPassGfx).mapAndCardZoomTilemap.at(84) as *mut c_void,
                    16,
                    10,
                    12,
                    7,
                    16,
                );
            } else {
                FillBgTilemapBufferRect(2, 0, 16, 10, 12, 7, 16);
            }
        }
        _ => {
            return;
        }
    }
    CopyBgTilemapBufferToVram(2);
    if zoomedIn != 0 {
        SetBgAffine(
            2,
            (sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][0] as i32) << 8,
            (sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][1] as i32) << 8,
            sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][0],
            sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][1],
            MathUtil_Inv16((1.984375f32 as f32 * 256 as f32) as i16),
            MathUtil_Inv16((1.984375f32 as f32 * 256 as f32) as i16),
            0,
        );
    } else {
        SetBgAffine(
            2,
            (sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][0] as i32) << 8,
            (sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][1] as i32) << 8,
            sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][0],
            sBgAffineCoords[(*sPassData).areaToShow() as i32 - 1][1],
            MathUtil_Inv16(256),
            MathUtil_Inv16(256),
            0,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateAreaHighlight(cursorArea: u8, previousCursorArea: u8) {
    match previousCursorArea {
        CURSOR_AREA_MAP => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                (*sPassGfx).mapAndCardTilemap as *mut c_void,
                16,
                3,
                12,
                7,
                17,
            );
        }
        CURSOR_AREA_CARD => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                (*sPassGfx).mapAndCardTilemap.at(336) as *mut c_void,
                16,
                10,
                12,
                7,
                17,
            );
        }
        CURSOR_AREA_RECORD => {
            if (*sPassData).hasBattleRecord() != 0 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1,
                    (*sPassGfx).battleRecordTilemap as *mut c_void,
                    2,
                    10,
                    12,
                    3,
                    17,
                );
            } else if cursorArea == CURSOR_AREA_NOTHING || cursorArea > CURSOR_AREA_CANCEL {
                return;
            }
        }
        CURSOR_AREA_CANCEL => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                gFrontierPassCancelButton_Tilemap.as_ptr().cast_mut() as *mut c_void,
                21,
                0,
                9,
                2,
                17,
            );
        }
        _ => {
            if cursorArea == CURSOR_AREA_NOTHING || cursorArea > CURSOR_AREA_CANCEL {
                return;
            }
        }
    }
    match cursorArea {
        CURSOR_AREA_MAP => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                (*sPassGfx).mapAndCardTilemap.at(168) as *mut c_void,
                16,
                3,
                12,
                7,
                17,
            );
        }
        CURSOR_AREA_CARD => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                (*sPassGfx).mapAndCardTilemap.at(504) as *mut c_void,
                16,
                10,
                12,
                7,
                17,
            );
        }
        CURSOR_AREA_RECORD => {
            if (*sPassData).hasBattleRecord() != 0 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1,
                    (*sPassGfx).battleRecordTilemap.at(72) as *mut c_void,
                    2,
                    10,
                    12,
                    3,
                    17,
                );
            } else {
                return;
            }
        }
        CURSOR_AREA_CANCEL => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                gFrontierPassCancelButtonHighlighted_Tilemap
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                21,
                0,
                9,
                2,
                17,
            );
        }
        _ => {
            if previousCursorArea == CURSOR_AREA_NOTHING || previousCursorArea > CURSOR_AREA_CANCEL
            {
                return;
            }
        }
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn DrawFrontierPassBg() {
    CopyToBgTilemapBuffer(
        1,
        gFrontierPassBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    UpdateAreaHighlight((*sPassData).cursorArea, (*sPassData).previousCursorArea);
    ShowHideZoomingArea(TRUE, (*sPassData).areaToShow());
    ShowAndPrintWindows();
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn LoadCursorAndSymbolSprites() {
    let mut spriteId: u8 = 0;
    let mut i: u8 = 0;
    FreeAllSpritePalettes();
    ResetAffineAnimData();
    LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
    LoadCompressedSpriteSheet((&raw const sCursorSpriteSheets[0]).cast_mut());
    LoadCompressedSpriteSheet((&raw const sCursorSpriteSheets[2]).cast_mut());
    spriteId = CreateSprite(
        (&raw const sSpriteTemplates_Cursors[0]).cast_mut(),
        (*sPassData).cursorX,
        (*sPassData).cursorY,
        0,
    );
    (*sPassGfx).cursorSprite = &raw mut gSprites[spriteId];
    (*(*sPassGfx).cursorSprite).oam.set_priority(0);
    i = 0;
    while i < NUM_FRONTIER_FACILITIES {
        if (*sPassData).facilitySymbols[i] != 0 {
            let mut sprite: SpriteTemplate = zeroed();
            sprite = *sSpriteTemplate_Medal;
            sprite.paletteTag += (*sPassData).facilitySymbols[i] as u16 - 1;
            spriteId = CreateSprite(
                &raw mut sprite,
                sPassAreasLayout[i as i32 + CURSOR_AREA_SYMBOL_TOWER - 1].xStart + 8,
                sPassAreasLayout[i as i32 + CURSOR_AREA_SYMBOL_TOWER - 1].yStart + 6,
                i + 1,
            );
            (*sPassGfx).symbolSprites[i] = &raw mut gSprites[spriteId];
            (*(*sPassGfx).symbolSprites[i]).oam.set_priority(2);
            StartSpriteAnim((*sPassGfx).symbolSprites[i], i);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FreeCursorAndSymbolSprites() {
    let mut i: u8 = 0;
    DestroySprite((*sPassGfx).cursorSprite);
    (*sPassGfx).cursorSprite = null_mut();
    i = 0;
    while i < NUM_FRONTIER_FACILITIES {
        if !(*sPassGfx).symbolSprites[i].is_null() {
            DestroySprite((*sPassGfx).symbolSprites[i]);
            (*sPassGfx).symbolSprites[i] = null_mut();
        }
        i += 1;
    }
    FreeAllSpritePalettes();
    FreeSpriteTilesByTag(TAG_MEDAL_SILVER);
    FreeSpriteTilesByTag(TAG_CURSOR);
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerHead(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn ShowFrontierMap(callback: Option<unsafe extern "C" fn()>) {
    if !sMapData.is_null() {
        SetMainCallback2(callback);
    }
    sMapData = AllocZeroed(12308) as *mut FrontierMapData;
    (*sMapData).callback = callback;
    ResetTasks();
    CreateTask(Some(Task_HandleFrontierMap), 0);
    SetMainCallback2(Some(CB2_FrontierPass));
}
pub(crate) unsafe extern "C" fn FreeFrontierMap() {
    ResetTasks();
    SetMainCallback2((*sMapData).callback);
    memset(sMapData as *mut u8, 0, 12308);
    Free(sMapData as *mut c_void);
    sMapData = null_mut();
}
pub(crate) unsafe extern "C" fn InitFrontierMap() -> u32 {
    match (*sPassData).state {
        0 => {
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            SetVBlankHBlankCallbacksToNull();
        }
        1 => {
            ResetGpuRegsAndBgs();
        }
        2 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            ResetPaletteFade();
            ResetTempTileDataBuffers();
        }
        3 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sMapBgTemplates.as_ptr().cast_mut(), 3);
            SetBgTilemapBuffer(0, (*sMapData).tilemapBuff0.as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(1, (*sMapData).tilemapBuff1.as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(2, (*sMapData).tilemapBuff2.as_mut_ptr() as *mut c_void);
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
        }
        4 => {
            InitWindows(sMapWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            PrintOnFrontierMap();
            DecompressAndCopyTileDataToVram(
                1,
                sMapScreen_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        5 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return FALSE as u32;
            }
            LoadPalette(
                gFrontierPassBg_Pal.as_ptr().cast_mut() as *mut c_void,
                0,
                416,
            );
            LoadPalette(GetTextWindowPalette(0) as *mut c_void, 240, 32);
            CopyToBgTilemapBuffer(
                2,
                sMapScreen_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(2);
        }
        6 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            InitFrontierMapSprites();
            SetVBlankCallback(Some(VBlankCB_FrontierPass));
            BlendPalettes(PALETTES_ALL, 16, 32767);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 32767);
        }
        7 => {
            if UpdatePaletteFade() != 0 {
                return FALSE as u32;
            }
            (*sPassData).state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    (*sPassData).state += 1;
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn ExitFrontierMap() -> u32 {
    match (*sPassData).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 32767);
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return FALSE as u32;
            }
            SetGpuReg(0x0, 0);
            HideBg(0);
            HideBg(1);
            HideBg(2);
        }
        2 => {
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            SetVBlankHBlankCallbacksToNull();
        }
        3 => {
            if !(*sMapData).cursorSprite.is_null() {
                DestroySprite((*sMapData).cursorSprite);
                FreeSpriteTilesByTag(TAG_CURSOR);
            }
            if !(*sMapData).mapIndicatorSprite.is_null() {
                DestroySprite((*sMapData).mapIndicatorSprite);
                FreeSpriteTilesByTag(TAG_MAP_INDICATOR);
            }
            if !(*sMapData).playerHeadSprite.is_null() {
                DestroySprite((*sMapData).playerHeadSprite);
                FreeSpriteTilesByTag(TAG_HEAD_MALE);
            }
            FreeAllWindowBuffers();
        }
        4 => {
            ResetGpuRegsAndBgs();
            ResetSpriteData();
            FreeAllSpritePalettes();
        }
        5 => {
            UnsetBgTilemapBuffer(0);
            UnsetBgTilemapBuffer(1);
            UnsetBgTilemapBuffer(2);
            (*sPassData).state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    (*sPassData).state += 1;
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Task_HandleFrontierMap(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        match *data {
            0 => {
                if InitFrontierMap() != 0 {
                    break 'l1;
                }
                return;
            }
            1 => {
                if gMain.newKeys as i32 & B_BUTTON != 0 {
                    PlaySE(SE_PC_OFF);
                    *data = 4;
                } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
                    if (*sMapData).cursorPos >= 6 {
                        HandleFrontierMapCursorMove(0);
                    } else {
                        *data = 2;
                    }
                } else if gMain.newKeys as i32 & DPAD_UP != 0 {
                    if (*sMapData).cursorPos == 0 {
                        HandleFrontierMapCursorMove(1);
                    } else {
                        *data = 3;
                    }
                }
                return;
            }
            2 => {
                if *data.at(1) > 3 {
                    HandleFrontierMapCursorMove(0);
                    *data.at(1) = 0;
                    *data = 1;
                } else {
                    (*(*sMapData).cursorSprite).y += 4;
                    *data.at(1) += 1;
                }
                return;
            }
            3 => {
                if *data.at(1) > 3 {
                    HandleFrontierMapCursorMove(1);
                    *data.at(1) = 0;
                    *data = 1;
                } else {
                    (*(*sMapData).cursorSprite).y -= 4;
                    *data.at(1) += 1;
                }
                return;
            }
            4 => {
                if ExitFrontierMap() != 0 {
                    break 'l1;
                }
                return;
            }
            5 => {
                DestroyTask(taskId);
                FreeFrontierMap();
                return;
            }
            _ => {}
        }
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn MapNumToFrontierFacilityId(mapNum: u16) -> u8 {
    if mapNum >= 5 && mapNum <= 8 || mapNum >= 15 && mapNum <= 17 {
        return 1;
    } else if mapNum == 18 || mapNum == 19 || mapNum == 20 || mapNum == 21 {
        return 2;
    } else if mapNum == 22 || mapNum == 23 || mapNum == 24 {
        return 3;
    } else if mapNum == 28 || mapNum == 29 || mapNum == 30 {
        return 4;
    } else if mapNum == 31 || mapNum == 32 || mapNum == 33 {
        return 5;
    } else if mapNum == 34
        || mapNum == 35
        || mapNum == 36
        || mapNum == 37
        || mapNum == 38
        || mapNum == 39
    {
        return 6;
    } else if mapNum == 25 || mapNum == 26 || mapNum == 27 {
        return 7;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InitFrontierMapSprites() {
    let mut sprite: SpriteTemplate = zeroed();
    let mut spriteId: u8 = 0;
    let mut id: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    FreeAllSpritePalettes();
    LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
    LoadCompressedSpriteSheet((&raw const sCursorSpriteSheets[0]).cast_mut());
    spriteId = CreateSprite(
        (&raw const sSpriteTemplates_Cursors[0]).cast_mut(),
        155,
        (*sMapData).cursorPos as i16 * 16 + 8,
        2,
    );
    (*sMapData).cursorSprite = &raw mut gSprites[spriteId];
    (*(*sMapData).cursorSprite).oam.set_priority(0);
    (*(*sMapData).cursorSprite).set_hFlip(TRUE as u16);
    StartSpriteAnim((*sMapData).cursorSprite, 1);
    LoadCompressedSpriteSheet((&raw const sCursorSpriteSheets[1]).cast_mut());
    spriteId = CreateSprite(
        (&raw const sSpriteTemplates_Cursors[1]).cast_mut(),
        sMapLandmarks[(*sMapData).cursorPos].x,
        sMapLandmarks[(*sMapData).cursorPos].y,
        1,
    );
    (*sMapData).mapIndicatorSprite = &raw mut gSprites[spriteId];
    (*(*sMapData).mapIndicatorSprite).oam.set_priority(0);
    StartSpriteAnim(
        (*sMapData).mapIndicatorSprite,
        sMapLandmarks[(*sMapData).cursorPos].animNum,
    );
    id = GetCurrentRegionMapSectionId();
    if id == MAPSEC_BATTLE_FRONTIER || id == MAPSEC_ARTISAN_CAVE {
        let mut mapNum: i8 = (*gSaveBlock1Ptr).location.mapNum;
        if mapNum == 4
            || mapNum == 14
                && ({
                    x = 55;
                    x
                }) != 0
        {
            x += (*gSaveBlock1Ptr).pos.x;
            y = (*gSaveBlock1Ptr).pos.y;
            x = x / 8;
            y = y / 8;
            id = 0;
        } else {
            id = MapNumToFrontierFacilityId(mapNum as u16);
            if id != 0 {
                x = sMapLandmarks[id as i32 - 1].x;
                y = sMapLandmarks[id as i32 - 1].y;
            } else {
                if (*gSaveBlock1Ptr).escapeWarp.mapNum == 14 {
                    x = (*gSaveBlock1Ptr).escapeWarp.x + 55;
                } else {
                    x = (*gSaveBlock1Ptr).escapeWarp.x;
                }
                y = (*gSaveBlock1Ptr).escapeWarp.y;
                x = x / 8;
                y = y / 8;
            }
        }
        LoadCompressedSpriteSheet(sHeadsSpriteSheet.as_ptr().cast_mut());
        sprite = *sSpriteTemplate_PlayerHead;
        sprite.paletteTag = (*gSaveBlock2Ptr).playerGender as u16 + TAG_HEAD_MALE;
        if id != 0 {
            spriteId = CreateSprite(&raw mut sprite, x, y, 0);
        } else {
            x *= 8;
            y *= 8;
            spriteId = CreateSprite(&raw mut sprite, x + 20, y + 36, 0);
        }
        (*sMapData).playerHeadSprite = &raw mut gSprites[spriteId];
        (*(*sMapData).playerHeadSprite).oam.set_priority(0);
        if (*gSaveBlock2Ptr).playerGender != MALE {
            StartSpriteAnim((*sMapData).playerHeadSprite, 1);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintOnFrontierMap() {
    let mut i: u8 = 0;
    i = 0;
    while i < MAP_WINDOW_COUNT {
        PutWindowTilemap(i);
        FillWindowPixelBuffer(i, 0);
        i += 1;
    }
    i = 0;
    while i < NUM_FRONTIER_FACILITIES {
        if i == (*sMapData).cursorPos {
            AddTextPrinterParameterized3(
                MAP_WINDOW_NAME,
                FONT_NARROW,
                4,
                i * 16 + 1,
                sTextColors[2].as_ptr().cast_mut(),
                0,
                sMapLandmarks[i].name,
            );
        } else {
            AddTextPrinterParameterized3(
                MAP_WINDOW_NAME,
                FONT_NARROW,
                4,
                i * 16 + 1,
                sTextColors[1].as_ptr().cast_mut(),
                0,
                sMapLandmarks[i].name,
            );
        }
        i += 1;
    }
    AddTextPrinterParameterized3(
        MAP_WINDOW_DESCRIPTION,
        FONT_NORMAL,
        4,
        0,
        sTextColors[0].as_ptr().cast_mut(),
        0,
        sMapLandmarks[(*sMapData).cursorPos].description,
    );
    i = 0;
    while i < MAP_WINDOW_COUNT {
        CopyWindowToVram(i, COPYWIN_FULL);
        i += 1;
    }
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn HandleFrontierMapCursorMove(direction: u8) {
    let mut oldCursorPos: u8 = 0;
    let mut i: u8 = 0;
    if direction != 0 {
        oldCursorPos = (*sMapData).cursorPos;
        (*sMapData).cursorPos = ((oldCursorPos as i32 + 6) % 7) as u8;
    } else {
        oldCursorPos = (*sMapData).cursorPos;
        (*sMapData).cursorPos = ((oldCursorPos as i32 + 1) % 7) as u8;
    }
    AddTextPrinterParameterized3(
        MAP_WINDOW_NAME,
        FONT_NARROW,
        4,
        oldCursorPos * 16 + 1,
        sTextColors[1].as_ptr().cast_mut(),
        0,
        sMapLandmarks[oldCursorPos].name,
    );
    AddTextPrinterParameterized3(
        MAP_WINDOW_NAME,
        FONT_NARROW,
        4,
        (*sMapData).cursorPos * 16 + 1,
        sTextColors[2].as_ptr().cast_mut(),
        0,
        sMapLandmarks[(*sMapData).cursorPos].name,
    );
    (*(*sMapData).cursorSprite).y = (*sMapData).cursorPos as i16 * 16 + 8;
    StartSpriteAnim(
        (*sMapData).mapIndicatorSprite,
        sMapLandmarks[(*sMapData).cursorPos].animNum,
    );
    (*(*sMapData).mapIndicatorSprite).x = sMapLandmarks[(*sMapData).cursorPos].x;
    (*(*sMapData).mapIndicatorSprite).y = sMapLandmarks[(*sMapData).cursorPos].y;
    FillWindowPixelBuffer(MAP_WINDOW_DESCRIPTION, 0);
    AddTextPrinterParameterized3(
        MAP_WINDOW_DESCRIPTION,
        FONT_NORMAL,
        4,
        0,
        sTextColors[0].as_ptr().cast_mut(),
        0,
        sMapLandmarks[(*sMapData).cursorPos].description,
    );
    i = 0;
    while i < MAP_WINDOW_COUNT {
        CopyWindowToVram(i, COPYWIN_FULL);
        i += 1;
    }
    CopyBgTilemapBufferToVram(0);
    PlaySE(SE_DEX_SCROLL);
}
