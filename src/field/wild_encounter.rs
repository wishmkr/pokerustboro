//! Translated from `src/wild_encounter.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gRoute101_LandMons gRoute101_LandMonsInfo gRoute102_LandMons gRoute102_LandMonsInfo gRoute102_WaterMons gRoute102_WaterMonsInfo gRoute102_FishingMons gRoute102_FishingMonsInfo gRoute103_LandMons gRoute103_LandMonsInfo gRoute103_WaterMons gRoute103_WaterMonsInfo gRoute103_FishingMons gRoute103_FishingMonsInfo gRoute104_LandMons gRoute104_LandMonsInfo gRoute104_WaterMons gRoute104_WaterMonsInfo gRoute104_FishingMons gRoute104_FishingMonsInfo gRoute105_WaterMons gRoute105_WaterMonsInfo gRoute105_FishingMons gRoute105_FishingMonsInfo gRoute110_LandMons gRoute110_LandMonsInfo gRoute110_WaterMons gRoute110_WaterMonsInfo gRoute110_FishingMons gRoute110_FishingMonsInfo gRoute111_LandMons gRoute111_LandMonsInfo gRoute111_WaterMons gRoute111_WaterMonsInfo gRoute111_RockSmashMons gRoute111_RockSmashMonsInfo gRoute111_FishingMons gRoute111_FishingMonsInfo gRoute112_LandMons gRoute112_LandMonsInfo gRoute113_LandMons gRoute113_LandMonsInfo gRoute114_LandMons gRoute114_LandMonsInfo gRoute114_WaterMons gRoute114_WaterMonsInfo gRoute114_RockSmashMons gRoute114_RockSmashMonsInfo gRoute114_FishingMons gRoute114_FishingMonsInfo gRoute116_LandMons gRoute116_LandMonsInfo gRoute117_LandMons gRoute117_LandMonsInfo gRoute117_WaterMons gRoute117_WaterMonsInfo gRoute117_FishingMons gRoute117_FishingMonsInfo gRoute118_LandMons gRoute118_LandMonsInfo gRoute118_WaterMons gRoute118_WaterMonsInfo gRoute118_FishingMons gRoute118_FishingMonsInfo gRoute124_WaterMons gRoute124_WaterMonsInfo gRoute124_FishingMons gRoute124_FishingMonsInfo gPetalburgWoods_LandMons gPetalburgWoods_LandMonsInfo gRusturfTunnel_LandMons gRusturfTunnel_LandMonsInfo gGraniteCave_1F_LandMons gGraniteCave_1F_LandMonsInfo gGraniteCave_B1F_LandMons gGraniteCave_B1F_LandMonsInfo gMtPyre_1F_LandMons gMtPyre_1F_LandMonsInfo gVictoryRoad_1F_LandMons gVictoryRoad_1F_LandMonsInfo gSafariZone_South_LandMons gSafariZone_South_LandMonsInfo gUnderwater_Route126_WaterMons gUnderwater_Route126_WaterMonsInfo gAbandonedShip_Rooms_B1F_WaterMons gAbandonedShip_Rooms_B1F_WaterMonsInfo gAbandonedShip_Rooms_B1F_FishingMons gAbandonedShip_Rooms_B1F_FishingMonsInfo gGraniteCave_B2F_LandMons gGraniteCave_B2F_LandMonsInfo gGraniteCave_B2F_RockSmashMons gGraniteCave_B2F_RockSmashMonsInfo gFieryPath_LandMons gFieryPath_LandMonsInfo gMeteorFalls_B1F_2R_LandMons gMeteorFalls_B1F_2R_LandMonsInfo gMeteorFalls_B1F_2R_WaterMons gMeteorFalls_B1F_2R_WaterMonsInfo gMeteorFalls_B1F_2R_FishingMons gMeteorFalls_B1F_2R_FishingMonsInfo gJaggedPass_LandMons gJaggedPass_LandMonsInfo gRoute106_WaterMons gRoute106_WaterMonsInfo gRoute106_FishingMons gRoute106_FishingMonsInfo gRoute107_WaterMons gRoute107_WaterMonsInfo gRoute107_FishingMons gRoute107_FishingMonsInfo gRoute108_WaterMons gRoute108_WaterMonsInfo gRoute108_FishingMons gRoute108_FishingMonsInfo gRoute109_WaterMons gRoute109_WaterMonsInfo gRoute109_FishingMons gRoute109_FishingMonsInfo gRoute115_LandMons gRoute115_LandMonsInfo gRoute115_WaterMons gRoute115_WaterMonsInfo gRoute115_FishingMons gRoute115_FishingMonsInfo gNewMauville_Inside_LandMons gNewMauville_Inside_LandMonsInfo gRoute119_LandMons gRoute119_LandMonsInfo gRoute119_WaterMons gRoute119_WaterMonsInfo gRoute119_FishingMons gRoute119_FishingMonsInfo gRoute120_LandMons gRoute120_LandMonsInfo gRoute120_WaterMons gRoute120_WaterMonsInfo gRoute120_FishingMons gRoute120_FishingMonsInfo gRoute121_LandMons gRoute121_LandMonsInfo gRoute121_WaterMons gRoute121_WaterMonsInfo gRoute121_FishingMons gRoute121_FishingMonsInfo gRoute122_WaterMons gRoute122_WaterMonsInfo gRoute122_FishingMons gRoute122_FishingMonsInfo gRoute123_LandMons gRoute123_LandMonsInfo gRoute123_WaterMons gRoute123_WaterMonsInfo gRoute123_FishingMons gRoute123_FishingMonsInfo gMtPyre_2F_LandMons gMtPyre_2F_LandMonsInfo gMtPyre_3F_LandMons gMtPyre_3F_LandMonsInfo gMtPyre_4F_LandMons gMtPyre_4F_LandMonsInfo gMtPyre_5F_LandMons gMtPyre_5F_LandMonsInfo gMtPyre_6F_LandMons gMtPyre_6F_LandMonsInfo gMtPyre_Exterior_LandMons gMtPyre_Exterior_LandMonsInfo gMtPyre_Summit_LandMons gMtPyre_Summit_LandMonsInfo gGraniteCave_StevensRoom_LandMons gGraniteCave_StevensRoom_LandMonsInfo gRoute125_WaterMons gRoute125_WaterMonsInfo gRoute125_FishingMons gRoute125_FishingMonsInfo gRoute126_WaterMons gRoute126_WaterMonsInfo gRoute126_FishingMons gRoute126_FishingMonsInfo gRoute127_WaterMons gRoute127_WaterMonsInfo gRoute127_FishingMons gRoute127_FishingMonsInfo gRoute128_WaterMons gRoute128_WaterMonsInfo gRoute128_FishingMons gRoute128_FishingMonsInfo gRoute129_WaterMons gRoute129_WaterMonsInfo gRoute129_FishingMons gRoute129_FishingMonsInfo gRoute130_LandMons gRoute130_LandMonsInfo gRoute130_WaterMons gRoute130_WaterMonsInfo gRoute130_FishingMons gRoute130_FishingMonsInfo gRoute131_WaterMons gRoute131_WaterMonsInfo gRoute131_FishingMons gRoute131_FishingMonsInfo gRoute132_WaterMons gRoute132_WaterMonsInfo gRoute132_FishingMons gRoute132_FishingMonsInfo gRoute133_WaterMons gRoute133_WaterMonsInfo gRoute133_FishingMons gRoute133_FishingMonsInfo gRoute134_WaterMons gRoute134_WaterMonsInfo gRoute134_FishingMons gRoute134_FishingMonsInfo gAbandonedShip_HiddenFloorCorridors_WaterMons gAbandonedShip_HiddenFloorCorridors_WaterMonsInfo gAbandonedShip_HiddenFloorCorridors_FishingMons gAbandonedShip_HiddenFloorCorridors_FishingMonsInfo gSeafloorCavern_Room1_LandMons gSeafloorCavern_Room1_LandMonsInfo gSeafloorCavern_Room2_LandMons gSeafloorCavern_Room2_LandMonsInfo gSeafloorCavern_Room3_LandMons gSeafloorCavern_Room3_LandMonsInfo gSeafloorCavern_Room4_LandMons gSeafloorCavern_Room4_LandMonsInfo gSeafloorCavern_Room5_LandMons gSeafloorCavern_Room5_LandMonsInfo gSeafloorCavern_Room6_LandMons gSeafloorCavern_Room6_LandMonsInfo gSeafloorCavern_Room6_WaterMons gSeafloorCavern_Room6_WaterMonsInfo gSeafloorCavern_Room6_FishingMons gSeafloorCavern_Room6_FishingMonsInfo gSeafloorCavern_Room7_LandMons gSeafloorCavern_Room7_LandMonsInfo gSeafloorCavern_Room7_WaterMons gSeafloorCavern_Room7_WaterMonsInfo gSeafloorCavern_Room7_FishingMons gSeafloorCavern_Room7_FishingMonsInfo gSeafloorCavern_Room8_LandMons gSeafloorCavern_Room8_LandMonsInfo gSeafloorCavern_Entrance_WaterMons gSeafloorCavern_Entrance_WaterMonsInfo gSeafloorCavern_Entrance_FishingMons gSeafloorCavern_Entrance_FishingMonsInfo gCaveOfOrigin_Entrance_LandMons gCaveOfOrigin_Entrance_LandMonsInfo gCaveOfOrigin_1F_LandMons gCaveOfOrigin_1F_LandMonsInfo gCaveOfOrigin_UnusedRubySapphireMap1_LandMons gCaveOfOrigin_UnusedRubySapphireMap1_LandMonsInfo gCaveOfOrigin_UnusedRubySapphireMap2_LandMons gCaveOfOrigin_UnusedRubySapphireMap2_LandMonsInfo gCaveOfOrigin_UnusedRubySapphireMap3_LandMons gCaveOfOrigin_UnusedRubySapphireMap3_LandMonsInfo gNewMauville_Entrance_LandMons gNewMauville_Entrance_LandMonsInfo gSafariZone_Southwest_LandMons gSafariZone_Southwest_LandMonsInfo gSafariZone_Southwest_WaterMons gSafariZone_Southwest_WaterMonsInfo gSafariZone_Southwest_FishingMons gSafariZone_Southwest_FishingMonsInfo gSafariZone_North_LandMons gSafariZone_North_LandMonsInfo gSafariZone_North_RockSmashMons gSafariZone_North_RockSmashMonsInfo gSafariZone_Northwest_LandMons gSafariZone_Northwest_LandMonsInfo gSafariZone_Northwest_WaterMons gSafariZone_Northwest_WaterMonsInfo gSafariZone_Northwest_FishingMons gSafariZone_Northwest_FishingMonsInfo gVictoryRoad_B1F_LandMons gVictoryRoad_B1F_LandMonsInfo gVictoryRoad_B1F_RockSmashMons gVictoryRoad_B1F_RockSmashMonsInfo gVictoryRoad_B2F_LandMons gVictoryRoad_B2F_LandMonsInfo gVictoryRoad_B2F_WaterMons gVictoryRoad_B2F_WaterMonsInfo gVictoryRoad_B2F_FishingMons gVictoryRoad_B2F_FishingMonsInfo gMeteorFalls_1F_1R_LandMons gMeteorFalls_1F_1R_LandMonsInfo gMeteorFalls_1F_1R_WaterMons gMeteorFalls_1F_1R_WaterMonsInfo gMeteorFalls_1F_1R_FishingMons gMeteorFalls_1F_1R_FishingMonsInfo gMeteorFalls_1F_2R_LandMons gMeteorFalls_1F_2R_LandMonsInfo gMeteorFalls_1F_2R_WaterMons gMeteorFalls_1F_2R_WaterMonsInfo gMeteorFalls_1F_2R_FishingMons gMeteorFalls_1F_2R_FishingMonsInfo gMeteorFalls_B1F_1R_LandMons gMeteorFalls_B1F_1R_LandMonsInfo gMeteorFalls_B1F_1R_WaterMons gMeteorFalls_B1F_1R_WaterMonsInfo gMeteorFalls_B1F_1R_FishingMons gMeteorFalls_B1F_1R_FishingMonsInfo gShoalCave_LowTideStairsRoom_LandMons gShoalCave_LowTideStairsRoom_LandMonsInfo gShoalCave_LowTideLowerRoom_LandMons gShoalCave_LowTideLowerRoom_LandMonsInfo gShoalCave_LowTideInnerRoom_LandMons gShoalCave_LowTideInnerRoom_LandMonsInfo gShoalCave_LowTideInnerRoom_WaterMons gShoalCave_LowTideInnerRoom_WaterMonsInfo gShoalCave_LowTideInnerRoom_FishingMons gShoalCave_LowTideInnerRoom_FishingMonsInfo gShoalCave_LowTideEntranceRoom_LandMons gShoalCave_LowTideEntranceRoom_LandMonsInfo gShoalCave_LowTideEntranceRoom_WaterMons gShoalCave_LowTideEntranceRoom_WaterMonsInfo gShoalCave_LowTideEntranceRoom_FishingMons gShoalCave_LowTideEntranceRoom_FishingMonsInfo gLilycoveCity_WaterMons gLilycoveCity_WaterMonsInfo gLilycoveCity_FishingMons gLilycoveCity_FishingMonsInfo gDewfordTown_WaterMons gDewfordTown_WaterMonsInfo gDewfordTown_FishingMons gDewfordTown_FishingMonsInfo gSlateportCity_WaterMons gSlateportCity_WaterMonsInfo gSlateportCity_FishingMons gSlateportCity_FishingMonsInfo gMossdeepCity_WaterMons gMossdeepCity_WaterMonsInfo gMossdeepCity_FishingMons gMossdeepCity_FishingMonsInfo gPacifidlogTown_WaterMons gPacifidlogTown_WaterMonsInfo gPacifidlogTown_FishingMons gPacifidlogTown_FishingMonsInfo gEverGrandeCity_WaterMons gEverGrandeCity_WaterMonsInfo gEverGrandeCity_FishingMons gEverGrandeCity_FishingMonsInfo gPetalburgCity_WaterMons gPetalburgCity_WaterMonsInfo gPetalburgCity_FishingMons gPetalburgCity_FishingMonsInfo gUnderwater_Route124_WaterMons gUnderwater_Route124_WaterMonsInfo gShoalCave_LowTideIceRoom_LandMons gShoalCave_LowTideIceRoom_LandMonsInfo gSkyPillar_1F_LandMons gSkyPillar_1F_LandMonsInfo gSootopolisCity_WaterMons gSootopolisCity_WaterMonsInfo gSootopolisCity_FishingMons gSootopolisCity_FishingMonsInfo gSkyPillar_3F_LandMons gSkyPillar_3F_LandMonsInfo gSkyPillar_5F_LandMons gSkyPillar_5F_LandMonsInfo gSafariZone_Southeast_LandMons gSafariZone_Southeast_LandMonsInfo gSafariZone_Southeast_WaterMons gSafariZone_Southeast_WaterMonsInfo gSafariZone_Southeast_FishingMons gSafariZone_Southeast_FishingMonsInfo gSafariZone_Northeast_LandMons gSafariZone_Northeast_LandMonsInfo gSafariZone_Northeast_RockSmashMons gSafariZone_Northeast_RockSmashMonsInfo gMagmaHideout_1F_LandMons gMagmaHideout_1F_LandMonsInfo gMagmaHideout_2F_1R_LandMons gMagmaHideout_2F_1R_LandMonsInfo gMagmaHideout_2F_2R_LandMons gMagmaHideout_2F_2R_LandMonsInfo gMagmaHideout_3F_1R_LandMons gMagmaHideout_3F_1R_LandMonsInfo gMagmaHideout_3F_2R_LandMons gMagmaHideout_3F_2R_LandMonsInfo gMagmaHideout_4F_LandMons gMagmaHideout_4F_LandMonsInfo gMagmaHideout_3F_3R_LandMons gMagmaHideout_3F_3R_LandMonsInfo gMagmaHideout_2F_3R_LandMons gMagmaHideout_2F_3R_LandMonsInfo gMirageTower_1F_LandMons gMirageTower_1F_LandMonsInfo gMirageTower_2F_LandMons gMirageTower_2F_LandMonsInfo gMirageTower_3F_LandMons gMirageTower_3F_LandMonsInfo gMirageTower_4F_LandMons gMirageTower_4F_LandMonsInfo gDesertUnderpass_LandMons gDesertUnderpass_LandMonsInfo gArtisanCave_B1F_LandMons gArtisanCave_B1F_LandMonsInfo gArtisanCave_1F_LandMons gArtisanCave_1F_LandMonsInfo gAlteringCave1_LandMons gAlteringCave1_LandMonsInfo gAlteringCave2_LandMons gAlteringCave2_LandMonsInfo gAlteringCave3_LandMons gAlteringCave3_LandMonsInfo gAlteringCave4_LandMons gAlteringCave4_LandMonsInfo gAlteringCave5_LandMons gAlteringCave5_LandMonsInfo gAlteringCave6_LandMons gAlteringCave6_LandMonsInfo gAlteringCave7_LandMons gAlteringCave7_LandMonsInfo gAlteringCave8_LandMons gAlteringCave8_LandMonsInfo gAlteringCave9_LandMons gAlteringCave9_LandMonsInfo gMeteorFalls_StevensCave_LandMons gMeteorFalls_StevensCave_LandMonsInfo gWildMonHeaders gBattlePyramid_1_LandMons gBattlePyramid_1_LandMonsInfo gBattlePyramid_2_LandMons gBattlePyramid_2_LandMonsInfo gBattlePyramid_3_LandMons gBattlePyramid_3_LandMonsInfo gBattlePyramid_4_LandMons gBattlePyramid_4_LandMonsInfo gBattlePyramid_5_LandMons gBattlePyramid_5_LandMonsInfo gBattlePyramid_6_LandMons gBattlePyramid_6_LandMonsInfo gBattlePyramid_7_LandMons gBattlePyramid_7_LandMonsInfo gBattlePyramidWildMonHeaders gBattlePike_1_LandMons gBattlePike_1_LandMonsInfo gBattlePike_2_LandMons gBattlePike_2_LandMonsInfo gBattlePike_3_LandMons gBattlePike_3_LandMonsInfo gBattlePike_4_LandMons gBattlePike_4_LandMonsInfo gBattlePikeWildMonHeaders sWildFeebas sRoute119WaterTileData
#[allow(unused_imports)]
use crate::data::wild_encounter::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWildEncountersDisabled: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFeebasRngValue: u32 = 0u32;

