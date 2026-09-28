//! Translated from `src/intro.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sIntroDrops_Pal sIntroLogo_Pal sIntroDropsLogo_Gfx sIntro1Bg_Pal sIntro1Bg0_Tilemap sIntro1Bg1_Tilemap sIntro1Bg2_Tilemap sIntro1Bg3_Tilemap sIntro1Bg_Gfx sIntroPokeball_Pal sIntroPokeball_Tilemap sIntroPokeball_Gfx sIntroStreaks_Pal sIntroStreaks_Gfx sIntroStreaks_Tilemap sIntroRayquzaOrb_Pal sIntroMisc_Pal sIntroMisc_Gfx sIntroFlygonSilhouette_Pal sIntroLati_Gfx sUnusedData sSpriteSheet_Sparkle sSpritePalette_Sparkle sOamData_Sparkle sAnim_Sparkle sAnims_Sparkle sSpriteTemplate_Sparkle sSparkleCoords sSpriteSheet_RunningPokemon sSpritePalettes_RunningPokemon sOamData_Volbeat sAnim_Volbeat sAnims_Volbeat sSpriteTemplate_Volbeat sOamData_Torchic sAnim_Torchic_Walk sAnim_Torchic_Run sAnim_Torchic_Trip sAnims_Torchic sSpriteTemplate_Torchic sOamData_Manectric sAnim_Manectric sAnims_Manectric sSpriteTemplate_Manectric sSpriteSheet_Lightning sSpritePalette_Lightning sOamData_Lightning sAnim_Lightning_Top sAnim_Lightning_Middle sAnim_Lightning_Bottom sAnims_Lightning sSpriteTemplate_Lightning sGroudonRockData sSpriteSheet_Bubbles sSpritePalette_Bubbles sKyogreBubbleData sOamData_Bubbles sAnim_Bubbles sAnims_Bubbles sSpriteTemplate_Bubbles sOamData_WaterDrop sAnim_WaterDrop_UpperHalf sAnim_WaterDrop_LowerHalf sAnim_WaterDrop_Reflection sAnim_WaterDrop_Ripple sAnims_WaterDrop sSpriteTemplate_WaterDrop sAnim_PlayerBicycle_Fast sAnim_PlayerBicycle_Slow sAnim_PlayerBicycle_LookBack sAnim_PlayerBicycle_LookForward sAnims_PlayerBicycle sOamData_GameFreakLetter sOamData_PresentsLetter sOamData_GameFreakLogo sAnim_GameFreakLetter_G sAnim_GameFreakLetter_A sAnim_GameFreakLetter_M sAnim_GameFreakLetter_E sAnim_GameFreakLetter_F sAnim_GameFreakLetter_R sAnim_GameFreakLetter_K sAnim_PresentsLetter_P sAnim_PresentsLetter_R sAnim_PresentsLetter_E sAnim_PresentsLetter_S sAnim_PresentsLetter_N sAnim_PresentsLetter_T sAnim_GameFreakLogo sAnims_GameFreakLetter sAnims_PresentsLetter sAnims_GameFreakLogo sGameFreakLetterData sPresentsLetterData sAffineAnim_GameFreak_Small sAffineAnim_GameFreak_GrowAndShrink sAffineAnim_GameFreak_GrowBig sAffineAnim_GameFreak_GrowMedium sAffineAnims_GameFreak sGameFreakLettersMoveSpeed sSpriteTemplate_GameFreakLetter sSpriteTemplate_PresentsLetter sSpriteTemplate_GameFreakLogo sGameFreakLetterStartDelays sOamData_FlygonSilhouette sAnim_FlygonSilhouette sAnims_FlygonSilhouette sSpriteTemplate_FlygonSilhouette sSpriteSheet_WaterDropsAndLogo sSpriteSheet_FlygonSilhouette sSpritePalettes_Intro1 sOamData_RayquazaOrb sAnim_RayquazaOrb sAnims_RayquazaOrb sSpriteTemplate_RayquazaOrb sSpriteSheet_RayquazaOrb sSpritePalette_RayquazaOrb

const COLOR_CHANGES: i16 = 9;
const COLOSSEUM_GAME_CODE: u32 = 0x65366347;
const COPYRIGHT_INITIALIZE: u8 = 0;
const COPYRIGHT_START_FADE: u8 = 140;
const COPYRIGHT_START_INTRO: u8 = 141;
const DROP_ANIM_LOWER_HALF: u8 = 1;
const DROP_ANIM_REFLECTION: u8 = 2;
const DROP_ANIM_RIPPLE: u8 = 3;
const NARROW_HEIGHT: i16 = 32;
const NUM_BUBBLES_IN_SET: i32 = 6;
const NUM_GF_LETTERS: u16 = 9;
const TIMER_BIG_DROP_FALLS: u32 = 251;
const TIMER_BIG_DROP_START: u32 = 76;
const TIMER_END_PAN_UP: u32 = 904;
const TIMER_END_SCENE_1: u32 = 1007;
const TIMER_END_SCENE_2: u32 = 1946;
const TIMER_FLYGON_ENTER: u32 = 1394;
const TIMER_FLYGON_SILHOUETTE_APPEAR: u32 = 832;
const TIMER_LOGO_APPEAR: u32 = 128;
const TIMER_LOGO_BLEND_OUT: u32 = 256;
const TIMER_LOGO_DISAPPEAR: u32 = 272;
const TIMER_LOGO_LETTERS_COLOR: u32 = 144;
const TIMER_MANECTRIC_ENTER: u32 = 1088;
const TIMER_MANECTRIC_RUN_CIRCULAR: u32 = 1168;
const TIMER_PLAYER_DRIFT_BACK: u32 = 1109;
const TIMER_PLAYER_EXIT: u32 = 1727;
const TIMER_PLAYER_HOLD_POSITION: u32 = 1576;
const TIMER_PLAYER_MOVE_BACKWARD: u32 = 1398;
const TIMER_PLAYER_MOVE_FORWARD: u32 = 1214;
const TIMER_POKEBALL_FADE: u32 = 28;
const TIMER_SMALL_DROP_1: u32 = 368;
const TIMER_SMALL_DROP_2: u32 = 384;
const TIMER_SPARKLES: u32 = 560;
const TIMER_START_LEGENDARIES: u32 = 43;
const TIMER_START_SCENE_2: u32 = 1026;
const TIMER_START_SCENE_3: u32 = 2068;
const TIMER_TORCHIC_ENTER: u32 = 1224;
const TIMER_TORCHIC_EXIT: u32 = 1856;
const TIMER_TORCHIC_SPEED_UP: u32 = 1735;
const TORCHIC_ANIM_RUN: u8 = 1;
const TORCHIC_ANIM_TRIP: u8 = 2;
const TORCHIC_ANIM_WALK: u8 = 0;
const VOLBEAT_ENTER: i16 = 1;
const VOLBEAT_EXIT: i16 = 7;
const VOLBEAT_FIGURE_8: i16 = 6;
const VOLBEAT_INIT_FIGURE_8: i16 = 5;
const VOLBEAT_WAIT_ENTER: i16 = 0;
const VOLBEAT_WAIT_STATE: i16 = 8;
const VOLBEAT_ZIP_BACKWARD: i16 = 2;
const VOLBEAT_ZIP_DOWN: i16 = 3;
const VOLBEAT_ZIP_FORWARD: i16 = 4;

static sAnims_PlayerBicycle: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::intro::sAnims_PlayerBicycle).cast());
static sGameFreakLetterData: Table<CArray<CArray<i16, 2>, 9>> =
    Table((&raw const crate::data::intro::sGameFreakLetterData).cast());
static sGameFreakLetterStartDelays: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::intro::sGameFreakLetterStartDelays).cast());
static sGameFreakLettersMoveSpeed: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::intro::sGameFreakLettersMoveSpeed).cast());
static sGroudonRockData: Table<CArray<CArray<i16, 3>, 6>> =
    Table((&raw const crate::data::intro::sGroudonRockData).cast());
static sIntro1Bg0_Tilemap: Table<CArray<u32, 237>> =
    Table((&raw const crate::data::intro::sIntro1Bg0_Tilemap).cast());
static sIntro1Bg1_Tilemap: Table<CArray<u32, 205>> =
    Table((&raw const crate::data::intro::sIntro1Bg1_Tilemap).cast());
static sIntro1Bg2_Tilemap: Table<CArray<u32, 188>> =
    Table((&raw const crate::data::intro::sIntro1Bg2_Tilemap).cast());
static sIntro1Bg3_Tilemap: Table<CArray<u32, 134>> =
    Table((&raw const crate::data::intro::sIntro1Bg3_Tilemap).cast());
static sIntro1Bg_Gfx: Table<CArray<u32, 2140>> =
    Table((&raw const crate::data::intro::sIntro1Bg_Gfx).cast());
static sIntro1Bg_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::intro::sIntro1Bg_Pal).cast());
static sIntroPokeball_Gfx: Table<CArray<u32, 725>> =
    Table((&raw const crate::data::intro::sIntroPokeball_Gfx).cast());
static sIntroPokeball_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::intro::sIntroPokeball_Pal).cast());
static sIntroPokeball_Tilemap: Table<CArray<u32, 76>> =
    Table((&raw const crate::data::intro::sIntroPokeball_Tilemap).cast());
static sKyogreBubbleData: Table<CArray<CArray<i16, 3>, 12>> =
    Table((&raw const crate::data::intro::sKyogreBubbleData).cast());
static sSparkleCoords: Table<CArray<CArray<u8, 2>, 12>> =
    Table((&raw const crate::data::intro::sSparkleCoords).cast());
static sSpritePalette_Bubbles: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_Bubbles).cast());
static sSpritePalette_Lightning: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_Lightning).cast());
static sSpritePalette_RayquazaOrb: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_RayquazaOrb).cast());
static sSpritePalette_Sparkle: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::intro::sSpritePalette_Sparkle).cast());
static sSpritePalettes_Intro1: Table<CArray<SpritePalette, 4>> =
    Table((&raw const crate::data::intro::sSpritePalettes_Intro1).cast());
static sSpritePalettes_RunningPokemon: Table<CArray<SpritePalette, 4>> =
    Table((&raw const crate::data::intro::sSpritePalettes_RunningPokemon).cast());
