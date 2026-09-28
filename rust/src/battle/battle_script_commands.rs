//! Translated from `src/battle_script_commands.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static BattleScript_AbilityNoSpecificStatLoss: CArray<u8, 0>;
    static BattleScript_AbilityNoStatLoss: CArray<u8, 0>;
    static BattleScript_AllStatsUp: CArray<u8, 0>;
    static BattleScript_AlreadyAtFullHp: CArray<u8, 0>;
    static BattleScript_AtkDefDown: CArray<u8, 0>;
    static BattleScript_BRNPrevention: CArray<u8, 0>;
    static BattleScript_ButItFailed: CArray<u8, 0>;
    static BattleScript_CastformChange: CArray<u8, 0>;
    static BattleScript_DampStopsExplosion: CArray<u8, 0>;
    static BattleScript_DefrostedViaFireMove: CArray<u8, 0>;
    static BattleScript_DestinyBondTakesLife: CArray<u8, 0>;
    static BattleScript_EnduredMsg: CArray<u8, 0>;
    static BattleScript_FaintAttacker: CArray<u8, 0>;
    static BattleScript_FaintTarget: CArray<u8, 0>;
    static BattleScript_FlinchPrevention: CArray<u8, 0>;
    static BattleScript_FlushMessageBox: CArray<u8, 0>;
    static BattleScript_FocusBandActivates: CArray<u8, 0>;
    static BattleScript_GrudgeTakesPP: CArray<u8, 0>;
    static BattleScript_HitFromCritCalc: CArray<u8, 0>;
    static BattleScript_ItemSteal: CArray<u8, 0>;
    static BattleScript_KnockedOff: CArray<u8, 0>;
    static BattleScript_LeechSeedFree: CArray<u8, 0>;
    static BattleScript_LevelUp: CArray<u8, 0>;
    static BattleScript_MagicCoatBounce: CArray<u8, 0>;
    static BattleScript_MistProtected: CArray<u8, 0>;
    static BattleScript_MoveEnd: CArray<u8, 0>;
    static BattleScript_MoveMissedPause: CArray<u8, 0>;
    static BattleScript_NoItemSteal: CArray<u8, 0>;
    static BattleScript_NoPPForMove: CArray<u8, 0>;
    static BattleScript_ObliviousPreventsAttraction: CArray<u8, 0>;
    static BattleScript_OneHitKOMsg: CArray<u8, 0>;
    static BattleScript_PRLZPrevention: CArray<u8, 0>;
    static BattleScript_PSNPrevention: CArray<u8, 0>;
    static BattleScript_Pausex20: CArray<u8, 0>;
    static BattleScript_PresentHealTarget: CArray<u8, 0>;
    static BattleScript_PrintPayDayMoneyString: CArray<u8, 0>;
    static BattleScript_RageIsBuilding: CArray<u8, 0>;
    static BattleScript_RapidSpinAway: CArray<u8, 0>;
    static BattleScript_SAtkDown2: CArray<u8, 0>;
    static BattleScript_ShakeBallThrow: CArray<u8, 0>;
    static BattleScript_SnatchedMove: CArray<u8, 0>;
    static BattleScript_SpikesFree: CArray<u8, 0>;
    static BattleScript_SpikesOnAttacker: CArray<u8, 0>;
    static BattleScript_SpikesOnFaintedBattler: CArray<u8, 0>;
    static BattleScript_SpikesOnTarget: CArray<u8, 0>;
    static BattleScript_StatDown: CArray<u8, 0>;
    static BattleScript_StatUp: CArray<u8, 0>;
    static BattleScript_StickyHoldActivates: CArray<u8, 0>;
    static BattleScript_SturdyPreventsOHKO: CArray<u8, 0>;
    static BattleScript_SubstituteFade: CArray<u8, 0>;
    static BattleScript_SuccessBallThrow: CArray<u8, 0>;
    static BattleScript_SuccessForceOut: CArray<u8, 0>;
    static BattleScript_TargetPRLZHeal: CArray<u8, 0>;
    static BattleScript_TookAttack: CArray<u8, 0>;
    static BattleScript_TrainerBallBlock: CArray<u8, 0>;
    static BattleScript_WallyBallThrow: CArray<u8, 0>;
    static BattleScript_WrapFree: CArray<u8, 0>;
    static mut gAbsentBattlerFlags: u8;
    static mut gActionsByTurnOrder: CArray<u8, 4>;
    static mut gActiveBattler: u8;
    static mut gBattleBufferB: CArray<CArray<u8, 512>, 4>;
    static mut gBattleCommunication: CArray<u8, 8>;
    static mut gBattleControllerExecFlags: u32;
    static mut gBattleEnvironment: u8;
    static mut gBattleMainFunc: Option<unsafe extern "C" fn()>;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static mut gBattleMoveDamage: i32;
    static mut gBattleMovePower: u16;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleOutcome: u8;
    static mut gBattleResources: *mut BattleResources;
    static mut gBattleResults: BattleResults;
    static mut gBattleScripting: BattleScripting;
    static gBattleScriptsForMoveEffects: CArray<*mut u8, 0>;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTextBuff1: CArray<u8, 16>;
    static mut gBattleTextBuff2: CArray<u8, 16>;
    static mut gBattleTextBuff3: CArray<u8, 16>;
    static mut gBattleTypeFlags: u32;
    static mut gBattleWeather: u16;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattlerAttacker: u8;
    static mut gBattlerByTurnOrder: CArray<u8, 4>;
    static mut gBattlerFainted: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerTarget: u8;
    static mut gBattlersCount: u8;
    static mut gBattlescriptCurrInstr: *mut u8;
    static mut gBideDmg: CArray<i32, 4>;
    static mut gBideTarget: CArray<u8, 4>;
    static gBitTable: CArray<u32, 0>;
    static mut gCalledMove: u16;
    static mut gChosenActionByBattler: CArray<u8, 4>;
    static mut gChosenMove: u16;
    static mut gChosenMoveByBattler: CArray<u16, 4>;
    static mut gChosenMovePos: u8;
    static mut gCritMultiplier: u8;
    static mut gCurrMovePos: u8;
    static mut gCurrentActionFuncId: u8;
    static mut gCurrentMove: u16;
    static mut gCurrentTurnActionNumber: u8;
    static mut gDisableStructs: CArray<DisableStruct, 4>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gDynamicBasePower: u16;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gEnigmaBerries: CArray<BattleEnigmaBerry, 4>;
    static mut gExpShareExp: u16;
    static mut gHitMarker: u32;
    static mut gHpDealt: i32;
    static mut gLastHitBy: CArray<u8, 4>;
    static mut gLastHitByType: CArray<u16, 4>;
    static mut gLastLandedMoves: CArray<u16, 4>;
    static mut gLastMoves: CArray<u16, 4>;
    static mut gLastPrintedMoves: CArray<u16, 4>;
    static mut gLastResultingMoves: CArray<u16, 4>;
    static mut gLastUsedAbility: u8;
    static mut gLastUsedItem: u16;
    static mut gLeveledUpInBattle: u8;
    static mut gLockedMoves: CArray<u16, 4>;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMain: Main;
    static gMissStringIds: CArray<u16, 0>;
    static mut gMoveResultFlags: u8;
    static mut gMoveToLearn: u16;
    static mut gMultiHitCounter: u8;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPartnerTrainerId: u16;
    static mut gPauseCounterBattle: u16;
    static mut gPaydayMoney: u16;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gPotentialItemEffectBattler: u8;
    static mut gProtectStructs: CArray<ProtectStruct, 4>;
    static gRefereeStringsTable: CArray<*mut u8, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSentPokesToOpponent: CArray<u8, 2>;
    static mut gSideStatuses: CArray<u16, 2>;
    static mut gSideTimers: CArray<SideTimer, 2>;
    static mut gSpecialStatuses: CArray<SpecialStatus, 4>;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStatuses3: CArray<u32, 4>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattleYesNoChoice: CArray<u8, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static mut gTrainerBattleOpponent_B: u16;
    static gTrainerMoneyTable: CArray<TrainerMoney, 0>;
    static gTrainers: CArray<Trainer, 0>;
    static gTrappingMoves: CArray<u16, 0>;
    static gTypeEffectiveness: CArray<u8, 336>;
    static mut gWishFutureKnock: WishFutureKnock;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn AddMoney(a0: *mut u32, a1: u32);
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AdjustFriendship(a0: *mut Pokemon, a1: u8);
    fn AdjustFriendshipOnBattleFaint(a0: u8);
    fn AtkCanceler_UnableToUseMove() -> u8;
    fn BattleArena_AddSkillPoints(a0: u8);
    fn BattleArena_InitPoints();
    fn BattleArena_ShowJudgmentWindow(a0: *mut u8) -> u8;
    fn BattleMainCB2();
    fn BattlePalace_TryEscapeStatus(a0: u8) -> u32;
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleScriptPop();
    fn BattleScriptPush(a0: *mut u8);
    fn BattleScriptPushCursor();
    fn BattleScriptPushCursorAndCallback(a0: *mut u8);
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitBallThrowAnim(a0: u8, a1: u8);
    fn BtlController_EmitBattleAnimation(a0: u8, a1: u8, a2: u16);
    fn BtlController_EmitCantSwitch(a0: u8);
    fn BtlController_EmitChoosePokemon(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut u8);
    fn BtlController_EmitDrawPartyStatusSummary(a0: u8, a1: *mut HpAndStatus, a2: u8);
    fn BtlController_EmitEndLinkBattle(a0: u8, a1: u8);
    fn BtlController_EmitExpUpdate(a0: u8, a1: u8, a2: u16);
    fn BtlController_EmitFaintAnimation(a0: u8);
    fn BtlController_EmitFaintingCry(a0: u8);
    fn BtlController_EmitGetMonData(a0: u8, a1: u8, a2: u8);
    fn BtlController_EmitHealthBarUpdate(a0: u8, a1: u16);
    fn BtlController_EmitHidePartyStatusSummary(a0: u8);
    fn BtlController_EmitHitAnimation(a0: u8);
    fn BtlController_EmitLinkStandbyMsg(a0: u8, a1: u8, a2: u32);
    fn BtlController_EmitMoveAnimation(
        a0: u8,
        a1: u16,
        a2: u8,
        a3: u16,
        a4: i32,
        a5: u8,
        a6: *mut DisableStruct,
        a7: u8,
    );
    fn BtlController_EmitPlayFanfareOrBGM(a0: u8, a1: u16, a2: u8);
    fn BtlController_EmitPlaySE(a0: u8, a1: u16);
    fn BtlController_EmitPrintSelectionString(a0: u8, a1: u16);
    fn BtlController_EmitResetActionMoveSelection(a0: u8, a1: u8);
    fn BtlController_EmitReturnMonToBall(a0: u8, a1: u8);
    fn BtlController_EmitSetMonData(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut c_void);
    fn BtlController_EmitSpriteInvisibility(a0: u8, a1: u8);
    fn BtlController_EmitStatusAnimation(a0: u8, a1: u8, a2: u32);
    fn BtlController_EmitStatusIconUpdate(a0: u8, a1: u32, a2: u32);
    fn BtlController_EmitSwitchInAnim(a0: u8, a1: u8, a2: u8);
    fn BtlController_EmitTrainerSlide(a0: u8);
    fn BtlController_EmitTrainerSlideBack(a0: u8);
    fn BtlController_EmitYesNoBox(a0: u8);
    fn CalculateBaseDamage(
        a0: *mut BattlePokemon,
        a1: *mut BattlePokemon,
        a2: u32,
        a3: u16,
        a4: u16,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> i32;
    fn CalculatePlayerPartyCount() -> u8;
    fn CancelMultiTurnMoves(a0: u8);
    fn CastformDataTypeChange(a0: u8) -> u8;
    fn CheckMoveLimitations(a0: u8, a1: u8, a2: u8) -> u8;
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyToWindowPixelBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountAliveMonsInBattle(a0: u8) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DisplayCaughtMonDexPage(a0: u16, a1: u32, a2: u32) -> u8;
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn DrawArenaRefereeTextBox();
    fn DrawLevelUpWindowPg1(a0: u16, a1: *mut u16, a2: *mut u16, a3: u8, a4: u8, a5: u8);
    fn DrawLevelUpWindowPg2(a0: u16, a1: *mut u16, a2: u8, a3: u8, a4: u8);
    fn EraseArenaRefereeTextBox();
    fn FaintClearSetData();
    fn FlagGet(a0: u16) -> u8;
    fn FreeAllWindowBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetAbilityBySpecies(a0: u16, a1: u8) -> u8;
    fn GetBattlePyramidPickupItemId() -> u16;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerForBattleScript(a0: u8) -> u8;
    fn GetBattlerMultiplayerId(a0: u16) -> i32;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetCurrentMapType() -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetItemHoldEffect(a0: u16) -> u8;
    fn GetItemHoldEffectParam(a0: u16) -> u8;
    fn GetLinkTrainerFlankId(a0: u8) -> u16;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMonIconPtr(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMonLevelUpWindowStats(a0: *mut Pokemon, a1: *mut u16);
    fn GetMonNickname(a0: *mut Pokemon, a1: *mut u8) -> *mut u8;
    fn GetMoveSlotToReplace() -> u8;
    fn GetMoveTarget(a0: u16, a1: u8) -> u8;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetPCBoxToSendMon() -> u16;
    fn GetPokedexHeightWeight(a0: u16, a1: u8) -> u16;
    fn GetScaledHPFraction(a0: i16, a1: i16, a2: u8) -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetValidMonIconPalettePtr(a0: u16) -> *mut u16;
    fn GiveMonToPlayer(a0: *mut Pokemon) -> u8;
    fn GiveMoveToBattleMon(a0: *mut BattlePokemon, a1: u16) -> u16;
    fn HandleLowHpMusicChange(a0: *mut Pokemon, a1: u8);
    fn HandleSetPokedexFlag(a0: u16, a1: u8, a2: u32);
    fn HasNoMonsToSwitch(a0: u8, a1: u8, a2: u8) -> u8;
    fn InBattlePike() -> u8;
    fn IncrementGameStat(a0: u8);
    fn InitBattleBgsVideo();
    fn IsCryFinished() -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsHMMove2(a0: u16) -> u32;
    fn IsMonDisobedient() -> u8;
    fn IsMultiBattle() -> u8;
    fn IsRunningFromBattleImpossible() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn IsTradedMon(a0: *mut Pokemon) -> u8;
    fn ItemBattleEffects(a0: u8, a1: u8, a2: u8) -> u8;
    fn LoadBattleTextboxAndBackground();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn MarkBattlerForControllerExec(a0: u8);
    fn MonGainEVs(a0: *mut Pokemon, a1: u16);
    fn MonTryLearningNewMove(a0: *mut Pokemon, a1: u8) -> u16;
    fn PlayBGM(a0: u16);
    fn PlaySE(a0: u16);
    fn PokemonUseItemEffects(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8) -> u8;
    fn PrepareStringBattle(a0: u16, a1: u8);
    fn PressurePPLose(a0: u8, a1: u8, a2: u16);
    fn PressurePPLoseOnUsingImprison(a0: u8);
    fn PressurePPLoseOnUsingPerishSong(a0: u8);
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RecordAbilityBattle(a0: u8, a1: u8);
    fn RecordItemEffectBattle(a0: u8, a1: u8);
    fn RecordedBattle_SetBattlerAction(a0: u8, a1: u8);
    fn RemoveBattleMonPPBonus(a0: *mut BattlePokemon, a1: u8);
    fn RemoveMonPPBonus(a0: *mut Pokemon, a1: u8);
    fn ResetSentPokesToOpponentValue();
    fn ReshowBattleScreenAfterMenu();
    fn SetBattleMonMoveSlot(a0: *mut BattlePokemon, a1: u16, a2: u8);
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShouldShowBoxWasFullMessage() -> u8;
    fn ShowBg(a0: u8);
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut Pokemon,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn Sqrt(a0: u32) -> u16;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn SwitchInClearSetData();
    fn SwitchPartyOrder(a0: u8);
    fn SwitchPartyOrderInGameMulti(a0: u8, a1: u8);
    fn SwitchPartyOrderLinkMulti(a0: u8, a1: u8, a2: u8);
    fn TryRunFromBattle(a0: u8) -> u8;
    fn UpdateSentPokesToOpponentValue(a0: u8);
    fn VBlankCB_Battle();
    fn VarGet(a0: u16) -> u16;
    fn WasUnableToUseMove(a0: u8) -> u8;
    fn WriteColorChangeControlCode(a0: *mut u8, a1: u32, a2: u8) -> *mut u8;
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
}

pub(crate) unsafe extern "C" fn Cmd_attackcanceler() {
    let mut i: i32 = 0;
    if gBattleOutcome != 0 {
        gCurrentActionFuncId = B_ACTION_FINISHED;
        return;
    }
    if gBattleMons[gBattlerAttacker].hp == 0 && gHitMarker & HITMARKER_NO_ATTACKSTRING == 0 {
        gHitMarker |= HITMARKER_UNABLE_TO_USE_MOVE;
        gBattlescriptCurrInstr = BattleScript_MoveEnd.as_ptr().cast_mut();
        return;
    }
    if AtkCanceler_UnableToUseMove() != 0 {
        return;
    }
    if AbilityBattleEffects(ABILITYEFFECT_MOVES_BLOCK, gBattlerTarget, 0, 0, 0) != 0 {
        return;
    }
    if gBattleMons[gBattlerAttacker].pp[gCurrMovePos] == 0
        && gCurrentMove != MOVE_STRUGGLE
        && gHitMarker & 0x800200 == 0
        && gBattleMons[gBattlerAttacker].status2 & STATUS2_MULTIPLETURNS == 0
    {
        gBattlescriptCurrInstr = BattleScript_NoPPForMove.as_ptr().cast_mut();
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        return;
    }
    gHitMarker &= 0xff7fffff;
    if gHitMarker & HITMARKER_OBEYS == 0
        && gBattleMons[gBattlerAttacker].status2 & STATUS2_MULTIPLETURNS == 0
    {
        i = IsMonDisobedient() as i32;
        match i {
            0 => {}
            2 => {
                gHitMarker |= HITMARKER_OBEYS;
                return;
            }
            _ => {
                gMoveResultFlags |= MOVE_RESULT_MISSED;
                return;
            }
        }
    }
    gHitMarker |= HITMARKER_OBEYS;
    if gProtectStructs[gBattlerTarget].bounceMove() != 0
        && gBattleMoves[gCurrentMove].flags as i32 & FLAG_MAGIC_COAT_AFFECTED != 0
    {
        PressurePPLose(gBattlerAttacker, gBattlerTarget, MOVE_MAGIC_COAT);
        gProtectStructs[gBattlerTarget].set_bounceMove(FALSE as u32);
        BattleScriptPushCursor();
        gBattlescriptCurrInstr = BattleScript_MagicCoatBounce.as_ptr().cast_mut();
        return;
    }
    i = 0;
    while i < gBattlersCount as i32 {
        if gProtectStructs[gBattlerByTurnOrder[i]].stealMove() != 0
            && gBattleMoves[gCurrentMove].flags as i32 & FLAG_SNATCH_AFFECTED != 0
        {
            PressurePPLose(gBattlerAttacker, gBattlerByTurnOrder[i], MOVE_SNATCH);
            gProtectStructs[gBattlerByTurnOrder[i]].set_stealMove(FALSE as u32);
            gBattleScripting.battler = gBattlerByTurnOrder[i];
            BattleScriptPushCursor();
            gBattlescriptCurrInstr = BattleScript_SnatchedMove.as_ptr().cast_mut();
            return;
        }
        i += 1;
    }
    if gSpecialStatuses[gBattlerTarget].lightningRodRedirected() != 0 {
        gSpecialStatuses[gBattlerTarget].set_lightningRodRedirected(FALSE as u32);
        gLastUsedAbility = ABILITY_LIGHTNING_ROD;
        BattleScriptPushCursor();
        gBattlescriptCurrInstr = BattleScript_TookAttack.as_ptr().cast_mut();
        RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
    } else if gProtectStructs[gBattlerTarget].protected() != 0
        && gBattleMoves[gCurrentMove].flags as i32 & 2 != 0
        && (gCurrentMove != MOVE_CURSE
            || (gBattleMons[gBattlerAttacker].types[0] == TYPE_GHOST
                || gBattleMons[gBattlerAttacker].types[1] == TYPE_GHOST))
        && (IsTwoTurnsMove(gCurrentMove) == 0
            || gBattleMons[gBattlerAttacker].status2 & STATUS2_MULTIPLETURNS != 0)
    {
        CancelMultiTurnMoves(gBattlerAttacker);
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gLastLandedMoves[gBattlerTarget] = 0;
        gLastHitByType[gBattlerTarget] = 0;
        gBattleCommunication[6] = B_MSG_PROTECTED;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn JumpIfMoveFailed(adder: u8, r#move: u16) {
    let mut BS_ptr: *mut u8 = gBattlescriptCurrInstr.at(adder);
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        gLastLandedMoves[gBattlerTarget] = 0;
        gLastHitByType[gBattlerTarget] = 0;
        BS_ptr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24) as usize
            as *mut u8;
    } else {
        TrySetDestinyBondToHappen();
        if AbilityBattleEffects(ABILITYEFFECT_ABSORBING, gBattlerTarget, 0, 0, r#move) != 0 {
            return;
        }
    }
    gBattlescriptCurrInstr = BS_ptr;
}
pub(crate) unsafe extern "C" fn Cmd_jumpifaffectedbyprotect() {
    if gProtectStructs[gBattlerTarget].protected() != 0
        && gBattleMoves[gCurrentMove].flags as i32 & 2 != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        JumpIfMoveFailed(5, 0);
        gBattleCommunication[6] = B_MSG_PROTECTED;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn JumpIfMoveAffectedByProtect(r#move: u16) -> u8 {
    let mut affected: u8 = FALSE;
    if gProtectStructs[gBattlerTarget].protected() != 0
        && gBattleMoves[gCurrentMove].flags as i32 & 2 != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        JumpIfMoveFailed(7, r#move);
        gBattleCommunication[6] = B_MSG_PROTECTED;
        affected = TRUE;
    }
    return affected;
}
pub(crate) unsafe extern "C" fn AccuracyCalcHelper(r#move: u16) -> u8 {
    if gStatuses3[gBattlerTarget] & STATUS3_ALWAYS_HITS != 0
        && gDisableStructs[gBattlerTarget].battlerWithSureHit == gBattlerAttacker
    {
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    if gHitMarker & HITMARKER_IGNORE_ON_AIR == 0 && gStatuses3[gBattlerTarget] & STATUS3_ON_AIR != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    gHitMarker &= 0xfffeffff;
    if gHitMarker & HITMARKER_IGNORE_UNDERGROUND == 0
        && gStatuses3[gBattlerTarget] & STATUS3_UNDERGROUND != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    gHitMarker &= 0xfffdffff;
    if gHitMarker & 0x40000 == 0 && gStatuses3[gBattlerTarget] & 0x40000 != 0 {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    gHitMarker &= 0xfffbffff;
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
        && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
        && gBattleWeather as i32 & B_WEATHER_RAIN != 0
        && gBattleMoves[r#move].effect == EFFECT_THUNDER
        || (gBattleMoves[r#move].effect == EFFECT_ALWAYS_HIT
            || gBattleMoves[r#move].effect == EFFECT_VITAL_THROW)
    {
        JumpIfMoveFailed(7, r#move);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Cmd_accuracycheck() {
    let mut r#move: u16 =
        *gBattlescriptCurrInstr.at(5) as u16 + ((*gBattlescriptCurrInstr.at(5).at(1) as u16) << 8);
    if r#move == NO_ACC_CALC || r#move == NO_ACC_CALC_CHECK_LOCK_ON {
        if gStatuses3[gBattlerTarget] & STATUS3_ALWAYS_HITS != 0
            && r#move == NO_ACC_CALC_CHECK_LOCK_ON
            && gDisableStructs[gBattlerTarget].battlerWithSureHit == gBattlerAttacker
        {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        } else if gStatuses3[gBattlerTarget] & 0x400c0 != 0 {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        } else if JumpIfMoveAffectedByProtect(0) == 0 {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        }
    } else {
        let mut r#type: u8 = 0;
        let mut moveAcc: u8 = 0;
        let mut holdEffect: u8 = 0;
        let mut param: u8 = 0;
        let mut buff: i8 = 0;
        let mut calc: u16 = 0;
        if r#move == ACC_CURR_MOVE {
            r#move = gCurrentMove;
        }
        if (*gBattleStruct).dynamicMoveType != 0 {
            r#type = (*gBattleStruct).dynamicMoveType & 63;
        } else {
            r#type = gBattleMoves[r#move].r#type;
        }
        if JumpIfMoveAffectedByProtect(r#move) != 0 {
            return;
        }
        if AccuracyCalcHelper(r#move) != 0 {
            return;
        }
        if gBattleMons[gBattlerTarget].status2 & STATUS2_FORESIGHT != 0 {
            let mut acc: u8 = gBattleMons[gBattlerAttacker].statStages[6] as u8;
            buff = acc as i8;
        } else {
            let mut acc: u8 = gBattleMons[gBattlerAttacker].statStages[6] as u8;
            buff = acc as i8 + DEFAULT_STAT_STAGE - gBattleMons[gBattlerTarget].statStages[7];
        }
        if buff < MIN_STAT_STAGE {
            buff = MIN_STAT_STAGE;
        }
        if buff > MAX_STAT_STAGE {
            buff = MAX_STAT_STAGE;
        }
        moveAcc = gBattleMoves[r#move].accuracy;
        if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
            && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
            && gBattleWeather as i32 & B_WEATHER_SUN != 0
            && gBattleMoves[r#move].effect == EFFECT_THUNDER
        {
            moveAcc = 50;
        }
        calc = sAccuracyStageRatios[buff].dividend as u16 * moveAcc as u16;
        calc = div_i32(calc as i32, sAccuracyStageRatios[buff].divisor as i32) as u16;
        if gBattleMons[gBattlerAttacker].ability == ABILITY_COMPOUND_EYES {
            calc = (calc as i32 * 130 / 100) as u16;
        }
        if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
            && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
            && gBattleMons[gBattlerTarget].ability == ABILITY_SAND_VEIL
            && gBattleWeather as i32 & B_WEATHER_SANDSTORM != 0
        {
            calc = (calc as i32 * 80 / 100) as u16;
        }
        if gBattleMons[gBattlerAttacker].ability == ABILITY_HUSTLE && r#type < 9 {
            calc = (calc as i32 * 80 / 100) as u16;
        }
        if gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY {
            holdEffect = gEnigmaBerries[gBattlerTarget].holdEffect;
            param = gEnigmaBerries[gBattlerTarget].holdEffectParam;
        } else {
            holdEffect = GetItemHoldEffect(gBattleMons[gBattlerTarget].item);
            param = GetItemHoldEffectParam(gBattleMons[gBattlerTarget].item);
        }
        gPotentialItemEffectBattler = gBattlerTarget;
        if holdEffect == HOLD_EFFECT_EVASION_UP {
            calc = (calc as i32 * (100 - param as i32) / 100) as u16;
        }
        if Random() as i32 % 100 + 1 > calc as i32 {
            gMoveResultFlags |= MOVE_RESULT_MISSED;
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
                && (gBattleMoves[r#move].target == MOVE_TARGET_BOTH
                    || gBattleMoves[r#move].target == MOVE_TARGET_FOES_AND_ALLY)
            {
                gBattleCommunication[6] = B_MSG_AVOIDED_ATK;
            } else {
                gBattleCommunication[6] = B_MSG_MISSED;
            }
            CheckWonderGuardAndLevitate();
        }
        JumpIfMoveFailed(7, r#move);
    }
}
pub(crate) unsafe extern "C" fn Cmd_attackstring() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gHitMarker & 1536 == 0 {
        PrepareStringBattle(STRINGID_USEDMOVE, gBattlerAttacker);
        gHitMarker |= HITMARKER_ATTACKSTRING_PRINTED;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    gBattleCommunication[7] = 0;
}
pub(crate) unsafe extern "C" fn Cmd_ppreduce() {
    let mut ppToDeduct: i32 = 1;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gSpecialStatuses[gBattlerAttacker].ppNotAffectedByPressure() == 0 {
        match gBattleMoves[gCurrentMove].target {
            MOVE_TARGET_FOES_AND_ALLY => {
                ppToDeduct += AbilityBattleEffects(
                    ABILITYEFFECT_COUNT_ON_FIELD,
                    gBattlerAttacker,
                    ABILITY_PRESSURE,
                    0,
                    0,
                ) as i32;
            }
            MOVE_TARGET_BOTH | MOVE_TARGET_OPPONENTS_FIELD => {
                ppToDeduct += AbilityBattleEffects(
                    ABILITYEFFECT_COUNT_OTHER_SIDE,
                    gBattlerAttacker,
                    ABILITY_PRESSURE,
                    0,
                    0,
                ) as i32;
            }
            _ => {
                if gBattlerAttacker != gBattlerTarget
                    && gBattleMons[gBattlerTarget].ability == ABILITY_PRESSURE
                {
                    ppToDeduct += 1;
                }
            }
        }
    }
    if gHitMarker & 2560 == 0 && gBattleMons[gBattlerAttacker].pp[gCurrMovePos] != 0 {
        gProtectStructs[gBattlerAttacker].set_notFirstStrike(1);
        if gBattleMons[gBattlerAttacker].pp[gCurrMovePos] as i32 > ppToDeduct {
            gBattleMons[gBattlerAttacker].pp[gCurrMovePos] -= ppToDeduct as u8;
        } else {
            gBattleMons[gBattlerAttacker].pp[gCurrMovePos] = 0;
        }
        if gBattleMons[gBattlerAttacker].status2 & 0x200000 == 0
            && gDisableStructs[gBattlerAttacker].mimickedMoves() as u32 & gBitTable[gCurrMovePos]
                == 0
        {
            gActiveBattler = gBattlerAttacker;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_PPMOVE1_BATTLE + gCurrMovePos,
                0,
                1,
                &raw mut gBattleMons[gBattlerAttacker].pp[gCurrMovePos] as *mut c_void,
            );
            MarkBattlerForControllerExec(gBattlerAttacker);
        }
    }
    gHitMarker &= 0xfffff7ff;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_critcalc() {
    let mut holdEffect: u8 = 0;
    let mut item: u16 = 0;
    let mut critChance: u16 = 0;
    item = gBattleMons[gBattlerAttacker].item;
    if item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gBattlerAttacker].holdEffect;
    } else {
        holdEffect = GetItemHoldEffect(item);
    }
    gPotentialItemEffectBattler = gBattlerAttacker;
    critChance = 2 * (gBattleMons[gBattlerAttacker].status2 & STATUS2_FOCUS_ENERGY != 0) as u16
        + (gBattleMoves[gCurrentMove].effect == EFFECT_HIGH_CRITICAL) as u16
        + (gBattleMoves[gCurrentMove].effect == EFFECT_SKY_ATTACK) as u16
        + (gBattleMoves[gCurrentMove].effect == EFFECT_BLAZE_KICK) as u16
        + (gBattleMoves[gCurrentMove].effect == EFFECT_POISON_TAIL) as u16
        + (holdEffect == HOLD_EFFECT_SCOPE_LENS) as u16
        + 2 * (holdEffect == HOLD_EFFECT_LUCKY_PUNCH
            && gBattleMons[gBattlerAttacker].species == SPECIES_CHANSEY) as u16
        + 2 * (holdEffect == HOLD_EFFECT_STICK
            && gBattleMons[gBattlerAttacker].species == SPECIES_FARFETCHD) as u16;
    if critChance >= 5 {
        critChance = 4;
    }
    if gBattleMons[gBattlerTarget].ability != ABILITY_BATTLE_ARMOR
        && gBattleMons[gBattlerTarget].ability != ABILITY_SHELL_ARMOR
        && gStatuses3[gBattlerAttacker] & STATUS3_CANT_SCORE_A_CRIT == 0
        && gBattleTypeFlags & 528 == 0
        && rem_i32(Random() as i32, sCriticalHitChance[critChance] as i32) == 0
    {
        gCritMultiplier = 2;
    } else {
        gCritMultiplier = 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_damagecalc() {
    let mut sideStatus: u16 = gSideStatuses[GetBattlerPosition(gBattlerTarget) as i32 & 1];
    gBattleMoveDamage = CalculateBaseDamage(
        &raw mut gBattleMons[gBattlerAttacker],
        &raw mut gBattleMons[gBattlerTarget],
        gCurrentMove as u32,
        sideStatus,
        gDynamicBasePower,
        (*gBattleStruct).dynamicMoveType,
        gBattlerAttacker,
        gBattlerTarget,
    );
    gBattleMoveDamage =
        gBattleMoveDamage * gCritMultiplier as i32 * gBattleScripting.dmgMultiplier as i32;
    if gStatuses3[gBattlerAttacker] & STATUS3_CHARGED_UP != 0
        && gBattleMoves[gCurrentMove].r#type == TYPE_ELECTRIC
    {
        gBattleMoveDamage *= 2;
    }
    if gProtectStructs[gBattlerAttacker].helpingHand() != 0 {
        gBattleMoveDamage = gBattleMoveDamage * 15 / 10;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AI_CalcDmg(attacker: u8, defender: u8) {
    let mut sideStatus: u16 = gSideStatuses[GetBattlerPosition(defender) as i32 & 1];
    gBattleMoveDamage = CalculateBaseDamage(
        &raw mut gBattleMons[attacker],
        &raw mut gBattleMons[defender],
        gCurrentMove as u32,
        sideStatus,
        gDynamicBasePower,
        (*gBattleStruct).dynamicMoveType,
        attacker,
        defender,
    );
    gDynamicBasePower = 0;
    gBattleMoveDamage =
        gBattleMoveDamage * gCritMultiplier as i32 * gBattleScripting.dmgMultiplier as i32;
    if gStatuses3[attacker] & STATUS3_CHARGED_UP != 0
        && gBattleMoves[gCurrentMove].r#type == TYPE_ELECTRIC
    {
        gBattleMoveDamage *= 2;
    }
    if gProtectStructs[attacker].helpingHand() != 0 {
        gBattleMoveDamage = gBattleMoveDamage * 15 / 10;
    }
}
pub(crate) unsafe extern "C" fn ModulateDmgByType(multiplier: u8) {
    gBattleMoveDamage = gBattleMoveDamage * multiplier as i32 / 10;
    if gBattleMoveDamage == 0 && multiplier != 0 {
        gBattleMoveDamage = 1;
    }
    match multiplier {
        TYPE_MUL_NO_EFFECT => {
            gMoveResultFlags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
            gMoveResultFlags &= 251;
            gMoveResultFlags &= 253;
        }
        TYPE_MUL_NOT_EFFECTIVE => {
            if gBattleMoves[gCurrentMove].power != 0
                && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
            {
                if gMoveResultFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    gMoveResultFlags &= 253;
                } else {
                    gMoveResultFlags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                }
            }
        }
        TYPE_MUL_SUPER_EFFECTIVE => {
            if gBattleMoves[gCurrentMove].power != 0
                && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
            {
                if gMoveResultFlags as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
                    gMoveResultFlags &= 251;
                } else {
                    gMoveResultFlags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_typecalc() {
    let mut i: i32 = 0;
    let mut moveType: u8 = 0;
    if gCurrentMove == MOVE_STRUGGLE {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        return;
    }
    if (*gBattleStruct).dynamicMoveType != 0 {
        moveType = (*gBattleStruct).dynamicMoveType & 63;
    } else {
        moveType = gBattleMoves[gCurrentMove].r#type;
    }
    if gBattleMons[gBattlerAttacker].types[0] == moveType
        || gBattleMons[gBattlerAttacker].types[1] == moveType
    {
        gBattleMoveDamage = gBattleMoveDamage * 15;
        gBattleMoveDamage = gBattleMoveDamage / 10;
    }
    if gBattleMons[gBattlerTarget].ability == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        gLastUsedAbility = gBattleMons[gBattlerTarget].ability;
        gMoveResultFlags |= 9;
        gLastLandedMoves[gBattlerTarget] = 0;
        gLastHitByType[gBattlerTarget] = 0;
        gBattleCommunication[6] = B_MSG_GROUND_MISS;
        RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
    } else {
        while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
            if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
                if gBattleMons[gBattlerTarget].status2 & STATUS2_FORESIGHT != 0 {
                    break;
                }
                i += 3;
                continue;
            } else if gTypeEffectiveness[i + 0] == moveType {
                if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[0] {
                    ModulateDmgByType(gTypeEffectiveness[i + 2]);
                }
                if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1]
                    && gBattleMons[gBattlerTarget].types[0] != gBattleMons[gBattlerTarget].types[1]
                {
                    ModulateDmgByType(gTypeEffectiveness[i + 2]);
                }
            }
            i += 3;
        }
    }
    if gBattleMons[gBattlerTarget].ability == ABILITY_WONDER_GUARD
        && AttacksThisTurn(gBattlerAttacker, gCurrentMove) == 2
        && (gMoveResultFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0
            || gMoveResultFlags as i32 & 6 == 6)
        && gBattleMoves[gCurrentMove].power != 0
    {
        gLastUsedAbility = ABILITY_WONDER_GUARD;
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gLastLandedMoves[gBattlerTarget] = 0;
        gLastHitByType[gBattlerTarget] = 0;
        gBattleCommunication[6] = B_MSG_AVOIDED_DMG;
        RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
    }
    if gMoveResultFlags as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
        gProtectStructs[gBattlerAttacker].set_targetNotAffected(1);
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn CheckWonderGuardAndLevitate() {
    let mut flags: u8 = 0;
    let mut i: i32 = 0;
    let mut moveType: u8 = 0;
    if gCurrentMove == MOVE_STRUGGLE || gBattleMoves[gCurrentMove].power == 0 {
        return;
    }
    if (*gBattleStruct).dynamicMoveType != 0 {
        moveType = (*gBattleStruct).dynamicMoveType & 63;
    } else {
        moveType = gBattleMoves[gCurrentMove].r#type;
    }
    if gBattleMons[gBattlerTarget].ability == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        gLastUsedAbility = ABILITY_LEVITATE;
        gBattleCommunication[6] = B_MSG_GROUND_MISS;
        RecordAbilityBattle(gBattlerTarget, ABILITY_LEVITATE);
        return;
    }
    while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
        if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
            if gBattleMons[gBattlerTarget].status2 & STATUS2_FORESIGHT != 0 {
                break;
            }
            i += 3;
            continue;
        }
        if gTypeEffectiveness[i + 0] == moveType {
            if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[0]
                && gTypeEffectiveness[i + 2] == TYPE_MUL_NO_EFFECT
            {
                gMoveResultFlags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
                gProtectStructs[gBattlerAttacker].set_targetNotAffected(1);
            }
            if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1]
                && gBattleMons[gBattlerTarget].types[0] != gBattleMons[gBattlerTarget].types[1]
                && gTypeEffectiveness[i + 2] == TYPE_MUL_NO_EFFECT
            {
                gMoveResultFlags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
                gProtectStructs[gBattlerAttacker].set_targetNotAffected(1);
            }
            if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[0]
                && gTypeEffectiveness[i + 2] == 20
            {
                flags |= 1;
            }
            if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1]
                && gBattleMons[gBattlerTarget].types[0] != gBattleMons[gBattlerTarget].types[1]
                && gTypeEffectiveness[i + 2] == TYPE_MUL_SUPER_EFFECTIVE
            {
                flags |= 1;
            }
            if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[0]
                && gTypeEffectiveness[i + 2] == 5
            {
                flags |= 2;
            }
            if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1]
                && gBattleMons[gBattlerTarget].types[0] != gBattleMons[gBattlerTarget].types[1]
                && gTypeEffectiveness[i + 2] == TYPE_MUL_NOT_EFFECTIVE
            {
                flags |= 2;
            }
        }
        i += 3;
    }
    if gBattleMons[gBattlerTarget].ability == ABILITY_WONDER_GUARD
        && AttacksThisTurn(gBattlerAttacker, gCurrentMove) == 2
    {
        if (flags as i32 & 2 != 0 || flags as i32 & 1 == 0) && gBattleMoves[gCurrentMove].power != 0
        {
            gLastUsedAbility = ABILITY_WONDER_GUARD;
            gBattleCommunication[6] = B_MSG_AVOIDED_DMG;
            RecordAbilityBattle(gBattlerTarget, ABILITY_WONDER_GUARD);
        }
    }
}
pub(crate) unsafe extern "C" fn ModulateDmgByType2(multiplier: u8, r#move: u16, flags: *mut u8) {
    gBattleMoveDamage = gBattleMoveDamage * multiplier as i32 / 10;
    if gBattleMoveDamage == 0 && multiplier != 0 {
        gBattleMoveDamage = 1;
    }
    match multiplier {
        TYPE_MUL_NO_EFFECT => {
            *flags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
            *flags &= 251;
            *flags &= 253;
        }
        TYPE_MUL_NOT_EFFECTIVE => {
            if gBattleMoves[r#move].power != 0 && *flags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
                if *flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    *flags &= 253;
                } else {
                    *flags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                }
            }
        }
        TYPE_MUL_SUPER_EFFECTIVE => {
            if gBattleMoves[r#move].power != 0 && *flags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
                if *flags as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
                    *flags &= 251;
                } else {
                    *flags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TypeCalc(r#move: u16, attacker: u8, defender: u8) -> u8 {
    let mut i: i32 = 0;
    let mut flags: u8 = 0;
    let mut moveType: u8 = 0;
    if r#move == MOVE_STRUGGLE {
        return 0;
    }
    moveType = gBattleMoves[r#move].r#type;
    if gBattleMons[attacker].types[0] == moveType || gBattleMons[attacker].types[1] == moveType {
        gBattleMoveDamage = gBattleMoveDamage * 15;
        gBattleMoveDamage = gBattleMoveDamage / 10;
    }
    if gBattleMons[defender].ability == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        flags |= 9;
    } else {
        while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
            if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
                if gBattleMons[defender].status2 & STATUS2_FORESIGHT != 0 {
                    break;
                }
                i += 3;
                continue;
            } else if gTypeEffectiveness[i + 0] == moveType {
                if gTypeEffectiveness[i + 1] == gBattleMons[defender].types[0] {
                    ModulateDmgByType2(gTypeEffectiveness[i + 2], r#move, &raw mut flags);
                }
                if gTypeEffectiveness[i + 1] == gBattleMons[defender].types[1]
                    && gBattleMons[defender].types[0] != gBattleMons[defender].types[1]
                {
                    ModulateDmgByType2(gTypeEffectiveness[i + 2], r#move, &raw mut flags);
                }
            }
            i += 3;
        }
    }
    if gBattleMons[defender].ability == ABILITY_WONDER_GUARD
        && flags as i32 & MOVE_RESULT_MISSED as i32 == 0
        && AttacksThisTurn(attacker, r#move) == 2
        && (flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0 || flags as i32 & 6 == 6)
        && gBattleMoves[r#move].power != 0
    {
        flags |= MOVE_RESULT_MISSED;
    }
    return flags;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AI_TypeCalc(r#move: u16, targetSpecies: u16, targetAbility: u8) -> u8 {
    let mut i: i32 = 0;
    let mut flags: u8 = 0;
    let mut type1: u8 = gSpeciesInfo[targetSpecies].types[0];
    let mut type2: u8 = gSpeciesInfo[targetSpecies].types[1];
    let mut moveType: u8 = 0;
    if r#move == MOVE_STRUGGLE {
        return 0;
    }
    moveType = gBattleMoves[r#move].r#type;
    if targetAbility == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        flags = 9;
    } else {
        while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
            if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
                i += 3;
                continue;
            }
            if gTypeEffectiveness[i + 0] == moveType {
                if gTypeEffectiveness[i + 1] == type1 {
                    ModulateDmgByType2(gTypeEffectiveness[i + 2], r#move, &raw mut flags);
                }
                if gTypeEffectiveness[i + 1] == type2 && type1 != type2 {
                    ModulateDmgByType2(gTypeEffectiveness[i + 2], r#move, &raw mut flags);
                }
            }
            i += 3;
        }
    }
    if targetAbility == ABILITY_WONDER_GUARD
        && (flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0 || flags as i32 & 6 == 6)
        && gBattleMoves[r#move].power != 0
    {
        flags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
    }
    return flags;
}
pub(crate) unsafe extern "C" fn ApplyRandomDmgMultiplier() {
    let mut rand: u16 = Random();
    let mut randPercent: u16 = 100 - (rand as i32 % 16) as u16;
    if gBattleMoveDamage != 0 {
        gBattleMoveDamage *= randPercent as i32;
        gBattleMoveDamage = gBattleMoveDamage / 100;
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
    }
}
pub(crate) unsafe extern "C" fn Unused_ApplyRandomDmgMultiplier() {
    ApplyRandomDmgMultiplier();
}
pub(crate) unsafe extern "C" fn Cmd_adjustnormaldamage() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    ApplyRandomDmgMultiplier();
    if gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gBattlerTarget].holdEffect;
        param = gEnigmaBerries[gBattlerTarget].holdEffectParam;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[gBattlerTarget].item);
        param = GetItemHoldEffectParam(gBattleMons[gBattlerTarget].item);
    }
    gPotentialItemEffectBattler = gBattlerTarget;
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget, holdEffect);
        gSpecialStatuses[gBattlerTarget].set_focusBanded(1);
    }
    if gBattleMons[gBattlerTarget].status2 & STATUS2_SUBSTITUTE == 0
        && (gBattleMoves[gCurrentMove].effect == EFFECT_FALSE_SWIPE
            || gProtectStructs[gBattlerTarget].endured() != 0
            || gSpecialStatuses[gBattlerTarget].focusBanded() != 0)
        && gBattleMons[gBattlerTarget].hp as i32 <= gBattleMoveDamage
    {
        gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 - 1;
        if gProtectStructs[gBattlerTarget].endured() != 0 {
            gMoveResultFlags |= MOVE_RESULT_FOE_ENDURED;
        } else if gSpecialStatuses[gBattlerTarget].focusBanded() != 0 {
            gMoveResultFlags |= MOVE_RESULT_FOE_HUNG_ON;
            gLastUsedItem = gBattleMons[gBattlerTarget].item;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_adjustnormaldamage2() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    ApplyRandomDmgMultiplier();
    if gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gBattlerTarget].holdEffect;
        param = gEnigmaBerries[gBattlerTarget].holdEffectParam;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[gBattlerTarget].item);
        param = GetItemHoldEffectParam(gBattleMons[gBattlerTarget].item);
    }
    gPotentialItemEffectBattler = gBattlerTarget;
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget, holdEffect);
        gSpecialStatuses[gBattlerTarget].set_focusBanded(1);
    }
    if gBattleMons[gBattlerTarget].status2 & STATUS2_SUBSTITUTE == 0
        && (gProtectStructs[gBattlerTarget].endured() != 0
            || gSpecialStatuses[gBattlerTarget].focusBanded() != 0)
        && gBattleMons[gBattlerTarget].hp as i32 <= gBattleMoveDamage
    {
        gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 - 1;
        if gProtectStructs[gBattlerTarget].endured() != 0 {
            gMoveResultFlags |= MOVE_RESULT_FOE_ENDURED;
        } else if gSpecialStatuses[gBattlerTarget].focusBanded() != 0 {
            gMoveResultFlags |= MOVE_RESULT_FOE_HUNG_ON;
            gLastUsedItem = gBattleMons[gBattlerTarget].item;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_attackanimation() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gHitMarker & HITMARKER_NO_ANIMATIONS != 0
        && (gCurrentMove != MOVE_TRANSFORM && gCurrentMove != MOVE_SUBSTITUTE)
    {
        BattleScriptPush(gBattlescriptCurrInstr.at(1));
        gBattlescriptCurrInstr = BattleScript_Pausex20.as_ptr().cast_mut();
        gBattleScripting.animTurn += 1;
        gBattleScripting.animTargetsHit += 1;
    } else {
        if (gBattleMoves[gCurrentMove].target as i32 & MOVE_TARGET_BOTH as i32 != 0
            || gBattleMoves[gCurrentMove].target as i32 & MOVE_TARGET_FOES_AND_ALLY as i32 != 0
            || gBattleMoves[gCurrentMove].target as i32 & MOVE_TARGET_DEPENDS as i32 != 0)
            && gBattleScripting.animTargetsHit != 0
        {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
            return;
        }
        if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
            let mut multihit: u8 = 0;
            gActiveBattler = gBattlerAttacker;
            if gBattleMons[gBattlerTarget].status2 & STATUS2_SUBSTITUTE != 0 {
                multihit = gMultiHitCounter;
            } else if gMultiHitCounter != 0 && gMultiHitCounter != 1 {
                if gBattleMons[gBattlerTarget].hp as i32 <= gBattleMoveDamage {
                    multihit = 1;
                } else {
                    multihit = gMultiHitCounter;
                }
            } else {
                multihit = gMultiHitCounter;
            }
            BtlController_EmitMoveAnimation(
                B_COMM_TO_CONTROLLER,
                gCurrentMove,
                gBattleScripting.animTurn,
                gBattleMovePower,
                gBattleMoveDamage,
                gBattleMons[gBattlerAttacker].friendship,
                &raw mut gDisableStructs[gBattlerAttacker],
                multihit,
            );
            gBattleScripting.animTurn += 1;
            gBattleScripting.animTargetsHit += 1;
            MarkBattlerForControllerExec(gBattlerAttacker);
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        } else {
            BattleScriptPush(gBattlescriptCurrInstr.at(1));
            gBattlescriptCurrInstr = BattleScript_Pausex20.as_ptr().cast_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitanimation() {
    if gBattleControllerExecFlags == 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_healthbarupdate() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gBattleMons[gActiveBattler].status2 & STATUS2_SUBSTITUTE != 0
            && gDisableStructs[gActiveBattler].substituteHP != 0
            && gHitMarker & HITMARKER_IGNORE_SUBSTITUTE == 0
        {
            PrepareStringBattle(STRINGID_SUBSTITUTEDAMAGED, gActiveBattler);
        } else {
            let mut healthValue: i16 = 0;
            let mut currDmg: i32 = gBattleMoveDamage;
            let mut maxPossibleDmgValue: i32 = 10000;
            if currDmg <= maxPossibleDmgValue {
                healthValue = currDmg as i16;
            } else {
                healthValue = maxPossibleDmgValue as i16;
            }
            BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, healthValue as u16);
            MarkBattlerForControllerExec(gActiveBattler);
            if GetBattlerSide(gActiveBattler) == 0 && gBattleMoveDamage > 0 {
                gBattleResults.set_playerMonWasDamaged(TRUE);
            }
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_datahpupdate() {
    let mut moveType: u32 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if (*gBattleStruct).dynamicMoveType == 0 {
        moveType = gBattleMoves[gCurrentMove].r#type as u32;
    } else if (*gBattleStruct).dynamicMoveType as i32 & F_DYNAMIC_TYPE_IGNORE_PHYSICALITY == 0 {
        moveType = (*gBattleStruct).dynamicMoveType as u32 & DYNAMIC_TYPE_MASK;
    } else {
        moveType = gBattleMoves[gCurrentMove].r#type as u32;
    }
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gBattleMons[gActiveBattler].status2 & STATUS2_SUBSTITUTE != 0
            && gDisableStructs[gActiveBattler].substituteHP != 0
            && gHitMarker & HITMARKER_IGNORE_SUBSTITUTE == 0
        {
            if gDisableStructs[gActiveBattler].substituteHP as i32 >= gBattleMoveDamage {
                if gSpecialStatuses[gActiveBattler].shellBellDmg == 0 {
                    gSpecialStatuses[gActiveBattler].shellBellDmg = gBattleMoveDamage;
                }
                gDisableStructs[gActiveBattler].substituteHP -= gBattleMoveDamage as u8;
                gHpDealt = gBattleMoveDamage;
            } else {
                if gSpecialStatuses[gActiveBattler].shellBellDmg == 0 {
                    gSpecialStatuses[gActiveBattler].shellBellDmg =
                        gDisableStructs[gActiveBattler].substituteHP as i32;
                }
                gHpDealt = gDisableStructs[gActiveBattler].substituteHP as i32;
                gDisableStructs[gActiveBattler].substituteHP = 0;
            }
            if gDisableStructs[gActiveBattler].substituteHP == 0 {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
                BattleScriptPushCursor();
                gBattlescriptCurrInstr = BattleScript_SubstituteFade.as_ptr().cast_mut();
                return;
            }
        } else {
            gHitMarker &= 0xfffffeff;
            if gBattleMoveDamage < 0 {
                gBattleMons[gActiveBattler].hp += (gBattleMoveDamage as u16).wrapping_neg();
                if gBattleMons[gActiveBattler].hp > gBattleMons[gActiveBattler].maxHP {
                    gBattleMons[gActiveBattler].hp = gBattleMons[gActiveBattler].maxHP;
                }
            } else {
                if gHitMarker & HITMARKER_IGNORE_BIDE != 0 {
                    gHitMarker &= 0xffffffdf;
                } else {
                    gBideDmg[gActiveBattler] += gBattleMoveDamage;
                    if *gBattlescriptCurrInstr.at(1) == BS_TARGET {
                        gBideTarget[gActiveBattler] = gBattlerAttacker;
                    } else {
                        gBideTarget[gActiveBattler] = gBattlerTarget;
                    }
                }
                if gBattleMons[gActiveBattler].hp as i32 > gBattleMoveDamage {
                    gBattleMons[gActiveBattler].hp -= gBattleMoveDamage as u16;
                    gHpDealt = gBattleMoveDamage;
                } else {
                    gHpDealt = gBattleMons[gActiveBattler].hp as i32;
                    gBattleMons[gActiveBattler].hp = 0;
                }
                if gSpecialStatuses[gActiveBattler].shellBellDmg == 0
                    && gHitMarker & HITMARKER_PASSIVE_HP_UPDATE == 0
                {
                    gSpecialStatuses[gActiveBattler].shellBellDmg = gHpDealt;
                }
                if moveType < 9
                    && gHitMarker & HITMARKER_PASSIVE_HP_UPDATE == 0
                    && gCurrentMove != MOVE_PAIN_SPLIT
                {
                    gProtectStructs[gActiveBattler].physicalDmg = gHpDealt as u32;
                    gSpecialStatuses[gActiveBattler].physicalDmg = gHpDealt;
                    if *gBattlescriptCurrInstr.at(1) == BS_TARGET {
                        gProtectStructs[gActiveBattler].physicalBattlerId = gBattlerAttacker;
                        gSpecialStatuses[gActiveBattler].physicalBattlerId = gBattlerAttacker;
                    } else {
                        gProtectStructs[gActiveBattler].physicalBattlerId = gBattlerTarget;
                        gSpecialStatuses[gActiveBattler].physicalBattlerId = gBattlerTarget;
                    }
                } else if moveType >= 9 && gHitMarker & HITMARKER_PASSIVE_HP_UPDATE == 0 {
                    gProtectStructs[gActiveBattler].specialDmg = gHpDealt as u32;
                    gSpecialStatuses[gActiveBattler].specialDmg = gHpDealt;
                    if *gBattlescriptCurrInstr.at(1) == BS_TARGET {
                        gProtectStructs[gActiveBattler].specialBattlerId = gBattlerAttacker;
                        gSpecialStatuses[gActiveBattler].specialBattlerId = gBattlerAttacker;
                    } else {
                        gProtectStructs[gActiveBattler].specialBattlerId = gBattlerTarget;
                        gSpecialStatuses[gActiveBattler].specialBattlerId = gBattlerTarget;
                    }
                }
            }
            gHitMarker &= 0xffefffff;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_HP_BATTLE,
                0,
                2,
                &raw mut gBattleMons[gActiveBattler].hp as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
    } else {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gSpecialStatuses[gActiveBattler].shellBellDmg == 0 {
            gSpecialStatuses[gActiveBattler].shellBellDmg = IGNORE_SHELL_BELL;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_critmessage() {
    if gBattleControllerExecFlags == 0 {
        if gCritMultiplier == 2 && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0 {
            PrepareStringBattle(STRINGID_CRITICALHIT, gBattlerAttacker);
            gBattleCommunication[7] = 1;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_effectivenesssound() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = gBattlerTarget;
    if gMoveResultFlags as i32 & MOVE_RESULT_MISSED as i32 == 0 {
        match gMoveResultFlags as i32 & 254 {
            MOVE_RESULT_SUPER_EFFECTIVE => {
                BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_SUPER_EFFECTIVE);
                MarkBattlerForControllerExec(gActiveBattler);
            }
            MOVE_RESULT_NOT_VERY_EFFECTIVE => {
                BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_NOT_EFFECTIVE);
                MarkBattlerForControllerExec(gActiveBattler);
            }
            8 | MOVE_RESULT_FAILED => {}
            _ => {
                if gMoveResultFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
                    BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_SUPER_EFFECTIVE);
                    MarkBattlerForControllerExec(gActiveBattler);
                } else if gMoveResultFlags as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
                    BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_NOT_EFFECTIVE);
                    MarkBattlerForControllerExec(gActiveBattler);
                } else if gMoveResultFlags as i32 & 40 == 0 {
                    BtlController_EmitPlaySE(B_COMM_TO_CONTROLLER, SE_EFFECTIVE);
                    MarkBattlerForControllerExec(gActiveBattler);
                }
            }
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_resultmessage() {
    let mut stringId: u32 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gMoveResultFlags as i32 & MOVE_RESULT_MISSED as i32 != 0
        && (gMoveResultFlags as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 == 0
            || gBattleCommunication[6] > B_MSG_AVOIDED_ATK)
    {
        stringId = gMissStringIds[gBattleCommunication[6]] as u32;
        gBattleCommunication[7] = 1;
    } else {
        gBattleCommunication[7] = 1;
        match gMoveResultFlags as i32 & 254 {
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
                gLastUsedItem = gBattleMons[gBattlerTarget].item;
                gPotentialItemEffectBattler = gBattlerTarget;
                gMoveResultFlags &= 63;
                BattleScriptPushCursor();
                gBattlescriptCurrInstr = BattleScript_FocusBandActivates.as_ptr().cast_mut();
                return;
            }
            _ => {
                if gMoveResultFlags as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
                    stringId = STRINGID_ITDOESNTAFFECT;
                } else if gMoveResultFlags as i32 & MOVE_RESULT_ONE_HIT_KO != 0 {
                    gMoveResultFlags &= 239;
                    gMoveResultFlags &= 253;
                    gMoveResultFlags &= 251;
                    BattleScriptPushCursor();
                    gBattlescriptCurrInstr = BattleScript_OneHitKOMsg.as_ptr().cast_mut();
                    return;
                } else if gMoveResultFlags as i32 & MOVE_RESULT_FOE_ENDURED as i32 != 0 {
                    gMoveResultFlags &= 63;
                    BattleScriptPushCursor();
                    gBattlescriptCurrInstr = BattleScript_EnduredMsg.as_ptr().cast_mut();
                    return;
                } else if gMoveResultFlags as i32 & MOVE_RESULT_FOE_HUNG_ON as i32 != 0 {
                    gLastUsedItem = gBattleMons[gBattlerTarget].item;
                    gPotentialItemEffectBattler = gBattlerTarget;
                    gMoveResultFlags &= 63;
                    BattleScriptPushCursor();
                    gBattlescriptCurrInstr = BattleScript_FocusBandActivates.as_ptr().cast_mut();
                    return;
                } else if gMoveResultFlags as i32 & MOVE_RESULT_FAILED != 0 {
                    stringId = STRINGID_BUTITFAILED;
                } else {
                    gBattleCommunication[7] = 0;
                }
            }
        }
    }
    if stringId != 0 {
        PrepareStringBattle(stringId as u16, gBattlerAttacker);
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_printstring() {
    if gBattleControllerExecFlags == 0 {
        let mut var: u16 = *gBattlescriptCurrInstr.at(1) as u16
            + ((*gBattlescriptCurrInstr.at(1).at(1) as u16) << 8);
        PrepareStringBattle(var, gBattlerAttacker);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
        gBattleCommunication[7] = 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_printselectionstring() {
    gActiveBattler = gBattlerAttacker;
    BtlController_EmitPrintSelectionString(
        B_COMM_TO_CONTROLLER,
        *gBattlescriptCurrInstr.at(1) as u16 + ((*gBattlescriptCurrInstr.at(1).at(1) as u16) << 8),
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
    gBattleCommunication[7] = 1;
}
pub(crate) unsafe extern "C" fn Cmd_waitmessage() {
    if gBattleControllerExecFlags == 0 {
        if gBattleCommunication[7] == 0 {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
        } else {
            let mut toWait: u16 = *gBattlescriptCurrInstr.at(1) as u16
                + ((*gBattlescriptCurrInstr.at(1).at(1) as u16) << 8);
            if ({
                gPauseCounterBattle += 1;
                gPauseCounterBattle
            }) >= toWait
            {
                gPauseCounterBattle = 0;
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
                gBattleCommunication[7] = 0;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_printfromtable() {
    if gBattleControllerExecFlags == 0 {
        let mut ptr: *mut u16 = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8 as *mut u16;
        ptr = ptr.at(gBattleCommunication[5]);
        PrepareStringBattle(*ptr, gBattlerAttacker);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        gBattleCommunication[7] = 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_printselectionstringfromtable() {
    if gBattleControllerExecFlags == 0 {
        let mut ptr: *mut u16 = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8 as *mut u16;
        ptr = ptr.at(gBattleCommunication[5]);
        gActiveBattler = gBattlerAttacker;
        BtlController_EmitPrintSelectionString(B_COMM_TO_CONTROLLER, *ptr);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        gBattleCommunication[7] = 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerTurnOrderNum(battler: u8) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < gBattlersCount as i32 {
        if gBattlerByTurnOrder[i] == battler {
            break;
        }
        i += 1;
    }
    return i as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMoveEffect(primary: u8, certain: u8) {
    let mut statusChanged: u32 = FALSE as u32;
    let mut affectsUser: u8 = 0;
    let mut noSunCanFreeze: u32 = TRUE as u32;
    if gBattleCommunication[3] as i32 & MOVE_EFFECT_AFFECTS_USER as i32 != 0 {
        gEffectBattler = gBattlerAttacker;
        gBattleCommunication[3] &= 191;
        affectsUser = MOVE_EFFECT_AFFECTS_USER;
        gBattleScripting.battler = gBattlerTarget;
    } else {
        gEffectBattler = gBattlerTarget;
        gBattleScripting.battler = gBattlerAttacker;
    }
    if gBattleMons[gEffectBattler].ability == ABILITY_SHIELD_DUST
        && gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT == 0
        && primary == 0
        && gBattleCommunication[3] <= 9
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        gBattleCommunication[3] = 0;
        return;
    }
    if gSideStatuses[GetBattlerPosition(gEffectBattler) as i32 & 1] as i32 & SIDE_STATUS_SAFEGUARD
        != 0
        && gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT == 0
        && primary == 0
        && gBattleCommunication[3] <= 7
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        gBattleCommunication[3] = 0;
        return;
    }
    if gBattleMons[gEffectBattler].hp == 0
        && gBattleCommunication[3] != MOVE_EFFECT_PAYDAY
        && gBattleCommunication[3] != MOVE_EFFECT_STEAL_ITEM
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        gBattleCommunication[3] = 0;
        return;
    }
    if gBattleMons[gEffectBattler].status2 & STATUS2_SUBSTITUTE != 0
        && affectsUser != MOVE_EFFECT_AFFECTS_USER
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        gBattleCommunication[3] = 0;
        return;
    }
    if gBattleCommunication[3] <= PRIMARY_STATUS_MOVE_EFFECT {
        'l1: {
            match sStatusFlagsForMoveEffects[gBattleCommunication[3]] {
                STATUS1_SLEEP => {
                    if gBattleMons[gEffectBattler].ability != ABILITY_SOUNDPROOF {
                        gActiveBattler = 0;
                        while gActiveBattler < gBattlersCount
                            && gBattleMons[gActiveBattler].status2 & STATUS2_UPROAR == 0
                        {
                            gActiveBattler += 1;
                        }
                    } else {
                        gActiveBattler = gBattlersCount;
                    }
                    if gBattleMons[gEffectBattler].status1 != 0 {
                        break 'l1;
                    }
                    if gActiveBattler != gBattlersCount {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].ability == ABILITY_VITAL_SPIRIT {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].ability == ABILITY_INSOMNIA {
                        break 'l1;
                    }
                    CancelMultiTurnMoves(gEffectBattler);
                    statusChanged = TRUE as u32;
                }
                STATUS1_POISON => {
                    if gBattleMons[gEffectBattler].ability == ABILITY_IMMUNITY
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        gLastUsedAbility = ABILITY_IMMUNITY;
                        RecordAbilityBattle(gEffectBattler, ABILITY_IMMUNITY);
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_PSNPrevention.as_ptr().cast_mut();
                        if gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                            gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS;
                            gHitMarker &= 0xffffdfff;
                        } else {
                            gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS;
                        }
                        gBattleCommunication[3] = 0;
                        return;
                    }
                    if (gBattleMons[gEffectBattler].types[0] == TYPE_POISON
                        || gBattleMons[gEffectBattler].types[1] == TYPE_POISON
                        || (gBattleMons[gEffectBattler].types[0] == TYPE_STEEL
                            || gBattleMons[gEffectBattler].types[1] == TYPE_STEEL))
                        && gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_PSNPrevention.as_ptr().cast_mut();
                        gBattleCommunication[5] = B_MSG_STATUS_HAD_NO_EFFECT;
                        gBattleCommunication[3] = 0;
                        return;
                    }
                    if gBattleMons[gEffectBattler].types[0] == TYPE_POISON
                        || gBattleMons[gEffectBattler].types[1] == TYPE_POISON
                    {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].types[0] == TYPE_STEEL
                        || gBattleMons[gEffectBattler].types[1] == TYPE_STEEL
                    {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].status1 != 0 {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].ability == ABILITY_IMMUNITY {
                        break 'l1;
                    }
                    statusChanged = TRUE as u32;
                }
                STATUS1_BURN => {
                    if gBattleMons[gEffectBattler].ability == ABILITY_WATER_VEIL
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        gLastUsedAbility = ABILITY_WATER_VEIL;
                        RecordAbilityBattle(gEffectBattler, ABILITY_WATER_VEIL);
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_BRNPrevention.as_ptr().cast_mut();
                        if gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                            gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS;
                            gHitMarker &= 0xffffdfff;
                        } else {
                            gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS;
                        }
                        gBattleCommunication[3] = 0;
                        return;
                    }
                    if (gBattleMons[gEffectBattler].types[0] == TYPE_FIRE
                        || gBattleMons[gEffectBattler].types[1] == TYPE_FIRE)
                        && gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_BRNPrevention.as_ptr().cast_mut();
                        gBattleCommunication[5] = B_MSG_STATUS_HAD_NO_EFFECT;
                        gBattleCommunication[3] = 0;
                        return;
                    }
                    if gBattleMons[gEffectBattler].types[0] == TYPE_FIRE
                        || gBattleMons[gEffectBattler].types[1] == TYPE_FIRE
                    {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].ability == ABILITY_WATER_VEIL {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].status1 != 0 {
                        break 'l1;
                    }
                    statusChanged = TRUE as u32;
                }
                STATUS1_FREEZE => {
                    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0
                        && AbilityBattleEffects(19, 0, 77, 0, 0) == 0
                        && gBattleWeather as i32 & B_WEATHER_SUN != 0
                    {
                        noSunCanFreeze = FALSE as u32;
                    }
                    if gBattleMons[gEffectBattler].types[0] == TYPE_ICE
                        || gBattleMons[gEffectBattler].types[1] == TYPE_ICE
                    {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].status1 != 0 {
                        break 'l1;
                    }
                    if noSunCanFreeze == FALSE as u32 {
                        break 'l1;
                    }
                    if gBattleMons[gEffectBattler].ability == ABILITY_MAGMA_ARMOR {
                        break 'l1;
                    }
                    CancelMultiTurnMoves(gEffectBattler);
                    statusChanged = TRUE as u32;
                }
                STATUS1_PARALYSIS => {
                    if gBattleMons[gEffectBattler].ability == ABILITY_LIMBER {
                        if primary == TRUE || certain == MOVE_EFFECT_CERTAIN {
                            gLastUsedAbility = ABILITY_LIMBER;
                            RecordAbilityBattle(gEffectBattler, ABILITY_LIMBER);
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr =
                                BattleScript_PRLZPrevention.as_ptr().cast_mut();
                            if gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS;
                                gHitMarker &= 0xffffdfff;
                            } else {
                                gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS;
                            }
                            gBattleCommunication[3] = 0;
                            return;
                        } else {
                            break 'l1;
                        }
                    }
                    if gBattleMons[gEffectBattler].status1 != 0 {
                        break 'l1;
                    }
                    statusChanged = TRUE as u32;
                }
                STATUS1_TOXIC_POISON => {
                    if gBattleMons[gEffectBattler].ability == ABILITY_IMMUNITY
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        gLastUsedAbility = ABILITY_IMMUNITY;
                        RecordAbilityBattle(gEffectBattler, ABILITY_IMMUNITY);
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_PSNPrevention.as_ptr().cast_mut();
                        if gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                            gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_ABILITY_STATUS;
                            gHitMarker &= 0xffffdfff;
                        } else {
                            gBattleCommunication[5] = B_MSG_ABILITY_PREVENTS_MOVE_STATUS;
                        }
                        gBattleCommunication[3] = 0;
                        return;
                    }
                    if (gBattleMons[gEffectBattler].types[0] == TYPE_POISON
                        || gBattleMons[gEffectBattler].types[1] == TYPE_POISON
                        || (gBattleMons[gEffectBattler].types[0] == TYPE_STEEL
                            || gBattleMons[gEffectBattler].types[1] == TYPE_STEEL))
                        && gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0
                        && (primary == TRUE || certain == MOVE_EFFECT_CERTAIN)
                    {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_PSNPrevention.as_ptr().cast_mut();
                        gBattleCommunication[5] = B_MSG_STATUS_HAD_NO_EFFECT;
                        gBattleCommunication[3] = 0;
                        return;
                    }
                    if gBattleMons[gEffectBattler].status1 != 0 {
                        break 'l1;
                    }
                    if !(gBattleMons[gEffectBattler].types[0] == TYPE_POISON
                        || gBattleMons[gEffectBattler].types[1] == TYPE_POISON)
                        && !(gBattleMons[gEffectBattler].types[0] == TYPE_STEEL
                            || gBattleMons[gEffectBattler].types[1] == TYPE_STEEL)
                    {
                        if gBattleMons[gEffectBattler].ability == ABILITY_IMMUNITY {
                            break 'l1;
                        }
                        gBattleMons[gEffectBattler].status1 &= 0xffffff7f;
                        gBattleMons[gEffectBattler].status1 &= 0xfffffff7;
                        statusChanged = TRUE as u32;
                        break 'l1;
                    } else {
                        gMoveResultFlags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
                    }
                }
                _ => {}
            }
        }
        if statusChanged == TRUE as u32 {
            BattleScriptPush(gBattlescriptCurrInstr.at(1));
            if sStatusFlagsForMoveEffects[gBattleCommunication[3]] == STATUS1_SLEEP {
                gBattleMons[gEffectBattler].status1 |= (Random() as u32 & 3) + 2 << 0;
            } else {
                gBattleMons[gEffectBattler].status1 |=
                    sStatusFlagsForMoveEffects[gBattleCommunication[3]];
            }
            gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
            gActiveBattler = gEffectBattler;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_STATUS_BATTLE,
                0,
                4,
                &raw mut gBattleMons[gEffectBattler].status1 as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler);
            if gHitMarker & HITMARKER_STATUS_ABILITY_EFFECT != 0 {
                gBattleCommunication[5] = B_MSG_STATUSED_BY_ABILITY;
                gHitMarker &= 0xffffdfff;
            } else {
                gBattleCommunication[5] = B_MSG_STATUSED;
            }
            if gBattleCommunication[3] == MOVE_EFFECT_POISON
                || gBattleCommunication[3] == MOVE_EFFECT_TOXIC
                || gBattleCommunication[3] == MOVE_EFFECT_PARALYSIS
                || gBattleCommunication[3] == 3
            {
                let mut synchronizeEffect: *mut u8 =
                    &raw mut (*gBattleStruct).synchronizeMoveEffect;
                *synchronizeEffect = gBattleCommunication[3];
                gHitMarker |= HITMARKER_SYNCHRONIZE_EFFECT;
            }
            return;
        } else if statusChanged == FALSE as u32 {
            gBattleCommunication[3] = 0;
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
            return;
        }
        return;
    } else {
        if gBattleMons[gEffectBattler].status2 & sStatusFlagsForMoveEffects[gBattleCommunication[3]]
            != 0
        {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        } else {
            let mut side: u8 = 0;
            'l3: {
                match gBattleCommunication[3] {
                    MOVE_EFFECT_CONFUSION => {
                        if gBattleMons[gEffectBattler].ability == ABILITY_OWN_TEMPO
                            || gBattleMons[gEffectBattler].status2 & STATUS2_CONFUSION != 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleMons[gEffectBattler].status2 |=
                                (Random() as i32 % 4) as u32 + 2 << 0;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
                        }
                    }
                    MOVE_EFFECT_FLINCH => {
                        if gBattleMons[gEffectBattler].ability == ABILITY_INNER_FOCUS {
                            if primary == TRUE || certain == MOVE_EFFECT_CERTAIN {
                                gLastUsedAbility = ABILITY_INNER_FOCUS;
                                RecordAbilityBattle(gEffectBattler, ABILITY_INNER_FOCUS);
                                gBattlescriptCurrInstr =
                                    BattleScript_FlinchPrevention.as_ptr().cast_mut();
                            } else {
                                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                            }
                        } else {
                            if GetBattlerTurnOrderNum(gEffectBattler) > gCurrentTurnActionNumber {
                                gBattleMons[gEffectBattler].status2 |=
                                    sStatusFlagsForMoveEffects[gBattleCommunication[3]];
                            }
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        }
                    }
                    MOVE_EFFECT_UPROAR => {
                        if gBattleMons[gEffectBattler].status2 & STATUS2_UPROAR == 0 {
                            gBattleMons[gEffectBattler].status2 |= STATUS2_MULTIPLETURNS;
                            gLockedMoves[gEffectBattler] = gCurrentMove;
                            gBattleMons[gEffectBattler].status2 |= (Random() as u32 & 3) + 2 << 4;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
                        } else {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        }
                    }
                    MOVE_EFFECT_PAYDAY => {
                        if GetBattlerPosition(gBattlerAttacker) as i32 & 1 == B_SIDE_PLAYER as i32 {
                            let mut payday: u16 = gPaydayMoney;
                            gPaydayMoney += gBattleMons[gBattlerAttacker].level as u16 * 5;
                            if payday > gPaydayMoney {
                                gPaydayMoney = 0xFFFF;
                            }
                        }
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
                    }
                    MOVE_EFFECT_TRI_ATTACK => {
                        if gBattleMons[gEffectBattler].status1 != 0 {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleCommunication[3] = (Random() as i32 % 3) as u8 + 3;
                            SetMoveEffect(0, 0);
                        }
                    }
                    MOVE_EFFECT_CHARGING => {
                        gBattleMons[gEffectBattler].status2 |= STATUS2_MULTIPLETURNS;
                        gLockedMoves[gEffectBattler] = gCurrentMove;
                        gProtectStructs[gEffectBattler].set_chargingTurn(1);
                        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                    }
                    MOVE_EFFECT_WRAP => {
                        if gBattleMons[gEffectBattler].status2 & STATUS2_WRAPPED != 0 {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleMons[gEffectBattler].status2 |= (Random() as u32 & 3) + 3 << 13;
                            *(*gBattleStruct)
                                .wrappedMove
                                .as_mut_ptr()
                                .at(gEffectBattler as i32 * 2) = gCurrentMove as u8;
                            *(*gBattleStruct)
                                .wrappedMove
                                .as_mut_ptr()
                                .at(gEffectBattler as i32 * 2)
                                .at(1) = (gCurrentMove >> 8) as u8;
                            *(*gBattleStruct).wrappedBy.as_mut_ptr().at(gEffectBattler) =
                                gBattlerAttacker;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
                            gBattleCommunication[5] = 0;
                            loop {
                                if gBattleCommunication[5] >= MULTISTRING_CHOOSER {
                                    break;
                                }
                                if gTrappingMoves[gBattleCommunication[5]] == gCurrentMove {
                                    break;
                                }
                                gBattleCommunication[5] += 1;
                            }
                        }
                    }
                    MOVE_EFFECT_RECOIL_25 => {
                        gBattleMoveDamage = gHpDealt / 4;
                        if gBattleMoveDamage == 0 {
                            gBattleMoveDamage = 1;
                        }
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
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
                            gBattleCommunication[3] - MOVE_EFFECT_ATK_PLUS_1 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleScripting.animArg1 = gBattleCommunication[3] & 63;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = BattleScript_StatUp.as_ptr().cast_mut();
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
                            gBattleCommunication[3] - MOVE_EFFECT_ATK_MINUS_1 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleScripting.animArg1 = gBattleCommunication[3] & 63;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = BattleScript_StatDown.as_ptr().cast_mut();
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
                            gBattleCommunication[3] - MOVE_EFFECT_ATK_PLUS_2 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleScripting.animArg1 = gBattleCommunication[3] & 63;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = BattleScript_StatUp.as_ptr().cast_mut();
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
                            gBattleCommunication[3] - MOVE_EFFECT_ATK_MINUS_2 + 1,
                            affectsUser,
                            null_mut(),
                        ) != 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleScripting.animArg1 = gBattleCommunication[3] & 63;
                            gBattleScripting.animArg2 = 0;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = BattleScript_StatDown.as_ptr().cast_mut();
                        }
                    }
                    MOVE_EFFECT_RECHARGE => {
                        gBattleMons[gEffectBattler].status2 |= STATUS2_RECHARGE;
                        gDisableStructs[gEffectBattler].rechargeTimer = 2;
                        gLockedMoves[gEffectBattler] = gCurrentMove;
                        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                    }
                    MOVE_EFFECT_RAGE => {
                        gBattleMons[gBattlerAttacker].status2 |= STATUS2_RAGE;
                        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                    }
                    MOVE_EFFECT_STEAL_ITEM => {
                        if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                            break 'l3;
                        }
                        side = GetBattlerSide(gBattlerAttacker);
                        if GetBattlerSide(gBattlerAttacker) == B_SIDE_OPPONENT
                            && gBattleTypeFlags & 0xa3f0902 == 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else if gBattleTypeFlags & 0xa3f0902 == 0
                            && gWishFutureKnock.knockedOffMons[side] as u32
                                & gBitTable[gBattlerPartyIndexes[gBattlerAttacker]]
                                != 0
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else if gBattleMons[gBattlerTarget].item != 0
                            && gBattleMons[gBattlerTarget].ability == ABILITY_STICKY_HOLD
                        {
                            BattleScriptPushCursor();
                            gBattlescriptCurrInstr = BattleScript_NoItemSteal.as_ptr().cast_mut();
                            gLastUsedAbility = gBattleMons[gBattlerTarget].ability;
                            RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
                        } else if gBattleMons[gBattlerAttacker].item != ITEM_NONE
                            || gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY
                            || (gBattleMons[gBattlerTarget].item == ITEM_ORANGE_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_HARBOR_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_GLITTER_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_MECH_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_WOOD_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_WAVE_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_BEAD_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_SHADOW_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_TROPIC_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_DREAM_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_FAB_MAIL
                                || gBattleMons[gBattlerTarget].item == ITEM_RETRO_MAIL)
                            || gBattleMons[gBattlerTarget].item == ITEM_NONE
                        {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            let mut changedItem: *mut u16 =
                                &raw mut (*gBattleStruct).changedItems[gBattlerAttacker];
                            gLastUsedItem = {
                                *changedItem = gBattleMons[gBattlerTarget].item;
                                *changedItem
                            };
                            gBattleMons[gBattlerTarget].item = ITEM_NONE;
                            gActiveBattler = gBattlerAttacker;
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_HELDITEM_BATTLE,
                                0,
                                2,
                                &raw mut gLastUsedItem as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gBattlerAttacker);
                            gActiveBattler = gBattlerTarget;
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_HELDITEM_BATTLE,
                                0,
                                2,
                                &raw mut gBattleMons[gBattlerTarget].item as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gBattlerTarget);
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = BattleScript_ItemSteal.as_ptr().cast_mut();
                            *(&raw mut (*gBattleStruct).choicedMove[gBattlerTarget] as *mut u8) = 0;
                            *(&raw mut (*gBattleStruct).choicedMove[gBattlerTarget] as *mut u8)
                                .at(1) = 0;
                        }
                    }
                    MOVE_EFFECT_PREVENT_ESCAPE => {
                        gBattleMons[gBattlerTarget].status2 |= STATUS2_ESCAPE_PREVENTION;
                        gDisableStructs[gBattlerTarget].battlerPreventingEscape = gBattlerAttacker;
                        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                    }
                    MOVE_EFFECT_NIGHTMARE => {
                        gBattleMons[gBattlerTarget].status2 |= STATUS2_NIGHTMARE;
                        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                    }
                    MOVE_EFFECT_ALL_STATS_UP => {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_AllStatsUp.as_ptr().cast_mut();
                    }
                    MOVE_EFFECT_RAPIDSPIN => {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_RapidSpinAway.as_ptr().cast_mut();
                    }
                    MOVE_EFFECT_REMOVE_PARALYSIS => {
                        if gBattleMons[gBattlerTarget].status1 & STATUS1_PARALYSIS == 0 {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleMons[gBattlerTarget].status1 &= 0xffffffbf;
                            gActiveBattler = gBattlerTarget;
                            BtlController_EmitSetMonData(
                                B_COMM_TO_CONTROLLER,
                                REQUEST_STATUS_BATTLE,
                                0,
                                4,
                                &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
                            );
                            MarkBattlerForControllerExec(gActiveBattler);
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr =
                                BattleScript_TargetPRLZHeal.as_ptr().cast_mut();
                        }
                    }
                    MOVE_EFFECT_ATK_DEF_DOWN => {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_AtkDefDown.as_ptr().cast_mut();
                    }
                    MOVE_EFFECT_RECOIL_33 => {
                        gBattleMoveDamage = gHpDealt / 3;
                        if gBattleMoveDamage == 0 {
                            gBattleMoveDamage = 1;
                        }
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = sMoveEffectBS_Ptrs[gBattleCommunication[3]];
                    }
                    MOVE_EFFECT_THRASH => {
                        if gBattleMons[gEffectBattler].status2 & STATUS2_LOCK_CONFUSE != 0 {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        } else {
                            gBattleMons[gEffectBattler].status2 |= STATUS2_MULTIPLETURNS;
                            gLockedMoves[gEffectBattler] = gCurrentMove;
                            gBattleMons[gEffectBattler].status2 |= (Random() as u32 & 1) + 2 << 10;
                        }
                    }
                    MOVE_EFFECT_KNOCK_OFF => {
                        if gBattleMons[gEffectBattler].ability == ABILITY_STICKY_HOLD {
                            if gBattleMons[gEffectBattler].item == ITEM_NONE {
                                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                            } else {
                                gLastUsedAbility = ABILITY_STICKY_HOLD;
                                gBattlescriptCurrInstr =
                                    BattleScript_StickyHoldActivates.as_ptr().cast_mut();
                                RecordAbilityBattle(gEffectBattler, ABILITY_STICKY_HOLD);
                            }
                            break 'l3;
                        }
                        if gBattleMons[gEffectBattler].item != 0 {
                            side = GetBattlerSide(gEffectBattler);
                            gLastUsedItem = gBattleMons[gEffectBattler].item;
                            gBattleMons[gEffectBattler].item = ITEM_NONE;
                            gWishFutureKnock.knockedOffMons[side] |=
                                gBitTable[gBattlerPartyIndexes[gEffectBattler]] as u8;
                            BattleScriptPush(gBattlescriptCurrInstr.at(1));
                            gBattlescriptCurrInstr = BattleScript_KnockedOff.as_ptr().cast_mut();
                            *(&raw mut (*gBattleStruct).choicedMove[gEffectBattler] as *mut u8) = 0;
                            *(&raw mut (*gBattleStruct).choicedMove[gEffectBattler] as *mut u8)
                                .at(1) = 0;
                        } else {
                            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
                        }
                    }
                    MOVE_EFFECT_SP_ATK_TWO_DOWN => {
                        BattleScriptPush(gBattlescriptCurrInstr.at(1));
                        gBattlescriptCurrInstr = BattleScript_SAtkDown2.as_ptr().cast_mut();
                    }
                    _ => {}
                }
            }
        }
    }
    gBattleCommunication[3] = 0;
}
pub(crate) unsafe extern "C" fn Cmd_seteffectwithchance() {
    let mut percentChance: u32 = 0;
    if gBattleMons[gBattlerAttacker].ability == ABILITY_SERENE_GRACE {
        percentChance = gBattleMoves[gCurrentMove].secondaryEffectChance as u32 * 2;
    } else {
        percentChance = gBattleMoves[gCurrentMove].secondaryEffectChance as u32;
    }
    if gBattleCommunication[3] as i32 & MOVE_EFFECT_CERTAIN as i32 != 0
        && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
    {
        gBattleCommunication[3] &= 127;
        SetMoveEffect(FALSE, MOVE_EFFECT_CERTAIN);
    } else if ((Random() as i32 % 100) as u32) < percentChance
        && gBattleCommunication[3] != 0
        && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
    {
        if percentChance >= 100 {
            SetMoveEffect(FALSE, MOVE_EFFECT_CERTAIN);
        } else {
            SetMoveEffect(0, 0);
        }
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
    gBattleCommunication[3] = 0;
    gBattleScripting.multihitMoveEffect = 0;
}
pub(crate) unsafe extern "C" fn Cmd_seteffectprimary() {
    SetMoveEffect(TRUE, 0);
}
pub(crate) unsafe extern "C" fn Cmd_seteffectsecondary() {
    SetMoveEffect(0, 0);
}
pub(crate) unsafe extern "C" fn Cmd_clearstatusfromeffect() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    if gBattleCommunication[3] <= PRIMARY_STATUS_MOVE_EFFECT {
        gBattleMons[gActiveBattler].status1 &= !sStatusFlagsForMoveEffects[gBattleCommunication[3]];
    } else {
        gBattleMons[gActiveBattler].status2 &= !sStatusFlagsForMoveEffects[gBattleCommunication[3]];
    }
    gBattleCommunication[3] = 0;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    gBattleScripting.multihitMoveEffect = 0;
}
pub(crate) unsafe extern "C" fn Cmd_tryfaintmon() {
    let mut BS_ptr: *mut u8 = null_mut();
    if *gBattlescriptCurrInstr.at(2) != 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gHitMarker & gBitTable[gActiveBattler] << 28 != 0 {
            BS_ptr = (*gBattlescriptCurrInstr.at(3) as i32
                | (*gBattlescriptCurrInstr.at(3).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(3).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(3).at(3) as i32) << 24) as usize
                as *mut u8;
            BattleScriptPop();
            gBattlescriptCurrInstr = BS_ptr;
            gSideStatuses[GetBattlerSide(gActiveBattler)] &= 65023;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        }
    } else {
        let mut battler: u8 = 0;
        if *gBattlescriptCurrInstr.at(1) == 1 {
            gActiveBattler = gBattlerAttacker;
            battler = gBattlerTarget;
            BS_ptr = BattleScript_FaintAttacker.as_ptr().cast_mut();
        } else {
            gActiveBattler = gBattlerTarget;
            battler = gBattlerAttacker;
            BS_ptr = BattleScript_FaintTarget.as_ptr().cast_mut();
        }
        if gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0
            && gBattleMons[gActiveBattler].hp == 0
        {
            gHitMarker |= gBitTable[gActiveBattler] << 28;
            BattleScriptPush(gBattlescriptCurrInstr.at(7));
            gBattlescriptCurrInstr = BS_ptr;
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                gHitMarker |= HITMARKER_PLAYER_FAINTED;
                if gBattleResults.playerFaintCounter < 255 {
                    gBattleResults.playerFaintCounter += 1;
                }
                AdjustFriendshipOnBattleFaint(gActiveBattler);
            } else {
                if gBattleResults.opponentFaintCounter < 255 {
                    gBattleResults.opponentFaintCounter += 1;
                }
                gBattleResults.lastOpponentSpecies = GetMonData3(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
                    MON_DATA_SPECIES,
                    null_mut(),
                ) as u16;
            }
            if gHitMarker & HITMARKER_DESTINYBOND != 0 && gBattleMons[gBattlerAttacker].hp != 0 {
                gHitMarker &= 0xffffffbf;
                BattleScriptPush(gBattlescriptCurrInstr);
                gBattleMoveDamage = gBattleMons[battler].hp as i32;
                gBattlescriptCurrInstr = BattleScript_DestinyBondTakesLife.as_ptr().cast_mut();
            }
            if gStatuses3[gBattlerTarget] & STATUS3_GRUDGE != 0
                && gHitMarker & HITMARKER_GRUDGE == 0
                && GetBattlerSide(gBattlerAttacker) != GetBattlerSide(gBattlerTarget)
                && gBattleMons[gBattlerAttacker].hp != 0
                && gCurrentMove != MOVE_STRUGGLE
            {
                let mut moveIndex: u8 = *(*gBattleStruct)
                    .chosenMovePositions
                    .as_mut_ptr()
                    .at(gBattlerAttacker);
                gBattleMons[gBattlerAttacker].pp[moveIndex] = 0;
                BattleScriptPush(gBattlescriptCurrInstr);
                gBattlescriptCurrInstr = BattleScript_GrudgeTakesPP.as_ptr().cast_mut();
                gActiveBattler = gBattlerAttacker;
                BtlController_EmitSetMonData(
                    B_COMM_TO_CONTROLLER,
                    moveIndex + REQUEST_PPMOVE1_BATTLE,
                    0,
                    1,
                    &raw mut gBattleMons[gActiveBattler].pp[moveIndex] as *mut c_void,
                );
                MarkBattlerForControllerExec(gActiveBattler);
                gBattleTextBuff1[0] = 0xFD;
                gBattleTextBuff1[1] = 2;
                gBattleTextBuff1[2] = gBattleMons[gBattlerAttacker].moves[moveIndex] as u8 & 0xFF;
                gBattleTextBuff1[3] =
                    ((gBattleMons[gBattlerAttacker].moves[moveIndex] as i32 & 0xFF00) >> 8) as u8;
                gBattleTextBuff1[4] = 0xFF;
            }
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_dofaintanimation() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        BtlController_EmitFaintAnimation(B_COMM_TO_CONTROLLER);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    }
}
pub(crate) unsafe extern "C" fn Cmd_cleareffectsonfaint() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gBattleTypeFlags & BATTLE_TYPE_ARENA == 0 || gBattleMons[gActiveBattler].hp == 0 {
            gBattleMons[gActiveBattler].status1 = 0;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_STATUS_BATTLE,
                0,
                4,
                &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
        FaintClearSetData();
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifstatus() {
    let mut battler: u8 = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    let mut flags: u32 = *gBattlescriptCurrInstr.at(2) as u32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as u32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as u32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as u32) << 24);
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(6) as i32
        + ((*gBattlescriptCurrInstr.at(6).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(6).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(6).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if gBattleMons[battler].status1 & flags != 0 && gBattleMons[battler].hp != 0 {
        gBattlescriptCurrInstr = jumpPtr;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifstatus2() {
    let mut battler: u8 = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    let mut flags: u32 = *gBattlescriptCurrInstr.at(2) as u32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as u32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as u32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as u32) << 24);
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(6) as i32
        + ((*gBattlescriptCurrInstr.at(6).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(6).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(6).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if gBattleMons[battler].status2 & flags != 0 && gBattleMons[battler].hp != 0 {
        gBattlescriptCurrInstr = jumpPtr;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifability() {
    let mut battler: u8 = 0;
    let mut ability: u8 = *gBattlescriptCurrInstr.at(2);
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(3) as i32
        + ((*gBattlescriptCurrInstr.at(3).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(3).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(3).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if *gBattlescriptCurrInstr.at(1) == BS_ATTACKER_SIDE {
        battler = AbilityBattleEffects(
            ABILITYEFFECT_CHECK_BATTLER_SIDE,
            gBattlerAttacker,
            ability,
            0,
            0,
        );
        if battler != 0 {
            gLastUsedAbility = ability;
            gBattlescriptCurrInstr = jumpPtr;
            RecordAbilityBattle(battler - 1, gLastUsedAbility);
            gBattleScripting.battlerWithAbility = battler - 1;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        }
    } else if *gBattlescriptCurrInstr.at(1) == BS_NOT_ATTACKER_SIDE {
        battler = AbilityBattleEffects(
            ABILITYEFFECT_CHECK_OTHER_SIDE,
            gBattlerAttacker,
            ability,
            0,
            0,
        );
        if battler != 0 {
            gLastUsedAbility = ability;
            gBattlescriptCurrInstr = jumpPtr;
            RecordAbilityBattle(battler - 1, gLastUsedAbility);
            gBattleScripting.battlerWithAbility = battler - 1;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        }
    } else {
        battler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gBattleMons[battler].ability == ability {
            gLastUsedAbility = ability;
            gBattlescriptCurrInstr = jumpPtr;
            RecordAbilityBattle(battler, gLastUsedAbility);
            gBattleScripting.battlerWithAbility = battler;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifsideaffecting() {
    let mut side: u8 = 0;
    let mut flags: u16 = 0;
    let mut jumpPtr: *mut u8 = null_mut();
    if *gBattlescriptCurrInstr.at(1) == 1 {
        side = GetBattlerPosition(gBattlerAttacker) & 1;
    } else {
        side = GetBattlerPosition(gBattlerTarget) & 1;
    }
    flags =
        *gBattlescriptCurrInstr.at(2) as u16 + ((*gBattlescriptCurrInstr.at(2).at(1) as u16) << 8);
    jumpPtr = (*gBattlescriptCurrInstr.at(4) as i32
        + ((*gBattlescriptCurrInstr.at(4).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(4).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(4).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    if gSideStatuses[side] as i32 & flags as i32 != 0 {
        gBattlescriptCurrInstr = jumpPtr;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifstat() {
    let mut ret: u8 = 0;
    let mut battler: u8 = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    let mut value: u8 = gBattleMons[battler].statStages[*gBattlescriptCurrInstr.at(3)] as u8;
    match *gBattlescriptCurrInstr.at(2) {
        CMP_EQUAL => {
            if value == *gBattlescriptCurrInstr.at(4) {
                ret += 1;
            }
        }
        CMP_NOT_EQUAL => {
            if value != *gBattlescriptCurrInstr.at(4) {
                ret += 1;
            }
        }
        CMP_GREATER_THAN => {
            if value > *gBattlescriptCurrInstr.at(4) {
                ret += 1;
            }
        }
        CMP_LESS_THAN => {
            if value < *gBattlescriptCurrInstr.at(4) {
                ret += 1;
            }
        }
        CMP_COMMON_BITS => {
            if value as i32 & *gBattlescriptCurrInstr.at(4) as i32 != 0 {
                ret += 1;
            }
        }
        CMP_NO_COMMON_BITS => {
            if value as i32 & *gBattlescriptCurrInstr.at(4) as i32 == 0 {
                ret += 1;
            }
        }
        _ => {}
    }
    if ret != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(5) as i32
            + ((*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8)
            + ((*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16)
            + ((*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24))
            as usize as *mut c_void as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(9);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifstatus3condition() {
    let mut status: u32 = 0;
    let mut jumpPtr: *mut u8 = null_mut();
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    status = *gBattlescriptCurrInstr.at(2) as u32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as u32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as u32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as u32) << 24);
    jumpPtr = (*gBattlescriptCurrInstr.at(7) as i32
        + ((*gBattlescriptCurrInstr.at(7).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(7).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(7).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    if *gBattlescriptCurrInstr.at(6) != 0 {
        if gStatuses3[gActiveBattler] & status != 0 {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(11);
        } else {
            gBattlescriptCurrInstr = jumpPtr;
        }
    } else {
        if gStatuses3[gActiveBattler] & status != 0 {
            gBattlescriptCurrInstr = jumpPtr;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(11);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpiftype() {
    let mut battler: u8 = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    let mut r#type: u8 = *gBattlescriptCurrInstr.at(2);
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(3) as i32
        + ((*gBattlescriptCurrInstr.at(3).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(3).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(3).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    if gBattleMons[battler].types[0] == r#type || gBattleMons[battler].types[1] == r#type {
        gBattlescriptCurrInstr = jumpPtr;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_getexp() {
    let mut item: u16 = 0;
    let mut i: i32 = 0;
    let mut holdEffect: u8 = 0;
    let mut sentIn: i32 = 0;
    let mut viaExpShare: i32 = 0;
    let mut exp: *mut u16 = &raw mut (*gBattleStruct).expValue;
    gBattlerFainted = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    sentIn = gSentPokesToOpponent[(gBattlerFainted as i32 & 2) >> 1] as i32;
    'l1: {
        let sw1: u8 = gBattleScripting.getexpState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if GetBattlerSide(gBattlerFainted) != B_SIDE_OPPONENT
                || gBattleTypeFlags & 0x63f0982 != 0
            {
                gBattleScripting.getexpState = 6;
            } else {
                gBattleScripting.getexpState += 1;
                (*gBattleStruct).givenExpMons |=
                    gBitTable[gBattlerPartyIndexes[gBattlerFainted]] as u8;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            {
                let mut calculatedExp: u16 = 0;
                let mut viaSentIn: i32 = 0;
                viaSentIn = 0;
                i = 0;
                while i < PARTY_SIZE {
                    'l2: {
                        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) == 0
                            || GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) == 0
                        {
                            break 'l2;
                        }
                        if gBitTable[i] & sentIn as u32 != 0 {
                            viaSentIn += 1;
                        }
                        item = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) as u16;
                        if item == ITEM_ENIGMA_BERRY {
                            holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
                        } else {
                            holdEffect = GetItemHoldEffect(item);
                        }
                        if holdEffect == HOLD_EFFECT_EXP_SHARE {
                            viaExpShare += 1;
                        }
                    }
                    i += 1;
                }
                calculatedExp = (gSpeciesInfo[gBattleMons[gBattlerFainted].species].expYield as i32
                    * gBattleMons[gBattlerFainted].level as i32
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
                    gExpShareExp = div_i32(calculatedExp as i32 / 2, viaExpShare) as u16;
                    if gExpShareExp == 0 {
                        gExpShareExp = 1;
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
                    gExpShareExp = 0;
                }
                gBattleScripting.getexpState += 1;
                (*gBattleStruct).expGetterMonId = 0;
                (*gBattleStruct).sentInPokes = sentIn as u8;
            }
        }
        if fall || sw1 == 2 {
            fall = true;
            if gBattleControllerExecFlags == 0 {
                item = GetMonData2(
                    &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                    MON_DATA_HELD_ITEM,
                ) as u16;
                if item == ITEM_ENIGMA_BERRY {
                    holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
                } else {
                    holdEffect = GetItemHoldEffect(item);
                }
                if holdEffect != HOLD_EFFECT_EXP_SHARE
                    && (*gBattleStruct).sentInPokes as i32 & 1 == 0
                {
                    *(&raw mut (*gBattleStruct).sentInPokes) >>= 1;
                    gBattleScripting.getexpState = 5;
                    gBattleMoveDamage = 0;
                } else if GetMonData2(
                    &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                    MON_DATA_LEVEL,
                ) == MAX_LEVEL
                {
                    *(&raw mut (*gBattleStruct).sentInPokes) >>= 1;
                    gBattleScripting.getexpState = 5;
                    gBattleMoveDamage = 0;
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER == 0
                        && gBattleMons[0].hp != 0
                        && (*gBattleStruct).wildVictorySong == 0
                    {
                        BattleStopLowHpSound();
                        PlayBGM(MUS_VICTORY_WILD);
                        (*gBattleStruct).wildVictorySong += 1;
                    }
                    if GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_HP,
                    ) != 0
                    {
                        if (*gBattleStruct).sentInPokes as i32 & 1 != 0 {
                            gBattleMoveDamage = *exp as i32;
                        } else {
                            gBattleMoveDamage = 0;
                        }
                        if holdEffect == HOLD_EFFECT_EXP_SHARE {
                            gBattleMoveDamage += gExpShareExp as i32;
                        }
                        if holdEffect == HOLD_EFFECT_LUCKY_EGG {
                            gBattleMoveDamage = gBattleMoveDamage * 150 / 100;
                        }
                        if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                            gBattleMoveDamage = gBattleMoveDamage * 150 / 100;
                        }
                        if IsTradedMon(&raw mut gPlayerParty[(*gBattleStruct).expGetterMonId]) != 0
                        {
                            if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
                                && (*gBattleStruct).expGetterMonId >= 3
                            {
                                i = STRINGID_EMPTYSTRING4;
                            } else {
                                gBattleMoveDamage = gBattleMoveDamage * 150 / 100;
                                i = STRINGID_ABOOSTED;
                            }
                        } else {
                            i = STRINGID_EMPTYSTRING4;
                        }
                        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                            if gBattlerPartyIndexes[2] == (*gBattleStruct).expGetterMonId as u16
                                && gAbsentBattlerFlags as u32 & gBitTable[2] == 0
                            {
                                (*gBattleStruct).expGetterBattlerId = 2;
                            } else if gAbsentBattlerFlags as u32 & gBitTable[0] == 0 {
                                (*gBattleStruct).expGetterBattlerId = 0;
                            } else {
                                (*gBattleStruct).expGetterBattlerId = 2;
                            }
                        } else {
                            (*gBattleStruct).expGetterBattlerId = 0;
                        }
                        gBattleTextBuff1[0] = 0xFD;
                        gBattleTextBuff1[1] = 4;
                        gBattleTextBuff1[2] = (*gBattleStruct).expGetterBattlerId;
                        gBattleTextBuff1[3] = (*gBattleStruct).expGetterMonId;
                        gBattleTextBuff1[4] = 0xFF;
                        gBattleTextBuff2[0] = 0xFD;
                        gBattleTextBuff2[1] = 0;
                        gBattleTextBuff2[2] = i as u8;
                        gBattleTextBuff2[3] = ((i & 0xFF00) >> 8) as u8;
                        gBattleTextBuff2[4] = 0xFF;
                        gBattleTextBuff3[0] = 0xFD;
                        gBattleTextBuff3[1] = 1;
                        gBattleTextBuff3[2] = 4;
                        gBattleTextBuff3[3] = 5;
                        gBattleTextBuff3[4] = gBattleMoveDamage as u8;
                        gBattleTextBuff3[5] = ((gBattleMoveDamage & 0x0000FF00) >> 8) as u8;
                        gBattleTextBuff3[6] = ((gBattleMoveDamage & 0x00FF0000) >> 16) as u8;
                        gBattleTextBuff3[7] = ((gBattleMoveDamage as u32 & 0xFF000000) >> 24) as u8;
                        gBattleTextBuff3[8] = 0xFF;
                        PrepareStringBattle(
                            STRINGID_PKMNGAINEDEXP,
                            (*gBattleStruct).expGetterBattlerId,
                        );
                        MonGainEVs(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            gBattleMons[gBattlerFainted].species,
                        );
                    }
                    (*gBattleStruct).sentInPokes >>= 1;
                    gBattleScripting.getexpState += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if gBattleControllerExecFlags == 0 {
                gBattleBufferB[(*gBattleStruct).expGetterBattlerId][0] = 0;
                if GetMonData2(
                    &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                    MON_DATA_HP,
                ) != 0
                    && GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_LEVEL,
                    ) != MAX_LEVEL
                {
                    (*(*gBattleResources).beforeLvlUp).stats[0] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_MAX_HP,
                    ) as u16;
                    (*(*gBattleResources).beforeLvlUp).stats[1] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_ATK,
                    ) as u16;
                    (*(*gBattleResources).beforeLvlUp).stats[2] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_DEF,
                    ) as u16;
                    (*(*gBattleResources).beforeLvlUp).stats[3] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_SPEED,
                    ) as u16;
                    (*(*gBattleResources).beforeLvlUp).stats[4] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_SPATK,
                    ) as u16;
                    (*(*gBattleResources).beforeLvlUp).stats[5] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_SPDEF,
                    ) as u16;
                    gActiveBattler = (*gBattleStruct).expGetterBattlerId;
                    BtlController_EmitExpUpdate(
                        B_COMM_TO_CONTROLLER,
                        (*gBattleStruct).expGetterMonId,
                        gBattleMoveDamage as u16,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                }
                gBattleScripting.getexpState += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if gBattleControllerExecFlags == 0 {
                gActiveBattler = (*gBattleStruct).expGetterBattlerId;
                if gBattleBufferB[gActiveBattler][0] == CONTROLLER_TWORETURNVALUES
                    && gBattleBufferB[gActiveBattler][1] == RET_VALUE_LEVELED_UP
                {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
                        && gBattlerPartyIndexes[gActiveBattler]
                            == (*gBattleStruct).expGetterMonId as u16
                    {
                        HandleLowHpMusicChange(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                            gActiveBattler,
                        );
                    }
                    gBattleTextBuff1[0] = 0xFD;
                    gBattleTextBuff1[1] = 4;
                    gBattleTextBuff1[2] = gActiveBattler;
                    gBattleTextBuff1[3] = (*gBattleStruct).expGetterMonId;
                    gBattleTextBuff1[4] = 0xFF;
                    gBattleTextBuff2[0] = 0xFD;
                    gBattleTextBuff2[1] = 1;
                    gBattleTextBuff2[2] = 1;
                    gBattleTextBuff2[3] = 3;
                    gBattleTextBuff2[4] = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_LEVEL,
                    ) as u8;
                    gBattleTextBuff2[5] = 0xFF;
                    BattleScriptPushCursor();
                    gLeveledUpInBattle |= gBitTable[(*gBattleStruct).expGetterMonId] as u8;
                    gBattlescriptCurrInstr = BattleScript_LevelUp.as_ptr().cast_mut();
                    gBattleMoveDamage = gBattleBufferB[gActiveBattler][2] as i32
                        | (gBattleBufferB[gActiveBattler][3] as i32) << 8;
                    AdjustFriendship(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        FRIENDSHIP_EVENT_GROW_LEVEL,
                    );
                    if gBattlerPartyIndexes[0] == (*gBattleStruct).expGetterMonId as u16
                        && gBattleMons[0].hp != 0
                    {
                        gBattleMons[0].level = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_LEVEL,
                        ) as u8;
                        gBattleMons[0].hp = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_HP,
                        ) as u16;
                        gBattleMons[0].maxHP = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_MAX_HP,
                        ) as u16;
                        gBattleMons[0].attack = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_ATK,
                        ) as u16;
                        gBattleMons[0].defense = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_DEF,
                        ) as u16;
                        gBattleMons[0].speed = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPEED,
                        ) as u16;
                        gBattleMons[0].speed = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPEED,
                        ) as u16;
                        gBattleMons[0].spAttack = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPATK,
                        ) as u16;
                        gBattleMons[0].spDefense = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPDEF,
                        ) as u16;
                    }
                    if gBattlerPartyIndexes[2] == (*gBattleStruct).expGetterMonId as u16
                        && gBattleMons[2].hp != 0
                        && gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
                    {
                        gBattleMons[2].level = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_LEVEL,
                        ) as u8;
                        gBattleMons[2].hp = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_HP,
                        ) as u16;
                        gBattleMons[2].maxHP = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_MAX_HP,
                        ) as u16;
                        gBattleMons[2].attack = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_ATK,
                        ) as u16;
                        gBattleMons[2].defense = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_DEF,
                        ) as u16;
                        gBattleMons[2].speed = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPEED,
                        ) as u16;
                        gBattleMons[2].speed = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPEED,
                        ) as u16;
                        gBattleMons[2].spAttack = GetMonData2(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            MON_DATA_SPATK,
                        ) as u16;
                    }
                    gBattleScripting.getexpState = 5;
                } else {
                    gBattleMoveDamage = 0;
                    gBattleScripting.getexpState = 5;
                }
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if gBattleMoveDamage != 0 {
                gBattleScripting.getexpState = 3;
            } else {
                (*gBattleStruct).expGetterMonId += 1;
                if (*gBattleStruct).expGetterMonId < PARTY_SIZE as u8 {
                    gBattleScripting.getexpState = 2;
                } else {
                    gBattleScripting.getexpState = 6;
                }
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if gBattleControllerExecFlags == 0 {
                gBattleMons[gBattlerFainted].item = ITEM_NONE;
                gBattleMons[gBattlerFainted].ability = ABILITY_NONE;
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_checkteamslost() {
    let mut HP_count: u16 = 0;
    let mut i: i32 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId == TRAINER_STEVEN_PARTNER
    {
        i = 0;
        while i < MULTI_PARTY_SIZE {
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != 0
                && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
            {
                HP_count += GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) as u16;
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != 0
                && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
                && (gBattleTypeFlags & BATTLE_TYPE_ARENA == 0
                    || (*gBattleStruct).arenaLostPlayerMons as u32 & gBitTable[i] == 0)
            {
                HP_count += GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) as u16;
            }
            i += 1;
        }
    }
    if HP_count == 0 {
        gBattleOutcome |= B_OUTCOME_LOST;
    }
    HP_count = 0;
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData2(&raw mut gEnemyParty[i], MON_DATA_SPECIES) != 0
            && GetMonData2(&raw mut gEnemyParty[i], MON_DATA_IS_EGG) == 0
            && (gBattleTypeFlags & BATTLE_TYPE_ARENA == 0
                || (*gBattleStruct).arenaLostOpponentMons as u32 & gBitTable[i] == 0)
        {
            HP_count += GetMonData2(&raw mut gEnemyParty[i], MON_DATA_HP) as u16;
        }
        i += 1;
    }
    if HP_count == 0 {
        gBattleOutcome |= B_OUTCOME_WON;
    }
    if gBattleOutcome == 0 && gBattleTypeFlags & 0x2000002 != 0 {
        let mut emptyPlayerSpots: i32 = 0;
        let mut emptyOpponentSpots: i32 = 0;
        i = 0;
        while i < gBattlersCount as i32 {
            if gHitMarker & shl_i32(0x10000000, i as u32) as u32 != 0
                && gSpecialStatuses[i].faintedHasReplacement() == 0
            {
                emptyPlayerSpots += 1;
            }
            i += 2;
        }
        emptyOpponentSpots = 0;
        i = 1;
        while i < gBattlersCount as i32 {
            if gHitMarker & shl_i32(0x10000000, i as u32) as u32 != 0
                && gSpecialStatuses[i].faintedHasReplacement() == 0
            {
                emptyOpponentSpots += 1;
            }
            i += 2;
        }
        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
            if emptyOpponentSpots + emptyPlayerSpots > 1 {
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                    + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
                    + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
                    + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
                    as usize as *mut c_void as *mut u8;
            } else {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
            }
        } else {
            if emptyOpponentSpots != 0 && emptyPlayerSpots != 0 {
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                    + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
                    + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
                    + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
                    as usize as *mut c_void as *mut u8;
            } else {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
            }
        }
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn MoveValuesCleanUp() {
    gMoveResultFlags = 0;
    gBattleScripting.dmgMultiplier = 1;
    gCritMultiplier = 1;
    gBattleCommunication[3] = 0;
    gBattleCommunication[6] = 0;
    gHitMarker &= 0xffffffbf;
    gHitMarker &= 0xffffbfff;
}
pub(crate) unsafe extern "C" fn Cmd_movevaluescleanup() {
    MoveValuesCleanUp();
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setmultihit() {
    gMultiHitCounter = *gBattlescriptCurrInstr.at(1);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_decrementmultihit() {
    if ({
        gMultiHitCounter -= 1;
        gMultiHitCounter
    }) == 0
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
            + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
            + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
            as usize as *mut c_void as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_goto() {
    gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_jumpifbyte() {
    let mut caseID: u8 = *gBattlescriptCurrInstr.at(1);
    let mut memByte: *mut u8 = (*gBattlescriptCurrInstr.at(2) as i32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut value: u8 = *gBattlescriptCurrInstr.at(6);
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(7) as i32
        + ((*gBattlescriptCurrInstr.at(7).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(7).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(7).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(11);
    match caseID {
        CMP_EQUAL => {
            if *memByte == value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_NOT_EQUAL => {
            if *memByte != value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_GREATER_THAN => {
            if *memByte > value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_LESS_THAN => {
            if *memByte < value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_COMMON_BITS => {
            if *memByte as i32 & value as i32 != 0 {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_NO_COMMON_BITS => {
            if *memByte as i32 & value as i32 == 0 {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifhalfword() {
    let mut caseID: u8 = *gBattlescriptCurrInstr.at(1);
    let mut memHword: *mut u16 = (*gBattlescriptCurrInstr.at(2) as i32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    let mut value: u16 =
        *gBattlescriptCurrInstr.at(6) as u16 + ((*gBattlescriptCurrInstr.at(6).at(1) as u16) << 8);
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(8) as i32
        + ((*gBattlescriptCurrInstr.at(8).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(8).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(8).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(12);
    match caseID {
        CMP_EQUAL => {
            if *memHword == value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_NOT_EQUAL => {
            if *memHword != value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_GREATER_THAN => {
            if *memHword > value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_LESS_THAN => {
            if *memHword < value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_COMMON_BITS => {
            if *memHword as i32 & value as i32 != 0 {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_NO_COMMON_BITS => {
            if *memHword as i32 & value as i32 == 0 {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifword() {
    let mut caseID: u8 = *gBattlescriptCurrInstr.at(1);
    let mut memWord: *mut u32 = (*gBattlescriptCurrInstr.at(2) as i32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u32;
    let mut value: u32 = *gBattlescriptCurrInstr.at(6) as u32
        | (*gBattlescriptCurrInstr.at(6).at(1) as u32) << 8
        | (*gBattlescriptCurrInstr.at(6).at(2) as u32) << 16
        | (*gBattlescriptCurrInstr.at(6).at(3) as u32) << 24;
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(10) as i32
        + ((*gBattlescriptCurrInstr.at(10).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(10).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(10).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(14);
    match caseID {
        CMP_EQUAL => {
            if *memWord == value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_NOT_EQUAL => {
            if *memWord != value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_GREATER_THAN => {
            if *memWord > value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_LESS_THAN => {
            if *memWord < value {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_COMMON_BITS => {
            if *memWord & value != 0 {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        CMP_NO_COMMON_BITS => {
            if *memWord & value == 0 {
                gBattlescriptCurrInstr = jumpPtr;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifarrayequal() {
    let mut mem1: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut mem2: *mut u8 = (*gBattlescriptCurrInstr.at(5) as i32
        + ((*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut size: u32 = *gBattlescriptCurrInstr.at(9) as u32;
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(10) as i32
        + ((*gBattlescriptCurrInstr.at(10).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(10).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(10).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut i: u8 = 0;
    i = 0;
    while (i as u32) < size {
        if *mem1 != *mem2 {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(14);
            break;
        }
        mem1 = mem1.at(1);
        mem2 = mem2.at(1);
        i += 1;
    }
    if i as u32 == size {
        gBattlescriptCurrInstr = jumpPtr;
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifarraynotequal() {
    let mut equalBytes: u8 = 0;
    let mut mem1: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut mem2: *mut u8 = (*gBattlescriptCurrInstr.at(5) as i32
        + ((*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut size: u32 = *gBattlescriptCurrInstr.at(9) as u32;
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(10) as i32
        + ((*gBattlescriptCurrInstr.at(10).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(10).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(10).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut i: u8 = 0;
    i = 0;
    while (i as u32) < size {
        if *mem1 == *mem2 {
            equalBytes += 1;
        }
        mem1 = mem1.at(1);
        mem2 = mem2.at(1);
        i += 1;
    }
    if equalBytes as u32 != size {
        gBattlescriptCurrInstr = jumpPtr;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(14);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setbyte() {
    let mut memByte: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte = *gBattlescriptCurrInstr.at(5);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_addbyte() {
    let mut memByte: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte += *gBattlescriptCurrInstr.at(5);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_subbyte() {
    let mut memByte: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte -= *gBattlescriptCurrInstr.at(5);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_copyarray() {
    let mut dest: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut src: *mut u8 = (*gBattlescriptCurrInstr.at(5) as i32
        + ((*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut size: i32 = *gBattlescriptCurrInstr.at(9) as i32;
    let mut i: i32 = 0;
    i = 0;
    while i < size {
        *dest.at(i) = *src.at(i);
        i += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
}
pub(crate) unsafe extern "C" fn Cmd_copyarraywithindex() {
    let mut dest: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut src: *mut u8 = (*gBattlescriptCurrInstr.at(5) as i32
        + ((*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    let mut index: *mut u8 = (*gBattlescriptCurrInstr.at(9) as i32
        + ((*gBattlescriptCurrInstr.at(9).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(9).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(9).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    let mut size: i32 = *gBattlescriptCurrInstr.at(13) as i32;
    let mut i: i32 = 0;
    i = 0;
    while i < size {
        *dest.at(i) = *src.at(i + *index as i32);
        i += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(14);
}
pub(crate) unsafe extern "C" fn Cmd_orbyte() {
    let mut memByte: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte |= *gBattlescriptCurrInstr.at(5);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_orhalfword() {
    let mut memHword: *mut u16 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    let mut val: u16 =
        *gBattlescriptCurrInstr.at(5) as u16 + ((*gBattlescriptCurrInstr.at(5).at(1) as u16) << 8);
    *memHword |= val;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
}
pub(crate) unsafe extern "C" fn Cmd_orword() {
    let mut memWord: *mut u32 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u32;
    let mut val: u32 = *gBattlescriptCurrInstr.at(5) as u32
        + ((*gBattlescriptCurrInstr.at(5).at(1) as u32) << 8)
        + ((*gBattlescriptCurrInstr.at(5).at(2) as u32) << 16)
        + ((*gBattlescriptCurrInstr.at(5).at(3) as u32) << 24);
    *memWord |= val;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(9);
}
pub(crate) unsafe extern "C" fn Cmd_bicbyte() {
    let mut memByte: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u8;
    *memByte &= !*gBattlescriptCurrInstr.at(5);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_bichalfword() {
    let mut memHword: *mut u16 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u16;
    let mut val: u16 =
        *gBattlescriptCurrInstr.at(5) as u16 + ((*gBattlescriptCurrInstr.at(5).at(1) as u16) << 8);
    *memHword &= !val;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
}
pub(crate) unsafe extern "C" fn Cmd_bicword() {
    let mut memWord: *mut u32 = (*gBattlescriptCurrInstr.at(1) as i32
        + ((*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24))
        as usize as *mut c_void as *mut u32;
    let mut val: u32 = *gBattlescriptCurrInstr.at(5) as u32
        + ((*gBattlescriptCurrInstr.at(5).at(1) as u32) << 8)
        + ((*gBattlescriptCurrInstr.at(5).at(2) as u32) << 16)
        + ((*gBattlescriptCurrInstr.at(5).at(3) as u32) << 24);
    *memWord &= !val;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(9);
}
pub(crate) unsafe extern "C" fn Cmd_pause() {
    if gBattleControllerExecFlags == 0 {
        let mut value: u16 = *gBattlescriptCurrInstr.at(1) as u16
            + ((*gBattlescriptCurrInstr.at(1).at(1) as u16) << 8);
        if ({
            gPauseCounterBattle += 1;
            gPauseCounterBattle
        }) >= value
        {
            gPauseCounterBattle = 0;
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitstate() {
    if gBattleControllerExecFlags == 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_healthbar_update() {
    if *gBattlescriptCurrInstr.at(1) == BS_TARGET {
        gActiveBattler = gBattlerTarget;
    } else {
        gActiveBattler = gBattlerAttacker;
    }
    BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, gBattleMoveDamage as u16);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_return() {
    BattleScriptPop();
}
pub(crate) unsafe extern "C" fn Cmd_end() {
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        BattleArena_AddSkillPoints(gBattlerAttacker);
    }
    gMoveResultFlags = 0;
    gActiveBattler = 0;
    gCurrentActionFuncId = B_ACTION_TRY_FINISH;
}
pub(crate) unsafe extern "C" fn Cmd_end2() {
    gActiveBattler = 0;
    gCurrentActionFuncId = B_ACTION_TRY_FINISH;
}
pub(crate) unsafe extern "C" fn Cmd_end3() {
    BattleScriptPop();
    if (*(*gBattleResources).battleCallbackStack).size != 0 {
        (*(*gBattleResources).battleCallbackStack).size -= 1;
    }
    gBattleMainFunc = (*(*gBattleResources).battleCallbackStack).function
        [(*(*gBattleResources).battleCallbackStack).size];
}
pub(crate) unsafe extern "C" fn Cmd_call() {
    BattleScriptPush(gBattlescriptCurrInstr.at(5));
    gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24) as usize
        as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_jumpiftype2() {
    let mut battler: u8 = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    if *gBattlescriptCurrInstr.at(2) == gBattleMons[battler].types[0]
        || *gBattlescriptCurrInstr.at(2) == gBattleMons[battler].types[1]
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(3) as i32
            | (*gBattlescriptCurrInstr.at(3).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(3).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(3).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifabilitypresent() {
    if AbilityBattleEffects(
        ABILITYEFFECT_CHECK_ON_FIELD,
        0,
        *gBattlescriptCurrInstr.at(1),
        0,
        0,
    ) != 0
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
            | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_endselectionscript() {
    *(*gBattleStruct)
        .selectionScriptFinished
        .as_mut_ptr()
        .at(gBattlerAttacker) = TRUE;
}
pub(crate) unsafe extern "C" fn Cmd_playanimation() {
    let mut argumentPtr: *mut u16 = null_mut();
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    argumentPtr = (*gBattlescriptCurrInstr.at(3) as i32
        + ((*gBattlescriptCurrInstr.at(3).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(3).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(3).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u16;
    if *gBattlescriptCurrInstr.at(2) == B_ANIM_STATS_CHANGE
        || *gBattlescriptCurrInstr.at(2) == B_ANIM_SNATCH_MOVE
        || *gBattlescriptCurrInstr.at(2) == 2
    {
        BtlController_EmitBattleAnimation(
            B_COMM_TO_CONTROLLER,
            *gBattlescriptCurrInstr.at(2),
            *argumentPtr,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    } else if gHitMarker & HITMARKER_NO_ANIMATIONS != 0 {
        BattleScriptPush(gBattlescriptCurrInstr.at(7));
        gBattlescriptCurrInstr = BattleScript_Pausex20.as_ptr().cast_mut();
    } else if *gBattlescriptCurrInstr.at(2) == B_ANIM_RAIN_CONTINUES
        || *gBattlescriptCurrInstr.at(2) == B_ANIM_SUN_CONTINUES
        || *gBattlescriptCurrInstr.at(2) == B_ANIM_SANDSTORM_CONTINUES
        || *gBattlescriptCurrInstr.at(2) == B_ANIM_HAIL_CONTINUES
    {
        BtlController_EmitBattleAnimation(
            B_COMM_TO_CONTROLLER,
            *gBattlescriptCurrInstr.at(2),
            *argumentPtr,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    } else if gStatuses3[gActiveBattler] & STATUS3_SEMI_INVULNERABLE != 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    } else {
        BtlController_EmitBattleAnimation(
            B_COMM_TO_CONTROLLER,
            *gBattlescriptCurrInstr.at(2),
            *argumentPtr,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_playanimation_var() {
    let mut argumentPtr: *mut u16 = null_mut();
    let mut animationIdPtr: *mut u8 = null_mut();
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    animationIdPtr = (*gBattlescriptCurrInstr.at(2) as i32
        + ((*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
    argumentPtr = (*gBattlescriptCurrInstr.at(6) as i32
        + ((*gBattlescriptCurrInstr.at(6).at(1) as i32) << 8)
        + ((*gBattlescriptCurrInstr.at(6).at(2) as i32) << 16)
        + ((*gBattlescriptCurrInstr.at(6).at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u16;
    if *animationIdPtr == B_ANIM_STATS_CHANGE
        || *animationIdPtr == B_ANIM_SNATCH_MOVE
        || *animationIdPtr == B_ANIM_SUBSTITUTE_FADE
    {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, *animationIdPtr, *argumentPtr);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    } else if gHitMarker & HITMARKER_NO_ANIMATIONS != 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    } else if *animationIdPtr == B_ANIM_RAIN_CONTINUES
        || *animationIdPtr == B_ANIM_SUN_CONTINUES
        || *animationIdPtr == B_ANIM_SANDSTORM_CONTINUES
        || *animationIdPtr == B_ANIM_HAIL_CONTINUES
    {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, *animationIdPtr, *argumentPtr);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    } else if gStatuses3[gActiveBattler] & STATUS3_SEMI_INVULNERABLE != 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    } else {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, *animationIdPtr, *argumentPtr);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setgraphicalstatchangevalues() {
    let mut value: u8 = 0;
    match gBattleScripting.statChanger as i32 & 0xF0 {
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
    gBattleScripting.animArg1 = (gBattleScripting.statChanger & 0xF) + value - 1;
    gBattleScripting.animArg2 = 0;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_playstatchangeanimation() {
    let mut currStat: u32 = 0;
    let mut statAnimId: u16 = 0;
    let mut changeableStatsCount: i32 = 0;
    let mut statsToCheck: u8 = 0;
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    statsToCheck = *gBattlescriptCurrInstr.at(2);
    if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_NEGATIVE != 0 {
        let mut startingStatAnimId: i16 = 0;
        if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
            startingStatAnimId = STAT_ANIM_MINUS2;
        } else {
            startingStatAnimId = STAT_ANIM_MINUS1;
        }
        while statsToCheck != 0 {
            if statsToCheck as i32 & 1 != 0 {
                if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_CANT_PREVENT != 0 {
                    if gBattleMons[gActiveBattler].statStages[currStat] > MIN_STAT_STAGE {
                        statAnimId = startingStatAnimId as u16 + currStat as u16;
                        changeableStatsCount += 1;
                    }
                } else if gSideTimers[GetBattlerPosition(gActiveBattler) as i32 & 1].mistTimer == 0
                    && gBattleMons[gActiveBattler].ability != ABILITY_CLEAR_BODY
                    && gBattleMons[gActiveBattler].ability != ABILITY_WHITE_SMOKE
                    && !(gBattleMons[gActiveBattler].ability == ABILITY_KEEN_EYE
                        && currStat == STAT_ACC)
                    && !(gBattleMons[gActiveBattler].ability == ABILITY_HYPER_CUTTER
                        && currStat == STAT_ATK as u32)
                {
                    if gBattleMons[gActiveBattler].statStages[currStat] > MIN_STAT_STAGE {
                        statAnimId = startingStatAnimId as u16 + currStat as u16;
                        changeableStatsCount += 1;
                    }
                }
            }
            statsToCheck >>= 1;
            currStat += 1;
        }
        if changeableStatsCount > 1 {
            if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
                statAnimId = STAT_ANIM_MULTIPLE_MINUS2;
            } else {
                statAnimId = STAT_ANIM_MULTIPLE_MINUS1;
            }
        }
    } else {
        let mut startingStatAnimId: i16 = 0;
        if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
            startingStatAnimId = STAT_ANIM_PLUS2;
        } else {
            startingStatAnimId = STAT_ANIM_PLUS1;
        }
        while statsToCheck != 0 {
            if statsToCheck as i32 & 1 != 0
                && gBattleMons[gActiveBattler].statStages[currStat] < MAX_STAT_STAGE
            {
                statAnimId = startingStatAnimId as u16 + currStat as u16;
                changeableStatsCount += 1;
            }
            statsToCheck >>= 1;
            currStat += 1;
        }
        if changeableStatsCount > 1 {
            if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_BY_TWO != 0 {
                statAnimId = STAT_ANIM_MULTIPLE_PLUS2;
            } else {
                statAnimId = STAT_ANIM_MULTIPLE_PLUS1;
            }
        }
    }
    if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_MULTIPLE_STATS != 0
        && changeableStatsCount < 2
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(4);
    } else if changeableStatsCount != 0 && gBattleScripting.statAnimPlayed == 0 {
        BtlController_EmitBattleAnimation(B_COMM_TO_CONTROLLER, B_ANIM_STATS_CHANGE, statAnimId);
        MarkBattlerForControllerExec(gActiveBattler);
        if *gBattlescriptCurrInstr.at(3) as i32 & STAT_CHANGE_MULTIPLE_STATS != 0
            && changeableStatsCount > 1
        {
            gBattleScripting.statAnimPlayed = TRUE;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(4);
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(4);
    }
}
pub(crate) unsafe extern "C" fn Cmd_moveend() {
    let mut i: i32 = 0;
    let mut effect: u32 = FALSE as u32;
    let mut moveType: u8 = 0;
    let mut holdEffectAtk: u8 = 0;
    let mut choicedMoveAtk: *mut u16 = null_mut();
    let mut endMode: u8 = 0;
    let mut endState: u8 = 0;
    let mut originallyUsedMove: u16 = 0;
    if gChosenMove == MOVE_UNAVAILABLE {
        originallyUsedMove = MOVE_NONE;
    } else {
        originallyUsedMove = gChosenMove;
    }
    endMode = *gBattlescriptCurrInstr.at(1);
    endState = *gBattlescriptCurrInstr.at(2);
    if gBattleMons[gBattlerAttacker].item == ITEM_ENIGMA_BERRY {
        holdEffectAtk = gEnigmaBerries[gBattlerAttacker].holdEffect;
    } else {
        holdEffectAtk = GetItemHoldEffect(gBattleMons[gBattlerAttacker].item);
    }
    choicedMoveAtk = &raw mut (*gBattleStruct).choicedMove[gBattlerAttacker];
    if (*gBattleStruct).dynamicMoveType != 0 {
        moveType = (*gBattleStruct).dynamicMoveType & 63;
    } else {
        moveType = gBattleMoves[gCurrentMove].r#type;
    }
    loop {
        'l2: {
            match gBattleScripting.moveendState {
                MOVEEND_RAGE => {
                    if gBattleMons[gBattlerTarget].status2 & STATUS2_RAGE != 0
                        && gBattleMons[gBattlerTarget].hp != 0
                        && gBattlerAttacker != gBattlerTarget
                        && GetBattlerSide(gBattlerAttacker) != GetBattlerSide(gBattlerTarget)
                        && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && (gSpecialStatuses[gBattlerTarget].physicalDmg != 0
                            || gSpecialStatuses[gBattlerTarget].specialDmg != 0)
                        && gBattleMoves[gCurrentMove].power != 0
                        && gBattleMons[gBattlerTarget].statStages[1] < MAX_STAT_STAGE
                    {
                        gBattleMons[gBattlerTarget].statStages[1] += 1;
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr = BattleScript_RageIsBuilding.as_ptr().cast_mut();
                        effect = TRUE as u32;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_DEFROST => {
                    if gBattleMons[gBattlerTarget].status1 & STATUS1_FREEZE != 0
                        && gBattleMons[gBattlerTarget].hp != 0
                        && gBattlerAttacker != gBattlerTarget
                        && gSpecialStatuses[gBattlerTarget].specialDmg != 0
                        && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        && moveType == TYPE_FIRE
                    {
                        gBattleMons[gBattlerTarget].status1 &= 0xffffffdf;
                        gActiveBattler = gBattlerTarget;
                        BtlController_EmitSetMonData(
                            B_COMM_TO_CONTROLLER,
                            REQUEST_STATUS_BATTLE,
                            0,
                            4,
                            &raw mut gBattleMons[gBattlerTarget].status1 as *mut c_void,
                        );
                        MarkBattlerForControllerExec(gActiveBattler);
                        BattleScriptPushCursor();
                        gBattlescriptCurrInstr =
                            BattleScript_DefrostedViaFireMove.as_ptr().cast_mut();
                        effect = TRUE as u32;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_SYNCHRONIZE_TARGET => {
                    if AbilityBattleEffects(ABILITYEFFECT_SYNCHRONIZE, gBattlerTarget, 0, 0, 0) != 0
                    {
                        effect = TRUE as u32;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_ON_DAMAGE_ABILITIES => {
                    if AbilityBattleEffects(ABILITYEFFECT_ON_DAMAGE, gBattlerTarget, 0, 0, 0) != 0 {
                        effect = TRUE as u32;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_IMMUNITY_ABILITIES => {
                    if AbilityBattleEffects(ABILITYEFFECT_IMMUNITY, 0, 0, 0, 0) != 0 {
                        effect = TRUE as u32;
                    } else {
                        gBattleScripting.moveendState += 1;
                    }
                }
                MOVEEND_SYNCHRONIZE_ATTACKER => {
                    if AbilityBattleEffects(
                        ABILITYEFFECT_ATK_SYNCHRONIZE,
                        gBattlerAttacker,
                        0,
                        0,
                        0,
                    ) != 0
                    {
                        effect = TRUE as u32;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_CHOICE_MOVE => {
                    if gHitMarker & HITMARKER_OBEYS != 0
                        && holdEffectAtk == HOLD_EFFECT_CHOICE_BAND
                        && gChosenMove != MOVE_STRUGGLE
                        && (*choicedMoveAtk == MOVE_NONE || *choicedMoveAtk == MOVE_UNAVAILABLE)
                    {
                        if gChosenMove == MOVE_BATON_PASS
                            && gMoveResultFlags as i32 & MOVE_RESULT_FAILED == 0
                        {
                            gBattleScripting.moveendState += 1;
                            break 'l2;
                        }
                        *choicedMoveAtk = gChosenMove;
                    }
                    i = 0;
                    while i < MAX_MON_MOVES {
                        if gBattleMons[gBattlerAttacker].moves[i] == *choicedMoveAtk {
                            break;
                        }
                        i += 1;
                    }
                    if i == MAX_MON_MOVES {
                        *choicedMoveAtk = MOVE_NONE;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_CHANGED_ITEMS => {
                    i = 0;
                    while i < gBattlersCount as i32 {
                        let mut changedItem: *mut u16 = &raw mut (*gBattleStruct).changedItems[i];
                        if *changedItem != ITEM_NONE {
                            gBattleMons[i].item = *changedItem;
                            *changedItem = ITEM_NONE;
                        }
                        i += 1;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_ITEM_EFFECTS_ALL => {
                    if ItemBattleEffects(ITEMEFFECT_MOVE_END, 0, 0) != 0 {
                        effect = TRUE as u32;
                    } else {
                        gBattleScripting.moveendState += 1;
                    }
                }
                MOVEEND_KINGSROCK_SHELLBELL => {
                    if ItemBattleEffects(ITEMEFFECT_KINGSROCK_SHELLBELL, 0, 0) != 0 {
                        effect = TRUE as u32;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_ATTACKER_INVISIBLE => {
                    if gStatuses3[gBattlerAttacker] & STATUS3_SEMI_INVULNERABLE != 0
                        && gHitMarker & HITMARKER_NO_ANIMATIONS != 0
                    {
                        gActiveBattler = gBattlerAttacker;
                        BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, TRUE);
                        MarkBattlerForControllerExec(gActiveBattler);
                        gBattleScripting.moveendState += 1;
                        return;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_ATTACKER_VISIBLE => {
                    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0
                        || gStatuses3[gBattlerAttacker] & STATUS3_SEMI_INVULNERABLE == 0
                        || WasUnableToUseMove(gBattlerAttacker) != 0
                    {
                        gActiveBattler = gBattlerAttacker;
                        BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, FALSE);
                        MarkBattlerForControllerExec(gActiveBattler);
                        gStatuses3[gBattlerAttacker] &= 0xfffbff3f;
                        gSpecialStatuses[gBattlerAttacker].set_restoredBattlerSprite(1);
                        gBattleScripting.moveendState += 1;
                        return;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_TARGET_VISIBLE => {
                    if gSpecialStatuses[gBattlerTarget].restoredBattlerSprite() == 0
                        && gBattlerTarget < gBattlersCount
                        && gStatuses3[gBattlerTarget] & STATUS3_SEMI_INVULNERABLE == 0
                    {
                        gActiveBattler = gBattlerTarget;
                        BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, FALSE);
                        MarkBattlerForControllerExec(gActiveBattler);
                        gStatuses3[gBattlerTarget] &= 0xfffbff3f;
                        gBattleScripting.moveendState += 1;
                        return;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_SUBSTITUTE => {
                    i = 0;
                    while i < gBattlersCount as i32 {
                        if gDisableStructs[i].substituteHP == 0 {
                            gBattleMons[i].status2 &= 0xfeffffff;
                        }
                        i += 1;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_UPDATE_LAST_MOVES => {
                    if gHitMarker & HITMARKER_SWAP_ATTACKER_TARGET != 0 {
                        gActiveBattler = gBattlerAttacker;
                        gBattlerAttacker = gBattlerTarget;
                        gBattlerTarget = gActiveBattler;
                        gHitMarker &= 0xffffefff;
                    }
                    if gHitMarker & HITMARKER_ATTACKSTRING_PRINTED != 0 {
                        gLastPrintedMoves[gBattlerAttacker] = gChosenMove;
                    }
                    if gAbsentBattlerFlags as u32 & gBitTable[gBattlerAttacker] == 0
                        && (*gBattleStruct).absentBattlerFlags as u32 & gBitTable[gBattlerAttacker]
                            == 0
                        && gBattleMoves[originallyUsedMove].effect != EFFECT_BATON_PASS
                    {
                        if gHitMarker & HITMARKER_OBEYS != 0 {
                            gLastMoves[gBattlerAttacker] = gChosenMove;
                            gLastResultingMoves[gBattlerAttacker] = gCurrentMove;
                        } else {
                            gLastMoves[gBattlerAttacker] = MOVE_UNAVAILABLE;
                            gLastResultingMoves[gBattlerAttacker] = MOVE_UNAVAILABLE;
                        }
                        if gHitMarker & gBitTable[gBattlerTarget] << 28 == 0 {
                            gLastHitBy[gBattlerTarget] = gBattlerAttacker;
                        }
                        if gHitMarker & HITMARKER_OBEYS != 0
                            && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                        {
                            if gChosenMove == MOVE_UNAVAILABLE {
                                gLastLandedMoves[gBattlerTarget] = gChosenMove;
                            } else {
                                gLastLandedMoves[gBattlerTarget] = gCurrentMove;
                                if (*gBattleStruct).dynamicMoveType != 0 {
                                    gLastHitByType[gBattlerTarget] =
                                        (*gBattleStruct).dynamicMoveType as u16 & 63;
                                } else {
                                    gLastHitByType[gBattlerTarget] =
                                        gBattleMoves[gCurrentMove].r#type as u16;
                                }
                            }
                        } else {
                            gLastLandedMoves[gBattlerTarget] = MOVE_UNAVAILABLE;
                        }
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_MIRROR_MOVE => {
                    if gAbsentBattlerFlags as u32 & gBitTable[gBattlerAttacker] == 0
                        && (*gBattleStruct).absentBattlerFlags as u32 & gBitTable[gBattlerAttacker]
                            == 0
                        && gBattleMoves[originallyUsedMove].flags as i32 & FLAG_MIRROR_MOVE_AFFECTED
                            != 0
                        && gHitMarker & HITMARKER_OBEYS != 0
                        && gBattlerAttacker != gBattlerTarget
                        && gHitMarker & gBitTable[gBattlerTarget] << 28 == 0
                        && gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT == 0
                    {
                        let mut target: u8 = 0;
                        let mut attacker: u8 = 0;
                        *(*gBattleStruct)
                            .lastTakenMove
                            .as_mut_ptr()
                            .at(gBattlerTarget as i32 * 2) = gChosenMove as u8;
                        *(*gBattleStruct)
                            .lastTakenMove
                            .as_mut_ptr()
                            .at(gBattlerTarget as i32 * 2)
                            .at(1) = (gChosenMove >> 8) as u8;
                        target = gBattlerTarget;
                        attacker = gBattlerAttacker;
                        *(*gBattleStruct)
                            .lastTakenMoveFrom
                            .as_mut_ptr()
                            .at(attacker as i32 * 2 + target as i32 * 8) = gChosenMove as u8;
                        target = gBattlerTarget;
                        attacker = gBattlerAttacker;
                        *(*gBattleStruct)
                            .lastTakenMoveFrom
                            .as_mut_ptr()
                            .at(attacker as i32 * 2 + target as i32 * 8)
                            .at(1) = (gChosenMove >> 8) as u8;
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_NEXT_TARGET => {
                    if gHitMarker & HITMARKER_UNABLE_TO_USE_MOVE == 0
                        && gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
                        && gProtectStructs[gBattlerAttacker].chargingTurn() == 0
                        && gBattleMoves[gCurrentMove].target == MOVE_TARGET_BOTH
                        && gHitMarker & HITMARKER_NO_ATTACKSTRING == 0
                    {
                        let mut battler: u8 =
                            GetBattlerAtPosition(GetBattlerPosition(gBattlerTarget) ^ 2);
                        if gBattleMons[battler].hp != 0 {
                            gBattlerTarget = battler;
                            gHitMarker |= HITMARKER_NO_ATTACKSTRING;
                            gBattleScripting.moveendState = 0;
                            MoveValuesCleanUp();
                            BattleScriptPush(
                                gBattleScriptsForMoveEffects[gBattleMoves[gCurrentMove].effect],
                            );
                            gBattlescriptCurrInstr =
                                BattleScript_FlushMessageBox.as_ptr().cast_mut();
                            return;
                        } else {
                            gHitMarker |= HITMARKER_NO_ATTACKSTRING;
                        }
                    }
                    gBattleScripting.moveendState += 1;
                }
                MOVEEND_COUNT => {}
                _ => {}
            }
        }
        if endMode == 1 && effect == FALSE as u32 {
            gBattleScripting.moveendState = MOVEEND_COUNT;
        }
        if endMode == 2 && endState == gBattleScripting.moveendState {
            gBattleScripting.moveendState = MOVEEND_COUNT;
        }
        if !(gBattleScripting.moveendState != MOVEEND_COUNT && effect == FALSE as u32) {
            break;
        }
    }
    if gBattleScripting.moveendState == MOVEEND_COUNT && effect == FALSE as u32 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
    }
}
pub(crate) unsafe extern "C" fn Cmd_typecalc2() {
    let mut flags: u8 = 0;
    let mut i: i32 = 0;
    let mut moveType: u8 = gBattleMoves[gCurrentMove].r#type;
    if gBattleMons[gBattlerTarget].ability == ABILITY_LEVITATE && moveType == TYPE_GROUND {
        gLastUsedAbility = gBattleMons[gBattlerTarget].ability;
        gMoveResultFlags |= 9;
        gLastLandedMoves[gBattlerTarget] = 0;
        gBattleCommunication[6] = B_MSG_GROUND_MISS;
        RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
    } else {
        while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
            if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
                if gBattleMons[gBattlerTarget].status2 & STATUS2_FORESIGHT != 0 {
                    break;
                } else {
                    i += 3;
                    continue;
                }
            }
            if gTypeEffectiveness[i + 0] == moveType {
                if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[0] {
                    if gTypeEffectiveness[i + 2] == TYPE_MUL_NO_EFFECT {
                        gMoveResultFlags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
                        break;
                    }
                    if gTypeEffectiveness[i + 2] == TYPE_MUL_NOT_EFFECTIVE {
                        flags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                    }
                    if gTypeEffectiveness[i + 2] == TYPE_MUL_SUPER_EFFECTIVE {
                        flags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
                    }
                }
                if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1] {
                    if gBattleMons[gBattlerTarget].types[0] != gBattleMons[gBattlerTarget].types[1]
                        && gTypeEffectiveness[i + 2] == TYPE_MUL_NO_EFFECT
                    {
                        gMoveResultFlags |= MOVE_RESULT_DOESNT_AFFECT_FOE;
                        break;
                    }
                    if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1]
                        && gBattleMons[gBattlerTarget].types[0]
                            != gBattleMons[gBattlerTarget].types[1]
                        && gTypeEffectiveness[i + 2] == TYPE_MUL_NOT_EFFECTIVE
                    {
                        flags |= MOVE_RESULT_NOT_VERY_EFFECTIVE as u8;
                    }
                    if gTypeEffectiveness[i + 1] == gBattleMons[gBattlerTarget].types[1]
                        && gBattleMons[gBattlerTarget].types[0]
                            != gBattleMons[gBattlerTarget].types[1]
                        && gTypeEffectiveness[i + 2] == TYPE_MUL_SUPER_EFFECTIVE
                    {
                        flags |= MOVE_RESULT_SUPER_EFFECTIVE as u8;
                    }
                }
            }
            i += 3;
        }
    }
    if gBattleMons[gBattlerTarget].ability == ABILITY_WONDER_GUARD
        && flags as i32 & MOVE_RESULT_NO_EFFECT == 0
        && AttacksThisTurn(gBattlerAttacker, gCurrentMove) == 2
        && (flags as i32 & MOVE_RESULT_SUPER_EFFECTIVE == 0 || flags as i32 & 6 == 6)
        && gBattleMoves[gCurrentMove].power != 0
    {
        gLastUsedAbility = ABILITY_WONDER_GUARD;
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gLastLandedMoves[gBattlerTarget] = 0;
        gBattleCommunication[6] = B_MSG_AVOIDED_DMG;
        RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
    }
    if gMoveResultFlags as i32 & MOVE_RESULT_DOESNT_AFFECT_FOE as i32 != 0 {
        gProtectStructs[gBattlerAttacker].set_targetNotAffected(1);
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_returnatktoball() {
    gActiveBattler = gBattlerAttacker;
    if gHitMarker & gBitTable[gActiveBattler] << 28 == 0 {
        BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, FALSE);
        MarkBattlerForControllerExec(gActiveBattler);
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_getswitchedmondata() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    gBattlerPartyIndexes[gActiveBattler] = *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler) as u16;
    BtlController_EmitGetMonData(
        B_COMM_TO_CONTROLLER,
        REQUEST_ALL_BATTLE,
        gBitTable[gBattlerPartyIndexes[gActiveBattler]] as u8,
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_switchindataupdate() {
    let mut oldData: BattlePokemon = zeroed();
    let mut i: i32 = 0;
    let mut monData: *mut u8 = null_mut();
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    oldData = gBattleMons[gActiveBattler];
    monData = &raw mut gBattleMons[gActiveBattler] as *mut u8;
    i = 0;
    while i < 88 {
        *monData.at(i) = gBattleBufferB[gActiveBattler][4 + i];
        i += 1;
    }
    gBattleMons[gActiveBattler].types[0] =
        gSpeciesInfo[gBattleMons[gActiveBattler].species].types[0];
    gBattleMons[gActiveBattler].types[1] =
        gSpeciesInfo[gBattleMons[gActiveBattler].species].types[1];
    gBattleMons[gActiveBattler].ability = GetAbilityBySpecies(
        gBattleMons[gActiveBattler].species,
        gBattleMons[gActiveBattler].abilityNum() as u8,
    );
    i = GetBattlerSide(gActiveBattler) as i32;
    if gWishFutureKnock.knockedOffMons[i] as u32 & gBitTable[gBattlerPartyIndexes[gActiveBattler]]
        != 0
    {
        gBattleMons[gActiveBattler].item = ITEM_NONE;
    }
    if gBattleMoves[gCurrentMove].effect == EFFECT_BATON_PASS {
        i = 0;
        while i < NUM_BATTLE_STATS {
            gBattleMons[gActiveBattler].statStages[i] = oldData.statStages[i];
            i += 1;
        }
        gBattleMons[gActiveBattler].status2 = oldData.status2;
    }
    SwitchInClearSetData();
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0
        && gBattleMons[gActiveBattler].maxHP as i32 / 2 >= gBattleMons[gActiveBattler].hp as i32
        && gBattleMons[gActiveBattler].hp != 0
        && gBattleMons[gActiveBattler].status1 & STATUS1_SLEEP == 0
    {
        (*gBattleStruct).palaceFlags |= gBitTable[gActiveBattler] as u8;
    }
    gBattleScripting.battler = gActiveBattler;
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 7;
    gBattleTextBuff1[2] = gActiveBattler;
    gBattleTextBuff1[3] = gBattlerPartyIndexes[gActiveBattler] as u8;
    gBattleTextBuff1[4] = 0xFF;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_switchinanim() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT && gBattleTypeFlags & 0x63f0902 == 0 {
        HandleSetPokedexFlag(
            SpeciesToNationalPokedexNum(gBattleMons[gActiveBattler].species),
            FLAG_SET_SEEN,
            gBattleMons[gActiveBattler].personality,
        );
    }
    gAbsentBattlerFlags &= !(gBitTable[gActiveBattler] as u8);
    BtlController_EmitSwitchInAnim(
        B_COMM_TO_CONTROLLER,
        gBattlerPartyIndexes[gActiveBattler] as u8,
        *gBattlescriptCurrInstr.at(2),
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        BattleArena_InitPoints();
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifcantswitch() {
    let mut i: i32 = 0;
    let mut lastMonId: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1) & 127);
    if *gBattlescriptCurrInstr.at(1) as i32 & SWITCH_IGNORE_ESCAPE_PREVENTION == 0
        && (gBattleMons[gActiveBattler].status2 & 0x400e000 != 0
            || gStatuses3[gActiveBattler] & STATUS3_ROOTED != 0)
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
            | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
            as usize as *mut u8;
    } else if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
        if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT {
            party = gEnemyParty.as_mut_ptr();
        } else {
            party = gPlayerParty.as_mut_ptr();
        }
        lastMonId = 0;
        if gActiveBattler as i32 & 2 != 0 {
            lastMonId = MULTI_PARTY_SIZE;
        }
        i = lastMonId;
        while i < lastMonId + MULTI_PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && gBattlerPartyIndexes[gActiveBattler] as i32 != i
            {
                break;
            }
            i += 1;
        }
        if i == lastMonId + MULTI_PARTY_SIZE {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
                | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                party = gPlayerParty.as_mut_ptr();
                lastMonId = 0;
                if GetLinkTrainerFlankId(GetBattlerMultiplayerId(gActiveBattler as u16) as u8)
                    == TRUE as u16
                {
                    lastMonId = MULTI_PARTY_SIZE;
                }
            } else {
                party = gEnemyParty.as_mut_ptr();
                if gActiveBattler == 1 {
                    lastMonId = 0;
                } else {
                    lastMonId = MULTI_PARTY_SIZE;
                }
            }
        } else {
            if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT {
                party = gEnemyParty.as_mut_ptr();
            } else {
                party = gPlayerParty.as_mut_ptr();
            }
            lastMonId = 0;
            if GetLinkTrainerFlankId(GetBattlerMultiplayerId(gActiveBattler as u16) as u8)
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
                && gBattlerPartyIndexes[gActiveBattler] as i32 != i
            {
                break;
            }
            i += 1;
        }
        if i == lastMonId + MULTI_PARTY_SIZE {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
                | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0
        && GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT
    {
        party = gEnemyParty.as_mut_ptr();
        lastMonId = 0;
        if gActiveBattler == B_POSITION_OPPONENT_RIGHT {
            lastMonId = 3;
        }
        i = lastMonId;
        while i < lastMonId + 3 {
            if GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && GetMonData2(party.at(i), MON_DATA_HP) != 0
                && gBattlerPartyIndexes[gActiveBattler] as i32 != i
            {
                break;
            }
            i += 1;
        }
        if i == lastMonId + 3 {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
                | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
        }
    } else {
        let mut battlerIn1: u8 = 0;
        let mut battlerIn2: u8 = 0;
        if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT {
            battlerIn1 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                battlerIn2 = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
            } else {
                battlerIn2 = battlerIn1;
            }
            party = gEnemyParty.as_mut_ptr();
        } else {
            battlerIn1 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                battlerIn2 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            } else {
                battlerIn2 = battlerIn1;
            }
            party = gPlayerParty.as_mut_ptr();
        }
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(party.at(i), MON_DATA_HP) != 0
                && GetMonData2(party.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(party.at(i), MON_DATA_IS_EGG) == 0
                && i != gBattlerPartyIndexes[battlerIn1] as i32
                && i != gBattlerPartyIndexes[battlerIn2] as i32
            {
                break;
            }
            i += 1;
        }
        if i == PARTY_SIZE {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
                | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseMonToSendOut(slotId: u8) {
    *(*gBattleStruct)
        .battlerPartyIndexes
        .as_mut_ptr()
        .at(gActiveBattler) = gBattlerPartyIndexes[gActiveBattler] as u8;
    *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler) = PARTY_SIZE as u8;
    (*gBattleStruct).field_93 &= !(gBitTable[gActiveBattler] as u8);
    BtlController_EmitChoosePokemon(
        B_COMM_TO_CONTROLLER,
        PARTY_ACTION_SEND_OUT,
        slotId,
        ABILITY_NONE,
        (*gBattleStruct).battlerPartyOrders[gActiveBattler].as_mut_ptr(),
    );
    MarkBattlerForControllerExec(gActiveBattler);
}
pub(crate) unsafe extern "C" fn Cmd_openpartyscreen() {
    let mut flags: u32 = 0;
    let mut hitmarkerFaintBits: u8 = 0;
    let mut battler: u8 = 0;
    let mut jumpPtr: *mut u8 = null_mut();
    battler = 0;
    flags = 0;
    jumpPtr = (*gBattlescriptCurrInstr.at(2) as i32
        | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24) as usize as *mut u8;
    if *gBattlescriptCurrInstr.at(1) == BS_FAINTED_LINK_MULTIPLE_1 {
        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 || gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
            gActiveBattler = 0;
            while gActiveBattler < gBattlersCount {
                if gHitMarker & gBitTable[gActiveBattler] << 28 != 0 {
                    if HasNoMonsToSwitch(gActiveBattler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
                        gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                        gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                        BtlController_EmitLinkStandbyMsg(
                            B_COMM_TO_CONTROLLER,
                            LINK_STANDBY_MSG_ONLY,
                            FALSE as u32,
                        );
                        MarkBattlerForControllerExec(gActiveBattler);
                    } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                        ChooseMonToSendOut(PARTY_SIZE as u8);
                        gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                    }
                } else {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                }
                gActiveBattler += 1;
            }
        } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
            let mut hasReplacement_0: u8 = 0;
            let mut hasReplacement_1: u8 = 0;
            let mut hasReplacement_2: u8 = 0;
            let mut hasReplacement_3: u8 = 0;
            hitmarkerFaintBits = (gHitMarker >> 28) as u8;
            if gBitTable[0] & hitmarkerFaintBits as u32 != 0 {
                gActiveBattler = 0;
                if HasNoMonsToSwitch(gActiveBattler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
                    gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                    gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler);
                } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                    ChooseMonToSendOut((*gBattleStruct).monToSwitchIntoId[2]);
                    gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                } else {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                    flags |= 1;
                }
            }
            if gBitTable[2] & hitmarkerFaintBits as u32 != 0
                && gBitTable[0] & hitmarkerFaintBits as u32 == 0
            {
                gActiveBattler = 2;
                if HasNoMonsToSwitch(gActiveBattler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
                    gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                    gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler);
                } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                    ChooseMonToSendOut((*gBattleStruct).monToSwitchIntoId[0]);
                    gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                } else if flags & 1 == 0 {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                }
            }
            if gBitTable[1] & hitmarkerFaintBits as u32 != 0 {
                gActiveBattler = 1;
                if HasNoMonsToSwitch(gActiveBattler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
                    gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                    gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler);
                } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                    ChooseMonToSendOut((*gBattleStruct).monToSwitchIntoId[3]);
                    gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                } else {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                    flags |= 2;
                }
            }
            if gBitTable[3] & hitmarkerFaintBits as u32 != 0
                && gBitTable[1] & hitmarkerFaintBits as u32 == 0
            {
                gActiveBattler = 3;
                if HasNoMonsToSwitch(gActiveBattler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
                    gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                    gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                    BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                    MarkBattlerForControllerExec(gActiveBattler);
                } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                    ChooseMonToSendOut((*gBattleStruct).monToSwitchIntoId[1]);
                    gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                } else if flags & 2 == 0 {
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                }
            }
            hasReplacement_0 = gSpecialStatuses[0].faintedHasReplacement() as u8;
            if hasReplacement_0 == 0 {
                hasReplacement_2 = gSpecialStatuses[2].faintedHasReplacement() as u8;
                if hasReplacement_2 == 0 && hitmarkerFaintBits != 0 {
                    if gAbsentBattlerFlags as u32 & gBitTable[0] != 0 {
                        gActiveBattler = 2;
                    } else {
                        gActiveBattler = 0;
                    }
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                }
            }
            hasReplacement_1 = gSpecialStatuses[1].faintedHasReplacement() as u8;
            if hasReplacement_1 == 0 {
                hasReplacement_3 = gSpecialStatuses[3].faintedHasReplacement() as u8;
                if hasReplacement_3 == 0 && hitmarkerFaintBits != 0 {
                    if gAbsentBattlerFlags as u32 & gBitTable[1] != 0 {
                        gActiveBattler = 3;
                    } else {
                        gActiveBattler = 1;
                    }
                    BtlController_EmitLinkStandbyMsg(
                        B_COMM_TO_CONTROLLER,
                        LINK_STANDBY_MSG_ONLY,
                        FALSE as u32,
                    );
                    MarkBattlerForControllerExec(gActiveBattler);
                }
            }
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    } else if *gBattlescriptCurrInstr.at(1) == BS_FAINTED_LINK_MULTIPLE_2 {
        if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                hitmarkerFaintBits = (gHitMarker >> 28) as u8;
                if gBitTable[2] & hitmarkerFaintBits as u32 != 0
                    && gBitTable[0] & hitmarkerFaintBits as u32 != 0
                {
                    gActiveBattler = 2;
                    if HasNoMonsToSwitch(gActiveBattler, gBattleBufferB[0][1], PARTY_SIZE as u8)
                        != 0
                    {
                        gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                        gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                        BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                        MarkBattlerForControllerExec(gActiveBattler);
                    } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                        ChooseMonToSendOut((*gBattleStruct).monToSwitchIntoId[0]);
                        gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                    }
                }
                if gBitTable[3] & hitmarkerFaintBits as u32 != 0
                    && hitmarkerFaintBits as u32 & gBitTable[1] != 0
                {
                    gActiveBattler = 3;
                    if HasNoMonsToSwitch(gActiveBattler, gBattleBufferB[1][1], PARTY_SIZE as u8)
                        != 0
                    {
                        gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
                        gHitMarker &= !(gBitTable[gActiveBattler] << 28);
                        BtlController_EmitCantSwitch(B_COMM_TO_CONTROLLER);
                        MarkBattlerForControllerExec(gActiveBattler);
                    } else if gSpecialStatuses[gActiveBattler].faintedHasReplacement() == 0 {
                        ChooseMonToSendOut((*gBattleStruct).monToSwitchIntoId[1]);
                        gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(TRUE as u32);
                    }
                }
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
            } else {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
            }
        } else {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
        }
        hitmarkerFaintBits = (gHitMarker >> 28) as u8;
        gBattlerFainted = 0;
        while gBitTable[gBattlerFainted] & hitmarkerFaintBits as u32 == 0
            && gBattlerFainted < gBattlersCount
        {
            gBattlerFainted += 1;
        }
        if gBattlerFainted == gBattlersCount {
            gBattlescriptCurrInstr = jumpPtr;
        }
    } else {
        if *gBattlescriptCurrInstr.at(1) as i32 & PARTY_SCREEN_OPTIONAL != 0 {
            hitmarkerFaintBits = PARTY_ACTION_CHOOSE_MON;
        } else {
            hitmarkerFaintBits = PARTY_ACTION_SEND_OUT;
        }
        battler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1) & 127);
        if gSpecialStatuses[battler].faintedHasReplacement() != 0 {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
        } else if HasNoMonsToSwitch(battler, PARTY_SIZE as u8, PARTY_SIZE as u8) != 0 {
            gActiveBattler = battler;
            gAbsentBattlerFlags |= gBitTable[gActiveBattler] as u8;
            gHitMarker &= !(gBitTable[gActiveBattler] << 28);
            gBattlescriptCurrInstr = jumpPtr;
        } else {
            gActiveBattler = battler;
            *(*gBattleStruct)
                .battlerPartyIndexes
                .as_mut_ptr()
                .at(gActiveBattler) = gBattlerPartyIndexes[gActiveBattler] as u8;
            *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler) = PARTY_SIZE as u8;
            (*gBattleStruct).field_93 &= !(gBitTable[gActiveBattler] as u8);
            BtlController_EmitChoosePokemon(
                B_COMM_TO_CONTROLLER,
                hitmarkerFaintBits,
                *(*gBattleStruct)
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler as i32 ^ 2),
                ABILITY_NONE,
                (*gBattleStruct).battlerPartyOrders[gActiveBattler].as_mut_ptr(),
            );
            MarkBattlerForControllerExec(gActiveBattler);
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
            if GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_LEFT
                && gBattleResults.playerSwitchesCounter < 255
            {
                gBattleResults.playerSwitchesCounter += 1;
            }
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                gActiveBattler = 0;
                while gActiveBattler < gBattlersCount {
                    if gActiveBattler != battler {
                        BtlController_EmitLinkStandbyMsg(
                            B_COMM_TO_CONTROLLER,
                            LINK_STANDBY_MSG_ONLY,
                            FALSE as u32,
                        );
                        MarkBattlerForControllerExec(gActiveBattler);
                    }
                    gActiveBattler += 1;
                }
            } else {
                gActiveBattler = GetBattlerAtPosition(GetBattlerPosition(battler) ^ 1);
                if gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] != 0 {
                    gActiveBattler ^= BIT_FLANK;
                }
                BtlController_EmitLinkStandbyMsg(
                    B_COMM_TO_CONTROLLER,
                    LINK_STANDBY_MSG_ONLY,
                    FALSE as u32,
                );
                MarkBattlerForControllerExec(gActiveBattler);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_switchhandleorder() {
    let mut i: i32 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    'l1: {
        let sw1: u8 = *gBattlescriptCurrInstr.at(2);
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            i = 0;
            while i < gBattlersCount as i32 {
                if gBattleBufferB[i][0] == CONTROLLER_CHOSENMONRETURNVALUE {
                    *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(i) = gBattleBufferB[i][1];
                    if (*gBattleStruct).field_93 as u32 & gBitTable[i] == 0 {
                        RecordedBattle_SetBattlerAction(i as u8, gBattleBufferB[i][1]);
                        (*gBattleStruct).field_93 |= gBitTable[i] as u8;
                    }
                }
                i += 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
                SwitchPartyOrder(gActiveBattler);
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*gBattleStruct).field_93 as u32 & gBitTable[gActiveBattler] == 0 {
                RecordedBattle_SetBattlerAction(gActiveBattler, gBattleBufferB[gActiveBattler][1]);
                (*gBattleStruct).field_93 |= gBitTable[gActiveBattler] as u8;
            }
        }
        if fall || sw1 == 3 {
            fall = true;
            gBattleCommunication[0] = gBattleBufferB[gActiveBattler][1];
            *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gActiveBattler) = gBattleBufferB[gActiveBattler][1];
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
            {
                *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(gActiveBattler as i32 * 3) &= 0xF;
                *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(gActiveBattler as i32 * 3) |= gBattleBufferB[gActiveBattler][2] & 0xF0;
                *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(gActiveBattler as i32 * 3)
                    .at(1) = gBattleBufferB[gActiveBattler][3];
                *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at((gActiveBattler as i32 ^ 2) * 3) &= 0xF0;
                *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at((gActiveBattler as i32 ^ 2) * 3) |=
                    ((gBattleBufferB[gActiveBattler][2] as i32 & 0xF0) >> 4) as u8;
                *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at((gActiveBattler as i32 ^ 2) * 3)
                    .at(2) = gBattleBufferB[gActiveBattler][3];
            } else if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
                SwitchPartyOrderInGameMulti(
                    gActiveBattler,
                    *(*gBattleStruct)
                        .monToSwitchIntoId
                        .as_mut_ptr()
                        .at(gActiveBattler),
                );
            } else {
                SwitchPartyOrder(gActiveBattler);
            }
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 6;
            gBattleTextBuff1[2] = gBattleMons[gBattlerAttacker].species as u8;
            gBattleTextBuff1[3] =
                ((gBattleMons[gBattlerAttacker].species as i32 & 0xFF00) >> 8) as u8;
            gBattleTextBuff1[4] = 0xFF;
            gBattleTextBuff2[0] = 0xFD;
            gBattleTextBuff2[1] = 7;
            gBattleTextBuff2[2] = gActiveBattler;
            gBattleTextBuff2[3] = gBattleBufferB[gActiveBattler][1];
            gBattleTextBuff2[4] = 0xFF;
            break 'l1;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_switchineffects() {
    let mut i: i32 = 0;
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    UpdateSentPokesToOpponentValue(gActiveBattler);
    gHitMarker &= !(gBitTable[gActiveBattler] << 28);
    gSpecialStatuses[gActiveBattler].set_faintedHasReplacement(FALSE as u32);
    if gSideStatuses[GetBattlerSide(gActiveBattler)] as i32 & SIDE_STATUS_SPIKES_DAMAGED == 0
        && gSideStatuses[GetBattlerSide(gActiveBattler)] as i32 & SIDE_STATUS_SPIKES != 0
        && !(gBattleMons[gActiveBattler].types[0] == TYPE_FLYING
            || gBattleMons[gActiveBattler].types[1] == TYPE_FLYING)
        && gBattleMons[gActiveBattler].ability != ABILITY_LEVITATE
    {
        let mut spikesDmg: u8 = 0;
        gSideStatuses[GetBattlerSide(gActiveBattler)] |= SIDE_STATUS_SPIKES_DAMAGED as u16;
        gBattleMons[gActiveBattler].status2 &= 0xfdffffff;
        gHitMarker &= 0xffffffbf;
        spikesDmg = (5 - gSideTimers[GetBattlerSide(gActiveBattler)].spikesAmount) * 2;
        gBattleMoveDamage = div_i32(gBattleMons[gActiveBattler].maxHP as i32, spikesDmg as i32);
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattleScripting.battler = gActiveBattler;
        BattleScriptPushCursor();
        if *gBattlescriptCurrInstr.at(1) == BS_TARGET {
            gBattlescriptCurrInstr = BattleScript_SpikesOnTarget.as_ptr().cast_mut();
        } else if *gBattlescriptCurrInstr.at(1) == 1 {
            gBattlescriptCurrInstr = BattleScript_SpikesOnAttacker.as_ptr().cast_mut();
        } else {
            gBattlescriptCurrInstr = BattleScript_SpikesOnFaintedBattler.as_ptr().cast_mut();
        }
    } else {
        if gBattleMons[gActiveBattler].ability == ABILITY_TRUANT
            && gDisableStructs[gActiveBattler].truantSwitchInHack() == 0
        {
            gDisableStructs[gActiveBattler].set_truantCounter(1);
        }
        gDisableStructs[gActiveBattler].set_truantSwitchInHack(0);
        if AbilityBattleEffects(0, gActiveBattler, 0, 0, 0) == 0
            && ItemBattleEffects(0, gActiveBattler, 0) == 0
        {
            gSideStatuses[GetBattlerSide(gActiveBattler)] &= 65023;
            i = 0;
            while i < gBattlersCount as i32 {
                if gBattlerByTurnOrder[i] == gActiveBattler {
                    gActionsByTurnOrder[i] = B_ACTION_CANCEL_PARTNER;
                }
                i += 1;
            }
            i = 0;
            while i < gBattlersCount as i32 {
                let mut hpOnSwitchout: *mut u16 =
                    &raw mut (*gBattleStruct).hpOnSwitchout[GetBattlerSide(i as u8)];
                *hpOnSwitchout = gBattleMons[i].hp;
                i += 1;
            }
            if *gBattlescriptCurrInstr.at(1) == BS_FAINTED_LINK_MULTIPLE_1 {
                let mut hitmarkerFaintBits: u32 = gHitMarker >> 28;
                gBattlerFainted += 1;
                loop {
                    if hitmarkerFaintBits & gBitTable[gBattlerFainted] != 0
                        && gAbsentBattlerFlags as u32 & gBitTable[gBattlerFainted] == 0
                    {
                        break;
                    }
                    if gBattlerFainted >= gBattlersCount {
                        break;
                    }
                    gBattlerFainted += 1;
                }
            }
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_trainerslidein() {
    gActiveBattler = GetBattlerAtPosition(*gBattlescriptCurrInstr.at(1));
    BtlController_EmitTrainerSlide(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_playse() {
    gActiveBattler = gBattlerAttacker;
    BtlController_EmitPlaySE(
        B_COMM_TO_CONTROLLER,
        *gBattlescriptCurrInstr.at(1) as u16 + ((*gBattlescriptCurrInstr.at(1).at(1) as u16) << 8),
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_fanfare() {
    gActiveBattler = gBattlerAttacker;
    BtlController_EmitPlayFanfareOrBGM(
        B_COMM_TO_CONTROLLER,
        *gBattlescriptCurrInstr.at(1) as u16 + ((*gBattlescriptCurrInstr.at(1).at(1) as u16) << 8),
        FALSE,
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_playfaintcry() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    BtlController_EmitFaintingCry(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_endlinkbattle() {
    gActiveBattler = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
    BtlController_EmitEndLinkBattle(B_COMM_TO_CONTROLLER, gBattleOutcome);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_returntoball() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, TRUE);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_handlelearnnewmove() {
    let mut learnedMovePtr: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    let mut nothingToLearnPtr: *mut u8 = (*gBattlescriptCurrInstr.at(5) as i32
        | (*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24)
        as usize as *mut u8;
    let mut learnMove: u16 = MonTryLearningNewMove(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
        *gBattlescriptCurrInstr.at(9),
    );
    while learnMove == MON_ALREADY_KNOWS_MOVE {
        learnMove = MonTryLearningNewMove(
            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
            FALSE,
        );
    }
    if learnMove == MOVE_NONE {
        gBattlescriptCurrInstr = nothingToLearnPtr;
    } else if learnMove == MON_HAS_MAX_MOVES {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(10);
    } else {
        gActiveBattler = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        if gBattlerPartyIndexes[gActiveBattler] == (*gBattleStruct).expGetterMonId as u16
            && gBattleMons[gActiveBattler].status2 & STATUS2_TRANSFORMED == 0
        {
            GiveMoveToBattleMon(&raw mut gBattleMons[gActiveBattler], learnMove);
        }
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
            gActiveBattler = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            if gBattlerPartyIndexes[gActiveBattler] == (*gBattleStruct).expGetterMonId as u16
                && gBattleMons[gActiveBattler].status2 & STATUS2_TRANSFORMED == 0
            {
                GiveMoveToBattleMon(&raw mut gBattleMons[gActiveBattler], learnMove);
            }
        }
        gBattlescriptCurrInstr = learnedMovePtr;
    }
}
pub(crate) unsafe extern "C" fn Cmd_yesnoboxlearnmove() {
    gActiveBattler = 0;
    match gBattleScripting.learnMoveState {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(gText_BattleYesNoChoice.as_ptr().cast_mut(), B_WIN_YESNO);
            gBattleScripting.learnMoveState += 1;
            gBattleCommunication[1] = 0;
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if gMain.newKeys as i32 & DPAD_UP != 0 && gBattleCommunication[1] != 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 0;
                BattleCreateYesNoCursorAt(0);
            }
            if gMain.newKeys as i32 & DPAD_DOWN != 0 && gBattleCommunication[1] == 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 1;
                BattleCreateYesNoCursorAt(1);
            }
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if gBattleCommunication[1] == 0 {
                    HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                    gBattleScripting.learnMoveState += 1;
                } else {
                    gBattleScripting.learnMoveState = 5;
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                gBattleScripting.learnMoveState = 5;
            }
        }
        2 => {
            if gPaletteFade.active() == 0 {
                FreeAllWindowBuffers();
                ShowSelectMovePokemonSummaryScreen(
                    gPlayerParty.as_mut_ptr(),
                    (*gBattleStruct).expGetterMonId,
                    gPlayerPartyCount - 1,
                    Some(ReshowBattleScreenAfterMenu),
                    gMoveToLearn,
                );
                gBattleScripting.learnMoveState += 1;
            }
        }
        3 => {
            if gPaletteFade.active() == 0
                && gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
            {
                gBattleScripting.learnMoveState += 1;
            }
        }
        4 => {
            if gPaletteFade.active() == 0
                && gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
            {
                let mut movePosition: u8 = GetMoveSlotToReplace();
                if movePosition == MAX_MON_MOVES as u8 {
                    gBattleScripting.learnMoveState = 5;
                } else {
                    let mut r#move: u16 = GetMonData2(
                        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                        MON_DATA_MOVE1 + movePosition as i32,
                    ) as u16;
                    if IsHMMove2(r#move) != 0 {
                        PrepareStringBattle(STRINGID_HMMOVESCANTBEFORGOTTEN, gActiveBattler);
                        gBattleScripting.learnMoveState = 6;
                    } else {
                        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                            as usize as *mut u8;
                        gBattleTextBuff2[0] = 0xFD;
                        gBattleTextBuff2[1] = 2;
                        gBattleTextBuff2[2] = r#move as u8 & 0xFF;
                        gBattleTextBuff2[3] = ((r#move as i32 & 0xFF00) >> 8) as u8;
                        gBattleTextBuff2[4] = 0xFF;
                        RemoveMonPPBonus(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            movePosition,
                        );
                        SetMonMoveSlot(
                            &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
                            gMoveToLearn,
                            movePosition,
                        );
                        if gBattlerPartyIndexes[0] == (*gBattleStruct).expGetterMonId as u16
                            && (gBattleMons[0].status2 & 0x200000 == 0
                                && gDisableStructs[0].mimickedMoves() as u32
                                    & gBitTable[movePosition]
                                    == 0)
                        {
                            RemoveBattleMonPPBonus(&raw mut gBattleMons[0], movePosition);
                            SetBattleMonMoveSlot(
                                &raw mut gBattleMons[0],
                                gMoveToLearn,
                                movePosition,
                            );
                        }
                        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
                            && gBattlerPartyIndexes[2] == (*gBattleStruct).expGetterMonId as u16
                            && (gBattleMons[2].status2 & 0x200000 == 0
                                && gDisableStructs[2].mimickedMoves() as u32
                                    & gBitTable[movePosition]
                                    == 0)
                        {
                            RemoveBattleMonPPBonus(&raw mut gBattleMons[2], movePosition);
                            SetBattleMonMoveSlot(
                                &raw mut gBattleMons[2],
                                gMoveToLearn,
                                movePosition,
                            );
                        }
                    }
                }
            }
        }
        5 => {
            HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        }
        6 => {
            if gBattleControllerExecFlags == 0 {
                gBattleScripting.learnMoveState = 2;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_yesnoboxstoplearningmove() {
    match gBattleScripting.learnMoveState {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(gText_BattleYesNoChoice.as_ptr().cast_mut(), B_WIN_YESNO);
            gBattleScripting.learnMoveState += 1;
            gBattleCommunication[1] = 0;
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if gMain.newKeys as i32 & DPAD_UP != 0 && gBattleCommunication[1] != 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 0;
                BattleCreateYesNoCursorAt(0);
            }
            if gMain.newKeys as i32 & DPAD_DOWN != 0 && gBattleCommunication[1] == 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 1;
                BattleCreateYesNoCursorAt(1);
            }
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if gBattleCommunication[1] != 0 {
                    gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                        as usize as *mut u8;
                } else {
                    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
                }
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                    | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                    | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                    | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                    as usize as *mut u8;
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_hitanimation() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    } else if gHitMarker & HITMARKER_IGNORE_SUBSTITUTE == 0
        || gBattleMons[gActiveBattler].status2 & STATUS2_SUBSTITUTE == 0
        || gDisableStructs[gActiveBattler].substituteHP == 0
    {
        BtlController_EmitHitAnimation(B_COMM_TO_CONTROLLER);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    }
}
pub(crate) unsafe extern "C" fn GetTrainerMoneyToGive(trainerId: u16) -> u32 {
    let mut i: u32 = 0;
    let mut lastMonLevel: u32 = 0;
    let mut moneyReward: u32 = 0;
    if trainerId == TRAINER_SECRET_BASE {
        moneyReward = 20
            * (*(*gBattleResources).secretBase).party.levels[0] as u32
            * (*gBattleStruct).moneyMultiplier as u32;
    } else {
        match gTrainers[trainerId].partyFlags {
            0 => {
                let mut party: *mut TrainerMonNoItemDefaultMoves =
                    gTrainers[trainerId].party.NoItemDefaultMoves;
                lastMonLevel = (*party.at(gTrainers[trainerId].partySize as i32 - 1)).lvl as u32;
            }
            F_TRAINER_PARTY_CUSTOM_MOVESET => {
                let mut party: *mut TrainerMonNoItemCustomMoves =
                    gTrainers[trainerId].party.NoItemCustomMoves;
                lastMonLevel = (*party.at(gTrainers[trainerId].partySize as i32 - 1)).lvl as u32;
            }
            F_TRAINER_PARTY_HELD_ITEM => {
                let mut party: *mut TrainerMonItemDefaultMoves =
                    gTrainers[trainerId].party.ItemDefaultMoves;
                lastMonLevel = (*party.at(gTrainers[trainerId].partySize as i32 - 1)).lvl as u32;
            }
            3 => {
                let mut party: *mut TrainerMonItemCustomMoves =
                    gTrainers[trainerId].party.ItemCustomMoves;
                lastMonLevel = (*party.at(gTrainers[trainerId].partySize as i32 - 1)).lvl as u32;
            }
            _ => {}
        }
        while gTrainerMoneyTable[i].classId != 0xFF {
            if gTrainerMoneyTable[i].classId == gTrainers[trainerId].trainerClass {
                break;
            }
            i += 1;
        }
        if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            moneyReward = 4
                * lastMonLevel
                * (*gBattleStruct).moneyMultiplier as u32
                * gTrainerMoneyTable[i].value as u32;
        } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
            moneyReward = 4
                * lastMonLevel
                * (*gBattleStruct).moneyMultiplier as u32
                * 2
                * gTrainerMoneyTable[i].value as u32;
        } else {
            moneyReward = 4
                * lastMonLevel
                * (*gBattleStruct).moneyMultiplier as u32
                * gTrainerMoneyTable[i].value as u32;
        }
    }
    return moneyReward;
}
pub(crate) unsafe extern "C" fn Cmd_getmoneyreward() {
    let mut moneyReward: u32 = GetTrainerMoneyToGive(gTrainerBattleOpponent_A);
    if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        moneyReward += GetTrainerMoneyToGive(gTrainerBattleOpponent_B);
    }
    AddMoney(&raw mut (*gSaveBlock1Ptr).money, moneyReward);
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 1;
    gBattleTextBuff1[2] = 4;
    gBattleTextBuff1[3] = 5;
    gBattleTextBuff1[4] = moneyReward as u8;
    gBattleTextBuff1[5] = ((moneyReward & 0x0000FF00) >> 8) as u8;
    gBattleTextBuff1[6] = ((moneyReward & 0x00FF0000) >> 16) as u8;
    gBattleTextBuff1[7] = ((moneyReward & 0xFF000000) >> 24) as u8;
    gBattleTextBuff1[8] = 0xFF;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_updatebattlermoves() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    match gBattleCommunication[0] {
        0 => {
            BtlController_EmitGetMonData(B_COMM_TO_CONTROLLER, REQUEST_ALL_BATTLE, 0);
            MarkBattlerForControllerExec(gActiveBattler);
            gBattleCommunication[0] += 1;
        }
        1 => {
            if gBattleControllerExecFlags == 0 {
                let mut i: i32 = 0;
                let mut bufferPoke: *mut BattlePokemon =
                    &raw mut gBattleBufferB[gActiveBattler][4] as *mut BattlePokemon;
                i = 0;
                while i < MAX_MON_MOVES {
                    gBattleMons[gActiveBattler].moves[i] = (*bufferPoke).moves[i];
                    gBattleMons[gActiveBattler].pp[i] = (*bufferPoke).pp[i];
                    i += 1;
                }
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_swapattackerwithtarget() {
    gActiveBattler = gBattlerAttacker;
    gBattlerAttacker = gBattlerTarget;
    gBattlerTarget = gActiveBattler;
    if gHitMarker & HITMARKER_SWAP_ATTACKER_TARGET != 0 {
        gHitMarker &= 0xffffefff;
    } else {
        gHitMarker |= HITMARKER_SWAP_ATTACKER_TARGET;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_incrementgamestat() {
    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
        IncrementGameStat(*gBattlescriptCurrInstr.at(1));
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_drawpartystatussummary() {
    let mut i: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut hpStatuses: CArray<HpAndStatus, 6> = zeroed();
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32
            || GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG
        {
            hpStatuses[i].hp = 0xFFFF;
            hpStatuses[i].status = 0;
        } else {
            hpStatuses[i].hp = GetMonData2(party.at(i), MON_DATA_HP) as u16;
            hpStatuses[i].status = GetMonData2(party.at(i), MON_DATA_STATUS);
        }
        i += 1;
    }
    BtlController_EmitDrawPartyStatusSummary(B_COMM_TO_CONTROLLER, hpStatuses.as_mut_ptr(), 1);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_hidepartystatussummary() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    BtlController_EmitHidePartyStatusSummary(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_jumptocalledmove() {
    if *gBattlescriptCurrInstr.at(1) != 0 {
        gCurrentMove = gCalledMove;
    } else {
        gChosenMove = {
            gCurrentMove = gCalledMove;
            gCurrentMove
        };
    }
    gBattlescriptCurrInstr = gBattleScriptsForMoveEffects[gBattleMoves[gCurrentMove].effect];
}
pub(crate) unsafe extern "C" fn Cmd_statusanimation() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        if gStatuses3[gActiveBattler] & STATUS3_SEMI_INVULNERABLE == 0
            && gDisableStructs[gActiveBattler].substituteHP == 0
            && gHitMarker & HITMARKER_NO_ANIMATIONS == 0
        {
            BtlController_EmitStatusAnimation(
                B_COMM_TO_CONTROLLER,
                FALSE,
                gBattleMons[gActiveBattler].status1,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    }
}
pub(crate) unsafe extern "C" fn Cmd_status2animation() {
    let mut wantedToAnimate: u32 = 0;
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        wantedToAnimate = *gBattlescriptCurrInstr.at(2) as u32
            | (*gBattlescriptCurrInstr.at(2).at(1) as u32) << 8
            | (*gBattlescriptCurrInstr.at(2).at(2) as u32) << 16
            | (*gBattlescriptCurrInstr.at(2).at(3) as u32) << 24;
        if gStatuses3[gActiveBattler] & STATUS3_SEMI_INVULNERABLE == 0
            && gDisableStructs[gActiveBattler].substituteHP == 0
            && gHitMarker & HITMARKER_NO_ANIMATIONS == 0
        {
            BtlController_EmitStatusAnimation(
                B_COMM_TO_CONTROLLER,
                TRUE,
                gBattleMons[gActiveBattler].status2 & wantedToAnimate,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_chosenstatusanimation() {
    let mut wantedStatus: u32 = 0;
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        wantedStatus = *gBattlescriptCurrInstr.at(3) as u32
            | (*gBattlescriptCurrInstr.at(3).at(1) as u32) << 8
            | (*gBattlescriptCurrInstr.at(3).at(2) as u32) << 16
            | (*gBattlescriptCurrInstr.at(3).at(3) as u32) << 24;
        if gStatuses3[gActiveBattler] & STATUS3_SEMI_INVULNERABLE == 0
            && gDisableStructs[gActiveBattler].substituteHP == 0
            && gHitMarker & HITMARKER_NO_ANIMATIONS == 0
        {
            BtlController_EmitStatusAnimation(
                B_COMM_TO_CONTROLLER,
                *gBattlescriptCurrInstr.at(2),
                wantedStatus,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_yesnobox() {
    match gBattleCommunication[0] {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(gText_BattleYesNoChoice.as_ptr().cast_mut(), B_WIN_YESNO);
            gBattleCommunication[0] += 1;
            gBattleCommunication[1] = 0;
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if gMain.newKeys as i32 & DPAD_UP != 0 && gBattleCommunication[1] != 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 0;
                BattleCreateYesNoCursorAt(0);
            }
            if gMain.newKeys as i32 & DPAD_DOWN != 0 && gBattleCommunication[1] == 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 1;
                BattleCreateYesNoCursorAt(1);
            }
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                gBattleCommunication[1] = 1;
                PlaySE(SE_SELECT);
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_cancelallactions() {
    let mut i: i32 = 0;
    i = 0;
    while i < gBattlersCount as i32 {
        gActionsByTurnOrder[i] = B_ACTION_CANCEL_PARTNER;
        i += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_adjustsetdamage() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    if gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gBattlerTarget].holdEffect;
        param = gEnigmaBerries[gBattlerTarget].holdEffectParam;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[gBattlerTarget].item);
        param = GetItemHoldEffectParam(gBattleMons[gBattlerTarget].item);
    }
    gPotentialItemEffectBattler = gBattlerTarget;
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget, holdEffect);
        gSpecialStatuses[gBattlerTarget].set_focusBanded(1);
    }
    if gBattleMons[gBattlerTarget].status2 & STATUS2_SUBSTITUTE == 0
        && (gBattleMoves[gCurrentMove].effect == EFFECT_FALSE_SWIPE
            || gProtectStructs[gBattlerTarget].endured() != 0
            || gSpecialStatuses[gBattlerTarget].focusBanded() != 0)
        && gBattleMons[gBattlerTarget].hp as i32 <= gBattleMoveDamage
    {
        gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 - 1;
        if gProtectStructs[gBattlerTarget].endured() != 0 {
            gMoveResultFlags |= MOVE_RESULT_FOE_ENDURED;
        } else if gSpecialStatuses[gBattlerTarget].focusBanded() != 0 {
            gMoveResultFlags |= MOVE_RESULT_FOE_HUNG_ON;
            gLastUsedItem = gBattleMons[gBattlerTarget].item;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_removeitem() {
    let mut usedHeldItem: *mut u16 = null_mut();
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    usedHeldItem = &raw mut (*gBattleStruct).usedHeldItems[gActiveBattler];
    *usedHeldItem = gBattleMons[gActiveBattler].item;
    gBattleMons[gActiveBattler].item = ITEM_NONE;
    BtlController_EmitSetMonData(
        B_COMM_TO_CONTROLLER,
        REQUEST_HELDITEM_BATTLE,
        0,
        2,
        &raw mut gBattleMons[gActiveBattler].item as *mut c_void,
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_atknameinbuff1() {
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 7;
    gBattleTextBuff1[2] = gBattlerAttacker;
    gBattleTextBuff1[3] = gBattlerPartyIndexes[gBattlerAttacker] as u8;
    gBattleTextBuff1[4] = 0xFF;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_drawlvlupbox() {
    if gBattleScripting.drawlvlupboxState == 0 {
        if IsMonGettingExpSentOut() != 0 {
            gBattleScripting.drawlvlupboxState = 3;
        } else {
            gBattleScripting.drawlvlupboxState = 1;
        }
    }
    match gBattleScripting.drawlvlupboxState {
        1 => {
            gBattle_BG2_Y = 96;
            SetBgAttribute(2, BG_ATTR_PRIORITY, 0);
            ShowBg(2);
            InitLevelUpBanner();
            gBattleScripting.drawlvlupboxState = 2;
        }
        2 => {
            if SlideInLevelUpBanner() == 0 {
                gBattleScripting.drawlvlupboxState = 3;
            }
        }
        3 => {
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 256;
            SetBgAttribute(0, BG_ATTR_PRIORITY, 1);
            SetBgAttribute(1, BG_ATTR_PRIORITY, 0);
            ShowBg(0);
            ShowBg(1);
            HandleBattleWindow(18, 7, 29, 19, WINDOW_BG1);
            gBattleScripting.drawlvlupboxState = 4;
        }
        4 => {
            DrawLevelUpWindow1();
            PutWindowTilemap(B_WIN_LEVEL_UP_BOX);
            CopyWindowToVram(B_WIN_LEVEL_UP_BOX, COPYWIN_FULL);
            gBattleScripting.drawlvlupboxState += 1;
        }
        5 | 7 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                gBattle_BG1_Y = 0;
                gBattleScripting.drawlvlupboxState += 1;
            }
        }
        6 => {
            if gMain.newKeys != 0 {
                PlaySE(SE_SELECT);
                DrawLevelUpWindow2();
                CopyWindowToVram(B_WIN_LEVEL_UP_BOX, COPYWIN_GFX);
                gBattleScripting.drawlvlupboxState += 1;
            }
        }
        8 => {
            if gMain.newKeys != 0 {
                PlaySE(SE_SELECT);
                HandleBattleWindow(18, 7, 29, 19, 129);
                gBattleScripting.drawlvlupboxState += 1;
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
                gBattleScripting.drawlvlupboxState = 10;
            }
        }
        10 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetBgAttribute(0, BG_ATTR_PRIORITY, 0);
                SetBgAttribute(1, BG_ATTR_PRIORITY, 1);
                ShowBg(0);
                ShowBg(1);
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DrawLevelUpWindow1() {
    let mut currStats: CArray<u16, 6> = zeroed();
    GetMonLevelUpWindowStats(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
        currStats.as_mut_ptr(),
    );
    DrawLevelUpWindowPg1(
        13,
        (*(*gBattleResources).beforeLvlUp).stats.as_mut_ptr(),
        currStats.as_mut_ptr(),
        TEXT_DYNAMIC_COLOR_5,
        0xD,
        TEXT_DYNAMIC_COLOR_6,
    );
}
pub(crate) unsafe extern "C" fn DrawLevelUpWindow2() {
    let mut currStats: CArray<u16, 6> = zeroed();
    GetMonLevelUpWindowStats(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
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
pub(crate) unsafe extern "C" fn InitLevelUpBanner() {
    gBattle_BG2_Y = 0;
    gBattle_BG2_X = LEVEL_UP_BANNER_START;
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
pub(crate) unsafe extern "C" fn SlideInLevelUpBanner() -> u8 {
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return TRUE;
    }
    if gBattle_BG2_X == LEVEL_UP_BANNER_END {
        return FALSE;
    }
    if gBattle_BG2_X == LEVEL_UP_BANNER_START {
        DrawLevelUpBannerText();
    }
    gBattle_BG2_X += 8;
    if gBattle_BG2_X >= LEVEL_UP_BANNER_END {
        gBattle_BG2_X = LEVEL_UP_BANNER_END;
    }
    return (gBattle_BG2_X != LEVEL_UP_BANNER_END) as u8;
}
pub(crate) unsafe extern "C" fn DrawLevelUpBannerText() {
    let mut monLevel: u16 = 0;
    let mut monGender: u8 = 0;
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    let mut txtPtr: *mut u8 = null_mut();
    let mut var: u32 = 0;
    monLevel = GetMonData2(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
        MON_DATA_LEVEL,
    ) as u16;
    monGender = GetMonGender(&raw mut gPlayerParty[(*gBattleStruct).expGetterMonId]);
    GetMonNickname(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
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
    txtPtr = gStringVar4.as_mut_ptr();
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
    var = txtPtr as usize as u32;
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
pub(crate) unsafe extern "C" fn SlideOutLevelUpBanner() -> u8 {
    if gBattle_BG2_X == LEVEL_UP_BANNER_START {
        return FALSE;
    }
    if gBattle_BG2_X as i32 - 16 < LEVEL_UP_BANNER_START as i32 {
        gBattle_BG2_X = LEVEL_UP_BANNER_START;
    } else {
        gBattle_BG2_X -= 16;
    }
    return (gBattle_BG2_X != LEVEL_UP_BANNER_START) as u8;
}
pub(crate) unsafe extern "C" fn PutMonIconOnLvlUpBanner() {
    let mut spriteId: u8 = 0;
    let mut iconPal: *mut u16 = null_mut();
    let mut iconSheet: SpriteSheet = zeroed();
    let mut iconPalSheet: SpritePalette = zeroed();
    let mut species: u16 = GetMonData2(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
        MON_DATA_SPECIES,
    ) as u16;
    let mut personality: u32 = GetMonData2(
        &raw mut gPlayerParty[(*gBattleStruct).expGetterMonId],
        MON_DATA_PERSONALITY,
    );
    let mut iconPtr: *mut u8 = GetMonIconPtr(species, personality, 1);
    iconSheet.data = iconPtr as *mut c_void;
    iconSheet.size = 0x200;
    iconSheet.tag = TAG_LVLUP_BANNER_MON_ICON;
    iconPal = GetValidMonIconPalettePtr(species);
    iconPalSheet.data = iconPal;
    iconPalSheet.tag = TAG_LVLUP_BANNER_MON_ICON;
    LoadSpriteSheet(&raw mut iconSheet);
    LoadSpritePalette(&raw mut iconPalSheet);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_MonIconOnLvlUpBanner).cast_mut(),
        256,
        10,
        0,
    );
    gSprites[spriteId].data[0] = FALSE as i16;
    gSprites[spriteId].data[1] = gBattle_BG2_X as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconOnLvlUpBanner(sprite: *mut Sprite) {
    (*sprite).x2 = (*sprite).data[1] - gBattle_BG2_X as i16;
    if (*sprite).x2 != 0 {
        (*sprite).data[0] = TRUE as i16;
    } else if (*sprite).data[0] != 0 {
        DestroySprite(sprite);
        FreeSpriteTilesByTag(TAG_LVLUP_BANNER_MON_ICON);
        FreeSpritePaletteByTag(TAG_LVLUP_BANNER_MON_ICON);
    }
}
pub(crate) unsafe extern "C" fn IsMonGettingExpSentOut() -> u32 {
    if gBattlerPartyIndexes[0] == (*gBattleStruct).expGetterMonId as u16 {
        return TRUE as u32;
    }
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && gBattlerPartyIndexes[2] == (*gBattleStruct).expGetterMonId as u16
    {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Cmd_resetsentmonsvalue() {
    ResetSentPokesToOpponentValue();
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setatktoplayer0() {
    gBattlerAttacker = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_makevisible() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    BtlController_EmitSpriteInvisibility(B_COMM_TO_CONTROLLER, FALSE);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_recordlastability() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    RecordAbilityBattle(gActiveBattler, gLastUsedAbility);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMoveToLearnIntoBattleTextBuff2() {
    gBattleTextBuff2[0] = 0xFD;
    gBattleTextBuff2[1] = 2;
    gBattleTextBuff2[2] = gMoveToLearn as u8 & 0xFF;
    gBattleTextBuff2[3] = ((gMoveToLearn as i32 & 0xFF00) >> 8) as u8;
    gBattleTextBuff2[4] = 0xFF;
}
pub(crate) unsafe extern "C" fn Cmd_buffermovetolearn() {
    BufferMoveToLearnIntoBattleTextBuff2();
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_jumpifplayerran() {
    if TryRunFromBattle(gBattlerFainted) != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_hpthresholds() {
    let mut opposingBattler: u8 = 0;
    let mut result: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        opposingBattler = gActiveBattler ^ 1;
        result = div_i32(
            gBattleMons[opposingBattler].hp as i32 * 100,
            gBattleMons[opposingBattler].maxHP as i32,
        );
        if result == 0 {
            result = 1;
        }
        if result > 69 || gBattleMons[opposingBattler].hp == 0 {
            (*gBattleStruct).hpScale = 0;
        } else if result > 39 {
            (*gBattleStruct).hpScale = 1;
        } else if result > 9 {
            (*gBattleStruct).hpScale = 2;
        } else {
            (*gBattleStruct).hpScale = 3;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_hpthresholds2() {
    let mut opposingBattler: u8 = 0;
    let mut result: i32 = 0;
    let mut hpSwitchout: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        opposingBattler = gActiveBattler ^ 1;
        hpSwitchout = *(*gBattleStruct)
            .hpOnSwitchout
            .as_mut_ptr()
            .at(GetBattlerSide(opposingBattler)) as u8;
        result = div_i32(
            (hpSwitchout as i32 - gBattleMons[opposingBattler].hp as i32) * 100,
            hpSwitchout as i32,
        );
        if gBattleMons[opposingBattler].hp >= hpSwitchout as u16 {
            (*gBattleStruct).hpScale = 0;
        } else if result <= 29 {
            (*gBattleStruct).hpScale = 1;
        } else if result <= 69 {
            (*gBattleStruct).hpScale = 2;
        } else {
            (*gBattleStruct).hpScale = 3;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_useitemonopponent() {
    gBattlerInMenuId = gBattlerAttacker;
    PokemonUseItemEffects(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker]],
        gLastUsedItem,
        gBattlerPartyIndexes[gBattlerAttacker] as u8,
        0,
        TRUE,
    );
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_various() {
    let mut side: u8 = 0;
    let mut i: i32 = 0;
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    match *gBattlescriptCurrInstr.at(2) {
        VARIOUS_CANCEL_MULTI_TURN_MOVES => {
            CancelMultiTurnMoves(gActiveBattler);
        }
        VARIOUS_SET_MAGIC_COAT_TARGET => {
            gBattlerAttacker = gBattlerTarget;
            side = GetBattlerSide(gBattlerAttacker) ^ 1;
            if gSideTimers[side].followmeTimer != 0
                && gBattleMons[gSideTimers[side].followmeTarget].hp != 0
            {
                gBattlerTarget = gSideTimers[side].followmeTarget;
            } else {
                gBattlerTarget = gActiveBattler;
            }
        }
        VARIOUS_IS_RUNNING_IMPOSSIBLE => {
            gBattleCommunication[0] = IsRunningFromBattleImpossible();
        }
        VARIOUS_GET_MOVE_TARGET => {
            gBattlerTarget = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
        }
        VARIOUS_GET_BATTLER_FAINTED => {
            if gHitMarker & gBitTable[gActiveBattler] << 28 != 0 {
                gBattleCommunication[0] = TRUE;
            } else {
                gBattleCommunication[0] = 0;
            }
        }
        VARIOUS_RESET_INTIMIDATE_TRACE_BITS => {
            gSpecialStatuses[gActiveBattler].set_intimidatedMon(0);
            gSpecialStatuses[gActiveBattler].set_traced(0);
        }
        VARIOUS_UPDATE_CHOICE_MOVE_ON_LVL_UP => {
            if gBattlerPartyIndexes[0] == (*gBattleStruct).expGetterMonId as u16
                || gBattlerPartyIndexes[2] == (*gBattleStruct).expGetterMonId as u16
            {
                let mut choicedMove: *mut u16 = null_mut();
                if gBattlerPartyIndexes[0] == (*gBattleStruct).expGetterMonId as u16 {
                    gActiveBattler = 0;
                } else {
                    gActiveBattler = 2;
                }
                choicedMove = &raw mut (*gBattleStruct).choicedMove[gActiveBattler];
                i = 0;
                while i < MAX_MON_MOVES {
                    if gBattleMons[gActiveBattler].moves[i] == *choicedMove {
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
            if gBattleTypeFlags & 3 == 0
                && gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
                && gBattleMons[0].hp != 0
                && gBattleMons[1].hp != 0
            {
                gHitMarker &= 0xffbfffff;
            }
        }
        VARIOUS_PALACE_FLAVOR_TEXT => {
            gBattleCommunication[0] = 0;
            gBattleScripting.battler = {
                gActiveBattler = gBattleCommunication[1];
                gActiveBattler
            };
            if (*gBattleStruct).palaceFlags as u32 & gBitTable[gActiveBattler] == 0
                && gBattleMons[gActiveBattler].maxHP as i32 / 2
                    >= gBattleMons[gActiveBattler].hp as i32
                && gBattleMons[gActiveBattler].hp != 0
                && gBattleMons[gActiveBattler].status1 & STATUS1_SLEEP == 0
            {
                (*gBattleStruct).palaceFlags |= gBitTable[gActiveBattler] as u8;
                gBattleCommunication[0] = TRUE;
                gBattleCommunication[5] = sBattlePalaceNatureToFlavorTextId
                    [GetNatureFromPersonality(gBattleMons[gActiveBattler].personality)];
            }
        }
        VARIOUS_ARENA_JUDGMENT_WINDOW => {
            i = BattleArena_ShowJudgmentWindow(&raw mut gBattleCommunication[0]) as i32;
            if i == ARENA_RESULT_RUNNING as i32 {
                return;
            }
            gBattleCommunication[1] = i as u8;
        }
        VARIOUS_ARENA_OPPONENT_MON_LOST => {
            gBattleMons[1].hp = 0;
            gHitMarker |= gBitTable[1] << 28;
            (*gBattleStruct).arenaLostOpponentMons |= gBitTable[gBattlerPartyIndexes[1]] as u8;
            gDisableStructs[1].set_truantSwitchInHack(1);
        }
        VARIOUS_ARENA_PLAYER_MON_LOST => {
            gBattleMons[0].hp = 0;
            gHitMarker |= gBitTable[0] << 28;
            gHitMarker |= HITMARKER_PLAYER_FAINTED;
            (*gBattleStruct).arenaLostPlayerMons |= gBitTable[gBattlerPartyIndexes[0]] as u8;
            gDisableStructs[0].set_truantSwitchInHack(1);
        }
        VARIOUS_ARENA_BOTH_MONS_LOST => {
            gBattleMons[0].hp = 0;
            gBattleMons[1].hp = 0;
            gHitMarker |= gBitTable[0] << 28;
            gHitMarker |= gBitTable[1] << 28;
            gHitMarker |= HITMARKER_PLAYER_FAINTED;
            (*gBattleStruct).arenaLostPlayerMons |= gBitTable[gBattlerPartyIndexes[0]] as u8;
            (*gBattleStruct).arenaLostOpponentMons |= gBitTable[gBattlerPartyIndexes[1]] as u8;
            gDisableStructs[0].set_truantSwitchInHack(1);
            gDisableStructs[1].set_truantSwitchInHack(1);
        }
        VARIOUS_EMIT_YESNOBOX => {
            BtlController_EmitYesNoBox(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        VARIOUS_DRAW_ARENA_REF_TEXT_BOX => {
            DrawArenaRefereeTextBox();
        }
        VARIOUS_ERASE_ARENA_REF_TEXT_BOX => {
            EraseArenaRefereeTextBox();
        }
        VARIOUS_ARENA_JUDGMENT_STRING => {
            BattleStringExpandPlaceholdersToDisplayedString(
                gRefereeStringsTable[*gBattlescriptCurrInstr.at(1)],
            );
            BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), ARENA_WIN_JUDGMENT_TEXT);
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
            gActiveBattler = 1;
            if gBattleMons[gActiveBattler].hp != 0 {
                BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, FALSE);
                MarkBattlerForControllerExec(gActiveBattler);
            }
        }
        VARIOUS_RETURN_OPPONENT_MON2 => {
            if gBattlersCount > 3 {
                gActiveBattler = 3;
                if gBattleMons[gActiveBattler].hp != 0 {
                    BtlController_EmitReturnMonToBall(B_COMM_TO_CONTROLLER, FALSE);
                    MarkBattlerForControllerExec(gActiveBattler);
                }
            }
        }
        VARIOUS_VOLUME_DOWN => {
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x55);
        }
        VARIOUS_VOLUME_UP => {
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        }
        VARIOUS_SET_ALREADY_STATUS_MOVE_ATTEMPT => {
            (*gBattleStruct).alreadyStatusedMoveAttempt |= gBitTable[gActiveBattler] as u8;
        }
        VARIOUS_PALACE_TRY_ESCAPE_STATUS => {
            if BattlePalace_TryEscapeStatus(gActiveBattler) != 0 {
                return;
            }
        }
        VARIOUS_SET_TELEPORT_OUTCOME => {
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                gBattleOutcome = B_OUTCOME_PLAYER_TELEPORTED;
            } else {
                gBattleOutcome = B_OUTCOME_MON_TELEPORTED;
            }
        }
        VARIOUS_PLAY_TRAINER_DEFEATED_MUSIC => {
            BtlController_EmitPlayFanfareOrBGM(B_COMM_TO_CONTROLLER, MUS_VICTORY_TRAINER, TRUE);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        _ => {}
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_setprotectlike() {
    let mut notLastTurn: u8 = TRUE;
    let mut lastMove: u16 = gLastResultingMoves[gBattlerAttacker];
    if lastMove != MOVE_PROTECT && lastMove != MOVE_DETECT && lastMove != MOVE_ENDURE {
        gDisableStructs[gBattlerAttacker].protectUses = 0;
    }
    if gCurrentTurnActionNumber as i32 == gBattlersCount as i32 - 1 {
        notLastTurn = FALSE;
    }
    if sProtectSuccessRates[gDisableStructs[gBattlerAttacker].protectUses] >= Random()
        && notLastTurn != 0
    {
        if gBattleMoves[gCurrentMove].effect == EFFECT_PROTECT {
            gProtectStructs[gBattlerAttacker].set_protected(1);
            gBattleCommunication[5] = B_MSG_PROTECTED_ITSELF;
        }
        if gBattleMoves[gCurrentMove].effect == EFFECT_ENDURE {
            gProtectStructs[gBattlerAttacker].set_endured(1);
            gBattleCommunication[5] = B_MSG_BRACED_ITSELF;
        }
        gDisableStructs[gBattlerAttacker].protectUses += 1;
    } else {
        gDisableStructs[gBattlerAttacker].protectUses = 0;
        gBattleCommunication[5] = B_MSG_PROTECT_FAILED;
        gMoveResultFlags |= MOVE_RESULT_MISSED;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_tryexplosion() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gBattlerTarget = 0;
    while gBattlerTarget < gBattlersCount {
        if gBattleMons[gBattlerTarget].ability == ABILITY_DAMP {
            break;
        }
        gBattlerTarget += 1;
    }
    if gBattlerTarget == gBattlersCount {
        gActiveBattler = gBattlerAttacker;
        gBattleMoveDamage = gBattleMons[gActiveBattler].hp as i32;
        BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, INSTANT_HP_BAR_DROP as u16);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
        gBattlerTarget = 0;
        'l3: while gBattlerTarget < gBattlersCount {
            'l2: {
                if gBattlerTarget == gBattlerAttacker {
                    break 'l2;
                }
                if gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] == 0 {
                    break 'l3;
                }
            }
            gBattlerTarget += 1;
        }
    } else {
        gLastUsedAbility = ABILITY_DAMP;
        RecordAbilityBattle(gBattlerTarget, gBattleMons[gBattlerTarget].ability);
        gBattlescriptCurrInstr = BattleScript_DampStopsExplosion.as_ptr().cast_mut();
    }
}
pub(crate) unsafe extern "C" fn Cmd_setatkhptozero() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = gBattlerAttacker;
    gBattleMons[gActiveBattler].hp = 0;
    BtlController_EmitSetMonData(
        B_COMM_TO_CONTROLLER,
        REQUEST_HP_BATTLE,
        0,
        2,
        &raw mut gBattleMons[gActiveBattler].hp as *mut c_void,
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_jumpifnexttargetvalid() {
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        gBattlerTarget += 1;
        'l2: loop {
            'l1: {
                if gBattlerTarget == gBattlerAttacker {
                    break 'l1;
                }
                if gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] == 0 {
                    break 'l2;
                }
            }
            gBattlerTarget += 1;
        }
        if gBattlerTarget >= gBattlersCount {
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        } else {
            gBattlescriptCurrInstr = jumpPtr;
        }
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_tryhealhalfhealth() {
    let mut failPtr: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if *gBattlescriptCurrInstr.at(5) == BS_ATTACKER {
        gBattlerTarget = gBattlerAttacker;
    }
    gBattleMoveDamage = gBattleMons[gBattlerTarget].maxHP as i32 / 2;
    if gBattleMoveDamage == 0 {
        gBattleMoveDamage = 1;
    }
    gBattleMoveDamage *= -1;
    if gBattleMons[gBattlerTarget].hp == gBattleMons[gBattlerTarget].maxHP {
        gBattlescriptCurrInstr = failPtr;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_trymirrormove() {
    let mut validMovesCount: i32 = 0;
    let mut i: i32 = 0;
    let mut r#move: u16 = 0;
    let mut validMoves: CArray<u16, 4> = zeroed();
    i = 0;
    while i < 3 {
        validMoves[i] = MOVE_NONE;
        i += 1;
    }
    validMovesCount = 0;
    i = 0;
    while i < gBattlersCount as i32 {
        if i != gBattlerAttacker as i32 {
            r#move = *(*gBattleStruct)
                .lastTakenMoveFrom
                .as_mut_ptr()
                .at(i * 2 + gBattlerAttacker as i32 * 8) as u16
                | (*(*gBattleStruct)
                    .lastTakenMoveFrom
                    .as_mut_ptr()
                    .at(i * 2 + gBattlerAttacker as i32 * 8)
                    .at(1) as u16)
                    << 8;
            if r#move != MOVE_NONE && r#move != MOVE_UNAVAILABLE {
                validMoves[validMovesCount] = r#move;
                validMovesCount += 1;
            }
        }
        i += 1;
    }
    r#move = *(*gBattleStruct)
        .lastTakenMove
        .as_mut_ptr()
        .at(gBattlerAttacker as i32 * 2) as u16
        | (*(*gBattleStruct)
            .lastTakenMove
            .as_mut_ptr()
            .at(gBattlerAttacker as i32 * 2)
            .at(1) as u16)
            << 8;
    if r#move != MOVE_NONE && r#move != MOVE_UNAVAILABLE {
        gHitMarker &= 0xfffffbff;
        gCurrentMove = r#move;
        gBattlerTarget = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
        gBattlescriptCurrInstr = gBattleScriptsForMoveEffects[gBattleMoves[gCurrentMove].effect];
    } else if validMovesCount != 0 {
        gHitMarker &= 0xfffffbff;
        i = rem_i32(Random() as i32, validMovesCount);
        gCurrentMove = validMoves[i];
        gBattlerTarget = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
        gBattlescriptCurrInstr = gBattleScriptsForMoveEffects[gBattleMoves[gCurrentMove].effect];
    } else {
        gSpecialStatuses[gBattlerAttacker].set_ppNotAffectedByPressure(TRUE as u32);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setrain() {
    if gBattleWeather as i32 & B_WEATHER_RAIN != 0 {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_WEATHER_FAILED;
    } else {
        gBattleWeather = B_WEATHER_RAIN_TEMPORARY;
        gBattleCommunication[5] = B_MSG_STARTED_RAIN;
        gWishFutureKnock.weatherDuration = 5;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setreflect() {
    if gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & SIDE_STATUS_REFLECT] as i32
        & SIDE_STATUS_REFLECT
        != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_SIDE_STATUS_FAILED;
    } else {
        gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & SIDE_STATUS_REFLECT] |=
            SIDE_STATUS_REFLECT as u16;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].reflectTimer = 5;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].reflectBattlerId =
            gBattlerAttacker;
        if gBattleTypeFlags & 1 != 0 && CountAliveMonsInBattle(1) == 2 {
            gBattleCommunication[5] = B_MSG_SET_REFLECT_DOUBLE;
        } else {
            gBattleCommunication[5] = B_MSG_SET_REFLECT_SINGLE;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setseeded() {
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0
        || gStatuses3[gBattlerTarget] & STATUS3_LEECHSEED != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_LEECH_SEED_MISS;
    } else if gBattleMons[gBattlerTarget].types[0] == TYPE_GRASS
        || gBattleMons[gBattlerTarget].types[1] == TYPE_GRASS
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_LEECH_SEED_FAIL;
    } else {
        gStatuses3[gBattlerTarget] |= gBattlerAttacker as u32;
        gStatuses3[gBattlerTarget] |= STATUS3_LEECHSEED;
        gBattleCommunication[5] = B_MSG_LEECH_SEED_SET;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_manipulatedamage() {
    match *gBattlescriptCurrInstr.at(1) {
        DMG_CHANGE_SIGN => {
            gBattleMoveDamage *= -1;
        }
        DMG_RECOIL_FROM_MISS => {
            gBattleMoveDamage = gBattleMoveDamage / 2;
            if gBattleMoveDamage == 0 {
                gBattleMoveDamage = 1;
            }
            if gBattleMons[gBattlerTarget].maxHP as i32 / 2 < gBattleMoveDamage {
                gBattleMoveDamage = gBattleMons[gBattlerTarget].maxHP as i32 / 2;
            }
        }
        DMG_DOUBLED => {
            gBattleMoveDamage *= 2;
        }
        _ => {}
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_trysetrest() {
    let mut failJump: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    gActiveBattler = {
        gBattlerTarget = gBattlerAttacker;
        gBattlerTarget
    };
    gBattleMoveDamage = gBattleMons[gBattlerTarget].maxHP as i32 * -1;
    if gBattleMons[gBattlerTarget].hp == gBattleMons[gBattlerTarget].maxHP {
        gBattlescriptCurrInstr = failJump;
    } else {
        if gBattleMons[gBattlerTarget].status1 & 248 != 0 {
            gBattleCommunication[5] = B_MSG_REST_STATUSED;
        } else {
            gBattleCommunication[5] = B_MSG_REST;
        }
        gBattleMons[gBattlerTarget].status1 = 3;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            0,
            4,
            &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifnotfirstturn() {
    let mut failJump: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if gDisableStructs[gBattlerAttacker].isFirstTurn != 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = failJump;
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop() {
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UproarWakeUpCheck(battler: u8) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    'l2: while i < gBattlersCount as i32 {
        'l1: {
            if gBattleMons[i].status2 & STATUS2_UPROAR == 0
                || gBattleMons[battler].ability == ABILITY_SOUNDPROOF
            {
                break 'l1;
            }
            gBattleScripting.battler = i as u8;
            if gBattlerTarget == 0xFF {
                gBattlerTarget = i as u8;
            } else if gBattlerTarget as i32 == i {
                gBattleCommunication[5] = B_MSG_CANT_SLEEP_UPROAR;
            } else {
                gBattleCommunication[5] = B_MSG_UPROAR_KEPT_AWAKE;
            }
            break 'l2;
        }
        i += 1;
    }
    if i == gBattlersCount as i32 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifcantmakeasleep() {
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if UproarWakeUpCheck(gBattlerTarget) != 0 {
        gBattlescriptCurrInstr = jumpPtr;
    } else if gBattleMons[gBattlerTarget].ability == ABILITY_INSOMNIA
        || gBattleMons[gBattlerTarget].ability == ABILITY_VITAL_SPIRIT
    {
        gLastUsedAbility = gBattleMons[gBattlerTarget].ability;
        gBattleCommunication[5] = B_MSG_STAYED_AWAKE_USING;
        gBattlescriptCurrInstr = jumpPtr;
        RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_stockpile() {
    if gDisableStructs[gBattlerAttacker].stockpileCounter == 3 {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_CANT_STOCKPILE;
    } else {
        gDisableStructs[gBattlerAttacker].stockpileCounter += 1;
        gBattleTextBuff1[0] = 0xFD;
        gBattleTextBuff1[1] = 1;
        gBattleTextBuff1[2] = 1;
        gBattleTextBuff1[3] = 1;
        gBattleTextBuff1[4] = gDisableStructs[gBattlerAttacker].stockpileCounter;
        gBattleTextBuff1[5] = 0xFF;
        gBattleCommunication[5] = B_MSG_STOCKPILED;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_stockpiletobasedamage() {
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if gDisableStructs[gBattlerAttacker].stockpileCounter == 0 {
        gBattlescriptCurrInstr = jumpPtr;
    } else {
        if gBattleCommunication[6] != B_MSG_PROTECTED {
            gBattleMoveDamage = CalculateBaseDamage(
                &raw mut gBattleMons[gBattlerAttacker],
                &raw mut gBattleMons[gBattlerTarget],
                gCurrentMove as u32,
                gSideStatuses[GetBattlerPosition(gBattlerTarget) as i32 & 1],
                0,
                0,
                gBattlerAttacker,
                gBattlerTarget,
            ) * gDisableStructs[gBattlerAttacker].stockpileCounter as i32;
            gBattleScripting.animTurn = gDisableStructs[gBattlerAttacker].stockpileCounter;
            if gProtectStructs[gBattlerAttacker].helpingHand() != 0 {
                gBattleMoveDamage = gBattleMoveDamage * 15 / 10;
            }
        }
        gDisableStructs[gBattlerAttacker].stockpileCounter = 0;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_stockpiletohpheal() {
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(1) as i32
        | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
        as usize as *mut u8;
    if gDisableStructs[gBattlerAttacker].stockpileCounter == 0 {
        gBattlescriptCurrInstr = jumpPtr;
        gBattleCommunication[5] = B_MSG_SWALLOW_FAILED;
    } else if gBattleMons[gBattlerAttacker].maxHP == gBattleMons[gBattlerAttacker].hp {
        gDisableStructs[gBattlerAttacker].stockpileCounter = 0;
        gBattlescriptCurrInstr = jumpPtr;
        gBattlerTarget = gBattlerAttacker;
        gBattleCommunication[5] = B_MSG_SWALLOW_FULL_HP;
    } else {
        gBattleMoveDamage = div_i32(
            gBattleMons[gBattlerAttacker].maxHP as i32,
            shl_i32(
                1,
                3 - gDisableStructs[gBattlerAttacker].stockpileCounter as u32,
            ),
        );
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattleMoveDamage *= -1;
        gBattleScripting.animTurn = gDisableStructs[gBattlerAttacker].stockpileCounter;
        gDisableStructs[gBattlerAttacker].stockpileCounter = 0;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        gBattlerTarget = gBattlerAttacker;
    }
}
pub(crate) unsafe extern "C" fn Cmd_negativedamage() {
    gBattleMoveDamage = -(gHpDealt / 2);
    if gBattleMoveDamage == 0 {
        gBattleMoveDamage = -1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn ChangeStatBuffs(
    mut statValue: i8,
    statId: u8,
    mut flags: u8,
    BS_ptr: *mut u8,
) -> u8 {
    let mut certain: u8 = FALSE;
    let mut notProtectAffected: u8 = FALSE;
    let mut index: u32 = 0;
    if flags as i32 & MOVE_EFFECT_AFFECTS_USER as i32 != 0 {
        gActiveBattler = gBattlerAttacker;
    } else {
        gActiveBattler = gBattlerTarget;
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
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 5;
    gBattleTextBuff1[2] = statId;
    gBattleTextBuff1[3] = 0xFF;
    if statValue <= -1 {
        if gSideTimers[GetBattlerPosition(gActiveBattler) as i32 & 1].mistTimer != 0
            && certain == 0
            && gCurrentMove != MOVE_CURSE
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                if gSpecialStatuses[gActiveBattler].statLowered() != 0 {
                    gBattlescriptCurrInstr = BS_ptr;
                } else {
                    BattleScriptPush(BS_ptr);
                    gBattleScripting.battler = gActiveBattler;
                    gBattlescriptCurrInstr = BattleScript_MistProtected.as_ptr().cast_mut();
                    gSpecialStatuses[gActiveBattler].set_statLowered(1);
                }
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if gCurrentMove != MOVE_CURSE
            && notProtectAffected != TRUE
            && JumpIfMoveAffectedByProtect(0) != 0
        {
            gBattlescriptCurrInstr = BattleScript_ButItFailed.as_ptr().cast_mut();
            return STAT_CHANGE_DIDNT_WORK;
        } else if (gBattleMons[gActiveBattler].ability == ABILITY_CLEAR_BODY
            || gBattleMons[gActiveBattler].ability == ABILITY_WHITE_SMOKE)
            && certain == 0
            && gCurrentMove != MOVE_CURSE
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                if gSpecialStatuses[gActiveBattler].statLowered() != 0 {
                    gBattlescriptCurrInstr = BS_ptr;
                } else {
                    BattleScriptPush(BS_ptr);
                    gBattleScripting.battler = gActiveBattler;
                    gBattlescriptCurrInstr = BattleScript_AbilityNoStatLoss.as_ptr().cast_mut();
                    gLastUsedAbility = gBattleMons[gActiveBattler].ability;
                    RecordAbilityBattle(gActiveBattler, gLastUsedAbility);
                    gSpecialStatuses[gActiveBattler].set_statLowered(1);
                }
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if gBattleMons[gActiveBattler].ability == ABILITY_KEEN_EYE
            && certain == 0
            && statId == STAT_ACC as u8
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                BattleScriptPush(BS_ptr);
                gBattleScripting.battler = gActiveBattler;
                gBattlescriptCurrInstr = BattleScript_AbilityNoSpecificStatLoss.as_ptr().cast_mut();
                gLastUsedAbility = gBattleMons[gActiveBattler].ability;
                RecordAbilityBattle(gActiveBattler, gLastUsedAbility);
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if gBattleMons[gActiveBattler].ability == ABILITY_HYPER_CUTTER
            && certain == 0
            && statId == STAT_ATK
        {
            if flags == STAT_CHANGE_ALLOW_PTR {
                BattleScriptPush(BS_ptr);
                gBattleScripting.battler = gActiveBattler;
                gBattlescriptCurrInstr = BattleScript_AbilityNoSpecificStatLoss.as_ptr().cast_mut();
                gLastUsedAbility = gBattleMons[gActiveBattler].ability;
                RecordAbilityBattle(gActiveBattler, gLastUsedAbility);
            }
            return STAT_CHANGE_DIDNT_WORK;
        } else if gBattleMons[gActiveBattler].ability == ABILITY_SHIELD_DUST && flags == 0 {
            return STAT_CHANGE_DIDNT_WORK;
        } else {
            statValue = -(statValue >> 4 & 7);
            gBattleTextBuff2[0] = B_BUFF_PLACEHOLDER_BEGIN;
            index = 1;
            if statValue == -2 {
                gBattleTextBuff2[1] = B_BUFF_STRING;
                gBattleTextBuff2[2] = STRINGID_STATHARSHLY;
                gBattleTextBuff2[3] = 0;
                index = 4;
            }
            gBattleTextBuff2[{
                let t1 = index;
                index += 1;
                t1
            }] = B_BUFF_STRING;
            gBattleTextBuff2[{
                let t2 = index;
                index += 1;
                t2
            }] = STRINGID_STATFELL;
            gBattleTextBuff2[{
                let t3 = index;
                index += 1;
                t3
            }] = 0;
            gBattleTextBuff2[index] = B_BUFF_EOS;
            if gBattleMons[gActiveBattler].statStages[statId] == MIN_STAT_STAGE {
                gBattleCommunication[5] = B_MSG_STAT_WONT_DECREASE;
            } else {
                gBattleCommunication[5] = (gBattlerTarget == gActiveBattler) as u8;
            }
        }
    } else {
        statValue = statValue >> 4 & 7;
        gBattleTextBuff2[0] = B_BUFF_PLACEHOLDER_BEGIN;
        index = 1;
        if statValue == 2 {
            gBattleTextBuff2[1] = B_BUFF_STRING;
            gBattleTextBuff2[2] = STRINGID_STATSHARPLY;
            gBattleTextBuff2[3] = 0;
            index = 4;
        }
        gBattleTextBuff2[{
            let t4 = index;
            index += 1;
            t4
        }] = B_BUFF_STRING;
        gBattleTextBuff2[{
            let t5 = index;
            index += 1;
            t5
        }] = STRINGID_STATROSE;
        gBattleTextBuff2[{
            let t6 = index;
            index += 1;
            t6
        }] = 0;
        gBattleTextBuff2[index] = B_BUFF_EOS;
        if gBattleMons[gActiveBattler].statStages[statId] == MAX_STAT_STAGE {
            gBattleCommunication[5] = B_MSG_STAT_WONT_INCREASE;
        } else {
            gBattleCommunication[5] = (gBattlerTarget == gActiveBattler) as u8;
        }
    }
    gBattleMons[gActiveBattler].statStages[statId] += statValue;
    if gBattleMons[gActiveBattler].statStages[statId] < MIN_STAT_STAGE {
        gBattleMons[gActiveBattler].statStages[statId] = MIN_STAT_STAGE;
    }
    if gBattleMons[gActiveBattler].statStages[statId] > MAX_STAT_STAGE {
        gBattleMons[gActiveBattler].statStages[statId] = MAX_STAT_STAGE;
    }
    if gBattleCommunication[5] == B_MSG_STAT_WONT_INCREASE
        && flags as i32 & STAT_CHANGE_ALLOW_PTR as i32 != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
    }
    if gBattleCommunication[5] == B_MSG_STAT_WONT_INCREASE
        && flags as i32 & STAT_CHANGE_ALLOW_PTR as i32 == 0
    {
        return STAT_CHANGE_DIDNT_WORK;
    }
    return STAT_CHANGE_WORKED;
}
pub(crate) unsafe extern "C" fn Cmd_statbuffchange() {
    let mut jumpPtr: *mut u8 = (*gBattlescriptCurrInstr.at(2) as i32
        | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
        | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
        | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
        as usize as *mut u8;
    if ChangeStatBuffs(
        gBattleScripting.statChanger as i8 & -16,
        gBattleScripting.statChanger & 0xF,
        *gBattlescriptCurrInstr.at(1),
        jumpPtr,
    ) == STAT_CHANGE_WORKED
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_normalisebuffs() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < gBattlersCount as i32 {
        j = 0;
        while j < NUM_BATTLE_STATS {
            gBattleMons[i].statStages[j] = DEFAULT_STAT_STAGE;
            j += 1;
        }
        i += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setbide() {
    gBattleMons[gBattlerAttacker].status2 |= STATUS2_MULTIPLETURNS;
    gLockedMoves[gBattlerAttacker] = gCurrentMove;
    gBideDmg[gBattlerAttacker] = 0;
    gBattleMons[gBattlerAttacker].status2 |= 512;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_confuseifrepeatingattackends() {
    if gBattleMons[gBattlerAttacker].status2 & STATUS2_LOCK_CONFUSE == 0 {
        gBattleCommunication[3] = 117;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setmultihitcounter() {
    if *gBattlescriptCurrInstr.at(1) != 0 {
        gMultiHitCounter = *gBattlescriptCurrInstr.at(1);
    } else {
        gMultiHitCounter = Random() as u8 & 3;
        if gMultiHitCounter > 1 {
            gMultiHitCounter = (Random() as u8 & 3) + 2;
        } else {
            gMultiHitCounter += 2;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_initmultihitstring() {
    gBattleScripting.multihitString[0] = 0xFD;
    gBattleScripting.multihitString[1] = 1;
    gBattleScripting.multihitString[2] = 1;
    gBattleScripting.multihitString[3] = 1;
    gBattleScripting.multihitString[4] = 0;
    gBattleScripting.multihitString[5] = 0xFF;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn TryDoForceSwitchOut() -> u8 {
    if gBattleMons[gBattlerAttacker].level >= gBattleMons[gBattlerTarget].level {
        *(*gBattleStruct)
            .battlerPartyIndexes
            .as_mut_ptr()
            .at(gBattlerTarget) = gBattlerPartyIndexes[gBattlerTarget] as u8;
    } else {
        let mut random: u16 = Random() & 0xFF;
        if (random as i32
            * (gBattleMons[gBattlerAttacker].level as i32
                + gBattleMons[gBattlerTarget].level as i32)
            >> 8) as u32
            + 1
            <= (gBattleMons[gBattlerTarget].level as i32 / 4) as u32
        {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
            return FALSE;
        }
        *(*gBattleStruct)
            .battlerPartyIndexes
            .as_mut_ptr()
            .at(gBattlerTarget) = gBattlerPartyIndexes[gBattlerTarget] as u8;
    }
    gBattlescriptCurrInstr = BattleScript_SuccessForceOut.as_ptr().cast_mut();
    return TRUE;
}
pub(crate) unsafe extern "C" fn Cmd_forcerandomswitch() {
    let mut i: i32 = 0;
    let mut battler1PartyId: i32 = 0;
    let mut battler2PartyId: i32 = 0;
    let mut firstMonId: i32 = 0;
    let mut lastMonId: i32 = 0;
    let mut monsCount: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut validMons: i32 = 0;
    let mut minNeeded: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
            party = gPlayerParty.as_mut_ptr();
        } else {
            party = gEnemyParty.as_mut_ptr();
        }
        if gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0
            && gBattleTypeFlags & BATTLE_TYPE_LINK != 0
            || gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0
                && gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0
            || gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        {
            if gBattlerTarget as i32 & BIT_FLANK as i32 != B_FLANK_LEFT {
                firstMonId = 3;
                lastMonId = PARTY_SIZE;
            } else {
                firstMonId = 0;
                lastMonId = 3;
            }
            monsCount = 3;
            minNeeded = 1;
            battler2PartyId = gBattlerPartyIndexes[gBattlerTarget] as i32;
            battler1PartyId = gBattlerPartyIndexes[gBattlerTarget as i32 ^ 2] as i32;
        } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
            && gBattleTypeFlags & BATTLE_TYPE_LINK != 0
            || gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
                && gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0
        {
            if GetLinkTrainerFlankId(GetBattlerMultiplayerId(gBattlerTarget as u16) as u8)
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
            battler2PartyId = gBattlerPartyIndexes[gBattlerTarget] as i32;
            battler1PartyId = gBattlerPartyIndexes[gBattlerTarget as i32 ^ 2] as i32;
        } else if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
                firstMonId = 0;
                lastMonId = PARTY_SIZE;
                monsCount = PARTY_SIZE;
                minNeeded = 2;
            } else {
                if gBattlerTarget as i32 & BIT_FLANK as i32 != B_FLANK_LEFT {
                    firstMonId = 3;
                    lastMonId = PARTY_SIZE;
                } else {
                    firstMonId = 0;
                    lastMonId = 3;
                }
                monsCount = 3;
                minNeeded = 1;
            }
            battler2PartyId = gBattlerPartyIndexes[gBattlerTarget] as i32;
            battler1PartyId = gBattlerPartyIndexes[gBattlerTarget as i32 ^ 2] as i32;
        } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
            firstMonId = 0;
            lastMonId = PARTY_SIZE;
            monsCount = PARTY_SIZE;
            minNeeded = 2;
            battler2PartyId = gBattlerPartyIndexes[gBattlerTarget] as i32;
            battler1PartyId = gBattlerPartyIndexes[gBattlerTarget as i32 ^ 2] as i32;
        } else {
            firstMonId = 0;
            lastMonId = PARTY_SIZE;
            monsCount = PARTY_SIZE;
            minNeeded = 1;
            battler2PartyId = gBattlerPartyIndexes[gBattlerTarget] as i32;
            battler1PartyId = gBattlerPartyIndexes[gBattlerTarget] as i32;
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
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
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
            *(*gBattleStruct)
                .monToSwitchIntoId
                .as_mut_ptr()
                .at(gBattlerTarget) = i as u8;
            if IsMultiBattle() == 0 {
                SwitchPartyOrder(gBattlerTarget);
            }
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0
                && gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0
                || gBattleTypeFlags & BATTLE_TYPE_LINK != 0
                    && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
                || gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0
                    && gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0
                || gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0
                    && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
            {
                SwitchPartyOrderLinkMulti(gBattlerTarget, i as u8, 0);
                SwitchPartyOrderLinkMulti(gBattlerTarget ^ 2, i as u8, 1);
            }
            if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
                SwitchPartyOrderInGameMulti(gBattlerTarget, i as u8);
            }
        }
    } else {
        TryDoForceSwitchOut();
    }
}
pub(crate) unsafe extern "C" fn Cmd_tryconversiontypechange() {
    let mut validMoves: u8 = 0;
    let mut moveChecked: u8 = 0;
    let mut moveType: u8 = 0;
    while validMoves < MAX_MON_MOVES as u8 {
        if gBattleMons[gBattlerAttacker].moves[validMoves] == MOVE_NONE {
            break;
        }
        validMoves += 1;
    }
    moveChecked = 0;
    while moveChecked < validMoves {
        moveType = gBattleMoves[gBattleMons[gBattlerAttacker].moves[moveChecked]].r#type;
        if moveType == TYPE_MYSTERY {
            if gBattleMons[gBattlerAttacker].types[0] == TYPE_GHOST
                || gBattleMons[gBattlerAttacker].types[1] == TYPE_GHOST
            {
                moveType = TYPE_GHOST;
            } else {
                moveType = TYPE_NORMAL;
            }
        }
        if moveType != gBattleMons[gBattlerAttacker].types[0]
            && moveType != gBattleMons[gBattlerAttacker].types[1]
        {
            break;
        }
        moveChecked += 1;
    }
    if moveChecked == validMoves {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
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
            moveType = gBattleMoves[gBattleMons[gBattlerAttacker].moves[moveChecked]].r#type;
            if moveType == TYPE_MYSTERY {
                if gBattleMons[gBattlerAttacker].types[0] == TYPE_GHOST
                    || gBattleMons[gBattlerAttacker].types[1] == TYPE_GHOST
                {
                    moveType = TYPE_GHOST;
                } else {
                    moveType = TYPE_NORMAL;
                }
            }
            if !(moveType == gBattleMons[gBattlerAttacker].types[0]
                || moveType == gBattleMons[gBattlerAttacker].types[1])
            {
                break;
            }
        }
        gBattleMons[gBattlerAttacker].types[0] = moveType;
        gBattleMons[gBattlerAttacker].types[1] = moveType;
        gBattleTextBuff1[0] = 0xFD;
        gBattleTextBuff1[1] = 3;
        gBattleTextBuff1[2] = moveType;
        gBattleTextBuff1[3] = 0xFF;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_givepaydaymoney() {
    if gBattleTypeFlags & 0x2000002 == 0 && gPaydayMoney != 0 {
        let mut bonusMoney: u32 = gPaydayMoney as u32 * (*gBattleStruct).moneyMultiplier as u32;
        AddMoney(&raw mut (*gSaveBlock1Ptr).money, bonusMoney);
        gBattleTextBuff1[0] = 0xFD;
        gBattleTextBuff1[1] = 1;
        gBattleTextBuff1[2] = 2;
        gBattleTextBuff1[3] = 5;
        gBattleTextBuff1[4] = bonusMoney as u8;
        gBattleTextBuff1[5] = ((bonusMoney & 0x0000FF00) >> 8) as u8;
        gBattleTextBuff1[6] = 0xFF;
        BattleScriptPush(gBattlescriptCurrInstr.at(1));
        gBattlescriptCurrInstr = BattleScript_PrintPayDayMoneyString.as_ptr().cast_mut();
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setlightscreen() {
    if gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & 1] as i32
        & SIDE_STATUS_LIGHTSCREEN
        != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_SIDE_STATUS_FAILED;
    } else {
        gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & 1] |=
            SIDE_STATUS_LIGHTSCREEN as u16;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].lightscreenTimer = 5;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].lightscreenBattlerId =
            gBattlerAttacker;
        if gBattleTypeFlags & 1 != 0 && CountAliveMonsInBattle(1) == 2 {
            gBattleCommunication[5] = B_MSG_SET_LIGHTSCREEN_DOUBLE;
        } else {
            gBattleCommunication[5] = B_MSG_SET_LIGHTSCREEN_SINGLE;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_tryKO() {
    let mut holdEffect: u8 = 0;
    let mut param: u8 = 0;
    if gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gBattlerTarget].holdEffect;
        param = gEnigmaBerries[gBattlerTarget].holdEffectParam;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[gBattlerTarget].item);
        param = GetItemHoldEffectParam(gBattleMons[gBattlerTarget].item);
    }
    gPotentialItemEffectBattler = gBattlerTarget;
    if holdEffect == HOLD_EFFECT_FOCUS_BAND && Random() as i32 % 100 < param as i32 {
        RecordItemEffectBattle(gBattlerTarget, HOLD_EFFECT_FOCUS_BAND);
        gSpecialStatuses[gBattlerTarget].set_focusBanded(1);
    }
    if gBattleMons[gBattlerTarget].ability == ABILITY_STURDY {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gLastUsedAbility = ABILITY_STURDY;
        gBattlescriptCurrInstr = BattleScript_SturdyPreventsOHKO.as_ptr().cast_mut();
        RecordAbilityBattle(gBattlerTarget, ABILITY_STURDY);
    } else {
        let mut chance: u16 = 0;
        if gStatuses3[gBattlerTarget] & STATUS3_ALWAYS_HITS == 0 {
            chance = gBattleMoves[gCurrentMove].accuracy as u16
                + (gBattleMons[gBattlerAttacker].level as u16
                    - gBattleMons[gBattlerTarget].level as u16);
            if Random() as i32 % 100 + 1 < chance as i32
                && gBattleMons[gBattlerAttacker].level >= gBattleMons[gBattlerTarget].level
            {
                chance = TRUE as u16;
            } else {
                chance = FALSE as u16;
            }
        } else if gDisableStructs[gBattlerTarget].battlerWithSureHit == gBattlerAttacker
            && gBattleMons[gBattlerAttacker].level >= gBattleMons[gBattlerTarget].level
        {
            chance = TRUE as u16;
        } else {
            chance = gBattleMoves[gCurrentMove].accuracy as u16
                + (gBattleMons[gBattlerAttacker].level as u16
                    - gBattleMons[gBattlerTarget].level as u16);
            if Random() as i32 % 100 + 1 < chance as i32
                && gBattleMons[gBattlerAttacker].level >= gBattleMons[gBattlerTarget].level
            {
                chance = TRUE as u16;
            } else {
                chance = FALSE as u16;
            }
        }
        if chance != 0 {
            if gProtectStructs[gBattlerTarget].endured() != 0 {
                gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 - 1;
                gMoveResultFlags |= MOVE_RESULT_FOE_ENDURED;
            } else if gSpecialStatuses[gBattlerTarget].focusBanded() != 0 {
                gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 - 1;
                gMoveResultFlags |= MOVE_RESULT_FOE_HUNG_ON;
                gLastUsedItem = gBattleMons[gBattlerTarget].item;
            } else {
                gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32;
                gMoveResultFlags |= MOVE_RESULT_ONE_HIT_KO as u8;
            }
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        } else {
            gMoveResultFlags |= MOVE_RESULT_MISSED;
            if gBattleMons[gBattlerAttacker].level >= gBattleMons[gBattlerTarget].level {
                gBattleCommunication[5] = B_MSG_KO_MISS;
            } else {
                gBattleCommunication[5] = B_MSG_KO_UNAFFECTED;
            }
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_damagetohalftargethp() {
    gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 / 2;
    if gBattleMoveDamage == 0 {
        gBattleMoveDamage = 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setsandstorm() {
    if gBattleWeather as i32 & B_WEATHER_SANDSTORM != 0 {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_WEATHER_FAILED;
    } else {
        gBattleWeather = B_WEATHER_SANDSTORM_TEMPORARY;
        gBattleCommunication[5] = B_MSG_STARTED_SANDSTORM;
        gWishFutureKnock.weatherDuration = 5;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_weatherdamage() {
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0 {
        if gBattleWeather as i32 & B_WEATHER_SANDSTORM != 0 {
            if gBattleMons[gBattlerAttacker].types[0] != TYPE_ROCK
                && gBattleMons[gBattlerAttacker].types[0] != TYPE_STEEL
                && gBattleMons[gBattlerAttacker].types[0] != TYPE_GROUND
                && gBattleMons[gBattlerAttacker].types[1] != TYPE_ROCK
                && gBattleMons[gBattlerAttacker].types[1] != TYPE_STEEL
                && gBattleMons[gBattlerAttacker].types[1] != TYPE_GROUND
                && gBattleMons[gBattlerAttacker].ability != ABILITY_SAND_VEIL
                && gStatuses3[gBattlerAttacker] & STATUS3_UNDERGROUND == 0
                && gStatuses3[gBattlerAttacker] & STATUS3_UNDERWATER == 0
            {
                gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 16;
                if gBattleMoveDamage == 0 {
                    gBattleMoveDamage = 1;
                }
            } else {
                gBattleMoveDamage = 0;
            }
        }
        if gBattleWeather as i32 & B_WEATHER_HAIL != 0 {
            if !(gBattleMons[gBattlerAttacker].types[0] == TYPE_ICE
                || gBattleMons[gBattlerAttacker].types[1] == TYPE_ICE)
                && gStatuses3[gBattlerAttacker] & STATUS3_UNDERGROUND == 0
                && gStatuses3[gBattlerAttacker] & STATUS3_UNDERWATER == 0
            {
                gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 16;
                if gBattleMoveDamage == 0 {
                    gBattleMoveDamage = 1;
                }
            } else {
                gBattleMoveDamage = 0;
            }
        }
    } else {
        gBattleMoveDamage = 0;
    }
    if gAbsentBattlerFlags as u32 & gBitTable[gBattlerAttacker] != 0 {
        gBattleMoveDamage = 0;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_tryinfatuating() {
    let mut monAttacker: *mut Pokemon = null_mut();
    let mut monTarget: *mut Pokemon = null_mut();
    let mut speciesAttacker: u16 = 0;
    let mut speciesTarget: u16 = 0;
    let mut personalityAttacker: u32 = 0;
    let mut personalityTarget: u32 = 0;
    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
        monAttacker = &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerAttacker]];
    } else {
        monAttacker = &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker]];
    }
    if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
        monTarget = &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerTarget]];
    } else {
        monTarget = &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]];
    }
    speciesAttacker = GetMonData2(monAttacker, MON_DATA_SPECIES) as u16;
    personalityAttacker = GetMonData2(monAttacker, MON_DATA_PERSONALITY);
    speciesTarget = GetMonData2(monTarget, MON_DATA_SPECIES) as u16;
    personalityTarget = GetMonData2(monTarget, MON_DATA_PERSONALITY);
    if gBattleMons[gBattlerTarget].ability == ABILITY_OBLIVIOUS {
        gBattlescriptCurrInstr = BattleScript_ObliviousPreventsAttraction.as_ptr().cast_mut();
        gLastUsedAbility = ABILITY_OBLIVIOUS;
        RecordAbilityBattle(gBattlerTarget, ABILITY_OBLIVIOUS);
    } else {
        if GetGenderFromSpeciesAndPersonality(speciesAttacker, personalityAttacker)
            == GetGenderFromSpeciesAndPersonality(speciesTarget, personalityTarget)
            || gBattleMons[gBattlerTarget].status2 & STATUS2_INFATUATION != 0
            || GetGenderFromSpeciesAndPersonality(speciesAttacker, personalityAttacker)
                == MON_GENDERLESS
            || GetGenderFromSpeciesAndPersonality(speciesTarget, personalityTarget)
                == MON_GENDERLESS
        {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            gBattleMons[gBattlerTarget].status2 |= gBitTable[gBattlerAttacker] << 16;
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_updatestatusicon() {
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if *gBattlescriptCurrInstr.at(1) != BS_ATTACKER_WITH_PARTNER {
        gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
        BtlController_EmitStatusIconUpdate(
            B_COMM_TO_CONTROLLER,
            gBattleMons[gActiveBattler].status1,
            gBattleMons[gActiveBattler].status2,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    } else {
        gActiveBattler = gBattlerAttacker;
        if gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0 {
            BtlController_EmitStatusIconUpdate(
                B_COMM_TO_CONTROLLER,
                gBattleMons[gActiveBattler].status1,
                gBattleMons[gActiveBattler].status2,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
            gActiveBattler = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 2);
            if gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0 {
                BtlController_EmitStatusIconUpdate(
                    B_COMM_TO_CONTROLLER,
                    gBattleMons[gActiveBattler].status1,
                    gBattleMons[gActiveBattler].status2,
                );
                MarkBattlerForControllerExec(gActiveBattler);
            }
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setmist() {
    if gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].mistTimer != 0 {
        gMoveResultFlags |= MOVE_RESULT_FAILED as u8;
        gBattleCommunication[5] = B_MSG_MIST_FAILED;
    } else {
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].mistTimer = 5;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].mistBattlerId =
            gBattlerAttacker;
        gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & 1] |= SIDE_STATUS_MIST;
        gBattleCommunication[5] = B_MSG_SET_MIST;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setfocusenergy() {
    if gBattleMons[gBattlerAttacker].status2 & STATUS2_FOCUS_ENERGY != 0 {
        gMoveResultFlags |= MOVE_RESULT_FAILED as u8;
        gBattleCommunication[5] = B_MSG_FOCUS_ENERGY_FAILED;
    } else {
        gBattleMons[gBattlerAttacker].status2 |= STATUS2_FOCUS_ENERGY;
        gBattleCommunication[5] = B_MSG_GETTING_PUMPED;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_transformdataexecution() {
    gChosenMove = MOVE_UNAVAILABLE;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    if gBattleMons[gBattlerTarget].status2 & STATUS2_TRANSFORMED != 0
        || gStatuses3[gBattlerTarget] & STATUS3_SEMI_INVULNERABLE != 0
    {
        gMoveResultFlags |= MOVE_RESULT_FAILED as u8;
        gBattleCommunication[5] = B_MSG_TRANSFORM_FAILED;
    } else {
        let mut i: i32 = 0;
        let mut battleMonAttacker: *mut u8 = null_mut();
        let mut battleMonTarget: *mut u8 = null_mut();
        gBattleMons[gBattlerAttacker].status2 |= STATUS2_TRANSFORMED;
        gDisableStructs[gBattlerAttacker].disabledMove = MOVE_NONE;
        gDisableStructs[gBattlerAttacker].set_disableTimer(0);
        gDisableStructs[gBattlerAttacker].transformedMonPersonality =
            gBattleMons[gBattlerTarget].personality;
        gDisableStructs[gBattlerAttacker].set_mimickedMoves(0);
        gBattleTextBuff1[0] = 0xFD;
        gBattleTextBuff1[1] = 6;
        gBattleTextBuff1[2] = gBattleMons[gBattlerTarget].species as u8;
        gBattleTextBuff1[3] = ((gBattleMons[gBattlerTarget].species as i32 & 0xFF00) >> 8) as u8;
        gBattleTextBuff1[4] = 0xFF;
        battleMonAttacker = &raw mut gBattleMons[gBattlerAttacker] as *mut u8;
        battleMonTarget = &raw mut gBattleMons[gBattlerTarget] as *mut u8;
        i = 0;
        while i < 36 {
            *battleMonAttacker.at(i) = *battleMonTarget.at(i);
            i += 1;
        }
        i = 0;
        while i < MAX_MON_MOVES {
            if gBattleMoves[gBattleMons[gBattlerAttacker].moves[i]].pp < 5 {
                gBattleMons[gBattlerAttacker].pp[i] =
                    gBattleMoves[gBattleMons[gBattlerAttacker].moves[i]].pp;
            } else {
                gBattleMons[gBattlerAttacker].pp[i] = 5;
            }
            i += 1;
        }
        gActiveBattler = gBattlerAttacker;
        BtlController_EmitResetActionMoveSelection(B_COMM_TO_CONTROLLER, RESET_MOVE_SELECTION);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattleCommunication[5] = B_MSG_TRANSFORMED;
    }
}
pub(crate) unsafe extern "C" fn Cmd_setsubstitute() {
    let mut hp: u32 = (gBattleMons[gBattlerAttacker].maxHP as i32 / 4) as u32;
    if gBattleMons[gBattlerAttacker].maxHP as i32 / 4 == 0 {
        hp = 1;
    }
    if gBattleMons[gBattlerAttacker].hp as u32 <= hp {
        gBattleMoveDamage = 0;
        gBattleCommunication[5] = B_MSG_SUBSTITUTE_FAILED;
    } else {
        gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 4;
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattleMons[gBattlerAttacker].status2 |= STATUS2_SUBSTITUTE;
        gBattleMons[gBattlerAttacker].status2 &= 0xffff1fff;
        gDisableStructs[gBattlerAttacker].substituteHP = gBattleMoveDamage as u8;
        gBattleCommunication[5] = B_MSG_SET_SUBSTITUTE;
        gHitMarker |= HITMARKER_IGNORE_SUBSTITUTE;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn IsMoveUncopyableByMimic(r#move: u16) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while sMovesForbiddenToCopy[i] != MIMIC_FORBIDDEN_END && sMovesForbiddenToCopy[i] != r#move {
        i += 1;
    }
    return (sMovesForbiddenToCopy[i] != MIMIC_FORBIDDEN_END) as u8;
}
pub(crate) unsafe extern "C" fn Cmd_mimicattackcopy() {
    gChosenMove = MOVE_UNAVAILABLE;
    if IsMoveUncopyableByMimic(gLastMoves[gBattlerTarget]) != 0
        || gBattleMons[gBattlerAttacker].status2 & STATUS2_TRANSFORMED != 0
        || gLastMoves[gBattlerTarget] == MOVE_NONE
        || gLastMoves[gBattlerTarget] == MOVE_UNAVAILABLE
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        let mut i: i32 = 0;
        i = 0;
        while i < MAX_MON_MOVES {
            if gBattleMons[gBattlerAttacker].moves[i] == gLastMoves[gBattlerTarget] {
                break;
            }
            i += 1;
        }
        if i == MAX_MON_MOVES {
            gBattleMons[gBattlerAttacker].moves[gCurrMovePos] = gLastMoves[gBattlerTarget];
            if gBattleMoves[gLastMoves[gBattlerTarget]].pp < 5 {
                gBattleMons[gBattlerAttacker].pp[gCurrMovePos] =
                    gBattleMoves[gLastMoves[gBattlerTarget]].pp;
            } else {
                gBattleMons[gBattlerAttacker].pp[gCurrMovePos] = 5;
            }
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 2;
            gBattleTextBuff1[2] = gLastMoves[gBattlerTarget] as u8 & 0xFF;
            gBattleTextBuff1[3] = ((gLastMoves[gBattlerTarget] as i32 & 0xFF00) >> 8) as u8;
            gBattleTextBuff1[4] = 0xFF;
            gDisableStructs[gBattlerAttacker].set_mimickedMoves(
                gDisableStructs[gBattlerAttacker].mimickedMoves() | gBitTable[gCurrMovePos] as u8,
            );
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        } else {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_metronome() {
    loop {
        let mut i: i32 = 0;
        gCurrentMove = (Random() & 0x1FF) + 1;
        if gCurrentMove >= MOVES_COUNT {
            continue;
        }
        i = 0;
        while i < MAX_MON_MOVES {
            i += 1;
        }
        i = -1;
        loop {
            i += 1;
            if sMovesForbiddenToCopy[i] == gCurrentMove {
                break;
            }
            if sMovesForbiddenToCopy[i] == METRONOME_FORBIDDEN_END {
                break;
            }
        }
        if sMovesForbiddenToCopy[i] == METRONOME_FORBIDDEN_END {
            gHitMarker &= 0xfffffbff;
            gBattlescriptCurrInstr =
                gBattleScriptsForMoveEffects[gBattleMoves[gCurrentMove].effect];
            gBattlerTarget = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_dmgtolevel() {
    gBattleMoveDamage = gBattleMons[gBattlerAttacker].level as i32;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_psywavedamageeffect() {
    let mut randDamage: i32 = 0;
    while ({
        randDamage = Random() as i32 % 16;
        randDamage
    }) > 10
    {}
    randDamage *= 10;
    gBattleMoveDamage = gBattleMons[gBattlerAttacker].level as i32 * (randDamage + 50) / 100;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_counterdamagecalculator() {
    let mut sideAttacker: u8 = GetBattlerSide(gBattlerAttacker);
    let mut sideTarget: u8 = GetBattlerSide(gProtectStructs[gBattlerAttacker].physicalBattlerId);
    if gProtectStructs[gBattlerAttacker].physicalDmg != 0
        && sideAttacker != sideTarget
        && gBattleMons[gProtectStructs[gBattlerAttacker].physicalBattlerId].hp != 0
    {
        gBattleMoveDamage = gProtectStructs[gBattlerAttacker].physicalDmg as i32 * 2;
        if gSideTimers[sideTarget].followmeTimer != 0
            && gBattleMons[gSideTimers[sideTarget].followmeTarget].hp != 0
        {
            gBattlerTarget = gSideTimers[sideTarget].followmeTarget;
        } else {
            gBattlerTarget = gProtectStructs[gBattlerAttacker].physicalBattlerId;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gSpecialStatuses[gBattlerAttacker].set_ppNotAffectedByPressure(1);
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_mirrorcoatdamagecalculator() {
    let mut sideAttacker: u8 = GetBattlerSide(gBattlerAttacker);
    let mut sideTarget: u8 = GetBattlerSide(gProtectStructs[gBattlerAttacker].specialBattlerId);
    if gProtectStructs[gBattlerAttacker].specialDmg != 0
        && sideAttacker != sideTarget
        && gBattleMons[gProtectStructs[gBattlerAttacker].specialBattlerId].hp != 0
    {
        gBattleMoveDamage = gProtectStructs[gBattlerAttacker].specialDmg as i32 * 2;
        if gSideTimers[sideTarget].followmeTimer != 0
            && gBattleMons[gSideTimers[sideTarget].followmeTarget].hp != 0
        {
            gBattlerTarget = gSideTimers[sideTarget].followmeTarget;
        } else {
            gBattlerTarget = gProtectStructs[gBattlerAttacker].specialBattlerId;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gSpecialStatuses[gBattlerAttacker].set_ppNotAffectedByPressure(1);
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_disablelastusedattack() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if gBattleMons[gBattlerTarget].moves[i] == gLastMoves[gBattlerTarget] {
            break;
        }
        i += 1;
    }
    if gDisableStructs[gBattlerTarget].disabledMove == MOVE_NONE
        && i != MAX_MON_MOVES
        && gBattleMons[gBattlerTarget].pp[i] != 0
    {
        gBattleTextBuff1[0] = 0xFD;
        gBattleTextBuff1[1] = 2;
        gBattleTextBuff1[2] = gBattleMons[gBattlerTarget].moves[i] as u8 & 0xFF;
        gBattleTextBuff1[3] = ((gBattleMons[gBattlerTarget].moves[i] as i32 & 0xFF00) >> 8) as u8;
        gBattleTextBuff1[4] = 0xFF;
        gDisableStructs[gBattlerTarget].disabledMove = gBattleMons[gBattlerTarget].moves[i];
        gDisableStructs[gBattlerTarget].set_disableTimer((Random() as u8 & 3) + 2);
        gDisableStructs[gBattlerTarget]
            .set_disableTimerStartValue(gDisableStructs[gBattlerTarget].disableTimer());
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetencore() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if gBattleMons[gBattlerTarget].moves[i] == gLastMoves[gBattlerTarget] {
            break;
        }
        i += 1;
    }
    if gLastMoves[gBattlerTarget] == MOVE_STRUGGLE
        || gLastMoves[gBattlerTarget] == MOVE_ENCORE
        || gLastMoves[gBattlerTarget] == MOVE_MIRROR_MOVE
    {
        i = MAX_MON_MOVES;
    }
    if gDisableStructs[gBattlerTarget].encoredMove == MOVE_NONE
        && i != MAX_MON_MOVES
        && gBattleMons[gBattlerTarget].pp[i] != 0
    {
        gDisableStructs[gBattlerTarget].encoredMove = gBattleMons[gBattlerTarget].moves[i];
        gDisableStructs[gBattlerTarget].encoredMovePos = i as u8;
        gDisableStructs[gBattlerTarget].set_encoreTimer((Random() as u8 & 3) + 3);
        gDisableStructs[gBattlerTarget]
            .set_encoreTimerStartValue(gDisableStructs[gBattlerTarget].encoreTimer());
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_painsplitdmgcalc() {
    if gBattleMons[gBattlerTarget].status2 & STATUS2_SUBSTITUTE == 0 {
        let mut hpDiff: i32 =
            (gBattleMons[gBattlerAttacker].hp as i32 + gBattleMons[gBattlerTarget].hp as i32) / 2;
        let mut painSplitHp: i32 = {
            gBattleMoveDamage = gBattleMons[gBattlerTarget].hp as i32 - hpDiff;
            gBattleMoveDamage
        };
        let mut storeLoc: *mut u8 = &raw mut gBattleScripting.painSplitHp as *mut c_void as *mut u8;
        *storeLoc = painSplitHp as u8;
        *storeLoc.at(1) = ((painSplitHp & 0x0000FF00) >> 8) as u8;
        *storeLoc.at(2) = ((painSplitHp & 0x00FF0000) >> 16) as u8;
        *storeLoc.at(3) = ((painSplitHp as u32 & 0xFF000000) >> 24) as u8;
        gBattleMoveDamage = gBattleMons[gBattlerAttacker].hp as i32 - hpDiff;
        gSpecialStatuses[gBattlerTarget].shellBellDmg = IGNORE_SHELL_BELL;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_settypetorandomresistance() {
    if gLastLandedMoves[gBattlerAttacker] == MOVE_NONE
        || gLastLandedMoves[gBattlerAttacker] == MOVE_UNAVAILABLE
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else if IsTwoTurnsMove(gLastLandedMoves[gBattlerAttacker]) != 0
        && gBattleMons[gLastHitBy[gBattlerAttacker]].status2 & STATUS2_MULTIPLETURNS != 0
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut rands: i32 = 0;
        rands = 0;
        while rands < 1000 {
            while ({
                i = Random() as i32 % 128;
                i
            }) > 112
            {}
            i *= 3;
            if gTypeEffectiveness[i + 0] as u16 == gLastHitByType[gBattlerAttacker]
                && gTypeEffectiveness[i + 2] <= TYPE_MUL_NOT_EFFECTIVE
                && !(gBattleMons[gBattlerAttacker].types[0] == gTypeEffectiveness[i + 1]
                    || gBattleMons[gBattlerAttacker].types[1] == gTypeEffectiveness[i + 1])
            {
                gBattleMons[gBattlerAttacker].types[0] = gTypeEffectiveness[i + 1];
                gBattleMons[gBattlerAttacker].types[1] = gTypeEffectiveness[i + 1];
                gBattleTextBuff1[0] = 0xFD;
                gBattleTextBuff1[1] = 3;
                gBattleTextBuff1[2] = gTypeEffectiveness[i + 1];
                gBattleTextBuff1[3] = 0xFF;
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
                return;
            }
            rands += 1;
        }
        j = 0;
        rands = 0;
        while rands < 336 {
            match gTypeEffectiveness[j + 0] {
                TYPE_ENDTABLE | TYPE_FORESIGHT => {}
                _ => {
                    if gTypeEffectiveness[j + 0] as u16 == gLastHitByType[gBattlerAttacker]
                        && gTypeEffectiveness[j + 2] <= 5
                        && !(gBattleMons[gBattlerAttacker].types[0] == gTypeEffectiveness[i + 1]
                            || gBattleMons[gBattlerAttacker].types[1] == gTypeEffectiveness[i + 1])
                    {
                        gBattleMons[gBattlerAttacker].types[0] = gTypeEffectiveness[rands + 1];
                        gBattleMons[gBattlerAttacker].types[1] = gTypeEffectiveness[rands + 1];
                        gBattleTextBuff1[0] = 0xFD;
                        gBattleTextBuff1[1] = 3;
                        gBattleTextBuff1[2] = gTypeEffectiveness[rands + 1];
                        gBattleTextBuff1[3] = 0xFF;
                        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
                        return;
                    }
                }
            }
            j += 3;
            rands += 3;
        }
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_setalwayshitflag() {
    gStatuses3[gBattlerTarget] &= 0xffffffe7;
    gStatuses3[gBattlerTarget] |= 16;
    gDisableStructs[gBattlerTarget].battlerWithSureHit = gBattlerAttacker;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_copymovepermanently() {
    gChosenMove = MOVE_UNAVAILABLE;
    if gBattleMons[gBattlerAttacker].status2 & STATUS2_TRANSFORMED == 0
        && gLastPrintedMoves[gBattlerTarget] != MOVE_STRUGGLE
        && gLastPrintedMoves[gBattlerTarget] != MOVE_NONE
        && gLastPrintedMoves[gBattlerTarget] != MOVE_UNAVAILABLE
        && gLastPrintedMoves[gBattlerTarget] != MOVE_SKETCH
    {
        let mut i: i32 = 0;
        i = 0;
        'l2: while i < MAX_MON_MOVES {
            'l1: {
                if gBattleMons[gBattlerAttacker].moves[i] == MOVE_SKETCH {
                    break 'l1;
                }
                if gBattleMons[gBattlerAttacker].moves[i] == gLastPrintedMoves[gBattlerTarget] {
                    break 'l2;
                }
            }
            i += 1;
        }
        if i != MAX_MON_MOVES {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            let mut movePPData: MovePPInfo = zeroed();
            gBattleMons[gBattlerAttacker].moves[gCurrMovePos] = gLastPrintedMoves[gBattlerTarget];
            gBattleMons[gBattlerAttacker].pp[gCurrMovePos] =
                gBattleMoves[gLastPrintedMoves[gBattlerTarget]].pp;
            gActiveBattler = gBattlerAttacker;
            i = 0;
            while i < MAX_MON_MOVES {
                movePPData.moves[i] = gBattleMons[gBattlerAttacker].moves[i];
                movePPData.pp[i] = gBattleMons[gBattlerAttacker].pp[i];
                i += 1;
            }
            movePPData.ppBonuses = gBattleMons[gBattlerAttacker].ppBonuses;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_MOVES_PP_BATTLE,
                0,
                16,
                &raw mut movePPData as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler);
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 2;
            gBattleTextBuff1[2] = gLastPrintedMoves[gBattlerTarget] as u8 & 0xFF;
            gBattleTextBuff1[3] = ((gLastPrintedMoves[gBattlerTarget] as i32 & 0xFF00) >> 8) as u8;
            gBattleTextBuff1[4] = 0xFF;
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        }
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn IsTwoTurnsMove(r#move: u16) -> u8 {
    if gBattleMoves[r#move].effect == EFFECT_SKULL_BASH
        || gBattleMoves[r#move].effect == EFFECT_RAZOR_WIND
        || gBattleMoves[r#move].effect == EFFECT_SKY_ATTACK
        || gBattleMoves[r#move].effect == EFFECT_SOLAR_BEAM
        || gBattleMoves[r#move].effect == EFFECT_SEMI_INVULNERABLE
        || gBattleMoves[r#move].effect == EFFECT_BIDE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsInvalidForSleepTalkOrAssist(r#move: u16) -> u8 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AttacksThisTurn(battler: u8, r#move: u16) -> u8 {
    if gBattleMoves[r#move].effect == EFFECT_SOLAR_BEAM
        && gBattleWeather as i32 & B_WEATHER_SUN != 0
    {
        return 2;
    }
    if gBattleMoves[r#move].effect == EFFECT_SKULL_BASH
        || gBattleMoves[r#move].effect == EFFECT_RAZOR_WIND
        || gBattleMoves[r#move].effect == EFFECT_SKY_ATTACK
        || gBattleMoves[r#move].effect == EFFECT_SOLAR_BEAM
        || gBattleMoves[r#move].effect == EFFECT_SEMI_INVULNERABLE
        || gBattleMoves[r#move].effect == EFFECT_BIDE
    {
        if gHitMarker & HITMARKER_CHARGING != 0 {
            return 1;
        }
    }
    return 2;
}
pub(crate) unsafe extern "C" fn Cmd_trychoosesleeptalkmove() {
    let mut i: i32 = 0;
    let mut unusableMovesBits: u8 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if IsInvalidForSleepTalkOrAssist(gBattleMons[gBattlerAttacker].moves[i]) != 0
            || gBattleMons[gBattlerAttacker].moves[i] == MOVE_FOCUS_PUNCH
            || gBattleMons[gBattlerAttacker].moves[i] == MOVE_UPROAR
            || IsTwoTurnsMove(gBattleMons[gBattlerAttacker].moves[i]) != 0
        {
            unusableMovesBits |= gBitTable[i] as u8;
        }
        i += 1;
    }
    unusableMovesBits = CheckMoveLimitations(gBattlerAttacker, unusableMovesBits, 253);
    if unusableMovesBits == ALL_MOVES_MASK {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
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
        gCalledMove = gBattleMons[gBattlerAttacker].moves[movePosition];
        gCurrMovePos = movePosition as u8;
        gHitMarker &= 0xfffffbff;
        gBattlerTarget = GetMoveTarget(gCalledMove, NO_TARGET_OVERRIDE);
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_setdestinybond() {
    gBattleMons[gBattlerAttacker].status2 |= STATUS2_DESTINY_BOND;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn TrySetDestinyBondToHappen() {
    let mut sideAttacker: u8 = GetBattlerSide(gBattlerAttacker);
    let mut sideTarget: u8 = GetBattlerSide(gBattlerTarget);
    if gBattleMons[gBattlerTarget].status2 & STATUS2_DESTINY_BOND != 0
        && sideAttacker != sideTarget
        && gHitMarker & HITMARKER_GRUDGE == 0
    {
        gHitMarker |= HITMARKER_DESTINYBOND;
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetdestinybondtohappen() {
    TrySetDestinyBondToHappen();
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_remaininghptopower() {
    let mut i: i32 = 0;
    let mut hpFraction: i32 = GetScaledHPFraction(
        gBattleMons[gBattlerAttacker].hp as i16,
        gBattleMons[gBattlerAttacker].maxHP as i16,
        48,
    ) as i32;
    i = 0;
    while i < 12 {
        if hpFraction <= sFlailHpScaleToPowerTable[i] as i32 {
            break;
        }
        i += 2;
    }
    gDynamicBasePower = sFlailHpScaleToPowerTable[i + 1] as u16;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_tryspiteppreduce() {
    if gLastMoves[gBattlerTarget] != MOVE_NONE && gLastMoves[gBattlerTarget] != MOVE_UNAVAILABLE {
        let mut i: i32 = 0;
        i = 0;
        while i < MAX_MON_MOVES {
            if gLastMoves[gBattlerTarget] == gBattleMons[gBattlerTarget].moves[i] {
                break;
            }
            i += 1;
        }
        if i != MAX_MON_MOVES && gBattleMons[gBattlerTarget].pp[i] > 1 {
            let mut ppToDeduct: i32 = (Random() as i32 & 3) + 2;
            if (gBattleMons[gBattlerTarget].pp[i] as i32) < ppToDeduct {
                ppToDeduct = gBattleMons[gBattlerTarget].pp[i] as i32;
            }
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 2;
            gBattleTextBuff1[2] = gLastMoves[gBattlerTarget] as u8 & 0xFF;
            gBattleTextBuff1[3] = ((gLastMoves[gBattlerTarget] as i32 & 0xFF00) >> 8) as u8;
            gBattleTextBuff1[4] = 0xFF;
            ConvertIntToDecimalStringN(
                gBattleTextBuff2.as_mut_ptr(),
                ppToDeduct,
                STR_CONV_MODE_LEFT_ALIGN,
                1,
            );
            gBattleTextBuff2[0] = 0xFD;
            gBattleTextBuff2[1] = 1;
            gBattleTextBuff2[2] = 1;
            gBattleTextBuff2[3] = 1;
            gBattleTextBuff2[4] = ppToDeduct as u8;
            gBattleTextBuff2[5] = 0xFF;
            gBattleMons[gBattlerTarget].pp[i] -= ppToDeduct as u8;
            gActiveBattler = gBattlerTarget;
            if gDisableStructs[gActiveBattler].mimickedMoves() as u32 & gBitTable[i] == 0
                && gBattleMons[gActiveBattler].status2 & STATUS2_TRANSFORMED == 0
            {
                BtlController_EmitSetMonData(
                    B_COMM_TO_CONTROLLER,
                    REQUEST_PPMOVE1_BATTLE + i as u8,
                    0,
                    1,
                    &raw mut gBattleMons[gActiveBattler].pp[i] as *mut c_void,
                );
                MarkBattlerForControllerExec(gActiveBattler);
            }
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
            if gBattleMons[gBattlerTarget].pp[i] == 0 {
                CancelMultiTurnMoves(gBattlerTarget);
            }
        } else {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        }
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_healpartystatus() {
    let mut zero: u32 = 0;
    let mut toHeal: u8 = 0;
    if gCurrentMove == MOVE_HEAL_BELL {
        let mut party: *mut Pokemon = null_mut();
        let mut i: i32 = 0;
        gBattleCommunication[5] = B_MSG_BELL;
        if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
            party = gPlayerParty.as_mut_ptr();
        } else {
            party = gEnemyParty.as_mut_ptr();
        }
        if gBattleMons[gBattlerAttacker].ability != ABILITY_SOUNDPROOF {
            gBattleMons[gBattlerAttacker].status1 = 0;
            gBattleMons[gBattlerAttacker].status2 &= 0xf7ffffff;
        } else {
            RecordAbilityBattle(gBattlerAttacker, gBattleMons[gBattlerAttacker].ability);
            gBattleCommunication[5] |= B_MSG_BELL_SOUNDPROOF_ATTACKER;
        }
        gActiveBattler = {
            gBattleScripting.battler =
                GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 2);
            gBattleScripting.battler
        };
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0
        {
            if gBattleMons[gActiveBattler].ability != ABILITY_SOUNDPROOF {
                gBattleMons[gActiveBattler].status1 = 0;
                gBattleMons[gActiveBattler].status2 &= 0xf7ffffff;
            } else {
                RecordAbilityBattle(gActiveBattler, gBattleMons[gActiveBattler].ability);
                gBattleCommunication[5] |= B_MSG_BELL_SOUNDPROOF_PARTNER;
            }
        }
        i = 0;
        while i < PARTY_SIZE {
            let mut species: u16 = GetMonData2(party.at(i), MON_DATA_SPECIES_OR_EGG) as u16;
            let mut abilityNum: u8 = GetMonData2(party.at(i), MON_DATA_ABILITY_NUM) as u8;
            if species != SPECIES_NONE && species != SPECIES_EGG as u16 {
                let mut ability: u8 = 0;
                if gBattlerPartyIndexes[gBattlerAttacker] as i32 == i {
                    ability = gBattleMons[gBattlerAttacker].ability;
                } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
                    && gBattlerPartyIndexes[gActiveBattler] as i32 == i
                    && gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0
                {
                    ability = gBattleMons[gActiveBattler].ability;
                } else {
                    ability = GetAbilityBySpecies(species, abilityNum);
                }
                if ability != ABILITY_SOUNDPROOF {
                    toHeal |= shl_i32(1, i as u32) as u8;
                }
            }
            i += 1;
        }
    } else {
        gBattleCommunication[5] = B_MSG_SOOTHING_AROMA;
        toHeal = 63;
        gBattleMons[gBattlerAttacker].status1 = 0;
        gBattleMons[gBattlerAttacker].status2 &= 0xf7ffffff;
        gActiveBattler = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 2);
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0
        {
            gBattleMons[gActiveBattler].status1 = 0;
            gBattleMons[gActiveBattler].status2 &= 0xf7ffffff;
        }
    }
    if toHeal != 0 {
        gActiveBattler = gBattlerAttacker;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            toHeal,
            4,
            &raw mut zero as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_cursetarget() {
    if gBattleMons[gBattlerTarget].status2 & STATUS2_CURSED != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattleMons[gBattlerTarget].status2 |= STATUS2_CURSED;
        gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 2;
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetspikes() {
    let mut targetSide: u8 = GetBattlerSide(gBattlerAttacker) ^ 1;
    if gSideTimers[targetSide].spikesAmount == 3 {
        gSpecialStatuses[gBattlerAttacker].set_ppNotAffectedByPressure(1);
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gSideStatuses[targetSide] |= SIDE_STATUS_SPIKES as u16;
        gSideTimers[targetSide].spikesAmount += 1;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setforesight() {
    gBattleMons[gBattlerTarget].status2 |= STATUS2_FORESIGHT;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_trysetperishsong() {
    let mut i: i32 = 0;
    let mut notAffectedCount: i32 = 0;
    i = 0;
    while i < gBattlersCount as i32 {
        if gStatuses3[i] & STATUS3_PERISH_SONG != 0 || gBattleMons[i].ability == ABILITY_SOUNDPROOF
        {
            notAffectedCount += 1;
        } else {
            gStatuses3[i] |= STATUS3_PERISH_SONG;
            gDisableStructs[i].set_perishSongTimer(3);
            gDisableStructs[i].set_perishSongTimerStartValue(3);
        }
        i += 1;
    }
    PressurePPLoseOnUsingPerishSong(gBattlerAttacker);
    if notAffectedCount == gBattlersCount as i32 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_rolloutdamagecalculation() {
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        CancelMultiTurnMoves(gBattlerAttacker);
        gBattlescriptCurrInstr = BattleScript_MoveMissedPause.as_ptr().cast_mut();
    } else {
        let mut i: i32 = 0;
        if gBattleMons[gBattlerAttacker].status2 & STATUS2_MULTIPLETURNS == 0 {
            gDisableStructs[gBattlerAttacker].set_rolloutTimer(5);
            gDisableStructs[gBattlerAttacker].set_rolloutTimerStartValue(5);
            gBattleMons[gBattlerAttacker].status2 |= STATUS2_MULTIPLETURNS;
            gLockedMoves[gBattlerAttacker] = gCurrentMove;
        }
        if ({
            gDisableStructs[gBattlerAttacker]
                .set_rolloutTimer(gDisableStructs[gBattlerAttacker].rolloutTimer() - 1);
            gDisableStructs[gBattlerAttacker].rolloutTimer()
        }) == 0
        {
            gBattleMons[gBattlerAttacker].status2 &= 0xffffefff;
        }
        gDynamicBasePower = gBattleMoves[gCurrentMove].power as u16;
        i = 1;
        while i < 5 - gDisableStructs[gBattlerAttacker].rolloutTimer() as i32 {
            gDynamicBasePower *= 2;
            i += 1;
        }
        if gBattleMons[gBattlerAttacker].status2 & STATUS2_DEFENSE_CURL != 0 {
            gDynamicBasePower *= 2;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifconfusedandstatmaxed() {
    if gBattleMons[gBattlerTarget].status2 & STATUS2_CONFUSION != 0
        && gBattleMons[gBattlerTarget].statStages[*gBattlescriptCurrInstr.at(1)] == MAX_STAT_STAGE
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
            | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_furycuttercalc() {
    if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0 {
        gDisableStructs[gBattlerAttacker].furyCutterCounter = 0;
        gBattlescriptCurrInstr = BattleScript_MoveMissedPause.as_ptr().cast_mut();
    } else {
        let mut i: i32 = 0;
        if gDisableStructs[gBattlerAttacker].furyCutterCounter != 5 {
            gDisableStructs[gBattlerAttacker].furyCutterCounter += 1;
        }
        gDynamicBasePower = gBattleMoves[gCurrentMove].power as u16;
        i = 1;
        while i < gDisableStructs[gBattlerAttacker].furyCutterCounter as i32 {
            gDynamicBasePower *= 2;
            i += 1;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_friendshiptodamagecalculation() {
    if gBattleMoves[gCurrentMove].effect == EFFECT_RETURN {
        gDynamicBasePower = (10 * gBattleMons[gBattlerAttacker].friendship as i32 / 25) as u16;
    } else {
        gDynamicBasePower = (10
            * (MAX_FRIENDSHIP as i32 - gBattleMons[gBattlerAttacker].friendship as i32)
            / 25) as u16;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_presentdamagecalculation() {
    let mut rand: i32 = Random() as i32 & 0xFF;
    if rand < 102 {
        gDynamicBasePower = 40;
    } else if rand < 178 {
        gDynamicBasePower = 80;
    } else if rand < 204 {
        gDynamicBasePower = 120;
    } else {
        gBattleMoveDamage = gBattleMons[gBattlerTarget].maxHP as i32 / 4;
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattleMoveDamage *= -1;
    }
    if rand < 204 {
        gBattlescriptCurrInstr = BattleScript_HitFromCritCalc.as_ptr().cast_mut();
    } else if gBattleMons[gBattlerTarget].maxHP == gBattleMons[gBattlerTarget].hp {
        gBattlescriptCurrInstr = BattleScript_AlreadyAtFullHp.as_ptr().cast_mut();
    } else {
        gMoveResultFlags &= 247;
        gBattlescriptCurrInstr = BattleScript_PresentHealTarget.as_ptr().cast_mut();
    }
}
pub(crate) unsafe extern "C" fn Cmd_setsafeguard() {
    if gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & 1] as i32 & SIDE_STATUS_SAFEGUARD
        != 0
    {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_SIDE_STATUS_FAILED;
    } else {
        gSideStatuses[GetBattlerPosition(gBattlerAttacker) as i32 & 1] |=
            SIDE_STATUS_SAFEGUARD as u16;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].safeguardTimer = 5;
        gSideTimers[GetBattlerPosition(gBattlerAttacker) as i32 & 1].safeguardBattlerId =
            gBattlerAttacker;
        gBattleCommunication[5] = 5;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_magnitudedamagecalculation() {
    let mut magnitude: i32 = Random() as i32 % 100;
    if magnitude < 5 {
        gDynamicBasePower = 10;
        magnitude = 4;
    } else if magnitude < 15 {
        gDynamicBasePower = 30;
        magnitude = 5;
    } else if magnitude < 35 {
        gDynamicBasePower = 50;
        magnitude = 6;
    } else if magnitude < 65 {
        gDynamicBasePower = 70;
        magnitude = 7;
    } else if magnitude < 85 {
        gDynamicBasePower = 90;
        magnitude = 8;
    } else if magnitude < 95 {
        gDynamicBasePower = 110;
        magnitude = 9;
    } else {
        gDynamicBasePower = 150;
        magnitude = 10;
    }
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 1;
    gBattleTextBuff1[2] = 1;
    gBattleTextBuff1[3] = 2;
    gBattleTextBuff1[4] = magnitude as u8;
    gBattleTextBuff1[5] = 0xFF;
    gBattlerTarget = 0;
    'l2: while gBattlerTarget < gBattlersCount {
        'l1: {
            if gBattlerTarget == gBattlerAttacker {
                break 'l1;
            }
            if gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] == 0 {
                break 'l2;
            }
        }
        gBattlerTarget += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_jumpifnopursuitswitchdmg() {
    if gMultiHitCounter == 1 {
        if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
            gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        } else {
            gBattlerTarget = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        }
    } else {
        if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
            gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
        } else {
            gBattlerTarget = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
        }
    }
    if gChosenActionByBattler[gBattlerTarget] == B_ACTION_USE_MOVE
        && gBattlerAttacker == *(*gBattleStruct).moveTarget.as_mut_ptr().at(gBattlerTarget)
        && gBattleMons[gBattlerTarget].status1 & 39 == 0
        && gBattleMons[gBattlerAttacker].hp != 0
        && gDisableStructs[gBattlerTarget].truantCounter() == 0
        && gChosenMoveByBattler[gBattlerTarget] == MOVE_PURSUIT
    {
        let mut i: i32 = 0;
        i = 0;
        while i < gBattlersCount as i32 {
            if gBattlerByTurnOrder[i] == gBattlerTarget {
                gActionsByTurnOrder[i] = B_ACTION_TRY_FINISH;
            }
            i += 1;
        }
        gCurrentMove = MOVE_PURSUIT;
        gCurrMovePos = {
            gChosenMovePos = *(*gBattleStruct)
                .chosenMovePositions
                .as_mut_ptr()
                .at(gBattlerTarget);
            gChosenMovePos
        };
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        gBattleScripting.animTurn = 1;
        gHitMarker &= 0xfffffbff;
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_setsunny() {
    if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_WEATHER_FAILED;
    } else {
        gBattleWeather = B_WEATHER_SUN_TEMPORARY;
        gBattleCommunication[5] = B_MSG_STARTED_SUNLIGHT;
        gWishFutureKnock.weatherDuration = 5;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_maxattackhalvehp() {
    let mut halfHp: u32 = (gBattleMons[gBattlerAttacker].maxHP as i32 / 2) as u32;
    if gBattleMons[gBattlerAttacker].maxHP as i32 / 2 == 0 {
        halfHp = 1;
    }
    if gBattleMons[gBattlerAttacker].statStages[1] < MAX_STAT_STAGE
        && gBattleMons[gBattlerAttacker].hp as u32 > halfHp
    {
        gBattleMons[gBattlerAttacker].statStages[1] = MAX_STAT_STAGE;
        gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 2;
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_copyfoestats() {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_BATTLE_STATS {
        gBattleMons[gBattlerAttacker].statStages[i] = gBattleMons[gBattlerTarget].statStages[i];
        i += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
}
pub(crate) unsafe extern "C" fn Cmd_rapidspinfree() {
    if gBattleMons[gBattlerAttacker].status2 & STATUS2_WRAPPED != 0 {
        gBattleScripting.battler = gBattlerTarget;
        gBattleMons[gBattlerAttacker].status2 &= 0xffff1fff;
        gBattlerTarget = *(*gBattleStruct).wrappedBy.as_mut_ptr().at(gBattlerAttacker);
        gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN;
        gBattleTextBuff1[1] = B_BUFF_MOVE;
        gBattleTextBuff1[2] = *(*gBattleStruct)
            .wrappedMove
            .as_mut_ptr()
            .at(gBattlerAttacker as i32 * 2);
        gBattleTextBuff1[3] = *(*gBattleStruct)
            .wrappedMove
            .as_mut_ptr()
            .at(gBattlerAttacker as i32 * 2)
            .at(1);
        gBattleTextBuff1[4] = B_BUFF_EOS;
        BattleScriptPushCursor();
        gBattlescriptCurrInstr = BattleScript_WrapFree.as_ptr().cast_mut();
    } else if gStatuses3[gBattlerAttacker] & STATUS3_LEECHSEED != 0 {
        gStatuses3[gBattlerAttacker] &= 0xfffffffb;
        gStatuses3[gBattlerAttacker] &= 0xfffffffc;
        BattleScriptPushCursor();
        gBattlescriptCurrInstr = BattleScript_LeechSeedFree.as_ptr().cast_mut();
    } else if gSideStatuses[GetBattlerSide(gBattlerAttacker)] as i32 & SIDE_STATUS_SPIKES != 0 {
        gSideStatuses[GetBattlerSide(gBattlerAttacker)] &= 65519;
        gSideTimers[GetBattlerSide(gBattlerAttacker)].spikesAmount = 0;
        BattleScriptPushCursor();
        gBattlescriptCurrInstr = BattleScript_SpikesFree.as_ptr().cast_mut();
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setdefensecurlbit() {
    gBattleMons[gBattlerAttacker].status2 |= STATUS2_DEFENSE_CURL;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_recoverbasedonsunlight() {
    gBattlerTarget = gBattlerAttacker;
    if gBattleMons[gBattlerAttacker].hp != gBattleMons[gBattlerAttacker].maxHP {
        if gBattleWeather == 0
            || !(AbilityBattleEffects(19, 0, 13, 0, 0) == 0
                && AbilityBattleEffects(19, 0, 77, 0, 0) == 0)
        {
            gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 2;
        } else if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
            gBattleMoveDamage = 20 * gBattleMons[gBattlerAttacker].maxHP as i32 / 30;
        } else {
            gBattleMoveDamage = gBattleMons[gBattlerAttacker].maxHP as i32 / 4;
        }
        if gBattleMoveDamage == 0 {
            gBattleMoveDamage = 1;
        }
        gBattleMoveDamage *= -1;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_hiddenpowercalc() {
    let mut powerBits: u8 = ((gBattleMons[gBattlerAttacker].hpIV() & 2) >> 1) as u8
        | (gBattleMons[gBattlerAttacker].attackIV() as u8 & 2) << 0
        | (gBattleMons[gBattlerAttacker].defenseIV() as u8 & 2) << 1
        | (gBattleMons[gBattlerAttacker].speedIV() as u8 & 2) << 2
        | (gBattleMons[gBattlerAttacker].spAttackIV() as u8 & 2) << 3
        | (gBattleMons[gBattlerAttacker].spDefenseIV() as u8 & 2) << 4;
    let mut typeBits: u8 = (gBattleMons[gBattlerAttacker].hpIV() as u8 & 1) << 0
        | (gBattleMons[gBattlerAttacker].attackIV() as u8 & 1) << 1
        | (gBattleMons[gBattlerAttacker].defenseIV() as u8 & 1) << 2
        | (gBattleMons[gBattlerAttacker].speedIV() as u8 & 1) << 3
        | (gBattleMons[gBattlerAttacker].spAttackIV() as u8 & 1) << 4
        | (gBattleMons[gBattlerAttacker].spDefenseIV() as u8 & 1) << 5;
    gDynamicBasePower = (40 * powerBits as i32 / 63) as u16 + 30;
    (*gBattleStruct).dynamicMoveType = (15 * typeBits as i32 / 63) as u8 + 1;
    if (*gBattleStruct).dynamicMoveType >= TYPE_MYSTERY {
        (*gBattleStruct).dynamicMoveType += 1;
    }
    (*gBattleStruct).dynamicMoveType |= 192;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_selectfirstvalidtarget() {
    gBattlerTarget = 0;
    'l2: while gBattlerTarget < gBattlersCount {
        'l1: {
            if gBattlerTarget == gBattlerAttacker {
                break 'l1;
            }
            if gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] == 0 {
                break 'l2;
            }
        }
        gBattlerTarget += 1;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_trysetfutureattack() {
    if gWishFutureKnock.futureSightCounter[gBattlerTarget] != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gSideStatuses[GetBattlerPosition(gBattlerTarget) as i32 & 1] |= SIDE_STATUS_FUTUREATTACK;
        gWishFutureKnock.futureSightMove[gBattlerTarget] = gCurrentMove;
        gWishFutureKnock.futureSightAttacker[gBattlerTarget] = gBattlerAttacker;
        gWishFutureKnock.futureSightCounter[gBattlerTarget] = 3;
        gWishFutureKnock.futureSightDmg[gBattlerTarget] = CalculateBaseDamage(
            &raw mut gBattleMons[gBattlerAttacker],
            &raw mut gBattleMons[gBattlerTarget],
            gCurrentMove as u32,
            gSideStatuses[GetBattlerPosition(gBattlerTarget) as i32 & 1],
            0,
            0,
            gBattlerAttacker,
            gBattlerTarget,
        );
        if gProtectStructs[gBattlerAttacker].helpingHand() != 0 {
            gWishFutureKnock.futureSightDmg[gBattlerTarget] =
                gWishFutureKnock.futureSightDmg[gBattlerTarget] * 15 / 10;
        }
        if gCurrentMove == MOVE_DOOM_DESIRE {
            gBattleCommunication[5] = B_MSG_DOOM_DESIRE;
        } else {
            gBattleCommunication[5] = B_MSG_FUTURE_SIGHT;
        }
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_trydobeatup() {
    let mut party: *mut Pokemon = null_mut();
    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
        party = gPlayerParty.as_mut_ptr();
    } else {
        party = gEnemyParty.as_mut_ptr();
    }
    if gBattleMons[gBattlerTarget].hp == 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        let mut beforeLoop: u8 = gBattleCommunication[0];
        while gBattleCommunication[0] < PARTY_SIZE as u8 {
            if GetMonData2(party.at(gBattleCommunication[0]), MON_DATA_HP) != 0
                && GetMonData2(party.at(gBattleCommunication[0]), MON_DATA_SPECIES_OR_EGG) != 0
                && GetMonData2(party.at(gBattleCommunication[0]), MON_DATA_SPECIES_OR_EGG)
                    != SPECIES_EGG
                && GetMonData2(party.at(gBattleCommunication[0]), MON_DATA_STATUS) == 0
            {
                break;
            }
            gBattleCommunication[0] += 1;
        }
        if gBattleCommunication[0] < PARTY_SIZE as u8 {
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 4;
            gBattleTextBuff1[2] = gBattlerAttacker;
            gBattleTextBuff1[3] = gBattleCommunication[0];
            gBattleTextBuff1[4] = 0xFF;
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(9);
            gBattleMoveDamage = gSpeciesInfo
                [GetMonData2(party.at(gBattleCommunication[0]), MON_DATA_SPECIES)]
            .baseAttack as i32;
            gBattleMoveDamage *= gBattleMoves[gCurrentMove].power as i32;
            gBattleMoveDamage *=
                (GetMonData2(party.at(gBattleCommunication[0]), MON_DATA_LEVEL) * 2 / 5) as i32 + 2;
            gBattleMoveDamage = div_i32(
                gBattleMoveDamage,
                gSpeciesInfo[gBattleMons[gBattlerTarget].species].baseDefense as i32,
            );
            gBattleMoveDamage = gBattleMoveDamage / 50 + 2;
            if gProtectStructs[gBattlerAttacker].helpingHand() != 0 {
                gBattleMoveDamage = gBattleMoveDamage * 15 / 10;
            }
            gBattleCommunication[0] += 1;
        } else if beforeLoop != 0 {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        } else {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(5) as i32
                | (*gBattlescriptCurrInstr.at(5).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(5).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(5).at(3) as i32) << 24)
                as usize as *mut u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_setsemiinvulnerablebit() {
    match gCurrentMove {
        MOVE_FLY | MOVE_BOUNCE => {
            gStatuses3[gBattlerAttacker] |= STATUS3_ON_AIR;
        }
        MOVE_DIG => {
            gStatuses3[gBattlerAttacker] |= STATUS3_UNDERGROUND;
        }
        MOVE_DIVE => {
            gStatuses3[gBattlerAttacker] |= STATUS3_UNDERWATER;
        }
        _ => {}
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_clearsemiinvulnerablebit() {
    match gCurrentMove {
        MOVE_FLY | MOVE_BOUNCE => {
            gStatuses3[gBattlerAttacker] &= 0xffffffbf;
        }
        MOVE_DIG => {
            gStatuses3[gBattlerAttacker] &= 0xffffff7f;
        }
        MOVE_DIVE => {
            gStatuses3[gBattlerAttacker] &= 0xfffbffff;
        }
        _ => {}
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setminimize() {
    if gHitMarker & HITMARKER_OBEYS != 0 {
        gStatuses3[gBattlerAttacker] |= STATUS3_MINIMIZED;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_sethail() {
    if gBattleWeather as i32 & B_WEATHER_HAIL != 0 {
        gMoveResultFlags |= MOVE_RESULT_MISSED;
        gBattleCommunication[5] = B_MSG_WEATHER_FAILED;
    } else {
        gBattleWeather = B_WEATHER_HAIL_TEMPORARY;
        gBattleCommunication[5] = 5;
        gWishFutureKnock.weatherDuration = 5;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_trymemento() {
    if gBattleMons[gBattlerTarget].statStages[1] == MIN_STAT_STAGE
        && gBattleMons[gBattlerTarget].statStages[4] == MIN_STAT_STAGE
        && gBattleCommunication[6] != B_MSG_PROTECTED
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gActiveBattler = gBattlerAttacker;
        gBattleMoveDamage = gBattleMons[gActiveBattler].hp as i32;
        BtlController_EmitHealthBarUpdate(B_COMM_TO_CONTROLLER, INSTANT_HP_BAR_DROP as u16);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setforcedtarget() {
    gSideTimers[GetBattlerSide(gBattlerAttacker)].followmeTimer = 1;
    gSideTimers[GetBattlerSide(gBattlerAttacker)].followmeTarget = gBattlerAttacker;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setcharge() {
    gStatuses3[gBattlerAttacker] |= STATUS3_CHARGED_UP;
    gDisableStructs[gBattlerAttacker].set_chargeTimer(2);
    gDisableStructs[gBattlerAttacker].set_chargeTimerStartValue(2);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_callenvironmentattack() {
    gHitMarker &= 0xfffffbff;
    gCurrentMove = sNaturePowerMoves[gBattleEnvironment];
    gBattlerTarget = GetMoveTarget(gCurrentMove, NO_TARGET_OVERRIDE);
    BattleScriptPush(gBattleScriptsForMoveEffects[gBattleMoves[gCurrentMove].effect]);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_cureifburnedparalyzedorpoisoned() {
    if gBattleMons[gBattlerAttacker].status1 & 216 != 0 {
        gBattleMons[gBattlerAttacker].status1 = 0;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        gActiveBattler = gBattlerAttacker;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_STATUS_BATTLE,
            0,
            4,
            &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_settorment() {
    if gBattleMons[gBattlerTarget].status2 & 0x80000000 != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattleMons[gBattlerTarget].status2 |= 0x80000000;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifnodamage() {
    if gProtectStructs[gBattlerAttacker].physicalDmg != 0
        || gProtectStructs[gBattlerAttacker].specialDmg != 0
    {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_settaunt() {
    if gDisableStructs[gBattlerTarget].tauntTimer() == 0 {
        gDisableStructs[gBattlerTarget].set_tauntTimer(2);
        gDisableStructs[gBattlerTarget].set_tauntTimer2(2);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysethelpinghand() {
    gBattlerTarget = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 2);
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] == 0
        && gProtectStructs[gBattlerAttacker].helpingHand() == 0
        && gProtectStructs[gBattlerTarget].helpingHand() == 0
    {
        gProtectStructs[gBattlerTarget].set_helpingHand(1);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_tryswapitems() {
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0
        || GetBattlerSide(gBattlerAttacker) == B_SIDE_OPPONENT && gBattleTypeFlags & 0xa3f0902 == 0
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        let mut sideAttacker: u8 = GetBattlerSide(gBattlerAttacker);
        let mut sideTarget: u8 = GetBattlerSide(gBattlerTarget);
        if gBattleTypeFlags & 0xa3f0902 == 0
            && (gWishFutureKnock.knockedOffMons[sideAttacker] as u32
                & gBitTable[gBattlerPartyIndexes[gBattlerAttacker]]
                != 0
                || gWishFutureKnock.knockedOffMons[sideTarget] as u32
                    & gBitTable[gBattlerPartyIndexes[gBattlerTarget]]
                    != 0)
        {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        } else if gBattleMons[gBattlerAttacker].item == ITEM_NONE
            && gBattleMons[gBattlerTarget].item == ITEM_NONE
            || gBattleMons[gBattlerAttacker].item == ITEM_ENIGMA_BERRY
            || gBattleMons[gBattlerTarget].item == ITEM_ENIGMA_BERRY
            || (gBattleMons[gBattlerAttacker].item == ITEM_ORANGE_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_HARBOR_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_GLITTER_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_MECH_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_WOOD_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_WAVE_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_BEAD_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_SHADOW_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_TROPIC_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_DREAM_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_FAB_MAIL
                || gBattleMons[gBattlerAttacker].item == ITEM_RETRO_MAIL)
            || (gBattleMons[gBattlerTarget].item == ITEM_ORANGE_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_HARBOR_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_GLITTER_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_MECH_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_WOOD_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_WAVE_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_BEAD_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_SHADOW_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_TROPIC_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_DREAM_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_FAB_MAIL
                || gBattleMons[gBattlerTarget].item == ITEM_RETRO_MAIL)
        {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        } else if gBattleMons[gBattlerTarget].ability == ABILITY_STICKY_HOLD {
            gBattlescriptCurrInstr = BattleScript_StickyHoldActivates.as_ptr().cast_mut();
            gLastUsedAbility = gBattleMons[gBattlerTarget].ability;
            RecordAbilityBattle(gBattlerTarget, gLastUsedAbility);
        } else {
            let mut oldItemAtk: u16 = 0;
            let mut newItemAtk: *mut u16 = null_mut();
            newItemAtk = &raw mut (*gBattleStruct).changedItems[gBattlerAttacker];
            oldItemAtk = gBattleMons[gBattlerAttacker].item;
            *newItemAtk = gBattleMons[gBattlerTarget].item;
            gBattleMons[gBattlerAttacker].item = ITEM_NONE;
            gBattleMons[gBattlerTarget].item = oldItemAtk;
            gActiveBattler = gBattlerAttacker;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_HELDITEM_BATTLE,
                0,
                2,
                newItemAtk as *mut c_void,
            );
            MarkBattlerForControllerExec(gBattlerAttacker);
            gActiveBattler = gBattlerTarget;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_HELDITEM_BATTLE,
                0,
                2,
                &raw mut gBattleMons[gBattlerTarget].item as *mut c_void,
            );
            MarkBattlerForControllerExec(gBattlerTarget);
            *(&raw mut (*gBattleStruct).choicedMove[gBattlerTarget] as *mut u8) = 0;
            *(&raw mut (*gBattleStruct).choicedMove[gBattlerTarget] as *mut u8).at(1) = 0;
            *(&raw mut (*gBattleStruct).choicedMove[gBattlerAttacker] as *mut u8) = 0;
            *(&raw mut (*gBattleStruct).choicedMove[gBattlerAttacker] as *mut u8).at(1) = 0;
            gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 10;
            gBattleTextBuff1[2] = *newItemAtk as u8;
            gBattleTextBuff1[3] = ((*newItemAtk as i32 & 0xFF00) >> 8) as u8;
            gBattleTextBuff1[4] = 0xFF;
            gBattleTextBuff2[0] = 0xFD;
            gBattleTextBuff2[1] = 10;
            gBattleTextBuff2[2] = oldItemAtk as u8;
            gBattleTextBuff2[3] = ((oldItemAtk as i32 & 0xFF00) >> 8) as u8;
            gBattleTextBuff2[4] = 0xFF;
            if oldItemAtk != ITEM_NONE && *newItemAtk != ITEM_NONE {
                gBattleCommunication[5] = B_MSG_ITEM_SWAP_BOTH;
            } else if oldItemAtk == ITEM_NONE && *newItemAtk != ITEM_NONE {
                gBattleCommunication[5] = B_MSG_ITEM_SWAP_TAKEN;
            } else {
                gBattleCommunication[5] = B_MSG_ITEM_SWAP_GIVEN;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_trycopyability() {
    if gBattleMons[gBattlerTarget].ability != ABILITY_NONE
        && gBattleMons[gBattlerTarget].ability != ABILITY_WONDER_GUARD
    {
        gBattleMons[gBattlerAttacker].ability = gBattleMons[gBattlerTarget].ability;
        gLastUsedAbility = gBattleMons[gBattlerTarget].ability;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_trywish() {
    match *gBattlescriptCurrInstr.at(1) {
        0 => {
            if gWishFutureKnock.wishCounter[gBattlerAttacker] == 0 {
                gWishFutureKnock.wishCounter[gBattlerAttacker] = 2;
                gWishFutureKnock.wishMonId[gBattlerAttacker] =
                    gBattlerPartyIndexes[gBattlerAttacker] as u8;
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
            } else {
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
                    | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
                    | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
                    | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        1 => {
            gBattleTextBuff1[0] = 0xFD;
            gBattleTextBuff1[1] = 4;
            gBattleTextBuff1[2] = gBattlerTarget;
            gBattleTextBuff1[3] = gWishFutureKnock.wishMonId[gBattlerTarget];
            gBattleTextBuff1[4] = 0xFF;
            gBattleMoveDamage = gBattleMons[gBattlerTarget].maxHP as i32 / 2;
            if gBattleMoveDamage == 0 {
                gBattleMoveDamage = 1;
            }
            gBattleMoveDamage *= -1;
            if gBattleMons[gBattlerTarget].hp == gBattleMons[gBattlerTarget].maxHP {
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
                    | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
                    | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
                    | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
                    as usize as *mut u8;
            } else {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetroots() {
    if gStatuses3[gBattlerAttacker] & STATUS3_ROOTED != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gStatuses3[gBattlerAttacker] |= STATUS3_ROOTED;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_doubledamagedealtifdamaged() {
    if gProtectStructs[gBattlerAttacker].physicalDmg != 0
        && gProtectStructs[gBattlerAttacker].physicalBattlerId == gBattlerTarget
        || gProtectStructs[gBattlerAttacker].specialDmg != 0
            && gProtectStructs[gBattlerAttacker].specialBattlerId == gBattlerTarget
    {
        gBattleScripting.dmgMultiplier = 2;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_setyawn() {
    if gStatuses3[gBattlerTarget] & STATUS3_YAWN != 0
        || gBattleMons[gBattlerTarget].status1 & STATUS1_ANY != 0
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gStatuses3[gBattlerTarget] |= 4096;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setdamagetohealthdifference() {
    if gBattleMons[gBattlerTarget].hp <= gBattleMons[gBattlerAttacker].hp {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattleMoveDamage =
            gBattleMons[gBattlerTarget].hp as i32 - gBattleMons[gBattlerAttacker].hp as i32;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_scaledamagebyhealthratio() {
    if gDynamicBasePower == 0 {
        let mut power: u8 = gBattleMoves[gCurrentMove].power;
        gDynamicBasePower = div_i32(
            gBattleMons[gBattlerAttacker].hp as i32 * power as i32,
            gBattleMons[gBattlerAttacker].maxHP as i32,
        ) as u16;
        if gDynamicBasePower == 0 {
            gDynamicBasePower = 1;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_tryswapabilities() {
    if gBattleMons[gBattlerAttacker].ability == ABILITY_NONE
        && gBattleMons[gBattlerTarget].ability == ABILITY_NONE
        || gBattleMons[gBattlerAttacker].ability == ABILITY_WONDER_GUARD
        || gBattleMons[gBattlerTarget].ability == ABILITY_WONDER_GUARD
        || gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0
    {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        let mut abilityAtk: u8 = gBattleMons[gBattlerAttacker].ability;
        gBattleMons[gBattlerAttacker].ability = gBattleMons[gBattlerTarget].ability;
        gBattleMons[gBattlerTarget].ability = abilityAtk;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_tryimprison() {
    if gStatuses3[gBattlerAttacker] & STATUS3_IMPRISONED_OTHERS != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        let mut battler: u8 = 0;
        let mut sideAttacker: u8 = 0;
        sideAttacker = GetBattlerSide(gBattlerAttacker);
        PressurePPLoseOnUsingImprison(gBattlerAttacker);
        battler = 0;
        while battler < gBattlersCount {
            if sideAttacker != GetBattlerSide(battler) {
                let mut attackerMoveId: i32 = 0;
                attackerMoveId = 0;
                while attackerMoveId < MAX_MON_MOVES {
                    let mut i: i32 = 0;
                    i = 0;
                    while i < MAX_MON_MOVES {
                        if gBattleMons[gBattlerAttacker].moves[attackerMoveId]
                            == gBattleMons[battler].moves[i]
                            && gBattleMons[gBattlerAttacker].moves[attackerMoveId] != MOVE_NONE
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
                    gStatuses3[gBattlerAttacker] |= STATUS3_IMPRISONED_OTHERS;
                    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
                    break;
                }
            }
            battler += 1;
        }
        if battler == gBattlersCount {
            gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                as usize as *mut u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetgrudge() {
    if gStatuses3[gBattlerAttacker] & STATUS3_GRUDGE != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gStatuses3[gBattlerAttacker] |= STATUS3_GRUDGE;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_weightdamagecalculation() {
    let mut i: i32 = 0;
    i = 0;
    while sWeightToDamageTable[i] != 0xFFFF {
        if sWeightToDamageTable[i]
            > GetPokedexHeightWeight(
                SpeciesToNationalPokedexNum(gBattleMons[gBattlerTarget].species),
                1,
            )
        {
            break;
        }
        i += 2;
    }
    if sWeightToDamageTable[i] != 0xFFFF {
        gDynamicBasePower = sWeightToDamageTable[i + 1];
    } else {
        gDynamicBasePower = 120;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_assistattackselect() {
    let mut chooseableMovesNo: i32 = 0;
    let mut party: *mut Pokemon = null_mut();
    let mut monId: i32 = 0;
    let mut moveIndex: i32 = 0;
    let mut validMoves: *mut u16 = (*gBattleStruct).assistPossibleMoves.as_mut_ptr();
    if GetBattlerPosition(gBattlerAttacker) as i32 & 1 != B_SIDE_PLAYER as i32 {
        party = gEnemyParty.as_mut_ptr();
    } else {
        party = gPlayerParty.as_mut_ptr();
    }
    monId = 0;
    while monId < PARTY_SIZE {
        'l1: {
            if monId == gBattlerPartyIndexes[gBattlerAttacker] as i32 {
                break 'l1;
            }
            if GetMonData2(party.at(monId), MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32 {
                break 'l1;
            }
            if GetMonData2(party.at(monId), MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG {
                break 'l1;
            }
            moveIndex = 0;
            while moveIndex < MAX_MON_MOVES {
                'l3: {
                    let mut i: i32 = 0;
                    let mut r#move: u16 =
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
                moveIndex += 1;
            }
        }
        monId += 1;
    }
    if chooseableMovesNo != 0 {
        gHitMarker &= 0xfffffbff;
        gCalledMove = *validMoves.at((Random() as i32 & 0xFF) * chooseableMovesNo >> 8);
        gBattlerTarget = GetMoveTarget(gCalledMove, NO_TARGET_OVERRIDE);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetmagiccoat() {
    gBattlerTarget = gBattlerAttacker;
    gSpecialStatuses[gBattlerAttacker].set_ppNotAffectedByPressure(1);
    if gCurrentTurnActionNumber as i32 == gBattlersCount as i32 - 1 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gProtectStructs[gBattlerAttacker].set_bounceMove(TRUE as u32);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_trysetsnatch() {
    gSpecialStatuses[gBattlerAttacker].set_ppNotAffectedByPressure(1);
    if gCurrentTurnActionNumber as i32 == gBattlersCount as i32 - 1 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gProtectStructs[gBattlerAttacker].set_stealMove(1);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_trygetintimidatetarget() {
    let mut side: u8 = 0;
    gBattleScripting.battler = (*gBattleStruct).intimidateBattler;
    side = GetBattlerSide(gBattleScripting.battler);
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 9;
    gBattleTextBuff1[2] = gBattleMons[gBattleScripting.battler].ability;
    gBattleTextBuff1[3] = 0xFF;
    'l2: while gBattlerTarget < gBattlersCount {
        'l1: {
            if GetBattlerSide(gBattlerTarget) == side {
                break 'l1;
            }
            if gAbsentBattlerFlags as u32 & gBitTable[gBattlerTarget] == 0 {
                break 'l2;
            }
        }
        gBattlerTarget += 1;
    }
    if gBattlerTarget >= gBattlersCount {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_switchoutabilities() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    match gBattleMons[gActiveBattler].ability {
        ABILITY_NATURAL_CURE => {
            gBattleMons[gActiveBattler].status1 = 0;
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_STATUS_BATTLE,
                gBitTable[*(*gBattleStruct)
                    .battlerPartyIndexes
                    .as_mut_ptr()
                    .at(gActiveBattler)] as u8,
                4,
                &raw mut gBattleMons[gActiveBattler].status1 as *mut c_void,
            );
            MarkBattlerForControllerExec(gActiveBattler);
        }
        _ => {}
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_jumpifhasnohp() {
    gActiveBattler = GetBattlerForBattleScript(*gBattlescriptCurrInstr.at(1));
    if gBattleMons[gActiveBattler].hp == 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(2) as i32
            | (*gBattlescriptCurrInstr.at(2).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(2).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(2).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(6);
    }
}
pub(crate) unsafe extern "C" fn Cmd_getsecretpowereffect() {
    match gBattleEnvironment {
        BATTLE_ENVIRONMENT_GRASS => {
            gBattleCommunication[3] = MOVE_EFFECT_POISON;
        }
        BATTLE_ENVIRONMENT_LONG_GRASS => {
            gBattleCommunication[3] = MOVE_EFFECT_SLEEP;
        }
        BATTLE_ENVIRONMENT_SAND => {
            gBattleCommunication[3] = MOVE_EFFECT_ACC_MINUS_1;
        }
        BATTLE_ENVIRONMENT_UNDERWATER => {
            gBattleCommunication[3] = MOVE_EFFECT_DEF_MINUS_1;
        }
        BATTLE_ENVIRONMENT_WATER => {
            gBattleCommunication[3] = MOVE_EFFECT_ATK_MINUS_1;
        }
        BATTLE_ENVIRONMENT_POND => {
            gBattleCommunication[3] = MOVE_EFFECT_SPD_MINUS_1;
        }
        BATTLE_ENVIRONMENT_MOUNTAIN => {
            gBattleCommunication[3] = MOVE_EFFECT_CONFUSION;
        }
        BATTLE_ENVIRONMENT_CAVE => {
            gBattleCommunication[3] = MOVE_EFFECT_FLINCH;
        }
        _ => {
            gBattleCommunication[3] = MOVE_EFFECT_PARALYSIS;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_pickup() {
    let mut i: i32 = 0;
    let mut species: u16 = 0;
    let mut heldItem: u16 = 0;
    let mut ability: u8 = 0;
    if InBattlePike() != 0 {
    } else if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        i = 0;
        while i < PARTY_SIZE {
            species = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16;
            heldItem = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) as u16;
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_ABILITY_NUM) != 0 {
                ability = gSpeciesInfo[species].abilities[1];
            } else {
                ability = gSpeciesInfo[species].abilities[0];
            }
            if ability == ABILITY_PICKUP
                && species != SPECIES_NONE
                && species != SPECIES_EGG as u16
                && heldItem == ITEM_NONE
                && Random() as i32 % 10 == 0
            {
                heldItem = GetBattlePyramidPickupItemId();
                SetMonData(
                    &raw mut gPlayerParty[i],
                    MON_DATA_HELD_ITEM,
                    &raw mut heldItem as *mut c_void,
                );
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < PARTY_SIZE {
            species = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16;
            heldItem = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) as u16;
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_ABILITY_NUM) != 0 {
                ability = gSpeciesInfo[species].abilities[1];
            } else {
                ability = gSpeciesInfo[species].abilities[0];
            }
            if ability == ABILITY_PICKUP
                && species != SPECIES_NONE
                && species != SPECIES_EGG as u16
                && heldItem == ITEM_NONE
                && Random() as i32 % 10 == 0
            {
                let mut j: i32 = 0;
                let mut rand: i32 = Random() as i32 % 100;
                let mut lvlDivBy10: u8 =
                    ((GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) - 1) / 10) as u8;
                if lvlDivBy10 > 9 {
                    lvlDivBy10 = 9;
                }
                j = 0;
                while j < 9 {
                    if sPickupProbabilities[j] as i32 > rand {
                        SetMonData(
                            &raw mut gPlayerParty[i],
                            MON_DATA_HELD_ITEM,
                            (&raw const sPickupItems[lvlDivBy10 as i32 + j]).cast_mut()
                                as *mut c_void,
                        );
                        break;
                    } else if rand == 99 || rand == 98 {
                        SetMonData(
                            &raw mut gPlayerParty[i],
                            MON_DATA_HELD_ITEM,
                            (&raw const sRarePickupItems[lvlDivBy10 as i32 + (99 - rand)])
                                .cast_mut() as *mut c_void,
                        );
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_docastformchangeanimation() {
    gActiveBattler = gBattleScripting.battler;
    if gBattleMons[gActiveBattler].status2 & STATUS2_SUBSTITUTE != 0 {
        *(&raw mut (*gBattleStruct).formToChangeInto) |= CASTFORM_SUBSTITUTE as u8;
    }
    BtlController_EmitBattleAnimation(
        B_COMM_TO_CONTROLLER,
        B_ANIM_CASTFORM_CHANGE,
        (*gBattleStruct).formToChangeInto as u16,
    );
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_trycastformdatachange() {
    let mut form: u8 = 0;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
    form = CastformDataTypeChange(gBattleScripting.battler);
    if form != 0 {
        BattleScriptPushCursorAndCallback(BattleScript_CastformChange.as_ptr().cast_mut());
        *(&raw mut (*gBattleStruct).formToChangeInto) = form - 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_settypebasedhalvers() {
    let mut worked: u8 = FALSE;
    if gBattleMoves[gCurrentMove].effect == EFFECT_MUD_SPORT {
        if gStatuses3[gBattlerAttacker] & STATUS3_MUDSPORT == 0 {
            gStatuses3[gBattlerAttacker] |= STATUS3_MUDSPORT;
            gBattleCommunication[5] = B_MSG_WEAKEN_ELECTRIC;
            worked = TRUE;
        }
    } else {
        if gStatuses3[gBattlerAttacker] & STATUS3_WATERSPORT == 0 {
            gStatuses3[gBattlerAttacker] |= STATUS3_WATERSPORT;
            gBattleCommunication[5] = B_MSG_WEAKEN_FIRE;
            worked = TRUE;
        }
    }
    if worked != 0 {
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_setweatherballtype() {
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0 {
        if gBattleWeather as i32 & B_WEATHER_ANY != 0 {
            gBattleScripting.dmgMultiplier = 2;
        }
        if gBattleWeather as i32 & B_WEATHER_RAIN != 0 {
            *(&raw mut (*gBattleStruct).dynamicMoveType) = 139;
        } else if gBattleWeather as i32 & B_WEATHER_SANDSTORM != 0 {
            *(&raw mut (*gBattleStruct).dynamicMoveType) = 133;
        } else if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
            *(&raw mut (*gBattleStruct).dynamicMoveType) = 138;
        } else if gBattleWeather as i32 & B_WEATHER_HAIL != 0 {
            *(&raw mut (*gBattleStruct).dynamicMoveType) = 143;
        } else {
            *(&raw mut (*gBattleStruct).dynamicMoveType) = F_DYNAMIC_TYPE_SET;
        }
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_tryrecycleitem() {
    let mut usedHeldItem: *mut u16 = null_mut();
    gActiveBattler = gBattlerAttacker;
    usedHeldItem = &raw mut (*gBattleStruct).usedHeldItems[gActiveBattler];
    if *usedHeldItem != ITEM_NONE && gBattleMons[gActiveBattler].item == ITEM_NONE {
        gLastUsedItem = *usedHeldItem;
        *usedHeldItem = ITEM_NONE;
        gBattleMons[gActiveBattler].item = gLastUsedItem;
        BtlController_EmitSetMonData(
            B_COMM_TO_CONTROLLER,
            REQUEST_HELDITEM_BATTLE,
            0,
            2,
            &raw mut gBattleMons[gActiveBattler].item as *mut c_void,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_settypetoenvironment() {
    if !(gBattleMons[gBattlerAttacker].types[0] == sEnvironmentToType[gBattleEnvironment]
        || gBattleMons[gBattlerAttacker].types[1] == sEnvironmentToType[gBattleEnvironment])
    {
        gBattleMons[gBattlerAttacker].types[0] = sEnvironmentToType[gBattleEnvironment];
        gBattleMons[gBattlerAttacker].types[1] = sEnvironmentToType[gBattleEnvironment];
        gBattleTextBuff1[0] = 0xFD;
        gBattleTextBuff1[1] = 3;
        gBattleTextBuff1[2] = sEnvironmentToType[gBattleEnvironment];
        gBattleTextBuff1[3] = 0xFF;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_pursuitdoubles() {
    gActiveBattler = GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) ^ 2);
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && gAbsentBattlerFlags as u32 & gBitTable[gActiveBattler] == 0
        && gChosenActionByBattler[gActiveBattler] == B_ACTION_USE_MOVE
        && gChosenMoveByBattler[gActiveBattler] == MOVE_PURSUIT
    {
        gActionsByTurnOrder[gActiveBattler] = B_ACTION_TRY_FINISH;
        gCurrentMove = MOVE_PURSUIT;
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
        gBattleScripting.animTurn = 1;
        gBattleScripting.pursuitDoublesAttacker = gBattlerAttacker;
        gBattlerAttacker = gActiveBattler;
    } else {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    }
}
pub(crate) unsafe extern "C" fn Cmd_snatchsetbattlers() {
    gEffectBattler = gBattlerAttacker;
    if gBattlerAttacker == gBattlerTarget {
        gBattlerAttacker = {
            gBattlerTarget = gBattleScripting.battler;
            gBattlerTarget
        };
    } else {
        gBattlerTarget = gBattleScripting.battler;
    }
    gBattleScripting.battler = gEffectBattler;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_removelightscreenreflect() {
    let mut opposingSide: u8 = GetBattlerSide(gBattlerAttacker) ^ 1;
    if gSideTimers[opposingSide].reflectTimer != 0
        || gSideTimers[opposingSide].lightscreenTimer != 0
    {
        gSideStatuses[opposingSide] &= 65534;
        gSideStatuses[opposingSide] &= 65533;
        gSideTimers[opposingSide].reflectTimer = 0;
        gSideTimers[opposingSide].lightscreenTimer = 0;
        gBattleScripting.animTurn = 1;
        gBattleScripting.animTargetsHit = 1;
    } else {
        gBattleScripting.animTurn = 0;
        gBattleScripting.animTargetsHit = 0;
    }
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_handleballthrow() {
    let mut ballMultiplier: u8 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = gBattlerAttacker;
    gBattlerTarget = gBattlerAttacker ^ 1;
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, BALL_TRAINER_BLOCK);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = BattleScript_TrainerBallBlock.as_ptr().cast_mut();
    } else if gBattleTypeFlags & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
        BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, BALL_3_SHAKES_SUCCESS);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattlescriptCurrInstr = BattleScript_WallyBallThrow.as_ptr().cast_mut();
    } else {
        let mut odds: u32 = 0;
        let mut catchRate: u8 = 0;
        if gLastUsedItem == ITEM_SAFARI_BALL {
            catchRate = ((*gBattleStruct).safariCatchFactor as i32 * 1275 / 100) as u8;
        } else {
            catchRate = gSpeciesInfo[gBattleMons[gBattlerTarget].species].catchRate;
        }
        if gLastUsedItem > ITEM_SAFARI_BALL {
            match gLastUsedItem {
                ITEM_NET_BALL => {
                    if gBattleMons[gBattlerTarget].types[0] == TYPE_WATER
                        || gBattleMons[gBattlerTarget].types[1] == TYPE_WATER
                        || (gBattleMons[gBattlerTarget].types[0] == TYPE_BUG
                            || gBattleMons[gBattlerTarget].types[1] == TYPE_BUG)
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
                    if gBattleMons[gBattlerTarget].level < 40 {
                        ballMultiplier = 40 - gBattleMons[gBattlerTarget].level;
                        if ballMultiplier <= 9 {
                            ballMultiplier = 10;
                        }
                    } else {
                        ballMultiplier = 10;
                    }
                }
                ITEM_REPEAT_BALL => {
                    if GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(gBattleMons[gBattlerTarget].species),
                        FLAG_GET_CAUGHT,
                    ) != 0
                    {
                        ballMultiplier = 30;
                    } else {
                        ballMultiplier = 10;
                    }
                }
                ITEM_TIMER_BALL => {
                    ballMultiplier = gBattleResults.battleTurnCounter + 10;
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
            ballMultiplier = sBallCatchBonuses[gLastUsedItem as i32 - ITEM_ULTRA_BALL];
        }
        odds = div_i32(
            catchRate as i32 * ballMultiplier as i32 / 10
                * (gBattleMons[gBattlerTarget].maxHP as i32 * 3
                    - gBattleMons[gBattlerTarget].hp as i32 * 2),
            3 * gBattleMons[gBattlerTarget].maxHP as i32,
        ) as u32;
        if gBattleMons[gBattlerTarget].status1 & 39 != 0 {
            odds *= 2;
        }
        if gBattleMons[gBattlerTarget].status1 & 216 != 0 {
            odds = odds * 15 / 10;
        }
        if gLastUsedItem != ITEM_SAFARI_BALL {
            if gLastUsedItem == ITEM_MASTER_BALL {
                gBattleResults.set_usedMasterBall(TRUE);
            } else {
                if gBattleResults.catchAttempts[gLastUsedItem as i32 - ITEM_ULTRA_BALL] < 255 {
                    gBattleResults.catchAttempts[gLastUsedItem as i32 - ITEM_ULTRA_BALL] += 1;
                }
            }
        }
        if odds > 254 {
            BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, BALL_3_SHAKES_SUCCESS);
            MarkBattlerForControllerExec(gActiveBattler);
            gBattlescriptCurrInstr = BattleScript_SuccessBallThrow.as_ptr().cast_mut();
            SetMonData(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]],
                MON_DATA_POKEBALL,
                &raw mut gLastUsedItem as *mut c_void,
            );
            if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                gBattleCommunication[5] = 0;
            } else {
                gBattleCommunication[5] = 1;
            }
        } else {
            let mut shakes: u8 = 0;
            odds = Sqrt(Sqrt(div_u32(0xff0000, odds)) as u32) as u32;
            odds = div_u32(0xffff0, odds);
            shakes = 0;
            while shakes < BALL_3_SHAKES_SUCCESS && (Random() as u32) < odds {
                shakes += 1;
            }
            if gLastUsedItem == ITEM_MASTER_BALL {
                shakes = BALL_3_SHAKES_SUCCESS;
            }
            BtlController_EmitBallThrowAnim(B_COMM_TO_CONTROLLER, shakes);
            MarkBattlerForControllerExec(gActiveBattler);
            if shakes == BALL_3_SHAKES_SUCCESS {
                gBattlescriptCurrInstr = BattleScript_SuccessBallThrow.as_ptr().cast_mut();
                SetMonData(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]],
                    MON_DATA_POKEBALL,
                    &raw mut gLastUsedItem as *mut c_void,
                );
                if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                    gBattleCommunication[5] = 0;
                } else {
                    gBattleCommunication[5] = 1;
                }
            } else {
                gBattleCommunication[5] = shakes;
                gBattlescriptCurrInstr = BattleScript_ShakeBallThrow.as_ptr().cast_mut();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_givecaughtmon() {
    if GiveMonToPlayer(&raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]])
        != MON_GIVEN_TO_PARTY
    {
        if ShouldShowBoxWasFullMessage() == 0 {
            gBattleCommunication[5] = B_MSG_SENT_SOMEONES_PC;
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
            );
            GetMonData3(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                MON_DATA_NICKNAME,
                gStringVar2.as_mut_ptr(),
            );
        } else {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
            );
            GetMonData3(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                MON_DATA_NICKNAME,
                gStringVar2.as_mut_ptr(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                GetBoxNamePtr(GetPCBoxToSendMon() as u8),
            );
            gBattleCommunication[5] = B_MSG_SOMEONES_BOX_FULL;
        }
        if FlagGet(FLAG_SYS_PC_LANETTE) != 0 {
            gBattleCommunication[5] += 1;
        }
    }
    gBattleResults.caughtMonSpecies = GetMonData3(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    GetMonData3(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
        MON_DATA_NICKNAME,
        gBattleResults.caughtMonNick.as_mut_ptr(),
    );
    gBattleResults.set_caughtMonBall(GetMonData3(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
        MON_DATA_POKEBALL,
        null_mut(),
    ) as u8);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_trysetcaughtmondexflags() {
    let mut species: u16 =
        GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) as u16;
    let mut personality: u32 =
        GetMonData3(&raw mut gEnemyParty[0], MON_DATA_PERSONALITY, null_mut());
    if GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), FLAG_GET_CAUGHT) != 0 {
        gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
            | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
            | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
            | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
            as usize as *mut u8;
    } else {
        HandleSetPokedexFlag(
            SpeciesToNationalPokedexNum(species),
            FLAG_SET_CAUGHT,
            personality,
        );
        gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
    }
}
pub(crate) unsafe extern "C" fn Cmd_displaydexinfo() {
    let mut species: u16 =
        GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) as u16;
    match gBattleCommunication[0] {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gBattleCommunication[0] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                FreeAllWindowBuffers();
                gBattleCommunication[1] = DisplayCaughtMonDexPage(
                    SpeciesToNationalPokedexNum(species),
                    gBattleMons[gBattlerTarget].otId,
                    gBattleMons[gBattlerTarget].personality,
                );
                gBattleCommunication[0] += 1;
            }
        }
        2 => {
            if gPaletteFade.active() == 0
                && gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
                && gTasks[gBattleCommunication[1]].isActive == 0
            {
                SetVBlankCallback(Some(VBlankCB_Battle));
                gBattleCommunication[0] += 1;
            }
        }
        3 => {
            InitBattleBgsVideo();
            LoadBattleTextboxAndBackground();
            gBattle_BG3_X = 256;
            gBattleCommunication[0] += 1;
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                BeginNormalPaletteFade(PALETTES_BG, 0, 16, 0, 0);
                ShowBg(0);
                ShowBg(3);
                gBattleCommunication[0] += 1;
            }
        }
        5 => {
            if gPaletteFade.active() == 0 {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleBattleWindow(xStart: u8, yStart: u8, xEnd: u8, yEnd: u8, flags: u8) {
    let mut destY: i32 = 0;
    let mut destX: i32 = 0;
    let mut var: u16 = 0;
    destY = yStart as i32;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleCreateYesNoCursorAt(cursorPosition: u8) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleDestroyYesNoCursorAt(cursorPosition: u8) {
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
pub(crate) unsafe extern "C" fn Cmd_trygivecaughtmonnick() {
    match gBattleCommunication[0] {
        0 => {
            HandleBattleWindow(24, 8, 29, 13, 0);
            BattlePutTextOnWindow(gText_BattleYesNoChoice.as_ptr().cast_mut(), B_WIN_YESNO);
            gBattleCommunication[0] += 1;
            gBattleCommunication[1] = 0;
            BattleCreateYesNoCursorAt(0);
        }
        1 => {
            if gMain.newKeys as i32 & DPAD_UP != 0 && gBattleCommunication[1] != 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 0;
                BattleCreateYesNoCursorAt(0);
            }
            if gMain.newKeys as i32 & DPAD_DOWN != 0 && gBattleCommunication[1] == 0 {
                PlaySE(SE_SELECT);
                BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                gBattleCommunication[1] = 1;
                BattleCreateYesNoCursorAt(1);
            }
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if gBattleCommunication[1] == 0 {
                    gBattleCommunication[0] += 1;
                    BeginFastPaletteFade(3);
                } else {
                    gBattleCommunication[0] = 4;
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                gBattleCommunication[0] = 4;
            }
        }
        2 => {
            if gPaletteFade.active() == 0 {
                GetMonData3(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                    MON_DATA_NICKNAME,
                    (*gBattleStruct).caughtMonNick.as_mut_ptr(),
                );
                FreeAllWindowBuffers();
                DoNamingScreen(
                    NAMING_SCREEN_CAUGHT_MON,
                    (*gBattleStruct).caughtMonNick.as_mut_ptr(),
                    GetMonData2(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                        MON_DATA_SPECIES,
                    ) as u16,
                    GetMonGender(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                    ) as u16,
                    GetMonData3(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                        MON_DATA_PERSONALITY,
                        null_mut(),
                    ),
                    Some(BattleMainCB2),
                );
                gBattleCommunication[0] += 1;
            }
        }
        3 => {
            if gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
                && gPaletteFade.active() == 0
            {
                SetMonData(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker as i32 ^ 1]],
                    MON_DATA_NICKNAME,
                    (*gBattleStruct).caughtMonNick.as_mut_ptr() as *mut c_void,
                );
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                    | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                    | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                    | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        4 => {
            if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(5);
            } else {
                gBattlescriptCurrInstr = (*gBattlescriptCurrInstr.at(1) as i32
                    | (*gBattlescriptCurrInstr.at(1).at(1) as i32) << 8
                    | (*gBattlescriptCurrInstr.at(1).at(2) as i32) << 16
                    | (*gBattlescriptCurrInstr.at(1).at(3) as i32) << 24)
                    as usize as *mut u8;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_subattackerhpbydmg() {
    gBattleMons[gBattlerAttacker].hp -= gBattleMoveDamage as u16;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_removeattackerstatus1() {
    gBattleMons[gBattlerAttacker].status1 = 0;
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_finishaction() {
    gCurrentActionFuncId = B_ACTION_FINISHED;
}
pub(crate) unsafe extern "C" fn Cmd_finishturn() {
    gCurrentActionFuncId = B_ACTION_FINISHED;
    gCurrentTurnActionNumber = gBattlersCount;
}
pub(crate) unsafe extern "C" fn Cmd_trainerslideout() {
    gActiveBattler = GetBattlerAtPosition(*gBattlescriptCurrInstr.at(1));
    BtlController_EmitTrainerSlideBack(B_COMM_TO_CONTROLLER);
    MarkBattlerForControllerExec(gActiveBattler);
    gBattlescriptCurrInstr = gBattlescriptCurrInstr.at(2);
}
