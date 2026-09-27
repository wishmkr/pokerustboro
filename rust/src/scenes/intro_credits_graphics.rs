//! Translated from `src/intro_credits_graphics.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sGrass_Pal sGrassSunset_Pal sGrassNight_Pal sGrass_Gfx sGrass_Tilemap sCloudsBg_Pal sCloudsBgSunset_Pal sCloudsBg_Gfx sCloudsBg_Tilemap sClouds_Pal sCloudsSunset_Pal sClouds_Gfx sTrees_Pal sTreesSunset_Pal sTrees_Gfx sTrees_Tilemap sTreesSmall_Pal sTreesSmall_Gfx sHouses_Pal sHouses_Gfx sHouseSilhouette_Pal sHouses_Tilemap sHouseSilhouette_Gfx sBrendanCredits_Pal sBrendanCredits_Gfx sMayCredits_Pal sUnused sMayCredits_Gfx sBicycle_Gfx sLatios_Pal sLatios_Gfx sLatias_Pal sLatias_Gfx sSpriteTemplate_MovingScenery sSpriteSheet_Clouds sAnim_Cloud_Largest sAnim_Cloud_Large sAnim_Cloud_Small sAnim_Cloud_Smallest sAnims_Clouds sSpriteMetadata_Clouds sSpriteSheet_TreesSmall sAnim_Trees_0 sAnim_Trees_1 sAnim_Trees_2 sAnims_Trees sSpriteMetadata_Trees sSpriteSheet_HouseSilhouette sAnim_HouseSilhouette sAnims_HouseSilhouette sSpriteMetadata_HouseSilhouette sOamData_Player sAnim_Player sAnims_Player sSpriteTemplate_Brendan sSpriteTemplate_May sOamData_Bicycle sAnim_Bicycle sAnims_Bicycle sSpriteTemplate_BrendanBicycle sSpriteTemplate_MayBicycle sOamData_Flygon sAnim_FlygonLeft sAnim_FlygonRight sAnims_Flygon sSpriteTemplate_FlygonLatios sSpriteTemplate_FlygonLatias gSpriteSheet_IntroBrendan gSpriteSheet_IntroMay gSpriteSheet_IntroBicycle sSpriteSheet_IntroFlygon_Unused gSpriteSheet_IntroFlygon gSpritePalettes_IntroPlayerFlygon gSpriteSheet_CreditsBrendan gSpriteSheet_CreditsMay gSpriteSheet_CreditsBicycle sSpriteSheet_Latios sSpriteSheet_Latias gSpritePalettes_Credits gSpriteSheet_CreditsRivalBrendan gSpriteSheet_CreditsRivalMay
#[allow(unused_imports)]
use crate::data::intro_credits_graphics::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroCredits_MovingSceneryVBase: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroCredits_MovingSceneryVOffset: i16 = 0i16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroCredits_MovingSceneryState: i16 = 0i16;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadIntroPart2Graphics(scenery: u8) {
    unsafe {
        let mut scenery = scenery;
        LZ77UnCompVram(
            ((&raw const sGrass_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100679680i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sGrass_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100694016i32) as usize as *mut u8),
        );
        LoadPalette(
            (((&raw const sGrass_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        'l1: {
            let __sw1 = ((scenery) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                LZ77UnCompVram(
                    ((&raw const sCloudsBg_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sCloudsBg_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100675584i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sCloudsBg_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    96u16,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_Clouds).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadPalette(
                    (((&raw const sClouds_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    256u16,
                    32u16,
                );
                CreateCloudSprites();
                break 'l1;
            }
            if __sw1 == 1i32 {
                LZ77UnCompVram(
                    ((&raw const sTrees_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sTrees_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100675584i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sTrees_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    32u16,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_TreesSmall).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadPalette(
                    (((&raw const sTreesSmall_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    256u16,
                    32u16,
                );
                CreateTreeSprites();
                break 'l1;
            }
        }
        ((&raw mut gIntroCredits_MovingSceneryState)
            .cast::<u8>()
            .cast::<i16>())
        .write(0i16);
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetIntroPart2BgCnt(scenery: u8) {
    unsafe {
        let mut scenery = scenery;
        'l1: {
            let __sw1 = ((scenery) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                SetGpuReg(14u8, 1539u16);
                SetGpuReg(12u8, 1794u16);
                SetGpuReg(10u8, 3845u16);
                SetGpuReg(0u8, 7744u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetGpuReg(14u8, 1539u16);
                SetGpuReg(12u8, 1794u16);
                SetGpuReg(10u8, 3845u16);
                SetGpuReg(0u8, 7744u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetGpuReg(14u8, 1539u16);
                SetGpuReg(12u8, 1794u16);
                SetGpuReg(10u8, 3845u16);
                SetGpuReg(0u8, 7744u16);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadCreditsSceneGraphics(scene: u8) {
    unsafe {
        let mut scene = scene;
        LZ77UnCompVram(
            ((&raw const sGrass_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100679680i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sGrass_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100694016i32) as usize as *mut u8),
        );
        'l1: {
            let __sw1 = ((scene) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 || !__matched {
                LoadPalette(
                    (((&raw const sGrass_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    240u16,
                    32u16,
                );
                LZ77UnCompVram(
                    ((&raw const sCloudsBg_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sCloudsBg_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100675584i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sCloudsBg_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    96u16,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_Clouds).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LZ77UnCompVram(
                    ((&raw const sClouds_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100728832i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sClouds_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    256u16,
                    32u16,
                );
                CreateCloudSprites();
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadPalette(
                    (((&raw const sGrassSunset_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    240u16,
                    32u16,
                );
                LZ77UnCompVram(
                    ((&raw const sCloudsBg_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sCloudsBg_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100675584i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sCloudsBgSunset_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    96u16,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_Clouds).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LZ77UnCompVram(
                    ((&raw const sClouds_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100728832i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sCloudsSunset_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    256u16,
                    32u16,
                );
                CreateCloudSprites();
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                LoadPalette(
                    (((&raw const sGrassSunset_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    240u16,
                    32u16,
                );
                LZ77UnCompVram(
                    ((&raw const sTrees_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sTrees_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100675584i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sTreesSunset_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    32u16,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_TreesSmall).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadPalette(
                    (((&raw const sTreesSunset_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    256u16,
                    32u16,
                );
                CreateTreeSprites();
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadPalette(
                    (((&raw const sGrassNight_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    240u16,
                    32u16,
                );
                LZ77UnCompVram(
                    ((&raw const sHouses_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sHouses_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100675584i32) as usize as *mut u8),
                );
                LoadPalette(
                    (((&raw const sHouses_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    64u16,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSpriteSheet_HouseSilhouette)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadPalette(
                    (((&raw const sHouseSilhouette_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    256u16,
                    32u16,
                );
                CreateHouseSprites();
                break 'l1;
            }
        }
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(8u8);
        ((&raw mut gIntroCredits_MovingSceneryState)
            .cast::<u8>()
            .cast::<i16>())
        .write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCreditsSceneBgCnt(scene: u8) {
    unsafe {
        let mut scene = scene;
        SetGpuReg(14u8, 1539u16);
        SetGpuReg(12u8, 1794u16);
        SetGpuReg(10u8, 3845u16);
        SetGpuReg(0u8, 8000u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBicycleBgAnimationTask(
    mode: u8,
    bg1Speed: u16,
    bg2Speed: u16,
    bg3Speed: u16,
) -> u8 {
    unsafe {
        let mut mode = mode;
        let mut bg1Speed = bg1Speed;
        let mut bg2Speed = bg2Speed;
        let mut bg3Speed = bg3Speed;
        let mut taskId: u8 = CreateTask(Some(Task_BicycleBgAnimation), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((mode) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((bg1Speed) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((bg2Speed) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((bg3Speed) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(8i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(0i16);
        Task_BicycleBgAnimation(taskId);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_BicycleBgAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut bg1Speed: i16 = 0i16;
        let mut bg2Speed: i16 = 0i16;
        let mut bg3Speed: i16 = 0i16;
        let mut offset: i32 = 0i32;
        bg1Speed = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        if ((bg1Speed) as i32) != 0i32 {
            offset = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16)
                .wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u16) as i32),
                );
            offset = (offset).wrapping_sub(((((bg1Speed) as u16) as i32) << 4));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((offset >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((offset) as i16));
            SetGpuReg(
                20u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u16),
            );
            SetGpuReg(
                22u8,
                ((((((&raw mut gIntroCredits_MovingSceneryVBase)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((&raw mut gIntroCredits_MovingSceneryVOffset)
                            .cast::<u8>()
                            .cast::<i16>())
                        .read()) as i32),
                    )) as u16),
            );
        }
        bg2Speed = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read();
        if ((bg2Speed) as i32) != 0i32 {
            offset = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                << 16)
                .wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as u16) as i32),
                );
            offset = (offset).wrapping_sub(((((bg2Speed) as u16) as i32) << 4));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((offset >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((offset) as i16));
            SetGpuReg(
                24u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u16),
            );
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                != 0i32
            {
                SetGpuReg(
                    26u8,
                    ((((((&raw mut gIntroCredits_MovingSceneryVBase)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((&raw mut gIntroCredits_MovingSceneryVOffset)
                                .cast::<u8>()
                                .cast::<i16>())
                            .read()) as i32),
                        )) as u16),
                );
            } else {
                SetGpuReg(
                    26u8,
                    ((&raw mut gIntroCredits_MovingSceneryVBase)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read(),
                );
            }
        }
        bg3Speed = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .read();
        if ((bg3Speed) as i32) != 0i32 {
            offset = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .read()) as i32)
                << 16)
                .wrapping_add(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as u16) as i32),
                );
            offset = (offset).wrapping_sub(((((bg3Speed) as u16) as i32) << 4));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(((offset >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .write(((offset) as i16));
            SetGpuReg(
                28u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .read()) as u16),
            );
            SetGpuReg(
                30u8,
                ((&raw mut gIntroCredits_MovingSceneryVBase)
                    .cast::<u8>()
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CycleSceneryPalette(mode: u8) {
    unsafe {
        let mut mode = mode;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        'l1: {
            let __sw1 = ((mode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                if (((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read()
                    & 3u32)
                    != 0)
                    || ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                        7,
                        1,
                        false,
                    ) as u16)
                        != 0)
                {
                    break 'l1;
                }
                if ((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read()
                    & 4u32)
                    != 0
                {
                    x = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(9))
                    .read();
                    y = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(10))
                    .read();
                } else {
                    x = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(10))
                    .read();
                    y = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(9))
                    .read();
                }
                LoadPalette((&raw mut x).cast::<u8>(), 9u16, 2u16);
                LoadPalette((&raw mut y).cast::<u8>(), 10u16, 2u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read()
                    & 3u32)
                    != 0)
                    || ((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                        7,
                        1,
                        false,
                    ) as u16)
                        != 0)
                {
                    break 'l1;
                }
                if ((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read()
                    & 4u32)
                    != 0
                {
                    x = 15655u16;
                    y = 661u16;
                } else {
                    x = 796u16;
                    y = 15655u16;
                }
                LoadPalette((&raw mut x).cast::<u8>(), 12u16, 2u16);
                LoadPalette((&raw mut y).cast::<u8>(), 13u16, 2u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MovingScenery(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i32 = 0i32;
        let mut state: i16 = ((&raw mut gIntroCredits_MovingSceneryState)
            .cast::<u8>()
            .cast::<i16>())
        .read();
        if ((state) as i32) != 2i32 {
            'l1: {
                let __sw1 = ((state) as i32);
                let __matched = __sw1 == 0i32;
                if !__matched {
                    DestroySprite(sprite);
                    break 'l1;
                }
                if __sw1 == 0i32 {
                    x = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 16)
                        | (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .read()) as u16) as i32))
                        .wrapping_add(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as u16) as i32),
                        );
                    ((sprite).wrapping_add(32).cast::<i16>()).write(((x >> 16) as i16));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                        .write(((x) as i16));
                    if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 255i32 {
                        ((sprite).wrapping_add(32).cast::<i16>()).write((-32i16));
                    }
                    if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
                        ((sprite).wrapping_add(38).cast::<i16>()).write(
                            (((((((&raw mut gIntroCredits_MovingSceneryVBase)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(
                                    ((((&raw mut gIntroCredits_MovingSceneryVOffset)
                                        .cast::<u8>()
                                        .cast::<i16>())
                                    .read()) as i32),
                                ))
                            .wrapping_neg()) as i16),
                        );
                    } else {
                        ((sprite).wrapping_add(38).cast::<i16>()).write(
                            ((((((&raw mut gIntroCredits_MovingSceneryVBase)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_neg()) as i16),
                        );
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMovingScenerySprites(
    hasVerticalMove: u8,
    metadata: *mut u8,
    anims: *mut *mut u8,
    numSprites: u8,
) {
    unsafe {
        let mut hasVerticalMove = hasVerticalMove;
        let mut metadata = metadata;
        let mut anims = anims;
        let mut numSprites = numSprites;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numSprites) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_MovingScenery)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(1))
                            .read()) as i16),
                        (((((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(2))
                            .read()) as i16),
                        (((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(3))
                            .read(),
                    );
                    CalcCenterToCornerVec(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((sprite) as i32) as isize * 68),
                        (crate::c::bf_read(
                            ((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(0),
                            4,
                            2,
                            false,
                        ) as u8),
                        (crate::c::bf_read(
                            ((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(0),
                            6,
                            2,
                            false,
                        ) as u8),
                        0u8,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((sprite) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (3u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((sprite) as i32) as isize * 68))
                        .wrapping_add(1),
                        6,
                        2,
                        ((crate::c::bf_read(
                            ((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(0),
                            4,
                            2,
                            false,
                        ) as u8) as u32) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((sprite) as i32) as isize * 68))
                        .wrapping_add(3),
                        6,
                        2,
                        ((crate::c::bf_read(
                            ((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(0),
                            6,
                            2,
                            false,
                        ) as u8) as u32) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((sprite) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        (0u16) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((sprite) as i32) as isize * 68))
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                    .write(anims);
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((sprite) as i32) as isize * 68),
                        (crate::c::bf_read(
                            ((metadata).wrapping_offset(((i) as i32) as isize * 8)).wrapping_add(0),
                            0,
                            4,
                            false,
                        ) as u8),
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((sprite) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((hasVerticalMove) as i16));
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((sprite) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        (((((metadata).wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read()) as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((sprite) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCloudSprites() {
    unsafe {
        CreateMovingScenerySprites(
            0u8,
            ((&raw const sSpriteMetadata_Clouds).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw const sAnims_Clouds)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
            9u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateTreeSprites() {
    unsafe {
        CreateMovingScenerySprites(
            1u8,
            ((&raw const sSpriteMetadata_Trees).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw const sAnims_Trees)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
            12u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateHouseSprites() {
    unsafe {
        CreateMovingScenerySprites(
            1u8,
            ((&raw const sSpriteMetadata_HouseSilhouette)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((&raw const sAnims_HouseSilhouette)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
            6u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Player(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Bicycle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16) as i32,
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(8i32)) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateIntroBrendanSprite(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut playerSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_Brendan).cast::<u8>().cast_mut(),
            x,
            y,
            2u8,
        );
        let mut bicycleSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_BrendanBicycle)
                .cast::<u8>()
                .cast_mut(),
            x,
            ((((y) as i32).wrapping_add(8i32)) as i16),
            3u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bicycleSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((playerSpriteId) as i16));
        return playerSpriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateIntroMaySprite(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut playerSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_May).cast::<u8>().cast_mut(),
            x,
            y,
            2u8,
        );
        let mut bicycleSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_MayBicycle)
                .cast::<u8>()
                .cast_mut(),
            x,
            ((((y) as i32).wrapping_add(8i32)) as i16),
            3u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bicycleSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((playerSpriteId) as i16));
        return playerSpriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlygonLeftHalf(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlygonRightHalf(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16) as i32,
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateIntroFlygonSprite_Unused(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut leftSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_FlygonLatios)
                .cast::<u8>()
                .cast_mut(),
            ((((x) as i32).wrapping_sub(32i32)) as i16),
            y,
            5u8,
        );
        let mut rightSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_FlygonLatios)
                .cast::<u8>()
                .cast_mut(),
            ((((x) as i32).wrapping_add(32i32)) as i16),
            y,
            6u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((rightSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((leftSpriteId) as i16));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((rightSpriteId) as i32) as isize * 68),
            1u8,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((rightSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_FlygonRightHalf));
        return leftSpriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateIntroFlygonSprite(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut leftSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_FlygonLatias)
                .cast::<u8>()
                .cast_mut(),
            ((((x) as i32).wrapping_sub(32i32)) as i16),
            y,
            5u8,
        );
        let mut rightSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_FlygonLatias)
                .cast::<u8>()
                .cast_mut(),
            ((((x) as i32).wrapping_add(32i32)) as i16),
            y,
            6u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((rightSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((leftSpriteId) as i16));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((rightSpriteId) as i32) as isize * 68),
            1u8,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((rightSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_FlygonRightHalf));
        return leftSpriteId;
    }
}
