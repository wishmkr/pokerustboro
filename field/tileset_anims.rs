//! Translated from `src/tileset_anims.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gTilesetAnims_General_Flower_Frame1 gTilesetAnims_General_Flower_Frame0 gTilesetAnims_General_Flower_Frame2 tileset_anims_space_0 gTilesetAnims_General_Flower gTilesetAnims_General_Water_Frame0 gTilesetAnims_General_Water_Frame1 gTilesetAnims_General_Water_Frame2 gTilesetAnims_General_Water_Frame3 gTilesetAnims_General_Water_Frame4 gTilesetAnims_General_Water_Frame5 gTilesetAnims_General_Water_Frame6 gTilesetAnims_General_Water_Frame7 gTilesetAnims_General_Water gTilesetAnims_General_SandWaterEdge_Frame0 gTilesetAnims_General_SandWaterEdge_Frame1 gTilesetAnims_General_SandWaterEdge_Frame2 gTilesetAnims_General_SandWaterEdge_Frame3 gTilesetAnims_General_SandWaterEdge_Frame4 gTilesetAnims_General_SandWaterEdge_Frame5 gTilesetAnims_General_SandWaterEdge_Frame6 gTilesetAnims_General_SandWaterEdge gTilesetAnims_General_Waterfall_Frame0 gTilesetAnims_General_Waterfall_Frame1 gTilesetAnims_General_Waterfall_Frame2 gTilesetAnims_General_Waterfall_Frame3 gTilesetAnims_General_Waterfall gTilesetAnims_General_LandWaterEdge_Frame0 gTilesetAnims_General_LandWaterEdge_Frame1 gTilesetAnims_General_LandWaterEdge_Frame2 gTilesetAnims_General_LandWaterEdge_Frame3 gTilesetAnims_General_LandWaterEdge gTilesetAnims_Lavaridge_Steam_Frame0 gTilesetAnims_Lavaridge_Steam_Frame1 gTilesetAnims_Lavaridge_Steam_Frame2 gTilesetAnims_Lavaridge_Steam_Frame3 gTilesetAnims_Lavaridge_Steam gTilesetAnims_Pacifidlog_LogBridges_Frame0 gTilesetAnims_Pacifidlog_LogBridges_Frame1 gTilesetAnims_Pacifidlog_LogBridges_Frame2 gTilesetAnims_Pacifidlog_LogBridges gTilesetAnims_Underwater_Seaweed_Frame0 gTilesetAnims_Underwater_Seaweed_Frame1 gTilesetAnims_Underwater_Seaweed_Frame2 gTilesetAnims_Underwater_Seaweed_Frame3 gTilesetAnims_Underwater_Seaweed gTilesetAnims_Pacifidlog_WaterCurrents_Frame0 gTilesetAnims_Pacifidlog_WaterCurrents_Frame1 gTilesetAnims_Pacifidlog_WaterCurrents_Frame2 gTilesetAnims_Pacifidlog_WaterCurrents_Frame3 gTilesetAnims_Pacifidlog_WaterCurrents_Frame4 gTilesetAnims_Pacifidlog_WaterCurrents_Frame5 gTilesetAnims_Pacifidlog_WaterCurrents_Frame6 gTilesetAnims_Pacifidlog_WaterCurrents_Frame7 gTilesetAnims_Pacifidlog_WaterCurrents gTilesetAnims_Mauville_Flower1_Frame0 gTilesetAnims_Mauville_Flower1_Frame1 gTilesetAnims_Mauville_Flower1_Frame2 gTilesetAnims_Mauville_Flower1_Frame3 gTilesetAnims_Mauville_Flower1_Frame4 gTilesetAnims_Mauville_Flower2_Frame0 gTilesetAnims_Mauville_Flower2_Frame1 gTilesetAnims_Mauville_Flower2_Frame2 gTilesetAnims_Mauville_Flower2_Frame3 gTilesetAnims_Mauville_Flower2_Frame4 tileset_anims_space_1 gTilesetAnims_Mauville_Flower1_VDests gTilesetAnims_Mauville_Flower2_VDests gTilesetAnims_Mauville_Flower1 gTilesetAnims_Mauville_Flower2 gTilesetAnims_Mauville_Flower1_B gTilesetAnims_Mauville_Flower2_B gTilesetAnims_Rustboro_WindyWater_Frame0 gTilesetAnims_Rustboro_WindyWater_Frame1 gTilesetAnims_Rustboro_WindyWater_Frame2 gTilesetAnims_Rustboro_WindyWater_Frame3 gTilesetAnims_Rustboro_WindyWater_Frame4 gTilesetAnims_Rustboro_WindyWater_Frame5 gTilesetAnims_Rustboro_WindyWater_Frame6 gTilesetAnims_Rustboro_WindyWater_Frame7 gTilesetAnims_Rustboro_WindyWater_VDests gTilesetAnims_Rustboro_WindyWater gTilesetAnims_Rustboro_Fountain_Frame0 gTilesetAnims_Rustboro_Fountain_Frame1 tileset_anims_space_2 gTilesetAnims_Rustboro_Fountain gTilesetAnims_Lavaridge_Cave_Lava_Frame0 gTilesetAnims_Lavaridge_Cave_Lava_Frame1 gTilesetAnims_Lavaridge_Cave_Lava_Frame2 gTilesetAnims_Lavaridge_Cave_Lava_Frame3 gTilesetAnims_Lavaridge_Cave_Lava_Frame4 gTilesetAnims_Lavaridge_Cave_Lava_Frame5 gTilesetAnims_Lavaridge_Cave_Lava_Frame6 gTilesetAnims_Lavaridge_Cave_Lava_Frame7 tileset_anims_space_3 gTilesetAnims_Lavaridge_Cave_Lava gTilesetAnims_EverGrande_Flowers_Frame0 gTilesetAnims_EverGrande_Flowers_Frame1 gTilesetAnims_EverGrande_Flowers_Frame2 gTilesetAnims_EverGrande_Flowers_Frame3 gTilesetAnims_EverGrande_Flowers_Frame4 gTilesetAnims_EverGrande_Flowers_Frame5 gTilesetAnims_EverGrande_Flowers_Frame6 gTilesetAnims_EverGrande_Flowers_Frame7 tileset_anims_space_4 gTilesetAnims_EverGrande_VDests gTilesetAnims_EverGrande_Flowers gTilesetAnims_Dewford_Flag_Frame0 gTilesetAnims_Dewford_Flag_Frame1 gTilesetAnims_Dewford_Flag_Frame2 gTilesetAnims_Dewford_Flag_Frame3 gTilesetAnims_Dewford_Flag gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame0 gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame1 gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame2 gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame3 gTilesetAnims_BattleFrontierOutsideWest_Flag gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame0 gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame1 gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame2 gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame3 gTilesetAnims_BattleFrontierOutsideEast_Flag gTilesetAnims_Slateport_Balloons_Frame0 gTilesetAnims_Slateport_Balloons_Frame1 gTilesetAnims_Slateport_Balloons_Frame2 gTilesetAnims_Slateport_Balloons_Frame3 gTilesetAnims_Slateport_Balloons gTilesetAnims_Building_TvTurnedOn_Frame0 gTilesetAnims_Building_TvTurnedOn_Frame1 gTilesetAnims_Building_TvTurnedOn gTilesetAnims_SootopolisGym_SideWaterfall_Frame0 gTilesetAnims_SootopolisGym_SideWaterfall_Frame1 gTilesetAnims_SootopolisGym_SideWaterfall_Frame2 gTilesetAnims_SootopolisGym_FrontWaterfall_Frame0 gTilesetAnims_SootopolisGym_FrontWaterfall_Frame1 gTilesetAnims_SootopolisGym_FrontWaterfall_Frame2 gTilesetAnims_SootopolisGym_SideWaterfall gTilesetAnims_SootopolisGym_FrontWaterfall gTilesetAnims_EliteFour_FloorLight_Frame0 gTilesetAnims_EliteFour_FloorLight_Frame1 gTilesetAnims_EliteFour_WallLights_Frame0 gTilesetAnims_EliteFour_WallLights_Frame1 gTilesetAnims_EliteFour_WallLights_Frame2 gTilesetAnims_EliteFour_WallLights_Frame3 tileset_anims_space_5 gTilesetAnims_EliteFour_WallLights gTilesetAnims_EliteFour_FloorLight gTilesetAnims_MauvilleGym_ElectricGates_Frame0 gTilesetAnims_MauvilleGym_ElectricGates_Frame1 tileset_anims_space_6 gTilesetAnims_MauvilleGym_ElectricGates gTilesetAnims_BikeShop_BlinkingLights_Frame0 gTilesetAnims_BikeShop_BlinkingLights_Frame1 tileset_anims_space_7 gTilesetAnims_BikeShop_BlinkingLights gTilesetAnims_Sootopolis_StormyWater_Frame0 gTilesetAnims_Sootopolis_StormyWater_Frame1 gTilesetAnims_Sootopolis_StormyWater_Frame2 gTilesetAnims_Sootopolis_StormyWater_Frame3 gTilesetAnims_Sootopolis_StormyWater_Frame4 gTilesetAnims_Sootopolis_StormyWater_Frame5 gTilesetAnims_Sootopolis_StormyWater_Frame6 gTilesetAnims_Sootopolis_StormyWater_Frame7 tileset_anims_space_8 gTilesetAnims_Unused1_Frame0 gTilesetAnims_Unused1_Frame1 gTilesetAnims_Unused1_Frame2 gTilesetAnims_Unused1_Frame3 gTilesetAnims_Sootopolis_StormyWater gTilesetAnims_BattlePyramid_Torch_Frame0 gTilesetAnims_BattlePyramid_Torch_Frame1 gTilesetAnims_BattlePyramid_Torch_Frame2 tileset_anims_space_9 gTilesetAnims_BattlePyramid_StatueShadow_Frame0 gTilesetAnims_BattlePyramid_StatueShadow_Frame1 gTilesetAnims_BattlePyramid_StatueShadow_Frame2 tileset_anims_space_10 gTilesetAnims_Unused2_Frame0 tileset_anims_space_11 gTilesetAnims_Unused2_Frame1 gTilesetAnims_BattlePyramid_Torch gTilesetAnims_BattlePyramid_StatueShadow sTilesetAnims_BattleDomeFloorLightPals
#[allow(unused_imports)]
use crate::data::tileset_anims::*;

