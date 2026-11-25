//! Translated from `src/battle_script_commands.c` by tools/rustport/c2rs.py.
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
    clippy::int_plus_one,
    clippy::manual_clamp,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_ai_script_commands::{RecordAbilityBattle, RecordItemEffectBattle};
use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_arena::{
    BattleArena_AddSkillPoints, BattleArena_InitPoints, BattleArena_ShowJudgmentWindow,
    DrawArenaRefereeTextBox, EraseArenaRefereeTextBox,
};
use crate::battle_bg::{InitBattleBgsVideo, LoadBattleTextboxAndBackground};
use crate::battle_controllers::{
    BtlController_EmitBallThrowAnim, BtlController_EmitBattleAnimation,
    BtlController_EmitCantSwitch, BtlController_EmitChoosePokemon,
    BtlController_EmitDrawPartyStatusSummary, BtlController_EmitEndLinkBattle,
    BtlController_EmitExpUpdate, BtlController_EmitFaintAnimation, BtlController_EmitFaintingCry,
    BtlController_EmitGetMonData, BtlController_EmitHealthBarUpdate,
    BtlController_EmitHidePartyStatusSummary, BtlController_EmitHitAnimation,
    BtlController_EmitLinkStandbyMsg, BtlController_EmitMoveAnimation,
    BtlController_EmitPlayFanfareOrBGM, BtlController_EmitPlaySE,
    BtlController_EmitPrintSelectionString, BtlController_EmitResetActionMoveSelection,
    BtlController_EmitReturnMonToBall, BtlController_EmitSetMonData,
    BtlController_EmitSpriteInvisibility, BtlController_EmitStatusAnimation,
    BtlController_EmitStatusIconUpdate, BtlController_EmitSwitchInAnim,
    BtlController_EmitTrainerSlide, BtlController_EmitTrainerSlideBack, BtlController_EmitYesNoBox,
};
use crate::battle_gfx_sfx_util::{BattleStopLowHpSound, HandleLowHpMusicChange};
use crate::battle_interface::GetScaledHPFraction;
use crate::battle_main::{
    BattleMainCB2, FaintClearSetData, IsRunningFromBattleImpossible, SwitchInClearSetData,
    SwitchPartyOrder, VBlankCB_Battle, gAbsentBattlerFlags, gActiveBattler, gBattle_BG1_X,
    gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattle_BG3_X, gBattleControllerExecFlags,
    gBattleEnvironment, gBattleMainFunc, gBattleMons, gBattleMoveDamage, gBattleMovePower,
    gBattleOutcome, gBattleResources, gBattleResults, gBattleScripting, gBattleStruct,
    gBattleTypeFlags, gBattleWeather, gBattlerAttacker, gBattlerFainted, gBattlerInMenuId,
    gBattlerTarget, gBattlersCount, gBattlescriptCurrInstr, gBideDmg, gCalledMove, gChosenMove,
    gChosenMovePos, gCritMultiplier, gCurrMovePos, gCurrentActionFuncId, gCurrentMove,
    gCurrentTurnActionNumber, gDisableStructs, gDynamicBasePower, gEffectBattler, gEnigmaBerries,
    gExpShareExp, gHitMarker, gHpDealt, gLastUsedAbility, gLastUsedItem, gLeveledUpInBattle,
    gMoveResultFlags, gMoveToLearn, gMultiHitCounter, gPauseCounterBattle, gPaydayMoney,
    gPotentialItemEffectBattler, gProtectStructs, gSideTimers, gSpecialStatuses, gStatuses3,
    gWishFutureKnock,
};
use crate::battle_main::{
    gActionsByTurnOrder, gBattleBufferB, gBattleCommunication, gBattleTextBuff1, gBattleTextBuff2,
    gBattleTextBuff3, gBattlerByTurnOrder, gBattlerPartyIndexes, gBideTarget,
    gChosenActionByBattler, gDisplayedStringBattle, gLastHitBy, gLastHitByType, gLastLandedMoves,
    gLastMoves, gLastPrintedMoves, gLastResultingMoves, gLockedMoves, gSentPokesToOpponent,
    gSideStatuses,
};
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString,
};
use crate::battle_pike::InBattlePike;
use crate::battle_pyramid::{CurrentBattlePyramidLocation, GetBattlePyramidPickupItemId};
use crate::battle_setup::{gPartnerTrainerId, gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
use crate::battle_util::{
    AbilityBattleEffects, AtkCanceler_UnableToUseMove, BattleScriptPop, BattleScriptPush,
    BattleScriptPushCursor, BattleScriptPushCursorAndCallback, CancelMultiTurnMoves,
    CastformDataTypeChange, CheckMoveLimitations, GetBattlerForBattleScript, GetMoveTarget,
    HasNoMonsToSwitch, IsMonDisobedient, ItemBattleEffects, MarkBattlerForControllerExec,
    PrepareStringBattle, PressurePPLose, PressurePPLoseOnUsingImprison,
    PressurePPLoseOnUsingPerishSong, ResetSentPokesToOpponentValue, TryRunFromBattle,
    UpdateSentPokesToOpponentValue, WasUnableToUseMove,
};
use crate::battle_util2::{
    AdjustFriendshipOnBattleFaint, BattlePalace_TryEscapeStatus, SwitchPartyOrderInGameMulti,
};
use crate::bg::{CopyBgTilemapBufferToVram, IsDma3ManagerBusyWithBgCopy, SetBgAttribute, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, VarGet};
use crate::field_specials::{GetPCBoxToSendMon, ShouldShowBoxWasFullMessage};
use crate::item::{GetItemHoldEffect, GetItemHoldEffectParam};
use crate::load_save::gSaveBlock1Ptr;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayVolumeControl};
use crate::menu_specialized::{
    DrawLevelUpWindowPg1, DrawLevelUpWindowPg2, GetMonLevelUpWindowStats,
};
use crate::money::AddMoney;
use crate::naming_screen::DoNamingScreen;
use crate::overworld::{GetCurrentMapType, IncrementGameStat};
use crate::palette::{BeginFastPaletteFade, BeginNormalPaletteFade, LoadPalette, gPaletteFade};
use crate::party_menu::{GetMonNickname, IsMultiBattle, SwitchPartyOrderLinkMulti};
use crate::pokedex::{DisplayCaughtMonDexPage, GetPokedexHeightWeight, GetSetPokedexFlag};
use crate::pokemon::{
    AdjustFriendship, CalculateBaseDamage, CalculatePlayerPartyCount, CountAliveMonsInBattle,
    GetAbilityBySpecies, GetBattlerMultiplayerId, GetGenderFromSpeciesAndPersonality,
    GetLinkTrainerFlankId, GetMonData2, GetMonData3, GetMonGender, GetNatureFromPersonality,
    GiveMonToPlayer, GiveMoveToBattleMon, HandleSetPokedexFlag, IsHMMove2, IsTradedMon, MonGainEVs,
    MonTryLearningNewMove, PokemonUseItemEffects, RemoveBattleMonPPBonus, RemoveMonPPBonus,
    SetBattleMonMoveSlot, SetMonData, SetMonMoveSlot, SpeciesToNationalPokedexNum, gEnemyParty,
    gPlayerParty, gPlayerPartyCount,
};
use crate::pokemon_icon::{GetMonIconPtr, GetValidMonIconPalettePtr};
use crate::pokemon_storage_system::GetBoxNamePtr;
use crate::pokemon_summary_screen::{GetMoveSlotToReplace, ShowSelectMovePokemonSummaryScreen};
use crate::random::Random;
use crate::recorded_battle::RecordedBattle_SetBattlerAction;
use crate::reshow_battle_screen::ReshowBattleScreenAfterMenu;
use crate::sound::{IsCryFinished, PlayBGM, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::string_util::{StringFill, WriteColorChangeControlCode};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::gTasks;
use crate::text::IsTextPrinterActive;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
use crate::window::{ClearWindowTilemap, CopyWindowToVram, FreeAllWindowBuffers, PutWindowTilemap};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
}
/// `CopyToWindowPixelBuffer` with this module's view of its types.
#[inline]
fn CopyToWindowPixelBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::window::CopyToWindowPixelBuffer(a0, a1 as _, a2, a3);
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const sDestroy: usize = 0;
const sXOffset: usize = 1;
// Data tables (translate with cdata.py): gBattleScriptingCommandsTable sAccuracyStageRatios sCriticalHitChance sStatusFlagsForMoveEffects sMoveEffectBS_Ptrs sUnusedWinTemplate sLevelUpBanner_Pal sLevelUpBanner_Gfx sRubyLevelUpStatBoxStats sOamData_MonIconOnLvlUpBanner sSpriteTemplate_MonIconOnLvlUpBanner sProtectSuccessRates sMovesForbiddenToCopy sFlailHpScaleToPowerTable sNaturePowerMoves sWeightToDamageTable sPickupItems sRarePickupItems sPickupProbabilities sEnvironmentToType sBallCatchBonuses gBattlePalaceNatureToMoveGroupLikelihood sBattlePalaceNatureToFlavorTextId

/// `struct StatFractions`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct StatFractions {
    pub dividend: u8,
    pub divisor: u8,
}

unsafe impl Sync for StatFractions {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<StatFractions>() == 4);
    assert!(offset_of!(StatFractions, dividend) == 0);
    assert!(offset_of!(StatFractions, divisor) == 1);
};

const ASSIST_FORBIDDEN_END: u16 = 65535;
const LEVEL_UP_BANNER_END: u16 = 512;
const LEVEL_UP_BANNER_START: u16 = 416;
const METRONOME_FORBIDDEN_END: u16 = 65535;
const MIMIC_FORBIDDEN_END: u16 = 65534;
const STAT_CHANGE_DIDNT_WORK: u8 = 1;
const STAT_CHANGE_WORKED: u8 = 0;
const TAG_LVLUP_BANNER_MON_ICON: u16 = 55130;

static sAccuracyStageRatios: Table<CArray<StatFractions, 13>> =
    Table((&raw const crate::data::battle_script_commands::sAccuracyStageRatios).cast());
static sBallCatchBonuses: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_script_commands::sBallCatchBonuses).cast());
static sBattlePalaceNatureToFlavorTextId: Table<CArray<u8, 25>> = Table(
    (&raw const crate::data::battle_script_commands::sBattlePalaceNatureToFlavorTextId).cast(),
);
static sCriticalHitChance: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::battle_script_commands::sCriticalHitChance).cast());
static sEnvironmentToType: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::battle_script_commands::sEnvironmentToType).cast());
static sFlailHpScaleToPowerTable: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::battle_script_commands::sFlailHpScaleToPowerTable).cast());
static sLevelUpBanner_Gfx: Table<CArray<u32, 50>> =
    Table((&raw const crate::data::battle_script_commands::sLevelUpBanner_Gfx).cast());
static sLevelUpBanner_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_script_commands::sLevelUpBanner_Pal).cast());
static sMoveEffectBS_Ptrs: Table<CArray<*mut u8, 39>> =
    Table((&raw const crate::data::battle_script_commands::sMoveEffectBS_Ptrs).cast());
static sMovesForbiddenToCopy: Table<CArray<u16, 20>> =
    Table((&raw const crate::data::battle_script_commands::sMovesForbiddenToCopy).cast());
static sNaturePowerMoves: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::battle_script_commands::sNaturePowerMoves).cast());
static sPickupItems: Table<CArray<u16, 18>> =
    Table((&raw const crate::data::battle_script_commands::sPickupItems).cast());
static sPickupProbabilities: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::battle_script_commands::sPickupProbabilities).cast());
static sProtectSuccessRates: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::battle_script_commands::sProtectSuccessRates).cast());
static sRarePickupItems: Table<CArray<u16, 11>> =
    Table((&raw const crate::data::battle_script_commands::sRarePickupItems).cast());
static sSpriteTemplate_MonIconOnLvlUpBanner: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_script_commands::sSpriteTemplate_MonIconOnLvlUpBanner).cast(),
);
static sStatusFlagsForMoveEffects: Table<CArray<u32, 60>> =
    Table((&raw const crate::data::battle_script_commands::sStatusFlagsForMoveEffects).cast());
static sWeightToDamageTable: Table<CArray<u16, 12>> =
    Table((&raw const crate::data::battle_script_commands::sWeightToDamageTable).cast());

/// `AddTextPrinter` with this module's view of its types.
#[inline]
fn AddTextPrinter(
    a0: *mut TextPrinterTemplate,
    a1: u8,
    a2: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe { crate::text::AddTextPrinter(a0 as _, a1, core::mem::transmute(a2)) }
}
/// `Sqrt` with this module's view of its types.
#[inline]
fn Sqrt(a0: u32) -> u16 {
    unsafe { crate::syscall::Sqrt(a0) }
}

pub(crate) unsafe fn Cmd_attackcanceler() {
    let mut i: i32 = 0;
    if gBattleOutcome.get() != 0 {
        gCurrentActionFuncId.set(B_ACTION_FINISHED);
        return;
    }
    if ({
        let gl_i0 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i0].hp)
    }) == 0
        && gHitMarker.get() & HITMARKER_NO_ATTACKSTRING == 0
    {
        gHitMarker.set(gHitMarker.get() | (HITMARKER_UNABLE_TO_USE_MOVE));
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveEnd
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        return;
    }
    if AtkCanceler_UnableToUseMove() != 0 {
        return;
    }
    if AbilityBattleEffects(ABILITYEFFECT_MOVES_BLOCK, gBattlerTarget.get(), 0, 0, 0) != 0 {
        return;
    }
    if ({
        let gl_i1 = gBattlerAttacker.get();
        let gl_i2 = gCurrMovePos.get();
        crate::gl!(gBattleMons[gl_i1].pp[gl_i2])
    }) == 0
        && gCurrentMove.get() != MOVE_STRUGGLE
        && gHitMarker.get() & 0x800200 == 0
        && ({
            let gl_i3 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i3].status2)
        }) & STATUS2_MULTIPLETURNS
            == 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_NoPPForMove
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        return;
    }
    gHitMarker.set(gHitMarker.get() & (0xff7fffff));
    if gHitMarker.get() & HITMARKER_OBEYS == 0
        && ({
            let gl_i4 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i4].status2)
        }) & STATUS2_MULTIPLETURNS
            == 0
    {
        i = IsMonDisobedient() as i32;
        match i {
            0 => {}
            2 => {
                gHitMarker.set(gHitMarker.get() | (HITMARKER_OBEYS));
                return;
            }
            _ => {
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
                return;
            }
        }
    }
    gHitMarker.set(gHitMarker.get() | (HITMARKER_OBEYS));
    if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].bounceMove() != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .flags as i32
            & FLAG_MAGIC_COAT_AFFECTED
            != 0
    {
        PressurePPLose(
            gBattlerAttacker.get(),
            gBattlerTarget.get(),
            MOVE_MAGIC_COAT,
        );
        (*gProtectStructs.as_ptr())[gBattlerTarget.get()].set_bounceMove(FALSE as u32);
        BattleScriptPushCursor();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MagicCoatBounce
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        return;
    }
    i = 0;
    while i < gBattlersCount.get() as i32 {
        if (*gProtectStructs.as_ptr())[crate::gl!(gBattlerByTurnOrder[i])].stealMove() != 0
            && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .flags as i32
                & FLAG_SNATCH_AFFECTED
                != 0
        {
            PressurePPLose(
                gBattlerAttacker.get(),
                crate::gl!(gBattlerByTurnOrder[i]),
                MOVE_SNATCH,
            );
            (*gProtectStructs.as_ptr())[crate::gl!(gBattlerByTurnOrder[i])]
                .set_stealMove(FALSE as u32);
            crate::gl!(gBattleScripting.battler = crate::gl!(gBattlerByTurnOrder[i]));
            BattleScriptPushCursor();
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SnatchedMove
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            return;
        }
        i += 1;
    }
    if (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].lightningRodRedirected() != 0 {
        (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].set_lightningRodRedirected(FALSE as u32);
        gLastUsedAbility.set(ABILITY_LIGHTNING_ROD);
        BattleScriptPushCursor();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_TookAttack
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
    } else if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].protected() != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .flags as i32
            & 2
            != 0
        && (gCurrentMove.get() != MOVE_CURSE
            || (({
                let gl_i5 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i5].types[0])
            }) == TYPE_GHOST
                || ({
                    let gl_i6 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i6].types[1])
                }) == TYPE_GHOST))
        && (IsTwoTurnsMove(gCurrentMove.get()) == 0
            || ({
                let gl_i7 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i7].status2)
            }) & STATUS2_MULTIPLETURNS
                != 0)
    {
        CancelMultiTurnMoves(gBattlerAttacker.get());
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        {
            let gl_v = 0;
            let gl_i0 = gBattlerTarget.get();
            crate::gl!(gLastLandedMoves[gl_i0] = gl_v)
        };
        {
            let gl_v = 0;
            let gl_i1 = gBattlerTarget.get();
            crate::gl!(gLastHitByType[gl_i1] = gl_v)
        };
        crate::gl!(gBattleCommunication[6] = B_MSG_PROTECTED);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
unsafe fn JumpIfMoveFailed(adder: u8, r#move: u16) {
    let mut BS_ptr: *mut u8 = (*gBattlescriptCurrInstr.as_ptr()).at(adder);
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        {
            let gl_v = 0;
            let gl_i2 = gBattlerTarget.get();
            crate::gl!(gLastLandedMoves[gl_i2] = gl_v)
        };
        {
            let gl_v = 0;
            let gl_i3 = gBattlerTarget.get();
            crate::gl!(gLastHitByType[gl_i3] = gl_v)
        };
        BS_ptr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        TrySetDestinyBondToHappen();
        if AbilityBattleEffects(ABILITYEFFECT_ABSORBING, gBattlerTarget.get(), 0, 0, r#move) != 0 {
            return;
        }
    }
    crate::gl!(gBattlescriptCurrInstr = BS_ptr);
}
pub(crate) unsafe fn Cmd_jumpifaffectedbyprotect() {
    if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].protected() != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .flags as i32
            & 2
            != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        JumpIfMoveFailed(5, 0);
        crate::gl!(gBattleCommunication[6] = B_MSG_PROTECTED);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
unsafe fn JumpIfMoveAffectedByProtect(r#move: u16) -> u8 {
    let mut affected: u8 = FALSE;
    if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].protected() != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .flags as i32
            & 2
            != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        JumpIfMoveFailed(7, r#move);
        crate::gl!(gBattleCommunication[6] = B_MSG_PROTECTED);
        affected = TRUE;
    }
    affected
}
unsafe fn AccuracyCalcHelper(r#move: u16) -> u8 {
    if {
        let gl_i4 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i4])
    } & STATUS3_ALWAYS_HITS
        != 0
        && ({
            let gl_i8 = gBattlerTarget.get();
            crate::gl!(gDisableStructs[gl_i8].battlerWithSureHit)
        }) == gBattlerAttacker.get()
    {
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    if gHitMarker.get() & HITMARKER_IGNORE_ON_AIR == 0 && {
        let gl_i5 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i5])
    } & STATUS3_ON_AIR
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    gHitMarker.set(gHitMarker.get() & (0xfffeffff));
    if gHitMarker.get() & HITMARKER_IGNORE_UNDERGROUND == 0 && {
        let gl_i6 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i6])
    } & STATUS3_UNDERGROUND
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    gHitMarker.set(gHitMarker.get() & (0xfffdffff));
    if gHitMarker.get() & 0x40000 == 0 && {
        let gl_i7 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i7])
    } & 0x40000
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    gHitMarker.set(gHitMarker.get() & (0xfffbffff));
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
        && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
        && gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_THUNDER
        || ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_ALWAYS_HIT
            || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .effect
                == EFFECT_VITAL_THROW)
    {
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn Cmd_accuracycheck() {
    let mut r#move: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(5) as u16
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as u16) << 8);
    if r#move == NO_ACC_CALC || r#move == NO_ACC_CALC_CHECK_LOCK_ON {
        if {
            let gl_i8 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i8])
        } & STATUS3_ALWAYS_HITS
            != 0
            && r#move == NO_ACC_CALC_CHECK_LOCK_ON
            && ({
                let gl_i9 = gBattlerTarget.get();
                crate::gl!(gDisableStructs[gl_i9].battlerWithSureHit)
            }) == gBattlerAttacker.get()
        {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        } else if {
            let gl_i9 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i9])
        } & 0x400c0
            != 0
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else if JumpIfMoveAffectedByProtect(0) == 0 {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        }
    } else {
        let mut r#type: u8 = 0;
        let mut holdEffect: u8 = 0;
        let mut param: u8 = 0;
        let mut buff: i8 = 0;
        if r#move == ACC_CURR_MOVE {
            r#move = gCurrentMove.get();
        }
        if (*crate::gl!(gBattleStruct)).dynamicMoveType != 0 {
            r#type = (*crate::gl!(gBattleStruct)).dynamicMoveType & 63;
        } else {
            r#type = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[r#move]
                .r#type;
        }
        if JumpIfMoveAffectedByProtect(r#move) != 0 {
            return;
        }
        if AccuracyCalcHelper(r#move) != 0 {
            return;
        }
        if ({
            let gl_i10 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i10].status2)
        }) & STATUS2_FORESIGHT
            != 0
        {
            let acc: u8 = ({
                let gl_i11 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i11].statStages[6])
            }) as u8;
            buff = acc as i8;
        } else {
            let acc: u8 = ({
                let gl_i12 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i12].statStages[6])
            }) as u8;
            buff = acc as i8 + DEFAULT_STAT_STAGE
                - ({
                    let gl_i13 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i13].statStages[7])
                });
        }
        if buff < MIN_STAT_STAGE {
            buff = MIN_STAT_STAGE;
        }
        if buff > MAX_STAT_STAGE {
            buff = MAX_STAT_STAGE;
        }
        let mut moveAcc: u8 = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[r#move]
            .accuracy;
        if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
            && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
            && gBattleWeather.get() as i32 & B_WEATHER_SUN != 0
            && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .effect
                == EFFECT_THUNDER
        {
            moveAcc = 50;
        }
        let mut calc: u16 = sAccuracyStageRatios[buff].dividend as u16 * moveAcc as u16;
        calc = div_i32(calc as i32, sAccuracyStageRatios[buff].divisor as i32) as u16;
        if ({
            let gl_i14 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i14].ability)
        }) == ABILITY_COMPOUND_EYES
        {
            calc = (calc as i32 * 130 / 100) as u16;
        }
        if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
            && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
            && ({
                let gl_i15 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i15].ability)
            }) == ABILITY_SAND_VEIL
            && gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM != 0
        {
            calc = (calc as i32 * 80 / 100) as u16;
        }
        if ({
            let gl_i16 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i16].ability)
        }) == ABILITY_HUSTLE
            && r#type < 9
        {
            calc = (calc as i32 * 80 / 100) as u16;
        }
        if ({
            let gl_i17 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i17].item)
        }) == ITEM_ENIGMA_BERRY
        {
            holdEffect = {
                let gl_i10 = gBattlerTarget.get();
                crate::gl!(gEnigmaBerries[gl_i10].holdEffect)
            };
            param = {
                let gl_i11 = gBattlerTarget.get();
                crate::gl!(gEnigmaBerries[gl_i11].holdEffectParam)
            };
        } else {
            holdEffect = GetItemHoldEffect({
                let gl_i18 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i18].item)
            });
            param = GetItemHoldEffectParam({
                let gl_i19 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i19].item)
            });
        }
        gPotentialItemEffectBattler.set(gBattlerTarget.get());
        if holdEffect == HOLD_EFFECT_EVASION_UP {
            calc = (calc as i32 * (100 - param as i32) / 100) as u16;
        }
        if Random() as i32 % 100 + 1 > calc as i32 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
            if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
                && ((*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[r#move]
                    .target
                    == MOVE_TARGET_BOTH
                    || (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[r#move]
                        .target
                        == MOVE_TARGET_FOES_AND_ALLY)
            {
                crate::gl!(gBattleCommunication[6] = B_MSG_AVOIDED_ATK);
            } else {
                crate::gl!(gBattleCommunication[6] = B_MSG_MISSED);
            }
            CheckWonderGuardAndLevitate();
        }
        JumpIfMoveFailed(7, r#move);
    }
}
pub(crate) unsafe fn Cmd_attackstring() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gHitMarker.get() & 1536 == 0 {
        PrepareStringBattle(STRINGID_USEDMOVE, gBattlerAttacker.get());
        gHitMarker.set(gHitMarker.get() | (HITMARKER_ATTACKSTRING_PRINTED));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    crate::gl!(gBattleCommunication[7] = 0);
}
pub(crate) unsafe fn Cmd_ppreduce() {
    let mut ppToDeduct: i32 = 1;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()].ppNotAffectedByPressure() == 0 {
        match (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .target
        {
            MOVE_TARGET_FOES_AND_ALLY => {
                ppToDeduct += AbilityBattleEffects(
                    ABILITYEFFECT_COUNT_ON_FIELD,
                    gBattlerAttacker.get(),
                    ABILITY_PRESSURE,
                    0,
                    0,
                ) as i32;
            }
            MOVE_TARGET_BOTH | MOVE_TARGET_OPPONENTS_FIELD => {
                ppToDeduct += AbilityBattleEffects(
                    ABILITYEFFECT_COUNT_OTHER_SIDE,
                    gBattlerAttacker.get(),
                    ABILITY_PRESSURE,
                    0,
                    0,
                ) as i32;
            }
            _ => {
                if gBattlerAttacker.get() != gBattlerTarget.get()
                    && ({
                        let gl_i20 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i20].ability)
                    }) == ABILITY_PRESSURE
                {
                    ppToDeduct += 1;
                }
            }
        }
    }
    if gHitMarker.get() & 2560 == 0
        && ({
            let gl_i21 = gBattlerAttacker.get();
            let gl_i22 = gCurrMovePos.get();
            crate::gl!(gBattleMons[gl_i21].pp[gl_i22])
        }) != 0
    {
        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_notFirstStrike(1);
        if ({
            let gl_i23 = gBattlerAttacker.get();
            let gl_i24 = gCurrMovePos.get();
            crate::gl!(gBattleMons[gl_i23].pp[gl_i24])
        }) as i32
            > ppToDeduct
        {
            {
                let gl_i25 = gBattlerAttacker.get();
                let gl_i26 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i25].pp[gl_i26] -= ppToDeduct as u8)
            };
        } else {
            {
                let gl_i27 = gBattlerAttacker.get();
                let gl_i28 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i27].pp[gl_i28] = 0)
            };
        }
        if ({
            let gl_i29 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i29].status2)
        }) & 0x200000
            == 0
            && (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gCurrMovePos.get()]
                == 0
        {
            gActiveBattler.set(gBattlerAttacker.get());
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_PPMOVE1_BATTLE + gCurrMovePos.get(),
                0,
                1,
                ({
                    let gl_i30 = gBattlerAttacker.get();
                    let gl_i31 = gCurrMovePos.get();
                    crate::gl!(&raw gBattleMons[gl_i30].pp[gl_i31])
                }) as *mut c_void,
            );
            MarkBattlerForControllerExec(gBattlerAttacker.get());
        }
    }
    gHitMarker.set(gHitMarker.get() & (0xfffff7ff));
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_critcalc() {
    let mut holdEffect: u8 = 0;
    let mut item: u16 = 0;
    item = {
        let gl_i32 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i32].item)
    };
    if item == ITEM_ENIGMA_BERRY {
        holdEffect = {
            let gl_i12 = gBattlerAttacker.get();
            crate::gl!(gEnigmaBerries[gl_i12].holdEffect)
        };
    } else {
        holdEffect = GetItemHoldEffect(item);
    }
    gPotentialItemEffectBattler.set(gBattlerAttacker.get());
    let mut critChance: u16 = 2
        * (({
            let gl_i33 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i33].status2)
        }) & STATUS2_FOCUS_ENERGY
            != 0) as u16
        + ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_HIGH_CRITICAL) as u16
        + ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_SKY_ATTACK) as u16
        + ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_BLAZE_KICK) as u16
        + ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_POISON_TAIL) as u16
        + (holdEffect == HOLD_EFFECT_SCOPE_LENS) as u16
        + 2 * (holdEffect == HOLD_EFFECT_LUCKY_PUNCH
            && ({
                let gl_i34 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i34].species)
            }) == SPECIES_CHANSEY) as u16
        + 2 * (holdEffect == HOLD_EFFECT_STICK
            && ({
                let gl_i35 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i35].species)
            }) == SPECIES_FARFETCHD) as u16;
    if critChance >= 5 {
        critChance = 4;
    }
    if ({
        let gl_i36 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i36].ability)
    }) != ABILITY_BATTLE_ARMOR
        && ({
            let gl_i37 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i37].ability)
        }) != ABILITY_SHELL_ARMOR
        && {
            let gl_i13 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i13])
        } & STATUS3_CANT_SCORE_A_CRIT
            == 0
        && gBattleTypeFlags.get() & 528 == 0
        && rem_i32(Random() as i32, sCriticalHitChance[critChance] as i32) == 0
    {
        gCritMultiplier.set(2);
    } else {
        gCritMultiplier.set(1);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_damagecalc() {
    let sideStatus: u16 = {
        let gl_i14 = GetBattlerPosition(gBattlerTarget.get()) as i32 & 1;
        crate::gl!(gSideStatuses[gl_i14])
    };
    gBattleMoveDamage.set(CalculateBaseDamage(
        {
            let gl_i38 = gBattlerAttacker.get();
            crate::gl!(&raw gBattleMons[gl_i38])
        },
        {
            let gl_i39 = gBattlerTarget.get();
            crate::gl!(&raw gBattleMons[gl_i39])
        },
        gCurrentMove.get() as u32,
        sideStatus,
        gDynamicBasePower.get(),
        (*crate::gl!(gBattleStruct)).dynamicMoveType,
        gBattlerAttacker.get(),
        gBattlerTarget.get(),
    ));
    gBattleMoveDamage.set(
        gBattleMoveDamage.get()
            * gCritMultiplier.get() as i32
            * crate::gl!(gBattleScripting.dmgMultiplier) as i32,
    );
    if {
        let gl_i15 = gBattlerAttacker.get();
        crate::gl!(gStatuses3[gl_i15])
    } & STATUS3_CHARGED_UP
        != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .r#type
            == TYPE_ELECTRIC
    {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * (2));
    }
    if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].helpingHand() != 0 {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * 15 / 10);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub unsafe fn AI_CalcDmg(attacker: u8, defender: u8) {
    let sideStatus: u16 = {
        let gl_i16 = GetBattlerPosition(defender) as i32 & 1;
        crate::gl!(gSideStatuses[gl_i16])
    };
    gBattleMoveDamage.set(CalculateBaseDamage(
        crate::gl!(&raw gBattleMons[attacker]),
        crate::gl!(&raw gBattleMons[defender]),
        gCurrentMove.get() as u32,
        sideStatus,
        gDynamicBasePower.get(),
        (*crate::gl!(gBattleStruct)).dynamicMoveType,
        attacker,
        defender,
    ));
    gDynamicBasePower.set(0);
    gBattleMoveDamage.set(
        gBattleMoveDamage.get()
            * gCritMultiplier.get() as i32
            * crate::gl!(gBattleScripting.dmgMultiplier) as i32,
    );
    if crate::gl!(gStatuses3[attacker]) & STATUS3_CHARGED_UP != 0
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .r#type
            == TYPE_ELECTRIC
    {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * (2));
    }
    if (*gProtectStructs.as_ptr())[attacker].helpingHand() != 0 {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * 15 / 10);
    }
}
unsafe fn ModulateDmgByType(multiplier: u8) {
    gBattleMoveDamage.set(gBattleMoveDamage.get() * multiplier as i32 / 10);
    if gBattleMoveDamage.get() == 0 && multiplier != 0 {
        gBattleMoveDamage.set(1);
    }
    match multiplier {
        TYPE_MUL_NO_EFFECT => {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_DOESNT_AFFECT_FOE));
            gMoveResultFlags.set(gMoveResultFlags.get() & (251));
            gMoveResultFlags.set(gMoveResultFlags.get() & (253));
        }
        TYPE_MUL_NOT_EFFECTIVE => {
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .power
                != 0
                && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
            {
                if gMoveResultFlags.get() as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    gMoveResultFlags.set(gMoveResultFlags.get() & (253));
                } else {
                    gMoveResultFlags
                        .set(gMoveResultFlags.get() | (MOVE_RESULT_NOT_VERY_EFFECTIVE as u8));
                }
            }
        }
        TYPE_MUL_SUPER_EFFECTIVE
            if (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
            .power
                != 0
                && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0 =>
        {
            if gMoveResultFlags.get() as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
                gMoveResultFlags.set(gMoveResultFlags.get() & (251));
            } else {
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_SUPER_EFFECTIVE as u8));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_typecalc() {
    let mut i: i32 = 0;
    let mut moveType: u8 = 0;
    if gCurrentMove.get() == MOVE_STRUGGLE {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        return;
    }
    if (*crate::gl!(gBattleStruct)).dynamicMoveType != 0 {
        moveType = (*crate::gl!(gBattleStruct)).dynamicMoveType & 63;
    } else {
        moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .r#type;
    }
    if ({
        let gl_i40 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i40].types[0])
    }) == moveType
        || ({
            let gl_i41 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i41].types[1])
        }) == moveType
    {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * (15));
        gBattleMoveDamage.set(gBattleMoveDamage.get() / (10));
    }
    if ({
        let gl_i42 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i42].ability)
    }) == ABILITY_LEVITATE
        && moveType == TYPE_GROUND
    {
        gLastUsedAbility.set({
            let gl_i43 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i43].ability)
        });
        gMoveResultFlags.set(gMoveResultFlags.get() | (9));
        {
            let gl_v = 0;
            let gl_i17 = gBattlerTarget.get();
            crate::gl!(gLastLandedMoves[gl_i17] = gl_v)
        };
        {
            let gl_v = 0;
            let gl_i18 = gBattlerTarget.get();
            crate::gl!(gLastHitByType[gl_i18] = gl_v)
        };
        crate::gl!(gBattleCommunication[6] = B_MSG_GROUND_MISS);
        RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
    } else {
        while (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())
            [i]
            != TYPE_ENDTABLE
        {
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == TYPE_FORESIGHT
            {
                if ({
                    let gl_i44 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i44].status2)
                }) & STATUS2_FORESIGHT
                    != 0
                {
                    break;
                }
                i += 3;
                continue;
            } else if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == moveType
            {
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == ({
                        let gl_i45 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i45].types[0])
                    })
                {
                    ModulateDmgByType(
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2],
                    );
                }
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == ({
                        let gl_i46 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i46].types[1])
                    })
                    && ({
                        let gl_i47 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i47].types[0])
                    }) != ({
                        let gl_i48 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i48].types[1])
                    })
                {
                    ModulateDmgByType(
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2],
                    );
                }
            }
            i += 3;
        }
    }
    if ({
        let gl_i49 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i49].ability)
    }) == ABILITY_WONDER_GUARD
        && AttacksThisTurn(gBattlerAttacker.get(), gCurrentMove.get()) == 2
        && (gMoveResultFlags.get() as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0
            || gMoveResultFlags.get() as i32 & 6 == 6)
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .power
            != 0
    {
        gLastUsedAbility.set(ABILITY_WONDER_GUARD);
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        {
            let gl_v = 0;
            let gl_i19 = gBattlerTarget.get();
            crate::gl!(gLastLandedMoves[gl_i19] = gl_v)
        };
        {
            let gl_v = 0;
            let gl_i20 = gBattlerTarget.get();
            crate::gl!(gLastHitByType[gl_i20] = gl_v)
        };
        crate::gl!(gBattleCommunication[6] = B_MSG_AVOIDED_DMG);
        RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
    }
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_targetNotAffected(1);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
unsafe fn CheckWonderGuardAndLevitate() {
    let mut flags: u8 = 0;
    let mut i: i32 = 0;
    let mut moveType: u8 = 0;
    if gCurrentMove.get() == MOVE_STRUGGLE
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .power
            == 0
    {
        return;
    }
    if (*crate::gl!(gBattleStruct)).dynamicMoveType != 0 {
        moveType = (*crate::gl!(gBattleStruct)).dynamicMoveType & 63;
    } else {
        moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .r#type;
    }
    if ({
        let gl_i50 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i50].ability)
    }) == ABILITY_LEVITATE
        && moveType == TYPE_GROUND
    {
        gLastUsedAbility.set(ABILITY_LEVITATE);
        crate::gl!(gBattleCommunication[6] = B_MSG_GROUND_MISS);
        RecordAbilityBattle(gBattlerTarget.get(), ABILITY_LEVITATE);
        return;
    }
    while (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())[i]
        != TYPE_ENDTABLE
    {
        if (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())[i]
            == TYPE_FORESIGHT
        {
            if ({
                let gl_i51 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i51].status2)
            }) & STATUS2_FORESIGHT
                != 0
            {
                break;
            }
            i += 3;
            continue;
        }
        if (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())[i]
            == moveType
        {
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == ({
                    let gl_i52 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i52].types[0])
                })
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    == TYPE_MUL_NO_EFFECT
            {
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_DOESNT_AFFECT_FOE));
                (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_targetNotAffected(1);
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == ({
                    let gl_i53 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i53].types[1])
                })
                && ({
                    let gl_i54 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i54].types[0])
                }) != ({
                    let gl_i55 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i55].types[1])
                })
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    == TYPE_MUL_NO_EFFECT
            {
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_DOESNT_AFFECT_FOE));
                (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_targetNotAffected(1);
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == ({
                    let gl_i56 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i56].types[0])
                })
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    == 20
            {
                flags |= 1;
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == ({
                    let gl_i57 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i57].types[1])
                })
                && ({
                    let gl_i58 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i58].types[0])
                }) != ({
                    let gl_i59 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i59].types[1])
                })
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    == TYPE_MUL_SUPER_EFFECTIVE
            {
                flags |= 1;
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == ({
                    let gl_i60 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i60].types[0])
                })
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    == 5
            {
                flags |= 2;
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i + 1]
                == ({
                    let gl_i61 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i61].types[1])
                })
                && ({
                    let gl_i62 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i62].types[0])
                }) != ({
                    let gl_i63 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i63].types[1])
                })
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    == TYPE_MUL_NOT_EFFECTIVE
            {
                flags |= 2;
            }
        }
        i += 3;
    }
    if ({
        let gl_i64 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i64].ability)
    }) == ABILITY_WONDER_GUARD
        && AttacksThisTurn(gBattlerAttacker.get(), gCurrentMove.get()) == 2
        && (flags as i32 & 2 != 0 || flags as i32 & 1 == 0)
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .power
            != 0
    {
        gLastUsedAbility.set(ABILITY_WONDER_GUARD);
        crate::gl!(gBattleCommunication[6] = B_MSG_AVOIDED_DMG);
        RecordAbilityBattle(gBattlerTarget.get(), ABILITY_WONDER_GUARD);
    }
}
unsafe fn ModulateDmgByType2(multiplier: u8, r#move: u16, flags: *mut u8) {
    gBattleMoveDamage.set(gBattleMoveDamage.get() * multiplier as i32 / 10);
    if gBattleMoveDamage.get() == 0 && multiplier != 0 {
        gBattleMoveDamage.set(1);
    }
    match multiplier {
        TYPE_MUL_NO_EFFECT => {
            *flags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
            *flags &= 251;
            *flags &= 253;
        }
        TYPE_MUL_NOT_EFFECTIVE => {
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .power
                != 0
                && *flags as i32 & MOVE_RESULT_NO_EFFECT == 0
            {
                if *flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    *flags &= 253;
                } else {
                    *flags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                }
            }
        }
        TYPE_MUL_SUPER_EFFECTIVE
            if (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[r#move]
                .power
                != 0
                && *flags as i32 & MOVE_RESULT_NO_EFFECT == 0 =>
        {
            if *flags as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
                *flags &= 251;
            } else {
                *flags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
            }
        }
        _ => {}
    }
}
pub unsafe fn TypeCalc(r#move: u16, attacker: u8, defender: u8) -> u8 {
    let mut i: i32 = 0;
    let mut flags: u8 = 0;
    if r#move == MOVE_STRUGGLE {
        return 0;
    }
    let moveType: u8 = (*(&raw const crate::data::pokemon::gBattleMoves)
        .cast::<CArray<BattleMove, 0>>())[r#move]
        .r#type;
    if crate::gl!(gBattleMons[attacker].types[0]) == moveType
        || crate::gl!(gBattleMons[attacker].types[1]) == moveType
    {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * (15));
        gBattleMoveDamage.set(gBattleMoveDamage.get() / (10));
    }
    if crate::gl!(gBattleMons[defender].ability) == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        flags |= 9;
    } else {
        while (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())
            [i]
            != TYPE_ENDTABLE
        {
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == TYPE_FORESIGHT
            {
                if crate::gl!(gBattleMons[defender].status2) & STATUS2_FORESIGHT != 0 {
                    break;
                }
                i += 3;
                continue;
            } else if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == moveType
            {
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == crate::gl!(gBattleMons[defender].types[0])
                {
                    ModulateDmgByType2(
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2],
                        r#move,
                        &raw mut flags,
                    );
                }
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == crate::gl!(gBattleMons[defender].types[1])
                    && crate::gl!(gBattleMons[defender].types[0])
                        != crate::gl!(gBattleMons[defender].types[1])
                {
                    ModulateDmgByType2(
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2],
                        r#move,
                        &raw mut flags,
                    );
                }
            }
            i += 3;
        }
    }
    if crate::gl!(gBattleMons[defender].ability) == ABILITY_WONDER_GUARD
        && flags as i32 & MOVE_RESULT_MISSED as i32 == 0
        && AttacksThisTurn(attacker, r#move) == 2
        && (flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0 || flags as i32 & 6 == 6)
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .power
            != 0
    {
        flags |= MOVE_RESULT_MISSED;
    }
    flags
}
pub unsafe fn AI_TypeCalc(r#move: u16, targetSpecies: u16, targetAbility: u8) -> u8 {
    let mut i: i32 = 0;
    let mut flags: u8 = 0;
    let type1: u8 = (*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[targetSpecies]
        .types[0];
    let type2: u8 = (*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[targetSpecies]
        .types[1];
    if r#move == MOVE_STRUGGLE {
        return 0;
    }
    let moveType: u8 = (*(&raw const crate::data::pokemon::gBattleMoves)
        .cast::<CArray<BattleMove, 0>>())[r#move]
        .r#type;
    if targetAbility == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        flags = 9;
    } else {
        while (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())
            [i]
            != TYPE_ENDTABLE
        {
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == TYPE_FORESIGHT
            {
                i += 3;
                continue;
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == moveType
            {
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == type1
                {
                    ModulateDmgByType2(
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2],
                        r#move,
                        &raw mut flags,
                    );
                }
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == type2
                    && type1 != type2
                {
                    ModulateDmgByType2(
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2],
                        r#move,
                        &raw mut flags,
                    );
                }
            }
            i += 3;
        }
    }
    if targetAbility == ABILITY_WONDER_GUARD
        && (flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0 || flags as i32 & 6 == 6)
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .power
            != 0
    {
        flags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
    }
    flags
}
fn ApplyRandomDmgMultiplier() {
    let rand: u16 = Random();
    let randPercent: u16 = 100 - (rand as i32 % 16) as u16;
    if gBattleMoveDamage.get() != 0 {
        gBattleMoveDamage.set(gBattleMoveDamage.get() * (randPercent as i32));
        gBattleMoveDamage.set(gBattleMoveDamage.get() / (100));
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
    }
}
fn Unused_ApplyRandomDmgMultiplier() {
    ApplyRandomDmgMultiplier();
}
pub(crate) unsafe fn Cmd_adjustnormaldamage() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    ApplyRandomDmgMultiplier();
    if ({
        let gl_i65 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i65].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffect = {
            let gl_i21 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i21].holdEffect)
        };
        param = {
            let gl_i22 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i22].holdEffectParam)
        };
    } else {
        holdEffect = GetItemHoldEffect({
            let gl_i66 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i66].item)
        });
        param = GetItemHoldEffectParam({
            let gl_i67 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i67].item)
        });
    }
    gPotentialItemEffectBattler.set(gBattlerTarget.get());
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget.get(), holdEffect);
        (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].set_focusBanded(1);
    }
    if ({
        let gl_i68 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i68].status2)
    }) & STATUS2_SUBSTITUTE
        == 0
        && ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_FALSE_SWIPE
            || (*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0
            || (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0)
        && ({
            let gl_i69 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i69].hp)
        }) as i32
            <= gBattleMoveDamage.get()
    {
        gBattleMoveDamage.set(
            ({
                let gl_i70 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i70].hp)
            }) as i32
                - 1,
        );
        if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_ENDURED));
        } else if (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_HUNG_ON));
            gLastUsedItem.set({
                let gl_i71 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i71].item)
            });
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_adjustnormaldamage2() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    ApplyRandomDmgMultiplier();
    if ({
        let gl_i72 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i72].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffect = {
            let gl_i23 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i23].holdEffect)
        };
        param = {
            let gl_i24 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i24].holdEffectParam)
        };
    } else {
        holdEffect = GetItemHoldEffect({
            let gl_i73 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i73].item)
        });
        param = GetItemHoldEffectParam({
            let gl_i74 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i74].item)
        });
    }
    gPotentialItemEffectBattler.set(gBattlerTarget.get());
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget.get(), holdEffect);
        (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].set_focusBanded(1);
    }
    if ({
        let gl_i75 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i75].status2)
    }) & STATUS2_SUBSTITUTE
        == 0
        && ((*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0
            || (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0)
        && ({
            let gl_i76 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i76].hp)
        }) as i32
            <= gBattleMoveDamage.get()
    {
        gBattleMoveDamage.set(
            ({
                let gl_i77 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i77].hp)
            }) as i32
                - 1,
        );
        if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_ENDURED));
        } else if (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_HUNG_ON));
            gLastUsedItem.set({
                let gl_i78 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i78].item)
            });
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_attackanimation() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gHitMarker.get() & HITMARKER_NO_ANIMATIONS != 0
        && (gCurrentMove.get() != MOVE_TRANSFORM && gCurrentMove.get() != MOVE_SUBSTITUTE)
    {
        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_Pausex20
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        crate::gl!(gBattleScripting.animTurn += 1);
        crate::gl!(gBattleScripting.animTargetsHit += 1);
    } else {
        if ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .target as i32
            & MOVE_TARGET_BOTH as i32
            != 0
            || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .target as i32
                & MOVE_TARGET_FOES_AND_ALLY as i32
                != 0
            || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .target as i32
                & MOVE_TARGET_DEPENDS as i32
                != 0)
            && crate::gl!(gBattleScripting.animTargetsHit) != 0
        {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
            return;
        }
        if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0 {
            let mut multihit: u8 = 0;
            gActiveBattler.set(gBattlerAttacker.get());
            if ({
                let gl_i79 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i79].status2)
            }) & STATUS2_SUBSTITUTE
                != 0
            {
                multihit = gMultiHitCounter.get();
            } else if gMultiHitCounter.get() != 0 && gMultiHitCounter.get() != 1 {
                if ({
                    let gl_i80 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i80].hp)
                }) as i32
                    <= gBattleMoveDamage.get()
                {
                    multihit = 1;
                } else {
                    multihit = gMultiHitCounter.get();
                }
            } else {
                multihit = gMultiHitCounter.get();
            }
            BtlController_EmitMoveAnimation(
                B_COMM_TO_CONTROLLER,
                gCurrentMove.get(),
                crate::gl!(gBattleScripting.animTurn),
                gBattleMovePower.get(),
                gBattleMoveDamage.get(),
                {
                    let gl_i81 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i81].friendship)
                },
                {
                    let gl_i82 = gBattlerAttacker.get();
                    crate::gl!(&raw gDisableStructs[gl_i82])
                },
                multihit,
            );
            crate::gl!(gBattleScripting.animTurn += 1);
            crate::gl!(gBattleScripting.animTargetsHit += 1);
            MarkBattlerForControllerExec(gBattlerAttacker.get());
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        } else {
            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_Pausex20
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
        }
    }
}
pub(crate) unsafe fn Cmd_waitanimation() {
    if gBattleControllerExecFlags.get() == 0 {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_healthbarupdate() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        if ({
            let gl_i83 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i83].status2)
        }) & STATUS2_SUBSTITUTE
            != 0
            && ({
                let gl_i84 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i84].substituteHP)
            }) != 0
            && gHitMarker.get() & HITMARKER_IGNORE_SUBSTITUTE == 0
        {
            PrepareStringBattle(STRINGID_SUBSTITUTEDAMAGED, gActiveBattler.get());
        } else {
            let mut healthValue: i16 = 0;
            let currDmg: i32 = gBattleMoveDamage.get();
            let maxPossibleDmgValue: i32 = 10000;
            if currDmg <= maxPossibleDmgValue {
                healthValue = currDmg as i16;
            } else {
                healthValue = maxPossibleDmgValue as i16;
            }
            BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, healthValue as u16);
            MarkBattlerForControllerExec(gActiveBattler.get());
            if GetBattlerSide(gActiveBattler.get()) == 0 && gBattleMoveDamage.get() > 0 {
                (*gBattleResults.as_ptr()).set_playerMonWasDamaged(TRUE);
            }
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_datahpupdate() {
    let mut moveType: u32 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if (*crate::gl!(gBattleStruct)).dynamicMoveType == 0 {
        moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .r#type as u32;
    } else if (*crate::gl!(gBattleStruct)).dynamicMoveType as i32
        & F_DYNAMIC_TYPE_IGNORE_PHYSICALITY
        == 0
    {
        moveType = (*crate::gl!(gBattleStruct)).dynamicMoveType as u32 & DYNAMIC_TYPE_MASK;
    } else {
        moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .r#type as u32;
    }
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        if ({
            let gl_i85 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i85].status2)
        }) & STATUS2_SUBSTITUTE
            != 0
            && ({
                let gl_i86 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i86].substituteHP)
            }) != 0
            && gHitMarker.get() & HITMARKER_IGNORE_SUBSTITUTE == 0
        {
            if ({
                let gl_i87 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i87].substituteHP)
            }) as i32
                >= gBattleMoveDamage.get()
            {
                if {
                    let gl_i25 = gActiveBattler.get();
                    crate::gl!(gSpecialStatuses[gl_i25].shellBellDmg)
                } == 0
                {
                    {
                        let gl_v = gBattleMoveDamage.get();
                        let gl_i26 = gActiveBattler.get();
                        crate::gl!(gSpecialStatuses[gl_i26].shellBellDmg = gl_v)
                    };
                }
                {
                    let gl_i88 = gActiveBattler.get();
                    crate::gl!(
                        gDisableStructs[gl_i88].substituteHP -= gBattleMoveDamage.get() as u8
                    )
                };
                gHpDealt.set(gBattleMoveDamage.get());
            } else {
                if {
                    let gl_i27 = gActiveBattler.get();
                    crate::gl!(gSpecialStatuses[gl_i27].shellBellDmg)
                } == 0
                {
                    {
                        let gl_v = ({
                            let gl_i89 = gActiveBattler.get();
                            crate::gl!(gDisableStructs[gl_i89].substituteHP)
                        }) as i32;
                        let gl_i28 = gActiveBattler.get();
                        crate::gl!(gSpecialStatuses[gl_i28].shellBellDmg = gl_v)
                    };
                }
                gHpDealt.set(
                    ({
                        let gl_i90 = gActiveBattler.get();
                        crate::gl!(gDisableStructs[gl_i90].substituteHP)
                    }) as i32,
                );
                {
                    let gl_i91 = gActiveBattler.get();
                    crate::gl!(gDisableStructs[gl_i91].substituteHP = 0)
                };
            }
            if ({
                let gl_i92 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i92].substituteHP)
            }) == 0
            {
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
                BattleScriptPushCursor();
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SubstituteFade
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                return;
            }
        } else {
            gHitMarker.set(gHitMarker.get() & (0xfffffeff));
            if gBattleMoveDamage.get() < 0 {
                {
                    let gl_v = (gBattleMoveDamage.get() as u16).wrapping_neg();
                    let gl_i93 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i93].hp += gl_v)
                };
                if ({
                    let gl_i94 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i94].hp)
                }) > ({
                    let gl_i95 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i95].maxHP)
                }) {
                    {
                        let gl_i96 = gActiveBattler.get();
                        crate::gl!(
                            gBattleMons[gl_i96].hp = ({
                                let gl_i97 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i97].maxHP)
                            })
                        )
                    };
                }
            } else {
                if gHitMarker.get() & HITMARKER_IGNORE_BIDE != 0 {
                    gHitMarker.set(gHitMarker.get() & (0xffffffdf));
                } else {
                    {
                        let gl_v = gBattleMoveDamage.get();
                        let gl_i29 = gActiveBattler.get();
                        crate::gl!(gBideDmg[gl_i29] += gl_v)
                    };
                    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_TARGET {
                        {
                            let gl_v = gBattlerAttacker.get();
                            let gl_i30 = gActiveBattler.get();
                            crate::gl!(gBideTarget[gl_i30] = gl_v)
                        };
                    } else {
                        {
                            let gl_v = gBattlerTarget.get();
                            let gl_i31 = gActiveBattler.get();
                            crate::gl!(gBideTarget[gl_i31] = gl_v)
                        };
                    }
                }
                if ({
                    let gl_i98 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i98].hp)
                }) as i32
                    > gBattleMoveDamage.get()
                {
                    {
                        let gl_i99 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i99].hp -= gBattleMoveDamage.get() as u16)
                    };
                    gHpDealt.set(gBattleMoveDamage.get());
                } else {
                    gHpDealt.set(
                        ({
                            let gl_i100 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i100].hp)
                        }) as i32,
                    );
                    {
                        let gl_i101 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i101].hp = 0)
                    };
                }
                if {
                    let gl_i32 = gActiveBattler.get();
                    crate::gl!(gSpecialStatuses[gl_i32].shellBellDmg)
                } == 0
                    && gHitMarker.get() & HITMARKER_PASSIVE_HP_UPDATE == 0
                {
                    {
                        let gl_v = gHpDealt.get();
                        let gl_i33 = gActiveBattler.get();
                        crate::gl!(gSpecialStatuses[gl_i33].shellBellDmg = gl_v)
                    };
                }
                if moveType < 9
                    && gHitMarker.get() & HITMARKER_PASSIVE_HP_UPDATE == 0
                    && gCurrentMove.get() != MOVE_PAIN_SPLIT
                {
                    {
                        let gl_v = gHpDealt.get() as u32;
                        let gl_i34 = gActiveBattler.get();
                        crate::gl!(gProtectStructs[gl_i34].physicalDmg = gl_v)
                    };
                    {
                        let gl_v = gHpDealt.get();
                        let gl_i35 = gActiveBattler.get();
                        crate::gl!(gSpecialStatuses[gl_i35].physicalDmg = gl_v)
                    };
                    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_TARGET {
                        {
                            let gl_v = gBattlerAttacker.get();
                            let gl_i36 = gActiveBattler.get();
                            crate::gl!(gProtectStructs[gl_i36].physicalBattlerId = gl_v)
                        };
                        {
                            let gl_v = gBattlerAttacker.get();
                            let gl_i37 = gActiveBattler.get();
                            crate::gl!(gSpecialStatuses[gl_i37].physicalBattlerId = gl_v)
                        };
                    } else {
                        {
                            let gl_v = gBattlerTarget.get();
                            let gl_i38 = gActiveBattler.get();
                            crate::gl!(gProtectStructs[gl_i38].physicalBattlerId = gl_v)
                        };
                        {
                            let gl_v = gBattlerTarget.get();
                            let gl_i39 = gActiveBattler.get();
                            crate::gl!(gSpecialStatuses[gl_i39].physicalBattlerId = gl_v)
                        };
                    }
                } else if moveType >= 9 && gHitMarker.get() & HITMARKER_PASSIVE_HP_UPDATE == 0 {
                    {
                        let gl_v = gHpDealt.get() as u32;
                        let gl_i40 = gActiveBattler.get();
                        crate::gl!(gProtectStructs[gl_i40].specialDmg = gl_v)
                    };
                    {
                        let gl_v = gHpDealt.get();
                        let gl_i41 = gActiveBattler.get();
                        crate::gl!(gSpecialStatuses[gl_i41].specialDmg = gl_v)
                    };
                    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_TARGET {
                        {
                            let gl_v = gBattlerAttacker.get();
                            let gl_i42 = gActiveBattler.get();
                            crate::gl!(gProtectStructs[gl_i42].specialBattlerId = gl_v)
                        };
                        {
                            let gl_v = gBattlerAttacker.get();
                            let gl_i43 = gActiveBattler.get();
                            crate::gl!(gSpecialStatuses[gl_i43].specialBattlerId = gl_v)
                        };
                    } else {
                        {
                            let gl_v = gBattlerTarget.get();
                            let gl_i44 = gActiveBattler.get();
                            crate::gl!(gProtectStructs[gl_i44].specialBattlerId = gl_v)
                        };
                        {
                            let gl_v = gBattlerTarget.get();
                            let gl_i45 = gActiveBattler.get();
                            crate::gl!(gSpecialStatuses[gl_i45].specialBattlerId = gl_v)
                        };
                    }
                }
            }
            gHitMarker.set(gHitMarker.get() & (0xffefffff));
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_HP_BATTLE,
                0,
                2,
                ({
                    let gl_i102 = gActiveBattler.get();
                    crate::gl!(&raw gBattleMons[gl_i102].hp)
                }) as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
    } else {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        if {
            let gl_i46 = gActiveBattler.get();
            crate::gl!(gSpecialStatuses[gl_i46].shellBellDmg)
        } == 0
        {
            {
                let gl_v = IGNORE_SHELL_BELL;
                let gl_i47 = gActiveBattler.get();
                crate::gl!(gSpecialStatuses[gl_i47].shellBellDmg = gl_v)
            };
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_critmessage() {
    if gBattleControllerExecFlags.get() == 0 {
        if gCritMultiplier.get() == 2 && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
        {
            PrepareStringBattle(STRINGID_CRITICALHIT, gBattlerAttacker.get());
            crate::gl!(gBattleCommunication[7] = 1);
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_effectivenesssound() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(gBattlerTarget.get());
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_MISSED as i32 == 0 {
        match gMoveResultFlags.get() as i32 & 254 {
            MOVE_RESULT_SUPER_EFFECTIVE => {
                BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_SUPER_EFFECTIVE);
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
            MOVE_RESULT_NOT_VERY_EFFECTIVE => {
                BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_NOT_EFFECTIVE);
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
            8 | MOVE_RESULT_FAILED => {}
            _ => {
                if gMoveResultFlags.get() as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_SUPER_EFFECTIVE);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                } else if gMoveResultFlags.get() as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
                    BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_NOT_EFFECTIVE);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                } else if gMoveResultFlags.get() as i32 & 40 == 0 {
                    BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_EFFECTIVE);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
            }
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_resultmessage() {
    let mut stringId: u32 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_MISSED as i32 != 0
        && (gMoveResultFlags.get() as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 == 0
            || crate::gl!(gBattleCommunication[6]) > B_MSG_AVOIDED_ATK)
    {
        stringId = (*(&raw const crate::data::battle_message::gMissStringIds)
            .cast::<CArray<u16, 0>>())[crate::gl!(gBattleCommunication[6])]
            as u32;
        crate::gl!(gBattleCommunication[7] = 1);
    } else {
        crate::gl!(gBattleCommunication[7] = 1);
        match gMoveResultFlags.get() as i32 & 254 {
            MOVE_RESULT_SUPER_EFFECTIVE => {
                stringId = STRINGID_SUPEREFFECTIVE;
            }
            MOVE_RESULT_NOT_VERY_EFFECTIVE => {
                stringId = STRINGID_NOTVERYEFFECTIVE as u32;
            }
            MOVE_RESULT_ONE_HIT_KO => {
                stringId = STRINGID_ONEHITKO;
            }
            64 => {
                stringId = STRINGID_PKMNENDUREDHIT;
            }
            MOVE_RESULT_FAILED => {
                stringId = STRINGID_BUTITFAILED;
            }
            8 => {
                stringId = STRINGID_ITDOESNTAFFECT;
            }
            128 => {
                gLastUsedItem.set({
                    let gl_i103 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i103].item)
                });
                gPotentialItemEffectBattler.set(gBattlerTarget.get());
                gMoveResultFlags.set(gMoveResultFlags.get() & (63));
                BattleScriptPushCursor();
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_FocusBandActivates
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                return;
            }
            _ => {
                if gMoveResultFlags.get() as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
                    stringId = STRINGID_ITDOESNTAFFECT;
                } else if gMoveResultFlags.get() as i32 & MOVE_RESULT_ONE_HIT_KO != 0 {
                    gMoveResultFlags.set(gMoveResultFlags.get() & (239));
                    gMoveResultFlags.set(gMoveResultFlags.get() & (253));
                    gMoveResultFlags.set(gMoveResultFlags.get() & (251));
                    BattleScriptPushCursor();
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_OneHitKOMsg
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    return;
                } else if gMoveResultFlags.get() as i32 & MOVE_RESULT_FOE_ENDURED as i32 != 0 {
                    gMoveResultFlags.set(gMoveResultFlags.get() & (63));
                    BattleScriptPushCursor();
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_EnduredMsg
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    return;
                } else if gMoveResultFlags.get() as i32 & MOVE_RESULT_FOE_HUNG_ON as i32 != 0 {
                    gLastUsedItem.set({
                        let gl_i104 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i104].item)
                    });
                    gPotentialItemEffectBattler.set(gBattlerTarget.get());
                    gMoveResultFlags.set(gMoveResultFlags.get() & (63));
                    BattleScriptPushCursor();
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_FocusBandActivates
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    return;
                } else if gMoveResultFlags.get() as i32 & MOVE_RESULT_FAILED != 0 {
                    stringId = STRINGID_BUTITFAILED;
                } else {
                    crate::gl!(gBattleCommunication[7] = 0);
                }
            }
        }
    }
    if stringId != 0 {
        PrepareStringBattle(stringId as u16, gBattlerAttacker.get());
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_printstring() {
    if gBattleControllerExecFlags.get() == 0 {
        let var: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(1) as u16
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as u16) << 8);
        PrepareStringBattle(var, gBattlerAttacker.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
        crate::gl!(gBattleCommunication[7] = 1);
    }
}
pub(crate) unsafe fn Cmd_printselectionstring() {
    gActiveBattler.set(gBattlerAttacker.get());
    BtlController_EmitPrintSelectionString(
        B_COMM_TO_CONTROLLER,
        *(*gBattlescriptCurrInstr.as_ptr()).at(1) as u16
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as u16) << 8),
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
    crate::gl!(gBattleCommunication[7] = 1);
}
pub(crate) unsafe fn Cmd_waitmessage() {
    if gBattleControllerExecFlags.get() == 0 {
        if crate::gl!(gBattleCommunication[7]) == 0 {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
        } else {
            let toWait: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(1) as u16
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as u16) << 8);
            if ({
                gPauseCounterBattle.set(gPauseCounterBattle.get() + (1));
                gPauseCounterBattle.get()
            }) >= toWait
            {
                gPauseCounterBattle.set(0);
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
                crate::gl!(gBattleCommunication[7] = 0);
            }
        }
    }
}
pub(crate) unsafe fn Cmd_printfromtable() {
    if gBattleControllerExecFlags.get() == 0 {
        let mut ptr: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
            as usize as *mut u8 as *mut u16;
        ptr = ptr.at(crate::gl!(gBattleCommunication[5]));
        PrepareStringBattle(*ptr, gBattlerAttacker.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        crate::gl!(gBattleCommunication[7] = 1);
    }
}
pub(crate) unsafe fn Cmd_printselectionstringfromtable() {
    if gBattleControllerExecFlags.get() == 0 {
        let mut ptr: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
            as usize as *mut u8 as *mut u16;
        ptr = ptr.at(crate::gl!(gBattleCommunication[5]));
        gActiveBattler.set(gBattlerAttacker.get());
        BtlController_EmitPrintSelectionString(B_COMM_TO_CONTROLLER, *ptr);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        crate::gl!(gBattleCommunication[7] = 1);
    }
}
pub fn GetBattlerTurnOrderNum(battler: u8) -> u8 {
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if crate::gl!(gBattlerByTurnOrder[i]) == battler {
            break;
        }
        i += 1;
    }
    i as u8
}
pub unsafe fn SetMoveEffect(primary: u8, certain: u8) {
    let mut statusChanged: u32 = FALSE as u32;
    let mut affectsUser: u8 = 0;
    let mut noSunCanFreeze: u32 = TRUE as u32;
    if crate::gl!(gBattleCommunication[3]) as i32 & MOVE_EFFECT_AFFECTS_USER as i32 != 0 {
        gEffectBattler.set(gBattlerAttacker.get());
        crate::gl!(gBattleCommunication[3] &= 191);
        affectsUser = MOVE_EFFECT_AFFECTS_USER;
        crate::gl!(gBattleScripting.battler = gBattlerTarget.get());
    } else {
        gEffectBattler.set(gBattlerTarget.get());
        crate::gl!(gBattleScripting.battler = gBattlerAttacker.get());
    }
    if ({
        let gl_i105 = gEffectBattler.get();
        crate::gl!(gBattleMons[gl_i105].ability)
    }) == ABILITY_SHIELD_DUST
        && gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT == 0
        && primary == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
            .cast::<CArray<u8, 8>>()
            .cast_mut())[3]
            <= 9
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        crate::gl!(gBattleCommunication[3] = 0);
        return;
    }
    if {
        let gl_i48 = GetBattlerPosition(gEffectBattler.get()) as i32 & 1;
        crate::gl!(gSideStatuses[gl_i48])
    } as i32
        & SIDE_STATUS_SAFEGUARD
        != 0
        && gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT == 0
        && primary == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
            .cast::<CArray<u8, 8>>()
            .cast_mut())[3]
            <= 7
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        crate::gl!(gBattleCommunication[3] = 0);
        return;
    }
    if ({
        let gl_i106 = gEffectBattler.get();
        crate::gl!(gBattleMons[gl_i106].hp)
    }) == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
            .cast::<CArray<u8, 8>>()
            .cast_mut())[3]
            != MOVE_EFFECT_PAYDAY
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
            .cast::<CArray<u8, 8>>()
            .cast_mut())[3]
            != MOVE_EFFECT_STEAL_ITEM
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        crate::gl!(gBattleCommunication[3] = 0);
        return;
    }
    if ({
        let gl_i107 = gEffectBattler.get();
        crate::gl!(gBattleMons[gl_i107].status2)
    }) & STATUS2_SUBSTITUTE
        != 0
        && affectsUser != MOVE_EFFECT_AFFECTS_USER
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        crate::gl!(gBattleCommunication[3] = 0);
        return;
    }
    if crate::gl!(gBattleCommunication[3]) <= PRIMARY_STATUS_MOVE_EFFECT {
        'l1: {
            match sStatusFlagsForMoveEffects[crate::gl!(gBattleCommunication[3])] {
                STATUS1_SLEEP => {
                    if ({
                        let gl_i108 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i108].ability)
                    }) != ABILITY_SOUNDPROOF
                    {
                        gActiveBattler.set(0);
                        while gActiveBattler.get() < gBattlersCount.get()
                            && ({
                                let gl_i109 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i109].status2)
                            }) & STATUS2_UPROAR
                                == 0
                        {
                            gActiveBattler.set(gActiveBattler.get() + (1));
                        }
                    } else {
                        gActiveBattler.set(gBattlersCount.get());
                    }
                    if ({
                        let gl_i110 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i110].status1)
                    }) != 0
                    {
                        break 'l1;
                    }
                    if gActiveBattler.get() != gBattlersCount.get() {
                        break 'l1;
                    }
                    if ({
                        let gl_i111 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i111].ability)
                    }) == ABILITY_VITAL_SPIRIT
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i112 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i112].ability)
                    }) == ABILITY_INSOMNIA
                    {
                        break 'l1;
                    }
                    CancelMultiTurnMoves(gEffectBattler.get());
                    statusChanged = TRUE as u32;
                }
                STATUS1_POISON => {
                    if ({
                        let gl_i113 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i113].ability)
                    }) == ABILITY_IMMUNITY
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        gLastUsedAbility.set(ABILITY_IMMUNITY);
                        RecordAbilityBattle(gEffectBattler.get(), ABILITY_IMMUNITY);
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PSNPrevention
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        if gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                            crate::gl!(
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS
                            );
                            gHitMarker.set(gHitMarker.get() & (0xffffdfff));
                        } else {
                            crate::gl!(
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS
                            );
                        }
                        crate::gl!(gBattleCommunication[3] = 0);
                        return;
                    }
                    if (({
                        let gl_i114 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i114].types[0])
                    }) == TYPE_POISON
                        || ({
                            let gl_i115 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i115].types[1])
                        }) == TYPE_POISON
                        || (({
                            let gl_i116 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i116].types[0])
                        }) == TYPE_STEEL
                            || ({
                                let gl_i117 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i117].types[1])
                            }) == TYPE_STEEL))
                        && gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PSNPrevention
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        crate::gl!(gBattleCommunication[5] = B_MSG_STATUS_HAD_NO_EFFECT);
                        crate::gl!(gBattleCommunication[3] = 0);
                        return;
                    }
                    if ({
                        let gl_i118 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i118].types[0])
                    }) == TYPE_POISON
                        || ({
                            let gl_i119 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i119].types[1])
                        }) == TYPE_POISON
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i120 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i120].types[0])
                    }) == TYPE_STEEL
                        || ({
                            let gl_i121 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i121].types[1])
                        }) == TYPE_STEEL
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i122 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i122].status1)
                    }) != 0
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i123 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i123].ability)
                    }) == ABILITY_IMMUNITY
                    {
                        break 'l1;
                    }
                    statusChanged = TRUE as u32;
                }
                STATUS1_BURN => {
                    if ({
                        let gl_i124 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i124].ability)
                    }) == ABILITY_WATER_VEIL
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        gLastUsedAbility.set(ABILITY_WATER_VEIL);
                        RecordAbilityBattle(gEffectBattler.get(), ABILITY_WATER_VEIL);
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_BRNPrevention
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        if gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                            crate::gl!(
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS
                            );
                            gHitMarker.set(gHitMarker.get() & (0xffffdfff));
                        } else {
                            crate::gl!(
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS
                            );
                        }
                        crate::gl!(gBattleCommunication[3] = 0);
                        return;
                    }
                    if (({
                        let gl_i125 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i125].types[0])
                    }) == TYPE_FIRE
                        || ({
                            let gl_i126 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i126].types[1])
                        }) == TYPE_FIRE)
                        && gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_BRNPrevention
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        crate::gl!(gBattleCommunication[5] = B_MSG_STATUS_HAD_NO_EFFECT);
                        crate::gl!(gBattleCommunication[3] = 0);
                        return;
                    }
                    if ({
                        let gl_i127 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i127].types[0])
                    }) == TYPE_FIRE
                        || ({
                            let gl_i128 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i128].types[1])
                        }) == TYPE_FIRE
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i129 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i129].ability)
                    }) == ABILITY_WATER_VEIL
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i130 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i130].status1)
                    }) != 0
                    {
                        break 'l1;
                    }
                    statusChanged = TRUE as u32;
                }
                STATUS1_FREEZE => {
                    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
                        && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
                        && gBattleWeather.get() as i32 & B_WEATHER_SUN != 0
                    {
                        noSunCanFreeze = FALSE as u32;
                    }
                    if ({
                        let gl_i131 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i131].types[0])
                    }) == TYPE_ICE
                        || ({
                            let gl_i132 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i132].types[1])
                        }) == TYPE_ICE
                    {
                        break 'l1;
                    }
                    if ({
                        let gl_i133 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i133].status1)
                    }) != 0
                    {
                        break 'l1;
                    }
                    if noSunCanFreeze == FALSE as u32 {
                        break 'l1;
                    }
                    if ({
                        let gl_i134 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i134].ability)
                    }) == ABILITY_MAGMA_ARMOR
                    {
                        break 'l1;
                    }
                    CancelMultiTurnMoves(gEffectBattler.get());
                    statusChanged = TRUE as u32;
                }
                STATUS1_PARALYSIS => {
                    if ({
                        let gl_i135 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i135].ability)
                    }) == ABILITY_LIMBER
                    {
                        if primary == TRUE || certain == MOVE_EFFECT_CERTAIN {
                            gLastUsedAbility.set(ABILITY_LIMBER);
                            RecordAbilityBattle(gEffectBattler.get(), ABILITY_LIMBER);
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_PRLZPrevention
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            if gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                                crate::gl!(
                                    gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS
                                );
                                gHitMarker.set(gHitMarker.get() & (0xffffdfff));
                            } else {
                                crate::gl!(
                                    gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS
                                );
                            }
                            crate::gl!(gBattleCommunication[3] = 0);
                            return;
                        } else {
                            break 'l1;
                        }
                    }
                    if ({
                        let gl_i136 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i136].status1)
                    }) != 0
                    {
                        break 'l1;
                    }
                    statusChanged = TRUE as u32;
                }
                STATUS1_TOXIC_POISON => {
                    if ({
                        let gl_i137 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i137].ability)
                    }) == ABILITY_IMMUNITY
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        gLastUsedAbility.set(ABILITY_IMMUNITY);
                        RecordAbilityBattle(gEffectBattler.get(), ABILITY_IMMUNITY);
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PSNPrevention
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        if gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                            crate::gl!(
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS
                            );
                            gHitMarker.set(gHitMarker.get() & (0xffffdfff));
                        } else {
                            crate::gl!(
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS
                            );
                        }
                        crate::gl!(gBattleCommunication[3] = 0);
                        return;
                    }
                    if (({
                        let gl_i138 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i138].types[0])
                    }) == TYPE_POISON
                        || ({
                            let gl_i139 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i139].types[1])
                        }) == TYPE_POISON
                        || (({
                            let gl_i140 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i140].types[0])
                        }) == TYPE_STEEL
                            || ({
                                let gl_i141 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i141].types[1])
                            }) == TYPE_STEEL))
                        && gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PSNPrevention
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        crate::gl!(gBattleCommunication[5] = B_MSG_STATUS_HAD_NO_EFFECT);
                        crate::gl!(gBattleCommunication[3] = 0);
                        return;
                    }
                    if ({
                        let gl_i142 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i142].status1)
                    }) != 0
                    {
                        break 'l1;
                    }
                    if !(({
                        let gl_i143 = gEffectBattler.get();
                        crate::gl!(gBattleMons[gl_i143].types[0])
                    }) == TYPE_POISON
                        || ({
                            let gl_i144 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i144].types[1])
                        }) == TYPE_POISON)
                        && !(({
                            let gl_i145 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i145].types[0])
                        }) == TYPE_STEEL
                            || ({
                                let gl_i146 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i146].types[1])
                            }) == TYPE_STEEL)
                    {
                        if ({
                            let gl_i147 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i147].ability)
                        }) == ABILITY_IMMUNITY
                        {
                            break 'l1;
                        }
                        {
                            let gl_i148 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i148].status1 &= 0xffffff7f)
                        };
                        {
                            let gl_i149 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i149].status1 &= 0xfffffff7)
                        };
                        statusChanged = TRUE as u32;
                        break 'l1;
                    } else {
                        gMoveResultFlags
                            .set(gMoveResultFlags.get() | (MOVE_RESULT_DOESNT_AFFECT_FOE));
                    }
                }
                _ => {}
            }
        }
        if statusChanged == TRUE as u32 {
            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
            if sStatusFlagsForMoveEffects[crate::gl!(gBattleCommunication[3])] == STATUS1_SLEEP {
                {
                    let gl_v = (Random() as u32 & 3) + 2;
                    let gl_i150 = gEffectBattler.get();
                    crate::gl!(gBattleMons[gl_i150].status1 |= gl_v)
                };
            } else {
                {
                    let gl_v = sStatusFlagsForMoveEffects[crate::gl!(gBattleCommunication[3])];
                    let gl_i151 = gEffectBattler.get();
                    crate::gl!(gBattleMons[gl_i151].status1 |= gl_v)
                };
            }
            crate::gl!(
                gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
            );
            gActiveBattler.set(gEffectBattler.get());
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_STATUS_BATTLE,
                0,
                4,
                ({
                    let gl_i152 = gEffectBattler.get();
                    crate::gl!(&raw gBattleMons[gl_i152].status1)
                }) as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
            if gHitMarker.get() & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                crate::gl!(gBattleCommunication[5] = B_MSG_STATUSED_BY_ABILITY);
                gHitMarker.set(gHitMarker.get() & (0xffffdfff));
            } else {
                crate::gl!(gBattleCommunication[5] = B_MSG_STATUSED);
            }
            if crate::gl!(gBattleCommunication[3]) == MOVE_EFFECT_POISON
                || crate::gl!(gBattleCommunication[3]) == MOVE_EFFECT_TOXIC
                || crate::gl!(gBattleCommunication[3]) == MOVE_EFFECT_PARALYSIS
                || crate::gl!(gBattleCommunication[3]) == 3
            {
                let synchronizeEffect: *mut u8 =
                    &raw mut (*crate::gl!(gBattleStruct)).synchronizeMoveEffect;
                *synchronizeEffect = crate::gl!(gBattleCommunication[3]);
                gHitMarker.set(gHitMarker.get() | (HITMARKER_SYNCHRONIZE_EFFECT));
            }
            return;
        } else if statusChanged == FALSE as u32 {
            crate::gl!(gBattleCommunication[3] = 0);
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
            return;
        }
        return;
    } else {
        if ({
            let gl_i153 = gEffectBattler.get();
            crate::gl!(gBattleMons[gl_i153].status2)
        }) & sStatusFlagsForMoveEffects[crate::gl!(gBattleCommunication[3])]
            != 0
        {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        } else {
            let mut side: u8 = 0;
            'l3: {
                match crate::gl!(gBattleCommunication[3]) {
                    MOVE_EFFECT_CONFUSION => {
                        if ({
                            let gl_i154 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i154].ability)
                        }) == ABILITY_OWN_TEMPO
                            || ({
                                let gl_i155 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i155].status2)
                            }) & STATUS2_CONFUSION
                                != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            {
                                let gl_v = (Random() as i32 % 4) as u32 + 2;
                                let gl_i156 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i156].status2 |= gl_v)
                            };
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
                            );
                        }
                    }
                    MOVE_EFFECT_FLINCH => {
                        if ({
                            let gl_i157 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i157].ability)
                        }) == ABILITY_INNER_FOCUS
                        {
                            if primary == TRUE || certain == MOVE_EFFECT_CERTAIN {
                                gLastUsedAbility.set(ABILITY_INNER_FOCUS);
                                RecordAbilityBattle(gEffectBattler.get(), ABILITY_INNER_FOCUS);
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_FlinchPrevention
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                            } else {
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*gBattlescriptCurrInstr.as_ptr()).at(1)
                                );
                            }
                        } else {
                            if GetBattlerTurnOrderNum(gEffectBattler.get())
                                > gCurrentTurnActionNumber.get()
                            {
                                {
                                    let gl_v = sStatusFlagsForMoveEffects
                                        [crate::gl!(gBattleCommunication[3])];
                                    let gl_i158 = gEffectBattler.get();
                                    crate::gl!(gBattleMons[gl_i158].status2 |= gl_v)
                                };
                            }
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        }
                    }
                    MOVE_EFFECT_UPROAR => {
                        if ({
                            let gl_i159 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i159].status2)
                        }) & STATUS2_UPROAR
                            == 0
                        {
                            {
                                let gl_i160 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i160].status2 |= STATUS2_MULTIPLETURNS)
                            };
                            {
                                let gl_v = gCurrentMove.get();
                                let gl_i49 = gEffectBattler.get();
                                crate::gl!(gLockedMoves[gl_i49] = gl_v)
                            };
                            {
                                let gl_v = ((Random() as u32 & 3) + 2) << 4;
                                let gl_i161 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i161].status2 |= gl_v)
                            };
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
                            );
                        } else {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        }
                    }
                    MOVE_EFFECT_PAYDAY => {
                        if GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1
                            == B_SIDE_PLAYER as i32
                        {
                            let payday: u16 = gPaydayMoney.get();
                            gPaydayMoney.set(
                                gPaydayMoney.get()
                                    + (({
                                        let gl_i162 = gBattlerAttacker.get();
                                        crate::gl!(gBattleMons[gl_i162].level)
                                    }) as u16
                                        * 5),
                            );
                            if payday > gPaydayMoney.get() {
                                gPaydayMoney.set(0xFFFF);
                            }
                        }
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
                        );
                    }
                    MOVE_EFFECT_TRI_ATTACK => {
                        if ({
                            let gl_i163 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i163].status1)
                        }) != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            crate::gl!(gBattleCommunication[3] = (Random() as i32 % 3) as u8 + 3);
                            SetMoveEffect(0, 0);
                        }
                    }
                    MOVE_EFFECT_CHARGING => {
                        {
                            let gl_i164 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i164].status2 |= STATUS2_MULTIPLETURNS)
                        };
                        {
                            let gl_v = gCurrentMove.get();
                            let gl_i50 = gEffectBattler.get();
                            crate::gl!(gLockedMoves[gl_i50] = gl_v)
                        };
                        (*gProtectStructs.as_ptr())[gEffectBattler.get()].set_chargingTurn(1);
                        crate::gl!(
                            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                        );
                    }
                    MOVE_EFFECT_WRAP => {
                        if ({
                            let gl_i165 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i165].status2)
                        }) & STATUS2_WRAPPED
                            != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            {
                                let gl_v = ((Random() as u32 & 3) + 3) << 13;
                                let gl_i166 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i166].status2 |= gl_v)
                            };
                            *(*crate::gl!(gBattleStruct))
                                .wrappedMove
                                .as_mut_ptr()
                                .at(gEffectBattler.get() as i32 * 2) = gCurrentMove.get() as u8;
                            *(*crate::gl!(gBattleStruct))
                                .wrappedMove
                                .as_mut_ptr()
                                .at(gEffectBattler.get() as i32 * 2)
                                .at(1) = (gCurrentMove.get() >> 8) as u8;
                            *(*crate::gl!(gBattleStruct))
                                .wrappedBy
                                .as_mut_ptr()
                                .at(gEffectBattler.get()) = gBattlerAttacker.get();
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
                            );
                            crate::gl!(gBattleCommunication[5] = 0);
                            loop {
                                if crate::gl!(gBattleCommunication[5]) >= MULTISTRING_CHOOSER {
                                    break;
                                }
                                if (*(&raw const crate::data::battle_message::gTrappingMoves)
                                    .cast::<CArray<u16, 0>>())[crate::gl!(gBattleCommunication[5])]
                                    == gCurrentMove.get()
                                {
                                    break;
                                }
                                crate::gl!(gBattleCommunication[5] += 1);
                            }
                        }
                    }
                    MOVE_EFFECT_RECOIL_25 => {
                        gBattleMoveDamage.set(gHpDealt.get() / 4);
                        if gBattleMoveDamage.get() == 0 {
                            gBattleMoveDamage.set(1);
                        }
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
                        );
                    }
                    MOVE_EFFECT_ATK_PLUS_1
                    | MOVE_EFFECT_DEF_PLUS_1
                    | MOVE_EFFECT_SPD_PLUS_1
                    | MOVE_EFFECT_SP_ATK_PLUS_1
                    | MOVE_EFFECT_SP_DEF_PLUS_1
                    | MOVE_EFFECT_ACC_PLUS_1
                    | MOVE_EFFECT_EVS_PLUS_1 => {
                        if ChangeStatBuffs(
                            16,
                            crate::gl!(gBattleCommunication[3]) - MOVE_EFFECT_ATK_PLUS_1 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            crate::gl!(
                                gBattleScripting.animArg1 =
                                    crate::gl!(gBattleCommunication[3]) & 63
                            );
                            crate::gl!(gBattleScripting.animArg2 = 0);
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_StatUp
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                            );
                        }
                    }
                    MOVE_EFFECT_ATK_MINUS_1
                    | MOVE_EFFECT_DEF_MINUS_1
                    | MOVE_EFFECT_SPD_MINUS_1
                    | MOVE_EFFECT_SP_ATK_MINUS_1
                    | MOVE_EFFECT_SP_DEF_MINUS_1
                    | MOVE_EFFECT_ACC_MINUS_1
                    | MOVE_EFFECT_EVS_MINUS_1 => {
                        if ChangeStatBuffs(
                            -112,
                            crate::gl!(gBattleCommunication[3]) - MOVE_EFFECT_ATK_MINUS_1 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            crate::gl!(
                                gBattleScripting.animArg1 =
                                    crate::gl!(gBattleCommunication[3]) & 63
                            );
                            crate::gl!(gBattleScripting.animArg2 = 0);
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_StatDown
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                            );
                        }
                    }
                    MOVE_EFFECT_ATK_PLUS_2
                    | MOVE_EFFECT_DEF_PLUS_2
                    | MOVE_EFFECT_SPD_PLUS_2
                    | MOVE_EFFECT_SP_ATK_PLUS_2
                    | MOVE_EFFECT_SP_DEF_PLUS_2
                    | MOVE_EFFECT_ACC_PLUS_2
                    | MOVE_EFFECT_EVS_PLUS_2 => {
                        if ChangeStatBuffs(
                            32,
                            crate::gl!(gBattleCommunication[3]) - MOVE_EFFECT_ATK_PLUS_2 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            crate::gl!(
                                gBattleScripting.animArg1 =
                                    crate::gl!(gBattleCommunication[3]) & 63
                            );
                            crate::gl!(gBattleScripting.animArg2 = 0);
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_StatUp
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                            );
                        }
                    }
                    MOVE_EFFECT_ATK_MINUS_2
                    | MOVE_EFFECT_DEF_MINUS_2
                    | MOVE_EFFECT_SPD_MINUS_2
                    | MOVE_EFFECT_SP_ATK_MINUS_2
                    | MOVE_EFFECT_SP_DEF_MINUS_2
                    | MOVE_EFFECT_ACC_MINUS_2
                    | MOVE_EFFECT_EVS_MINUS_2 => {
                        if ChangeStatBuffs(
                            -96,
                            crate::gl!(gBattleCommunication[3]) - MOVE_EFFECT_ATK_MINUS_2 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            crate::gl!(
                                gBattleScripting.animArg1 =
                                    crate::gl!(gBattleCommunication[3]) & 63
                            );
                            crate::gl!(gBattleScripting.animArg2 = 0);
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_StatDown
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                            );
                        }
                    }
                    MOVE_EFFECT_RECHARGE => {
                        {
                            let gl_i167 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i167].status2 |= STATUS2_RECHARGE)
                        };
                        {
                            let gl_i168 = gEffectBattler.get();
                            crate::gl!(gDisableStructs[gl_i168].rechargeTimer = 2)
                        };
                        {
                            let gl_v = gCurrentMove.get();
                            let gl_i51 = gEffectBattler.get();
                            crate::gl!(gLockedMoves[gl_i51] = gl_v)
                        };
                        crate::gl!(
                            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                        );
                    }
                    MOVE_EFFECT_RAGE => {
                        {
                            let gl_i169 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i169].status2 |= STATUS2_RAGE)
                        };
                        crate::gl!(
                            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                        );
                    }
                    MOVE_EFFECT_STEAL_ITEM => {
                        if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER_HILL != 0 {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                            break 'l3;
                        }
                        side = GetBattlerSide(gBattlerAttacker.get());
                        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_OPPONENT
                            && gBattleTypeFlags.get() & 0xa3f0902 == 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else if gBattleTypeFlags.get() & 0xa3f0902 == 0
                            && crate::gl!(gWishFutureKnock.knockedOffMons[side]) as u32
                                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[{
                                    let gl_i170 = gBattlerAttacker.get();
                                    crate::gl!(gBattlerPartyIndexes[gl_i170])
                                }]
                                != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else if ({
                            let gl_i171 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i171].item)
                        }) != 0
                            && ({
                                let gl_i172 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i172].ability)
                            }) == ABILITY_STICKY_HOLD
                        {
                            BattleScriptPushCursor();
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_NoItemSteal
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            gLastUsedAbility.set({
                                let gl_i173 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i173].ability)
                            });
                            RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
                        } else if ({
                            let gl_i174 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i174].item)
                        }) != ITEM_NONE
                            || ({
                                let gl_i175 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i175].item)
                            }) == ITEM_ENIGMA_BERRY
                            || (({
                                let gl_i176 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i176].item)
                            }) == ITEM_ORANGE_MAIL
                                || ({
                                    let gl_i177 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i177].item)
                                }) == ITEM_HARBOR_MAIL
                                || ({
                                    let gl_i178 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i178].item)
                                }) == ITEM_GLITTER_MAIL
                                || ({
                                    let gl_i179 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i179].item)
                                }) == ITEM_MECH_MAIL
                                || ({
                                    let gl_i180 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i180].item)
                                }) == ITEM_WOOD_MAIL
                                || ({
                                    let gl_i181 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i181].item)
                                }) == ITEM_WAVE_MAIL
                                || ({
                                    let gl_i182 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i182].item)
                                }) == ITEM_BEAD_MAIL
                                || ({
                                    let gl_i183 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i183].item)
                                }) == ITEM_SHADOW_MAIL
                                || ({
                                    let gl_i184 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i184].item)
                                }) == ITEM_TROPIC_MAIL
                                || ({
                                    let gl_i185 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i185].item)
                                }) == ITEM_DREAM_MAIL
                                || ({
                                    let gl_i186 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i186].item)
                                }) == ITEM_FAB_MAIL
                                || ({
                                    let gl_i187 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i187].item)
                                }) == ITEM_RETRO_MAIL)
                            || ({
                                let gl_i188 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i188].item)
                            }) == ITEM_NONE
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            let changedItem: *mut u16 = &raw mut (*crate::gl!(gBattleStruct))
                                .changedItems[gBattlerAttacker.get()];
                            gLastUsedItem.set({
                                *changedItem = {
                                    let gl_i189 = gBattlerTarget.get();
                                    crate::gl!(gBattleMons[gl_i189].item)
                                };
                                *changedItem
                            });
                            {
                                let gl_i190 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i190].item = ITEM_NONE)
                            };
                            gActiveBattler.set(gBattlerAttacker.get());
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_HELDITEM_BATTLE,
                                0,
                                2,
                                gLastUsedItem.as_ptr() as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gBattlerAttacker.get());
                            gActiveBattler.set(gBattlerTarget.get());
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_HELDITEM_BATTLE,
                                0,
                                2,
                                ({
                                    let gl_i191 = gBattlerTarget.get();
                                    crate::gl!(&raw gBattleMons[gl_i191].item)
                                }) as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gBattlerTarget.get());
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ItemSteal
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                            );
                            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove
                                [gBattlerTarget.get()] as *mut u8) = 0;
                            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove
                                [gBattlerTarget.get()] as *mut u8)
                                .at(1) = 0;
                        }
                    }
                    MOVE_EFFECT_PREVENT_ESCAPE => {
                        {
                            let gl_i192 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i192].status2 |= STATUS2_ESCAPE_PREVENTION)
                        };
                        {
                            let gl_i193 = gBattlerTarget.get();
                            crate::gl!(
                                gDisableStructs[gl_i193].battlerPreventingEscape =
                                    gBattlerAttacker.get()
                            )
                        };
                        crate::gl!(
                            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                        );
                    }
                    MOVE_EFFECT_NIGHTMARE => {
                        {
                            let gl_i194 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i194].status2 |= STATUS2_NIGHTMARE)
                        };
                        crate::gl!(
                            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                        );
                    }
                    MOVE_EFFECT_ALL_STATS_UP => {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_AllStatsUp
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                    }
                    MOVE_EFFECT_RAPIDSPIN => {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_RapidSpinAway
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                    }
                    MOVE_EFFECT_REMOVE_PARALYSIS => {
                        if ({
                            let gl_i195 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i195].status1)
                        }) & STATUS1_PARALYSIS
                            == 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            {
                                let gl_i196 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i196].status1 &= 0xffffffbf)
                            };
                            gActiveBattler.set(gBattlerTarget.get());
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_STATUS_BATTLE,
                                0,
                                4,
                                ({
                                    let gl_i197 = gActiveBattler.get();
                                    crate::gl!(&raw gBattleMons[gl_i197].status1)
                                }) as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gActiveBattler.get());
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_TargetPRLZHeal
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                        }
                    }
                    MOVE_EFFECT_ATK_DEF_DOWN => {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_AtkDefDown
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                    }
                    MOVE_EFFECT_RECOIL_33 => {
                        gBattleMoveDamage.set(gHpDealt.get() / 3);
                        if gBattleMoveDamage.get() == 0 {
                            gBattleMoveDamage.set(1);
                        }
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                sMoveEffectBS_Ptrs[crate::gl!(gBattleCommunication[3])]
                        );
                    }
                    MOVE_EFFECT_THRASH => {
                        if ({
                            let gl_i198 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i198].status2)
                        }) & STATUS2_LOCK_CONFUSE
                            != 0
                        {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        } else {
                            {
                                let gl_i199 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i199].status2 |= STATUS2_MULTIPLETURNS)
                            };
                            {
                                let gl_v = gCurrentMove.get();
                                let gl_i52 = gEffectBattler.get();
                                crate::gl!(gLockedMoves[gl_i52] = gl_v)
                            };
                            {
                                let gl_v = ((Random() as u32 & 1) + 2) << 10;
                                let gl_i200 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i200].status2 |= gl_v)
                            };
                        }
                    }
                    MOVE_EFFECT_KNOCK_OFF => {
                        if ({
                            let gl_i201 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i201].ability)
                        }) == ABILITY_STICKY_HOLD
                        {
                            if ({
                                let gl_i202 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i202].item)
                            }) == ITEM_NONE
                            {
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*gBattlescriptCurrInstr.as_ptr()).at(1)
                                );
                            } else {
                                gLastUsedAbility.set(ABILITY_STICKY_HOLD);
                                crate::gl!(
                                    gBattlescriptCurrInstr =
                                        (*crate::asmdata::BattleScript_StickyHoldActivates
                                            .cast::<CArray<u8, 0>>())
                                        .as_ptr()
                                        .cast_mut()
                                );
                                RecordAbilityBattle(gEffectBattler.get(), ABILITY_STICKY_HOLD);
                            }
                            break 'l3;
                        }
                        if ({
                            let gl_i203 = gEffectBattler.get();
                            crate::gl!(gBattleMons[gl_i203].item)
                        }) != 0
                        {
                            side = GetBattlerSide(gEffectBattler.get());
                            gLastUsedItem.set({
                                let gl_i204 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i204].item)
                            });
                            {
                                let gl_i205 = gEffectBattler.get();
                                crate::gl!(gBattleMons[gl_i205].item = ITEM_NONE)
                            };
                            crate::gl!(
                                gWishFutureKnock.knockedOffMons[side] |= gBitTable[{
                                    let gl_i206 = gEffectBattler.get();
                                    crate::gl!(gBattlerPartyIndexes[gl_i206])
                                }]
                                    as u8
                            );
                            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                            crate::gl!(
                                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_KnockedOff
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                            );
                            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove
                                [gEffectBattler.get()] as *mut u8) = 0;
                            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove
                                [gEffectBattler.get()] as *mut u8)
                                .at(1) = 0;
                        } else {
                            crate::gl!(
                                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1)
                            );
                        }
                    }
                    MOVE_EFFECT_SP_ATK_TWO_DOWN => {
                        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SAtkDown2
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                    }
                    _ => {}
                }
            }
        }
    }
    crate::gl!(gBattleCommunication[3] = 0);
}
pub(crate) unsafe fn Cmd_seteffectwithchance() {
    let mut percentChance: u32 = 0;
    if ({
        let gl_i207 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i207].ability)
    }) == ABILITY_SERENE_GRACE
    {
        percentChance = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .secondaryEffectChance as u32
            * 2;
    } else {
        percentChance = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .secondaryEffectChance as u32;
    }
    if crate::gl!(gBattleCommunication[3]) as i32 & MOVE_EFFECT_CERTAIN as i32 != 0
        && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
    {
        crate::gl!(gBattleCommunication[3] &= 127);
        SetMoveEffect(FALSE, MOVE_EFFECT_CERTAIN);
    } else if ((Random() as i32 % 100) as u32) < percentChance
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
            .cast::<CArray<u8, 8>>()
            .cast_mut())[3]
            != 0
        && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
    {
        if percentChance >= 100 {
            SetMoveEffect(FALSE, MOVE_EFFECT_CERTAIN);
        } else {
            SetMoveEffect(0, 0);
        }
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
    crate::gl!(gBattleCommunication[3] = 0);
    crate::gl!(gBattleScripting.multihitMoveEffect = 0);
}
pub(crate) unsafe fn Cmd_seteffectprimary() {
    SetMoveEffect(TRUE, 0);
}
pub(crate) unsafe fn Cmd_seteffectsecondary() {
    SetMoveEffect(0, 0);
}
pub(crate) unsafe fn Cmd_clearstatusfromeffect() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    if crate::gl!(gBattleCommunication[3]) <= PRIMARY_STATUS_MOVE_EFFECT {
        {
            let gl_v = !sStatusFlagsForMoveEffects[crate::gl!(gBattleCommunication[3])];
            let gl_i208 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i208].status1 &= gl_v)
        };
    } else {
        {
            let gl_v = !sStatusFlagsForMoveEffects[crate::gl!(gBattleCommunication[3])];
            let gl_i209 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i209].status2 &= gl_v)
        };
    }
    crate::gl!(gBattleCommunication[3] = 0);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    crate::gl!(gBattleScripting.multihitMoveEffect = 0);
}
pub(crate) unsafe fn Cmd_tryfaintmon() {
    let mut BS_ptr: *mut u8 = null_mut();
    if *(*gBattlescriptCurrInstr.as_ptr()).at(2) != 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        if gHitMarker.get()
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
                << 28
            != 0
        {
            BS_ptr = (*(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8;
            BattleScriptPop();
            crate::gl!(gBattlescriptCurrInstr = BS_ptr);
            {
                let gl_v = 65023;
                let gl_i53 = GetBattlerSide(gActiveBattler.get());
                crate::gl!(gSideStatuses[gl_i53] &= gl_v)
            };
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        }
    } else {
        let mut battler: u8 = 0;
        if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == 1 {
            gActiveBattler.set(gBattlerAttacker.get());
            battler = gBattlerTarget.get();
            BS_ptr = (*crate::asmdata::BattleScript_FaintAttacker.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        } else {
            gActiveBattler.set(gBattlerTarget.get());
            battler = gBattlerAttacker.get();
            BS_ptr = (*crate::asmdata::BattleScript_FaintTarget.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
            == 0
            && ({
                let gl_i210 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i210].hp)
            }) == 0
        {
            gHitMarker.set(gHitMarker.get() | (gBitTable[gActiveBattler.get()] << 28));
            BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(7));
            crate::gl!(gBattlescriptCurrInstr = BS_ptr);
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
                gHitMarker.set(gHitMarker.get() | (HITMARKER_PLAYER_FAINTED));
                crate::gl!(
                    gBattleResults.playerFaintCounter = (*gBattleResults.as_ptr())
                        .playerFaintCounter
                        .saturating_add(1)
                );
                AdjustFriendshipOnBattleFaint(gActiveBattler.get());
            } else {
                crate::gl!(
                    gBattleResults.opponentFaintCounter = (*gBattleResults.as_ptr())
                        .opponentFaintCounter
                        .saturating_add(1)
                );
                crate::gl!(
                    gBattleResults.lastOpponentSpecies = GetMonData3(
                        &raw mut (*gEnemyParty.as_ptr())[{
                            let gl_i211 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i211])
                        }],
                        MON_DATA_SPECIES,
                        null_mut(),
                    ) as u16
                );
            }
            if gHitMarker.get() & HITMARKER_DESTINYBOND != 0
                && ({
                    let gl_i212 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i212].hp)
                }) != 0
            {
                gHitMarker.set(gHitMarker.get() & (0xffffffbf));
                BattleScriptPush(crate::gl!(gBattlescriptCurrInstr));
                gBattleMoveDamage.set(crate::gl!(gBattleMons[battler].hp) as i32);
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_DestinyBondTakesLife
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
            }
            if {
                let gl_i54 = gBattlerTarget.get();
                crate::gl!(gStatuses3[gl_i54])
            } & STATUS3_GRUDGE
                != 0
                && gHitMarker.get() & HITMARKER_GRUDGE == 0
                && GetBattlerSide(gBattlerAttacker.get()) != GetBattlerSide(gBattlerTarget.get())
                && ({
                    let gl_i213 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i213].hp)
                }) != 0
                && gCurrentMove.get() != MOVE_STRUGGLE
            {
                let moveIndex: u8 = *(*crate::gl!(gBattleStruct))
                    .chosenMovePositions
                    .as_mut_ptr()
                    .at(gBattlerAttacker.get());
                {
                    let gl_i214 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i214].pp[moveIndex] = 0)
                };
                BattleScriptPush(crate::gl!(gBattlescriptCurrInstr));
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_GrudgeTakesPP
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                gActiveBattler.set(gBattlerAttacker.get());
                BtlController_EmitSetMonData(
                    B_COMM_TO_CONTROLLER,
                    moveIndex + REQUEST_PPMOVE1_BATTLE,
                    0,
                    1,
                    ({
                        let gl_i215 = gActiveBattler.get();
                        crate::gl!(&raw gBattleMons[gl_i215].pp[moveIndex])
                    }) as *mut c_void,
                );
                MarkBattlerForControllerExec(gActiveBattler.get());
                crate::gl!(gBattleTextBuff1[0] = 0xFD);
                crate::gl!(gBattleTextBuff1[1] = 2);
                crate::gl!(
                    gBattleTextBuff1[2] = ({
                        let gl_i216 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i216].moves[moveIndex])
                    }) as u8
                );
                crate::gl!(
                    gBattleTextBuff1[3] = ((({
                        let gl_i217 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i217].moves[moveIndex])
                    }) as i32
                        & 0xFF00)
                        >> 8) as u8
                );
                crate::gl!(gBattleTextBuff1[4] = 0xFF);
            }
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        }
    }
}
pub(crate) unsafe fn Cmd_dofaintanimation() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        BtlController_EmitFaintAnimation(B_COMM_TO_CONTROLLER);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    }
}
pub(crate) unsafe fn Cmd_cleareffectsonfaint() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA == 0
            || ({
                let gl_i218 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i218].hp)
            }) == 0
        {
            {
                let gl_i219 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i219].status1 = 0)
            };
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_STATUS_BATTLE,
                0,
                4,
                ({
                    let gl_i220 = gActiveBattler.get();
                    crate::gl!(&raw gBattleMons[gl_i220].status1)
                }) as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        FaintClearSetData();
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    }
}
pub(crate) unsafe fn Cmd_jumpifstatus() {
    let battler: u8 = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    let flags: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(2) as u32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as u32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as u32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as u32) << 24);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(6) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if crate::gl!(gBattleMons[battler].status1) & flags != 0
        && crate::gl!(gBattleMons[battler].hp) != 0
    {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_jumpifstatus2() {
    let battler: u8 = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    let flags: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(2) as u32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as u32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as u32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as u32) << 24);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(6) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if crate::gl!(gBattleMons[battler].status2) & flags != 0
        && crate::gl!(gBattleMons[battler].hp) != 0
    {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_jumpifability() {
    let mut battler: u8 = 0;
    let ability: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(2);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_ATTACKER_SIDE {
        battler = AbilityBattleEffects(
            ABILITYEFFECT_CHECK_BATTLER_SIDE,
            gBattlerAttacker.get(),
            ability,
            0,
            0,
        );
        if battler != 0 {
            gLastUsedAbility.set(ability);
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            RecordAbilityBattle(battler - 1, gLastUsedAbility.get());
            crate::gl!(gBattleScripting.battlerWithAbility = battler - 1);
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        }
    } else if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_NOT_ATTACKER_SIDE {
        battler = AbilityBattleEffects(
            ABILITYEFFECT_CHECK_OTHER_SIDE,
            gBattlerAttacker.get(),
            ability,
            0,
            0,
        );
        if battler != 0 {
            gLastUsedAbility.set(ability);
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            RecordAbilityBattle(battler - 1, gLastUsedAbility.get());
            crate::gl!(gBattleScripting.battlerWithAbility = battler - 1);
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        }
    } else {
        battler = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
        if crate::gl!(gBattleMons[battler].ability) == ability {
            gLastUsedAbility.set(ability);
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            RecordAbilityBattle(battler, gLastUsedAbility.get());
            crate::gl!(gBattleScripting.battlerWithAbility = battler);
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
        }
    }
}
pub(crate) unsafe fn Cmd_jumpifsideaffecting() {
    let mut side: u8 = 0;
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == 1 {
        side = GetBattlerPosition(gBattlerAttacker.get()) & 1;
    } else {
        side = GetBattlerPosition(gBattlerTarget.get()) & 1;
    }
    let flags: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(2) as u16
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as u16) << 8);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(4) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(4).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(4).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(4).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if crate::gl!(gSideStatuses[side]) as i32 & flags as i32 != 0 {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(8));
    }
}
pub(crate) unsafe fn Cmd_jumpifstat() {
    let mut ret: u8 = 0;
    let battler: u8 = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    let value: u8 = ({
        let gl_i221 = *(*gBattlescriptCurrInstr.as_ptr()).at(3);
        crate::gl!(gBattleMons[battler].statStages[gl_i221])
    }) as u8;
    match *(*gBattlescriptCurrInstr.as_ptr()).at(2) {
        CMP_EQUAL => {
            if value == *(*gBattlescriptCurrInstr.as_ptr()).at(4) {
                ret += 1;
            }
        }
        CMP_NOT_EQUAL => {
            if value != *(*gBattlescriptCurrInstr.as_ptr()).at(4) {
                ret += 1;
            }
        }
        CMP_GREATER_THAN => {
            if value > *(*gBattlescriptCurrInstr.as_ptr()).at(4) {
                ret += 1;
            }
        }
        CMP_LESS_THAN => {
            if value < *(*gBattlescriptCurrInstr.as_ptr()).at(4) {
                ret += 1;
            }
        }
        CMP_COMMON_BITS => {
            if value as i32 & *(*gBattlescriptCurrInstr.as_ptr()).at(4) as i32 != 0 {
                ret += 1;
            }
        }
        CMP_NO_COMMON_BITS
            if value as i32 & *(*gBattlescriptCurrInstr.as_ptr()).at(4) as i32 == 0 =>
        {
            ret += 1;
        }
        _ => {}
    }
    if ret != 0 {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8)
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16)
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24))
                as usize as *mut c_void as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(9));
    }
}
pub(crate) unsafe fn Cmd_jumpifstatus3condition() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let status: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(2) as u32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as u32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as u32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as u32) << 24);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(7) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(7).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(7).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(7).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if *(*gBattlescriptCurrInstr.as_ptr()).at(6) != 0 {
        if {
            let gl_i55 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i55])
        } & status
            != 0
        {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(11));
        } else {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        }
    } else {
        if {
            let gl_i56 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i56])
        } & status
            != 0
        {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(11));
        }
    }
}
pub(crate) unsafe fn Cmd_jumpiftype() {
    let battler: u8 = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    let r#type: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(2);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if crate::gl!(gBattleMons[battler].types[0]) == r#type
        || crate::gl!(gBattleMons[battler].types[1]) == r#type
    {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_getexp() {
    let mut item: u16 = 0;
    let mut i: i32 = 0;
    let mut holdEffect: u8 = 0;
    let mut viaExpShare: i32 = 0;
    let exp: *mut u16 = &raw mut (*crate::gl!(gBattleStruct)).expValue;
    gBattlerFainted.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let sentIn: i32 = {
        let gl_i57 = (gBattlerFainted.get() as i32 & 2) >> 1;
        crate::gl!(gSentPokesToOpponent[gl_i57])
    } as i32;
    'l1: {
        let sw1: u8 = crate::gl!(gBattleScripting.getexpState);
        let mut fall = false;
        if sw1 == 0 {
            if GetBattlerSide(gBattlerFainted.get()) != B_SIDE_OPPONENT
                || gBattleTypeFlags.get() & 0x63f0982 != 0
            {
                crate::gl!(gBattleScripting.getexpState = 6);
            } else {
                crate::gl!(gBattleScripting.getexpState += 1);
                (*crate::gl!(gBattleStruct)).givenExpMons |= gBitTable[{
                    let gl_i222 = gBattlerFainted.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i222])
                }] as u8;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            {
                let mut viaSentIn: i32 = 0;
                for i in 0..PARTY_SIZE {
                    'l2: {
                        if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES) == 0
                            || GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) == 0
                        {
                            break 'l2;
                        }
                        if gBitTable[i] & sentIn as u32 != 0 {
                            viaSentIn += 1;
                        }
                        item = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HELD_ITEM)
                            as u16;
                        if item == ITEM_ENIGMA_BERRY {
                            holdEffect = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffect;
                        } else {
                            holdEffect = GetItemHoldEffect(item);
                        }
                        if holdEffect == HOLD_EFFECT_EXP_SHARE {
                            viaExpShare += 1;
                        }
                    }
                }
                let calculatedExp: u16 = ((*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[{
                    let gl_i223 = gBattlerFainted.get();
                    crate::gl!(gBattleMons[gl_i223].species)
                }]
                .expYield as i32
                    * ({
                        let gl_i224 = gBattlerFainted.get();
                        crate::gl!(gBattleMons[gl_i224].level)
                    }) as i32
                    / 7) as u16;
                if viaExpShare != 0 {
                    *exp = (if viaSentIn != 0 {
                        div_i32(calculatedExp as i32 / 2, viaSentIn)
                    } else {
                        0
                    }) as u16;
                    if *exp == 0 {
                        *exp = 1;
                    }
                    gExpShareExp.set(div_i32(calculatedExp as i32 / 2, viaExpShare) as u16);
                    if gExpShareExp.get() == 0 {
                        gExpShareExp.set(1);
                    }
                } else {
                    *exp = (if viaSentIn != 0 {
                        div_i32(calculatedExp as i32, viaSentIn)
                    } else {
                        0
                    }) as u16;
                    if *exp == 0 {
                        *exp = 1;
                    }
                    gExpShareExp.set(0);
                }
                crate::gl!(gBattleScripting.getexpState += 1);
                (*crate::gl!(gBattleStruct)).expGetterMonId = 0;
                (*crate::gl!(gBattleStruct)).sentInPokes = sentIn as u8;
            }
        }
        if fall || sw1 == 2 {
            if gBattleControllerExecFlags.get() == 0 {
                item = GetMonData2(
                    &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
                    MON_DATA_HELD_ITEM,
                ) as u16;
                if item == ITEM_ENIGMA_BERRY {
                    holdEffect = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffect;
                } else {
                    holdEffect = GetItemHoldEffect(item);
                }
                if holdEffect != HOLD_EFFECT_EXP_SHARE
                    && (*crate::gl!(gBattleStruct)).sentInPokes as i32 & 1 == 0
                {
                    (*crate::gl!(gBattleStruct)).sentInPokes >>= 1;
                    crate::gl!(gBattleScripting.getexpState = 5);
                    gBattleMoveDamage.set(0);
                } else if GetMonData2(
                    &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
                    MON_DATA_LEVEL,
                ) == MAX_LEVEL
                {
                    (*crate::gl!(gBattleStruct)).sentInPokes >>= 1;
                    crate::gl!(gBattleScripting.getexpState = 5);
                    gBattleMoveDamage.set(0);
                } else {
                    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER == 0
                        && crate::gl!(gBattleMons[0].hp) != 0
                        && (*crate::gl!(gBattleStruct)).wildVictorySong == 0
                    {
                        BattleStopLowHpSound();
                        PlayBGM(MUS_VICTORY_WILD);
                        (*crate::gl!(gBattleStruct)).wildVictorySong += 1;
                    }
                    if GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_HP,
                    ) != 0
                    {
                        if (*crate::gl!(gBattleStruct)).sentInPokes as i32 & 1 != 0 {
                            gBattleMoveDamage.set(*exp as i32);
                        } else {
                            gBattleMoveDamage.set(0);
                        }
                        if holdEffect == HOLD_EFFECT_EXP_SHARE {
                            gBattleMoveDamage
                                .set(gBattleMoveDamage.get() + (gExpShareExp.get() as i32));
                        }
                        if holdEffect == HOLD_EFFECT_LUCKY_EGG {
                            gBattleMoveDamage.set(gBattleMoveDamage.get() * 150 / 100);
                        }
                        if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 {
                            gBattleMoveDamage.set(gBattleMoveDamage.get() * 150 / 100);
                        }
                        if IsTradedMon(
                            &raw mut (*gPlayerParty.as_ptr())
                                [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        ) != 0
                        {
                            if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
                                && (*crate::gl!(gBattleStruct)).expGetterMonId >= 3
                            {
                                i = STRINGID_EMPTYSTRING4;
                            } else {
                                gBattleMoveDamage.set(gBattleMoveDamage.get() * 150 / 100);
                                i = STRINGID_ABOOSTED;
                            }
                        } else {
                            i = STRINGID_EMPTYSTRING4;
                        }
                        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
                            if crate::gl!(gBattlerPartyIndexes[2])
                                == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                                && gAbsentBattlerFlags.get() as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[2]
                                    == 0
                            {
                                (*crate::gl!(gBattleStruct)).expGetterBattlerId = 2;
                            } else if gAbsentBattlerFlags.get() as u32
                                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[0]
                                == 0
                            {
                                (*crate::gl!(gBattleStruct)).expGetterBattlerId = 0;
                            } else {
                                (*crate::gl!(gBattleStruct)).expGetterBattlerId = 2;
                            }
                        } else {
                            (*crate::gl!(gBattleStruct)).expGetterBattlerId = 0;
                        }
                        crate::gl!(gBattleTextBuff1[0] = 0xFD);
                        crate::gl!(gBattleTextBuff1[1] = 4);
                        crate::gl!(
                            gBattleTextBuff1[2] = (*crate::gl!(gBattleStruct)).expGetterBattlerId
                        );
                        crate::gl!(
                            gBattleTextBuff1[3] = (*crate::gl!(gBattleStruct)).expGetterMonId
                        );
                        crate::gl!(gBattleTextBuff1[4] = 0xFF);
                        crate::gl!(gBattleTextBuff2[0] = 0xFD);
                        crate::gl!(gBattleTextBuff2[1] = 0);
                        crate::gl!(gBattleTextBuff2[2] = i as u8);
                        crate::gl!(gBattleTextBuff2[3] = ((i & 0xFF00) >> 8) as u8);
                        crate::gl!(gBattleTextBuff2[4] = 0xFF);
                        crate::gl!(gBattleTextBuff3[0] = 0xFD);
                        crate::gl!(gBattleTextBuff3[1] = 1);
                        crate::gl!(gBattleTextBuff3[2] = 4);
                        crate::gl!(gBattleTextBuff3[3] = 5);
                        crate::gl!(gBattleTextBuff3[4] = gBattleMoveDamage.get() as u8);
                        crate::gl!(
                            gBattleTextBuff3[5] =
                                ((gBattleMoveDamage.get() & 0x0000FF00) >> 8) as u8
                        );
                        crate::gl!(
                            gBattleTextBuff3[6] =
                                ((gBattleMoveDamage.get() & 0x00FF0000) >> 16) as u8
                        );
                        crate::gl!(
                            gBattleTextBuff3[7] =
                                ((gBattleMoveDamage.get() as u32 & 0xFF000000) >> 24) as u8
                        );
                        crate::gl!(gBattleTextBuff3[8] = 0xFF);
                        PrepareStringBattle(
                            STRINGID_PKMNGAINEDEXP,
                            (*crate::gl!(gBattleStruct)).expGetterBattlerId,
                        );
                        MonGainEVs(
                            &raw mut (*gPlayerParty.as_ptr())
                                [(*crate::gl!(gBattleStruct)).expGetterMonId],
                            {
                                let gl_i225 = gBattlerFainted.get();
                                crate::gl!(gBattleMons[gl_i225].species)
                            },
                        );
                    }
                    (*crate::gl!(gBattleStruct)).sentInPokes >>= 1;
                    crate::gl!(gBattleScripting.getexpState += 1);
                }
            }
            break 'l1;
        }
        if sw1 == 3 {
            if gBattleControllerExecFlags.get() == 0 {
                {
                    let gl_v = 0;
                    let gl_i58 = (*crate::gl!(gBattleStruct)).expGetterBattlerId;
                    crate::gl!(gBattleBufferB[gl_i58][0] = gl_v)
                };
                if GetMonData2(
                    &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
                    MON_DATA_HP,
                ) != 0
                    && GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_LEVEL,
                    ) != MAX_LEVEL
                {
                    (*(*crate::gl!(gBattleResources)).beforeLvlUp).stats[0] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_MAX_HP,
                    )
                        as u16;
                    (*(*crate::gl!(gBattleResources)).beforeLvlUp).stats[1] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_ATK,
                    )
                        as u16;
                    (*(*crate::gl!(gBattleResources)).beforeLvlUp).stats[2] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_DEF,
                    )
                        as u16;
                    (*(*crate::gl!(gBattleResources)).beforeLvlUp).stats[3] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_SPEED,
                    )
                        as u16;
                    (*(*crate::gl!(gBattleResources)).beforeLvlUp).stats[4] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_SPATK,
                    )
                        as u16;
                    (*(*crate::gl!(gBattleResources)).beforeLvlUp).stats[5] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_SPDEF,
                    )
                        as u16;
                    gActiveBattler.set((*crate::gl!(gBattleStruct)).expGetterBattlerId);
                    BtlController_EmitExpUpdate(
                        B_COMM_TO_CONTROLLER,
                        (*crate::gl!(gBattleStruct)).expGetterMonId,
                        gBattleMoveDamage.get() as u16,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
                crate::gl!(gBattleScripting.getexpState += 1);
            }
            break 'l1;
        }
        if sw1 == 4 {
            if gBattleControllerExecFlags.get() == 0 {
                gActiveBattler.set((*crate::gl!(gBattleStruct)).expGetterBattlerId);
                if {
                    let gl_i59 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i59][0])
                } == CONTROLLER_TWORETURNVALUES
                    && (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferB;).cast_const())
                        .cast::<CArray<CArray<u8, 512>, 4>>()
                        .cast_mut())[gActiveBattler.get()][1]
                        == RET_VALUE_LEVELED_UP
                {
                    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
                        && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;)
                            .cast_const())
                        .cast::<CArray<u16, 4>>()
                        .cast_mut())[gActiveBattler.get()]
                            == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                    {
                        HandleLowHpMusicChange(
                            &raw mut (*gPlayerParty.as_ptr())[{
                                let gl_i226 = gActiveBattler.get();
                                crate::gl!(gBattlerPartyIndexes[gl_i226])
                            }],
                            gActiveBattler.get(),
                        );
                    }
                    crate::gl!(gBattleTextBuff1[0] = 0xFD);
                    crate::gl!(gBattleTextBuff1[1] = 4);
                    crate::gl!(gBattleTextBuff1[2] = gActiveBattler.get());
                    crate::gl!(gBattleTextBuff1[3] = (*crate::gl!(gBattleStruct)).expGetterMonId);
                    crate::gl!(gBattleTextBuff1[4] = 0xFF);
                    crate::gl!(gBattleTextBuff2[0] = 0xFD);
                    crate::gl!(gBattleTextBuff2[1] = 1);
                    crate::gl!(gBattleTextBuff2[2] = 1);
                    crate::gl!(gBattleTextBuff2[3] = 3);
                    crate::gl!(
                        gBattleTextBuff2[4] = GetMonData2(
                            &raw mut (*gPlayerParty.as_ptr())
                                [(*crate::gl!(gBattleStruct)).expGetterMonId],
                            MON_DATA_LEVEL,
                        ) as u8
                    );
                    crate::gl!(gBattleTextBuff2[5] = 0xFF);
                    BattleScriptPushCursor();
                    gLeveledUpInBattle.set(
                        gLeveledUpInBattle.get()
                            | (gBitTable[(*crate::gl!(gBattleStruct)).expGetterMonId] as u8),
                    );
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LevelUp
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    gBattleMoveDamage.set(
                        {
                            let gl_i60 = gActiveBattler.get();
                            crate::gl!(gBattleBufferB[gl_i60][2])
                        } as i32
                            | ({
                                let gl_i61 = gActiveBattler.get();
                                crate::gl!(gBattleBufferB[gl_i61][3])
                            } as i32)
                                << 8,
                    );
                    AdjustFriendship(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        FRIENDSHIP_EVENT_GROW_LEVEL,
                    );
                    if crate::gl!(gBattlerPartyIndexes[0])
                        == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                        && crate::gl!(gBattleMons[0].hp) != 0
                    {
                        crate::gl!(
                            gBattleMons[0].level = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_LEVEL,
                            ) as u8
                        );
                        crate::gl!(
                            gBattleMons[0].hp = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_HP,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].maxHP = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_MAX_HP,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].attack = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_ATK,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].defense = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_DEF,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].speed = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPEED,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].speed = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPEED,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].spAttack = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPATK,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[0].spDefense = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPDEF,
                            ) as u16
                        );
                    }
                    if crate::gl!(gBattlerPartyIndexes[2])
                        == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                        && crate::gl!(gBattleMons[2].hp) != 0
                        && gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
                    {
                        crate::gl!(
                            gBattleMons[2].level = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_LEVEL,
                            ) as u8
                        );
                        crate::gl!(
                            gBattleMons[2].hp = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_HP,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[2].maxHP = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_MAX_HP,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[2].attack = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_ATK,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[2].defense = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_DEF,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[2].speed = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPEED,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[2].speed = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPEED,
                            ) as u16
                        );
                        crate::gl!(
                            gBattleMons[2].spAttack = GetMonData2(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [(*crate::gl!(gBattleStruct)).expGetterMonId],
                                MON_DATA_SPATK,
                            ) as u16
                        );
                    }
                    crate::gl!(gBattleScripting.getexpState = 5);
                } else {
                    gBattleMoveDamage.set(0);
                    crate::gl!(gBattleScripting.getexpState = 5);
                }
            }
            break 'l1;
        }
        if sw1 == 5 {
            if gBattleMoveDamage.get() != 0 {
                crate::gl!(gBattleScripting.getexpState = 3);
            } else {
                (*crate::gl!(gBattleStruct)).expGetterMonId += 1;
                if (*crate::gl!(gBattleStruct)).expGetterMonId < PARTY_SIZE as u8 {
                    crate::gl!(gBattleScripting.getexpState = 2);
                } else {
                    crate::gl!(gBattleScripting.getexpState = 6);
                }
            }
            break 'l1;
        }
        if sw1 == 6 {
            if gBattleControllerExecFlags.get() == 0 {
                {
                    let gl_i227 = gBattlerFainted.get();
                    crate::gl!(gBattleMons[gl_i227].item = ITEM_NONE)
                };
                {
                    let gl_i228 = gBattlerFainted.get();
                    crate::gl!(gBattleMons[gl_i228].ability = ABILITY_NONE)
                };
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Cmd_checkteamslost() {
    let mut HP_count: u16 = 0;
    let mut i: i32 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId.get() == TRAINER_STEVEN_PARTNER
    {
        for i in 0..MULTI_PARTY_SIZE {
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES) != 0
                && GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_IS_EGG) == 0
            {
                HP_count += GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) as u16;
            }
        }
    } else {
        for i in 0..PARTY_SIZE {
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES) != 0
                && GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_IS_EGG) == 0
                && (gBattleTypeFlags.get() & BATTLE_TYPE_ARENA == 0
                    || (*crate::gl!(gBattleStruct)).arenaLostPlayerMons as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        == 0)
            {
                HP_count += GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) as u16;
            }
        }
    }
    if HP_count == 0 {
        gBattleOutcome.set(gBattleOutcome.get() | (B_OUTCOME_LOST));
    }
    HP_count = 0;
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_SPECIES) != 0
            && GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_IS_EGG) == 0
            && (gBattleTypeFlags.get() & BATTLE_TYPE_ARENA == 0
                || (*crate::gl!(gBattleStruct)).arenaLostOpponentMons as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                    == 0)
        {
            HP_count += GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_HP) as u16;
        }
        i += 1;
    }
    if HP_count == 0 {
        gBattleOutcome.set(gBattleOutcome.get() | (B_OUTCOME_WON));
    }
    if gBattleOutcome.get() == 0 && gBattleTypeFlags.get() & 0x2000002 != 0 {
        let mut emptyPlayerSpots: i32 = 0;
        i = 0;
        while i < gBattlersCount.get() as i32 {
            if gHitMarker.get() & shl_i32(0x10000000, i as u32) as u32 != 0
                && (*gSpecialStatuses.as_ptr())[i].faintedHasReplacement() == 0
            {
                emptyPlayerSpots += 1;
            }
            i += 2;
        }
        let mut emptyOpponentSpots: i32 = 0;
        i = 1;
        while i < gBattlersCount.get() as i32 {
            if gHitMarker.get() & shl_i32(0x10000000, i as u32) as u32 != 0
                && (*gSpecialStatuses.as_ptr())[i].faintedHasReplacement() == 0
            {
                emptyOpponentSpots += 1;
            }
            i += 2;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
            if emptyOpponentSpots + emptyPlayerSpots > 1 {
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
                        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
                        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
                        as usize as *mut c_void
                        as *mut u8
                );
            } else {
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
            }
        } else {
            if emptyOpponentSpots != 0 && emptyPlayerSpots != 0 {
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
                        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
                        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
                        as usize as *mut c_void
                        as *mut u8
                );
            } else {
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
            }
        }
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
fn MoveValuesCleanUp() {
    gMoveResultFlags.set(0);
    crate::gl!(gBattleScripting.dmgMultiplier = 1);
    gCritMultiplier.set(1);
    crate::gl!(gBattleCommunication[3] = 0);
    crate::gl!(gBattleCommunication[6] = 0);
    gHitMarker.set(gHitMarker.get() & (0xffffffbf));
    gHitMarker.set(gHitMarker.get() & (0xffffbfff));
}
pub(crate) unsafe fn Cmd_movevaluescleanup() {
    MoveValuesCleanUp();
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setmultihit() {
    gMultiHitCounter.set(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_decrementmultihit() {
    if ({
        gMultiHitCounter.set(gMultiHitCounter.get() - (1));
        gMultiHitCounter.get()
    }) == 0
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
                + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
                as usize as *mut c_void as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_goto() {
    crate::gl!(
        gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
            as usize as *mut c_void as *mut u8
    );
}
pub(crate) unsafe fn Cmd_jumpifbyte() {
    let caseID: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(1);
    let memByte: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let value: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(6);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(7) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(7).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(7).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(7).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(11));
    match caseID {
        CMP_EQUAL => {
            if *memByte == value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_NOT_EQUAL => {
            if *memByte != value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_GREATER_THAN => {
            if *memByte > value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_LESS_THAN => {
            if *memByte < value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_COMMON_BITS => {
            if *memByte as i32 & value as i32 != 0 {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_NO_COMMON_BITS if *memByte as i32 & value as i32 == 0 => {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_jumpifhalfword() {
    let caseID: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(1);
    let memHword: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    let value: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(6) as u16
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(1) as u16) << 8);
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(8) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(8).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(8).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(8).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(12));
    match caseID {
        CMP_EQUAL => {
            if *memHword == value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_NOT_EQUAL => {
            if *memHword != value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_GREATER_THAN => {
            if *memHword > value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_LESS_THAN => {
            if *memHword < value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_COMMON_BITS => {
            if *memHword as i32 & value as i32 != 0 {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_NO_COMMON_BITS if *memHword as i32 & value as i32 == 0 => {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_jumpifword() {
    let caseID: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(1);
    let memWord: *mut u32 = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u32;
    let value: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(6) as u32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(1) as u32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(2) as u32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(3) as u32) << 24;
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(10) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(14));
    match caseID {
        CMP_EQUAL => {
            if *memWord == value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_NOT_EQUAL => {
            if *memWord != value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_GREATER_THAN => {
            if *memWord > value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_LESS_THAN => {
            if *memWord < value {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_COMMON_BITS => {
            if *memWord & value != 0 {
                crate::gl!(gBattlescriptCurrInstr = jumpPtr);
            }
        }
        CMP_NO_COMMON_BITS if *memWord & value == 0 => {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_jumpifarrayequal() {
    let mut mem1: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut mem2: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let size: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(9) as u32;
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(10) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut i: u8 = 0;
    while (i as u32) < size {
        if *mem1 != *mem2 {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(14));
            break;
        }
        mem1 = mem1.at(1);
        mem2 = mem2.at(1);
        i += 1;
    }
    if i as u32 == size {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    }
}
pub(crate) unsafe fn Cmd_jumpifarraynotequal() {
    let mut equalBytes: u8 = 0;
    let mut mem1: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut mem2: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let size: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(9) as u32;
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(10) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(10).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut i: u8 = 0;
    while (i as u32) < size {
        if *mem1 == *mem2 {
            equalBytes += 1;
        }
        mem1 = mem1.at(1);
        mem2 = mem2.at(1);
        i += 1;
    }
    if equalBytes as u32 != size {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(14));
    }
}
pub(crate) unsafe fn Cmd_setbyte() {
    let memByte: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte = *(*gBattlescriptCurrInstr.as_ptr()).at(5);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
}
pub(crate) unsafe fn Cmd_addbyte() {
    let memByte: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte += *(*gBattlescriptCurrInstr.as_ptr()).at(5);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
}
pub(crate) unsafe fn Cmd_subbyte() {
    let memByte: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte -= *(*gBattlescriptCurrInstr.as_ptr()).at(5);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
}
pub(crate) unsafe fn Cmd_copyarray() {
    let dest: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let src: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let size: i32 = *(*gBattlescriptCurrInstr.as_ptr()).at(9) as i32;
    for i in 0..size {
        *dest.at(i) = *src.at(i);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
}
pub(crate) unsafe fn Cmd_copyarraywithindex() {
    let dest: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let src: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let index: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(9) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(9).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(9).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(9).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let size: i32 = *(*gBattlescriptCurrInstr.as_ptr()).at(13) as i32;
    for i in 0..size {
        *dest.at(i) = *src.at(i + *index as i32);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(14));
}
pub(crate) unsafe fn Cmd_orbyte() {
    let memByte: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte |= *(*gBattlescriptCurrInstr.as_ptr()).at(5);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
}
pub(crate) unsafe fn Cmd_orhalfword() {
    let memHword: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    let val: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(5) as u16
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as u16) << 8);
    *memHword |= val;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
}
pub(crate) unsafe fn Cmd_orword() {
    let memWord: *mut u32 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u32;
    let val: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(5) as u32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as u32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as u32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as u32) << 24);
    *memWord |= val;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(9));
}
pub(crate) unsafe fn Cmd_bicbyte() {
    let memByte: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte &= !*(*gBattlescriptCurrInstr.as_ptr()).at(5);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
}
pub(crate) unsafe fn Cmd_bichalfword() {
    let memHword: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    let val: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(5) as u16
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as u16) << 8);
    *memHword &= !val;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
}
pub(crate) unsafe fn Cmd_bicword() {
    let memWord: *mut u32 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u32;
    let val: u32 = *(*gBattlescriptCurrInstr.as_ptr()).at(5) as u32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as u32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as u32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as u32) << 24);
    *memWord &= !val;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(9));
}
pub(crate) unsafe fn Cmd_pause() {
    if gBattleControllerExecFlags.get() == 0 {
        let value: u16 = *(*gBattlescriptCurrInstr.as_ptr()).at(1) as u16
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as u16) << 8);
        if ({
            gPauseCounterBattle.set(gPauseCounterBattle.get() + (1));
            gPauseCounterBattle.get()
        }) >= value
        {
            gPauseCounterBattle.set(0);
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
        }
    }
}
pub(crate) unsafe fn Cmd_waitstate() {
    if gBattleControllerExecFlags.get() == 0 {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_healthbar_update() {
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_TARGET {
        gActiveBattler.set(gBattlerTarget.get());
    } else {
        gActiveBattler.set(gBattlerAttacker.get());
    }
    BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, gBattleMoveDamage.get() as u16);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_return() {
    BattleScriptPop();
}
pub(crate) unsafe fn Cmd_end() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
        BattleArena_AddSkillPoints(gBattlerAttacker.get());
    }
    gMoveResultFlags.set(0);
    gActiveBattler.set(0);
    gCurrentActionFuncId.set(B_ACTION_TRY_FINISH);
}
pub(crate) fn Cmd_end2() {
    gActiveBattler.set(0);
    gCurrentActionFuncId.set(B_ACTION_TRY_FINISH);
}
pub(crate) unsafe fn Cmd_end3() {
    BattleScriptPop();
    if (*(*crate::gl!(gBattleResources)).battleCallbackStack).size != 0 {
        (*(*crate::gl!(gBattleResources)).battleCallbackStack).size -= 1;
    }
    crate::gl!(
        gBattleMainFunc = (*(*crate::gl!(gBattleResources)).battleCallbackStack).function
            [(*(*crate::gl!(gBattleResources)).battleCallbackStack).size]
    );
}
pub(crate) unsafe fn Cmd_call() {
    BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(5));
    crate::gl!(
        gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
            as usize as *mut u8
    );
}
pub(crate) unsafe fn Cmd_jumpiftype2() {
    let battler: u8 = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    if *(*gBattlescriptCurrInstr.as_ptr()).at(2) == crate::gl!(gBattleMons[battler].types[0])
        || *(*gBattlescriptCurrInstr.as_ptr()).at(2) == crate::gl!(gBattleMons[battler].types[1])
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_jumpifabilitypresent() {
    if AbilityBattleEffects(
        ABILITYEFFECT_CHECK_ON_FIELD,
        0,
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        0,
        0,
    ) != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_endselectionscript() {
    *(*crate::gl!(gBattleStruct))
        .selectionScriptFinished
        .as_mut_ptr()
        .at(gBattlerAttacker.get()) = TRUE;
}
pub(crate) unsafe fn Cmd_playanimation() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let argumentPtr: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    if *(*gBattlescriptCurrInstr.as_ptr()).at(2) == B_ANIM_STATS_CHANGE
        || *(*gBattlescriptCurrInstr.as_ptr()).at(2) == B_ANIM_SNATCH_MOVE
        || *(*gBattlescriptCurrInstr.as_ptr()).at(2) == 2
    {
        BtlController_EmitBattleAnimation(
            B_COMM_TO_CONTROLLER,
            *(*gBattlescriptCurrInstr.as_ptr()).at(2),
            *argumentPtr,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    } else if gHitMarker.get() & HITMARKER_NO_ANIMATIONS != 0 {
        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(7));
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_Pausex20
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else if *(*gBattlescriptCurrInstr.as_ptr()).at(2) == B_ANIM_RAIN_CONTINUES
        || *(*gBattlescriptCurrInstr.as_ptr()).at(2) == B_ANIM_SUN_CONTINUES
        || *(*gBattlescriptCurrInstr.as_ptr()).at(2) == B_ANIM_SANDSTORM_CONTINUES
        || *(*gBattlescriptCurrInstr.as_ptr()).at(2) == B_ANIM_HAIL_CONTINUES
    {
        BtlController_EmitBattleAnimation(
            B_COMM_TO_CONTROLLER,
            *(*gBattlescriptCurrInstr.as_ptr()).at(2),
            *argumentPtr,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    } else if {
        let gl_i62 = gActiveBattler.get();
        crate::gl!(gStatuses3[gl_i62])
    } & STATUS3_SEMI_INVULNERABLE
        != 0
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    } else {
        BtlController_EmitBattleAnimation(
            B_COMM_TO_CONTROLLER,
            *(*gBattlescriptCurrInstr.as_ptr()).at(2),
            *argumentPtr,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_playanimation_var() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let animationIdPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let argumentPtr: *mut u16 = (*(*gBattlescriptCurrInstr.as_ptr()).at(6) as i32
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(1) as i32) << 8)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(2) as i32) << 16)
        + ((*(*gBattlescriptCurrInstr.as_ptr()).at(6).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    if *animationIdPtr == B_ANIM_STATS_CHANGE
        || *animationIdPtr == B_ANIM_SNATCH_MOVE
        || *animationIdPtr == B_ANIM_SUBSTITUTE_FADE
    {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, *animationIdPtr, *argumentPtr);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    } else if gHitMarker.get() & HITMARKER_NO_ANIMATIONS != 0 {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    } else if *animationIdPtr == B_ANIM_RAIN_CONTINUES
        || *animationIdPtr == B_ANIM_SUN_CONTINUES
        || *animationIdPtr == B_ANIM_SANDSTORM_CONTINUES
        || *animationIdPtr == B_ANIM_HAIL_CONTINUES
    {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, *animationIdPtr, *argumentPtr);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    } else if {
        let gl_i63 = gActiveBattler.get();
        crate::gl!(gStatuses3[gl_i63])
    } & STATUS3_SEMI_INVULNERABLE
        != 0
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    } else {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, *animationIdPtr, *argumentPtr);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    }
}
pub(crate) unsafe fn Cmd_setgraphicalstatchangevalues() {
    let mut value: u8 = 0;
    match crate::gl!(gBattleScripting.statChanger) as i32 & 0xF0 {
        16 => {
            value = 15;
        }
        32 => {
            value = 39;
        }
        144 => {
            value = 22;
        }
        160 => {
            value = 46;
        }
        _ => {}
    }
    crate::gl!(
        gBattleScripting.animArg1 = (crate::gl!(gBattleScripting.statChanger) & 0xF) + value - 1
    );
    crate::gl!(gBattleScripting.animArg2 = 0);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_playstatchangeanimation() {
    let mut currStat: u32 = 0;
    let mut statAnimId: u16 = 0;
    let mut changeableStatsCount: i32 = 0;
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let mut statsToCheck: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(2);
    if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_NEGATIVE != 0 {
        let mut startingStatAnimId: i16 = 0;
        if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
            startingStatAnimId = STAT_ANIM_MINUS2;
        } else {
            startingStatAnimId = STAT_ANIM_MINUS1;
        }
        while statsToCheck != 0 {
            if statsToCheck as i32 & 1 != 0 {
                if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_CANT_PREVENT != 0
                {
                    if ({
                        let gl_i229 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i229].statStages[currStat])
                    }) > MIN_STAT_STAGE
                    {
                        statAnimId = startingStatAnimId as u16 + currStat as u16;
                        changeableStatsCount += 1;
                    }
                } else if {
                    let gl_i64 = GetBattlerPosition(gActiveBattler.get()) as i32 & 1;
                    crate::gl!(gSideTimers[gl_i64].mistTimer)
                } == 0
                    && ({
                        let gl_i230 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i230].ability)
                    }) != ABILITY_CLEAR_BODY
                    && ({
                        let gl_i231 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i231].ability)
                    }) != ABILITY_WHITE_SMOKE
                    && !(({
                        let gl_i232 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i232].ability)
                    }) == ABILITY_KEEN_EYE
                        && currStat == STAT_ACC)
                    && !(({
                        let gl_i233 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i233].ability)
                    }) == ABILITY_HYPER_CUTTER
                        && currStat == STAT_ATK as u32)
                    && ({
                        let gl_i234 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i234].statStages[currStat])
                    }) > MIN_STAT_STAGE
                {
                    statAnimId = startingStatAnimId as u16 + currStat as u16;
                    changeableStatsCount += 1;
                }
            }
            statsToCheck >>= 1;
            currStat += 1;
        }
        if changeableStatsCount > 1 {
            if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
                statAnimId = STAT_ANIM_MULTIPLE_MINUS2;
            } else {
                statAnimId = STAT_ANIM_MULTIPLE_MINUS1;
            }
        }
    } else {
        let mut startingStatAnimId: i16 = 0;
        if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
            startingStatAnimId = STAT_ANIM_PLUS2;
        } else {
            startingStatAnimId = STAT_ANIM_PLUS1;
        }
        while statsToCheck != 0 {
            if statsToCheck as i32 & 1 != 0
                && ({
                    let gl_i235 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i235].statStages[currStat])
                }) < MAX_STAT_STAGE
            {
                statAnimId = startingStatAnimId as u16 + currStat as u16;
                changeableStatsCount += 1;
            }
            statsToCheck >>= 1;
            currStat += 1;
        }
        if changeableStatsCount > 1 {
            if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
                statAnimId = STAT_ANIM_MULTIPLE_PLUS2;
            } else {
                statAnimId = STAT_ANIM_MULTIPLE_PLUS1;
            }
        }
    }
    if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_MULTIPLE_STATS != 0
        && changeableStatsCount < 2
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(4));
    } else if changeableStatsCount != 0 && crate::gl!(gBattleScripting.statAnimPlayed) == 0 {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, B_ANIM_STATS_CHANGE, statAnimId);
        MarkBattlerForControllerExec(gActiveBattler.get());
        if *(*gBattlescriptCurrInstr.as_ptr()).at(3) as i32 & STAT_CHANGE_MULTIPLE_STATS != 0
            && changeableStatsCount > 1
        {
            crate::gl!(gBattleScripting.statAnimPlayed = TRUE);
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(4));
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(4));
    }
}
pub(crate) unsafe fn Cmd_moveend() {
    let mut i: i32 = 0;
    let mut effect: u32 = FALSE as u32;
    let mut moveType: u8 = 0;
    let mut holdEffectAtk: u8 = 0;
    let mut originallyUsedMove: u16 = 0;
    if gChosenMove.get() == MOVE_UNAVAILABLE {
        originallyUsedMove = MOVE_NONE;
    } else {
        originallyUsedMove = gChosenMove.get();
    }
    let endMode: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(1);
    let endState: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(2);
    if ({
        let gl_i236 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i236].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffectAtk = {
            let gl_i65 = gBattlerAttacker.get();
            crate::gl!(gEnigmaBerries[gl_i65].holdEffect)
        };
    } else {
        holdEffectAtk = GetItemHoldEffect({
            let gl_i237 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i237].item)
        });
    }
    let choicedMoveAtk: *mut u16 =
        &raw mut (*crate::gl!(gBattleStruct)).choicedMove[gBattlerAttacker.get()];
    if (*crate::gl!(gBattleStruct)).dynamicMoveType != 0 {
        moveType = (*crate::gl!(gBattleStruct)).dynamicMoveType & 63;
    } else {
        moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .r#type;
    }
    loop {
        'l2: {
            match crate::gl!(gBattleScripting.moveendState) {
                MOVEEND_RAGE => {
                    if ({
                        let gl_i238 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i238].status2)
                    }) & STATUS2_RAGE
                        != 0
                        && ({
                            let gl_i239 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i239].hp)
                        }) != 0
                        && gBattlerAttacker.get() != gBattlerTarget.get()
                        && GetBattlerSide(gBattlerAttacker.get())
                            != GetBattlerSide(gBattlerTarget.get())
                        && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && ({
                            let gl_i66 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i66].physicalDmg)
                        } != 0
                            || {
                                let gl_i67 = gBattlerTarget.get();
                                crate::gl!(gSpecialStatuses[gl_i67].specialDmg)
                            } != 0)
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                        .power
                            != 0
                        && ({
                            let gl_i240 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i240].statStages[1])
                        }) < MAX_STAT_STAGE
                    {
                        {
                            let gl_i241 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i241].statStages[1] += 1)
                        };
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_RageIsBuilding
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut()
                        );
                        effect = TRUE as u32;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_DEFROST => {
                    if ({
                        let gl_i242 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i242].status1)
                    }) & STATUS1_FREEZE
                        != 0
                        && ({
                            let gl_i243 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i243].hp)
                        }) != 0
                        && gBattlerAttacker.get() != gBattlerTarget.get()
                        && {
                            let gl_i68 = gBattlerTarget.get();
                            crate::gl!(gSpecialStatuses[gl_i68].specialDmg)
                        } != 0
                        && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && moveType == TYPE_FIRE
                    {
                        {
                            let gl_i244 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i244].status1 &= 0xffffffdf)
                        };
                        gActiveBattler.set(gBattlerTarget.get());
                        BtlController_EmitSetMonData(
                            B_COMM_TO_CONTROLLER,
                            REQUEST_STATUS_BATTLE,
                            0,
                            4,
                            ({
                                let gl_i245 = gBattlerTarget.get();
                                crate::gl!(&raw gBattleMons[gl_i245].status1)
                            }) as *mut c_void,
                        );
                        MarkBattlerForControllerExec(gActiveBattler.get());
                        BattleScriptPushCursor();
                        crate::gl!(
                            gBattlescriptCurrInstr =
                                (*crate::asmdata::BattleScript_DefrostedViaFireMove
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut()
                        );
                        effect = TRUE as u32;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_SYNCHRONIZE_TARGET => {
                    if AbilityBattleEffects(
                        ABILITYEFFECT_SYNCHRONIZE,
                        gBattlerTarget.get(),
                        0,
                        0,
                        0,
                    ) != 0
                    {
                        effect = TRUE as u32;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_ON_DAMAGE_ABILITIES => {
                    if AbilityBattleEffects(ABILITYEFFECT_ON_DAMAGE, gBattlerTarget.get(), 0, 0, 0)
                        != 0
                    {
                        effect = TRUE as u32;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_IMMUNITY_ABILITIES => {
                    if AbilityBattleEffects(ABILITYEFFECT_IMMUNITY, 0, 0, 0, 0) != 0 {
                        effect = TRUE as u32;
                    } else {
                        crate::gl!(gBattleScripting.moveendState += 1);
                    }
                }
                MOVEEND_SYNCHRONIZE_ATTACKER => {
                    if AbilityBattleEffects(
                        ABILITYEFFECT_ATK_SYNCHRONIZE,
                        gBattlerAttacker.get(),
                        0,
                        0,
                        0,
                    ) != 0
                    {
                        effect = TRUE as u32;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_CHOICE_MOVE => {
                    if gHitMarker.get() & HITMARKER_OBEYS != 0
                        && holdEffectAtk == HOLD_EFFECT_CHOICE_BAND
                        && gChosenMove.get() != MOVE_STRUGGLE
                        && (*choicedMoveAtk == MOVE_NONE || *choicedMoveAtk == MOVE_UNAVAILABLE)
                    {
                        if gChosenMove.get() == MOVE_BATON_PASS
                            && gMoveResultFlags.get() as i32 & MOVE_RESULT_FAILED == 0
                        {
                            crate::gl!(gBattleScripting.moveendState += 1);
                            break 'l2;
                        }
                        *choicedMoveAtk = gChosenMove.get();
                    }
                    i = 0;
                    while i < MAX_MON_MOVES {
                        if ({
                            let gl_i246 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i246].moves[i])
                        }) == *choicedMoveAtk
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i == MAX_MON_MOVES {
                        *choicedMoveAtk = MOVE_NONE;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_CHANGED_ITEMS => {
                    for i in 0..(gBattlersCount.get() as i32) {
                        let changedItem: *mut u16 =
                            &raw mut (*crate::gl!(gBattleStruct)).changedItems[i];
                        if *changedItem != ITEM_NONE {
                            crate::gl!(gBattleMons[i].item = *changedItem);
                            *changedItem = ITEM_NONE;
                        }
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_ITEM_EFFECTS_ALL => {
                    if ItemBattleEffects(ITEMEFFECT_MOVE_END, 0, 0) != 0 {
                        effect = TRUE as u32;
                    } else {
                        crate::gl!(gBattleScripting.moveendState += 1);
                    }
                }
                MOVEEND_KINGSROCK_SHELLBELL => {
                    if ItemBattleEffects(ITEMEFFECT_KINGSROCK_SHELLBELL, 0, 0) != 0 {
                        effect = TRUE as u32;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_ATTACKER_INVISIBLE => {
                    if {
                        let gl_i69 = gBattlerAttacker.get();
                        crate::gl!(gStatuses3[gl_i69])
                    } & STATUS3_SEMI_INVULNERABLE
                        != 0
                        && gHitMarker.get() & HITMARKER_NO_ANIMATIONS != 0
                    {
                        gActiveBattler.set(gBattlerAttacker.get());
                        BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, TRUE);
                        MarkBattlerForControllerExec(gActiveBattler.get());
                        crate::gl!(gBattleScripting.moveendState += 1);
                        return;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_ATTACKER_VISIBLE => {
                    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0
                        || {
                            let gl_i70 = gBattlerAttacker.get();
                            crate::gl!(gStatuses3[gl_i70])
                        } & STATUS3_SEMI_INVULNERABLE
                            == 0
                        || WasUnableToUseMove(gBattlerAttacker.get()) != 0
                    {
                        gActiveBattler.set(gBattlerAttacker.get());
                        BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, FALSE);
                        MarkBattlerForControllerExec(gActiveBattler.get());
                        {
                            let gl_v = 0xfffbff3f;
                            let gl_i71 = gBattlerAttacker.get();
                            crate::gl!(gStatuses3[gl_i71] &= gl_v)
                        };
                        (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()]
                            .set_restoredBattlerSprite(1);
                        crate::gl!(gBattleScripting.moveendState += 1);
                        return;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_TARGET_VISIBLE => {
                    if (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].restoredBattlerSprite()
                        == 0
                        && gBattlerTarget.get() < gBattlersCount.get()
                        && {
                            let gl_i72 = gBattlerTarget.get();
                            crate::gl!(gStatuses3[gl_i72])
                        } & STATUS3_SEMI_INVULNERABLE
                            == 0
                    {
                        gActiveBattler.set(gBattlerTarget.get());
                        BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, FALSE);
                        MarkBattlerForControllerExec(gActiveBattler.get());
                        {
                            let gl_v = 0xfffbff3f;
                            let gl_i73 = gBattlerTarget.get();
                            crate::gl!(gStatuses3[gl_i73] &= gl_v)
                        };
                        crate::gl!(gBattleScripting.moveendState += 1);
                        return;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_SUBSTITUTE => {
                    for i in 0..(gBattlersCount.get() as i32) {
                        if crate::gl!(gDisableStructs[i].substituteHP) == 0 {
                            crate::gl!(gBattleMons[i].status2 &= 0xfeffffff);
                        }
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_UPDATE_LAST_MOVES => {
                    if gHitMarker.get() & HITMARKER_SWAP_ATTACKER_TARGET != 0 {
                        gActiveBattler.set(gBattlerAttacker.get());
                        gBattlerAttacker.set(gBattlerTarget.get());
                        gBattlerTarget.set(gActiveBattler.get());
                        gHitMarker.set(gHitMarker.get() & (0xffffefff));
                    }
                    if gHitMarker.get() & HITMARKER_ATTACKSTRING_PRINTED != 0 {
                        {
                            let gl_v = gChosenMove.get();
                            let gl_i74 = gBattlerAttacker.get();
                            crate::gl!(gLastPrintedMoves[gl_i74] = gl_v)
                        };
                    }
                    if gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gBattlerAttacker.get()]
                        == 0
                        && (*crate::gl!(gBattleStruct)).absentBattlerFlags as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerAttacker.get()]
                            == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[originallyUsedMove]
                            .effect
                            != EFFECT_BATON_PASS
                    {
                        if gHitMarker.get() & HITMARKER_OBEYS != 0 {
                            {
                                let gl_v = gChosenMove.get();
                                let gl_i75 = gBattlerAttacker.get();
                                crate::gl!(gLastMoves[gl_i75] = gl_v)
                            };
                            {
                                let gl_v = gCurrentMove.get();
                                let gl_i76 = gBattlerAttacker.get();
                                crate::gl!(gLastResultingMoves[gl_i76] = gl_v)
                            };
                        } else {
                            {
                                let gl_v = MOVE_UNAVAILABLE;
                                let gl_i77 = gBattlerAttacker.get();
                                crate::gl!(gLastMoves[gl_i77] = gl_v)
                            };
                            {
                                let gl_v = MOVE_UNAVAILABLE;
                                let gl_i78 = gBattlerAttacker.get();
                                crate::gl!(gLastResultingMoves[gl_i78] = gl_v)
                            };
                        }
                        if gHitMarker.get()
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerTarget.get()]
                                << 28
                            == 0
                        {
                            {
                                let gl_v = gBattlerAttacker.get();
                                let gl_i79 = gBattlerTarget.get();
                                crate::gl!(gLastHitBy[gl_i79] = gl_v)
                            };
                        }
                        if gHitMarker.get() & HITMARKER_OBEYS != 0
                            && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                        {
                            if gChosenMove.get() == MOVE_UNAVAILABLE {
                                {
                                    let gl_v = gChosenMove.get();
                                    let gl_i80 = gBattlerTarget.get();
                                    crate::gl!(gLastLandedMoves[gl_i80] = gl_v)
                                };
                            } else {
                                {
                                    let gl_v = gCurrentMove.get();
                                    let gl_i81 = gBattlerTarget.get();
                                    crate::gl!(gLastLandedMoves[gl_i81] = gl_v)
                                };
                                if (*crate::gl!(gBattleStruct)).dynamicMoveType != 0 {
                                    {
                                        let gl_v = (*crate::gl!(gBattleStruct)).dynamicMoveType
                                            as u16
                                            & 63;
                                        let gl_i82 = gBattlerTarget.get();
                                        crate::gl!(gLastHitByType[gl_i82] = gl_v)
                                    };
                                } else {
                                    {
                                        let gl_v =
                                            (*(&raw const crate::data::pokemon::gBattleMoves)
                                                .cast::<CArray<BattleMove, 0>>())
                                                [gCurrentMove.get()]
                                            .r#type
                                                as u16;
                                        let gl_i83 = gBattlerTarget.get();
                                        crate::gl!(gLastHitByType[gl_i83] = gl_v)
                                    };
                                }
                            }
                        } else {
                            {
                                let gl_v = MOVE_UNAVAILABLE;
                                let gl_i84 = gBattlerTarget.get();
                                crate::gl!(gLastLandedMoves[gl_i84] = gl_v)
                            };
                        }
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_MIRROR_MOVE => {
                    if gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gBattlerAttacker.get()]
                        == 0
                        && (*crate::gl!(gBattleStruct)).absentBattlerFlags as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerAttacker.get()]
                            == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[originallyUsedMove]
                            .flags as i32
                            & FLAG_MIRROR_MOVE_AFFECTED
                            != 0
                        && gHitMarker.get() & HITMARKER_OBEYS != 0
                        && gBattlerAttacker.get() != gBattlerTarget.get()
                        && gHitMarker.get()
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerTarget.get()]
                                << 28
                            == 0
                        && gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT == 0
                    {
                        *(*crate::gl!(gBattleStruct))
                            .lastTakenMove
                            .as_mut_ptr()
                            .at(gBattlerTarget.get() as i32 * 2) = gChosenMove.get() as u8;
                        *(*crate::gl!(gBattleStruct))
                            .lastTakenMove
                            .as_mut_ptr()
                            .at(gBattlerTarget.get() as i32 * 2)
                            .at(1) = (gChosenMove.get() >> 8) as u8;
                        let mut target: u8 = gBattlerTarget.get();
                        let mut attacker: u8 = gBattlerAttacker.get();
                        *(*crate::gl!(gBattleStruct))
                            .lastTakenMoveFrom
                            .as_mut_ptr()
                            .at(attacker as i32 * 2 + target as i32 * 8) = gChosenMove.get() as u8;
                        target = gBattlerTarget.get();
                        attacker = gBattlerAttacker.get();
                        *(*crate::gl!(gBattleStruct))
                            .lastTakenMoveFrom
                            .as_mut_ptr()
                            .at(attacker as i32 * 2 + target as i32 * 8)
                            .at(1) = (gChosenMove.get() >> 8) as u8;
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_NEXT_TARGET => {
                    if gHitMarker.get() & HITMARKER_UNABLE_TO_USE_MOVE == 0
                        && gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
                        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].chargingTurn() == 0
                        && (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                        .target
                            == MOVE_TARGET_BOTH
                        && gHitMarker.get() & HITMARKER_NO_ATTACKSTRING == 0
                    {
                        let battler: u8 =
                            GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget.get()) ^ 2);
                        if crate::gl!(gBattleMons[battler].hp) != 0 {
                            gBattlerTarget.set(battler);
                            gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_ATTACKSTRING));
                            crate::gl!(gBattleScripting.moveendState = 0);
                            MoveValuesCleanUp();
                            BattleScriptPush(
                                (*crate::asmdata::gBattleScriptsForMoveEffects
                                    .cast::<CArray<*mut u8, 0>>())
                                    [(*(&raw const crate::data::pokemon::gBattleMoves)
                                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                                    .effect],
                            );
                            crate::gl!(
                                gBattlescriptCurrInstr =
                                    (*crate::asmdata::BattleScript_FlushMessageBox
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut()
                            );
                            return;
                        } else {
                            gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_ATTACKSTRING));
                        }
                    }
                    crate::gl!(gBattleScripting.moveendState += 1);
                }
                MOVEEND_COUNT => {}
                _ => {}
            }
        }
        if endMode == 1 && effect == FALSE as u32 {
            crate::gl!(gBattleScripting.moveendState = MOVEEND_COUNT);
        }
        if endMode == 2 && endState == crate::gl!(gBattleScripting.moveendState) {
            crate::gl!(gBattleScripting.moveendState = MOVEEND_COUNT);
        }
        if !(crate::gl!(gBattleScripting.moveendState) != MOVEEND_COUNT && effect == FALSE as u32) {
            break;
        }
    }
    if crate::gl!(gBattleScripting.moveendState) == MOVEEND_COUNT && effect == FALSE as u32 {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
    }
}
pub(crate) unsafe fn Cmd_typecalc2() {
    let mut flags: u8 = 0;
    let mut i: i32 = 0;
    let moveType: u8 = (*(&raw const crate::data::pokemon::gBattleMoves)
        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
    .r#type;
    if ({
        let gl_i247 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i247].ability)
    }) == ABILITY_LEVITATE
        && moveType == TYPE_GROUND
    {
        gLastUsedAbility.set({
            let gl_i248 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i248].ability)
        });
        gMoveResultFlags.set(gMoveResultFlags.get() | (9));
        {
            let gl_v = 0;
            let gl_i85 = gBattlerTarget.get();
            crate::gl!(gLastLandedMoves[gl_i85] = gl_v)
        };
        crate::gl!(gBattleCommunication[6] = B_MSG_GROUND_MISS);
        RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
    } else {
        while (*(&raw const crate::data::battle_main::gTypeEffectiveness).cast::<CArray<u8, 336>>())
            [i]
            != TYPE_ENDTABLE
        {
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == TYPE_FORESIGHT
            {
                if ({
                    let gl_i249 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i249].status2)
                }) & STATUS2_FORESIGHT
                    != 0
                {
                    break;
                } else {
                    i += 3;
                    continue;
                }
            }
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i]
                == moveType
            {
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == ({
                        let gl_i250 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i250].types[0])
                    })
                {
                    if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 2]
                        == TYPE_MUL_NO_EFFECT
                    {
                        gMoveResultFlags
                            .set(gMoveResultFlags.get() | (MOVE_RESULT_DOESNT_AFFECT_FOE));
                        break;
                    }
                    if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 2]
                        == TYPE_MUL_NOT_EFFECTIVE
                    {
                        flags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                    }
                    if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 2]
                        == TYPE_MUL_SUPER_EFFECTIVE
                    {
                        flags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
                    }
                }
                if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    == ({
                        let gl_i251 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i251].types[1])
                    })
                {
                    if ({
                        let gl_i252 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i252].types[0])
                    }) != ({
                        let gl_i253 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i253].types[1])
                    }) && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 2]
                        == TYPE_MUL_NO_EFFECT
                    {
                        gMoveResultFlags
                            .set(gMoveResultFlags.get() | (MOVE_RESULT_DOESNT_AFFECT_FOE));
                        break;
                    }
                    if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 1]
                        == ({
                            let gl_i254 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i254].types[1])
                        })
                        && ({
                            let gl_i255 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i255].types[0])
                        }) != ({
                            let gl_i256 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i256].types[1])
                        })
                        && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2]
                            == TYPE_MUL_NOT_EFFECTIVE
                    {
                        flags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                    }
                    if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 1]
                        == ({
                            let gl_i257 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i257].types[1])
                        })
                        && ({
                            let gl_i258 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i258].types[0])
                        }) != ({
                            let gl_i259 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i259].types[1])
                        })
                        && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 2]
                            == TYPE_MUL_SUPER_EFFECTIVE
                    {
                        flags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
                    }
                }
            }
            i += 3;
        }
    }
    if ({
        let gl_i260 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i260].ability)
    }) == ABILITY_WONDER_GUARD
        && flags as i32 & MOVE_RESULT_NO_EFFECT == 0
        && AttacksThisTurn(gBattlerAttacker.get(), gCurrentMove.get()) == 2
        && (flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0 || flags as i32 & 6 == 6)
        && (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .power
            != 0
    {
        gLastUsedAbility.set(ABILITY_WONDER_GUARD);
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        {
            let gl_v = 0;
            let gl_i86 = gBattlerTarget.get();
            crate::gl!(gLastLandedMoves[gl_i86] = gl_v)
        };
        crate::gl!(gBattleCommunication[6] = B_MSG_AVOIDED_DMG);
        RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
    }
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_targetNotAffected(1);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_returnatktoball() {
    gActiveBattler.set(gBattlerAttacker.get());
    if gHitMarker.get()
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
            << 28
        == 0
    {
        BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, FALSE);
        MarkBattlerForControllerExec(gActiveBattler.get());
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_getswitchedmondata() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    {
        let gl_v = *(*crate::gl!(gBattleStruct))
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) as u16;
        let gl_i261 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i261] = gl_v)
    };
    BtlController_EmitGetMonData(
        B_COMM_TO_CONTROLLER,
        REQUEST_ALL_BATTLE,
        gBitTable[{
            let gl_i262 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i262])
        }] as u8,
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_switchindataupdate() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let oldData: BattlePokemon = {
        let gl_i263 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i263])
    };
    let monData: *mut u8 = ({
        let gl_i264 = gActiveBattler.get();
        crate::gl!(&raw gBattleMons[gl_i264])
    }) as *mut u8;
    for i in 0..88i32 {
        *monData.at(i) = {
            let gl_i87 = gActiveBattler.get();
            let gl_i88 = 4 + i;
            crate::gl!(gBattleBufferB[gl_i87][gl_i88])
        };
    }
    {
        let gl_v = (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[{
            let gl_i266 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i266].species)
        }]
        .types[0];
        let gl_i265 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i265].types[0] = gl_v)
    };
    {
        let gl_v = (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[{
            let gl_i268 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i268].species)
        }]
        .types[1];
        let gl_i267 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i267].types[1] = gl_v)
    };
    {
        let gl_v = GetAbilityBySpecies(
            {
                let gl_i270 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i270].species)
            },
            (*gBattleMons.as_ptr())[gActiveBattler.get()].abilityNum() as u8,
        );
        let gl_i269 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i269].ability = gl_v)
    };
    let i: i32 = GetBattlerSide(gActiveBattler.get()) as i32;
    if crate::gl!(gWishFutureKnock.knockedOffMons[i]) as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[{
            let gl_i271 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i271])
        }]
        != 0
    {
        {
            let gl_i272 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i272].item = ITEM_NONE)
        };
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove.get()]
    .effect
        == EFFECT_BATON_PASS
    {
        for i in 0..NUM_BATTLE_STATS {
            {
                let gl_i273 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i273].statStages[i] = oldData.statStages[i])
            };
        }
        {
            let gl_i274 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i274].status2 = oldData.status2)
        };
    }
    SwitchInClearSetData();
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0
        && ({
            let gl_i275 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i275].maxHP)
        }) as i32
            / 2
            >= ({
                let gl_i276 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i276].hp)
            }) as i32
        && ({
            let gl_i277 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i277].hp)
        }) != 0
        && ({
            let gl_i278 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i278].status1)
        }) & STATUS1_SLEEP
            == 0
    {
        (*crate::gl!(gBattleStruct)).palaceFlags |= gBitTable[gActiveBattler.get()] as u8;
    }
    crate::gl!(gBattleScripting.battler = gActiveBattler.get());
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 7);
    crate::gl!(gBattleTextBuff1[2] = gActiveBattler.get());
    crate::gl!(
        gBattleTextBuff1[3] = ({
            let gl_i279 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i279])
        }) as u8
    );
    crate::gl!(gBattleTextBuff1[4] = 0xFF);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_switchinanim() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT
        && gBattleTypeFlags.get() & 0x63f0902 == 0
    {
        HandleSetPokedexFlag(
            SpeciesToNationalPokedexNum({
                let gl_i280 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i280].species)
            }),
            FLAG_SET_SEEN,
            {
                let gl_i281 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i281].personality)
            },
        );
    }
    gAbsentBattlerFlags.set(gAbsentBattlerFlags.get() & (!(gBitTable[gActiveBattler.get()] as u8)));
    BtlController_EmitSwitchInAnim(
        B_COMM_TO_CONTROLLER,
        ({
            let gl_i282 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i282])
        }) as u8,
        *(*gBattlescriptCurrInstr.as_ptr()).at(2),
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
        BattleArena_InitPoints();
    }
}
pub(crate) unsafe fn Cmd_jumpifcantswitch() {
    let mut i: i32 = 0;
    let mut lastMonId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1) & 127,
    ));
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32 & SWITCH_IGNORE_ESCAPE_PREVENTION == 0
        && (({
            let gl_i283 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i283].status2)
        }) & 0x400e000
            != 0
            || {
                let gl_i89 = gActiveBattler.get();
                crate::gl!(gStatuses3[gl_i89])
            } & STATUS3_ROOTED
                != 0)
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
        if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT {
            party = (*gEnemyParty.as_ptr()).as_mut_ptr();
        } else {
            party = (*gPlayerParty.as_ptr()).as_mut_ptr();
        }
        lastMonId = 0;
        if gActiveBattler.get() as i32 & 2 != 0 {
            lastMonId = MULTI_PARTY_SIZE;
        }
        i = lastMonId;
        while i < lastMonId + MULTI_PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;).cast_const())
                    .cast::<CArray<u16, 4>>()
                    .cast_mut())[gActiveBattler.get()] as i32
                    != i
            {
                break;
            }
            i += 1;
        }
        if i == lastMonId + MULTI_PARTY_SIZE {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
                party = (*gPlayerParty.as_ptr()).as_mut_ptr();
                lastMonId = 0;
                if GetLinkTrainerFlankId(GetBattlerMultiplayerId(gActiveBattler.get() as u16) as u8)
                    == TRUE as u16
                {
                    lastMonId = MULTI_PARTY_SIZE;
                }
            } else {
                party = (*gEnemyParty.as_ptr()).as_mut_ptr();
                if gActiveBattler.get() == 1 {
                    lastMonId = 0;
                } else {
                    lastMonId = MULTI_PARTY_SIZE;
                }
            }
        } else {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT {
                party = (*gEnemyParty.as_ptr()).as_mut_ptr();
            } else {
                party = (*gPlayerParty.as_ptr()).as_mut_ptr();
            }
            lastMonId = 0;
            if GetLinkTrainerFlankId(GetBattlerMultiplayerId(gActiveBattler.get() as u16) as u8)
                == TRUE as u16
            {
                lastMonId = MULTI_PARTY_SIZE;
            }
        }
        i = lastMonId;
        while i < lastMonId + MULTI_PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;).cast_const())
                    .cast::<CArray<u16, 4>>()
                    .cast_mut())[gActiveBattler.get()] as i32
                    != i
            {
                break;
            }
            i += 1;
        }
        if i == lastMonId + MULTI_PARTY_SIZE {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0
        && GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT
    {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
        lastMonId = 0;
        if gActiveBattler.get() == B_POSITION_OPPONENT_RIGHT {
            lastMonId = 3;
        }
        i = lastMonId;
        while i < lastMonId + 3 {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;).cast_const())
                    .cast::<CArray<u16, 4>>()
                    .cast_mut())[gActiveBattler.get()] as i32
                    != i
            {
                break;
            }
            i += 1;
        }
        if i == lastMonId + 3 {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
        }
    } else {
        let mut battlerIn1: u8 = 0;
        let mut battlerIn2: u8 = 0;
        if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT {
            battlerIn1 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
                battlerIn2 = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
            } else {
                battlerIn2 = battlerIn1;
            }
            party = (*gEnemyParty.as_ptr()).as_mut_ptr();
        } else {
            battlerIn1 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
                battlerIn2 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            } else {
                battlerIn2 = battlerIn1;
            }
            party = (*gPlayerParty.as_ptr()).as_mut_ptr();
        }
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && i != crate::gl!(gBattlerPartyIndexes[battlerIn1]) as i32
                && i != crate::gl!(gBattlerPartyIndexes[battlerIn2]) as i32
            {
                break;
            }
            i += 1;
        }
        if i == PARTY_SIZE {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
        }
    }
}
unsafe fn ChooseMonToSendOut(slotId: u8) {
    *(*crate::gl!(gBattleStruct))
        .battlerPartyIndexes
        .as_mut_ptr()
        .at(gActiveBattler.get()) = ({
        let gl_i284 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i284])
    }) as u8;
    *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get()) = PARTY_SIZE as u8;
    (*crate::gl!(gBattleStruct)).field_93 &= !(gBitTable[gActiveBattler.get()] as u8);
    BtlController_EmitChoosePokemon(
        B_COMM_TO_CONTROLLER,
        PARTY_ACTION_SEND_OUT,
        slotId,
        ABILITY_NONE,
        (*crate::gl!(gBattleStruct)).battlerPartyOrders[gActiveBattler.get()].as_mut_ptr(),
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
}
pub(crate) unsafe fn Cmd_openpartyscreen() {
    let mut hitmarkerFaintBits: u8 = 0;
    let mut battler: u8 = 0;
    let mut flags: u32 = 0;
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
        as usize as *mut u8;
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_FAINTED_LINK_MULTIPLE_1 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            || gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0
        {
            gActiveBattler.set(0);
            while gActiveBattler.get() < gBattlersCount.get() {
                if gHitMarker.get()
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gActiveBattler.get()]
                        << 28
                    != 0
                {
                    if HasNoMonsToSwitch(gActiveBattler.get(), PARTY_SIZE as u8, PARTY_SIZE as u8)
                        != 0
                    {
                        gAbsentBattlerFlags.set(
                            gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8),
                        );
                        gHitMarker
                            .set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                        BtlController_EmitLinkStandbyMsg(
                            B_COMM_TO_CONTROLLER,
                            LINK_STANDBY_MSG_ONLY,
                            FALSE as u32,
                        );
                        MarkBattlerForControllerExec(gActiveBattler.get());
                    } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .faintedHasReplacement()
                        == 0
                    {
                        ChooseMonToSendOut(PARTY_SIZE as u8);
                        (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                            .set_faintedHasReplacement(TRUE as u32);
                    }
                } else {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
                gActiveBattler.set(gActiveBattler.get() + (1));
            }
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
            let mut hasReplacement_2: u8 = 0;
            let mut hasReplacement_3: u8 = 0;
            hitmarkerFaintBits = (gHitMarker.get() >> 28) as u8;
            if gBitTable[0] & hitmarkerFaintBits as u32 != 0 {
                gActiveBattler.set(0);
                if HasNoMonsToSwitch(gActiveBattler.get(), PARTY_SIZE as u8, PARTY_SIZE as u8) != 0
                {
                    gAbsentBattlerFlags
                        .set(gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8));
                    gHitMarker.set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].faintedHasReplacement()
                    == 0
                {
                    ChooseMonToSendOut((*crate::gl!(gBattleStruct)).monToSwitchIntoId[2]);
                    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .set_faintedHasReplacement(TRUE as u32);
                } else {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                    flags |= 1;
                }
            }
            if gBitTable[2] & hitmarkerFaintBits as u32 != 0
                && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[0]
                    & hitmarkerFaintBits as u32
                    == 0
            {
                gActiveBattler.set(2);
                if HasNoMonsToSwitch(gActiveBattler.get(), PARTY_SIZE as u8, PARTY_SIZE as u8) != 0
                {
                    gAbsentBattlerFlags
                        .set(gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8));
                    gHitMarker.set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].faintedHasReplacement()
                    == 0
                {
                    ChooseMonToSendOut((*crate::gl!(gBattleStruct)).monToSwitchIntoId[0]);
                    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .set_faintedHasReplacement(TRUE as u32);
                } else if flags & 1 == 0 {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
            }
            if gBitTable[1] & hitmarkerFaintBits as u32 != 0 {
                gActiveBattler.set(1);
                if HasNoMonsToSwitch(gActiveBattler.get(), PARTY_SIZE as u8, PARTY_SIZE as u8) != 0
                {
                    gAbsentBattlerFlags
                        .set(gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8));
                    gHitMarker.set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].faintedHasReplacement()
                    == 0
                {
                    ChooseMonToSendOut((*crate::gl!(gBattleStruct)).monToSwitchIntoId[3]);
                    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .set_faintedHasReplacement(TRUE as u32);
                } else {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                    flags |= 2;
                }
            }
            if gBitTable[3] & hitmarkerFaintBits as u32 != 0
                && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[1]
                    & hitmarkerFaintBits as u32
                    == 0
            {
                gActiveBattler.set(3);
                if HasNoMonsToSwitch(gActiveBattler.get(), PARTY_SIZE as u8, PARTY_SIZE as u8) != 0
                {
                    gAbsentBattlerFlags
                        .set(gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8));
                    gHitMarker.set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].faintedHasReplacement()
                    == 0
                {
                    ChooseMonToSendOut((*crate::gl!(gBattleStruct)).monToSwitchIntoId[1]);
                    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .set_faintedHasReplacement(TRUE as u32);
                } else if flags & 2 == 0 {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
            }
            let hasReplacement_0: u8 =
                (*gSpecialStatuses.as_ptr())[0].faintedHasReplacement() as u8;
            if hasReplacement_0 == 0 {
                hasReplacement_2 = (*gSpecialStatuses.as_ptr())[2].faintedHasReplacement() as u8;
                if hasReplacement_2 == 0 && hitmarkerFaintBits != 0 {
                    if gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[0]
                        != 0
                    {
                        gActiveBattler.set(2);
                    } else {
                        gActiveBattler.set(0);
                    }
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
            }
            let hasReplacement_1: u8 =
                (*gSpecialStatuses.as_ptr())[1].faintedHasReplacement() as u8;
            if hasReplacement_1 == 0 {
                hasReplacement_3 = (*gSpecialStatuses.as_ptr())[3].faintedHasReplacement() as u8;
                if hasReplacement_3 == 0 && hitmarkerFaintBits != 0 {
                    if gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[1]
                        != 0
                    {
                        gActiveBattler.set(3);
                    } else {
                        gActiveBattler.set(1);
                    }
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
            }
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    } else if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_FAINTED_LINK_MULTIPLE_2 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
                hitmarkerFaintBits = (gHitMarker.get() >> 28) as u8;
                if gBitTable[2] & hitmarkerFaintBits as u32 != 0
                    && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[0]
                        & hitmarkerFaintBits as u32
                        != 0
                {
                    gActiveBattler.set(2);
                    if HasNoMonsToSwitch(
                        gActiveBattler.get(),
                        crate::gl!(gBattleBufferB[0][1]),
                        PARTY_SIZE as u8,
                    ) != 0
                    {
                        gAbsentBattlerFlags.set(
                            gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8),
                        );
                        gHitMarker
                            .set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                        BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                        MarkBattlerForControllerExec(gActiveBattler.get());
                    } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .faintedHasReplacement()
                        == 0
                    {
                        ChooseMonToSendOut((*crate::gl!(gBattleStruct)).monToSwitchIntoId[0]);
                        (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                            .set_faintedHasReplacement(TRUE as u32);
                    }
                }
                if gBitTable[3] & hitmarkerFaintBits as u32 != 0
                    && hitmarkerFaintBits as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[1]
                        != 0
                {
                    gActiveBattler.set(3);
                    if HasNoMonsToSwitch(
                        gActiveBattler.get(),
                        crate::gl!(gBattleBufferB[1][1]),
                        PARTY_SIZE as u8,
                    ) != 0
                    {
                        gAbsentBattlerFlags.set(
                            gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8),
                        );
                        gHitMarker
                            .set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
                        BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                        MarkBattlerForControllerExec(gActiveBattler.get());
                    } else if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                        .faintedHasReplacement()
                        == 0
                    {
                        ChooseMonToSendOut((*crate::gl!(gBattleStruct)).monToSwitchIntoId[1]);
                        (*gSpecialStatuses.as_ptr())[gActiveBattler.get()]
                            .set_faintedHasReplacement(TRUE as u32);
                    }
                }
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
            } else {
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
            }
        } else {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
        }
        hitmarkerFaintBits = (gHitMarker.get() >> 28) as u8;
        gBattlerFainted.set(0);
        while gBitTable[gBattlerFainted.get()] & hitmarkerFaintBits as u32 == 0
            && gBattlerFainted.get() < gBattlersCount.get()
        {
            gBattlerFainted.set(gBattlerFainted.get() + (1));
        }
        if gBattlerFainted.get() == gBattlersCount.get() {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        }
    } else {
        if *(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32 & PARTY_SCREEN_OPTIONAL != 0 {
            hitmarkerFaintBits = PARTY_ACTION_CHOOSE_MON;
        } else {
            hitmarkerFaintBits = PARTY_ACTION_SEND_OUT;
        }
        battler = GetBattlerForBattleScript(*(*gBattlescriptCurrInstr.as_ptr()).at(1) & 127);
        if (*gSpecialStatuses.as_ptr())[battler].faintedHasReplacement() != 0 {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
        } else if HasNoMonsToSwitch(battler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
            gActiveBattler.set(battler);
            gAbsentBattlerFlags
                .set(gAbsentBattlerFlags.get() | (gBitTable[gActiveBattler.get()] as u8));
            gHitMarker.set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        } else {
            gActiveBattler.set(battler);
            *(*crate::gl!(gBattleStruct))
                .battlerPartyIndexes
                .as_mut_ptr()
                .at(gActiveBattler.get()) = ({
                let gl_i285 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i285])
            }) as u8;
            *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler.get()) = PARTY_SIZE as u8;
            (*crate::gl!(gBattleStruct)).field_93 &= !(gBitTable[gActiveBattler.get()] as u8);
            BtlController_EmitChoosePokemon(
                B_COMM_TO_CONTROLLER,
                hitmarkerFaintBits,
                *(*crate::gl!(gBattleStruct))
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler.get() as i32 ^ 2),
                ABILITY_NONE,
                (*crate::gl!(gBattleStruct)).battlerPartyOrders[gActiveBattler.get()].as_mut_ptr(),
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
            if GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_LEFT
                && crate::gl!(gBattleResults.playerSwitchesCounter) < 255
            {
                crate::gl!(gBattleResults.playerSwitchesCounter += 1);
            }
            if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
                gActiveBattler.set(0);
                while gActiveBattler.get() < gBattlersCount.get() {
                    if gActiveBattler.get() != battler {
                        BtlController_EmitLinkStandbyMsg(
                            B_COMM_TO_CONTROLLER,
                            LINK_STANDBY_MSG_ONLY,
                            FALSE as u32,
                        );
                        MarkBattlerForControllerExec(gActiveBattler.get());
                    }
                    gActiveBattler.set(gActiveBattler.get() + (1));
                }
            } else {
                gActiveBattler.set(GetBattlerAtPosition(GetBattlerPosition(battler) ^ 1));
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gActiveBattler.get()]
                    != 0
                {
                    gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
                }
                BtlController_EmitLinkStandbyMsg(
                    B_COMM_TO_CONTROLLER,
                    LINK_STANDBY_MSG_ONLY,
                    FALSE as u32,
                );
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
        }
    }
}
pub(crate) unsafe fn Cmd_switchhandleorder() {
    let mut i: i32 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    'l1: {
        let sw1: u8 = *(*gBattlescriptCurrInstr.as_ptr()).at(2);
        let mut fall = false;
        if sw1 == 0 {
            i = 0;
            while i < gBattlersCount.get() as i32 {
                if crate::gl!(gBattleBufferB[i][0]) == CONTROLLER_CHOSENMONRETURNVALUE {
                    *(*crate::gl!(gBattleStruct))
                        .monToSwitchIntoId
                        .as_mut_ptr()
                        .at(i) = crate::gl!(gBattleBufferB[i][1]);
                    if (*crate::gl!(gBattleStruct)).field_93 as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        == 0
                    {
                        RecordedBattle_SetBattlerAction(i as u8, crate::gl!(gBattleBufferB[i][1]));
                        (*crate::gl!(gBattleStruct)).field_93 |= gBitTable[i] as u8;
                    }
                }
                i += 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
                SwitchPartyOrder(gActiveBattler.get());
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*crate::gl!(gBattleStruct)).field_93 as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gActiveBattler.get()]
                == 0
            {
                RecordedBattle_SetBattlerAction(gActiveBattler.get(), {
                    let gl_i90 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i90][1])
                });
                (*crate::gl!(gBattleStruct)).field_93 |= gBitTable[gActiveBattler.get()] as u8;
            }
        }
        if fall || sw1 == 3 {
            crate::gl!(
                gBattleCommunication[0] = {
                    let gl_i91 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i91][1])
                }
            );
            *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler.get()) = {
                let gl_i92 = gActiveBattler.get();
                crate::gl!(gBattleBufferB[gl_i92][1])
            };
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            {
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(gActiveBattler.get() as i32 * 3) &= 0xF;
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(gActiveBattler.get() as i32 * 3) |= {
                    let gl_i93 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i93][2])
                } & 0xF0;
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(gActiveBattler.get() as i32 * 3)
                    .at(1) = {
                    let gl_i94 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i94][3])
                };
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at((gActiveBattler.get() as i32 ^ 2) * 3) &= 0xF0;
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at((gActiveBattler.get() as i32 ^ 2) * 3) |= (({
                    let gl_i95 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i95][2])
                } as i32
                    & 0xF0)
                    >> 4) as u8;
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at((gActiveBattler.get() as i32 ^ 2) * 3)
                    .at(2) = {
                    let gl_i96 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i96][3])
                };
            } else if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
                SwitchPartyOrderInGameMulti(
                    gActiveBattler.get(),
                    *(*crate::gl!(gBattleStruct))
                        .monToSwitchIntoId
                        .as_mut_ptr()
                        .at(gActiveBattler.get()),
                );
            } else {
                SwitchPartyOrder(gActiveBattler.get());
            }
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 6);
            crate::gl!(
                gBattleTextBuff1[2] = ({
                    let gl_i286 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i286].species)
                }) as u8
            );
            crate::gl!(
                gBattleTextBuff1[3] = ((({
                    let gl_i287 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i287].species)
                }) as i32
                    & 0xFF00)
                    >> 8) as u8
            );
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            crate::gl!(gBattleTextBuff2[0] = 0xFD);
            crate::gl!(gBattleTextBuff2[1] = 7);
            crate::gl!(gBattleTextBuff2[2] = gActiveBattler.get());
            crate::gl!(
                gBattleTextBuff2[3] = {
                    let gl_i97 = gActiveBattler.get();
                    crate::gl!(gBattleBufferB[gl_i97][1])
                }
            );
            crate::gl!(gBattleTextBuff2[4] = 0xFF);
            break 'l1;
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
}
pub(crate) unsafe fn Cmd_switchineffects() {
    let mut i: i32 = 0;
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    UpdateSentPokesToOpponentValue(gActiveBattler.get());
    gHitMarker.set(gHitMarker.get() & (!(gBitTable[gActiveBattler.get()] << 28)));
    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].set_faintedHasReplacement(FALSE as u32);
    if {
        let gl_i98 = GetBattlerSide(gActiveBattler.get());
        crate::gl!(gSideStatuses[gl_i98])
    } as i32
        & SIDE_STATUS_SPIKES_DAMAGED
        == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gSideStatuses;).cast_const())
            .cast::<CArray<u16, 2>>()
            .cast_mut())[GetBattlerSide(gActiveBattler.get())] as i32
            & SIDE_STATUS_SPIKES
            != 0
        && !(({
            let gl_i288 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i288].types[0])
        }) == TYPE_FLYING
            || ({
                let gl_i289 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i289].types[1])
            }) == TYPE_FLYING)
        && ({
            let gl_i290 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i290].ability)
        }) != ABILITY_LEVITATE
    {
        {
            let gl_v = SIDE_STATUS_SPIKES_DAMAGED as u16;
            let gl_i99 = GetBattlerSide(gActiveBattler.get());
            crate::gl!(gSideStatuses[gl_i99] |= gl_v)
        };
        {
            let gl_i291 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i291].status2 &= 0xfdffffff)
        };
        gHitMarker.set(gHitMarker.get() & (0xffffffbf));
        let spikesDmg: u8 = (5 - {
            let gl_i100 = GetBattlerSide(gActiveBattler.get());
            crate::gl!(gSideTimers[gl_i100].spikesAmount)
        }) * 2;
        gBattleMoveDamage.set(div_i32(
            ({
                let gl_i292 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i292].maxHP)
            }) as i32,
            spikesDmg as i32,
        ));
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        crate::gl!(gBattleScripting.battler = gActiveBattler.get());
        BattleScriptPushCursor();
        if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_TARGET {
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SpikesOnTarget
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
        } else if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == 1 {
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SpikesOnAttacker
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
        } else {
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SpikesOnFaintedBattler
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
        }
    } else {
        if ({
            let gl_i293 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i293].ability)
        }) == ABILITY_TRUANT
            && (*gDisableStructs.as_ptr())[gActiveBattler.get()].truantSwitchInHack() == 0
        {
            (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_truantCounter(1);
        }
        (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_truantSwitchInHack(0);
        if AbilityBattleEffects(0, gActiveBattler.get(), 0, 0, 0) == 0
            && ItemBattleEffects(0, gActiveBattler.get(), 0) == 0
        {
            {
                let gl_v = 65023;
                let gl_i101 = GetBattlerSide(gActiveBattler.get());
                crate::gl!(gSideStatuses[gl_i101] &= gl_v)
            };
            for i in 0..(gBattlersCount.get() as i32) {
                if crate::gl!(gBattlerByTurnOrder[i]) == gActiveBattler.get() {
                    crate::gl!(gActionsByTurnOrder[i] = B_ACTION_CANCEL_PARTNER);
                }
            }
            i = 0;
            while i < gBattlersCount.get() as i32 {
                let hpOnSwitchout: *mut u16 =
                    &raw mut (*crate::gl!(gBattleStruct)).hpOnSwitchout[GetBattlerSide(i as u8)];
                *hpOnSwitchout = crate::gl!(gBattleMons[i].hp);
                i += 1;
            }
            if *(*gBattlescriptCurrInstr.as_ptr()).at(1) == BS_FAINTED_LINK_MULTIPLE_1 {
                let hitmarkerFaintBits: u32 = gHitMarker.get() >> 28;
                gBattlerFainted.set(gBattlerFainted.get() + (1));
                loop {
                    if hitmarkerFaintBits
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gBattlerFainted.get()]
                        != 0
                        && gAbsentBattlerFlags.get() as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerFainted.get()]
                            == 0
                    {
                        break;
                    }
                    if gBattlerFainted.get() >= gBattlersCount.get() {
                        break;
                    }
                    gBattlerFainted.set(gBattlerFainted.get() + (1));
                }
            }
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
        }
    }
}
pub(crate) unsafe fn Cmd_trainerslidein() {
    gActiveBattler.set(GetBattlerAtPosition(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    BtlController_EmitTrainerSlide(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_playse() {
    gActiveBattler.set(gBattlerAttacker.get());
    BtlController_EmitPlaySE(
        B_COMM_TO_CONTROLLER,
        *(*gBattlescriptCurrInstr.as_ptr()).at(1) as u16
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as u16) << 8),
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
}
pub(crate) unsafe fn Cmd_fanfare() {
    gActiveBattler.set(gBattlerAttacker.get());
    BtlController_EmitPlayFanfareOrBGM(
        B_COMM_TO_CONTROLLER,
        *(*gBattlescriptCurrInstr.as_ptr()).at(1) as u16
            + ((*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as u16) << 8),
        FALSE,
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
}
pub(crate) unsafe fn Cmd_playfaintcry() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    BtlController_EmitFaintingCry(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_endlinkbattle() {
    gActiveBattler.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
    BtlController_EmitEndLinkBattle(B_COMM_TO_CONTROLLER, gBattleOutcome.get());
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_returntoball() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, TRUE);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_handlelearnnewmove() {
    let learnedMovePtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    let nothingToLearnPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24)
        as usize as *mut u8;
    let mut learnMove: u16 = MonTryLearningNewMove(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        *(*gBattlescriptCurrInstr.as_ptr()).at(9),
    );
    while learnMove == MON_ALREADY_KNOWS_MOVE {
        learnMove = MonTryLearningNewMove(
            &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
            FALSE,
        );
    }
    if learnMove == MOVE_NONE {
        crate::gl!(gBattlescriptCurrInstr = nothingToLearnPtr);
    } else if learnMove == MON_HAS_MAX_MOVES {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(10));
    } else {
        gActiveBattler.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
        if ({
            let gl_i294 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i294])
        }) == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
            && ({
                let gl_i295 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i295].status2)
            }) & STATUS2_TRANSFORMED
                == 0
        {
            GiveMoveToBattleMon(
                {
                    let gl_i296 = gActiveBattler.get();
                    crate::gl!(&raw gBattleMons[gl_i296])
                },
                learnMove,
            );
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
            gActiveBattler.set(GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT));
            if ({
                let gl_i297 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i297])
            }) == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                && ({
                    let gl_i298 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i298].status2)
                }) & STATUS2_TRANSFORMED
                    == 0
            {
                GiveMoveToBattleMon(
                    {
                        let gl_i299 = gActiveBattler.get();
                        crate::gl!(&raw gBattleMons[gl_i299])
                    },
                    learnMove,
                );
            }
        }
        crate::gl!(gBattlescriptCurrInstr = learnedMovePtr);
    }
}
pub(crate) unsafe fn Cmd_yesnoboxlearnmove() {
    gActiveBattler.set(0);
    match crate::gl!(gBattleScripting.learnMoveState) {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                B_WIN_YESNO,
            );
            crate::gl!(gBattleScripting.learnMoveState += 1);
            crate::gl!(gBattleCommunication[1] = 0);
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    != 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 0);
                BattleCreateYesNoCursorAt(0);
            }
            if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    == 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 1);
                BattleCreateYesNoCursorAt(1);
            }
            if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if crate::gl!(gBattleCommunication[1]) == 0 {
                    HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                    crate::gl!(gBattleScripting.learnMoveState += 1);
                } else {
                    crate::gl!(gBattleScripting.learnMoveState = 5);
                }
            } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                crate::gl!(gBattleScripting.learnMoveState = 5);
            }
        }
        2 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                FreeAllWindowBuffers();
                ShowSelectMovePokemonSummaryScreen(
                    (*gPlayerParty.as_ptr()).as_mut_ptr(),
                    (*crate::gl!(gBattleStruct)).expGetterMonId,
                    gPlayerPartyCount.get() - 1,
                    Some(ReshowBattleScreenAfterMenu),
                    gMoveToLearn.get(),
                );
                crate::gl!(gBattleScripting.learnMoveState += 1);
            }
        }
        3 => {
            if (*gPaletteFade.as_ptr()).active() == 0
                && crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
            {
                crate::gl!(gBattleScripting.learnMoveState += 1);
            }
        }
        4 => {
            if (*gPaletteFade.as_ptr()).active() == 0
                && crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
            {
                let movePosition: u8 = GetMoveSlotToReplace();
                if movePosition == MAX_MON_MOVES as u8 {
                    crate::gl!(gBattleScripting.learnMoveState = 5);
                } else {
                    let r#move: u16 = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())
                            [(*crate::gl!(gBattleStruct)).expGetterMonId],
                        MON_DATA_MOVE1 + movePosition as i32,
                    ) as u16;
                    if IsHMMove2(r#move) != 0 {
                        PrepareStringBattle(STRINGID_HMMOVESCANTBEFORGOTTEN, gActiveBattler.get());
                        crate::gl!(gBattleScripting.learnMoveState = 6);
                    } else {
                        crate::gl!(
                            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1)
                                as i32
                                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                                as usize
                                as *mut u8
                        );
                        crate::gl!(gBattleTextBuff2[0] = 0xFD);
                        crate::gl!(gBattleTextBuff2[1] = 2);
                        crate::gl!(gBattleTextBuff2[2] = r#move as u8);
                        crate::gl!(gBattleTextBuff2[3] = ((r#move as i32 & 0xFF00) >> 8) as u8);
                        crate::gl!(gBattleTextBuff2[4] = 0xFF);
                        RemoveMonPPBonus(
                            &raw mut (*gPlayerParty.as_ptr())
                                [(*crate::gl!(gBattleStruct)).expGetterMonId],
                            movePosition,
                        );
                        SetMonMoveSlot(
                            &raw mut (*gPlayerParty.as_ptr())
                                [(*crate::gl!(gBattleStruct)).expGetterMonId],
                            gMoveToLearn.get(),
                            movePosition,
                        );
                        if crate::gl!(gBattlerPartyIndexes[0])
                            == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                            && (crate::gl!(gBattleMons[0].status2) & 0x200000 == 0
                                && (*gDisableStructs.as_ptr())[0].mimickedMoves() as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[movePosition]
                                    == 0)
                        {
                            RemoveBattleMonPPBonus(crate::gl!(&raw gBattleMons[0]), movePosition);
                            SetBattleMonMoveSlot(
                                crate::gl!(&raw gBattleMons[0]),
                                gMoveToLearn.get(),
                                movePosition,
                            );
                        }
                        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
                            && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;)
                                .cast_const())
                            .cast::<CArray<u16, 4>>()
                            .cast_mut())[2]
                                == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                            && (crate::gl!(gBattleMons[2].status2) & 0x200000 == 0
                                && (*gDisableStructs.as_ptr())[2].mimickedMoves() as u32
                                    & (*(&raw const crate::util::gBitTable)
                                        .cast::<CArray<u32, 0>>())[movePosition]
                                    == 0)
                        {
                            RemoveBattleMonPPBonus(crate::gl!(&raw gBattleMons[2]), movePosition);
                            SetBattleMonMoveSlot(
                                crate::gl!(&raw gBattleMons[2]),
                                gMoveToLearn.get(),
                                movePosition,
                            );
                        }
                    }
                }
            }
        }
        5 => {
            HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        }
        6 if gBattleControllerExecFlags.get() == 0 => {
            crate::gl!(gBattleScripting.learnMoveState = 2);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_yesnoboxstoplearningmove() {
    match crate::gl!(gBattleScripting.learnMoveState) {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                B_WIN_YESNO,
            );
            crate::gl!(gBattleScripting.learnMoveState += 1);
            crate::gl!(gBattleCommunication[1] = 0);
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    != 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 0);
                BattleCreateYesNoCursorAt(0);
            }
            if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    == 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 1);
                BattleCreateYesNoCursorAt(1);
            }
            if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if crate::gl!(gBattleCommunication[1]) != 0 {
                    crate::gl!(
                        gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                            | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                            as usize as *mut u8
                    );
                } else {
                    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
                }
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
            } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                        as usize as *mut u8
                );
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_hitanimation() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    } else if gHitMarker.get() & HITMARKER_IGNORE_SUBSTITUTE == 0
        || ({
            let gl_i300 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i300].status2)
        }) & STATUS2_SUBSTITUTE
            == 0
        || ({
            let gl_i301 = gActiveBattler.get();
            crate::gl!(gDisableStructs[gl_i301].substituteHP)
        }) == 0
    {
        BtlController_EmitHitAnimation(B_COMM_TO_CONTROLLER);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    }
}
unsafe fn GetTrainerMoneyToGive(trainerId: u16) -> u32 {
    let mut i: u32 = 0;
    let mut lastMonLevel: u32 = 0;
    let mut moneyReward: u32 = 0;
    if trainerId == TRAINER_SECRET_BASE {
        moneyReward = 20
            * (*(*crate::gl!(gBattleResources)).secretBase).party.levels[0] as u32
            * (*crate::gl!(gBattleStruct)).moneyMultiplier as u32;
    } else {
        match (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [trainerId]
            .partyFlags
        {
            0 => {
                let party: *mut TrainerMonNoItemDefaultMoves =
                    (*(&raw const crate::data::data_tables::gTrainers)
                        .cast::<CArray<Trainer, 0>>())[trainerId]
                        .party
                        .NoItemDefaultMoves;
                lastMonLevel = (*party.at((*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerId]
                    .partySize as i32
                    - 1))
                .lvl as u32;
            }
            F_TRAINER_PARTY_CUSTOM_MOVESET => {
                let party: *mut TrainerMonNoItemCustomMoves =
                    (*(&raw const crate::data::data_tables::gTrainers)
                        .cast::<CArray<Trainer, 0>>())[trainerId]
                        .party
                        .NoItemCustomMoves;
                lastMonLevel = (*party.at((*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerId]
                    .partySize as i32
                    - 1))
                .lvl as u32;
            }
            F_TRAINER_PARTY_HELD_ITEM => {
                let party: *mut TrainerMonItemDefaultMoves =
                    (*(&raw const crate::data::data_tables::gTrainers)
                        .cast::<CArray<Trainer, 0>>())[trainerId]
                        .party
                        .ItemDefaultMoves;
                lastMonLevel = (*party.at((*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerId]
                    .partySize as i32
                    - 1))
                .lvl as u32;
            }
            3 => {
                let party: *mut TrainerMonItemCustomMoves =
                    (*(&raw const crate::data::data_tables::gTrainers)
                        .cast::<CArray<Trainer, 0>>())[trainerId]
                        .party
                        .ItemCustomMoves;
                lastMonLevel = (*party.at((*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerId]
                    .partySize as i32
                    - 1))
                .lvl as u32;
            }
            _ => {}
        }
        while (*(&raw const crate::data::battle_main::gTrainerMoneyTable)
            .cast::<CArray<TrainerMoney, 0>>())[i]
            .classId
            != 0xFF
        {
            if (*(&raw const crate::data::battle_main::gTrainerMoneyTable)
                .cast::<CArray<TrainerMoney, 0>>())[i]
                .classId
                == (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                    [trainerId]
                    .trainerClass
            {
                break;
            }
            i += 1;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            moneyReward = 4
                * lastMonLevel
                * (*crate::gl!(gBattleStruct)).moneyMultiplier as u32
                * (*(&raw const crate::data::battle_main::gTrainerMoneyTable)
                    .cast::<CArray<TrainerMoney, 0>>())[i]
                    .value as u32;
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
            moneyReward = 4
                * lastMonLevel
                * (*crate::gl!(gBattleStruct)).moneyMultiplier as u32
                * 2
                * (*(&raw const crate::data::battle_main::gTrainerMoneyTable)
                    .cast::<CArray<TrainerMoney, 0>>())[i]
                    .value as u32;
        } else {
            moneyReward = 4
                * lastMonLevel
                * (*crate::gl!(gBattleStruct)).moneyMultiplier as u32
                * (*(&raw const crate::data::battle_main::gTrainerMoneyTable)
                    .cast::<CArray<TrainerMoney, 0>>())[i]
                    .value as u32;
        }
    }
    moneyReward
}
pub(crate) unsafe fn Cmd_getmoneyreward() {
    let mut moneyReward: u32 = GetTrainerMoneyToGive(gTrainerBattleOpponent_A.get());
    if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        moneyReward += GetTrainerMoneyToGive(gTrainerBattleOpponent_B.get());
    }
    AddMoney(&raw mut (*crate::gl!(gSaveBlock1Ptr)).money, moneyReward);
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 1);
    crate::gl!(gBattleTextBuff1[2] = 4);
    crate::gl!(gBattleTextBuff1[3] = 5);
    crate::gl!(gBattleTextBuff1[4] = moneyReward as u8);
    crate::gl!(gBattleTextBuff1[5] = ((moneyReward & 0x0000FF00) >> 8) as u8);
    crate::gl!(gBattleTextBuff1[6] = ((moneyReward & 0x00FF0000) >> 16) as u8);
    crate::gl!(gBattleTextBuff1[7] = ((moneyReward & 0xFF000000) >> 24) as u8);
    crate::gl!(gBattleTextBuff1[8] = 0xFF);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_updatebattlermoves() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            BtlController_EmitGetMonData(B_COMM_TO_CONTROLLER, REQUEST_ALL_BATTLE, 0);
            MarkBattlerForControllerExec(gActiveBattler.get());
            crate::gl!(gBattleCommunication[0] += 1);
        }
        1 if gBattleControllerExecFlags.get() == 0 => {
            let bufferPoke: *mut BattlePokemon =
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferB;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut BattlePokemon;
            for i in 0..MAX_MON_MOVES {
                {
                    let gl_i302 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i302].moves[i] = (*bufferPoke).moves[i])
                };
                {
                    let gl_i303 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i303].pp[i] = (*bufferPoke).pp[i])
                };
            }
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_swapattackerwithtarget() {
    gActiveBattler.set(gBattlerAttacker.get());
    gBattlerAttacker.set(gBattlerTarget.get());
    gBattlerTarget.set(gActiveBattler.get());
    if gHitMarker.get() & HITMARKER_SWAP_ATTACKER_TARGET != 0 {
        gHitMarker.set(gHitMarker.get() & (0xffffefff));
    } else {
        gHitMarker.set(gHitMarker.get() | (HITMARKER_SWAP_ATTACKER_TARGET));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_incrementgamestat() {
    if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
        IncrementGameStat(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_drawpartystatussummary() {
    let mut party: *mut Pokemon = null_mut();
    let mut hpStatuses: CArray<HpAndStatus, 6> = zeroed();
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    for i in 0..PARTY_SIZE {
        if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32
            || GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG
        {
            hpStatuses[i].hp = 0xFFFF;
            hpStatuses[i].status = 0;
        } else {
            hpStatuses[i].hp = GetMonData2(party.at(i), MON_DATA_HP) as u16;
            hpStatuses[i].status = GetMonData2(party.at(i), MON_DATA_STATUS);
        }
    }
    BtlController_EmitDrawPartyStatusSummary(B_COMM_TO_CONTROLLER, hpStatuses.as_mut_ptr(), 1);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_hidepartystatussummary() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    BtlController_EmitHidePartyStatusSummary(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_jumptocalledmove() {
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) != 0 {
        gCurrentMove.set(gCalledMove.get());
    } else {
        gChosenMove.set({
            gCurrentMove.set(gCalledMove.get());
            gCurrentMove.get()
        });
    }
    crate::gl!(
        gBattlescriptCurrInstr = (*crate::asmdata::gBattleScriptsForMoveEffects
            .cast::<CArray<*mut u8, 0>>())[(*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .effect]
    );
}
pub(crate) unsafe fn Cmd_statusanimation() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        if {
            let gl_i102 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i102])
        } & STATUS3_SEMI_INVULNERABLE
            == 0
            && ({
                let gl_i304 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i304].substituteHP)
            }) == 0
            && gHitMarker.get() & HITMARKER_NO_ANIMATIONS == 0
        {
            BtlController_EmitStatusAnimation(B_COMM_TO_CONTROLLER, FALSE, {
                let gl_i305 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i305].status1)
            });
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    }
}
pub(crate) unsafe fn Cmd_status2animation() {
    let mut wantedToAnimate: u32 = 0;
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        wantedToAnimate = *(*gBattlescriptCurrInstr.as_ptr()).at(2) as u32
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as u32) << 8
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as u32) << 16
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as u32) << 24;
        if {
            let gl_i103 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i103])
        } & STATUS3_SEMI_INVULNERABLE
            == 0
            && ({
                let gl_i306 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i306].substituteHP)
            }) == 0
            && gHitMarker.get() & HITMARKER_NO_ANIMATIONS == 0
        {
            BtlController_EmitStatusAnimation(
                B_COMM_TO_CONTROLLER,
                TRUE,
                ({
                    let gl_i307 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i307].status2)
                }) & wantedToAnimate,
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_chosenstatusanimation() {
    let mut wantedStatus: u32 = 0;
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        wantedStatus = *(*gBattlescriptCurrInstr.as_ptr()).at(3) as u32
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(1) as u32) << 8
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(2) as u32) << 16
            | (*(*gBattlescriptCurrInstr.as_ptr()).at(3).at(3) as u32) << 24;
        if {
            let gl_i104 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i104])
        } & STATUS3_SEMI_INVULNERABLE
            == 0
            && ({
                let gl_i308 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i308].substituteHP)
            }) == 0
            && gHitMarker.get() & HITMARKER_NO_ANIMATIONS == 0
        {
            BtlController_EmitStatusAnimation(
                B_COMM_TO_CONTROLLER,
                *(*gBattlescriptCurrInstr.as_ptr()).at(2),
                wantedStatus,
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(7));
    }
}
pub(crate) unsafe fn Cmd_yesnobox() {
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                B_WIN_YESNO,
            );
            crate::gl!(gBattleCommunication[0] += 1);
            crate::gl!(gBattleCommunication[1] = 0);
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    != 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 0);
                BattleCreateYesNoCursorAt(0);
            }
            if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    == 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 1);
                BattleCreateYesNoCursorAt(1);
            }
            if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                crate::gl!(gBattleCommunication[1] = 1);
                PlaySE(SE_SELECT);
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
            } else if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_cancelallactions() {
    for i in 0..(gBattlersCount.get() as i32) {
        crate::gl!(gActionsByTurnOrder[i] = B_ACTION_CANCEL_PARTNER);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_adjustsetdamage() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    if ({
        let gl_i309 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i309].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffect = {
            let gl_i105 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i105].holdEffect)
        };
        param = {
            let gl_i106 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i106].holdEffectParam)
        };
    } else {
        holdEffect = GetItemHoldEffect({
            let gl_i310 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i310].item)
        });
        param = GetItemHoldEffectParam({
            let gl_i311 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i311].item)
        });
    }
    gPotentialItemEffectBattler.set(gBattlerTarget.get());
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget.get(), holdEffect);
        (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].set_focusBanded(1);
    }
    if ({
        let gl_i312 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i312].status2)
    }) & STATUS2_SUBSTITUTE
        == 0
        && ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_FALSE_SWIPE
            || (*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0
            || (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0)
        && ({
            let gl_i313 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i313].hp)
        }) as i32
            <= gBattleMoveDamage.get()
    {
        gBattleMoveDamage.set(
            ({
                let gl_i314 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i314].hp)
            }) as i32
                - 1,
        );
        if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_ENDURED));
        } else if (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0 {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_HUNG_ON));
            gLastUsedItem.set({
                let gl_i315 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i315].item)
            });
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_removeitem() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    let usedHeldItem: *mut u16 =
        &raw mut (*crate::gl!(gBattleStruct)).usedHeldItems[gActiveBattler.get()];
    *usedHeldItem = {
        let gl_i316 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i316].item)
    };
    {
        let gl_i317 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i317].item = ITEM_NONE)
    };
    BtlController_EmitSetMonData(
        B_COMM_TO_CONTROLLER,
        REQUEST_HELDITEM_BATTLE,
        0,
        2,
        ({
            let gl_i318 = gActiveBattler.get();
            crate::gl!(&raw gBattleMons[gl_i318].item)
        }) as *mut c_void,
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_atknameinbuff1() {
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 7);
    crate::gl!(gBattleTextBuff1[2] = gBattlerAttacker.get());
    crate::gl!(
        gBattleTextBuff1[3] = ({
            let gl_i319 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i319])
        }) as u8
    );
    crate::gl!(gBattleTextBuff1[4] = 0xFF);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_drawlvlupbox() {
    if crate::gl!(gBattleScripting.drawlvlupboxState) == 0 {
        if IsMonGettingExpSentOut() != 0 {
            crate::gl!(gBattleScripting.drawlvlupboxState = 3);
        } else {
            crate::gl!(gBattleScripting.drawlvlupboxState = 1);
        }
    }
    match crate::gl!(gBattleScripting.drawlvlupboxState) {
        1 => {
            gBattle_BG2_Y.set(96);
            SetBgAttribute(2, BG_ATTR_PRIORITY, 0);
            ShowBg(2);
            InitLevelUpBanner();
            crate::gl!(gBattleScripting.drawlvlupboxState = 2);
        }
        2 => {
            if SlideInLevelUpBanner() == 0 {
                crate::gl!(gBattleScripting.drawlvlupboxState = 3);
            }
        }
        3 => {
            gBattle_BG1_X.set(0);
            gBattle_BG1_Y.set(256);
            SetBgAttribute(0, BG_ATTR_PRIORITY, 1);
            SetBgAttribute(1, BG_ATTR_PRIORITY, 0);
            ShowBg(0);
            ShowBg(1);
            HandleBattleWindow(18, 7, 29, 19, WINDOW_BG1);
            crate::gl!(gBattleScripting.drawlvlupboxState = 4);
        }
        4 => {
            DrawLevelUpWindow1();
            PutWindowTilemap(B_WIN_LEVEL_UP_BOX);
            CopyWindowToVram(B_WIN_LEVEL_UP_BOX, COPYWIN_FULL);
            crate::gl!(gBattleScripting.drawlvlupboxState += 1);
        }
        5 | 7 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                gBattle_BG1_Y.set(0);
                crate::gl!(gBattleScripting.drawlvlupboxState += 1);
            }
        }
        6 => {
            if crate::gl!(gMain.newKeys) != 0 {
                PlaySE(SE_SELECT);
                DrawLevelUpWindow2();
                CopyWindowToVram(B_WIN_LEVEL_UP_BOX, COPYWIN_GFX);
                crate::gl!(gBattleScripting.drawlvlupboxState += 1);
            }
        }
        8 => {
            if crate::gl!(gMain.newKeys) != 0 {
                PlaySE(SE_SELECT);
                HandleBattleWindow(18, 7, 29, 19, 129);
                crate::gl!(gBattleScripting.drawlvlupboxState += 1);
            }
        }
        9 => {
            if SlideOutLevelUpBanner() == 0 {
                ClearWindowTilemap(B_WIN_LEVEL_UP_BANNER);
                CopyWindowToVram(B_WIN_LEVEL_UP_BANNER, COPYWIN_MAP);
                ClearWindowTilemap(B_WIN_LEVEL_UP_BOX);
                CopyWindowToVram(B_WIN_LEVEL_UP_BOX, COPYWIN_MAP);
                SetBgAttribute(2, BG_ATTR_PRIORITY, 2);
                ShowBg(2);
                crate::gl!(gBattleScripting.drawlvlupboxState = 10);
            }
        }
        10 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            SetBgAttribute(0, BG_ATTR_PRIORITY, 0);
            SetBgAttribute(1, BG_ATTR_PRIORITY, 1);
            ShowBg(0);
            ShowBg(1);
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        }
        _ => {}
    }
}
unsafe fn DrawLevelUpWindow1() {
    let mut currStats: CArray<u16, 6> = zeroed();
    GetMonLevelUpWindowStats(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        currStats.as_mut_ptr(),
    );
    DrawLevelUpWindowPg1(
        13,
        (*(*crate::gl!(gBattleResources)).beforeLvlUp)
            .stats
            .as_mut_ptr(),
        currStats.as_mut_ptr(),
        TEXT_DYNAMIC_COLOR_5,
        0xD,
        TEXT_DYNAMIC_COLOR_6,
    );
}
unsafe fn DrawLevelUpWindow2() {
    let mut currStats: CArray<u16, 6> = zeroed();
    GetMonLevelUpWindowStats(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        currStats.as_mut_ptr(),
    );
    DrawLevelUpWindowPg2(
        13,
        currStats.as_mut_ptr(),
        TEXT_DYNAMIC_COLOR_5,
        0xD,
        TEXT_DYNAMIC_COLOR_6,
    );
}
unsafe fn InitLevelUpBanner() {
    gBattle_BG2_Y.set(0);
    gBattle_BG2_X.set(LEVEL_UP_BANNER_START);
    LoadPalette(
        sLevelUpBanner_Pal.as_ptr().cast_mut() as *mut c_void,
        96,
        32,
    );
    CopyToWindowPixelBuffer(
        B_WIN_LEVEL_UP_BANNER,
        sLevelUpBanner_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    PutWindowTilemap(B_WIN_LEVEL_UP_BANNER);
    CopyWindowToVram(B_WIN_LEVEL_UP_BANNER, COPYWIN_FULL);
    PutMonIconOnLvlUpBanner();
}
unsafe fn SlideInLevelUpBanner() -> u8 {
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return TRUE;
    }
    if gBattle_BG2_X.get() == LEVEL_UP_BANNER_END {
        return FALSE;
    }
    if gBattle_BG2_X.get() == LEVEL_UP_BANNER_START {
        DrawLevelUpBannerText();
    }
    gBattle_BG2_X.set(gBattle_BG2_X.get() + (8));
    if gBattle_BG2_X.get() >= LEVEL_UP_BANNER_END {
        gBattle_BG2_X.set(LEVEL_UP_BANNER_END);
    }
    (gBattle_BG2_X.get() != LEVEL_UP_BANNER_END) as u8
}
unsafe fn DrawLevelUpBannerText() {
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    let monLevel: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        MON_DATA_LEVEL,
    ) as u16;
    let monGender: u8 = GetMonGender(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
    );
    GetMonNickname(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        gStringVar4.as_mut_ptr(),
    );
    printerTemplate.currentChar = gStringVar4.as_mut_ptr();
    printerTemplate.windowId = B_WIN_LEVEL_UP_BANNER;
    printerTemplate.fontId = FONT_SMALL;
    printerTemplate.x = 32;
    printerTemplate.y = 0;
    printerTemplate.currentX = 32;
    printerTemplate.currentY = 0;
    printerTemplate.letterSpacing = 0;
    printerTemplate.lineSpacing = 0;
    printerTemplate.set_unk(0);
    printerTemplate.set_fgColor(TEXT_COLOR_WHITE);
    printerTemplate.set_bgColor(TEXT_COLOR_TRANSPARENT);
    printerTemplate.set_shadowColor(TEXT_COLOR_DARK_GRAY);
    AddTextPrinter(&raw mut printerTemplate, TEXT_SKIP_DRAW, None);
    let mut txtPtr: *mut u8 = gStringVar4.as_mut_ptr();
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = CHAR_EXTRA_SYMBOL;
    *({
        let t2 = txtPtr;
        txtPtr = txtPtr.at(1);
        t2
    }) = CHAR_LV_2;
    let mut var: u32 = txtPtr as usize as u32;
    txtPtr = ConvertIntToDecimalStringN(txtPtr, monLevel as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    var = txtPtr as usize as u32 - var;
    txtPtr = StringFill(txtPtr, CHAR_SPACER, 4 - var as u16);
    if monGender != MON_GENDERLESS {
        if monGender == MON_MALE {
            txtPtr = WriteColorChangeControlCode(txtPtr, 0, TEXT_DYNAMIC_COLOR_3);
            txtPtr = WriteColorChangeControlCode(txtPtr, 1, TEXT_DYNAMIC_COLOR_4);
            *({
                let t3 = txtPtr;
                txtPtr = txtPtr.at(1);
                t3
            }) = CHAR_MALE;
        } else {
            txtPtr = WriteColorChangeControlCode(txtPtr, 0, TEXT_DYNAMIC_COLOR_5);
            txtPtr = WriteColorChangeControlCode(txtPtr, 1, TEXT_DYNAMIC_COLOR_6);
            *({
                let t4 = txtPtr;
                txtPtr = txtPtr.at(1);
                t4
            }) = CHAR_FEMALE;
        }
        *({
            let t5 = txtPtr;
            txtPtr = txtPtr.at(1);
            t5
        }) = EOS;
    }
    printerTemplate.y = 10;
    printerTemplate.currentY = 10;
    AddTextPrinter(&raw mut printerTemplate, TEXT_SKIP_DRAW, None);
    CopyWindowToVram(B_WIN_LEVEL_UP_BANNER, COPYWIN_GFX);
}
fn SlideOutLevelUpBanner() -> u8 {
    if gBattle_BG2_X.get() == LEVEL_UP_BANNER_START {
        return FALSE;
    }
    if gBattle_BG2_X.get() as i32 - 16 < LEVEL_UP_BANNER_START as i32 {
        gBattle_BG2_X.set(LEVEL_UP_BANNER_START);
    } else {
        gBattle_BG2_X.set(gBattle_BG2_X.get() - (16));
    }
    (gBattle_BG2_X.get() != LEVEL_UP_BANNER_START) as u8
}
unsafe fn PutMonIconOnLvlUpBanner() {
    let mut iconSheet: SpriteSheet = zeroed();
    let mut iconPalSheet: SpritePalette = zeroed();
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        MON_DATA_SPECIES,
    ) as u16;
    let personality: u32 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[(*crate::gl!(gBattleStruct)).expGetterMonId],
        MON_DATA_PERSONALITY,
    );
    let iconPtr: *mut u8 = GetMonIconPtr(species, personality, 1);
    iconSheet.data = iconPtr as *mut c_void;
    iconSheet.size = 0x200;
    iconSheet.tag = TAG_LVLUP_BANNER_MON_ICON;
    let iconPal: *mut u16 = GetValidMonIconPalettePtr(species);
    iconPalSheet.data = iconPal;
    iconPalSheet.tag = TAG_LVLUP_BANNER_MON_ICON;
    LoadSpriteSheet(&raw mut iconSheet);
    LoadSpritePalette(&raw mut iconPalSheet);
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_MonIconOnLvlUpBanner).cast_mut(),
        256,
        10,
        0,
    );
    {
        let gl_i320 = sDestroy;
        crate::gl!(gSprites[spriteId].data[gl_i320] = FALSE as i16)
    };
    {
        let gl_i321 = sXOffset;
        crate::gl!(gSprites[spriteId].data[gl_i321] = gBattle_BG2_X.get() as i16)
    };
}
pub(crate) unsafe fn SpriteCB_MonIconOnLvlUpBanner(sprite: *mut Sprite) {
    (*sprite).x2 = (*sprite).data[sXOffset] - gBattle_BG2_X.get() as i16;
    if (*sprite).x2 != 0 {
        (*sprite).data[sDestroy] = TRUE as i16;
    } else if (*sprite).data[sDestroy] != 0 {
        DestroySprite(sprite);
        FreeSpriteTilesByTag(TAG_LVLUP_BANNER_MON_ICON);
        FreeSpritePaletteByTag(TAG_LVLUP_BANNER_MON_ICON);
    }
}
unsafe fn IsMonGettingExpSentOut() -> u32 {
    if crate::gl!(gBattlerPartyIndexes[0]) == (*crate::gl!(gBattleStruct)).expGetterMonId as u16 {
        return TRUE as u32;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[2]
            == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
    {
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn Cmd_resetsentmonsvalue() {
    ResetSentPokesToOpponentValue();
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setatktoplayer0() {
    gBattlerAttacker.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_makevisible() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, FALSE);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_recordlastability() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    RecordAbilityBattle(gActiveBattler.get(), gLastUsedAbility.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub fn BufferMoveToLearnIntoBattleTextBuff2() {
    crate::gl!(gBattleTextBuff2[0] = 0xFD);
    crate::gl!(gBattleTextBuff2[1] = 2);
    crate::gl!(gBattleTextBuff2[2] = gMoveToLearn.get() as u8);
    crate::gl!(gBattleTextBuff2[3] = ((gMoveToLearn.get() as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(gBattleTextBuff2[4] = 0xFF);
}
pub(crate) unsafe fn Cmd_buffermovetolearn() {
    BufferMoveToLearnIntoBattleTextBuff2();
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_jumpifplayerran() {
    if TryRunFromBattle(gBattlerFainted.get()) != 0 {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_hpthresholds() {
    let mut opposingBattler: u8 = 0;
    let mut result: i32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        opposingBattler = gActiveBattler.get() ^ 1;
        result = div_i32(
            crate::gl!(gBattleMons[opposingBattler].hp) as i32 * 100,
            crate::gl!(gBattleMons[opposingBattler].maxHP) as i32,
        );
        if result == 0 {
            result = 1;
        }
        if result > 69 || crate::gl!(gBattleMons[opposingBattler].hp) == 0 {
            (*crate::gl!(gBattleStruct)).hpScale = 0;
        } else if result > 39 {
            (*crate::gl!(gBattleStruct)).hpScale = 1;
        } else if result > 9 {
            (*crate::gl!(gBattleStruct)).hpScale = 2;
        } else {
            (*crate::gl!(gBattleStruct)).hpScale = 3;
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_hpthresholds2() {
    let mut opposingBattler: u8 = 0;
    let mut result: i32 = 0;
    let mut hpSwitchout: u8 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        opposingBattler = gActiveBattler.get() ^ 1;
        hpSwitchout = *(*crate::gl!(gBattleStruct))
            .hpOnSwitchout
            .as_mut_ptr()
            .at(GetBattlerSide(opposingBattler)) as u8;
        result = div_i32(
            (hpSwitchout as i32 - crate::gl!(gBattleMons[opposingBattler].hp) as i32) * 100,
            hpSwitchout as i32,
        );
        if crate::gl!(gBattleMons[opposingBattler].hp) >= hpSwitchout as u16 {
            (*crate::gl!(gBattleStruct)).hpScale = 0;
        } else if result <= 29 {
            (*crate::gl!(gBattleStruct)).hpScale = 1;
        } else if result <= 69 {
            (*crate::gl!(gBattleStruct)).hpScale = 2;
        } else {
            (*crate::gl!(gBattleStruct)).hpScale = 3;
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_useitemonopponent() {
    gBattlerInMenuId.set(gBattlerAttacker.get());
    PokemonUseItemEffects(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i322 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i322])
        }],
        gLastUsedItem.get(),
        ({
            let gl_i323 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i323])
        }) as u8,
        0,
        TRUE,
    );
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_various() {
    let mut side: u8 = 0;
    let mut i: i32 = 0;
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    match *(*gBattlescriptCurrInstr.as_ptr()).at(2) {
        VARIOUS_CANCEL_MULTI_TURN_MOVES => {
            CancelMultiTurnMoves(gActiveBattler.get());
        }
        VARIOUS_SET_MAGIC_COAT_TARGET => {
            gBattlerAttacker.set(gBattlerTarget.get());
            side = GetBattlerSide(gBattlerAttacker.get()) ^ 1;
            if crate::gl!(gSideTimers[side].followmeTimer) != 0
                && ({
                    let gl_i324 = crate::gl!(gSideTimers[side].followmeTarget);
                    crate::gl!(gBattleMons[gl_i324].hp)
                }) != 0
            {
                gBattlerTarget.set(crate::gl!(gSideTimers[side].followmeTarget));
            } else {
                gBattlerTarget.set(gActiveBattler.get());
            }
        }
        VARIOUS_IS_RUNNING_IMPOSSIBLE => {
            crate::gl!(gBattleCommunication[0] = IsRunningFromBattleImpossible());
        }
        VARIOUS_GET_MOVE_TARGET => {
            gBattlerTarget.set(GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE));
        }
        VARIOUS_GET_BATTLER_FAINTED => {
            if gHitMarker.get()
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gActiveBattler.get()]
                    << 28
                != 0
            {
                crate::gl!(gBattleCommunication[0] = TRUE);
            } else {
                crate::gl!(gBattleCommunication[0] = 0);
            }
        }
        VARIOUS_RESET_INTIMIDATE_TRACE_BITS => {
            (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].set_intimidatedMon(0);
            (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].set_traced(0);
        }
        VARIOUS_UPDATE_CHOICE_MOVE_ON_LVL_UP => {
            if crate::gl!(gBattlerPartyIndexes[0])
                == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                || crate::gl!(gBattlerPartyIndexes[2])
                    == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
            {
                let mut choicedMove: *mut u16 = null_mut();
                if crate::gl!(gBattlerPartyIndexes[0])
                    == (*crate::gl!(gBattleStruct)).expGetterMonId as u16
                {
                    gActiveBattler.set(0);
                } else {
                    gActiveBattler.set(2);
                }
                choicedMove =
                    &raw mut (*crate::gl!(gBattleStruct)).choicedMove[gActiveBattler.get()];
                i = 0;
                while i < MAX_MON_MOVES {
                    if ({
                        let gl_i325 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i325].moves[i])
                    }) == *choicedMove
                    {
                        break;
                    }
                    i += 1;
                }
                if i == MAX_MON_MOVES {
                    *choicedMove = MOVE_NONE;
                }
            }
        }
        VARIOUS_RESET_PLAYER_FAINTED => {
            if gBattleTypeFlags.get() & 3 == 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
                && crate::gl!(gBattleMons[0].hp) != 0
                && crate::gl!(gBattleMons[1].hp) != 0
            {
                gHitMarker.set(gHitMarker.get() & (0xffbfffff));
            }
        }
        VARIOUS_PALACE_FLAVOR_TEXT => {
            crate::gl!(gBattleCommunication[0] = 0);
            crate::gl!(
                gBattleScripting.battler = {
                    gActiveBattler.set(crate::gl!(gBattleCommunication[1]));
                    gActiveBattler.get()
                }
            );
            if (*crate::gl!(gBattleStruct)).palaceFlags as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gActiveBattler.get()]
                == 0
                && ({
                    let gl_i326 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i326].maxHP)
                }) as i32
                    / 2
                    >= ({
                        let gl_i327 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i327].hp)
                    }) as i32
                && ({
                    let gl_i328 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i328].hp)
                }) != 0
                && ({
                    let gl_i329 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i329].status1)
                }) & STATUS1_SLEEP
                    == 0
            {
                (*crate::gl!(gBattleStruct)).palaceFlags |= gBitTable[gActiveBattler.get()] as u8;
                crate::gl!(gBattleCommunication[0] = TRUE);
                crate::gl!(
                    gBattleCommunication[5] = sBattlePalaceNatureToFlavorTextId
                        [GetNatureFromPersonality({
                            let gl_i330 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i330].personality)
                        })]
                );
            }
        }
        VARIOUS_ARENA_JUDGMENT_WINDOW => {
            i = BattleArena_ShowJudgmentWindow(
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;)
                    .cast_const())
                .cast::<CArray<u8, 8>>()
                .cast_mut())[0],
            ) as i32;
            if i == ARENA_RESULT_RUNNING as i32 {
                return;
            }
            crate::gl!(gBattleCommunication[1] = i as u8);
        }
        VARIOUS_ARENA_OPPONENT_MON_LOST => {
            crate::gl!(gBattleMons[1].hp = 0);
            gHitMarker.set(gHitMarker.get() | (gBitTable[1] << 28));
            (*crate::gl!(gBattleStruct)).arenaLostOpponentMons |=
                gBitTable[crate::gl!(gBattlerPartyIndexes[1])] as u8;
            (*gDisableStructs.as_ptr())[1].set_truantSwitchInHack(1);
        }
        VARIOUS_ARENA_PLAYER_MON_LOST => {
            crate::gl!(gBattleMons[0].hp = 0);
            gHitMarker.set(gHitMarker.get() | (gBitTable[0] << 28));
            gHitMarker.set(gHitMarker.get() | (HITMARKER_PLAYER_FAINTED));
            (*crate::gl!(gBattleStruct)).arenaLostPlayerMons |=
                gBitTable[crate::gl!(gBattlerPartyIndexes[0])] as u8;
            (*gDisableStructs.as_ptr())[0].set_truantSwitchInHack(1);
        }
        VARIOUS_ARENA_BOTH_MONS_LOST => {
            crate::gl!(gBattleMons[0].hp = 0);
            crate::gl!(gBattleMons[1].hp = 0);
            gHitMarker.set(gHitMarker.get() | (gBitTable[0] << 28));
            gHitMarker.set(gHitMarker.get() | (gBitTable[1] << 28));
            gHitMarker.set(gHitMarker.get() | (HITMARKER_PLAYER_FAINTED));
            (*crate::gl!(gBattleStruct)).arenaLostPlayerMons |=
                gBitTable[crate::gl!(gBattlerPartyIndexes[0])] as u8;
            (*crate::gl!(gBattleStruct)).arenaLostOpponentMons |=
                gBitTable[crate::gl!(gBattlerPartyIndexes[1])] as u8;
            (*gDisableStructs.as_ptr())[0].set_truantSwitchInHack(1);
            (*gDisableStructs.as_ptr())[1].set_truantSwitchInHack(1);
        }
        VARIOUS_EMIT_YESNOBOX => {
            BtlController_EmitYesNoBox(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        VARIOUS_DRAW_ARENA_REF_TEXT_BOX => {
            DrawArenaRefereeTextBox();
        }
        VARIOUS_ERASE_ARENA_REF_TEXT_BOX => {
            EraseArenaRefereeTextBox();
        }
        VARIOUS_ARENA_JUDGMENT_STRING => {
            BattleStringExpandPlaceholdersToDisplayedString(
                (*(&raw const crate::data::battle_message::gRefereeStringsTable)
                    .cast::<CArray<*mut u8, 0>>())[*(*gBattlescriptCurrInstr.as_ptr()).at(1)],
            );
            BattlePutTextOnWindow(
                (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
                ARENA_WIN_JUDGMENT_TEXT,
            );
        }
        VARIOUS_ARENA_WAIT_STRING => {
            if IsTextPrinterActive(ARENA_WIN_JUDGMENT_TEXT) != 0 {
                return;
            }
        }
        VARIOUS_WAIT_CRY => {
            if IsCryFinished() == 0 {
                return;
            }
        }
        VARIOUS_RETURN_OPPONENT_MON1 => {
            gActiveBattler.set(1);
            if ({
                let gl_i331 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i331].hp)
            }) != 0
            {
                BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, FALSE);
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
        }
        VARIOUS_RETURN_OPPONENT_MON2 => {
            if gBattlersCount.get() > 3 {
                gActiveBattler.set(3);
                if ({
                    let gl_i332 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i332].hp)
                }) != 0
                {
                    BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, FALSE);
                    MarkBattlerForControllerExec(gActiveBattler.get());
                }
            }
        }
        VARIOUS_VOLUME_DOWN => {
            m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x55);
        }
        VARIOUS_VOLUME_UP => {
            m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x100);
        }
        VARIOUS_SET_ALREADY_STATUS_MOVE_ATTEMPT => {
            (*crate::gl!(gBattleStruct)).alreadyStatusedMoveAttempt |=
                gBitTable[gActiveBattler.get()] as u8;
        }
        VARIOUS_PALACE_TRY_ESCAPE_STATUS => {
            if BattlePalace_TryEscapeStatus(gActiveBattler.get()) != 0 {
                return;
            }
        }
        VARIOUS_SET_TELEPORT_OUTCOME => {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
                gBattleOutcome.set(B_OUTCOME_PLAYER_TELEPORTED);
            } else {
                gBattleOutcome.set(B_OUTCOME_MON_TELEPORTED);
            }
        }
        VARIOUS_PLAY_TRAINER_DEFEATED_MUSIC => {
            BtlController_EmitPlayFanfareOrBGM(B_COMM_TO_CONTROLLER, MUS_VICTORY_TRAINER, TRUE);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        _ => {}
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(3));
}
pub(crate) unsafe fn Cmd_setprotectlike() {
    let mut notLastTurn: u8 = TRUE;
    let lastMove: u16 = {
        let gl_i107 = gBattlerAttacker.get();
        crate::gl!(gLastResultingMoves[gl_i107])
    };
    if lastMove != MOVE_PROTECT && lastMove != MOVE_DETECT && lastMove != MOVE_ENDURE {
        {
            let gl_i333 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i333].protectUses = 0)
        };
    }
    if gCurrentTurnActionNumber.get() as i32 == gBattlersCount.get() as i32 - 1 {
        notLastTurn = FALSE;
    }
    if sProtectSuccessRates[{
        let gl_i334 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i334].protectUses)
    }] >= Random()
        && notLastTurn != 0
    {
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_PROTECT
        {
            (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_protected(1);
            crate::gl!(gBattleCommunication[5] = B_MSG_PROTECTED_ITSELF);
        }
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [gCurrentMove.get()]
        .effect
            == EFFECT_ENDURE
        {
            (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_endured(1);
            crate::gl!(gBattleCommunication[5] = B_MSG_BRACED_ITSELF);
        }
        {
            let gl_i335 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i335].protectUses += 1)
        };
    } else {
        {
            let gl_i336 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i336].protectUses = 0)
        };
        crate::gl!(gBattleCommunication[5] = B_MSG_PROTECT_FAILED);
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_tryexplosion() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gBattlerTarget.set(0);
    while gBattlerTarget.get() < gBattlersCount.get() {
        if ({
            let gl_i337 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i337].ability)
        }) == ABILITY_DAMP
        {
            break;
        }
        gBattlerTarget.set(gBattlerTarget.get() + (1));
    }
    if gBattlerTarget.get() == gBattlersCount.get() {
        gActiveBattler.set(gBattlerAttacker.get());
        gBattleMoveDamage.set(
            ({
                let gl_i338 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i338].hp)
            }) as i32,
        );
        BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, INSTANT_HP_BAR_DROP as u16);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        gBattlerTarget.set(0);
        'l3: while gBattlerTarget.get() < gBattlersCount.get() {
            'l2: {
                if gBattlerTarget.get() == gBattlerAttacker.get() {
                    break 'l2;
                }
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gBattlerTarget.get()]
                    == 0
                {
                    break 'l3;
                }
            }
            gBattlerTarget.set(gBattlerTarget.get() + (1));
        }
    } else {
        gLastUsedAbility.set(ABILITY_DAMP);
        RecordAbilityBattle(gBattlerTarget.get(), {
            let gl_i339 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i339].ability)
        });
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_DampStopsExplosion
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    }
}
pub(crate) unsafe fn Cmd_setatkhptozero() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(gBattlerAttacker.get());
    {
        let gl_i340 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i340].hp = 0)
    };
    BtlController_EmitSetMonData(
        B_COMM_TO_CONTROLLER,
        REQUEST_HP_BATTLE,
        0,
        2,
        ({
            let gl_i341 = gActiveBattler.get();
            crate::gl!(&raw gBattleMons[gl_i341].hp)
        }) as *mut c_void,
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_jumpifnexttargetvalid() {
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        gBattlerTarget.set(gBattlerTarget.get() + (1));
        'l2: loop {
            'l1: {
                if gBattlerTarget.get() == gBattlerAttacker.get() {
                    break 'l1;
                }
                if gAbsentBattlerFlags.get() as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                        [gBattlerTarget.get()]
                    == 0
                {
                    break 'l2;
                }
            }
            gBattlerTarget.set(gBattlerTarget.get() + (1));
        }
        if gBattlerTarget.get() >= gBattlersCount.get() {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        } else {
            crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        }
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_tryhealhalfhealth() {
    let failPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if *(*gBattlescriptCurrInstr.as_ptr()).at(5) == BS_ATTACKER {
        gBattlerTarget.set(gBattlerAttacker.get());
    }
    gBattleMoveDamage.set(
        ({
            let gl_i342 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i342].maxHP)
        }) as i32
            / 2,
    );
    if gBattleMoveDamage.get() == 0 {
        gBattleMoveDamage.set(1);
    }
    gBattleMoveDamage.set(-gBattleMoveDamage.get());
    if ({
        let gl_i343 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i343].hp)
    }) == ({
        let gl_i344 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i344].maxHP)
    }) {
        crate::gl!(gBattlescriptCurrInstr = failPtr);
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_trymirrormove() {
    let mut r#move: u16 = 0;
    let mut validMoves: CArray<u16, 4> = zeroed();
    for i in 0..3i32 {
        validMoves[i] = MOVE_NONE;
    }
    let mut validMovesCount: i32 = 0;
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if i != gBattlerAttacker.get() as i32 {
            r#move = *(*crate::gl!(gBattleStruct))
                .lastTakenMoveFrom
                .as_mut_ptr()
                .at(i * 2 + gBattlerAttacker.get() as i32 * 8) as u16
                | (*(*crate::gl!(gBattleStruct))
                    .lastTakenMoveFrom
                    .as_mut_ptr()
                    .at(i * 2 + gBattlerAttacker.get() as i32 * 8)
                    .at(1) as u16)
                    << 8;
            if r#move != MOVE_NONE && r#move != MOVE_UNAVAILABLE {
                validMoves[validMovesCount] = r#move;
                validMovesCount += 1;
            }
        }
        i += 1;
    }
    r#move = *(*crate::gl!(gBattleStruct))
        .lastTakenMove
        .as_mut_ptr()
        .at(gBattlerAttacker.get() as i32 * 2) as u16
        | (*(*crate::gl!(gBattleStruct))
            .lastTakenMove
            .as_mut_ptr()
            .at(gBattlerAttacker.get() as i32 * 2)
            .at(1) as u16)
            << 8;
    if r#move != MOVE_NONE && r#move != MOVE_UNAVAILABLE {
        gHitMarker.set(gHitMarker.get() & (0xfffffbff));
        gCurrentMove.set(r#move);
        gBattlerTarget.set(GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE));
        crate::gl!(
            gBattlescriptCurrInstr =
                (*crate::asmdata::gBattleScriptsForMoveEffects.cast::<CArray<*mut u8, 0>>())
                    [(*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                    .effect]
        );
    } else if validMovesCount != 0 {
        gHitMarker.set(gHitMarker.get() & (0xfffffbff));
        i = rem_i32(Random() as i32, validMovesCount);
        gCurrentMove.set(validMoves[i]);
        gBattlerTarget.set(GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE));
        crate::gl!(
            gBattlescriptCurrInstr =
                (*crate::asmdata::gBattleScriptsForMoveEffects.cast::<CArray<*mut u8, 0>>())
                    [(*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                    .effect]
        );
    } else {
        (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()]
            .set_ppNotAffectedByPressure(TRUE as u32);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_setrain() {
    if gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0 {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_WEATHER_FAILED);
    } else {
        gBattleWeather.set(B_WEATHER_RAIN_TEMPORARY);
        crate::gl!(gBattleCommunication[5] = B_MSG_STARTED_RAIN);
        crate::gl!(gWishFutureKnock.weatherDuration = 5);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setreflect() {
    if {
        let gl_i108 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & SIDE_STATUS_REFLECT;
        crate::gl!(gSideStatuses[gl_i108])
    } as i32
        & SIDE_STATUS_REFLECT
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_SIDE_STATUS_FAILED);
    } else {
        {
            let gl_v = SIDE_STATUS_REFLECT as u16;
            let gl_i109 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & SIDE_STATUS_REFLECT;
            crate::gl!(gSideStatuses[gl_i109] |= gl_v)
        };
        {
            let gl_v = 5;
            let gl_i110 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i110].reflectTimer = gl_v)
        };
        {
            let gl_v = gBattlerAttacker.get();
            let gl_i111 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i111].reflectBattlerId = gl_v)
        };
        if gBattleTypeFlags.get() & 1 != 0 && CountAliveMonsInBattle(1) == 2 {
            crate::gl!(gBattleCommunication[5] = B_MSG_SET_REFLECT_DOUBLE);
        } else {
            crate::gl!(gBattleCommunication[5] = B_MSG_SET_REFLECT_SINGLE);
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setseeded() {
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0 || {
        let gl_i112 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i112])
    } & STATUS3_LEECHSEED
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_LEECH_SEED_MISS);
    } else if ({
        let gl_i345 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i345].types[0])
    }) == TYPE_GRASS
        || ({
            let gl_i346 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i346].types[1])
        }) == TYPE_GRASS
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_LEECH_SEED_FAIL);
    } else {
        {
            let gl_v = gBattlerAttacker.get() as u32;
            let gl_i113 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i113] |= gl_v)
        };
        {
            let gl_v = STATUS3_LEECHSEED;
            let gl_i114 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i114] |= gl_v)
        };
        crate::gl!(gBattleCommunication[5] = B_MSG_LEECH_SEED_SET);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_manipulatedamage() {
    match *(*gBattlescriptCurrInstr.as_ptr()).at(1) {
        DMG_CHANGE_SIGN => {
            gBattleMoveDamage.set(-gBattleMoveDamage.get());
        }
        DMG_RECOIL_FROM_MISS => {
            gBattleMoveDamage.set(gBattleMoveDamage.get() / (2));
            if gBattleMoveDamage.get() == 0 {
                gBattleMoveDamage.set(1);
            }
            if ({
                let gl_i347 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i347].maxHP)
            }) as i32
                / 2
                < gBattleMoveDamage.get()
            {
                gBattleMoveDamage.set(
                    ({
                        let gl_i348 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i348].maxHP)
                    }) as i32
                        / 2,
                );
            }
        }
        DMG_DOUBLED => {
            gBattleMoveDamage.set(gBattleMoveDamage.get() * (2));
        }
        _ => {}
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_trysetrest() {
    let failJump: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    gActiveBattler.set({
        gBattlerTarget.set(gBattlerAttacker.get());
        gBattlerTarget.get()
    });
    gBattleMoveDamage.set(
        -(({
            let gl_i349 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i349].maxHP)
        }) as i32),
    );
    if ({
        let gl_i350 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i350].hp)
    }) == ({
        let gl_i351 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i351].maxHP)
    }) {
        crate::gl!(gBattlescriptCurrInstr = failJump);
    } else {
        if ({
            let gl_i352 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i352].status1)
        }) & 248
            != 0
        {
            crate::gl!(gBattleCommunication[5] = B_MSG_REST_STATUSED);
        } else {
            crate::gl!(gBattleCommunication[5] = B_MSG_REST);
        }
        {
            let gl_i353 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i353].status1 = 3)
        };
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            0,
            4,
            ({
                let gl_i354 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i354].status1)
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_jumpifnotfirstturn() {
    let failJump: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if ({
        let gl_i355 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i355].isFirstTurn)
    }) != 0
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(gBattlescriptCurrInstr = failJump);
    }
}
pub(crate) unsafe fn Cmd_nop() {
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
#[unsafe(no_mangle)]
pub fn UproarWakeUpCheck(battler: u8) -> u8 {
    let mut i: i32 = 0;
    'l2: while i < gBattlersCount.get() as i32 {
        'l1: {
            if crate::gl!(gBattleMons[i].status2) & STATUS2_UPROAR == 0
                || crate::gl!(gBattleMons[battler].ability) == ABILITY_SOUNDPROOF
            {
                break 'l1;
            }
            crate::gl!(gBattleScripting.battler = i as u8);
            if gBattlerTarget.get() == 0xFF {
                gBattlerTarget.set(i as u8);
            } else if gBattlerTarget.get() as i32 == i {
                crate::gl!(gBattleCommunication[5] = B_MSG_CANT_SLEEP_UPROAR);
            } else {
                crate::gl!(gBattleCommunication[5] = B_MSG_UPROAR_KEPT_AWAKE);
            }
            break 'l2;
        }
        i += 1;
    }
    if i == gBattlersCount.get() as i32 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Cmd_jumpifcantmakeasleep() {
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if UproarWakeUpCheck(gBattlerTarget.get()) != 0 {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else if ({
        let gl_i356 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i356].ability)
    }) == ABILITY_INSOMNIA
        || ({
            let gl_i357 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i357].ability)
        }) == ABILITY_VITAL_SPIRIT
    {
        gLastUsedAbility.set({
            let gl_i358 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i358].ability)
        });
        crate::gl!(gBattleCommunication[5] = B_MSG_STAYED_AWAKE_USING);
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_stockpile() {
    if ({
        let gl_i359 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i359].stockpileCounter)
    }) == 3
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_CANT_STOCKPILE);
    } else {
        {
            let gl_i360 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i360].stockpileCounter += 1)
        };
        crate::gl!(gBattleTextBuff1[0] = 0xFD);
        crate::gl!(gBattleTextBuff1[1] = 1);
        crate::gl!(gBattleTextBuff1[2] = 1);
        crate::gl!(gBattleTextBuff1[3] = 1);
        crate::gl!(
            gBattleTextBuff1[4] = ({
                let gl_i361 = gBattlerAttacker.get();
                crate::gl!(gDisableStructs[gl_i361].stockpileCounter)
            })
        );
        crate::gl!(gBattleTextBuff1[5] = 0xFF);
        crate::gl!(gBattleCommunication[5] = B_MSG_STOCKPILED);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_stockpiletobasedamage() {
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if ({
        let gl_i362 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i362].stockpileCounter)
    }) == 0
    {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
    } else {
        if crate::gl!(gBattleCommunication[6]) != B_MSG_PROTECTED {
            gBattleMoveDamage.set(
                CalculateBaseDamage(
                    {
                        let gl_i363 = gBattlerAttacker.get();
                        crate::gl!(&raw gBattleMons[gl_i363])
                    },
                    {
                        let gl_i364 = gBattlerTarget.get();
                        crate::gl!(&raw gBattleMons[gl_i364])
                    },
                    gCurrentMove.get() as u32,
                    {
                        let gl_i115 = GetBattlerPosition(gBattlerTarget.get()) as i32 & 1;
                        crate::gl!(gSideStatuses[gl_i115])
                    },
                    0,
                    0,
                    gBattlerAttacker.get(),
                    gBattlerTarget.get(),
                ) * ({
                    let gl_i365 = gBattlerAttacker.get();
                    crate::gl!(gDisableStructs[gl_i365].stockpileCounter)
                }) as i32,
            );
            crate::gl!(
                gBattleScripting.animTurn = ({
                    let gl_i366 = gBattlerAttacker.get();
                    crate::gl!(gDisableStructs[gl_i366].stockpileCounter)
                })
            );
            if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].helpingHand() != 0 {
                gBattleMoveDamage.set(gBattleMoveDamage.get() * 15 / 10);
            }
        }
        {
            let gl_i367 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i367].stockpileCounter = 0)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_stockpiletohpheal() {
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if ({
        let gl_i368 = gBattlerAttacker.get();
        crate::gl!(gDisableStructs[gl_i368].stockpileCounter)
    }) == 0
    {
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        crate::gl!(gBattleCommunication[5] = B_MSG_SWALLOW_FAILED);
    } else if ({
        let gl_i369 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i369].maxHP)
    }) == ({
        let gl_i370 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i370].hp)
    }) {
        {
            let gl_i371 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i371].stockpileCounter = 0)
        };
        crate::gl!(gBattlescriptCurrInstr = jumpPtr);
        gBattlerTarget.set(gBattlerAttacker.get());
        crate::gl!(gBattleCommunication[5] = B_MSG_SWALLOW_FULL_HP);
    } else {
        gBattleMoveDamage.set(div_i32(
            ({
                let gl_i372 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i372].maxHP)
            }) as i32,
            shl_i32(
                1,
                3 - ({
                    let gl_i373 = gBattlerAttacker.get();
                    crate::gl!(gDisableStructs[gl_i373].stockpileCounter)
                }) as u32,
            ),
        ));
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        gBattleMoveDamage.set(-gBattleMoveDamage.get());
        crate::gl!(
            gBattleScripting.animTurn = ({
                let gl_i374 = gBattlerAttacker.get();
                crate::gl!(gDisableStructs[gl_i374].stockpileCounter)
            })
        );
        {
            let gl_i375 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i375].stockpileCounter = 0)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        gBattlerTarget.set(gBattlerAttacker.get());
    }
}
pub(crate) unsafe fn Cmd_negativedamage() {
    gBattleMoveDamage.set(-(gHpDealt.get() / 2));
    if gBattleMoveDamage.get() == 0 {
        gBattleMoveDamage.set(-1);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
unsafe fn ChangeStatBuffs(mut statValue: i8, statId: u8, mut flags: u8, BS_ptr: *mut u8) -> u8 {
    let mut certain: u8 = FALSE;
    let mut notProtectAffected: u8 = FALSE;
    let mut index: u32 = 0;
    if flags as i32 & MOVE_EFFECT_AFFECTS_USER as i32 != 0 {
        gActiveBattler.set(gBattlerAttacker.get());
    } else {
        gActiveBattler.set(gBattlerTarget.get());
    }
    flags &= 191;
    if flags as i32 & MOVE_EFFECT_CERTAIN as i32 != 0 {
        certain += 1;
    }
    flags &= 127;
    if flags as i32 & STAT_CHANGE_NOT_PROTECT_AFFECTED != 0 {
        notProtectAffected += 1;
    }
    flags &= 223;
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 5);
    crate::gl!(gBattleTextBuff1[2] = statId);
    crate::gl!(gBattleTextBuff1[3] = 0xFF);
    if statValue <= -1 {
        if {
            let gl_i116 = GetBattlerPosition(gActiveBattler.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i116].mistTimer)
        } != 0
            && certain == 0
            && gCurrentMove.get() != MOVE_CURSE
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].statLowered() != 0 {
                    crate::gl!(gBattlescriptCurrInstr = BS_ptr);
                } else {
                    BattleScriptPush(BS_ptr);
                    crate::gl!(gBattleScripting.battler = gActiveBattler.get());
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MistProtected
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].set_statLowered(1);
                }
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if gCurrentMove.get() != MOVE_CURSE
            && notProtectAffected != TRUE
            && JumpIfMoveAffectedByProtect(0) != 0
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ButItFailed
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            return STAT_CHANGE_DIDNT_WORK;
        } else if (({
            let gl_i376 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i376].ability)
        }) == ABILITY_CLEAR_BODY
            || ({
                let gl_i377 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i377].ability)
            }) == ABILITY_WHITE_SMOKE)
            && certain == 0
            && gCurrentMove.get() != MOVE_CURSE
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                if (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].statLowered() != 0 {
                    crate::gl!(gBattlescriptCurrInstr = BS_ptr);
                } else {
                    BattleScriptPush(BS_ptr);
                    crate::gl!(gBattleScripting.battler = gActiveBattler.get());
                    crate::gl!(
                        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_AbilityNoStatLoss
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                    );
                    gLastUsedAbility.set({
                        let gl_i378 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i378].ability)
                    });
                    RecordAbilityBattle(gActiveBattler.get(), gLastUsedAbility.get());
                    (*gSpecialStatuses.as_ptr())[gActiveBattler.get()].set_statLowered(1);
                }
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if ({
            let gl_i379 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i379].ability)
        }) == ABILITY_KEEN_EYE
            && certain == 0
            && statId == STAT_ACC as u8
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                BattleScriptPush(BS_ptr);
                crate::gl!(gBattleScripting.battler = gActiveBattler.get());
                crate::gl!(
                    gBattlescriptCurrInstr =
                        (*crate::asmdata::BattleScript_AbilityNoSpecificStatLoss
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                );
                gLastUsedAbility.set({
                    let gl_i380 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i380].ability)
                });
                RecordAbilityBattle(gActiveBattler.get(), gLastUsedAbility.get());
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if ({
            let gl_i381 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i381].ability)
        }) == ABILITY_HYPER_CUTTER
            && certain == 0
            && statId == STAT_ATK
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                BattleScriptPush(BS_ptr);
                crate::gl!(gBattleScripting.battler = gActiveBattler.get());
                crate::gl!(
                    gBattlescriptCurrInstr =
                        (*crate::asmdata::BattleScript_AbilityNoSpecificStatLoss
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                );
                gLastUsedAbility.set({
                    let gl_i382 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i382].ability)
                });
                RecordAbilityBattle(gActiveBattler.get(), gLastUsedAbility.get());
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if ({
            let gl_i383 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i383].ability)
        }) == ABILITY_SHIELD_DUST
            && flags == 0
        {
            return STAT_CHANGE_DIDNT_WORK;
        } else {
            statValue = -(statValue >> 4 & 7);
            crate::gl!(gBattleTextBuff2[0] = B_BUFF_PLACEHOLDER_BEGIN);
            index = 1;
            if statValue == -2 {
                crate::gl!(gBattleTextBuff2[1] = B_BUFF_STRING);
                crate::gl!(gBattleTextBuff2[2] = STRINGID_STATHARSHLY);
                crate::gl!(gBattleTextBuff2[3] = 0);
                index = 4;
            }
            {
                let gl_v = B_BUFF_STRING;
                let gl_i117 = {
                    let t1 = index;
                    index += 1;
                    t1
                };
                crate::gl!(gBattleTextBuff2[gl_i117] = gl_v)
            };
            {
                let gl_v = STRINGID_STATFELL;
                let gl_i118 = {
                    let t2 = index;
                    index += 1;
                    t2
                };
                crate::gl!(gBattleTextBuff2[gl_i118] = gl_v)
            };
            {
                let gl_v = 0;
                let gl_i119 = {
                    let t3 = index;
                    index += 1;
                    t3
                };
                crate::gl!(gBattleTextBuff2[gl_i119] = gl_v)
            };
            crate::gl!(gBattleTextBuff2[index] = B_BUFF_EOS);
            if ({
                let gl_i384 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i384].statStages[statId])
            }) == MIN_STAT_STAGE
            {
                crate::gl!(gBattleCommunication[5] = B_MSG_STAT_WONT_DECREASE);
            } else {
                crate::gl!(
                    gBattleCommunication[5] = (gBattlerTarget.get() == gActiveBattler.get()) as u8
                );
            }
        }
    } else {
        statValue = statValue >> 4 & 7;
        crate::gl!(gBattleTextBuff2[0] = B_BUFF_PLACEHOLDER_BEGIN);
        index = 1;
        if statValue == 2 {
            crate::gl!(gBattleTextBuff2[1] = B_BUFF_STRING);
            crate::gl!(gBattleTextBuff2[2] = STRINGID_STATSHARPLY);
            crate::gl!(gBattleTextBuff2[3] = 0);
            index = 4;
        }
        {
            let gl_v = B_BUFF_STRING;
            let gl_i120 = {
                let t4 = index;
                index += 1;
                t4
            };
            crate::gl!(gBattleTextBuff2[gl_i120] = gl_v)
        };
        {
            let gl_v = STRINGID_STATROSE;
            let gl_i121 = {
                let t5 = index;
                index += 1;
                t5
            };
            crate::gl!(gBattleTextBuff2[gl_i121] = gl_v)
        };
        {
            let gl_v = 0;
            let gl_i122 = {
                let t6 = index;
                index += 1;
                t6
            };
            crate::gl!(gBattleTextBuff2[gl_i122] = gl_v)
        };
        crate::gl!(gBattleTextBuff2[index] = B_BUFF_EOS);
        if ({
            let gl_i385 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i385].statStages[statId])
        }) == MAX_STAT_STAGE
        {
            crate::gl!(gBattleCommunication[5] = B_MSG_STAT_WONT_INCREASE);
        } else {
            crate::gl!(
                gBattleCommunication[5] = (gBattlerTarget.get() == gActiveBattler.get()) as u8
            );
        }
    }
    {
        let gl_i386 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i386].statStages[statId] += statValue)
    };
    if ({
        let gl_i387 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i387].statStages[statId])
    }) < MIN_STAT_STAGE
    {
        {
            let gl_i388 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i388].statStages[statId] = MIN_STAT_STAGE)
        };
    }
    if ({
        let gl_i389 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i389].statStages[statId])
    }) > MAX_STAT_STAGE
    {
        {
            let gl_i390 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i390].statStages[statId] = MAX_STAT_STAGE)
        };
    }
    if crate::gl!(gBattleCommunication[5]) == B_MSG_STAT_WONT_INCREASE
        && flags as i32 & STAT_CHANGE_ALLOW_PTR as i32 != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
    }
    if crate::gl!(gBattleCommunication[5]) == B_MSG_STAT_WONT_INCREASE
        && flags as i32 & STAT_CHANGE_ALLOW_PTR as i32 == 0
    {
        return STAT_CHANGE_DIDNT_WORK;
    }
    STAT_CHANGE_WORKED
}
pub(crate) unsafe fn Cmd_statbuffchange() {
    let jumpPtr: *mut u8 = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
        as usize as *mut u8;
    if ChangeStatBuffs(
        crate::gl!(gBattleScripting.statChanger) as i8 & -16,
        crate::gl!(gBattleScripting.statChanger) & 0xF,
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        jumpPtr,
    ) == STAT_CHANGE_WORKED
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_normalisebuffs() {
    for i in 0..(gBattlersCount.get() as i32) {
        for j in 0..NUM_BATTLE_STATS {
            crate::gl!(gBattleMons[i].statStages[j] = DEFAULT_STAT_STAGE);
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setbide() {
    {
        let gl_i391 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i391].status2 |= STATUS2_MULTIPLETURNS)
    };
    {
        let gl_v = gCurrentMove.get();
        let gl_i123 = gBattlerAttacker.get();
        crate::gl!(gLockedMoves[gl_i123] = gl_v)
    };
    {
        let gl_v = 0;
        let gl_i124 = gBattlerAttacker.get();
        crate::gl!(gBideDmg[gl_i124] = gl_v)
    };
    {
        let gl_i392 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i392].status2 |= 512)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_confuseifrepeatingattackends() {
    if ({
        let gl_i393 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i393].status2)
    }) & STATUS2_LOCK_CONFUSE
        == 0
    {
        crate::gl!(gBattleCommunication[3] = 117);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setmultihitcounter() {
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) != 0 {
        gMultiHitCounter.set(*(*gBattlescriptCurrInstr.as_ptr()).at(1));
    } else {
        gMultiHitCounter.set(Random() as u8 & 3);
        if gMultiHitCounter.get() > 1 {
            gMultiHitCounter.set((Random() as u8 & 3) + 2);
        } else {
            gMultiHitCounter.set(gMultiHitCounter.get() + (2));
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_initmultihitstring() {
    crate::gl!(gBattleScripting.multihitString[0] = 0xFD);
    crate::gl!(gBattleScripting.multihitString[1] = 1);
    crate::gl!(gBattleScripting.multihitString[2] = 1);
    crate::gl!(gBattleScripting.multihitString[3] = 1);
    crate::gl!(gBattleScripting.multihitString[4] = 0);
    crate::gl!(gBattleScripting.multihitString[5] = 0xFF);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
unsafe fn TryDoForceSwitchOut() -> u8 {
    if ({
        let gl_i394 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i394].level)
    }) >= ({
        let gl_i395 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i395].level)
    }) {
        *(*crate::gl!(gBattleStruct))
            .battlerPartyIndexes
            .as_mut_ptr()
            .at(gBattlerTarget.get()) = ({
            let gl_i396 = gBattlerTarget.get();
            crate::gl!(gBattlerPartyIndexes[gl_i396])
        }) as u8;
    } else {
        let random: u16 = Random() & 0xFF;
        if ((random as i32
            * (({
                let gl_i397 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i397].level)
            }) as i32
                + ({
                    let gl_i398 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i398].level)
                }) as i32))
            >> 8) as u32
            + 1
            <= (({
                let gl_i399 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i399].level)
            }) as i32
                / 4) as u32
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
            return FALSE;
        }
        *(*crate::gl!(gBattleStruct))
            .battlerPartyIndexes
            .as_mut_ptr()
            .at(gBattlerTarget.get()) = ({
            let gl_i400 = gBattlerTarget.get();
            crate::gl!(gBattlerPartyIndexes[gl_i400])
        }) as u8;
    }
    crate::gl!(
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SuccessForceOut
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut()
    );
    TRUE
}
pub(crate) unsafe fn Cmd_forcerandomswitch() {
    let mut i: i32 = 0;
    let mut battler1PartyId: i32 = 0;
    let mut battler2PartyId: i32 = 0;
    let mut firstMonId: i32 = 0;
    let mut lastMonId: i32 = 0;
    let mut monsCount: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut validMons: i32 = 0;
    let mut minNeeded: i32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 {
        if GetBattlerSide(gBattlerTarget.get()) == B_SIDE_PLAYER {
            party = (*gPlayerParty.as_ptr()).as_mut_ptr();
        } else {
            party = (*gEnemyParty.as_ptr()).as_mut_ptr();
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0
            && gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
            || gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0
            || gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
        {
            if gBattlerTarget.get() as i32 & BIT_FLANK as i32 != B_FLANK_LEFT {
                firstMonId = 3;
                lastMonId = PARTY_SIZE;
            } else {
                firstMonId = 0;
                lastMonId = 3;
            }
            monsCount = 3;
            minNeeded = 1;
            battler2PartyId = ({
                let gl_i401 = gBattlerTarget.get();
                crate::gl!(gBattlerPartyIndexes[gl_i401])
            }) as i32;
            battler1PartyId = ({
                let gl_i402 = gBattlerTarget.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i402])
            }) as i32;
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            && gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
            || gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0
        {
            if GetLinkTrainerFlankId(GetBattlerMultiplayerId(gBattlerTarget.get() as u16) as u8)
                == B_FLANK_RIGHT
            {
                firstMonId = 3;
                lastMonId = PARTY_SIZE;
            } else {
                firstMonId = 0;
                lastMonId = 3;
            }
            monsCount = 3;
            minNeeded = 1;
            battler2PartyId = ({
                let gl_i403 = gBattlerTarget.get();
                crate::gl!(gBattlerPartyIndexes[gl_i403])
            }) as i32;
            battler1PartyId = ({
                let gl_i404 = gBattlerTarget.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i404])
            }) as i32;
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            if GetBattlerSide(gBattlerTarget.get()) == B_SIDE_PLAYER {
                firstMonId = 0;
                lastMonId = PARTY_SIZE;
                monsCount = PARTY_SIZE;
                minNeeded = 2;
            } else {
                if gBattlerTarget.get() as i32 & BIT_FLANK as i32 != B_FLANK_LEFT {
                    firstMonId = 3;
                    lastMonId = PARTY_SIZE;
                } else {
                    firstMonId = 0;
                    lastMonId = 3;
                }
                monsCount = 3;
                minNeeded = 1;
            }
            battler2PartyId = ({
                let gl_i405 = gBattlerTarget.get();
                crate::gl!(gBattlerPartyIndexes[gl_i405])
            }) as i32;
            battler1PartyId = ({
                let gl_i406 = gBattlerTarget.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i406])
            }) as i32;
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
            firstMonId = 0;
            lastMonId = PARTY_SIZE;
            monsCount = PARTY_SIZE;
            minNeeded = 2;
            battler2PartyId = ({
                let gl_i407 = gBattlerTarget.get();
                crate::gl!(gBattlerPartyIndexes[gl_i407])
            }) as i32;
            battler1PartyId = ({
                let gl_i408 = gBattlerTarget.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i408])
            }) as i32;
        } else {
            firstMonId = 0;
            lastMonId = PARTY_SIZE;
            monsCount = PARTY_SIZE;
            minNeeded = 1;
            battler2PartyId = ({
                let gl_i409 = gBattlerTarget.get();
                crate::gl!(gBattlerPartyIndexes[gl_i409])
            }) as i32;
            battler1PartyId = ({
                let gl_i410 = gBattlerTarget.get();
                crate::gl!(gBattlerPartyIndexes[gl_i410])
            }) as i32;
        }
        i = firstMonId;
        while i < lastMonId {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
            {
                validMons += 1;
            }
            i += 1;
        }
        if validMons <= minNeeded {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            if TryDoForceSwitchOut() != 0 {
                loop {
                    loop {
                        i = rem_i32(Random() as i32, monsCount);
                        i += firstMonId;
                        if !(i == battler2PartyId || i == battler1PartyId) {
                            break;
                        }
                    }
                    if !(GetMonData2(party.at(i), MON_DATA_SPECIES) == SPECIES_NONE as u32
                        || GetMonData2(party.at(i), MON_DATA_IS_EGG) == TRUE as u32
                        || GetMonData2(party.at(i), MON_DATA_HP) == 0)
                    {
                        break;
                    }
                }
            }
            *(*crate::gl!(gBattleStruct))
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gBattlerTarget.get()) = i as u8;
            if IsMultiBattle() == 0 {
                SwitchPartyOrder(gBattlerTarget.get());
            }
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0
                || gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
                    && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
                || gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0
                    && gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0
                || gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0
                    && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            {
                SwitchPartyOrderLinkMulti(gBattlerTarget.get(), i as u8, 0);
                SwitchPartyOrderLinkMulti(gBattlerTarget.get() ^ 2, i as u8, 1);
            }
            if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
                SwitchPartyOrderInGameMulti(gBattlerTarget.get(), i as u8);
            }
        }
    } else {
        TryDoForceSwitchOut();
    }
}
pub(crate) unsafe fn Cmd_tryconversiontypechange() {
    let mut validMoves: u8 = 0;
    let mut moveType: u8 = 0;
    while validMoves < MAX_MON_MOVES as u8 {
        if ({
            let gl_i411 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i411].moves[validMoves])
        }) == MOVE_NONE
        {
            break;
        }
        validMoves += 1;
    }
    let mut moveChecked: u8 = 0;
    while moveChecked < validMoves {
        moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[{
            let gl_i412 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i412].moves[moveChecked])
        }]
        .r#type;
        if moveType == TYPE_MYSTERY {
            if ({
                let gl_i413 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i413].types[0])
            }) == TYPE_GHOST
                || ({
                    let gl_i414 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i414].types[1])
                }) == TYPE_GHOST
            {
                moveType = TYPE_GHOST;
            } else {
                moveType = TYPE_NORMAL;
            }
        }
        if moveType
            != ({
                let gl_i415 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i415].types[0])
            })
            && moveType
                != ({
                    let gl_i416 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i416].types[1])
                })
        {
            break;
        }
        moveChecked += 1;
    }
    if moveChecked == validMoves {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        loop {
            while ({
                moveChecked = (if 0 != 0 {
                    Random() as i32 % 4
                } else {
                    Random() as i32 & 3
                }) as u8;
                moveChecked
            }) >= validMoves
            {}
            moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[{
                let gl_i417 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i417].moves[moveChecked])
            }]
            .r#type;
            if moveType == TYPE_MYSTERY {
                if ({
                    let gl_i418 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i418].types[0])
                }) == TYPE_GHOST
                    || ({
                        let gl_i419 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i419].types[1])
                    }) == TYPE_GHOST
                {
                    moveType = TYPE_GHOST;
                } else {
                    moveType = TYPE_NORMAL;
                }
            }
            if !(moveType
                == ({
                    let gl_i420 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i420].types[0])
                })
                || moveType
                    == ({
                        let gl_i421 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i421].types[1])
                    }))
            {
                break;
            }
        }
        {
            let gl_i422 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i422].types[0] = moveType)
        };
        {
            let gl_i423 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i423].types[1] = moveType)
        };
        crate::gl!(gBattleTextBuff1[0] = 0xFD);
        crate::gl!(gBattleTextBuff1[1] = 3);
        crate::gl!(gBattleTextBuff1[2] = moveType);
        crate::gl!(gBattleTextBuff1[3] = 0xFF);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_givepaydaymoney() {
    if gBattleTypeFlags.get() & 0x2000002 == 0 && gPaydayMoney.get() != 0 {
        let bonusMoney: u32 =
            gPaydayMoney.get() as u32 * (*crate::gl!(gBattleStruct)).moneyMultiplier as u32;
        AddMoney(&raw mut (*crate::gl!(gSaveBlock1Ptr)).money, bonusMoney);
        crate::gl!(gBattleTextBuff1[0] = 0xFD);
        crate::gl!(gBattleTextBuff1[1] = 1);
        crate::gl!(gBattleTextBuff1[2] = 2);
        crate::gl!(gBattleTextBuff1[3] = 5);
        crate::gl!(gBattleTextBuff1[4] = bonusMoney as u8);
        crate::gl!(gBattleTextBuff1[5] = ((bonusMoney & 0x0000FF00) >> 8) as u8);
        crate::gl!(gBattleTextBuff1[6] = 0xFF);
        BattleScriptPush((*gBattlescriptCurrInstr.as_ptr()).at(1));
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintPayDayMoneyString
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_setlightscreen() {
    if {
        let gl_i125 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
        crate::gl!(gSideStatuses[gl_i125])
    } as i32
        & SIDE_STATUS_LIGHTSCREEN
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_SIDE_STATUS_FAILED);
    } else {
        {
            let gl_v = SIDE_STATUS_LIGHTSCREEN as u16;
            let gl_i126 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideStatuses[gl_i126] |= gl_v)
        };
        {
            let gl_v = 5;
            let gl_i127 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i127].lightscreenTimer = gl_v)
        };
        {
            let gl_v = gBattlerAttacker.get();
            let gl_i128 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i128].lightscreenBattlerId = gl_v)
        };
        if gBattleTypeFlags.get() & 1 != 0 && CountAliveMonsInBattle(1) == 2 {
            crate::gl!(gBattleCommunication[5] = B_MSG_SET_LIGHTSCREEN_DOUBLE);
        } else {
            crate::gl!(gBattleCommunication[5] = B_MSG_SET_LIGHTSCREEN_SINGLE);
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_tryKO() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    if ({
        let gl_i424 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i424].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffect = {
            let gl_i129 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i129].holdEffect)
        };
        param = {
            let gl_i130 = gBattlerTarget.get();
            crate::gl!(gEnigmaBerries[gl_i130].holdEffectParam)
        };
    } else {
        holdEffect = GetItemHoldEffect({
            let gl_i425 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i425].item)
        });
        param = GetItemHoldEffectParam({
            let gl_i426 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i426].item)
        });
    }
    gPotentialItemEffectBattler.set(gBattlerTarget.get());
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget.get(), HOLD_EFFECT_FOCUS_BAND);
        (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].set_focusBanded(1);
    }
    if ({
        let gl_i427 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i427].ability)
    }) == ABILITY_STURDY
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        gLastUsedAbility.set(ABILITY_STURDY);
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SturdyPreventsOHKO
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        RecordAbilityBattle(gBattlerTarget.get(), ABILITY_STURDY);
    } else {
        let mut chance: u16 = 0;
        if {
            let gl_i131 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i131])
        } & STATUS3_ALWAYS_HITS
            == 0
        {
            chance = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
            .accuracy as u16
                + (({
                    let gl_i428 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i428].level)
                }) as u16
                    - ({
                        let gl_i429 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i429].level)
                    }) as u16);
            if Random() as i32 % 100 + 1 < chance as i32
                && ({
                    let gl_i430 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i430].level)
                }) >= ({
                    let gl_i431 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i431].level)
                })
            {
                chance = TRUE as u16;
            } else {
                chance = FALSE as u16;
            }
        } else if ({
            let gl_i432 = gBattlerTarget.get();
            crate::gl!(gDisableStructs[gl_i432].battlerWithSureHit)
        }) == gBattlerAttacker.get()
            && ({
                let gl_i433 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i433].level)
            }) >= ({
                let gl_i434 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i434].level)
            })
        {
            chance = TRUE as u16;
        } else {
            chance = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
            .accuracy as u16
                + (({
                    let gl_i435 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i435].level)
                }) as u16
                    - ({
                        let gl_i436 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i436].level)
                    }) as u16);
            if Random() as i32 % 100 + 1 < chance as i32
                && ({
                    let gl_i437 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i437].level)
                }) >= ({
                    let gl_i438 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i438].level)
                })
            {
                chance = TRUE as u16;
            } else {
                chance = FALSE as u16;
            }
        }
        if chance != 0 {
            if (*gProtectStructs.as_ptr())[gBattlerTarget.get()].endured() != 0 {
                gBattleMoveDamage.set(
                    ({
                        let gl_i439 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i439].hp)
                    }) as i32
                        - 1,
                );
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_ENDURED));
            } else if (*gSpecialStatuses.as_ptr())[gBattlerTarget.get()].focusBanded() != 0 {
                gBattleMoveDamage.set(
                    ({
                        let gl_i440 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i440].hp)
                    }) as i32
                        - 1,
                );
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FOE_HUNG_ON));
                gLastUsedItem.set({
                    let gl_i441 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i441].item)
                });
            } else {
                gBattleMoveDamage.set(
                    ({
                        let gl_i442 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i442].hp)
                    }) as i32,
                );
                gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_ONE_HIT_KO as u8));
            }
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        } else {
            gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
            if ({
                let gl_i443 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i443].level)
            }) >= ({
                let gl_i444 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i444].level)
            }) {
                crate::gl!(gBattleCommunication[5] = B_MSG_KO_MISS);
            } else {
                crate::gl!(gBattleCommunication[5] = B_MSG_KO_UNAFFECTED);
            }
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
    }
}
pub(crate) unsafe fn Cmd_damagetohalftargethp() {
    gBattleMoveDamage.set(
        ({
            let gl_i445 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i445].hp)
        }) as i32
            / 2,
    );
    if gBattleMoveDamage.get() == 0 {
        gBattleMoveDamage.set(1);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setsandstorm() {
    if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM != 0 {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_WEATHER_FAILED);
    } else {
        gBattleWeather.set(B_WEATHER_SANDSTORM_TEMPORARY);
        crate::gl!(gBattleCommunication[5] = B_MSG_STARTED_SANDSTORM);
        crate::gl!(gWishFutureKnock.weatherDuration = 5);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_weatherdamage() {
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0 {
        if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM != 0 {
            if ({
                let gl_i446 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i446].types[0])
            }) != TYPE_ROCK
                && ({
                    let gl_i447 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i447].types[0])
                }) != TYPE_STEEL
                && ({
                    let gl_i448 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i448].types[0])
                }) != TYPE_GROUND
                && ({
                    let gl_i449 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i449].types[1])
                }) != TYPE_ROCK
                && ({
                    let gl_i450 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i450].types[1])
                }) != TYPE_STEEL
                && ({
                    let gl_i451 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i451].types[1])
                }) != TYPE_GROUND
                && ({
                    let gl_i452 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i452].ability)
                }) != ABILITY_SAND_VEIL
                && {
                    let gl_i132 = gBattlerAttacker.get();
                    crate::gl!(gStatuses3[gl_i132])
                } & STATUS3_UNDERGROUND
                    == 0
                && {
                    let gl_i133 = gBattlerAttacker.get();
                    crate::gl!(gStatuses3[gl_i133])
                } & STATUS3_UNDERWATER
                    == 0
            {
                gBattleMoveDamage.set(
                    ({
                        let gl_i453 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i453].maxHP)
                    }) as i32
                        / 16,
                );
                if gBattleMoveDamage.get() == 0 {
                    gBattleMoveDamage.set(1);
                }
            } else {
                gBattleMoveDamage.set(0);
            }
        }
        if gBattleWeather.get() as i32 & B_WEATHER_HAIL != 0 {
            if !(({
                let gl_i454 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i454].types[0])
            }) == TYPE_ICE
                || ({
                    let gl_i455 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i455].types[1])
                }) == TYPE_ICE)
                && {
                    let gl_i134 = gBattlerAttacker.get();
                    crate::gl!(gStatuses3[gl_i134])
                } & STATUS3_UNDERGROUND
                    == 0
                && {
                    let gl_i135 = gBattlerAttacker.get();
                    crate::gl!(gStatuses3[gl_i135])
                } & STATUS3_UNDERWATER
                    == 0
            {
                gBattleMoveDamage.set(
                    ({
                        let gl_i456 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i456].maxHP)
                    }) as i32
                        / 16,
                );
                if gBattleMoveDamage.get() == 0 {
                    gBattleMoveDamage.set(1);
                }
            } else {
                gBattleMoveDamage.set(0);
            }
        }
    } else {
        gBattleMoveDamage.set(0);
    }
    if gAbsentBattlerFlags.get() as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerAttacker.get()]
        != 0
    {
        gBattleMoveDamage.set(0);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_tryinfatuating() {
    let mut monAttacker: *mut Pokemon = null_mut();
    let mut monTarget: *mut Pokemon = null_mut();
    if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
        monAttacker = &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i457 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i457])
        }];
    } else {
        monAttacker = &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i458 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i458])
        }];
    }
    if GetBattlerSide(gBattlerTarget.get()) == B_SIDE_PLAYER {
        monTarget = &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i459 = gBattlerTarget.get();
            crate::gl!(gBattlerPartyIndexes[gl_i459])
        }];
    } else {
        monTarget = &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i460 = gBattlerTarget.get();
            crate::gl!(gBattlerPartyIndexes[gl_i460])
        }];
    }
    let speciesAttacker: u16 = GetMonData2(monAttacker, MON_DATA_SPECIES) as u16;
    let personalityAttacker: u32 = GetMonData2(monAttacker, MON_DATA_PERSONALITY);
    let speciesTarget: u16 = GetMonData2(monTarget, MON_DATA_SPECIES) as u16;
    let personalityTarget: u32 = GetMonData2(monTarget, MON_DATA_PERSONALITY);
    if ({
        let gl_i461 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i461].ability)
    }) == ABILITY_OBLIVIOUS
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ObliviousPreventsAttraction
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        gLastUsedAbility.set(ABILITY_OBLIVIOUS);
        RecordAbilityBattle(gBattlerTarget.get(), ABILITY_OBLIVIOUS);
    } else {
        if GetGenderFromSpeciesAndPersonality(speciesAttacker, personalityAttacker)
            == GetGenderFromSpeciesAndPersonality(speciesTarget, personalityTarget)
            || ({
                let gl_i462 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i462].status2)
            }) & STATUS2_INFATUATION
                != 0
            || GetGenderFromSpeciesAndPersonality(speciesAttacker, personalityAttacker)
                == MON_GENDERLESS
            || GetGenderFromSpeciesAndPersonality(speciesTarget, personalityTarget)
                == MON_GENDERLESS
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            {
                let gl_i463 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i463].status2 |= gBitTable[gBattlerAttacker.get()] << 16)
            };
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        }
    }
}
pub(crate) unsafe fn Cmd_updatestatusicon() {
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if *(*gBattlescriptCurrInstr.as_ptr()).at(1) != BS_ATTACKER_WITH_PARTNER {
        gActiveBattler.set(GetBattlerForBattleScript(
            *(*gBattlescriptCurrInstr.as_ptr()).at(1),
        ));
        BtlController_EmitStatusIconUpdate(
            B_COMM_TO_CONTROLLER,
            {
                let gl_i464 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i464].status1)
            },
            {
                let gl_i465 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i465].status2)
            },
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    } else {
        gActiveBattler.set(gBattlerAttacker.get());
        if gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
            == 0
        {
            BtlController_EmitStatusIconUpdate(
                B_COMM_TO_CONTROLLER,
                {
                    let gl_i466 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i466].status1)
                },
                {
                    let gl_i467 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i467].status2)
                },
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
            gActiveBattler.set(GetBattlerAtPosition(
                GetBattlerPosition(gBattlerAttacker.get()) ^ 2,
            ));
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gActiveBattler.get()]
                == 0
            {
                BtlController_EmitStatusIconUpdate(
                    B_COMM_TO_CONTROLLER,
                    {
                        let gl_i468 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i468].status1)
                    },
                    {
                        let gl_i469 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i469].status2)
                    },
                );
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
    }
}
pub(crate) unsafe fn Cmd_setmist() {
    if {
        let gl_i136 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
        crate::gl!(gSideTimers[gl_i136].mistTimer)
    } != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FAILED as u8));
        crate::gl!(gBattleCommunication[5] = B_MSG_MIST_FAILED);
    } else {
        {
            let gl_v = 5;
            let gl_i137 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i137].mistTimer = gl_v)
        };
        {
            let gl_v = gBattlerAttacker.get();
            let gl_i138 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i138].mistBattlerId = gl_v)
        };
        {
            let gl_v = SIDE_STATUS_MIST;
            let gl_i139 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideStatuses[gl_i139] |= gl_v)
        };
        crate::gl!(gBattleCommunication[5] = B_MSG_SET_MIST);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setfocusenergy() {
    if ({
        let gl_i470 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i470].status2)
    }) & STATUS2_FOCUS_ENERGY
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FAILED as u8));
        crate::gl!(gBattleCommunication[5] = B_MSG_FOCUS_ENERGY_FAILED);
    } else {
        {
            let gl_i471 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i471].status2 |= STATUS2_FOCUS_ENERGY)
        };
        crate::gl!(gBattleCommunication[5] = B_MSG_GETTING_PUMPED);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_transformdataexecution() {
    gChosenMove.set(MOVE_UNAVAILABLE);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    if ({
        let gl_i472 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i472].status2)
    }) & STATUS2_TRANSFORMED
        != 0
        || {
            let gl_i140 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i140])
        } & STATUS3_SEMI_INVULNERABLE
            != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_FAILED as u8));
        crate::gl!(gBattleCommunication[5] = B_MSG_TRANSFORM_FAILED);
    } else {
        {
            let gl_i473 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i473].status2 |= STATUS2_TRANSFORMED)
        };
        {
            let gl_i474 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i474].disabledMove = MOVE_NONE)
        };
        (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_disableTimer(0);
        {
            let gl_i475 = gBattlerAttacker.get();
            crate::gl!(
                gDisableStructs[gl_i475].transformedMonPersonality = ({
                    let gl_i476 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i476].personality)
                })
            )
        };
        (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_mimickedMoves(0);
        crate::gl!(gBattleTextBuff1[0] = 0xFD);
        crate::gl!(gBattleTextBuff1[1] = 6);
        crate::gl!(
            gBattleTextBuff1[2] = ({
                let gl_i477 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i477].species)
            }) as u8
        );
        crate::gl!(
            gBattleTextBuff1[3] = ((({
                let gl_i478 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i478].species)
            }) as i32
                & 0xFF00)
                >> 8) as u8
        );
        crate::gl!(gBattleTextBuff1[4] = 0xFF);
        let battleMonAttacker: *mut u8 = ({
            let gl_i479 = gBattlerAttacker.get();
            crate::gl!(&raw gBattleMons[gl_i479])
        }) as *mut u8;
        let battleMonTarget: *mut u8 = ({
            let gl_i480 = gBattlerTarget.get();
            crate::gl!(&raw gBattleMons[gl_i480])
        }) as *mut u8;
        let mut i: i32 = 0;
        while i < 36 {
            *battleMonAttacker.at(i) = *battleMonTarget.at(i);
            i += 1;
        }
        for i in 0..MAX_MON_MOVES {
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
                let gl_i481 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i481].moves[i])
            }]
            .pp < 5
            {
                {
                    let gl_v =
                        (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[{
                            let gl_i483 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i483].moves[i])
                        }]
                        .pp;
                    let gl_i482 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i482].pp[i] = gl_v)
                };
            } else {
                {
                    let gl_i484 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i484].pp[i] = 5)
                };
            }
        }
        gActiveBattler.set(gBattlerAttacker.get());
        BtlController_EmitResetActionMoveSelection(B_COMM_TO_CONTROLLER, RESET_MOVE_SELECTION);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattleCommunication[5] = B_MSG_TRANSFORMED);
    }
}
pub(crate) unsafe fn Cmd_setsubstitute() {
    let mut hp: u32 = (({
        let gl_i485 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i485].maxHP)
    }) as i32
        / 4) as u32;
    if ({
        let gl_i486 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i486].maxHP)
    }) as i32
        / 4
        == 0
    {
        hp = 1;
    }
    if ({
        let gl_i487 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i487].hp)
    }) as u32
        <= hp
    {
        gBattleMoveDamage.set(0);
        crate::gl!(gBattleCommunication[5] = B_MSG_SUBSTITUTE_FAILED);
    } else {
        gBattleMoveDamage.set(
            ({
                let gl_i488 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i488].maxHP)
            }) as i32
                / 4,
        );
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        {
            let gl_i489 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i489].status2 |= STATUS2_SUBSTITUTE)
        };
        {
            let gl_i490 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i490].status2 &= 0xffff1fff)
        };
        {
            let gl_i491 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i491].substituteHP = gBattleMoveDamage.get() as u8)
        };
        crate::gl!(gBattleCommunication[5] = B_MSG_SET_SUBSTITUTE);
        gHitMarker.set(gHitMarker.get() | (HITMARKER_IGNORE_SUBSTITUTE));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
fn IsMoveUncopyableByMimic(r#move: u16) -> u8 {
    let mut i: i32 = 0;
    while sMovesForbiddenToCopy[i] != MIMIC_FORBIDDEN_END && sMovesForbiddenToCopy[i] != r#move {
        i += 1;
    }
    (sMovesForbiddenToCopy[i] != MIMIC_FORBIDDEN_END) as u8
}
pub(crate) unsafe fn Cmd_mimicattackcopy() {
    gChosenMove.set(MOVE_UNAVAILABLE);
    if IsMoveUncopyableByMimic({
        let gl_i141 = gBattlerTarget.get();
        crate::gl!(gLastMoves[gl_i141])
    }) != 0
        || ({
            let gl_i492 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i492].status2)
        }) & STATUS2_TRANSFORMED
            != 0
        || {
            let gl_i142 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i142])
        } == MOVE_NONE
        || {
            let gl_i143 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i143])
        } == MOVE_UNAVAILABLE
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        let mut i: i32 = 0;
        while i < MAX_MON_MOVES {
            if ({
                let gl_i493 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i493].moves[i])
            }) == {
                let gl_i144 = gBattlerTarget.get();
                crate::gl!(gLastMoves[gl_i144])
            } {
                break;
            }
            i += 1;
        }
        if i == MAX_MON_MOVES {
            {
                let gl_v = {
                    let gl_i145 = gBattlerTarget.get();
                    crate::gl!(gLastMoves[gl_i145])
                };
                let gl_i494 = gBattlerAttacker.get();
                let gl_i495 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i494].moves[gl_i495] = gl_v)
            };
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[{
                let gl_i146 = gBattlerTarget.get();
                crate::gl!(gLastMoves[gl_i146])
            }]
            .pp < 5
            {
                {
                    let gl_v =
                        (*(&raw const crate::data::pokemon::gBattleMoves)
                            .cast::<CArray<BattleMove, 0>>())[{
                            let gl_i147 = gBattlerTarget.get();
                            crate::gl!(gLastMoves[gl_i147])
                        }]
                        .pp;
                    let gl_i496 = gBattlerAttacker.get();
                    let gl_i497 = gCurrMovePos.get();
                    crate::gl!(gBattleMons[gl_i496].pp[gl_i497] = gl_v)
                };
            } else {
                {
                    let gl_i498 = gBattlerAttacker.get();
                    let gl_i499 = gCurrMovePos.get();
                    crate::gl!(gBattleMons[gl_i498].pp[gl_i499] = 5)
                };
            }
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 2);
            crate::gl!(
                gBattleTextBuff1[2] = {
                    let gl_i148 = gBattlerTarget.get();
                    crate::gl!(gLastMoves[gl_i148])
                } as u8
            );
            crate::gl!(
                gBattleTextBuff1[3] = (({
                    let gl_i149 = gBattlerTarget.get();
                    crate::gl!(gLastMoves[gl_i149])
                } as i32
                    & 0xFF00)
                    >> 8) as u8
            );
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_mimickedMoves(
                (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].mimickedMoves()
                    | gBitTable[gCurrMovePos.get()] as u8,
            );
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        } else {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
    }
}
pub(crate) unsafe fn Cmd_metronome() {
    loop {
        gCurrentMove.set((Random() & 0x1FF) + 1);
        if gCurrentMove.get() >= MOVES_COUNT {
            continue;
        }
        for i in 0..MAX_MON_MOVES {}
        let mut i: i32 = -1;
        loop {
            i += 1;
            if sMovesForbiddenToCopy[i] == gCurrentMove.get() {
                break;
            }
            if sMovesForbiddenToCopy[i] == METRONOME_FORBIDDEN_END {
                break;
            }
        }
        if sMovesForbiddenToCopy[i] == METRONOME_FORBIDDEN_END {
            gHitMarker.set(gHitMarker.get() & (0xfffffbff));
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::gBattleScriptsForMoveEffects
                    .cast::<CArray<*mut u8, 0>>())
                    [(*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                    .effect]
            );
            gBattlerTarget.set(GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE));
            return;
        }
    }
}
pub(crate) unsafe fn Cmd_dmgtolevel() {
    gBattleMoveDamage.set(
        ({
            let gl_i500 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i500].level)
        }) as i32,
    );
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_psywavedamageeffect() {
    let mut randDamage: i32 = 0;
    while ({
        randDamage = Random() as i32 % 16;
        randDamage
    }) > 10
    {}
    randDamage *= 10;
    gBattleMoveDamage.set(
        ({
            let gl_i501 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i501].level)
        }) as i32
            * (randDamage + 50)
            / 100,
    );
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_counterdamagecalculator() {
    let sideAttacker: u8 = GetBattlerSide(gBattlerAttacker.get());
    let sideTarget: u8 = GetBattlerSide({
        let gl_i150 = gBattlerAttacker.get();
        crate::gl!(gProtectStructs[gl_i150].physicalBattlerId)
    });
    if {
        let gl_i151 = gBattlerAttacker.get();
        crate::gl!(gProtectStructs[gl_i151].physicalDmg)
    } != 0
        && sideAttacker != sideTarget
        && ({
            let gl_i502 = {
                let gl_i152 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i152].physicalBattlerId)
            };
            crate::gl!(gBattleMons[gl_i502].hp)
        }) != 0
    {
        gBattleMoveDamage.set(
            {
                let gl_i153 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i153].physicalDmg)
            } as i32
                * 2,
        );
        if crate::gl!(gSideTimers[sideTarget].followmeTimer) != 0
            && ({
                let gl_i503 = crate::gl!(gSideTimers[sideTarget].followmeTarget);
                crate::gl!(gBattleMons[gl_i503].hp)
            }) != 0
        {
            gBattlerTarget.set(crate::gl!(gSideTimers[sideTarget].followmeTarget));
        } else {
            gBattlerTarget.set({
                let gl_i154 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i154].physicalBattlerId)
            });
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()].set_ppNotAffectedByPressure(1);
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_mirrorcoatdamagecalculator() {
    let sideAttacker: u8 = GetBattlerSide(gBattlerAttacker.get());
    let sideTarget: u8 = GetBattlerSide({
        let gl_i155 = gBattlerAttacker.get();
        crate::gl!(gProtectStructs[gl_i155].specialBattlerId)
    });
    if {
        let gl_i156 = gBattlerAttacker.get();
        crate::gl!(gProtectStructs[gl_i156].specialDmg)
    } != 0
        && sideAttacker != sideTarget
        && ({
            let gl_i504 = {
                let gl_i157 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i157].specialBattlerId)
            };
            crate::gl!(gBattleMons[gl_i504].hp)
        }) != 0
    {
        gBattleMoveDamage.set(
            {
                let gl_i158 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i158].specialDmg)
            } as i32
                * 2,
        );
        if crate::gl!(gSideTimers[sideTarget].followmeTimer) != 0
            && ({
                let gl_i505 = crate::gl!(gSideTimers[sideTarget].followmeTarget);
                crate::gl!(gBattleMons[gl_i505].hp)
            }) != 0
        {
            gBattlerTarget.set(crate::gl!(gSideTimers[sideTarget].followmeTarget));
        } else {
            gBattlerTarget.set({
                let gl_i159 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i159].specialBattlerId)
            });
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()].set_ppNotAffectedByPressure(1);
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_disablelastusedattack() {
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        if ({
            let gl_i506 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i506].moves[i])
        }) == {
            let gl_i160 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i160])
        } {
            break;
        }
        i += 1;
    }
    if ({
        let gl_i507 = gBattlerTarget.get();
        crate::gl!(gDisableStructs[gl_i507].disabledMove)
    }) == MOVE_NONE
        && i != MAX_MON_MOVES
        && ({
            let gl_i508 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i508].pp[i])
        }) != 0
    {
        crate::gl!(gBattleTextBuff1[0] = 0xFD);
        crate::gl!(gBattleTextBuff1[1] = 2);
        crate::gl!(
            gBattleTextBuff1[2] = ({
                let gl_i509 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i509].moves[i])
            }) as u8
        );
        crate::gl!(
            gBattleTextBuff1[3] = ((({
                let gl_i510 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i510].moves[i])
            }) as i32
                & 0xFF00)
                >> 8) as u8
        );
        crate::gl!(gBattleTextBuff1[4] = 0xFF);
        {
            let gl_i511 = gBattlerTarget.get();
            crate::gl!(
                gDisableStructs[gl_i511].disabledMove = ({
                    let gl_i512 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i512].moves[i])
                })
            )
        };
        (*gDisableStructs.as_ptr())[gBattlerTarget.get()]
            .set_disableTimer((Random() as u8 & 3) + 2);
        (*gDisableStructs.as_ptr())[gBattlerTarget.get()].set_disableTimerStartValue(
            (*gDisableStructs.as_ptr())[gBattlerTarget.get()].disableTimer(),
        );
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_trysetencore() {
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        if ({
            let gl_i513 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i513].moves[i])
        }) == {
            let gl_i161 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i161])
        } {
            break;
        }
        i += 1;
    }
    if {
        let gl_i162 = gBattlerTarget.get();
        crate::gl!(gLastMoves[gl_i162])
    } == MOVE_STRUGGLE
        || {
            let gl_i163 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i163])
        } == MOVE_ENCORE
        || {
            let gl_i164 = gBattlerTarget.get();
            crate::gl!(gLastMoves[gl_i164])
        } == MOVE_MIRROR_MOVE
    {
        i = MAX_MON_MOVES;
    }
    if ({
        let gl_i514 = gBattlerTarget.get();
        crate::gl!(gDisableStructs[gl_i514].encoredMove)
    }) == MOVE_NONE
        && i != MAX_MON_MOVES
        && ({
            let gl_i515 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i515].pp[i])
        }) != 0
    {
        {
            let gl_i516 = gBattlerTarget.get();
            crate::gl!(
                gDisableStructs[gl_i516].encoredMove = ({
                    let gl_i517 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i517].moves[i])
                })
            )
        };
        {
            let gl_i518 = gBattlerTarget.get();
            crate::gl!(gDisableStructs[gl_i518].encoredMovePos = i as u8)
        };
        (*gDisableStructs.as_ptr())[gBattlerTarget.get()].set_encoreTimer((Random() as u8 & 3) + 3);
        (*gDisableStructs.as_ptr())[gBattlerTarget.get()].set_encoreTimerStartValue(
            (*gDisableStructs.as_ptr())[gBattlerTarget.get()].encoreTimer(),
        );
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_painsplitdmgcalc() {
    if ({
        let gl_i519 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i519].status2)
    }) & STATUS2_SUBSTITUTE
        == 0
    {
        let hpDiff: i32 = (({
            let gl_i520 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i520].hp)
        }) as i32
            + ({
                let gl_i521 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i521].hp)
            }) as i32)
            / 2;
        let painSplitHp: i32 = {
            gBattleMoveDamage.set(
                ({
                    let gl_i522 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i522].hp)
                }) as i32
                    - hpDiff,
            );
            gBattleMoveDamage.get()
        };
        let storeLoc: *mut u8 =
            crate::gl!(&raw gBattleScripting.painSplitHp) as *mut c_void as *mut u8;
        *storeLoc = painSplitHp as u8;
        *storeLoc.at(1) = ((painSplitHp & 0x0000FF00) >> 8) as u8;
        *storeLoc.at(2) = ((painSplitHp & 0x00FF0000) >> 16) as u8;
        *storeLoc.at(3) = ((painSplitHp as u32 & 0xFF000000) >> 24) as u8;
        gBattleMoveDamage.set(
            ({
                let gl_i523 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i523].hp)
            }) as i32
                - hpDiff,
        );
        {
            let gl_v = IGNORE_SHELL_BELL;
            let gl_i165 = gBattlerTarget.get();
            crate::gl!(gSpecialStatuses[gl_i165].shellBellDmg = gl_v)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_settypetorandomresistance() {
    if {
        let gl_i166 = gBattlerAttacker.get();
        crate::gl!(gLastLandedMoves[gl_i166])
    } == MOVE_NONE
        || {
            let gl_i167 = gBattlerAttacker.get();
            crate::gl!(gLastLandedMoves[gl_i167])
        } == MOVE_UNAVAILABLE
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else if IsTwoTurnsMove({
        let gl_i168 = gBattlerAttacker.get();
        crate::gl!(gLastLandedMoves[gl_i168])
    }) != 0
        && ({
            let gl_i524 = {
                let gl_i169 = gBattlerAttacker.get();
                crate::gl!(gLastHitBy[gl_i169])
            };
            crate::gl!(gBattleMons[gl_i524].status2)
        }) & STATUS2_MULTIPLETURNS
            != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        let mut i: i32 = 0;
        for rands in 0..1000i32 {
            while ({
                i = Random() as i32 % 128;
                i
            }) > 112
            {}
            i *= 3;
            if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[i] as u16
                == {
                    let gl_i170 = gBattlerAttacker.get();
                    crate::gl!(gLastHitByType[gl_i170])
                }
                && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 2]
                    <= TYPE_MUL_NOT_EFFECTIVE
                && !(({
                    let gl_i525 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i525].types[0])
                }) == (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                    .cast::<CArray<u8, 336>>())[i + 1]
                    || ({
                        let gl_i526 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i526].types[1])
                    }) == (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 1])
            {
                {
                    let gl_v = (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 1];
                    let gl_i527 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i527].types[0] = gl_v)
                };
                {
                    let gl_v = (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[i + 1];
                    let gl_i528 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i528].types[1] = gl_v)
                };
                crate::gl!(gBattleTextBuff1[0] = 0xFD);
                crate::gl!(gBattleTextBuff1[1] = 3);
                crate::gl!(
                    gBattleTextBuff1[2] =
                        (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 1]
                );
                crate::gl!(gBattleTextBuff1[3] = 0xFF);
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
                return;
            }
        }
        let mut j: i32 = 0;
        let mut rands: i32 = 0;
        while rands < 336 {
            match (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                .cast::<CArray<u8, 336>>())[j]
            {
                TYPE_ENDTABLE | TYPE_FORESIGHT => {}
                _ => {
                    if (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                        .cast::<CArray<u8, 336>>())[j] as u16
                        == {
                            let gl_i171 = gBattlerAttacker.get();
                            crate::gl!(gLastHitByType[gl_i171])
                        }
                        && (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[j + 2]
                            <= 5
                        && !(({
                            let gl_i529 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i529].types[0])
                        }) == (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                            .cast::<CArray<u8, 336>>())[i + 1]
                            || ({
                                let gl_i530 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i530].types[1])
                            }) == (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                                .cast::<CArray<u8, 336>>())[i + 1])
                    {
                        {
                            let gl_v = (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                                .cast::<CArray<u8, 336>>())[rands + 1];
                            let gl_i531 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i531].types[0] = gl_v)
                        };
                        {
                            let gl_v = (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                                .cast::<CArray<u8, 336>>())[rands + 1];
                            let gl_i532 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i532].types[1] = gl_v)
                        };
                        crate::gl!(gBattleTextBuff1[0] = 0xFD);
                        crate::gl!(gBattleTextBuff1[1] = 3);
                        crate::gl!(
                            gBattleTextBuff1[2] =
                                (*(&raw const crate::data::battle_main::gTypeEffectiveness)
                                    .cast::<CArray<u8, 336>>())[rands + 1]
                        );
                        crate::gl!(gBattleTextBuff1[3] = 0xFF);
                        crate::gl!(
                            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5)
                        );
                        return;
                    }
                }
            }
            j += 3;
            rands += 3;
        }
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_setalwayshitflag() {
    {
        let gl_v = 0xffffffe7;
        let gl_i172 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i172] &= gl_v)
    };
    {
        let gl_v = 16;
        let gl_i173 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i173] |= gl_v)
    };
    {
        let gl_i533 = gBattlerTarget.get();
        crate::gl!(gDisableStructs[gl_i533].battlerWithSureHit = gBattlerAttacker.get())
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_copymovepermanently() {
    gChosenMove.set(MOVE_UNAVAILABLE);
    if ({
        let gl_i534 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i534].status2)
    }) & STATUS2_TRANSFORMED
        == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gLastPrintedMoves;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gBattlerTarget.get()]
            != MOVE_STRUGGLE
        && (*(crate::gl!(&raw @at crate::battle_main::gLastPrintedMoves;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gBattlerTarget.get()]
            != MOVE_NONE
        && (*(crate::gl!(&raw @at crate::battle_main::gLastPrintedMoves;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gBattlerTarget.get()]
            != MOVE_UNAVAILABLE
        && (*(crate::gl!(&raw @at crate::battle_main::gLastPrintedMoves;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gBattlerTarget.get()]
            != MOVE_SKETCH
    {
        let mut i: i32 = 0;
        'l2: while i < MAX_MON_MOVES {
            'l1: {
                if ({
                    let gl_i535 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i535].moves[i])
                }) == MOVE_SKETCH
                {
                    break 'l1;
                }
                if ({
                    let gl_i536 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i536].moves[i])
                }) == {
                    let gl_i174 = gBattlerTarget.get();
                    crate::gl!(gLastPrintedMoves[gl_i174])
                } {
                    break 'l2;
                }
            }
            i += 1;
        }
        if i != MAX_MON_MOVES {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            let mut movePPData: MovePPInfo = zeroed();
            {
                let gl_v = {
                    let gl_i175 = gBattlerTarget.get();
                    crate::gl!(gLastPrintedMoves[gl_i175])
                };
                let gl_i537 = gBattlerAttacker.get();
                let gl_i538 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i537].moves[gl_i538] = gl_v)
            };
            {
                let gl_v = (*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[{
                    let gl_i176 = gBattlerTarget.get();
                    crate::gl!(gLastPrintedMoves[gl_i176])
                }]
                .pp;
                let gl_i539 = gBattlerAttacker.get();
                let gl_i540 = gCurrMovePos.get();
                crate::gl!(gBattleMons[gl_i539].pp[gl_i540] = gl_v)
            };
            gActiveBattler.set(gBattlerAttacker.get());
            for i in 0..MAX_MON_MOVES {
                movePPData.moves[i] = {
                    let gl_i541 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i541].moves[i])
                };
                movePPData.pp[i] = {
                    let gl_i542 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i542].pp[i])
                };
            }
            movePPData.ppBonuses = {
                let gl_i543 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i543].ppBonuses)
            };
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_MOVES_PP_BATTLE,
                0,
                16,
                &raw mut movePPData as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler.get());
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 2);
            crate::gl!(
                gBattleTextBuff1[2] = {
                    let gl_i177 = gBattlerTarget.get();
                    crate::gl!(gLastPrintedMoves[gl_i177])
                } as u8
            );
            crate::gl!(
                gBattleTextBuff1[3] = (({
                    let gl_i178 = gBattlerTarget.get();
                    crate::gl!(gLastPrintedMoves[gl_i178])
                } as i32
                    & 0xFF00)
                    >> 8) as u8
            );
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        }
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
unsafe fn IsTwoTurnsMove(r#move: u16) -> u8 {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
        .effect
        == EFFECT_SKULL_BASH
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_RAZOR_WIND
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_SKY_ATTACK
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_SOLAR_BEAM
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_SEMI_INVULNERABLE
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_BIDE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn IsInvalidForSleepTalkOrAssist(r#move: u16) -> u8 {
    if r#move == MOVE_NONE
        || r#move == MOVE_SLEEP_TALK
        || r#move == MOVE_ASSIST
        || r#move == MOVE_MIRROR_MOVE
        || r#move == MOVE_METRONOME
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn AttacksThisTurn(battler: u8, r#move: u16) -> u8 {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
        .effect
        == EFFECT_SOLAR_BEAM
        && gBattleWeather.get() as i32 & B_WEATHER_SUN != 0
    {
        return 2;
    }
    if ((*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
        .effect
        == EFFECT_SKULL_BASH
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_RAZOR_WIND
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_SKY_ATTACK
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_SOLAR_BEAM
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_SEMI_INVULNERABLE
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .effect
            == EFFECT_BIDE)
        && gHitMarker.get() & HITMARKER_CHARGING != 0
    {
        return 1;
    }
    2
}
pub(crate) unsafe fn Cmd_trychoosesleeptalkmove() {
    let mut unusableMovesBits: u8 = 0;
    for i in 0..MAX_MON_MOVES {
        if IsInvalidForSleepTalkOrAssist({
            let gl_i544 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i544].moves[i])
        }) != 0
            || ({
                let gl_i545 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i545].moves[i])
            }) == MOVE_FOCUS_PUNCH
            || ({
                let gl_i546 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i546].moves[i])
            }) == MOVE_UPROAR
            || IsTwoTurnsMove({
                let gl_i547 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i547].moves[i])
            }) != 0
        {
            unusableMovesBits |= gBitTable[i] as u8;
        }
    }
    unusableMovesBits = CheckMoveLimitations(gBattlerAttacker.get(), unusableMovesBits, 253);
    if unusableMovesBits == ALL_MOVES_MASK {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        let mut movePosition: u32 = 0;
        loop {
            movePosition = (if 0 != 0 {
                Random() as i32 % 4
            } else {
                Random() as i32 & 3
            }) as u32;
            if gBitTable[movePosition] & unusableMovesBits as u32 == 0 {
                break;
            }
        }
        gCalledMove.set({
            let gl_i548 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i548].moves[movePosition])
        });
        gCurrMovePos.set(movePosition as u8);
        gHitMarker.set(gHitMarker.get() & (0xfffffbff));
        gBattlerTarget.set(GetMoveTarget(gCalledMove.get(), NO_TARGET_OVERRIDE));
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_setdestinybond() {
    {
        let gl_i549 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i549].status2 |= STATUS2_DESTINY_BOND)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
unsafe fn TrySetDestinyBondToHappen() {
    let sideAttacker: u8 = GetBattlerSide(gBattlerAttacker.get());
    let sideTarget: u8 = GetBattlerSide(gBattlerTarget.get());
    if ({
        let gl_i550 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i550].status2)
    }) & STATUS2_DESTINY_BOND
        != 0
        && sideAttacker != sideTarget
        && gHitMarker.get() & HITMARKER_GRUDGE == 0
    {
        gHitMarker.set(gHitMarker.get() | (HITMARKER_DESTINYBOND));
    }
}
pub(crate) unsafe fn Cmd_trysetdestinybondtohappen() {
    TrySetDestinyBondToHappen();
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_remaininghptopower() {
    let hpFraction: i32 = GetScaledHPFraction(
        ({
            let gl_i551 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i551].hp)
        }) as i16,
        ({
            let gl_i552 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i552].maxHP)
        }) as i16,
        48,
    ) as i32;
    let mut i: i32 = 0;
    while i < 12 {
        if hpFraction <= sFlailHpScaleToPowerTable[i] as i32 {
            break;
        }
        i += 2;
    }
    gDynamicBasePower.set(sFlailHpScaleToPowerTable[i + 1] as u16);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_tryspiteppreduce() {
    if {
        let gl_i179 = gBattlerTarget.get();
        crate::gl!(gLastMoves[gl_i179])
    } != MOVE_NONE
        && (*(crate::gl!(&raw @at crate::battle_main::gLastMoves;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gBattlerTarget.get()]
            != MOVE_UNAVAILABLE
    {
        let mut i: i32 = 0;
        while i < MAX_MON_MOVES {
            if {
                let gl_i180 = gBattlerTarget.get();
                crate::gl!(gLastMoves[gl_i180])
            } == ({
                let gl_i553 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i553].moves[i])
            }) {
                break;
            }
            i += 1;
        }
        if i != MAX_MON_MOVES
            && ({
                let gl_i554 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i554].pp[i])
            }) > 1
        {
            let mut ppToDeduct: i32 = (Random() as i32 & 3) + 2;
            if (({
                let gl_i555 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i555].pp[i])
            }) as i32)
                < ppToDeduct
            {
                ppToDeduct = ({
                    let gl_i556 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i556].pp[i])
                }) as i32;
            }
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 2);
            crate::gl!(
                gBattleTextBuff1[2] = {
                    let gl_i181 = gBattlerTarget.get();
                    crate::gl!(gLastMoves[gl_i181])
                } as u8
            );
            crate::gl!(
                gBattleTextBuff1[3] = (({
                    let gl_i182 = gBattlerTarget.get();
                    crate::gl!(gLastMoves[gl_i182])
                } as i32
                    & 0xFF00)
                    >> 8) as u8
            );
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            ConvertIntToDecimalStringN(
                (*gBattleTextBuff2.as_ptr()).as_mut_ptr(),
                ppToDeduct,
                STR_CONV_MODE_LEFT_ALIGN,
                1,
            );
            crate::gl!(gBattleTextBuff2[0] = 0xFD);
            crate::gl!(gBattleTextBuff2[1] = 1);
            crate::gl!(gBattleTextBuff2[2] = 1);
            crate::gl!(gBattleTextBuff2[3] = 1);
            crate::gl!(gBattleTextBuff2[4] = ppToDeduct as u8);
            crate::gl!(gBattleTextBuff2[5] = 0xFF);
            {
                let gl_i557 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i557].pp[i] -= ppToDeduct as u8)
            };
            gActiveBattler.set(gBattlerTarget.get());
            if (*gDisableStructs.as_ptr())[gActiveBattler.get()].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                == 0
                && ({
                    let gl_i558 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i558].status2)
                }) & STATUS2_TRANSFORMED
                    == 0
            {
                BtlController_EmitSetMonData(
                    B_COMM_TO_CONTROLLER,
                    REQUEST_PPMOVE1_BATTLE + i as u8,
                    0,
                    1,
                    ({
                        let gl_i559 = gActiveBattler.get();
                        crate::gl!(&raw gBattleMons[gl_i559].pp[i])
                    }) as *mut c_void,
                );
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
            if ({
                let gl_i560 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i560].pp[i])
            }) == 0
            {
                CancelMultiTurnMoves(gBattlerTarget.get());
            }
        } else {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_healpartystatus() {
    let mut zero: u32 = 0;
    let mut toHeal: u8 = 0;
    if gCurrentMove.get() == MOVE_HEAL_BELL {
        let mut party: *mut Pokemon = null_mut();
        crate::gl!(gBattleCommunication[5] = B_MSG_BELL);
        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
            party = (*gPlayerParty.as_ptr()).as_mut_ptr();
        } else {
            party = (*gEnemyParty.as_ptr()).as_mut_ptr();
        }
        if ({
            let gl_i561 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i561].ability)
        }) != ABILITY_SOUNDPROOF
        {
            {
                let gl_i562 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i562].status1 = 0)
            };
            {
                let gl_i563 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i563].status2 &= 0xf7ffffff)
            };
        } else {
            RecordAbilityBattle(gBattlerAttacker.get(), {
                let gl_i564 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i564].ability)
            });
            crate::gl!(gBattleCommunication[5] |= B_MSG_BELL_SOUNDPROOF_ATTACKER);
        }
        gActiveBattler.set({
            crate::gl!(
                gBattleScripting.battler =
                    GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker.get()) ^ 2)
            );
            crate::gl!(gBattleScripting.battler)
        });
        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
            && gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gActiveBattler.get()]
                == 0
        {
            if ({
                let gl_i565 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i565].ability)
            }) != ABILITY_SOUNDPROOF
            {
                {
                    let gl_i566 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i566].status1 = 0)
                };
                {
                    let gl_i567 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i567].status2 &= 0xf7ffffff)
                };
            } else {
                RecordAbilityBattle(gActiveBattler.get(), {
                    let gl_i568 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i568].ability)
                });
                crate::gl!(gBattleCommunication[5] |= B_MSG_BELL_SOUNDPROOF_PARTNER);
            }
        }
        for i in 0..PARTY_SIZE {
            let species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) as u16;
            let abilityNum: u8 = GetMonData2(party.at(i), MON_DATA_ABILITY_NUM) as u8;
            if species != SPECIES_NONE && species != SPECIES_EGG as u16 {
                let mut ability: u8 = 0;
                if ({
                    let gl_i569 = gBattlerAttacker.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i569])
                }) as i32
                    == i
                {
                    ability = {
                        let gl_i570 = gBattlerAttacker.get();
                        crate::gl!(gBattleMons[gl_i570].ability)
                    };
                } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
                    && (*(crate::gl!(&raw @at crate::battle_main::gBattlerPartyIndexes;)
                        .cast_const())
                    .cast::<CArray<u16, 4>>()
                    .cast_mut())[gActiveBattler.get()] as i32
                        == i
                    && gAbsentBattlerFlags.get() as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gActiveBattler.get()]
                        == 0
                {
                    ability = {
                        let gl_i571 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i571].ability)
                    };
                } else {
                    ability = GetAbilityBySpecies(species, abilityNum);
                }
                if ability != ABILITY_SOUNDPROOF {
                    toHeal |= shl_i32(1, i as u32) as u8;
                }
            }
        }
    } else {
        crate::gl!(gBattleCommunication[5] = B_MSG_SOOTHING_AROMA);
        toHeal = 63;
        {
            let gl_i572 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i572].status1 = 0)
        };
        {
            let gl_i573 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i573].status2 &= 0xf7ffffff)
        };
        gActiveBattler.set(GetBattlerAtPosition(
            GetBattlerPosition(gBattlerAttacker.get()) ^ 2,
        ));
        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
            && gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gActiveBattler.get()]
                == 0
        {
            {
                let gl_i574 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i574].status1 = 0)
            };
            {
                let gl_i575 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i575].status2 &= 0xf7ffffff)
            };
        }
    }
    if toHeal != 0 {
        gActiveBattler.set(gBattlerAttacker.get());
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            toHeal,
            4,
            &raw mut zero as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_cursetarget() {
    if ({
        let gl_i576 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i576].status2)
    }) & STATUS2_CURSED
        != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        {
            let gl_i577 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i577].status2 |= STATUS2_CURSED)
        };
        gBattleMoveDamage.set(
            ({
                let gl_i578 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i578].maxHP)
            }) as i32
                / 2,
        );
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_trysetspikes() {
    let targetSide: u8 = GetBattlerSide(gBattlerAttacker.get()) ^ 1;
    if crate::gl!(gSideTimers[targetSide].spikesAmount) == 3 {
        (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()].set_ppNotAffectedByPressure(1);
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gSideStatuses[targetSide] |= SIDE_STATUS_SPIKES as u16);
        crate::gl!(gSideTimers[targetSide].spikesAmount += 1);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_setforesight() {
    {
        let gl_i579 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i579].status2 |= STATUS2_FORESIGHT)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_trysetperishsong() {
    let mut notAffectedCount: i32 = 0;
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if crate::gl!(gStatuses3[i]) & STATUS3_PERISH_SONG != 0
            || crate::gl!(gBattleMons[i].ability) == ABILITY_SOUNDPROOF
        {
            notAffectedCount += 1;
        } else {
            crate::gl!(gStatuses3[i] |= STATUS3_PERISH_SONG);
            (*gDisableStructs.as_ptr())[i].set_perishSongTimer(3);
            (*gDisableStructs.as_ptr())[i].set_perishSongTimerStartValue(3);
        }
        i += 1;
    }
    PressurePPLoseOnUsingPerishSong(gBattlerAttacker.get());
    if notAffectedCount == gBattlersCount.get() as i32 {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_rolloutdamagecalculation() {
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        CancelMultiTurnMoves(gBattlerAttacker.get());
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveMissedPause
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else {
        if ({
            let gl_i580 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i580].status2)
        }) & STATUS2_MULTIPLETURNS
            == 0
        {
            (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_rolloutTimer(5);
            (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_rolloutTimerStartValue(5);
            {
                let gl_i581 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i581].status2 |= STATUS2_MULTIPLETURNS)
            };
            {
                let gl_v = gCurrentMove.get();
                let gl_i183 = gBattlerAttacker.get();
                crate::gl!(gLockedMoves[gl_i183] = gl_v)
            };
        }
        if ({
            (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_rolloutTimer(
                (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].rolloutTimer() - 1,
            );
            (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].rolloutTimer()
        }) == 0
        {
            {
                let gl_i582 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i582].status2 &= 0xffffefff)
            };
        }
        gDynamicBasePower.set(
            (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .power as u16,
        );
        let mut i: i32 = 1;
        while i < 5 - (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].rolloutTimer() as i32 {
            gDynamicBasePower.set(gDynamicBasePower.get() * (2));
            i += 1;
        }
        if ({
            let gl_i583 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i583].status2)
        }) & STATUS2_DEFENSE_CURL
            != 0
        {
            gDynamicBasePower.set(gDynamicBasePower.get() * (2));
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_jumpifconfusedandstatmaxed() {
    if ({
        let gl_i584 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i584].status2)
    }) & STATUS2_CONFUSION
        != 0
        && ({
            let gl_i585 = gBattlerTarget.get();
            let gl_i586 = *(*gBattlescriptCurrInstr.as_ptr()).at(1);
            crate::gl!(gBattleMons[gl_i585].statStages[gl_i586])
        }) == MAX_STAT_STAGE
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_furycuttercalc() {
    if gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        {
            let gl_i587 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i587].furyCutterCounter = 0)
        };
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_MoveMissedPause
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else {
        if ({
            let gl_i588 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i588].furyCutterCounter)
        }) != 5
        {
            {
                let gl_i589 = gBattlerAttacker.get();
                crate::gl!(gDisableStructs[gl_i589].furyCutterCounter += 1)
            };
        }
        gDynamicBasePower.set(
            (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .power as u16,
        );
        for i in 1..(({
            let gl_i590 = gBattlerAttacker.get();
            crate::gl!(gDisableStructs[gl_i590].furyCutterCounter)
        }) as i32)
        {
            gDynamicBasePower.set(gDynamicBasePower.get() * (2));
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_friendshiptodamagecalculation() {
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove.get()]
    .effect
        == EFFECT_RETURN
    {
        gDynamicBasePower.set(
            (10 * ({
                let gl_i591 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i591].friendship)
            }) as i32
                / 25) as u16,
        );
    } else {
        gDynamicBasePower.set(
            (10 * (MAX_FRIENDSHIP as i32
                - ({
                    let gl_i592 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i592].friendship)
                }) as i32)
                / 25) as u16,
        );
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_presentdamagecalculation() {
    let rand: i32 = Random() as i32 & 0xFF;
    if rand < 102 {
        gDynamicBasePower.set(40);
    } else if rand < 178 {
        gDynamicBasePower.set(80);
    } else if rand < 204 {
        gDynamicBasePower.set(120);
    } else {
        gBattleMoveDamage.set(
            ({
                let gl_i593 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i593].maxHP)
            }) as i32
                / 4,
        );
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        gBattleMoveDamage.set(-gBattleMoveDamage.get());
    }
    if rand < 204 {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_HitFromCritCalc
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else if ({
        let gl_i594 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i594].maxHP)
    }) == ({
        let gl_i595 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i595].hp)
    }) {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_AlreadyAtFullHp
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else {
        gMoveResultFlags.set(gMoveResultFlags.get() & (247));
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PresentHealTarget
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    }
}
pub(crate) unsafe fn Cmd_setsafeguard() {
    if {
        let gl_i184 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
        crate::gl!(gSideStatuses[gl_i184])
    } as i32
        & SIDE_STATUS_SAFEGUARD
        != 0
    {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_SIDE_STATUS_FAILED);
    } else {
        {
            let gl_v = SIDE_STATUS_SAFEGUARD as u16;
            let gl_i185 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideStatuses[gl_i185] |= gl_v)
        };
        {
            let gl_v = 5;
            let gl_i186 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i186].safeguardTimer = gl_v)
        };
        {
            let gl_v = gBattlerAttacker.get();
            let gl_i187 = GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1;
            crate::gl!(gSideTimers[gl_i187].safeguardBattlerId = gl_v)
        };
        crate::gl!(gBattleCommunication[5] = 5);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_magnitudedamagecalculation() {
    let mut magnitude: i32 = Random() as i32 % 100;
    if magnitude < 5 {
        gDynamicBasePower.set(10);
        magnitude = 4;
    } else if magnitude < 15 {
        gDynamicBasePower.set(30);
        magnitude = 5;
    } else if magnitude < 35 {
        gDynamicBasePower.set(50);
        magnitude = 6;
    } else if magnitude < 65 {
        gDynamicBasePower.set(70);
        magnitude = 7;
    } else if magnitude < 85 {
        gDynamicBasePower.set(90);
        magnitude = 8;
    } else if magnitude < 95 {
        gDynamicBasePower.set(110);
        magnitude = 9;
    } else {
        gDynamicBasePower.set(150);
        magnitude = 10;
    }
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 1);
    crate::gl!(gBattleTextBuff1[2] = 1);
    crate::gl!(gBattleTextBuff1[3] = 2);
    crate::gl!(gBattleTextBuff1[4] = magnitude as u8);
    crate::gl!(gBattleTextBuff1[5] = 0xFF);
    gBattlerTarget.set(0);
    'l2: while gBattlerTarget.get() < gBattlersCount.get() {
        'l1: {
            if gBattlerTarget.get() == gBattlerAttacker.get() {
                break 'l1;
            }
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gBattlerTarget.get()]
                == 0
            {
                break 'l2;
            }
        }
        gBattlerTarget.set(gBattlerTarget.get() + (1));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_jumpifnopursuitswitchdmg() {
    if gMultiHitCounter.get() == 1 {
        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
            gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT));
        } else {
            gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
        }
    } else {
        if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
            gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT));
        } else {
            gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT));
        }
    }
    if {
        let gl_i188 = gBattlerTarget.get();
        crate::gl!(gChosenActionByBattler[gl_i188])
    } == B_ACTION_USE_MOVE
        && gBattlerAttacker.get()
            == *(*crate::gl!(gBattleStruct))
                .moveTarget
                .as_mut_ptr()
                .at(gBattlerTarget.get())
        && ({
            let gl_i596 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i596].status1)
        }) & 39
            == 0
        && ({
            let gl_i597 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i597].hp)
        }) != 0
        && (*gDisableStructs.as_ptr())[gBattlerTarget.get()].truantCounter() == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gChosenMoveByBattler;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gBattlerTarget.get()]
            == MOVE_PURSUIT
    {
        for i in 0..(gBattlersCount.get() as i32) {
            if crate::gl!(gBattlerByTurnOrder[i]) == gBattlerTarget.get() {
                crate::gl!(gActionsByTurnOrder[i] = B_ACTION_TRY_FINISH);
            }
        }
        gCurrentMove.set(MOVE_PURSUIT);
        gCurrMovePos.set({
            gChosenMovePos.set(
                *(*crate::gl!(gBattleStruct))
                    .chosenMovePositions
                    .as_mut_ptr()
                    .at(gBattlerTarget.get()),
            );
            gChosenMovePos.get()
        });
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        crate::gl!(gBattleScripting.animTurn = 1);
        gHitMarker.set(gHitMarker.get() & (0xfffffbff));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_setsunny() {
    if gBattleWeather.get() as i32 & B_WEATHER_SUN != 0 {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_WEATHER_FAILED);
    } else {
        gBattleWeather.set(B_WEATHER_SUN_TEMPORARY);
        crate::gl!(gBattleCommunication[5] = B_MSG_STARTED_SUNLIGHT);
        crate::gl!(gWishFutureKnock.weatherDuration = 5);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_maxattackhalvehp() {
    let mut halfHp: u32 = (({
        let gl_i598 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i598].maxHP)
    }) as i32
        / 2) as u32;
    if ({
        let gl_i599 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i599].maxHP)
    }) as i32
        / 2
        == 0
    {
        halfHp = 1;
    }
    if ({
        let gl_i600 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i600].statStages[1])
    }) < MAX_STAT_STAGE
        && ({
            let gl_i601 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i601].hp)
        }) as u32
            > halfHp
    {
        {
            let gl_i602 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i602].statStages[1] = MAX_STAT_STAGE)
        };
        gBattleMoveDamage.set(
            ({
                let gl_i603 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i603].maxHP)
            }) as i32
                / 2,
        );
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_copyfoestats() {
    for i in 0..NUM_BATTLE_STATS {
        {
            let gl_i604 = gBattlerAttacker.get();
            crate::gl!(
                gBattleMons[gl_i604].statStages[i] = ({
                    let gl_i605 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i605].statStages[i])
                })
            )
        };
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
}
pub(crate) unsafe fn Cmd_rapidspinfree() {
    if ({
        let gl_i606 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i606].status2)
    }) & STATUS2_WRAPPED
        != 0
    {
        crate::gl!(gBattleScripting.battler = gBattlerTarget.get());
        {
            let gl_i607 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i607].status2 &= 0xffff1fff)
        };
        gBattlerTarget.set(
            *(*crate::gl!(gBattleStruct))
                .wrappedBy
                .as_mut_ptr()
                .at(gBattlerAttacker.get()),
        );
        crate::gl!(gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN);
        crate::gl!(gBattleTextBuff1[1] = B_BUFF_MOVE);
        crate::gl!(
            gBattleTextBuff1[2] = *(*crate::gl!(gBattleStruct))
                .wrappedMove
                .as_mut_ptr()
                .at(gBattlerAttacker.get() as i32 * 2)
        );
        crate::gl!(
            gBattleTextBuff1[3] = *(*crate::gl!(gBattleStruct))
                .wrappedMove
                .as_mut_ptr()
                .at(gBattlerAttacker.get() as i32 * 2)
                .at(1)
        );
        crate::gl!(gBattleTextBuff1[4] = B_BUFF_EOS);
        BattleScriptPushCursor();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_WrapFree
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else if {
        let gl_i189 = gBattlerAttacker.get();
        crate::gl!(gStatuses3[gl_i189])
    } & STATUS3_LEECHSEED
        != 0
    {
        {
            let gl_v = 0xfffffffb;
            let gl_i190 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i190] &= gl_v)
        };
        {
            let gl_v = 0xfffffffc;
            let gl_i191 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i191] &= gl_v)
        };
        BattleScriptPushCursor();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LeechSeedFree
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else if {
        let gl_i192 = GetBattlerSide(gBattlerAttacker.get());
        crate::gl!(gSideStatuses[gl_i192])
    } as i32
        & SIDE_STATUS_SPIKES
        != 0
    {
        {
            let gl_v = 65519;
            let gl_i193 = GetBattlerSide(gBattlerAttacker.get());
            crate::gl!(gSideStatuses[gl_i193] &= gl_v)
        };
        {
            let gl_v = 0;
            let gl_i194 = GetBattlerSide(gBattlerAttacker.get());
            crate::gl!(gSideTimers[gl_i194].spikesAmount = gl_v)
        };
        BattleScriptPushCursor();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SpikesFree
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    }
}
pub(crate) unsafe fn Cmd_setdefensecurlbit() {
    {
        let gl_i608 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i608].status2 |= STATUS2_DEFENSE_CURL)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_recoverbasedonsunlight() {
    gBattlerTarget.set(gBattlerAttacker.get());
    if ({
        let gl_i609 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i609].hp)
    }) != ({
        let gl_i610 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i610].maxHP)
    }) {
        if gBattleWeather.get() == 0
            || !(AbilityBattleEffects(19, 0, 13, 0, 0) == 0
                && AbilityBattleEffects(19, 0, 77, 0, 0) == 0)
        {
            gBattleMoveDamage.set(
                ({
                    let gl_i611 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i611].maxHP)
                }) as i32
                    / 2,
            );
        } else if gBattleWeather.get() as i32 & B_WEATHER_SUN != 0 {
            gBattleMoveDamage.set(
                20 * ({
                    let gl_i612 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i612].maxHP)
                }) as i32
                    / 30,
            );
        } else {
            gBattleMoveDamage.set(
                ({
                    let gl_i613 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i613].maxHP)
                }) as i32
                    / 4,
            );
        }
        if gBattleMoveDamage.get() == 0 {
            gBattleMoveDamage.set(1);
        }
        gBattleMoveDamage.set(-gBattleMoveDamage.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_hiddenpowercalc() {
    let powerBits: u8 = (((*gBattleMons.as_ptr())[gBattlerAttacker.get()].hpIV() & 2) >> 1) as u8
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].attackIV() as u8 & 2)
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].defenseIV() as u8 & 2) << 1
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].speedIV() as u8 & 2) << 2
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].spAttackIV() as u8 & 2) << 3
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].spDefenseIV() as u8 & 2) << 4;
    let typeBits: u8 = ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].hpIV() as u8 & 1)
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].attackIV() as u8 & 1) << 1
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].defenseIV() as u8 & 1) << 2
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].speedIV() as u8 & 1) << 3
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].spAttackIV() as u8 & 1) << 4
        | ((*gBattleMons.as_ptr())[gBattlerAttacker.get()].spDefenseIV() as u8 & 1) << 5;
    gDynamicBasePower.set((40 * powerBits as i32 / 63) as u16 + 30);
    (*crate::gl!(gBattleStruct)).dynamicMoveType = (15 * typeBits as i32 / 63) as u8 + 1;
    if (*crate::gl!(gBattleStruct)).dynamicMoveType >= TYPE_MYSTERY {
        (*crate::gl!(gBattleStruct)).dynamicMoveType += 1;
    }
    (*crate::gl!(gBattleStruct)).dynamicMoveType |= 192;
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_selectfirstvalidtarget() {
    gBattlerTarget.set(0);
    'l2: while gBattlerTarget.get() < gBattlersCount.get() {
        'l1: {
            if gBattlerTarget.get() == gBattlerAttacker.get() {
                break 'l1;
            }
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gBattlerTarget.get()]
                == 0
            {
                break 'l2;
            }
        }
        gBattlerTarget.set(gBattlerTarget.get() + (1));
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_trysetfutureattack() {
    if {
        let gl_i195 = gBattlerTarget.get();
        crate::gl!(gWishFutureKnock.futureSightCounter[gl_i195])
    } != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        {
            let gl_v = SIDE_STATUS_FUTUREATTACK;
            let gl_i196 = GetBattlerPosition(gBattlerTarget.get()) as i32 & 1;
            crate::gl!(gSideStatuses[gl_i196] |= gl_v)
        };
        {
            let gl_v = gCurrentMove.get();
            let gl_i197 = gBattlerTarget.get();
            crate::gl!(gWishFutureKnock.futureSightMove[gl_i197] = gl_v)
        };
        {
            let gl_v = gBattlerAttacker.get();
            let gl_i198 = gBattlerTarget.get();
            crate::gl!(gWishFutureKnock.futureSightAttacker[gl_i198] = gl_v)
        };
        {
            let gl_v = 3;
            let gl_i199 = gBattlerTarget.get();
            crate::gl!(gWishFutureKnock.futureSightCounter[gl_i199] = gl_v)
        };
        {
            let gl_v = CalculateBaseDamage(
                {
                    let gl_i614 = gBattlerAttacker.get();
                    crate::gl!(&raw gBattleMons[gl_i614])
                },
                {
                    let gl_i615 = gBattlerTarget.get();
                    crate::gl!(&raw gBattleMons[gl_i615])
                },
                gCurrentMove.get() as u32,
                {
                    let gl_i201 = GetBattlerPosition(gBattlerTarget.get()) as i32 & 1;
                    crate::gl!(gSideStatuses[gl_i201])
                },
                0,
                0,
                gBattlerAttacker.get(),
                gBattlerTarget.get(),
            );
            let gl_i200 = gBattlerTarget.get();
            crate::gl!(gWishFutureKnock.futureSightDmg[gl_i200] = gl_v)
        };
        if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].helpingHand() != 0 {
            {
                let gl_v = {
                    let gl_i203 = gBattlerTarget.get();
                    crate::gl!(gWishFutureKnock.futureSightDmg[gl_i203])
                } * 15
                    / 10;
                let gl_i202 = gBattlerTarget.get();
                crate::gl!(gWishFutureKnock.futureSightDmg[gl_i202] = gl_v)
            };
        }
        if gCurrentMove.get() == MOVE_DOOM_DESIRE {
            crate::gl!(gBattleCommunication[5] = B_MSG_DOOM_DESIRE);
        } else {
            crate::gl!(gBattleCommunication[5] = B_MSG_FUTURE_SIGHT);
        }
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_trydobeatup() {
    let mut party: *mut Pokemon = null_mut();
    if GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_PLAYER {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    }
    if ({
        let gl_i616 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i616].hp)
    }) == 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        let beforeLoop: u8 = crate::gl!(gBattleCommunication[0]);
        while crate::gl!(gBattleCommunication[0]) < PARTY_SIZE as u8 {
            if GetMonData2(party.at(crate::gl!(gBattleCommunication[0])), MON_DATA_HP) != 0
                && GetMonData2(
                    party.at(crate::gl!(gBattleCommunication[0])),
                    MON_DATA_SPECIES_OR_EGG,
                ) != 0
                && GetMonData2(
                    party.at(crate::gl!(gBattleCommunication[0])),
                    MON_DATA_SPECIES_OR_EGG,
                ) != SPECIES_EGG
                && GetMonData2(
                    party.at(crate::gl!(gBattleCommunication[0])),
                    MON_DATA_STATUS,
                ) == 0
            {
                break;
            }
            crate::gl!(gBattleCommunication[0] += 1);
        }
        if crate::gl!(gBattleCommunication[0]) < PARTY_SIZE as u8 {
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 4);
            crate::gl!(gBattleTextBuff1[2] = gBattlerAttacker.get());
            crate::gl!(gBattleTextBuff1[3] = crate::gl!(gBattleCommunication[0]));
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(9));
            gBattleMoveDamage.set(
                (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                    [GetMonData2(
                        party.at(crate::gl!(gBattleCommunication[0])),
                        MON_DATA_SPECIES,
                    )]
                .baseAttack as i32,
            );
            {
                let rhs = (*(&raw const crate::data::pokemon::gBattleMoves)
                    .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
                .power as i32;
                gBattleMoveDamage.set(gBattleMoveDamage.get() * rhs)
            };
            {
                let rhs = (GetMonData2(
                    party.at(crate::gl!(gBattleCommunication[0])),
                    MON_DATA_LEVEL,
                ) * 2
                    / 5) as i32
                    + 2;
                gBattleMoveDamage.set(gBattleMoveDamage.get() * rhs)
            };
            gBattleMoveDamage.set(div_i32(
                gBattleMoveDamage.get(),
                (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                    [{
                        let gl_i617 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i617].species)
                    }]
                .baseDefense as i32,
            ));
            gBattleMoveDamage.set(gBattleMoveDamage.get() / 50 + 2);
            if (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].helpingHand() != 0 {
                gBattleMoveDamage.set(gBattleMoveDamage.get() * 15 / 10);
            }
            crate::gl!(gBattleCommunication[0] += 1);
        } else if beforeLoop != 0 {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(5) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(5).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
    }
}
pub(crate) unsafe fn Cmd_setsemiinvulnerablebit() {
    match gCurrentMove.get() {
        MOVE_FLY | MOVE_BOUNCE => {
            {
                let gl_v = STATUS3_ON_AIR;
                let gl_i204 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i204] |= gl_v)
            };
        }
        MOVE_DIG => {
            {
                let gl_v = STATUS3_UNDERGROUND;
                let gl_i205 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i205] |= gl_v)
            };
        }
        MOVE_DIVE => {
            {
                let gl_v = STATUS3_UNDERWATER;
                let gl_i206 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i206] |= gl_v)
            };
        }
        _ => {}
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_clearsemiinvulnerablebit() {
    match gCurrentMove.get() {
        MOVE_FLY | MOVE_BOUNCE => {
            {
                let gl_v = 0xffffffbf;
                let gl_i207 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i207] &= gl_v)
            };
        }
        MOVE_DIG => {
            {
                let gl_v = 0xffffff7f;
                let gl_i208 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i208] &= gl_v)
            };
        }
        MOVE_DIVE => {
            {
                let gl_v = 0xfffbffff;
                let gl_i209 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i209] &= gl_v)
            };
        }
        _ => {}
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setminimize() {
    if gHitMarker.get() & HITMARKER_OBEYS != 0 {
        {
            let gl_v = STATUS3_MINIMIZED;
            let gl_i210 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i210] |= gl_v)
        };
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_sethail() {
    if gBattleWeather.get() as i32 & B_WEATHER_HAIL != 0 {
        gMoveResultFlags.set(gMoveResultFlags.get() | (MOVE_RESULT_MISSED));
        crate::gl!(gBattleCommunication[5] = B_MSG_WEATHER_FAILED);
    } else {
        gBattleWeather.set(B_WEATHER_HAIL_TEMPORARY);
        crate::gl!(gBattleCommunication[5] = 5);
        crate::gl!(gWishFutureKnock.weatherDuration = 5);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_trymemento() {
    if ({
        let gl_i618 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i618].statStages[1])
    }) == MIN_STAT_STAGE
        && ({
            let gl_i619 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i619].statStages[4])
        }) == MIN_STAT_STAGE
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
            .cast::<CArray<u8, 8>>()
            .cast_mut())[6]
            != B_MSG_PROTECTED
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        gActiveBattler.set(gBattlerAttacker.get());
        gBattleMoveDamage.set(
            ({
                let gl_i620 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i620].hp)
            }) as i32,
        );
        BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, INSTANT_HP_BAR_DROP as u16);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_setforcedtarget() {
    {
        let gl_v = 1;
        let gl_i211 = GetBattlerSide(gBattlerAttacker.get());
        crate::gl!(gSideTimers[gl_i211].followmeTimer = gl_v)
    };
    {
        let gl_v = gBattlerAttacker.get();
        let gl_i212 = GetBattlerSide(gBattlerAttacker.get());
        crate::gl!(gSideTimers[gl_i212].followmeTarget = gl_v)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setcharge() {
    {
        let gl_v = STATUS3_CHARGED_UP;
        let gl_i213 = gBattlerAttacker.get();
        crate::gl!(gStatuses3[gl_i213] |= gl_v)
    };
    (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_chargeTimer(2);
    (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].set_chargeTimerStartValue(2);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_callenvironmentattack() {
    gHitMarker.set(gHitMarker.get() & (0xfffffbff));
    gCurrentMove.set(sNaturePowerMoves[gBattleEnvironment.get()]);
    gBattlerTarget.set(GetMoveTarget(gCurrentMove.get(), NO_TARGET_OVERRIDE));
    BattleScriptPush(
        (*crate::asmdata::gBattleScriptsForMoveEffects.cast::<CArray<*mut u8, 0>>())
            [(*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [gCurrentMove.get()]
            .effect],
    );
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_cureifburnedparalyzedorpoisoned() {
    if ({
        let gl_i621 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i621].status1)
    }) & 216
        != 0
    {
        {
            let gl_i622 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i622].status1 = 0)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        gActiveBattler.set(gBattlerAttacker.get());
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            0,
            4,
            ({
                let gl_i623 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i623].status1)
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_settorment() {
    if ({
        let gl_i624 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i624].status2)
    }) & 0x80000000
        != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        {
            let gl_i625 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i625].status2 |= 0x80000000)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_jumpifnodamage() {
    if {
        let gl_i214 = gBattlerAttacker.get();
        crate::gl!(gProtectStructs[gl_i214].physicalDmg)
    } != 0
        || {
            let gl_i215 = gBattlerAttacker.get();
            crate::gl!(gProtectStructs[gl_i215].specialDmg)
        } != 0
    {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_settaunt() {
    if (*gDisableStructs.as_ptr())[gBattlerTarget.get()].tauntTimer() == 0 {
        (*gDisableStructs.as_ptr())[gBattlerTarget.get()].set_tauntTimer(2);
        (*gDisableStructs.as_ptr())[gBattlerTarget.get()].set_tauntTimer2(2);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_trysethelpinghand() {
    gBattlerTarget.set(GetBattlerAtPosition(
        GetBattlerPosition(gBattlerAttacker.get()) ^ 2,
    ));
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
        && gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget.get()]
            == 0
        && (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].helpingHand() == 0
        && (*gProtectStructs.as_ptr())[gBattlerTarget.get()].helpingHand() == 0
    {
        (*gProtectStructs.as_ptr())[gBattlerTarget.get()].set_helpingHand(1);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_tryswapitems() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER_HILL != 0
        || GetBattlerSide(gBattlerAttacker.get()) == B_SIDE_OPPONENT
            && gBattleTypeFlags.get() & 0xa3f0902 == 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        let sideAttacker: u8 = GetBattlerSide(gBattlerAttacker.get());
        let sideTarget: u8 = GetBattlerSide(gBattlerTarget.get());
        if gBattleTypeFlags.get() & 0xa3f0902 == 0
            && (crate::gl!(gWishFutureKnock.knockedOffMons[sideAttacker]) as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[{
                    let gl_i626 = gBattlerAttacker.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i626])
                }]
                != 0
                || crate::gl!(gWishFutureKnock.knockedOffMons[sideTarget]) as u32
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[{
                        let gl_i627 = gBattlerTarget.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i627])
                    }]
                    != 0)
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else if ({
            let gl_i628 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i628].item)
        }) == ITEM_NONE
            && ({
                let gl_i629 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i629].item)
            }) == ITEM_NONE
            || ({
                let gl_i630 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i630].item)
            }) == ITEM_ENIGMA_BERRY
            || ({
                let gl_i631 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i631].item)
            }) == ITEM_ENIGMA_BERRY
            || (({
                let gl_i632 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i632].item)
            }) == ITEM_ORANGE_MAIL
                || ({
                    let gl_i633 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i633].item)
                }) == ITEM_HARBOR_MAIL
                || ({
                    let gl_i634 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i634].item)
                }) == ITEM_GLITTER_MAIL
                || ({
                    let gl_i635 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i635].item)
                }) == ITEM_MECH_MAIL
                || ({
                    let gl_i636 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i636].item)
                }) == ITEM_WOOD_MAIL
                || ({
                    let gl_i637 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i637].item)
                }) == ITEM_WAVE_MAIL
                || ({
                    let gl_i638 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i638].item)
                }) == ITEM_BEAD_MAIL
                || ({
                    let gl_i639 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i639].item)
                }) == ITEM_SHADOW_MAIL
                || ({
                    let gl_i640 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i640].item)
                }) == ITEM_TROPIC_MAIL
                || ({
                    let gl_i641 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i641].item)
                }) == ITEM_DREAM_MAIL
                || ({
                    let gl_i642 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i642].item)
                }) == ITEM_FAB_MAIL
                || ({
                    let gl_i643 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i643].item)
                }) == ITEM_RETRO_MAIL)
            || (({
                let gl_i644 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i644].item)
            }) == ITEM_ORANGE_MAIL
                || ({
                    let gl_i645 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i645].item)
                }) == ITEM_HARBOR_MAIL
                || ({
                    let gl_i646 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i646].item)
                }) == ITEM_GLITTER_MAIL
                || ({
                    let gl_i647 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i647].item)
                }) == ITEM_MECH_MAIL
                || ({
                    let gl_i648 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i648].item)
                }) == ITEM_WOOD_MAIL
                || ({
                    let gl_i649 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i649].item)
                }) == ITEM_WAVE_MAIL
                || ({
                    let gl_i650 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i650].item)
                }) == ITEM_BEAD_MAIL
                || ({
                    let gl_i651 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i651].item)
                }) == ITEM_SHADOW_MAIL
                || ({
                    let gl_i652 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i652].item)
                }) == ITEM_TROPIC_MAIL
                || ({
                    let gl_i653 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i653].item)
                }) == ITEM_DREAM_MAIL
                || ({
                    let gl_i654 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i654].item)
                }) == ITEM_FAB_MAIL
                || ({
                    let gl_i655 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i655].item)
                }) == ITEM_RETRO_MAIL)
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        } else if ({
            let gl_i656 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i656].ability)
        }) == ABILITY_STICKY_HOLD
        {
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_StickyHoldActivates
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            gLastUsedAbility.set({
                let gl_i657 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i657].ability)
            });
            RecordAbilityBattle(gBattlerTarget.get(), gLastUsedAbility.get());
        } else {
            let newItemAtk: *mut u16 =
                &raw mut (*crate::gl!(gBattleStruct)).changedItems[gBattlerAttacker.get()];
            let oldItemAtk: u16 = {
                let gl_i658 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i658].item)
            };
            *newItemAtk = {
                let gl_i659 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i659].item)
            };
            {
                let gl_i660 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i660].item = ITEM_NONE)
            };
            {
                let gl_i661 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i661].item = oldItemAtk)
            };
            gActiveBattler.set(gBattlerAttacker.get());
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_HELDITEM_BATTLE,
                0,
                2,
                newItemAtk as *mut c_void,
            );
            MarkBattlerForControllerExec(gBattlerAttacker.get());
            gActiveBattler.set(gBattlerTarget.get());
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_HELDITEM_BATTLE,
                0,
                2,
                ({
                    let gl_i662 = gBattlerTarget.get();
                    crate::gl!(&raw gBattleMons[gl_i662].item)
                }) as *mut c_void,
            );
            MarkBattlerForControllerExec(gBattlerTarget.get());
            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gBattlerTarget.get()] as *mut u8) =
                0;
            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gBattlerTarget.get()] as *mut u8)
                .at(1) = 0;
            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gBattlerAttacker.get()]
                as *mut u8) = 0;
            *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gBattlerAttacker.get()]
                as *mut u8)
                .at(1) = 0;
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 10);
            crate::gl!(gBattleTextBuff1[2] = *newItemAtk as u8);
            crate::gl!(gBattleTextBuff1[3] = ((*newItemAtk as i32 & 0xFF00) >> 8) as u8);
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            crate::gl!(gBattleTextBuff2[0] = 0xFD);
            crate::gl!(gBattleTextBuff2[1] = 10);
            crate::gl!(gBattleTextBuff2[2] = oldItemAtk as u8);
            crate::gl!(gBattleTextBuff2[3] = ((oldItemAtk as i32 & 0xFF00) >> 8) as u8);
            crate::gl!(gBattleTextBuff2[4] = 0xFF);
            if oldItemAtk != ITEM_NONE && *newItemAtk != ITEM_NONE {
                crate::gl!(gBattleCommunication[5] = B_MSG_ITEM_SWAP_BOTH);
            } else if oldItemAtk == ITEM_NONE && *newItemAtk != ITEM_NONE {
                crate::gl!(gBattleCommunication[5] = B_MSG_ITEM_SWAP_TAKEN);
            } else {
                crate::gl!(gBattleCommunication[5] = B_MSG_ITEM_SWAP_GIVEN);
            }
        }
    }
}
pub(crate) unsafe fn Cmd_trycopyability() {
    if ({
        let gl_i663 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i663].ability)
    }) != ABILITY_NONE
        && ({
            let gl_i664 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i664].ability)
        }) != ABILITY_WONDER_GUARD
    {
        {
            let gl_i665 = gBattlerAttacker.get();
            crate::gl!(
                gBattleMons[gl_i665].ability = ({
                    let gl_i666 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i666].ability)
                })
            )
        };
        gLastUsedAbility.set({
            let gl_i667 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i667].ability)
        });
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_trywish() {
    match *(*gBattlescriptCurrInstr.as_ptr()).at(1) {
        0 => {
            if {
                let gl_i216 = gBattlerAttacker.get();
                crate::gl!(gWishFutureKnock.wishCounter[gl_i216])
            } == 0
            {
                {
                    let gl_v = 2;
                    let gl_i217 = gBattlerAttacker.get();
                    crate::gl!(gWishFutureKnock.wishCounter[gl_i217] = gl_v)
                };
                {
                    let gl_v = ({
                        let gl_i668 = gBattlerAttacker.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i668])
                    }) as u8;
                    let gl_i218 = gBattlerAttacker.get();
                    crate::gl!(gWishFutureKnock.wishMonId[gl_i218] = gl_v)
                };
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
            } else {
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        1 => {
            crate::gl!(gBattleTextBuff1[0] = 0xFD);
            crate::gl!(gBattleTextBuff1[1] = 4);
            crate::gl!(gBattleTextBuff1[2] = gBattlerTarget.get());
            crate::gl!(
                gBattleTextBuff1[3] = {
                    let gl_i219 = gBattlerTarget.get();
                    crate::gl!(gWishFutureKnock.wishMonId[gl_i219])
                }
            );
            crate::gl!(gBattleTextBuff1[4] = 0xFF);
            gBattleMoveDamage.set(
                ({
                    let gl_i669 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i669].maxHP)
                }) as i32
                    / 2,
            );
            if gBattleMoveDamage.get() == 0 {
                gBattleMoveDamage.set(1);
            }
            gBattleMoveDamage.set(-gBattleMoveDamage.get());
            if ({
                let gl_i670 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i670].hp)
            }) == ({
                let gl_i671 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i671].maxHP)
            }) {
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            } else {
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_trysetroots() {
    if {
        let gl_i220 = gBattlerAttacker.get();
        crate::gl!(gStatuses3[gl_i220])
    } & STATUS3_ROOTED
        != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        {
            let gl_v = STATUS3_ROOTED;
            let gl_i221 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i221] |= gl_v)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_doubledamagedealtifdamaged() {
    if {
        let gl_i222 = gBattlerAttacker.get();
        crate::gl!(gProtectStructs[gl_i222].physicalDmg)
    } != 0
        && {
            let gl_i223 = gBattlerAttacker.get();
            crate::gl!(gProtectStructs[gl_i223].physicalBattlerId)
        } == gBattlerTarget.get()
        || {
            let gl_i224 = gBattlerAttacker.get();
            crate::gl!(gProtectStructs[gl_i224].specialDmg)
        } != 0
            && {
                let gl_i225 = gBattlerAttacker.get();
                crate::gl!(gProtectStructs[gl_i225].specialBattlerId)
            } == gBattlerTarget.get()
    {
        crate::gl!(gBattleScripting.dmgMultiplier = 2);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_setyawn() {
    if {
        let gl_i226 = gBattlerTarget.get();
        crate::gl!(gStatuses3[gl_i226])
    } & STATUS3_YAWN
        != 0
        || ({
            let gl_i672 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i672].status1)
        }) & STATUS1_ANY
            != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        {
            let gl_v = 4096;
            let gl_i227 = gBattlerTarget.get();
            crate::gl!(gStatuses3[gl_i227] |= gl_v)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_setdamagetohealthdifference() {
    if ({
        let gl_i673 = gBattlerTarget.get();
        crate::gl!(gBattleMons[gl_i673].hp)
    }) <= ({
        let gl_i674 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i674].hp)
    }) {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        gBattleMoveDamage.set(
            ({
                let gl_i675 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i675].hp)
            }) as i32
                - ({
                    let gl_i676 = gBattlerAttacker.get();
                    crate::gl!(gBattleMons[gl_i676].hp)
                }) as i32,
        );
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_scaledamagebyhealthratio() {
    if gDynamicBasePower.get() == 0 {
        let power: u8 = (*(&raw const crate::data::pokemon::gBattleMoves)
            .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
        .power;
        gDynamicBasePower.set(div_i32(
            ({
                let gl_i677 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i677].hp)
            }) as i32
                * power as i32,
            ({
                let gl_i678 = gBattlerAttacker.get();
                crate::gl!(gBattleMons[gl_i678].maxHP)
            }) as i32,
        ) as u16);
        if gDynamicBasePower.get() == 0 {
            gDynamicBasePower.set(1);
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_tryswapabilities() {
    if ({
        let gl_i679 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i679].ability)
    }) == ABILITY_NONE
        && ({
            let gl_i680 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i680].ability)
        }) == ABILITY_NONE
        || ({
            let gl_i681 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i681].ability)
        }) == ABILITY_WONDER_GUARD
        || ({
            let gl_i682 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i682].ability)
        }) == ABILITY_WONDER_GUARD
        || gMoveResultFlags.get() as i32 & MOVE_RESULT_NO_EFFECT != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        let abilityAtk: u8 = {
            let gl_i683 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i683].ability)
        };
        {
            let gl_i684 = gBattlerAttacker.get();
            crate::gl!(
                gBattleMons[gl_i684].ability = ({
                    let gl_i685 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i685].ability)
                })
            )
        };
        {
            let gl_i686 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i686].ability = abilityAtk)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_tryimprison() {
    if {
        let gl_i228 = gBattlerAttacker.get();
        crate::gl!(gStatuses3[gl_i228])
    } & STATUS3_IMPRISONED_OTHERS
        != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        let sideAttacker: u8 = GetBattlerSide(gBattlerAttacker.get());
        PressurePPLoseOnUsingImprison(gBattlerAttacker.get());
        let mut battler: u8 = 0;
        while battler < gBattlersCount.get() {
            if sideAttacker != GetBattlerSide(battler) {
                let mut attackerMoveId: i32 = 0;
                while attackerMoveId < MAX_MON_MOVES {
                    let mut i: i32 = 0;
                    while i < MAX_MON_MOVES {
                        if ({
                            let gl_i687 = gBattlerAttacker.get();
                            crate::gl!(gBattleMons[gl_i687].moves[attackerMoveId])
                        }) == crate::gl!(gBattleMons[battler].moves[i])
                            && ({
                                let gl_i688 = gBattlerAttacker.get();
                                crate::gl!(gBattleMons[gl_i688].moves[attackerMoveId])
                            }) != MOVE_NONE
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i != MAX_MON_MOVES {
                        break;
                    }
                    attackerMoveId += 1;
                }
                if attackerMoveId != MAX_MON_MOVES {
                    {
                        let gl_v = STATUS3_IMPRISONED_OTHERS;
                        let gl_i229 = gBattlerAttacker.get();
                        crate::gl!(gStatuses3[gl_i229] |= gl_v)
                    };
                    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
                    break;
                }
            }
            battler += 1;
        }
        if battler == gBattlersCount.get() {
            crate::gl!(
                gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                    | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                    as usize as *mut u8
            );
        }
    }
}
pub(crate) unsafe fn Cmd_trysetgrudge() {
    if {
        let gl_i230 = gBattlerAttacker.get();
        crate::gl!(gStatuses3[gl_i230])
    } & STATUS3_GRUDGE
        != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        {
            let gl_v = STATUS3_GRUDGE;
            let gl_i231 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i231] |= gl_v)
        };
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_weightdamagecalculation() {
    let mut i: i32 = 0;
    while sWeightToDamageTable[i] != 0xFFFF {
        if sWeightToDamageTable[i]
            > GetPokedexHeightWeight(
                SpeciesToNationalPokedexNum({
                    let gl_i689 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i689].species)
                }),
                1,
            )
        {
            break;
        }
        i += 2;
    }
    if sWeightToDamageTable[i] != 0xFFFF {
        gDynamicBasePower.set(sWeightToDamageTable[i + 1]);
    } else {
        gDynamicBasePower.set(120);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_assistattackselect() {
    let mut chooseableMovesNo: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let validMoves: *mut u16 = (*crate::gl!(gBattleStruct))
        .assistPossibleMoves
        .as_mut_ptr();
    if GetBattlerPosition(gBattlerAttacker.get()) as i32 & 1 != B_SIDE_PLAYER as i32 {
        party = (*gEnemyParty.as_ptr()).as_mut_ptr();
    } else {
        party = (*gPlayerParty.as_ptr()).as_mut_ptr();
    }
    for monId in 0..PARTY_SIZE {
        'l1: {
            if monId
                == ({
                    let gl_i690 = gBattlerAttacker.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i690])
                }) as i32
            {
                break 'l1;
            }
            if GetMonData2(party.at(monId), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32 {
                break 'l1;
            }
            if GetMonData2(party.at(monId), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG {
                break 'l1;
            }
            for moveIndex in 0..MAX_MON_MOVES {
                'l3: {
                    let mut i: i32 = 0;
                    let r#move: u16 =
                        GetMonData2(party.at(monId), MON_DATA_MOVE1 + moveIndex) as u16;
                    if IsInvalidForSleepTalkOrAssist(r#move) != 0 {
                        break 'l3;
                    }
                    while sMovesForbiddenToCopy[i] != ASSIST_FORBIDDEN_END
                        && r#move != sMovesForbiddenToCopy[i]
                    {
                        i += 1;
                    }
                    if sMovesForbiddenToCopy[i] != ASSIST_FORBIDDEN_END {
                        break 'l3;
                    }
                    if r#move == MOVE_NONE {
                        break 'l3;
                    }
                    *validMoves.at(chooseableMovesNo) = r#move;
                    chooseableMovesNo += 1;
                }
            }
        }
    }
    if chooseableMovesNo != 0 {
        gHitMarker.set(gHitMarker.get() & (0xfffffbff));
        gCalledMove.set(*validMoves.at(((Random() as i32 & 0xFF) * chooseableMovesNo) >> 8));
        gBattlerTarget.set(GetMoveTarget(gCalledMove.get(), NO_TARGET_OVERRIDE));
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_trysetmagiccoat() {
    gBattlerTarget.set(gBattlerAttacker.get());
    (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()].set_ppNotAffectedByPressure(1);
    if gCurrentTurnActionNumber.get() as i32 == gBattlersCount.get() as i32 - 1 {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_bounceMove(TRUE as u32);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_trysetsnatch() {
    (*gSpecialStatuses.as_ptr())[gBattlerAttacker.get()].set_ppNotAffectedByPressure(1);
    if gCurrentTurnActionNumber.get() as i32 == gBattlersCount.get() as i32 - 1 {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].set_stealMove(1);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_trygetintimidatetarget() {
    crate::gl!(gBattleScripting.battler = (*crate::gl!(gBattleStruct)).intimidateBattler);
    let side: u8 = GetBattlerSide(crate::gl!(gBattleScripting.battler));
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 9);
    crate::gl!(
        gBattleTextBuff1[2] = ({
            let gl_i691 = crate::gl!(gBattleScripting.battler);
            crate::gl!(gBattleMons[gl_i691].ability)
        })
    );
    crate::gl!(gBattleTextBuff1[3] = 0xFF);
    'l2: while gBattlerTarget.get() < gBattlersCount.get() {
        'l1: {
            if GetBattlerSide(gBattlerTarget.get()) == side {
                break 'l1;
            }
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gBattlerTarget.get()]
                == 0
            {
                break 'l2;
            }
        }
        gBattlerTarget.set(gBattlerTarget.get() + (1));
    }
    if gBattlerTarget.get() >= gBattlersCount.get() {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_switchoutabilities() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    if ({
        let gl_i692 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i692].ability)
    }) == ABILITY_NATURAL_CURE
    {
        {
            let gl_i693 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i693].status1 = 0)
        };
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            gBitTable[*(*crate::gl!(gBattleStruct))
                .battlerPartyIndexes
                .as_mut_ptr()
                .at(gActiveBattler.get())] as u8,
            4,
            ({
                let gl_i694 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i694].status1)
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
pub(crate) unsafe fn Cmd_jumpifhasnohp() {
    gActiveBattler.set(GetBattlerForBattleScript(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    if ({
        let gl_i695 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i695].hp)
    }) == 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(2) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(2).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(6));
    }
}
pub(crate) unsafe fn Cmd_getsecretpowereffect() {
    match gBattleEnvironment.get() {
        BATTLE_ENVIRONMENT_GRASS => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_POISON);
        }
        BATTLE_ENVIRONMENT_LONG_GRASS => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_SLEEP);
        }
        BATTLE_ENVIRONMENT_SAND => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_ACC_MINUS_1);
        }
        BATTLE_ENVIRONMENT_UNDERWATER => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_DEF_MINUS_1);
        }
        BATTLE_ENVIRONMENT_WATER => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_ATK_MINUS_1);
        }
        BATTLE_ENVIRONMENT_POND => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_SPD_MINUS_1);
        }
        BATTLE_ENVIRONMENT_MOUNTAIN => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_CONFUSION);
        }
        BATTLE_ENVIRONMENT_CAVE => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_FLINCH);
        }
        _ => {
            crate::gl!(gBattleCommunication[3] = MOVE_EFFECT_PARALYSIS);
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_pickup() {
    let mut species: u16 = 0;
    let mut heldItem: u16 = 0;
    let mut ability: u8 = 0;
    if InBattlePike() != 0 {
    } else if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        for i in 0..PARTY_SIZE {
            species = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG) as u16;
            heldItem = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HELD_ITEM) as u16;
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_ABILITY_NUM) != 0 {
                ability = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[1];
            } else {
                ability = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[0];
            }
            if ability == ABILITY_PICKUP
                && species != SPECIES_NONE
                && species != SPECIES_EGG as u16
                && heldItem == ITEM_NONE
                && Random() as i32 % 10 == 0
            {
                heldItem = GetBattlePyramidPickupItemId();
                SetMonData(
                    crate::gl!(&raw gPlayerParty[i]),
                    MON_DATA_HELD_ITEM,
                    &raw mut heldItem as *mut c_void,
                );
            }
        }
    } else {
        for i in 0..PARTY_SIZE {
            species = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG) as u16;
            heldItem = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HELD_ITEM) as u16;
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_ABILITY_NUM) != 0 {
                ability = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[1];
            } else {
                ability = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .abilities[0];
            }
            if ability == ABILITY_PICKUP
                && species != SPECIES_NONE
                && species != SPECIES_EGG as u16
                && heldItem == ITEM_NONE
                && Random() as i32 % 10 == 0
            {
                let rand: i32 = Random() as i32 % 100;
                let mut lvlDivBy10: u8 =
                    ((GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_LEVEL) - 1) / 10)
                        as u8;
                if lvlDivBy10 > 9 {
                    lvlDivBy10 = 9;
                }
                for j in 0..9i32 {
                    if sPickupProbabilities[j] as i32 > rand {
                        SetMonData(
                            crate::gl!(&raw gPlayerParty[i]),
                            MON_DATA_HELD_ITEM,
                            (&raw const sPickupItems[lvlDivBy10 as i32 + j]).cast_mut()
                                as *mut c_void,
                        );
                        break;
                    } else if rand == 99 || rand == 98 {
                        SetMonData(
                            crate::gl!(&raw gPlayerParty[i]),
                            MON_DATA_HELD_ITEM,
                            (&raw const sRarePickupItems[lvlDivBy10 as i32 + (99 - rand)])
                                .cast_mut() as *mut c_void,
                        );
                        break;
                    }
                }
            }
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_docastformchangeanimation() {
    gActiveBattler.set(crate::gl!(gBattleScripting.battler));
    if ({
        let gl_i696 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i696].status2)
    }) & STATUS2_SUBSTITUTE
        != 0
    {
        (*crate::gl!(gBattleStruct)).formToChangeInto |= CASTFORM_SUBSTITUTE as u8;
    }
    BtlController_EmitBattleAnimation(
        B_COMM_TO_CONTROLLER,
        B_ANIM_CASTFORM_CHANGE,
        (*crate::gl!(gBattleStruct)).formToChangeInto as u16,
    );
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_trycastformdatachange() {
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
    let form: u8 = CastformDataTypeChange(crate::gl!(gBattleScripting.battler));
    if form != 0 {
        BattleScriptPushCursorAndCallback(
            (*crate::asmdata::BattleScript_CastformChange.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        (*crate::gl!(gBattleStruct)).formToChangeInto = form - 1;
    }
}
pub(crate) unsafe fn Cmd_settypebasedhalvers() {
    let mut worked: u8 = FALSE;
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove.get()]
    .effect
        == EFFECT_MUD_SPORT
    {
        if {
            let gl_i232 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i232])
        } & STATUS3_MUDSPORT
            == 0
        {
            {
                let gl_v = STATUS3_MUDSPORT;
                let gl_i233 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i233] |= gl_v)
            };
            crate::gl!(gBattleCommunication[5] = B_MSG_WEAKEN_ELECTRIC);
            worked = TRUE;
        }
    } else {
        if {
            let gl_i234 = gBattlerAttacker.get();
            crate::gl!(gStatuses3[gl_i234])
        } & STATUS3_WATERSPORT
            == 0
        {
            {
                let gl_v = STATUS3_WATERSPORT;
                let gl_i235 = gBattlerAttacker.get();
                crate::gl!(gStatuses3[gl_i235] |= gl_v)
            };
            crate::gl!(gBattleCommunication[5] = B_MSG_WEAKEN_FIRE);
            worked = TRUE;
        }
    }
    if worked != 0 {
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_setweatherballtype() {
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0 {
        if gBattleWeather.get() as i32 & B_WEATHER_ANY != 0 {
            crate::gl!(gBattleScripting.dmgMultiplier = 2);
        }
        if gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0 {
            (*crate::gl!(gBattleStruct)).dynamicMoveType = 139;
        } else if gBattleWeather.get() as i32 & B_WEATHER_SANDSTORM != 0 {
            (*crate::gl!(gBattleStruct)).dynamicMoveType = 133;
        } else if gBattleWeather.get() as i32 & B_WEATHER_SUN != 0 {
            (*crate::gl!(gBattleStruct)).dynamicMoveType = 138;
        } else if gBattleWeather.get() as i32 & B_WEATHER_HAIL != 0 {
            (*crate::gl!(gBattleStruct)).dynamicMoveType = 143;
        } else {
            (*crate::gl!(gBattleStruct)).dynamicMoveType = F_DYNAMIC_TYPE_SET;
        }
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_tryrecycleitem() {
    gActiveBattler.set(gBattlerAttacker.get());
    let usedHeldItem: *mut u16 =
        &raw mut (*crate::gl!(gBattleStruct)).usedHeldItems[gActiveBattler.get()];
    if *usedHeldItem != ITEM_NONE
        && ({
            let gl_i697 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i697].item)
        }) == ITEM_NONE
    {
        gLastUsedItem.set(*usedHeldItem);
        *usedHeldItem = ITEM_NONE;
        {
            let gl_i698 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i698].item = gLastUsedItem.get())
        };
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_HELDITEM_BATTLE,
            0,
            2,
            ({
                let gl_i699 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i699].item)
            }) as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_settypetoenvironment() {
    if !(({
        let gl_i700 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i700].types[0])
    }) == sEnvironmentToType[gBattleEnvironment.get()]
        || ({
            let gl_i701 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i701].types[1])
        }) == sEnvironmentToType[gBattleEnvironment.get()])
    {
        {
            let gl_i702 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i702].types[0] = sEnvironmentToType[gBattleEnvironment.get()])
        };
        {
            let gl_i703 = gBattlerAttacker.get();
            crate::gl!(gBattleMons[gl_i703].types[1] = sEnvironmentToType[gBattleEnvironment.get()])
        };
        crate::gl!(gBattleTextBuff1[0] = 0xFD);
        crate::gl!(gBattleTextBuff1[1] = 3);
        crate::gl!(gBattleTextBuff1[2] = sEnvironmentToType[gBattleEnvironment.get()]);
        crate::gl!(gBattleTextBuff1[3] = 0xFF);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_pursuitdoubles() {
    gActiveBattler.set(GetBattlerAtPosition(
        GetBattlerPosition(gBattlerAttacker.get()) ^ 2,
    ));
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
        && gAbsentBattlerFlags.get() as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
            == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gChosenActionByBattler;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()]
            == B_ACTION_USE_MOVE
        && (*(crate::gl!(&raw @at crate::battle_main::gChosenMoveByBattler;).cast_const())
            .cast::<CArray<u16, 4>>()
            .cast_mut())[gActiveBattler.get()]
            == MOVE_PURSUIT
    {
        {
            let gl_v = B_ACTION_TRY_FINISH;
            let gl_i236 = gActiveBattler.get();
            crate::gl!(gActionsByTurnOrder[gl_i236] = gl_v)
        };
        gCurrentMove.set(MOVE_PURSUIT);
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
        crate::gl!(gBattleScripting.animTurn = 1);
        crate::gl!(gBattleScripting.pursuitDoublesAttacker = gBattlerAttacker.get());
        gBattlerAttacker.set(gActiveBattler.get());
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    }
}
pub(crate) unsafe fn Cmd_snatchsetbattlers() {
    gEffectBattler.set(gBattlerAttacker.get());
    if gBattlerAttacker.get() == gBattlerTarget.get() {
        gBattlerAttacker.set({
            gBattlerTarget.set(crate::gl!(gBattleScripting.battler));
            gBattlerTarget.get()
        });
    } else {
        gBattlerTarget.set(crate::gl!(gBattleScripting.battler));
    }
    crate::gl!(gBattleScripting.battler = gEffectBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_removelightscreenreflect() {
    let opposingSide: u8 = GetBattlerSide(gBattlerAttacker.get()) ^ 1;
    if crate::gl!(gSideTimers[opposingSide].reflectTimer) != 0
        || crate::gl!(gSideTimers[opposingSide].lightscreenTimer) != 0
    {
        crate::gl!(gSideStatuses[opposingSide] &= 65534);
        crate::gl!(gSideStatuses[opposingSide] &= 65533);
        crate::gl!(gSideTimers[opposingSide].reflectTimer = 0);
        crate::gl!(gSideTimers[opposingSide].lightscreenTimer = 0);
        crate::gl!(gBattleScripting.animTurn = 1);
        crate::gl!(gBattleScripting.animTargetsHit = 1);
    } else {
        crate::gl!(gBattleScripting.animTurn = 0);
        crate::gl!(gBattleScripting.animTargetsHit = 0);
    }
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_handleballthrow() {
    let mut ballMultiplier: u8 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(gBattlerAttacker.get());
    gBattlerTarget.set(gBattlerAttacker.get() ^ 1);
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 {
        BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, BALL_TRAINER_BLOCK);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_TrainerBallBlock
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
        BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, BALL_3_SHAKES_SUCCESS);
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_WallyBallThrow
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    } else {
        let mut catchRate: u8 = 0;
        if gLastUsedItem.get() == ITEM_SAFARI_BALL {
            catchRate = ((*crate::gl!(gBattleStruct)).safariCatchFactor as i32 * 1275 / 100) as u8;
        } else {
            catchRate = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                .cast::<CArray<SpeciesInfo, 0>>())[{
                let gl_i704 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i704].species)
            }]
            .catchRate;
        }
        if gLastUsedItem.get() > ITEM_SAFARI_BALL {
            match gLastUsedItem.get() {
                ITEM_NET_BALL => {
                    if ({
                        let gl_i705 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i705].types[0])
                    }) == TYPE_WATER
                        || ({
                            let gl_i706 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i706].types[1])
                        }) == TYPE_WATER
                        || (({
                            let gl_i707 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i707].types[0])
                        }) == TYPE_BUG
                            || ({
                                let gl_i708 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i708].types[1])
                            }) == TYPE_BUG)
                    {
                        ballMultiplier = 30;
                    } else {
                        ballMultiplier = 10;
                    }
                }
                ITEM_DIVE_BALL => {
                    if GetCurrentMapType() == MAP_TYPE_UNDERWATER {
                        ballMultiplier = 35;
                    } else {
                        ballMultiplier = 10;
                    }
                }
                ITEM_NEST_BALL => {
                    if ({
                        let gl_i709 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i709].level)
                    }) < 40
                    {
                        ballMultiplier = 40
                            - ({
                                let gl_i710 = gBattlerTarget.get();
                                crate::gl!(gBattleMons[gl_i710].level)
                            });
                        if ballMultiplier <= 9 {
                            ballMultiplier = 10;
                        }
                    } else {
                        ballMultiplier = 10;
                    }
                }
                ITEM_REPEAT_BALL => {
                    if GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum({
                            let gl_i711 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i711].species)
                        }),
                        FLAG_GET_CAUGHT,
                    ) != 0
                    {
                        ballMultiplier = 30;
                    } else {
                        ballMultiplier = 10;
                    }
                }
                ITEM_TIMER_BALL => {
                    ballMultiplier = crate::gl!(gBattleResults.battleTurnCounter) + 10;
                    if ballMultiplier > 40 {
                        ballMultiplier = 40;
                    }
                }
                11 | ITEM_PREMIER_BALL => {
                    ballMultiplier = 10;
                }
                _ => {}
            }
        } else {
            ballMultiplier = sBallCatchBonuses[gLastUsedItem.get() as i32 - ITEM_ULTRA_BALL];
        }
        let mut odds: u32 = div_i32(
            catchRate as i32 * ballMultiplier as i32 / 10
                * (({
                    let gl_i712 = gBattlerTarget.get();
                    crate::gl!(gBattleMons[gl_i712].maxHP)
                }) as i32
                    * 3
                    - ({
                        let gl_i713 = gBattlerTarget.get();
                        crate::gl!(gBattleMons[gl_i713].hp)
                    }) as i32
                        * 2),
            3 * ({
                let gl_i714 = gBattlerTarget.get();
                crate::gl!(gBattleMons[gl_i714].maxHP)
            }) as i32,
        ) as u32;
        if ({
            let gl_i715 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i715].status1)
        }) & 39
            != 0
        {
            odds *= 2;
        }
        if ({
            let gl_i716 = gBattlerTarget.get();
            crate::gl!(gBattleMons[gl_i716].status1)
        }) & 216
            != 0
        {
            odds = odds * 15 / 10;
        }
        if gLastUsedItem.get() != ITEM_SAFARI_BALL {
            if gLastUsedItem.get() == ITEM_MASTER_BALL {
                (*gBattleResults.as_ptr()).set_usedMasterBall(TRUE);
            } else {
                {
                    let gl_v = (*gBattleResults.as_ptr()).catchAttempts
                        [gLastUsedItem.get() as i32 - ITEM_ULTRA_BALL]
                        .saturating_add(1);
                    let gl_i237 = gLastUsedItem.get() as i32 - ITEM_ULTRA_BALL;
                    crate::gl!(gBattleResults.catchAttempts[gl_i237] = gl_v)
                };
            }
        }
        if odds > 254 {
            BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, BALL_3_SHAKES_SUCCESS);
            MarkBattlerForControllerExec(gActiveBattler.get());
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SuccessBallThrow
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            SetMonData(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i717 = gBattlerTarget.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i717])
                }],
                MON_DATA_POKEBALL,
                gLastUsedItem.as_ptr() as *mut c_void,
            );
            if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                crate::gl!(gBattleCommunication[5] = 0);
            } else {
                crate::gl!(gBattleCommunication[5] = 1);
            }
        } else {
            odds = Sqrt(Sqrt(div_u32(0xff0000, odds)) as u32) as u32;
            odds = div_u32(0xffff0, odds);
            let mut shakes: u8 = 0;
            while shakes < BALL_3_SHAKES_SUCCESS && (Random() as u32) < odds {
                shakes += 1;
            }
            if gLastUsedItem.get() == ITEM_MASTER_BALL {
                shakes = BALL_3_SHAKES_SUCCESS;
            }
            BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, shakes);
            MarkBattlerForControllerExec(gActiveBattler.get());
            if shakes == BALL_3_SHAKES_SUCCESS {
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SuccessBallThrow
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                SetMonData(
                    &raw mut (*gEnemyParty.as_ptr())[{
                        let gl_i718 = gBattlerTarget.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i718])
                    }],
                    MON_DATA_POKEBALL,
                    gLastUsedItem.as_ptr() as *mut c_void,
                );
                if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                    crate::gl!(gBattleCommunication[5] = 0);
                } else {
                    crate::gl!(gBattleCommunication[5] = 1);
                }
            } else {
                crate::gl!(gBattleCommunication[5] = shakes);
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_ShakeBallThrow
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
            }
        }
    }
}
pub(crate) unsafe fn Cmd_givecaughtmon() {
    if GiveMonToPlayer(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i719 = gBattlerAttacker.get() as i32 ^ 1;
            crate::gl!(gBattlerPartyIndexes[gl_i719])
        }],
    ) != MON_GIVEN_TO_PARTY
    {
        if ShouldShowBoxWasFullMessage() == 0 {
            crate::gl!(gBattleCommunication[5] = B_MSG_SENT_SOMEONES_PC);
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
            );
            GetMonData3(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i720 = gBattlerAttacker.get() as i32 ^ 1;
                    crate::gl!(gBattlerPartyIndexes[gl_i720])
                }],
                MON_DATA_NICKNAME,
                (*gStringVar2.as_ptr()).as_mut_ptr(),
            );
        } else {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
            );
            GetMonData3(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i721 = gBattlerAttacker.get() as i32 ^ 1;
                    crate::gl!(gBattlerPartyIndexes[gl_i721])
                }],
                MON_DATA_NICKNAME,
                (*gStringVar2.as_ptr()).as_mut_ptr(),
            );
            StringCopy(
                (*gStringVar3.as_ptr()).as_mut_ptr(),
                GetBoxNamePtr(GetPCBoxToSendMon() as u8),
            );
            crate::gl!(gBattleCommunication[5] = B_MSG_SOMEONES_BOX_FULL);
        }
        if FlagGet(FLAG_SYS_PC_LANETTE) != 0 {
            crate::gl!(gBattleCommunication[5] += 1);
        }
    }
    crate::gl!(
        gBattleResults.caughtMonSpecies = GetMonData3(
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i722 = gBattlerAttacker.get() as i32 ^ 1;
                crate::gl!(gBattlerPartyIndexes[gl_i722])
            }],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16
    );
    GetMonData3(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i723 = gBattlerAttacker.get() as i32 ^ 1;
            crate::gl!(gBattlerPartyIndexes[gl_i723])
        }],
        MON_DATA_NICKNAME,
        (*gBattleResults.as_ptr()).caughtMonNick.as_mut_ptr(),
    );
    (*gBattleResults.as_ptr()).set_caughtMonBall(GetMonData3(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i724 = gBattlerAttacker.get() as i32 ^ 1;
            crate::gl!(gBattlerPartyIndexes[gl_i724])
        }],
        MON_DATA_POKEBALL,
        null_mut(),
    ) as u8);
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_trysetcaughtmondexflags() {
    let species: u16 = GetMonData3(
        crate::gl!(&raw gEnemyParty[0]),
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    let personality: u32 = GetMonData3(
        crate::gl!(&raw gEnemyParty[0]),
        MON_DATA_PERSONALITY,
        null_mut(),
    );
    if GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), FLAG_GET_CAUGHT) != 0 {
        crate::gl!(
            gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                as usize as *mut u8
        );
    } else {
        HandleSetPokedexFlag(
            SpeciesToNationalPokedexNum(species),
            FLAG_SET_CAUGHT,
            personality,
        );
        crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
    }
}
pub(crate) unsafe fn Cmd_displaydexinfo() {
    let species: u16 = GetMonData3(
        crate::gl!(&raw gEnemyParty[0]),
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            crate::gl!(gBattleCommunication[0] += 1);
        }
        1 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                FreeAllWindowBuffers();
                crate::gl!(
                    gBattleCommunication[1] = DisplayCaughtMonDexPage(
                        SpeciesToNationalPokedexNum(species),
                        {
                            let gl_i725 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i725].otId)
                        },
                        {
                            let gl_i726 = gBattlerTarget.get();
                            crate::gl!(gBattleMons[gl_i726].personality)
                        },
                    )
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        2 => {
            if (*gPaletteFade.as_ptr()).active() == 0
                && crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
                && (*gTasks.as_ptr())[crate::gl!(gBattleCommunication[1])].isActive == 0
            {
                SetVBlankCallback(Some(VBlankCB_Battle));
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        3 => {
            InitBattleBgsVideo();
            LoadBattleTextboxAndBackground();
            gBattle_BG3_X.set(256);
            crate::gl!(gBattleCommunication[0] += 1);
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                BeginNormalPaletteFade(PALETTES_BG, 0, 16, 0, 0);
                ShowBg(0);
                ShowBg(3);
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        5 if (*gPaletteFade.as_ptr()).active() == 0 => {
            crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
        }
        _ => {}
    }
}
pub unsafe fn HandleBattleWindow(xStart: u8, yStart: u8, xEnd: u8, yEnd: u8, flags: u8) {
    let mut destX: i32 = 0;
    let mut var: u16 = 0;
    let mut destY: i32 = yStart as i32;
    while destY <= yEnd as i32 {
        destX = xStart as i32;
        while destX <= xEnd as i32 {
            if destY == yStart as i32 {
                if destX == xStart as i32 {
                    var = 0x1022;
                } else if destX == xEnd as i32 {
                    var = 0x1024;
                } else {
                    var = 0x1023;
                }
            } else if destY == yEnd as i32 {
                if destX == xStart as i32 {
                    var = 0x1028;
                } else if destX == xEnd as i32 {
                    var = 0x102A;
                } else {
                    var = 0x1029;
                }
            } else {
                if destX == xStart as i32 {
                    var = 0x1025;
                } else if destX == xEnd as i32 {
                    var = 0x1027;
                } else {
                    var = 0x1026;
                }
            }
            if flags as i32 & WINDOW_CLEAR as i32 != 0 {
                var = 0;
            }
            if flags as i32 & WINDOW_BG1 as i32 != 0 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1,
                    &raw mut var as *mut c_void,
                    destX as u8,
                    destY as u8,
                    1,
                    1,
                    0x11,
                );
            } else {
                CopyToBgTilemapBufferRect_ChangePalette(
                    0,
                    &raw mut var as *mut c_void,
                    destX as u8,
                    destY as u8,
                    1,
                    1,
                    0x11,
                );
            }
            destX += 1;
        }
        destY += 1;
    }
}
pub unsafe fn BattleCreateYesNoCursorAt(cursorPosition: u8) {
    let mut src: CArray<u16, 2> = zeroed();
    src[0] = 1;
    src[1] = 2;
    CopyToBgTilemapBufferRect_ChangePalette(
        0,
        src.as_mut_ptr() as *mut c_void,
        0x19,
        9 + 2 * cursorPosition,
        1,
        2,
        0x11,
    );
    CopyBgTilemapBufferToVram(0);
}
pub unsafe fn BattleDestroyYesNoCursorAt(cursorPosition: u8) {
    let mut src: CArray<u16, 2> = zeroed();
    src[0] = 0x1016;
    src[1] = 0x1016;
    CopyToBgTilemapBufferRect_ChangePalette(
        0,
        src.as_mut_ptr() as *mut c_void,
        0x19,
        9 + 2 * cursorPosition,
        1,
        2,
        0x11,
    );
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe fn Cmd_trygivecaughtmonnick() {
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                B_WIN_YESNO,
            );
            crate::gl!(gBattleCommunication[0] += 1);
            crate::gl!(gBattleCommunication[1] = 0);
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    != 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 0);
                BattleCreateYesNoCursorAt(0);
            }
            if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleCommunication;).cast_const())
                    .cast::<CArray<u8, 8>>()
                    .cast_mut())[1]
                    == 0
            {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                crate::gl!(gBattleCommunication[1] = 1);
                BattleCreateYesNoCursorAt(1);
            }
            if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if crate::gl!(gBattleCommunication[1]) == 0 {
                    crate::gl!(gBattleCommunication[0] += 1);
                    BeginFastPaletteFade(3);
                } else {
                    crate::gl!(gBattleCommunication[0] = 4);
                }
            } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                crate::gl!(gBattleCommunication[0] = 4);
            }
        }
        2 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                GetMonData3(
                    &raw mut (*gEnemyParty.as_ptr())[{
                        let gl_i727 = gBattlerAttacker.get() as i32 ^ 1;
                        crate::gl!(gBattlerPartyIndexes[gl_i727])
                    }],
                    MON_DATA_NICKNAME,
                    (*crate::gl!(gBattleStruct)).caughtMonNick.as_mut_ptr(),
                );
                FreeAllWindowBuffers();
                DoNamingScreen(
                    NAMING_SCREEN_CAUGHT_MON,
                    (*crate::gl!(gBattleStruct)).caughtMonNick.as_mut_ptr(),
                    GetMonData2(
                        &raw mut (*gEnemyParty.as_ptr())[{
                            let gl_i728 = gBattlerAttacker.get() as i32 ^ 1;
                            crate::gl!(gBattlerPartyIndexes[gl_i728])
                        }],
                        MON_DATA_SPECIES,
                    ) as u16,
                    GetMonGender(
                        &raw mut (*gEnemyParty.as_ptr())[{
                            let gl_i729 = gBattlerAttacker.get() as i32 ^ 1;
                            crate::gl!(gBattlerPartyIndexes[gl_i729])
                        }],
                    ) as u16,
                    GetMonData3(
                        &raw mut (*gEnemyParty.as_ptr())[{
                            let gl_i730 = gBattlerAttacker.get() as i32 ^ 1;
                            crate::gl!(gBattlerPartyIndexes[gl_i730])
                        }],
                        MON_DATA_PERSONALITY,
                        null_mut(),
                    ),
                    Some(BattleMainCB2),
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        3 => {
            if crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
                && (*gPaletteFade.as_ptr()).active() == 0
            {
                SetMonData(
                    &raw mut (*gEnemyParty.as_ptr())[{
                        let gl_i731 = gBattlerAttacker.get() as i32 ^ 1;
                        crate::gl!(gBattlerPartyIndexes[gl_i731])
                    }],
                    MON_DATA_NICKNAME,
                    (*crate::gl!(gBattleStruct)).caughtMonNick.as_mut_ptr() as *mut c_void,
                );
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        4 => {
            if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(5));
            } else {
                crate::gl!(
                    gBattlescriptCurrInstr = (*(*gBattlescriptCurrInstr.as_ptr()).at(1) as i32
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(1) as i32) << 8
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(2) as i32) << 16
                        | (*(*gBattlescriptCurrInstr.as_ptr()).at(1).at(3) as i32) << 24)
                        as usize as *mut u8
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Cmd_subattackerhpbydmg() {
    {
        let gl_i732 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i732].hp -= gBattleMoveDamage.get() as u16)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) unsafe fn Cmd_removeattackerstatus1() {
    {
        let gl_i733 = gBattlerAttacker.get();
        crate::gl!(gBattleMons[gl_i733].status1 = 0)
    };
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(1));
}
pub(crate) fn Cmd_finishaction() {
    gCurrentActionFuncId.set(B_ACTION_FINISHED);
}
pub(crate) fn Cmd_finishturn() {
    gCurrentActionFuncId.set(B_ACTION_FINISHED);
    gCurrentTurnActionNumber.set(gBattlersCount.get());
}
pub(crate) unsafe fn Cmd_trainerslideout() {
    gActiveBattler.set(GetBattlerAtPosition(
        *(*gBattlescriptCurrInstr.as_ptr()).at(1),
    ));
    BtlController_EmitTrainerSlideBack(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler.get());
    crate::gl!(gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.as_ptr()).at(2));
}
