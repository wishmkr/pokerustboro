//! Translated from `src/intro_credits_graphics.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sGrass_Pal sGrassSunset_Pal sGrassNight_Pal sGrass_Gfx sGrass_Tilemap sCloudsBg_Pal sCloudsBgSunset_Pal sCloudsBg_Gfx sCloudsBg_Tilemap sClouds_Pal sCloudsSunset_Pal sClouds_Gfx sTrees_Pal sTreesSunset_Pal sTrees_Gfx sTrees_Tilemap sTreesSmall_Pal sTreesSmall_Gfx sHouses_Pal sHouses_Gfx sHouseSilhouette_Pal sHouses_Tilemap sHouseSilhouette_Gfx sBrendanCredits_Pal sBrendanCredits_Gfx sMayCredits_Pal sUnused sMayCredits_Gfx sBicycle_Gfx sLatios_Pal sLatios_Gfx sLatias_Pal sLatias_Gfx sSpriteTemplate_MovingScenery sSpriteSheet_Clouds sAnim_Cloud_Largest sAnim_Cloud_Large sAnim_Cloud_Small sAnim_Cloud_Smallest sAnims_Clouds sSpriteMetadata_Clouds sSpriteSheet_TreesSmall sAnim_Trees_0 sAnim_Trees_1 sAnim_Trees_2 sAnims_Trees sSpriteMetadata_Trees sSpriteSheet_HouseSilhouette sAnim_HouseSilhouette sAnims_HouseSilhouette sSpriteMetadata_HouseSilhouette sOamData_Player sAnim_Player sAnims_Player sSpriteTemplate_Brendan sSpriteTemplate_May sOamData_Bicycle sAnim_Bicycle sAnims_Bicycle sSpriteTemplate_BrendanBicycle sSpriteTemplate_MayBicycle sOamData_Flygon sAnim_FlygonLeft sAnim_FlygonRight sAnims_Flygon sSpriteTemplate_FlygonLatios sSpriteTemplate_FlygonLatias gSpriteSheet_IntroBrendan gSpriteSheet_IntroMay gSpriteSheet_IntroBicycle sSpriteSheet_IntroFlygon_Unused gSpriteSheet_IntroFlygon gSpritePalettes_IntroPlayerFlygon gSpriteSheet_CreditsBrendan gSpriteSheet_CreditsMay gSpriteSheet_CreditsBicycle sSpriteSheet_Latios sSpriteSheet_Latias gSpritePalettes_Credits gSpriteSheet_CreditsRivalBrendan gSpriteSheet_CreditsRivalMay

/// `struct IntroCreditsSpriteMetadata`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct IntroCreditsSpriteMetadata {
    bits_0: u8,
    pub x: u8,
    pub y: u8,
    pub subpriority: u8,
    pub xOff: u16,
}

impl IntroCreditsSpriteMetadata {
    #[inline(always)]
    pub fn animNum(&self) -> u8 {
        ((self.bits_0 as u32 >> 0) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_animNum(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0xf << 0)) | ((v as u8 & 0xf) << 0);
    }
    #[inline(always)]
    pub fn shape(&self) -> u8 {
        ((self.bits_0 as u32 >> 4) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_shape(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x3 << 4)) | ((v as u8 & 0x3) << 4);
    }
    #[inline(always)]
    pub fn size(&self) -> u8 {
        ((self.bits_0 as u32 >> 6) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_size(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x3 << 6)) | ((v as u8 & 0x3) << 6);
    }
}

unsafe impl Sync for IntroCreditsSpriteMetadata {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<IntroCreditsSpriteMetadata>() == 8);
    assert!(offset_of!(IntroCreditsSpriteMetadata, bits_0) == 0);
    assert!(offset_of!(IntroCreditsSpriteMetadata, x) == 1);
    assert!(offset_of!(IntroCreditsSpriteMetadata, y) == 2);
    assert!(offset_of!(IntroCreditsSpriteMetadata, subpriority) == 3);
    assert!(offset_of!(IntroCreditsSpriteMetadata, xOff) == 4);
};

static sAnims_Clouds: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::intro_credits_graphics::sAnims_Clouds).cast());
static sAnims_HouseSilhouette: Table<CArray<*mut AnimCmd, 1>> =
    Table((&raw const crate::data::intro_credits_graphics::sAnims_HouseSilhouette).cast());
static sAnims_Trees: Table<CArray<*mut AnimCmd, 3>> =
    Table((&raw const crate::data::intro_credits_graphics::sAnims_Trees).cast());
