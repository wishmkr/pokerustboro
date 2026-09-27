//! Translated from `src/battle_ai_script_commands.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBattleAICmdTable sIgnoredPowerfulMoveEffects
#[allow(unused_imports)]
use crate::data::battle_ai_script_commands::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAIScriptPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattler_AI: u8 = 0u8;

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActiveBattler: u8;
    static mut gBattleAI_ScriptsTable: u8;
    static mut gBattleMons: u8;
    static mut gBattleMoveDamage: u8;
    static mut gBattleMoves: u8;
    static mut gBattleResources: u8;
    static mut gBattleResults: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattleWeather: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerTarget: u8;
    static mut gBitTable: u8;
    static mut gCritMultiplier: u8;
    static mut gCurrentMove: u8;
    static mut gDisableStructs: u8;
    static mut gDynamicBasePower: u8;
    static mut gEnemyParty: u8;
    static mut gLastMoves: u8;
    static mut gMoveResultFlags: u8;
    static mut gPlayerParty: u8;
    static mut gSideStatuses: u8;
    static mut gSpeciesInfo: u8;
    static mut gStatuses3: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    static mut gTrainers: u8;
    fn AI_CalcDmg(a0: u8, a1: u8);
    fn CheckMoveLimitations(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetAiScriptsInBattleFactory() -> u32;
    fn GetAiScriptsInRecordedBattle() -> u32;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetItemHoldEffect(a0: u16) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetWhoStrikesFirst(a0: u8, a1: u8, a2: u8) -> u8;
    fn Random() -> u16;
    fn TypeCalc(a0: u16, a1: u8, a2: u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAI_HandleItemUseBeforeAISetup(defaultScoreMoves: u8) {
    unsafe {
        let mut defaultScoreMoves = defaultScoreMoves;
        let mut i: i32 = 0i32;
        let mut data: *mut u8 = ((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < 84u32) {
                    break 'l1;
                }
                'l2: {
                    ((data).wrapping_offset((i) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0)
            && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 176097666u32) != 0))
        {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                                as isize
                                * 40,
                        ))
                        .wrapping_add(16))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(24)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(72))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .wrapping_add(24)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(80))
                                .read()) as i32) as isize,
                            ))
                            .write(
                                ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32) as isize
                                        * 40,
                                ))
                                .wrapping_add(16))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                            let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(24)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(80);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        BattleAI_SetupAIData(defaultScoreMoves);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAI_SetupAIData(defaultScoreMoves: u8) {
    unsafe {
        let mut defaultScoreMoves = defaultScoreMoves;
        let mut i: i32 = 0i32;
        let mut data: *mut u8 = ((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read();
        let mut moveLimitations: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < 28u32) {
                    break 'l1;
                }
                'l2: {
                    ((data).wrapping_offset((i) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if (((defaultScoreMoves) as i32) & 1i32) != 0 {
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .cast::<i8>())
                        .wrapping_offset((i) as isize))
                        .write(100i8);
                    } else {
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .cast::<i8>())
                        .wrapping_offset((i) as isize))
                        .write(0i8);
                    }
                    defaultScoreMoves = ((((defaultScoreMoves) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        moveLimitations =
            CheckMoveLimitations(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8, 255u8);
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    if (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset((i) as isize))
                    .read()
                        & ((moveLimitations) as u32))
                        != 0
                    {
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .cast::<i8>())
                        .wrapping_offset((i) as isize))
                        .write(0i8);
                    }
                    ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(24))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((100i32).wrapping_sub(crate::c::rem_i32(((Random()) as i32), 16i32)))
                            as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .write(0u8);
        ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>())
            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            ((&raw mut gBattlerTarget).cast::<u8>()).write(
                (((((Random()) as i32) & 2i32).wrapping_add(
                    (((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                        ^ 1i32),
                )) as u8),
            );
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read())
                != 0
            {
                let __p1 = (&raw mut gBattlerTarget).cast::<u8>();
                (__p1).write((((((__p1).read()) as i32) ^ 2i32) as u8));
            }
        } else {
            ((&raw mut gBattlerTarget).cast::<u8>()).write(
                ((((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) ^ 1i32)
                    as u8),
            );
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<u32>())
            .write(GetAiScriptsInRecordedBattle());
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12)
                .cast::<u32>())
                .write(1073741824u32);
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1024u32) != 0 {
                    ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<u32>())
                    .write(536870912u32);
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16u32) != 0 {
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12)
                        .cast::<u32>())
                        .write(2147483648u32);
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 524288u32) != 0 {
                            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(12)
                            .cast::<u32>())
                            .write(GetAiScriptsInBattleFactory());
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 205457664u32)
                                != 0
                            {
                                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(12)
                                .cast::<u32>())
                                .write(7u32);
                            } else {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32)
                                    != 0
                                {
                                    ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                        .wrapping_add(20)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12)
                                    .cast::<u32>())
                                    .write(
                                        (((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                                            ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 40,
                                        ))
                                        .wrapping_add(28)
                                        .cast::<u32>())
                                        .read()
                                            | ((((&raw mut gTrainers).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gTrainerBattleOpponent_B)
                                                        .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 40,
                                                ))
                                            .wrapping_add(28)
                                            .cast::<u32>())
                                            .read()),
                                    );
                                } else {
                                    ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                        .wrapping_add(20)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12)
                                    .cast::<u32>())
                                    .write(
                                        ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                                            ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 40,
                                        ))
                                        .wrapping_add(28)
                                        .cast::<u32>())
                                        .read(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            let __p2 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<u32>();
            (__p2).write(((__p2).read() | 128u32));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAI_ChooseMoveOrAction() -> u8 {
    unsafe {
        let mut savedCurrentMove: u16 = ((&raw mut gCurrentMove).cast::<u16>()).read();
        let mut ret: u8 = 0u8;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
            ret = ChooseMoveOrAction_Singles();
        } else {
            ret = ChooseMoveOrAction_Doubles();
        }
        ((&raw mut gCurrentMove).cast::<u16>()).write(savedCurrentMove);
        return ret;
    }
}
pub(crate) unsafe extern "C" fn ChooseMoveOrAction_Singles() -> u8 {
    unsafe {
        let mut currentMoveArray = crate::ffi::Align4([0u8; 4]);
        let mut consideredMoveArray = crate::ffi::Align4([0u8; 4]);
        let mut numOfBestMoves: u8 = 0u8;
        let mut i: i32 = 0i32;
        RecordLastUsedMoveByTarget();
        'l1: loop {
            if !(((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<u32>())
            .read()
                != 0u32)
            {
                break 'l1;
            }
            if (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<u32>())
            .read()
                & 1u32)
                != 0
            {
                (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .write(0u8);
                BattleAI_DoAIProcessing();
            }
            let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<u32>();
            (__p1).write(((__p1).read() >> 1));
            let __p2 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .write(0u8);
        }
        if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16))
        .read()) as i32)
            & 2i32)
            != 0
        {
            return 4u8;
        }
        if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16))
        .read()) as i32)
            & 4i32)
            != 0
        {
            return 5u8;
        }
        numOfBestMoves = 1u8;
        ((&raw mut currentMoveArray).cast::<u8>()).write(
            (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i8>())
            .read()) as u8),
        );
        ((&raw mut consideredMoveArray).cast::<u8>()).write(0u8);
        {
            i = 1i32;
            'l2: loop {
                if !(i < 4i32) {
                    break 'l2;
                }
                'l3: {
                    if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        if ((((&raw mut currentMoveArray).cast::<u8>()).read()) as i32)
                            == ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4))
                            .cast::<i8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            (((&raw mut currentMoveArray).cast::<u8>())
                                .wrapping_offset(((numOfBestMoves) as i32) as isize))
                            .write(
                                ((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .cast::<i8>())
                                .wrapping_offset((i) as isize))
                                .read()) as u8),
                            );
                            (((&raw mut consideredMoveArray).cast::<u8>()).wrapping_offset(
                                (({
                                    let __t3 = numOfBestMoves;
                                    numOfBestMoves = (numOfBestMoves).wrapping_add(1);
                                    __t3
                                }) as i32) as isize,
                            ))
                            .write(((i) as u8));
                        }
                        if ((((&raw mut currentMoveArray).cast::<u8>()).read()) as i32)
                            < ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4))
                            .cast::<i8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            numOfBestMoves = 1u8;
                            ((&raw mut currentMoveArray).cast::<u8>()).write(
                                ((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .cast::<i8>())
                                .wrapping_offset((i) as isize))
                                .read()) as u8),
                            );
                            ((&raw mut consideredMoveArray).cast::<u8>()).write(((i) as u8));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (((&raw mut consideredMoveArray).cast::<u8>()).wrapping_offset(
            (crate::c::rem_i32(((Random()) as i32), ((numOfBestMoves) as i32))) as isize,
        ))
        .read();
    }
}
pub(crate) unsafe extern "C" fn ChooseMoveOrAction_Doubles() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut scriptsToRun: i32 = 0i32;
        let mut bestMovePointsForTarget = crate::ffi::Align4([0u8; 8]);
        let mut mostViableTargetsArray = crate::ffi::Align4([0u8; 4]);
        let mut actionOrMoveIndex = crate::ffi::Align4([0u8; 4]);
        let mut mostViableMovesScores = crate::ffi::Align4([0u8; 4]);
        let mut mostViableMovesIndices = crate::ffi::Align4([0u8; 4]);
        let mut mostViableTargetsNo: i32 = 0i32;
        let mut mostViableMovesNo: i32 = 0i32;
        let mut mostMovePoints: i16 = 0i16;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (i == ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32))
                        || (((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(40)
                        .cast::<u16>())
                        .read()) as i32)
                            == 0i32)
                    {
                        (((&raw mut actionOrMoveIndex).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(255u8);
                        (((&raw mut bestMovePointsForTarget).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .write((-1i16));
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                            BattleAI_SetupAIData(
                                ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(146))
                                .read()) as i32)
                                    >> 4) as u8),
                            );
                        } else {
                            BattleAI_SetupAIData(15u8);
                        }
                        ((&raw mut gBattlerTarget).cast::<u8>()).write(((i) as u8));
                        if (i & 1i32)
                            != (((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                & 1i32)
                        {
                            RecordLastUsedMoveByTarget();
                        }
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .write(0u8);
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1))
                        .write(0u8);
                        scriptsToRun = ((((((((&raw mut gBattleResources).cast::<*mut u8>())
                            .read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12)
                        .cast::<u32>())
                        .read()) as i32);
                        'l3: loop {
                            if !(scriptsToRun != 0i32) {
                                break 'l3;
                            }
                            if (scriptsToRun & 1i32) != 0 {
                                (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                .read())
                                .write(0u8);
                                BattleAI_DoAIProcessing();
                            }
                            scriptsToRun = (scriptsToRun >> 1);
                            let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .write(0u8);
                        }
                        if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(16))
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            (((&raw mut actionOrMoveIndex).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(4u8);
                        } else {
                            if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(16))
                            .read()) as i32)
                                & 4i32)
                                != 0
                            {
                                (((&raw mut actionOrMoveIndex).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(5u8);
                            } else {
                                ((&raw mut mostViableMovesScores).cast::<u8>()).write(
                                    (((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4))
                                    .cast::<i8>())
                                    .read()) as u8),
                                );
                                ((&raw mut mostViableMovesIndices).cast::<u8>()).write(0u8);
                                mostViableMovesNo = 1i32;
                                {
                                    j = 1i32;
                                    'l4: loop {
                                        if !(j < 4i32) {
                                            break 'l4;
                                        }
                                        'l5: {
                                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut sBattler_AI)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 88,
                                                ))
                                            .wrapping_add(12))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 0i32
                                            {
                                                if ((((&raw mut mostViableMovesScores)
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    == ((((((((((&raw mut gBattleResources)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(20)
                                                    .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(4))
                                                    .cast::<i8>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as i32)
                                                {
                                                    (((&raw mut mostViableMovesScores)
                                                        .cast::<u8>())
                                                    .wrapping_offset((mostViableMovesNo) as isize))
                                                    .write(
                                                        ((((((((((&raw mut gBattleResources)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(20)
                                                        .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(4))
                                                        .cast::<i8>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as u8),
                                                    );
                                                    (((&raw mut mostViableMovesIndices)
                                                        .cast::<u8>())
                                                    .wrapping_offset((mostViableMovesNo) as isize))
                                                    .write(((j) as u8));
                                                    mostViableMovesNo =
                                                        (mostViableMovesNo).wrapping_add(1);
                                                }
                                                if ((((&raw mut mostViableMovesScores)
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    < ((((((((((&raw mut gBattleResources)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(20)
                                                    .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(4))
                                                    .cast::<i8>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as i32)
                                                {
                                                    ((&raw mut mostViableMovesScores).cast::<u8>()).write(((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read()).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>()).wrapping_offset((j) as isize)).read()) as u8));
                                                    ((&raw mut mostViableMovesIndices)
                                                        .cast::<u8>())
                                                    .write(((j) as u8));
                                                    mostViableMovesNo = 1i32;
                                                }
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                (((&raw mut actionOrMoveIndex).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    (((&raw mut mostViableMovesIndices).cast::<u8>())
                                        .wrapping_offset(
                                            (crate::c::rem_i32(
                                                ((Random()) as i32),
                                                mostViableMovesNo,
                                            )) as isize,
                                        ))
                                    .read(),
                                );
                                (((&raw mut bestMovePointsForTarget).cast::<i16>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    ((((&raw mut mostViableMovesScores).cast::<u8>()).read())
                                        as i16),
                                );
                                if (i
                                    == (((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32))
                                    && ((((((&raw mut bestMovePointsForTarget).cast::<i16>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        < 100i32)
                                {
                                    (((&raw mut bestMovePointsForTarget).cast::<i16>())
                                        .wrapping_offset((i) as isize))
                                    .write((-1i16));
                                    ((&raw mut mostViableMovesScores).cast::<u8>()).write(
                                        ((&raw mut mostViableMovesScores).cast::<u8>()).read(),
                                    );
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        mostMovePoints = ((&raw mut bestMovePointsForTarget).cast::<i16>()).read();
        ((&raw mut mostViableTargetsArray).cast::<i8>()).write(0i8);
        mostViableTargetsNo = 1i32;
        {
            i = 1i32;
            'l6: loop {
                if !(i < 4i32) {
                    break 'l6;
                }
                'l7: {
                    if ((mostMovePoints) as i32)
                        == (((((&raw mut bestMovePointsForTarget).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        (((&raw mut mostViableTargetsArray).cast::<i8>())
                            .wrapping_offset((mostViableTargetsNo) as isize))
                        .write(((i) as i8));
                        mostViableTargetsNo = (mostViableTargetsNo).wrapping_add(1);
                    }
                    if ((mostMovePoints) as i32)
                        < (((((&raw mut bestMovePointsForTarget).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        mostMovePoints = (((&raw mut bestMovePointsForTarget).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read();
                        ((&raw mut mostViableTargetsArray).cast::<i8>()).write(((i) as i8));
                        mostViableTargetsNo = 1i32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gBattlerTarget).cast::<u8>()).write(
            (((((&raw mut mostViableTargetsArray).cast::<i8>()).wrapping_offset(
                (crate::c::rem_i32(((Random()) as i32), mostViableTargetsNo)) as isize,
            ))
            .read()) as u8),
        );
        return (((&raw mut actionOrMoveIndex).cast::<u8>())
            .wrapping_offset(((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn BattleAI_DoAIProcessing() {
    unsafe {
        'l1: loop {
            if !((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                != 2i32)
            {
                break 'l1;
            }
            'l2: {
                let __sw1 = (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .read()) as i32);
                if __sw1 == 3i32 {
                    break 'l2;
                }
                if __sw1 == 0i32 {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((&raw mut gBattleAI_ScriptsTable).cast::<*mut u8>()).cast::<*mut u8>())
                            .wrapping_offset(
                                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(17))
                                .read()) as i32) as isize,
                            ))
                        .read(),
                    );
                    if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(36))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        == 0i32
                    {
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(0u16);
                    } else {
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(
                            ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1))
                                .read()) as i32) as isize,
                            ))
                            .read(),
                        );
                    }
                    let __p2 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read());
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    break 'l2;
                }
                if __sw1 == 1i32 {
                    if ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        (((((&raw const sBattleAICmdTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(
                            (((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .read()) as i32) as isize,
                        ))
                        .read())
                        .unwrap_unchecked()();
                    } else {
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .cast::<i8>())
                        .wrapping_offset(
                            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .read()) as i32) as isize,
                        ))
                        .write(0i8);
                        let __p3 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(16);
                        (__p3).write((((((__p3).read()) as i32) | 1i32) as u8));
                    }
                    if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(16))
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        let __p4 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1))
                        .read()) as i32)
                            < 4i32)
                            && (!((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(16))
                            .read()) as i32)
                                & 8i32)
                                != 0))
                        {
                            (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .write(0u8);
                        } else {
                            let __p5 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read());
                            (__p5).write(((__p5).read()).wrapping_add(1));
                        }
                        let __p6 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(16);
                        (__p6).write((((((__p6).read()) as i32) & (-2i32)) as u8));
                    }
                    break 'l2;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecordLastUsedMoveByTarget() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((((((&raw mut gLastMoves).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32)
                    {
                        break 'l1;
                    }
                    if (((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize
                                * 16,
                        ))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(
                            ((((&raw mut gLastMoves).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read(),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattlerMoveHistory(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordAbilityBattle(battler: u8, abilityId: u8) {
    unsafe {
        let mut battler = battler;
        let mut abilityId = abilityId;
        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(64))
        .cast::<u8>())
        .wrapping_offset(((battler) as i32) as isize))
        .write(abilityId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattlerAbilityHistory(battler: u8) {
    unsafe {
        let mut battler = battler;
        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(64))
        .cast::<u8>())
        .wrapping_offset(((battler) as i32) as isize))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordItemEffectBattle(battler: u8, itemEffect: u8) {
    unsafe {
        let mut battler = battler;
        let mut itemEffect = itemEffect;
        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(68))
        .cast::<u8>())
        .wrapping_offset(((battler) as i32) as isize))
        .write(itemEffect);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattlerItemEffectHistory(battler: u8) {
    unsafe {
        let mut battler = battler;
        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(68))
        .cast::<u8>())
        .wrapping_offset(((battler) as i32) as isize))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_less_than() {
    unsafe {
        let mut random: u16 = Random();
        if crate::c::rem_i32(((random) as i32), 256i32)
            < ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_greater_than() {
    unsafe {
        let mut random: u16 = Random();
        if crate::c::rem_i32(((random) as i32), 256i32)
            > ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_equal() {
    unsafe {
        let mut random: u16 = Random();
        if crate::c::rem_i32(((random) as i32), 256i32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_not_equal() {
    unsafe {
        let mut random: u16 = Random();
        if crate::c::rem_i32(((random) as i32), 256i32)
            != ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_score() {
    unsafe {
        let __p1 = (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i8>())
        .wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize,
        );
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32),
            )) as i8),
        );
        if ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<i8>())
        .wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            < 0i32
        {
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<i8>())
            .wrapping_offset(
                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize,
            ))
            .write(0i8);
        }
        let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_less_than() {
    unsafe {
        let mut battler: u16 = 0u16;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        if ((crate::c::div_i32(
            (100i32).wrapping_mul(
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read()) as i32),
            ),
            ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(44)
            .cast::<u16>())
            .read()) as i32),
        )) as u32)
            < ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(3))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_more_than() {
    unsafe {
        let mut battler: u16 = 0u16;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        if ((crate::c::div_i32(
            (100i32).wrapping_mul(
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read()) as i32),
            ),
            ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(44)
            .cast::<u16>())
            .read()) as i32),
        )) as u32)
            > ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(3))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_equal() {
    unsafe {
        let mut battler: u16 = 0u16;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        if ((crate::c::div_i32(
            (100i32).wrapping_mul(
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read()) as i32),
            ),
            ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(44)
            .cast::<u16>())
            .read()) as i32),
        )) as u32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(3))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_hp_not_equal() {
    unsafe {
        let mut battler: u16 = 0u16;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        if ((crate::c::div_i32(
            (100i32).wrapping_mul(
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read()) as i32),
            ),
            ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(44)
            .cast::<u16>())
            .read()) as i32),
        )) as u32)
            != ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(3))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if (((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(76)
        .cast::<u32>())
        .read()
            & status)
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_status() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if !((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(76)
        .cast::<u32>())
        .read()
            & status)
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status2() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if (((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>())
        .read()
            & status)
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_status2() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if !((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>())
        .read()
            & status)
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status3() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((battler) as i32) as isize))
        .read()
            & status)
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_status3() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if !((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((battler) as i32) as isize))
        .read()
            & status)
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_side_affecting() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut side: u32 = 0u32;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        side = ((((GetBattlerPosition(((battler) as u8))) as i32) & 1i32) as u32);
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if (((((((&raw mut gSideStatuses).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((side) as i32) as isize))
        .read()) as u32)
            & status)
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_side_affecting() {
    unsafe {
        let mut battler: u16 = 0u16;
        let mut side: u32 = 0u32;
        let mut status: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u16);
        }
        side = ((((GetBattlerPosition(((battler) as u8))) as i32) & 1i32) as u32);
        status = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        if !((((((((&raw mut gSideStatuses).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((side) as i32) as isize))
        .read()) as u32)
            & status)
            != 0)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(6))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(10));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_less_than() {
    unsafe {
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            < ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_more_than() {
    unsafe {
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            > ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_equal() {
    unsafe {
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_equal() {
    unsafe {
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            != ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_less_than_ptr() {
    unsafe {
        let mut value: *mut u8 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8);
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            < (((value).read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(9));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_more_than_ptr() {
    unsafe {
        let mut value: *mut u8 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8);
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            > (((value).read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(9));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_equal_ptr() {
    unsafe {
        let mut value: *mut u8 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8);
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            == (((value).read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(9));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_equal_ptr() {
    unsafe {
        let mut value: *mut u8 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8);
        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .read()
            != (((value).read()) as u32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(9));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_move() {
    unsafe {
        let mut r#move: u16 = ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        if ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            == ((r#move) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(3))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_move() {
    unsafe {
        let mut r#move: u16 = ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        if ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            != ((r#move) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(3))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_in_bytes() {
    unsafe {
        let mut ptr: *mut u8 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8);
        'l1: loop {
            if !((((ptr).read()) as i32) != 255i32) {
                break 'l1;
            }
            if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .read()
                == (((ptr).read()) as u32)
            {
                ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                    ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .read()) as i32)
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(5))
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8))
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(5))
                        .wrapping_offset(2))
                        .read()) as i32)
                            << 16))
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(5))
                        .wrapping_offset(3))
                        .read()) as i32)
                            << 24)) as usize as *mut u8),
                );
                return;
            }
            ptr = (ptr).wrapping_offset(1);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(9));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_in_bytes() {
    unsafe {
        let mut ptr: *mut u8 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8);
        'l1: loop {
            if !((((ptr).read()) as i32) != 255i32) {
                break 'l1;
            }
            if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .read()
                == (((ptr).read()) as u32)
            {
                let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p1).write(((__p1).read()).wrapping_offset(9));
                return;
            }
            ptr = (ptr).wrapping_offset(1);
        }
        ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(5))
            .read()) as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_in_hwords() {
    unsafe {
        let mut ptr: *mut u16 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8)
            .cast::<u16>();
        'l1: loop {
            if !((((ptr).read()) as i32) != 65535i32) {
                break 'l1;
            }
            if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .read()
                == (((ptr).read()) as u32)
            {
                ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                    ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(5))
                    .read()) as i32)
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(5))
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8))
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(5))
                        .wrapping_offset(2))
                        .read()) as i32)
                            << 16))
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(5))
                        .wrapping_offset(3))
                        .read()) as i32)
                            << 24)) as usize as *mut u8),
                );
                return;
            }
            ptr = (ptr).wrapping_offset(1);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(9));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_in_hwords() {
    unsafe {
        let mut ptr: *mut u16 = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as usize as *mut u8)
            .cast::<u16>();
        'l1: loop {
            if !((((ptr).read()) as i32) != 65535i32) {
                break 'l1;
            }
            if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .read()
                == (((ptr).read()) as u32)
            {
                let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p1).write(((__p1).read()).wrapping_offset(9));
                return;
            }
            ptr = (ptr).wrapping_offset(1);
        }
        ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(5))
            .read()) as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(5))
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_has_attacking_move() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32)
                        && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 12,
                        ))
                        .wrapping_add(1))
                        .read()) as i32)
                            != 0i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i == 4i32 {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        } else {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_has_no_attacking_moves() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32)
                        && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 12,
                        ))
                        .wrapping_add(1))
                        .read()) as i32)
                            != 0i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != 4i32 {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        } else {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_turn_count() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write((((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(19)).read()) as u32));
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_type() {
    unsafe {
        let mut typeVar: u8 = ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(1))
        .read();
        'l1: {
            let __sw1 = ((typeVar) as i32);
            if __sw1 == 1i32 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(33))
                    .cast::<u8>())
                    .read()) as u32),
                );
                break 'l1;
            }
            if __sw1 == 0i32 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(33))
                    .cast::<u8>())
                    .read()) as u32),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(33))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as u32),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(33))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as u32),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 12,
                    ))
                    .wrapping_add(2))
                    .read()) as u32),
                );
                break 'l1;
            }
        }
        let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn BattleAI_GetWantedBattler(wantedBattler: u8) -> u8 {
    unsafe {
        let mut wantedBattler = wantedBattler;
        'l1: {
            let __sw1 = ((wantedBattler) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 0i32 || __sw1 == 3i32 || __sw1 == 2i32;
            if __sw1 == 1i32 {
                return ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
            }
            if __sw1 == 0i32 || !__matched {
                return ((&raw mut gBattlerTarget).cast::<u8>()).read();
            }
            if __sw1 == 3i32 {
                return ((((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                    ^ 2i32) as u8);
            }
            if __sw1 == 2i32 {
                return ((((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_is_of_type() {
    unsafe {
        let mut battler: u8 = BattleAI_GetWantedBattler(
            ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
                .read(),
        );
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(33))
        .cast::<u8>())
        .read()) as i32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as i32))
            || (((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(33))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32))
        {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(1u32);
        } else {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(0u32);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_considered_move_power() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(1))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_how_powerful_move_is() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut checkedMove: i32 = 0i32;
        let mut moveDmgs = crate::ffi::Align4([0u8; 16]);
        {
            i = 0i32;
            'l1: loop {
                if !(((((((&raw const sIgnoredPowerfulMoveEffects)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 12,
                    ))
                    .read()) as i32)
                        == ((((((&raw const sIgnoredPowerfulMoveEffects)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(1))
        .read()) as i32)
            > 1i32)
            && (((((((&raw const sIgnoredPowerfulMoveEffects)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((i) as isize))
            .read()) as i32)
                == 65535i32)
        {
            ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).write(0u8);
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
            ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
            ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
            {
                checkedMove = 0i32;
                'l3: loop {
                    if !(checkedMove < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        {
                            i = 0i32;
                            'l5: loop {
                                if !(((((((&raw const sIgnoredPowerfulMoveEffects)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    != 65535i32)
                                {
                                    break 'l5;
                                }
                                'l6: {
                                    if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut sBattler_AI)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(12))
                                        .cast::<u16>())
                                        .wrapping_offset((checkedMove) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 12,
                                    ))
                                    .read()) as i32)
                                        == ((((((&raw const sIgnoredPowerfulMoveEffects)
                                            .cast::<u8>()
                                            .cast_mut()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l5;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset((checkedMove) as isize))
                        .read()) as i32)
                            != 0i32)
                            && (((((((&raw const sIgnoredPowerfulMoveEffects)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == 65535i32))
                            && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 88,
                                ))
                                .wrapping_add(12))
                                .cast::<u16>())
                                .wrapping_offset((checkedMove) as isize))
                                .read()) as i32) as isize
                                    * 12,
                            ))
                            .wrapping_add(1))
                            .read()) as i32)
                                > 1i32)
                        {
                            ((&raw mut gCurrentMove).cast::<u16>()).write(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 88,
                                ))
                                .wrapping_add(12))
                                .cast::<u16>())
                                .wrapping_offset((checkedMove) as isize))
                                .read(),
                            );
                            AI_CalcDmg(
                                ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
                                ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                            );
                            TypeCalc(
                                ((&raw mut gCurrentMove).cast::<u16>()).read(),
                                ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
                                ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                            );
                            (((&raw mut moveDmgs).cast::<i32>())
                                .wrapping_offset((checkedMove) as isize))
                            .write(crate::c::div_i32(
                                (((&raw mut gBattleMoveDamage).cast::<i32>()).read()).wrapping_mul(
                                    ((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(24))
                                    .cast::<u8>())
                                    .wrapping_offset((checkedMove) as isize))
                                    .read()) as i32),
                                ),
                                100i32,
                            ));
                            if (((&raw mut moveDmgs).cast::<i32>())
                                .wrapping_offset((checkedMove) as isize))
                            .read()
                                == 0i32
                            {
                                (((&raw mut moveDmgs).cast::<i32>())
                                    .wrapping_offset((checkedMove) as isize))
                                .write(1i32);
                            }
                        } else {
                            (((&raw mut moveDmgs).cast::<i32>())
                                .wrapping_offset((checkedMove) as isize))
                            .write(0i32);
                        }
                    }
                    checkedMove = (checkedMove).wrapping_add(1);
                }
            }
            {
                checkedMove = 0i32;
                'l7: loop {
                    if !(checkedMove < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        if (((&raw mut moveDmgs).cast::<i32>())
                            .wrapping_offset((checkedMove) as isize))
                        .read()
                            > (((&raw mut moveDmgs).cast::<i32>()).wrapping_offset(
                                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1))
                                .read()) as i32) as isize,
                            ))
                            .read()
                        {
                            break 'l7;
                        }
                    }
                    checkedMove = (checkedMove).wrapping_add(1);
                }
            }
            if checkedMove == 4i32 {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(2u32);
            } else {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(1u32);
            }
        } else {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(0u32);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_last_used_battler_move() {
    unsafe {
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(
                ((((((&raw mut gLastMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as u32),
            );
        } else {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(
                ((((((&raw mut gLastMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as u32),
            );
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_equal_() {
    unsafe {
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as u32)
            == ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .read()
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_equal_() {
    unsafe {
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as u32)
            != ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .read()
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_goes() {
    unsafe {
        if ((GetWhoStrikesFirst(
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
            1u8,
        )) as i32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_user_doesnt_go() {
    unsafe {
        if ((GetWhoStrikesFirst(
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
            1u8,
        )) as i32)
            != ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop_2A() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_2B() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_count_usable_party_mons() {
    unsafe {
        let mut battler: u8 = 0u8;
        let mut battlerOnField1: u8 = 0u8;
        let mut battlerOnField2: u8 = 0u8;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(0u32);
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            let mut position: u32 = 0u32;
            battlerOnField1 = ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as u8);
            position = ((((GetBattlerPosition(battler)) as i32) ^ 2i32) as u32);
            battlerOnField2 = ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((GetBattlerAtPosition(((position) as u8))) as i32) as isize))
            .read()) as u8);
        } else {
            battlerOnField1 = ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as u8);
            battlerOnField2 = ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as u8);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((i != ((battlerOnField1) as i32)) && (i != ((battlerOnField2) as i32)))
                        && (GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32)
                            != 0u32))
                        && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                            != 0u32))
                        && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                            != 412u32)
                    {
                        let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8)
                        .cast::<u32>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_considered_move() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_considered_move_effect() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 12,
            ))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_ability() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        if ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) != ((battler) as i32) {
            if ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(64))
            .cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32)
                != 0i32
            {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(64))
                    .cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read()) as u32),
                );
                let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p1).write(((__p1).read()).wrapping_offset(2));
                return;
            }
            if ((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(32))
            .read()) as i32)
                == 23i32)
                || (((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(32))
                .read()) as i32)
                    == 42i32))
                || (((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(32))
                .read()) as i32)
                    == 71i32)
            {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(32))
                    .read()) as u32),
                );
                let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p2).write(((__p2).read()).wrapping_offset(2));
                return;
            }
            if (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .cast::<u16>())
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(22))
            .cast::<u8>())
            .read()) as i32)
                != 0i32
            {
                if ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(22))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 0i32
                {
                    if (((Random()) as i32) & 1i32) != 0 {
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8)
                        .cast::<u32>())
                        .write(
                            (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(22))
                            .cast::<u8>())
                            .read()) as u32),
                        );
                    } else {
                        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8)
                        .cast::<u32>())
                        .write(
                            ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(22))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as u32),
                        );
                    }
                } else {
                    ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8)
                    .cast::<u32>())
                    .write(
                        (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .read()) as u32),
                    );
                }
            } else {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 88))
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(22))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as u32),
                );
            }
        } else {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(32))
                .read()) as u32),
            );
        }
        let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_check_ability() {
    unsafe {
        let mut battler: u32 = ((BattleAI_GetWantedBattler(
            ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
                .read(),
        )) as u32);
        let mut ability: u32 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(2))
        .read()) as u32);
        if (((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(1))
        .read()) as i32)
            == 0i32)
            || (((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
                == 2i32)
        {
            if ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(64))
            .cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32)
                != 0i32
            {
                ability = ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(64))
                .cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
                .read()) as u32);
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(ability);
            } else {
                if ((((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(32))
                .read()) as i32)
                    == 23i32)
                    || (((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(32))
                    .read()) as i32)
                        == 42i32))
                    || (((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(32))
                    .read()) as i32)
                        == 71i32)
                {
                    ability = ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(32))
                    .read()) as u32);
                } else {
                    if (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 88))
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(22))
                    .cast::<u8>())
                    .read()) as i32)
                        != 0i32
                    {
                        if ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != 0i32
                        {
                            let mut abilityDummyVariable: u8 = ((ability) as u8);
                            if ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(22))
                            .cast::<u8>())
                            .read()) as i32)
                                != ((abilityDummyVariable) as i32))
                                && (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                .wrapping_add(22))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    != ((abilityDummyVariable) as i32))
                            {
                                ability = (((((((&raw mut gSpeciesInfo).cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                .wrapping_add(22))
                                .cast::<u8>())
                                .read()) as u32);
                            } else {
                                ability = 0u32;
                            }
                        } else {
                            ability = (((((((&raw mut gSpeciesInfo).cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 28,
                                ))
                            .wrapping_add(22))
                            .cast::<u8>())
                            .read()) as u32);
                        }
                    } else {
                        ability = ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as u32);
                    }
                }
            }
        } else {
            ability = ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(32))
            .read()) as u32);
        }
        if ability == 0u32 {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(2u32);
        } else {
            if ability
                == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as u32)
            {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(1u32);
            } else {
                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .write(0u32);
            }
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_highest_type_effectiveness() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut dynamicMoveType: *mut u8 = core::ptr::null_mut();
        ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
        dynamicMoveType = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19);
        (dynamicMoveType).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(40i32);
                    ((&raw mut gCurrentMove).cast::<u16>()).write(
                        ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    if ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) != 0i32 {
                        TypeCalc(
                            ((&raw mut gCurrentMove).cast::<u16>()).read(),
                            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
                            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                        );
                        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 120i32 {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(80i32);
                        }
                        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 240i32 {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(160i32);
                        }
                        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 30i32 {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(20i32);
                        }
                        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 15i32 {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(10i32);
                        }
                        if (((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 8i32)
                            != 0
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(0i32);
                        }
                        if ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8)
                        .cast::<u32>())
                        .read()
                            < ((((&raw mut gBattleMoveDamage).cast::<i32>()).read()) as u32)
                        {
                            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8)
                            .cast::<u32>())
                            .write(((((&raw mut gBattleMoveDamage).cast::<i32>()).read()) as u32));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_type_effectiveness() {
    unsafe {
        let mut damageVar: u8 = 0u8;
        ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(40i32);
        ((&raw mut gCurrentMove).cast::<u16>()).write(
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        TypeCalc(
            ((&raw mut gCurrentMove).cast::<u16>()).read(),
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
        );
        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 120i32 {
            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(80i32);
        }
        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 240i32 {
            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(160i32);
        }
        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 30i32 {
            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(20i32);
        }
        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 15i32 {
            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(10i32);
        }
        if (((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 8i32) != 0 {
            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(0i32);
        }
        damageVar = ((((&raw mut gBattleMoveDamage).cast::<i32>()).read()) as u8);
        if ((damageVar) as i32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop_32() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_33() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_if_status_in_party() {
    unsafe {
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut statusToCompareTo: u32 = 0u32;
        let mut battler: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            let __matched = __sw1 == 1i32;
            if __sw1 == 1i32 {
                battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
                break 'l1;
            }
            if !__matched {
                battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
                break 'l1;
            }
        }
        party = (if ((GetBattlerSide(battler)) as i32) == 0i32 {
            (&raw mut gPlayerParty).cast::<u8>()
        } else {
            (&raw mut gEnemyParty).cast::<u8>()
        });
        statusToCompareTo = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        {
            i = 0i32;
            'l2: loop {
                if !(i < 6i32) {
                    break 'l2;
                }
                'l3: {
                    let mut species: u16 =
                        ((GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32)) as u16);
                    let mut hp: u16 =
                        ((GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32)) as u16);
                    let mut status: u32 =
                        GetMonData2((party).wrapping_offset((i) as isize * 100), 55i32);
                    if (((((species) as i32) != 0i32) && (((species) as i32) != 412i32))
                        && (((hp) as i32) != 0i32))
                        && (status == statusToCompareTo)
                    {
                        ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                            ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(6))
                            .read()) as i32)
                                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_offset(6))
                                .wrapping_offset(1))
                                .read()) as i32)
                                    << 8))
                                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_offset(6))
                                .wrapping_offset(2))
                                .read()) as i32)
                                    << 16))
                                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_offset(6))
                                .wrapping_offset(3))
                                .read()) as i32)
                                    << 24)) as usize as *mut u8),
                        );
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(10));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_status_not_in_party() {
    unsafe {
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut statusToCompareTo: u32 = 0u32;
        let mut battler: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            let __matched = __sw1 == 1i32;
            if __sw1 == 1i32 {
                battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
                break 'l1;
            }
            if !__matched {
                battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
                break 'l1;
            }
        }
        party = (if ((GetBattlerSide(battler)) as i32) == 0i32 {
            (&raw mut gPlayerParty).cast::<u8>()
        } else {
            (&raw mut gEnemyParty).cast::<u8>()
        });
        statusToCompareTo = ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        {
            i = 0i32;
            'l2: loop {
                if !(i < 6i32) {
                    break 'l2;
                }
                'l3: {
                    let mut species: u16 =
                        ((GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32)) as u16);
                    let mut hp: u16 =
                        ((GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32)) as u16);
                    let mut status: u32 =
                        GetMonData2((party).wrapping_offset((i) as isize * 100), 55i32);
                    if (((((species) as i32) != 0i32) && (((species) as i32) != 412i32))
                        && (((hp) as i32) != 0i32))
                        && (status == statusToCompareTo)
                    {
                        let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                        (__p2).write(((__p2).read()).wrapping_offset(10));
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(6))
            .read()) as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(6))
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_weather() {
    unsafe {
        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 7i32) != 0 {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(1u32);
        }
        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 24i32) != 0 {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(2u32);
        }
        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 96i32) != 0 {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(0u32);
        }
        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 128i32) != 0 {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(3u32);
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_effect() {
    unsafe {
        if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32) as isize
                * 12,
        ))
        .read()) as i32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_not_effect() {
    unsafe {
        if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32) as isize
                * 12,
        ))
        .read()) as i32)
            != ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_less_than() {
    unsafe {
        let mut battler: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u32);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u32);
        }
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(24))
        .cast::<i8>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            < ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(3))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(4))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_more_than() {
    unsafe {
        let mut battler: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u32);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u32);
        }
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(24))
        .cast::<i8>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            > ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(3))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(4))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_equal() {
    unsafe {
        let mut battler: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u32);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u32);
        }
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(24))
        .cast::<i8>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            == ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(3))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(4))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_stat_level_not_equal() {
    unsafe {
        let mut battler: u32 = 0u32;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as u32);
        } else {
            battler = ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as u32);
        }
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(24))
        .cast::<i8>())
        .wrapping_offset(
            ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            != ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(3))
            .read()) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(4))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_can_faint() {
    unsafe {
        if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(1))
        .read()) as i32)
            < 2i32
        {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
            return;
        }
        ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
        ((&raw mut gCurrentMove).cast::<u16>()).write(
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        AI_CalcDmg(
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
        );
        TypeCalc(
            ((&raw mut gCurrentMove).cast::<u16>()).read(),
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
        );
        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
            (((&raw mut gBattleMoveDamage).cast::<i32>()).read()).wrapping_mul(
                ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(24))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32) as isize,
                ))
                .read()) as i32),
            ),
            100i32,
        ));
        if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(40)
        .cast::<u16>())
        .read()) as i32)
            <= ((&raw mut gBattleMoveDamage).cast::<i32>()).read()
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p2).write(((__p2).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_cant_faint() {
    unsafe {
        if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(1))
        .read()) as i32)
            < 2i32
        {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
            return;
        }
        ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
        ((&raw mut gCurrentMove).cast::<u16>()).write(
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        AI_CalcDmg(
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
        );
        TypeCalc(
            ((&raw mut gCurrentMove).cast::<u16>()).read(),
            ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read(),
            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
        );
        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
            (((&raw mut gBattleMoveDamage).cast::<i32>()).read()).wrapping_mul(
                ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(24))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32) as isize,
                ))
                .read()) as i32),
            ),
            100i32,
        ));
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(40)
        .cast::<u16>())
        .read()) as i32)
            > ((&raw mut gBattleMoveDamage).cast::<i32>()).read()
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p2).write(((__p2).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_has_move() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut movePtr: *mut u16 =
            ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(2))
                .cast::<u16>();
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == (((movePtr).read()) as i32)
                            {
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == 4i32 {
                    let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p2).write(((__p2).read()).wrapping_offset(8));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(4))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    (((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) ^ 2i32)
                        as isize
                        * 88,
                ))
                .wrapping_add(40)
                .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(8));
                    break 'l1;
                } else {
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 4i32) {
                                break 'l4;
                            }
                            'l5: {
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32) as isize
                                        * 88,
                                ))
                                .wrapping_add(12))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == (((movePtr).read()) as i32)
                                {
                                    break 'l4;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if i == 4i32 {
                    let __p4 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p4).write(((__p4).read()).wrapping_offset(8));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(4))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 || __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            if (((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize
                                    * 16,
                            ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == (((movePtr).read()) as i32)
                            {
                                break 'l6;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == 4i32 {
                    let __p5 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p5).write(((__p5).read()).wrapping_offset(8));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(4))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_doesnt_have_move() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut movePtr: *mut u16 =
            ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(2))
                .cast::<u16>();
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 1i32 || __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == (((movePtr).read()) as i32)
                            {
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i != 4i32 {
                    let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p2).write(((__p2).read()).wrapping_offset(8));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(4))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 || __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            if (((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize
                                    * 16,
                            ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == (((movePtr).read()) as i32)
                            {
                                break 'l4;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i != 4i32 {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(8));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(4))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(4))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_has_move_with_effect() {
    unsafe {
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 1i32 || __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                != 0i32)
                                && ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 12,
                                ))
                                .read()) as i32)
                                    == ((((((&raw mut gAIScriptPtr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(2))
                                    .read()) as i32))
                            {
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == 4i32 {
                    let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p2).write(((__p2).read()).wrapping_offset(7));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 || __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            if (((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                != 0i32)
                                && ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    (((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(24)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 12,
                                ))
                                .read()) as i32)
                                    == ((((((&raw mut gAIScriptPtr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(2))
                                    .read()) as i32))
                            {
                                break 'l4;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == 4i32 {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(7));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_doesnt_have_move_with_effect() {
    unsafe {
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 1i32 || __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                != 0i32)
                                && ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 12,
                                ))
                                .read()) as i32)
                                    == ((((((&raw mut gAIScriptPtr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(2))
                                    .read()) as i32))
                            {
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i != 4i32 {
                    let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p2).write(((__p2).read()).wrapping_offset(7));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 || __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            if (((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize
                                    * 16,
                            ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read())
                                != 0)
                                && ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    (((((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(24)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 12,
                                ))
                                .read()) as i32)
                                    == ((((((&raw mut gAIScriptPtr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(2))
                                    .read()) as i32))
                            {
                                break 'l4;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i != 4i32 {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(7));
                } else {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_any_move_disabled_or_encored() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(2))
            .read()) as i32)
            == 0i32
        {
            if ((((((&raw mut gDisableStructs).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p1).write(((__p1).read()).wrapping_offset(7));
            } else {
                ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                    ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(3))
                    .read()) as i32)
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8))
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .wrapping_offset(2))
                        .read()) as i32)
                            << 16))
                        | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .wrapping_offset(3))
                        .read()) as i32)
                            << 24)) as usize as *mut u8),
                );
            }
        } else {
            if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as i32)
                != 1i32
            {
                let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p2).write(((__p2).read()).wrapping_offset(7));
            } else {
                if ((((((&raw mut gDisableStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 28))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32)
                    != 0i32
                {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(3))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(3))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                } else {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(7));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_curr_move_disabled_or_encored() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as i32)
                    == ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(2))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                } else {
                    let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p2).write(((__p2).read()).wrapping_offset(6));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32)
                    == ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(2))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                } else {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(6));
                }
                break 'l1;
            }
            if !__matched {
                let __p4 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                (__p4).write(((__p4).read()).wrapping_offset(6));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_flee() {
    unsafe {
        let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16);
        (__p1).write((((((__p1).read()) as i32) | 11i32) as u8));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_random_safari_flee() {
    unsafe {
        let mut safariFleeRate: u8 = ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
            .wrapping_add(123))
        .read()) as i32)
            .wrapping_mul(5i32)) as u8);
        if (((crate::c::rem_i32(((Random()) as i32), 100i32)) as u8) as i32)
            < ((safariFleeRate) as i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_watch() {
    unsafe {
        let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16);
        (__p1).write((((((__p1).read()) as i32) | 13i32) as u8));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_hold_effect() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        if ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) != ((battler) as i32) {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(
                ((GetItemHoldEffect(
                    ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(68))
                    .cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read()) as u16),
                )) as u32),
            );
        } else {
            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<u32>())
            .write(
                ((GetItemHoldEffect(
                    ((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(46)
                    .cast::<u16>())
                    .read(),
                )) as u32),
            );
        }
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_holds_item() {
    unsafe {
        let mut battler: u8 = BattleAI_GetWantedBattler(
            ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
                .read(),
        );
        let mut item: u16 = 0u16;
        let mut itemLo: u8 = 0u8;
        let mut itemHi: u8 = 0u8;
        if (((battler) as i32) & 1i32)
            == (((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
        {
            item = ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(46)
            .cast::<u16>())
            .read();
        } else {
            item = ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(68))
            .cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
            .read()) as u16);
        }
        itemHi = ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(2))
        .read();
        itemLo = ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(3))
        .read();
        if (((itemLo) as i32) | ((itemHi) as i32)) == ((item) as i32) {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(4))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(4))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_gender() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((GetGenderFromSpeciesAndPersonality(
                ((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .cast::<u16>())
                .read(),
                ((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(72)
                .cast::<u32>())
                .read(),
            )) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_is_first_turn_for() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((&raw mut gDisableStructs).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(22))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_stockpile_count() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((&raw mut gDisableStructs).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(9))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_is_double_battle() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32));
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_used_held_item() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            (((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(184))
                .cast::<u16>())
            .wrapping_offset(((battler) as i32) as isize))
            .cast::<u8>())
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_move_type_from_result() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(2))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_move_power_from_result() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(1))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_move_effect_from_result() {
    unsafe {
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .read()) as i32) as isize
                    * 12,
            ))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_get_protect_count() {
    unsafe {
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            battler = ((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattlerTarget).cast::<u8>()).read();
        }
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            ((((((&raw mut gDisableStructs).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(8))
            .read()) as u32),
        );
        let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop_52() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_53() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_54() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_55() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_56() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop_57() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_call() {
    unsafe {
        AIStackPushVar(
            (((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(5),
        );
        ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_goto() {
    unsafe {
        ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
            ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32)
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_end() {
    unsafe {
        if ((AIStackPop()) as i32) == 0i32 {
            let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16);
            (__p1).write((((((__p1).read()) as i32) | 1i32) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_level_cond() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                        * 88,
                ))
                .wrapping_add(42))
                .read()) as i32)
                    > ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(42))
                    .read()) as i32)
                {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(2))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                } else {
                    let __p2 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p2).write(((__p2).read()).wrapping_offset(6));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                        * 88,
                ))
                .wrapping_add(42))
                .read()) as i32)
                    < ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(42))
                    .read()) as i32)
                {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(2))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                } else {
                    let __p3 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(6));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                        * 88,
                ))
                .wrapping_add(42))
                .read()) as i32)
                    == ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(42))
                    .read()) as i32)
                {
                    ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                        ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(2))
                        .read()) as i32)
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 16))
                            | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(2))
                            .wrapping_offset(3))
                            .read()) as i32)
                                << 24)) as usize as *mut u8),
                    );
                } else {
                    let __p4 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
                    (__p4).write(((__p4).read()).wrapping_offset(6));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_target_taunted() {
    unsafe {
        if ((crate::c::bf_read(
            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(19),
            0,
            4,
            false,
        ) as u8) as i32)
            != 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_target_not_taunted() {
    unsafe {
        if ((crate::c::bf_read(
            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(19),
            0,
            4,
            false,
        ) as u8) as i32)
            == 0i32
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_target_is_ally() {
    unsafe {
        if (((((&raw mut sBattler_AI).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
            == (((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) & 1i32)
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(1))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(5));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_if_flash_fired() {
    unsafe {
        let mut battler: u8 = BattleAI_GetWantedBattler(
            ((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(1))
                .read(),
        );
        if ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .cast::<u32>())
        .wrapping_offset(((battler) as i32) as isize))
        .read()
            & 1u32)
            != 0
        {
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                ((((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(2))
                .read()) as i32)
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(2))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as usize as *mut u8),
            );
        } else {
            let __p1 = (&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(6));
        }
    }
}
pub(crate) unsafe extern "C" fn AIStackPushVar(var: *mut u8) {
    unsafe {
        let mut var = var;
        (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(var);
    }
}
pub(crate) unsafe extern "C" fn AIStackPushVar_cursor() {
    unsafe {
        (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn AIStackPop() -> u8 {
    unsafe {
        if ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((&raw mut gAIScriptPtr).cast::<u8>().cast::<*mut u8>()).write(
                (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
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
