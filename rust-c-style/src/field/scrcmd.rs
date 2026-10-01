//! Translated from `src/scrcmd.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gNullScriptPtr sScriptConditionTable sScriptStringVars

static sScriptConditionTable: Table<CArray<CArray<u8, 3>, 6>> =
    Table((&raw const crate::data::scrcmd::sScriptConditionTable).cast());
static sScriptStringVars: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::scrcmd::sScriptStringVars).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRamScriptRetAddr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAddressOffset: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPauseCounter: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingNpcId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingNpcMapGroup: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingNpcMapNum: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFieldEffectScriptId: u16 = 0;
pub(crate) static mut sBrailleWindowId: u8 = 0;

unsafe extern "C" {
    static gDecorations: CArray<Decoration, 0>;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gLocalTime: Time;
    static mut gMain: Main;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPaletteDecompressionBuffer: CArray<u8, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedObjectEvent: u8;
    static mut gSpecialVar_0x8000: u16;
    static mut gSpecialVar_0x8001: u16;
    static mut gSpecialVar_0x8002: u16;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_ContestCategory: u16;
    static mut gSpecialVar_Result: u16;
    static gSpecials: CArray<Option<unsafe extern "C" fn() -> u16>, 0>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gStdScripts: CArray<*mut u8, 0>;
    static mut gStdScripts_End: CArray<*mut u8, 0>;
    static gStdStrings: CArray<*mut u8, 0>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTextFlags: TextFlags;
    fn ActivatePerStepCallback(a0: u8);
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddCoins(a0: u16) -> u8;
    fn AddMoney(a0: *mut u32, a1: u32);
    fn AddPCItem(a0: u16, a1: u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AnimateFlash(a0: u8);
    fn BattleSetup_ConfigureTrainerBattle(a0: *mut u8) -> *mut u8;
    fn BattleSetup_GetScriptAddrAfterBattle() -> *mut u8;
    fn BattleSetup_GetTrainerPostBattleScript() -> *mut u8;
    fn BattleSetup_StartScriptedWildBattle();
    fn BattleSetup_StartTrainerBattle();
    fn BufferContestName(a0: *mut u8, a1: u8);
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeAmountInMoneyBox(a0: i32);
    fn CheckBagHasItem(a0: u16, a1: u16) -> u8;
    fn CheckBagHasSpace(a0: u16, a1: u16) -> u8;
    fn CheckHasDecoration(a0: u8) -> u8;
    fn CheckPCHasItem(a0: u16, a1: u16) -> u8;
    fn ChooseContestMon();
    fn ClearRamScript();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearTrainerFlag(a0: u16);
    fn ContestLinkTransfer(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyItemNameHandlePlural(a0: u16, a1: *mut u8, a2: u32);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountDigits(a0: i32) -> u32;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateDecorationShop1Menu(a0: *mut u16);
    fn CreateDecorationShop2Menu(a0: *mut u16);
    fn CreatePokemartMenu(a0: *mut u16);
    fn CreateScriptedWildMon(a0: u16, a1: u8, a2: u16);
    fn CreateVirtualObject(a0: u8, a1: u8, a2: i16, a3: i16, a4: u8, a5: u8) -> u8;
    fn CreateWindowTemplate(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u16,
    ) -> WindowTemplate;
    fn DecorationAdd(a0: u8) -> u8;
    fn DecorationCheckSpace(a0: u8) -> u8;
    fn DecorationRemove(a0: u8) -> i8;
    fn DoCurrentWeather();
    fn DoDiveWarp();
    fn DoDoorWarp();
    fn DoFallWarp();
    fn DoMossdeepGymWarp();
    fn DoSpinEnterWarp();
    fn DoTeleportTileWarp();
    fn DoTimeBasedEvents();
    fn DoWarp();
    fn DoWhiteFadeWarp();
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawMoneyBox(a0: i32, a1: u8, a2: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FadeInBGM(a0: u8);
    fn FadeOutBGMTemporarily(a0: u8);
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldAnimateDoorClose(a0: u32, a1: u32) -> i8;
    fn FieldAnimateDoorOpen(a0: u32, a1: u32) -> i8;
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldIsDoorAnimationRunning() -> u8;
    fn FieldSetDoorClosed(a0: u32, a1: u32);
    fn FieldSetDoorOpened(a0: u32, a1: u32);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FreeRotatingTilePuzzle();
    fn FreezeForApproachingTrainers();
    fn FreezeObjects_WaitForPlayer();
    fn FreezeObjects_WaitForPlayerAndSelected();
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetCoins() -> u16;
    fn GetCurrentApproachingTrainerObjectEventId() -> u8;
    fn GetDoorSoundEffect(a0: u32, a1: u32) -> u32;
    fn GetLeadMonIndex() -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMoney(a0: *mut u32) -> u32;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetPocketByItemId(a0: u16) -> u8;
    fn GetSavedRamScriptIfValid() -> *mut u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTrainerClassNameFromId(a0: u16) -> *mut u8;
    fn GetTrainerNameFromId(a0: u16) -> *mut u8;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn HasTrainerBeenFought(a0: u16) -> u8;
    fn HideCoinsWindow();
    fn HideFieldMessageBox();
    fn HideMoneyBox();
    fn IncrementGameStat(a0: u8);
    fn InitRotatingTilePuzzle(a0: u8);
    fn IsBGMPausedOrStopped() -> u8;
    fn IsCryFinished() -> u8;
    fn IsEnoughMoney(a0: *mut u32, a1: u32) -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsFieldMessageBoxHidden() -> u8;
    fn IsFreezeObjectAndPlayerFinished() -> u8;
    fn IsFreezePlayerFinished() -> u8;
    fn IsFreezeSelectedObjectAndPlayerFinished() -> u8;
    fn IsOverworldLinkActive() -> u32;
    fn IsPokeNewsActive(a0: u8) -> u8;
    fn IsSEPlaying() -> u8;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MonKnowsMove(a0: *mut Pokemon, a1: u16) -> u8;
    fn MoveRotatingTileObjects(a0: u8) -> u16;
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventFaceOppositeDirection(a0: *mut ObjectEvent, a1: u8) -> u8;
    fn ObjectEventTurnByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: u8);
    fn Overworld_ChangeMusicTo(a0: u16);
    fn Overworld_ChangeMusicToDefault();
    fn Overworld_SetSavedMusic(a0: u16);
    fn PlantBerryTree(a0: u8, a1: u8, a2: u8, a3: u8);
    fn PlayCry_Script(a0: u16, a1: u8);
    fn PlayFanfare(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PlaySlotMachine(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PrintCoinsString(a0: u32);
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveCoins(a0: u16) -> u8;
    fn RemoveMoney(a0: *mut u32, a1: u32);
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn RemoveWindow(a0: u8);
    fn ResetInitialPlayerAvatarState();
    fn ResetObjectSubpriority(a0: u8, a1: u8, a2: u8);
    fn RtcCalcLocalTime();
    fn RtcInitLocalTimeOffset(a0: i32, a1: i32);
    fn ScriptCall(a0: *mut ScriptContext, a1: *mut u8);
    fn ScriptContext_Stop();
    fn ScriptGiveEgg(a0: u16) -> u8;
    fn ScriptGiveMon(a0: u16, a1: u8, a2: u16, a3: u32, a4: u32, a5: u8) -> u8;
    fn ScriptJump(a0: *mut ScriptContext, a1: *mut u8);
    fn ScriptMenu_HidePokemonPic() -> Option<unsafe extern "C" fn() -> u8>;
    fn ScriptMenu_Multichoice(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn ScriptMenu_MultichoiceGrid(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn ScriptMenu_MultichoiceWithDefault(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn ScriptMenu_ShowPokemonPic(a0: u16, a1: u8, a2: u8) -> u8;
    fn ScriptMenu_YesNo(a0: u8, a1: u8) -> u8;
    fn ScriptMovement_IsObjectMovementFinished(a0: u8, a1: u8, a2: u8) -> u8;
    fn ScriptMovement_StartObjectMovementScript(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn ScriptMovement_UnfreezeObjectEvents();
    fn ScriptReadHalfword(a0: *mut ScriptContext) -> u16;
    fn ScriptReadWord(a0: *mut ScriptContext) -> u32;
    fn ScriptReturn(a0: *mut ScriptContext);
    fn ScriptSetMonMoveSlot(a0: u8, a1: u16, a2: u8);
    fn SetContestWinnerForPainting(a0: i32);
    fn SetCurrentMapLayout(a0: u16);
    fn SetDynamicWarpWithCoords(a0: i32, a1: i8, a2: i8, a3: i8, a4: i8, a5: i8);
    fn SetEscapeWarp(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetFixedDiveWarp(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetFixedHoleWarp(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetFlashLevel(a0: i32);
    fn SetLastHealLocationWarp(a0: u8);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMysteryEventScriptStatus(a0: u32);
    fn SetObjEventTemplateCoords(a0: u8, a1: i16, a2: i16);
    fn SetObjEventTemplateMovementType(a0: u8, a1: u8);
    fn SetObjectInvisibility(a0: u8, a1: u8, a2: u8, a3: u8);
    fn SetObjectSubpriority(a0: u8, a1: u8, a2: u8, a3: u8);
    fn SetSavedWeather(a0: u32);
    fn SetSavedWeatherFromCurrMapHeader();
    fn SetSpinStartFacingDir(a0: u8);
    fn SetTrainerFlag(a0: u16);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetWarpDestinationToFixedHoleWarp(a0: i16, a1: i16);
    fn SetupNativeScript(a0: *mut ScriptContext, a1: Option<unsafe extern "C" fn() -> u8>);
    fn ShowCoinsWindow(a0: u32, a1: u8, a2: u8);
    fn ShowContestPainting();
    fn ShowContestResults();
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn ShowPokenavFieldMessage(a0: *mut u8) -> u8;
    fn StartContest();
    fn StopScript(a0: *mut ScriptContext);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn TryMoveObjectEventToMapCoords(a0: u8, a1: u8, a2: u8, a3: i16, a4: i16);
    fn TryOverrideObjectEventTemplateCoords(a0: u8, a1: u8, a2: u8);
    fn TrySpawnObjectEvent(a0: u8, a1: u8, a2: u8) -> u8;
    fn TurnRotatingTileObjects();
    fn TurnVirtualObject(a0: u8, a1: u8);
    fn UnfreezeObjectEvents();
    fn VarGet(a0: u16) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_nop(ctx: *mut ScriptContext) -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_nop1(ctx: *mut ScriptContext) -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_end(ctx: *mut ScriptContext) -> u8 {
    StopScript(ctx);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotonative(ctx: *mut ScriptContext) -> u8 {
    let mut addr: Option<unsafe extern "C" fn() -> u8> = core::mem::transmute::<
        usize,
        Option<unsafe extern "C" fn() -> u8>,
    >(ScriptReadWord(ctx) as usize);
    SetupNativeScript(ctx, addr);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_special(ctx: *mut ScriptContext) -> u8 {
    let mut index: u16 = ScriptReadHalfword(ctx);
    gSpecials[index].unwrap_unchecked()();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_specialvar(ctx: *mut ScriptContext) -> u8 {
    let mut var: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *var = gSpecials[ScriptReadHalfword(ctx)].unwrap_unchecked()();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_callnative(ctx: *mut ScriptContext) -> u8 {
    let mut func: Option<unsafe extern "C" fn()> =
        core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(ScriptReadWord(ctx) as usize);
    func.unwrap_unchecked()();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitstate(ctx: *mut ScriptContext) -> u8 {
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_goto(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    ScriptJump(ctx, ptr);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_return(ctx: *mut ScriptContext) -> u8 {
    ScriptReturn(ctx);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_call(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    ScriptCall(ctx, ptr);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_goto_if(ctx: *mut ScriptContext) -> u8 {
    let mut condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let mut ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptJump(ctx, ptr);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_call_if(ctx: *mut ScriptContext) -> u8 {
    let mut condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let mut ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptCall(ctx, ptr);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setvaddress(ctx: *mut ScriptContext) -> u8 {
    let mut addr1: u32 = (*ctx).scriptPtr as usize as u32 - 1;
    let mut addr2: u32 = ScriptReadWord(ctx);
    sAddressOffset = addr2 - addr1;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vgoto(ctx: *mut ScriptContext) -> u8 {
    let mut addr: u32 = ScriptReadWord(ctx);
    ScriptJump(ctx, (addr - sAddressOffset) as usize as *mut u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vcall(ctx: *mut ScriptContext) -> u8 {
    let mut addr: u32 = ScriptReadWord(ctx);
    ScriptCall(ctx, (addr - sAddressOffset) as usize as *mut u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vgoto_if(ctx: *mut ScriptContext) -> u8 {
    let mut condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let mut ptr: *mut u8 = (ScriptReadWord(ctx) - sAddressOffset) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptJump(ctx, ptr);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vcall_if(ctx: *mut ScriptContext) -> u8 {
    let mut condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let mut ptr: *mut u8 = (ScriptReadWord(ctx) - sAddressOffset) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptCall(ctx, ptr);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotostd(ctx: *mut ScriptContext) -> u8 {
    let mut index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut ptr: *mut *mut u8 = &raw mut gStdScripts[index];
    if ptr < gStdScripts_End.as_mut_ptr() {
        ScriptJump(ctx, *ptr);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_callstd(ctx: *mut ScriptContext) -> u8 {
    let mut index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut ptr: *mut *mut u8 = &raw mut gStdScripts[index];
    if ptr < gStdScripts_End.as_mut_ptr() {
        ScriptCall(ctx, *ptr);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotostd_if(ctx: *mut ScriptContext) -> u8 {
    let mut condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let mut index: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        let mut ptr: *mut *mut u8 = &raw mut gStdScripts[index];
        if ptr < gStdScripts_End.as_mut_ptr() {
            ScriptJump(ctx, *ptr);
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_callstd_if(ctx: *mut ScriptContext) -> u8 {
    let mut condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let mut index: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        let mut ptr: *mut *mut u8 = &raw mut gStdScripts[index];
        if ptr < gStdScripts_End.as_mut_ptr() {
            ScriptCall(ctx, *ptr);
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_returnram(ctx: *mut ScriptContext) -> u8 {
    ScriptJump(ctx, gRamScriptRetAddr);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_endram(ctx: *mut ScriptContext) -> u8 {
    ClearRamScript();
    StopScript(ctx);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmysteryeventstatus(ctx: *mut ScriptContext) -> u8 {
    let mut status: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    SetMysteryEventScriptStatus(status as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_loadword(ctx: *mut ScriptContext) -> u8 {
    let mut index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).data[index] = ScriptReadWord(ctx);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_loadbytefromptr(ctx: *mut ScriptContext) -> u8 {
    let mut index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).data[index] = *(ScriptReadWord(ctx) as usize as *mut u8) as u32;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setptr(ctx: *mut ScriptContext) -> u8 {
    let mut value: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    *(ScriptReadWord(ctx) as usize as *mut u8) = value;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_loadbyte(ctx: *mut ScriptContext) -> u8 {
    let mut index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).data[index] = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    }) as u32;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setptrbyte(ctx: *mut ScriptContext) -> u8 {
    let mut index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    *(ScriptReadWord(ctx) as usize as *mut u8) = (*ctx).data[index] as u8;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copylocal(ctx: *mut ScriptContext) -> u8 {
    let mut destIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut srcIndex: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    (*ctx).data[destIndex] = (*ctx).data[srcIndex];
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copybyte(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    *ptr = *(ScriptReadWord(ctx) as usize as *mut u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setvar(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = ScriptReadHalfword(ctx);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copyvar(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = *GetVarPointer(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setorcopyvar(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = VarGet(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Compare(a: u16, b: u16) -> u8 {
    if a < b {
        return 0;
    }
    if a == b {
        return 1;
    }
    return 2;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_local_to_local(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    let mut value2: u8 = (*ctx).data[*({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    })] as u8;
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_local_to_value(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    let mut value2: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_local_to_ptr(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    let mut value2: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_ptr_to_local(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    let mut value2: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_ptr_to_value(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    let mut value2: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_ptr_to_ptr(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    let mut value2: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_var_to_value(ctx: *mut ScriptContext) -> u8 {
    let mut value1: u16 = *GetVarPointer(ScriptReadHalfword(ctx));
    let mut value2: u16 = ScriptReadHalfword(ctx);
    (*ctx).comparisonResult = Compare(value1, value2);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_var_to_var(ctx: *mut ScriptContext) -> u8 {
    let mut ptr1: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    let mut ptr2: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    (*ctx).comparisonResult = Compare(*ptr1, *ptr2);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addvar(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr += ScriptReadHalfword(ctx);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_subvar(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr -= VarGet(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_random(ctx: *mut ScriptContext) -> u8 {
    let mut max: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = rem_i32(Random() as i32, max as i32) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_additem(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = AddBagItem(itemId, quantity as u8 as u16) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removeitem(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = RemoveBagItem(itemId, quantity as u8 as u16) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkitemspace(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = CheckBagHasSpace(itemId, quantity as u8 as u16) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkitem(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = CheckBagHasItem(itemId, quantity as u8 as u16) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkitemtype(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = GetPocketByItemId(itemId) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addpcitem(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = AddPCItem(itemId, quantity) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkpcitem(ctx: *mut ScriptContext) -> u8 {
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = CheckPCHasItem(itemId, quantity) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_adddecoration(ctx: *mut ScriptContext) -> u8 {
    let mut decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = DecorationAdd(decorId as u8) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removedecoration(ctx: *mut ScriptContext) -> u8 {
    let mut decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = DecorationRemove(decorId as u8) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkdecorspace(ctx: *mut ScriptContext) -> u8 {
    let mut decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = DecorationCheckSpace(decorId as u8) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkdecor(ctx: *mut ScriptContext) -> u8 {
    let mut decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = CheckHasDecoration(decorId as u8) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setflag(ctx: *mut ScriptContext) -> u8 {
    FlagSet(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_clearflag(ctx: *mut ScriptContext) -> u8 {
    FlagClear(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkflag(ctx: *mut ScriptContext) -> u8 {
    (*ctx).comparisonResult = FlagGet(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_incrementgamestat(ctx: *mut ScriptContext) -> u8 {
    IncrementGameStat(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_animateflash(ctx: *mut ScriptContext) -> u8 {
    AnimateFlash(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
    );
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setflashlevel(ctx: *mut ScriptContext) -> u8 {
    SetFlashLevel(VarGet(ScriptReadHalfword(ctx)) as i32);
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsPaletteNotActive() -> u8 {
    if gPaletteFade.active() == 0 {
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
pub unsafe extern "C" fn ScrCmd_fadescreen(ctx: *mut ScriptContext) -> u8 {
    FadeScreen(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
        0,
    );
    SetupNativeScript(ctx, Some(IsPaletteNotActive));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadescreenspeed(ctx: *mut ScriptContext) -> u8 {
    let mut mode: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut speed: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    FadeScreen(mode, speed as i8);
    SetupNativeScript(ctx, Some(IsPaletteNotActive));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadescreenswapbuffers(ctx: *mut ScriptContext) -> u8 {
    let mut mode: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    match mode {
        FADE_FROM_BLACK | FADE_FROM_WHITE => {
            CpuSet(
                gPaletteDecompressionBuffer.as_mut_ptr() as *mut c_void,
                gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
                0x4000100,
            );
            FadeScreen(mode, 0);
        }
        _ => {
            CpuSet(
                gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
                gPaletteDecompressionBuffer.as_mut_ptr() as *mut c_void,
                0x4000100,
            );
            FadeScreen(mode, 0);
        }
    }
    SetupNativeScript(ctx, Some(IsPaletteNotActive));
    return TRUE;
}
pub(crate) unsafe extern "C" fn RunPauseTimer() -> u8 {
    if ({
        sPauseCounter -= 1;
        sPauseCounter
    }) == 0
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_delay(ctx: *mut ScriptContext) -> u8 {
    sPauseCounter = ScriptReadHalfword(ctx);
    SetupNativeScript(ctx, Some(RunPauseTimer));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_initclock(ctx: *mut ScriptContext) -> u8 {
    let mut hour: u8 = VarGet(ScriptReadHalfword(ctx)) as u8;
    let mut minute: u8 = VarGet(ScriptReadHalfword(ctx)) as u8;
    RtcInitLocalTimeOffset(hour as i32, minute as i32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dotimebasedevents(ctx: *mut ScriptContext) -> u8 {
    DoTimeBasedEvents();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gettime(ctx: *mut ScriptContext) -> u8 {
    RtcCalcLocalTime();
    gSpecialVar_0x8000 = gLocalTime.hours as u16;
    gSpecialVar_0x8001 = gLocalTime.minutes as u16;
    gSpecialVar_0x8002 = gLocalTime.seconds as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setweather(ctx: *mut ScriptContext) -> u8 {
    let mut weather: u16 = VarGet(ScriptReadHalfword(ctx));
    SetSavedWeather(weather as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_resetweather(ctx: *mut ScriptContext) -> u8 {
    SetSavedWeatherFromCurrMapHeader();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_doweather(ctx: *mut ScriptContext) -> u8 {
    DoCurrentWeather();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setstepcallback(ctx: *mut ScriptContext) -> u8 {
    ActivatePerStepCallback(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmaplayoutindex(ctx: *mut ScriptContext) -> u8 {
    let mut value: u16 = VarGet(ScriptReadHalfword(ctx));
    SetCurrentMapLayout(value);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warp(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpsilent(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoDiveWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpdoor(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoDoorWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warphole(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    PlayerGetDestCoords(&raw mut x as *mut i16, &raw mut y as *mut i16);
    if mapGroup == 255 && mapNum == 255 {
        SetWarpDestinationToFixedHoleWarp(
            x as i16 - MAP_OFFSET as i16,
            y as i16 - MAP_OFFSET as i16,
        );
    } else {
        SetWarpDestination(
            mapGroup as i8,
            mapNum as i8,
            WARP_ID_NONE,
            x as i8 - MAP_OFFSET as i8,
            y as i8 - MAP_OFFSET as i8,
        );
    }
    DoFallWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpteleport(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoTeleportTileWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpmossdeepgym(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoMossdeepGymWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setwarp(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdynamicwarp(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetDynamicWarpWithCoords(
        0,
        mapGroup as i8,
        mapNum as i8,
        warpId as i8,
        x as i8,
        y as i8,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdivewarp(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetFixedDiveWarp(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setholewarp(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetFixedHoleWarp(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setescapewarp(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetEscapeWarp(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_getplayerxy(ctx: *mut ScriptContext) -> u8 {
    let mut pX: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    let mut pY: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *pX = (*gSaveBlock1Ptr).pos.x as u16;
    *pY = (*gSaveBlock1Ptr).pos.y as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_getpartysize(ctx: *mut ScriptContext) -> u8 {
    gSpecialVar_Result = CalculatePlayerPartyCount() as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playse(ctx: *mut ScriptContext) -> u8 {
    PlaySE(ScriptReadHalfword(ctx));
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitForSoundEffectFinish() -> u8 {
    if IsSEPlaying() == 0 {
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
pub unsafe extern "C" fn ScrCmd_waitse(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(WaitForSoundEffectFinish));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playfanfare(ctx: *mut ScriptContext) -> u8 {
    PlayFanfare(ScriptReadHalfword(ctx));
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitForFanfareFinish() -> u8 {
    return IsFanfareTaskInactive();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitfanfare(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(WaitForFanfareFinish));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playbgm(ctx: *mut ScriptContext) -> u8 {
    let mut songId: u16 = ScriptReadHalfword(ctx);
    let mut save: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if save == TRUE {
        Overworld_SetSavedMusic(songId);
    }
    PlayNewMapMusic(songId);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_savebgm(ctx: *mut ScriptContext) -> u8 {
    Overworld_SetSavedMusic(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadedefaultbgm(ctx: *mut ScriptContext) -> u8 {
    Overworld_ChangeMusicToDefault();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadenewbgm(ctx: *mut ScriptContext) -> u8 {
    Overworld_ChangeMusicTo(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadeoutbgm(ctx: *mut ScriptContext) -> u8 {
    let mut speed: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if speed != 0 {
        FadeOutBGMTemporarily(4 * speed);
    } else {
        FadeOutBGMTemporarily(4);
    }
    SetupNativeScript(ctx, Some(IsBGMPausedOrStopped));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadeinbgm(ctx: *mut ScriptContext) -> u8 {
    let mut speed: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if speed != 0 {
        FadeInBGM(4 * speed);
    } else {
        FadeInBGM(4);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_applymovement(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut movementScript: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    ScriptMovement_StartObjectMovementScript(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        movementScript as *mut u8,
    );
    sMovingNpcId = localId;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_applymovementat(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut movementScript: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ScriptMovement_StartObjectMovementScript(
        localId as u8,
        mapNum,
        mapGroup,
        movementScript as *mut u8,
    );
    sMovingNpcId = localId;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitForMovementFinish() -> u8 {
    return ScriptMovement_IsObjectMovementFinished(
        sMovingNpcId as u8,
        sMovingNpcMapNum as u8,
        sMovingNpcMapGroup as u8,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmovement(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    if localId != LOCALID_NONE as u16 {
        sMovingNpcId = localId;
    }
    sMovingNpcMapGroup = (*gSaveBlock1Ptr).location.mapGroup as u16;
    sMovingNpcMapNum = (*gSaveBlock1Ptr).location.mapNum as u16;
    SetupNativeScript(ctx, Some(WaitForMovementFinish));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmovementat(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = 0;
    let mut mapNum: u8 = 0;
    if localId != LOCALID_NONE as u16 {
        sMovingNpcId = localId;
    }
    mapGroup = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    mapNum = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    sMovingNpcMapGroup = mapGroup as u16;
    sMovingNpcMapNum = mapNum as u16;
    SetupNativeScript(ctx, Some(WaitForMovementFinish));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removeobject(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    RemoveObjectEventByLocalIdAndMap(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removeobjectat(ctx: *mut ScriptContext) -> u8 {
    let mut objectId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    RemoveObjectEventByLocalIdAndMap(objectId as u8, mapNum, mapGroup);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addobject(ctx: *mut ScriptContext) -> u8 {
    let mut objectId: u16 = VarGet(ScriptReadHalfword(ctx));
    TrySpawnObjectEvent(
        objectId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addobjectat(ctx: *mut ScriptContext) -> u8 {
    let mut objectId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    TrySpawnObjectEvent(objectId as u8, mapNum, mapGroup);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectxy(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    TryMoveObjectEventToMapCoords(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        x as i16,
        y as i16,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectxyperm(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetObjEventTemplateCoords(localId as u8, x as i16, y as i16);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copyobjectxytoperm(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    TryOverrideObjectEventTemplateCoords(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showobjectat(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    SetObjectInvisibility(localId as u8, mapNum, mapGroup, FALSE);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hideobjectat(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    SetObjectInvisibility(localId as u8, mapNum, mapGroup, TRUE);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectsubpriority(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut priority: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    SetObjectSubpriority(localId as u8, mapNum, mapGroup, priority + 83);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_resetobjectsubpriority(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ResetObjectSubpriority(localId as u8, mapNum, mapGroup);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_faceplayer(ctx: *mut ScriptContext) -> u8 {
    if gObjectEvents[gSelectedObjectEvent].active() != 0 {
        ObjectEventFaceOppositeDirection(
            &raw mut gObjectEvents[gSelectedObjectEvent],
            GetPlayerFacingDirection(),
        );
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_turnobject(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut direction: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    ObjectEventTurnByLocalIdAndMap(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        direction,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectmovementtype(ctx: *mut ScriptContext) -> u8 {
    let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut movementType: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    SetObjEventTemplateMovementType(localId as u8, movementType);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_createvobject(ctx: *mut ScriptContext) -> u8 {
    let mut graphicsId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut virtualObjId: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    let mut elevation: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut direction: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    CreateVirtualObject(
        graphicsId,
        virtualObjId,
        x as i16,
        y as i16,
        elevation,
        direction,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_turnvobject(ctx: *mut ScriptContext) -> u8 {
    let mut virtualObjId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut direction: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    TurnVirtualObject(virtualObjId, direction);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_lockall(ctx: *mut ScriptContext) -> u8 {
    if IsOverworldLinkActive() != 0 {
        return FALSE;
    } else {
        FreezeObjects_WaitForPlayer();
        SetupNativeScript(ctx, Some(IsFreezePlayerFinished));
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_lock(ctx: *mut ScriptContext) -> u8 {
    if IsOverworldLinkActive() != 0 {
        return FALSE;
    } else {
        if gObjectEvents[gSelectedObjectEvent].active() != 0 {
            FreezeObjects_WaitForPlayerAndSelected();
            SetupNativeScript(ctx, Some(IsFreezeSelectedObjectAndPlayerFinished));
        } else {
            FreezeObjects_WaitForPlayer();
            SetupNativeScript(ctx, Some(IsFreezePlayerFinished));
        }
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_releaseall(ctx: *mut ScriptContext) -> u8 {
    let mut playerObjectId: u8 = 0;
    HideFieldMessageBox();
    playerObjectId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
    ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[playerObjectId]);
    ScriptMovement_UnfreezeObjectEvents();
    UnfreezeObjectEvents();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_release(ctx: *mut ScriptContext) -> u8 {
    let mut playerObjectId: u8 = 0;
    HideFieldMessageBox();
    if gObjectEvents[gSelectedObjectEvent].active() != 0 {
        ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[gSelectedObjectEvent]);
    }
    playerObjectId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
    ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[playerObjectId]);
    ScriptMovement_UnfreezeObjectEvents();
    UnfreezeObjectEvents();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_message(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    ShowFieldMessage(msg);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokenavcall(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    ShowPokenavFieldMessage(msg);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_messageautoscroll(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    gTextFlags.set_autoScroll(TRUE);
    gTextFlags.set_forceMidTextSpeed(TRUE);
    ShowFieldAutoScrollMessage(msg);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_messageinstant(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    LoadMessageBoxAndBorderGfx();
    DrawDialogueFrame(0, TRUE);
    AddTextPrinterParameterized(0, FONT_NORMAL, msg, 0, 1, 0, None);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmessage(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(IsFieldMessageBoxHidden));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_closemessage(ctx: *mut ScriptContext) -> u8 {
    HideFieldMessageBox();
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitForAorBPress() -> u8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        return TRUE;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitbuttonpress(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(WaitForAorBPress));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_yesnobox(ctx: *mut ScriptContext) -> u8 {
    let mut left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    if ScriptMenu_YesNo(left, top) == TRUE {
        ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_multichoice(ctx: *mut ScriptContext) -> u8 {
    let mut left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut ignoreBPress: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    if ScriptMenu_Multichoice(left, top, multichoiceId, ignoreBPress) == TRUE {
        ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_multichoicedefault(ctx: *mut ScriptContext) -> u8 {
    let mut left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut defaultChoice: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    let mut ignoreBPress: u8 = *({
        let t10 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t10
    });
    if ScriptMenu_MultichoiceWithDefault(left, top, multichoiceId, ignoreBPress, defaultChoice)
        == TRUE
    {
        ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_drawbox(ctx: *mut ScriptContext) -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_multichoicegrid(ctx: *mut ScriptContext) -> u8 {
    let mut left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut numColumns: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    let mut ignoreBPress: u8 = *({
        let t10 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t10
    });
    if ScriptMenu_MultichoiceGrid(left, top, multichoiceId, ignoreBPress, numColumns) == TRUE {
        ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_erasebox(ctx: *mut ScriptContext) -> u8 {
    let mut left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut right: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut bottom: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_drawboxtext(ctx: *mut ScriptContext) -> u8 {
    let mut left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut ignoreBPress: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showmonpic(ctx: *mut ScriptContext) -> u8 {
    let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ScriptMenu_ShowPokemonPic(species, x, y);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hidemonpic(ctx: *mut ScriptContext) -> u8 {
    let mut func: Option<unsafe extern "C" fn() -> u8> = ScriptMenu_HidePokemonPic();
    if func.is_none() {
        return FALSE;
    }
    SetupNativeScript(ctx, func);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showcontestpainting(ctx: *mut ScriptContext) -> u8 {
    let mut contestWinnerId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if contestWinnerId != CONTEST_WINNER_ARTIST {
        SetContestWinnerForPainting(contestWinnerId as i32);
    }
    ShowContestPainting();
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_braillemessage(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    let mut winTemplate: WindowTemplate = zeroed();
    let mut i: i32 = 0;
    let mut width: u8 = 0;
    let mut height: u8 = 0;
    let mut xWindow: u8 = 0;
    let mut yWindow: u8 = 0;
    let mut xText: u8 = 0;
    let mut yText: u8 = 0;
    let mut temp: u8 = 0;
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), ptr.at(6));
    width = (GetStringWidth(FONT_BRAILLE, gStringVar4.as_mut_ptr(), -1) / 8) as u8;
    if width > 28 {
        width = 28;
    }
    i = 0;
    height = 4;
    while gStringVar4[i] != EOS {
        if gStringVar4[{
            let t1 = i;
            i += 1;
            t1
        }] == CHAR_NEWLINE
        {
            height += 3;
        }
    }
    if height > 18 {
        height = 18;
    }
    temp = width + 2;
    xWindow = ((30 - temp as i32) / 2) as u8;
    temp = height + 2;
    yText = ((20 - temp as i32) / 2) as u8;
    xText = xWindow;
    xWindow += 1;
    yWindow = yText;
    yText += 2;
    xText = (xWindow - xText - 1) * 8 + 3;
    yText = (yText - yWindow - 1) * 8;
    winTemplate = CreateWindowTemplate(0, xWindow, yWindow + 1, width, height, 0xF, 0x1);
    sBrailleWindowId = AddWindow(&raw mut winTemplate) as u8;
    LoadUserWindowBorderGfx(sBrailleWindowId, 0x214, 224);
    DrawStdWindowFrame(sBrailleWindowId, FALSE);
    PutWindowTilemap(sBrailleWindowId);
    FillWindowPixelBuffer(sBrailleWindowId, 17);
    AddTextPrinterParameterized(
        sBrailleWindowId,
        FONT_BRAILLE,
        gStringVar4.as_mut_ptr(),
        xText,
        yText,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sBrailleWindowId, COPYWIN_FULL);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_closebraillemessage(ctx: *mut ScriptContext) -> u8 {
    CloseBrailleWindow();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vmessage(ctx: *mut ScriptContext) -> u8 {
    let mut msg: u32 = ScriptReadWord(ctx);
    ShowFieldMessage((msg - sAddressOffset) as usize as *mut u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferspeciesname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        gSpeciesNames[species].as_ptr().cast_mut(),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferleadmonspeciesname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut dest: *mut u8 = sScriptStringVars[stringVarIndex];
    let mut partyIndex: u8 = GetLeadMonIndex();
    let mut species: u32 = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPECIES,
        null_mut(),
    );
    StringCopy(dest, gSpeciesNames[species].as_ptr().cast_mut());
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferpartymonnick(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_NICKNAME,
        sScriptStringVars[stringVarIndex],
    );
    StringGet_Nickname(sScriptStringVars[stringVarIndex]);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferitemname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    CopyItemName(itemId, sScriptStringVars[stringVarIndex]);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferitemnameplural(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut quantity: u16 = VarGet(ScriptReadHalfword(ctx));
    CopyItemNameHandlePlural(itemId, sScriptStringVars[stringVarIndex], quantity as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferdecorationname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut decorId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        gDecorations[decorId].name.as_ptr().cast_mut(),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffermovename(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut r#move: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        gMoveNames[r#move].as_ptr().cast_mut(),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffernumberstring(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut num: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut numDigits: u8 = CountDigits(num as i32) as u8;
    ConvertIntToDecimalStringN(
        sScriptStringVars[stringVarIndex],
        num as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        numDigits,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferstdstring(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(sScriptStringVars[stringVarIndex], gStdStrings[index]);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffercontestname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut category: u16 = VarGet(ScriptReadHalfword(ctx));
    BufferContestName(sScriptStringVars[stringVarIndex], category as u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferstring(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut text: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    StringCopy(sScriptStringVars[stringVarIndex], text);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vbuffermessage(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u8 = (ScriptReadWord(ctx) - sAddressOffset) as usize as *mut u8;
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), ptr);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vbufferstring(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut addr: u32 = ScriptReadWord(ctx);
    let mut src: *mut u8 = (addr - sAddressOffset) as usize as *mut u8;
    let mut dest: *mut u8 = sScriptStringVars[stringVarIndex];
    StringCopy(dest, src);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferboxname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut boxId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        GetBoxNamePtr(boxId as u8),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_givemon(ctx: *mut ScriptContext) -> u8 {
    let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut level: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut item: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut unkParam1: u32 = ScriptReadWord(ctx);
    let mut unkParam2: u32 = ScriptReadWord(ctx);
    let mut unkParam3: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    gSpecialVar_Result =
        ScriptGiveMon(species, level, item, unkParam1, unkParam2, unkParam3) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_giveegg(ctx: *mut ScriptContext) -> u8 {
    let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = ScriptGiveEgg(species) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmonmove(ctx: *mut ScriptContext) -> u8 {
    let mut partyIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut slot: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut r#move: u16 = ScriptReadHalfword(ctx);
    ScriptSetMonMoveSlot(partyIndex, r#move, slot);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkpartymove(ctx: *mut ScriptContext) -> u8 {
    let mut i: u8 = 0;
    let mut r#move: u16 = ScriptReadHalfword(ctx);
    gSpecialVar_Result = PARTY_SIZE as u16;
    i = 0;
    while i < PARTY_SIZE as u8 {
        let mut species: u16 =
            GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut()) as u16;
        if species == 0 {
            break;
        }
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
            && MonKnowsMove(&raw mut gPlayerParty[i], r#move) == TRUE
        {
            gSpecialVar_Result = i as u16;
            gSpecialVar_0x8004 = species;
            break;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addmoney(ctx: *mut ScriptContext) -> u8 {
    let mut amount: u32 = ScriptReadWord(ctx);
    let mut ignore: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if ignore == 0 {
        AddMoney(&raw mut (*gSaveBlock1Ptr).money, amount);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removemoney(ctx: *mut ScriptContext) -> u8 {
    let mut amount: u32 = ScriptReadWord(ctx);
    let mut ignore: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if ignore == 0 {
        RemoveMoney(&raw mut (*gSaveBlock1Ptr).money, amount);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkmoney(ctx: *mut ScriptContext) -> u8 {
    let mut amount: u32 = ScriptReadWord(ctx);
    let mut ignore: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if ignore == 0 {
        gSpecialVar_Result = IsEnoughMoney(&raw mut (*gSaveBlock1Ptr).money, amount) as u16;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showmoneybox(ctx: *mut ScriptContext) -> u8 {
    let mut x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut ignore: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    if ignore == 0 {
        DrawMoneyBox(GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32, x, y);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hidemoneybox(ctx: *mut ScriptContext) -> u8 {
    HideMoneyBox();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_updatemoneybox(ctx: *mut ScriptContext) -> u8 {
    let mut x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut ignore: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    if ignore == 0 {
        ChangeAmountInMoneyBox(GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showcoinsbox(ctx: *mut ScriptContext) -> u8 {
    let mut x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ShowCoinsWindow(GetCoins() as u32, x, y);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hidecoinsbox(ctx: *mut ScriptContext) -> u8 {
    let mut x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    HideCoinsWindow();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_updatecoinsbox(ctx: *mut ScriptContext) -> u8 {
    let mut x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    PrintCoinsString(GetCoins() as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_trainerbattle(ctx: *mut ScriptContext) -> u8 {
    (*ctx).scriptPtr = BattleSetup_ConfigureTrainerBattle((*ctx).scriptPtr);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dotrainerbattle(ctx: *mut ScriptContext) -> u8 {
    BattleSetup_StartTrainerBattle();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotopostbattlescript(ctx: *mut ScriptContext) -> u8 {
    (*ctx).scriptPtr = BattleSetup_GetScriptAddrAfterBattle();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotobeatenscript(ctx: *mut ScriptContext) -> u8 {
    (*ctx).scriptPtr = BattleSetup_GetTrainerPostBattleScript();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checktrainerflag(ctx: *mut ScriptContext) -> u8 {
    let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
    (*ctx).comparisonResult = HasTrainerBeenFought(index);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_settrainerflag(ctx: *mut ScriptContext) -> u8 {
    let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
    SetTrainerFlag(index);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_cleartrainerflag(ctx: *mut ScriptContext) -> u8 {
    let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
    ClearTrainerFlag(index);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setwildbattle(ctx: *mut ScriptContext) -> u8 {
    let mut species: u16 = ScriptReadHalfword(ctx);
    let mut level: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut item: u16 = ScriptReadHalfword(ctx);
    CreateScriptedWildMon(species, level, item);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dowildbattle(ctx: *mut ScriptContext) -> u8 {
    BattleSetup_StartScriptedWildBattle();
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokemart(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    CreatePokemartMenu(ptr as *mut u16);
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokemartdecoration(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    CreateDecorationShop1Menu(ptr as *mut u16);
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokemartdecoration2(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    CreateDecorationShop2Menu(ptr as *mut u16);
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playslotmachine(ctx: *mut ScriptContext) -> u8 {
    let mut machineId: u8 = VarGet(ScriptReadHalfword(ctx)) as u8;
    PlaySlotMachine(machineId, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setberrytree(ctx: *mut ScriptContext) -> u8 {
    let mut treeId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut berry: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut growthStage: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    if berry == 0 {
        PlantBerryTree(treeId, berry, growthStage, FALSE);
    } else {
        PlantBerryTree(treeId, berry, growthStage, FALSE);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_getpokenewsactive(ctx: *mut ScriptContext) -> u8 {
    let mut newsKind: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = IsPokeNewsActive(newsKind as u8) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_choosecontestmon(ctx: *mut ScriptContext) -> u8 {
    ChooseContestMon();
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_startcontest(ctx: *mut ScriptContext) -> u8 {
    StartContest();
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showcontestresults(ctx: *mut ScriptContext) -> u8 {
    ShowContestResults();
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_contestlinktransfer(ctx: *mut ScriptContext) -> u8 {
    ContestLinkTransfer(gSpecialVar_ContestCategory as u8);
    ScriptContext_Stop();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dofieldeffect(ctx: *mut ScriptContext) -> u8 {
    let mut effectId: u16 = VarGet(ScriptReadHalfword(ctx));
    sFieldEffectScriptId = effectId;
    FieldEffectStart(sFieldEffectScriptId as u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setfieldeffectargument(ctx: *mut ScriptContext) -> u8 {
    let mut argNum: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    gFieldEffectArguments[argNum] = VarGet(ScriptReadHalfword(ctx)) as i16 as i32;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitForFieldEffectFinish() -> u8 {
    if FieldEffectActiveListContains(sFieldEffectScriptId as u8) == 0 {
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
pub unsafe extern "C" fn ScrCmd_waitfieldeffect(ctx: *mut ScriptContext) -> u8 {
    sFieldEffectScriptId = VarGet(ScriptReadHalfword(ctx));
    SetupNativeScript(ctx, Some(WaitForFieldEffectFinish));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setrespawn(ctx: *mut ScriptContext) -> u8 {
    let mut healLocationId: u16 = VarGet(ScriptReadHalfword(ctx));
    SetLastHealLocationWarp(healLocationId as u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkplayergender(ctx: *mut ScriptContext) -> u8 {
    gSpecialVar_Result = (*gSaveBlock2Ptr).playerGender as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playmoncry(ctx: *mut ScriptContext) -> u8 {
    let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut mode: u16 = VarGet(ScriptReadHalfword(ctx));
    PlayCry_Script(species, mode as u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmoncry(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(IsCryFinished));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmetatile(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut metatileId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut isImpassable: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    if isImpassable == 0 {
        MapGridSetMetatileIdAt(x as i32, y as i32, metatileId);
    } else {
        MapGridSetMetatileIdAt(x as i32, y as i32, metatileId | MAPGRID_IMPASSABLE);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_opendoor(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    PlaySE(GetDoorSoundEffect(x as u32, y as u32) as u16);
    FieldAnimateDoorOpen(x as u32, y as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_closedoor(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    FieldAnimateDoorClose(x as u32, y as u32);
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsDoorAnimationStopped() -> u8 {
    if FieldIsDoorAnimationRunning() == 0 {
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
pub unsafe extern "C" fn ScrCmd_waitdooranim(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(IsDoorAnimationStopped));
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdooropen(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    FieldSetDoorOpened(x as u32, y as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdoorclosed(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    FieldSetDoorClosed(x as u32, y as u32);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addelevmenuitem(ctx: *mut ScriptContext) -> u8 {
    let mut v3: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut v5: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut v7: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut v9: u16 = VarGet(ScriptReadHalfword(ctx));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showelevmenu(ctx: *mut ScriptContext) -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkcoins(ctx: *mut ScriptContext) -> u8 {
    let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = GetCoins();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addcoins(ctx: *mut ScriptContext) -> u8 {
    let mut coins: u16 = VarGet(ScriptReadHalfword(ctx));
    if AddCoins(coins) == TRUE {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removecoins(ctx: *mut ScriptContext) -> u8 {
    let mut coins: u16 = VarGet(ScriptReadHalfword(ctx));
    if RemoveCoins(coins) == TRUE {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_moverotatingtileobjects(ctx: *mut ScriptContext) -> u8 {
    let mut puzzleNumber: u16 = VarGet(ScriptReadHalfword(ctx));
    sMovingNpcId = MoveRotatingTileObjects(puzzleNumber as u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_turnrotatingtileobjects(ctx: *mut ScriptContext) -> u8 {
    TurnRotatingTileObjects();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_initrotatingtilepuzzle(ctx: *mut ScriptContext) -> u8 {
    let mut isTrickHouse: u16 = VarGet(ScriptReadHalfword(ctx));
    InitRotatingTilePuzzle(isTrickHouse as u8);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_freerotatingtilepuzzle(ctx: *mut ScriptContext) -> u8 {
    FreeRotatingTilePuzzle();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_selectapproachingtrainer(ctx: *mut ScriptContext) -> u8 {
    gSelectedObjectEvent = GetCurrentApproachingTrainerObjectEventId();
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_lockfortrainer(ctx: *mut ScriptContext) -> u8 {
    if IsOverworldLinkActive() != 0 {
        return FALSE;
    } else {
        if gObjectEvents[gSelectedObjectEvent].active() != 0 {
            FreezeForApproachingTrainers();
            SetupNativeScript(ctx, Some(IsFreezeObjectAndPlayerFinished));
        }
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmodernfatefulencounter(ctx: *mut ScriptContext) -> u8 {
    let mut isModernFatefulEncounter: u8 = TRUE;
    let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    SetMonData(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        &raw mut isModernFatefulEncounter as *mut c_void,
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkmodernfatefulencounter(ctx: *mut ScriptContext) -> u8 {
    let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        null_mut(),
    ) as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_trywondercardscript(ctx: *mut ScriptContext) -> u8 {
    let mut script: *mut u8 = GetSavedRamScriptIfValid();
    if !script.is_null() {
        gRamScriptRetAddr = (*ctx).scriptPtr;
        ScriptJump(ctx, script);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpspinenter(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    SetSpinStartFacingDir(GetPlayerFacingDirection());
    DoSpinEnterWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmonmetlocation(ctx: *mut ScriptContext) -> u8 {
    let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut location: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if partyIndex < PARTY_SIZE as u16 {
        SetMonData(
            &raw mut gPlayerParty[partyIndex],
            MON_DATA_MET_LOCATION,
            &raw mut location as *mut c_void,
        );
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CloseBrailleWindow() {
    ClearStdWindowAndFrame(sBrailleWindowId, TRUE);
    RemoveWindow(sBrailleWindowId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffertrainerclassname(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut trainerClassId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        GetTrainerClassNameFromId(trainerClassId),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffertrainername(ctx: *mut ScriptContext) -> u8 {
    let mut stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut trainerClassId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        GetTrainerNameFromId(trainerClassId),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMovingNpcId(npcId: u16) {
    sMovingNpcId = npcId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpwhitefade(ctx: *mut ScriptContext) -> u8 {
    let mut mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mut mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let mut warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoWhiteFadeWarp();
    ResetInitialPlayerAvatarState();
    return TRUE;
}