static sSpriteSheet_Bubbles: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_Bubbles).cast());
static sSpriteSheet_FlygonSilhouette: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_FlygonSilhouette).cast());
static sSpriteSheet_Lightning: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_Lightning).cast());
static sSpriteSheet_RayquazaOrb: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_RayquazaOrb).cast());
static sSpriteSheet_RunningPokemon: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::intro::sSpriteSheet_RunningPokemon).cast());
static sSpriteSheet_Sparkle: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_Sparkle).cast());
static sSpriteSheet_WaterDropsAndLogo: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro::sSpriteSheet_WaterDropsAndLogo).cast());
static sSpriteTemplate_Bubbles: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Bubbles).cast());
static sSpriteTemplate_FlygonSilhouette: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_FlygonSilhouette).cast());
static sSpriteTemplate_GameFreakLetter: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_GameFreakLetter).cast());
static sSpriteTemplate_GameFreakLogo: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_GameFreakLogo).cast());
static sSpriteTemplate_Lightning: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Lightning).cast());
static sSpriteTemplate_Manectric: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Manectric).cast());
static sSpriteTemplate_RayquazaOrb: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_RayquazaOrb).cast());
static sSpriteTemplate_Sparkle: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Sparkle).cast());
static sSpriteTemplate_Torchic: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Torchic).cast());
static sSpriteTemplate_Volbeat: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_Volbeat).cast());
static sSpriteTemplate_WaterDrop: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro::sSpriteTemplate_WaterDrop).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIntroCharacterGender: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedVar: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFlygonYOffset: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gIntroFrameCounter: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMultibootProgramStruct: GcmbStruct = unsafe { zeroed() };

