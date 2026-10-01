//! Translated from `src/tileset_anims.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations
)]

use crate::battle_transition::Task_BattleTransition_Intro;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::fieldmap::gMapHeader;
use crate::palette::gPaletteFade;
use crate::palette::gPlttBufferUnfaded;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
// Data tables (translate with cdata.py): gTilesetAnims_General_Flower_Frame1 gTilesetAnims_General_Flower_Frame0 gTilesetAnims_General_Flower_Frame2 tileset_anims_space_0 gTilesetAnims_General_Flower gTilesetAnims_General_Water_Frame0 gTilesetAnims_General_Water_Frame1 gTilesetAnims_General_Water_Frame2 gTilesetAnims_General_Water_Frame3 gTilesetAnims_General_Water_Frame4 gTilesetAnims_General_Water_Frame5 gTilesetAnims_General_Water_Frame6 gTilesetAnims_General_Water_Frame7 gTilesetAnims_General_Water gTilesetAnims_General_SandWaterEdge_Frame0 gTilesetAnims_General_SandWaterEdge_Frame1 gTilesetAnims_General_SandWaterEdge_Frame2 gTilesetAnims_General_SandWaterEdge_Frame3 gTilesetAnims_General_SandWaterEdge_Frame4 gTilesetAnims_General_SandWaterEdge_Frame5 gTilesetAnims_General_SandWaterEdge_Frame6 gTilesetAnims_General_SandWaterEdge gTilesetAnims_General_Waterfall_Frame0 gTilesetAnims_General_Waterfall_Frame1 gTilesetAnims_General_Waterfall_Frame2 gTilesetAnims_General_Waterfall_Frame3 gTilesetAnims_General_Waterfall gTilesetAnims_General_LandWaterEdge_Frame0 gTilesetAnims_General_LandWaterEdge_Frame1 gTilesetAnims_General_LandWaterEdge_Frame2 gTilesetAnims_General_LandWaterEdge_Frame3 gTilesetAnims_General_LandWaterEdge gTilesetAnims_Lavaridge_Steam_Frame0 gTilesetAnims_Lavaridge_Steam_Frame1 gTilesetAnims_Lavaridge_Steam_Frame2 gTilesetAnims_Lavaridge_Steam_Frame3 gTilesetAnims_Lavaridge_Steam gTilesetAnims_Pacifidlog_LogBridges_Frame0 gTilesetAnims_Pacifidlog_LogBridges_Frame1 gTilesetAnims_Pacifidlog_LogBridges_Frame2 gTilesetAnims_Pacifidlog_LogBridges gTilesetAnims_Underwater_Seaweed_Frame0 gTilesetAnims_Underwater_Seaweed_Frame1 gTilesetAnims_Underwater_Seaweed_Frame2 gTilesetAnims_Underwater_Seaweed_Frame3 gTilesetAnims_Underwater_Seaweed gTilesetAnims_Pacifidlog_WaterCurrents_Frame0 gTilesetAnims_Pacifidlog_WaterCurrents_Frame1 gTilesetAnims_Pacifidlog_WaterCurrents_Frame2 gTilesetAnims_Pacifidlog_WaterCurrents_Frame3 gTilesetAnims_Pacifidlog_WaterCurrents_Frame4 gTilesetAnims_Pacifidlog_WaterCurrents_Frame5 gTilesetAnims_Pacifidlog_WaterCurrents_Frame6 gTilesetAnims_Pacifidlog_WaterCurrents_Frame7 gTilesetAnims_Pacifidlog_WaterCurrents gTilesetAnims_Mauville_Flower1_Frame0 gTilesetAnims_Mauville_Flower1_Frame1 gTilesetAnims_Mauville_Flower1_Frame2 gTilesetAnims_Mauville_Flower1_Frame3 gTilesetAnims_Mauville_Flower1_Frame4 gTilesetAnims_Mauville_Flower2_Frame0 gTilesetAnims_Mauville_Flower2_Frame1 gTilesetAnims_Mauville_Flower2_Frame2 gTilesetAnims_Mauville_Flower2_Frame3 gTilesetAnims_Mauville_Flower2_Frame4 tileset_anims_space_1 gTilesetAnims_Mauville_Flower1_VDests gTilesetAnims_Mauville_Flower2_VDests gTilesetAnims_Mauville_Flower1 gTilesetAnims_Mauville_Flower2 gTilesetAnims_Mauville_Flower1_B gTilesetAnims_Mauville_Flower2_B gTilesetAnims_Rustboro_WindyWater_Frame0 gTilesetAnims_Rustboro_WindyWater_Frame1 gTilesetAnims_Rustboro_WindyWater_Frame2 gTilesetAnims_Rustboro_WindyWater_Frame3 gTilesetAnims_Rustboro_WindyWater_Frame4 gTilesetAnims_Rustboro_WindyWater_Frame5 gTilesetAnims_Rustboro_WindyWater_Frame6 gTilesetAnims_Rustboro_WindyWater_Frame7 gTilesetAnims_Rustboro_WindyWater_VDests gTilesetAnims_Rustboro_WindyWater gTilesetAnims_Rustboro_Fountain_Frame0 gTilesetAnims_Rustboro_Fountain_Frame1 tileset_anims_space_2 gTilesetAnims_Rustboro_Fountain gTilesetAnims_Lavaridge_Cave_Lava_Frame0 gTilesetAnims_Lavaridge_Cave_Lava_Frame1 gTilesetAnims_Lavaridge_Cave_Lava_Frame2 gTilesetAnims_Lavaridge_Cave_Lava_Frame3 gTilesetAnims_Lavaridge_Cave_Lava_Frame4 gTilesetAnims_Lavaridge_Cave_Lava_Frame5 gTilesetAnims_Lavaridge_Cave_Lava_Frame6 gTilesetAnims_Lavaridge_Cave_Lava_Frame7 tileset_anims_space_3 gTilesetAnims_Lavaridge_Cave_Lava gTilesetAnims_EverGrande_Flowers_Frame0 gTilesetAnims_EverGrande_Flowers_Frame1 gTilesetAnims_EverGrande_Flowers_Frame2 gTilesetAnims_EverGrande_Flowers_Frame3 gTilesetAnims_EverGrande_Flowers_Frame4 gTilesetAnims_EverGrande_Flowers_Frame5 gTilesetAnims_EverGrande_Flowers_Frame6 gTilesetAnims_EverGrande_Flowers_Frame7 tileset_anims_space_4 gTilesetAnims_EverGrande_VDests gTilesetAnims_EverGrande_Flowers gTilesetAnims_Dewford_Flag_Frame0 gTilesetAnims_Dewford_Flag_Frame1 gTilesetAnims_Dewford_Flag_Frame2 gTilesetAnims_Dewford_Flag_Frame3 gTilesetAnims_Dewford_Flag gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame0 gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame1 gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame2 gTilesetAnims_BattleFrontierOutsideWest_Flag_Frame3 gTilesetAnims_BattleFrontierOutsideWest_Flag gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame0 gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame1 gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame2 gTilesetAnims_BattleFrontierOutsideEast_Flag_Frame3 gTilesetAnims_BattleFrontierOutsideEast_Flag gTilesetAnims_Slateport_Balloons_Frame0 gTilesetAnims_Slateport_Balloons_Frame1 gTilesetAnims_Slateport_Balloons_Frame2 gTilesetAnims_Slateport_Balloons_Frame3 gTilesetAnims_Slateport_Balloons gTilesetAnims_Building_TvTurnedOn_Frame0 gTilesetAnims_Building_TvTurnedOn_Frame1 gTilesetAnims_Building_TvTurnedOn gTilesetAnims_SootopolisGym_SideWaterfall_Frame0 gTilesetAnims_SootopolisGym_SideWaterfall_Frame1 gTilesetAnims_SootopolisGym_SideWaterfall_Frame2 gTilesetAnims_SootopolisGym_FrontWaterfall_Frame0 gTilesetAnims_SootopolisGym_FrontWaterfall_Frame1 gTilesetAnims_SootopolisGym_FrontWaterfall_Frame2 gTilesetAnims_SootopolisGym_SideWaterfall gTilesetAnims_SootopolisGym_FrontWaterfall gTilesetAnims_EliteFour_FloorLight_Frame0 gTilesetAnims_EliteFour_FloorLight_Frame1 gTilesetAnims_EliteFour_WallLights_Frame0 gTilesetAnims_EliteFour_WallLights_Frame1 gTilesetAnims_EliteFour_WallLights_Frame2 gTilesetAnims_EliteFour_WallLights_Frame3 tileset_anims_space_5 gTilesetAnims_EliteFour_WallLights gTilesetAnims_EliteFour_FloorLight gTilesetAnims_MauvilleGym_ElectricGates_Frame0 gTilesetAnims_MauvilleGym_ElectricGates_Frame1 tileset_anims_space_6 gTilesetAnims_MauvilleGym_ElectricGates gTilesetAnims_BikeShop_BlinkingLights_Frame0 gTilesetAnims_BikeShop_BlinkingLights_Frame1 tileset_anims_space_7 gTilesetAnims_BikeShop_BlinkingLights gTilesetAnims_Sootopolis_StormyWater_Frame0 gTilesetAnims_Sootopolis_StormyWater_Frame1 gTilesetAnims_Sootopolis_StormyWater_Frame2 gTilesetAnims_Sootopolis_StormyWater_Frame3 gTilesetAnims_Sootopolis_StormyWater_Frame4 gTilesetAnims_Sootopolis_StormyWater_Frame5 gTilesetAnims_Sootopolis_StormyWater_Frame6 gTilesetAnims_Sootopolis_StormyWater_Frame7 tileset_anims_space_8 gTilesetAnims_Unused1_Frame0 gTilesetAnims_Unused1_Frame1 gTilesetAnims_Unused1_Frame2 gTilesetAnims_Unused1_Frame3 gTilesetAnims_Sootopolis_StormyWater gTilesetAnims_BattlePyramid_Torch_Frame0 gTilesetAnims_BattlePyramid_Torch_Frame1 gTilesetAnims_BattlePyramid_Torch_Frame2 tileset_anims_space_9 gTilesetAnims_BattlePyramid_StatueShadow_Frame0 gTilesetAnims_BattlePyramid_StatueShadow_Frame1 gTilesetAnims_BattlePyramid_StatueShadow_Frame2 tileset_anims_space_10 gTilesetAnims_Unused2_Frame0 tileset_anims_space_11 gTilesetAnims_Unused2_Frame1 gTilesetAnims_BattlePyramid_Torch gTilesetAnims_BattlePyramid_StatueShadow sTilesetAnims_BattleDomeFloorLightPals

