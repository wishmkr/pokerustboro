//! Translated from `src/battle_setup.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBattleTransitionTable_Wild sBattleTransitionTable_Trainer sBattleTransitionTable_BattleFrontier sBattleTransitionTable_BattlePyramid sBattleTransitionTable_BattleDome sOrdinaryBattleParams sContinueScriptBattleParams sDoubleBattleParams sOrdinaryNoIntroBattleParams sContinueScriptDoubleBattleParams sTrainerBOrdinaryBattleParams sTrainerBContinueScriptBattleParams gRematchTable sBadgeFlags
#[allow(unused_imports)]
use crate::data::battle_setup::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBattleMode: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerBattleOpponent_A: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerBattleOpponent_B: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPartnerTrainerId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerObjectEventLocalId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerAIntroSpeech: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBIntroSpeech: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerADefeatSpeech: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBDefeatSpeech: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerVictorySpeech: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerCannotBattleSpeech: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBattleEndScript: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerABattleScriptRetAddr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTrainerBBattleScriptRetAddr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sShouldCheckTrainerBScript: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNoOfPossibleTrainerRetScripts: u8 = 0u8;

unsafe extern "C" {
    static mut EventScript_DoNoIntroTrainerBattle: u8;
    static mut EventScript_StartTrainerApproach: u8;
    static mut EventScript_TestSignpostMsg: u8;
    static mut EventScript_TryDoDoubleRematchBattle: u8;
    static mut EventScript_TryDoDoubleTrainerBattle: u8;
    static mut EventScript_TryDoNormalTrainerBattle: u8;
    static mut EventScript_TryDoRematchBattle: u8;
    static mut EventScript_TryGetTrainerScript: u8;
    static mut gApproachingTrainerId: u8;
    static mut gApproachingTrainers: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u8;
    static mut gEnemyParty: u8;
    static mut gFieldCallback: u8;
    static mut gGameVersion: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gNoOfApproachingTrainers: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedObjectEvent: u8;
    static mut gSpecialVar_LastTalked: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_EmptyString2: u8;
    static mut gTrainers: u8;
    static mut gWhichTrainerToFaceAfterBattle: u8;
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
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMaleMon(a0: *mut u8, a1: u16, a2: u8);
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
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
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
    fn SetTrainerMovementType(a0: *mut u8, a1: u8);
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessageFromBuffer() -> u8;
    fn StopPlayerAvatar();
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn UpdateGymLeaderRematch();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroMonData(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn Task_BattleStart(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if !((FldEffPoison_IsActive()) != 0) {
                    BattleTransition_StartOnField(((((data).wrapping_offset(1)).read()) as u8));
                    ClearMirageTowerPulseBlendEffect();
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((IsBattleTransitionDone()) as i32) == 1i32 {
                    CleanupOverworldWindowsAndTilemaps();
                    SetMainCallback2(Some(CB2_InitBattle));
                    RestartWildEncounterImmunitySteps();
                    ClearPoisonStepCounter();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateBattleStartTask(transition: u8, song: u16) {
    unsafe {
        let mut transition = transition;
        let mut song = song;
        let mut taskId: u8 = CreateTask(Some(Task_BattleStart), 1u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((transition) as i16));
        PlayMapChosenOrBattleBGM(song);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartWildBattle() {
    unsafe {
        if (GetSafariZoneFlag()) != 0 {
            DoSafariBattle();
        } else {
            DoStandardWildBattle();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartBattlePikeWildBattle() {
    unsafe {
        DoBattlePikeWildBattle();
    }
}
pub(crate) unsafe extern "C" fn DoStandardWildBattle() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        StopPlayerAvatar();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            VarSet(16398u16, 0u16);
            let __p1 = (&raw mut gBattleTypeFlags).cast::<u32>();
            (__p1).write(((__p1).read() | 2097152u32));
        }
        CreateBattleStartTask(GetWildBattleTransition(), 0u16);
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartRoamerBattle() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        StopPlayerAvatar();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(1024u32);
        CreateBattleStartTask(GetWildBattleTransition(), 0u16);
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
pub(crate) unsafe extern "C" fn DoSafariBattle() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        StopPlayerAvatar();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndSafariBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(128u32);
        CreateBattleStartTask(GetWildBattleTransition(), 0u16);
    }
}
pub(crate) unsafe extern "C" fn DoBattlePikeWildBattle() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        StopPlayerAvatar();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(1048576u32);
        CreateBattleStartTask(GetWildBattleTransition(), 0u16);
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
pub(crate) unsafe extern "C" fn DoTrainerBattle() {
    unsafe {
        CreateBattleStartTask(GetTrainerBattleTransition(), 0u16);
        IncrementGameStat(7u8);
        IncrementGameStat(9u8);
        TryUpdateGymLeaderRematchFromTrainer();
    }
}
pub(crate) unsafe extern "C" fn DoBattlePyramidTrainerHillBattle() {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            CreateBattleStartTask(GetSpecialBattleTransition(10i32), 0u16);
        } else {
            CreateBattleStartTask(GetSpecialBattleTransition(11i32), 0u16);
        }
        IncrementGameStat(7u8);
        IncrementGameStat(9u8);
        TryUpdateGymLeaderRematchFromTrainer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartWallyTutorialBattle() {
    unsafe {
        CreateMaleMon((&raw mut gEnemyParty).cast::<u8>(), 392u16, 5u8);
        LockPlayerFieldControls();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(512u32);
        CreateBattleStartTask(8u8, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartScriptedWildBattle() {
    unsafe {
        LockPlayerFieldControls();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndScriptedWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        CreateBattleStartTask(GetWildBattleTransition(), 0u16);
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartLatiBattle() {
    unsafe {
        LockPlayerFieldControls();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndScriptedWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(8192u32);
        CreateBattleStartTask(GetWildBattleTransition(), 0u16);
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartLegendaryBattle() {
    unsafe {
        LockPlayerFieldControls();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndScriptedWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(8192u32);
        'l1: {
            let __sw1 = GetMonData3(
                (&raw mut gEnemyParty).cast::<u8>(),
                11i32,
                core::ptr::null_mut(),
            );
            let __matched = __sw1 == 405u32
                || __sw1 == 404u32
                || __sw1 == 406u32
                || __sw1 == 410u32
                || __sw1 == 249u32
                || __sw1 == 250u32
                || __sw1 == 151u32;
            if __sw1 == 405u32 || !__matched {
                let __p2 = (&raw mut gBattleTypeFlags).cast::<u32>();
                (__p2).write(((__p2).read() | 268435456u32));
                CreateBattleStartTask(23u8, 480u16);
                break 'l1;
            }
            if __sw1 == 404u32 {
                let __p3 = (&raw mut gBattleTypeFlags).cast::<u32>();
                (__p3).write(((__p3).read() | 536870912u32));
                CreateBattleStartTask(22u8, 480u16);
                break 'l1;
            }
            if __sw1 == 406u32 {
                let __p4 = (&raw mut gBattleTypeFlags).cast::<u32>();
                (__p4).write(((__p4).read() | 1073741824u32));
                CreateBattleStartTask(24u8, 470u16);
                break 'l1;
            }
            if __sw1 == 410u32 {
                CreateBattleStartTask(0u8, 551u16);
                break 'l1;
            }
            if __sw1 == 249u32 || __sw1 == 250u32 {
                CreateBattleStartTask(0u8, 553u16);
                break 'l1;
            }
            if __sw1 == 151u32 {
                CreateBattleStartTask(10u8, 472u16);
                break 'l1;
            }
        }
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartGroudonKyogreBattle() {
    unsafe {
        LockPlayerFieldControls();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndScriptedWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(12288u32);
        if ((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) == 2i32 {
            CreateBattleStartTask(11u8, 480u16);
        } else {
            CreateBattleStartTask(6u8, 480u16);
        }
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartRegiBattle() {
    unsafe {
        let mut transitionId: u8 = 0u8;
        let mut species: u16 = 0u16;
        LockPlayerFieldControls();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndScriptedWildBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(24576u32);
        species = ((GetMonData2((&raw mut gEnemyParty).cast::<u8>(), 11i32)) as u16);
        'l1: {
            let __sw1 = ((species) as i32);
            let __matched = __sw1 == 401i32 || __sw1 == 402i32 || __sw1 == 403i32;
            if __sw1 == 401i32 {
                transitionId = 21u8;
                break 'l1;
            }
            if __sw1 == 402i32 {
                transitionId = 19u8;
                break 'l1;
            }
            if __sw1 == 403i32 {
                transitionId = 20u8;
                break 'l1;
            }
            if !__matched {
                transitionId = 10u8;
                break 'l1;
            }
        }
        CreateBattleStartTask(transitionId, 479u16);
        IncrementGameStat(7u8);
        IncrementGameStat(8u8);
        IncrementDailyWildBattles();
        TryUpdateGymLeaderRematchFromWild();
    }
}
pub(crate) unsafe extern "C" fn CB2_EndWildBattle() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((83886080i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(512i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        ResetOamRange(0u8, 128u8);
        if ((IsPlayerDefeated(((((&raw mut gBattleOutcome).cast::<u8>()).read()) as u32)) == 1u32)
            && (((CurrentBattlePyramidLocation()) as i32) == 0i32))
            && (!((InBattlePike()) != 0))
        {
            SetMainCallback2(Some(CB2_WhiteOut));
        } else {
            SetMainCallback2(Some(CB2_ReturnToField));
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_ReturnToFieldNoScriptCheckMusic));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_EndScriptedWildBattle() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((83886080i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(512i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        ResetOamRange(0u8, 128u8);
        if IsPlayerDefeated(((((&raw mut gBattleOutcome).cast::<u8>()).read()) as u32)) == 1u32 {
            if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
            } else {
                SetMainCallback2(Some(CB2_WhiteOut));
            }
        } else {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_GetEnvironmentId() -> u8 {
    unsafe {
        let mut tileBehavior: u16 = 0u16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        tileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16);
        if (MetatileBehavior_IsTallGrass(((tileBehavior) as u8))) != 0 {
            return 0u8;
        }
        if (MetatileBehavior_IsLongGrass(((tileBehavior) as u8))) != 0 {
            return 1u8;
        }
        if (MetatileBehavior_IsSandOrDeepSand(((tileBehavior) as u8))) != 0 {
            return 2u8;
        }
        'l1: {
            let __sw1 = (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (MetatileBehavior_IsIndoorEncounter(((tileBehavior) as u8))) != 0 {
                    return 8u8;
                }
                if (MetatileBehavior_IsSurfableWaterOrUnderwater(((tileBehavior) as u8))) != 0 {
                    return 5u8;
                }
                return 7u8;
            }
            if __sw1 == 8i32 || __sw1 == 9i32 {
                return 8u8;
            }
            if __sw1 == 5i32 {
                return 3u8;
            }
            if __sw1 == 6i32 {
                if (MetatileBehavior_IsSurfableWaterOrUnderwater(((tileBehavior) as u8))) != 0 {
                    return 4u8;
                }
                return 9u8;
            }
        }
        if (MetatileBehavior_IsDeepOrOceanWater(((tileBehavior) as u8))) != 0 {
            return 4u8;
        }
        if (MetatileBehavior_IsSurfableWaterOrUnderwater(((tileBehavior) as u8))) != 0 {
            return 5u8;
        }
        if (MetatileBehavior_IsMountain(((tileBehavior) as u8))) != 0 {
            return 6u8;
        }
        if (TestPlayerAvatarFlags(8u8)) != 0 {
            if ((MetatileBehavior_GetBridgeType(((tileBehavior) as u8))) as i32) != 0i32 {
                return 5u8;
            }
            if ((MetatileBehavior_IsBridgeOverWater(((tileBehavior) as u8))) as i32) == 1i32 {
                return 4u8;
            }
        }
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 28i32)
        {
            return 2u8;
        }
        if ((GetSavedWeather()) as i32) == 8i32 {
            return 2u8;
        }
        return 9u8;
    }
}
pub(crate) unsafe extern "C" fn GetBattleTransitionTypeByMap() -> u8 {
    unsafe {
        let mut tileBehavior: u16 = 0u16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        tileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16);
        if (GetFlashLevel()) != 0 {
            return 2u8;
        }
        if (MetatileBehavior_IsSurfableWaterOrUnderwater(((tileBehavior) as u8))) != 0 {
            return 3u8;
        }
        'l1: {
            let __sw1 = (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
            let __matched = __sw1 == 4i32 || __sw1 == 5i32;
            if __sw1 == 4i32 {
                return 1u8;
            }
            if __sw1 == 5i32 {
                return 3u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetSumOfPlayerPartyLevel(numMons: u8) -> u16 {
    unsafe {
        let mut numMons = numMons;
        let mut sum: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u32 = GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        65i32,
                    );
                    if ((species != 412u32) && (species != 0u32))
                        && (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            57i32,
                        ) != 0u32)
                    {
                        sum = ((((sum) as u32).wrapping_add(GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            56i32,
                        ))) as u8);
                        if (({
                            let __t1 = (numMons).wrapping_sub(1);
                            numMons = __t1;
                            __t1
                        }) as i32)
                            == 0i32
                        {
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((sum) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetSumOfEnemyPartyLevel(opponentId: u16, numMons: u8) -> u8 {
    unsafe {
        let mut opponentId = opponentId;
        let mut numMons = numMons;
        let mut i: u8 = 0u8;
        let mut sum: u8 = 0u8;
        let mut count: u32 = ((numMons) as u32);
        if ((((((&raw mut gTrainers).cast::<u8>())
            .wrapping_offset(((opponentId) as i32) as isize * 40))
        .wrapping_add(32))
        .read()) as u32)
            < count
        {
            count = ((((((&raw mut gTrainers).cast::<u8>())
                .wrapping_offset(((opponentId) as i32) as isize * 40))
            .wrapping_add(32))
            .read()) as u32);
        }
        sum = 0u8;
        'l1: {
            let __sw1 = (((((&raw mut gTrainers).cast::<u8>())
                .wrapping_offset(((opponentId) as i32) as isize * 40))
            .read()) as i32);
            if __sw1 == 0i32 {
                {
                    let mut party: *mut u8 = core::ptr::null_mut();
                    party = (((((&raw mut gTrainers).cast::<u8>())
                        .wrapping_offset(((opponentId) as i32) as isize * 40))
                    .wrapping_add(36))
                    .cast::<*mut u8>())
                    .read();
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as u32) < count) {
                                break 'l2;
                            }
                            'l3: {
                                sum = ((((sum) as i32).wrapping_add(
                                    (((((party).wrapping_offset(((i) as i32) as isize * 8))
                                        .wrapping_add(2))
                                    .read()) as i32),
                                )) as u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    let mut party: *mut u8 = core::ptr::null_mut();
                    party = (((((&raw mut gTrainers).cast::<u8>())
                        .wrapping_offset(((opponentId) as i32) as isize * 40))
                    .wrapping_add(36))
                    .cast::<*mut u8>())
                    .read();
                    {
                        i = 0u8;
                        'l4: loop {
                            if !(((i) as u32) < count) {
                                break 'l4;
                            }
                            'l5: {
                                sum = ((((sum) as i32).wrapping_add(
                                    (((((party).wrapping_offset(((i) as i32) as isize * 16))
                                        .wrapping_add(2))
                                    .read()) as i32),
                                )) as u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    let mut party: *mut u8 = core::ptr::null_mut();
                    party = (((((&raw mut gTrainers).cast::<u8>())
                        .wrapping_offset(((opponentId) as i32) as isize * 40))
                    .wrapping_add(36))
                    .cast::<*mut u8>())
                    .read();
                    {
                        i = 0u8;
                        'l6: loop {
                            if !(((i) as u32) < count) {
                                break 'l6;
                            }
                            'l7: {
                                sum = ((((sum) as i32).wrapping_add(
                                    (((((party).wrapping_offset(((i) as i32) as isize * 8))
                                        .wrapping_add(2))
                                    .read()) as i32),
                                )) as u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    let mut party: *mut u8 = core::ptr::null_mut();
                    party = (((((&raw mut gTrainers).cast::<u8>())
                        .wrapping_offset(((opponentId) as i32) as isize * 40))
                    .wrapping_add(36))
                    .cast::<*mut u8>())
                    .read();
                    {
                        i = 0u8;
                        'l8: loop {
                            if !(((i) as u32) < count) {
                                break 'l8;
                            }
                            'l9: {
                                sum = ((((sum) as i32).wrapping_add(
                                    (((((party).wrapping_offset(((i) as i32) as isize * 16))
                                        .wrapping_add(2))
                                    .read()) as i32),
                                )) as u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
        }
        return sum;
    }
}
pub(crate) unsafe extern "C" fn GetWildBattleTransition() -> u8 {
    unsafe {
        let mut transitionType: u8 = GetBattleTransitionTypeByMap();
        let mut enemyLevel: u8 = ((GetMonData2((&raw mut gEnemyParty).cast::<u8>(), 56i32)) as u8);
        let mut playerLevel: u8 = ((GetSumOfPlayerPartyLevel(1u8)) as u8);
        if ((enemyLevel) as i32) < ((playerLevel) as i32) {
            if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                return 0u8;
            } else {
                return (((((&raw const sBattleTransitionTable_Wild)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((transitionType) as i32) as isize * 2))
                .cast::<u8>())
                .read();
            }
        } else {
            if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                return 10u8;
            } else {
                return ((((((&raw const sBattleTransitionTable_Wild)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((transitionType) as i32) as isize * 2))
                .cast::<u8>())
                .wrapping_offset(1))
                .read();
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetTrainerBattleTransition() -> u8 {
    unsafe {
        let mut minPartyCount: u8 = 0u8;
        let mut transitionType: u8 = 0u8;
        let mut enemyLevel: u8 = 0u8;
        let mut playerLevel: u8 = 0u8;
        if ((((&raw mut gTrainerBattleOpponent_A)
            .cast::<u8>()
            .cast::<u16>())
        .read()) as i32)
            == 1024i32
        {
            return 16u8;
        }
        if ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 31i32
        {
            if ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                == 261i32
            {
                return 12u8;
            }
            if ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                == 262i32
            {
                return 13u8;
            }
            if ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                == 263i32
            {
                return 14u8;
            }
            if ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                == 264i32
            {
                return 15u8;
            }
            return 16u8;
        }
        if ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 38i32
        {
            return 16u8;
        }
        if ((((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 9i32)
            || (((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                ((((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(1))
            .read()) as i32)
                == 53i32))
            || (((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                ((((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(1))
            .read()) as i32)
                == 49i32)
        {
            return 18u8;
        }
        if ((((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 3i32)
            || (((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                ((((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(1))
            .read()) as i32)
                == 13i32))
            || (((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                ((((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(1))
            .read()) as i32)
                == 11i32)
        {
            return 17u8;
        }
        if ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(24))
        .read()) as i32)
            == 1i32
        {
            minPartyCount = 2u8;
        } else {
            minPartyCount = 1u8;
        }
        transitionType = GetBattleTransitionTypeByMap();
        enemyLevel = GetSumOfEnemyPartyLevel(
            ((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
            minPartyCount,
        );
        playerLevel = ((GetSumOfPlayerPartyLevel(minPartyCount)) as u8);
        if ((enemyLevel) as i32) < ((playerLevel) as i32) {
            return (((((&raw const sBattleTransitionTable_Trainer)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((transitionType) as i32) as isize * 2))
            .cast::<u8>())
            .read();
        } else {
            return ((((((&raw const sBattleTransitionTable_Trainer)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((transitionType) as i32) as isize * 2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpecialBattleTransition(id: i32) -> u8 {
    unsafe {
        let mut id = id;
        let mut var: u16 = 0u16;
        let mut enemyLevel: u8 = ((GetMonData2((&raw mut gEnemyParty).cast::<u8>(), 56i32)) as u8);
        let mut playerLevel: u8 = ((GetSumOfPlayerPartyLevel(1u8)) as u8);
        if ((enemyLevel) as i32) < ((playerLevel) as i32) {
            'l1: {
                let __sw1 = id;
                if __sw1 == 11i32 || __sw1 == 12i32 || __sw1 == 13i32 {
                    return 4u8;
                }
                if __sw1 == 10i32 {
                    return ((((&raw const sBattleTransitionTable_BattlePyramid)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(3u32, 1u32)))
                            as i32) as isize,
                    ))
                    .read();
                }
                if __sw1 == 3i32 {
                    return ((((&raw const sBattleTransitionTable_BattleDome)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(4u32, 1u32)))
                            as i32) as isize,
                    ))
                    .read();
                }
            }
            if ((VarGet(16590u16)) as i32) != 3i32 {
                return ((((&raw const sBattleTransitionTable_BattleFrontier)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(12u32, 1u32)))
                        as i32) as isize,
                ))
                .read();
            }
        } else {
            'l2: {
                let __sw2 = id;
                if __sw2 == 11i32 || __sw2 == 12i32 || __sw2 == 13i32 {
                    return 3u8;
                }
                if __sw2 == 10i32 {
                    return ((((&raw const sBattleTransitionTable_BattlePyramid)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(3u32, 1u32)))
                            as i32) as isize,
                    ))
                    .read();
                }
                if __sw2 == 3i32 {
                    return ((((&raw const sBattleTransitionTable_BattleDome)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(4u32, 1u32)))
                            as i32) as isize,
                    ))
                    .read();
                }
            }
            if ((VarGet(16590u16)) as i32) != 3i32 {
                return ((((&raw const sBattleTransitionTable_BattleFrontier)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(12u32, 1u32)))
                        as i32) as isize,
                ))
                .read();
            }
        }
        var = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1640))
        .cast::<u16>())
        .wrapping_offset(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_mul(2i32))
            .wrapping_add(0i32)) as isize,
        ))
        .read()) as i32)
            .wrapping_add(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(2i32))
                    .wrapping_add(1i32)) as isize,
                ))
                .read()) as i32),
            )) as u16);
        return ((((&raw const sBattleTransitionTable_BattleFrontier)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::rem_u32(((var) as u32), crate::c::div_u32(12u32, 1u32))) as i32) as isize,
        ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseStarter() {
    unsafe {
        SetMainCallback2(Some(CB2_ChooseStarter));
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_GiveStarter));
    }
}
pub(crate) unsafe extern "C" fn CB2_GiveStarter() {
    unsafe {
        let mut starterMon: u16 = 0u16;
        (GetVarPointer(16419u16)).write(((&raw mut gSpecialVar_Result).cast::<u16>()).read());
        starterMon = GetStarterPokemon(((&raw mut gSpecialVar_Result).cast::<u16>()).read());
        ScriptGiveMon(starterMon, 5u8, 0u16, 0u32, 0u32, 0u8);
        ResetTasks();
        PlayBattleBGM();
        SetMainCallback2(Some(CB2_StartFirstBattle));
        BattleTransition_Start(0u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_StartFirstBattle() {
    unsafe {
        UpdatePaletteFade();
        RunTasks();
        if ((IsBattleTransitionDone()) as i32) == 1i32 {
            ((&raw mut gBattleTypeFlags).cast::<u32>()).write(16u32);
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_EndFirstBattle));
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_InitBattle));
            RestartWildEncounterImmunitySteps();
            ClearPoisonStepCounter();
            IncrementGameStat(7u8);
            IncrementGameStat(8u8);
            IncrementDailyWildBattles();
            TryUpdateGymLeaderRematchFromWild();
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_EndFirstBattle() {
    unsafe {
        Overworld_ClearSavedMusic();
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
pub(crate) unsafe extern "C" fn TryUpdateGymLeaderRematchFromWild() {
    unsafe {
        if crate::c::rem_u32(GetGameStat(8u8), 60u32) == 0u32 {
            UpdateGymLeaderRematch();
        }
    }
}
pub(crate) unsafe extern "C" fn TryUpdateGymLeaderRematchFromTrainer() {
    unsafe {
        if crate::c::rem_u32(GetGameStat(9u8), 20u32) == 0u32 {
            UpdateGymLeaderRematch();
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArg32(ptr: *mut u8) -> u32 {
    unsafe {
        let mut ptr = ptr;
        return (((((((ptr).read()) as i32) | (((((ptr).wrapping_offset(1)).read()) as i32) << 8))
            | (((((ptr).wrapping_offset(2)).read()) as i32) << 16))
            | (((((ptr).wrapping_offset(3)).read()) as i32) << 24)) as u32);
    }
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArg16(ptr: *mut u8) -> u16 {
    unsafe {
        let mut ptr = ptr;
        return (((((ptr).read()) as i32) | (((((ptr).wrapping_offset(1)).read()) as i32) << 8))
            as u16);
    }
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArg8(ptr: *mut u8) -> u8 {
    unsafe {
        let mut ptr = ptr;
        return (ptr).read();
    }
}
pub(crate) unsafe extern "C" fn GetTrainerAFlag() -> u16 {
    unsafe {
        return (((1280i32).wrapping_add(
            ((((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32),
        )) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetTrainerBFlag() -> u16 {
    unsafe {
        return (((1280i32).wrapping_add(
            ((((&raw mut gTrainerBattleOpponent_B)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32),
        )) as u16);
    }
}
pub(crate) unsafe extern "C" fn IsPlayerDefeated(battleOutcome: u32) -> u32 {
    unsafe {
        let mut battleOutcome = battleOutcome;
        'l1: {
            let __sw1 = battleOutcome;
            let __matched = __sw1 == 2u32
                || __sw1 == 3u32
                || __sw1 == 1u32
                || __sw1 == 4u32
                || __sw1 == 5u32
                || __sw1 == 6u32
                || __sw1 == 7u32;
            if __sw1 == 2u32 || __sw1 == 3u32 {
                return 1u32;
            }
            if __sw1 == 1u32 || __sw1 == 4u32 || __sw1 == 5u32 || __sw1 == 6u32 || __sw1 == 7u32 {
                return 0u32;
            }
            if !__matched {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTrainerOpponentIds() {
    unsafe {
        ((&raw mut gTrainerBattleOpponent_A)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut gTrainerBattleOpponent_B)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn InitTrainerBattleVariables() {
    unsafe {
        ((&raw mut sTrainerBattleMode).cast::<u8>().cast::<u16>()).write(0u16);
        if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
            ((&raw mut sTrainerAIntroSpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
            ((&raw mut sTrainerADefeatSpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
            ((&raw mut sTrainerABattleScriptRetAddr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        } else {
            ((&raw mut sTrainerBIntroSpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
            ((&raw mut sTrainerBDefeatSpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
            ((&raw mut sTrainerBBattleScriptRetAddr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        ((&raw mut sTrainerObjectEventLocalId)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sTrainerVictorySpeech)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((&raw mut sTrainerCannotBattleSpeech)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((&raw mut sTrainerBattleEndScript)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn SetU8(ptr: *mut u8, value: u8) {
    unsafe {
        let mut ptr = ptr;
        let mut value = value;
        (ptr).write(value);
    }
}
pub(crate) unsafe extern "C" fn SetU16(ptr: *mut u8, value: u16) {
    unsafe {
        let mut ptr = ptr;
        let mut value = value;
        ((ptr).cast::<u16>()).write(value);
    }
}
pub(crate) unsafe extern "C" fn SetU32(ptr: *mut u8, value: u32) {
    unsafe {
        let mut ptr = ptr;
        let mut value = value;
        ((ptr).cast::<u32>()).write(value);
    }
}
pub(crate) unsafe extern "C" fn SetPtr(ptr: *mut u8, value: *mut u8) {
    unsafe {
        let mut ptr = ptr;
        let mut value = value;
        ((ptr).cast::<*mut u8>()).write(value);
    }
}
pub(crate) unsafe extern "C" fn TrainerBattleLoadArgs(specs: *mut u8, data: *mut u8) {
    unsafe {
        let mut specs = specs;
        let mut data = data;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            'l2: {
                let __sw1 = ((((specs).wrapping_add(4)).read()) as i32);
                if __sw1 == 0i32 {
                    SetU8(
                        ((specs).cast::<*mut u8>()).read(),
                        TrainerBattleLoadArg8(data),
                    );
                    data = (data).wrapping_offset(1);
                    break 'l2;
                }
                if __sw1 == 1i32 {
                    SetU16(
                        ((specs).cast::<*mut u8>()).read(),
                        TrainerBattleLoadArg16(data),
                    );
                    data = (data).wrapping_offset(2);
                    break 'l2;
                }
                if __sw1 == 2i32 {
                    SetU32(
                        ((specs).cast::<*mut u8>()).read(),
                        TrainerBattleLoadArg32(data),
                    );
                    data = (data).wrapping_offset(4);
                    break 'l2;
                }
                if __sw1 == 3i32 {
                    SetU8(((specs).cast::<*mut u8>()).read(), 0u8);
                    break 'l2;
                }
                if __sw1 == 4i32 {
                    SetU16(((specs).cast::<*mut u8>()).read(), 0u16);
                    break 'l2;
                }
                if __sw1 == 5i32 {
                    SetU32(((specs).cast::<*mut u8>()).read(), 0u32);
                    break 'l2;
                }
                if __sw1 == 6i32 {
                    SetPtr(((specs).cast::<*mut u8>()).read(), data);
                    return;
                }
            }
            specs = (specs).wrapping_offset(8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMapVarsToTrainer() {
    unsafe {
        if ((((&raw mut sTrainerObjectEventLocalId)
            .cast::<u8>()
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).write(
                ((&raw mut sTrainerObjectEventLocalId)
                    .cast::<u8>()
                    .cast::<u16>())
                .read(),
            );
            ((&raw mut gSelectedObjectEvent).cast::<u8>()).write(GetObjectEventIdByLocalIdAndMap(
                ((((&raw mut sTrainerObjectEventLocalId)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as u8),
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as u8),
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as u8),
            ));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_ConfigureTrainerBattle(data: *mut u8) -> *mut u8 {
    unsafe {
        let mut data = data;
        InitTrainerBattleVariables();
        ((&raw mut sTrainerBattleMode).cast::<u8>().cast::<u16>())
            .write(((TrainerBattleLoadArg8(data)) as u16));
        'l1: {
            let __sw1 =
                ((((&raw mut sTrainerBattleMode).cast::<u8>().cast::<u16>()).read()) as i32);
            let __matched = __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 2i32
                || __sw1 == 1i32
                || __sw1 == 6i32
                || __sw1 == 8i32
                || __sw1 == 7i32
                || __sw1 == 5i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32;
            if __sw1 == 3i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sOrdinaryNoIntroBattleParams)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    data,
                );
                return (&raw mut EventScript_DoNoIntroTrainerBattle).cast::<u8>();
            }
            if __sw1 == 4i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sDoubleBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                    data,
                );
                SetMapVarsToTrainer();
                return (&raw mut EventScript_TryDoDoubleTrainerBattle).cast::<u8>();
            }
            if __sw1 == 2i32 {
                if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
                    TrainerBattleLoadArgs(
                        ((&raw const sContinueScriptBattleParams)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        data,
                    );
                    SetMapVarsToTrainer();
                } else {
                    TrainerBattleLoadArgs(
                        ((&raw const sTrainerBContinueScriptBattleParams)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        data,
                    );
                }
                return (&raw mut EventScript_TryDoNormalTrainerBattle).cast::<u8>();
            }
            if __sw1 == 1i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sContinueScriptBattleParams)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    data,
                );
                SetMapVarsToTrainer();
                return (&raw mut EventScript_TryDoNormalTrainerBattle).cast::<u8>();
            }
            if __sw1 == 6i32 || __sw1 == 8i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sContinueScriptDoubleBattleParams)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    data,
                );
                SetMapVarsToTrainer();
                return (&raw mut EventScript_TryDoDoubleTrainerBattle).cast::<u8>();
            }
            if __sw1 == 7i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sDoubleBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                    data,
                );
                SetMapVarsToTrainer();
                ((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .write(GetRematchTrainerId(
                    ((&raw mut gTrainerBattleOpponent_A)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read(),
                ));
                return (&raw mut EventScript_TryDoDoubleRematchBattle).cast::<u8>();
            }
            if __sw1 == 5i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sOrdinaryBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                    data,
                );
                SetMapVarsToTrainer();
                ((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .write(GetRematchTrainerId(
                    ((&raw mut gTrainerBattleOpponent_A)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read(),
                ));
                return (&raw mut EventScript_TryDoRematchBattle).cast::<u8>();
            }
            if __sw1 == 9i32 {
                if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
                    TrainerBattleLoadArgs(
                        ((&raw const sOrdinaryBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                        data,
                    );
                    SetMapVarsToTrainer();
                    ((&raw mut gTrainerBattleOpponent_A)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(LocalIdToPyramidTrainerId(
                        ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
                    ));
                } else {
                    TrainerBattleLoadArgs(
                        ((&raw const sTrainerBOrdinaryBattleParams)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        data,
                    );
                    ((&raw mut gTrainerBattleOpponent_B)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(LocalIdToPyramidTrainerId(
                        ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
                    ));
                }
                return (&raw mut EventScript_TryDoNormalTrainerBattle).cast::<u8>();
            }
            if __sw1 == 10i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sOrdinaryBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                    data,
                );
                return core::ptr::null_mut();
            }
            if __sw1 == 11i32 {
                TrainerBattleLoadArgs(
                    ((&raw const sTrainerBOrdinaryBattleParams)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    data,
                );
                return core::ptr::null_mut();
            }
            if __sw1 == 12i32 {
                if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
                    TrainerBattleLoadArgs(
                        ((&raw const sOrdinaryBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                        data,
                    );
                    SetMapVarsToTrainer();
                    ((&raw mut gTrainerBattleOpponent_A)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(LocalIdToHillTrainerId(
                        ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
                    ));
                } else {
                    TrainerBattleLoadArgs(
                        ((&raw const sTrainerBOrdinaryBattleParams)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        data,
                    );
                    ((&raw mut gTrainerBattleOpponent_B)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(LocalIdToHillTrainerId(
                        ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
                    ));
                }
                return (&raw mut EventScript_TryDoNormalTrainerBattle).cast::<u8>();
            }
            if !__matched {
                if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
                    TrainerBattleLoadArgs(
                        ((&raw const sOrdinaryBattleParams).cast::<u8>().cast_mut()).cast::<u8>(),
                        data,
                    );
                    SetMapVarsToTrainer();
                } else {
                    TrainerBattleLoadArgs(
                        ((&raw const sTrainerBOrdinaryBattleParams)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        data,
                    );
                }
                return (&raw mut EventScript_TryDoNormalTrainerBattle).cast::<u8>();
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfigureAndSetUpOneTrainerBattle(
    trainerObjEventId: u8,
    trainerScript: *mut u8,
) {
    unsafe {
        let mut trainerObjEventId = trainerObjEventId;
        let mut trainerScript = trainerScript;
        ((&raw mut gSelectedObjectEvent).cast::<u8>()).write(trainerObjEventId);
        ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).write(
            ((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((trainerObjEventId) as i32) as isize * 36))
            .wrapping_add(8))
            .read()) as u16),
        );
        BattleSetup_ConfigureTrainerBattle((trainerScript).wrapping_offset(1));
        ScriptContext_SetupScript((&raw mut EventScript_StartTrainerApproach).cast::<u8>());
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConfigureTwoTrainersBattle(trainerObjEventId: u8, trainerScript: *mut u8) {
    unsafe {
        let mut trainerObjEventId = trainerObjEventId;
        let mut trainerScript = trainerScript;
        ((&raw mut gSelectedObjectEvent).cast::<u8>()).write(trainerObjEventId);
        ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).write(
            ((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((trainerObjEventId) as i32) as isize * 36))
            .wrapping_add(8))
            .read()) as u16),
        );
        BattleSetup_ConfigureTrainerBattle((trainerScript).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpTwoTrainersBattle() {
    unsafe {
        ScriptContext_SetupScript((&raw mut EventScript_StartTrainerApproach).cast::<u8>());
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerFlagFromScriptPointer(data: *mut u8) -> u32 {
    unsafe {
        let mut data = data;
        let mut flag: u32 = ((TrainerBattleLoadArg16((data).wrapping_offset(2))) as u32);
        return ((FlagGet((((1280u32).wrapping_add(flag)) as u16))) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrainerFacingDirection() {
    unsafe {
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
        );
        SetTrainerMovementType(
            objectEvent,
            GetTrainerFacingDirectionMovementType(
                ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
            ),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerBattleMode() -> u8 {
    unsafe {
        return ((((&raw mut sTrainerBattleMode).cast::<u8>().cast::<u16>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerFlag() -> u8 {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            return GetBattlePyramidTrainerFlag(
                ((&raw mut gSelectedObjectEvent).cast::<u8>()).read(),
            );
        } else {
            if (InTrainerHill()) != 0 {
                return GetHillTrainerFlag(((&raw mut gSelectedObjectEvent).cast::<u8>()).read());
            } else {
                return FlagGet(GetTrainerAFlag());
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetBattledTrainersFlags() {
    unsafe {
        if ((((&raw mut gTrainerBattleOpponent_B)
            .cast::<u8>()
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            FlagSet(GetTrainerBFlag());
        }
        FlagSet(GetTrainerAFlag());
    }
}
pub(crate) unsafe extern "C" fn SetBattledTrainerFlag() {
    unsafe {
        FlagSet(GetTrainerAFlag());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasTrainerBeenFought(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        return FlagGet((((1280i32).wrapping_add(((trainerId) as i32))) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrainerFlag(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        FlagSet((((1280i32).wrapping_add(((trainerId) as i32))) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTrainerFlag(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        FlagClear((((1280i32).wrapping_add(((trainerId) as i32))) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartTrainerBattle() {
    unsafe {
        if ((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32) == 2i32 {
            ((&raw mut gBattleTypeFlags).cast::<u32>()).write(32777u32);
        } else {
            ((&raw mut gBattleTypeFlags).cast::<u32>()).write(8u32);
        }
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            VarSet(16398u16, 0u16);
            let __p1 = (&raw mut gBattleTypeFlags).cast::<u32>();
            (__p1).write(((__p1).read() | 2097152u32));
            if ((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32) == 2i32 {
                FillFrontierTrainersParties(1u8);
                ZeroMonData(((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(100));
                ZeroMonData(((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200));
                ZeroMonData(((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400));
                ZeroMonData(((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(500));
            } else {
                FillFrontierTrainerParty(1u8);
                ZeroMonData(((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(100));
                ZeroMonData(((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200));
            }
            MarkApproachingPyramidTrainersAsBattled();
        } else {
            if (InTrainerHillChallenge()) != 0 {
                let __p2 = (&raw mut gBattleTypeFlags).cast::<u32>();
                (__p2).write(((__p2).read() | 67108864u32));
                if ((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32) == 2i32 {
                    FillHillTrainersParties();
                } else {
                    FillHillTrainerParty();
                }
                SetHillTrainerFlag();
            }
        }
        ((&raw mut sNoOfPossibleTrainerRetScripts)
            .cast::<u8>()
            .cast::<u8>())
        .write(((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read());
        ((&raw mut gNoOfApproachingTrainers).cast::<u8>()).write(0u8);
        ((&raw mut sShouldCheckTrainerBScript)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((&raw mut gWhichTrainerToFaceAfterBattle).cast::<u16>()).write(0u16);
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndTrainerBattle));
        if (((CurrentBattlePyramidLocation()) as i32) != 0i32) || ((InTrainerHillChallenge()) != 0)
        {
            DoBattlePyramidTrainerHillBattle();
        } else {
            DoTrainerBattle();
        }
        ScriptContext_Stop();
    }
}
pub(crate) unsafe extern "C" fn CB2_EndTrainerBattle() {
    unsafe {
        if ((((&raw mut gTrainerBattleOpponent_A)
            .cast::<u8>()
            .cast::<u16>())
        .read()) as i32)
            == 1024i32
        {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        } else {
            if IsPlayerDefeated(((((&raw mut gBattleOutcome).cast::<u8>()).read()) as u32)) == 1u32
            {
                if (((CurrentBattlePyramidLocation()) as i32) != 0i32)
                    || ((InTrainerHillChallenge()) != 0)
                {
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                } else {
                    SetMainCallback2(Some(CB2_WhiteOut));
                }
            } else {
                SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                if (((CurrentBattlePyramidLocation()) as i32) == 0i32)
                    && (!((InTrainerHillChallenge()) != 0))
                {
                    RegisterTrainerInMatchCall();
                    SetBattledTrainersFlags();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_EndRematchBattle() {
    unsafe {
        if ((((&raw mut gTrainerBattleOpponent_A)
            .cast::<u8>()
            .cast::<u16>())
        .read()) as i32)
            == 1024i32
        {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        } else {
            if IsPlayerDefeated(((((&raw mut gBattleOutcome).cast::<u8>()).read()) as u32)) == 1u32
            {
                SetMainCallback2(Some(CB2_WhiteOut));
            } else {
                SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                RegisterTrainerInMatchCall();
                SetBattledTrainersFlags();
                HandleRematchVarsOnBattleEnd();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_StartRematchBattle() {
    unsafe {
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(8u32);
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_EndRematchBattle));
        DoTrainerBattle();
        ScriptContext_Stop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerIntroSpeech() {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            if (((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32) == 0i32)
                || (((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32) == 1i32)
            {
                CopyPyramidTrainerSpeechBefore(LocalIdToPyramidTrainerId(
                    ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
                ));
            } else {
                CopyPyramidTrainerSpeechBefore(LocalIdToPyramidTrainerId(
                    ((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gApproachingTrainers).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32)
                                as isize
                                * 12,
                        ))
                        .read()) as i32) as isize
                            * 36,
                    ))
                    .wrapping_add(8))
                    .read(),
                ));
            }
            ShowFieldMessageFromBuffer();
        } else {
            if (InTrainerHillChallenge()) != 0 {
                if (((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32) == 0i32)
                    || (((((&raw mut gNoOfApproachingTrainers).cast::<u8>()).read()) as i32)
                        == 1i32)
                {
                    CopyTrainerHillTrainerText(
                        2u8,
                        LocalIdToHillTrainerId(
                            ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
                        ),
                    );
                } else {
                    CopyTrainerHillTrainerText(
                        2u8,
                        LocalIdToHillTrainerId(
                            ((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gApproachingTrainers).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gApproachingTrainerId).cast::<u8>()).read())
                                        as i32) as isize
                                        * 12,
                                ))
                                .read()) as i32) as isize
                                    * 36,
                            ))
                            .wrapping_add(8))
                            .read(),
                        ),
                    );
                }
                ShowFieldMessageFromBuffer();
            } else {
                ShowFieldMessage(GetIntroSpeechOfApproachingTrainer());
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_GetScriptAddrAfterBattle() -> *mut u8 {
    unsafe {
        if ((((&raw mut sTrainerBattleEndScript)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            return ((&raw mut sTrainerBattleEndScript)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read();
        } else {
            return (&raw mut EventScript_TestSignpostMsg).cast::<u8>();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleSetup_GetTrainerPostBattleScript() -> *mut u8 {
    unsafe {
        if (((&raw mut sShouldCheckTrainerBScript)
            .cast::<u8>()
            .cast::<u8>())
        .read())
            != 0
        {
            ((&raw mut sShouldCheckTrainerBScript)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            if ((((&raw mut sTrainerBBattleScriptRetAddr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                ((&raw mut gWhichTrainerToFaceAfterBattle).cast::<u16>()).write(1u16);
                return ((&raw mut sTrainerBBattleScriptRetAddr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read();
            }
        } else {
            if ((((&raw mut sTrainerABattleScriptRetAddr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                ((&raw mut gWhichTrainerToFaceAfterBattle).cast::<u16>()).write(0u16);
                return ((&raw mut sTrainerABattleScriptRetAddr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read();
            }
        }
        return (&raw mut EventScript_TryGetTrainerScript).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerCantBattleSpeech() {
    unsafe {
        ShowFieldMessage(GetTrainerCantBattleSpeech());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTrainerEncounterMusic() {
    unsafe {
        let mut trainerId: u16 = 0u16;
        let mut music: u16 = 0u16;
        if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
            trainerId = ((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read();
        } else {
            trainerId = ((&raw mut gTrainerBattleOpponent_B)
                .cast::<u8>()
                .cast::<u16>())
            .read();
        }
        if (((((&raw mut sTrainerBattleMode).cast::<u8>().cast::<u16>()).read()) as i32) != 1i32)
            && (((((&raw mut sTrainerBattleMode).cast::<u8>().cast::<u16>()).read()) as i32)
                != 8i32)
        {
            'l1: {
                let __sw1 = ((GetTrainerEncounterMusicId(trainerId)) as i32);
                let __matched = __sw1 == 0i32
                    || __sw1 == 1i32
                    || __sw1 == 2i32
                    || __sw1 == 4i32
                    || __sw1 == 5i32
                    || __sw1 == 6i32
                    || __sw1 == 7i32
                    || __sw1 == 8i32
                    || __sw1 == 9i32
                    || __sw1 == 10i32
                    || __sw1 == 11i32
                    || __sw1 == 12i32
                    || __sw1 == 13i32;
                if __sw1 == 0i32 {
                    music = 380u16;
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    music = 407u16;
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    music = 379u16;
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    music = 416u16;
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    music = 417u16;
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    music = 419u16;
                    break 'l1;
                }
                if __sw1 == 7i32 {
                    music = 441u16;
                    break 'l1;
                }
                if __sw1 == 8i32 {
                    music = 385u16;
                    break 'l1;
                }
                if __sw1 == 9i32 {
                    music = 449u16;
                    break 'l1;
                }
                if __sw1 == 10i32 {
                    music = 450u16;
                    break 'l1;
                }
                if __sw1 == 11i32 {
                    music = 451u16;
                    break 'l1;
                }
                if __sw1 == 12i32 {
                    music = 453u16;
                    break 'l1;
                }
                if __sw1 == 13i32 {
                    music = 397u16;
                    break 'l1;
                }
                if !__matched {
                    music = 423u16;
                }
            }
            PlayNewMapMusic(music);
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnEmptyStringIfNull(string: *mut u8) -> *mut u8 {
    unsafe {
        let mut string = string;
        if ((string) as usize) == 0usize {
            return (&raw mut gText_EmptyString2).cast::<u8>();
        } else {
            return string;
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn GetIntroSpeechOfApproachingTrainer() -> *mut u8 {
    unsafe {
        if ((((&raw mut gApproachingTrainerId).cast::<u8>()).read()) as i32) == 0i32 {
            return ReturnEmptyStringIfNull(
                ((&raw mut sTrainerAIntroSpeech)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
        } else {
            return ReturnEmptyStringIfNull(
                ((&raw mut sTrainerBIntroSpeech)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerALoseText() -> *mut u8 {
    unsafe {
        let mut string: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut gTrainerBattleOpponent_A)
            .cast::<u8>()
            .cast::<u16>())
        .read()) as i32)
            == 1024i32
        {
            string = GetSecretBaseTrainerLoseText();
        } else {
            string = ((&raw mut sTrainerADefeatSpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read();
        }
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ReturnEmptyStringIfNull(string),
        );
        return (&raw mut gStringVar4).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerBLoseText() -> *mut u8 {
    unsafe {
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ReturnEmptyStringIfNull(
                ((&raw mut sTrainerBDefeatSpeech)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            ),
        );
        return (&raw mut gStringVar4).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerWonSpeech() -> *mut u8 {
    unsafe {
        return ReturnEmptyStringIfNull(
            ((&raw mut sTrainerVictorySpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetTrainerCantBattleSpeech() -> *mut u8 {
    unsafe {
        return ReturnEmptyStringIfNull(
            ((&raw mut sTrainerCannotBattleSpeech)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn FirstBattleTrainerIdToRematchTableId(
    table: *mut u8,
    trainerId: u16,
) -> i32 {
    unsafe {
        let mut table = table;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((table).wrapping_offset((i) as isize * 16)).cast::<u16>()).read())
                        as i32)
                        == ((trainerId) as i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn TrainerIdToRematchTableId(table: *mut u8, trainerId: u16) -> i32 {
    unsafe {
        let mut table = table;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 5i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((table).wrapping_offset((i) as isize * 16)).cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    == 0i32
                                {
                                    break 'l3;
                                }
                                if ((((((table).wrapping_offset((i) as isize * 16)).cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    == ((trainerId) as i32)
                                {
                                    return i;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn IsRematchForbidden(rematchTableId: i32) -> u32 {
    unsafe {
        let mut rematchTableId = rematchTableId;
        if rematchTableId >= 73i32 {
            return 1u32;
        } else {
            if rematchTableId == 64i32 {
                return ((!((FlagGet(126u16)) != 0)) as u32);
            } else {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn SetRematchIdForTrainer(table: *mut u8, tableId: u32) {
    unsafe {
        let mut table = table;
        let mut tableId = tableId;
        let mut i: i32 = 0i32;
        {
            i = 1i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut trainerId: u16 = ((((table)
                        .wrapping_offset(((tableId) as i32) as isize * 16))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    if ((trainerId) as i32) == 0i32 {
                        break 'l1;
                    }
                    if !((HasTrainerBeenFought(trainerId)) != 0) {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2506))
            .cast::<u8>())
        .wrapping_offset(((tableId) as i32) as isize))
        .write(((i) as u8));
    }
}
pub(crate) unsafe extern "C" fn UpdateRandomTrainerRematches(
    table: *mut u8,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    unsafe {
        let mut table = table;
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut i: i32 = 0i32;
        let mut ret: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i <= 64i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((table).wrapping_offset((i) as isize * 16))
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as i32)
                        == ((mapGroup) as i32))
                        && ((((((table).wrapping_offset((i) as isize * 16))
                            .wrapping_add(12)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((mapNum) as i32)))
                        && (!((IsRematchForbidden(i)) != 0))
                    {
                        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2506))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            ret = 1u32;
                        } else {
                            if ((FlagGet((((348i32).wrapping_add(i)) as u16))) != 0)
                                && (crate::c::rem_i32(((Random()) as i32), 100i32) <= 30i32)
                            {
                                SetRematchIdForTrainer(table, ((i) as u32));
                                ret = 1u32;
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRematchIfDefeated(rematchTableId: i32) {
    unsafe {
        let mut rematchTableId = rematchTableId;
        if ((HasTrainerBeenFought(
            (((((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((rematchTableId) as isize * 16))
            .cast::<u16>())
            .read(),
        )) as i32)
            == 1i32
        {
            SetRematchIdForTrainer(
                ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
                ((rematchTableId) as u32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DoesSomeoneWantRematchIn_(
    table: *mut u8,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    unsafe {
        let mut table = table;
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((table).wrapping_offset((i) as isize * 16))
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as i32)
                        == ((mapGroup) as i32))
                        && ((((((table).wrapping_offset((i) as isize * 16))
                            .wrapping_add(12)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((mapNum) as i32)))
                        && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2506))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn IsRematchTrainerIn_(
    table: *mut u8,
    mapGroup: u16,
    mapNum: u16,
) -> u32 {
    unsafe {
        let mut table = table;
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((table).wrapping_offset((i) as isize * 16))
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as i32)
                        == ((mapGroup) as i32))
                        && ((((((table).wrapping_offset((i) as isize * 16))
                            .wrapping_add(12)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((mapNum) as i32))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn IsFirstTrainerIdReadyForRematch(
    table: *mut u8,
    firstBattleTrainerId: u16,
) -> u8 {
    unsafe {
        let mut table = table;
        let mut firstBattleTrainerId = firstBattleTrainerId;
        let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
        if tableId == (-1i32) {
            return 0u8;
        }
        if tableId >= 100i32 {
            return 0u8;
        }
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2506))
            .cast::<u8>())
        .wrapping_offset((tableId) as isize))
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsTrainerReadyForRematch_(table: *mut u8, trainerId: u16) -> u8 {
    unsafe {
        let mut table = table;
        let mut trainerId = trainerId;
        let mut tableId: i32 = TrainerIdToRematchTableId(table, trainerId);
        if tableId == (-1i32) {
            return 0u8;
        }
        if tableId >= 100i32 {
            return 0u8;
        }
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2506))
            .cast::<u8>())
        .wrapping_offset((tableId) as isize))
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetRematchTrainerIdFromTable(
    table: *mut u8,
    firstBattleTrainerId: u16,
) -> u16 {
    unsafe {
        let mut table = table;
        let mut firstBattleTrainerId = firstBattleTrainerId;
        let mut trainerEntry: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
        if tableId == (-1i32) {
            return 0u16;
        }
        trainerEntry = (table).wrapping_offset((tableId) as isize * 16);
        {
            i = 1i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((trainerEntry).cast::<u16>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        == 0i32
                    {
                        return (((trainerEntry).cast::<u16>())
                            .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                        .read();
                    }
                    if !((HasTrainerBeenFought(
                        (((trainerEntry).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                    )) != 0)
                    {
                        return (((trainerEntry).cast::<u16>()).wrapping_offset((i) as isize))
                            .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (((trainerEntry).cast::<u16>()).wrapping_offset(4)).read();
    }
}
pub(crate) unsafe extern "C" fn GetLastBeatenRematchTrainerIdFromTable(
    table: *mut u8,
    firstBattleTrainerId: u16,
) -> u16 {
    unsafe {
        let mut table = table;
        let mut firstBattleTrainerId = firstBattleTrainerId;
        let mut trainerEntry: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
        if tableId == (-1i32) {
            return 0u16;
        }
        trainerEntry = (table).wrapping_offset((tableId) as isize * 16);
        {
            i = 1i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((trainerEntry).cast::<u16>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        == 0i32
                    {
                        return (((trainerEntry).cast::<u16>())
                            .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                        .read();
                    }
                    if !((HasTrainerBeenFought(
                        (((trainerEntry).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                    )) != 0)
                    {
                        return (((trainerEntry).cast::<u16>())
                            .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (((trainerEntry).cast::<u16>()).wrapping_offset(4)).read();
    }
}
pub(crate) unsafe extern "C" fn ClearTrainerWantRematchState(
    table: *mut u8,
    firstBattleTrainerId: u16,
) {
    unsafe {
        let mut table = table;
        let mut firstBattleTrainerId = firstBattleTrainerId;
        let mut tableId: i32 = TrainerIdToRematchTableId(table, firstBattleTrainerId);
        if tableId != (-1i32) {
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2506))
                .cast::<u8>())
            .wrapping_offset((tableId) as isize))
            .write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetTrainerMatchCallFlag(trainerId: u32) -> u32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .cast::<u16>())
                    .read()) as u32)
                        == trainerId
                    {
                        return (((348i32).wrapping_add(i)) as u32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u32;
    }
}
pub(crate) unsafe extern "C" fn RegisterTrainerInMatchCall() {
    unsafe {
        if (FlagGet(303u16)) != 0 {
            let mut matchCallFlagId: u32 = GetTrainerMatchCallFlag(
                ((((&raw mut gTrainerBattleOpponent_A)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as u32),
            );
            if matchCallFlagId != 65535u32 {
                FlagSet(((matchCallFlagId) as u16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WasSecondRematchWon(
    table: *mut u8,
    firstBattleTrainerId: u16,
) -> u8 {
    unsafe {
        let mut table = table;
        let mut firstBattleTrainerId = firstBattleTrainerId;
        let mut tableId: i32 = FirstBattleTrainerIdToRematchTableId(table, firstBattleTrainerId);
        if tableId == (-1i32) {
            return 0u8;
        }
        if !((HasTrainerBeenFought(
            ((((table).wrapping_offset((tableId) as isize * 16)).cast::<u16>()).wrapping_offset(1))
                .read(),
        )) != 0)
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn HasAtLeastFiveBadges() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        {
            count = 0i32;
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((FlagGet(
                        ((((&raw const sBadgeFlags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    )) as i32)
                        == 1i32
                    {
                        if {
                            let __t1 = (count).wrapping_add(1);
                            count = __t1;
                            __t1
                        } >= 5i32
                        {
                            return 1u32;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementRematchStepCounter() {
    unsafe {
        if (HasAtLeastFiveBadges()) != 0 {
            if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2504)
                .cast::<u16>())
            .read()) as i32)
                >= 255i32
            {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2504)
                    .cast::<u16>())
                .write(255u16);
            } else {
                let __p1 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2504)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsRematchStepCounterMaxed() -> u32 {
    unsafe {
        if ((HasAtLeastFiveBadges()) != 0)
            && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2504)
                .cast::<u16>())
            .read()) as i32)
                >= 255i32)
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryUpdateRandomTrainerRematches(mapGroup: u16, mapNum: u16) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        if ((IsRematchStepCounterMaxed()) != 0)
            && (UpdateRandomTrainerRematches(
                ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
                mapGroup,
                mapNum,
            ) == 1u32)
        {
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2504)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesSomeoneWantRematchIn(mapGroup: u16, mapNum: u16) -> u32 {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        return DoesSomeoneWantRematchIn_(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            mapGroup,
            mapNum,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRematchTrainerIn(mapGroup: u16, mapNum: u16) -> u32 {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        return IsRematchTrainerIn_(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            mapGroup,
            mapNum,
        );
    }
}
pub(crate) unsafe extern "C" fn GetRematchTrainerId(trainerId: u16) -> u16 {
    unsafe {
        let mut trainerId = trainerId;
        return GetRematchTrainerIdFromTable(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            trainerId,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLastBeatenRematchTrainerId(trainerId: u16) -> u16 {
    unsafe {
        let mut trainerId = trainerId;
        return GetLastBeatenRematchTrainerIdFromTable(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            trainerId,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldTryRematchBattle() -> u8 {
    unsafe {
        if (IsFirstTrainerIdReadyForRematch(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        )) != 0
        {
            return 1u8;
        }
        return WasSecondRematchWon(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTrainerReadyForRematch() -> u8 {
    unsafe {
        return IsTrainerReadyForRematch_(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn HandleRematchVarsOnBattleEnd() {
    unsafe {
        ClearTrainerWantRematchState(
            ((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A)
                .cast::<u8>()
                .cast::<u16>())
            .read(),
        );
        SetBattledTrainersFlags();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldTryGetTrainerScript() {
    unsafe {
        if ((((&raw mut sNoOfPossibleTrainerRetScripts)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            > 1i32
        {
            ((&raw mut sNoOfPossibleTrainerRetScripts)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            ((&raw mut sShouldCheckTrainerBScript)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut sShouldCheckTrainerBScript)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountBattledRematchTeams(trainerId: u16) -> u16 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        if ((HasTrainerBeenFought(
            (((((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((trainerId) as i32) as isize * 16))
            .cast::<u16>())
            .read(),
        )) as i32)
            != 1i32
        {
            return 0u16;
        }
        {
            i = 1i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((trainerId) as i32) as isize * 16))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                    if !((HasTrainerBeenFought(
                        ((((((&raw const gRematchTable).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((trainerId) as i32) as isize * 16))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    )) != 0)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((i) as u16);
    }
}
