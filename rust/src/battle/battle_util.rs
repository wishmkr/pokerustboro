//! Translated from `src/battle_util.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPkblToEscapeFactor sGoNearCounterToCatchFactor sGoNearCounterToEscapeFactor sSoundMovesTable
#[allow(unused_imports)]
use crate::data::battle_util::*;

unsafe extern "C" {
    static mut BattleScript_AbilityCuredStatus: u8;
    static mut BattleScript_ActionSwitch: u8;
    static mut BattleScript_ApplySecondaryEffect: u8;
    static mut BattleScript_ArenaDoJudgment: u8;
    static mut BattleScript_BerryConfuseHealEnd2: u8;
    static mut BattleScript_BerryCureBrnEnd2: u8;
    static mut BattleScript_BerryCureBrnRet: u8;
    static mut BattleScript_BerryCureChosenStatusEnd2: u8;
    static mut BattleScript_BerryCureChosenStatusRet: u8;
    static mut BattleScript_BerryCureConfusionEnd2: u8;
    static mut BattleScript_BerryCureConfusionRet: u8;
    static mut BattleScript_BerryCureFrzEnd2: u8;
    static mut BattleScript_BerryCureFrzRet: u8;
    static mut BattleScript_BerryCureParRet: u8;
    static mut BattleScript_BerryCurePrlzEnd2: u8;
    static mut BattleScript_BerryCurePsnEnd2: u8;
    static mut BattleScript_BerryCurePsnRet: u8;
    static mut BattleScript_BerryCureSlpEnd2: u8;
    static mut BattleScript_BerryCureSlpRet: u8;
    static mut BattleScript_BerryFocusEnergyEnd2: u8;
    static mut BattleScript_BerryPPHealEnd2: u8;
    static mut BattleScript_BerryStatRaiseEnd2: u8;
    static mut BattleScript_BideAttack: u8;
    static mut BattleScript_BideNoEnergyToAttack: u8;
    static mut BattleScript_BideStoringEnergy: u8;
    static mut BattleScript_BurnTurnDmg: u8;
    static mut BattleScript_CastformChange: u8;
    static mut BattleScript_ColorChangeActivates: u8;
    static mut BattleScript_CurseTurnDmg: u8;
    static mut BattleScript_CuteCharmActivates: u8;
    static mut BattleScript_DamagingWeatherContinues: u8;
    static mut BattleScript_DisabledNoMore: u8;
    static mut BattleScript_DrizzleActivates: u8;
    static mut BattleScript_DroughtActivates: u8;
    static mut BattleScript_EncoredNoMore: u8;
    static mut BattleScript_FlashFireBoost: u8;
    static mut BattleScript_FlashFireBoost_PPLoss: u8;
    static mut BattleScript_GiveExp: u8;
    static mut BattleScript_HandleFaintedMon: u8;
    static mut BattleScript_IgnoresAndFallsAsleep: u8;
    static mut BattleScript_IgnoresAndHitsItself: u8;
    static mut BattleScript_IgnoresAndUsesRandomMove: u8;
    static mut BattleScript_IgnoresWhileAsleep: u8;
    static mut BattleScript_IngrainTurnHeal: u8;
    static mut BattleScript_IntimidateActivates: u8;
    static mut BattleScript_IntimidateActivatesEnd3: u8;
    static mut BattleScript_ItemHealHP_End2: u8;
    static mut BattleScript_ItemHealHP_RemoveItem: u8;
    static mut BattleScript_ItemHealHP_Ret: u8;
    static mut BattleScript_LeechSeedTurnDrain: u8;
    static mut BattleScript_MonMadeMoveUseless: u8;
    static mut BattleScript_MonMadeMoveUseless_PPLoss: u8;
    static mut BattleScript_MonTookFutureAttack: u8;
    static mut BattleScript_MonWokeUpInUproar: u8;
    static mut BattleScript_MoveHPDrain: u8;
    static mut BattleScript_MoveHPDrain_PPLoss: u8;
    static mut BattleScript_MoveUsedFlinched: u8;
    static mut BattleScript_MoveUsedIsAsleep: u8;
    static mut BattleScript_MoveUsedIsConfused: u8;
    static mut BattleScript_MoveUsedIsConfusedNoMore: u8;
    static mut BattleScript_MoveUsedIsDisabled: u8;
    static mut BattleScript_MoveUsedIsFrozen: u8;
    static mut BattleScript_MoveUsedIsImprisoned: u8;
    static mut BattleScript_MoveUsedIsInLove: u8;
    static mut BattleScript_MoveUsedIsInLoveCantAttack: u8;
    static mut BattleScript_MoveUsedIsParalyzed: u8;
    static mut BattleScript_MoveUsedIsTaunted: u8;
    static mut BattleScript_MoveUsedLoafingAround: u8;
    static mut BattleScript_MoveUsedMustRecharge: u8;
    static mut BattleScript_MoveUsedUnfroze: u8;
    static mut BattleScript_MoveUsedWokeUp: u8;
    static mut BattleScript_NightmareTurnDmg: u8;
    static mut BattleScript_NoMovesLeft: u8;
    static mut BattleScript_OverworldWeatherStarts: u8;
    static mut BattleScript_PerishSongCountGoesDown: u8;
    static mut BattleScript_PerishSongTakesLife: u8;
    static mut BattleScript_PoisonTurnDmg: u8;
    static mut BattleScript_PrintFailedToRunString: u8;
    static mut BattleScript_PrintUproarOverTurns: u8;
    static mut BattleScript_RainContinuesOrEnds: u8;
    static mut BattleScript_RainDishActivates: u8;
    static mut BattleScript_RoughSkinActivates: u8;
    static mut BattleScript_SafeguardEnds: u8;
    static mut BattleScript_SandStormHailEnds: u8;
    static mut BattleScript_SandstreamActivates: u8;
    static mut BattleScript_SelectingDisabledMove: u8;
    static mut BattleScript_SelectingDisabledMoveInPalace: u8;
    static mut BattleScript_SelectingImprisonedMove: u8;
    static mut BattleScript_SelectingImprisonedMoveInPalace: u8;
    static mut BattleScript_SelectingMoveWithNoPP: u8;
    static mut BattleScript_SelectingNotAllowedMoveChoiceItem: u8;
    static mut BattleScript_SelectingNotAllowedMoveTaunt: u8;
    static mut BattleScript_SelectingNotAllowedMoveTauntInPalace: u8;
    static mut BattleScript_SelectingTormentedMove: u8;
    static mut BattleScript_SelectingTormentedMoveInPalace: u8;
    static mut BattleScript_ShedSkinActivates: u8;
    static mut BattleScript_SideStatusWoreOff: u8;
    static mut BattleScript_SoundproofProtected: u8;
    static mut BattleScript_SpeedBoostActivates: u8;
    static mut BattleScript_SunlightContinues: u8;
    static mut BattleScript_SunlightFaded: u8;
    static mut BattleScript_SynchronizeActivates: u8;
    static mut BattleScript_ThrashConfuses: u8;
    static mut BattleScript_TraceActivates: u8;
    static mut BattleScript_WhiteHerbEnd2: u8;
    static mut BattleScript_WhiteHerbRet: u8;
    static mut BattleScript_WishComesTrue: u8;
    static mut BattleScript_WrapEnds: u8;
    static mut BattleScript_WrapTurnDmg: u8;
    static mut BattleScript_YawnMakesAsleep: u8;
    static mut gAbsentBattlerFlags: u8;
    static mut gActionSelectionCursor: u8;
    static mut gActionsByTurnOrder: u8;
    static mut gActiveBattler: u8;
    static mut gBattleBufferB: u8;
    static mut gBattleCommunication: u8;
    static mut gBattleControllerExecFlags: u8;
    static mut gBattleMainFunc: u8;
    static mut gBattleMons: u8;
    static mut gBattleMoveDamage: u8;
    static mut gBattleMoves: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleResources: u8;
    static mut gBattleResults: u8;
    static mut gBattleScripting: u8;
    static mut gBattleScriptingCommandsTable: u8;
    static mut gBattleScriptsForMoveEffects: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattleWeather: u8;
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerByTurnOrder: u8;
    static mut gBattlerFainted: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerTarget: u8;
    static mut gBattlersCount: u8;
    static mut gBattlescriptCurrInstr: u8;
    static mut gBattlescriptsForBallThrow: u8;
    static mut gBattlescriptsForRunningByItem: u8;
    static mut gBattlescriptsForSafariActions: u8;
    static mut gBattlescriptsForUsingItem: u8;
    static mut gBideDmg: u8;
    static mut gBideTarget: u8;
    static mut gBitTable: u8;
    static mut gCalledMove: u8;
    static mut gChosenActionByBattler: u8;
    static mut gChosenMove: u8;
    static mut gChosenMoveByBattler: u8;
    static mut gChosenMovePos: u8;
    static mut gCritMultiplier: u8;
    static mut gCurrMovePos: u8;
    static mut gCurrentActionFuncId: u8;
    static mut gCurrentMove: u8;
    static mut gCurrentTurnActionNumber: u8;
    static mut gDisableStructs: u8;
    static mut gDynamicBasePower: u8;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: u8;
    static mut gEnigmaBerries: u8;
    static mut gHitMarker: u8;
    static mut gLastHitByType: u8;
    static mut gLastLandedMoves: u8;
    static mut gLastMoves: u8;
    static mut gLastUsedAbility: u8;
    static mut gLastUsedItem: u8;
    static mut gLockedMoves: u8;
    static mut gMoveResultFlags: u8;
    static mut gMoveSelectionCursor: u8;
    static mut gMultiHitCounter: u8;
    static mut gNumSafariBalls: u8;
    static mut gPalaceSelectionBattleScripts: u8;
    static mut gPlayerParty: u8;
    static mut gPotentialItemEffectBattler: u8;
    static mut gProtectStructs: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectionBattleScripts: u8;
    static mut gSentPokesToOpponent: u8;
    static mut gSideStatuses: u8;
    static mut gSideTimers: u8;
    static mut gSpecialStatuses: u8;
    static mut gStatusConditionString_BurnJpn: u8;
    static mut gStatusConditionString_ConfusionJpn: u8;
    static mut gStatusConditionString_IceJpn: u8;
    static mut gStatusConditionString_LoveJpn: u8;
    static mut gStatusConditionString_ParalysisJpn: u8;
    static mut gStatusConditionString_PoisonJpn: u8;
    static mut gStatusConditionString_SleepJpn: u8;
    static mut gStatuses3: u8;
    static mut gWishFutureKnock: u8;
    fn BattleArena_AddMindPoints(a0: u8);
    fn BattleTurnPassed();
    fn BtlController_EmitPrintString(a0: u8, a1: u16);
    fn BtlController_EmitSetMonData(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut u8);
    fn CalculateBaseDamage(
        a0: *mut u8,
        a1: *mut u8,
        a2: u32,
        a3: u16,
        a4: u16,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> i32;
    fn CalculatePPWithBonus(a0: u16, a1: u8, a2: u8) -> u8;
    fn CountTrailingZeroBits(a0: u32) -> i32;
    fn CurrentBattlePyramidLocation() -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerMultiplayerId(a0: u16) -> i32;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerTurnOrderNum(a0: u8) -> u8;
    fn GetCurrentWeather() -> u8;
    fn GetFlavorRelationByPersonality(a0: u32, a1: u8) -> i8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetItemHoldEffect(a0: u16) -> u8;
    fn GetItemHoldEffectParam(a0: u16) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkTrainerFlankId(a0: u8) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetPyramidRunMultiplier() -> u8;
    fn GetWhoStrikesFirst(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsOtherTrainer(a0: u32, a1: *mut u8) -> u8;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn RecordAbilityBattle(a0: u8, a1: u8);
    fn RecordItemEffectBattle(a0: u8, a1: u8);
    fn RunBattleScriptCommands();
    fn RunBattleScriptCommands_PopCallbacksStack();
    fn SetMoveEffect(a0: u8, a1: u8);
    fn SpecialStatusesClear();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwapTurnOrder(a0: u8, a1: u8);
    fn UproarWakeUpCheck(a0: u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_UseMove() {
    unsafe {
        let mut side: u8 = 0u8;
        let mut var: u8 = 4u8;
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        if (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(145)).read())
            as u32)
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read())
            != 0
        {
            ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(12u8);
            return;
        }
        ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419)).write(0u8);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        ((&raw mut gMultiHitCounter).cast::<u8>()).write(0u8);
        (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(6)).write(0u8);
        ((&raw mut gCurrMovePos).cast::<u8>()).write({
            let __v1 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                .wrapping_add(128))
            .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read();
            ((&raw mut gChosenMovePos).cast::<u8>()).write(__v1);
            __v1
        });
        if (crate::c::bf_read(
            (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 16,
            ))
            .wrapping_add(0),
            2,
            1,
            false,
        ) as u32)
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 16,
                ))
                .wrapping_add(0),
                2,
                1,
                (0u32) as i32,
            );
            ((&raw mut gCurrentMove).cast::<u16>()).write({
                let __v2 = 165u16;
                ((&raw mut gChosenMove).cast::<u16>()).write(__v2);
                __v2
            });
            let __p3 = (&raw mut gHitMarker).cast::<u32>();
            (__p3).write(((__p3).read() | 2048u32));
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(12))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(GetMoveTarget(165u16, 0u8));
        } else {
            if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
            ))
            .wrapping_add(80)
            .cast::<u32>())
            .read()
                & 4096u32)
                != 0)
                || ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(80)
                .cast::<u32>())
                .read()
                    & 4194304u32)
                    != 0)
            {
                ((&raw mut gCurrentMove).cast::<u16>()).write({
                    let __v4 = ((((&raw mut gLockedMoves).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read();
                    ((&raw mut gChosenMove).cast::<u16>()).write(__v4);
                    __v4
                });
            } else {
                if (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32)
                    != 0i32)
                    && (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 28,
                    ))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32)
                        == ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 28,
                            ))
                            .wrapping_add(12))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32))
                {
                    ((&raw mut gCurrentMove).cast::<u16>()).write({
                        let __v5 = ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read();
                        ((&raw mut gChosenMove).cast::<u16>()).write(__v5);
                        __v5
                    });
                    ((&raw mut gCurrMovePos).cast::<u8>()).write({
                        let __v6 = ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(12))
                        .read();
                        ((&raw mut gChosenMovePos).cast::<u8>()).write(__v6);
                        __v6
                    });
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(12))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(GetMoveTarget(
                        ((&raw mut gCurrentMove).cast::<u16>()).read(),
                        0u8,
                    ));
                } else {
                    if (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 28,
                    ))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        && (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32)
                            != ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(12))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32))
                    {
                        ((&raw mut gCurrMovePos).cast::<u8>()).write({
                            let __v7 = ((((&raw mut gDisableStructs).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                            .wrapping_add(12))
                            .read();
                            ((&raw mut gChosenMovePos).cast::<u8>()).write(__v7);
                            __v7
                        });
                        ((&raw mut gCurrentMove).cast::<u16>()).write({
                            let __v8 = ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read();
                            ((&raw mut gChosenMove).cast::<u16>()).write(__v8);
                            __v8
                        });
                        ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .write(0u16);
                        ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(12))
                        .write(0u8);
                        crate::c::bf_write(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 28,
                            ))
                            .wrapping_add(14),
                            0,
                            4,
                            (0u8) as i32,
                        );
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .write(GetMoveTarget(
                            ((&raw mut gCurrentMove).cast::<u16>()).read(),
                            0u8,
                        ));
                    } else {
                        if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            != ((((((&raw mut gChosenMoveByBattler).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                        {
                            ((&raw mut gCurrentMove).cast::<u16>()).write({
                                let __v9 = ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(12))
                                .cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read();
                                ((&raw mut gChosenMove).cast::<u16>()).write(__v9);
                                __v9
                            });
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .write(GetMoveTarget(
                                ((&raw mut gCurrentMove).cast::<u16>()).read(),
                                0u8,
                            ));
                        } else {
                            ((&raw mut gCurrentMove).cast::<u16>()).write({
                                let __v10 = ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(12))
                                .cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read();
                                ((&raw mut gChosenMove).cast::<u16>()).write(__v10);
                                __v10
                            });
                        }
                    }
                }
            }
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(40)
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32) == 0i32
            {
                (((&raw mut gBattleResults).cast::<u8>())
                    .wrapping_add(34)
                    .cast::<u16>())
                .write(((&raw mut gCurrentMove).cast::<u16>()).read());
            } else {
                (((&raw mut gBattleResults).cast::<u8>())
                    .wrapping_add(36)
                    .cast::<u16>())
                .write(((&raw mut gCurrentMove).cast::<u16>()).read());
            }
        }
        side = ((((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32)
            ^ 1i32) as u8);
        if (((((((((&raw mut gSideTimers).cast::<u8>())
            .wrapping_offset(((side) as i32) as isize * 12))
        .wrapping_add(8))
        .read()) as i32)
            != 0i32)
            && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(6))
            .read()) as i32)
                == 0i32))
            && (((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32)
                != ((GetBattlerSide(
                    ((((&raw mut gSideTimers).cast::<u8>())
                        .wrapping_offset(((side) as i32) as isize * 12))
                    .wrapping_add(9))
                    .read(),
                )) as i32)))
            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSideTimers).cast::<u8>())
                    .wrapping_offset(((side) as i32) as isize * 12))
                .wrapping_add(9))
                .read()) as i32) as isize
                    * 88,
            ))
            .wrapping_add(40)
            .cast::<u16>())
            .read()) as i32)
                != 0i32)
        {
            ((&raw mut gBattlerTarget).cast::<u8>()).write(
                ((((&raw mut gSideTimers).cast::<u8>())
                    .wrapping_offset(((side) as i32) as isize * 12))
                .wrapping_add(9))
                .read(),
            );
        } else {
            if (((((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                && (((((((&raw mut gSideTimers).cast::<u8>())
                    .wrapping_offset(((side) as i32) as isize * 12))
                .wrapping_add(8))
                .read()) as i32)
                    == 0i32))
                && ((((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1))
                .read()) as i32)
                    != 0i32)
                    || (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(6))
                    .read()) as i32)
                        != 16i32)))
                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(12))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 88,
                ))
                .wrapping_add(32))
                .read()) as i32)
                    != 31i32))
                && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(2))
                .read()) as i32)
                    == 13i32)
            {
                side = GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                {
                    ((&raw mut gActiveBattler).cast::<u8>()).write(0u8);
                    'l1: loop {
                        if !(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            if (((((side) as i32)
                                != ((GetBattlerSide(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                )) as i32))
                                && (((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(12))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    != ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == 31i32))
                                && (((GetBattlerTurnOrderNum(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                )) as i32)
                                    < ((var) as i32))
                            {
                                var = GetBattlerTurnOrderNum(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                );
                            }
                        }
                        let __p11 = (&raw mut gActiveBattler).cast::<u8>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                if ((var) as i32) == 4i32 {
                    if (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gChosenMove).cast::<u16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(6))
                    .read()) as i32)
                        & 4i32)
                        != 0
                    {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            if (((Random()) as i32) & 1i32) != 0 {
                                ((&raw mut gBattlerTarget).cast::<u8>())
                                    .write(GetBattlerAtPosition(1u8));
                            } else {
                                ((&raw mut gBattlerTarget).cast::<u8>())
                                    .write(GetBattlerAtPosition(3u8));
                            }
                        } else {
                            if (((Random()) as i32) & 1i32) != 0 {
                                ((&raw mut gBattlerTarget).cast::<u8>())
                                    .write(GetBattlerAtPosition(0u8));
                            } else {
                                ((&raw mut gBattlerTarget).cast::<u8>())
                                    .write(GetBattlerAtPosition(2u8));
                            }
                        }
                    } else {
                        ((&raw mut gBattlerTarget).cast::<u8>()).write(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read(),
                        );
                    }
                    if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0
                    {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            != ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                                as i32)
                        {
                            ((&raw mut gBattlerTarget).cast::<u8>()).write(GetBattlerAtPosition(
                                ((((GetBattlerPosition(
                                    ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                                )) as i32)
                                    ^ 2i32) as u8),
                            ));
                        } else {
                            ((&raw mut gBattlerTarget).cast::<u8>()).write(GetBattlerAtPosition(
                                ((((GetBattlerPosition(
                                    ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                )) as i32)
                                    ^ 1i32) as u8),
                            ));
                            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read())
                                != 0
                            {
                                ((&raw mut gBattlerTarget).cast::<u8>()).write(
                                    GetBattlerAtPosition(
                                        ((((GetBattlerPosition(
                                            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                                        )) as i32)
                                            ^ 2i32) as u8),
                                    ),
                                );
                            }
                        }
                    }
                } else {
                    ((&raw mut gActiveBattler).cast::<u8>()).write(
                        (((&raw mut gBattlerByTurnOrder).cast::<u8>())
                            .wrapping_offset(((var) as i32) as isize))
                        .read(),
                    );
                    RecordAbilityBattle(
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(32))
                        .read(),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSpecialStatuses).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 20,
                        ))
                        .wrapping_add(0),
                        1,
                        1,
                        (1u32) as i32,
                    );
                    ((&raw mut gBattlerTarget).cast::<u8>())
                        .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                }
            } else {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                    && ((((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gChosenMove).cast::<u16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(6))
                    .read()) as i32)
                        & 4i32)
                        != 0)
                {
                    if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32)
                        == 0i32
                    {
                        if (((Random()) as i32) & 1i32) != 0 {
                            ((&raw mut gBattlerTarget).cast::<u8>())
                                .write(GetBattlerAtPosition(1u8));
                        } else {
                            ((&raw mut gBattlerTarget).cast::<u8>())
                                .write(GetBattlerAtPosition(3u8));
                        }
                    } else {
                        if (((Random()) as i32) & 1i32) != 0 {
                            ((&raw mut gBattlerTarget).cast::<u8>())
                                .write(GetBattlerAtPosition(0u8));
                        } else {
                            ((&raw mut gBattlerTarget).cast::<u8>())
                                .write(GetBattlerAtPosition(2u8));
                        }
                    }
                    if ((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0)
                        && (((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            != ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                                as i32))
                    {
                        ((&raw mut gBattlerTarget).cast::<u8>()).write(GetBattlerAtPosition(
                            ((((GetBattlerPosition(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                                as i32)
                                ^ 2i32) as u8),
                        ));
                    }
                } else {
                    ((&raw mut gBattlerTarget).cast::<u8>()).write(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0
                    {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            != ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                                as i32)
                        {
                            ((&raw mut gBattlerTarget).cast::<u8>()).write(GetBattlerAtPosition(
                                ((((GetBattlerPosition(
                                    ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                                )) as i32)
                                    ^ 2i32) as u8),
                            ));
                        } else {
                            ((&raw mut gBattlerTarget).cast::<u8>()).write(GetBattlerAtPosition(
                                ((((GetBattlerPosition(
                                    ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                )) as i32)
                                    ^ 1i32) as u8),
                            ));
                            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read())
                                != 0
                            {
                                ((&raw mut gBattlerTarget).cast::<u8>()).write(
                                    GetBattlerAtPosition(
                                        ((((GetBattlerPosition(
                                            ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                                        )) as i32)
                                            ^ 2i32) as u8),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0)
            && ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 16,
                ))
                .wrapping_add(2),
                4,
                1,
                false,
            ) as u32)
                != 0)
        {
            if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
            ))
            .wrapping_add(40)
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(12u8);
                return;
            } else {
                if ((((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as usize)
                    != 0usize
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(4u8);
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                        ((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    ((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(core::ptr::null_mut());
                } else {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(4u8);
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                        .write((&raw mut BattleScript_MoveUsedLoafingAround).cast::<u8>());
                }
            }
        } else {
            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                ((((&raw mut gBattleScriptsForMoveEffects).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
                        ))
                        .read()) as i32) as isize,
                    ))
                .read(),
            );
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0 {
            BattleArena_AddMindPoints(((&raw mut gBattlerAttacker).cast::<u8>()).read());
        }
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_Switch() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        {
            ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1)).write(7u8);
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3)).write(
                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(88))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4)).write(255u8);
        }
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
            .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
            .write((&raw mut BattleScript_ActionSwitch).cast::<u8>());
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
        if (((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(2)).read()) as i32) < 255i32 {
            let __p1 = ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_UseItem() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write({
            let __v1 = (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read();
            ((&raw mut gBattlerTarget).cast::<u8>()).write(__v1);
            __v1
        });
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ClearFuryCutterDestinyBondGrudge(((&raw mut gBattlerAttacker).cast::<u8>()).read());
        ((&raw mut gLastUsedItem).cast::<u16>()).write(
            (((((((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                | ((((((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i32)
                    << 8)) as u16),
        );
        if ((((&raw mut gLastUsedItem).cast::<u16>()).read()) as i32) <= 12i32 {
            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                ((((&raw mut gBattlescriptsForBallThrow).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gLastUsedItem).cast::<u16>()).read()) as i32) as isize,
                    ))
                .read(),
            );
        } else {
            if (((((&raw mut gLastUsedItem).cast::<u16>()).read()) as i32) == 80i32)
                || (((((&raw mut gLastUsedItem).cast::<u16>()).read()) as i32) == 81i32)
            {
                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                    (((&raw mut gBattlescriptsForRunningByItem).cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .read(),
                );
            } else {
                if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                        (((&raw mut gBattlescriptsForUsingItem).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .read(),
                    );
                } else {
                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                        .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                    'l1: {
                        let __sw2 = ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(196))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) >> 1)
                                as isize,
                        ))
                        .read()) as i32);
                        if __sw2 == 1i32 || __sw2 == 2i32 {
                            break 'l1;
                        }
                        if __sw2 == 3i32 {
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(0u8);
                            if (((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(198))
                            .cast::<u8>())
                            .wrapping_offset(
                                (crate::c::div_i32(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32),
                                    2i32,
                                )) as isize,
                            ))
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                if (((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                            as i32),
                                        2i32,
                                    )) as isize,
                                ))
                                .read()) as i32)
                                    & 62i32)
                                    != 0
                                {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(5u8);
                                }
                            } else {
                                'l2: loop {
                                    if !(!((((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(198))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (crate::c::div_i32(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32),
                                            2i32,
                                        )) as isize,
                                    ))
                                    .read()) as i32)
                                        & 1i32)
                                        != 0))
                                    {
                                        break 'l2;
                                    }
                                    let __p3 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(198))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (crate::c::div_i32(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32),
                                            2i32,
                                        )) as isize,
                                    );
                                    (__p3).write((((((__p3).read()) as i32) >> 1) as u8));
                                    let __p4 = ((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5);
                                    (__p4).write(((__p4).read()).wrapping_add(1));
                                }
                            }
                            break 'l1;
                        }
                        if __sw2 == 4i32 {
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(4u8);
                            if (((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(198))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) >> 1)
                                    as isize,
                            ))
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(5u8);
                            } else {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(1u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                {
                                    ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(1))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(2))
                                        .write(210u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(3))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(4))
                                        .write(255u8);
                                }
                                'l3: loop {
                                    if !(!((((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(198))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                            as i32)
                                            >> 1) as isize,
                                    ))
                                    .read()) as i32)
                                        & 1i32)
                                        != 0))
                                    {
                                        break 'l3;
                                    }
                                    let __p5 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(198))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (crate::c::div_i32(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32),
                                            2i32,
                                        )) as isize,
                                    );
                                    (__p5).write((((((__p5).read()) as i32) >> 1) as u8));
                                    let __p6 = ((&raw mut gBattleTextBuff1).cast::<u8>())
                                        .wrapping_offset(2);
                                    (__p6).write(((__p6).read()).wrapping_add(1));
                                }
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(
                                        (((((((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(2))
                                        .read()) as i32)
                                            .wrapping_add(14i32))
                                            as u8),
                                    );
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(0u8);
                            }
                            break 'l1;
                        }
                        if __sw2 == 5i32 {
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(0u8);
                            break 'l1;
                        }
                    }
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                        ((((&raw mut gBattlescriptsForUsingItem).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(196))
                            .cast::<u8>())
                            .wrapping_offset(
                                (crate::c::div_i32(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32),
                                    2i32,
                                )) as isize,
                            ))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                }
            }
        }
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryRunFromBattle(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut effect: u8 = 0u8;
        let mut holdEffect: u8 = 0u8;
        let mut pyramidMultiplier: u8 = 0u8;
        let mut speedVar: u8 = 0u8;
        if ((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32
        {
            holdEffect = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(
                ((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
        }
        ((&raw mut gPotentialItemEffectBattler).cast::<u8>()).write(battler);
        if ((holdEffect) as i32) == 37i32 {
            ((&raw mut gLastUsedItem).cast::<u16>()).write(
                ((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
            crate::c::bf_write(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(1),
                3,
                2,
                (1u32) as i32,
            );
            effect = (effect).wrapping_add(1);
        } else {
            if ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(32))
            .read()) as i32)
                == 50i32
            {
                if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                    let __p1 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(108);
                    (__p1).write(((__p1).read()).wrapping_add(1));
                    pyramidMultiplier = GetPyramidRunMultiplier();
                    speedVar = (((crate::c::div_i32(
                        ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 88))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_mul(((pyramidMultiplier) as i32)),
                        ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset((((battler) as i32) ^ 1i32) as isize * 88))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32),
                    ))
                    .wrapping_add(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(108))
                            .read()) as i32)
                            .wrapping_mul(30i32),
                    )) as u8);
                    if ((speedVar) as i32) > (((Random()) as i32) & 255i32) {
                        ((&raw mut gLastUsedAbility).cast::<u8>()).write(50u8);
                        crate::c::bf_write(
                            (((&raw mut gProtectStructs).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 16))
                            .wrapping_add(1),
                            3,
                            2,
                            (2u32) as i32,
                        );
                        effect = (effect).wrapping_add(1);
                    }
                } else {
                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(50u8);
                    crate::c::bf_write(
                        (((&raw mut gProtectStructs).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 16))
                        .wrapping_add(1),
                        3,
                        2,
                        (2u32) as i32,
                    );
                    effect = (effect).wrapping_add(1);
                }
            } else {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 71237888u32) != 0)
                    && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0)
                {
                    effect = (effect).wrapping_add(1);
                } else {
                    if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
                        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                            pyramidMultiplier = GetPyramidRunMultiplier();
                            speedVar = (((crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_mul(((pyramidMultiplier) as i32)),
                                ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((((battler) as i32) ^ 1i32) as isize * 88))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read()) as i32),
                            ))
                            .wrapping_add(
                                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(108))
                                .read()) as i32)
                                    .wrapping_mul(30i32),
                            )) as u8);
                            if ((speedVar) as i32) > (((Random()) as i32) & 255i32) {
                                effect = (effect).wrapping_add(1);
                            }
                        } else {
                            if ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read()) as i32)
                                < ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((((battler) as i32) ^ 1i32) as isize * 88))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read()) as i32)
                            {
                                speedVar = (((crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        .wrapping_mul(128i32),
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        (((battler) as i32) ^ 1i32) as isize * 88,
                                    ))
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read()) as i32),
                                ))
                                .wrapping_add(
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(108))
                                    .read()) as i32)
                                        .wrapping_mul(30i32),
                                )) as u8);
                                if ((speedVar) as i32) > (((Random()) as i32) & 255i32) {
                                    effect = (effect).wrapping_add(1);
                                }
                            } else {
                                effect = (effect).wrapping_add(1);
                            }
                        }
                    }
                    let __p2 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(108);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
            }
        }
        if ((effect) as i32) != 0i32 {
            ((&raw mut gCurrentTurnActionNumber).cast::<u8>())
                .write(((&raw mut gBattlersCount).cast::<u8>()).read());
            ((&raw mut gBattleOutcome).cast::<u8>()).write(4u8);
        }
        return effect;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_Run() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
            ((&raw mut gCurrentTurnActionNumber).cast::<u8>())
                .write(((&raw mut gBattlersCount).cast::<u8>()).read());
            {
                ((&raw mut gActiveBattler).cast::<u8>()).write(0u8);
                'l1: loop {
                    if !(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                        < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            if (((((&raw mut gChosenActionByBattler).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                                == 3i32
                            {
                                let __p1 = (&raw mut gBattleOutcome).cast::<u8>();
                                (__p1).write((((((__p1).read()) as i32) | 2i32) as u8));
                            }
                        } else {
                            if (((((&raw mut gChosenActionByBattler).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                                == 3i32
                            {
                                let __p2 = (&raw mut gBattleOutcome).cast::<u8>();
                                (__p2).write((((((__p2).read()) as i32) | 1i32) as u8));
                            }
                        }
                    }
                    let __p3 = (&raw mut gActiveBattler).cast::<u8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
            }
            let __p4 = (&raw mut gBattleOutcome).cast::<u8>();
            (__p4).write((((((__p4).read()) as i32) | 128i32) as u8));
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                3,
                1,
                (1u8) as i32,
            );
        } else {
            if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32) == 0i32
            {
                if !((TryRunFromBattle(((&raw mut gBattlerAttacker).cast::<u8>()).read())) != 0) {
                    ClearFuryCutterDestinyBondGrudge(
                        ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                    );
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(3u8);
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                        .write((&raw mut BattleScript_PrintFailedToRunString).cast::<u8>());
                    ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
                }
            } else {
                if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(80)
                .cast::<u32>())
                .read()
                    & 67166208u32)
                    != 0
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(4u8);
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                        .write((&raw mut BattleScript_PrintFailedToRunString).cast::<u8>());
                    ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
                } else {
                    ((&raw mut gCurrentTurnActionNumber).cast::<u8>())
                        .write(((&raw mut gBattlersCount).cast::<u8>()).read());
                    ((&raw mut gBattleOutcome).cast::<u8>()).write(6u8);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_WatchesCarefully() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
            (((&raw mut gBattlescriptsForSafariActions).cast::<*mut u8>()).cast::<*mut u8>())
                .read(),
        );
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_SafariZoneBallThrow() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        let __p1 = (&raw mut gNumSafariBalls).cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        ((&raw mut gLastUsedItem).cast::<u16>()).write(5u16);
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
            ((((&raw mut gBattlescriptsForBallThrow).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(5))
            .read(),
        );
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_ThrowPokeblock() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(
            (((((((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_sub(1i32)) as u8),
        );
        ((&raw mut gLastUsedItem).cast::<u16>()).write(
            (((((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as u16),
        );
        if (((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(31)).read()) as i32) < 255i32 {
            let __p1 = ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(31);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(122)).read())
            as i32)
            < 3i32
        {
            let __p2 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(122);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123)).read())
            as i32)
            > 1i32
        {
            if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123)).read())
                as i32)
                < ((((((((&raw const sPkblToEscapeFactor).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(122))
                            .read()) as i32) as isize
                            * 3,
                    ))
                .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read()) as i32)
            {
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123))
                    .write(1u8);
            } else {
                let __p3 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        ((((((((&raw const sPkblToEscapeFactor).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(122))
                            .read()) as i32) as isize
                                * 3,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .read()) as i32) as isize,
                        ))
                        .read()) as i32),
                    )) as u8),
                );
            }
        }
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
            ((((&raw mut gBattlescriptsForSafariActions).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(2))
            .read(),
        );
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_GoNear() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        let __p1 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(124);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw const sGoNearCounterToCatchFactor)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(121))
                        .read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as u8),
        );
        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(124)).read())
            as i32)
            > 20i32
        {
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(124)).write(20u8);
        }
        let __p2 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((&raw const sGoNearCounterToEscapeFactor)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(121))
                        .read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as u8),
        );
        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123)).read())
            as i32)
            > 20i32
        {
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123)).write(20u8);
        }
        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(121)).read())
            as i32)
            < 3i32
        {
            let __p3 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(121);
            (__p3).write(((__p3).read()).wrapping_add(1));
            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(0u8);
        } else {
            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(1u8);
        }
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
            ((((&raw mut gBattlescriptsForSafariActions).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(1))
            .read(),
        );
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_SafariZoneRun() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        PlaySE(17u16);
        ((&raw mut gCurrentTurnActionNumber).cast::<u8>())
            .write(((&raw mut gBattlersCount).cast::<u8>()).read());
        ((&raw mut gBattleOutcome).cast::<u8>()).write(4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_WallyBallThrow() {
    unsafe {
        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
            (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        {
            ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1)).write(7u8);
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3)).write(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as u8),
            );
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4)).write(255u8);
        }
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
            ((((&raw mut gBattlescriptsForSafariActions).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(3))
            .read(),
        );
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(10u8);
        (((&raw mut gActionsByTurnOrder).cast::<u8>()).wrapping_offset(1)).write(12u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_TryFinish() {
    unsafe {
        if !((HandleFaintedMonActions()) != 0) {
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77)).write(0u8);
            ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(12u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_NothingIsFainted() {
    unsafe {
        let __p1 = (&raw mut gCurrentTurnActionNumber).cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(
            (((&raw mut gActionsByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        let __p2 = (&raw mut gHitMarker).cast::<u32>();
        (__p2).write(((__p2).read() & 4058550959u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_ActionFinished() {
    unsafe {
        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(92)).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
        .write(6u8);
        let __p1 = (&raw mut gCurrentTurnActionNumber).cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(
            (((&raw mut gActionsByTurnOrder).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        SpecialStatusesClear();
        let __p2 = (&raw mut gHitMarker).cast::<u32>();
        (__p2).write(((__p2).read() & 4058550959u32));
        ((&raw mut gCurrentMove).cast::<u16>()).write(0u16);
        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(0i32);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(24)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(25)).write(0u8);
        ((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastHitByType).cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).write(0u8);
        ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(20)).write(0u8);
        (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3)).write(0u8);
        (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(4)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(22)).write(0u8);
        ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerForBattleScript(caseId: u8) -> u8 {
    unsafe {
        let mut caseId = caseId;
        let mut ret: u8 = 0u8;
        'l1: {
            let __sw1 = ((caseId) as i32);
            if __sw1 == 0i32 {
                ret = ((&raw mut gBattlerTarget).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 1i32 {
                ret = ((&raw mut gBattlerAttacker).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 2i32 {
                ret = ((&raw mut gEffectBattler).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 7i32 {
                ret = 0u8;
                break 'l1;
            }
            if __sw1 == 10i32 {
                ret = (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).read();
                break 'l1;
            }
            if __sw1 == 3i32 {
                ret = ((&raw mut gBattlerFainted).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 5i32 {
                ret = ((&raw mut gBattlerFainted).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 6i32 || __sw1 == 8i32 || __sw1 == 9i32 || __sw1 == 11i32 {
                ret = GetBattlerAtPosition(0u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                ret = GetBattlerAtPosition(1u8);
                break 'l1;
            }
            if __sw1 == 13i32 {
                ret = GetBattlerAtPosition(2u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                ret = GetBattlerAtPosition(3u8);
                break 'l1;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PressurePPLose(target: u8, attacker: u8, r#move: u16) {
    unsafe {
        let mut target = target;
        let mut attacker = attacker;
        let mut r#move = r#move;
        let mut moveIndex: i32 = 0i32;
        if ((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((target) as i32) as isize * 88))
        .wrapping_add(32))
        .read()) as i32)
            != 46i32
        {
            return;
        }
        {
            moveIndex = 0i32;
            'l1: loop {
                if !(moveIndex < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((attacker) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((moveIndex) as isize))
                    .read()) as i32)
                        == ((r#move) as i32)
                    {
                        break 'l1;
                    }
                }
                moveIndex = (moveIndex).wrapping_add(1);
            }
        }
        if moveIndex == 4i32 {
            return;
        }
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((attacker) as i32) as isize * 88))
        .wrapping_add(36))
        .cast::<u8>())
        .wrapping_offset((moveIndex) as isize))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((attacker) as i32) as isize * 88))
            .wrapping_add(36))
            .cast::<u8>())
            .wrapping_offset((moveIndex) as isize);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        if (!((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((attacker) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>())
        .read()
            & 2097152u32)
            != 0))
            && (!((((crate::c::bf_read(
                (((&raw mut gDisableStructs).cast::<u8>())
                    .wrapping_offset(((attacker) as i32) as isize * 28))
                .wrapping_add(24),
                4,
                4,
                false,
            ) as u8) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                    .wrapping_offset((moveIndex) as isize))
                .read())
                != 0))
        {
            ((&raw mut gActiveBattler).cast::<u8>()).write(attacker);
            BtlController_EmitSetMonData(
                0u8,
                (((9i32).wrapping_add(moveIndex)) as u8),
                0u8,
                1u8,
                (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(36))
                .cast::<u8>())
                .wrapping_offset((moveIndex) as isize),
            );
            MarkBattlerForControllerExec(((&raw mut gActiveBattler).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PressurePPLoseOnUsingImprison(attacker: u8) {
    unsafe {
        let mut attacker = attacker;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut imprisonPos: i32 = 4i32;
        let mut atkSide: u8 = GetBattlerSide(attacker);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((atkSide) as i32) != ((GetBattlerSide(((i) as u8))) as i32))
                        && (((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(32))
                        .read()) as i32)
                            == 46i32)
                    {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((attacker) as i32) as isize * 88))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        == 286i32
                                    {
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if j != 4i32 {
                            imprisonPos = j;
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((attacker) as i32) as isize * 88))
                            .wrapping_add(36))
                            .cast::<u8>())
                            .wrapping_offset((j) as isize))
                            .read()) as i32)
                                != 0i32
                            {
                                let __p1 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((attacker) as i32) as isize * 88))
                                .wrapping_add(36))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize);
                                (__p1).write(((__p1).read()).wrapping_sub(1));
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (imprisonPos != 4i32)
            && ((!((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((attacker) as i32) as isize * 88))
            .wrapping_add(80)
            .cast::<u32>())
            .read()
                & 2097152u32)
                != 0))
                && (!((((crate::c::bf_read(
                    (((&raw mut gDisableStructs).cast::<u8>())
                        .wrapping_offset(((attacker) as i32) as isize * 28))
                    .wrapping_add(24),
                    4,
                    4,
                    false,
                ) as u8) as u32)
                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset((imprisonPos) as isize))
                    .read())
                    != 0)))
        {
            ((&raw mut gActiveBattler).cast::<u8>()).write(attacker);
            BtlController_EmitSetMonData(
                0u8,
                (((9i32).wrapping_add(imprisonPos)) as u8),
                0u8,
                1u8,
                (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(36))
                .cast::<u8>())
                .wrapping_offset((imprisonPos) as isize),
            );
            MarkBattlerForControllerExec(((&raw mut gActiveBattler).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PressurePPLoseOnUsingPerishSong(attacker: u8) {
    unsafe {
        let mut attacker = attacker;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut perishSongPos: i32 = 4i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(32))
                    .read()) as i32)
                        == 46i32)
                        && (i != ((attacker) as i32))
                    {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((attacker) as i32) as isize * 88))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        == 195i32
                                    {
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if j != 4i32 {
                            perishSongPos = j;
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((attacker) as i32) as isize * 88))
                            .wrapping_add(36))
                            .cast::<u8>())
                            .wrapping_offset((j) as isize))
                            .read()) as i32)
                                != 0i32
                            {
                                let __p1 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((attacker) as i32) as isize * 88))
                                .wrapping_add(36))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize);
                                (__p1).write(((__p1).read()).wrapping_sub(1));
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (perishSongPos != 4i32)
            && ((!((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((attacker) as i32) as isize * 88))
            .wrapping_add(80)
            .cast::<u32>())
            .read()
                & 2097152u32)
                != 0))
                && (!((((crate::c::bf_read(
                    (((&raw mut gDisableStructs).cast::<u8>())
                        .wrapping_offset(((attacker) as i32) as isize * 28))
                    .wrapping_add(24),
                    4,
                    4,
                    false,
                ) as u8) as u32)
                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset((perishSongPos) as isize))
                    .read())
                    != 0)))
        {
            ((&raw mut gActiveBattler).cast::<u8>()).write(attacker);
            BtlController_EmitSetMonData(
                0u8,
                (((9i32).wrapping_add(perishSongPos)) as u8),
                0u8,
                1u8,
                (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(36))
                .cast::<u8>())
                .wrapping_offset((perishSongPos) as isize),
            );
            MarkBattlerForControllerExec(((&raw mut gActiveBattler).cast::<u8>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn MarkAllBattlersForControllerExec() {
    unsafe {
        let mut i: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        let __p1 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
                        (__p1).write(
                            ((__p1).read()
                                | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()
                                    << 28)),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        let __p2 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
                        (__p2).write(
                            ((__p2).read()
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MarkBattlerForControllerExec(battler: u8) {
    unsafe {
        let mut battler = battler;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            let __p1 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
            (__p1).write(
                ((__p1).read()
                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()
                        << 28)),
            );
        } else {
            let __p2 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
            (__p2).write(
                ((__p2).read()
                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MarkBattlerReceivedLinkData(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
                    (__p1).write(
                        ((__p1).read()
                            | crate::c::shl_u32(
                                ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                .read(),
                                ((i << 2) as u32),
                            )),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
        (__p2).write(
            ((__p2).read() & ((!(crate::c::shl_i32(268435456i32, ((battler) as u32)))) as u32)),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CancelMultiTurnMoves(battler: u8) {
    unsafe {
        let mut battler = battler;
        let __p1 = (((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>();
        (__p1).write(((__p1).read() & 4294963199u32));
        let __p2 = (((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>();
        (__p2).write(((__p2).read() & 4294964223u32));
        let __p3 = (((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>();
        (__p3).write(((__p3).read() & 4294967183u32));
        let __p4 = (((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>();
        (__p4).write(((__p4).read() & 4294966527u32));
        let __p5 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((battler) as i32) as isize);
        (__p5).write(((__p5).read() & 4294704959u32));
        crate::c::bf_write(
            (((&raw mut gDisableStructs).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(17),
            0,
            4,
            (0u8) as i32,
        );
        ((((&raw mut gDisableStructs).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 28))
        .wrapping_add(16))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WasUnableToUseMove(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        if (((((((((crate::c::bf_read(
            (((&raw mut gProtectStructs).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 16))
            .wrapping_add(0),
            7,
            1,
            false,
        ) as u32)
            != 0)
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(1),
                1,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(1),
                5,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(1),
                6,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(1),
                7,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(2),
                0,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(2),
                1,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(2),
                2,
                1,
                false,
            ) as u32)
                != 0))
            || ((crate::c::bf_read(
                (((&raw mut gProtectStructs).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 16))
                .wrapping_add(1),
                0,
                1,
                false,
            ) as u32)
                != 0)
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
pub unsafe extern "C" fn PrepareStringBattle(stringId: u16, battler: u8) {
    unsafe {
        let mut stringId = stringId;
        let mut battler = battler;
        ((&raw mut gActiveBattler).cast::<u8>()).write(battler);
        BtlController_EmitPrintString(0u8, stringId);
        MarkBattlerForControllerExec(((&raw mut gActiveBattler).cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSentPokesToOpponentValue() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut bits: u32 = 0u32;
        ((&raw mut gSentPokesToOpponent).cast::<u8>()).write(0u8);
        (((&raw mut gSentPokesToOpponent).cast::<u8>()).wrapping_offset(1)).write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    bits = (bits
                        | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read());
                }
                i = (i).wrapping_add(2i32);
            }
        }
        {
            i = 1i32;
            'l3: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut gSentPokesToOpponent).cast::<u8>())
                        .wrapping_offset(((i & 2i32) >> 1) as isize))
                    .write(((bits) as u8));
                }
                i = (i).wrapping_add(2i32);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpponentSwitchInResetSentPokesToOpponentValue(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut i: i32 = 0i32;
        let mut bits: u32 = 0u32;
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            let mut flank: u8 = (((((battler) as i32) & 2i32) >> 1) as u8);
            (((&raw mut gSentPokesToOpponent).cast::<u8>())
                .wrapping_offset(((flank) as i32) as isize))
            .write(0u8);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if !((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read())
                            != 0)
                        {
                            bits = (bits
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                .read());
                        }
                    }
                    i = (i).wrapping_add(2i32);
                }
            }
            (((&raw mut gSentPokesToOpponent).cast::<u8>())
                .wrapping_offset(((flank) as i32) as isize))
            .write(((bits) as u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSentPokesToOpponentValue(battler: u8) {
    unsafe {
        let mut battler = battler;
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            OpponentSwitchInResetSentPokesToOpponentValue(battler);
        } else {
            let mut i: i32 = 0i32;
            {
                i = 1i32;
                'l1: loop {
                    if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        let __p1 = ((&raw mut gSentPokesToOpponent).cast::<u8>())
                            .wrapping_offset(((i & 2i32) >> 1) as isize);
                        (__p1).write(
                            (((((__p1).read()) as u32)
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((battler) as i32) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                .read()) as u8),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleScriptPush(bsPtr: *mut u8) {
    unsafe {
        let mut bsPtr = bsPtr;
        (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(bsPtr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleScriptPushCursor() {
    unsafe {
        (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleScriptPop() {
    unsafe {
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
            (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .cast::<*mut u8>())
            .wrapping_offset(
                (({
                    let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32);
                    let __t2 = ((__p1).read()).wrapping_sub(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32) as isize,
            ))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetCantSelectMoveBattleScript() -> u8 {
    unsafe {
        let mut limitations: u8 = 0u8;
        let mut r#move: u16 = ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(12))
        .cast::<u16>())
        .wrapping_offset(
            (((((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32) as isize,
        ))
        .read();
        let mut holdEffect: u8 = 0u8;
        let mut choicedMove: *mut u16 = (((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
            .wrapping_add(200))
        .cast::<u16>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize);
        if (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
        ))
        .wrapping_add(4)
        .cast::<u16>())
        .read()) as i32)
            == ((r#move) as i32))
            && (((r#move) as i32) != 0i32)
        {
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                .write(((&raw mut gActiveBattler).cast::<u8>()).read());
            ((&raw mut gCurrentMove).cast::<u16>()).write(r#move);
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                ((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingDisabledMoveInPalace).cast::<u8>());
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
            } else {
                ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingDisabledMove).cast::<u8>());
                limitations = 1u8;
            }
        }
        if ((((r#move) as i32)
            == ((((((&raw mut gLastMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32))
            && (((r#move) as i32) != 165i32))
            && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
            ))
            .wrapping_add(80)
            .cast::<u32>())
            .read()
                & 2147483648u32)
                != 0)
        {
            CancelMultiTurnMoves(((&raw mut gActiveBattler).cast::<u8>()).read());
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                ((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingTormentedMoveInPalace).cast::<u8>());
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
            } else {
                ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingTormentedMove).cast::<u8>());
                limitations = (limitations).wrapping_add(1);
            }
        }
        if (((crate::c::bf_read(
            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(19),
            0,
            4,
            false,
        ) as u8) as i32)
            != 0i32)
            && (((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(1))
            .read()) as i32)
                == 0i32)
        {
            ((&raw mut gCurrentMove).cast::<u16>()).write(r#move);
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                ((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingNotAllowedMoveTauntInPalace).cast::<u8>());
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
            } else {
                ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingNotAllowedMoveTaunt).cast::<u8>());
                limitations = (limitations).wrapping_add(1);
            }
        }
        if (GetImprisonedMovesCount(((&raw mut gActiveBattler).cast::<u8>()).read(), r#move)) != 0 {
            ((&raw mut gCurrentMove).cast::<u16>()).write(r#move);
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                ((((&raw mut gPalaceSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingImprisonedMoveInPalace).cast::<u8>());
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
            } else {
                ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingImprisonedMove).cast::<u8>());
                limitations = (limitations).wrapping_add(1);
            }
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32
        {
            holdEffect = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(7))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(
                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
        }
        ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
        if (((((holdEffect) as i32) == 29i32) && ((((choicedMove).read()) as i32) != 0i32))
            && ((((choicedMove).read()) as i32) != 65535i32))
            && ((((choicedMove).read()) as i32) != ((r#move) as i32))
        {
            ((&raw mut gCurrentMove).cast::<u16>()).write((choicedMove).read());
            ((&raw mut gLastUsedItem).cast::<u16>()).write(
                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
            } else {
                ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingNotAllowedMoveChoiceItem).cast::<u8>());
                limitations = (limitations).wrapping_add(1);
            }
        }
        if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(36))
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            == 0i32
        {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
            } else {
                ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .write((&raw mut BattleScript_SelectingMoveWithNoPP).cast::<u8>());
                limitations = (limitations).wrapping_add(1);
            }
        }
        return limitations;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckMoveLimitations(battler: u8, unusableMoves: u8, check: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut unusableMoves = unusableMoves;
        let mut check = check;
        let mut holdEffect: u8 = 0u8;
        let mut choicedMove: *mut u16 = (((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
            .wrapping_add(200))
        .cast::<u16>())
        .wrapping_offset(((battler) as i32) as isize);
        let mut i: i32 = 0i32;
        if ((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32
        {
            holdEffect = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(
                ((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
        }
        ((&raw mut gPotentialItemEffectBattler).cast::<u8>()).write(battler);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32)
                        && ((((check) as i32) & 1i32) != 0)
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if (((((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(36))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32)
                        && ((((check) as i32) & 2i32) != 0)
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if (((((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((((((&raw mut gDisableStructs).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 28))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32))
                        && ((((check) as i32) & 4i32) != 0)
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if ((((((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((((((&raw mut gLastMoves).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32))
                        && ((((check) as i32) & 8i32) != 0))
                        && ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 2147483648u32)
                            != 0)
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if (((crate::c::bf_read(
                        (((&raw mut gDisableStructs).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 28))
                        .wrapping_add(19),
                        0,
                        4,
                        false,
                    ) as u8)
                        != 0)
                        && ((((check) as i32) & 16i32) != 0))
                        && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 12,
                        ))
                        .wrapping_add(1))
                        .read()) as i32)
                            == 0i32)
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if ((GetImprisonedMovesCount(
                        battler,
                        ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 88))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    )) != 0)
                        && ((((check) as i32) & 32i32) != 0)
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if ((crate::c::bf_read(
                        (((&raw mut gDisableStructs).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 28))
                        .wrapping_add(14),
                        0,
                        4,
                        false,
                    ) as u8)
                        != 0)
                        && (((((((&raw mut gDisableStructs).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32)
                            != ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32))
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                    if (((((holdEffect) as i32) == 29i32)
                        && ((((choicedMove).read()) as i32) != 0i32))
                        && ((((choicedMove).read()) as i32) != 65535i32))
                        && ((((choicedMove).read()) as i32)
                            != ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32))
                    {
                        unusableMoves = ((((unusableMoves) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return unusableMoves;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AreAllMovesUnusable() -> u8 {
    unsafe {
        let mut unusable: u8 =
            CheckMoveLimitations(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8, 255u8);
        if ((unusable) as i32) == 15i32 {
            crate::c::bf_write(
                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                ))
                .wrapping_add(0),
                2,
                1,
                (1u32) as i32,
            );
            ((((&raw mut gSelectionBattleScripts).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
            .write((&raw mut BattleScript_NoMovesLeft).cast::<u8>());
        } else {
            crate::c::bf_write(
                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                ))
                .wrapping_add(0),
                2,
                1,
                (0u32) as i32,
            );
        }
        return ((((unusable) as i32) == 15i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetImprisonedMovesCount(battler: u8, r#move: u16) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        let mut imprisonedMoves: u8 = 0u8;
        let mut battlerSide: u8 = GetBattlerSide(battler);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((battlerSide) as i32) != ((GetBattlerSide(((i) as u8))) as i32))
                        && ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset((i) as isize))
                        .read()
                            & 8192u32)
                            != 0)
                    {
                        let mut j: i32 = 0i32;
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((r#move) as i32)
                                        == ((((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset((i) as isize * 88))
                                        .wrapping_add(12))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if j < 4i32 {
                            imprisonedMoves = (imprisonedMoves).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return imprisonedMoves;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoFieldEndTurnEffects() -> u8 {
    unsafe {
        let mut effect: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            ((&raw mut gBattlerAttacker).cast::<u8>()).write(0u8);
            'l1: loop {
                if !((((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                    && ((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0))
                {
                    break 'l1;
                }
                'l2: {}
                let __p1 = (&raw mut gBattlerAttacker).cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        {
            ((&raw mut gBattlerTarget).cast::<u8>()).write(0u8);
            'l3: loop {
                if !((((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                    && ((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0))
                {
                    break 'l3;
                }
                'l4: {}
                let __p2 = (&raw mut gBattlerTarget).cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        }
        'l5: loop {
            'l6: {
                let mut side: u8 = 0u8;
                'l7: {
                    let __sw3 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .read()) as i32);
                    let mut __fall = false;
                    if __sw3 == 0i32 {
                        __fall = true;
                        {
                            i = 0i32;
                            'l8: loop {
                                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                                {
                                    break 'l8;
                                }
                                'l9: {
                                    (((&raw mut gBattlerByTurnOrder).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(((i) as u8));
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = 0i32;
                            'l10: loop {
                                if !(i
                                    < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                                        .wrapping_sub(1i32))
                                {
                                    break 'l10;
                                }
                                'l11: {
                                    let mut j: i32 = 0i32;
                                    {
                                        j = (i).wrapping_add(1i32);
                                        'l12: loop {
                                            if !(j
                                                < ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                                    as i32))
                                            {
                                                break 'l12;
                                            }
                                            'l13: {
                                                if (GetWhoStrikesFirst(
                                                    (((&raw mut gBattlerByTurnOrder).cast::<u8>())
                                                        .wrapping_offset((i) as isize))
                                                    .read(),
                                                    (((&raw mut gBattlerByTurnOrder).cast::<u8>())
                                                        .wrapping_offset((j) as isize))
                                                    .read(),
                                                    0u8,
                                                )) != 0
                                                {
                                                    SwapTurnOrder(((i) as u8), ((j) as u8));
                                                }
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(3))
                            .write(
                                ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(3))
                                .read()) as i32)
                                    .wrapping_add(1i32)) as u8),
                            );
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(219))
                            .write(0u8);
                    }
                    if __fall || __sw3 == 1i32 {
                        __fall = true;
                        'l14: loop {
                            if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read()) as i32)
                                < 2i32)
                            {
                                break 'l14;
                            }
                            side = ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read();
                            ((&raw mut gActiveBattler).cast::<u8>()).write({
                                let __v4 = ((((&raw mut gSideTimers).cast::<u8>())
                                    .wrapping_offset(((side) as i32) as isize * 12))
                                .wrapping_add(1))
                                .read();
                                ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v4);
                                __v4
                            });
                            if (((((((&raw mut gSideStatuses).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((side) as i32) as isize))
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                if (({
                                    let __p5 = (((&raw mut gSideTimers).cast::<u8>())
                                        .wrapping_offset(((side) as i32) as isize * 12));
                                    let __t6 = ((__p5).read()).wrapping_sub(1);
                                    (__p5).write(__t6);
                                    __t6
                                }) as i32)
                                    == 0i32
                                {
                                    let __p7 = (((&raw mut gSideStatuses).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(((side) as i32) as isize);
                                    (__p7).write((((((__p7).read()) as i32) & (-2i32)) as u16));
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_SideStatusWoreOff).cast::<u8>(),
                                    );
                                    {
                                        ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(1))
                                        .write(2u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(2))
                                        .write(115u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write(0u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(4))
                                        .write(255u8);
                                    }
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                            let __p8 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219);
                            (__p8).write(((__p8).read()).wrapping_add(1));
                            if ((effect) as i32) != 0i32 {
                                break 'l14;
                            }
                        }
                        if ((effect) as i32) == 0i32 {
                            let __p9 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(3);
                            (__p9).write(((__p9).read()).wrapping_add(1));
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .write(0u8);
                        }
                        break 'l7;
                    }
                    if __sw3 == 2i32 {
                        __fall = true;
                        'l15: loop {
                            if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read()) as i32)
                                < 2i32)
                            {
                                break 'l15;
                            }
                            side = ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read();
                            ((&raw mut gActiveBattler).cast::<u8>()).write({
                                let __v10 = ((((&raw mut gSideTimers).cast::<u8>())
                                    .wrapping_offset(((side) as i32) as isize * 12))
                                .wrapping_add(3))
                                .read();
                                ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v10);
                                __v10
                            });
                            if (((((((&raw mut gSideStatuses).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((side) as i32) as isize))
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                if (({
                                    let __p11 = (((&raw mut gSideTimers).cast::<u8>())
                                        .wrapping_offset(((side) as i32) as isize * 12))
                                    .wrapping_add(2);
                                    let __t12 = ((__p11).read()).wrapping_sub(1);
                                    (__p11).write(__t12);
                                    __t12
                                }) as i32)
                                    == 0i32
                                {
                                    let __p13 = (((&raw mut gSideStatuses).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(((side) as i32) as isize);
                                    (__p13).write((((((__p13).read()) as i32) & (-3i32)) as u16));
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_SideStatusWoreOff).cast::<u8>(),
                                    );
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(side);
                                    {
                                        ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(1))
                                        .write(2u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(2))
                                        .write(113u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write(0u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(4))
                                        .write(255u8);
                                    }
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                            let __p14 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219);
                            (__p14).write(((__p14).read()).wrapping_add(1));
                            if ((effect) as i32) != 0i32 {
                                break 'l15;
                            }
                        }
                        if ((effect) as i32) == 0i32 {
                            let __p15 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(3);
                            (__p15).write(((__p15).read()).wrapping_add(1));
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .write(0u8);
                        }
                        break 'l7;
                    }
                    if __sw3 == 3i32 {
                        __fall = true;
                        'l16: loop {
                            if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read()) as i32)
                                < 2i32)
                            {
                                break 'l16;
                            }
                            side = ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read();
                            ((&raw mut gActiveBattler).cast::<u8>()).write({
                                let __v16 = ((((&raw mut gSideTimers).cast::<u8>())
                                    .wrapping_offset(((side) as i32) as isize * 12))
                                .wrapping_add(5))
                                .read();
                                ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v16);
                                __v16
                            });
                            if (((((((&raw mut gSideTimers).cast::<u8>())
                                .wrapping_offset(((side) as i32) as isize * 12))
                            .wrapping_add(4))
                            .read()) as i32)
                                != 0i32)
                                && ((({
                                    let __p17 = (((&raw mut gSideTimers).cast::<u8>())
                                        .wrapping_offset(((side) as i32) as isize * 12))
                                    .wrapping_add(4);
                                    let __t18 = ((__p17).read()).wrapping_sub(1);
                                    (__p17).write(__t18);
                                    __t18
                                }) as i32)
                                    == 0i32)
                            {
                                let __p19 = (((&raw mut gSideStatuses).cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(((side) as i32) as isize);
                                (__p19).write((((((__p19).read()) as i32) & (-257i32)) as u16));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_SideStatusWoreOff).cast::<u8>(),
                                );
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(side);
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(2u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(54u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4))
                                        .write(255u8);
                                }
                                effect = (effect).wrapping_add(1);
                            }
                            let __p20 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219);
                            (__p20).write(((__p20).read()).wrapping_add(1));
                            if ((effect) as i32) != 0i32 {
                                break 'l16;
                            }
                        }
                        if ((effect) as i32) == 0i32 {
                            let __p21 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(3);
                            (__p21).write(((__p21).read()).wrapping_add(1));
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .write(0u8);
                        }
                        break 'l7;
                    }
                    if __sw3 == 4i32 {
                        __fall = true;
                        'l17: loop {
                            if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read()) as i32)
                                < 2i32)
                            {
                                break 'l17;
                            }
                            side = ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read();
                            ((&raw mut gActiveBattler).cast::<u8>()).write({
                                let __v22 = ((((&raw mut gSideTimers).cast::<u8>())
                                    .wrapping_offset(((side) as i32) as isize * 12))
                                .wrapping_add(7))
                                .read();
                                ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v22);
                                __v22
                            });
                            if (((((((&raw mut gSideStatuses).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((side) as i32) as isize))
                            .read()) as i32)
                                & 32i32)
                                != 0
                            {
                                if (({
                                    let __p23 = (((&raw mut gSideTimers).cast::<u8>())
                                        .wrapping_offset(((side) as i32) as isize * 12))
                                    .wrapping_add(6);
                                    let __t24 = ((__p23).read()).wrapping_sub(1);
                                    (__p23).write(__t24);
                                    __t24
                                }) as i32)
                                    == 0i32
                                {
                                    let __p25 = (((&raw mut gSideStatuses).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(((side) as i32) as isize);
                                    (__p25).write((((((__p25).read()) as i32) & (-33i32)) as u16));
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_SafeguardEnds).cast::<u8>(),
                                    );
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                            let __p26 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219);
                            (__p26).write(((__p26).read()).wrapping_add(1));
                            if ((effect) as i32) != 0i32 {
                                break 'l17;
                            }
                        }
                        if ((effect) as i32) == 0i32 {
                            let __p27 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(3);
                            (__p27).write(((__p27).read()).wrapping_add(1));
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .write(0u8);
                        }
                        break 'l7;
                    }
                    if __sw3 == 5i32 {
                        __fall = true;
                        'l18: loop {
                            if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219))
                            .read()) as i32)
                                < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                            {
                                break 'l18;
                            }
                            ((&raw mut gActiveBattler).cast::<u8>()).write(
                                (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(219))
                                    .read()) as i32) as isize,
                                ))
                                .read(),
                            );
                            if (((((((((&raw mut gWishFutureKnock).cast::<u8>())
                                .wrapping_add(32))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                != 0i32)
                                && ((({
                                    let __p28 = ((((&raw mut gWishFutureKnock).cast::<u8>())
                                        .wrapping_add(32))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    );
                                    let __t29 = ((__p28).read()).wrapping_sub(1);
                                    (__p28).write(__t29);
                                    __t29
                                }) as i32)
                                    == 0i32))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32)
                            {
                                ((&raw mut gBattlerTarget).cast::<u8>())
                                    .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                                BattleScriptExecute(
                                    (&raw mut BattleScript_WishComesTrue).cast::<u8>(),
                                );
                                effect = (effect).wrapping_add(1);
                            }
                            let __p30 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(219);
                            (__p30).write(((__p30).read()).wrapping_add(1));
                            if ((effect) as i32) != 0i32 {
                                break 'l18;
                            }
                        }
                        if ((effect) as i32) == 0i32 {
                            let __p31 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(3);
                            (__p31).write(((__p31).read()).wrapping_add(1));
                        }
                        break 'l7;
                    }
                    if __sw3 == 6i32 {
                        __fall = true;
                        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 7i32) != 0
                        {
                            if !((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                & 4i32)
                                != 0)
                            {
                                if (({
                                    let __p32 =
                                        ((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(40);
                                    let __t33 = ((__p32).read()).wrapping_sub(1);
                                    (__p32).write(__t33);
                                    __t33
                                }) as i32)
                                    == 0i32
                                {
                                    let __p34 = (&raw mut gBattleWeather).cast::<u16>();
                                    (__p34).write((((((__p34).read()) as i32) & (-2i32)) as u16));
                                    let __p35 = (&raw mut gBattleWeather).cast::<u16>();
                                    (__p35).write((((((__p35).read()) as i32) & (-3i32)) as u16));
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(2u8);
                                } else {
                                    if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                        & 2i32)
                                        != 0
                                    {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(1u8);
                                    } else {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(0u8);
                                    }
                                }
                            } else {
                                if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                    & 2i32)
                                    != 0
                                {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(1u8);
                                } else {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(0u8);
                                }
                            }
                            BattleScriptExecute(
                                (&raw mut BattleScript_RainContinuesOrEnds).cast::<u8>(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p36 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(3);
                        (__p36).write(((__p36).read()).wrapping_add(1));
                        break 'l7;
                    }
                    if __sw3 == 7i32 {
                        __fall = true;
                        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 24i32)
                            != 0
                        {
                            if (!((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                & 16i32)
                                != 0))
                                && ((({
                                    let __p37 =
                                        ((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(40);
                                    let __t38 = ((__p37).read()).wrapping_sub(1);
                                    (__p37).write(__t38);
                                    __t38
                                }) as i32)
                                    == 0i32)
                            {
                                let __p39 = (&raw mut gBattleWeather).cast::<u16>();
                                (__p39).write((((((__p39).read()) as i32) & (-9i32)) as u16));
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_SandStormHailEnds).cast::<u8>());
                            } else {
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_DamagingWeatherContinues).cast::<u8>(),
                                );
                            }
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                .write(12u8);
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(0u8);
                            BattleScriptExecute(
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p40 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(3);
                        (__p40).write(((__p40).read()).wrapping_add(1));
                        break 'l7;
                    }
                    if __sw3 == 8i32 {
                        __fall = true;
                        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 96i32)
                            != 0
                        {
                            if (!((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                & 64i32)
                                != 0))
                                && ((({
                                    let __p41 =
                                        ((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(40);
                                    let __t42 = ((__p41).read()).wrapping_sub(1);
                                    (__p41).write(__t42);
                                    __t42
                                }) as i32)
                                    == 0i32)
                            {
                                let __p43 = (&raw mut gBattleWeather).cast::<u16>();
                                (__p43).write((((((__p43).read()) as i32) & (-33i32)) as u16));
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_SunlightFaded).cast::<u8>());
                            } else {
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_SunlightContinues).cast::<u8>());
                            }
                            BattleScriptExecute(
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p44 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(3);
                        (__p44).write(((__p44).read()).wrapping_add(1));
                        break 'l7;
                    }
                    if __sw3 == 9i32 {
                        __fall = true;
                        if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 128i32)
                            != 0
                        {
                            if (({
                                let __p45 =
                                    ((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(40);
                                let __t46 = ((__p45).read()).wrapping_sub(1);
                                (__p45).write(__t46);
                                __t46
                            }) as i32)
                                == 0i32
                            {
                                let __p47 = (&raw mut gBattleWeather).cast::<u16>();
                                (__p47).write((((((__p47).read()) as i32) & (-129i32)) as u16));
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_SandStormHailEnds).cast::<u8>());
                            } else {
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_DamagingWeatherContinues).cast::<u8>(),
                                );
                            }
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                .write(13u8);
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(1u8);
                            BattleScriptExecute(
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p48 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(3);
                        (__p48).write(((__p48).read()).wrapping_add(1));
                        break 'l7;
                    }
                    if __sw3 == 10i32 {
                        __fall = true;
                        effect = (effect).wrapping_add(1);
                        break 'l7;
                    }
                }
            }
            if !(((effect) as i32) == 0i32) {
                break 'l5;
            }
        }
        return ((core::mem::transmute::<_, usize>(
            ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) != (BattleTurnPassed as *const () as usize)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattlerEndTurnEffects() -> u8 {
    unsafe {
        let mut effect: u8 = 0u8;
        let __p1 = (&raw mut gHitMarker).cast::<u32>();
        (__p1).write(((__p1).read() | 16777248u32));
        'l1: loop {
            if !((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(1)).read())
                as i32)
                < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                && ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).read()) as i32)
                    <= 19i32))
            {
                break 'l1;
            }
            ((&raw mut gActiveBattler).cast::<u8>()).write({
                let __v2 = (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(1))
                        .read()) as i32) as isize,
                ))
                .read();
                ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v2);
                __v2
            });
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read())
                != 0
            {
                let __p3 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
            } else {
                'l2: {
                    let __sw4 =
                        (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).read()) as i32);
                    if __sw4 == 0i32 {
                        if (((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()
                            & 1024u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32)))
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32),
                                16i32,
                            ));
                            if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                            }
                            let __p5 = (&raw mut gBattleMoveDamage).cast::<i32>();
                            (__p5).write(((__p5).read()).wrapping_mul((-1i32)));
                            BattleScriptExecute(
                                (&raw mut BattleScript_IngrainTurnHeal).cast::<u8>(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p6 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p6).write(((__p6).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 1i32 {
                        if (AbilityBattleEffects(
                            1u8,
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            0u8,
                            0u8,
                            0u16,
                        )) != 0
                        {
                            effect = (effect).wrapping_add(1);
                        }
                        let __p7 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p7).write(((__p7).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 2i32 {
                        if (ItemBattleEffects(
                            1u8,
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            0u8,
                        )) != 0
                        {
                            effect = (effect).wrapping_add(1);
                        }
                        let __p8 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p8).write(((__p8).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 18i32 {
                        if (ItemBattleEffects(
                            1u8,
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            1u8,
                        )) != 0
                        {
                            effect = (effect).wrapping_add(1);
                        }
                        let __p9 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p9).write(((__p9).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 3i32 {
                        if (((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()
                            & 4u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()
                                    & 3u32) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32))
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            ((&raw mut gBattlerTarget).cast::<u8>()).write(
                                ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()
                                    & 3u32) as u8),
                            );
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32),
                                8i32,
                            ));
                            if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                            }
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                .write(((&raw mut gBattlerTarget).cast::<u8>()).read());
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            BattleScriptExecute(
                                (&raw mut BattleScript_LeechSeedTurnDrain).cast::<u8>(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p10 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p10).write(((__p10).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 4i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 8u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32),
                                8i32,
                            ));
                            if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                            }
                            BattleScriptExecute((&raw mut BattleScript_PoisonTurnDmg).cast::<u8>());
                            effect = (effect).wrapping_add(1);
                        }
                        let __p11 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p11).write(((__p11).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 5i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 128u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32),
                                16i32,
                            ));
                            if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                            }
                            if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 3840u32)
                                != 3840u32
                            {
                                let __p12 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p12).write(((__p12).read()).wrapping_add(256u32));
                            }
                            let __p13 = (&raw mut gBattleMoveDamage).cast::<i32>();
                            (__p13).write(
                                (((((__p13).read()) as u32).wrapping_mul(
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 3840u32)
                                        >> 8),
                                )) as i32),
                            );
                            BattleScriptExecute((&raw mut BattleScript_PoisonTurnDmg).cast::<u8>());
                            effect = (effect).wrapping_add(1);
                        }
                        let __p14 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p14).write(((__p14).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 6i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 16u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32),
                                8i32,
                            ));
                            if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                            }
                            BattleScriptExecute((&raw mut BattleScript_BurnTurnDmg).cast::<u8>());
                            effect = (effect).wrapping_add(1);
                        }
                        let __p15 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 7i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 134217728u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 7u32)
                                != 0
                            {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        4i32,
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                BattleScriptExecute(
                                    (&raw mut BattleScript_NightmareTurnDmg).cast::<u8>(),
                                );
                                effect = (effect).wrapping_add(1);
                            } else {
                                let __p16 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p16).write(((__p16).read() & 4160749567u32));
                            }
                        }
                        let __p17 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p17).write(((__p17).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 8i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 268435456u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(crate::c::div_i32(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32),
                                4i32,
                            ));
                            if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                            }
                            BattleScriptExecute((&raw mut BattleScript_CurseTurnDmg).cast::<u8>());
                            effect = (effect).wrapping_add(1);
                        }
                        let __p18 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p18).write(((__p18).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 9i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 57344u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            let __p19 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p19).write(((__p19).read()).wrapping_sub(8192u32));
                            if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 57344u32)
                                != 0
                            {
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(
                                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                            .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                .wrapping_mul(2i32))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(
                                        (((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                .wrapping_mul(2i32))
                                                as isize,
                                        ))
                                        .wrapping_offset(1))
                                        .read(),
                                    );
                                ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                    .write(2u8);
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                    .write(
                                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                            .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                .wrapping_mul(2i32))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                    .write(
                                        (((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                .wrapping_mul(2i32))
                                                as isize,
                                        ))
                                        .wrapping_offset(1))
                                        .read(),
                                    );
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4))
                                    .write(255u8);
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_WrapTurnDmg).cast::<u8>());
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        16i32,
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                            } else {
                                ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                    .write(2u8);
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                    .write(
                                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                            .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                .wrapping_mul(2i32))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                    .write(
                                        (((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                .wrapping_mul(2i32))
                                                as isize,
                                        ))
                                        .wrapping_offset(1))
                                        .read(),
                                    );
                                (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4))
                                    .write(255u8);
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_WrapEnds).cast::<u8>());
                            }
                            BattleScriptExecute(
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read(),
                            );
                            effect = (effect).wrapping_add(1);
                        }
                        let __p20 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p20).write(((__p20).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 10i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 112u32)
                            != 0
                        {
                            {
                                ((&raw mut gBattlerAttacker).cast::<u8>()).write(0u8);
                                'l3: loop {
                                    if !(((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                        as i32)
                                        < ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                            as i32))
                                    {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0)
                                            && (((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gBattlerAttacker).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize
                                                        * 88,
                                                ))
                                            .wrapping_add(32))
                                            .read())
                                                as i32)
                                                != 43i32)
                                        {
                                            let __p21 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gBattlerAttacker).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize
                                                        * 88,
                                                ))
                                            .wrapping_add(76)
                                            .cast::<u32>();
                                            (__p21).write(((__p21).read() & 4294967288u32));
                                            let __p22 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gBattlerAttacker).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize
                                                        * 88,
                                                ))
                                            .wrapping_add(80)
                                            .cast::<u32>();
                                            (__p22).write(((__p22).read() & 4160749567u32));
                                            (((&raw mut gBattleCommunication).cast::<u8>())
                                                .wrapping_offset(5))
                                            .write(1u8);
                                            BattleScriptExecute(
                                                (&raw mut BattleScript_MonWokeUpInUproar)
                                                    .cast::<u8>(),
                                            );
                                            ((&raw mut gActiveBattler).cast::<u8>()).write(
                                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                            );
                                            BtlController_EmitSetMonData(
                                                0u8,
                                                40u8,
                                                0u8,
                                                4u8,
                                                ((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler).cast::<u8>())
                                                            .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                .wrapping_add(76)
                                                .cast::<u32>())
                                                .cast::<u8>(),
                                            );
                                            MarkBattlerForControllerExec(
                                                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                            );
                                            break 'l3;
                                        }
                                    }
                                    let __p23 = (&raw mut gBattlerAttacker).cast::<u8>();
                                    (__p23).write(((__p23).read()).wrapping_add(1));
                                }
                            }
                            if ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                != ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                            {
                                effect = 2u8;
                                break 'l2;
                            } else {
                                ((&raw mut gBattlerAttacker).cast::<u8>())
                                    .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                                let __p24 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p24).write(((__p24).read()).wrapping_sub(16u32));
                                if (WasUnableToUseMove(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                )) != 0
                                {
                                    CancelMultiTurnMoves(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    );
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(1u8);
                                } else {
                                    if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 112u32)
                                        != 0
                                    {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(0u8);
                                        let __p25 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(80)
                                        .cast::<u32>();
                                        (__p25).write(((__p25).read() | 4096u32));
                                    } else {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(1u8);
                                        CancelMultiTurnMoves(
                                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                        );
                                    }
                                }
                                BattleScriptExecute(
                                    (&raw mut BattleScript_PrintUproarOverTurns).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                        }
                        if ((effect) as i32) != 2i32 {
                            let __p26 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                            (__p26).write(((__p26).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw4 == 11i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 3072u32)
                            != 0
                        {
                            let __p27 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p27).write(((__p27).read()).wrapping_sub(1024u32));
                            if (WasUnableToUseMove(((&raw mut gActiveBattler).cast::<u8>()).read()))
                                != 0
                            {
                                CancelMultiTurnMoves(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                );
                            } else {
                                if (!((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 3072u32)
                                    != 0))
                                    && ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 4096u32)
                                        != 0)
                                {
                                    let __p28 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(80)
                                    .cast::<u32>();
                                    (__p28).write(((__p28).read() & 4294963199u32));
                                    if !((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 7u32)
                                        != 0)
                                    {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write(71u8);
                                        SetMoveEffect(1u8, 0u8);
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0
                                        {
                                            BattleScriptExecute(
                                                (&raw mut BattleScript_ThrashConfuses).cast::<u8>(),
                                            );
                                        }
                                        effect = (effect).wrapping_add(1);
                                    }
                                }
                            }
                        }
                        let __p29 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p29).write(((__p29).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 12i32 {
                        if ((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(11),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            != 0i32
                        {
                            let mut i: i32 = 0i32;
                            {
                                i = 0i32;
                                'l5: loop {
                                    if !(i < 4i32) {
                                        break 'l5;
                                    }
                                    'l6: {
                                        if ((((((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            == ((((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize
                                                        * 88,
                                                ))
                                            .wrapping_add(12))
                                            .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                        {
                                            break 'l5;
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            if i == 4i32 {
                                ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(4)
                                .cast::<u16>())
                                .write(0u16);
                                crate::c::bf_write(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(11),
                                    0,
                                    4,
                                    (0u8) as i32,
                                );
                            } else {
                                if (({
                                    let __t30 = (crate::c::bf_read(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(11),
                                        0,
                                        4,
                                        false,
                                    ) as u8)
                                        .wrapping_sub(1);
                                    crate::c::bf_write(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(11),
                                        0,
                                        4,
                                        (__t30) as i32,
                                    );
                                    __t30
                                }) as i32)
                                    == 0i32
                                {
                                    ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                    .write(0u16);
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_DisabledNoMore).cast::<u8>(),
                                    );
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                        }
                        let __p31 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p31).write(((__p31).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 13i32 {
                        if ((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(14),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            != 0i32
                        {
                            if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(12))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                != ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read()) as i32)
                            {
                                ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .write(0u16);
                                crate::c::bf_write(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(14),
                                    0,
                                    4,
                                    (0u8) as i32,
                                );
                            } else {
                                if ((({
                                    let __t32 = (crate::c::bf_read(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(14),
                                        0,
                                        4,
                                        false,
                                    ) as u8)
                                        .wrapping_sub(1);
                                    crate::c::bf_write(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(14),
                                        0,
                                        4,
                                        (__t32) as i32,
                                    );
                                    __t32
                                }) as i32)
                                    == 0i32)
                                    || (((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(36))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(12))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32)
                                        == 0i32)
                                {
                                    ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .write(0u16);
                                    crate::c::bf_write(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(14),
                                        0,
                                        4,
                                        (0u8) as i32,
                                    );
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_EncoredNoMore).cast::<u8>(),
                                    );
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                        }
                        let __p33 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p33).write(((__p33).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 14i32 {
                        if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()
                            & 24u32)
                            != 0
                        {
                            let __p34 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p34).write(((__p34).read()).wrapping_sub(8u32));
                        }
                        let __p35 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p35).write(((__p35).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 15i32 {
                        if ((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(18),
                            0,
                            4,
                            false,
                        ) as u8)
                            != 0)
                            && ((({
                                let __t36 = (crate::c::bf_read(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(18),
                                    0,
                                    4,
                                    false,
                                ) as u8)
                                    .wrapping_sub(1);
                                crate::c::bf_write(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(18),
                                    0,
                                    4,
                                    (__t36) as i32,
                                );
                                __t36
                            }) as i32)
                                == 0i32)
                        {
                            let __p37 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p37).write(((__p37).read() & 4294966783u32));
                        }
                        let __p38 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p38).write(((__p38).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 16i32 {
                        if (crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(19),
                            0,
                            4,
                            false,
                        ) as u8)
                            != 0
                        {
                            crate::c::bf_write(
                                (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(19),
                                0,
                                4,
                                ((crate::c::bf_read(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(19),
                                    0,
                                    4,
                                    false,
                                ) as u8)
                                    .wrapping_sub(1)) as i32,
                            );
                        }
                        let __p39 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p39).write(((__p39).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 17i32 {
                        if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()
                            & 6144u32)
                            != 0
                        {
                            let __p40 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p40).write(((__p40).read()).wrapping_sub(2048u32));
                            if ((((!((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()
                                & 6144u32)
                                != 0))
                                && (!((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 255u32)
                                    != 0)))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(32))
                                .read()) as i32)
                                    != 72i32))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(32))
                                .read()) as i32)
                                    != 15i32))
                                && (!((UproarWakeUpCheck(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                )) != 0))
                            {
                                CancelMultiTurnMoves(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                );
                                let __p41 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p41).write(
                                    ((__p41).read()
                                        | (((((Random()) as i32) & 3i32).wrapping_add(2i32) << 0)
                                            as u32)),
                                );
                                BtlController_EmitSetMonData(
                                    0u8,
                                    40u8,
                                    0u8,
                                    4u8,
                                    ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .cast::<u8>(),
                                );
                                MarkBattlerForControllerExec(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                );
                                ((&raw mut gEffectBattler).cast::<u8>())
                                    .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                                BattleScriptExecute(
                                    (&raw mut BattleScript_YawnMakesAsleep).cast::<u8>(),
                                );
                                effect = (effect).wrapping_add(1);
                            }
                        }
                        let __p42 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read());
                        (__p42).write(((__p42).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 19i32 {
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).write(0u8);
                        let __p43 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(1);
                        (__p43).write(((__p43).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                if ((effect) as i32) != 0i32 {
                    return effect;
                }
            }
        }
        let __p44 = (&raw mut gHitMarker).cast::<u32>();
        (__p44).write(((__p44).read() & 4278190047u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleWishPerishSongOnTurnEnd() -> u8 {
    unsafe {
        let __p1 = (&raw mut gHitMarker).cast::<u32>();
        (__p1).write(((__p1).read() | 16777248u32));
        'l1: {
            let __sw2 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(416))
                .read()) as i32);
            let mut __fall = false;
            if __sw2 == 0i32 {
                __fall = true;
                'l2: loop {
                    if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(417))
                    .read()) as i32)
                        < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                    {
                        break 'l2;
                    }
                    ((&raw mut gActiveBattler).cast::<u8>()).write(
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417))
                            .read(),
                    );
                    if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0
                    {
                        let __p3 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        continue 'l2;
                    }
                    let __p4 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    if ((((((((&raw mut gWishFutureKnock).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        != 0i32)
                        && ((({
                            let __p5 = (((&raw mut gWishFutureKnock).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            let __t6 = ((__p5).read()).wrapping_sub(1);
                            (__p5).write(__t6);
                            __t6
                        }) as i32)
                            == 0i32))
                        && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(40)
                        .cast::<u16>())
                        .read()) as i32)
                            != 0i32)
                    {
                        if (((((((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(24))
                            .cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            == 248i32
                        {
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(0u8);
                        } else {
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(1u8);
                        }
                        {
                            ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                .write(2u8);
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2)).write(
                                (((((((((&raw mut gWishFutureKnock).cast::<u8>())
                                    .wrapping_add(24))
                                .cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    & 255i32) as u8),
                            );
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3)).write(
                                ((((((((((&raw mut gWishFutureKnock).cast::<u8>())
                                    .wrapping_add(24))
                                .cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    & 65280i32)
                                    >> 8) as u8),
                            );
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4))
                                .write(255u8);
                        }
                        ((&raw mut gBattlerTarget).cast::<u8>())
                            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
                            (((((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(4))
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read(),
                        );
                        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                            (((((&raw mut gWishFutureKnock).cast::<u8>()).wrapping_add(8))
                                .cast::<i32>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read(),
                        );
                        ((((&raw mut gSpecialStatuses).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize
                                * 20,
                        ))
                        .wrapping_add(4)
                        .cast::<i32>())
                        .write(65535i32);
                        BattleScriptExecute(
                            (&raw mut BattleScript_MonTookFutureAttack).cast::<u8>(),
                        );
                        if (((((((&raw mut gWishFutureKnock).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read()) as i32)
                            == 0i32)
                            && (((((((&raw mut gWishFutureKnock).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        ^ 2i32) as isize,
                                ))
                            .read()) as i32)
                                == 0i32)
                        {
                            let __p7 = (((&raw mut gSideStatuses).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((GetBattlerPosition(
                                        ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                                    )) as i32)
                                        & 1i32) as isize,
                                );
                            (__p7).write((((((__p7).read()) as i32) & (-65i32)) as u16));
                        }
                        return 1u8;
                    }
                }
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(416))
                    .write(1u8);
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417))
                    .write(0u8);
            }
            if __fall || __sw2 == 1i32 {
                __fall = true;
                'l3: loop {
                    if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(417))
                    .read()) as i32)
                        < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                    {
                        break 'l3;
                    }
                    ((&raw mut gActiveBattler).cast::<u8>()).write({
                        let __v8 = (((&raw mut gBattlerByTurnOrder).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(417))
                            .read()) as i32) as isize,
                        ))
                        .read();
                        ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v8);
                        __v8
                    });
                    if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read())
                        != 0
                    {
                        let __p9 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                        continue 'l3;
                    }
                    let __p10 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()
                        & 32u32)
                        != 0
                    {
                        {
                            ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                .write(1u8);
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                .write(1u8);
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                .write(1u8);
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4)).write(
                                (crate::c::bf_read(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(15),
                                    0,
                                    4,
                                    false,
                                ) as u8),
                            );
                            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(5))
                                .write(255u8);
                        }
                        if ((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(15),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            == 0i32
                        {
                            let __p11 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p11).write(((__p11).read() & 4294967263u32));
                            ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32),
                            );
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_PerishSongTakesLife).cast::<u8>());
                        } else {
                            crate::c::bf_write(
                                (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(15),
                                0,
                                4,
                                ((crate::c::bf_read(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(15),
                                    0,
                                    4,
                                    false,
                                ) as u8)
                                    .wrapping_sub(1)) as i32,
                            );
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                (&raw mut BattleScript_PerishSongCountGoesDown).cast::<u8>(),
                            );
                        }
                        BattleScriptExecute(
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read(),
                        );
                        return 1u8;
                    }
                }
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(416))
                    .write(2u8);
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(417))
                    .write(0u8);
            }
            if __fall || __sw2 == 2i32 {
                __fall = true;
                if ((((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0)
                    && (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(218))
                        .read()) as i32)
                        == 2i32))
                    && ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_add(40)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32))
                    && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(88))
                        .wrapping_add(40)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                {
                    let mut i: i32 = 0i32;
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 2i32) {
                                break 'l4;
                            }
                            'l5: {
                                CancelMultiTurnMoves(((i) as u8));
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                        .write((&raw mut BattleScript_ArenaDoJudgment).cast::<u8>());
                    BattleScriptExecute((&raw mut BattleScript_ArenaDoJudgment).cast::<u8>());
                    let __p12 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(416);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                    return 1u8;
                }
                break 'l1;
            }
        }
        let __p13 = (&raw mut gHitMarker).cast::<u32>();
        (__p13).write(((__p13).read() & 4278190047u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleFaintedMonActions() -> u8 {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0 {
            return 0u8;
        }
        'l1: loop {
            'l2: {
                let mut i: i32 = 0i32;
                'l3: {
                    let __sw1 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(77))
                    .read()) as i32);
                    let mut __fall = false;
                    if __sw1 == 0i32 {
                        __fall = true;
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(78))
                            .write(0u8);
                        let __p2 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                        {
                            i = 0i32;
                            'l4: loop {
                                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                                {
                                    break 'l4;
                                }
                                'l5: {
                                    if ((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                        as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset((i) as isize))
                                        .read())
                                        != 0)
                                        && (!((HasNoMonsToSwitch(((i) as u8), 6u8, 6u8)) != 0))
                                    {
                                        let __p3 = (&raw mut gAbsentBattlerFlags).cast::<u8>();
                                        (__p3).write(
                                            (((((__p3).read()) as u32)
                                                & !(((((&raw mut gBitTable).cast::<u32>())
                                                    .cast::<u32>())
                                                .wrapping_offset((i) as isize))
                                                .read()))
                                                as u8),
                                        );
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    if __fall || __sw1 == 1i32 {
                        __fall = true;
                        'l6: loop {
                            'l7: {
                                ((&raw mut gBattlerFainted).cast::<u8>()).write({
                                    let __v4 = ((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(78))
                                    .read();
                                    ((&raw mut gBattlerTarget).cast::<u8>()).write(__v4);
                                    __v4
                                });
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(78))
                                    .read()) as i32) as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    == 0i32)
                                    && (!((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(223))
                                    .read()) as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gBattleStruct)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(78))
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                        != 0)))
                                    && (!((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                        as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                                    .read())
                                                .wrapping_add(78))
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                        != 0))
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_GiveExp).cast::<u8>(),
                                    );
                                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(77))
                                    .write(2u8);
                                    return 1u8;
                                }
                            }
                            if !((({
                                let __p5 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(78);
                                let __t6 = ((__p5).read()).wrapping_add(1);
                                (__p5).write(__t6);
                                __t6
                            }) as i32)
                                != ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                            {
                                break 'l6;
                            }
                        }
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77))
                            .write(3u8);
                        break 'l3;
                    }
                    if __sw1 == 2i32 {
                        __fall = true;
                        OpponentSwitchInResetSentPokesToOpponentValue(
                            ((&raw mut gBattlerFainted).cast::<u8>()).read(),
                        );
                        if (({
                            let __p7 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(78);
                            let __t8 = ((__p7).read()).wrapping_add(1);
                            (__p7).write(__t8);
                            __t8
                        }) as i32)
                            == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                        {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(77))
                            .write(3u8);
                        } else {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(77))
                            .write(1u8);
                        }
                        break 'l3;
                    }
                    if __sw1 == 3i32 {
                        __fall = true;
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(78))
                            .write(0u8);
                        let __p9 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                    if __fall || __sw1 == 4i32 {
                        __fall = true;
                        'l8: loop {
                            'l9: {
                                ((&raw mut gBattlerFainted).cast::<u8>()).write({
                                    let __v10 = ((((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(78))
                                    .read();
                                    ((&raw mut gBattlerTarget).cast::<u8>()).write(__v10);
                                    __v10
                                });
                                if (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(78))
                                    .read()) as i32) as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    == 0i32)
                                    && (!((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                        as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                                    .read())
                                                .wrapping_add(78))
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                        != 0))
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_HandleFaintedMon).cast::<u8>(),
                                    );
                                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(77))
                                    .write(5u8);
                                    return 1u8;
                                }
                            }
                            if !((({
                                let __p11 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(78);
                                let __t12 = ((__p11).read()).wrapping_add(1);
                                (__p11).write(__t12);
                                __t12
                            }) as i32)
                                != ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                            {
                                break 'l8;
                            }
                        }
                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77))
                            .write(6u8);
                        break 'l3;
                    }
                    if __sw1 == 5i32 {
                        __fall = true;
                        if (({
                            let __p13 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(78);
                            let __t14 = ((__p13).read()).wrapping_add(1);
                            (__p13).write(__t14);
                            __t14
                        }) as i32)
                            == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                        {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(77))
                            .write(6u8);
                        } else {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(77))
                            .write(4u8);
                        }
                        break 'l3;
                    }
                    if __sw1 == 6i32 {
                        __fall = true;
                        if ((((AbilityBattleEffects(9u8, 0u8, 0u8, 0u8, 0u16)) != 0)
                            || ((AbilityBattleEffects(11u8, 0u8, 0u8, 0u8, 0u16)) != 0))
                            || ((ItemBattleEffects(1u8, 0u8, 1u8)) != 0))
                            || ((AbilityBattleEffects(6u8, 0u8, 0u8, 0u8, 0u16)) != 0)
                        {
                            return 1u8;
                        }
                        let __p15 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 7i32 {
                        __fall = true;
                        break 'l3;
                    }
                }
            }
            if !(((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(77)).read())
                as i32)
                != 7i32)
            {
                break 'l1;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryClearRageStatuses() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>())
                    .read()
                        & 8388608u32)
                        != 0)
                        && (((((((&raw mut gChosenMoveByBattler).cast::<u16>()).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 99i32)
                    {
                        let __p1 = (((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p1).write(((__p1).read() & 4286578687u32));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AtkCanceler_UnableToUseMove() -> u8 {
    unsafe {
        let mut effect: u8 = 0u8;
        let mut bideDmg: *mut i32 = ((&raw mut gBattleScripting).cast::<u8>())
            .wrapping_add(4)
            .cast::<i32>();
        'l1: loop {
            'l2: {
                'l3: {
                    let __sw1 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(419))
                    .read()) as i32);
                    if __sw1 == 0i32 {
                        let __p2 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p2).write(((__p2).read() & 4261412863u32));
                        let __p3 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            );
                        (__p3).write(((__p3).read() & 4294950911u32));
                        let __p4 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 1i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 7u32)
                            != 0
                        {
                            if (UproarWakeUpCheck(
                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                            )) != 0
                            {
                                let __p5 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p5).write(((__p5).read() & 4294967288u32));
                                let __p6 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p6).write(((__p6).read() & 4160749567u32));
                                BattleScriptPushCursor();
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(1u8);
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_MoveUsedWokeUp).cast::<u8>());
                                effect = 2u8;
                            } else {
                                let mut toSub: u8 = 0u8;
                                if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == 48i32
                                {
                                    toSub = 2u8;
                                } else {
                                    toSub = 1u8;
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    < ((toSub) as u32)
                                {
                                    let __p7 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(76)
                                    .cast::<u32>();
                                    (__p7).write(((__p7).read() & 4294967288u32));
                                } else {
                                    let __p8 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(76)
                                    .cast::<u32>();
                                    (__p8).write(((__p8).read()).wrapping_sub(((toSub) as u32)));
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0
                                {
                                    if (((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32)
                                        != 173i32)
                                        && (((((&raw mut gCurrentMove).cast::<u16>()).read())
                                            as i32)
                                            != 214i32)
                                    {
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_MoveUsedIsAsleep)
                                                    .cast::<u8>(),
                                            );
                                        let __p9 = (&raw mut gHitMarker).cast::<u32>();
                                        (__p9).write(((__p9).read() | 524288u32));
                                        effect = 2u8;
                                    }
                                } else {
                                    let __p10 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(80)
                                    .cast::<u32>();
                                    (__p10).write(((__p10).read() & 4160749567u32));
                                    BattleScriptPushCursor();
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(0u8);
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                        .write((&raw mut BattleScript_MoveUsedWokeUp).cast::<u8>());
                                    effect = 2u8;
                                }
                            }
                        }
                        let __p11 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p11).write(((__p11).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 2i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 32u32)
                            != 0
                        {
                            if (crate::c::rem_i32(((Random()) as i32), 5i32)) != 0 {
                                if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 12,
                                ))
                                .read()) as i32)
                                    != 125i32
                                {
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                        (&raw mut BattleScript_MoveUsedIsFrozen).cast::<u8>(),
                                    );
                                    let __p12 = (&raw mut gHitMarker).cast::<u32>();
                                    (__p12).write(((__p12).read() | 512u32));
                                } else {
                                    let __p13 = (((&raw mut gBattleStruct).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(419);
                                    (__p13).write(((__p13).read()).wrapping_add(1));
                                    break 'l3;
                                }
                            } else {
                                let __p14 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p14).write(((__p14).read() & 4294967263u32));
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_MoveUsedUnfroze).cast::<u8>());
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(0u8);
                            }
                            effect = 2u8;
                        }
                        let __p15 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 3i32 {
                        if (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(32))
                        .read()) as i32)
                            == 54i32)
                            && ((crate::c::bf_read(
                                (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 28,
                                ))
                                .wrapping_add(24),
                                0,
                                1,
                                false,
                            ) as u8)
                                != 0)
                        {
                            CancelMultiTurnMoves(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            let __p16 = (&raw mut gHitMarker).cast::<u32>();
                            (__p16).write(((__p16).read() | 524288u32));
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                .write(0u8);
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedLoafingAround).cast::<u8>());
                            let __p17 = (&raw mut gMoveResultFlags).cast::<u8>();
                            (__p17).write((((((__p17).read()) as i32) | 1i32) as u8));
                            effect = 1u8;
                        }
                        let __p18 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p18).write(((__p18).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 4i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 4194304u32)
                            != 0
                        {
                            let __p19 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p19).write(((__p19).read() & 4290772991u32));
                            ((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 28,
                            ))
                            .wrapping_add(25))
                            .write(0u8);
                            CancelMultiTurnMoves(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedMustRecharge).cast::<u8>());
                            let __p20 = (&raw mut gHitMarker).cast::<u32>();
                            (__p20).write(((__p20).read() | 524288u32));
                            effect = 1u8;
                        }
                        let __p21 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p21).write(((__p21).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 5i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 8u32)
                            != 0
                        {
                            let __p22 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p22).write(((__p22).read() & 4294967287u32));
                            crate::c::bf_write(
                                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 16,
                                ))
                                .wrapping_add(2),
                                2,
                                1,
                                (1u32) as i32,
                            );
                            CancelMultiTurnMoves(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedFlinched).cast::<u8>());
                            let __p23 = (&raw mut gHitMarker).cast::<u32>();
                            (__p23).write(((__p23).read() | 524288u32));
                            effect = 1u8;
                        }
                        let __p24 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p24).write(((__p24).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 6i32 {
                        if (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32)
                            == ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32))
                            && (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 28,
                            ))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            crate::c::bf_write(
                                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 16,
                                ))
                                .wrapping_add(1),
                                7,
                                1,
                                (1u32) as i32,
                            );
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            CancelMultiTurnMoves(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedIsDisabled).cast::<u8>());
                            let __p25 = (&raw mut gHitMarker).cast::<u32>();
                            (__p25).write(((__p25).read() | 524288u32));
                            effect = 1u8;
                        }
                        let __p26 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p26).write(((__p26).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 7i32 {
                        if ((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 28,
                            ))
                            .wrapping_add(19),
                            0,
                            4,
                            false,
                        ) as u8)
                            != 0)
                            && (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize
                                    * 12,
                            ))
                            .wrapping_add(1))
                            .read()) as i32)
                                == 0i32)
                        {
                            crate::c::bf_write(
                                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 16,
                                ))
                                .wrapping_add(2),
                                0,
                                1,
                                (1u32) as i32,
                            );
                            CancelMultiTurnMoves(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedIsTaunted).cast::<u8>());
                            let __p27 = (&raw mut gHitMarker).cast::<u32>();
                            (__p27).write(((__p27).read() | 524288u32));
                            effect = 1u8;
                        }
                        let __p28 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p28).write(((__p28).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 8i32 {
                        if (GetImprisonedMovesCount(
                            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                            ((&raw mut gCurrentMove).cast::<u16>()).read(),
                        )) != 0
                        {
                            crate::c::bf_write(
                                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 16,
                                ))
                                .wrapping_add(1),
                                5,
                                1,
                                (1u32) as i32,
                            );
                            CancelMultiTurnMoves(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedIsImprisoned).cast::<u8>());
                            let __p29 = (&raw mut gHitMarker).cast::<u32>();
                            (__p29).write(((__p29).read() | 524288u32));
                            effect = 1u8;
                        }
                        let __p30 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p30).write(((__p30).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 9i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 7u32)
                            != 0
                        {
                            let __p31 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p31).write(((__p31).read()).wrapping_sub(1u32));
                            if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 7u32)
                                != 0
                            {
                                if (((Random()) as i32) & 1i32) != 0 {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(0u8);
                                    BattleScriptPushCursor();
                                } else {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(1u8);
                                    ((&raw mut gBattlerTarget).cast::<u8>())
                                        .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        CalculateBaseDamage(
                                            ((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ),
                                            ((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ),
                                            1u32,
                                            0u16,
                                            40u16,
                                            0u8,
                                            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                        ),
                                    );
                                    crate::c::bf_write(
                                        (((&raw mut gProtectStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 16,
                                            ))
                                        .wrapping_add(1),
                                        0,
                                        1,
                                        (1u32) as i32,
                                    );
                                    let __p32 = (&raw mut gHitMarker).cast::<u32>();
                                    (__p32).write(((__p32).read() | 524288u32));
                                }
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_MoveUsedIsConfused).cast::<u8>());
                            } else {
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_MoveUsedIsConfusedNoMore).cast::<u8>(),
                                );
                            }
                            effect = 1u8;
                        }
                        let __p33 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p33).write(((__p33).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 10i32 {
                        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 64u32)
                            != 0)
                            && (crate::c::rem_i32(((Random()) as i32), 4i32) == 0i32)
                        {
                            crate::c::bf_write(
                                (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 16,
                                ))
                                .wrapping_add(0),
                                7,
                                1,
                                (1u32) as i32,
                            );
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedIsParalyzed).cast::<u8>());
                            let __p34 = (&raw mut gHitMarker).cast::<u32>();
                            (__p34).write(((__p34).read() | 524288u32));
                            effect = 1u8;
                        }
                        let __p35 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p35).write(((__p35).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 11i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 983040u32)
                            != 0
                        {
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).write(
                                ((CountTrailingZeroBits(
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 983040u32)
                                        >> 16),
                                )) as u8),
                            );
                            if (((Random()) as i32) & 1i32) != 0 {
                                BattleScriptPushCursor();
                            } else {
                                BattleScriptPush(
                                    (&raw mut BattleScript_MoveUsedIsInLoveCantAttack).cast::<u8>(),
                                );
                                let __p36 = (&raw mut gHitMarker).cast::<u32>();
                                (__p36).write(((__p36).read() | 524288u32));
                                crate::c::bf_write(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    6,
                                    1,
                                    (1u32) as i32,
                                );
                                CancelMultiTurnMoves(
                                    ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                );
                            }
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_MoveUsedIsInLove).cast::<u8>());
                            effect = 1u8;
                        }
                        let __p37 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p37).write(((__p37).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 12i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 768u32)
                            != 0
                        {
                            let __p38 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p38).write(((__p38).read()).wrapping_sub(256u32));
                            if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 768u32)
                                != 0
                            {
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_BideStoringEnergy).cast::<u8>());
                            } else {
                                if (((((&raw mut gBideDmg).cast::<i32>()).cast::<i32>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read())
                                    != 0
                                {
                                    ((&raw mut gCurrentMove).cast::<u16>()).write(117u16);
                                    (bideDmg).write(
                                        (((((&raw mut gBideDmg).cast::<i32>()).cast::<i32>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                        .wrapping_mul(2i32),
                                    );
                                    ((&raw mut gBattlerTarget).cast::<u8>()).write(
                                        (((&raw mut gBideTarget).cast::<u8>()).wrapping_offset(
                                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                    if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                        as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                        != 0
                                    {
                                        ((&raw mut gBattlerTarget).cast::<u8>())
                                            .write(GetMoveTarget(117u16, 1u8));
                                    }
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                        .write((&raw mut BattleScript_BideAttack).cast::<u8>());
                                } else {
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                        (&raw mut BattleScript_BideNoEnergyToAttack).cast::<u8>(),
                                    );
                                }
                            }
                            effect = 1u8;
                        }
                        let __p39 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p39).write(((__p39).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 13i32 {
                        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 32u32)
                            != 0
                        {
                            if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize
                                    * 12,
                            ))
                            .read()) as i32)
                                == 125i32
                            {
                                let __p40 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p40).write(((__p40).read() & 4294967263u32));
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_MoveUsedUnfroze).cast::<u8>());
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(1u8);
                            }
                            effect = 2u8;
                        }
                        let __p41 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419);
                        (__p41).write(((__p41).read()).wrapping_add(1));
                        break 'l3;
                    }
                    if __sw1 == 14i32 {
                        break 'l3;
                    }
                }
            }
            if !((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(419))
                .read()) as i32)
                != 14i32)
                && (((effect) as i32) == 0i32))
            {
                break 'l1;
            }
        }
        if ((effect) as i32) == 2i32 {
            ((&raw mut gActiveBattler).cast::<u8>())
                .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
            BtlController_EmitSetMonData(
                0u8,
                40u8,
                0u8,
                4u8,
                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(76)
                .cast::<u32>())
                .cast::<u8>(),
            );
            MarkBattlerForControllerExec(((&raw mut gActiveBattler).cast::<u8>()).read());
        }
        return effect;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasNoMonsToSwitch(
    battler: u8,
    partyIdBattlerOn1: u8,
    partyIdBattlerOn2: u8,
) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut partyIdBattlerOn1 = partyIdBattlerOn1;
        let mut partyIdBattlerOn2 = partyIdBattlerOn2;
        let mut playerId: u8 = 0u8;
        let mut flankId: u8 = 0u8;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
            return 0u8;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0 {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                party = (&raw mut gPlayerParty).cast::<u8>();
            } else {
                party = (&raw mut gEnemyParty).cast::<u8>();
            }
            playerId = ((crate::c::div_i32((((battler) as i32) & 2i32), 2i32)) as u8);
            {
                i = ((playerId) as i32).wrapping_mul(crate::c::div_i32(6i32, 2i32));
                'l1: loop {
                    if !(i
                        < (((playerId) as i32).wrapping_mul(crate::c::div_i32(6i32, 2i32)))
                            .wrapping_add(crate::c::div_i32(6i32, 2i32)))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if ((GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32)
                            != 0u32)
                            && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                                != 0u32))
                            && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                                != 412u32)
                        {
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return ((i
                == (((playerId) as i32).wrapping_mul(crate::c::div_i32(6i32, 2i32)))
                    .wrapping_add(crate::c::div_i32(6i32, 2i32))) as u8);
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8388608u32) != 0 {
                    if ((GetBattlerSide(battler)) as i32) == 0i32 {
                        party = (&raw mut gPlayerParty).cast::<u8>();
                        flankId = ((GetBattlerMultiplayerId(((battler) as u16))) as u8);
                        playerId = ((GetLinkTrainerFlankId(flankId)) as u8);
                    } else {
                        party = (&raw mut gEnemyParty).cast::<u8>();
                        if ((battler) as i32) == 1i32 {
                            playerId = 0u8;
                        } else {
                            playerId = 1u8;
                        }
                    }
                } else {
                    flankId = ((GetBattlerMultiplayerId(((battler) as u16))) as u8);
                    if ((GetBattlerSide(battler)) as i32) == 0i32 {
                        party = (&raw mut gPlayerParty).cast::<u8>();
                    } else {
                        party = (&raw mut gEnemyParty).cast::<u8>();
                    }
                    playerId = ((GetLinkTrainerFlankId(flankId)) as u8);
                }
                {
                    i = ((playerId) as i32).wrapping_mul(crate::c::div_i32(6i32, 2i32));
                    'l3: loop {
                        if !(i
                            < (((playerId) as i32).wrapping_mul(crate::c::div_i32(6i32, 2i32)))
                                .wrapping_add(crate::c::div_i32(6i32, 2i32)))
                        {
                            break 'l3;
                        }
                        'l4: {
                            if ((GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32)
                                != 0u32)
                                && (GetMonData2(
                                    (party).wrapping_offset((i) as isize * 100),
                                    65i32,
                                ) != 0u32))
                                && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                                    != 412u32)
                            {
                                break 'l3;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return ((i
                    == (((playerId) as i32).wrapping_mul(crate::c::div_i32(6i32, 2i32)))
                        .wrapping_add(crate::c::div_i32(6i32, 2i32)))
                    as u8);
            } else {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0)
                    && (((GetBattlerSide(battler)) as i32) == 1i32)
                {
                    party = (&raw mut gEnemyParty).cast::<u8>();
                    if ((battler) as i32) == 1i32 {
                        playerId = 0u8;
                    } else {
                        playerId = ((crate::c::div_i32(6i32, 2i32)) as u8);
                    }
                    {
                        i = ((playerId) as i32);
                        'l5: loop {
                            if !(i
                                < ((playerId) as i32).wrapping_add(crate::c::div_i32(6i32, 2i32)))
                            {
                                break 'l5;
                            }
                            'l6: {
                                if ((GetMonData2(
                                    (party).wrapping_offset((i) as isize * 100),
                                    57i32,
                                ) != 0u32)
                                    && (GetMonData2(
                                        (party).wrapping_offset((i) as isize * 100),
                                        65i32,
                                    ) != 0u32))
                                    && (GetMonData2(
                                        (party).wrapping_offset((i) as isize * 100),
                                        65i32,
                                    ) != 412u32)
                                {
                                    break 'l5;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    return ((i == ((playerId) as i32).wrapping_add(3i32)) as u8);
                } else {
                    if ((GetBattlerSide(battler)) as i32) == 1i32 {
                        flankId = GetBattlerAtPosition(1u8);
                        playerId = GetBattlerAtPosition(3u8);
                        party = (&raw mut gEnemyParty).cast::<u8>();
                    } else {
                        flankId = GetBattlerAtPosition(0u8);
                        playerId = GetBattlerAtPosition(2u8);
                        party = (&raw mut gPlayerParty).cast::<u8>();
                    }
                    if ((partyIdBattlerOn1) as i32) == 6i32 {
                        partyIdBattlerOn1 = ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(((flankId) as i32) as isize))
                        .read()) as u8);
                    }
                    if ((partyIdBattlerOn2) as i32) == 6i32 {
                        partyIdBattlerOn2 = ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(((playerId) as i32) as isize))
                        .read()) as u8);
                    }
                    {
                        i = 0i32;
                        'l7: loop {
                            if !(i < 6i32) {
                                break 'l7;
                            }
                            'l8: {
                                if ((((((GetMonData2(
                                    (party).wrapping_offset((i) as isize * 100),
                                    57i32,
                                ) != 0u32)
                                    && (GetMonData2(
                                        (party).wrapping_offset((i) as isize * 100),
                                        65i32,
                                    ) != 0u32))
                                    && (GetMonData2(
                                        (party).wrapping_offset((i) as isize * 100),
                                        65i32,
                                    ) != 412u32))
                                    && (i != ((partyIdBattlerOn1) as i32)))
                                    && (i != ((partyIdBattlerOn2) as i32)))
                                    && (i
                                        != ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(92))
                                        .cast::<u8>())
                                        .wrapping_offset(((flankId) as i32) as isize))
                                        .read())
                                            as i32)))
                                    && (i
                                        != ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(92))
                                        .cast::<u8>())
                                        .wrapping_offset(((playerId) as i32) as isize))
                                        .read())
                                            as i32))
                                {
                                    break 'l7;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    return ((i == 6i32) as u8);
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CastformDataTypeChange(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut formChange: u8 = 0u8;
        if ((((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .cast::<u16>())
        .read()) as i32)
            != 385i32)
            || (((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(32))
            .read()) as i32)
                != 59i32))
            || (((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(40)
            .cast::<u16>())
            .read()) as i32)
                == 0i32)
        {
            return 0u8;
        }
        if (!((!((AbilityBattleEffects(19u8, 0u8, 13u8, 0u8, 0u16)) != 0))
            && (!((AbilityBattleEffects(19u8, 0u8, 77u8, 0u8, 0u16)) != 0))))
            && (!(((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 0i32)
                || (((((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32)))
        {
            {
                (((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .write(0u8);
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(0u8);
            }
            return 1u8;
        }
        if !((!((AbilityBattleEffects(19u8, 0u8, 13u8, 0u8, 0u16)) != 0))
            && (!((AbilityBattleEffects(19u8, 0u8, 77u8, 0u8, 0u16)) != 0)))
        {
            return 0u8;
        }
        if (!((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 231i32) != 0))
            && (!(((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 0i32)
                || (((((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32)))
        {
            {
                (((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .write(0u8);
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(0u8);
            }
            formChange = 1u8;
        }
        if ((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 96i32) != 0)
            && (!(((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 10i32)
                || (((((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 10i32)))
        {
            {
                (((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .write(10u8);
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(10u8);
            }
            formChange = 2u8;
        }
        if ((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 7i32) != 0)
            && (!(((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 11i32)
                || (((((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 11i32)))
        {
            {
                (((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .write(11u8);
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(11u8);
            }
            formChange = 3u8;
        }
        if ((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 128i32) != 0)
            && (!(((((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 15i32)
                || (((((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 15i32)))
        {
            {
                (((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .write(15u8);
                ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(15u8);
            }
            formChange = 4u8;
        }
        return formChange;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AbilityBattleEffects(
    caseID: u8,
    battler: u8,
    ability: u8,
    special: u8,
    moveArg: u16,
) -> u8 {
    unsafe {
        let mut caseID = caseID;
        let mut battler = battler;
        let mut ability = ability;
        let mut special = special;
        let mut moveArg = moveArg;
        let mut effect: u8 = 0u8;
        let mut pokeAtk: *mut u8 = core::ptr::null_mut();
        let mut pokeDef: *mut u8 = core::ptr::null_mut();
        let mut speciesAtk: u16 = 0u16;
        let mut speciesDef: u16 = 0u16;
        let mut pidAtk: u32 = 0u32;
        let mut pidDef: u32 = 0u32;
        if ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
            >= ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
        {
            ((&raw mut gBattlerAttacker).cast::<u8>()).write(battler);
        }
        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            pokeAtk = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        } else {
            pokeAtk = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        }
        if ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
            >= ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
        {
            ((&raw mut gBattlerTarget).cast::<u8>()).write(battler);
        }
        if ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read())) as i32) == 0i32 {
            pokeDef = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        } else {
            pokeDef = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            );
        }
        speciesAtk = ((GetMonData2(pokeAtk, 11i32)) as u16);
        pidAtk = GetMonData2(pokeAtk, 0i32);
        speciesDef = ((GetMonData2(pokeDef, 11i32)) as u16);
        pidDef = GetMonData2(pokeDef, 0i32);
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0) {
            let mut moveType: u8 = 0u8;
            let mut i: i32 = 0i32;
            let mut r#move: u16 = 0u16;
            let mut side: u8 = 0u8;
            let mut target1: u8 = 0u8;
            if (special) != 0 {
                ((&raw mut gLastUsedAbility).cast::<u8>()).write(special);
            } else {
                ((&raw mut gLastUsedAbility).cast::<u8>()).write(
                    ((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(32))
                    .read(),
                );
            }
            if (moveArg) != 0 {
                r#move = moveArg;
            } else {
                r#move = ((&raw mut gCurrentMove).cast::<u16>()).read();
            }
            {
                if (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).read())
                    != 0
                {
                    moveType = ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .read()) as i32)
                        & 63i32) as u8);
                } else {
                    moveType = ((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 12))
                    .wrapping_add(2))
                    .read();
                }
            }
            'l1: {
                let __sw1 = ((caseID) as i32);
                if __sw1 == 0i32 {
                    if ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                        >= ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                    {
                        ((&raw mut gBattlerAttacker).cast::<u8>()).write(battler);
                    }
                    'l2: {
                        let __sw2 = ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32);
                        if __sw2 == 255i32 {
                            if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32)
                                != 0)
                            {
                                'l3: {
                                    let __sw3 = ((GetCurrentWeather()) as i32);
                                    if __sw3 == 3i32 || __sw3 == 5i32 || __sw3 == 13i32 {
                                        if !((((((&raw mut gBattleWeather).cast::<u16>()).read())
                                            as i32)
                                            & 7i32)
                                            != 0)
                                        {
                                            ((&raw mut gBattleWeather).cast::<u16>()).write(5u16);
                                            (((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(16))
                                            .write(10u8);
                                            (((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(23))
                                            .write(battler);
                                            effect = (effect).wrapping_add(1);
                                        }
                                        break 'l3;
                                    }
                                    if __sw3 == 8i32 {
                                        if !((((((&raw mut gBattleWeather).cast::<u16>()).read())
                                            as i32)
                                            & 24i32)
                                            != 0)
                                        {
                                            ((&raw mut gBattleWeather).cast::<u16>()).write(24u16);
                                            (((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(16))
                                            .write(12u8);
                                            (((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(23))
                                            .write(battler);
                                            effect = (effect).wrapping_add(1);
                                        }
                                        break 'l3;
                                    }
                                    if __sw3 == 12i32 {
                                        if !((((((&raw mut gBattleWeather).cast::<u16>()).read())
                                            as i32)
                                            & 96i32)
                                            != 0)
                                        {
                                            ((&raw mut gBattleWeather).cast::<u16>()).write(96u16);
                                            (((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(16))
                                            .write(11u8);
                                            (((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(23))
                                            .write(battler);
                                            effect = (effect).wrapping_add(1);
                                        }
                                        break 'l3;
                                    }
                                }
                            }
                            if ((effect) as i32) != 0i32 {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(GetCurrentWeather());
                                BattleScriptPushCursorAndCallback(
                                    (&raw mut BattleScript_OverworldWeatherStarts).cast::<u8>(),
                                );
                            }
                            break 'l2;
                        }
                        if __sw2 == 2i32 {
                            if !((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                & 4i32)
                                != 0)
                            {
                                ((&raw mut gBattleWeather).cast::<u16>()).write(5u16);
                                BattleScriptPushCursorAndCallback(
                                    (&raw mut BattleScript_DrizzleActivates).cast::<u8>(),
                                );
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(battler);
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l2;
                        }
                        if __sw2 == 45i32 {
                            if !((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                & 16i32)
                                != 0)
                            {
                                ((&raw mut gBattleWeather).cast::<u16>()).write(24u16);
                                BattleScriptPushCursorAndCallback(
                                    (&raw mut BattleScript_SandstreamActivates).cast::<u8>(),
                                );
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(battler);
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l2;
                        }
                        if __sw2 == 70i32 {
                            if !((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32)
                                & 64i32)
                                != 0)
                            {
                                ((&raw mut gBattleWeather).cast::<u16>()).write(96u16);
                                BattleScriptPushCursorAndCallback(
                                    (&raw mut BattleScript_DroughtActivates).cast::<u8>(),
                                );
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(battler);
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l2;
                        }
                        if __sw2 == 22i32 {
                            if !((crate::c::bf_read(
                                (((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 20))
                                .wrapping_add(0),
                                3,
                                1,
                                false,
                            ) as u32)
                                != 0)
                            {
                                let __p4 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(((battler) as i32) as isize);
                                (__p4).write(((__p4).read() | 524288u32));
                                crate::c::bf_write(
                                    (((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 20))
                                    .wrapping_add(0),
                                    3,
                                    1,
                                    (1u32) as i32,
                                );
                            }
                            break 'l2;
                        }
                        if __sw2 == 59i32 {
                            effect = CastformDataTypeChange(battler);
                            if ((effect) as i32) != 0i32 {
                                BattleScriptPushCursorAndCallback(
                                    (&raw mut BattleScript_CastformChange).cast::<u8>(),
                                );
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(battler);
                                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(127))
                                .write(((((effect) as i32).wrapping_sub(1i32)) as u8));
                            }
                            break 'l2;
                        }
                        if __sw2 == 36i32 {
                            if !((crate::c::bf_read(
                                (((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 20))
                                .wrapping_add(0),
                                4,
                                1,
                                false,
                            ) as u32)
                                != 0)
                            {
                                let __p5 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(((battler) as i32) as isize);
                                (__p5).write(((__p5).read() | 1048576u32));
                                crate::c::bf_write(
                                    (((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 20))
                                    .wrapping_add(0),
                                    4,
                                    1,
                                    (1u32) as i32,
                                );
                            }
                            break 'l2;
                        }
                        if __sw2 == 13i32 || __sw2 == 77i32 {
                            {
                                {
                                    target1 = 0u8;
                                    'l4: loop {
                                        if !(((target1) as i32)
                                            < ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                                as i32))
                                        {
                                            break 'l4;
                                        }
                                        'l5: {
                                            effect = CastformDataTypeChange(target1);
                                            if ((effect) as i32) != 0i32 {
                                                BattleScriptPushCursorAndCallback(
                                                    (&raw mut BattleScript_CastformChange)
                                                        .cast::<u8>(),
                                                );
                                                (((&raw mut gBattleScripting).cast::<u8>())
                                                    .wrapping_add(23))
                                                .write(target1);
                                                ((((&raw mut gBattleStruct).cast::<*mut u8>())
                                                    .read())
                                                .wrapping_add(127))
                                                .write(
                                                    ((((effect) as i32).wrapping_sub(1i32)) as u8),
                                                );
                                                break 'l4;
                                            }
                                        }
                                        target1 = (target1).wrapping_add(1);
                                    }
                                }
                            }
                            break 'l2;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    if ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        ((&raw mut gBattlerAttacker).cast::<u8>()).write(battler);
                        'l6: {
                            let __sw6 =
                                ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32);
                            if __sw6 == 44i32 {
                                if (((!((AbilityBattleEffects(19u8, 0u8, 13u8, 0u8, 0u16)) != 0))
                                    && (!((AbilityBattleEffects(19u8, 0u8, 77u8, 0u8, 0u16))
                                        != 0)))
                                    && ((((((&raw mut gBattleWeather).cast::<u16>()).read())
                                        as i32)
                                        & 7i32)
                                        != 0))
                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        > ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(40)
                                        .cast::<u16>())
                                        .read()) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(44u8);
                                    BattleScriptPushCursorAndCallback(
                                        (&raw mut BattleScript_RainDishActivates).cast::<u8>(),
                                    );
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        crate::c::div_i32(
                                            ((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((battler) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(44)
                                            .cast::<u16>())
                                            .read())
                                                as i32),
                                            16i32,
                                        ),
                                    );
                                    if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                    }
                                    let __p7 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                    (__p7).write(((__p7).read()).wrapping_mul((-1i32)));
                                    effect = (effect).wrapping_add(1);
                                }
                                break 'l6;
                            }
                            if __sw6 == 61i32 {
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 255u32)
                                    != 0)
                                    && (crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32)
                                {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 136u32)
                                        != 0
                                    {
                                        StringCopy(
                                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                                            (&raw mut gStatusConditionString_PoisonJpn)
                                                .cast::<u8>(),
                                        );
                                    }
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 7u32)
                                        != 0
                                    {
                                        StringCopy(
                                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                                            (&raw mut gStatusConditionString_SleepJpn).cast::<u8>(),
                                        );
                                    }
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 64u32)
                                        != 0
                                    {
                                        StringCopy(
                                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                                            (&raw mut gStatusConditionString_ParalysisJpn)
                                                .cast::<u8>(),
                                        );
                                    }
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 16u32)
                                        != 0
                                    {
                                        StringCopy(
                                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                                            (&raw mut gStatusConditionString_BurnJpn).cast::<u8>(),
                                        );
                                    }
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 32u32)
                                        != 0
                                    {
                                        StringCopy(
                                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                                            (&raw mut gStatusConditionString_IceJpn).cast::<u8>(),
                                        );
                                    }
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .write(0u32);
                                    let __p8 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(80)
                                    .cast::<u32>();
                                    (__p8).write(((__p8).read() & 4160749567u32));
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                        .write({
                                            let __v9 = battler;
                                            ((&raw mut gActiveBattler).cast::<u8>()).write(__v9);
                                            __v9
                                        });
                                    BattleScriptPushCursorAndCallback(
                                        (&raw mut BattleScript_ShedSkinActivates).cast::<u8>(),
                                    );
                                    BtlController_EmitSetMonData(
                                        0u8,
                                        40u8,
                                        0u8,
                                        4u8,
                                        ((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .cast::<u8>(),
                                    );
                                    MarkBattlerForControllerExec(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    );
                                    effect = (effect).wrapping_add(1);
                                }
                                break 'l6;
                            }
                            if __sw6 == 3i32 {
                                if (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(3))
                                .read()) as i32)
                                    < 12i32)
                                    && (((((((&raw mut gDisableStructs).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 28))
                                    .wrapping_add(22))
                                    .read()) as i32)
                                        != 2i32)
                                {
                                    let __p10 = (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset(3);
                                    (__p10).write(((__p10).read()).wrapping_add(1));
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                        .write(17u8);
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                        .write(0u8);
                                    BattleScriptPushCursorAndCallback(
                                        (&raw mut BattleScript_SpeedBoostActivates).cast::<u8>(),
                                    );
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                        .write(battler);
                                    effect = (effect).wrapping_add(1);
                                }
                                break 'l6;
                            }
                            if __sw6 == 54i32 {
                                crate::c::bf_write(
                                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(24),
                                    0,
                                    1,
                                    ((((crate::c::bf_read(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(24),
                                        0,
                                        1,
                                        false,
                                    ) as u8) as i32)
                                        ^ 1i32) as u8) as i32,
                                );
                                break 'l6;
                            }
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    if ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32) == 43i32 {
                        {
                            i = 0i32;
                            'l7: loop {
                                if !(((((((&raw const sSoundMovesTable)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    != 65535i32)
                                {
                                    break 'l7;
                                }
                                'l8: {
                                    if ((((((&raw const sSoundMovesTable)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        == ((r#move) as i32)
                                    {
                                        break 'l7;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((((((&raw const sSoundMovesTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 65535i32
                        {
                            if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 88,
                            ))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 4096u32)
                                != 0
                            {
                                let __p11 = (&raw mut gHitMarker).cast::<u32>();
                                (__p11).write(((__p11).read() | 2048u32));
                            }
                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                .write((&raw mut BattleScript_SoundproofProtected).cast::<u8>());
                            effect = 1u8;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    if (r#move) != 0 {
                        'l9: {
                            let __sw12 =
                                ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32);
                            if __sw12 == 10i32 {
                                if (((moveType) as i32) == 13i32)
                                    && (((((((&raw mut gBattleMoves).cast::<u8>())
                                        .wrapping_offset(((r#move) as i32) as isize * 12))
                                    .wrapping_add(1))
                                    .read()) as i32)
                                        != 0i32)
                                {
                                    if (crate::c::bf_read(
                                        (((&raw mut gProtectStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 16,
                                            ))
                                        .wrapping_add(2),
                                        3,
                                        1,
                                        false,
                                    ) as u32)
                                        != 0
                                    {
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_MoveHPDrain).cast::<u8>(),
                                            );
                                    } else {
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_MoveHPDrain_PPLoss)
                                                    .cast::<u8>(),
                                            );
                                    }
                                    effect = 1u8;
                                }
                                break 'l9;
                            }
                            if __sw12 == 11i32 {
                                if (((moveType) as i32) == 11i32)
                                    && (((((((&raw mut gBattleMoves).cast::<u8>())
                                        .wrapping_offset(((r#move) as i32) as isize * 12))
                                    .wrapping_add(1))
                                    .read()) as i32)
                                        != 0i32)
                                {
                                    if (crate::c::bf_read(
                                        (((&raw mut gProtectStructs).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 16,
                                            ))
                                        .wrapping_add(2),
                                        3,
                                        1,
                                        false,
                                    ) as u32)
                                        != 0
                                    {
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_MoveHPDrain).cast::<u8>(),
                                            );
                                    } else {
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_MoveHPDrain_PPLoss)
                                                    .cast::<u8>(),
                                            );
                                    }
                                    effect = 1u8;
                                }
                                break 'l9;
                            }
                            if __sw12 == 18i32 {
                                if (((moveType) as i32) == 10i32)
                                    && (!((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 32u32)
                                        != 0))
                                {
                                    if !(((((((((&raw mut gBattleResources)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                    .read()
                                        & 1u32)
                                        != 0)
                                    {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(0u8);
                                        if (crate::c::bf_read(
                                            (((&raw mut gProtectStructs).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gBattlerAttacker).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize
                                                        * 16,
                                                ))
                                            .wrapping_add(2),
                                            3,
                                            1,
                                            false,
                                        ) as u32)
                                            != 0
                                        {
                                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                                .write(
                                                    (&raw mut BattleScript_FlashFireBoost)
                                                        .cast::<u8>(),
                                                );
                                        } else {
                                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                                .write(
                                                    (&raw mut BattleScript_FlashFireBoost_PPLoss)
                                                        .cast::<u8>(),
                                                );
                                        }
                                        let __p13 = ((((((&raw mut gBattleResources)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .cast::<u32>())
                                        .wrapping_offset(((battler) as i32) as isize);
                                        (__p13).write(((__p13).read() | 1u32));
                                        effect = 2u8;
                                    } else {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(1u8);
                                        if (crate::c::bf_read(
                                            (((&raw mut gProtectStructs).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gBattlerAttacker).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize
                                                        * 16,
                                                ))
                                            .wrapping_add(2),
                                            3,
                                            1,
                                            false,
                                        ) as u32)
                                            != 0
                                        {
                                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                                .write(
                                                    (&raw mut BattleScript_FlashFireBoost)
                                                        .cast::<u8>(),
                                                );
                                        } else {
                                            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                                .write(
                                                    (&raw mut BattleScript_FlashFireBoost_PPLoss)
                                                        .cast::<u8>(),
                                                );
                                        }
                                        effect = 2u8;
                                    }
                                }
                                break 'l9;
                            }
                        }
                        if ((effect) as i32) == 1i32 {
                            if ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(44)
                            .cast::<u16>())
                            .read()) as i32)
                                == ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                            {
                                if (crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(2),
                                    3,
                                    1,
                                    false,
                                ) as u32)
                                    != 0
                                {
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                        (&raw mut BattleScript_MonMadeMoveUseless).cast::<u8>(),
                                    );
                                } else {
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                        (&raw mut BattleScript_MonMadeMoveUseless_PPLoss)
                                            .cast::<u8>(),
                                    );
                                }
                            } else {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        4i32,
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                let __p14 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p14).write(((__p14).read()).wrapping_mul((-1i32)));
                            }
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    'l10: {
                        let __sw15 = ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32);
                        if __sw15 == 16i32 {
                            if (((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((r#move) as i32) != 165i32))
                                && (((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(1))
                                .read()) as i32)
                                    != 0i32))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && (!(((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(33))
                                .cast::<u8>())
                                .read()) as i32)
                                    == ((moveType) as i32))
                                    || (((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(33))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        == ((moveType) as i32)))))
                                && (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32)
                            {
                                {
                                    (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(33))
                                    .cast::<u8>())
                                    .write(moveType);
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(33))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .write(moveType);
                                }
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(3u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(moveType);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_ColorChangeActivates).cast::<u8>(),
                                );
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                        if __sw15 == 24i32 {
                            if ((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (!((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 1i32)
                                    != 0)
                            {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerAttacker).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        16i32,
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_RoughSkinActivates).cast::<u8>());
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                        if __sw15 == 27i32 {
                            if (((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (!((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 1i32)
                                    != 0))
                                && (crate::c::rem_i32(((Random()) as i32), 10i32) == 0i32)
                            {
                                'l11: loop {
                                    'l12: {
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write(((((Random()) as i32) & 3i32) as u8));
                                    }
                                    if !((((((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(3))
                                    .read()) as i32)
                                        == 0i32)
                                    {
                                        break 'l11;
                                    }
                                }
                                if (((((&raw mut gBattleCommunication).cast::<u8>())
                                    .wrapping_offset(3))
                                .read()) as i32)
                                    == 3i32
                                {
                                    let __p16 = ((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(3);
                                    (__p16).write(
                                        (((((__p16).read()) as i32).wrapping_add(2i32)) as u8),
                                    );
                                }
                                let __p17 = ((&raw mut gBattleCommunication).cast::<u8>())
                                    .wrapping_offset(3);
                                (__p17)
                                    .write((((((__p17).read()) as i32).wrapping_add(64i32)) as u8));
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_ApplySecondaryEffect).cast::<u8>(),
                                );
                                let __p18 = (&raw mut gHitMarker).cast::<u32>();
                                (__p18).write(((__p18).read() | 8192u32));
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                        if __sw15 == 38i32 {
                            if (((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (!((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 1i32)
                                    != 0))
                                && (crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32)
                            {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3))
                                    .write(66u8);
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_ApplySecondaryEffect).cast::<u8>(),
                                );
                                let __p19 = (&raw mut gHitMarker).cast::<u32>();
                                (__p19).write(((__p19).read() | 8192u32));
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                        if __sw15 == 9i32 {
                            if (((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (!((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 1i32)
                                    != 0))
                                && (crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32)
                            {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3))
                                    .write(69u8);
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_ApplySecondaryEffect).cast::<u8>(),
                                );
                                let __p20 = (&raw mut gHitMarker).cast::<u32>();
                                (__p20).write(((__p20).read() | 8192u32));
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                        if __sw15 == 49i32 {
                            if (((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (!((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 1i32)
                                    != 0))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && (crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32)
                            {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3))
                                    .write(67u8);
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                    (&raw mut BattleScript_ApplySecondaryEffect).cast::<u8>(),
                                );
                                let __p21 = (&raw mut gHitMarker).cast::<u32>();
                                (__p21).write(((__p21).read() | 8192u32));
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                        if __sw15 == 56i32 {
                            if (((((((((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (!((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 1i32)
                                    != 0))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32))
                                && (crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(32))
                                .read()) as i32)
                                    != 12i32))
                                && (((GetGenderFromSpeciesAndPersonality(speciesAtk, pidAtk))
                                    as i32)
                                    != ((GetGenderFromSpeciesAndPersonality(speciesDef, pidDef))
                                        as i32)))
                                && (!((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 983040u32)
                                    != 0)))
                                && (((GetGenderFromSpeciesAndPersonality(speciesAtk, pidAtk))
                                    as i32)
                                    != 255i32))
                                && (((GetGenderFromSpeciesAndPersonality(speciesDef, pidDef))
                                    as i32)
                                    != 255i32)
                            {
                                let __p22 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p22).write(
                                    ((__p22).read()
                                        | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read()
                                            << 16)),
                                );
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_CuteCharmActivates).cast::<u8>());
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l10;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    {
                        battler = 0u8;
                        'l13: loop {
                            if !(((battler) as i32)
                                < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                            {
                                break 'l13;
                            }
                            'l14: {
                                'l15: {
                                    let __sw23 = ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(32))
                                    .read())
                                        as i32);
                                    if __sw23 == 17i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 3976u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_PoisonJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 1u8;
                                        }
                                        break 'l15;
                                    }
                                    if __sw23 == 20i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_ConfusionJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 2u8;
                                        }
                                        break 'l15;
                                    }
                                    if __sw23 == 7i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 64u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_ParalysisJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 1u8;
                                        }
                                        break 'l15;
                                    }
                                    if __sw23 == 15i32 || __sw23 == 72i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0
                                        {
                                            let __p24 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(((battler) as i32) as isize * 88))
                                            .wrapping_add(80)
                                            .cast::<u32>();
                                            (__p24).write(((__p24).read() & 4160749567u32));
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_SleepJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 1u8;
                                        }
                                        break 'l15;
                                    }
                                    if __sw23 == 41i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 16u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_BurnJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 1u8;
                                        }
                                        break 'l15;
                                    }
                                    if __sw23 == 40i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 32u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_IceJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 1u8;
                                        }
                                        break 'l15;
                                    }
                                    if __sw23 == 12i32 {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 983040u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_LoveJpn)
                                                    .cast::<u8>(),
                                            );
                                            effect = 3u8;
                                        }
                                        break 'l15;
                                    }
                                }
                                if ((effect) as i32) != 0i32 {
                                    'l16: {
                                        let __sw25 = ((effect) as i32);
                                        if __sw25 == 1i32 {
                                            ((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((battler) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(76)
                                            .cast::<u32>())
                                            .write(0u32);
                                            break 'l16;
                                        }
                                        if __sw25 == 2i32 {
                                            let __p26 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(((battler) as i32) as isize * 88))
                                            .wrapping_add(80)
                                            .cast::<u32>();
                                            (__p26).write(((__p26).read() & 4294967288u32));
                                            break 'l16;
                                        }
                                        if __sw25 == 3i32 {
                                            let __p27 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(((battler) as i32) as isize * 88))
                                            .wrapping_add(80)
                                            .cast::<u32>();
                                            (__p27).write(((__p27).read() & 4293984255u32));
                                            break 'l16;
                                        }
                                    }
                                    BattleScriptPushCursor();
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                        (&raw mut BattleScript_AbilityCuredStatus).cast::<u8>(),
                                    );
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                        .write(battler);
                                    ((&raw mut gActiveBattler).cast::<u8>()).write(battler);
                                    BtlController_EmitSetMonData(
                                        0u8,
                                        40u8,
                                        0u8,
                                        4u8,
                                        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .cast::<u8>(),
                                    );
                                    MarkBattlerForControllerExec(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    );
                                    return effect;
                                }
                            }
                            battler = (battler).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    {
                        battler = 0u8;
                        'l17: loop {
                            if !(((battler) as i32)
                                < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                            {
                                break 'l17;
                            }
                            'l18: {
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == 59i32
                                {
                                    effect = CastformDataTypeChange(battler);
                                    if ((effect) as i32) != 0i32 {
                                        BattleScriptPushCursorAndCallback(
                                            (&raw mut BattleScript_CastformChange).cast::<u8>(),
                                        );
                                        (((&raw mut gBattleScripting).cast::<u8>())
                                            .wrapping_add(23))
                                        .write(battler);
                                        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                            .wrapping_add(127))
                                        .write(((((effect) as i32).wrapping_sub(1i32)) as u8));
                                        return effect;
                                    }
                                }
                            }
                            battler = (battler).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 7i32 {
                    if (((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32) == 28i32)
                        && ((((&raw mut gHitMarker).cast::<u32>()).read() & 16384u32) != 0)
                    {
                        let __p28 = (&raw mut gHitMarker).cast::<u32>();
                        (__p28).write(((__p28).read() & 4294950911u32));
                        let __p29 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(178);
                        (__p29).write((((((__p29).read()) as i32) & (-193i32)) as u8));
                        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(178))
                        .read()) as i32)
                            == 6i32
                        {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(178))
                            .write(2u8);
                        }
                        (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3)).write(
                            ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(178))
                            .read()) as i32)
                                .wrapping_add(64i32)) as u8),
                        );
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                            .write(((&raw mut gBattlerTarget).cast::<u8>()).read());
                        BattleScriptPushCursor();
                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                            .write((&raw mut BattleScript_SynchronizeActivates).cast::<u8>());
                        let __p30 = (&raw mut gHitMarker).cast::<u32>();
                        (__p30).write(((__p30).read() | 8192u32));
                        effect = (effect).wrapping_add(1);
                    }
                    break 'l1;
                }
                if __sw1 == 8i32 {
                    if (((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32) == 28i32)
                        && ((((&raw mut gHitMarker).cast::<u32>()).read() & 16384u32) != 0)
                    {
                        let __p31 = (&raw mut gHitMarker).cast::<u32>();
                        (__p31).write(((__p31).read() & 4294950911u32));
                        let __p32 =
                            (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(178);
                        (__p32).write((((((__p32).read()) as i32) & (-193i32)) as u8));
                        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(178))
                        .read()) as i32)
                            == 6i32
                        {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(178))
                            .write(2u8);
                        }
                        (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3)).write(
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(178))
                            .read(),
                        );
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                            .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                        BattleScriptPushCursor();
                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                            .write((&raw mut BattleScript_SynchronizeActivates).cast::<u8>());
                        let __p33 = (&raw mut gHitMarker).cast::<u32>();
                        (__p33).write(((__p33).read() | 8192u32));
                        effect = (effect).wrapping_add(1);
                    }
                    break 'l1;
                }
                if __sw1 == 9i32 {
                    {
                        i = 0i32;
                        'l19: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l19;
                            }
                            'l20: {
                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == 22i32)
                                    && ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset((i) as isize))
                                    .read()
                                        & 524288u32)
                                        != 0)
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(22u8);
                                    let __p34 = (((&raw mut gStatuses3).cast::<u32>())
                                        .cast::<u32>())
                                    .wrapping_offset((i) as isize);
                                    (__p34).write(((__p34).read() & 4294443007u32));
                                    BattleScriptPushCursorAndCallback(
                                        (&raw mut BattleScript_IntimidateActivatesEnd3)
                                            .cast::<u8>(),
                                    );
                                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(216))
                                    .write(((i) as u8));
                                    effect = (effect).wrapping_add(1);
                                    break 'l19;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 11i32 {
                    {
                        i = 0i32;
                        'l21: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l21;
                            }
                            'l22: {
                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == 36i32)
                                    && ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset((i) as isize))
                                    .read()
                                        & 1048576u32)
                                        != 0)
                                {
                                    let mut target2: u8 = 0u8;
                                    side = (((((GetBattlerPosition(((i) as u8))) as i32) ^ 1i32)
                                        & 1i32) as u8);
                                    target1 = GetBattlerAtPosition(side);
                                    target2 = GetBattlerAtPosition(
                                        ((((side) as i32).wrapping_add(2i32)) as u8),
                                    );
                                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32)
                                        != 0
                                    {
                                        if (((((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((target1) as i32) as isize * 88))
                                        .wrapping_add(32))
                                        .read())
                                            as i32)
                                            != 0i32)
                                            && (((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((target1) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(40)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                != 0i32))
                                            && (((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((target2) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(32))
                                            .read())
                                                as i32)
                                                != 0i32))
                                            && (((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((target2) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(40)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                != 0i32)
                                        {
                                            ((&raw mut gActiveBattler).cast::<u8>()).write(
                                                GetBattlerAtPosition(
                                                    (((((Random()) as i32) & 1i32)
                                                        .wrapping_mul(2i32)
                                                        | ((side) as i32))
                                                        as u8),
                                                ),
                                            );
                                            ((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset((i) as isize * 88))
                                            .wrapping_add(32))
                                            .write(
                                                ((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler).cast::<u8>())
                                                            .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                .wrapping_add(32))
                                                .read(),
                                            );
                                            ((&raw mut gLastUsedAbility).cast::<u8>()).write(
                                                ((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler).cast::<u8>())
                                                            .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                .wrapping_add(32))
                                                .read(),
                                            );
                                            effect = (effect).wrapping_add(1);
                                        } else {
                                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((target1) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(32))
                                            .read())
                                                as i32)
                                                != 0i32)
                                                && (((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((target1) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    != 0i32)
                                            {
                                                ((&raw mut gActiveBattler).cast::<u8>())
                                                    .write(target1);
                                                ((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 88))
                                                .wrapping_add(32))
                                                .write(
                                                    ((((&raw mut gBattleMons).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 88,
                                                        ))
                                                    .wrapping_add(32))
                                                    .read(),
                                                );
                                                ((&raw mut gLastUsedAbility).cast::<u8>()).write(
                                                    ((((&raw mut gBattleMons).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 88,
                                                        ))
                                                    .wrapping_add(32))
                                                    .read(),
                                                );
                                                effect = (effect).wrapping_add(1);
                                            } else {
                                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((target2) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(32))
                                                .read())
                                                    as i32)
                                                    != 0i32)
                                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((target2) as i32) as isize * 88,
                                                        ))
                                                    .wrapping_add(40)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        != 0i32)
                                                {
                                                    ((&raw mut gActiveBattler).cast::<u8>())
                                                        .write(target2);
                                                    ((((&raw mut gBattleMons).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 88))
                                                    .wrapping_add(32))
                                                    .write(
                                                        ((((&raw mut gBattleMons).cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize
                                                                    * 88,
                                                            ))
                                                        .wrapping_add(32))
                                                        .read(),
                                                    );
                                                    ((&raw mut gLastUsedAbility).cast::<u8>())
                                                        .write(
                                                            ((((&raw mut gBattleMons)
                                                                .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize
                                                                    * 88,
                                                            ))
                                                            .wrapping_add(32))
                                                            .read(),
                                                        );
                                                    effect = (effect).wrapping_add(1);
                                                }
                                            }
                                        }
                                    } else {
                                        ((&raw mut gActiveBattler).cast::<u8>()).write(target1);
                                        if ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((target1) as i32) as isize * 88))
                                        .wrapping_add(32))
                                        .read())
                                            != 0)
                                            && ((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((target1) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(40)
                                            .cast::<u16>())
                                            .read())
                                                != 0)
                                        {
                                            ((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset((i) as isize * 88))
                                            .wrapping_add(32))
                                            .write(
                                                ((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((target1) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(32))
                                                .read(),
                                            );
                                            ((&raw mut gLastUsedAbility).cast::<u8>()).write(
                                                ((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((target1) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(32))
                                                .read(),
                                            );
                                            effect = (effect).wrapping_add(1);
                                        }
                                    }
                                    if ((effect) as i32) != 0i32 {
                                        BattleScriptPushCursorAndCallback(
                                            (&raw mut BattleScript_TraceActivates).cast::<u8>(),
                                        );
                                        let __p35 = (((&raw mut gStatuses3).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset((i) as isize);
                                        (__p35).write(((__p35).read() & 4293918719u32));
                                        (((&raw mut gBattleScripting).cast::<u8>())
                                            .wrapping_add(23))
                                        .write(((i) as u8));
                                        {
                                            ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                            (((&raw mut gBattleTextBuff1).cast::<u8>())
                                                .wrapping_offset(1))
                                            .write(4u8);
                                            (((&raw mut gBattleTextBuff1).cast::<u8>())
                                                .wrapping_offset(2))
                                            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                                            (((&raw mut gBattleTextBuff1).cast::<u8>())
                                                .wrapping_offset(3))
                                            .write(
                                                ((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as u8),
                                            );
                                            (((&raw mut gBattleTextBuff1).cast::<u8>())
                                                .wrapping_offset(4))
                                            .write(255u8);
                                        }
                                        {
                                            ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
                                            (((&raw mut gBattleTextBuff2).cast::<u8>())
                                                .wrapping_offset(1))
                                            .write(9u8);
                                            (((&raw mut gBattleTextBuff2).cast::<u8>())
                                                .wrapping_offset(2))
                                            .write(
                                                ((&raw mut gLastUsedAbility).cast::<u8>()).read(),
                                            );
                                            (((&raw mut gBattleTextBuff2).cast::<u8>())
                                                .wrapping_offset(3))
                                            .write(255u8);
                                        }
                                        break 'l21;
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 10i32 {
                    {
                        i = 0i32;
                        'l23: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l23;
                            }
                            'l24: {
                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == 22i32)
                                    && ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset((i) as isize))
                                    .read()
                                        & 524288u32)
                                        != 0)
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(22u8);
                                    let __p36 = (((&raw mut gStatuses3).cast::<u32>())
                                        .cast::<u32>())
                                    .wrapping_offset((i) as isize);
                                    (__p36).write(((__p36).read() & 4294443007u32));
                                    BattleScriptPushCursor();
                                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(
                                        (&raw mut BattleScript_IntimidateActivates).cast::<u8>(),
                                    );
                                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(216))
                                    .write(((i) as u8));
                                    effect = (effect).wrapping_add(1);
                                    break 'l23;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 12i32 {
                    side = GetBattlerSide(battler);
                    {
                        i = 0i32;
                        'l25: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l25;
                            }
                            'l26: {
                                if (((GetBattlerSide(((i) as u8))) as i32) != ((side) as i32))
                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 88))
                                    .wrapping_add(32))
                                    .read()) as i32)
                                        == ((ability) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (((i).wrapping_add(1i32)) as u8);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 13i32 {
                    side = GetBattlerSide(battler);
                    {
                        i = 0i32;
                        'l27: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l27;
                            }
                            'l28: {
                                if (((GetBattlerSide(((i) as u8))) as i32) == ((side) as i32))
                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 88))
                                    .wrapping_add(32))
                                    .read()) as i32)
                                        == ((ability) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (((i).wrapping_add(1i32)) as u8);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 14i32 {
                    'l29: {
                        let __sw37 = ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32);
                        let __matched = __sw37 == 253i32 || __sw37 == 254i32;
                        if __sw37 == 253i32 {
                            {
                                i = 0i32;
                                'l30: loop {
                                    if !(i
                                        < ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                            as i32))
                                    {
                                        break 'l30;
                                    }
                                    'l31: {
                                        if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset((i) as isize))
                                        .read()
                                            & 65536u32)
                                            != 0
                                        {
                                            effect = (((i).wrapping_add(1i32)) as u8);
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            break 'l29;
                        }
                        if __sw37 == 254i32 {
                            {
                                i = 0i32;
                                'l32: loop {
                                    if !(i
                                        < ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                            as i32))
                                    {
                                        break 'l32;
                                    }
                                    'l33: {
                                        if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset((i) as isize))
                                        .read()
                                            & 131072u32)
                                            != 0
                                        {
                                            effect = (((i).wrapping_add(1i32)) as u8);
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            break 'l29;
                        }
                        if !__matched {
                            {
                                i = 0i32;
                                'l34: loop {
                                    if !(i
                                        < ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                            as i32))
                                    {
                                        break 'l34;
                                    }
                                    'l35: {
                                        if ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset((i) as isize * 88))
                                        .wrapping_add(32))
                                        .read()) as i32)
                                            == ((ability) as i32)
                                        {
                                            ((&raw mut gLastUsedAbility).cast::<u8>())
                                                .write(ability);
                                            effect = (((i).wrapping_add(1i32)) as u8);
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            break 'l29;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 19i32 {
                    {
                        i = 0i32;
                        'l36: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l36;
                            }
                            'l37: {
                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == ((ability) as i32))
                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 88))
                                    .wrapping_add(40)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        != 0i32)
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (((i).wrapping_add(1i32)) as u8);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 15i32 {
                    {
                        i = 0i32;
                        'l38: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l38;
                            }
                            'l39: {
                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == ((ability) as i32))
                                    && (i != ((battler) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (((i).wrapping_add(1i32)) as u8);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 16i32 {
                    side = GetBattlerSide(battler);
                    {
                        i = 0i32;
                        'l40: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l40;
                            }
                            'l41: {
                                if (((GetBattlerSide(((i) as u8))) as i32) != ((side) as i32))
                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 88))
                                    .wrapping_add(32))
                                    .read()) as i32)
                                        == ((ability) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 17i32 {
                    side = GetBattlerSide(battler);
                    {
                        i = 0i32;
                        'l42: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l42;
                            }
                            'l43: {
                                if (((GetBattlerSide(((i) as u8))) as i32) == ((side) as i32))
                                    && (((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 88))
                                    .wrapping_add(32))
                                    .read()) as i32)
                                        == ((ability) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 18i32 {
                    {
                        i = 0i32;
                        'l44: loop {
                            if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                                break 'l44;
                            }
                            'l45: {
                                if (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 88))
                                .wrapping_add(32))
                                .read()) as i32)
                                    == ((ability) as i32))
                                    && (i != ((battler) as i32))
                                {
                                    ((&raw mut gLastUsedAbility).cast::<u8>()).write(ability);
                                    effect = (effect).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
            if (((effect) != 0) && (((caseID) as i32) < 12i32))
                && (((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32) != 255i32)
            {
                RecordAbilityBattle(battler, ((&raw mut gLastUsedAbility).cast::<u8>()).read());
            }
        }
        return effect;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleScriptExecute(BS_ptr: *mut u8) {
    unsafe {
        let mut BS_ptr = BS_ptr;
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(BS_ptr);
        (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>()).read());
        ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(RunBattleScriptCommands_PopCallbacksStack));
        ((&raw mut gCurrentActionFuncId).cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleScriptPushCursorAndCallback(BS_ptr: *mut u8) {
    unsafe {
        let mut BS_ptr = BS_ptr;
        BattleScriptPushCursor();
        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).write(BS_ptr);
        (((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            (({
                let __p1 = (((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize,
        ))
        .write(((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>()).read());
        ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(RunBattleScriptCommands));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemBattleEffects(caseID: u8, battler: u8, moveTurn: u8) -> u8 {
    unsafe {
        let mut caseID = caseID;
        let mut battler = battler;
        let mut moveTurn = moveTurn;
        let mut i: i32 = 0i32;
        let mut effect: u8 = 0u8;
        let mut changedPP: u8 = 0u8;
        let mut battlerHoldEffect: u8 = 0u8;
        let mut atkHoldEffect: u8 = 0u8;
        let mut defHoldEffect: u8 = 0u8;
        let mut battlerHoldEffectParam: u8 = 0u8;
        let mut atkHoldEffectParam: u8 = 0u8;
        let mut defHoldEffectParam: u8 = 0u8;
        let mut atkItem: u16 = 0u16;
        let mut defItem: u16 = 0u16;
        ((&raw mut gLastUsedItem).cast::<u16>()).write(
            ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(46)
            .cast::<u16>())
            .read(),
        );
        if ((((&raw mut gLastUsedItem).cast::<u16>()).read()) as i32) == 175i32 {
            battlerHoldEffect = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
            battlerHoldEffectParam = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 28))
            .wrapping_add(26))
            .read();
        } else {
            battlerHoldEffect = GetItemHoldEffect(((&raw mut gLastUsedItem).cast::<u16>()).read());
            battlerHoldEffectParam =
                GetItemHoldEffectParam(((&raw mut gLastUsedItem).cast::<u16>()).read());
        }
        atkItem = ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(46)
        .cast::<u16>())
        .read();
        if ((atkItem) as i32) == 175i32 {
            atkHoldEffect = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(7))
            .read();
            atkHoldEffectParam = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(26))
            .read();
        } else {
            atkHoldEffect = GetItemHoldEffect(atkItem);
            atkHoldEffectParam = GetItemHoldEffectParam(atkItem);
        }
        defItem = ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(46)
        .cast::<u16>())
        .read();
        if ((defItem) as i32) == 175i32 {
            defHoldEffect = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(7))
            .read();
            defHoldEffectParam = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(26))
            .read();
        } else {
            defHoldEffect = GetItemHoldEffect(defItem);
            defHoldEffectParam = GetItemHoldEffectParam(defItem);
        }
        'l1: {
            let __sw1 = ((caseID) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = ((battlerHoldEffect) as i32);
                    if __sw2 == 32i32 {
                        if ((GetBattlerSide(battler)) as i32) == 0i32 {
                            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(74))
                            .write(2u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 23i32 {
                        {
                            i = 0i32;
                            'l3: loop {
                                if !(i < 8i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        < 6i32
                                    {
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(24))
                                        .cast::<i8>())
                                        .wrapping_offset((i) as isize))
                                        .write(6i8);
                                        effect = 5u8;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((effect) as i32) != 0i32 {
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                .write(battler);
                            ((&raw mut gPotentialItemEffectBattler).cast::<u8>()).write(battler);
                            ((&raw mut gActiveBattler).cast::<u8>()).write({
                                let __v3 = battler;
                                ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v3);
                                __v3
                            });
                            BattleScriptExecute((&raw mut BattleScript_WhiteHerbEnd2).cast::<u8>());
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read())
                    != 0
                {
                    'l5: {
                        let __sw4 = ((battlerHoldEffect) as i32);
                        if __sw4 == 1i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    2i32,
                                ))
                                && (!((moveTurn) != 0))
                            {
                                ((&raw mut gBattleMoveDamage).cast::<i32>())
                                    .write(((battlerHoldEffectParam) as i32));
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(((battlerHoldEffectParam) as i32))
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p5 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p5).write(((__p5).read()).wrapping_mul((-1i32)));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_ItemHealHP_RemoveItem).cast::<u8>(),
                                );
                                effect = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 7i32 {
                            if !((moveTurn) != 0) {
                                let mut mon: *mut u8 = core::ptr::null_mut();
                                let mut ppBonuses: u8 = 0u8;
                                let mut r#move: u16 = 0u16;
                                if ((GetBattlerSide(battler)) as i32) == 0i32 {
                                    mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((battler) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    );
                                } else {
                                    mon = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((battler) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    );
                                }
                                {
                                    i = 0i32;
                                    'l6: loop {
                                        if !(i < 4i32) {
                                            break 'l6;
                                        }
                                        'l7: {
                                            r#move = ((GetMonData2(mon, (13i32).wrapping_add(i)))
                                                as u16);
                                            changedPP =
                                                ((GetMonData2(mon, (17i32).wrapping_add(i))) as u8);
                                            ppBonuses = ((GetMonData2(mon, 21i32)) as u8);
                                            if ((r#move) != 0) && (((changedPP) as i32) == 0i32) {
                                                break 'l6;
                                            }
                                        }
                                        i = (i).wrapping_add(1);
                                    }
                                }
                                if i != 4i32 {
                                    let mut maxPP: u8 =
                                        CalculatePPWithBonus(r#move, ppBonuses, ((i) as u8));
                                    if ((changedPP) as i32)
                                        .wrapping_add(((battlerHoldEffectParam) as i32))
                                        > ((maxPP) as i32)
                                    {
                                        changedPP = maxPP;
                                    } else {
                                        changedPP = ((((changedPP) as i32)
                                            .wrapping_add(((battlerHoldEffectParam) as i32)))
                                            as u8);
                                    }
                                    {
                                        ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(1))
                                        .write(2u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(2))
                                        .write(((((r#move) as i32) & 255i32) as u8));
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write((((((r#move) as i32) & 65280i32) >> 8) as u8));
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(4))
                                        .write(255u8);
                                    }
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryPPHealEnd2).cast::<u8>(),
                                    );
                                    BtlController_EmitSetMonData(
                                        0u8,
                                        (((i).wrapping_add(9i32)) as u8),
                                        0u8,
                                        1u8,
                                        &raw mut changedPP,
                                    );
                                    MarkBattlerForControllerExec(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    );
                                    effect = 3u8;
                                }
                            }
                            break 'l5;
                        }
                        if __sw4 == 23i32 {
                            {
                                i = 0i32;
                                'l8: loop {
                                    if !(i < 8i32) {
                                        break 'l8;
                                    }
                                    'l9: {
                                        if ((((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(24))
                                        .cast::<i8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            < 6i32
                                        {
                                            ((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((battler) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(24))
                                            .cast::<i8>())
                                            .wrapping_offset((i) as isize))
                                            .write(6i8);
                                            effect = 5u8;
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            if ((effect) as i32) != 0i32 {
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(battler);
                                ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
                                    .write(battler);
                                ((&raw mut gActiveBattler).cast::<u8>()).write({
                                    let __v6 = battler;
                                    ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v6);
                                    __v6
                                });
                                BattleScriptExecute(
                                    (&raw mut BattleScript_WhiteHerbEnd2).cast::<u8>(),
                                );
                            }
                            break 'l5;
                        }
                        if __sw4 == 43i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                < ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32))
                                && (!((moveTurn) != 0))
                            {
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        16i32,
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).read(),
                                    )
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p7 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p7).write(((__p7).read()).wrapping_mul((-1i32)));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_ItemHealHP_End2).cast::<u8>(),
                                );
                                effect = 4u8;
                                RecordItemEffectBattle(battler, battlerHoldEffect);
                            }
                            break 'l5;
                        }
                        if __sw4 == 10i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    2i32,
                                ))
                                && (!((moveTurn) != 0))
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(8u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        ((battlerHoldEffectParam) as i32),
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).read(),
                                    )
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p8 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p8).write(((__p8).read()).wrapping_mul((-1i32)));
                                if ((GetFlavorRelationByPersonality(
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(72)
                                    .cast::<u32>())
                                    .read(),
                                    0u8,
                                )) as i32)
                                    < 0i32
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryConfuseHealEnd2).cast::<u8>(),
                                    );
                                } else {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_ItemHealHP_RemoveItem).cast::<u8>(),
                                    );
                                }
                                effect = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 11i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    2i32,
                                ))
                                && (!((moveTurn) != 0))
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(8u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(1u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        ((battlerHoldEffectParam) as i32),
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).read(),
                                    )
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p9 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p9).write(((__p9).read()).wrapping_mul((-1i32)));
                                if ((GetFlavorRelationByPersonality(
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(72)
                                    .cast::<u32>())
                                    .read(),
                                    1u8,
                                )) as i32)
                                    < 0i32
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryConfuseHealEnd2).cast::<u8>(),
                                    );
                                } else {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_ItemHealHP_RemoveItem).cast::<u8>(),
                                    );
                                }
                                effect = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 12i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    2i32,
                                ))
                                && (!((moveTurn) != 0))
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(8u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(2u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        ((battlerHoldEffectParam) as i32),
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).read(),
                                    )
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p10 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p10).write(((__p10).read()).wrapping_mul((-1i32)));
                                if ((GetFlavorRelationByPersonality(
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(72)
                                    .cast::<u32>())
                                    .read(),
                                    2u8,
                                )) as i32)
                                    < 0i32
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryConfuseHealEnd2).cast::<u8>(),
                                    );
                                } else {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_ItemHealHP_RemoveItem).cast::<u8>(),
                                    );
                                }
                                effect = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 13i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    2i32,
                                ))
                                && (!((moveTurn) != 0))
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(8u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(3u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        ((battlerHoldEffectParam) as i32),
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).read(),
                                    )
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p11 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p11).write(((__p11).read()).wrapping_mul((-1i32)));
                                if ((GetFlavorRelationByPersonality(
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(72)
                                    .cast::<u32>())
                                    .read(),
                                    3u8,
                                )) as i32)
                                    < 0i32
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryConfuseHealEnd2).cast::<u8>(),
                                    );
                                } else {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_ItemHealHP_RemoveItem).cast::<u8>(),
                                    );
                                }
                                effect = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 14i32 {
                            if (((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    2i32,
                                ))
                                && (!((moveTurn) != 0))
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(8u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(4u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        ((battlerHoldEffectParam) as i32),
                                    ),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(1i32);
                                }
                                if ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        ((&raw mut gBattleMoveDamage).cast::<i32>()).read(),
                                    )
                                    > ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(40)
                                                .cast::<u16>())
                                                .read())
                                                    as i32),
                                            ),
                                    );
                                }
                                let __p12 = (&raw mut gBattleMoveDamage).cast::<i32>();
                                (__p12).write(((__p12).read()).wrapping_mul((-1i32)));
                                if ((GetFlavorRelationByPersonality(
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(72)
                                    .cast::<u32>())
                                    .read(),
                                    4u8,
                                )) as i32)
                                    < 0i32
                                {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryConfuseHealEnd2).cast::<u8>(),
                                    );
                                } else {
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_ItemHealHP_RemoveItem).cast::<u8>(),
                                    );
                                }
                                effect = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 15i32 {
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    ((battlerHoldEffectParam) as i32),
                                ))
                                && (!((moveTurn) != 0)))
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    < 12i32)
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(1u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                {
                                    ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(1))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(2))
                                        .write(210u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(3))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(4))
                                        .write(255u8);
                                }
                                ((&raw mut gEffectBattler).cast::<u8>()).write(battler);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(26))
                                    .write(17u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(15u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(0u8);
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryStatRaiseEnd2).cast::<u8>(),
                                );
                                effect = 5u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 16i32 {
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    ((battlerHoldEffectParam) as i32),
                                ))
                                && (!((moveTurn) != 0)))
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    < 12i32)
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(2u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gEffectBattler).cast::<u8>()).write(battler);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(26))
                                    .write(18u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(16u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(0u8);
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryStatRaiseEnd2).cast::<u8>(),
                                );
                                effect = 5u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 17i32 {
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    ((battlerHoldEffectParam) as i32),
                                ))
                                && (!((moveTurn) != 0)))
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(3))
                                .read()) as i32)
                                    < 12i32)
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(3u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gEffectBattler).cast::<u8>()).write(battler);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(26))
                                    .write(19u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(17u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(0u8);
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryStatRaiseEnd2).cast::<u8>(),
                                );
                                effect = 5u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 18i32 {
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    ((battlerHoldEffectParam) as i32),
                                ))
                                && (!((moveTurn) != 0)))
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(4))
                                .read()) as i32)
                                    < 12i32)
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(4u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gEffectBattler).cast::<u8>()).write(battler);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(26))
                                    .write(20u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(18u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(0u8);
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryStatRaiseEnd2).cast::<u8>(),
                                );
                                effect = 5u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 19i32 {
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    ((battlerHoldEffectParam) as i32),
                                ))
                                && (!((moveTurn) != 0)))
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(5))
                                .read()) as i32)
                                    < 12i32)
                            {
                                {
                                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
                                        .write(5u8);
                                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3))
                                        .write(255u8);
                                }
                                ((&raw mut gEffectBattler).cast::<u8>()).write(battler);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(26))
                                    .write(21u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                    .write(19u8);
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                    .write(0u8);
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryStatRaiseEnd2).cast::<u8>(),
                                );
                                effect = 5u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 20i32 {
                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                <= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    ((battlerHoldEffectParam) as i32),
                                ))
                                && (!((moveTurn) != 0)))
                                && (!((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 1048576u32)
                                    != 0))
                            {
                                let __p13 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p13).write(((__p13).read() | 1048576u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryFocusEnergyEnd2).cast::<u8>(),
                                );
                                effect = 2u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 21i32 {
                            if (!((moveTurn) != 0))
                                && (((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    <= crate::c::div_i32(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                        .read()) as i32),
                                        ((battlerHoldEffectParam) as i32),
                                    ))
                            {
                                {
                                    i = 0i32;
                                    'l10: loop {
                                        if !(i < 5i32) {
                                            break 'l10;
                                        }
                                        'l11: {
                                            if ((((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(
                                                    ((battler) as i32) as isize * 88,
                                                ))
                                            .wrapping_add(24))
                                            .cast::<i8>())
                                            .wrapping_offset(((1i32).wrapping_add(i)) as isize))
                                            .read())
                                                as i32)
                                                < 12i32
                                            {
                                                break 'l10;
                                            }
                                        }
                                        i = (i).wrapping_add(1);
                                    }
                                }
                                if i != 5i32 {
                                    'l12: loop {
                                        'l13: {
                                            i = crate::c::rem_i32(((Random()) as i32), 5i32);
                                        }
                                        if !(((((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(24))
                                        .cast::<i8>())
                                        .wrapping_offset(((1i32).wrapping_add(i)) as isize))
                                        .read())
                                            as i32)
                                            == 12i32)
                                        {
                                            break 'l12;
                                        }
                                    }
                                    {
                                        ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(1))
                                        .write(5u8);
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(2))
                                        .write((((i).wrapping_add(1i32)) as u8));
                                        (((&raw mut gBattleTextBuff1).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write(255u8);
                                    }
                                    ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(1))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(2))
                                        .write(209u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(3))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(4))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(5))
                                        .write(210u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(6))
                                        .write(0u8);
                                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(7))
                                        .write(255u8);
                                    ((&raw mut gEffectBattler).cast::<u8>()).write(battler);
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(26))
                                        .write(
                                            (((((i).wrapping_add(1i32)).wrapping_add(32i32))
                                                .wrapping_add(0i32))
                                                as u8),
                                        );
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(16))
                                        .write(
                                            (((38i32).wrapping_add((i).wrapping_add(1i32))) as u8),
                                        );
                                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(17))
                                        .write(0u8);
                                    BattleScriptExecute(
                                        (&raw mut BattleScript_BerryStatRaiseEnd2).cast::<u8>(),
                                    );
                                    effect = 5u8;
                                }
                            }
                            break 'l5;
                        }
                        if __sw4 == 2i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 64u32)
                                != 0
                            {
                                let __p14 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p14).write(((__p14).read() & 4294967231u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCurePrlzEnd2).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 4i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 136u32)
                                != 0
                            {
                                let __p15 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p15).write(((__p15).read() & 4294963319u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCurePsnEnd2).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 5i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 16u32)
                                != 0
                            {
                                let __p16 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p16).write(((__p16).read() & 4294967279u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCureBrnEnd2).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 6i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 32u32)
                                != 0
                            {
                                let __p17 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p17).write(((__p17).read() & 4294967263u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCureFrzEnd2).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 3i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 7u32)
                                != 0
                            {
                                let __p18 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>();
                                (__p18).write(((__p18).read() & 4294967288u32));
                                let __p19 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p19).write(((__p19).read() & 4160749567u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCureSlpEnd2).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 8i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 7u32)
                                != 0
                            {
                                let __p20 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p20).write(((__p20).read() & 4294967288u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCureConfusionEnd2).cast::<u8>(),
                                );
                                effect = 2u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 9i32 {
                            if ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(76)
                            .cast::<u32>())
                            .read()
                                & 255u32)
                                != 0)
                                || ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0)
                            {
                                i = 0i32;
                                if (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 136u32)
                                    != 0
                                {
                                    StringCopy(
                                        (&raw mut gBattleTextBuff1).cast::<u8>(),
                                        (&raw mut gStatusConditionString_PoisonJpn).cast::<u8>(),
                                    );
                                    i = (i).wrapping_add(1);
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0
                                {
                                    let __p21 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(80)
                                    .cast::<u32>();
                                    (__p21).write(((__p21).read() & 4160749567u32));
                                    StringCopy(
                                        (&raw mut gBattleTextBuff1).cast::<u8>(),
                                        (&raw mut gStatusConditionString_SleepJpn).cast::<u8>(),
                                    );
                                    i = (i).wrapping_add(1);
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 64u32)
                                    != 0
                                {
                                    StringCopy(
                                        (&raw mut gBattleTextBuff1).cast::<u8>(),
                                        (&raw mut gStatusConditionString_ParalysisJpn).cast::<u8>(),
                                    );
                                    i = (i).wrapping_add(1);
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 16u32)
                                    != 0
                                {
                                    StringCopy(
                                        (&raw mut gBattleTextBuff1).cast::<u8>(),
                                        (&raw mut gStatusConditionString_BurnJpn).cast::<u8>(),
                                    );
                                    i = (i).wrapping_add(1);
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 32u32)
                                    != 0
                                {
                                    StringCopy(
                                        (&raw mut gBattleTextBuff1).cast::<u8>(),
                                        (&raw mut gStatusConditionString_IceJpn).cast::<u8>(),
                                    );
                                    i = (i).wrapping_add(1);
                                }
                                if (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0
                                {
                                    StringCopy(
                                        (&raw mut gBattleTextBuff1).cast::<u8>(),
                                        (&raw mut gStatusConditionString_ConfusionJpn).cast::<u8>(),
                                    );
                                    i = (i).wrapping_add(1);
                                }
                                if i <= 1i32 {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(0u8);
                                } else {
                                    (((&raw mut gBattleCommunication).cast::<u8>())
                                        .wrapping_offset(5))
                                    .write(1u8);
                                }
                                ((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .write(0u32);
                                let __p22 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p22).write(((__p22).read() & 4294967288u32));
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCureChosenStatusEnd2).cast::<u8>(),
                                );
                                effect = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw4 == 28i32 {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 983040u32)
                                != 0
                            {
                                let __p23 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p23).write(((__p23).read() & 4293984255u32));
                                StringCopy(
                                    (&raw mut gBattleTextBuff1).cast::<u8>(),
                                    (&raw mut gStatusConditionString_LoveJpn).cast::<u8>(),
                                );
                                BattleScriptExecute(
                                    (&raw mut BattleScript_BerryCureChosenStatusEnd2).cast::<u8>(),
                                );
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5))
                                    .write(0u8);
                                effect = 2u8;
                            }
                            break 'l5;
                        }
                    }
                    if ((effect) as i32) != 0i32 {
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                            .write(battler);
                        ((&raw mut gPotentialItemEffectBattler).cast::<u8>()).write(battler);
                        ((&raw mut gActiveBattler).cast::<u8>()).write({
                            let __v24 = battler;
                            ((&raw mut gBattlerAttacker).cast::<u8>()).write(__v24);
                            __v24
                        });
                        'l14: {
                            let __sw25 = ((effect) as i32);
                            if __sw25 == 1i32 {
                                BtlController_EmitSetMonData(
                                    0u8,
                                    40u8,
                                    0u8,
                                    4u8,
                                    ((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .cast::<u8>(),
                                );
                                MarkBattlerForControllerExec(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                );
                                break 'l14;
                            }
                            if __sw25 == 3i32 {
                                if (!((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 2097152u32)
                                    != 0))
                                    && (!((((crate::c::bf_read(
                                        (((&raw mut gDisableStructs).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 28))
                                        .wrapping_add(24),
                                        4,
                                        4,
                                        false,
                                    ) as u8) as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset((i) as isize))
                                        .read())
                                        != 0))
                                {
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(36))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(changedPP);
                                }
                                break 'l14;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    battler = 0u8;
                    'l15: loop {
                        if !(((battler) as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                        {
                            break 'l15;
                        }
                        'l16: {
                            ((&raw mut gLastUsedItem).cast::<u16>()).write(
                                ((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(46)
                                .cast::<u16>())
                                .read(),
                            );
                            if ((((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 88))
                            .wrapping_add(46)
                            .cast::<u16>())
                            .read()) as i32)
                                == 175i32
                            {
                                battlerHoldEffect = ((((&raw mut gEnigmaBerries).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 28))
                                .wrapping_add(7))
                                .read();
                                battlerHoldEffectParam = ((((&raw mut gEnigmaBerries)
                                    .cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 28))
                                .wrapping_add(26))
                                .read();
                            } else {
                                battlerHoldEffect = GetItemHoldEffect(
                                    ((&raw mut gLastUsedItem).cast::<u16>()).read(),
                                );
                                battlerHoldEffectParam = GetItemHoldEffectParam(
                                    ((&raw mut gLastUsedItem).cast::<u16>()).read(),
                                );
                            }
                            'l17: {
                                let __sw26 = ((battlerHoldEffect) as i32);
                                if __sw26 == 2i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 64u32)
                                        != 0
                                    {
                                        let __p27 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>();
                                        (__p27).write(((__p27).read() & 4294967231u32));
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureParRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 1u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 4i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 136u32)
                                        != 0
                                    {
                                        let __p28 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>();
                                        (__p28).write(((__p28).read() & 4294963319u32));
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCurePsnRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 1u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 5i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 16u32)
                                        != 0
                                    {
                                        let __p29 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>();
                                        (__p29).write(((__p29).read() & 4294967279u32));
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureBrnRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 1u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 6i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 32u32)
                                        != 0
                                    {
                                        let __p30 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>();
                                        (__p30).write(((__p30).read() & 4294967263u32));
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureFrzRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 1u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 3i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 7u32)
                                        != 0
                                    {
                                        let __p31 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>();
                                        (__p31).write(((__p31).read() & 4294967288u32));
                                        let __p32 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>();
                                        (__p32).write(((__p32).read() & 4160749567u32));
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureSlpRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 1u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 8i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 7u32)
                                        != 0
                                    {
                                        let __p33 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>();
                                        (__p33).write(((__p33).read() & 4294967288u32));
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureConfusionRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 2u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 28i32 {
                                    if (((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 983040u32)
                                        != 0
                                    {
                                        let __p34 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>();
                                        (__p34).write(((__p34).read() & 4293984255u32));
                                        StringCopy(
                                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                                            (&raw mut gStatusConditionString_LoveJpn).cast::<u8>(),
                                        );
                                        BattleScriptPushCursor();
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(0u8);
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureChosenStatusRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 2u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 9i32 {
                                    if ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 255u32)
                                        != 0)
                                        || ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0)
                                    {
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 136u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_PoisonJpn)
                                                    .cast::<u8>(),
                                            );
                                        }
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0
                                        {
                                            let __p35 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset(((battler) as i32) as isize * 88))
                                            .wrapping_add(80)
                                            .cast::<u32>();
                                            (__p35).write(((__p35).read() & 4160749567u32));
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_SleepJpn)
                                                    .cast::<u8>(),
                                            );
                                        }
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 64u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_ParalysisJpn)
                                                    .cast::<u8>(),
                                            );
                                        }
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 16u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_BurnJpn)
                                                    .cast::<u8>(),
                                            );
                                        }
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .read()
                                            & 32u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_IceJpn)
                                                    .cast::<u8>(),
                                            );
                                        }
                                        if (((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)
                                            != 0
                                        {
                                            StringCopy(
                                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                                (&raw mut gStatusConditionString_ConfusionJpn)
                                                    .cast::<u8>(),
                                            );
                                        }
                                        ((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(76)
                                        .cast::<u32>())
                                        .write(0u32);
                                        let __p36 = (((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize * 88))
                                        .wrapping_add(80)
                                        .cast::<u32>();
                                        (__p36).write(((__p36).read() & 4294967288u32));
                                        BattleScriptPushCursor();
                                        (((&raw mut gBattleCommunication).cast::<u8>())
                                            .wrapping_offset(5))
                                        .write(0u8);
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_BerryCureChosenStatusRet)
                                                    .cast::<u8>(),
                                            );
                                        effect = 1u8;
                                    }
                                    break 'l17;
                                }
                                if __sw26 == 23i32 {
                                    {
                                        i = 0i32;
                                        'l18: loop {
                                            if !(i < 8i32) {
                                                break 'l18;
                                            }
                                            'l19: {
                                                if ((((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((battler) as i32) as isize * 88,
                                                    ))
                                                .wrapping_add(24))
                                                .cast::<i8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    < 6i32
                                                {
                                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((battler) as i32) as isize * 88,
                                                        ))
                                                    .wrapping_add(24))
                                                    .cast::<i8>())
                                                    .wrapping_offset((i) as isize))
                                                    .write(6i8);
                                                    effect = 5u8;
                                                }
                                            }
                                            i = (i).wrapping_add(1);
                                        }
                                    }
                                    if ((effect) as i32) != 0i32 {
                                        (((&raw mut gBattleScripting).cast::<u8>())
                                            .wrapping_add(23))
                                        .write(battler);
                                        ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
                                            .write(battler);
                                        BattleScriptPushCursor();
                                        ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                            .write(
                                                (&raw mut BattleScript_WhiteHerbRet).cast::<u8>(),
                                            );
                                        return effect;
                                    }
                                    break 'l17;
                                }
                            }
                            if ((effect) as i32) != 0i32 {
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(battler);
                                ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
                                    .write(battler);
                                ((&raw mut gActiveBattler).cast::<u8>()).write(battler);
                                BtlController_EmitSetMonData(
                                    0u8,
                                    40u8,
                                    0u8,
                                    4u8,
                                    ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .cast::<u8>(),
                                );
                                MarkBattlerForControllerExec(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                );
                                break 'l15;
                            }
                        }
                        battler = (battler).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((&raw mut gBattleMoveDamage).cast::<i32>()).read()) != 0 {
                    'l20: {
                        let __sw37 = ((atkHoldEffect) as i32);
                        if __sw37 == 30i32 {
                            if ((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && ((((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(8)
                                .cast::<i32>())
                                .read()
                                    != 0i32)
                                    || (((((&raw mut gSpecialStatuses).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 20,
                                        ))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .read()
                                        != 0i32)))
                                && (crate::c::rem_i32(((Random()) as i32), 100i32)
                                    < ((atkHoldEffectParam) as i32)))
                                && ((((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 12,
                                ))
                                .wrapping_add(8))
                                .read()) as i32)
                                    & 32i32)
                                    != 0))
                                && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read())
                                    != 0)
                            {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(3))
                                    .write(8u8);
                                BattleScriptPushCursor();
                                SetMoveEffect(0u8, 0u8);
                                BattleScriptPop();
                            }
                            break 'l20;
                        }
                        if __sw37 == 62i32 {
                            if (((((!((((((&raw mut gMoveResultFlags).cast::<u8>()).read())
                                as i32)
                                & 41i32)
                                != 0))
                                && (((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(4)
                                .cast::<i32>())
                                .read()
                                    != 0i32))
                                && (((((&raw mut gSpecialStatuses).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                .wrapping_add(4)
                                .cast::<i32>())
                                .read()
                                    != 65535i32))
                                && (((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                    != ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32)))
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(40)
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32)
                            {
                                ((&raw mut gLastUsedItem).cast::<u16>()).write(atkItem);
                                ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
                                    .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                                (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(
                                    (crate::c::div_i32(
                                        ((((&raw mut gSpecialStatuses).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerTarget).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 20,
                                            ))
                                        .wrapping_add(4)
                                        .cast::<i32>())
                                        .read(),
                                        ((atkHoldEffectParam) as i32),
                                    ))
                                    .wrapping_mul((-1i32)),
                                );
                                if ((&raw mut gBattleMoveDamage).cast::<i32>()).read() == 0i32 {
                                    ((&raw mut gBattleMoveDamage).cast::<i32>()).write((-1i32));
                                }
                                ((((&raw mut gSpecialStatuses).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 20,
                                ))
                                .wrapping_add(4)
                                .cast::<i32>())
                                .write(0i32);
                                BattleScriptPushCursor();
                                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                                    .write((&raw mut BattleScript_ItemHealHP_Ret).cast::<u8>());
                                effect = (effect).wrapping_add(1);
                            }
                            break 'l20;
                        }
                    }
                }
                break 'l1;
            }
        }
        return effect;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearFuryCutterDestinyBondGrudge(battler: u8) {
    unsafe {
        let mut battler = battler;
        ((((&raw mut gDisableStructs).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 28))
        .wrapping_add(16))
        .write(0u8);
        let __p1 = (((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(80)
        .cast::<u32>();
        (__p1).write(((__p1).read() & 4261412863u32));
        let __p2 = (((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((battler) as i32) as isize);
        (__p2).write(((__p2).read() & 4294950911u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleAction_RunBattleScript() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags).cast::<u32>()).read() == 0u32 {
            (((((&raw mut gBattleScriptingCommandsTable)
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(
                (((((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>()).read()).read()) as i32)
                    as isize,
            ))
            .read())
            .unwrap_unchecked()();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMoveTarget(r#move: u16, setTarget: u8) -> u8 {
    unsafe {
        let mut r#move = r#move;
        let mut setTarget = setTarget;
        let mut targetBattler: u8 = 0u8;
        let mut moveTarget: u8 = 0u8;
        let mut side: u8 = 0u8;
        if ((setTarget) as i32) != 0i32 {
            moveTarget = ((((setTarget) as i32).wrapping_sub(1i32)) as u8);
        } else {
            moveTarget = ((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(6))
            .read();
        }
        'l1: {
            let __sw1 = ((moveTarget) as i32);
            if __sw1 == 0i32 {
                side = ((((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                    as i32)
                    ^ 1i32) as u8);
                if ((((((&raw mut gSideTimers).cast::<u8>())
                    .wrapping_offset(((side) as i32) as isize * 12))
                .wrapping_add(8))
                .read())
                    != 0)
                    && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gSideTimers).cast::<u8>())
                            .wrapping_offset(((side) as i32) as isize * 12))
                        .wrapping_add(9))
                        .read()) as i32) as isize
                            * 88,
                    ))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read())
                        != 0)
                {
                    targetBattler = ((((&raw mut gSideTimers).cast::<u8>())
                        .wrapping_offset(((side) as i32) as isize * 12))
                    .wrapping_add(9))
                    .read();
                } else {
                    side = GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                    'l2: loop {
                        'l3: {
                            targetBattler = ((crate::c::rem_i32(
                                ((Random()) as i32),
                                ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32),
                            )) as u8);
                        }
                        if !(((((targetBattler) as i32)
                            == ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32))
                            || (((side) as i32) == ((GetBattlerSide(targetBattler)) as i32)))
                            || ((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(((targetBattler) as i32) as isize))
                                .read())
                                != 0))
                        {
                            break 'l2;
                        }
                    }
                    if ((((((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 12))
                    .wrapping_add(2))
                    .read()) as i32)
                        == 13i32)
                        && ((AbilityBattleEffects(
                            16u8,
                            ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                            31u8,
                            0u8,
                            0u16,
                        )) != 0))
                        && (((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((targetBattler) as i32) as isize * 88))
                        .wrapping_add(32))
                        .read()) as i32)
                            != 31i32)
                    {
                        targetBattler = ((((targetBattler) as i32) ^ 2i32) as u8);
                        RecordAbilityBattle(
                            targetBattler,
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((targetBattler) as i32) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                        crate::c::bf_write(
                            (((&raw mut gSpecialStatuses).cast::<u8>())
                                .wrapping_offset(((targetBattler) as i32) as isize * 20))
                            .wrapping_add(0),
                            1,
                            1,
                            (1u32) as i32,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 8i32 || __sw1 == 32i32 || __sw1 == 64i32 {
                targetBattler = GetBattlerAtPosition(
                    (((((GetBattlerPosition(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                        as i32)
                        & 1i32)
                        ^ 1i32) as u8),
                );
                if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset(((targetBattler) as i32) as isize))
                    .read())
                    != 0
                {
                    targetBattler = ((((targetBattler) as i32) ^ 2i32) as u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                side = ((((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                    as i32)
                    ^ 1i32) as u8);
                if ((((((&raw mut gSideTimers).cast::<u8>())
                    .wrapping_offset(((side) as i32) as isize * 12))
                .wrapping_add(8))
                .read())
                    != 0)
                    && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gSideTimers).cast::<u8>())
                            .wrapping_offset(((side) as i32) as isize * 12))
                        .wrapping_add(9))
                        .read()) as i32) as isize
                            * 88,
                    ))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read())
                        != 0)
                {
                    targetBattler = ((((&raw mut gSideTimers).cast::<u8>())
                        .wrapping_offset(((side) as i32) as isize * 12))
                    .wrapping_add(9))
                    .read();
                } else {
                    if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                        && ((((moveTarget) as i32) & 4i32) != 0)
                    {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            if (((Random()) as i32) & 1i32) != 0 {
                                targetBattler = GetBattlerAtPosition(1u8);
                            } else {
                                targetBattler = GetBattlerAtPosition(3u8);
                            }
                        } else {
                            if (((Random()) as i32) & 1i32) != 0 {
                                targetBattler = GetBattlerAtPosition(0u8);
                            } else {
                                targetBattler = GetBattlerAtPosition(2u8);
                            }
                        }
                        if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(((targetBattler) as i32) as isize))
                            .read())
                            != 0
                        {
                            targetBattler = ((((targetBattler) as i32) ^ 2i32) as u8);
                        }
                    } else {
                        targetBattler = GetBattlerAtPosition(
                            (((((GetBattlerPosition(
                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                            )) as i32)
                                & 1i32)
                                ^ 1i32) as u8),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 16i32 {
                targetBattler = ((&raw mut gBattlerAttacker).cast::<u8>()).read();
                break 'l1;
            }
        }
        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(12)).cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
        .write(targetBattler);
        return targetBattler;
    }
}
pub(crate) unsafe extern "C" fn IsBattlerModernFatefulEncounter(battler: u8) -> u32 {
    unsafe {
        let mut battler = battler;
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            return 1u32;
        }
        if (GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 100,
            ),
            11i32,
            core::ptr::null_mut(),
        ) != 410u32)
            && (GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                11i32,
                core::ptr::null_mut(),
            ) != 151u32)
        {
            return 1u32;
        }
        return GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 100,
            ),
            80i32,
            core::ptr::null_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonDisobedient() -> u8 {
    unsafe {
        let mut rnd: i32 = 0i32;
        let mut calc: i32 = 0i32;
        let mut obedienceLevel: u8 = 0u8;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
            return 0u8;
        }
        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            return 0u8;
        }
        if (IsBattlerModernFatefulEncounter(((&raw mut gBattlerAttacker).cast::<u8>()).read())) != 0
        {
            if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0)
                && (((GetBattlerPosition(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                    as i32)
                    == 2i32)
            {
                return 0u8;
            }
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                return 0u8;
            }
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                return 0u8;
            }
            if !((IsOtherTrainer(
                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(84)
                .cast::<u32>())
                .read(),
                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(60))
                .cast::<u8>(),
            )) != 0)
            {
                return 0u8;
            }
            if (FlagGet(2158u16)) != 0 {
                return 0u8;
            }
            obedienceLevel = 10u8;
            if (FlagGet(2152u16)) != 0 {
                obedienceLevel = 30u8;
            }
            if (FlagGet(2154u16)) != 0 {
                obedienceLevel = 50u8;
            }
            if (FlagGet(2156u16)) != 0 {
                obedienceLevel = 70u8;
            }
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(42))
        .read()) as i32)
            <= ((obedienceLevel) as i32)
        {
            return 0u8;
        }
        rnd = (((Random()) as i32) & 255i32);
        calc = ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(42))
        .read()) as i32)
            .wrapping_add(((obedienceLevel) as i32)))
        .wrapping_mul(rnd)
            >> 8);
        if calc < ((obedienceLevel) as i32) {
            return 0u8;
        }
        if ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) == 99i32 {
            let __p1 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
            ))
            .wrapping_add(80)
            .cast::<u32>();
            (__p1).write(((__p1).read() & 4286578687u32));
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(76)
        .cast::<u32>())
        .read()
            & 7u32)
            != 0)
            && ((((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) == 173i32)
                || (((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) == 214i32))
        {
            ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                .write((&raw mut BattleScript_IgnoresWhileAsleep).cast::<u8>());
            return 1u8;
        }
        rnd = (((Random()) as i32) & 255i32);
        calc = ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(42))
        .read()) as i32)
            .wrapping_add(((obedienceLevel) as i32)))
        .wrapping_mul(rnd)
            >> 8);
        if calc < ((obedienceLevel) as i32) {
            calc = ((CheckMoveLimitations(
                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                ((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as u8),
                255u8,
            )) as i32);
            if calc == 15i32 {
                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(
                    ((if (0i32) != 0 {
                        crate::c::rem_i32(((Random()) as i32), 4i32)
                    } else {
                        (((Random()) as i32) & 3i32)
                    }) as u8),
                );
                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                    .write((&raw mut BattleScript_MoveUsedLoafingAround).cast::<u8>());
                return 1u8;
            } else {
                'l1: loop {
                    'l2: {
                        ((&raw mut gCurrMovePos).cast::<u8>()).write({
                            let __v2 = ((if (0i32) != 0 {
                                crate::c::rem_i32(((Random()) as i32), 4i32)
                            } else {
                                (((Random()) as i32) & 3i32)
                            }) as u8);
                            ((&raw mut gChosenMovePos).cast::<u8>()).write(__v2);
                            __v2
                        });
                    }
                    if !((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                        ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()
                        & ((calc) as u32))
                        != 0)
                    {
                        break 'l1;
                    }
                }
                ((&raw mut gCalledMove).cast::<u16>()).write(
                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gCurrMovePos).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                    .write((&raw mut BattleScript_IgnoresAndUsesRandomMove).cast::<u8>());
                ((&raw mut gBattlerTarget).cast::<u8>()).write(GetMoveTarget(
                    ((&raw mut gCalledMove).cast::<u16>()).read(),
                    0u8,
                ));
                let __p3 = (&raw mut gHitMarker).cast::<u32>();
                (__p3).write(((__p3).read() | 2097152u32));
                return 2u8;
            }
        } else {
            obedienceLevel = ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
            ))
            .wrapping_add(42))
            .read()) as i32)
                .wrapping_sub(((obedienceLevel) as i32))) as u8);
            calc = (((Random()) as i32) & 255i32);
            if (((calc < ((obedienceLevel) as i32))
                && (!((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(76)
                .cast::<u32>())
                .read()
                    & 255u32)
                    != 0)))
                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(32))
                .read()) as i32)
                    != 72i32))
                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(32))
                .read()) as i32)
                    != 15i32)
            {
                let mut i: i32 = 0i32;
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                            break 'l3;
                        }
                        'l4: {
                            if (((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 112u32)
                                != 0
                            {
                                break 'l3;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32) {
                    ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                        .write((&raw mut BattleScript_IgnoresAndFallsAsleep).cast::<u8>());
                    return 1u8;
                }
            }
            calc = (calc).wrapping_sub(((obedienceLevel) as i32));
            if calc < ((obedienceLevel) as i32) {
                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(CalculateBaseDamage(
                    ((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                    ),
                    ((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize * 88,
                    ),
                    1u32,
                    0u16,
                    40u16,
                    0u8,
                    ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                    ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                ));
                ((&raw mut gBattlerTarget).cast::<u8>())
                    .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                    .write((&raw mut BattleScript_IgnoresAndHitsItself).cast::<u8>());
                let __p4 = (&raw mut gHitMarker).cast::<u32>();
                (__p4).write(((__p4).read() | 524288u32));
                return 2u8;
            } else {
                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(5)).write(
                    ((if (0i32) != 0 {
                        crate::c::rem_i32(((Random()) as i32), 4i32)
                    } else {
                        (((Random()) as i32) & 3i32)
                    }) as u8),
                );
                ((&raw mut gBattlescriptCurrInstr).cast::<*mut u8>())
                    .write((&raw mut BattleScript_MoveUsedLoafingAround).cast::<u8>());
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