/// `__typeof__(sTilesetDMA3TransferBuffer[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sTilesetDMA3TransferBuffer_0_t {
    pub src: *mut u16,
    pub dest: *mut u16,
    pub size: u16,
}

unsafe impl Sync for sTilesetDMA3TransferBuffer_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sTilesetDMA3TransferBuffer_0_t>() == 12);
    assert!(offset_of!(sTilesetDMA3TransferBuffer_0_t, src) == 0);
    assert!(offset_of!(sTilesetDMA3TransferBuffer_0_t, dest) == 4);
    assert!(offset_of!(sTilesetDMA3TransferBuffer_0_t, size) == 8);
};

static gTilesetAnims_BattleFrontierOutsideEast_Flag: Table<CArray<*mut u16, 4>> = Table(
    (&raw const crate::data::tileset_anims::gTilesetAnims_BattleFrontierOutsideEast_Flag).cast(),
);
static gTilesetAnims_BattleFrontierOutsideWest_Flag: Table<CArray<*mut u16, 4>> = Table(
    (&raw const crate::data::tileset_anims::gTilesetAnims_BattleFrontierOutsideWest_Flag).cast(),
);
static gTilesetAnims_BattlePyramid_StatueShadow: Table<CArray<*mut u16, 3>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_BattlePyramid_StatueShadow).cast());
static gTilesetAnims_BattlePyramid_Torch: Table<CArray<*mut u16, 3>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_BattlePyramid_Torch).cast());
static gTilesetAnims_BikeShop_BlinkingLights: Table<CArray<*mut u16, 2>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_BikeShop_BlinkingLights).cast());
static gTilesetAnims_Building_TvTurnedOn: Table<CArray<*mut u16, 2>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Building_TvTurnedOn).cast());
static gTilesetAnims_Dewford_Flag: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Dewford_Flag).cast());
static gTilesetAnims_EliteFour_FloorLight: Table<CArray<*mut u16, 2>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_EliteFour_FloorLight).cast());
static gTilesetAnims_EliteFour_WallLights: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_EliteFour_WallLights).cast());
static gTilesetAnims_EverGrande_Flowers: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_EverGrande_Flowers).cast());
static gTilesetAnims_EverGrande_VDests: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_EverGrande_VDests).cast());
static gTilesetAnims_General_Flower: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_General_Flower).cast());
static gTilesetAnims_General_LandWaterEdge: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_General_LandWaterEdge).cast());
static gTilesetAnims_General_SandWaterEdge: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_General_SandWaterEdge).cast());
static gTilesetAnims_General_Water: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_General_Water).cast());
static gTilesetAnims_General_Waterfall: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_General_Waterfall).cast());
static gTilesetAnims_Lavaridge_Cave_Lava: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Lavaridge_Cave_Lava).cast());
static gTilesetAnims_Lavaridge_Steam: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Lavaridge_Steam).cast());
static gTilesetAnims_MauvilleGym_ElectricGates: Table<CArray<*mut u16, 2>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_MauvilleGym_ElectricGates).cast());
static gTilesetAnims_Mauville_Flower1: Table<CArray<*mut u16, 12>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Mauville_Flower1).cast());
static gTilesetAnims_Mauville_Flower1_B: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Mauville_Flower1_B).cast());
static gTilesetAnims_Mauville_Flower1_VDests: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Mauville_Flower1_VDests).cast());
static gTilesetAnims_Mauville_Flower2: Table<CArray<*mut u16, 12>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Mauville_Flower2).cast());
static gTilesetAnims_Mauville_Flower2_B: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Mauville_Flower2_B).cast());
static gTilesetAnims_Mauville_Flower2_VDests: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Mauville_Flower2_VDests).cast());
static gTilesetAnims_Pacifidlog_LogBridges: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Pacifidlog_LogBridges).cast());
static gTilesetAnims_Pacifidlog_WaterCurrents: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Pacifidlog_WaterCurrents).cast());
static gTilesetAnims_Rustboro_Fountain: Table<CArray<*mut u16, 2>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Rustboro_Fountain).cast());
static gTilesetAnims_Rustboro_WindyWater: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Rustboro_WindyWater).cast());
static gTilesetAnims_Rustboro_WindyWater_VDests: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Rustboro_WindyWater_VDests).cast());
static gTilesetAnims_Slateport_Balloons: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Slateport_Balloons).cast());
static gTilesetAnims_SootopolisGym_FrontWaterfall: Table<CArray<*mut u16, 3>> = Table(
    (&raw const crate::data::tileset_anims::gTilesetAnims_SootopolisGym_FrontWaterfall).cast(),
);
static gTilesetAnims_SootopolisGym_SideWaterfall: Table<CArray<*mut u16, 3>> = Table(
    (&raw const crate::data::tileset_anims::gTilesetAnims_SootopolisGym_SideWaterfall).cast(),
);
static gTilesetAnims_Sootopolis_StormyWater: Table<CArray<*mut u16, 8>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Sootopolis_StormyWater).cast());
static gTilesetAnims_Underwater_Seaweed: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::gTilesetAnims_Underwater_Seaweed).cast());
static sTilesetAnims_BattleDomeFloorLightPals: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::tileset_anims::sTilesetAnims_BattleDomeFloorLightPals).cast());