unsafe extern "C" {
    static gAncientPowerRockSpriteTemplate: SpriteTemplate;
    static gBattleAnimPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gBattleAnimPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gHeap: CArray<u8, 114688>;
    static gIntro3Bg_Pal: CArray<CArray<u16, 16>, 16>;
    static gIntroCloudsLeft_Tilemap: CArray<u32, 0>;
    static gIntroCloudsRight_Tilemap: CArray<u32, 0>;
    static gIntroCloudsSun_Tilemap: CArray<u32, 0>;
    static gIntroClouds_Gfx: CArray<u32, 0>;
    static gIntroCopyright_Gfx: CArray<u32, 0>;
    static gIntroCopyright_Pal: CArray<u16, 16>;
    static gIntroCopyright_Tilemap: CArray<u32, 0>;
    static mut gIntroCredits_MovingSceneryState: i16;
    static mut gIntroCredits_MovingSceneryVBase: u16;
    static mut gIntroCredits_MovingSceneryVOffset: i16;
    static gIntroGameFreakTextFade_Pal: CArray<u16, 0>;
    static gIntroGroudonBg_Tilemap: CArray<u32, 0>;
    static gIntroGroudon_Gfx: CArray<u32, 0>;
    static gIntroGroudon_Tilemap: CArray<u32, 0>;
    static gIntroKyogreBg_Tilemap: CArray<u32, 0>;
    static gIntroKyogre_Gfx: CArray<u32, 0>;
    static gIntroKyogre_Tilemap: CArray<u32, 0>;
    static gIntroLegendBg_Gfx: CArray<u32, 0>;
    static gIntroRayquazaClouds_Gfx: CArray<u32, 0>;
    static gIntroRayquazaClouds_Tilemap: CArray<u32, 0>;
    static gIntroRayquaza_Gfx: CArray<u32, 0>;
    static gIntroRayquaza_Tilemap: CArray<u32, 0>;
    static mut gMain: Main;
    static gMultiBootProgram_PokemonColosseum_Start: CArray<u16, 81920>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSaveFileStatus: u16;
    static mut gScanlineEffect: ScanlineEffect;
    static gSineTable: CArray<i16, 0>;
    static gSpritePalettes_IntroPlayerFlygon: CArray<SpritePalette, 0>;
    static gSpriteSheet_IntroBicycle: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_IntroBrendan: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_IntroFlygon: CArray<CompressedSpriteSheet, 0>;
    static gSpriteSheet_IntroMay: CArray<CompressedSpriteSheet, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gTitleScreenAlphaBlend: CArray<u16, 64>;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BgAffineSet(a0: *mut BgAffineSrcData, a1: *mut BgAffineDstData, a2: i32);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BuildOamBuffer();
    fn CB2_InitTitleScreen();
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateBicycleBgAnimationTask(a0: u8, a1: u16, a2: u16, a3: u16) -> u8;
    fn CreateIntroBrendanSprite(a0: i16, a1: i16) -> u8;
    fn CreateIntroFlygonSprite(a0: i16, a1: i16) -> u8;
    fn CreateIntroMaySprite(a0: i16, a1: i16) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CycleSceneryPalette(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FreeAllSpritePalettes();
    fn GameCubeMultiBoot_ExecuteProgram(a0: *mut GcmbStruct);
    fn GameCubeMultiBoot_HandleSerialInterrupt(a0: *mut GcmbStruct);
    fn GameCubeMultiBoot_Init(a0: *mut GcmbStruct);
    fn GameCubeMultiBoot_Main(a0: *mut GcmbStruct);
    fn GameCubeMultiBoot_Quit();
    fn GetSaveBlocksPointersBaseOffset() -> u16;
    fn InitHeap(a0: *mut c_void, a1: u32);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut CompressedSpritePalette) -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8;
    fn LoadGameSave(a0: u8) -> u8;
    fn LoadIntroPart2Graphics(a0: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn PlayCryInternal(a0: u16, a1: i8, a2: i8, a3: u8, a4: u8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetMenuAndMonGlobals();
    fn ResetPaletteFade();
    fn ResetSerial();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn Sav2_ClearSetDefault();
    fn Save_ResetSaveCounters();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_InitWave(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn ScanlineEffect_Stop();
    fn SerialCB();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetIntroPart2BgCnt(a0: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetPokemonCryStereo(a0: u32);
    fn SetSaveBlocksPointers(a0: u16);
    fn SetSerialCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn VBlankCB_Intro() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn MainCB2_Intro() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    if gMain.newKeys != 0 && gPaletteFade.active() == 0 {
        SetMainCallback2(Some(MainCB2_EndIntro));
    } else if gIntroFrameCounter != 0xffffffff {
        gIntroFrameCounter += 1;
    }
}
pub(crate) unsafe extern "C" fn MainCB2_EndIntro() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2(Some(CB2_InitTitleScreen));
    }
}
pub(crate) unsafe extern "C" fn LoadCopyrightGraphics(
    tilesetAddress: u16,
    tilemapAddress: u16,
    paletteOffset: u16,
) {
    LZ77UnCompVram(
        gIntroCopyright_Gfx.as_ptr().cast_mut(),
        (VRAM + tilesetAddress as i32) as usize as *mut c_void,
    );
    LZ77UnCompVram(
        gIntroCopyright_Tilemap.as_ptr().cast_mut(),
        (VRAM + tilemapAddress as i32) as usize as *mut c_void,
    );
    LoadPalette(
        gIntroCopyright_Pal.as_ptr().cast_mut() as *mut c_void,
        paletteOffset,
        32,
    );
}
pub(crate) unsafe extern "C" fn SerialCB_CopyrightScreen() {
    GameCubeMultiBoot_HandleSerialInterrupt(&raw mut gMultibootProgramStruct);
}
pub(crate) unsafe extern "C" fn SetUpCopyrightScreen() -> u8 {
    'l1: {
        let sw1: u8 = gMain.state;
        let matched = sw1 == COPYRIGHT_INITIALIZE
            || sw1 == COPYRIGHT_START_FADE
            || sw1 == COPYRIGHT_START_INTRO;
        let mut fall = false;
        if sw1 == COPYRIGHT_INITIALIZE {
            fall = true;
            SetVBlankCallback(None);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            *(PLTT as i32 as usize as *mut u16) = 32767;
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BG0HOFS, 0);
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        VRAM as usize as *mut c_void,
                        0x5006000,
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
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        83886082 as usize as *mut c_void,
                        0x10001ff,
                    );
                }
            }
            ResetPaletteFade();
            LoadCopyrightGraphics(0, 0x3800, 0);
            ScanlineEffect_Stop();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
            SetGpuReg(REG_OFFSET_BG0CNT, 1792);
            EnableInterrupts(INTR_FLAG_VBLANK);
            SetVBlankCallback(Some(VBlankCB_Intro));
            volatile_write(0x4000000 as usize as *mut u16, 320);
            SetSerialCallback(Some(SerialCB_CopyrightScreen));
            GameCubeMultiBoot_Init(&raw mut gMultibootProgramStruct);
        }
        if fall || !matched {
            fall = true;
            UpdatePaletteFade();
            gMain.state += 1;
            GameCubeMultiBoot_Main(&raw mut gMultibootProgramStruct);
            break 'l1;
        }
        if sw1 == COPYRIGHT_START_FADE {
            fall = true;
            GameCubeMultiBoot_Main(&raw mut gMultibootProgramStruct);
            if (&raw mut gMultibootProgramStruct.gcmb_field_2).read_volatile() != 1 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == COPYRIGHT_START_INTRO {
            fall = true;
            if UpdatePaletteFade() != 0 {
                break 'l1;
            }
            CreateTask(Some(Task_Scene1_Load), 0);
            SetMainCallback2(Some(MainCB2_Intro));
            if (&raw mut gMultibootProgramStruct.gcmb_field_2).read_volatile() != 0 {
                if (&raw mut gMultibootProgramStruct.gcmb_field_2).read_volatile() == 2 {
                    if *(33554604 as usize as *mut u32) == COLOSSEUM_GAME_CODE {
                        CpuSet(
                            (&raw const gMultiBootProgram_PokemonColosseum_Start).cast_mut()
                                as *mut c_void,
                            EWRAM_START as i32 as usize as *mut c_void,
                            0x14000,
                        );
                        *(33554604 as usize as *mut u32) = COLOSSEUM_GAME_CODE;
                    }
                    GameCubeMultiBoot_ExecuteProgram(&raw mut gMultibootProgramStruct);
                }
            } else {
                GameCubeMultiBoot_Quit();
                SetSerialCallback(Some(SerialCB));
            }
            return 0;
        }
    }
    return 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitCopyrightScreenAfterBootup() {
    if SetUpCopyrightScreen() == 0 {
        SetSaveBlocksPointers(GetSaveBlocksPointersBaseOffset());
        ResetMenuAndMonGlobals();
        Save_ResetSaveCounters();
        LoadGameSave(SAVE_NORMAL);
        if gSaveFileStatus == SAVE_STATUS_EMPTY as u16 || gSaveFileStatus == SAVE_STATUS_CORRUPT {
            Sav2_ClearSetDefault();
        }
        SetPokemonCryStereo((*gSaveBlock2Ptr).optionsSound() as u32);
        InitHeap(gHeap.as_mut_ptr() as *mut c_void, HEAP_SIZE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitCopyrightScreenAfterTitleScreen() {
    SetUpCopyrightScreen();
}
pub(crate) unsafe extern "C" fn Task_Scene1_Load(taskId: u8) {
    SetVBlankCallback(None);
    sIntroCharacterGender = (if 0 != 0 {
        Random() as i32 % 2
    } else {
        Random() as i32 & 1
    }) as u16;
    IntroResetGpuRegs();
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 80);
    SetGpuReg(REG_OFFSET_BG1VOFS, 24);
    SetGpuReg(REG_OFFSET_BG0VOFS, 40);
    LZ77UnCompVram(
        sIntro1Bg_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sIntro1Bg0_Tilemap.as_ptr().cast_mut(),
        0x6008000 as usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x6008800 as usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        sIntro1Bg1_Tilemap.as_ptr().cast_mut(),
        0x6009000 as usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x6009800 as usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        sIntro1Bg2_Tilemap.as_ptr().cast_mut(),
        0x600a000 as usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x600a800 as usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        sIntro1Bg3_Tilemap.as_ptr().cast_mut(),
        0x600b000 as usize as *mut c_void,
    );
    {
        {
            let mut _dest: *mut u16 = 0x600b800 as usize as *mut u16;
            let mut _size: u32 = BG_SCREEN_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LoadPalette(sIntro1Bg_Pal.as_ptr().cast_mut() as *mut c_void, 0, 512);
    SetGpuReg(REG_OFFSET_BG3CNT, 38403);
    SetGpuReg(REG_OFFSET_BG2CNT, 37890);
    SetGpuReg(REG_OFFSET_BG1CNT, 37377);
    SetGpuReg(REG_OFFSET_BG0CNT, 36864);
    LoadCompressedSpriteSheet(sSpriteSheet_WaterDropsAndLogo.as_ptr().cast_mut());
    LoadCompressedSpriteSheet(sSpriteSheet_FlygonSilhouette.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalettes_Intro1.as_ptr().cast_mut());
    LoadCompressedSpriteSheet(sSpriteSheet_Sparkle.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalette_Sparkle.as_ptr().cast_mut());
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[496] as *mut c_void,
        16,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[481] as *mut c_void,
        15,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[466] as *mut c_void,
        14,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[451] as *mut c_void,
        13,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[436] as *mut c_void,
        12,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[421] as *mut c_void,
        11,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[256] as *mut c_void,
        &raw mut gPlttBufferUnfaded[406] as *mut c_void,
        10,
    );
    CreateGameFreakLogoSprites(120, 80, 0);
    gTasks[taskId].data[0] = CreateWaterDrop(236, -14, 0x200, 1, 0x78, FALSE) as i16;
    gTasks[taskId].func = Some(Task_Scene1_FadeIn);
}
pub(crate) unsafe extern "C" fn Task_Scene1_FadeIn(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    SetVBlankCallback(Some(VBlankCB_Intro));
    SetGpuReg(0x0, 8000);
    gTasks[taskId].func = Some(Task_Scene1_WaterDrops);
    gIntroFrameCounter = 0;
    m4aSongNumStart(MUS_INTRO);
    ResetSerial();
}
pub(crate) unsafe extern "C" fn Task_Scene1_WaterDrops(taskId: u8) {
    if gIntroFrameCounter == TIMER_BIG_DROP_START {
        gSprites[gTasks[taskId].data[0]].data[0] = 1;
    }
    if gIntroFrameCounter == TIMER_LOGO_APPEAR {
        CreateTask(Some(Task_BlendLogoIn), 0);
    }
    if gIntroFrameCounter == TIMER_BIG_DROP_FALLS {
        gSprites[gTasks[taskId].data[0]].data[0] = 2;
    }
    if gIntroFrameCounter == TIMER_LOGO_BLEND_OUT {
        CreateTask(Some(Task_BlendLogoOut), 0);
    }
    if gIntroFrameCounter == TIMER_SMALL_DROP_1 {
        CreateWaterDrop(48, 0, 0x400, 5, 0x70, TRUE);
    }
    if gIntroFrameCounter == TIMER_SMALL_DROP_2 {
        CreateWaterDrop(200, 60, 0x400, 9, 0x80, TRUE);
    }
    if gIntroFrameCounter == TIMER_SPARKLES {
        CreateTask(Some(Task_CreateSparkles), 0);
    }
    if gIntroFrameCounter > TIMER_SPARKLES {
        gTasks[taskId].data[1] = 80;
        gTasks[taskId].data[2] = 0;
        gTasks[taskId].data[3] = 24;
        gTasks[taskId].data[4] = 0;
        gTasks[taskId].data[5] = 40;
        gTasks[taskId].data[6] = 0;
        gTasks[taskId].func = Some(Task_Scene1_PanUp);
    }
}
pub(crate) unsafe extern "C" fn Task_CreateSparkles(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if ({
        *data.at(2) += 1;
        *data.at(2)
    }) as i32
        & 1
        != 0
    {
        *data.at(3) += 1;
    }
    match *data {
        0 => {
            CreateSprite(
                (&raw const *sSpriteTemplate_Sparkle).cast_mut(),
                sSparkleCoords[*data.at(4)][0] as i16,
                sSparkleCoords[*data.at(4)][1] as i16 + *data.at(3),
                0,
            );
            *data += 1;
            *data.at(1) = 12;
            *data.at(4) += 1;
        }
        1 => {
            if ({
                *data.at(1) -= 1;
                *data.at(1)
            }) == 0
            {
                *data = 0;
            }
        }
        _ => {}
    }
    if *data.at(3) > 60 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 12
    {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_PanUp(taskId: u8) {
    if gIntroFrameCounter < TIMER_END_PAN_UP {
        let mut offset: i32 = 0;
        offset = ((gTasks[taskId].data[1] as i32) << 16) + gTasks[taskId].data[2] as u16 as i32;
        offset -= 0x6000;
        gTasks[taskId].data[1] = (offset >> 16) as i16;
        gTasks[taskId].data[2] = offset as i16;
        SetGpuReg(REG_OFFSET_BG2VOFS, gTasks[taskId].data[1] as u16);
        offset = ((gTasks[taskId].data[3] as i32) << 16) + gTasks[taskId].data[4] as u16 as i32;
        offset -= 0x8000;
        gTasks[taskId].data[3] = (offset >> 16) as i16;
        gTasks[taskId].data[4] = offset as i16;
        SetGpuReg(REG_OFFSET_BG1VOFS, gTasks[taskId].data[3] as u16);
        offset = ((gTasks[taskId].data[5] as i32) << 16) + gTasks[taskId].data[6] as u16 as i32;
        offset -= 0xC000;
        gTasks[taskId].data[5] = (offset >> 16) as i16;
        gTasks[taskId].data[6] = offset as i16;
        SetGpuReg(REG_OFFSET_BG0VOFS, gTasks[taskId].data[5] as u16);
        if gIntroFrameCounter == TIMER_FLYGON_SILHOUETTE_APPEAR {
            let mut spriteId: u8 = CreateSprite(
                (&raw const *sSpriteTemplate_FlygonSilhouette).cast_mut(),
                120,
                DISPLAY_HEIGHT as i16,
                10,
            );
            gSprites[spriteId].set_invisible(TRUE as u16);
        }
    } else {
        if gIntroFrameCounter > TIMER_END_SCENE_1 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 65535);
            gTasks[taskId].func = Some(Task_Scene1_End);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene1_End(taskId: u8) {
    if gIntroFrameCounter > TIMER_START_SCENE_2 {
        gTasks[taskId].func = Some(Task_Scene2_Load);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene2_Load(taskId: u8) {
    IntroResetGpuRegs();
    SetVBlankCallback(None);
    ResetSpriteData();
    FreeAllSpritePalettes();
    gIntroCredits_MovingSceneryVBase = 0;
    gIntroCredits_MovingSceneryVOffset = 0;
    sFlygonYOffset = 0;
    LoadIntroPart2Graphics(1);
    gTasks[taskId].func = Some(Task_Scene2_CreateSprites);
}
pub(crate) unsafe extern "C" fn Task_Scene2_CreateSprites(taskId: u8) {
    let mut spriteId: u8 = 0;
    if sIntroCharacterGender == MALE as u16 {
        LoadCompressedSpriteSheet(gSpriteSheet_IntroBrendan.as_ptr().cast_mut());
    } else {
        LoadCompressedSpriteSheet(gSpriteSheet_IntroMay.as_ptr().cast_mut());
    }
    LoadCompressedSpriteSheet(gSpriteSheet_IntroBicycle.as_ptr().cast_mut());
    LoadCompressedSpriteSheet(gSpriteSheet_IntroFlygon.as_ptr().cast_mut());
    spriteId = 0;
    while spriteId < 3 {
        LoadCompressedSpriteSheet((&raw const sSpriteSheet_RunningPokemon[spriteId]).cast_mut());
        spriteId += 1;
    }
    LoadSpritePalettes(gSpritePalettes_IntroPlayerFlygon.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalettes_RunningPokemon.as_ptr().cast_mut());
    CreateSprite(
        (&raw const *sSpriteTemplate_Manectric).cast_mut(),
        272,
        128,
        0,
    );
    CreateSprite(
        (&raw const *sSpriteTemplate_Torchic).cast_mut(),
        288,
        110,
        1,
    );
    if sIntroCharacterGender == MALE as u16 {
        spriteId = CreateIntroBrendanSprite(272, 100);
    } else {
        spriteId = CreateIntroMaySprite(272, 100);
    }
    gSprites[spriteId].callback = Some(SpriteCB_PlayerOnBicycle);
    gSprites[spriteId].anims = sAnims_PlayerBicycle.as_ptr().cast_mut();
    gTasks[taskId].data[1] = spriteId as i16;
    CreateSprite((&raw const *sSpriteTemplate_Volbeat).cast_mut(), 272, 80, 4);
    spriteId = CreateIntroFlygonSprite(-64, 60);
    gSprites[spriteId].callback = Some(SpriteCB_Flygon);
    gTasks[taskId].data[2] = spriteId as i16;
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
    SetVBlankCallback(Some(VBlankCB_Intro));
    gTasks[taskId].data[0] = CreateBicycleBgAnimationTask(1, 0x4000, 0x400, 0x10) as i16;
    SetIntroPart2BgCnt(1);
    gTasks[taskId].func = Some(Task_Scene2_BikeRide);
}
pub(crate) unsafe extern "C" fn Task_Scene2_BikeRide(taskId: u8) {
    let mut offset: u16 = 0;
    if gIntroFrameCounter == TIMER_TORCHIC_EXIT {
        gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_FROZEN;
        DestroyTask(gTasks[taskId].data[0] as u8);
    }
    if gIntroFrameCounter > TIMER_END_SCENE_2 {
        BeginNormalPaletteFade(PALETTES_ALL, 8, 0, 16, 65535);
        gTasks[taskId].func = Some(Task_Scene2_End);
    }
    if gIntroFrameCounter == TIMER_PLAYER_DRIFT_BACK {
        gSprites[gTasks[taskId].data[1]].data[0] = 1;
    }
    if gIntroFrameCounter == TIMER_PLAYER_MOVE_FORWARD {
        gSprites[gTasks[taskId].data[1]].data[0] = 0;
    }
    if gIntroFrameCounter == TIMER_FLYGON_ENTER {
        gSprites[gTasks[taskId].data[2]].data[0] = 1;
    }
    if gIntroFrameCounter == TIMER_PLAYER_MOVE_BACKWARD {
        gSprites[gTasks[taskId].data[1]].data[0] = 2;
    }
    if gIntroFrameCounter == TIMER_PLAYER_HOLD_POSITION {
        gSprites[gTasks[taskId].data[1]].data[0] = 3;
    }
    if gIntroFrameCounter == TIMER_PLAYER_EXIT {
        gSprites[gTasks[taskId].data[1]].data[0] = 4;
    }
    offset = Sin(gTasks[taskId].data[3] >> 2 & 0x7F, 48) as u16;
    sFlygonYOffset = offset;
    if gTasks[taskId].data[3] < 512 {
        gTasks[taskId].data[3] += 1;
    }
    CycleSceneryPalette(0);
}
pub(crate) unsafe extern "C" fn Task_Scene2_End(taskId: u8) {
    if gIntroFrameCounter > TIMER_START_SCENE_3 {
        gTasks[taskId].func = Some(Task_Scene3_Load);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Volbeat(sprite: *mut Sprite) {
    (*sprite).data[3] += 4;
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == VOLBEAT_WAIT_ENTER {
            fall = true;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) < 180
            {
                break 'l1;
            }
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == VOLBEAT_ENTER {
            fall = true;
            (*sprite).x -= 4;
            if (*sprite).x == 60 {
                (*sprite).data[0] = VOLBEAT_WAIT_STATE;
                (*sprite).data[1] = 20;
                (*sprite).data[2] = VOLBEAT_ZIP_BACKWARD;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_ZIP_BACKWARD {
            fall = true;
            (*sprite).x += 8;
            (*sprite).y -= 2;
            if (*sprite).x == 124 {
                (*sprite).data[0] = VOLBEAT_WAIT_STATE;
                (*sprite).data[1] = 20;
                (*sprite).data[2] = VOLBEAT_ZIP_DOWN;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_ZIP_DOWN {
            fall = true;
            (*sprite).y += 4;
            if (*sprite).y == 80 {
                (*sprite).data[0] = VOLBEAT_WAIT_STATE;
                (*sprite).data[1] = 10;
                (*sprite).data[2] = VOLBEAT_ZIP_FORWARD;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_ZIP_FORWARD {
            fall = true;
            (*sprite).x -= 8;
            (*sprite).y -= 2;
            if (*sprite).x == 60 {
                (*sprite).data[0] = VOLBEAT_WAIT_STATE;
                (*sprite).data[1] = 10;
                (*sprite).data[2] = VOLBEAT_INIT_FIGURE_8;
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_INIT_FIGURE_8 {
            fall = true;
            (*sprite).x += 60;
            (*sprite).data[4] = 0xC0;
            (*sprite).data[5] = 0x80;
            (*sprite).data[6] = 3;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == VOLBEAT_FIGURE_8 {
            fall = true;
            (*sprite).x2 = Sin((*sprite).data[4] as u8 as i16, 0x3C);
            (*sprite).y2 = Sin((*sprite).data[5] as u8 as i16, 0x14);
            (*sprite).data[4] += 2;
            (*sprite).data[5] += 4;
            if (*sprite).data[4] as i32 & 0xFF == 64 {
                (*sprite).set_hFlip(FALSE as u16);
                if ({
                    (*sprite).data[6] -= 1;
                    (*sprite).data[6]
                }) == 0
                {
                    (*sprite).x += (*sprite).x2;
                    (*sprite).x2 = 0;
                    (*sprite).data[0] += 1;
                }
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_EXIT {
            fall = true;
            (*sprite).x -= 2;
            (*sprite).y2 = Sin((*sprite).data[5] as u8 as i16, 0x14);
            (*sprite).data[5] += 4;
            if (*sprite).x < -16 {
                DestroySprite(sprite);
            }
            break 'l1;
        }
        if sw1 == VOLBEAT_WAIT_STATE {
            fall = true;
            (*sprite).y2 = Cos((*sprite).data[3] as u8 as i16, 2);
            if ({
                (*sprite).data[1] -= 1;
                (*sprite).data[1]
            }) == 0
            {
                (*sprite).data[0] = (*sprite).data[2];
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Torchic(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if gIntroFrameCounter == TIMER_TORCHIC_ENTER {
                StartSpriteAnim(sprite, TORCHIC_ANIM_RUN);
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if gIntroFrameCounter == TIMER_PLAYER_HOLD_POSITION {
                StartSpriteAnim(sprite, TORCHIC_ANIM_WALK);
                (*sprite).data[0] += 1;
            } else {
                (*sprite).data[1] += 64;
                if (*sprite).data[1] as i32 & 0xFF00 != 0 {
                    (*sprite).x -= 1;
                    (*sprite).data[1] &= 0xFF;
                }
            }
        }
        2 => {
            if gIntroFrameCounter != TIMER_TORCHIC_SPEED_UP {
                (*sprite).data[1] += 32;
                if (*sprite).data[1] as i32 & 0xFF00 != 0 {
                    (*sprite).x += 1;
                    (*sprite).data[1] &= 0xFF;
                }
            } else {
                StartSpriteAnim(sprite, TORCHIC_ANIM_RUN);
                (*sprite).data[0] += 1;
                (*sprite).data[2] = 80;
            }
        }
        3 => {
            if ({
                (*sprite).data[2] -= 1;
                (*sprite).data[2]
            }) != 0
            {
                (*sprite).data[1] += 64;
                if (*sprite).data[1] as i32 & 0xFF00 != 0 {
                    (*sprite).x -= 1;
                    (*sprite).data[1] &= 0xFF;
                }
            } else {
                StartSpriteAnim(sprite, TORCHIC_ANIM_TRIP);
                (*sprite).data[0] += 1;
            }
        }
        4 => {
            if (*sprite).animEnded() != 0 {
                (*sprite).x += 4;
            }
            if (*sprite).x > 336 {
                StartSpriteAnim(sprite, TORCHIC_ANIM_RUN);
                (*sprite).data[0] += 1;
            }
        }
        5 => {
            if gIntroFrameCounter >= TIMER_TORCHIC_EXIT {
                (*sprite).x -= 2;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Manectric(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if gIntroFrameCounter == TIMER_MANECTRIC_ENTER {
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            (*sprite).x -= 2;
            if gIntroFrameCounter != TIMER_MANECTRIC_RUN_CIRCULAR {
                break 'l1;
            }
            (*sprite).y -= 12;
            (*sprite).data[1] = 0x80;
            (*sprite).data[2] = 0;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 2 {
            fall = true;
            if (*sprite).x as i32 + (*sprite).x2 as i32 <= -32 {
                DestroySprite(sprite);
            } else {
                if (*sprite).data[1] as i32 & 0xFF < 64 {
                    (*sprite).x2 = Sin((*sprite).data[1] as u8 as i16, 16);
                } else {
                    if (*sprite).data[1] as i32 & 0xFF == 64 {
                        (*sprite).x -= 48;
                    }
                    (*sprite).x2 = Sin((*sprite).data[1] as u8 as i16, 64);
                }
                (*sprite).data[1] += 1;
                (*sprite).y2 = Cos((*sprite).data[2] as u8 as i16, 12);
                (*sprite).data[2] += 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_Load(taskId: u8) {
    IntroResetGpuRegs();
    LZ77UnCompVram(
        sIntroPokeball_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sIntroPokeball_Tilemap.as_ptr().cast_mut(),
        0x6004000 as usize as *mut c_void,
    );
    LoadPalette(
        sIntroPokeball_Pal.as_ptr().cast_mut() as *mut c_void,
        0,
        512,
    );
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].data[3] = 0;
    PanFadeAndZoomScreen(120, 80, 0, 0);
    ResetSpriteData();
    FreeAllSpritePalettes();
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
    SetGpuReg(REG_OFFSET_BG2CNT, 18563);
    SetGpuReg(REG_OFFSET_DISPCNT, 5185);
    gTasks[taskId].func = Some(Task_Scene3_SpinPokeball);
    gIntroFrameCounter = 0;
    m4aSongNumStart(MUS_INTRO_BATTLE);
}
pub(crate) unsafe extern "C" fn Task_Scene3_SpinPokeball(taskId: u8) {
    gTasks[taskId].data[0] += 0x400;
    if gTasks[taskId].data[1] <= 0x6BF {
        gTasks[taskId].data[1] += gTasks[taskId].data[2];
        gTasks[taskId].data[2] += 2;
    } else {
        gTasks[taskId].func = Some(Task_Scene3_WaitGroudon);
    }
    PanFadeAndZoomScreen(
        120,
        80,
        (if gTasks[taskId].data[1] != 0 {
            div_i32(0x10000, gTasks[taskId].data[1] as i32)
        } else {
            0
        }) as u16,
        gTasks[taskId].data[0] as u16,
    );
    if gIntroFrameCounter == TIMER_POKEBALL_FADE {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 65535);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_WaitGroudon(taskId: u8) {
    if gIntroFrameCounter > TIMER_START_LEGENDARIES {
        gTasks[taskId].func = Some(Task_Scene3_LoadGroudon);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadGroudon(taskId: u8) {
    if gPaletteFade.active() == 0 {
        IntroResetGpuRegs();
        ResetSpriteData();
        FreeAllSpritePalettes();
        gReservedSpritePaletteCount = 8;
        LZDecompressVram(
            gIntroGroudon_Gfx.as_ptr().cast_mut(),
            VRAM as usize as *mut c_void,
        );
        LZDecompressVram(
            gIntroGroudon_Tilemap.as_ptr().cast_mut(),
            0x600c000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gIntroLegendBg_Gfx.as_ptr().cast_mut(),
            0x6004000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gIntroGroudonBg_Tilemap.as_ptr().cast_mut(),
            0x600e000 as usize as *mut c_void,
        );
        LoadCompressedSpriteSheetUsingHeap((&raw const gBattleAnimPicTable[58]).cast_mut());
        LoadCompressedSpritePaletteUsingHeap((&raw const gBattleAnimPaletteTable[58]).cast_mut());
        CpuSet(
            gIntro3Bg_Pal.as_ptr().cast_mut() as *mut c_void,
            gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
            256,
        );
        gTasks[taskId].func = Some(Task_Scene3_InitGroudonBg);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_InitGroudonBg(taskId: u8) {
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    SetGpuReg(REG_OFFSET_WIN0V, DISPLAY_HEIGHT);
    SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 47232);
    SetGpuReg(REG_OFFSET_BG1CNT, 7173);
    SetGpuReg(REG_OFFSET_DISPCNT, 13889);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 65535);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = -96;
    gTasks[taskId].data[2] = -175;
    gTasks[taskId].data[3] = 0x100;
    PanFadeAndZoomScreen(
        gTasks[taskId].data[1] as u16,
        gTasks[taskId].data[2] as u16,
        gTasks[taskId].data[3] as u16,
        0,
    );
    gTasks[taskId].func = Some(Task_Scene3_NarrowWindow);
}
pub(crate) unsafe extern "C" fn Task_Scene3_NarrowWindow(taskId: u8) {
    if gTasks[taskId].data[0] != NARROW_HEIGHT {
        gTasks[taskId].data[0] += 4;
        SetGpuReg(
            REG_OFFSET_WIN0V,
            gTasks[taskId].data[0] as u16 * 256 - (gTasks[taskId].data[0] as u16 - DISPLAY_HEIGHT),
        );
    } else {
        SetGpuReg(REG_OFFSET_WIN0V, 8320);
        gTasks[taskId].func = Some(Task_Scene3_EndNarrowWindow);
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_EndNarrowWindow(taskId: u8) {
    gTasks[taskId].func = Some(Task_Scene3_StartGroudon);
}
pub(crate) unsafe extern "C" fn Task_Scene3_StartGroudon(taskId: u8) {
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].func = Some(Task_Scene3_Groudon);
    ScanlineEffect_InitWave(0, DISPLAY_HEIGHT as u8, 4, 4, 1, 4, 0);
}
pub(crate) unsafe extern "C" fn Task_Scene3_Groudon(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(5) += 1;
    if *data >= 1 && *data <= 7 && *data.at(5) % 2 == 0 {
        *data.at(4) ^= 3;
    }
    PanFadeAndZoomScreen(
        *data.at(1) as u16,
        *data.at(2) as u16 + *data.at(4) as u16,
        *data.at(3) as u16,
        0,
    );
    match *data {
        0 => {
            *data.at(1) += 16;
            if *data.at(1) == 160 {
                *data += 1;
                *data.at(6) = 2;
                *data.at(7) = 0x1E2;
                CreateGroudonRockSprites(taskId);
            }
        }
        1 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 2;
                CpuSet(
                    ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut gPlttBufferFaded[31] as *mut c_void,
                    1,
                );
                *data.at(7) += 2;
                if *data.at(7) == 0x1EC {
                    *data += 1;
                }
            }
        }
        2 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 2;
                *data += 1;
            }
        }
        3 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 2;
                CpuSet(
                    ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut gPlttBufferFaded[31] as *mut c_void,
                    1,
                );
                *data.at(7) -= 2;
                if *data.at(7) == 0x1E0 {
                    *data.at(6) = 8;
                    *data += 1;
                }
            }
        }
        4 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(1) = -96;
                *data.at(2) = 169;
                *data.at(6) = 3;
                *data += 1;
            }
        }
        5 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(1) = 80;
                *data.at(2) = 41;
                *data.at(6) = 16;
                PlayCryInternal(SPECIES_GROUDON, 0, 100, CRY_PRIORITY_NORMAL, 0);
                *data += 1;
            }
        }
        6 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(1) = 80;
                *data.at(2) = 40;
                *data += 1;
            }
        }
        7 => {
            *data.at(1) += 4;
            *data.at(2) += 4;
            *data.at(6) += 0x666;
            *data.at(3) = Sin(((*data.at(6) as i32 & 0xFF00) >> 8) as i16, 64) + 256;
            if *data.at(1) == 120 {
                BeginNormalPaletteFade(0xfffffffe, 3, 0, 16, 32767);
                *data.at(3) = 256;
                *data.at(4) = 0;
                *data += 1;
            }
        }
        8 => {
            if *data.at(3) != 0 {
                *data.at(3) -= 8;
            } else {
                *data += 1;
            }
        }
        9 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].func = Some(Task_Scene3_LoadKyogre);
                gScanlineEffect.state = 3;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateGroudonRockSprites(taskId: u8) {
    let mut i: i32 = 0;
    let mut spriteId: u8 = 0;
    i = 0;
    while i < 6 {
        spriteId = CreateSprite(
            (&raw const gAncientPowerRockSpriteTemplate).cast_mut(),
            sGroudonRockData[i][0],
            DISPLAY_HEIGHT as i16,
            i as u8,
        );
        gSprites[spriteId].callback = Some(SpriteCB_GroudonRocks);
        gSprites[spriteId].oam.set_priority(0);
        gSprites[spriteId].data[1] = i as i16;
        gSprites[spriteId].data[4] = taskId as i16;
        StartSpriteAnim(&raw mut gSprites[spriteId], sGroudonRockData[i][1] as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GroudonRocks(sprite: *mut Sprite) {
    (*sprite).data[3] += 1;
    if (*sprite).data[3] % 2 == 0 {
        (*sprite).y2 ^= 3;
    }
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[2] += sGroudonRockData[(*sprite).data[1]][2];
            (*sprite).y -= (((*sprite).data[2] as i32 & 0xFF00) >> 8) as i16;
            (*sprite).data[2] &= 0xFF;
            if gTasks[(*sprite).data[4]].data[0] > 7 {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if (*sprite).x < 120 {
                (*sprite).x -= 2;
            } else {
                (*sprite).x += 2;
            }
            if (*sprite).y < 80 {
                (*sprite).y -= 2;
            } else {
                (*sprite).y += 2;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadKyogre(taskId: u8) {
    ResetSpriteData();
    LZDecompressVram(
        gIntroKyogre_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroKyogre_Tilemap.as_ptr().cast_mut(),
        0x600c000 as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroKyogreBg_Tilemap.as_ptr().cast_mut(),
        0x600e000 as usize as *mut c_void,
    );
    LoadCompressedSpriteSheet(sSpriteSheet_Bubbles.as_ptr().cast_mut());
    LoadSpritePalette(sSpritePalette_Bubbles.as_ptr().cast_mut());
    BeginNormalPaletteFade(0xfffffffe, 0, 16, 0, 65535);
    gTasks[taskId].func = Some(Task_Scene3_Kyogre);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 336;
    gTasks[taskId].data[2] = 80;
    gTasks[taskId].data[6] = 16;
    gTasks[taskId].data[3] = 256;
    PanFadeAndZoomScreen(
        gTasks[taskId].data[1] as u16,
        gTasks[taskId].data[2] as u16,
        gTasks[taskId].data[3] as u16,
        0,
    );
    ScanlineEffect_InitWave(
        0,
        DISPLAY_HEIGHT as u8,
        4,
        4,
        1,
        SCANLINE_EFFECT_REG_BG1VOFS,
        0,
    );
}
pub(crate) unsafe extern "C" fn Task_Scene3_Kyogre(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    PanFadeAndZoomScreen(
        *data.at(1) as u16,
        *data.at(2) as u16,
        *data.at(3) as u16,
        0,
    );
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) != 0
            {
                break 'l1;
            }
            *data += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            *data.at(6) += 4;
            gTasks[taskId].data[1] = 344 - Sin(*data.at(6), 0x100);
            gTasks[taskId].data[2] = 84 - Cos(*data.at(6), 0x40);
            if *data.at(6) == 64 {
                *data.at(6) = 0x19;
                *data.at(7) = 1;
                *data += 1;
                CreateKyogreBubbleSprites_Body(0);
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                gTasks[taskId].data[1] += 256;
                gTasks[taskId].data[2] -= 258;
                *data.at(6) = 8;
                *data += 1;
                CreateKyogreBubbleSprites_Body(0);
                CreateKyogreBubbleSprites_Fins();
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                gTasks[taskId].data[1] -= 256;
                gTasks[taskId].data[2] += 258;
                *data.at(6) = 8;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                gTasks[taskId].data[2] -= 252;
                *data.at(6) = 8;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                gTasks[taskId].data[2] += 252;
                if *data.at(7) != 0 {
                    *data.at(6) = 12;
                    *data.at(7) -= 1;
                    *data = 2;
                } else {
                    *data.at(6) = 1;
                    *data += 1;
                    PlayCryInternal(SPECIES_KYOGRE as u16, 0, 120, CRY_PRIORITY_NORMAL, 0);
                }
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                *data.at(7) = 0x1EA;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                CpuSet(
                    ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut gPlttBufferFaded[47] as *mut c_void,
                    1,
                );
                *data.at(7) -= 2;
                if *data.at(7) == 0x1E0 {
                    *data += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 8 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                *data.at(7) = 0x1E2;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 9 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 4;
                CpuSet(
                    ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8)
                        .at(*data.at(7)) as *mut c_void,
                    &raw mut gPlttBufferFaded[47] as *mut c_void,
                    1,
                );
                *data.at(7) += 2;
                if *data.at(7) == 0x1EE {
                    *data.at(6) = 16;
                    *data += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                *data.at(6) = 0;
                *data += 1;
                CreateKyogreBubbleSprites_Body(taskId);
            }
            break 'l1;
        }
        if sw1 == 11 {
            fall = true;
            *data.at(6) += 4;
            *data.at(3) -= 8;
            gTasks[taskId].data[1] = Sin(*data.at(6), 0x3C) + 88;
            if *data.at(6) == 64 {
                BeginNormalPaletteFade(0xfffffffe, 3, 0, 16, 32767);
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 12 {
            fall = true;
            *data.at(6) += 4;
            *data.at(3) -= 8;
            gTasks[taskId].data[1] = Sin(*data.at(6), 0x14) + 128;
            if *data.at(6) == 128 {
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 13 {
            fall = true;
            if gPaletteFade.active() == 0 {
                gTasks[taskId].func = Some(Task_Scene3_LoadClouds1);
                gScanlineEffect.state = 3;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateKyogreBubbleSprites_Body(taskId: u8) {
    let mut i: i32 = 0;
    let mut spriteId: u8 = 0;
    i = 0;
    while i < NUM_BUBBLES_IN_SET {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Bubbles).cast_mut(),
            sKyogreBubbleData[i][0],
            sKyogreBubbleData[i][1],
            i as u8,
        );
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].data[5] = taskId as i16;
        gSprites[spriteId].data[6] = sKyogreBubbleData[i][2];
        gSprites[spriteId].data[7] = 64;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateKyogreBubbleSprites_Fins() {
    let mut i: i32 = 0;
    let mut spriteId: u8 = 0;
    i = 0;
    while i < NUM_BUBBLES_IN_SET {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Bubbles).cast_mut(),
            sKyogreBubbleData[i + NUM_BUBBLES_IN_SET][0],
            sKyogreBubbleData[i + NUM_BUBBLES_IN_SET][1],
            i as u8,
        );
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].data[6] = sKyogreBubbleData[i][2];
        gSprites[spriteId].data[7] = 64;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_KyogreBubbles(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if (*sprite).data[6] == 0 {
                (*sprite).data[1] = (*sprite).data[1] + 11 & 0xFF;
                (*sprite).x2 = Sin((*sprite).data[1], 4);
                (*sprite).data[2] += 48;
                (*sprite).y2 = -((*sprite).data[2] >> 8);
                if (*sprite).animEnded() != 0 {
                    DestroySprite(sprite);
                }
            } else if ({
                (*sprite).data[6] -= 1;
                (*sprite).data[6]
            }) == 0
            {
                StartSpriteAnim(sprite, 0);
                (*sprite).set_invisible(FALSE as u16);
            }
            if gTasks[(*sprite).data[5]].data[0] > 11 {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if (*sprite).x < 120 {
                (*sprite).x -= 3;
            } else {
                (*sprite).x += 3;
            }
            if (*sprite).y < 80 {
                (*sprite).y -= 3;
            } else {
                (*sprite).y += 3;
            }
            if (*sprite).y as u16 as i32 - 20 > 140 {
                DestroySprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadClouds1(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 135);
    SetGpuReg(REG_OFFSET_BLDALPHA, 7967);
    SetGpuReg(REG_OFFSET_BLDY, 31);
    SetGpuReg(REG_OFFSET_BG0CNT, 22528);
    SetGpuReg(REG_OFFSET_BG1CNT, 23044);
    SetGpuReg(REG_OFFSET_BG2CNT, 7174);
    SetGpuReg(0x0, 14144);
    SetGpuReg(REG_OFFSET_BG0HOFS, 80);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 65456);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    LZDecompressVram(
        gIntroClouds_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroClouds_Gfx.as_ptr().cast_mut(),
        0x6004000 as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroCloudsSun_Tilemap.as_ptr().cast_mut(),
        0x600e000 as usize as *mut c_void,
    );
    gTasks[taskId].func = Some(Task_Scene3_LoadClouds2);
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadClouds2(taskId: u8) {
    LZDecompressVram(
        gIntroCloudsLeft_Tilemap.as_ptr().cast_mut(),
        0x600c000 as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroCloudsRight_Tilemap.as_ptr().cast_mut(),
        0x600d000 as usize as *mut c_void,
    );
    gTasks[taskId].func = Some(Task_Scene3_InitClouds);
}
pub(crate) unsafe extern "C" fn Task_Scene3_InitClouds(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    gTasks[taskId].func = Some(Task_Scene3_Clouds);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[6] = 16;
}
pub(crate) unsafe extern "C" fn Task_Scene3_Clouds(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    SetGpuReg(REG_OFFSET_BG0HOFS, (*data.at(6) >> 8) as u16);
    SetGpuReg(
        REG_OFFSET_BG1HOFS,
        ((*data.at(6) >> 8) as u16).wrapping_neg(),
    );
    match *data {
        0 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                BeginNormalPaletteFade(0xfffffffe, 0, 16, 0, 65535);
                *data.at(6) = 20480;
                *data += 1;
            }
        }
        1 => {
            if *data.at(6) == 10240 {
                BeginNormalPaletteFade(65534, 3, 0, 16, 10569);
            }
            if *data.at(6) != 0 {
                *data.at(6) -= 128;
            } else if gPaletteFade.active() == 0 {
                gTasks[taskId].func = Some(Task_Scene3_LoadLightning);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadLightning(taskId: u8) {
    LZDecompressVram(
        gIntroRayquaza_Tilemap.as_ptr().cast_mut(),
        0x600e000 as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroRayquazaClouds_Tilemap.as_ptr().cast_mut(),
        0x600c000 as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroRayquaza_Gfx.as_ptr().cast_mut(),
        0x6004000 as usize as *mut c_void,
    );
    LZDecompressVram(
        gIntroRayquazaClouds_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    SetGpuReg(0x0, 13632);
    gTasks[taskId].func = Some(Task_Scene3_Lightning);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[6] = 1;
    gTasks[taskId].data[7] = 0;
    LoadCompressedSpriteSheetUsingHeap(sSpriteSheet_Lightning.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalette_Lightning.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn Task_Scene3_Lightning(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut spriteId: u8 = 0;
    match *data {
        0 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    200,
                    48,
                    0,
                );
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    200,
                    80,
                    1,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 1);
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    200,
                    112,
                    2,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 2);
                *data += 1;
                *data.at(6) = 72;
            }
        }
        1 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    40,
                    48,
                    0,
                );
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    40,
                    80,
                    1,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 1);
                spriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Lightning).cast_mut(),
                    40,
                    112,
                    2,
                );
                StartSpriteAnim(&raw mut gSprites[spriteId], 2);
                *data += 1;
                *data.at(6) = 48;
            }
        }
        2 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                gTasks[taskId].func = Some(Task_Scene3_LoadRayquazaAttack);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Lightning(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[1] = 0x1C2;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            CpuSet(
                ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8)
                    .at((*sprite).data[1]) as *mut c_void,
                &raw mut gPlttBufferFaded[93] as *mut c_void,
                1,
            );
            (*sprite).data[1] += 2;
            if (*sprite).data[1] != 0x1CE {
                break 'l1;
            }
            (*sprite).data[1] = 0x1CC;
            (*sprite).data[2] = 4;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 2 {
            fall = true;
            if ({
                (*sprite).data[2] -= 1;
                (*sprite).data[2]
            }) == 0
            {
                (*sprite).data[2] = 4;
                CpuSet(
                    ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8)
                        .at((*sprite).data[1]) as *mut c_void,
                    &raw mut gPlttBufferFaded[93] as *mut c_void,
                    1,
                );
                (*sprite).data[1] -= 2;
                if (*sprite).data[1] == 0x1C0 {
                    DestroySprite(sprite);
                }
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Scene3_LoadRayquazaAttack(taskId: u8) {
    let mut attackTaskId: u8 = 0;
    LoadCompressedSpriteSheet(sSpriteSheet_RayquazaOrb.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalette_RayquazaOrb.as_ptr().cast_mut());
    SetGpuReg(0x0, 13632);
    gTasks[taskId].func = Some(Task_Scene3_Rayquaza);
    BeginNormalPaletteFade(65502, 0, 16, 0, 10569);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 0xA8;
    gTasks[taskId].data[2] = -16;
    gTasks[taskId].data[3] = -136;
    gTasks[taskId].data[4] = -16;
    attackTaskId = CreateTask(Some(Task_RayquazaAttack), 0);
    gTasks[attackTaskId].data[4] = taskId as i16;
}
pub(crate) unsafe extern "C" fn Task_Scene3_Rayquaza(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(7) % 2 == 0 {
        *data.at(6) ^= 2;
    }
    *data.at(7) += 1;
    match *data {
        0 => {
            if *data.at(7) as i32 & 1 != 0 {
                *data.at(1) -= 2;
                *data.at(2) += 1;
                *data.at(3) += 2;
                *data.at(4) += 1;
            }
            if *data.at(1) == 0x68 {
                *data += 1;
                *data.at(5) = 1;
            }
        }
        1 => {
            *data += 1;
            *data.at(5) = 4;
        }
        2 => {
            *data.at(1) += 4;
            *data.at(2) -= 2;
            *data.at(3) -= 4;
            *data.at(4) -= 2;
            if gPaletteFade.active() == 0 {
                *data.at(5) = 0x8C;
                *data += 1;
            }
        }
        3 => {
            if ({
                *data.at(5) -= 1;
                *data.at(5)
            }) == 0
            {
                gTasks[taskId].func = Some(Task_EndIntroMovie);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_EndIntroMovie(taskId: u8) {
    DestroyTask(taskId);
    SetMainCallback2(Some(MainCB2_EndIntro));
}
pub(crate) unsafe extern "C" fn Task_RayquazaAttack(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(2) += 1;
    match *data {
        0 => {
            if *data.at(2) as i32 & 1 != 0 {
                CpuSet(
                    (((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8).at(418)
                        as *mut c_void as *mut u8)
                        .at(*data.at(1) as i32 * 2) as *mut c_void,
                    &raw mut gPlttBufferFaded[94] as *mut c_void,
                    1,
                );
                *data.at(1) += 1;
            }
            if *data.at(1) == 6 {
                *data += 1;
                *data.at(1) = 0;
                *data.at(3) = 10;
            }
        }
        1 => {
            if *data.at(3) == 0 {
                if *data.at(2) as i32 & 1 != 0 {
                    CpuSet(
                        (((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8).at(418)
                            as *mut c_void as *mut u8)
                            .at(*data.at(1) as i32 * 2) as *mut c_void,
                        &raw mut gPlttBufferFaded[88] as *mut c_void,
                        1,
                    );
                    *data.at(1) += 1;
                }
                if *data.at(1) == 6 {
                    *data += 1;
                    *data.at(3) = 10;
                }
            } else {
                *data.at(3) -= 1;
            }
        }
        2 => {
            if *data.at(3) == 0 {
                if *data.at(2) as i32 & 1 != 0 {
                    CpuSet(
                        (((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8).at(386)
                            as *mut c_void as *mut u8)
                            .at(*data.at(1) as i32 * 2) as *mut c_void,
                        &raw mut gPlttBufferFaded[92] as *mut c_void,
                        1,
                    );
                    *data.at(1) += 1;
                }
                if *data.at(1) == 6 {
                    spriteId = CreateSprite(
                        (&raw const *sSpriteTemplate_RayquazaOrb).cast_mut(),
                        120,
                        88,
                        15,
                    );
                    PlaySE(SE_INTRO_BLAST);
                    gSprites[spriteId].set_invisible(TRUE as u16);
                    gSprites[spriteId].data[3] = *data.at(4);
                    *data += 1;
                    *data.at(3) = 16;
                }
            } else {
                *data.at(3) -= 1;
            }
        }
        3 => {
            if *data.at(2) as i32 & 1 != 0 {
                if ({
                    *data.at(3) -= 1;
                    *data.at(3)
                }) != 0
                {
                    BlendPalette(80, 16, *data.at(3) as u8, 10569);
                    CpuSet(
                        ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8).at(428)
                            as *mut c_void,
                        &raw mut gPlttBufferFaded[94] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8).at(428)
                            as *mut c_void,
                        &raw mut gPlttBufferFaded[88] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        ((&raw const gIntro3Bg_Pal).cast_mut() as *mut c_void as *mut u8).at(396)
                            as *mut c_void,
                        &raw mut gPlttBufferFaded[92] as *mut c_void,
                        1,
                    );
                } else {
                    *data += 1;
                    *data.at(3) = 53;
                }
            }
        }
        4 => {
            if ({
                *data.at(3) -= 1;
                *data.at(3)
            }) == 0
            {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 32767);
                *data += 1;
            }
        }
        5 => {
            if gPaletteFade.active() == 0 {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn IntroResetGpuRegs() {
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
}
pub(crate) unsafe extern "C" fn Task_BlendLogoIn(taskId: u8) {
    match gTasks[taskId].data[0] {
        1 => {
            if gTasks[taskId].data[1] != 0 {
                let mut tmp: u8 = 0;
                gTasks[taskId].data[1] -= 1;
                tmp = (gTasks[taskId].data[1] / 2) as u8;
                SetGpuReg(REG_OFFSET_BLDALPHA, gTitleScreenAlphaBlend[tmp]);
            } else {
                SetGpuReg(REG_OFFSET_BLDALPHA, gTitleScreenAlphaBlend[0]);
                gTasks[taskId].data[1] = 16;
                gTasks[taskId].data[0] += 1;
            }
        }
        2 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            DestroyTask(taskId);
        }
        _ => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16192);
            SetGpuReg(REG_OFFSET_BLDALPHA, gTitleScreenAlphaBlend[31]);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            gTasks[taskId].data[1] = 64;
            gTasks[taskId].data[0] += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BlendLogoOut(taskId: u8) {
    match gTasks[taskId].data[0] {
        1 => {
            if gTasks[taskId].data[1] < 62 {
                let mut tmp: u8 = 0;
                gTasks[taskId].data[1] += 1;
                tmp = (gTasks[taskId].data[1] / 2) as u8;
                SetGpuReg(REG_OFFSET_BLDALPHA, gTitleScreenAlphaBlend[tmp]);
            } else {
                SetGpuReg(REG_OFFSET_BLDALPHA, gTitleScreenAlphaBlend[31]);
                gTasks[taskId].data[1] = 16;
                gTasks[taskId].data[0] += 1;
            }
        }
        2 => {
            if gTasks[taskId].data[1] != 0 {
                gTasks[taskId].data[1] -= 1;
            } else {
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                SetGpuReg(REG_OFFSET_BLDY, 0);
                DestroyTask(taskId);
            }
        }
        _ => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16192);
            SetGpuReg(REG_OFFSET_BLDALPHA, gTitleScreenAlphaBlend[0]);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            gTasks[taskId].data[1] = 0;
            gTasks[taskId].data[0] += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PanFadeAndZoomScreen(screenX: u16, screenY: u16, zoom: u16, alpha: u16) {
    let mut src: BgAffineSrcData = zeroed();
    let mut dest: BgAffineDstData = zeroed();
    src.texX = 0x8000;
    src.texY = 0x8000;
    src.scrX = screenX as i16;
    src.scrY = screenY as i16;
    src.sx = zoom as i16;
    src.sy = zoom as i16;
    src.alpha = alpha;
    BgAffineSet(&raw mut src, &raw mut dest, 1);
    SetGpuReg(REG_OFFSET_BG2PA, dest.pa as u16);
    SetGpuReg(REG_OFFSET_BG2PB, dest.pb as u16);
    SetGpuReg(REG_OFFSET_BG2PC, dest.pc as u16);
    SetGpuReg(REG_OFFSET_BG2PD, dest.pd as u16);
    SetGpuReg(REG_OFFSET_BG2X_L, dest.dx as u16);
    SetGpuReg(REG_OFFSET_BG2X_H, (dest.dx >> 16) as u16);
    SetGpuReg(REG_OFFSET_BG2Y_L, dest.dy as u16);
    SetGpuReg(REG_OFFSET_BG2Y_H, (dest.dy >> 16) as u16);
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_Ripple(sprite: *mut Sprite) {
    let mut palNum: u8 = 0;
    if (*sprite).data[2] >= 192 {
        if (*sprite).data[3] != 0 {
            (*sprite).data[3] -= 1;
        } else {
            (*sprite).set_invisible(FALSE as u16);
            SetOamMatrix(
                (*sprite).data[1] as u8,
                (*sprite).data[2] as u16,
                0,
                0,
                (*sprite).data[2] as u16,
            );
            (*sprite).data[2] = ((*sprite).data[2] as i32 * 95 / 100) as i16;
            palNum = (((*sprite).data[2] as i32 - 192) / 128) as u8 + 9;
            if palNum > 15 {
                palNum = 15;
            }
            (*sprite).oam.set_paletteNum(palNum as u16);
        }
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDropHalf(sprite: *mut Sprite) {
    if gSprites[(*sprite).data[7]].data[7] != 0 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        StartSpriteAnim(sprite, DROP_ANIM_RIPPLE);
        (*sprite).data[2] = 1024;
        (*sprite).data[3] = 8 * ((*sprite).data[1] & 3);
        (*sprite).callback = Some(SpriteCB_WaterDrop_Ripple);
        (*sprite).oam.set_shape(1);
        (*sprite).oam.set_size(3);
        CalcCenterToCornerVec(sprite, 1, 3, ST_OAM_AFFINE_ERASE);
    } else {
        (*sprite).x2 = gSprites[(*sprite).data[7]].x2;
        (*sprite).y2 = gSprites[(*sprite).data[7]].y2;
        (*sprite).x = gSprites[(*sprite).data[7]].x;
        (*sprite).y = gSprites[(*sprite).data[7]].y;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).callback = Some(SpriteCB_WaterDrop_Slide);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_Slide(sprite: *mut Sprite) {
    if (*sprite).x <= 116 {
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x += 4;
        (*sprite).x2 = -4;
        (*sprite).data[4] = 128;
        (*sprite).callback = Some(SpriteCB_WaterDrop_ReachLeafEnd);
    } else {
        let mut data2: u16 = 0;
        let mut data3: u16 = 0;
        let mut data4: u16 = 0;
        let mut sin1: i16 = 0;
        let mut sin2: i16 = 0;
        let mut sin3: i16 = 0;
        let mut sin4: i16 = 0;
        let mut var1: i16 = 0;
        let mut var2: i16 = 0;
        let mut var3: i16 = 0;
        let mut var4: i16 = 0;
        let mut temp: i16 = 0;
        data4 = (*sprite).data[4] as u16;
        sin1 = gSineTable[data4 as u8];
        sin2 = gSineTable[data4 as u8 as i32 + 64];
        (*sprite).data[4] += 2;
        (*sprite).y2 = sin1 / 32;
        (*sprite).x -= 1;
        if (*sprite).x as i32 & 1 != 0 {
            (*sprite).y += 1;
        }
        temp = (-(sin2 as i32) / 16) as i16;
        data2 = (*sprite).data[2] as u16;
        data3 = (*sprite).data[3] as u16;
        sin3 = gSineTable[temp as u8 as i32 - 16];
        sin4 = gSineTable[temp as u8 as i32 + 48];
        var1 = (sin4 as i32 * data2 as i32 / 256) as i16;
        var2 = (-(sin3 as i32) * data3 as i32 / 256) as i16;
        var3 = (sin3 as i32 * data2 as i32 / 256) as i16;
        var4 = (sin4 as i32 * data3 as i32 / 256) as i16;
        SetOamMatrix((*sprite).data[1] as u8, data2, 0, 0, data3);
        SetOamMatrix(
            (*sprite).data[1] as u8 + 1,
            var1 as u16,
            var3 as u16,
            var2 as u16,
            var4 as u16,
        );
        SetOamMatrix(
            (*sprite).data[1] as u8 + 2,
            var1 as u16,
            var3 as u16,
            var2 as u16 * 2,
            var4 as u16 * 2,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_ReachLeafEnd(sprite: *mut Sprite) {
    SetOamMatrix(
        (*sprite).data[1] as u8,
        (*sprite).data[6] as u16 + 64,
        0,
        0,
        (*sprite).data[6] as u16 + 64,
    );
    SetOamMatrix(
        (*sprite).data[1] as u8 + 1,
        (*sprite).data[6] as u16 + 64,
        0,
        0,
        (*sprite).data[6] as u16 + 64,
    );
    SetOamMatrix(
        (*sprite).data[1] as u8 + 2,
        (*sprite).data[6] as u16 + 64,
        0,
        0,
        (*sprite).data[6] as u16 + 64,
    );
    if (*sprite).data[4] != MAX_SPRITES as i16 {
        let mut sinIdx: u16 = 0;
        (*sprite).data[4] -= 8;
        sinIdx = (*sprite).data[4] as u16;
        (*sprite).x2 = gSineTable[sinIdx as u8 as i32 + 64] / 64;
        (*sprite).y2 = gSineTable[sinIdx as u8] / 64;
    } else {
        (*sprite).data[4] = 0;
        (*sprite).callback = Some(SpriteCB_WaterDrop_DangleFromLeaf);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_DangleFromLeaf(sprite: *mut Sprite) {
    if (*sprite).data[0] != 2 {
        let mut r2: i16 = 0;
        (*sprite).data[4] += 8;
        r2 = gSineTable[(*sprite).data[4] as u8] / 16 + 64;
        (*sprite).x2 = gSineTable[r2 as u8 as i32 + 64] / 64;
        (*sprite).y2 = gSineTable[r2 as u8] / 64;
    } else {
        (*sprite).callback = Some(SpriteCB_WaterDrop_Fall);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDrop_Fall(sprite: *mut Sprite) {
    if (*sprite).y < (*sprite).data[5] {
        (*sprite).y += 4;
    } else {
        (*sprite).data[7] = 1;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        StartSpriteAnim(sprite, DROP_ANIM_RIPPLE);
        (*sprite).data[2] = 1024;
        (*sprite).data[3] = 8 * ((*sprite).data[1] & 3);
        (*sprite).callback = Some(SpriteCB_WaterDrop_Ripple);
        (*sprite).oam.set_shape(1);
        (*sprite).oam.set_size(3);
        CalcCenterToCornerVec(sprite, 1, 3, ST_OAM_AFFINE_ERASE);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaterDropShort(sprite: *mut Sprite) {
    if (*sprite).y < (*sprite).data[5] {
        (*sprite).y += 4;
    } else {
        (*sprite).data[7] = 1;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        StartSpriteAnim(sprite, DROP_ANIM_RIPPLE);
        (*sprite).data[2] = 1024;
        (*sprite).data[3] = 8 * ((*sprite).data[1] & 3);
        (*sprite).callback = Some(SpriteCB_WaterDrop_Ripple);
        (*sprite).oam.set_shape(1);
        (*sprite).oam.set_size(3);
        CalcCenterToCornerVec(sprite, 1, 3, ST_OAM_AFFINE_ERASE);
    }
}
pub(crate) unsafe extern "C" fn CreateWaterDrop(
    x: i16,
    y: i16,
    c: u16,
    d: u16,
    e: u16,
    fallImmediately: u8,
) -> u8 {
    let mut spriteId: u8 = 0;
    let mut oldSpriteId: u8 = 0;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_WaterDrop).cast_mut(), x, y, 1);
    gSprites[spriteId].data[0] = 0;
    gSprites[spriteId].data[7] = 0;
    gSprites[spriteId].data[1] = d as i16;
    gSprites[spriteId].data[2] = c as i16;
    gSprites[spriteId].data[3] = c as i16;
    gSprites[spriteId].data[5] = e as i16;
    gSprites[spriteId].data[6] = c as i16;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(d as u32);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        0,
        ST_OAM_AFFINE_ERASE,
        ST_OAM_AFFINE_ERASE,
    );
    StartSpriteAnim(&raw mut gSprites[spriteId], DROP_ANIM_REFLECTION);
    if fallImmediately == 0 {
        gSprites[spriteId].callback = Some(SpriteCB_WaterDrop);
    } else {
        gSprites[spriteId].callback = Some(SpriteCB_WaterDropShort);
    }
    oldSpriteId = spriteId;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_WaterDrop).cast_mut(), x, y, 1);
    gSprites[spriteId].data[7] = oldSpriteId as i16;
    gSprites[spriteId].data[1] = d as i16 + 1;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(d as u32 + 1);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        0,
        ST_OAM_AFFINE_ERASE,
        ST_OAM_AFFINE_ERASE,
    );
    gSprites[spriteId].callback = Some(SpriteCB_WaterDropHalf);
    spriteId = CreateSprite((&raw const *sSpriteTemplate_WaterDrop).cast_mut(), x, y, 1);
    gSprites[spriteId].data[7] = oldSpriteId as i16;
    gSprites[spriteId].data[1] = d as i16 + 2;
    StartSpriteAnim(&raw mut gSprites[spriteId], DROP_ANIM_LOWER_HALF);
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(d as u32 + 2);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        0,
        ST_OAM_AFFINE_ERASE,
        ST_OAM_AFFINE_ERASE,
    );
    gSprites[spriteId].callback = Some(SpriteCB_WaterDropHalf);
    SetOamMatrix(d as u8, c + 32, 0, 0, c + 32);
    SetOamMatrix(d as u8 + 1, c + 32, 0, 0, c + 32);
    SetOamMatrix(d as u8 + 2, c + 32, 0, 0, 2 * (c + 32));
    return oldSpriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerOnBicycle(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            (*sprite).x -= 1;
        }
        1 => {
            StartSpriteAnimIfDifferent(sprite, 0);
            if gIntroFrameCounter & 7 != 0 {
                return;
            }
            (*sprite).x += 1;
        }
        2 => {
            if (*sprite).x <= 120 || gIntroFrameCounter & 7 != 0 {
                (*sprite).x += 1;
            }
        }
        3 => {}
        4 => {
            if (*sprite).x > -32 {
                (*sprite).x -= 2;
            }
        }
        _ => {}
    }
    if gIntroFrameCounter & 7 != 0 {
        return;
    }
    if (*sprite).y2 != 0 {
        (*sprite).y2 = 0;
    } else {
        match Random() as i32 & 3 {
            0 => {
                (*sprite).y2 = -1;
            }
            1 => {
                (*sprite).y2 = 1;
            }
            2 | 3 => {
                (*sprite).y2 = 0;
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Flygon(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {}
        1 => {
            if ((*sprite).x2 as i32 + (*sprite).x as i32) < 304 {
                (*sprite).x2 += 8;
            } else {
                (*sprite).data[0] = 2;
            }
        }
        2 => {
            if (*sprite).x2 as i32 + (*sprite).x as i32 > 120 {
                (*sprite).x2 -= 1;
            } else {
                (*sprite).data[0] = 3;
            }
        }
        3 => {
            if (*sprite).x2 > 0 {
                (*sprite).x2 -= 2;
            }
        }
        _ => {}
    }
    (*sprite).y2 = Sin((*sprite).data[1] as u8 as i16, 8) - sFlygonYOffset as i16;
    (*sprite).data[1] += 4;
}
pub(crate) unsafe extern "C" fn SpriteCB_LogoLetter(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if (*sprite).data[1] != 0 {
                (*sprite).data[1] -= 1;
            } else {
                (*sprite).set_invisible(FALSE as u16);
                StartSpriteAffineAnim(sprite, 1);
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if gIntroFrameCounter == TIMER_LOGO_LETTERS_COLOR {
                (*sprite).data[0] += 1;
                (*sprite).data[1] = COLOR_CHANGES;
                (*sprite).data[3] = 2;
            }
        }
        2 => {
            if (*sprite).data[3] == 0 {
                (*sprite).data[3] = 2;
                if (*sprite).data[1] != 0 {
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1]]).cast_mut()
                            as *mut c_void,
                        &raw mut gPlttBufferFaded[287] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1] as i32 + 16])
                            .cast_mut() as *mut c_void,
                        &raw mut gPlttBufferFaded[276] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1] as i32 + 32])
                            .cast_mut() as *mut c_void,
                        &raw mut gPlttBufferFaded[282] as *mut c_void,
                        1,
                    );
                    (*sprite).data[1] -= 1;
                } else {
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1]]).cast_mut()
                            as *mut c_void,
                        &raw mut gPlttBufferFaded[287] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1] as i32 + 16])
                            .cast_mut() as *mut c_void,
                        &raw mut gPlttBufferFaded[276] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1] as i32 + 32])
                            .cast_mut() as *mut c_void,
                        &raw mut gPlttBufferFaded[282] as *mut c_void,
                        1,
                    );
                    (*sprite).data[0] += 1;
                }
            } else {
                (*sprite).data[3] -= 1;
            }
        }
        3 => {
            if (*sprite).data[3] != 0 {
                (*sprite).data[3] -= 1;
            } else {
                (*sprite).data[3] = 2;
                if (*sprite).data[1] <= COLOR_CHANGES {
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1]]).cast_mut()
                            as *mut c_void,
                        &raw mut gPlttBufferFaded[287] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1] as i32 + 16])
                            .cast_mut() as *mut c_void,
                        &raw mut gPlttBufferFaded[276] as *mut c_void,
                        1,
                    );
                    CpuSet(
                        (&raw const gIntroGameFreakTextFade_Pal[(*sprite).data[1] as i32 + 32])
                            .cast_mut() as *mut c_void,
                        &raw mut gPlttBufferFaded[282] as *mut c_void,
                        1,
                    );
                    (*sprite).data[1] += 1;
                } else {
                    (*sprite).data[0] += 1;
                }
            }
        }
        4 => {
            if gIntroFrameCounter == TIMER_LOGO_DISAPPEAR {
                StartSpriteAffineAnim(sprite, 2);
                (*sprite).oam.set_objMode(ST_OAM_OBJ_BLEND);
                (*sprite).data[0] += 1;
            }
        }
        5 => {
            (*sprite).data[3] += sGameFreakLettersMoveSpeed[(*sprite).data[2]] as i16;
            (*sprite).x2 = (((*sprite).data[3] as i32 & 0xFF00) >> 8) as i16;
            if (*sprite).data[2] < 4 {
                let mut temp: i16 = (*sprite).x2;
                (*sprite).x2 = -temp;
            }
            if (*sprite).affineAnimEnded() != 0 {
                DestroySprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GameFreakLogo(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if gIntroFrameCounter == TIMER_LOGO_APPEAR {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if gIntroFrameCounter == TIMER_LOGO_DISAPPEAR {
                StartSpriteAffineAnim(sprite, 3);
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            if (*sprite).affineAnimEnded() != 0 {
                DestroySprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateGameFreakLogoSprites(x: i16, y: i16, unused: i16) -> u8 {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    i = 0;
    while i < NUM_GF_LETTERS {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_GameFreakLetter).cast_mut(),
            sGameFreakLetterData[i][1] + x,
            y - 4,
            0,
        );
        gSprites[spriteId].data[0] = 0;
        gSprites[spriteId].data[1] = sGameFreakLetterStartDelays[i] as i16;
        gSprites[spriteId].data[2] = i as i16;
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].oam.set_matrixNum(i as u32 + 12);
        StartSpriteAnim(
            &raw mut gSprites[spriteId],
            sGameFreakLetterData[i][0] as u8,
        );
        StartSpriteAffineAnim(&raw mut gSprites[spriteId], 0);
        i += 1;
    }
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_GameFreakLogo).cast_mut(),
        120,
        y - 6,
        0,
    );
    gSprites[spriteId].data[0] = 0;
    gSprites[spriteId].set_invisible(TRUE as u16);
    gSprites[spriteId].oam.set_matrixNum(i as u32 + 12);
    StartSpriteAffineAnim(&raw mut gSprites[spriteId], 1);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_FlygonSilhouette(sprite: *mut Sprite) {
    (*sprite).data[7] += 1;
    if (*sprite).data[0] != 0 {
        let mut sin: i16 = 0;
        let mut cos: i16 = 0;
        let mut a: i16 = 0;
        let mut b: i16 = 0;
        let mut c: i16 = 0;
        let mut d: i16 = 0;
        sin = gSineTable[(*sprite).data[2] as u8];
        cos = gSineTable[(*sprite).data[2] as u8 as i32 + 64];
        d = (cos as i32 * (*sprite).data[1] as i32 / 256) as i16;
        c = (-(sin as i32) * (*sprite).data[1] as i32 / 256) as i16;
        b = (sin as i32 * (*sprite).data[1] as i32 / 256) as i16;
        a = (cos as i32 * (*sprite).data[1] as i32 / 256) as i16;
        SetOamMatrix(1, a as u16, b as u16, c as u16, d as u16);
    }
    match (*sprite).data[0] {
        1 => {
            (*sprite).x2 = -Sin((*sprite).data[3] as u8 as i16, 140);
            (*sprite).y2 = -Sin((*sprite).data[3] as u8 as i16, 120);
            (*sprite).data[1] += 7;
            (*sprite).data[3] += 3;
            if (*sprite).x as i32 + (*sprite).x2 as i32 <= -16 {
                (*sprite).oam.set_priority(3);
                (*sprite).data[0] += 1;
                (*sprite).x = 20;
                (*sprite).y = 40;
                (*sprite).data[1] = 512;
                (*sprite).data[2] = 0;
                (*sprite).data[3] = 16;
            }
        }
        2 => {
            (*sprite).x2 = Sin((*sprite).data[3] as u8 as i16, 34);
            (*sprite).y2 = -Cos((*sprite).data[3] as u8 as i16, 60);
            (*sprite).data[1] += 2;
            if (*sprite).data[7] % 5 == 0 {
                (*sprite).data[3] += 1;
            }
        }
        _ => {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            (*sprite).oam.set_matrixNum(1);
            CalcCenterToCornerVec(
                sprite,
                1,
                ST_OAM_AFFINE_DOUBLE as u8,
                ST_OAM_AFFINE_DOUBLE as u8,
            );
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).data[0] = 1;
            (*sprite).data[1] = 128;
            (*sprite).data[2] = 0;
            (*sprite).data[3] = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RayquazaOrb(sprite: *mut Sprite) {
    let mut foo: u16 = 0;
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let matched = sw1 == 0 || sw1 == 1;
        let mut fall = false;
        if sw1 == 0 || !matched {
            fall = true;
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            (*sprite).oam.set_matrixNum(18);
            CalcCenterToCornerVec(
                sprite,
                0,
                ST_OAM_AFFINE_DOUBLE as u8,
                ST_OAM_AFFINE_DOUBLE as u8,
            );
            (*sprite).data[1] = 0;
            (*sprite).data[0] = 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[7] += 1;
            if (*sprite).data[7] as i32 & 1 != 0 {
                (*sprite).set_invisible(TRUE as u16);
            } else {
                (*sprite).set_invisible(FALSE as u16);
                if (*sprite).data[1] < 64 {
                    (*sprite).data[1] += 1;
                }
            }
            foo = 256 - (gSineTable[(*sprite).data[1] as u8] / 2) as u16;
            SetOamMatrix(18, foo, 0, 0, foo);
            break 'l1;
        }
    }
}