pub(crate) static mut sTilesetDMA3TransferBuffer: crate::ffi::Align4<[u8; 240]> =
    crate::ffi::Align4([0; 240]);
pub(crate) static mut sTilesetDMA3TransferBufferSize: u8 = 0u8;
pub(crate) static mut sPrimaryTilesetAnimCounter: u16 = 0u16;
pub(crate) static mut sPrimaryTilesetAnimCounterMax: u16 = 0u16;
pub(crate) static mut sSecondaryTilesetAnimCounter: u16 = 0u16;
pub(crate) static mut sSecondaryTilesetAnimCounterMax: u16 = 0u16;
pub(crate) static mut sPrimaryTilesetAnimCallback: Option<unsafe extern "C" fn(u16)> = None;
pub(crate) static mut sSecondaryTilesetAnimCallback: Option<unsafe extern "C" fn(u16)> = None;

unsafe extern "C" {
    static mut gMapHeader: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferUnfaded: u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Task_BattleTransition_Intro(a0: u8);
}

pub(crate) unsafe extern "C" fn ResetTilesetAnimBuffer() {
    unsafe {
        ((&raw mut sTilesetDMA3TransferBufferSize)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((&raw mut sTilesetDMA3TransferBuffer).cast::<u8>()).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        240u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AppendTilesetAnimToBuffer(
    src: *mut u16,
    dest: *mut u16,
    size: u16,
) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        let mut size = size;
        if ((((&raw mut sTilesetDMA3TransferBufferSize)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            < 20i32
        {
            (((((&raw mut sTilesetDMA3TransferBuffer).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sTilesetDMA3TransferBufferSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 12,
                ))
            .cast::<*mut u16>())
            .write(src);
            (((((&raw mut sTilesetDMA3TransferBuffer).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sTilesetDMA3TransferBufferSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 12,
                ))
            .wrapping_add(4)
            .cast::<*mut u16>())
            .write(dest);
            (((((&raw mut sTilesetDMA3TransferBuffer).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sTilesetDMA3TransferBufferSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 12,
                ))
            .wrapping_add(8)
            .cast::<u16>())
            .write(size);
            let __p1 = (&raw mut sTilesetDMA3TransferBufferSize)
                .cast::<u8>()
                .cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TransferTilesetAnimsBuffer() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sTilesetDMA3TransferBufferSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            (((((((&raw mut sTilesetDMA3TransferBuffer)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 12))
                                            .cast::<*mut u16>())
                                            .read())
                                                as usize
                                                as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            (((((((&raw mut sTilesetDMA3TransferBuffer)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 12))
                                            .wrapping_add(4)
                                            .cast::<*mut u16>())
                                            .read())
                                                as usize
                                                as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2147483648i32)
                                                | crate::c::div_i32(
                                                    (((((((&raw mut sTilesetDMA3TransferBuffer)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize * 12))
                                                    .wrapping_add(8)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32),
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sTilesetDMA3TransferBufferSize)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnimations() {
    unsafe {
        ResetTilesetAnimBuffer();
        _InitPrimaryTilesetAnimation();
        _InitSecondaryTilesetAnimation();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSecondaryTilesetAnimation() {
    unsafe {
        _InitSecondaryTilesetAnimation();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTilesetAnimations() {
    unsafe {
        ResetTilesetAnimBuffer();
        if (({
            let __p1 = (&raw mut sPrimaryTilesetAnimCounter)
                .cast::<u8>()
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= ((((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
        {
            ((&raw mut sPrimaryTilesetAnimCounter)
                .cast::<u8>()
                .cast::<u16>())
            .write(0u16);
        }
        if (({
            let __p3 = (&raw mut sSecondaryTilesetAnimCounter)
                .cast::<u8>()
                .cast::<u16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            >= ((((&raw mut sSecondaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
        {
            ((&raw mut sSecondaryTilesetAnimCounter)
                .cast::<u8>()
                .cast::<u16>())
            .write(0u16);
        }
        if (((&raw mut sPrimaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .read())
        .is_some()
        {
            (((&raw mut sPrimaryTilesetAnimCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u16)>>())
            .read())
            .unwrap_unchecked()(
                ((&raw mut sPrimaryTilesetAnimCounter)
                    .cast::<u8>()
                    .cast::<u16>())
                .read(),
            );
        }
        if (((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .read())
        .is_some()
        {
            (((&raw mut sSecondaryTilesetAnimCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u16)>>())
            .read())
            .unwrap_unchecked()(
                ((&raw mut sSecondaryTilesetAnimCounter)
                    .cast::<u8>()
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn _InitPrimaryTilesetAnimation() {
    unsafe {
        ((&raw mut sPrimaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sPrimaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sPrimaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
        if (!((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .read())
        .is_null())
            && (((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .is_some())
        {
            ((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe extern "C" fn _InitSecondaryTilesetAnimation() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
        if (!((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .is_null())
            && (((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .is_some())
        {
            ((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_General() {
    unsafe {
        ((&raw mut sPrimaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sPrimaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(256u16);
        ((&raw mut sPrimaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_General));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Building() {
    unsafe {
        ((&raw mut sPrimaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sPrimaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(256u16);
        ((&raw mut sPrimaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Building));
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_General(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 0i32 {
            QueueAnimTiles_General_Flower(((crate::c::div_i32(((timer) as i32), 16i32)) as u16));
        }
        if crate::c::rem_i32(((timer) as i32), 16i32) == 1i32 {
            QueueAnimTiles_General_Water(((crate::c::div_i32(((timer) as i32), 16i32)) as u16));
        }
        if crate::c::rem_i32(((timer) as i32), 16i32) == 2i32 {
            QueueAnimTiles_General_SandWaterEdge(
                ((crate::c::div_i32(((timer) as i32), 16i32)) as u16),
            );
        }
        if crate::c::rem_i32(((timer) as i32), 16i32) == 3i32 {
            QueueAnimTiles_General_Waterfall(((crate::c::div_i32(((timer) as i32), 16i32)) as u16));
        }
        if crate::c::rem_i32(((timer) as i32), 16i32) == 4i32 {
            QueueAnimTiles_General_LandWaterEdge(
                ((crate::c::div_i32(((timer) as i32), 16i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Building(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_Building_TVTurnedOn(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_General_Flower(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_General_Flower)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((508i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_General_Water(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u8 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(32u32, 4u32))) as u8);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_General_Water)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((432i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((30i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_General_SandWaterEdge(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(32u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_General_SandWaterEdge)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((464i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((10i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_General_Waterfall(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_General_Waterfall)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((496i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((6i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Petalburg() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Rustboro() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Rustboro));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Dewford() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Dewford));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Slateport() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Slateport));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Mauville() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounter)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Mauville));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Lavaridge() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Lavaridge));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Fallarbor() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Fortree() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Lilycove() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Mossdeep() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_EverGrande() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_EverGrande));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Pacifidlog() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounter)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Pacifidlog));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Sootopolis() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Sootopolis));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_BattleFrontierOutsideWest() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_BattleFrontierOutsideWest));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_BattleFrontierOutsideEast() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_BattleFrontierOutsideEast));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Underwater() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(128u16);
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Underwater));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_SootopolisGym() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(240u16);
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_SootopolisGym));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_Cave() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_Cave));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_EliteFour() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(128u16);
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_EliteFour));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_MauvilleGym() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_MauvilleGym));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_BikeShop() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_BikeShop));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_BattlePyramid() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_BattlePyramid));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTilesetAnim_BattleDome() {
    unsafe {
        ((&raw mut sSecondaryTilesetAnimCounter)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sSecondaryTilesetAnimCounterMax)
            .cast::<u8>()
            .cast::<u16>())
        .write(
            ((&raw mut sPrimaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        ((&raw mut sSecondaryTilesetAnimCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(Some(TilesetAnim_BattleDome));
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Rustboro(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                0u8,
            );
            QueueAnimTiles_Rustboro_Fountain(((crate::c::div_i32(((timer) as i32), 8i32)) as u16));
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 1i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                1u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 2i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                2u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 3i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                3u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 4i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                4u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 5i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                5u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 6i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                6u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 7i32 {
            QueueAnimTiles_Rustboro_WindyWater(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                7u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Dewford(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_Dewford_Flag(((crate::c::div_i32(((timer) as i32), 8i32)) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Slateport(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 0i32 {
            QueueAnimTiles_Slateport_Balloons(
                ((crate::c::div_i32(((timer) as i32), 16i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Mauville(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                0u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 1i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                1u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 2i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                2u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 3i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                3u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 4i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                4u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 5i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                5u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 6i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                6u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 7i32 {
            QueueAnimTiles_Mauville_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                7u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Lavaridge(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 0i32 {
            QueueAnimTiles_Lavaridge_Steam(((crate::c::div_i32(((timer) as i32), 16i32)) as u8));
        }
        if crate::c::rem_i32(((timer) as i32), 16i32) == 1i32 {
            QueueAnimTiles_Lavaridge_Lava(((crate::c::div_i32(((timer) as i32), 16i32)) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_EverGrande(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                0u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 1i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                1u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 2i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                2u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 3i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                3u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 4i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                4u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 5i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                5u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 6i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                6u8,
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 7i32 {
            QueueAnimTiles_EverGrande_Flowers(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
                7u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Pacifidlog(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 0i32 {
            QueueAnimTiles_Pacifidlog_LogBridges(
                ((crate::c::div_i32(((timer) as i32), 16i32)) as u8),
            );
        }
        if crate::c::rem_i32(((timer) as i32), 16i32) == 1i32 {
            QueueAnimTiles_Pacifidlog_WaterCurrents(
                ((crate::c::div_i32(((timer) as i32), 16i32)) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Sootopolis(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 0i32 {
            QueueAnimTiles_Sootopolis_StormyWater(
                ((crate::c::div_i32(((timer) as i32), 16i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Underwater(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 0i32 {
            QueueAnimTiles_Underwater_Seaweed(((crate::c::div_i32(((timer) as i32), 16i32)) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_Cave(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 16i32) == 1i32 {
            QueueAnimTiles_Cave_Lava(((crate::c::div_i32(((timer) as i32), 16i32)) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_BattleFrontierOutsideWest(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_BattleFrontierOutsideWest_Flag(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_BattleFrontierOutsideEast(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_BattleFrontierOutsideEast_Flag(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_General_LandWaterEdge(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_General_LandWaterEdge)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((480i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((10i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Lavaridge_Steam(timer: u8) {
    unsafe {
        let mut timer = timer;
        let mut i: u8 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u8);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Lavaridge_Steam)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((800i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
        i = ((crate::c::rem_i32(
            ((timer) as i32).wrapping_add(2i32),
            ((crate::c::div_u32(16u32, 4u32)) as i32),
        )) as u8);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Lavaridge_Steam)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((804i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Pacifidlog_LogBridges(timer: u8) {
    unsafe {
        let mut timer = timer;
        let mut i: u8 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u8);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Pacifidlog_LogBridges)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((976i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((30i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Underwater_Seaweed(timer: u8) {
    unsafe {
        let mut timer = timer;
        let mut i: u8 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u8);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Underwater_Seaweed)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((1008i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Pacifidlog_WaterCurrents(timer: u8) {
    unsafe {
        let mut timer = timer;
        let mut i: u8 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(32u32, 4u32))) as u8);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Pacifidlog_WaterCurrents)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((1008i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((8i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Mauville_Flowers(timer_div: u16, timer_mod: u8) {
    unsafe {
        let mut timer_div = timer_div;
        let mut timer_mod = timer_mod;
        timer_div = ((((timer_div) as i32).wrapping_sub(((timer_mod) as i32))) as u16);
        if ((timer_div) as u32)
            < (if crate::c::div_u32(48u32, 4u32) < crate::c::div_u32(48u32, 4u32) {
                crate::c::div_u32(48u32, 4u32)
            } else {
                crate::c::div_u32(48u32, 4u32)
            })
        {
            timer_div = ((crate::c::rem_u32(
                ((timer_div) as u32),
                (if crate::c::div_u32(48u32, 4u32) < crate::c::div_u32(48u32, 4u32) {
                    crate::c::div_u32(48u32, 4u32)
                } else {
                    crate::c::div_u32(48u32, 4u32)
                }),
            )) as u16);
            AppendTilesetAnimToBuffer(
                ((((&raw const gTilesetAnims_Mauville_Flower1)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_div) as i32) as isize))
                .read(),
                ((((&raw const gTilesetAnims_Mauville_Flower1_VDests)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_mod) as i32) as isize))
                .read(),
                (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
            );
            AppendTilesetAnimToBuffer(
                ((((&raw const gTilesetAnims_Mauville_Flower2)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_div) as i32) as isize))
                .read(),
                ((((&raw const gTilesetAnims_Mauville_Flower2_VDests)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_mod) as i32) as isize))
                .read(),
                (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
            );
        } else {
            timer_div = ((crate::c::rem_u32(
                ((timer_div) as u32),
                (if crate::c::div_u32(16u32, 4u32) < crate::c::div_u32(16u32, 4u32) {
                    crate::c::div_u32(16u32, 4u32)
                } else {
                    crate::c::div_u32(16u32, 4u32)
                }),
            )) as u16);
            AppendTilesetAnimToBuffer(
                ((((&raw const gTilesetAnims_Mauville_Flower1_B)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_div) as i32) as isize))
                .read(),
                ((((&raw const gTilesetAnims_Mauville_Flower1_VDests)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_mod) as i32) as isize))
                .read(),
                (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
            );
            AppendTilesetAnimToBuffer(
                ((((&raw const gTilesetAnims_Mauville_Flower2_B)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_div) as i32) as isize))
                .read(),
                ((((&raw const gTilesetAnims_Mauville_Flower2_VDests)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_mod) as i32) as isize))
                .read(),
                (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Rustboro_WindyWater(timer_div: u16, timer_mod: u8) {
    unsafe {
        let mut timer_div = timer_div;
        let mut timer_mod = timer_mod;
        timer_div = ((((timer_div) as i32).wrapping_sub(((timer_mod) as i32))) as u16);
        timer_div =
            ((crate::c::rem_u32(((timer_div) as u32), crate::c::div_u32(32u32, 4u32))) as u16);
        if !(((((&raw const gTilesetAnims_Rustboro_WindyWater)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u16>())
        .cast::<*mut u16>())
        .wrapping_offset(((timer_div) as i32) as isize))
        .read())
        .is_null()
        {
            AppendTilesetAnimToBuffer(
                ((((&raw const gTilesetAnims_Rustboro_WindyWater)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_div) as i32) as isize))
                .read(),
                ((((&raw const gTilesetAnims_Rustboro_WindyWater_VDests)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((timer_mod) as i32) as isize))
                .read(),
                (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Rustboro_Fountain(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(8u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Rustboro_Fountain)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((960i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Lavaridge_Lava(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Lavaridge_Cave_Lava)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((672i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_EverGrande_Flowers(timer_div: u16, timer_mod: u8) {
    unsafe {
        let mut timer_div = timer_div;
        let mut timer_mod = timer_mod;
        timer_div = ((((timer_div) as i32).wrapping_sub(((timer_mod) as i32))) as u16);
        timer_div =
            ((crate::c::rem_u32(((timer_div) as u32), crate::c::div_u32(32u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_EverGrande_Flowers)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((timer_div) as i32) as isize))
            .read(),
            ((((&raw const gTilesetAnims_EverGrande_VDests)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((timer_mod) as i32) as isize))
            .read(),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Cave_Lava(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Lavaridge_Cave_Lava)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((928i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Dewford_Flag(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Dewford_Flag)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((682i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((6i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_BattleFrontierOutsideWest_Flag(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_BattleFrontierOutsideWest_Flag)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((730i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((6i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_BattleFrontierOutsideEast_Flag(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_BattleFrontierOutsideEast_Flag)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((730i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((6i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Slateport_Balloons(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Slateport_Balloons)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((736i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_MauvilleGym(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 2i32) == 0i32 {
            QueueAnimTiles_MauvilleGym_ElectricGates(
                ((crate::c::div_i32(((timer) as i32), 2i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_SootopolisGym(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_SootopolisGym_Waterfalls(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_EliteFour(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 64i32) == 1i32 {
            QueueAnimTiles_EliteFour_GroundLights(
                ((crate::c::div_i32(((timer) as i32), 64i32)) as u16),
            );
        }
        if crate::c::rem_i32(((timer) as i32), 8i32) == 1i32 {
            QueueAnimTiles_EliteFour_WallLights(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_BikeShop(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 4i32) == 0i32 {
            QueueAnimTiles_BikeShop_BlinkingLights(
                ((crate::c::div_i32(((timer) as i32), 4i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_BattlePyramid(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 8i32) == 0i32 {
            QueueAnimTiles_BattlePyramid_Torch(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
            QueueAnimTiles_BattlePyramid_StatueShadow(
                ((crate::c::div_i32(((timer) as i32), 8i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_BattleDome(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 4i32) == 0i32 {
            BlendAnimPalette_BattleDome_FloorLights(
                ((crate::c::div_i32(((timer) as i32), 4i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TilesetAnim_BattleDome2(timer: u16) {
    unsafe {
        let mut timer = timer;
        if crate::c::rem_i32(((timer) as i32), 4i32) == 0i32 {
            BlendAnimPalette_BattleDome_FloorLightsNoBlend(
                ((crate::c::div_i32(((timer) as i32), 4i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Building_TVTurnedOn(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(8u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Building_TvTurnedOn)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((496i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_SootopolisGym_Waterfalls(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 = ((crate::c::rem_u32(
            ((timer) as u32),
            (if crate::c::div_u32(12u32, 4u32) < crate::c::div_u32(12u32, 4u32) {
                crate::c::div_u32(12u32, 4u32)
            } else {
                crate::c::div_u32(12u32, 4u32)
            }),
        )) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_SootopolisGym_SideWaterfall)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((1008i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((12i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_SootopolisGym_FrontWaterfall)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((976i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((20i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_EliteFour_WallLights(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(16u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_EliteFour_WallLights)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((1016i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((1i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_EliteFour_GroundLights(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(8u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_EliteFour_FloorLight)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((992i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((4i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_MauvilleGym_ElectricGates(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(8u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_MauvilleGym_ElectricGates)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((656i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((16i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_BikeShop_BlinkingLights(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(8u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_BikeShop_BlinkingLights)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((1008i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((9i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_Sootopolis_StormyWater(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(32u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_Sootopolis_StormyWater)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((752i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((96i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_BattlePyramid_Torch(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(12u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_BattlePyramid_Torch)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((663i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((8i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn QueueAnimTiles_BattlePyramid_StatueShadow(timer: u16) {
    unsafe {
        let mut timer = timer;
        let mut i: u16 =
            ((crate::c::rem_u32(((timer) as u32), crate::c::div_u32(12u32, 4u32))) as u16);
        AppendTilesetAnimToBuffer(
            ((((&raw const gTilesetAnims_BattlePyramid_StatueShadow)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
            (((100663296i32).wrapping_add((647i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                as usize as *mut u16),
            (((8i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn BlendAnimPalette_BattleDome_FloorLights(timer: u16) {
    unsafe {
        let mut timer = timer;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((((&raw const sTilesetAnims_BattleDomeFloorLightPals)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(
                                ((crate::c::rem_u32(
                                    ((timer) as u32),
                                    crate::c::div_u32(16u32, 4u32),
                                )) as i32) as isize,
                            ))
                            .read())
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(128))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    32u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        BlendPalette(
            128u16,
            16u16,
            ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                6,
                5,
                false,
            ) as u16) as u8),
            ((((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(6),
                0,
                15,
                false,
            ) as u16) as i32)
                & 32767i32) as u16),
        );
        if ((FindTaskIdByFunc(Some(Task_BattleTransition_Intro))) as i32) != 255i32 {
            ((&raw mut sSecondaryTilesetAnimCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u16)>>())
            .write(Some(TilesetAnim_BattleDome2));
            ((&raw mut sSecondaryTilesetAnimCounterMax)
                .cast::<u8>()
                .cast::<u16>())
            .write(32u16);
        }
    }
}
pub(crate) unsafe extern "C" fn BlendAnimPalette_BattleDome_FloorLightsNoBlend(timer: u16) {
    unsafe {
        let mut timer = timer;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((((&raw const sTilesetAnims_BattleDomeFloorLightPals)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u16>())
                            .cast::<*mut u16>())
                            .wrapping_offset(
                                ((crate::c::rem_u32(
                                    ((timer) as u32),
                                    crate::c::div_u32(16u32, 4u32),
                                )) as i32) as isize,
                            ))
                            .read())
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(128))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    32u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((FindTaskIdByFunc(Some(Task_BattleTransition_Intro))) as i32) == 255i32 {
            BlendPalette(
                128u16,
                16u16,
                ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(4),
                    6,
                    5,
                    false,
                ) as u16) as u8),
                ((((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(6),
                    0,
                    15,
                    false,
                ) as u16) as i32)
                    & 32767i32) as u16),
            );
            if !(({
                let __p1 = (&raw mut sSecondaryTilesetAnimCounterMax)
                    .cast::<u8>()
                    .cast::<u16>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) != 0)
            {
                ((&raw mut sSecondaryTilesetAnimCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn(u16)>>())
                .write(None);
            }
        }
    }
}