pub(crate) static mut sTilesetDMA3TransferBuffer: CArray<sTilesetDMA3TransferBuffer_0_t, 20> =
    unsafe { zeroed() };
pub(crate) static sTilesetDMA3TransferBufferSize: crate::global::Global<u8> =
    crate::global::Global::new(0);
pub(crate) static sPrimaryTilesetAnimCounter: crate::global::Global<u16> =
    crate::global::Global::new(0);
pub(crate) static sPrimaryTilesetAnimCounterMax: crate::global::Global<u16> =
    crate::global::Global::new(0);
pub(crate) static sSecondaryTilesetAnimCounter: crate::global::Global<u16> =
    crate::global::Global::new(0);
pub(crate) static sSecondaryTilesetAnimCounterMax: crate::global::Global<u16> =
    crate::global::Global::new(0);
pub(crate) static mut sPrimaryTilesetAnimCallback: Option<unsafe fn(u16)> = None;
pub(crate) static mut sSecondaryTilesetAnimCallback: Option<unsafe fn(u16)> = None;

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

unsafe fn ResetTilesetAnimBuffer() {
    sTilesetDMA3TransferBufferSize.set(0);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                sTilesetDMA3TransferBuffer.as_mut_ptr() as *mut c_void,
                0x500003c,
            );
        }
    }
}
unsafe fn AppendTilesetAnimToBuffer(src: *mut u16, dest: *mut u16, size: u16) {
    if sTilesetDMA3TransferBufferSize.get() < 20 {
        sTilesetDMA3TransferBuffer[sTilesetDMA3TransferBufferSize.get()].src = src;
        sTilesetDMA3TransferBuffer[sTilesetDMA3TransferBufferSize.get()].dest = dest;
        sTilesetDMA3TransferBuffer[sTilesetDMA3TransferBufferSize.get()].size = size;
        sTilesetDMA3TransferBufferSize.set(sTilesetDMA3TransferBufferSize.get() + 1);
    }
}
pub unsafe fn TransferTilesetAnimsBuffer() {
    let mut i: i32 = 0;
    while i < sTilesetDMA3TransferBufferSize.get() as i32 {
        {
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, sTilesetDMA3TransferBuffer[i].src as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        sTilesetDMA3TransferBuffer[i].dest as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(2),
                        0x80000000 | (sTilesetDMA3TransferBuffer[i].size as i32 / 2) as u32,
                    );
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
        i += 1;
    }
    sTilesetDMA3TransferBufferSize.set(0);
}
pub unsafe fn InitTilesetAnimations() {
    ResetTilesetAnimBuffer();
    _InitPrimaryTilesetAnimation();
    _InitSecondaryTilesetAnimation();
}
pub unsafe fn InitSecondaryTilesetAnimation() {
    _InitSecondaryTilesetAnimation();
}
pub unsafe fn UpdateTilesetAnimations() {
    ResetTilesetAnimBuffer();
    if ({
        sPrimaryTilesetAnimCounter.set(sPrimaryTilesetAnimCounter.get() + 1);
        sPrimaryTilesetAnimCounter.get()
    }) >= sPrimaryTilesetAnimCounterMax.get()
    {
        sPrimaryTilesetAnimCounter.set(0);
    }
    if ({
        sSecondaryTilesetAnimCounter.set(sSecondaryTilesetAnimCounter.get() + 1);
        sSecondaryTilesetAnimCounter.get()
    }) >= sSecondaryTilesetAnimCounterMax.get()
    {
        sSecondaryTilesetAnimCounter.set(0);
    }
    if sPrimaryTilesetAnimCallback.is_some() {
        sPrimaryTilesetAnimCallback.unwrap_unchecked()(sPrimaryTilesetAnimCounter.get());
    }
    if sSecondaryTilesetAnimCallback.is_some() {
        sSecondaryTilesetAnimCallback.unwrap_unchecked()(sSecondaryTilesetAnimCounter.get());
    }
}
unsafe fn _InitPrimaryTilesetAnimation() {
    sPrimaryTilesetAnimCounter.set(0);
    sPrimaryTilesetAnimCounterMax.set(0);
    sPrimaryTilesetAnimCallback = None;
    if !(*gMapHeader.mapLayout).primaryTileset.is_null()
        && (*(*gMapHeader.mapLayout).primaryTileset).callback.is_some()
    {
        (*(*gMapHeader.mapLayout).primaryTileset)
            .callback
            .unwrap_unchecked()();
    }
}
unsafe fn _InitSecondaryTilesetAnimation() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(0);
    sSecondaryTilesetAnimCallback = None;
    if !(*gMapHeader.mapLayout).secondaryTileset.is_null()
        && (*(*gMapHeader.mapLayout).secondaryTileset)
            .callback
            .is_some()
    {
        (*(*gMapHeader.mapLayout).secondaryTileset)
            .callback
            .unwrap_unchecked()();
    }
}
pub unsafe fn InitTilesetAnim_General() {
    sPrimaryTilesetAnimCounter.set(0);
    sPrimaryTilesetAnimCounterMax.set(256);
    sPrimaryTilesetAnimCallback = Some(TilesetAnim_General);
}
pub unsafe fn InitTilesetAnim_Building() {
    sPrimaryTilesetAnimCounter.set(0);
    sPrimaryTilesetAnimCounterMax.set(256);
    sPrimaryTilesetAnimCallback = Some(TilesetAnim_Building);
}
pub(crate) unsafe fn TilesetAnim_General(timer: u16) {
    if timer as i32 % 16 == 0 {
        QueueAnimTiles_General_Flower((timer as i32 / 16) as u16);
    }
    if timer as i32 % 16 == 1 {
        QueueAnimTiles_General_Water((timer as i32 / 16) as u16);
    }
    if timer as i32 % 16 == 2 {
        QueueAnimTiles_General_SandWaterEdge((timer as i32 / 16) as u16);
    }
    if timer as i32 % 16 == 3 {
        QueueAnimTiles_General_Waterfall((timer as i32 / 16) as u16);
    }
    if timer as i32 % 16 == 4 {
        QueueAnimTiles_General_LandWaterEdge((timer as i32 / 16) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_Building(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_Building_TVTurnedOn((timer as i32 / 8) as u16);
    }
}
unsafe fn QueueAnimTiles_General_Flower(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_General_Flower[i],
        100679552_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_General_Water(timer: u16) {
    let i: u8 = (timer % 8) as u8;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_General_Water[i],
        0x6003600_usize as *mut u16,
        960,
    );
}
unsafe fn QueueAnimTiles_General_SandWaterEdge(timer: u16) {
    let i: u16 = timer % 8;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_General_SandWaterEdge[i],
        0x6003a00_usize as *mut u16,
        320,
    );
}
unsafe fn QueueAnimTiles_General_Waterfall(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_General_Waterfall[i],
        0x6003e00_usize as *mut u16,
        192,
    );
}
pub unsafe fn InitTilesetAnim_Petalburg() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = None;
}
pub unsafe fn InitTilesetAnim_Rustboro() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Rustboro);
}
pub unsafe fn InitTilesetAnim_Dewford() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Dewford);
}
pub unsafe fn InitTilesetAnim_Slateport() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Slateport);
}
pub unsafe fn InitTilesetAnim_Mauville() {
    sSecondaryTilesetAnimCounter.set(sPrimaryTilesetAnimCounter.get());
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Mauville);
}
pub unsafe fn InitTilesetAnim_Lavaridge() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Lavaridge);
}
pub unsafe fn InitTilesetAnim_Fallarbor() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = None;
}
pub unsafe fn InitTilesetAnim_Fortree() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = None;
}
pub unsafe fn InitTilesetAnim_Lilycove() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = None;
}
pub unsafe fn InitTilesetAnim_Mossdeep() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = None;
}
pub unsafe fn InitTilesetAnim_EverGrande() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_EverGrande);
}
pub unsafe fn InitTilesetAnim_Pacifidlog() {
    sSecondaryTilesetAnimCounter.set(sPrimaryTilesetAnimCounter.get());
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Pacifidlog);
}
pub unsafe fn InitTilesetAnim_Sootopolis() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Sootopolis);
}
pub unsafe fn InitTilesetAnim_BattleFrontierOutsideWest() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_BattleFrontierOutsideWest);
}
pub unsafe fn InitTilesetAnim_BattleFrontierOutsideEast() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_BattleFrontierOutsideEast);
}
pub unsafe fn InitTilesetAnim_Underwater() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(128);
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Underwater);
}
pub unsafe fn InitTilesetAnim_SootopolisGym() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(240);
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_SootopolisGym);
}
pub unsafe fn InitTilesetAnim_Cave() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_Cave);
}
pub unsafe fn InitTilesetAnim_EliteFour() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(128);
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_EliteFour);
}
pub unsafe fn InitTilesetAnim_MauvilleGym() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_MauvilleGym);
}
pub unsafe fn InitTilesetAnim_BikeShop() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_BikeShop);
}
pub unsafe fn InitTilesetAnim_BattlePyramid() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_BattlePyramid);
}
pub unsafe fn InitTilesetAnim_BattleDome() {
    sSecondaryTilesetAnimCounter.set(0);
    sSecondaryTilesetAnimCounterMax.set(sPrimaryTilesetAnimCounterMax.get());
    sSecondaryTilesetAnimCallback = Some(TilesetAnim_BattleDome);
}
pub(crate) unsafe fn TilesetAnim_Rustboro(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 0);
        QueueAnimTiles_Rustboro_Fountain((timer as i32 / 8) as u16);
    }
    if timer as i32 % 8 == 1 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 1);
    }
    if timer as i32 % 8 == 2 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 2);
    }
    if timer as i32 % 8 == 3 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 3);
    }
    if timer as i32 % 8 == 4 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 4);
    }
    if timer as i32 % 8 == 5 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 5);
    }
    if timer as i32 % 8 == 6 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 6);
    }
    if timer as i32 % 8 == 7 {
        QueueAnimTiles_Rustboro_WindyWater((timer as i32 / 8) as u16, 7);
    }
}
pub(crate) unsafe fn TilesetAnim_Dewford(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_Dewford_Flag((timer as i32 / 8) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_Slateport(timer: u16) {
    if timer as i32 % 16 == 0 {
        QueueAnimTiles_Slateport_Balloons((timer as i32 / 16) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_Mauville(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 0);
    }
    if timer as i32 % 8 == 1 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 1);
    }
    if timer as i32 % 8 == 2 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 2);
    }
    if timer as i32 % 8 == 3 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 3);
    }
    if timer as i32 % 8 == 4 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 4);
    }
    if timer as i32 % 8 == 5 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 5);
    }
    if timer as i32 % 8 == 6 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 6);
    }
    if timer as i32 % 8 == 7 {
        QueueAnimTiles_Mauville_Flowers((timer as i32 / 8) as u16, 7);
    }
}
pub(crate) unsafe fn TilesetAnim_Lavaridge(timer: u16) {
    if timer as i32 % 16 == 0 {
        QueueAnimTiles_Lavaridge_Steam((timer as i32 / 16) as u8);
    }
    if timer as i32 % 16 == 1 {
        QueueAnimTiles_Lavaridge_Lava((timer as i32 / 16) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_EverGrande(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 0);
    }
    if timer as i32 % 8 == 1 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 1);
    }
    if timer as i32 % 8 == 2 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 2);
    }
    if timer as i32 % 8 == 3 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 3);
    }
    if timer as i32 % 8 == 4 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 4);
    }
    if timer as i32 % 8 == 5 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 5);
    }
    if timer as i32 % 8 == 6 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 6);
    }
    if timer as i32 % 8 == 7 {
        QueueAnimTiles_EverGrande_Flowers((timer as i32 / 8) as u16, 7);
    }
}
pub(crate) unsafe fn TilesetAnim_Pacifidlog(timer: u16) {
    if timer as i32 % 16 == 0 {
        QueueAnimTiles_Pacifidlog_LogBridges((timer as i32 / 16) as u8);
    }
    if timer as i32 % 16 == 1 {
        QueueAnimTiles_Pacifidlog_WaterCurrents((timer as i32 / 16) as u8);
    }
}
pub(crate) unsafe fn TilesetAnim_Sootopolis(timer: u16) {
    if timer as i32 % 16 == 0 {
        QueueAnimTiles_Sootopolis_StormyWater((timer as i32 / 16) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_Underwater(timer: u16) {
    if timer as i32 % 16 == 0 {
        QueueAnimTiles_Underwater_Seaweed((timer as i32 / 16) as u8);
    }
}
pub(crate) unsafe fn TilesetAnim_Cave(timer: u16) {
    if timer as i32 % 16 == 1 {
        QueueAnimTiles_Cave_Lava((timer as i32 / 16) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_BattleFrontierOutsideWest(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_BattleFrontierOutsideWest_Flag((timer as i32 / 8) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_BattleFrontierOutsideEast(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_BattleFrontierOutsideEast_Flag((timer as i32 / 8) as u16);
    }
}
unsafe fn QueueAnimTiles_General_LandWaterEdge(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_General_LandWaterEdge[i],
        0x6003c00_usize as *mut u16,
        320,
    );
}
unsafe fn QueueAnimTiles_Lavaridge_Steam(timer: u8) {
    let mut i: u8 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Lavaridge_Steam[i],
        0x6006400_usize as *mut u16,
        128,
    );
    i = ((timer as i32 + 2) % 4) as u8;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Lavaridge_Steam[i],
        100689024_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_Pacifidlog_LogBridges(timer: u8) {
    let i: u8 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Pacifidlog_LogBridges[i],
        0x6007a00_usize as *mut u16,
        960,
    );
}
unsafe fn QueueAnimTiles_Underwater_Seaweed(timer: u8) {
    let i: u8 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Underwater_Seaweed[i],
        0x6007e00_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_Pacifidlog_WaterCurrents(timer: u8) {
    let i: u8 = timer % 8;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Pacifidlog_WaterCurrents[i],
        0x6007e00_usize as *mut u16,
        256,
    );
}
unsafe fn QueueAnimTiles_Mauville_Flowers(mut timer_div: u16, timer_mod: u8) {
    timer_div -= timer_mod as u16;
    if (timer_div as u32) < (if 12 < 12 { 12 } else { 12 }) {
        timer_div = rem_u32(timer_div as u32, if 12 < 12 { 12 } else { 12 }) as u16;
        AppendTilesetAnimToBuffer(
            gTilesetAnims_Mauville_Flower1[timer_div],
            gTilesetAnims_Mauville_Flower1_VDests[timer_mod],
            128,
        );
        AppendTilesetAnimToBuffer(
            gTilesetAnims_Mauville_Flower2[timer_div],
            gTilesetAnims_Mauville_Flower2_VDests[timer_mod],
            128,
        );
    } else {
        timer_div = rem_u32(timer_div as u32, if 4 < 4 { 4 } else { 4 }) as u16;
        AppendTilesetAnimToBuffer(
            gTilesetAnims_Mauville_Flower1_B[timer_div],
            gTilesetAnims_Mauville_Flower1_VDests[timer_mod],
            128,
        );
        AppendTilesetAnimToBuffer(
            gTilesetAnims_Mauville_Flower2_B[timer_div],
            gTilesetAnims_Mauville_Flower2_VDests[timer_mod],
            128,
        );
    }
}
unsafe fn QueueAnimTiles_Rustboro_WindyWater(mut timer_div: u16, timer_mod: u8) {
    timer_div -= timer_mod as u16;
    timer_div %= 8;
    if !gTilesetAnims_Rustboro_WindyWater[timer_div].is_null() {
        AppendTilesetAnimToBuffer(
            gTilesetAnims_Rustboro_WindyWater[timer_div],
            gTilesetAnims_Rustboro_WindyWater_VDests[timer_mod],
            128,
        );
    }
}
unsafe fn QueueAnimTiles_Rustboro_Fountain(timer: u16) {
    let i: u16 = timer % 2;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Rustboro_Fountain[i],
        0x6007800_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_Lavaridge_Lava(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Lavaridge_Cave_Lava[i],
        0x6005400_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_EverGrande_Flowers(mut timer_div: u16, timer_mod: u8) {
    timer_div -= timer_mod as u16;
    timer_div %= 8;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_EverGrande_Flowers[timer_div],
        gTilesetAnims_EverGrande_VDests[timer_mod],
        128,
    );
}
unsafe fn QueueAnimTiles_Cave_Lava(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Lavaridge_Cave_Lava[i],
        0x6007400_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_Dewford_Flag(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Dewford_Flag[i],
        100685120_usize as *mut u16,
        192,
    );
}
unsafe fn QueueAnimTiles_BattleFrontierOutsideWest_Flag(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_BattleFrontierOutsideWest_Flag[i],
        100686656_usize as *mut u16,
        192,
    );
}
unsafe fn QueueAnimTiles_BattleFrontierOutsideEast_Flag(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_BattleFrontierOutsideEast_Flag[i],
        100686656_usize as *mut u16,
        192,
    );
}
unsafe fn QueueAnimTiles_Slateport_Balloons(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Slateport_Balloons[i],
        0x6005c00_usize as *mut u16,
        128,
    );
}
pub(crate) unsafe fn TilesetAnim_MauvilleGym(timer: u16) {
    if timer as i32 % 2 == 0 {
        QueueAnimTiles_MauvilleGym_ElectricGates((timer as i32 / 2) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_SootopolisGym(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_SootopolisGym_Waterfalls((timer as i32 / 8) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_EliteFour(timer: u16) {
    if timer as i32 % 64 == 1 {
        QueueAnimTiles_EliteFour_GroundLights((timer as i32 / 64) as u16);
    }
    if timer as i32 % 8 == 1 {
        QueueAnimTiles_EliteFour_WallLights((timer as i32 / 8) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_BikeShop(timer: u16) {
    if timer as i32 % 4 == 0 {
        QueueAnimTiles_BikeShop_BlinkingLights((timer as i32 / 4) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_BattlePyramid(timer: u16) {
    if timer as i32 % 8 == 0 {
        QueueAnimTiles_BattlePyramid_Torch((timer as i32 / 8) as u16);
        QueueAnimTiles_BattlePyramid_StatueShadow((timer as i32 / 8) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_BattleDome(timer: u16) {
    if timer as i32 % 4 == 0 {
        BlendAnimPalette_BattleDome_FloorLights((timer as i32 / 4) as u16);
    }
}
pub(crate) unsafe fn TilesetAnim_BattleDome2(timer: u16) {
    if timer as i32 % 4 == 0 {
        BlendAnimPalette_BattleDome_FloorLightsNoBlend((timer as i32 / 4) as u16);
    }
}
unsafe fn QueueAnimTiles_Building_TVTurnedOn(timer: u16) {
    let i: u16 = timer % 2;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Building_TvTurnedOn[i],
        0x6003e00_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_SootopolisGym_Waterfalls(timer: u16) {
    let i: u16 = rem_u32(timer as u32, if 3 < 3 { 3 } else { 3 }) as u16;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_SootopolisGym_SideWaterfall[i],
        0x6007e00_usize as *mut u16,
        384,
    );
    AppendTilesetAnimToBuffer(
        gTilesetAnims_SootopolisGym_FrontWaterfall[i],
        0x6007a00_usize as *mut u16,
        640,
    );
}
unsafe fn QueueAnimTiles_EliteFour_WallLights(timer: u16) {
    let i: u16 = timer % 4;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_EliteFour_WallLights[i],
        0x6007f00_usize as *mut u16,
        32,
    );
}
unsafe fn QueueAnimTiles_EliteFour_GroundLights(timer: u16) {
    let i: u16 = timer % 2;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_EliteFour_FloorLight[i],
        0x6007c00_usize as *mut u16,
        128,
    );
}
unsafe fn QueueAnimTiles_MauvilleGym_ElectricGates(timer: u16) {
    let i: u16 = timer % 2;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_MauvilleGym_ElectricGates[i],
        0x6005200_usize as *mut u16,
        NUM_TILES_IN_PRIMARY,
    );
}
unsafe fn QueueAnimTiles_BikeShop_BlinkingLights(timer: u16) {
    let i: u16 = timer % 2;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_BikeShop_BlinkingLights[i],
        0x6007e00_usize as *mut u16,
        288,
    );
}
unsafe fn QueueAnimTiles_Sootopolis_StormyWater(timer: u16) {
    let i: u16 = timer % 8;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_Sootopolis_StormyWater[i],
        0x6005e00_usize as *mut u16,
        3072,
    );
}
unsafe fn QueueAnimTiles_BattlePyramid_Torch(timer: u16) {
    let i: u16 = timer % 3;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_BattlePyramid_Torch[i],
        100684512_usize as *mut u16,
        256,
    );
}
unsafe fn QueueAnimTiles_BattlePyramid_StatueShadow(timer: u16) {
    let i: u16 = timer % 3;
    AppendTilesetAnimToBuffer(
        gTilesetAnims_BattlePyramid_StatueShadow[i],
        100684000_usize as *mut u16,
        256,
    );
}
unsafe fn BlendAnimPalette_BattleDome_FloorLights(timer: u16) {
    CpuSet(
        sTilesetAnims_BattleDomeFloorLightPals[timer % 4] as *mut c_void,
        &raw mut gPlttBufferUnfaded[128] as *mut c_void,
        16,
    );
    BlendPalette(
        128,
        16,
        gPaletteFade.y() as u8,
        gPaletteFade.blendColor() & 0x7FFF,
    );
    if FindTaskIdByFunc(Some(Task_BattleTransition_Intro)) != TASK_NONE {
        sSecondaryTilesetAnimCallback = Some(TilesetAnim_BattleDome2);
        sSecondaryTilesetAnimCounterMax.set(32);
    }
}
unsafe fn BlendAnimPalette_BattleDome_FloorLightsNoBlend(timer: u16) {
    CpuSet(
        sTilesetAnims_BattleDomeFloorLightPals[timer % 4] as *mut c_void,
        &raw mut gPlttBufferUnfaded[128] as *mut c_void,
        16,
    );
    if FindTaskIdByFunc(Some(Task_BattleTransition_Intro)) == TASK_NONE {
        BlendPalette(
            128,
            16,
            gPaletteFade.y() as u8,
            gPaletteFade.blendColor() & 0x7FFF,
        );
        if ({
            sSecondaryTilesetAnimCounterMax.set(sSecondaryTilesetAnimCounterMax.get() - 1);
            sSecondaryTilesetAnimCounterMax.get()
        }) == 0
        {
            sSecondaryTilesetAnimCallback = None;
        }
    }
}
