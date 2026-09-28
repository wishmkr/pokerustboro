//! Translated from `src/battle_setup.c` by tools/rustport/c2rs.py.
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

static gRematchTable: Table<CArray<RematchTrainer, 78>> =
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
pub(crate) static mut sTrainerBattleMode: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerBattleOpponent_A: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerBattleOpponent_B: u16 = 0;
#[unsafe(no_mangle)]
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
pub(crate) static mut sShouldCheckTrainerBScript: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNoOfPossibleTrainerRetScripts: u8 = 0;

unsafe extern "C" {
    static EventScript_DoNoIntroTrainerBattle: CArray<u8, 0>;
    static EventScript_StartTrainerApproach: CArray<u8, 0>;
    static EventScript_TestSignpostMsg: CArray<u8, 0>;
    static EventScript_TryDoDoubleRematchBattle: CArray<u8, 0>;
    static EventScript_TryDoDoubleTrainerBattle: CArray<u8, 0>;
    static EventScript_TryDoNormalTrainerBattle: CArray<u8, 0>;
    static EventScript_TryDoRematchBattle: CArray<u8, 0>;
    static EventScript_TryGetTrainerScript: CArray<u8, 0>;
    static mut gApproachingTrainerId: u8;
    static mut gApproachingTrainers: CArray<ApproachingTrainer, 2>;
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u32;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static gGameVersion: u8;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static mut gNoOfApproachingTrainers: u8;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedObjectEvent: u8;
    static mut gSpecialVar_LastTalked: u16;
    static mut gSpecialVar_Result: u16;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_EmptyString2: CArray<u8, 0>;
    static gTrainers: CArray<Trainer, 0>;
    static mut gWhichTrainerToFaceAfterBattle: u16;
    fn BattleTransition_Start(a0: u8);
    fn BattleTransition_StartOnField(a0: u8);
    fn CB2_ChooseStarter();
    fn CB2_EndSafariBattle();
    fn CB2_InitBattle();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_WhiteOut();
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearMirageTowerPulseBlendEffect();
    fn ClearPoisonStepCounter();
    fn CopyPyramidTrainerSpeechBefore(a0: u16);
    fn CopyTrainerHillTrainerText(a0: u8, a1: u16);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateMaleMon(a0: *mut Pokemon, a1: u16, a2: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn FieldCB_ReturnToFieldNoScriptCheckMusic();
    fn FillFrontierTrainerParty(a0: u8);
    fn FillFrontierTrainersParties(a0: u8);
    fn FillHillTrainerParty();
    fn FillHillTrainersParties();
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FldEffPoison_IsActive() -> u32;
    fn FreeAllWindowBuffers();
    fn FreezeObjectEvents();
    fn GetBattlePyramidTrainerFlag(a0: u8) -> u8;
    fn GetFlashLevel() -> u8;
    fn GetGameStat(a0: u8) -> u32;
    fn GetHillTrainerFlag(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetSafariZoneFlag() -> u32;
    fn GetSavedWeather() -> u8;
    fn GetSecretBaseTrainerLoseText() -> *mut u8;
    fn GetStarterPokemon(a0: u16) -> u16;
    fn GetTrainerEncounterMusicId(a0: u16) -> u8;
    fn GetTrainerFacingDirectionMovementType(a0: u8) -> u8;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn InBattlePike() -> u8;
    fn InTrainerHill() -> u32;
    fn InTrainerHillChallenge() -> u8;
    fn IncrementDailyWildBattles();
    fn IncrementGameStat(a0: u8);
    fn IsBattleTransitionDone() -> u8;
    fn LocalIdToHillTrainerId(a0: u8) -> u16;
    fn LocalIdToPyramidTrainerId(a0: u8) -> u16;
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MarkApproachingPyramidTrainersAsBattled();
    fn MetatileBehavior_GetBridgeType(a0: u8) -> u8;
    fn MetatileBehavior_IsBridgeOverWater(a0: u8) -> u8;
    fn MetatileBehavior_IsDeepOrOceanWater(a0: u8) -> u8;
    fn MetatileBehavior_IsIndoorEncounter(a0: u8) -> u8;
    fn MetatileBehavior_IsLongGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsMountain(a0: u8) -> u8;
    fn MetatileBehavior_IsSandOrDeepSand(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableWaterOrUnderwater(a0: u8) -> u8;
    fn MetatileBehavior_IsTallGrass(a0: u8) -> u8;
    fn Overworld_ClearSavedMusic();
    fn PlayBattleBGM();
    fn PlayMapChosenOrBattleBGM(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn Random() -> u16;
    fn ResetOamRange(a0: u8, a1: u8);
    fn ResetTasks();
    fn RestartWildEncounterImmunitySteps();
    fn RunTasks();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn ScriptContext_Stop();
    fn ScriptGiveMon(a0: u16, a1: u8, a2: u16, a3: u32, a4: u32, a5: u8) -> u8;
    fn SetHillTrainerFlag();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetTrainerMovementType(a0: *mut ObjectEvent, a1: u8);
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessageFromBuffer() -> u8;
    fn StopPlayerAvatar();
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn UpdateGymLeaderRematch();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroMonData(a0: *mut Pokemon);
}

pub(crate) unsafe extern "C" fn Task_BattleStart(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if FldEffPoison_IsActive() == 0 {
                BattleTransition_StartOnField(*data.at(1) as u8);
                ClearMirageTowerPulseBlendEffect();
                *data += 1;
            }
        }
        1 => {
            if IsBattleTransitionDone() == TRUE {
                CleanupOverworldWindowsAndTilemaps();
                SetMainCallback2(Some(CB2_InitBattle));
                RestartWildEncounterImmunitySteps();
                ClearPoisonStepCounter();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateBattleStartTask(transition: u8, song: u16) {
    let mut taskId: u8 = CreateTask(Some(Task_BattleStart), 1);
    gTasks[taskId].data[1] = transition as i16;
    PlayMapChosenOrBattleBGM(song);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartWildBattle() {
    if GetSafariZoneFlag() != 0 {
        DoSafariBattle();
    } else {
        DoStandardWildBattle();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartBattlePikeWildBattle() {
    DoBattlePikeWildBattle();
}
pub(crate) unsafe extern "C" fn DoStandardWildBattle() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartRoamerBattle() {
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
pub(crate) unsafe extern "C" fn DoSafariBattle() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    StopPlayerAvatar();
    gMain.savedCallback = Some(CB2_EndSafariBattle);
    gBattleTypeFlags = BATTLE_TYPE_SAFARI;
    CreateBattleStartTask(GetWildBattleTransition(), 0);
}
pub(crate) unsafe extern "C" fn DoBattlePikeWildBattle() {
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
pub(crate) unsafe extern "C" fn DoTrainerBattle() {
    CreateBattleStartTask(GetTrainerBattleTransition(), 0);
    IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
    IncrementGameStat(GAME_STAT_TRAINER_BATTLES);
    TryUpdateGymLeaderRematchFromTrainer();
}
pub(crate) unsafe extern "C" fn DoBattlePyramidTrainerHillBattle() {
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
pub unsafe extern "C" fn StartWallyTutorialBattle() {
    CreateMaleMon(&raw mut gEnemyParty[0], SPECIES_RALTS, 5);
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_ReturnToFieldContinueScriptPlayMapMusic);
    gBattleTypeFlags = BATTLE_TYPE_WALLY_TUTORIAL;
    CreateBattleStartTask(B_TRANSITION_SLICE, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartScriptedWildBattle() {
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
pub unsafe extern "C" fn BattleSetup_StartLatiBattle() {
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
pub unsafe extern "C" fn BattleSetup_StartLegendaryBattle() {
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
pub unsafe extern "C" fn StartGroudonKyogreBattle() {
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
pub unsafe extern "C" fn StartRegiBattle() {
    let mut transitionId: u8 = 0;
    let mut species: u16 = 0;
    LockPlayerFieldControls();
    gMain.savedCallback = Some(CB2_EndScriptedWildBattle);
    gBattleTypeFlags = 24576;
    species = GetMonData2(&raw mut gEnemyParty[0], MON_DATA_SPECIES) as u16;
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
pub(crate) unsafe extern "C" fn CB2_EndWildBattle() {
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
pub(crate) unsafe extern "C" fn CB2_EndScriptedWildBattle() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_GetEnvironmentId() -> u8 {
    let mut tileBehavior: u16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
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
    return BATTLE_ENVIRONMENT_PLAIN;
}
pub(crate) unsafe extern "C" fn GetBattleTransitionTypeByMap() -> u8 {
    let mut tileBehavior: u16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetSumOfPlayerPartyLevel(mut numMons: u8) -> u16 {
    let mut sum: u8 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        let mut species: u32 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG);
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
        i += 1;
    }
    return sum as u16;
}
pub(crate) unsafe extern "C" fn GetSumOfEnemyPartyLevel(opponentId: u16, numMons: u8) -> u8 {
    let mut i: u8 = 0;
    let mut sum: u8 = 0;
    let mut count: u32 = numMons as u32;
    if (gTrainers[opponentId].partySize as u32) < count {
        count = gTrainers[opponentId].partySize as u32;
    }
    sum = 0;
    match gTrainers[opponentId].partyFlags {
        0 => {
            let mut party: *mut TrainerMonNoItemDefaultMoves = null_mut();
            party = gTrainers[opponentId].party.NoItemDefaultMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        F_TRAINER_PARTY_CUSTOM_MOVESET => {
            let mut party: *mut TrainerMonNoItemCustomMoves = null_mut();
            party = gTrainers[opponentId].party.NoItemCustomMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        F_TRAINER_PARTY_HELD_ITEM => {
            let mut party: *mut TrainerMonItemDefaultMoves = null_mut();
            party = gTrainers[opponentId].party.ItemDefaultMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        3 => {
            let mut party: *mut TrainerMonItemCustomMoves = null_mut();
            party = gTrainers[opponentId].party.ItemCustomMoves;
            i = 0;
            while (i as u32) < count {
                sum += (*party.at(i)).lvl;
                i += 1;
            }
        }
        _ => {}
    }
    return sum;
}
pub(crate) unsafe extern "C" fn GetWildBattleTransition() -> u8 {
    let mut transitionType: u8 = GetBattleTransitionTypeByMap();
    let mut enemyLevel: u8 = GetMonData2(&raw mut gEnemyParty[0], MON_DATA_LEVEL) as u8;
    let mut playerLevel: u8 = GetSumOfPlayerPartyLevel(1) as u8;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerBattleTransition() -> u8 {
    let mut minPartyCount: u8 = 0;
    let mut transitionType: u8 = 0;
    let mut enemyLevel: u8 = 0;
    let mut playerLevel: u8 = 0;
    if gTrainerBattleOpponent_A == TRAINER_SECRET_BASE {
        return B_TRANSITION_CHAMPION;
    }
    if gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_ELITE_FOUR {
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
    if gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_CHAMPION {
        return B_TRANSITION_CHAMPION;
    }
    if gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_TEAM_MAGMA
        || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_MAGMA_LEADER
        || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_MAGMA_ADMIN
    {
        return B_TRANSITION_MAGMA;
    }
    if gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_TEAM_AQUA
        || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_AQUA_LEADER
        || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_AQUA_ADMIN
    {
        return B_TRANSITION_AQUA;
    }
    if gTrainers[gTrainerBattleOpponent_A].doubleBattle == TRUE {
        minPartyCount = 2;
    } else {
        minPartyCount = 1;
    }
    transitionType = GetBattleTransitionTypeByMap();
    enemyLevel = GetSumOfEnemyPartyLevel(gTrainerBattleOpponent_A, minPartyCount);
    playerLevel = GetSumOfPlayerPartyLevel(minPartyCount) as u8;
    if enemyLevel < playerLevel {
        return sBattleTransitionTable_Trainer[transitionType][0];
    } else {
        return sBattleTransitionTable_Trainer[transitionType][1];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpecialBattleTransition(id: i32) -> u8 {
    let mut var: u16 = 0;
    let mut enemyLevel: u8 = GetMonData2(&raw mut gEnemyParty[0], MON_DATA_LEVEL) as u8;
    let mut playerLevel: u8 = GetSumOfPlayerPartyLevel(1) as u8;
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
    var = (*gSaveBlock2Ptr).frontier.trainerIds
        [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 * 2 + 0]
        + (*gSaveBlock2Ptr).frontier.trainerIds
            [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 * 2 + 1];
    return sBattleTransitionTable_BattleFrontier[var % 12];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseStarter() {
    SetMainCallback2(Some(CB2_ChooseStarter));
    gMain.savedCallback = Some(CB2_GiveStarter);
}
pub(crate) unsafe extern "C" fn CB2_GiveStarter() {
    let mut starterMon: u16 = 0;
    *GetVarPointer(VAR_STARTER_MON) = gSpecialVar_Result;
    starterMon = GetStarterPokemon(gSpecialVar_Result);
    ScriptGiveMon(starterMon, 5, ITEM_NONE, 0, 0, 0);
    ResetTasks();
    PlayBattleBGM();
    SetMainCallback2(Some(CB2_StartFirstBattle));
    BattleTransition_Start(B_TRANSITION_BLUR);
}
pub(crate) unsafe extern "C" fn CB2_StartFirstBattle() {
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
pub(crate) unsafe extern "C" fn CB2_EndFirstBattle() {
    Overworld_ClearSavedMusic();
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
pub(crate) unsafe extern "C" fn TryUpdateGymLeaderRematchFromWild() {
    if GetGameStat(GAME_STAT_WILD_BATTLES) % 60 == 0 {
        UpdateGymLeaderRematch();
    }
}
pub(crate) unsafe extern "C" fn TryUpdateGymLeaderRematchFromTrainer() {
    if GetGameStat(GAME_STAT_TRAINER_BATTLES) % 20 == 0 {
        UpdateGymLeaderRematch();
    }
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArg32(ptr: *mut u8) -> u32 {
    return *ptr as u32
        | (*ptr.at(1) as u32) << 8
        | (*ptr.at(2) as u32) << 16
        | (*ptr.at(3) as u32) << 24;
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArg16(ptr: *mut u8) -> u16 {
    return *ptr as u16 | (*ptr.at(1) as u16) << 8;
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArg8(ptr: *mut u8) -> u8 {
    return *ptr;
}
pub(crate) unsafe extern "C" fn GetTrainerAFlag() -> u16 {
    return TRAINER_FLAGS_START + gTrainerBattleOpponent_A;
}
pub(crate) unsafe extern "C" fn GetTrainerBFlag() -> u16 {
    return TRAINER_FLAGS_START + gTrainerBattleOpponent_B;
}
pub(crate) unsafe extern "C" fn IsPlayerDefeated(battleOutcome: u32) -> u32 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTrainerOpponentIds() {
    gTrainerBattleOpponent_A = 0;
    gTrainerBattleOpponent_B = 0;
}
pub(crate) unsafe extern "C" fn InitTrainerBattleVariables() {
    sTrainerBattleMode = 0;
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
pub(crate) unsafe extern "C" fn SetU8(ptr: *mut c_void, value: u8) {
    *(ptr as *mut u8) = value;
}
pub(crate) unsafe extern "C" fn SetU16(ptr: *mut c_void, value: u16) {
    *(ptr as *mut u16) = value;
}
pub(crate) unsafe extern "C" fn SetU32(ptr: *mut c_void, value: u32) {
    *(ptr as *mut u32) = value;
}
pub(crate) unsafe extern "C" fn SetPtr(ptr: *mut c_void, value: *mut c_void) {
    *(ptr as *mut *mut c_void) = value;
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArgs(
    mut specs: *mut TrainerBattleParameter,
    mut data: *mut u8,
) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMapVarsToTrainer() {
    if sTrainerObjectEventLocalId != LOCALID_NONE as u16 {
        gSpecialVar_LastTalked = sTrainerObjectEventLocalId;
        gSelectedObjectEvent = GetObjectEventIdByLocalIdAndMap(
            sTrainerObjectEventLocalId as u8,
            (*gSaveBlock1Ptr).location.mapNum as u8,
            (*gSaveBlock1Ptr).location.mapGroup as u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_ConfigureTrainerBattle(data: *mut u8) -> *mut u8 {
    InitTrainerBattleVariables();
    sTrainerBattleMode = TrainerBattleLoadArg8(data) as u16;
    match sTrainerBattleMode {
        TRAINER_BATTLE_SINGLE_NO_INTRO_TEXT => {
            TrainerBattleLoadArgs(sOrdinaryNoIntroBattleParams.as_ptr().cast_mut(), data);
            return EventScript_DoNoIntroTrainerBattle.as_ptr().cast_mut();
        }
        TRAINER_BATTLE_DOUBLE => {
            TrainerBattleLoadArgs(sDoubleBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            return EventScript_TryDoDoubleTrainerBattle.as_ptr().cast_mut();
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
            return EventScript_TryDoNormalTrainerBattle.as_ptr().cast_mut();
        }
        TRAINER_BATTLE_CONTINUE_SCRIPT_NO_MUSIC => {
            TrainerBattleLoadArgs(sContinueScriptBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            return EventScript_TryDoNormalTrainerBattle.as_ptr().cast_mut();
        }
        TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE | TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE_NO_MUSIC => {
            TrainerBattleLoadArgs(sContinueScriptDoubleBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            return EventScript_TryDoDoubleTrainerBattle.as_ptr().cast_mut();
        }
        TRAINER_BATTLE_REMATCH_DOUBLE => {
            TrainerBattleLoadArgs(sDoubleBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            gTrainerBattleOpponent_A = GetRematchTrainerId(gTrainerBattleOpponent_A);
            return EventScript_TryDoDoubleRematchBattle.as_ptr().cast_mut();
        }
        TRAINER_BATTLE_REMATCH => {
            TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
            SetMapVarsToTrainer();
            gTrainerBattleOpponent_A = GetRematchTrainerId(gTrainerBattleOpponent_A);
            return EventScript_TryDoRematchBattle.as_ptr().cast_mut();
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
            return EventScript_TryDoNormalTrainerBattle.as_ptr().cast_mut();
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
            return EventScript_TryDoNormalTrainerBattle.as_ptr().cast_mut();
        }
        _ => {
            if gApproachingTrainerId == 0 {
                TrainerBattleLoadArgs(sOrdinaryBattleParams.as_ptr().cast_mut(), data);
                SetMapVarsToTrainer();
            } else {
                TrainerBattleLoadArgs(sTrainerBOrdinaryBattleParams.as_ptr().cast_mut(), data);
            }
            return EventScript_TryDoNormalTrainerBattle.as_ptr().cast_mut();
        }
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfigureAndSetUpOneTrainerBattle(
    trainerObjEventId: u8,
    trainerScript: *mut u8,
) {
    gSelectedObjectEvent = trainerObjEventId;
    gSpecialVar_LastTalked = gObjectEvents[trainerObjEventId].localId as u16;
    BattleSetup_ConfigureTrainerBattle(trainerScript.at(1));
    ScriptContext_SetupScript(EventScript_StartTrainerApproach.as_ptr().cast_mut());
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfigureTwoTrainersBattle(trainerObjEventId: u8, trainerScript: *mut u8) {
    gSelectedObjectEvent = trainerObjEventId;
    gSpecialVar_LastTalked = gObjectEvents[trainerObjEventId].localId as u16;
    BattleSetup_ConfigureTrainerBattle(trainerScript.at(1));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpTwoTrainersBattle() {
    ScriptContext_SetupScript(EventScript_StartTrainerApproach.as_ptr().cast_mut());
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerFlagFromScriptPointer(data: *mut u8) -> u32 {
    let mut flag: u32 = TrainerBattleLoadArg16(data.at(2)) as u32;
    return FlagGet(TRAINER_FLAGS_START + flag as u16) as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrainerFacingDirection() {
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gSelectedObjectEvent];
    SetTrainerMovementType(
        objectEvent,
        GetTrainerFacingDirectionMovementType((*objectEvent).facingDirection() as u8),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerBattleMode() -> u8 {
    return sTrainerBattleMode as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerFlag() -> u8 {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        return GetBattlePyramidTrainerFlag(gSelectedObjectEvent);
    } else if InTrainerHill() != 0 {
        return GetHillTrainerFlag(gSelectedObjectEvent);
    } else {
        return FlagGet(GetTrainerAFlag());
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetBattledTrainersFlags() {
    if gTrainerBattleOpponent_B != 0 {
        FlagSet(GetTrainerBFlag());
    }
    FlagSet(GetTrainerAFlag());
}
pub(crate) unsafe extern "C" fn SetBattledTrainerFlag() {
    FlagSet(GetTrainerAFlag());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasTrainerBeenFought(trainerId: u16) -> u8 {
    return FlagGet(TRAINER_FLAGS_START + trainerId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrainerFlag(trainerId: u16) {
    FlagSet(TRAINER_FLAGS_START + trainerId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTrainerFlag(trainerId: u16) {
    FlagClear(TRAINER_FLAGS_START + trainerId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartTrainerBattle() {
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
    sShouldCheckTrainerBScript = FALSE;
    gWhichTrainerToFaceAfterBattle = 0;
    gMain.savedCallback = Some(CB2_EndTrainerBattle);
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE || InTrainerHillChallenge() != 0 {
        DoBattlePyramidTrainerHillBattle();
    } else {
        DoTrainerBattle();
    }
    ScriptContext_Stop();
}
pub(crate) unsafe extern "C" fn CB2_EndTrainerBattle() {
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
pub(crate) unsafe extern "C" fn CB2_EndRematchBattle() {
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
pub unsafe extern "C" fn BattleSetup_StartRematchBattle() {
    gBattleTypeFlags = BATTLE_TYPE_TRAINER;
    gMain.savedCallback = Some(CB2_EndRematchBattle);
    DoTrainerBattle();
    ScriptContext_Stop();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerIntroSpeech() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_GetScriptAddrAfterBattle() -> *mut u8 {
    if !sTrainerBattleEndScript.is_null() {
        return sTrainerBattleEndScript;
    } else {
        return EventScript_TestSignpostMsg.as_ptr().cast_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_GetTrainerPostBattleScript() -> *mut u8 {
    if sShouldCheckTrainerBScript != 0 {
        sShouldCheckTrainerBScript = FALSE;
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
    return EventScript_TryGetTrainerScript.as_ptr().cast_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerCantBattleSpeech() {
    ShowFieldMessage(GetTrainerCantBattleSpeech());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTrainerEncounterMusic() {
    let mut trainerId: u16 = 0;
    let mut music: u16 = 0;
    if gApproachingTrainerId == 0 {
        trainerId = gTrainerBattleOpponent_A;
    } else {
        trainerId = gTrainerBattleOpponent_B;
    }
    if sTrainerBattleMode != TRAINER_BATTLE_CONTINUE_SCRIPT_NO_MUSIC
        && sTrainerBattleMode != TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE_NO_MUSIC
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
pub(crate) unsafe extern "C" fn ReturnEmptyStringIfNull(string: *mut u8) -> *mut u8 {
    if string.is_null() {
        return gText_EmptyString2.as_ptr().cast_mut();
    } else {
        return string;
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn GetIntroSpeechOfApproachingTrainer() -> *mut u8 {
    if gApproachingTrainerId == 0 {
        return ReturnEmptyStringIfNull(sTrainerAIntroSpeech);
    } else {
        return ReturnEmptyStringIfNull(sTrainerBIntroSpeech);
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerALoseText() -> *mut u8 {
    let mut string: *mut u8 = null_mut();
    if gTrainerBattleOpponent_A == TRAINER_SECRET_BASE {
        string = GetSecretBaseTrainerLoseText();
    } else {
        string = sTrainerADefeatSpeech;
    }
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), ReturnEmptyStringIfNull(string));
    return gStringVar4.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerBLoseText() -> *mut u8 {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        ReturnEmptyStringIfNull(sTrainerBDefeatSpeech),
    );
    return gStringVar4.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerWonSpeech() -> *mut u8 {
    return ReturnEmptyStringIfNull(sTrainerVictorySpeech);
}
pub(crate) unsafe extern "C" fn GetTrainerCantBattleSpeech() -> *mut u8 {
    return ReturnEmptyStringIfNull(sTrainerCannotBattleSpeech);
}
pub(crate) unsafe extern "C" fn FirstBattleTrainerIdToRematchTableId(
    table: *mut RematchTrainer,
    trainerId: u16,
) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < REMATCH_TABLE_ENTRIES {
        if (*table.at(i)).trainerIds[0] == trainerId {
            return i;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn TrainerIdToRematchTableId(
    table: *mut RematchTrainer,
    trainerId: u16,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < REMATCH_TABLE_ENTRIES {
        j = 0;
        while j < REMATCHES_COUNT {
            if (*table.at(i)).trainerIds[j] == 0 {
                break;
            }
            if (*table.at(i)).trainerIds[j] == trainerId {
                return i;
            }
            j += 1;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn IsRematchForbidden(rematchTableId: i32) -> u32 {
    if rematchTableId >= REMATCH_SIDNEY {
        return TRUE as u32;
    } else if rematchTableId == REMATCH_WALLY_VR as i32 {
        return (FlagGet(FLAG_DEFEATED_WALLY_VICTORY_ROAD) == 0) as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetRematchIdForTrainer(table: *mut RematchTrainer, tableId: u32) {
    let mut i: i32 = 0;
    i = 1;
    while i < REMATCHES_COUNT {
        let mut trainerId: u16 = (*table.at(tableId)).trainerIds[i];
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
pub(crate) unsafe extern "C" fn UpdateRandomTrainerRematches(
    table: *mut RematchTrainer,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    let mut i: i32 = 0;
    let mut ret: u32 = FALSE as u32;
    i = 0;
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
    return ret;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRematchIfDefeated(rematchTableId: i32) {
    if HasTrainerBeenFought(gRematchTable[rematchTableId].trainerIds[0]) == TRUE {
        SetRematchIdForTrainer(gRematchTable.as_ptr().cast_mut(), rematchTableId as u32);
    }
}
pub(crate) unsafe extern "C" fn DoesSomeoneWantRematchIn_(
    table: *mut RematchTrainer,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < REMATCH_TABLE_ENTRIES {
        if (*table.at(i)).mapGroup == mapGroup
            && (*table.at(i)).mapNum == mapNum
            && (*gSaveBlock1Ptr).trainerRematches[i] != 0
        {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn IsRematchTrainerIn_(
    table: *mut RematchTrainer,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < REMATCH_TABLE_ENTRIES {
        if (*table.at(i)).mapGroup == mapGroup && (*table.at(i)).mapNum == mapNum {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn IsFirstTrainerIdReadyForRematch(
    table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u8 {
    let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE;
    }
    if tableId >= MAX_REMATCH_ENTRIES {
        return FALSE;
    }
    if (*gSaveBlock1Ptr).trainerRematches[tableId] == 0 {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsTrainerReadyForRematch_(
    table: *mut RematchTrainer,
    trainerId: u16,
) -> u8 {
    let mut tableId: i32 = TrainerIdToRematchTableId(table, trainerId);
    if tableId == -1 {
        return FALSE;
    }
    if tableId >= MAX_REMATCH_ENTRIES {
        return FALSE;
    }
    if (*gSaveBlock1Ptr).trainerRematches[tableId] == 0 {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetRematchTrainerIdFromTable(
    mut table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u16 {
    let mut trainerEntry: *mut RematchTrainer = null_mut();
    let mut i: i32 = 0;
    let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE as u16;
    }
    trainerEntry = table.at(tableId);
    i = 1;
    while i < REMATCHES_COUNT {
        if (*trainerEntry).trainerIds[i] == 0 {
            return (*trainerEntry).trainerIds[i - 1];
        }
        if HasTrainerBeenFought((*trainerEntry).trainerIds[i]) == 0 {
            return (*trainerEntry).trainerIds[i];
        }
        i += 1;
    }
    return (*trainerEntry).trainerIds[4];
}
pub(crate) unsafe extern "C" fn GetLastBeatenRematchTrainerIdFromTable(
    mut table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u16 {
    let mut trainerEntry: *mut RematchTrainer = null_mut();
    let mut i: i32 = 0;
    let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE as u16;
    }
    trainerEntry = table.at(tableId);
    i = 1;
    while i < REMATCHES_COUNT {
        if (*trainerEntry).trainerIds[i] == 0 {
            return (*trainerEntry).trainerIds[i - 1];
        }
        if HasTrainerBeenFought((*trainerEntry).trainerIds[i]) == 0 {
            return (*trainerEntry).trainerIds[i - 1];
        }
        i += 1;
    }
    return (*trainerEntry).trainerIds[4];
}
pub(crate) unsafe extern "C" fn ClearTrainerWantRematchState(
    table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) {
    let mut tableId: i32 = TrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId != -1 {
        (*gSaveBlock1Ptr).trainerRematches[tableId] = 0;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerMatchCallFlag(trainerId: u32) -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < REMATCH_TABLE_ENTRIES {
        if gRematchTable[i].trainerIds[0] as u32 == trainerId {
            return TRAINER_REGISTERED_FLAGS_START as u32 + i as u32;
        }
        i += 1;
    }
    return 0xFFFF;
}
pub(crate) unsafe extern "C" fn RegisterTrainerInMatchCall() {
    if FlagGet(FLAG_HAS_MATCH_CALL) != 0 {
        let mut matchCallFlagId: u32 = GetTrainerMatchCallFlag(gTrainerBattleOpponent_A as u32);
        if matchCallFlagId != 0xFFFF {
            FlagSet(matchCallFlagId as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn WasSecondRematchWon(
    table: *mut RematchTrainer,
    firstBattleTrainerId: u16,
) -> u8 {
    let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
    if tableId == -1 {
        return FALSE;
    }
    if HasTrainerBeenFought((*table.at(tableId)).trainerIds[1]) == 0 {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn HasAtLeastFiveBadges() -> u32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    count = 0;
    i = 0;
    while i < 8 {
        if FlagGet(sBadgeFlags[i]) == TRUE {
            if ({
                count += 1;
                count
            }) >= 5
            {
                return TRUE as u32;
            }
        }
        i += 1;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementRematchStepCounter() {
    if HasAtLeastFiveBadges() != 0 {
        if (*gSaveBlock1Ptr).trainerRematchStepCounter >= STEP_COUNTER_MAX {
            (*gSaveBlock1Ptr).trainerRematchStepCounter = STEP_COUNTER_MAX;
        } else {
            (*gSaveBlock1Ptr).trainerRematchStepCounter += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn IsRematchStepCounterMaxed() -> u32 {
    if HasAtLeastFiveBadges() != 0
        && (*gSaveBlock1Ptr).trainerRematchStepCounter >= STEP_COUNTER_MAX
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryUpdateRandomTrainerRematches(mapGroup: u16, mapNum: u16) {
    if IsRematchStepCounterMaxed() != 0
        && UpdateRandomTrainerRematches(gRematchTable.as_ptr().cast_mut(), mapGroup, mapNum)
            == TRUE as u32
    {
        (*gSaveBlock1Ptr).trainerRematchStepCounter = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesSomeoneWantRematchIn(mapGroup: u16, mapNum: u16) -> u32 {
    return DoesSomeoneWantRematchIn_(gRematchTable.as_ptr().cast_mut(), mapGroup, mapNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRematchTrainerIn(mapGroup: u16, mapNum: u16) -> u32 {
    return IsRematchTrainerIn_(gRematchTable.as_ptr().cast_mut(), mapGroup, mapNum);
}
pub(crate) unsafe extern "C" fn GetRematchTrainerId(trainerId: u16) -> u16 {
    return GetRematchTrainerIdFromTable(gRematchTable.as_ptr().cast_mut(), trainerId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLastBeatenRematchTrainerId(trainerId: u16) -> u16 {
    return GetLastBeatenRematchTrainerIdFromTable(gRematchTable.as_ptr().cast_mut(), trainerId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldTryRematchBattle() -> u8 {
    if IsFirstTrainerIdReadyForRematch(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A)
        != 0
    {
        return TRUE;
    }
    return WasSecondRematchWon(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTrainerReadyForRematch() -> u8 {
    return IsTrainerReadyForRematch_(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A);
}
pub(crate) unsafe extern "C" fn HandleRematchVarsOnBattleEnd() {
    ClearTrainerWantRematchState(gRematchTable.as_ptr().cast_mut(), gTrainerBattleOpponent_A);
    SetBattledTrainersFlags();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldTryGetTrainerScript() {
    if sNoOfPossibleTrainerRetScripts > 1 {
        sNoOfPossibleTrainerRetScripts = 0;
        sShouldCheckTrainerBScript = TRUE;
        gSpecialVar_Result = TRUE as u16;
    } else {
        sShouldCheckTrainerBScript = FALSE;
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountBattledRematchTeams(trainerId: u16) -> u16 {
    let mut i: i32 = 0;
    if HasTrainerBeenFought(gRematchTable[trainerId].trainerIds[0]) != TRUE {
        return 0;
    }
    i = 1;
    while i < REMATCHES_COUNT {
        if gRematchTable[trainerId].trainerIds[i] == 0 {
            break;
        }
        if HasTrainerBeenFought(gRematchTable[trainerId].trainerIds[i]) == 0 {
            break;
        }
        i += 1;
    }
    return i as u16;
}
