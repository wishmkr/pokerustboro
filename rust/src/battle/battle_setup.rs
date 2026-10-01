//! Translated from `src/battle_setup.c` by tools/rustport/c2rs.py.
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
    clippy::manual_is_multiple_of,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gGameVersion;
use crate::agb_main::gMain;
use crate::battle_main::{CB2_InitBattle, gBattleOutcome, gBattleTypeFlags};
use crate::battle_pike::InBattlePike;
use crate::battle_pyramid::{
    CopyPyramidTrainerSpeechBefore, CurrentBattlePyramidLocation, GetBattlePyramidTrainerFlag,
    LocalIdToPyramidTrainerId, MarkApproachingPyramidTrainersAsBattled,
};
use crate::battle_tower::{FillFrontierTrainerParty, FillFrontierTrainersParties};
use crate::battle_transition::{
    BattleTransition_Start, BattleTransition_StartOnField, IsBattleTransitionDone,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagClear, FlagGet, FlagSet, GetVarPointer, VarGet, VarSet};
use crate::event_object_movement::{
    FreezeObjectEvents, GetObjectEventIdByLocalIdAndMap, GetTrainerFacingDirectionMovementType,
    SetTrainerMovementType,
};
use crate::ffi::{gSpecialVar_LastTalked, gSpecialVar_Result};
use crate::field_control_avatar::{
    ClearPoisonStepCounter, RestartWildEncounterImmunitySteps, gSelectedObjectEvent,
};
use crate::field_message_box::ShowFieldMessage;
use crate::field_message_box::ShowFieldMessageFromBuffer;
use crate::field_player_avatar::{
    PlayerGetDestCoords, StopPlayerAvatar, TestPlayerAvatarFlags, gObjectEvents,
};
use crate::field_screen_effect::FieldCB_ReturnToFieldNoScriptCheckMusic;
use crate::field_weather_effect::GetSavedWeather;
use crate::fieldmap::{MapGridGetMetatileBehaviorAt, gMapHeader};
use crate::fldeff_misc::FldEffPoison_IsActive;
use crate::gym_leader_rematch::UpdateGymLeaderRematch;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::metatile_behavior::{
    MetatileBehavior_GetBridgeType, MetatileBehavior_IsBridgeOverWater,
    MetatileBehavior_IsDeepOrOceanWater, MetatileBehavior_IsIndoorEncounter,
    MetatileBehavior_IsLongGrass, MetatileBehavior_IsMountain, MetatileBehavior_IsSandOrDeepSand,
    MetatileBehavior_IsSurfableWaterOrUnderwater, MetatileBehavior_IsTallGrass,
};
use crate::mirage_tower::ClearMirageTowerPulseBlendEffect;
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldContinueScriptPlayMapMusic, CB2_WhiteOut,
    CleanupOverworldWindowsAndTilemaps, GetFlashLevel, GetGameStat, IncrementGameStat,
    Overworld_ClearSavedMusic, gFieldCallback,
};
use crate::palette::UpdatePaletteFade;
use crate::pokemon::{
    CreateMaleMon, GetMonData2, GetMonData3, GetTrainerEncounterMusicId, PlayBattleBGM,
    PlayMapChosenOrBattleBGM, ZeroMonData, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::safari_zone::{CB2_EndSafariBattle, GetSafariZoneFlag};
use crate::script::ScriptContext_SetupScript;
use crate::script::{LockPlayerFieldControls, ScriptContext_Stop};
use crate::script_pokemon_util::ScriptGiveMon;
use crate::secret_base::GetSecretBaseTrainerLoseText;
use crate::sound::PlayNewMapMusic;
use crate::sprite::ResetOamRange;
use crate::starter_choose::{CB2_ChooseStarter, GetStarterPokemon};
use crate::string_util::StringExpandPlaceholders;
use crate::string_util::gStringVar4;
use crate::task::gTasks;
use crate::task::task_set;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::trainer_hill::{
    CopyTrainerHillTrainerText, FillHillTrainerParty, FillHillTrainersParties, GetHillTrainerFlag,
    InTrainerHill, InTrainerHillChallenge, LocalIdToHillTrainerId, SetHillTrainerFlag,
};
use crate::trainer_see::{
    gApproachingTrainerId, gApproachingTrainers, gNoOfApproachingTrainers,
    gWhichTrainerToFaceAfterBattle,
};
use crate::tv::IncrementDailyWildBattles;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::FreeAllWindowBuffers;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
// The C's names for task and sprite data slots.
const tTransition: usize = 1;
// Data tables (translate with cdata.py): sBattleTransitionTable_Wild sBattleTransitionTable_Trainer sBattleTransitionTable_BattleFrontier sBattleTransitionTable_BattlePyramid sBattleTransitionTable_BattleDome sOrdinaryBattleParams sContinueScriptBattleParams sDoubleBattleParams sOrdinaryNoIntroBattleParams sContinueScriptDoubleBattleParams sTrainerBOrdinaryBattleParams sTrainerBContinueScriptBattleParams gRematchTable sBadgeFlags

/// `struct TrainerBattleParameter`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrainerBattleParameter {
    pub varPtr: *mut core::ffi::c_void,
    pub ptrType: u8,
}

unsafe impl Sync for TrainerBattleParameter {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<TrainerBattleParameter>() == 8);
    assert!(offset_of!(TrainerBattleParameter, varPtr) == 0);
    assert!(offset_of!(TrainerBattleParameter, ptrType) == 4);
};

const STEP_COUNTER_MAX: u16 = 255;
const TRAINER_PARAM_CLEAR_VAL_16BIT: u8 = 4;
const TRAINER_PARAM_CLEAR_VAL_32BIT: u8 = 5;
const TRAINER_PARAM_CLEAR_VAL_8BIT: u8 = 3;
const TRAINER_PARAM_LOAD_SCRIPT_RET_ADDR: u8 = 6;
const TRAINER_PARAM_LOAD_VAL_16BIT: u8 = 1;
const TRAINER_PARAM_LOAD_VAL_32BIT: u8 = 2;
const TRAINER_PARAM_LOAD_VAL_8BIT: u8 = 0;
const TRANSITION_TYPE_CAVE: u8 = 1;
const TRANSITION_TYPE_FLASH: u8 = 2;
const TRANSITION_TYPE_NORMAL: u8 = 0;
const TRANSITION_TYPE_WATER: u8 = 3;

pub(crate) static gRematchTable: Table<CArray<RematchTrainer, 78>> =
    Table((&raw const crate::data::battle_setup::gRematchTable).cast());
static sBadgeFlags: Table<CArray<u16, 8>> =
    Table((&raw const crate::data::battle_setup::sBadgeFlags).cast());
static sBattleTransitionTable_BattleDome: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_setup::sBattleTransitionTable_BattleDome).cast());
static sBattleTransitionTable_BattleFrontier: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::battle_setup::sBattleTransitionTable_BattleFrontier).cast());
static sBattleTransitionTable_BattlePyramid: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_setup::sBattleTransitionTable_BattlePyramid).cast());
static sBattleTransitionTable_Trainer: Table<CArray<CArray<u8, 2>, 4>> =
    Table((&raw const crate::data::battle_setup::sBattleTransitionTable_Trainer).cast());
static sBattleTransitionTable_Wild: Table<CArray<CArray<u8, 2>, 4>> =
    Table((&raw const crate::data::battle_setup::sBattleTransitionTable_Wild).cast());
static sContinueScriptBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sContinueScriptBattleParams).cast());
static sContinueScriptDoubleBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sContinueScriptDoubleBattleParams).cast());
static sDoubleBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sDoubleBattleParams).cast());
static sOrdinaryBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sOrdinaryBattleParams).cast());
static sOrdinaryNoIntroBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sOrdinaryNoIntroBattleParams).cast());
static sTrainerBContinueScriptBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sTrainerBContinueScriptBattleParams).cast());
static sTrainerBOrdinaryBattleParams: Table<CArray<TrainerBattleParameter, 9>> =
    Table((&raw const crate::data::battle_setup::sTrainerBOrdinaryBattleParams).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sTrainerBattleMode: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerBattleOpponent_A: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerBattleOpponent_B: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gPartnerTrainerId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerObjectEventLocalId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerAIntroSpeech: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBIntroSpeech: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerADefeatSpeech: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBDefeatSpeech: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerVictorySpeech: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerCannotBattleSpeech: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBattleEndScript: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerABattleScriptRetAddr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBBattleScriptRetAddr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sShouldCheckTrainerBScript: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNoOfPossibleTrainerRetScripts: u8 = 0;

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn Task_BattleStart(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if FldEffPoison_IsActive() == 0 {
                BattleTransition_StartOnField(*data.at(1) as u8);
                ClearMirageTowerPulseBlendEffect();
                *data += 1;
            }
        }
        1 if IsBattleTransitionDone() == TRUE => {
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_InitBattle));
            RestartWildEncounterImmunitySteps();
            ClearPoisonStepCounter();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
