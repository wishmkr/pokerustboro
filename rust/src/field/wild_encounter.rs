//! Translated from `src/wild_encounter.c` by tools/rustport/c2rs.py.
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
pub(crate) static mut sWildEncountersDisabled: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFeebasRngValue: u32 = 0;

unsafe extern "C" {
    static EventScript_RepelWoreOff: CArray<u8, 0>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gMapHeader: MapHeader;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_Result: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    fn BattleSetup_StartBattlePikeWildBattle();
    fn BattleSetup_StartRoamerBattle();
    fn BattleSetup_StartWildBattle();
    fn CreateMonWithGenderNatureLetter(
        a0: *mut Pokemon,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CreateMonWithNature(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8);
    fn CurrentBattlePyramidLocation() -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn GenerateBattlePyramidWildMon();
    fn GetBattlePikeWildMonHeaderId() -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetMonAbility(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
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
    fn PokeblockGetGain(a0: u8, a1: *mut Pokeblock) -> i16;
    fn Random() -> u16;
    fn SafariZoneGetActivePokeblock() -> *mut Pokeblock;
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
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
    sWildEncountersDisabled = disabled;
}
pub(crate) unsafe extern "C" fn GetFeebasFishingSpotId(
    targetX: i16,
    targetY: i16,
    section: u8,
) -> u16 {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut yMin: u16 = sRoute119WaterTileData[section as i32 * 3 + 0];
    let mut yMax: u16 = sRoute119WaterTileData[section as i32 * 3 + 1];
    let mut spotId: u16 = sRoute119WaterTileData[section as i32 * 3 + 2];
    y = yMin;
    while y <= yMax {
        x = 0;
        while (x as i32) < (*gMapHeader.mapLayout).width {
            let mut behavior: u8 =
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
    return spotId + 1;
}
pub(crate) unsafe extern "C" fn CheckFeebas() -> u8 {
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
        i = 0;
        while i < NUM_FEEBAS_SPOTS {
            if spotId == feebasSpots[i] {
                return TRUE;
            }
            i += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FeebasRandom() -> u16 {
    sFeebasRngValue = 0x41c64e6d * sFeebasRngValue + 12345;
    return (sFeebasRngValue >> 16) as u16;
}
pub(crate) unsafe extern "C" fn FeebasSeedRng(seed: u16) {
    sFeebasRngValue = seed as u32;
}
pub(crate) unsafe extern "C" fn ChooseWildMonIndex_Land() -> u8 {
    let mut rand: u8 = (Random() as i32 % 100) as u8;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ChooseWildMonIndex_WaterRock() -> u8 {
    let mut rand: u8 = (Random() as i32 % 100) as u8;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ChooseWildMonIndex_Fishing(rod: u8) -> u8 {
    let mut wildMonIndex: u8 = 0;
    let mut rand: u8 = rem_i32(
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
            if rand >= ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_2
                && rand < ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_3
            {
                wildMonIndex = 3;
            }
            if rand >= ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_3
                && rand < ENCOUNTER_CHANCE_FISHING_MONS_GOOD_ROD_SLOT_4
            {
                wildMonIndex = 4;
            }
        }
        SUPER_ROD => {
            if rand < ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_5 {
                wildMonIndex = 5;
            }
            if rand >= ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_5
                && rand < ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_6
            {
                wildMonIndex = 6;
            }
            if rand >= ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_6
                && rand < ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_7
            {
                wildMonIndex = 7;
            }
            if rand >= ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_7
                && rand < ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_8
            {
                wildMonIndex = 8;
            }
            if rand >= ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_8
                && rand < ENCOUNTER_CHANCE_FISHING_MONS_SUPER_ROD_SLOT_9
            {
                wildMonIndex = 9;
            }
        }
        _ => {}
    }
    return wildMonIndex;
}
pub(crate) unsafe extern "C" fn ChooseWildMonLevel(wildPokemon: *mut WildPokemon) -> u8 {
    let mut min: u8 = 0;
    let mut max: u8 = 0;
    let mut range: u8 = 0;
    let mut rand: u8 = 0;
    if (*wildPokemon).maxLevel >= (*wildPokemon).minLevel {
        min = (*wildPokemon).minLevel;
        max = (*wildPokemon).maxLevel;
    } else {
        min = (*wildPokemon).maxLevel;
        max = (*wildPokemon).minLevel;
    }
    range = max - min + 1;
    rand = rem_i32(Random() as i32, range as i32) as u8;
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0 {
        let mut ability: u8 = GetMonAbility(&raw mut gPlayerParty[0]);
        if ability == ABILITY_HUSTLE
            || ability == ABILITY_VITAL_SPIRIT
            || ability == ABILITY_PRESSURE
        {
            if Random() as i32 % 2 == 0 {
                return max;
            }
            if rand != 0 {
                rand -= 1;
            }
        }
    }
    return min + rand;
}
pub(crate) unsafe extern "C" fn GetCurrentMapWildMonHeaderId() -> u16 {
    let mut i: u16 = 0;
    i = 0;
    loop {
        let mut wildHeader: *mut WildPokemonHeader = (&raw const gWildMonHeaders[i]).cast_mut();
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
    return HEADER_NONE;
}
pub(crate) unsafe extern "C" fn PickWildMonNature() -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut safariPokeblock: *mut Pokeblock = null_mut();
    let mut natures: CArray<u8, 25> = zeroed();
    if GetSafariZoneFlag() == TRUE as u32 && Random() as i32 % 100 < 80 {
        safariPokeblock = SafariZoneGetActivePokeblock();
        if !safariPokeblock.is_null() {
            i = 0;
            while i < NUM_NATURES {
                natures[i] = i;
                i += 1;
            }
            i = 0;
            while i < 24 {
                j = i + 1;
                while j < NUM_NATURES {
                    if Random() as i32 & 1 != 0 {
                        let mut temp: u8 = 0;
                        temp = natures[i];
                        natures[i] = natures[j];
                        natures[j] = temp;
                    }
                    j += 1;
                }
                i += 1;
            }
            i = 0;
            while i < NUM_NATURES {
                if PokeblockGetGain(natures[i], safariPokeblock) > 0 {
                    return natures[i];
                }
                i += 1;
            }
        }
    }
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0
        && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_SYNCHRONIZE
        && Random() as i32 % 2 == 0
    {
        return (GetMonData2(&raw mut gPlayerParty[0], MON_DATA_PERSONALITY) % 25) as u8;
    }
    return (Random() as i32 % 25) as u8;
}
pub(crate) unsafe extern "C" fn CreateWildMon(species: u16, level: u8) {
    let mut checkCuteCharm: u32 = 0;
    ZeroEnemyPartyMons();
    checkCuteCharm = TRUE as u32;
    match gSpeciesInfo[species].genderRatio {
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
        let mut leadingMonSpecies: u16 =
            GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SPECIES) as u16;
        let mut leadingMonPersonality: u32 =
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
pub(crate) unsafe extern "C" fn TryGenerateWildMon(
    wildMonInfo: *mut WildPokemonInfo,
    area: u8,
    flags: u8,
) -> u8 {
    let mut wildMonIndex: u8 = 0;
    let mut level: u8 = 0;
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
    level = ChooseWildMonLevel((*wildMonInfo).wildPokemon.at(wildMonIndex));
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn GenerateFishingWildMon(
    wildMonInfo: *mut WildPokemonInfo,
    rod: u8,
) -> u16 {
    let mut wildMonIndex: u8 = ChooseWildMonIndex_Fishing(rod);
    let mut level: u8 = ChooseWildMonLevel((*wildMonInfo).wildPokemon.at(wildMonIndex));
    CreateWildMon(
        (*(*wildMonInfo).wildPokemon.at(wildMonIndex)).species,
        level,
    );
    return (*(*wildMonInfo).wildPokemon.at(wildMonIndex)).species;
}
pub(crate) unsafe extern "C" fn SetUpMassOutbreakEncounter(flags: u8) -> u8 {
    let mut i: u16 = 0;
    if flags as i32 & WILD_CHECK_REPEL != 0
        && IsWildLevelAllowedByRepel((*gSaveBlock1Ptr).outbreakPokemonLevel) == 0
    {
        return FALSE;
    }
    CreateWildMon(
        (*gSaveBlock1Ptr).outbreakPokemonSpecies,
        (*gSaveBlock1Ptr).outbreakPokemonLevel,
    );
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        SetMonMoveSlot(
            &raw mut gEnemyParty[0],
            (*gSaveBlock1Ptr).outbreakPokemonMoves[i],
            i as u8,
        );
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn DoMassOutbreakEncounterTest() -> u8 {
    if (*gSaveBlock1Ptr).outbreakPokemonSpecies != SPECIES_NONE
        && (*gSaveBlock1Ptr).location.mapNum as i32
            == (*gSaveBlock1Ptr).outbreakLocationMapNum as i32
        && (*gSaveBlock1Ptr).location.mapGroup as i32
            == (*gSaveBlock1Ptr).outbreakLocationMapGroup as i32
    {
        if Random() as i32 % 100 < (*gSaveBlock1Ptr).outbreakPokemonProbability as i32 {
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn EncounterOddsCheck(encounterRate: u16) -> u8 {
    if Random() as i32 % 2880 < encounterRate as i32 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn WildEncounterCheck(
    mut encounterRate: u32,
    ignoreAbility: u8,
) -> u8 {
    encounterRate *= 16;
    if TestPlayerAvatarFlags(6) != 0 {
        encounterRate = encounterRate * 80 / 100;
    }
    ApplyFluteEncounterRateMod(&raw mut encounterRate);
    ApplyCleanseTagEncounterRateMod(&raw mut encounterRate);
    if ignoreAbility == 0 && GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0 {
        let mut ability: u32 = GetMonAbility(&raw mut gPlayerParty[0]) as u32;
        if ability == ABILITY_STENCH
            && gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR
        {
            encounterRate = encounterRate * 3 / 4;
        } else if ability == ABILITY_STENCH {
            encounterRate = encounterRate / 2;
        } else if ability == ABILITY_ILLUMINATE {
            encounterRate *= 2;
        } else if ability == ABILITY_WHITE_SMOKE as u32 {
            encounterRate = encounterRate / 2;
        } else if ability == ABILITY_ARENA_TRAP as u32 {
            encounterRate *= 2;
        } else if ability == 8 && (*gSaveBlock1Ptr).weather == 8 {
            encounterRate = encounterRate / 2;
        }
    }
    if encounterRate > MAX_ENCOUNTER_RATE {
        encounterRate = MAX_ENCOUNTER_RATE;
    }
    return EncounterOddsCheck(encounterRate as u16);
}
pub(crate) unsafe extern "C" fn AllowWildCheckOnNewMetatile() -> u8 {
    if Random() as i32 % 100 >= 60 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AreLegendariesInSootopolisPreventingEncounters() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup != 0 || (*gSaveBlock1Ptr).location.mapNum != 7 {
        return FALSE;
    }
    return FlagGet(FLAG_LEGENDARIES_IN_SOOTOPOLIS);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StandardWildEncounter(
    curMetatileBehavior: u16,
    prevMetatileBehavior: u16,
) -> u8 {
    let mut headerId: u16 = 0;
    let mut roamer: *mut Roamer = null_mut();
    if sWildEncountersDisabled == TRUE {
        return FALSE;
    }
    headerId = GetCurrentMapWildMonHeaderId();
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
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RockSmashWildEncounter() {
    let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId != HEADER_NONE {
        let mut wildPokemonInfo: *mut WildPokemonInfo = gWildMonHeaders[headerId].rockSmashMonsInfo;
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
pub unsafe extern "C" fn SweetScentWildEncounter() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut headerId: u16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    headerId = GetCurrentMapWildMonHeaderId();
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
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesCurrentMapHaveFishingMons() -> u8 {
    let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId != HEADER_NONE && !gWildMonHeaders[headerId].fishingMonsInfo.is_null() {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FishingWildEncounter(rod: u8) {
    let mut species: u16 = 0;
    if CheckFeebas() == TRUE {
        let mut level: u8 = ChooseWildMonLevel((&raw const *sWildFeebas).cast_mut());
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocalWildMon(isWaterMon: *mut u8) -> u16 {
    let mut headerId: u16 = 0;
    let mut landMonsInfo: *mut WildPokemonInfo = null_mut();
    let mut waterMonsInfo: *mut WildPokemonInfo = null_mut();
    *isWaterMon = FALSE;
    headerId = GetCurrentMapWildMonHeaderId();
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocalWaterMon() -> u16 {
    let mut headerId: u16 = GetCurrentMapWildMonHeaderId();
    if headerId != HEADER_NONE {
        let mut waterMonsInfo: *mut WildPokemonInfo = gWildMonHeaders[headerId].waterMonsInfo;
        if !waterMonsInfo.is_null() {
            return (*(*waterMonsInfo)
                .wildPokemon
                .at(ChooseWildMonIndex_WaterRock()))
            .species;
        }
    }
    return SPECIES_NONE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRepelCounter() -> u8 {
    let mut steps: u16 = 0;
    if InBattlePike() != 0 || CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        return FALSE;
    }
    if InUnionRoom() == TRUE as u32 {
        return FALSE;
    }
    steps = VarGet(VAR_REPEL_STEP_COUNT);
    if steps != 0 {
        steps -= 1;
        VarSet(VAR_REPEL_STEP_COUNT, steps);
        if steps == 0 {
            ScriptContext_SetupScript(EventScript_RepelWoreOff.as_ptr().cast_mut());
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsWildLevelAllowedByRepel(wildLevel: u8) -> u8 {
    let mut i: u8 = 0;
    if VarGet(VAR_REPEL_STEP_COUNT) == 0 {
        return TRUE;
    }
    i = 0;
    while i < PARTY_SIZE as u8 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) != 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
        {
            let mut ourLevel: u8 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) as u8;
            if wildLevel < ourLevel {
                return FALSE;
            } else {
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsAbilityAllowingEncounter(level: u8) -> u8 {
    let mut ability: u8 = 0;
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) != 0 {
        return TRUE;
    }
    ability = GetMonAbility(&raw mut gPlayerParty[0]);
    if ability == ABILITY_KEEN_EYE || ability == ABILITY_INTIMIDATE {
        let mut playerMonLevel: u8 = GetMonData2(&raw mut gPlayerParty[0], MON_DATA_LEVEL) as u8;
        if playerMonLevel > 5
            && level as i32 <= playerMonLevel as i32 - 5
            && Random() as i32 % 2 == 0
        {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryGetRandomWildMonIndexByType(
    wildMon: *mut WildPokemon,
    r#type: u8,
    numMon: u8,
    monIndex: *mut u8,
) -> u8 {
    let mut validIndexes: CArray<u8, 256> = zeroed();
    let mut i: u8 = 0;
    let mut validMonCount: u8 = 0;
    i = 0;
    while i < numMon {
        validIndexes[i] = 0;
        i += 1;
    }
    validMonCount = 0;
    i = 0;
    while i < numMon {
        if gSpeciesInfo[(*wildMon.at(i)).species].types[0] == r#type
            || gSpeciesInfo[(*wildMon.at(i)).species].types[1] == r#type
        {
            validIndexes[{
                let t1 = validMonCount;
                validMonCount += 1;
                t1
            }] = i;
        }
        i += 1;
    }
    if validMonCount == 0 || validMonCount == numMon {
        return FALSE;
    }
    *monIndex = validIndexes[rem_i32(Random() as i32, validMonCount as i32)];
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryGetAbilityInfluencedWildMonIndex(
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
    return TryGetRandomWildMonIndexByType(
        wildMon,
        r#type,
        NUM_LAND_MONS_ENCOUNTER_SLOTS as u8,
        monIndex,
    );
}
pub(crate) unsafe extern "C" fn ApplyFluteEncounterRateMod(encRate: *mut u32) {
    if FlagGet(FLAG_SYS_ENC_UP_ITEM) == TRUE {
        *encRate += *encRate / 2;
    } else if FlagGet(FLAG_SYS_ENC_DOWN_ITEM) == TRUE {
        *encRate = *encRate / 2;
    }
}
pub(crate) unsafe extern "C" fn ApplyCleanseTagEncounterRateMod(encRate: *mut u32) {
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_HELD_ITEM) == ITEM_CLEANSE_TAG {
        *encRate = *encRate * 2 / 3;
    }
}
