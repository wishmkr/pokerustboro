//! Translated from `src/wild_encounter.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::battle_pike::{
    GetBattlePikeWildMonHeaderId, InBattlePike, TryGenerateBattlePikeWildMon,
};
use crate::battle_pyramid::{CurrentBattlePyramidLocation, GenerateBattlePyramidWildMon};
use crate::battle_setup::{
    BattleSetup_StartBattlePikeWildBattle, BattleSetup_StartRoamerBattle,
    BattleSetup_StartWildBattle,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, VarGet, VarSet};
use crate::ffi::gSpecialVar_Result;
use crate::field_player_avatar::{
    GetXYCoordsOneStepInFrontOfPlayer, PlayerGetDestCoords, TestPlayerAvatarFlags,
};
use crate::fieldmap::{MapGridGetMetatileBehaviorAt, gMapHeader};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::metatile_behavior::{
    MetatileBehavior_IsBridgeOverWater, MetatileBehavior_IsLandWildEncounter,
    MetatileBehavior_IsSurfableAndNotWaterfall, MetatileBehavior_IsWaterWildEncounter,
};
use crate::overworld::IncrementGameStat;
use crate::pokeblock::PokeblockGetGain;
use crate::pokemon::{
    CreateMonWithGenderNatureLetter, CreateMonWithNature, GetGenderFromSpeciesAndPersonality,
    GetMonAbility, GetMonData2, SetMonMoveSlot, ZeroEnemyPartyMons, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::roamer::TryStartRoamerEncounter;
use crate::safari_zone::GetSafariZoneFlag;
use crate::script::ScriptContext_SetupScript;
use crate::tv::SetPokemonAnglerSpecies;
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::InUnionRoom;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): gRoute101_LandMons gRoute101_LandMonsInfo gRoute102_LandMons gRoute102_LandMonsInfo gRoute102_WaterMons gRoute102_WaterMonsInfo gRoute102_FishingMons gRoute102_FishingMonsInfo gRoute103_LandMons gRoute103_LandMonsInfo gRoute103_WaterMons gRoute103_WaterMonsInfo gRoute103_FishingMons gRoute103_FishingMonsInfo gRoute104_LandMons gRoute104_LandMonsInfo gRoute104_WaterMons gRoute104_WaterMonsInfo gRoute104_FishingMons gRoute104_FishingMonsInfo gRoute105_WaterMons gRoute105_WaterMonsInfo gRoute105_FishingMons gRoute105_FishingMonsInfo gRoute110_LandMons gRoute110_LandMonsInfo gRoute110_WaterMons gRoute110_WaterMonsInfo gRoute110_FishingMons gRoute110_FishingMonsInfo gRoute111_LandMons gRoute111_LandMonsInfo gRoute111_WaterMons gRoute111_WaterMonsInfo gRoute111_RockSmashMons gRoute111_RockSmashMonsInfo gRoute111_FishingMons gRoute111_FishingMonsInfo gRoute112_LandMons gRoute112_LandMonsInfo gRoute113_LandMons gRoute113_LandMonsInfo gRoute114_LandMons gRoute114_LandMonsInfo gRoute114_WaterMons gRoute114_WaterMonsInfo gRoute114_RockSmashMons gRoute114_RockSmashMonsInfo gRoute114_FishingMons gRoute114_FishingMonsInfo gRoute116_LandMons gRoute116_LandMonsInfo gRoute117_LandMons gRoute117_LandMonsInfo gRoute117_WaterMons gRoute117_WaterMonsInfo gRoute117_FishingMons gRoute117_FishingMonsInfo gRoute118_LandMons gRoute118_LandMonsInfo gRoute118_WaterMons gRoute118_WaterMonsInfo gRoute118_FishingMons gRoute118_FishingMonsInfo gRoute124_WaterMons gRoute124_WaterMonsInfo gRoute124_FishingMons gRoute124_FishingMonsInfo gPetalburgWoods_LandMons gPetalburgWoods_LandMonsInfo gRusturfTunnel_LandMons gRusturfTunnel_LandMonsInfo gGraniteCave_1F_LandMons gGraniteCave_1F_LandMonsInfo gGraniteCave_B1F_LandMons gGraniteCave_B1F_LandMonsInfo gMtPyre_1F_LandMons gMtPyre_1F_LandMonsInfo gVictoryRoad_1F_LandMons gVictoryRoad_1F_LandMonsInfo gSafariZone_South_LandMons gSafariZone_South_LandMonsInfo gUnderwater_Route126_WaterMons gUnderwater_Route126_WaterMonsInfo gAbandonedShip_Rooms_B1F_WaterMons gAbandonedShip_Rooms_B1F_WaterMonsInfo gAbandonedShip_Rooms_B1F_FishingMons gAbandonedShip_Rooms_B1F_FishingMonsInfo gGraniteCave_B2F_LandMons gGraniteCave_B2F_LandMonsInfo gGraniteCave_B2F_RockSmashMons gGraniteCave_B2F_RockSmashMonsInfo gFieryPath_LandMons gFieryPath_LandMonsInfo gMeteorFalls_B1F_2R_LandMons gMeteorFalls_B1F_2R_LandMonsInfo gMeteorFalls_B1F_2R_WaterMons gMeteorFalls_B1F_2R_WaterMonsInfo gMeteorFalls_B1F_2R_FishingMons gMeteorFalls_B1F_2R_FishingMonsInfo gJaggedPass_LandMons gJaggedPass_LandMonsInfo gRoute106_WaterMons gRoute106_WaterMonsInfo gRoute106_FishingMons gRoute106_FishingMonsInfo gRoute107_WaterMons gRoute107_WaterMonsInfo gRoute107_FishingMons gRoute107_FishingMonsInfo gRoute108_WaterMons gRoute108_WaterMonsInfo gRoute108_FishingMons gRoute108_FishingMonsInfo gRoute109_WaterMons gRoute109_WaterMonsInfo gRoute109_FishingMons gRoute109_FishingMonsInfo gRoute115_LandMons gRoute115_LandMonsInfo gRoute115_WaterMons gRoute115_WaterMonsInfo gRoute115_FishingMons gRoute115_FishingMonsInfo gNewMauville_Inside_LandMons gNewMauville_Inside_LandMonsInfo gRoute119_LandMons gRoute119_LandMonsInfo gRoute119_WaterMons gRoute119_WaterMonsInfo gRoute119_FishingMons gRoute119_FishingMonsInfo gRoute120_LandMons gRoute120_LandMonsInfo gRoute120_WaterMons gRoute120_WaterMonsInfo gRoute120_FishingMons gRoute120_FishingMonsInfo gRoute121_LandMons gRoute121_LandMonsInfo gRoute121_WaterMons gRoute121_WaterMonsInfo gRoute121_FishingMons gRoute121_FishingMonsInfo gRoute122_WaterMons gRoute122_WaterMonsInfo gRoute122_FishingMons gRoute122_FishingMonsInfo gRoute123_LandMons gRoute123_LandMonsInfo gRoute123_WaterMons gRoute123_WaterMonsInfo gRoute123_FishingMons gRoute123_FishingMonsInfo gMtPyre_2F_LandMons gMtPyre_2F_LandMonsInfo gMtPyre_3F_LandMons gMtPyre_3F_LandMonsInfo gMtPyre_4F_LandMons gMtPyre_4F_LandMonsInfo gMtPyre_5F_LandMons gMtPyre_5F_LandMonsInfo gMtPyre_6F_LandMons gMtPyre_6F_LandMonsInfo gMtPyre_Exterior_LandMons gMtPyre_Exterior_LandMonsInfo gMtPyre_Summit_LandMons gMtPyre_Summit_LandMonsInfo gGraniteCave_StevensRoom_LandMons gGraniteCave_StevensRoom_LandMonsInfo gRoute125_WaterMons gRoute125_WaterMonsInfo gRoute125_FishingMons gRoute125_FishingMonsInfo gRoute126_WaterMons gRoute126_WaterMonsInfo gRoute126_FishingMons gRoute126_FishingMonsInfo gRoute127_WaterMons gRoute127_WaterMonsInfo gRoute127_FishingMons gRoute127_FishingMonsInfo gRoute128_WaterMons gRoute128_WaterMonsInfo gRoute128_FishingMons gRoute128_FishingMonsInfo gRoute129_WaterMons gRoute129_WaterMonsInfo gRoute129_FishingMons gRoute129_FishingMonsInfo gRoute130_LandMons gRoute130_LandMonsInfo gRoute130_WaterMons gRoute130_WaterMonsInfo gRoute130_FishingMons gRoute130_FishingMonsInfo gRoute131_WaterMons gRoute131_WaterMonsInfo gRoute131_FishingMons gRoute131_FishingMonsInfo gRoute132_WaterMons gRoute132_WaterMonsInfo gRoute132_FishingMons gRoute132_FishingMonsInfo gRoute133_WaterMons gRoute133_WaterMonsInfo gRoute133_FishingMons gRoute133_FishingMonsInfo gRoute134_WaterMons gRoute134_WaterMonsInfo gRoute134_FishingMons gRoute134_FishingMonsInfo gAbandonedShip_HiddenFloorCorridors_WaterMons gAbandonedShip_HiddenFloorCorridors_WaterMonsInfo gAbandonedShip_HiddenFloorCorridors_FishingMons gAbandonedShip_HiddenFloorCorridors_FishingMonsInfo gSeafloorCavern_Room1_LandMons gSeafloorCavern_Room1_LandMonsInfo gSeafloorCavern_Room2_LandMons gSeafloorCavern_Room2_LandMonsInfo gSeafloorCavern_Room3_LandMons gSeafloorCavern_Room3_LandMonsInfo gSeafloorCavern_Room4_LandMons gSeafloorCavern_Room4_LandMonsInfo gSeafloorCavern_Room5_LandMons gSeafloorCavern_Room5_LandMonsInfo gSeafloorCavern_Room6_LandMons gSeafloorCavern_Room6_LandMonsInfo gSeafloorCavern_Room6_WaterMons gSeafloorCavern_Room6_WaterMonsInfo gSeafloorCavern_Room6_FishingMons gSeafloorCavern_Room6_FishingMonsInfo gSeafloorCavern_Room7_LandMons gSeafloorCavern_Room7_LandMonsInfo gSeafloorCavern_Room7_WaterMons gSeafloorCavern_Room7_WaterMonsInfo gSeafloorCavern_Room7_FishingMons gSeafloorCavern_Room7_FishingMonsInfo gSeafloorCavern_Room8_LandMons gSeafloorCavern_Room8_LandMonsInfo gSeafloorCavern_Entrance_WaterMons gSeafloorCavern_Entrance_WaterMonsInfo gSeafloorCavern_Entrance_FishingMons gSeafloorCavern_Entrance_FishingMonsInfo gCaveOfOrigin_Entrance_LandMons gCaveOfOrigin_Entrance_LandMonsInfo gCaveOfOrigin_1F_LandMons gCaveOfOrigin_1F_LandMonsInfo gCaveOfOrigin_UnusedRubySapphireMap1_LandMons gCaveOfOrigin_UnusedRubySapphireMap1_LandMonsInfo gCaveOfOrigin_UnusedRubySapphireMap2_LandMons gCaveOfOrigin_UnusedRubySapphireMap2_LandMonsInfo gCaveOfOrigin_UnusedRubySapphireMap3_LandMons gCaveOfOrigin_UnusedRubySapphireMap3_LandMonsInfo gNewMauville_Entrance_LandMons gNewMauville_Entrance_LandMonsInfo gSafariZone_Southwest_LandMons gSafariZone_Southwest_LandMonsInfo gSafariZone_Southwest_WaterMons gSafariZone_Southwest_WaterMonsInfo gSafariZone_Southwest_FishingMons gSafariZone_Southwest_FishingMonsInfo gSafariZone_North_LandMons gSafariZone_North_LandMonsInfo gSafariZone_North_RockSmashMons gSafariZone_North_RockSmashMonsInfo gSafariZone_Northwest_LandMons gSafariZone_Northwest_LandMonsInfo gSafariZone_Northwest_WaterMons gSafariZone_Northwest_WaterMonsInfo gSafariZone_Northwest_FishingMons gSafariZone_Northwest_FishingMonsInfo gVictoryRoad_B1F_LandMons gVictoryRoad_B1F_LandMonsInfo gVictoryRoad_B1F_RockSmashMons gVictoryRoad_B1F_RockSmashMonsInfo gVictoryRoad_B2F_LandMons gVictoryRoad_B2F_LandMonsInfo gVictoryRoad_B2F_WaterMons gVictoryRoad_B2F_WaterMonsInfo gVictoryRoad_B2F_FishingMons gVictoryRoad_B2F_FishingMonsInfo gMeteorFalls_1F_1R_LandMons gMeteorFalls_1F_1R_LandMonsInfo gMeteorFalls_1F_1R_WaterMons gMeteorFalls_1F_1R_WaterMonsInfo gMeteorFalls_1F_1R_FishingMons gMeteorFalls_1F_1R_FishingMonsInfo gMeteorFalls_1F_2R_LandMons gMeteorFalls_1F_2R_LandMonsInfo gMeteorFalls_1F_2R_WaterMons gMeteorFalls_1F_2R_WaterMonsInfo gMeteorFalls_1F_2R_FishingMons gMeteorFalls_1F_2R_FishingMonsInfo gMeteorFalls_B1F_1R_LandMons gMeteorFalls_B1F_1R_LandMonsInfo gMeteorFalls_B1F_1R_WaterMons gMeteorFalls_B1F_1R_WaterMonsInfo gMeteorFalls_B1F_1R_FishingMons gMeteorFalls_B1F_1R_FishingMonsInfo gShoalCave_LowTideStairsRoom_LandMons gShoalCave_LowTideStairsRoom_LandMonsInfo gShoalCave_LowTideLowerRoom_LandMons gShoalCave_LowTideLowerRoom_LandMonsInfo gShoalCave_LowTideInnerRoom_LandMons gShoalCave_LowTideInnerRoom_LandMonsInfo gShoalCave_LowTideInnerRoom_WaterMons gShoalCave_LowTideInnerRoom_WaterMonsInfo gShoalCave_LowTideInnerRoom_FishingMons gShoalCave_LowTideInnerRoom_FishingMonsInfo gShoalCave_LowTideEntranceRoom_LandMons gShoalCave_LowTideEntranceRoom_LandMonsInfo gShoalCave_LowTideEntranceRoom_WaterMons gShoalCave_LowTideEntranceRoom_WaterMonsInfo gShoalCave_LowTideEntranceRoom_FishingMons gShoalCave_LowTideEntranceRoom_FishingMonsInfo gLilycoveCity_WaterMons gLilycoveCity_WaterMonsInfo gLilycoveCity_FishingMons gLilycoveCity_FishingMonsInfo gDewfordTown_WaterMons gDewfordTown_WaterMonsInfo gDewfordTown_FishingMons gDewfordTown_FishingMonsInfo gSlateportCity_WaterMons gSlateportCity_WaterMonsInfo gSlateportCity_FishingMons gSlateportCity_FishingMonsInfo gMossdeepCity_WaterMons gMossdeepCity_WaterMonsInfo gMossdeepCity_FishingMons gMossdeepCity_FishingMonsInfo gPacifidlogTown_WaterMons gPacifidlogTown_WaterMonsInfo gPacifidlogTown_FishingMons gPacifidlogTown_FishingMonsInfo gEverGrandeCity_WaterMons gEverGrandeCity_WaterMonsInfo gEverGrandeCity_FishingMons gEverGrandeCity_FishingMonsInfo gPetalburgCity_WaterMons gPetalburgCity_WaterMonsInfo gPetalburgCity_FishingMons gPetalburgCity_FishingMonsInfo gUnderwater_Route124_WaterMons gUnderwater_Route124_WaterMonsInfo gShoalCave_LowTideIceRoom_LandMons gShoalCave_LowTideIceRoom_LandMonsInfo gSkyPillar_1F_LandMons gSkyPillar_1F_LandMonsInfo gSootopolisCity_WaterMons gSootopolisCity_WaterMonsInfo gSootopolisCity_FishingMons gSootopolisCity_FishingMonsInfo gSkyPillar_3F_LandMons gSkyPillar_3F_LandMonsInfo gSkyPillar_5F_LandMons gSkyPillar_5F_LandMonsInfo gSafariZone_Southeast_LandMons gSafariZone_Southeast_LandMonsInfo gSafariZone_Southeast_WaterMons gSafariZone_Southeast_WaterMonsInfo gSafariZone_Southeast_FishingMons gSafariZone_Southeast_FishingMonsInfo gSafariZone_Northeast_LandMons gSafariZone_Northeast_LandMonsInfo gSafariZone_Northeast_RockSmashMons gSafariZone_Northeast_RockSmashMonsInfo gMagmaHideout_1F_LandMons gMagmaHideout_1F_LandMonsInfo gMagmaHideout_2F_1R_LandMons gMagmaHideout_2F_1R_LandMonsInfo gMagmaHideout_2F_2R_LandMons gMagmaHideout_2F_2R_LandMonsInfo gMagmaHideout_3F_1R_LandMons gMagmaHideout_3F_1R_LandMonsInfo gMagmaHideout_3F_2R_LandMons gMagmaHideout_3F_2R_LandMonsInfo gMagmaHideout_4F_LandMons gMagmaHideout_4F_LandMonsInfo gMagmaHideout_3F_3R_LandMons gMagmaHideout_3F_3R_LandMonsInfo gMagmaHideout_2F_3R_LandMons gMagmaHideout_2F_3R_LandMonsInfo gMirageTower_1F_LandMons gMirageTower_1F_LandMonsInfo gMirageTower_2F_LandMons gMirageTower_2F_LandMonsInfo gMirageTower_3F_LandMons gMirageTower_3F_LandMonsInfo gMirageTower_4F_LandMons gMirageTower_4F_LandMonsInfo gDesertUnderpass_LandMons gDesertUnderpass_LandMonsInfo gArtisanCave_B1F_LandMons gArtisanCave_B1F_LandMonsInfo gArtisanCave_1F_LandMons gArtisanCave_1F_LandMonsInfo gAlteringCave1_LandMons gAlteringCave1_LandMonsInfo gAlteringCave2_LandMons gAlteringCave2_LandMonsInfo gAlteringCave3_LandMons gAlteringCave3_LandMonsInfo gAlteringCave4_LandMons gAlteringCave4_LandMonsInfo gAlteringCave5_LandMons gAlteringCave5_LandMonsInfo gAlteringCave6_LandMons gAlteringCave6_LandMonsInfo gAlteringCave7_LandMons gAlteringCave7_LandMonsInfo gAlteringCave8_LandMons gAlteringCave8_LandMonsInfo gAlteringCave9_LandMons gAlteringCave9_LandMonsInfo gMeteorFalls_StevensCave_LandMons gMeteorFalls_StevensCave_LandMonsInfo gWildMonHeaders gBattlePyramid_1_LandMons gBattlePyramid_1_LandMonsInfo gBattlePyramid_2_LandMons gBattlePyramid_2_LandMonsInfo gBattlePyramid_3_LandMons gBattlePyramid_3_LandMonsInfo gBattlePyramid_4_LandMons gBattlePyramid_4_LandMonsInfo gBattlePyramid_5_LandMons gBattlePyramid_5_LandMonsInfo gBattlePyramid_6_LandMons gBattlePyramid_6_LandMonsInfo gBattlePyramid_7_LandMons gBattlePyramid_7_LandMonsInfo gBattlePyramidWildMonHeaders gBattlePike_1_LandMons gBattlePike_1_LandMonsInfo gBattlePike_2_LandMons gBattlePike_2_LandMonsInfo gBattlePike_3_LandMons gBattlePike_3_LandMonsInfo gBattlePike_4_LandMons gBattlePike_4_LandMonsInfo gBattlePikeWildMonHeaders sWildFeebas sRoute119WaterTileData