static sCloudsBgSunset_Pal: Table<CArray<u16, 48>> =
    Table((&raw const crate::data::intro_credits_graphics::sCloudsBgSunset_Pal).cast());
static sCloudsBg_Gfx: Table<CArray<u32, 375>> =
    Table((&raw const crate::data::intro_credits_graphics::sCloudsBg_Gfx).cast());
static sCloudsBg_Pal: Table<CArray<u16, 48>> =
    Table((&raw const crate::data::intro_credits_graphics::sCloudsBg_Pal).cast());
static sCloudsBg_Tilemap: Table<CArray<u32, 180>> =
    Table((&raw const crate::data::intro_credits_graphics::sCloudsBg_Tilemap).cast());
static sCloudsSunset_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sCloudsSunset_Pal).cast());
static sClouds_Gfx: Table<CArray<u32, 79>> =
    Table((&raw const crate::data::intro_credits_graphics::sClouds_Gfx).cast());
static sClouds_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sClouds_Pal).cast());
static sGrassNight_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sGrassNight_Pal).cast());
static sGrassSunset_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sGrassSunset_Pal).cast());
static sGrass_Gfx: Table<CArray<u32, 288>> =
    Table((&raw const crate::data::intro_credits_graphics::sGrass_Gfx).cast());
static sGrass_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sGrass_Pal).cast());
static sGrass_Tilemap: Table<CArray<u32, 79>> =
    Table((&raw const crate::data::intro_credits_graphics::sGrass_Tilemap).cast());
static sHouseSilhouette_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sHouseSilhouette_Pal).cast());
static sHouses_Gfx: Table<CArray<u32, 123>> =
    Table((&raw const crate::data::intro_credits_graphics::sHouses_Gfx).cast());
static sHouses_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::intro_credits_graphics::sHouses_Pal).cast());
static sHouses_Tilemap: Table<CArray<u32, 171>> =
    Table((&raw const crate::data::intro_credits_graphics::sHouses_Tilemap).cast());
static sSpriteMetadata_Clouds: Table<CArray<IntroCreditsSpriteMetadata, 9>> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteMetadata_Clouds).cast());
static sSpriteMetadata_HouseSilhouette: Table<CArray<IntroCreditsSpriteMetadata, 6>> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteMetadata_HouseSilhouette).cast());
static sSpriteMetadata_Trees: Table<CArray<IntroCreditsSpriteMetadata, 12>> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteMetadata_Trees).cast());
static sSpriteSheet_Clouds: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteSheet_Clouds).cast());
static sSpriteSheet_HouseSilhouette: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteSheet_HouseSilhouette).cast());
static sSpriteSheet_TreesSmall: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteSheet_TreesSmall).cast());
static sSpriteTemplate_Brendan: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_Brendan).cast());
static sSpriteTemplate_BrendanBicycle: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_BrendanBicycle).cast());
static sSpriteTemplate_FlygonLatias: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_FlygonLatias).cast());
static sSpriteTemplate_FlygonLatios: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_FlygonLatios).cast());
static sSpriteTemplate_May: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_May).cast());
static sSpriteTemplate_MayBicycle: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_MayBicycle).cast());
static sSpriteTemplate_MovingScenery: Table<SpriteTemplate> =
    Table((&raw const crate::data::intro_credits_graphics::sSpriteTemplate_MovingScenery).cast());
static sTreesSmall_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sTreesSmall_Pal).cast());
static sTreesSunset_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sTreesSunset_Pal).cast());
static sTrees_Gfx: Table<CArray<u32, 418>> =
    Table((&raw const crate::data::intro_credits_graphics::sTrees_Gfx).cast());