unsafe extern "C" {
    static mut EventScript_RepelWoreOff: u8;
    static mut gEnemyParty: u8;
    static mut gMapHeader: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    fn BattleSetup_StartBattlePikeWildBattle();
    fn BattleSetup_StartRoamerBattle();
    fn BattleSetup_StartWildBattle();
    fn CreateMonWithGenderNatureLetter(
        a0: *mut u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CreateMonWithNature(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8);
    fn CurrentBattlePyramidLocation() -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn GenerateBattlePyramidWildMon();
    fn GetBattlePikeWildMonHeaderId() -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetSafariZoneFlag() -> u32;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn InBattlePike() -> u8;
    fn InUnionRoom() -> u32;
    fn IncrementGameStat(a0: u8);
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsBridgeOverWater(a0: u8) -> u8;
    fn MetatileBehavior_IsLandWildEncounter(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableAndNotWaterfall(a0: u8) -> u8;
    fn MetatileBehavior_IsWaterWildEncounter(a0: u8) -> u8;
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PokeblockGetGain(a0: u8, a1: *mut u8) -> i16;
    fn Random() -> u16;
    fn SafariZoneGetActivePokeblock() -> *mut u8;
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn SetPokemonAnglerSpecies(a0: u16);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TryGenerateBattlePikeWildMon(a0: u8) -> u32;
    fn TryStartRoamerEncounter() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableWildEncounters(disabled: u8) {
    unsafe {
        let mut disabled = disabled;
        ((&raw mut sWildEncountersDisabled).cast::<u8>().cast::<u8>()).write(disabled);
    }
}
pub(crate) unsafe extern "C" fn GetFeebasFishingSpotId(
    targetX: i16,
    targetY: i16,
    section: u8,
) -> u16 {
    unsafe {
        let mut targetX = targetX;
        let mut targetY = targetY;
        let mut section = section;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut yMin: u16 = ((((&raw const sRoute119WaterTileData)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((((section) as i32).wrapping_mul(3i32)).wrapping_add(0i32)) as isize))
        .read();
        let mut yMax: u16 = ((((&raw const sRoute119WaterTileData)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((((section) as i32).wrapping_mul(3i32)).wrapping_add(1i32)) as isize))
        .read();
        let mut spotId: u16 = ((((&raw const sRoute119WaterTileData)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((((section) as i32).wrapping_mul(3i32)).wrapping_add(2i32)) as isize))
        .read();
        {
            y = yMin;
            'l1: loop {
                if !(((y) as i32) <= ((yMax) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = 0u16;
                        'l3: loop {
                            if !(((x) as i32)
                                < (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>())
                                    .read())
                                .cast::<i32>())
                                .read())
                            {
                                break 'l3;
                            }
                            'l4: {
                                let mut behavior: u8 = ((MapGridGetMetatileBehaviorAt(
                                    ((x) as i32).wrapping_add(7i32),
                                    ((y) as i32).wrapping_add(7i32),
                                )) as u8);
                                if ((MetatileBehavior_IsSurfableAndNotWaterfall(behavior)) as i32)
                                    == 1i32
                                {
                                    spotId = (spotId).wrapping_add(1);
                                    if (((targetX) as i32) == ((x) as i32))
                                        && (((targetY) as i32) == ((y) as i32))
                                    {
                                        return spotId;
                                    }
                                }
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
        return ((((spotId) as i32).wrapping_add(1i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn CheckFeebas() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut feebasSpots = crate::ffi::Align4([0u8; 12]);
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut route119Section: u8 = 0u8;
        let mut spotId: u16 = 0u16;
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 34i32)
        {
            GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
            x = ((((x) as i32).wrapping_sub(7i32)) as i16);
            y = ((((y) as i32).wrapping_sub(7i32)) as i16);
            if (((y) as i32)
                >= (((((&raw const sRoute119WaterTileData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .read()) as i32))
                && (((y) as i32)
                    <= ((((((&raw const sRoute119WaterTileData)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32))
            {
                route119Section = 0u8;
            }
            if (((y) as i32)
                >= ((((((&raw const sRoute119WaterTileData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(3))
                .read()) as i32))
                && (((y) as i32)
                    <= ((((((&raw const sRoute119WaterTileData)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(4))
                    .read()) as i32))
            {
                route119Section = 1u8;
            }
            if (((y) as i32)
                >= ((((((&raw const sRoute119WaterTileData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(6))
                .read()) as i32))
                && (((y) as i32)
                    <= ((((((&raw const sRoute119WaterTileData)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(7))
                    .read()) as i32))
            {
                route119Section = 2u8;
            }
            if crate::c::rem_i32(((Random()) as i32), 100i32) > 49i32 {
                return 0u8;
            }
            FeebasSeedRng(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11880))
                    .cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
                .read(),
            );
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) != 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut feebasSpots).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(((crate::c::rem_i32(((FeebasRandom()) as i32), 447i32)) as u16));
                        if (((((&raw mut feebasSpots).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 0i32
                        {
                            (((&raw mut feebasSpots).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(447u16);
                        }
                        if ((((((&raw mut feebasSpots).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            < 1i32)
                            || ((((((&raw mut feebasSpots).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                >= 4i32)
                        {
                            i = (i).wrapping_add(1);
                        }
                    }
                }
            }
            spotId = GetFeebasFishingSpotId(x, y, route119Section);
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((spotId) as i32)
                            == (((((&raw mut feebasSpots).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                        {
                            return 1u8;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FeebasRandom() -> u16 {
    unsafe {
        ((&raw mut sFeebasRngValue).cast::<u8>().cast::<u32>()).write(
            ((1103515245u32)
                .wrapping_mul(((&raw mut sFeebasRngValue).cast::<u8>().cast::<u32>()).read()))
            .wrapping_add(12345u32),
        );
        return ((((&raw mut sFeebasRngValue).cast::<u8>().cast::<u32>()).read() >> 16) as u16);
    }
}
pub(crate) unsafe extern "C" fn FeebasSeedRng(seed: u16) {
    unsafe {
        let mut seed = seed;
        ((&raw mut sFeebasRngValue).cast::<u8>().cast::<u32>()).write(((seed) as u32));
    }
}
pub(crate) unsafe extern "C" fn ChooseWildMonIndex_Land() -> u8 {
    unsafe {
        let mut rand: u8 = ((crate::c::rem_i32(((Random()) as i32), 100i32)) as u8);
        if ((rand) as i32) < 20i32 {
            return 0u8;
        } else {
            if (((rand) as i32) >= 20i32) && (((rand) as i32) < 40i32) {
                return 1u8;
            } else {
                if (((rand) as i32) >= 40i32) && (((rand) as i32) < 50i32) {
                    return 2u8;
                } else {
                    if (((rand) as i32) >= 50i32) && (((rand) as i32) < 60i32) {
                        return 3u8;
                    } else {
                        if (((rand) as i32) >= 60i32) && (((rand) as i32) < 70i32) {
                            return 4u8;
                        } else {
                            if (((rand) as i32) >= 70i32) && (((rand) as i32) < 80i32) {
                                return 5u8;
                            } else {
                                if (((rand) as i32) >= 80i32) && (((rand) as i32) < 85i32) {
                                    return 6u8;
                                } else {
                                    if (((rand) as i32) >= 85i32) && (((rand) as i32) < 90i32) {
                                        return 7u8;
                                    } else {
                                        if (((rand) as i32) >= 90i32) && (((rand) as i32) < 94i32) {
                                            return 8u8;
                                        } else {
                                            if (((rand) as i32) >= 94i32)
                                                && (((rand) as i32) < 98i32)
                                            {
                                                return 9u8;
                                            } else {
                                                if (((rand) as i32) >= 98i32)
                                                    && (((rand) as i32) < 99i32)
                                                {
                                                    return 10u8;
                                                } else {
                                                    return 11u8;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseWildMonIndex_WaterRock() -> u8 {
    unsafe {
        let mut rand: u8 = ((crate::c::rem_i32(((Random()) as i32), 100i32)) as u8);
        if ((rand) as i32) < 60i32 {
            return 0u8;
        } else {
            if (((rand) as i32) >= 60i32) && (((rand) as i32) < 90i32) {
                return 1u8;
            } else {
                if (((rand) as i32) >= 90i32) && (((rand) as i32) < 95i32) {
                    return 2u8;
                } else {
                    if (((rand) as i32) >= 95i32) && (((rand) as i32) < 99i32) {
                        return 3u8;
                    } else {
                        return 4u8;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseWildMonIndex_Fishing(rod: u8) -> u8 {
    unsafe {
        let mut rod = rod;
        let mut wildMonIndex: u8 = 0u8;
        let mut rand: u8 = ((crate::c::rem_i32(
            ((Random()) as i32),
            (if (if 100i32 >= 100i32 { 100i32 } else { 100i32 }) >= 100i32 {
                (if 100i32 >= 100i32 { 100i32 } else { 100i32 })
            } else {
                100i32
            }),
        )) as u8);
        'l1: {
            let __sw1 = ((rod) as i32);
            if __sw1 == 0i32 {
                if ((rand) as i32) < 70i32 {
                    wildMonIndex = 0u8;
                } else {
                    wildMonIndex = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((rand) as i32) < 60i32 {
                    wildMonIndex = 2u8;
                }
                if (((rand) as i32) >= 60i32) && (((rand) as i32) < 80i32) {
                    wildMonIndex = 3u8;
                }
                if (((rand) as i32) >= 80i32) && (((rand) as i32) < 100i32) {
                    wildMonIndex = 4u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((rand) as i32) < 40i32 {
                    wildMonIndex = 5u8;
                }
                if (((rand) as i32) >= 40i32) && (((rand) as i32) < 80i32) {
                    wildMonIndex = 6u8;
                }
                if (((rand) as i32) >= 80i32) && (((rand) as i32) < 95i32) {
                    wildMonIndex = 7u8;
                }
                if (((rand) as i32) >= 95i32) && (((rand) as i32) < 99i32) {
                    wildMonIndex = 8u8;
                }
                if (((rand) as i32) >= 99i32) && (((rand) as i32) < 100i32) {
                    wildMonIndex = 9u8;
                }
                break 'l1;
            }
        }
        return wildMonIndex;
    }
}
pub(crate) unsafe extern "C" fn ChooseWildMonLevel(wildPokemon: *mut u8) -> u8 {
    unsafe {
        let mut wildPokemon = wildPokemon;
        let mut min: u8 = 0u8;
        let mut max: u8 = 0u8;
        let mut range: u8 = 0u8;
        let mut rand: u8 = 0u8;
        if ((((wildPokemon).wrapping_add(1)).read()) as i32) >= (((wildPokemon).read()) as i32) {
            min = (wildPokemon).read();
            max = ((wildPokemon).wrapping_add(1)).read();
        } else {
            min = ((wildPokemon).wrapping_add(1)).read();
            max = (wildPokemon).read();
        }
        range = (((((max) as i32).wrapping_sub(((min) as i32))).wrapping_add(1i32)) as u8);
        rand = ((crate::c::rem_i32(((Random()) as i32), ((range) as i32))) as u8);
        if !((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0) {
            let mut ability: u8 = GetMonAbility((&raw mut gPlayerParty).cast::<u8>());
            if ((((ability) as i32) == 55i32) || (((ability) as i32) == 72i32))
                || (((ability) as i32) == 46i32)
            {
                if crate::c::rem_i32(((Random()) as i32), 2i32) == 0i32 {
                    return max;
                }
                if ((rand) as i32) != 0i32 {
                    rand = (rand).wrapping_sub(1);
                }
            }
        }
        return ((((min) as i32).wrapping_add(((rand) as i32))) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetCurrentMapWildMonHeaderId() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                'l2: {
                    let mut wildHeader: *mut u8 =
                        (((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20);
                    if (((wildHeader).read()) as i32) == 255i32 {
                        break 'l1;
                    }
                    if (((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                    .read()) as i32)
                        == (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<i8>())
                        .read()) as i32))
                        && ((((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                        .wrapping_add(1))
                        .read()) as i32)
                            == (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as i32))
                    {
                        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<i8>())
                        .read()) as i32)
                            == 24i32)
                            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as i32)
                                == 106i32)
                        {
                            let mut alteringCaveId: u16 = VarGet(16446u16);
                            if ((alteringCaveId) as i32) >= 9i32 {
                                alteringCaveId = 0u16;
                            }
                            i = ((((i) as i32).wrapping_add(((alteringCaveId) as i32))) as u16);
                        }
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u16;
    }
}
pub(crate) unsafe extern "C" fn PickWildMonNature() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut safariPokeblock: *mut u8 = core::ptr::null_mut();
        let mut natures = crate::ffi::Align4([0u8; 25]);
        if (GetSafariZoneFlag() == 1u32) && (crate::c::rem_i32(((Random()) as i32), 100i32) < 80i32)
        {
            safariPokeblock = SafariZoneGetActivePokeblock();
            if ((safariPokeblock) as usize) != 0usize {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 25i32) {
                            break 'l1;
                        }
                        'l2: {
                            (((&raw mut natures).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(i);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32) < 24i32) {
                            break 'l3;
                        }
                        'l4: {
                            {
                                j = ((((i) as i32).wrapping_add(1i32)) as u8);
                                'l5: loop {
                                    if !(((j) as i32) < 25i32) {
                                        break 'l5;
                                    }
                                    'l6: {
                                        if (((Random()) as i32) & 1i32) != 0 {
                                            let mut temp: u8 = 0u8;
                                            {
                                                temp = (((&raw mut natures).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize))
                                                .read();
                                                (((&raw mut natures).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize))
                                                .write(
                                                    (((&raw mut natures).cast::<u8>())
                                                        .wrapping_offset(((j) as i32) as isize))
                                                    .read(),
                                                );
                                                (((&raw mut natures).cast::<u8>())
                                                    .wrapping_offset(((j) as i32) as isize))
                                                .write(temp);
                                            }
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l7: loop {
                        if !(((i) as i32) < 25i32) {
                            break 'l7;
                        }
                        'l8: {
                            if ((PokeblockGetGain(
                                (((&raw mut natures).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                safariPokeblock,
                            )) as i32)
                                > 0i32
                            {
                                return (((&raw mut natures).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read();
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        if ((!((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0))
            && (((GetMonAbility((&raw mut gPlayerParty).cast::<u8>())) as i32) == 28i32))
            && (crate::c::rem_i32(((Random()) as i32), 2i32) == 0i32)
        {
            return ((crate::c::rem_u32(
                GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 0i32),
                25u32,
            )) as u8);
        }
        return ((crate::c::rem_i32(((Random()) as i32), 25i32)) as u8);
    }
}
pub(crate) unsafe extern "C" fn CreateWildMon(species: u16, level: u8) {
    unsafe {
        let mut species = species;
        let mut level = level;
        let mut checkCuteCharm: u32 = 0u32;
        ZeroEnemyPartyMons();
        checkCuteCharm = 1u32;
        'l1: {
            let __sw1 = ((((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 254i32 || __sw1 == 255i32 {
                checkCuteCharm = 0u32;
                break 'l1;
            }
        }
        if ((((checkCuteCharm) != 0)
            && (!((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0)))
            && (((GetMonAbility((&raw mut gPlayerParty).cast::<u8>())) as i32) == 56i32))
            && (crate::c::rem_i32(((Random()) as i32), 3i32) != 0i32)
        {
            let mut leadingMonSpecies: u16 =
                ((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 11i32)) as u16);
            let mut leadingMonPersonality: u32 =
                GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 0i32);
            let mut gender: u8 =
                GetGenderFromSpeciesAndPersonality(leadingMonSpecies, leadingMonPersonality);
            if ((gender) as i32) == 254i32 {
                gender = 0u8;
            } else {
                gender = 254u8;
            }
            CreateMonWithGenderNatureLetter(
                (&raw mut gEnemyParty).cast::<u8>(),
                species,
                level,
                32u8,
                gender,
                PickWildMonNature(),
                0u8,
            );
            return;
        }
        CreateMonWithNature(
            (&raw mut gEnemyParty).cast::<u8>(),
            species,
            level,
            32u8,
            PickWildMonNature(),
        );
    }
}
pub(crate) unsafe extern "C" fn TryGenerateWildMon(
    wildMonInfo: *mut u8,
    area: u8,
    flags: u8,
) -> u8 {
    unsafe {
        let mut wildMonInfo = wildMonInfo;
        let mut area = area;
        let mut flags = flags;
        let mut wildMonIndex: u8 = 0u8;
        let mut level: u8 = 0u8;
        'l1: {
            let __sw1 = ((area) as i32);
            if __sw1 == 0i32 {
                if (TryGetAbilityInfluencedWildMonIndex(
                    ((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read(),
                    8u8,
                    42u8,
                    &raw mut wildMonIndex,
                )) != 0
                {
                    break 'l1;
                }
                if (TryGetAbilityInfluencedWildMonIndex(
                    ((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read(),
                    13u8,
                    9u8,
                    &raw mut wildMonIndex,
                )) != 0
                {
                    break 'l1;
                }
                wildMonIndex = ChooseWildMonIndex_Land();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (TryGetAbilityInfluencedWildMonIndex(
                    ((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read(),
                    13u8,
                    9u8,
                    &raw mut wildMonIndex,
                )) != 0
                {
                    break 'l1;
                }
                wildMonIndex = ChooseWildMonIndex_WaterRock();
                break 'l1;
            }
            if __sw1 == 2i32 {
                wildMonIndex = ChooseWildMonIndex_WaterRock();
                break 'l1;
            }
        }
        level = ChooseWildMonLevel(
            (((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_offset(((wildMonIndex) as i32) as isize * 4),
        );
        if ((((flags) as i32) & 1i32) != 0) && (!((IsWildLevelAllowedByRepel(level)) != 0)) {
            return 0u8;
        }
        if (((((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            != 358i32)
            && ((((flags) as i32) & 2i32) != 0))
            && (!((IsAbilityAllowingEncounter(level)) != 0))
        {
            return 0u8;
        }
        CreateWildMon(
            (((((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_offset(((wildMonIndex) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
            level,
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GenerateFishingWildMon(wildMonInfo: *mut u8, rod: u8) -> u16 {
    unsafe {
        let mut wildMonInfo = wildMonInfo;
        let mut rod = rod;
        let mut wildMonIndex: u8 = ChooseWildMonIndex_Fishing(rod);
        let mut level: u8 = ChooseWildMonLevel(
            (((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_offset(((wildMonIndex) as i32) as isize * 4),
        );
        CreateWildMon(
            (((((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_offset(((wildMonIndex) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
            level,
        );
        return (((((wildMonInfo).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_offset(((wildMonIndex) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn SetUpMassOutbreakEncounter(flags: u8) -> u8 {
    unsafe {
        let mut flags = flags;
        let mut i: u16 = 0u16;
        if ((((flags) as i32) & 1i32) != 0)
            && (!((IsWildLevelAllowedByRepel(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11156)).read(),
            )) != 0))
        {
            return 0u8;
        }
        CreateWildMon(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(11152)
                .cast::<u16>())
            .read(),
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11156)).read(),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonMoveSlot(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11160))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DoMassOutbreakEncounterTest() -> u8 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11152)
            .cast::<u16>())
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11154))
                    .read()) as i32)))
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11155))
                    .read()) as i32))
        {
            if crate::c::rem_i32(((Random()) as i32), 100i32)
                < ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11169))
                    .read()) as i32)
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EncounterOddsCheck(encounterRate: u16) -> u8 {
    unsafe {
        let mut encounterRate = encounterRate;
        if crate::c::rem_i32(((Random()) as i32), 2880i32) < ((encounterRate) as i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn WildEncounterCheck(encounterRate: u32, ignoreAbility: u8) -> u8 {
    unsafe {
        let mut encounterRate = encounterRate;
        let mut ignoreAbility = ignoreAbility;
        encounterRate = (encounterRate).wrapping_mul(16u32);
        if (TestPlayerAvatarFlags(6u8)) != 0 {
            encounterRate = crate::c::div_u32((encounterRate).wrapping_mul(80u32), 100u32);
        }
        ApplyFluteEncounterRateMod(&raw mut encounterRate);
        ApplyCleanseTagEncounterRateMod(&raw mut encounterRate);
        if (!((ignoreAbility) != 0))
            && (!((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0))
        {
            let mut ability: u32 = ((GetMonAbility((&raw mut gPlayerParty).cast::<u8>())) as u32);
            if (ability == 1u32)
                && ((((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read()) as i32)
                    == 361i32)
            {
                encounterRate = crate::c::div_u32((encounterRate).wrapping_mul(3u32), 4u32);
            } else {
                if ability == 1u32 {
                    encounterRate = crate::c::div_u32(encounterRate, 2u32);
                } else {
                    if ability == 35u32 {
                        encounterRate = (encounterRate).wrapping_mul(2u32);
                    } else {
                        if ability == 73u32 {
                            encounterRate = crate::c::div_u32(encounterRate, 2u32);
                        } else {
                            if ability == 71u32 {
                                encounterRate = (encounterRate).wrapping_mul(2u32);
                            } else {
                                if (ability == 8u32)
                                    && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(46))
                                    .read()) as i32)
                                        == 8i32)
                                {
                                    encounterRate = crate::c::div_u32(encounterRate, 2u32);
                                }
                            }
                        }
                    }
                }
            }
        }
        if encounterRate > 2880u32 {
            encounterRate = 2880u32;
        }
        return EncounterOddsCheck(((encounterRate) as u16));
    }
}
pub(crate) unsafe extern "C" fn AllowWildCheckOnNewMetatile() -> u8 {
    unsafe {
        if crate::c::rem_i32(((Random()) as i32), 100i32) >= 60i32 {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn AreLegendariesInSootopolisPreventingEncounters() -> u8 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            != 0i32)
            || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != 7i32)
        {
            return 0u8;
        }
        return FlagGet(83u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StandardWildEncounter(
    curMetatileBehavior: u16,
    prevMetatileBehavior: u16,
) -> u8 {
    unsafe {
        let mut curMetatileBehavior = curMetatileBehavior;
        let mut prevMetatileBehavior = prevMetatileBehavior;
        let mut headerId: u16 = 0u16;
        let mut roamer: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut sWildEncountersDisabled).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32
        {
            return 0u8;
        }
        headerId = GetCurrentMapWildMonHeaderId();
        if ((headerId) as i32) == 65535i32 {
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 358i32
            {
                headerId = ((GetBattlePikeWildMonHeaderId()) as u16);
                if (((prevMetatileBehavior) as i32) != ((curMetatileBehavior) as i32))
                    && (!((AllowWildCheckOnNewMetatile()) != 0))
                {
                    return 0u8;
                } else {
                    if ((WildEncounterCheck(
                        ((((((((&raw const gBattlePikeWildMonHeaders)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((headerId) as i32) as isize * 20))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .read()) as u32),
                        0u8,
                    )) as i32)
                        != 1i32
                    {
                        return 0u8;
                    } else {
                        if ((TryGenerateWildMon(
                            (((((&raw const gBattlePikeWildMonHeaders)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read(),
                            0u8,
                            2u8,
                        )) as i32)
                            != 1i32
                        {
                            return 0u8;
                        } else {
                            if !((TryGenerateBattlePikeWildMon(1u8)) != 0) {
                                return 0u8;
                            }
                        }
                    }
                }
                BattleSetup_StartBattlePikeWildBattle();
                return 1u8;
            }
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 361i32
            {
                headerId = (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
                .read();
                if (((prevMetatileBehavior) as i32) != ((curMetatileBehavior) as i32))
                    && (!((AllowWildCheckOnNewMetatile()) != 0))
                {
                    return 0u8;
                } else {
                    if ((WildEncounterCheck(
                        ((((((((&raw const gBattlePyramidWildMonHeaders)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((headerId) as i32) as isize * 20))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .read()) as u32),
                        0u8,
                    )) as i32)
                        != 1i32
                    {
                        return 0u8;
                    } else {
                        if ((TryGenerateWildMon(
                            (((((&raw const gBattlePyramidWildMonHeaders)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read(),
                            0u8,
                            2u8,
                        )) as i32)
                            != 1i32
                        {
                            return 0u8;
                        }
                    }
                }
                GenerateBattlePyramidWildMon();
                BattleSetup_StartWildBattle();
                return 1u8;
            }
        } else {
            if ((MetatileBehavior_IsLandWildEncounter(((curMetatileBehavior) as u8))) as i32)
                == 1i32
            {
                if (((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((headerId) as i32) as isize * 20))
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read()) as usize)
                    == 0usize
                {
                    return 0u8;
                } else {
                    if (((prevMetatileBehavior) as i32) != ((curMetatileBehavior) as i32))
                        && (!((AllowWildCheckOnNewMetatile()) != 0))
                    {
                        return 0u8;
                    } else {
                        if ((WildEncounterCheck(
                            ((((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .read()) as u32),
                            0u8,
                        )) as i32)
                            != 1i32
                        {
                            return 0u8;
                        }
                    }
                }
                if ((TryStartRoamerEncounter()) as i32) == 1i32 {
                    roamer =
                        (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12764);
                    if !((IsWildLevelAllowedByRepel(((roamer).wrapping_add(12)).read())) != 0) {
                        return 0u8;
                    }
                    BattleSetup_StartRoamerBattle();
                    return 1u8;
                } else {
                    if (((DoMassOutbreakEncounterTest()) as i32) == 1i32)
                        && (((SetUpMassOutbreakEncounter(3u8)) as i32) == 1i32)
                    {
                        BattleSetup_StartWildBattle();
                        return 1u8;
                    }
                    if ((TryGenerateWildMon(
                        (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read(),
                        0u8,
                        3u8,
                    )) as i32)
                        == 1i32
                    {
                        BattleSetup_StartWildBattle();
                        return 1u8;
                    }
                    return 0u8;
                }
            } else {
                if (((MetatileBehavior_IsWaterWildEncounter(((curMetatileBehavior) as u8))) as i32)
                    == 1i32)
                    || (((TestPlayerAvatarFlags(8u8)) != 0)
                        && (((MetatileBehavior_IsBridgeOverWater(((curMetatileBehavior) as u8)))
                            as i32)
                            == 1i32))
                {
                    if ((AreLegendariesInSootopolisPreventingEncounters()) as i32) == 1i32 {
                        return 0u8;
                    } else {
                        if (((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((headerId) as i32) as isize * 20))
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                        .read()) as usize)
                            == 0usize
                        {
                            return 0u8;
                        } else {
                            if (((prevMetatileBehavior) as i32) != ((curMetatileBehavior) as i32))
                                && (!((AllowWildCheckOnNewMetatile()) != 0))
                            {
                                return 0u8;
                            } else {
                                if ((WildEncounterCheck(
                                    ((((((((&raw const gWildMonHeaders)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((headerId) as i32) as isize * 20))
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                    .read())
                                    .read()) as u32),
                                    0u8,
                                )) as i32)
                                    != 1i32
                                {
                                    return 0u8;
                                }
                            }
                        }
                    }
                    if ((TryStartRoamerEncounter()) as i32) == 1i32 {
                        roamer = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12764);
                        if !((IsWildLevelAllowedByRepel(((roamer).wrapping_add(12)).read())) != 0) {
                            return 0u8;
                        }
                        BattleSetup_StartRoamerBattle();
                        return 1u8;
                    } else {
                        if ((TryGenerateWildMon(
                            (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read(),
                            1u8,
                            3u8,
                        )) as i32)
                            == 1i32
                        {
                            BattleSetup_StartWildBattle();
                            return 1u8;
                        }
                        return 0u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RockSmashWildEncounter() {
    unsafe {
        let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
        if ((headerId) as i32) != 65535i32 {
            let mut wildPokemonInfo: *mut u8 =
                (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((headerId) as i32) as isize * 20))
                .wrapping_add(12)
                .cast::<*mut u8>())
                .read();
            if ((wildPokemonInfo) as usize) == 0usize {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            } else {
                if (((WildEncounterCheck((((wildPokemonInfo).read()) as u32), 1u8)) as i32) == 1i32)
                    && (((TryGenerateWildMon(wildPokemonInfo, 2u8, 3u8)) as i32) == 1i32)
                {
                    BattleSetup_StartWildBattle();
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                }
            }
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SweetScentWildEncounter() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut headerId: u16 = 0u16;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        headerId = GetCurrentMapWildMonHeaderId();
        if ((headerId) as i32) == 65535i32 {
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 358i32
            {
                headerId = ((GetBattlePikeWildMonHeaderId()) as u16);
                if ((TryGenerateWildMon(
                    (((((&raw const gBattlePikeWildMonHeaders)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((headerId) as i32) as isize * 20))
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read(),
                    0u8,
                    0u8,
                )) as i32)
                    != 1i32
                {
                    return 0u8;
                }
                TryGenerateBattlePikeWildMon(0u8);
                BattleSetup_StartBattlePikeWildBattle();
                return 1u8;
            }
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 361i32
            {
                headerId = (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
                .read();
                if ((TryGenerateWildMon(
                    (((((&raw const gBattlePyramidWildMonHeaders)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((headerId) as i32) as isize * 20))
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read(),
                    0u8,
                    0u8,
                )) as i32)
                    != 1i32
                {
                    return 0u8;
                }
                GenerateBattlePyramidWildMon();
                BattleSetup_StartWildBattle();
                return 1u8;
            }
        } else {
            if ((MetatileBehavior_IsLandWildEncounter(
                ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
            )) as i32)
                == 1i32
            {
                if (((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((headerId) as i32) as isize * 20))
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read()) as usize)
                    == 0usize
                {
                    return 0u8;
                }
                if ((TryStartRoamerEncounter()) as i32) == 1i32 {
                    BattleSetup_StartRoamerBattle();
                    return 1u8;
                }
                if ((DoMassOutbreakEncounterTest()) as i32) == 1i32 {
                    SetUpMassOutbreakEncounter(0u8);
                } else {
                    TryGenerateWildMon(
                        (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read(),
                        0u8,
                        0u8,
                    );
                }
                BattleSetup_StartWildBattle();
                return 1u8;
            } else {
                if ((MetatileBehavior_IsWaterWildEncounter(
                    ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                )) as i32)
                    == 1i32
                {
                    if ((AreLegendariesInSootopolisPreventingEncounters()) as i32) == 1i32 {
                        return 0u8;
                    }
                    if (((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((headerId) as i32) as isize * 20))
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                    .read()) as usize)
                        == 0usize
                    {
                        return 0u8;
                    }
                    if ((TryStartRoamerEncounter()) as i32) == 1i32 {
                        BattleSetup_StartRoamerBattle();
                        return 1u8;
                    }
                    TryGenerateWildMon(
                        (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((headerId) as i32) as isize * 20))
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                        .read(),
                        1u8,
                        0u8,
                    );
                    BattleSetup_StartWildBattle();
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesCurrentMapHaveFishingMons() -> u8 {
    unsafe {
        let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
        if (((headerId) as i32) != 65535i32)
            && ((((((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((headerId) as i32) as isize * 20))
            .wrapping_add(16)
            .cast::<*mut u8>())
            .read()) as usize)
                != 0usize)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FishingWildEncounter(rod: u8) {
    unsafe {
        let mut rod = rod;
        let mut species: u16 = 0u16;
        if ((CheckFeebas()) as i32) == 1i32 {
            let mut level: u8 =
                ChooseWildMonLevel((&raw const sWildFeebas).cast::<u8>().cast_mut());
            species = (((&raw const sWildFeebas).cast::<u8>().cast_mut())
                .wrapping_add(2)
                .cast::<u16>())
            .read();
            CreateWildMon(species, level);
        } else {
            species = GenerateFishingWildMon(
                (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((GetCurrentMapWildMonHeaderId()) as i32) as isize * 20))
                .wrapping_add(16)
                .cast::<*mut u8>())
                .read(),
                rod,
            );
        }
        IncrementGameStat(12u8);
        SetPokemonAnglerSpecies(species);
        BattleSetup_StartWildBattle();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocalWildMon(isWaterMon: *mut u8) -> u16 {
    unsafe {
        let mut isWaterMon = isWaterMon;
        let mut headerId: u16 = 0u16;
        let mut landMonsInfo: *mut u8 = core::ptr::null_mut();
        let mut waterMonsInfo: *mut u8 = core::ptr::null_mut();
        (isWaterMon).write(0u8);
        headerId = GetCurrentMapWildMonHeaderId();
        if ((headerId) as i32) == 65535i32 {
            return 0u16;
        }
        landMonsInfo = (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((headerId) as i32) as isize * 20))
        .wrapping_add(4)
        .cast::<*mut u8>())
        .read();
        waterMonsInfo = (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((headerId) as i32) as isize * 20))
        .wrapping_add(8)
        .cast::<*mut u8>())
        .read();
        if (((landMonsInfo) as usize) == 0usize) && (((waterMonsInfo) as usize) == 0usize) {
            return 0u16;
        } else {
            if (((landMonsInfo) as usize) != 0usize) && (((waterMonsInfo) as usize) == 0usize) {
                return (((((landMonsInfo).wrapping_add(4).cast::<*mut u8>()).read())
                    .wrapping_offset(((ChooseWildMonIndex_Land()) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read();
            } else {
                if (((landMonsInfo) as usize) == 0usize) && (((waterMonsInfo) as usize) != 0usize) {
                    (isWaterMon).write(1u8);
                    return (((((waterMonsInfo).wrapping_add(4).cast::<*mut u8>()).read())
                        .wrapping_offset(((ChooseWildMonIndex_WaterRock()) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read();
                }
            }
        }
        if crate::c::rem_i32(((Random()) as i32), 100i32) < 80i32 {
            return (((((landMonsInfo).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_offset(((ChooseWildMonIndex_Land()) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read();
        } else {
            (isWaterMon).write(1u8);
            return (((((waterMonsInfo).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_offset(((ChooseWildMonIndex_WaterRock()) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocalWaterMon() -> u16 {
    unsafe {
        let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
        if ((headerId) as i32) != 65535i32 {
            let mut waterMonsInfo: *mut u8 =
                (((((&raw const gWildMonHeaders).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((headerId) as i32) as isize * 20))
                .wrapping_add(8)
                .cast::<*mut u8>())
                .read();
            if !(waterMonsInfo).is_null() {
                return (((((waterMonsInfo).wrapping_add(4).cast::<*mut u8>()).read())
                    .wrapping_offset(((ChooseWildMonIndex_WaterRock()) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read();
            }
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRepelCounter() -> u8 {
    unsafe {
        let mut steps: u16 = 0u16;
        if ((InBattlePike()) != 0) || (((CurrentBattlePyramidLocation()) as i32) != 0i32) {
            return 0u8;
        }
        if InUnionRoom() == 1u32 {
            return 0u8;
        }
        steps = VarGet(16417u16);
        if ((steps) as i32) != 0i32 {
            steps = (steps).wrapping_sub(1);
            VarSet(16417u16, steps);
            if ((steps) as i32) == 0i32 {
                ScriptContext_SetupScript((&raw mut EventScript_RepelWoreOff).cast::<u8>());
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsWildLevelAllowedByRepel(wildLevel: u8) -> u8 {
    unsafe {
        let mut wildLevel = wildLevel;
        let mut i: u8 = 0u8;
        if !((VarGet(16417u16)) != 0) {
            return 1u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        57i32,
                    )) != 0)
                        && (!((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            45i32,
                        )) != 0))
                    {
                        let mut ourLevel: u8 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            56i32,
                        )) as u8);
                        if ((wildLevel) as i32) < ((ourLevel) as i32) {
                            return 0u8;
                        } else {
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsAbilityAllowingEncounter(level: u8) -> u8 {
    unsafe {
        let mut level = level;
        let mut ability: u8 = 0u8;
        if (GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0 {
            return 1u8;
        }
        ability = GetMonAbility((&raw mut gPlayerParty).cast::<u8>());
        if (((ability) as i32) == 51i32) || (((ability) as i32) == 22i32) {
            let mut playerMonLevel: u8 =
                ((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 56i32)) as u8);
            if ((((playerMonLevel) as i32) > 5i32)
                && (((level) as i32) <= ((playerMonLevel) as i32).wrapping_sub(5i32)))
                && (!((crate::c::rem_i32(((Random()) as i32), 2i32)) != 0))
            {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryGetRandomWildMonIndexByType(
    wildMon: *mut u8,
    r#type: u8,
    numMon: u8,
    monIndex: *mut u8,
) -> u8 {
    unsafe {
        let mut wildMon = wildMon;
        let mut r#type = r#type;
        let mut numMon = numMon;
        let mut monIndex = monIndex;
        let mut validIndexes = crate::ffi::Align4([0u8; 256]);
        let mut i: u8 = 0u8;
        let mut validMonCount: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numMon) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut validIndexes).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            validMonCount = 0u8;
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < ((numMon) as i32)) {
                    break 'l3;
                }
                'l4: {
                    if ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        (((((wildMon).wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .read()) as i32)
                        == ((r#type) as i32))
                        || (((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            (((((wildMon).wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            == ((r#type) as i32))
                    {
                        (((&raw mut validIndexes).cast::<u8>()).wrapping_offset(
                            (({
                                let __t1 = validMonCount;
                                validMonCount = (validMonCount).wrapping_add(1);
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(i);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((validMonCount) as i32) == 0i32) || (((validMonCount) as i32) == ((numMon) as i32)) {
            return 0u8;
        }
        (monIndex).write(
            (((&raw mut validIndexes).cast::<u8>()).wrapping_offset(
                (crate::c::rem_i32(((Random()) as i32), ((validMonCount) as i32))) as isize,
            ))
            .read(),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryGetAbilityInfluencedWildMonIndex(
    wildMon: *mut u8,
    r#type: u8,
    ability: u8,
    monIndex: *mut u8,
) -> u8 {
    unsafe {
        let mut wildMon = wildMon;
        let mut r#type = r#type;
        let mut ability = ability;
        let mut monIndex = monIndex;
        if (GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0 {
            return 0u8;
        } else {
            if ((GetMonAbility((&raw mut gPlayerParty).cast::<u8>())) as i32) != ((ability) as i32)
            {
                return 0u8;
            } else {
                if crate::c::rem_i32(((Random()) as i32), 2i32) != 0i32 {
                    return 0u8;
                }
            }
        }
        return TryGetRandomWildMonIndexByType(wildMon, r#type, 12u8, monIndex);
    }
}
pub(crate) unsafe extern "C" fn ApplyFluteEncounterRateMod(encRate: *mut u32) {
    unsafe {
        let mut encRate = encRate;
        if ((FlagGet(2221u16)) as i32) == 1i32 {
            (encRate)
                .write(((encRate).read()).wrapping_add(crate::c::div_u32((encRate).read(), 2u32)));
        } else {
            if ((FlagGet(2222u16)) as i32) == 1i32 {
                (encRate).write(crate::c::div_u32((encRate).read(), 2u32));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyCleanseTagEncounterRateMod(encRate: *mut u32) {
    unsafe {
        let mut encRate = encRate;
        if GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 12i32) == 190u32 {
            (encRate).write(crate::c::div_u32(
                ((encRate).read()).wrapping_mul(2u32),
                3u32,
            ));
        }
    }
}