const HEADER_NONE: u16 = 65535;
const MAX_ENCOUNTER_RATE: u32 = 2880;
const NUM_FEEBAS_SPOTS: u8 = 6;
const NUM_FISHING_SPOTS: u16 = 447;
const WILD_AREA_LAND: u8 = 0;
const WILD_AREA_ROCKS: u8 = 2;
const WILD_AREA_WATER: u8 = 1;
const WILD_CHECK_KEEN_EYE: u8 = 2;
const WILD_CHECK_REPEL: i32 = 1;

static gBattlePikeWildMonHeaders: Table<CArray<WildPokemonHeader, 5>> =
    Table((&raw const crate::data::wild_encounter::gBattlePikeWildMonHeaders).cast());
static gBattlePyramidWildMonHeaders: Table<CArray<WildPokemonHeader, 8>> =
    Table((&raw const crate::data::wild_encounter::gBattlePyramidWildMonHeaders).cast());
static gWildMonHeaders: Table<CArray<WildPokemonHeader, 125>> =
    Table((&raw const crate::data::wild_encounter::gWildMonHeaders).cast());
static sRoute119WaterTileData: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::wild_encounter::sRoute119WaterTileData).cast());
static sWildFeebas: Table<WildPokemon> =
    Table((&raw const crate::data::wild_encounter::sWildFeebas).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sWildEncountersDisabled: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sFeebasRngValue: crate::global::Global<u32> = crate::global::Global::new(0);

/// `SafariZoneGetActivePokeblock` with this module's view of its types.
#[inline]
unsafe fn SafariZoneGetActivePokeblock() -> *mut Pokeblock {
    unsafe { crate::safari_zone::SafariZoneGetActivePokeblock() as *mut Pokeblock }
}

pub fn DisableWildEncounters(disabled: u8) {
    sWildEncountersDisabled.set(disabled);
}
unsafe fn GetFeebasFishingSpotId(targetX: i16, targetY: i16, section: u8) -> u16 {
    let mut x: u16 = 0;
    let yMin: u16 = sRoute119WaterTileData[section as i32 * 3];
    let yMax: u16 = sRoute119WaterTileData[section as i32 * 3 + 1];
    let mut spotId: u16 = sRoute119WaterTileData[section as i32 * 3 + 2];
    let mut y: u16 = yMin;
    while y <= yMax {
        x = 0;
        while (x as i32) < (*gMapHeader.mapLayout).width {
            let behavior: u8 =
                MapGridGetMetatileBehaviorAt(x as i32 + MAP_OFFSET, y as i32 + MAP_OFFSET) as u8;
            if MetatileBehavior_IsSurfableAndNotWaterfall(behavior) == TRUE {
                spotId += 1;
                if targetX as i32 == x as i32 && targetY as i32 == y as i32 {
                    return spotId;
                }
            }
            x += 1;
        }
        y += 1;
    }
    spotId + 1
}
unsafe fn CheckFeebas() -> u8 {
    let mut i: u8 = 0;
    let mut feebasSpots: CArray<u16, 6> = zeroed();
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut route119Section: u8 = 0;
    let mut spotId: u16 = 0;
    if (*gSaveBlock1Ptr).location.mapGroup == 0 && (*gSaveBlock1Ptr).location.mapNum == 34 {
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        x -= MAP_OFFSET as i16;
        y -= MAP_OFFSET as i16;
        if y as i32 >= sRoute119WaterTileData[0] as i32
            && y as i32 <= sRoute119WaterTileData[1] as i32
        {
            route119Section = 0;
        }
        if y as i32 >= sRoute119WaterTileData[3] as i32
            && y as i32 <= sRoute119WaterTileData[4] as i32
        {
            route119Section = 1;
        }
        if y as i32 >= sRoute119WaterTileData[6] as i32
            && y as i32 <= sRoute119WaterTileData[7] as i32
        {
            route119Section = 2;
        }
        if Random() as i32 % 100 > 49 {
            return FALSE;
        }
        FeebasSeedRng((*gSaveBlock1Ptr).dewfordTrends[0].rand);
        i = 0;
        while i != NUM_FEEBAS_SPOTS {
            feebasSpots[i] = (FeebasRandom() as i32 % 447) as u16;
            if feebasSpots[i] == 0 {
                feebasSpots[i] = NUM_FISHING_SPOTS;
            }
            if feebasSpots[i] < 1 || feebasSpots[i] >= 4 {
                i += 1;
            }
        }
        spotId = GetFeebasFishingSpotId(x, y, route119Section);
        for i in 0..NUM_FEEBAS_SPOTS {
            if spotId == feebasSpots[i] {
                return TRUE;
            }
        }
    }
    FALSE
}
fn FeebasRandom() -> u16 {
    sFeebasRngValue.set(0x41c64e6d * sFeebasRngValue.get() + 12345);
    (sFeebasRngValue.get() >> 16) as u16
}
unsafe fn FeebasSeedRng(seed: u16) {
    sFeebasRngValue.set(seed as u32);
}
unsafe fn ChooseWildMonIndex_Land() -> u8 {
    let rand: u8 = (Random() as i32 % 100) as u8;
    if rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_0 as u8 {
        return 0;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_0 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_1 as u8
    {
        return 1;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_1 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_2 as u8
    {
        return 2;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_2 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_3 as u8
    {
        return 3;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_3 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_4 as u8
    {
        return 4;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_4 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_5 as u8
    {
        return 5;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_5 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_6 as u8
    {
        return 6;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_6 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_7 as u8
    {
        return 7;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_7 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_8 as u8
    {
        return 8;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_8 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_9 as u8
    {
        return 9;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_9 as u8
        && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_10 as u8
    {
        return 10;
    } else {
        return 11;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ChooseWildMonIndex_WaterRock() -> u8 {
    let rand: u8 = (Random() as i32 % 100) as u8;
    if rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_0 as u8 {
        return 0;
    } else if rand >= ENCOUNTER_CHANCE_WATER_MONS_SLOT_0 as u8
        && rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_1 as u8
    {
        return 1;
    } else if rand >= ENCOUNTER_CHANCE_WATER_MONS_SLOT_1 as u8
        && rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_2 as u8
    {
        return 2;
    } else if rand >= ENCOUNTER_CHANCE_WATER_MONS_SLOT_2 as u8
        && rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_3 as u8
    {
        return 3;
    } else {
        return 4;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn ChooseWildMonIndex_Fishing(rod: u8) -> u8 {
    let mut wildMonIndex: u8 = 0;
    let rand: u8 = rem_i32(
        Random() as i32,
        if (if 100 >= 100 { 100 } else { 100 }) >= 100 {
            if 100 >= 100 { 100 } else { 100 }
        } else {
            100
        },
    ) as u8;
    match rod {
        OLD_ROD => {
            if rand < ENCOUNTER_CHANCE_FISHING_MONS_OLD_ROD_SLOT_0 {
                wildMonIndex = 0;
            } else {
                wildMonIndex = 1;
            }
        }
        GOOD_ROD => {
            if rand < ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_2 {
                wildMonIndex = 2;
            }
            if (ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_2
                ..ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_3)
                .contains(&rand)
            {
                wildMonIndex = 3;
            }
            if (ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_3
                ..ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_4)
                .contains(&rand)
            {
                wildMonIndex = 4;
            }
        }
        SUPER_ROD => {
            if rand < ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_5 {
                wildMonIndex = 5;
            }
            if (ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_5
                ..ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_6)
                .contains(&rand)
            {
                wildMonIndex = 6;
            }
            if (ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_6
                ..ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_7)
                .contains(&rand)
            {
                wildMonIndex = 7;
            }
            if (ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_7
                ..ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_8)
                .contains(&rand)
            {
                wildMonIndex = 8;
            }
            if (ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_8
                ..ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_9)
                .contains(&rand)
            {
                wildMonIndex = 9;
            }
        }
        _ => {}
    }
    wildMonIndex
}
unsafe fn ChooseWildMonLevel(wildPokemon: *mut WildPokemon) -> u8 {
    let mut min: u8 = 0;
    let mut max: u8 = 0;
    if (*wildPokemon).maxLevel >= (*wildPokemon).minLevel {
        min = (*wildPokemon).minLevel;
        max = (*wildPokemon).maxLevel;
    } else {
        min = (*wildPokemon).maxLevel;
        max = (*wildPokemon).minLevel;
    }
    let range: u8 = max - min + 1;
    let mut rand: u8 = rem_i32(Random() as i32, range as i32) as u8;
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0 {
        let ability: u8 = GetMonAbility(&raw mut gPlayerParty[0]);
        if ability == ABILITY_HUSTLE
            || ability == ABILITY_VITAL_SPIRIT
            || ability == ABILITY_PRESSURE
        {
            if Random() as i32 % 2 == 0 {
                return max;
            }
            rand = rand.saturating_sub(1);
        }
    }
    min + rand
}
unsafe fn GetCurrentMapWildMonHeaderId() -> u16 {
    let mut i: u16 = 0;
    loop {
        let wildHeader: *mut WildPokemonHeader = (&raw const gWildMonHeaders[i]).cast_mut();
        if (*wildHeader).mapGroup == 255 {
            break;
        }
        if gWildMonHeaders[i].mapGroup as i32 == (*gSaveBlock1Ptr).location.mapGroup as i32
            && gWildMonHeaders[i].mapNum as i32 == (*gSaveBlock1Ptr).location.mapNum as i32
        {
            if (*gSaveBlock1Ptr).location.mapGroup == 24 && (*gSaveBlock1Ptr).location.mapNum == 106
            {
                let mut alteringCaveId: u16 = VarGet(VAR_ALTERING_CAVE_WILD_SET);
                if alteringCaveId >= NUM_ALTERING_CAVE_TABLES {
                    alteringCaveId = 0;
                }
                i += alteringCaveId;
            }
            return i;
        }
        i += 1;
    }
    HEADER_NONE
}
unsafe fn PickWildMonNature() -> u8 {
    let mut i: u8 = 0;
    let mut safariPokeblock: *mut Pokeblock = null_mut();
    let mut natures: CArray<u8, 25> = zeroed();
    if GetSafariZoneFlag() == TRUE as u32 && Random() as i32 % 100 < 80 {
        safariPokeblock = SafariZoneGetActivePokeblock();
        if !safariPokeblock.is_null() {
            for i in 0..NUM_NATURES {
                natures[i] = i;
            }
            i = 0;
            while i < 24 {
                for j in (i + 1)..NUM_NATURES {
                    if Random() as i32 & 1 != 0 {
                        let temp: u8 = natures[i];
                        natures[i] = natures[j];
                        natures[j] = temp;
                    }
                }
                i += 1;
            }
            for i in 0..NUM_NATURES {
                if PokeblockGetGain(natures[i], safariPokeblock) > 0 {
                    return natures[i];
                }
            }
        }
    }
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0
        && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_SYNCHRONIZE
        && Random() as i32 % 2 == 0
    {
        return (GetMonData2(&raw mut gPlayerParty[0], MON_DATA_PERSONALITY) % 25) as u8;
    }
    (Random() as i32 % 25) as u8
}
unsafe fn CreateWildMon(species: u16, level: u8) {
    ZeroEnemyPartyMons();
    let mut checkCuteCharm: u32 = TRUE as u32;
    match (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
        [species]
        .genderRatio
    {
        MON_MALE | MON_FEMALE | MON_GENDERLESS => {
            checkCuteCharm = FALSE as u32;
        }
        _ => {}
    }
    if checkCuteCharm != 0
        && GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0
        && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_CUTE_CHARM
        && Random() as i32 % 3 != 0
    {
        let leadingMonSpecies: u16 = GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SPECIES) as u16;
        let leadingMonPersonality: u32 =
            GetMonData2(&raw mut gPlayerParty[0], MON_DATA_PERSONALITY);
        let mut gender: u8 =
            GetGenderFromSpeciesAndPersonality(leadingMonSpecies, leadingMonPersonality);
        if gender == MON_FEMALE {
            gender = MON_MALE;
        } else {
            gender = MON_FEMALE;
        }
        CreateMonWithGenderNatureLetter(
            &raw mut gEnemyParty[0],
            species,
            level,
            USE_RANDOM_IVS,
            gender,
            PickWildMonNature(),
            0,
        );
        return;
    }
    CreateMonWithNature(
        &raw mut gEnemyParty[0],
        species,
        level,
        USE_RANDOM_IVS,
        PickWildMonNature(),
    );
}
unsafe fn TryGenerateWildMon(wildMonInfo: *mut WildPokemonInfo, area: u8, flags: u8) -> u8 {
    let mut wildMonIndex: u8 = 0;
    'l1: {
        match area {
            WILD_AREA_LAND => {
                if TryGetAbilityInfluencedWildMonIndex(
                    (*wildMonInfo).wildPokemon,
                    TYPE_STEEL,
                    ABILITY_MAGNET_PULL,
                    &raw mut wildMonIndex,
                ) != 0
                {
                    break 'l1;
                }
                if TryGetAbilityInfluencedWildMonIndex(
                    (*wildMonInfo).wildPokemon,
                    TYPE_ELECTRIC,
                    ABILITY_STATIC,
                    &raw mut wildMonIndex,
                ) != 0
                {
                    break 'l1;
                }
                wildMonIndex = ChooseWildMonIndex_Land();
            }
            WILD_AREA_WATER => {
                if TryGetAbilityInfluencedWildMonIndex(
                    (*wildMonInfo).wildPokemon,
                    TYPE_ELECTRIC,
                    ABILITY_STATIC,
                    &raw mut wildMonIndex,
                ) != 0
                {
                    break 'l1;
                }
                wildMonIndex = ChooseWildMonIndex_WaterRock();
            }
            WILD_AREA_ROCKS => {
                wildMonIndex = ChooseWildMonIndex_WaterRock();
            }
            _ => {}
        }
    }
    let level: u8 = ChooseWildMonLevel((*wildMonInfo).wildPokemon.at(wildMonIndex));
    if flags as i32 & WILD_CHECK_REPEL != 0 && IsWildLevelAllowedByRepel(level) == 0 {
        return FALSE;
    }
    if gMapHeader.mapLayoutId != LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_ROOM_WILD_MONS
        && flags as i32 & WILD_CHECK_KEEN_EYE as i32 != 0
        && IsAbilityAllowingEncounter(level) == 0
    {
        return FALSE;
    }
    CreateWildMon(
        (*(*wildMonInfo).wildPokemon.at(wildMonIndex)).species,
        level,
    );
    TRUE
}
unsafe fn GenerateFishingWildMon(wildMonInfo: *mut WildPokemonInfo, rod: u8) -> u16 {
    let wildMonIndex: u8 = ChooseWildMonIndex_Fishing(rod);
    let level: u8 = ChooseWildMonLevel((*wildMonInfo).wildPokemon.at(wildMonIndex));
    CreateWildMon(
        (*(*wildMonInfo).wildPokemon.at(wildMonIndex)).species,
        level,
    );
    (*(*wildMonInfo).wildPokemon.at(wildMonIndex)).species
}
unsafe fn SetUpMassOutbreakEncounter(flags: u8) -> u8 {
    if flags as i32 & WILD_CHECK_REPEL != 0
        && IsWildLevelAllowedByRepel((*gSaveBlock1Ptr).outbreakPokemonLevel) == 0
    {
        return FALSE;
    }
    CreateWildMon(
        (*gSaveBlock1Ptr).outbreakPokemonSpecies,
        (*gSaveBlock1Ptr).outbreakPokemonLevel,
    );
    for i in 0..(MAX_MON_MOVES as u16) {
        SetMonMoveSlot(
            &raw mut gEnemyParty[0],
            (*gSaveBlock1Ptr).outbreakPokemonMoves[i],
            i as u8,
        );
    }
    TRUE
}
unsafe fn DoMassOutbreakEncounterTest() -> u8 {
    if (*gSaveBlock1Ptr).outbreakPokemonSpecies != SPECIES_NONE
        && (*gSaveBlock1Ptr).location.mapNum as i32
            == (*gSaveBlock1Ptr).outbreakLocationMapNum as i32
        && (*gSaveBlock1Ptr).location.mapGroup as i32
            == (*gSaveBlock1Ptr).outbreakLocationMapGroup as i32
        && Random() as i32 % 100 < (*gSaveBlock1Ptr).outbreakPokemonProbability as i32
    {
        return TRUE;
    }
    FALSE
}
fn EncounterOddsCheck(encounterRate: u16) -> u8 {
    if Random() as i32 % 2880 < encounterRate as i32 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn WildEncounterCheck(mut encounterRate: u32, ignoreAbility: u8) -> u8 {
    encounterRate *= 16;
    if TestPlayerAvatarFlags(6) != 0 {
        encounterRate = encounterRate * 80 / 100;
    }
    ApplyFluteEncounterRateMod(&raw mut encounterRate);
    ApplyCleanseTagEncounterRateMod(&raw mut encounterRate);
    if ignoreAbility == 0 && GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0 {
        let ability: u32 = GetMonAbility(&raw mut gPlayerParty[0]) as u32;
        if ability == ABILITY_STENCH
            && gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR
        {
            encounterRate = encounterRate * 3 / 4;
        } else if ability == ABILITY_STENCH {
            encounterRate /= 2;
        } else if ability == ABILITY_ILLUMINATE {
            encounterRate *= 2;
        } else if ability == ABILITY_WHITE_SMOKE as u32 {
            encounterRate /= 2;
        } else if ability == ABILITY_ARENA_TRAP as u32 {
            encounterRate *= 2;
        } else if ability == 8 && (*gSaveBlock1Ptr).weather == 8 {
            encounterRate /= 2;
        }
    }
    if encounterRate > MAX_ENCOUNTER_RATE {
        encounterRate = MAX_ENCOUNTER_RATE;
    }
    EncounterOddsCheck(encounterRate as u16)
}
fn AllowWildCheckOnNewMetatile() -> u8 {
    if Random() as i32 % 100 >= 60 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn AreLegendariesInSootopolisPreventingEncounters() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup != 0 || (*gSaveBlock1Ptr).location.mapNum != 7 {
        return FALSE;
    }
    FlagGet(FLAG_LEGENDARIES_IN_SOOTOPOLIS)
}
pub unsafe fn StandardWildEncounter(curMetatileBehavior: u16, prevMetatileBehavior: u16) -> u8 {
    let mut roamer: *mut Roamer = null_mut();
    if sWildEncountersDisabled.get() == TRUE {
        return FALSE;
    }
    let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId == HEADER_NONE {
        if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_ROOM_WILD_MONS {
            headerId = GetBattlePikeWildMonHeaderId() as u16;
            if prevMetatileBehavior != curMetatileBehavior && AllowWildCheckOnNewMetatile() == 0 {
                return FALSE;
            } else if WildEncounterCheck(
                (*gBattlePikeWildMonHeaders[headerId].landMonsInfo).encounterRate as u32,
                FALSE,
            ) != TRUE
            {
                return FALSE;
            } else if TryGenerateWildMon(
                gBattlePikeWildMonHeaders[headerId].landMonsInfo,
                WILD_AREA_LAND,
                WILD_CHECK_KEEN_EYE,
            ) != TRUE
            {
                return FALSE;
            } else if TryGenerateBattlePikeWildMon(TRUE) == 0 {
                return FALSE;
            }
            BattleSetup_StartBattlePikeWildBattle();
            return TRUE;
        }
        if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
            headerId = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum;
            if prevMetatileBehavior != curMetatileBehavior && AllowWildCheckOnNewMetatile() == 0 {
                return FALSE;
            } else if WildEncounterCheck(
                (*gBattlePyramidWildMonHeaders[headerId].landMonsInfo).encounterRate as u32,
                FALSE,
            ) != TRUE
            {
                return FALSE;
            } else if TryGenerateWildMon(
                gBattlePyramidWildMonHeaders[headerId].landMonsInfo,
                WILD_AREA_LAND,
                WILD_CHECK_KEEN_EYE,
            ) != TRUE
            {
                return FALSE;
            }
            GenerateBattlePyramidWildMon();
            BattleSetup_StartWildBattle();
            return TRUE;
        }
    } else {
        if MetatileBehavior_IsLandWildEncounter(curMetatileBehavior as u8) == TRUE {
            if gWildMonHeaders[headerId].landMonsInfo.is_null() {
                return FALSE;
            } else if prevMetatileBehavior != curMetatileBehavior
                && AllowWildCheckOnNewMetatile() == 0
            {
                return FALSE;
            } else if WildEncounterCheck(
                (*gWildMonHeaders[headerId].landMonsInfo).encounterRate as u32,
                FALSE,
            ) != TRUE
            {
                return FALSE;
            }
            if TryStartRoamerEncounter() == TRUE {
                roamer = &raw mut (*gSaveBlock1Ptr).roamer;
                if IsWildLevelAllowedByRepel((*roamer).level) == 0 {
                    return FALSE;
                }
                BattleSetup_StartRoamerBattle();
                return TRUE;
            } else {
                if DoMassOutbreakEncounterTest() == 1 && SetUpMassOutbreakEncounter(3) == 1 {
                    BattleSetup_StartWildBattle();
                    return TRUE;
                }
                if TryGenerateWildMon(gWildMonHeaders[headerId].landMonsInfo, WILD_AREA_LAND, 3)
                    == 1
                {
                    BattleSetup_StartWildBattle();
                    return TRUE;
                }
                return FALSE;
            }
        } else if MetatileBehavior_IsWaterWildEncounter(curMetatileBehavior as u8) == TRUE
            || TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0
                && MetatileBehavior_IsBridgeOverWater(curMetatileBehavior as u8) == TRUE
        {
            if AreLegendariesInSootopolisPreventingEncounters() == TRUE {
                return FALSE;
            } else if gWildMonHeaders[headerId].waterMonsInfo.is_null() {
                return FALSE;
            } else if prevMetatileBehavior != curMetatileBehavior
                && AllowWildCheckOnNewMetatile() == 0
            {
                return FALSE;
            } else if WildEncounterCheck(
                (*gWildMonHeaders[headerId].waterMonsInfo).encounterRate as u32,
                FALSE,
            ) != TRUE
            {
                return FALSE;
            }
            if TryStartRoamerEncounter() == TRUE {
                roamer = &raw mut (*gSaveBlock1Ptr).roamer;
                if IsWildLevelAllowedByRepel((*roamer).level) == 0 {
                    return FALSE;
                }
                BattleSetup_StartRoamerBattle();
                return TRUE;
            } else {
                if TryGenerateWildMon(gWildMonHeaders[headerId].waterMonsInfo, WILD_AREA_WATER, 3)
                    == 1
                {
                    BattleSetup_StartWildBattle();
                    return TRUE;
                }
                return FALSE;
            }
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn RockSmashWildEncounter() {
    let headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId != HEADER_NONE {
        let wildPokemonInfo: *mut WildPokemonInfo = gWildMonHeaders[headerId].rockSmashMonsInfo;
        if wildPokemonInfo.is_null() {
            gSpecialVar_Result = FALSE as u16;
        } else if WildEncounterCheck((*wildPokemonInfo).encounterRate as u32, TRUE) == TRUE
            && TryGenerateWildMon(wildPokemonInfo, WILD_AREA_ROCKS, 3) == 1
        {
            BattleSetup_StartWildBattle();
            gSpecialVar_Result = TRUE as u16;
        } else {
            gSpecialVar_Result = FALSE as u16;
        }
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SweetScentWildEncounter() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId == HEADER_NONE {
        if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_ROOM_WILD_MONS {
            headerId = GetBattlePikeWildMonHeaderId() as u16;
            if TryGenerateWildMon(
                gBattlePikeWildMonHeaders[headerId].landMonsInfo,
                WILD_AREA_LAND,
                0,
            ) != TRUE
            {
                return FALSE;
            }
            TryGenerateBattlePikeWildMon(FALSE);
            BattleSetup_StartBattlePikeWildBattle();
            return TRUE;
        }
        if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
            headerId = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum;
            if TryGenerateWildMon(
                gBattlePyramidWildMonHeaders[headerId].landMonsInfo,
                WILD_AREA_LAND,
                0,
            ) != TRUE
            {
                return FALSE;
            }
            GenerateBattlePyramidWildMon();
            BattleSetup_StartWildBattle();
            return TRUE;
        }
    } else {
        if MetatileBehavior_IsLandWildEncounter(
            MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8
        ) == TRUE
        {
            if gWildMonHeaders[headerId].landMonsInfo.is_null() {
                return FALSE;
            }
            if TryStartRoamerEncounter() == TRUE {
                BattleSetup_StartRoamerBattle();
                return TRUE;
            }
            if DoMassOutbreakEncounterTest() == TRUE {
                SetUpMassOutbreakEncounter(0);
            } else {
                TryGenerateWildMon(gWildMonHeaders[headerId].landMonsInfo, WILD_AREA_LAND, 0);
            }
            BattleSetup_StartWildBattle();
            return TRUE;
        } else if MetatileBehavior_IsWaterWildEncounter(MapGridGetMetatileBehaviorAt(
            x as i32, y as i32,
        ) as u8)
            == TRUE
        {
            if AreLegendariesInSootopolisPreventingEncounters() == TRUE {
                return FALSE;
            }
            if gWildMonHeaders[headerId].waterMonsInfo.is_null() {
                return FALSE;
            }
            if TryStartRoamerEncounter() == TRUE {
                BattleSetup_StartRoamerBattle();
                return TRUE;
            }
            TryGenerateWildMon(gWildMonHeaders[headerId].waterMonsInfo, WILD_AREA_WATER, 0);
            BattleSetup_StartWildBattle();
            return TRUE;
        }
    }
    FALSE
}
pub unsafe fn DoesCurrentMapHaveFishingMons() -> u8 {
    let headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId != HEADER_NONE && !gWildMonHeaders[headerId].fishingMonsInfo.is_null() {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn FishingWildEncounter(rod: u8) {
    let mut species: u16 = 0;
    if CheckFeebas() == TRUE {
        let level: u8 = ChooseWildMonLevel((&raw const *sWildFeebas).cast_mut());
        species = sWildFeebas.species;
        CreateWildMon(species, level);
    } else {
        species = GenerateFishingWildMon(
            gWildMonHeaders[GetCurrentMapWildMonHeaderId()].fishingMonsInfo,
            rod,
        );
    }
    IncrementGameStat(GAME_STAT_FISHING_ENCOUNTERS);
    SetPokemonAnglerSpecies(species);
    BattleSetup_StartWildBattle();
}
pub unsafe fn GetLocalWildMon(isWaterMon: *mut u8) -> u16 {
    let mut landMonsInfo: *mut WildPokemonInfo = null_mut();
    let mut waterMonsInfo: *mut WildPokemonInfo = null_mut();
    *isWaterMon = FALSE;
    let headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId == HEADER_NONE {
        return SPECIES_NONE;
    }
    landMonsInfo = gWildMonHeaders[headerId].landMonsInfo;
    waterMonsInfo = gWildMonHeaders[headerId].waterMonsInfo;
    if landMonsInfo.is_null() && waterMonsInfo.is_null() {
        return SPECIES_NONE;
    } else if !landMonsInfo.is_null() && waterMonsInfo.is_null() {
        return (*(*landMonsInfo).wildPokemon.at(ChooseWildMonIndex_Land())).species;
    } else if landMonsInfo.is_null() && !waterMonsInfo.is_null() {
        *isWaterMon = TRUE;
        return (*(*waterMonsInfo)
            .wildPokemon
            .at(ChooseWildMonIndex_WaterRock()))
        .species;
    }
    if Random() as i32 % 100 < 80 {
        return (*(*landMonsInfo).wildPokemon.at(ChooseWildMonIndex_Land())).species;
    } else {
        *isWaterMon = TRUE;
        return (*(*waterMonsInfo)
            .wildPokemon
            .at(ChooseWildMonIndex_WaterRock()))
        .species;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetLocalWaterMon() -> u16 {
    let headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId != HEADER_NONE {
        let waterMonsInfo: *mut WildPokemonInfo = gWildMonHeaders[headerId].waterMonsInfo;
        if !waterMonsInfo.is_null() {
            return (*(*waterMonsInfo)
                .wildPokemon
                .at(ChooseWildMonIndex_WaterRock()))
            .species;
        }
    }
    SPECIES_NONE
}
pub unsafe fn UpdateRepelCounter() -> u8 {
    if InBattlePike() != 0 || CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        return FALSE;
    }
    if InUnionRoom() == TRUE as u32 {
        return FALSE;
    }
    let mut steps: u16 = VarGet(VAR_REPEL_STEP_COUNT);
    if steps != 0 {
        steps -= 1;
        VarSet(VAR_REPEL_STEP_COUNT, steps);
        if steps == 0 {
            ScriptContext_SetupScript(
                (*crate::asmdata::EventScript_RepelWoreOff.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
    }
    FALSE
}
unsafe fn IsWildLevelAllowedByRepel(wildLevel: u8) -> u8 {
    if VarGet(VAR_REPEL_STEP_COUNT) == 0 {
        return TRUE;
    }
    for i in 0..(PARTY_SIZE as u8) {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) != 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
        {
            let ourLevel: u8 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) as u8;
            if wildLevel < ourLevel {
                return FALSE;
            } else {
                return TRUE;
            }
        }
    }
    FALSE
}
unsafe fn IsAbilityAllowingEncounter(level: u8) -> u8 {
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) != 0 {
        return TRUE;
    }
    let ability: u8 = GetMonAbility(&raw mut gPlayerParty[0]);
    if ability == ABILITY_KEEN_EYE || ability == ABILITY_INTIMIDATE {
        let playerMonLevel: u8 = GetMonData2(&raw mut gPlayerParty[0], MON_DATA_LEVEL) as u8;
        if playerMonLevel > 5
            && level as i32 <= playerMonLevel as i32 - 5
            && Random() as i32 % 2 == 0
        {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn TryGetRandomWildMonIndexByType(
    wildMon: *mut WildPokemon,
    r#type: u8,
    numMon: u8,
    monIndex: *mut u8,
) -> u8 {
    let mut validIndexes: CArray<u8, 256> = zeroed();
    let mut i: u8 = 0;
    while i < numMon {
        validIndexes[i] = 0;
        i += 1;
    }
    let mut validMonCount: u8 = 0;
    for i in 0..numMon {
        if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
            [(*wildMon.at(i)).species]
            .types[0]
            == r#type
            || (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*wildMon.at(i)).species]
                .types[1]
                == r#type
        {
            validIndexes[{
                let t1 = validMonCount;
                validMonCount += 1;
                t1
            }] = i;
        }
    }
    if validMonCount == 0 || validMonCount == numMon {
        return FALSE;
    }
    *monIndex = validIndexes[rem_i32(Random() as i32, validMonCount as i32)];
    TRUE
}
unsafe fn TryGetAbilityInfluencedWildMonIndex(
    wildMon: *mut WildPokemon,
    r#type: u8,
    ability: u8,
    monIndex: *mut u8,
) -> u8 {
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) != 0 {
        return FALSE;
    } else if GetMonAbility(&raw mut gPlayerParty[0]) != ability {
        return FALSE;
    } else if Random() as i32 % 2 != 0 {
        return FALSE;
    }
    TryGetRandomWildMonIndexByType(
        wildMon,
        r#type,
        NUM_LAND_MONS_ENCOUNTER_SLOTS as u8,
        monIndex,
    )
}
unsafe fn ApplyFluteEncounterRateMod(encRate: *mut u32) {
    if FlagGet(FLAG_SYS_ENC_UP_ITEM) == TRUE {
        *encRate += *encRate / 2;
    } else if FlagGet(FLAG_SYS_ENC_DOWN_ITEM) == TRUE {
        *encRate /= 2;
    }
}
unsafe fn ApplyCleanseTagEncounterRateMod(encRate: *mut u32) {
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_HELD_ITEM) == ITEM_CLEANSE_TAG {
        *encRate = *encRate * 2 / 3;
    }
}