unsafe fn CreateBattleStartTask(transition: u8, song: u16) {
    let taskId: u8 = CreateTask(Some(Task_BattleStart), 1);
    task_set(taskId, tTransition, transition as i16);
    PlayMapChosenOrBattleBGM(song);
}
pub unsafe fn BattleSetup_StartWildBattle() {
    if GetSafariZoneFlag() != 0 {
        DoSafariBattle();
    } else {
        DoStandardWildBattle();
    }
}
pub unsafe fn BattleSetup_StartBattlePikeWildBattle() {
    DoBattlePikeWildBattle();
}
unsafe fn DoStandardWildBattle() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    StopPlayerAvatar();
    gMain.savedCallback = Some(CB2_EndWildBattle);
    gBattleTypeFlags = 0;
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        VarSet(VAR_TEMP_PLAYING_PYRAMID_MUSIC, 0);
        gBattleTypeFlags |= BATTLE_TYPE_PYRAMID;
    }
    CreateBattleStartTask(GetWildBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
pub unsafe fn BattleSetup_StartRoamerBattle() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    StopPlayerAvatar();
    gMain.savedCallback = Some(CB2_EndWildBattle);
    gBattleTypeFlags = BATTLE_TYPE_ROAMER;
    CreateBattleStartTask(GetWildBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
unsafe fn DoSafariBattle() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    StopPlayerAvatar();
    gMain.savedCallback = Some(CB2_EndSafariBattle);
    gBattleTypeFlags = BATTLE_TYPE_SAFARI;
    CreateBattleStartTask(GetWildBattleTransition(), 0);
}
unsafe fn DoBattlePikeWildBattle() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    StopPlayerAvatar();
    gMain.savedCallback = Some(CB2_EndWildBattle);
    gBattleTypeFlags = BATTLE_TYPE_PIKE;
    CreateBattleStartTask(GetWildBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
unsafe fn DoTrainerBattle() {
    CreateBattleStartTask(GetTrainerBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_TRAINER_BATTLES);
    TryUpdateGymLeaderRematchFromTrainer();
}
unsafe fn DoBattlePyramidTrainerHillBattle() {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        CreateBattleStartTask(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_PYRAMID), 0);
    } else {
        CreateBattleStartTask(
            GetSpecialBattleTransition(B_TRANSITION_GROUP_TRAINER_HILL),
            0,
        );
    }
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_TRAINER_BATTLES);
    TryUpdateGymLeaderRematchFromTrainer();
}
#[unsafe(no_mangle)]
pub unsafe fn StartWallyTutorialBattle() {
    CreateMaleMon(&raw mut gEnemyParty[0], SPECIES_RALTS, 5);
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_ReturnToFieldContinueScriptPlayMapMusic);
    gBattleTypeFlags = BATTLE_TYPE_WALLY_TUTORIAL;
    CreateBattleStartTask(B_TRANSITION_SLICE, 0);
}
pub unsafe fn BattleSetup_StartScriptedWildBattle() {
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_EndScriptedWildBattle);
    gBattleTypeFlags = 0;
    CreateBattleStartTask(GetWildBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
#[unsafe(no_mangle)]
pub unsafe fn BattleSetup_StartLatiBattle() {
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_EndScriptedWildBattle);
    gBattleTypeFlags = BATTLE_TYPE_LEGENDARY;
    CreateBattleStartTask(GetWildBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
#[unsafe(no_mangle)]
pub unsafe fn BattleSetup_StartLegendaryBattle() {
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_EndScriptedWildBattle);
    gBattleTypeFlags = BATTLE_TYPE_LEGENDARY;
    match GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) {
        SPECIES_KYOGRE => {
            gBattleTypeFlags |= BATTLE_TYPE_KYOGRE;
            CreateBattleStartTask(B_TRANSITION_KYOGRE, MUS_VS_KYOGRE_GROUDON);
        }
        SPECIES_RAYQUAZA => {
            gBattleTypeFlags |= BATTLE_TYPE_RAYQUAZA;
            CreateBattleStartTask(B_TRANSITION_RAYQUAZA, MUS_VS_RAYQUAZA);
        }
        SPECIES_DEOXYS => {
            CreateBattleStartTask(B_TRANSITION_BLUR, MUS_RG_VS_DEOXYS);
        }
        SPECIES_LUGIA | SPECIES_HO_OH => {
            CreateBattleStartTask(B_TRANSITION_BLUR, MUS_RG_VS_LEGEND);
        }
        SPECIES_MEW => {
            CreateBattleStartTask(B_TRANSITION_GRID_SQUARES, MUS_VS_MEW);
        }
        _ => {
            gBattleTypeFlags |= BATTLE_TYPE_GROUDON;
            CreateBattleStartTask(B_TRANSITION_GROUDON, MUS_VS_KYOGRE_GROUDON);
        }
    }
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
#[unsafe(no_mangle)]
pub unsafe fn StartGroudonKyogreBattle() {
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_EndScriptedWildBattle);
    gBattleTypeFlags = 12288;
    if gGameVersion == VERSION_RUBY as u8 {
        CreateBattleStartTask(B_TRANSITION_ANGLED_WIPES, MUS_VS_KYOGRE_GROUDON);
    } else {
        CreateBattleStartTask(B_TRANSITION_RIPPLE, MUS_VS_KYOGRE_GROUDON);
    }
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
#[unsafe(no_mangle)]
pub unsafe fn StartRegiBattle() {
    let mut transitionId: u8 = 0;
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_EndScriptedWildBattle);
    gBattleTypeFlags = 24576;
    let species: u16 = GetMonData2(&raw mut gEnemyParty[0], MON_DATA_SPECIES) as u16;
    match species {
        SPECIES_REGIROCK => {
            transitionId = B_TRANSITION_REGIROCK;
        }
        SPECIES_REGICE => {
            transitionId = B_TRANSITION_REGICE;
        }
        SPECIES_REGISTEEL => {
            transitionId = B_TRANSITION_REGISTEEL;
        }
        _ => {
            transitionId = B_TRANSITION_GRID_SQUARES;
        }
    }
    CreateBattleStartTask(transitionId, MUS_VS_REGI);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_WILD_BATTLES);
    IncrementDailyWildBattles();
    TryUpdateGymLeaderRematchFromWild();
}
pub(crate) unsafe fn CB2_EndWildBattle() {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                BG_PLTT as i32 as usize as *mut c_void,
                0x1000100,
            );
        }
    }
    ResetOamRange(0, 128);
    if IsPlayerDefeated(gBattleOutcome as u32) == TRUE as u32
        && CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE
        && InBattlePike() == 0
    {
        SetMainCallback2(Some(CB2_WhiteOut));
    } else {
        SetMainCallback2(Some(CB2_ReturnToField));
        gFieldCallback = Some(FieldCB_ReturnToFieldNoScriptCheckMusic);
    }
}
pub(crate) unsafe fn CB2_EndScriptedWildBattle() {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                BG_PLTT as i32 as usize as *mut c_void,
                0x1000100,
            );
        }
    }
    ResetOamRange(0, 128);
    if IsPlayerDefeated(gBattleOutcome as u32) == TRUE as u32 {
        if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        } else {
            SetMainCallback2(Some(CB2_WhiteOut));
        }
    } else {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
pub unsafe fn BattleSetup_GetEnvironmentId() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    let tileBehavior: u16 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
    if MetatileBehavior_IsTallGrass(tileBehavior as u8) != 0 {
        return BATTLE_ENVIRONMENT_GRASS;
    }
    if MetatileBehavior_IsLongGrass(tileBehavior as u8) != 0 {
        return BATTLE_ENVIRONMENT_LONG_GRASS;
    }
    if MetatileBehavior_IsSandOrDeepSand(tileBehavior as u8) != 0 {
        return BATTLE_ENVIRONMENT_SAND;
    }
    match gMapHeader.mapType {
        MAP_TYPE_TOWN | MAP_TYPE_CITY | MAP_TYPE_ROUTE => {}
        MAP_TYPE_UNDERGROUND => {
            if MetatileBehavior_IsIndoorEncounter(tileBehavior as u8) != 0 {
                return BATTLE_ENVIRONMENT_BUILDING;
            }
            if MetatileBehavior_IsSurfableWaterOrUnderwater(tileBehavior as u8) != 0 {
                return BATTLE_ENVIRONMENT_POND;
            }
            return BATTLE_ENVIRONMENT_CAVE;
        }
        MAP_TYPE_INDOOR | MAP_TYPE_SECRET_BASE => {
            return BATTLE_ENVIRONMENT_BUILDING;
        }
        MAP_TYPE_UNDERWATER => {
            return BATTLE_ENVIRONMENT_UNDERWATER;
        }
        MAP_TYPE_OCEAN_ROUTE => {
            if MetatileBehavior_IsSurfableWaterOrUnderwater(tileBehavior as u8) != 0 {
                return BATTLE_ENVIRONMENT_WATER;
            }
            return BATTLE_ENVIRONMENT_PLAIN;
        }
        _ => {}
    }
    if MetatileBehavior_IsDeepOrOceanWater(tileBehavior as u8) != 0 {
        return BATTLE_ENVIRONMENT_WATER;
    }
    if MetatileBehavior_IsSurfableWaterOrUnderwater(tileBehavior as u8) != 0 {
        return BATTLE_ENVIRONMENT_POND;
    }
    if MetatileBehavior_IsMountain(tileBehavior as u8) != 0 {
        return BATTLE_ENVIRONMENT_MOUNTAIN;
    }
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
        if MetatileBehavior_GetBridgeType(tileBehavior as u8) != BRIDGE_TYPE_OCEAN {
            return BATTLE_ENVIRONMENT_POND;
        }
        if MetatileBehavior_IsBridgeOverWater(tileBehavior as u8) == TRUE {
            return BATTLE_ENVIRONMENT_WATER;
        }
    }
    if (*gSaveBlock1Ptr).location.mapGroup == 0 && (*gSaveBlock1Ptr).location.mapNum == 28 {
        return BATTLE_ENVIRONMENT_SAND;
    }
    if GetSavedWeather() == WEATHER_SANDSTORM {
        return BATTLE_ENVIRONMENT_SAND;
    }
    BATTLE_ENVIRONMENT_PLAIN
}
unsafe fn GetBattleTransitionTypeByMap() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    let tileBehavior: u16 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
    if GetFlashLevel() != 0 {
        return TRANSITION_TYPE_FLASH;
    }
    if MetatileBehavior_IsSurfableWaterOrUnderwater(tileBehavior as u8) != 0 {
        return TRANSITION_TYPE_WATER;
    }
    match gMapHeader.mapType {
        MAP_TYPE_UNDERGROUND => {
            return TRANSITION_TYPE_CAVE;
        }
        MAP_TYPE_UNDERWATER => {
            return TRANSITION_TYPE_WATER;
        }
        _ => {
            return TRANSITION_TYPE_NORMAL;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetSumOfPlayerPartyLevel(mut numMons: u8) -> u16 {
    let mut sum: u8 = 0;
    for i in 0..PARTY_SIZE {
        let species: u32 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG);
        if species != SPECIES_EGG
            && species != 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) != 0
        {
            sum += GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) as u8;
            if ({
                numMons -= 1;
                numMons
            }) == 0
            {
                break;
            }
        }
    }
    sum as u16
}
unsafe fn GetSumOfEnemyPartyLevel(opponentId: u16, numMons: u8) -> u8 {
    let mut i: u8 = 0;
    let mut count: u32 = numMons as u32;
    if ((*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())[opponentId]
        .partySize as u32)
        < count
    {
        count = (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [opponentId]
            .partySize as u32;
    }
    let mut sum: u8 = 0;
    match (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [opponentId]
        .partyFlags
    {
        0 => {
            let mut party: *mut TrainerMonNoItemDefaultMoves = null_mut();
            party = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[opponentId]
                .party
                .NoItemDefaultMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        F_TRAINER_PARTY_CUSTOM_MOVESET => {
            let mut party: *mut TrainerMonNoItemCustomMoves = null_mut();
            party = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[opponentId]
                .party
                .NoItemCustomMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        F_TRAINER_PARTY_HELD_ITEM => {
            let mut party: *mut TrainerMonItemDefaultMoves = null_mut();
            party = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[opponentId]
                .party
                .ItemDefaultMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        3 => {
            let mut party: *mut TrainerMonItemCustomMoves = null_mut();
            party = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[opponentId]
                .party
                .ItemCustomMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        _ => {}
    }
    sum
}
unsafe fn GetWildBattleTransition() -> u8 {
    let transitionType: u8 = GetBattleTransitionTypeByMap();
    let enemyLevel: u8 = GetMonData2(&raw mut gEnemyParty[0], MON_DATA_LEVEL) as u8;
    let playerLevel: u8 = GetSumOfPlayerPartyLevel(1) as u8;
    if enemyLevel < playerLevel {
        if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
            return B_TRANSITION_BLUR;
        } else {
            return sBattleTransitionTable_Wild[transitionType][0];
        }
    } else {
        if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
            return B_TRANSITION_GRID_SQUARES;
        } else {
            return sBattleTransitionTable_Wild[transitionType][1];
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetTrainerBattleTransition() -> u8 {
    let mut minPartyCount: u8 = 0;
    if gTrainerBattleOpponent_A == TRAINER_SECRET_BASE {
        return B_TRANSITION_CHAMPION;
    }
    if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [gTrainerBattleOpponent_A]
        .trainerClass
        == TRAINER_CLASS_ELITE_FOUR
    {
        if gTrainerBattleOpponent_A == TRAINER_SIDNEY {
            return B_TRANSITION_SIDNEY;
        }
        if gTrainerBattleOpponent_A == TRAINER_PHOEBE {
            return B_TRANSITION_PHOEBE;
        }
        if gTrainerBattleOpponent_A == TRAINER_GLACIA {
            return B_TRANSITION_GLACIA;
        }
        if gTrainerBattleOpponent_A == TRAINER_DRAKE {
            return B_TRANSITION_DRAKE;
        }
        return B_TRANSITION_CHAMPION;
    }
    if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [gTrainerBattleOpponent_A]
        .trainerClass
        == TRAINER_CLASS_CHAMPION
    {
        return B_TRANSITION_CHAMPION;
    }
    if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [gTrainerBattleOpponent_A]
        .trainerClass
        == TRAINER_CLASS_TEAM_MAGMA
        || (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [gTrainerBattleOpponent_A]
            .trainerClass
            == TRAINER_CLASS_MAGMA_LEADER
        || (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [gTrainerBattleOpponent_A]
            .trainerClass
            == TRAINER_CLASS_MAGMA_ADMIN
    {
        return B_TRANSITION_MAGMA;
    }
    if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [gTrainerBattleOpponent_A]
        .trainerClass
        == TRAINER_CLASS_TEAM_AQUA
        || (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [gTrainerBattleOpponent_A]
            .trainerClass
            == TRAINER_CLASS_AQUA_LEADER
        || (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [gTrainerBattleOpponent_A]
            .trainerClass
            == TRAINER_CLASS_AQUA_ADMIN
    {
        return B_TRANSITION_AQUA;
    }
    if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [gTrainerBattleOpponent_A]
        .doubleBattle
        == TRUE
    {
        minPartyCount = 2;
    } else {
        minPartyCount = 1;
    }
    let transitionType: u8 = GetBattleTransitionTypeByMap();
    let enemyLevel: u8 = GetSumOfEnemyPartyLevel(gTrainerBattleOpponent_A, minPartyCount);
    let playerLevel: u8 = GetSumOfPlayerPartyLevel(minPartyCount) as u8;
    if enemyLevel < playerLevel {
        return sBattleTransitionTable_Trainer[transitionType][0];
    } else {
        return sBattleTransitionTable_Trainer[transitionType][1];
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetSpecialBattleTransition(id: i32) -> u8 {
    let enemyLevel: u8 = GetMonData2(&raw mut gEnemyParty[0], MON_DATA_LEVEL) as u8;
    let playerLevel: u8 = GetSumOfPlayerPartyLevel(1) as u8;
    if enemyLevel < playerLevel {
        match id {
            B_TRANSITION_GROUP_TRAINER_HILL
            | B_TRANSITION_GROUP_SECRET_BASE
            | B_TRANSITION_GROUP_E_READER => {
                return B_TRANSITION_POKEBALLS_TRAIL;
            }
            B_TRANSITION_GROUP_B_PYRAMID => {
                return sBattleTransitionTable_BattlePyramid[Random() % 3];
            }
            B_TRANSITION_GROUP_B_DOME => {
                return sBattleTransitionTable_BattleDome[Random() % 4];
            }
            _ => {}
        }
        if VarGet(VAR_FRONTIER_BATTLE_MODE) != FRONTIER_MODE_LINK_MULTIS {
            return sBattleTransitionTable_BattleFrontier[Random() % 12];
        }
    } else {
        match id {
            B_TRANSITION_GROUP_TRAINER_HILL
            | B_TRANSITION_GROUP_SECRET_BASE
            | B_TRANSITION_GROUP_E_READER => {
                return B_TRANSITION_BIG_POKEBALL;
            }
            B_TRANSITION_GROUP_B_PYRAMID => {
                return sBattleTransitionTable_BattlePyramid[Random() % 3];
            }
            B_TRANSITION_GROUP_B_DOME => {
                return sBattleTransitionTable_BattleDome[Random() % 4];
            }
            _ => {}
        }
        if VarGet(VAR_FRONTIER_BATTLE_MODE) != FRONTIER_MODE_LINK_MULTIS {
            return sBattleTransitionTable_BattleFrontier[Random() % 12];
        }
    }
    let var: u16 = (*gSaveBlock2Ptr).frontier.trainerIds
        [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 * 2]
        + (*gSaveBlock2Ptr).frontier.trainerIds
            [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 * 2 + 1];
    sBattleTransitionTable_BattleFrontier[var % 12]
}
#[unsafe(no_mangle)]
pub unsafe fn ChooseStarter() {
    SetMainCallback2(Some(CB2_ChooseStarter));
    gMain.savedCallback = Some(CB2_GiveStarter);
}
pub(crate) unsafe fn CB2_GiveStarter() {
    *GetVarPointer(VAR_STARTER_MON) = *(&raw const crate::ffi::gSpecialVar_Result)
        .cast::<u16>()
        .cast_mut();
    let starterMon: u16 = GetStarterPokemon(
        *(&raw const crate::ffi::gSpecialVar_Result)
            .cast::<u16>()
            .cast_mut(),
    );
    ScriptGiveMon(starterMon, 5, ITEM_NONE, 0, 0, 0);
    ResetTasks();
    PlayBattleBGM();
    SetMainCallback2(Some(CB2_StartFirstBattle));
    BattleTransition_Start(B_TRANSITION_BLUR);
}
pub(crate) unsafe fn CB2_StartFirstBattle() {
    UpdatePaletteFade();
    RunTasks();
    if IsBattleTransitionDone() == TRUE {
        gBattleTypeFlags = BATTLE_TYPE_FIRST_BATTLE;
        gMain.savedCallback = Some(CB2_EndFirstBattle);
        FreeAllWindowBuffers();
        SetMainCallback2(Some(CB2_InitBattle));
        RestartWildEncounterImmunitySteps();
        ClearPoisonStepCounter();
        IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
        IncrementGameStat(GAME_STAT_WILD_BATTLES);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
pub(crate) unsafe fn CB2_EndFirstBattle() {
    Overworld_ClearSavedMusic();
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
unsafe fn TryUpdateGymLeaderRematchFromWild() {
    if GetGameStat(GAME_STAT_WILD_BATTLES) % 60 == 0 {
        UpdateGymLeaderRematch();
    }
}
unsafe fn TryUpdateGymLeaderRematchFromTrainer() {
    if GetGameStat(GAME_STAT_TRAINER_BATTLES) % 20 == 0 {
        UpdateGymLeaderRematch();
    }
}
unsafe fn TrainerBattleLoadArg32(ptr: *mut u8) -> u32 {
    *ptr as u32 | (*ptr.at(1) as u32) << 8 | (*ptr.at(2) as u32) << 16 | (*ptr.at(3) as u32) << 24
}
unsafe fn TrainerBattleLoadArg16(ptr: *mut u8) -> u16 {
    *ptr as u16 | (*ptr.at(1) as u16) << 8
}
unsafe fn TrainerBattleLoadArg8(ptr: *mut u8) -> u8 {
    *ptr
}
unsafe fn GetTrainerAFlag() -> u16 {
    TRAINER_FLAGS_START + gTrainerBattleOpponent_A
}
unsafe fn GetTrainerBFlag() -> u16 {
    TRAINER_FLAGS_START + gTrainerBattleOpponent_B
}
unsafe fn IsPlayerDefeated(battleOutcome: u32) -> u32 {
    match battleOutcome {
        2 | 3 => {
            return TRUE as u32;
        }
        1 | 4 | 5 | 6 | 7 => {
            return FALSE as u32;
        }
        _ => {
            return FALSE as u32;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ResetTrainerOpponentIds() {
    gTrainerBattleOpponent_A = 0;
    gTrainerBattleOpponent_B = 0;
}
unsafe fn InitTrainerBattleVariables() {
    sTrainerBattleMode.set(0);
    if gApproachingTrainerId == 0 {
        sTrainerAIntroSpeech = null_mut();
        sTrainerADefeatSpeech = null_mut();
        sTrainerABattleScriptRetAddr = null_mut();
    } else {
        sTrainerBIntroSpeech = null_mut();
        sTrainerBDefeatSpeech = null_mut();
        sTrainerBBattleScriptRetAddr = null_mut();
    }
    sTrainerObjectEventLocalId = 0;
    sTrainerVictorySpeech = null_mut();
    sTrainerCannotBattleSpeech = null_mut();
    sTrainerBattleEndScript = null_mut();
}
unsafe fn SetU8(ptr: *mut c_void, value: u8) {
    *(ptr as *mut u8) = value;
}
unsafe fn SetU16(ptr: *mut c_void, value: u16) {
    *(ptr as *mut u16) = value;
}
unsafe fn SetU32(ptr: *mut c_void, value: u32) {
    *(ptr as *mut u32) = value;
}
unsafe fn SetPtr(ptr: *mut c_void, value: *mut c_void) {
    *(ptr as *mut *mut c_void) = value;
}
unsafe fn TrainerBattleLoadArgs(mut specs: *mut TrainerBattleParameter, mut data: *mut u8) {
    loop {
        match (*specs).ptrType {
            TRAINER_PARAM_LOAD_VAL_8BIT => {
                SetU8((*specs).varPtr, TrainerBattleLoadArg8(data));
                data = data.at(1);
            }
            TRAINER_PARAM_LOAD_VAL_16BIT => {
                SetU16((*specs).varPtr, TrainerBattleLoadArg16(data));
                data = data.at(2);
            }
            TRAINER_PARAM_LOAD_VAL_32BIT => {
                SetU32((*specs).varPtr, TrainerBattleLoadArg32(data));
                data = data.at(4);
            }
            TRAINER_PARAM_CLEAR_VAL_8BIT => {
                SetU8((*specs).varPtr, 0);
            }
            TRAINER_PARAM_CLEAR_VAL_16BIT => {
                SetU16((*specs).varPtr, 0);
            }
            TRAINER_PARAM_CLEAR_VAL_32BIT => {
                SetU32((*specs).varPtr, 0);
            }
            TRAINER_PARAM_LOAD_SCRIPT_RET_ADDR => {
                SetPtr((*specs).varPtr, data as *mut c_void);
                return;
            }
            _ => {}
        }
        specs = specs.at(1);
    }
}
pub unsafe fn SetMapVarsToTrainer() {
    if sTrainerObjectEventLocalId != LOCALID_NONE as u16 {
        gSpecialVar_LastTalked = sTrainerObjectEventLocalId;
        gSelectedObjectEvent = GetObjectEventIdByLocalIdAndMap(
            sTrainerObjectEventLocalId as u8,
            (*gSaveBlock1Ptr).location.mapNum as u8,
            (*gSaveBlock1Ptr).location.mapGroup as u8,
        );
    }
}
pub unsafe fn BattleSetup_ConfigureTrainerBattle(data: *mut u8) -> *mut u8 {
    InitTrainerBattleVariables();
    sTrainerBattleMode.set(TrainerBattleLoadArg8(data) as u16);
    match sTrainerBattleMode.get() {
        TRAINER_BATTLE_SINGLE_NO_INTRO_TEXT => {
            TrainerBattleLoadArgs(sOrdinaryNoIntroBattleParams.as_ptr().cast_mut(), data);
            return (*crate::asmdata::EventScript_DoNoIntroTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_DOUBLE => {
            TrainerBattleLoadArgs(sDoubleBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            return (*crate::asmdata::EventScript_TryDoDoubleTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_CONTINUE_SCRIPT => {
            if gApproachingTrainerId == 0 {
                TrainerBattleLoadArgs(sContinueScriptBattleParams.as_ptr().cast_mut(), data);
                SetMapVarsToTrainer();
            } else {
                TrainerBattleLoadArgs(
                    sTrainerBContinueScriptBattleParams.as_ptr().cast_mut(),
                    data,
                );
            }
            return (*crate::asmdata::EventScript_TryDoNormalTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_CONTINUE_SCRIPT_NO_MUSIC => {
            TrainerBattleLoadArgs(sContinueScriptBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            return (*crate::asmdata::EventScript_TryDoNormalTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE | TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE_NO_MUSIC => {
            TrainerBattleLoadArgs(sContinueScriptDoubleBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            return (*crate::asmdata::EventScript_TryDoDoubleTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_REMATCH_DOUBLE => {
            TrainerBattleLoadArgs(sDoubleBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            gTrainerBattleOpponent_A = GetRematchTrainerId(gTrainerBattleOpponent_A);
            return (*crate::asmdata::EventScript_TryDoDoubleRematchBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_REMATCH => {
            TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            gTrainerBattleOpponent_A = GetRematchTrainerId(gTrainerBattleOpponent_A);
            return (*crate::asmdata::EventScript_TryDoRematchBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_PYRAMID => {
            if gApproachingTrainerId == 0 {
                TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
                SetMapVarsToTrainer();
                gTrainerBattleOpponent_A = LocalIdToPyramidTrainerId(gSpecialVar_LastTalked as u8);
            } else {
                TrainerBattleLoadArgs(sTrainerBOrdinaryBattleParams.as_ptr().cast_mut(), data);
                gTrainerBattleOpponent_B = LocalIdToPyramidTrainerId(gSpecialVar_LastTalked as u8);
            }
            return (*crate::asmdata::EventScript_TryDoNormalTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        TRAINER_BATTLE_SET_TRAINER_A => {
            TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
            return null_mut();
        }
        TRAINER_BATTLE_SET_TRAINER_B => {
            TrainerBattleLoadArgs(sTrainerBOrdinaryBattleParams.as_ptr().cast_mut(), data);
            return null_mut();
        }
        TRAINER_BATTLE_HILL => {
            if gApproachingTrainerId == 0 {
                TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
                SetMapVarsToTrainer();
                gTrainerBattleOpponent_A = LocalIdToHillTrainerId(gSpecialVar_LastTalked as u8);
            } else {
                TrainerBattleLoadArgs(sTrainerBOrdinaryBattleParams.as_ptr().cast_mut(), data);
                gTrainerBattleOpponent_B = LocalIdToHillTrainerId(gSpecialVar_LastTalked as u8);
            }
            return (*crate::asmdata::EventScript_TryDoNormalTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        _ => {
            if gApproachingTrainerId == 0 {
                TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
                SetMapVarsToTrainer();
            } else {
                TrainerBattleLoadArgs(sTrainerBOrdinaryBattleParams.as_ptr().cast_mut(), data);
            }
            return (*crate::asmdata::EventScript_TryDoNormalTrainerBattle.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn ConfigureAndSetUpOneTrainerBattle(trainerObjEventId: u8, trainerScript: *mut u8) {
    gSelectedObjectEvent = trainerObjEventId;
    gSpecialVar_LastTalked = gObjectEvents[trainerObjEventId].localId as u16;
    BattleSetup_ConfigureTrainerBattle(trainerScript.at(1));
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_StartTrainerApproach.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    LockPlayerFieldControls();
}
pub unsafe fn ConfigureTwoTrainersBattle(trainerObjEventId: u8, trainerScript: *mut u8) {
    gSelectedObjectEvent = trainerObjEventId;
    gSpecialVar_LastTalked = gObjectEvents[trainerObjEventId].localId as u16;
    BattleSetup_ConfigureTrainerBattle(trainerScript.at(1));
}
pub unsafe fn SetUpTwoTrainersBattle() {
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_StartTrainerApproach.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    LockPlayerFieldControls();
}
pub unsafe fn GetTrainerFlagFromScriptPointer(data: *mut u8) -> u32 {
    let flag: u32 = TrainerBattleLoadArg16(data.at(2)) as u32;
    FlagGet(TRAINER_FLAGS_START + flag as u16) as u32
}
#[unsafe(no_mangle)]
pub unsafe fn SetTrainerFacingDirection() {
    let objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gSelectedObjectEvent];
    SetTrainerMovementType(
        objectEvent,
        GetTrainerFacingDirectionMovementType((*objectEvent).facingDirection() as u8),
    );
}
#[unsafe(no_mangle)]
pub fn GetTrainerBattleMode() -> u8 {
    sTrainerBattleMode.get() as u8
}
#[unsafe(no_mangle)]
pub unsafe fn GetTrainerFlag() -> u8 {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        return GetBattlePyramidTrainerFlag(gSelectedObjectEvent);
    } else if InTrainerHill() != 0 {
        return GetHillTrainerFlag(gSelectedObjectEvent);
    } else {
        return FlagGet(GetTrainerAFlag());
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SetBattledTrainersFlags() {
    if gTrainerBattleOpponent_B != 0 {
        FlagSet(GetTrainerBFlag());
    }
    FlagSet(GetTrainerAFlag());
}
unsafe fn SetBattledTrainerFlag() {
    FlagSet(GetTrainerAFlag());
}
#[unsafe(no_mangle)]
pub unsafe fn HasTrainerBeenFought(trainerId: u16) -> u8 {
    FlagGet(TRAINER_FLAGS_START + trainerId)
}
pub unsafe fn SetTrainerFlag(trainerId: u16) {
    FlagSet(TRAINER_FLAGS_START + trainerId);
}
pub unsafe fn ClearTrainerFlag(trainerId: u16) {
    FlagClear(TRAINER_FLAGS_START + trainerId);
}
pub unsafe fn BattleSetup_StartTrainerBattle() {
    if gNoOfApproachingTrainers == 2 {
        gBattleTypeFlags = 32777;
    } else {
        gBattleTypeFlags = BATTLE_TYPE_TRAINER;
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        VarSet(VAR_TEMP_PLAYING_PYRAMID_MUSIC, 0);
        gBattleTypeFlags |= BATTLE_TYPE_PYRAMID;
        if gNoOfApproachingTrainers == 2 {
            FillFrontierTrainersParties(1);
            ZeroMonData(&raw mut gEnemyParty[1]);
            ZeroMonData(&raw mut gEnemyParty[2]);
            ZeroMonData(&raw mut gEnemyParty[4]);
            ZeroMonData(&raw mut gEnemyParty[5]);
        } else {
            FillFrontierTrainerParty(1);
            ZeroMonData(&raw mut gEnemyParty[1]);
            ZeroMonData(&raw mut gEnemyParty[2]);
        }
        MarkApproachingPyramidTrainersAsBattled();
    } else if InTrainerHillChallenge() != 0 {
        gBattleTypeFlags |= BATTLE_TYPE_TRAINER_HILL;
        if gNoOfApproachingTrainers == 2 {
            FillHillTrainersParties();
        } else {
            FillHillTrainerParty();
        }
        SetHillTrainerFlag();
    }
    sNoOfPossibleTrainerRetScripts = gNoOfApproachingTrainers;
    gNoOfApproachingTrainers = 0;
    sShouldCheckTrainerBScript.set(FALSE);
    gWhichTrainerToFaceAfterBattle = 0;
    gMain.savedCallback = Some(CB2_EndTrainerBattle);
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE || InTrainerHillChallenge() != 0 {
        DoBattlePyramidTrainerHillBattle();
    } else {
        DoTrainerBattle();
    }
    ScriptContext_Stop();
}
pub(crate) unsafe fn CB2_EndTrainerBattle() {
    if gTrainerBattleOpponent_A == TRAINER_SECRET_BASE {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    } else if IsPlayerDefeated(gBattleOutcome as u32) == TRUE as u32 {
        if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE || InTrainerHillChallenge() != 0
        {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        } else {
            SetMainCallback2(Some(CB2_WhiteOut));
        }
    } else {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE && InTrainerHillChallenge() == 0
        {
            RegisterTrainerInMatchCall();
            SetBattledTrainersFlags();
        }
    }
}
pub(crate) unsafe fn CB2_EndRematchBattle() {
    if gTrainerBattleOpponent_A == TRAINER_SECRET_BASE {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    } else if IsPlayerDefeated(gBattleOutcome as u32) == TRUE as u32 {
        SetMainCallback2(Some(CB2_WhiteOut));
    } else {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        RegisterTrainerInMatchCall();
        SetBattledTrainersFlags();
        HandleRematchVarsOnBattleEnd();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BattleSetup_StartRematchBattle() {
    gBattleTypeFlags = BATTLE_TYPE_TRAINER;
    gMain.savedCallback = Some(CB2_EndRematchBattle);
    DoTrainerBattle();
    ScriptContext_Stop();
}
#[unsafe(no_mangle)]
pub unsafe fn ShowTrainerIntroSpeech() {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        if gNoOfApproachingTrainers == 0 || gNoOfApproachingTrainers == 1 {
            CopyPyramidTrainerSpeechBefore(LocalIdToPyramidTrainerId(gSpecialVar_LastTalked as u8));
        } else {
            CopyPyramidTrainerSpeechBefore(LocalIdToPyramidTrainerId(
                gObjectEvents[gApproachingTrainers[gApproachingTrainerId].objectEventId].localId,
            ));
        }
        ShowFieldMessageFromBuffer();
    } else if InTrainerHillChallenge() != 0 {
        if gNoOfApproachingTrainers == 0 || gNoOfApproachingTrainers == 1 {
            CopyTrainerHillTrainerText(
                TRAINER_HILL_TEXT_INTRO,
                LocalIdToHillTrainerId(gSpecialVar_LastTalked as u8),
            );
        } else {
            CopyTrainerHillTrainerText(
                TRAINER_HILL_TEXT_INTRO,
                LocalIdToHillTrainerId(
                    gObjectEvents[gApproachingTrainers[gApproachingTrainerId].objectEventId]
                        .localId,
                ),
            );
        }
        ShowFieldMessageFromBuffer();
    } else {
        ShowFieldMessage(GetIntroSpeechOfApproachingTrainer());
    }
}
pub unsafe fn BattleSetup_GetScriptAddrAfterBattle() -> *mut u8 {
    if !sTrainerBattleEndScript.is_null() {
        return sTrainerBattleEndScript;
    } else {
        return (*crate::asmdata::EventScript_TestSignpostMsg.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn BattleSetup_GetTrainerPostBattleScript() -> *mut u8 {
    if sShouldCheckTrainerBScript.get() != 0 {
        sShouldCheckTrainerBScript.set(FALSE);
        if !sTrainerBBattleScriptRetAddr.is_null() {
            gWhichTrainerToFaceAfterBattle = 1;
            return sTrainerBBattleScriptRetAddr;
        }
    } else {
        if !sTrainerABattleScriptRetAddr.is_null() {
            gWhichTrainerToFaceAfterBattle = 0;
            return sTrainerABattleScriptRetAddr;
        }
    }
    (*crate::asmdata::EventScript_TryGetTrainerScript.cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut()
}
#[unsafe(no_mangle)]
pub unsafe fn ShowTrainerCantBattleSpeech() {
    ShowFieldMessage(GetTrainerCantBattleSpeech());
}
#[unsafe(no_mangle)]
pub unsafe fn PlayTrainerEncounterMusic() {
    let mut trainerId: u16 = 0;
    let mut music: u16 = 0;
    if gApproachingTrainerId == 0 {
        trainerId = gTrainerBattleOpponent_A;
    } else {
        trainerId = gTrainerBattleOpponent_B;
    }
    if sTrainerBattleMode.get() != TRAINER_BATTLE_CONTINUE_SCRIPT_NO_MUSIC
        && sTrainerBattleMode.get() != TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE_NO_MUSIC
    {
        match GetTrainerEncounterMusicId(trainerId) {
            TRAINER_ENCOUNTER_MUSIC_MALE => {
                music = MUS_ENCOUNTER_MALE;
            }
            TRAINER_ENCOUNTER_MUSIC_FEMALE => {
                music = MUS_ENCOUNTER_FEMALE;
            }
            TRAINER_ENCOUNTER_MUSIC_GIRL => {
                music = MUS_ENCOUNTER_GIRL;
            }
            TRAINER_ENCOUNTER_MUSIC_INTENSE => {
                music = MUS_ENCOUNTER_INTENSE;
            }
            TRAINER_ENCOUNTER_MUSIC_COOL => {
                music = MUS_ENCOUNTER_COOL;
            }
            TRAINER_ENCOUNTER_MUSIC_AQUA => {
                music = MUS_ENCOUNTER_AQUA;
            }
            TRAINER_ENCOUNTER_MUSIC_MAGMA => {
                music = MUS_ENCOUNTER_MAGMA;
            }
            TRAINER_ENCOUNTER_MUSIC_SWIMMER => {
                music = MUS_ENCOUNTER_SWIMMER;
            }
            TRAINER_ENCOUNTER_MUSIC_TWINS => {
                music = MUS_ENCOUNTER_TWINS;
            }
            TRAINER_ENCOUNTER_MUSIC_ELITE_FOUR => {
                music = MUS_ENCOUNTER_ELITE_FOUR;
            }
            TRAINER_ENCOUNTER_MUSIC_HIKER => {
                music = MUS_ENCOUNTER_HIKER;
            }
            TRAINER_ENCOUNTER_MUSIC_INTERVIEWER => {
                music = MUS_ENCOUNTER_INTERVIEWER;
            }
            TRAINER_ENCOUNTER_MUSIC_RICH => {
                music = MUS_ENCOUNTER_RICH;
            }
            _ => {
                music = MUS_ENCOUNTER_SUSPICIOUS;
            }
        }
        PlayNewMapMusic(music);
    }
}
unsafe fn ReturnEmptyStringIfNull(string: *mut u8) -> *mut u8 {
    if string.is_null() {
        return (*(&raw const crate::data::strings::gText_EmptyString2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        return string;
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
unsafe fn GetIntroSpeechOfApproachingTrainer() -> *mut u8 {
    if gApproachingTrainerId == 0 {
        return ReturnEmptyStringIfNull(sTrainerAIntroSpeech);
    } else {
        return ReturnEmptyStringIfNull(sTrainerBIntroSpeech);
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn GetTrainerALoseText() -> *mut u8 {
    let mut string: *mut u8 = null_mut();
    if gTrainerBattleOpponent_A == TRAINER_SECRET_BASE {
        string = GetSecretBaseTrainerLoseText();
    } else {
        string = sTrainerADefeatSpeech;
    }
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), ReturnEmptyStringIfNull(string));
    gStringVar4.as_mut_ptr()
}
pub unsafe fn GetTrainerBLoseText() -> *mut u8 {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        ReturnEmptyStringIfNull(sTrainerBDefeatSpeech),
    );
    gStringVar4.as_mut_ptr()
}
pub unsafe fn GetTrainerWonSpeech() -> *mut u8 {
    ReturnEmptyStringIfNull(sTrainerVictorySpeech)
}
unsafe fn GetTrainerCantBattleSpeech() -> *mut u8 {
    ReturnEmptyStringIfNull(sTrainerCannotBattleSpeech)
}
unsafe fn FirstBattleTrainerIdToRematchTableId(table: *mut RematchTrainer, trainerId: u16) -> i32 {
    for i in 0..REMATCH_TABLE_ENTRIES {
        if (*table.at(i)).trainerIds[0] == trainerId {
            return i;
        }
    }
    -1
}
unsafe fn TrainerIdToRematchTableId(table: *mut RematchTrainer, trainerId: u16) -> i32 {
    for i in 0..REMATCH_TABLE_ENTRIES {
        for j in 0..REMATCHES_COUNT {
            if (*table.at(i)).trainerIds[j] == 0 {
                break;
            }
            if (*table.at(i)).trainerIds[j] == trainerId {
                return i;
            }
        }
    }
    -1
}
unsafe fn IsRematchForbidden(rematchTableId: i32) -> u32 {
    if rematchTableId >= REMATCH_SIDNEY {
        return TRUE as u32;
    } else if rematchTableId == REMATCH_WALLY_VR as i32 {
        return (FlagGet(FLAG_DEFEATED_WALLY_VICTORY_ROAD) == 0) as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SetRematchIdForTrainer(table: *mut RematchTrainer, tableId: u32) {
    let mut i: i32 = 1;
    while i < REMATCHES_COUNT {
        let trainerId: u16 = (*table.at(tableId)).trainerIds[i];
        if trainerId == 0 {
            break;
        }
        if HasTrainerBeenFought(trainerId) == 0 {
            break;
        }
        i += 1;
    }
    (*gSaveBlock1Ptr).trainerRematches[tableId] = i as u8;
}
unsafe fn UpdateRandomTrainerRematches(
    table: *mut RematchTrainer,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    let mut ret: u32 = FALSE as u32;
    let mut i: i32 = 0;
    while i <= REMATCH_WALLY_VR as i32 {
        if (*table.at(i)).mapGroup == mapGroup
            && (*table.at(i)).mapNum == mapNum
            && IsRematchForbidden(i) == 0
        {
            if (*gSaveBlock1Ptr).trainerRematches[i] != 0 {
                ret = TRUE as u32;
            } else if FlagGet(TRAINER_REGISTERED_FLAGS_START + i as u16) != 0
                && Random() as i32 % 100 <= 30
            {
                SetRematchIdForTrainer(table, i as u32);
                ret = TRUE as u32;
            }
        }
        i += 1;
    }
    ret
}
pub unsafe fn UpdateRematchIfDefeated(rematchTableId: i32) {
    if HasTrainerBeenFought(gRematchTable[rematchTableId].trainerIds[0]) == TRUE {
        SetRematchIdForTrainer(gRematchTable.as_ptr().cast_mut(), rematchTableId as u32);
    }
}
unsafe fn DoesSomeoneWantRematchIn_(table: *mut RematchTrainer, mapGroup: u16, mapNum: u16) -> u32 {
    for i in 0..REMATCH_TABLE_ENTRIES {
        if (*table.at(i)).mapGroup == mapGroup
            && (*table.at(i)).mapNum == mapNum
            && (*gSaveBlock1Ptr).trainerRematches[i] != 0
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn IsRematchTrainerIn_(table: *mut RematchTrainer, mapGroup: u16, mapNum: u16) -> u32 {
    for i in 0..REMATCH_TABLE_ENTRIES {
        if (*table.at(i)).mapGroup == mapGroup && (*table.at(i)).mapNum == mapNum {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn IsFirstTrainerIdReadyForRematch(
    table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u8 {
    let tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE;
    }
    if tableId >= MAX_REMATCH_ENTRIES {
        return FALSE;
    }
    if (*gSaveBlock1Ptr).trainerRematches[tableId] == 0 {
        return FALSE;
    }
    TRUE
}
unsafe fn IsTrainerReadyForRematch_(table: *mut RematchTrainer, trainerId: u16) -> u8 {
    let tableId: i32 = TrainerIdToRematchTableId(table, trainerId);
    if tableId == -1 {
        return FALSE;
    }
    if tableId >= MAX_REMATCH_ENTRIES {
        return FALSE;
    }
    if (*gSaveBlock1Ptr).trainerRematches[tableId] == 0 {
        return FALSE;
    }
    TRUE
}
unsafe fn GetRematchTrainerIdFromTable(
    table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u16 {
    let tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE as u16;
    }
    let trainerEntry: *mut RematchTrainer = table.at(tableId);
    for i in 1..REMATCHES_COUNT {
        if (*trainerEntry).trainerIds[i] == 0 {
            return (*trainerEntry).trainerIds[i - 1];
        }
        if HasTrainerBeenFought((*trainerEntry).trainerIds[i]) == 0 {
            return (*trainerEntry).trainerIds[i];
        }
    }
    (*trainerEntry).trainerIds[4]
}
unsafe fn GetLastBeatenRematchTrainerIdFromTable(
    table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u16 {
    let tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE as u16;
    }
    let trainerEntry: *mut RematchTrainer = table.at(tableId);
    for i in 1..REMATCHES_COUNT {
        if (*trainerEntry).trainerIds[i] == 0 {
            return (*trainerEntry).trainerIds[i - 1];
        }
        if HasTrainerBeenFought((*trainerEntry).trainerIds[i]) == 0 {
            return (*trainerEntry).trainerIds[i - 1];
        }
    }
    (*trainerEntry).trainerIds[4]
}
unsafe fn ClearTrainerWantRematchState(table: *mut RematchTrainer, firstBattleTrainerId: u16) {
    let tableId: i32 = TrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId != -1 {
        (*gSaveBlock1Ptr).trainerRematches[tableId] = 0;
    }
}
unsafe fn GetTrainerMatchCallFlag(trainerId: u32) -> u32 {
    for i in 0..REMATCH_TABLE_ENTRIES {
        if gRematchTable[i].trainerIds[0] as u32 == trainerId {
            return TRAINER_REGISTERED_FLAGS_START as u32 + i as u32;
        }
    }
    0xFFFF
}
unsafe fn RegisterTrainerInMatchCall() {
    if FlagGet(FLAG_HAS_MATCH_CALL) != 0 {
        let matchCallFlagId: u32 = GetTrainerMatchCallFlag(gTrainerBattleOpponent_A as u32);
        if matchCallFlagId != 0xFFFF {
            FlagSet(matchCallFlagId as u16);
        }
    }
}
unsafe fn WasSecondRematchWon(table: *mut RematchTrainer, firstBattleTrainerId: u16) -> u8 {
    let tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE;
    }
    if HasTrainerBeenFought((*table.at(tableId)).trainerIds[1]) == 0 {
        return FALSE;
    }
    TRUE
}
unsafe fn HasAtLeastFiveBadges() -> u32 {
    let mut count: i32 = 0;
    for i in 0..8i32 {
        if FlagGet(sBadgeFlags[i]) == TRUE
            && ({
                count += 1;
                count
            }) >= 5
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn IncrementRematchStepCounter() {
    if HasAtLeastFiveBadges() != 0 {
        if (*gSaveBlock1Ptr).trainerRematchStepCounter >= STEP_COUNTER_MAX {
            (*gSaveBlock1Ptr).trainerRematchStepCounter = STEP_COUNTER_MAX;
        } else {
            (*gSaveBlock1Ptr).trainerRematchStepCounter += 1;
        }
    }
}
unsafe fn IsRematchStepCounterMaxed() -> u32 {
    if HasAtLeastFiveBadges() != 0
        && (*gSaveBlock1Ptr).trainerRematchStepCounter >= STEP_COUNTER_MAX
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn TryUpdateRandomTrainerRematches(mapGroup: u16, mapNum: u16) {
    if IsRematchStepCounterMaxed() != 0
        && UpdateRandomTrainerRematches(gRematchTable.as_ptr().cast_mut(), mapGroup, mapNum)
            == TRUE as u32
    {
        (*gSaveBlock1Ptr).trainerRematchStepCounter = 0;
    }
}
pub unsafe fn DoesSomeoneWantRematchIn(mapGroup: u16, mapNum: u16) -> u32 {
    DoesSomeoneWantRematchIn_(gRematchTable.as_ptr().cast_mut(), mapGroup, mapNum)
}
pub unsafe fn IsRematchTrainerIn(mapGroup: u16, mapNum: u16) -> u32 {
    IsRematchTrainerIn_(gRematchTable.as_ptr().cast_mut(), mapGroup, mapNum)
}
unsafe fn GetRematchTrainerId(trainerId: u16) -> u16 {
    GetRematchTrainerIdFromTable(gRematchTable.as_ptr().cast_mut(), trainerId)
}
pub unsafe fn GetLastBeatenRematchTrainerId(trainerId: u16) -> u16 {
    GetLastBeatenRematchTrainerIdFromTable(gRematchTable.as_ptr().cast_mut(), trainerId)
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldTryRematchBattle() -> u8 {
    if IsFirstTrainerIdReadyForRematch(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A)
        != 0
    {
        return TRUE;
    }
    WasSecondRematchWon(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A)
}
#[unsafe(no_mangle)]
pub unsafe fn IsTrainerReadyForRematch() -> u8 {
    IsTrainerReadyForRematch_(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A)
}
unsafe fn HandleRematchVarsOnBattleEnd() {
    ClearTrainerWantRematchState(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A);
    SetBattledTrainersFlags();
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldTryGetTrainerScript() {
    if sNoOfPossibleTrainerRetScripts > 1 {
        sNoOfPossibleTrainerRetScripts = 0;
        sShouldCheckTrainerBScript.set(TRUE);
        gSpecialVar_Result = TRUE as u16;
    } else {
        sShouldCheckTrainerBScript.set(FALSE);
        gSpecialVar_Result = FALSE as u16;
    }
}
pub unsafe fn CountBattledRematchTeams(trainerId: u16) -> u16 {
    if HasTrainerBeenFought(gRematchTable[trainerId].trainerIds[0]) != TRUE {
        return 0;
    }
    let mut i: i32 = 1;
    while i < REMATCHES_COUNT {
        if gRematchTable[trainerId].trainerIds[i] == 0 {
            break;
        }
        if HasTrainerBeenFought(gRematchTable[trainerId].trainerIds[i]) == 0 {
            break;
        }
        i += 1;
    }
    i as u16
}