static sTrees_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::intro_credits_graphics::sTrees_Pal).cast());
static sTrees_Tilemap: Table<CArray<u32, 193>> =
    Table((&raw const crate::data::intro_credits_graphics::sTrees_Tilemap).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroCredits_MovingSceneryVBase: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroCredits_MovingSceneryVOffset: i16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroCredits_MovingSceneryState: i16 = 0;

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadIntroPart2Graphics(scenery: u8) {
    LZ77UnCompVram(
        sGrass_Gfx.as_ptr().cast_mut(),
        0x6004000 as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sGrass_Tilemap.as_ptr().cast_mut(),
        0x6007800 as usize as *mut c_void,
    );
    LoadPalette((&raw const *sGrass_Pal).cast_mut() as *mut c_void, 240, 32);
    match scenery {
        1 => {
            LZ77UnCompVram(sTrees_Gfx.as_ptr().cast_mut(), VRAM as usize as *mut c_void);
            LZ77UnCompVram(
                sTrees_Tilemap.as_ptr().cast_mut(),
                0x6003000 as usize as *mut c_void,
            );
            LoadPalette((&raw const *sTrees_Pal).cast_mut() as *mut c_void, 0, 32);
            LoadCompressedSpriteSheet(sSpriteSheet_TreesSmall.as_ptr().cast_mut());
            LoadPalette(
                (&raw const *sTreesSmall_Pal).cast_mut() as *mut c_void,
                256,
                32,
            );
            CreateTreeSprites();
        }
        _ => {
            LZ77UnCompVram(
                sCloudsBg_Gfx.as_ptr().cast_mut(),
                VRAM as usize as *mut c_void,
            );
            LZ77UnCompVram(
                sCloudsBg_Tilemap.as_ptr().cast_mut(),
                0x6003000 as usize as *mut c_void,
            );
            LoadPalette((&raw const *sCloudsBg_Pal).cast_mut() as *mut c_void, 0, 96);
            LoadCompressedSpriteSheet(sSpriteSheet_Clouds.as_ptr().cast_mut());
            LoadPalette((&raw const *sClouds_Pal).cast_mut() as *mut c_void, 256, 32);
            CreateCloudSprites();
        }
    }
    gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_NORMAL;
    gReservedSpritePaletteCount = 8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetIntroPart2BgCnt(scenery: u8) {
    match scenery {
        1 => {
            SetGpuReg(REG_OFFSET_BG3CNT, 1539);
            SetGpuReg(REG_OFFSET_BG2CNT, 1794);
            SetGpuReg(REG_OFFSET_BG1CNT, 3845);
            SetGpuReg(0x0, 7744);
        }
        2 => {
            SetGpuReg(REG_OFFSET_BG3CNT, 1539);
            SetGpuReg(REG_OFFSET_BG2CNT, 1794);
            SetGpuReg(REG_OFFSET_BG1CNT, 3845);
            SetGpuReg(0x0, 7744);
        }
        _ => {
            SetGpuReg(REG_OFFSET_BG3CNT, 1539);
            SetGpuReg(REG_OFFSET_BG2CNT, 1794);
            SetGpuReg(REG_OFFSET_BG1CNT, 3845);
            SetGpuReg(0x0, 7744);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCreditsSceneGraphics(scene: u8) {
    LZ77UnCompVram(
        sGrass_Gfx.as_ptr().cast_mut(),
        0x6004000 as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sGrass_Tilemap.as_ptr().cast_mut(),
        0x6007800 as usize as *mut c_void,
    );
    match scene {
        1 => {
            LoadPalette(
                (&raw const *sGrassSunset_Pal).cast_mut() as *mut c_void,
                240,
                32,
            );
            LZ77UnCompVram(
                sCloudsBg_Gfx.as_ptr().cast_mut(),
                VRAM as usize as *mut c_void,
            );
            LZ77UnCompVram(
                sCloudsBg_Tilemap.as_ptr().cast_mut(),
                0x6003000 as usize as *mut c_void,
            );
            LoadPalette(
                (&raw const *sCloudsBgSunset_Pal).cast_mut() as *mut c_void,
                0,
                96,
            );
            LoadCompressedSpriteSheet(sSpriteSheet_Clouds.as_ptr().cast_mut());
            LZ77UnCompVram(
                sClouds_Gfx.as_ptr().cast_mut(),
                OBJ_VRAM0 as usize as *mut c_void,
            );
            LoadPalette(
                (&raw const *sCloudsSunset_Pal).cast_mut() as *mut c_void,
                256,
                32,
            );
            CreateCloudSprites();
        }
        SCENE_FOREST_RIVAL_ARRIVE | 3 => {
            LoadPalette(
                (&raw const *sGrassSunset_Pal).cast_mut() as *mut c_void,
                240,
                32,
            );
            LZ77UnCompVram(sTrees_Gfx.as_ptr().cast_mut(), VRAM as usize as *mut c_void);
            LZ77UnCompVram(
                sTrees_Tilemap.as_ptr().cast_mut(),
                0x6003000 as usize as *mut c_void,
            );
            LoadPalette(
                (&raw const *sTreesSunset_Pal).cast_mut() as *mut c_void,
                0,
                32,
            );
            LoadCompressedSpriteSheet(sSpriteSheet_TreesSmall.as_ptr().cast_mut());
            LoadPalette(
                (&raw const *sTreesSunset_Pal).cast_mut() as *mut c_void,
                256,
                32,
            );
            CreateTreeSprites();
        }
        4 => {
            LoadPalette(
                (&raw const *sGrassNight_Pal).cast_mut() as *mut c_void,
                240,
                32,
            );
            LZ77UnCompVram(
                sHouses_Gfx.as_ptr().cast_mut(),
                VRAM as usize as *mut c_void,
            );
            LZ77UnCompVram(
                sHouses_Tilemap.as_ptr().cast_mut(),
                0x6003000 as usize as *mut c_void,
            );
            LoadPalette((&raw const *sHouses_Pal).cast_mut() as *mut c_void, 0, 64);
            LoadCompressedSpriteSheet(sSpriteSheet_HouseSilhouette.as_ptr().cast_mut());
            LoadPalette(
                (&raw const *sHouseSilhouette_Pal).cast_mut() as *mut c_void,
                256,
                32,
            );
            CreateHouseSprites();
        }
        _ => {
            LoadPalette((&raw const *sGrass_Pal).cast_mut() as *mut c_void, 240, 32);
            LZ77UnCompVram(
                sCloudsBg_Gfx.as_ptr().cast_mut(),
                VRAM as usize as *mut c_void,
            );
            LZ77UnCompVram(
                sCloudsBg_Tilemap.as_ptr().cast_mut(),
                0x6003000 as usize as *mut c_void,
            );
            LoadPalette((&raw const *sCloudsBg_Pal).cast_mut() as *mut c_void, 0, 96);
            LoadCompressedSpriteSheet(sSpriteSheet_Clouds.as_ptr().cast_mut());
            LZ77UnCompVram(
                sClouds_Gfx.as_ptr().cast_mut(),
                OBJ_VRAM0 as usize as *mut c_void,
            );
            LoadPalette((&raw const *sClouds_Pal).cast_mut() as *mut c_void, 256, 32);
            CreateCloudSprites();
        }
    }
    gReservedSpritePaletteCount = 8;
    gIntroCredits_MovingSceneryState = INTROCRED_SCENERY_NORMAL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCreditsSceneBgCnt(scene: u8) {
    SetGpuReg(REG_OFFSET_BG3CNT, 1539);
    SetGpuReg(REG_OFFSET_BG2CNT, 1794);
    SetGpuReg(REG_OFFSET_BG1CNT, 3845);
    SetGpuReg(0x0, 8000);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBicycleBgAnimationTask(
    mode: u8,
    bg1Speed: u16,
    bg2Speed: u16,
    bg3Speed: u16,
) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_BicycleBgAnimation), 0);
    gTasks[taskId].data[0] = mode as i16;
    gTasks[taskId].data[1] = bg1Speed as i16;
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].data[3] = 0;
    gTasks[taskId].data[4] = bg2Speed as i16;
    gTasks[taskId].data[5] = 0;
    gTasks[taskId].data[6] = 0;
    gTasks[taskId].data[7] = bg3Speed as i16;
    gTasks[taskId].data[8] = 8;
    gTasks[taskId].data[9] = 0;
    Task_BicycleBgAnimation(taskId);
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_BicycleBgAnimation(taskId: u8) {
    let mut bg1Speed: i16 = 0;
    let mut bg2Speed: i16 = 0;
    let mut bg3Speed: i16 = 0;
    let mut offset: i32 = 0;
    bg1Speed = gTasks[taskId].data[1];
    if bg1Speed != 0 {
        offset = ((gTasks[taskId].data[2] as i32) << 16) + gTasks[taskId].data[3] as u16 as i32;
        offset -= (bg1Speed as u16 as i32) << 4;
        gTasks[taskId].data[2] = (offset >> 16) as i16;
        gTasks[taskId].data[3] = offset as i16;
        SetGpuReg(REG_OFFSET_BG1HOFS, gTasks[taskId].data[2] as u16);
        SetGpuReg(
            REG_OFFSET_BG1VOFS,
            gIntroCredits_MovingSceneryVBase + gIntroCredits_MovingSceneryVOffset as u16,
        );
    }
    bg2Speed = gTasks[taskId].data[4];
    if bg2Speed != 0 {
        offset = ((gTasks[taskId].data[5] as i32) << 16) + gTasks[taskId].data[6] as u16 as i32;
        offset -= (bg2Speed as u16 as i32) << 4;
        gTasks[taskId].data[5] = (offset >> 16) as i16;
        gTasks[taskId].data[6] = offset as i16;
        SetGpuReg(REG_OFFSET_BG2HOFS, gTasks[taskId].data[5] as u16);
        if gTasks[taskId].data[0] != 0 {
            SetGpuReg(
                REG_OFFSET_BG2VOFS,
                gIntroCredits_MovingSceneryVBase + gIntroCredits_MovingSceneryVOffset as u16,
            );
        } else {
            SetGpuReg(REG_OFFSET_BG2VOFS, gIntroCredits_MovingSceneryVBase);
        }
    }
    bg3Speed = gTasks[taskId].data[7];
    if bg3Speed != 0 {
        offset = ((gTasks[taskId].data[8] as i32) << 16) + gTasks[taskId].data[9] as u16 as i32;
        offset -= (bg3Speed as u16 as i32) << 4;
        gTasks[taskId].data[8] = (offset >> 16) as i16;
        gTasks[taskId].data[9] = offset as i16;
        SetGpuReg(REG_OFFSET_BG3HOFS, gTasks[taskId].data[8] as u16);
        SetGpuReg(REG_OFFSET_BG3VOFS, gIntroCredits_MovingSceneryVBase);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CycleSceneryPalette(mode: u8) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    'l1: {
        match mode {
            2 => {
                if gMain.vblankCounter1 & 3 != 0 || gPaletteFade.active() != 0 {
                    break 'l1;
                }
                if gMain.vblankCounter1 & 4 != 0 {
                    x = 15655;
                    y = 661;
                } else {
                    x = 796;
                    y = 15655;
                }
                LoadPalette(&raw mut x as *mut c_void, 12, 2);
                LoadPalette(&raw mut y as *mut c_void, 13, 2);
            }
            1 => {}
            _ => {
                if gMain.vblankCounter1 & 3 != 0 || gPaletteFade.active() != 0 {
                    break 'l1;
                }
                if gMain.vblankCounter1 & 4 != 0 {
                    x = gPlttBufferUnfaded[9];
                    y = gPlttBufferUnfaded[10];
                } else {
                    x = gPlttBufferUnfaded[10];
                    y = gPlttBufferUnfaded[9];
                }
                LoadPalette(&raw mut x as *mut c_void, 9, 2);
                LoadPalette(&raw mut y as *mut c_void, 10, 2);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MovingScenery(sprite: *mut Sprite) {
    let mut x: i32 = 0;
    let mut state: i16 = gIntroCredits_MovingSceneryState;
    if state != INTROCRED_SCENERY_FROZEN {
        match state {
            INTROCRED_SCENERY_NORMAL => {
                x = (((*sprite).x as i32) << 16 | (*sprite).data[2] as u16 as i32)
                    + (*sprite).data[1] as u16 as i32;
                (*sprite).x = (x >> 16) as i16;
                (*sprite).data[2] = x as i16;
                if (*sprite).x > 255 {
                    (*sprite).x = -32;
                }
                if (*sprite).data[0] != 0 {
                    (*sprite).y2 = -(gIntroCredits_MovingSceneryVBase as i16
                        + gIntroCredits_MovingSceneryVOffset);
                } else {
                    (*sprite).y2 = -(gIntroCredits_MovingSceneryVBase as i16);
                }
            }
            _ => {
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMovingScenerySprites(
    hasVerticalMove: u8,
    metadata: *mut IntroCreditsSpriteMetadata,
    anims: *mut *mut AnimCmd,
    numSprites: u8,
) {
    let mut i: u8 = 0;
    i = 0;
    while i < numSprites {
        let mut sprite: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_MovingScenery).cast_mut(),
            (*metadata.at(i)).x as i16,
            (*metadata.at(i)).y as i16,
            (*metadata.at(i)).subpriority,
        );
        CalcCenterToCornerVec(
            &raw mut gSprites[sprite],
            (*metadata.at(i)).shape(),
            (*metadata.at(i)).size(),
            ST_OAM_AFFINE_OFF as u8,
        );
        gSprites[sprite].oam.set_priority(3);
        gSprites[sprite]
            .oam
            .set_shape((*metadata.at(i)).shape() as u32);
        gSprites[sprite]
            .oam
            .set_size((*metadata.at(i)).size() as u32);
        gSprites[sprite].oam.set_paletteNum(0);
        gSprites[sprite].anims = anims;
        StartSpriteAnim(&raw mut gSprites[sprite], (*metadata.at(i)).animNum());
        gSprites[sprite].data[0] = hasVerticalMove as i16;
        gSprites[sprite].data[1] = (*metadata.at(i)).xOff as i16;
        gSprites[sprite].data[2] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateCloudSprites() {
    CreateMovingScenerySprites(
        FALSE,
        sSpriteMetadata_Clouds.as_ptr().cast_mut(),
        sAnims_Clouds.as_ptr().cast_mut(),
        9,
    );
}
pub(crate) unsafe extern "C" fn CreateTreeSprites() {
    CreateMovingScenerySprites(
        TRUE,
        sSpriteMetadata_Trees.as_ptr().cast_mut(),
        sAnims_Trees.as_ptr().cast_mut(),
        12,
    );
}
pub(crate) unsafe extern "C" fn CreateHouseSprites() {
    CreateMovingScenerySprites(
        TRUE,
        sSpriteMetadata_HouseSilhouette.as_ptr().cast_mut(),
        sAnims_HouseSilhouette.as_ptr().cast_mut(),
        6,
    );
}
pub(crate) unsafe extern "C" fn SpriteCB_Player(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn SpriteCB_Bicycle(sprite: *mut Sprite) {
    (*sprite).set_invisible(gSprites[(*sprite).data[0]].invisible());
    (*sprite).x = gSprites[(*sprite).data[0]].x;
    (*sprite).y = gSprites[(*sprite).data[0]].y + 8;
    (*sprite).x2 = gSprites[(*sprite).data[0]].x2;
    (*sprite).y2 = gSprites[(*sprite).data[0]].y2;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateIntroBrendanSprite(x: i16, y: i16) -> u8 {
    let mut playerSpriteId: u8 =
        CreateSprite((&raw const *sSpriteTemplate_Brendan).cast_mut(), x, y, 2);
    let mut bicycleSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_BrendanBicycle).cast_mut(),
        x,
        y + 8,
        3,
    );
    gSprites[bicycleSpriteId].data[0] = playerSpriteId as i16;
    return playerSpriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateIntroMaySprite(x: i16, y: i16) -> u8 {
    let mut playerSpriteId: u8 =
        CreateSprite((&raw const *sSpriteTemplate_May).cast_mut(), x, y, 2);
    let mut bicycleSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_MayBicycle).cast_mut(),
        x,
        y + 8,
        3,
    );
    gSprites[bicycleSpriteId].data[0] = playerSpriteId as i16;
    return playerSpriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_FlygonLeftHalf(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn SpriteCB_FlygonRightHalf(sprite: *mut Sprite) {
    (*sprite).set_invisible(gSprites[(*sprite).data[0]].invisible());
    (*sprite).y = gSprites[(*sprite).data[0]].y;
    (*sprite).x2 = gSprites[(*sprite).data[0]].x2;
    (*sprite).y2 = gSprites[(*sprite).data[0]].y2;
}
pub(crate) unsafe extern "C" fn CreateIntroFlygonSprite_Unused(x: i16, y: i16) -> u8 {
    let mut leftSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_FlygonLatios).cast_mut(),
        x - 32,
        y,
        5,
    );
    let mut rightSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_FlygonLatios).cast_mut(),
        x + 32,
        y,
        6,
    );
    gSprites[rightSpriteId].data[0] = leftSpriteId as i16;
    StartSpriteAnim(&raw mut gSprites[rightSpriteId], 1);
    gSprites[rightSpriteId].callback =
        Some(SpriteCB_FlygonRightHalf as unsafe extern "C" fn(*mut Sprite));
    return leftSpriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateIntroFlygonSprite(x: i16, y: i16) -> u8 {
    let mut leftSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_FlygonLatias).cast_mut(),
        x - 32,
        y,
        5,
    );
    let mut rightSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_FlygonLatias).cast_mut(),
        x + 32,
        y,
        6,
    );
    gSprites[rightSpriteId].data[0] = leftSpriteId as i16;
    StartSpriteAnim(&raw mut gSprites[rightSpriteId], 1);
    gSprites[rightSpriteId].callback =
        Some(SpriteCB_FlygonRightHalf as unsafe extern "C" fn(*mut Sprite));
    return leftSpriteId;
}
