//! Translated from `src/scrcmd.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gNullScriptPtr sScriptConditionTable sScriptStringVars
#[allow(unused_imports)]
use crate::data::scrcmd::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRamScriptRetAddr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAddressOffset: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPauseCounter: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingNpcId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingNpcMapGroup: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingNpcMapNum: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFieldEffectScriptId: u16 = 0u16;
pub(crate) static mut sBrailleWindowId: u8 = 0u8;

unsafe extern "C" {
    static mut gDecorations: u8;
    static mut gFieldEffectArguments: u8;
    static mut gLocalTime: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteDecompressionBuffer: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedObjectEvent: u8;
    static mut gSpecialVar_0x8000: u8;
    static mut gSpecialVar_0x8001: u8;
    static mut gSpecialVar_0x8002: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_ContestCategory: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpecials: u8;
    static mut gSpeciesNames: u8;
    static mut gStdScripts: u8;
    static mut gStdScripts_End: u8;
    static mut gStdStrings: u8;
    static mut gStringVar4: u8;
    static mut gTextFlags: u8;
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
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
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
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
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
    ) -> crate::c::Rec4<8>;
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
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
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
    fn MonKnowsMove(a0: *mut u8, a1: u16) -> u8;
    fn MoveRotatingTileObjects(a0: u8) -> u16;
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventFaceOppositeDirection(a0: *mut u8, a1: u8) -> u8;
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
    fn ScriptCall(a0: *mut u8, a1: *mut u8);
    fn ScriptContext_Stop();
    fn ScriptGiveEgg(a0: u16) -> u8;
    fn ScriptGiveMon(a0: u16, a1: u8, a2: u16, a3: u32, a4: u32, a5: u8) -> u8;
    fn ScriptJump(a0: *mut u8, a1: *mut u8);
    fn ScriptMenu_HidePokemonPic() -> Option<unsafe extern "C" fn() -> u8>;
    fn ScriptMenu_Multichoice(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn ScriptMenu_MultichoiceGrid(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn ScriptMenu_MultichoiceWithDefault(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn ScriptMenu_ShowPokemonPic(a0: u16, a1: u8, a2: u8) -> u8;
    fn ScriptMenu_YesNo(a0: u8, a1: u8) -> u8;
    fn ScriptMovement_IsObjectMovementFinished(a0: u8, a1: u8, a2: u8) -> u8;
    fn ScriptMovement_StartObjectMovementScript(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn ScriptMovement_UnfreezeObjectEvents();
    fn ScriptReadHalfword(a0: *mut u8) -> u16;
    fn ScriptReadWord(a0: *mut u8) -> u32;
    fn ScriptReturn(a0: *mut u8);
    fn ScriptSetMonMoveSlot(a0: u8, a1: u16, a2: u8);
    fn SetContestWinnerForPainting(a0: i32);
    fn SetCurrentMapLayout(a0: u16);
    fn SetDynamicWarpWithCoords(a0: i32, a1: i8, a2: i8, a3: i8, a4: i8, a5: i8);
    fn SetEscapeWarp(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetFixedDiveWarp(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetFixedHoleWarp(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetFlashLevel(a0: i32);
    fn SetLastHealLocationWarp(a0: u8);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
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
    fn SetupNativeScript(a0: *mut u8, a1: Option<unsafe extern "C" fn() -> u8>);
    fn ShowCoinsWindow(a0: u32, a1: u8, a2: u8);
    fn ShowContestPainting();
    fn ShowContestResults();
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn ShowPokenavFieldMessage(a0: *mut u8) -> u8;
    fn StartContest();
    fn StopScript(a0: *mut u8);
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
pub unsafe extern "C" fn ScrCmd_nop(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_nop1(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_end(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        StopScript(ctx);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotonative(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut addr: Option<unsafe extern "C" fn() -> u8> =
            (core::mem::transmute::<usize, Option<unsafe extern "C" fn() -> u8>>(
                (ScriptReadWord(ctx)) as usize,
            ));
        SetupNativeScript(ctx, addr);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_special(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u16 = ScriptReadHalfword(ctx);
        (((((&raw mut gSpecials).cast::<Option<unsafe extern "C" fn() -> u16>>())
            .cast::<Option<unsafe extern "C" fn() -> u16>>())
        .wrapping_offset(((index) as i32) as isize))
        .read())
        .unwrap_unchecked()();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_specialvar(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut var: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (var).write((((((&raw mut gSpecials)
            .cast::<Option<unsafe extern "C" fn() -> u16>>())
        .cast::<Option<unsafe extern "C" fn() -> u16>>())
        .wrapping_offset(((ScriptReadHalfword(ctx)) as i32) as isize))
        .read())
        .unwrap_unchecked()());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_callnative(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut func: Option<unsafe extern "C" fn()> = (core::mem::transmute::<
            usize,
            Option<unsafe extern "C" fn()>,
        >((ScriptReadWord(ctx)) as usize));
        (func).unwrap_unchecked()();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitstate(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_goto(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        ScriptJump(ctx, ptr);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_return(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ScriptReturn(ctx);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_call(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        ScriptCall(ctx, ptr);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_goto_if(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut condition: i32 = ((({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read()) as i32);
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        if ((((((((&raw const sScriptConditionTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((condition) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(((((ctx).wrapping_add(2)).read()) as i32) as isize))
        .read()) as i32)
            == 1i32
        {
            ScriptJump(ctx, ptr);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_call_if(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut condition: i32 = ((({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read()) as i32);
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        if ((((((((&raw const sScriptConditionTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((condition) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(((((ctx).wrapping_add(2)).read()) as i32) as isize))
        .read()) as i32)
            == 1i32
        {
            ScriptCall(ctx, ptr);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setvaddress(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut addr1: u32 =
            ((((ctx).wrapping_add(8).cast::<*mut u8>()).read()) as usize as u32).wrapping_sub(1u32);
        let mut addr2: u32 = ScriptReadWord(ctx);
        ((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).write((addr2).wrapping_sub(addr1));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vgoto(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut addr: u32 = ScriptReadWord(ctx);
        ScriptJump(
            ctx,
            (((addr).wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
                as usize as *mut u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vcall(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut addr: u32 = ScriptReadWord(ctx);
        ScriptCall(
            ctx,
            (((addr).wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
                as usize as *mut u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vgoto_if(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut condition: i32 = ((({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read()) as i32);
        let mut ptr: *mut u8 = (((ScriptReadWord(ctx))
            .wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
            as usize as *mut u8);
        if ((((((((&raw const sScriptConditionTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((condition) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(((((ctx).wrapping_add(2)).read()) as i32) as isize))
        .read()) as i32)
            == 1i32
        {
            ScriptJump(ctx, ptr);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vcall_if(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut condition: i32 = ((({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read()) as i32);
        let mut ptr: *mut u8 = (((ScriptReadWord(ctx))
            .wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
            as usize as *mut u8);
        if ((((((((&raw const sScriptConditionTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((condition) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(((((ctx).wrapping_add(2)).read()) as i32) as isize))
        .read()) as i32)
            == 1i32
        {
            ScriptCall(ctx, ptr);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotostd(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut ptr: *mut *mut u8 = (((&raw mut gStdScripts).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(((index) as i32) as isize);
        if ((ptr) as usize)
            < ((((&raw mut gStdScripts_End).cast::<*mut u8>()).cast::<*mut u8>()) as usize)
        {
            ScriptJump(ctx, (ptr).read());
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_callstd(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut ptr: *mut *mut u8 = (((&raw mut gStdScripts).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(((index) as i32) as isize);
        if ((ptr) as usize)
            < ((((&raw mut gStdScripts_End).cast::<*mut u8>()).cast::<*mut u8>()) as usize)
        {
            ScriptCall(ctx, (ptr).read());
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotostd_if(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut condition: i32 = ((({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read()) as i32);
        let mut index: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        if ((((((((&raw const sScriptConditionTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((condition) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(((((ctx).wrapping_add(2)).read()) as i32) as isize))
        .read()) as i32)
            == 1i32
        {
            let mut ptr: *mut *mut u8 = (((&raw mut gStdScripts).cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((index) as i32) as isize);
            if ((ptr) as usize)
                < ((((&raw mut gStdScripts_End).cast::<*mut u8>()).cast::<*mut u8>()) as usize)
            {
                ScriptJump(ctx, (ptr).read());
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_callstd_if(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut condition: i32 = ((({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read()) as i32);
        let mut index: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        if ((((((((&raw const sScriptConditionTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((condition) as isize * 3))
        .cast::<u8>())
        .wrapping_offset(((((ctx).wrapping_add(2)).read()) as i32) as isize))
        .read()) as i32)
            == 1i32
        {
            let mut ptr: *mut *mut u8 = (((&raw mut gStdScripts).cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((index) as i32) as isize);
            if ((ptr) as usize)
                < ((((&raw mut gStdScripts_End).cast::<*mut u8>()).cast::<*mut u8>()) as usize)
            {
                ScriptCall(ctx, (ptr).read());
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_returnram(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ScriptJump(
            ctx,
            ((&raw mut gRamScriptRetAddr).cast::<u8>().cast::<*mut u8>()).read(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_endram(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ClearRamScript();
        StopScript(ctx);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmysteryeventstatus(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut status: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        SetMysteryEventScriptStatus(((status) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_loadword(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(((index) as i32) as isize))
            .write(ScriptReadWord(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_loadbytefromptr(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(((index) as i32) as isize))
            .write(((((ScriptReadWord(ctx)) as usize as *mut u8).read()) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setptr(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((ScriptReadWord(ctx)) as usize as *mut u8).write(value);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_loadbyte(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(((index) as i32) as isize))
            .write(
                ((({
                    let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
                    let __t8 = (__p7).read();
                    (__p7).write(((__p7).read()).wrapping_offset(1));
                    __t8
                })
                .read()) as u32),
            );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setptrbyte(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((ScriptReadWord(ctx)) as usize as *mut u8).write(
            ((((((ctx).wrapping_add(100)).cast::<u32>())
                .wrapping_offset(((index) as i32) as isize))
            .read()) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copylocal(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut destIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut srcIndex: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(((destIndex) as i32) as isize))
            .write(
                ((((ctx).wrapping_add(100)).cast::<u32>())
                    .wrapping_offset(((srcIndex) as i32) as isize))
                .read(),
            );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copybyte(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        (ptr).write(((ScriptReadWord(ctx)) as usize as *mut u8).read());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setvar(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (ptr).write(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copyvar(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (ptr).write((GetVarPointer(ScriptReadHalfword(ctx))).read());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setorcopyvar(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (ptr).write(VarGet(ScriptReadHalfword(ctx)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Compare(a: u16, b: u16) -> u8 {
    unsafe {
        let mut a = a;
        let mut b = b;
        if ((a) as i32) < ((b) as i32) {
            return 0u8;
        }
        if ((a) as i32) == ((b) as i32) {
            return 1u8;
        }
        return 2u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_local_to_local(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u8 = ((((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(
            ((({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read()) as i32) as isize,
        ))
        .read()) as u8);
        let mut value2: u8 = ((((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(
            ((({
                let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t8 = (__p7).read();
                (__p7).write(((__p7).read()).wrapping_offset(1));
                __t8
            })
            .read()) as i32) as isize,
        ))
        .read()) as u8);
        ((ctx).wrapping_add(2)).write(Compare(((value1) as u16), ((value2) as u16)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_local_to_value(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u8 = ((((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(
            ((({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read()) as i32) as isize,
        ))
        .read()) as u8);
        let mut value2: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ((ctx).wrapping_add(2)).write(Compare(((value1) as u16), ((value2) as u16)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_local_to_ptr(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u8 = ((((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(
            ((({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read()) as i32) as isize,
        ))
        .read()) as u8);
        let mut value2: u8 = ((ScriptReadWord(ctx)) as usize as *mut u8).read();
        ((ctx).wrapping_add(2)).write(Compare(((value1) as u16), ((value2) as u16)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_ptr_to_local(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u8 = ((ScriptReadWord(ctx)) as usize as *mut u8).read();
        let mut value2: u8 = ((((((ctx).wrapping_add(100)).cast::<u32>()).wrapping_offset(
            ((({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read()) as i32) as isize,
        ))
        .read()) as u8);
        ((ctx).wrapping_add(2)).write(Compare(((value1) as u16), ((value2) as u16)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_ptr_to_value(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u8 = ((ScriptReadWord(ctx)) as usize as *mut u8).read();
        let mut value2: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((ctx).wrapping_add(2)).write(Compare(((value1) as u16), ((value2) as u16)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_ptr_to_ptr(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u8 = ((ScriptReadWord(ctx)) as usize as *mut u8).read();
        let mut value2: u8 = ((ScriptReadWord(ctx)) as usize as *mut u8).read();
        ((ctx).wrapping_add(2)).write(Compare(((value1) as u16), ((value2) as u16)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_var_to_value(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value1: u16 = (GetVarPointer(ScriptReadHalfword(ctx))).read();
        let mut value2: u16 = ScriptReadHalfword(ctx);
        ((ctx).wrapping_add(2)).write(Compare(value1, value2));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_compare_var_to_var(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr1: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        let mut ptr2: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        ((ctx).wrapping_add(2)).write(Compare((ptr1).read(), (ptr2).read()));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addvar(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (ptr).write(
            (((((ptr).read()) as i32).wrapping_add(((ScriptReadHalfword(ctx)) as i32))) as u16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_subvar(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (ptr).write(
            (((((ptr).read()) as i32).wrapping_sub(((VarGet(ScriptReadHalfword(ctx))) as i32)))
                as u16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_random(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut max: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((crate::c::rem_i32(((Random()) as i32), ((max) as i32))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_additem(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((AddBagItem(itemId, (((quantity) as u8) as u16))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removeitem(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((RemoveBagItem(itemId, (((quantity) as u8) as u16))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkitemspace(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((CheckBagHasSpace(itemId, (((quantity) as u8) as u16))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkitem(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((CheckBagHasItem(itemId, (((quantity) as u8) as u16))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkitemtype(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((GetPocketByItemId(itemId)) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addpcitem(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((AddPCItem(itemId, quantity)) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkpcitem(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((CheckPCHasItem(itemId, quantity)) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_adddecoration(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut decorId: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((DecorationAdd(((decorId) as u8))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removedecoration(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut decorId: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((DecorationRemove(((decorId) as u8))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkdecorspace(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut decorId: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((DecorationCheckSpace(((decorId) as u8))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkdecor(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut decorId: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((CheckHasDecoration(((decorId) as u8))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setflag(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        FlagSet(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_clearflag(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        FlagClear(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkflag(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((ctx).wrapping_add(2)).write(FlagGet(ScriptReadHalfword(ctx)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_incrementgamestat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        IncrementGameStat(
            ({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_animateflash(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        AnimateFlash(
            ({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read(),
        );
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setflashlevel(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetFlashLevel(((VarGet(ScriptReadHalfword(ctx))) as i32));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsPaletteNotActive() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
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
pub unsafe extern "C" fn ScrCmd_fadescreen(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        FadeScreen(
            ({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read(),
            0i8,
        );
        SetupNativeScript(ctx, Some(IsPaletteNotActive));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadescreenspeed(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mode: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut speed: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        FadeScreen(mode, ((speed) as i8));
        SetupNativeScript(ctx, Some(IsPaletteNotActive));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadescreenswapbuffers(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mode: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        'l1: {
            let __sw5 = ((mode) as i32);
            let __matched = __sw5 == 1i32 || __sw5 == 3i32 || __sw5 == 0i32 || __sw5 == 2i32;
            if __sw5 == 1i32 || __sw5 == 3i32 || !__matched {
                'l2: loop {
                    'l3: {
                        'l4: loop {
                            'l5: {
                                CpuSet(
                                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .cast::<u8>(),
                                    (&raw mut gPaletteDecompressionBuffer).cast::<u8>(),
                                    ((67108864i32
                                        | (crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l4;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                FadeScreen(mode, 0i8);
                break 'l1;
            }
            if __sw5 == 0i32 || __sw5 == 2i32 {
                'l6: loop {
                    'l7: {
                        'l8: loop {
                            'l9: {
                                CpuSet(
                                    (&raw mut gPaletteDecompressionBuffer).cast::<u8>(),
                                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .cast::<u8>(),
                                    ((67108864i32
                                        | (crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l8;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
                FadeScreen(mode, 0i8);
                break 'l1;
            }
        }
        SetupNativeScript(ctx, Some(IsPaletteNotActive));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn RunPauseTimer() -> u8 {
    unsafe {
        if (({
            let __p1 = (&raw mut sPauseCounter).cast::<u8>().cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
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
pub unsafe extern "C" fn ScrCmd_delay(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((&raw mut sPauseCounter).cast::<u8>().cast::<u16>()).write(ScriptReadHalfword(ctx));
        SetupNativeScript(ctx, Some(RunPauseTimer));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_initclock(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut hour: u8 = ((VarGet(ScriptReadHalfword(ctx))) as u8);
        let mut minute: u8 = ((VarGet(ScriptReadHalfword(ctx))) as u8);
        RtcInitLocalTimeOffset(((hour) as i32), ((minute) as i32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dotimebasedevents(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        DoTimeBasedEvents();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gettime(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        RtcCalcLocalTime();
        ((&raw mut gSpecialVar_0x8000).cast::<u16>()).write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(2)
                .cast::<i8>())
            .read()) as u16),
        );
        ((&raw mut gSpecialVar_0x8001).cast::<u16>()).write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(3)
                .cast::<i8>())
            .read()) as u16),
        );
        ((&raw mut gSpecialVar_0x8002).cast::<u16>()).write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(4)
                .cast::<i8>())
            .read()) as u16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setweather(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut weather: u16 = VarGet(ScriptReadHalfword(ctx));
        SetSavedWeather(((weather) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_resetweather(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetSavedWeatherFromCurrMapHeader();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_doweather(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        DoCurrentWeather();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setstepcallback(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ActivatePerStepCallback(
            ({
                let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                __t4
            })
            .read(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmaplayoutindex(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut value: u16 = VarGet(ScriptReadHalfword(ctx));
        SetCurrentMapLayout(value);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warp(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        DoWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpsilent(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        DoDiveWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpdoor(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        DoDoorWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warphole(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        PlayerGetDestCoords((&raw mut x).cast::<i16>(), (&raw mut y).cast::<i16>());
        if (((mapGroup) as i32) == 255i32) && (((mapNum) as i32) == 255i32) {
            SetWarpDestinationToFixedHoleWarp(
                ((((x) as i32).wrapping_sub(7i32)) as i16),
                ((((y) as i32).wrapping_sub(7i32)) as i16),
            );
        } else {
            SetWarpDestination(
                ((mapGroup) as i8),
                ((mapNum) as i8),
                (-1i8),
                ((((x) as i32).wrapping_sub(7i32)) as i8),
                ((((y) as i32).wrapping_sub(7i32)) as i8),
            );
        }
        DoFallWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpteleport(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        DoTeleportTileWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpmossdeepgym(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        DoMossdeepGymWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setwarp(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdynamicwarp(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetDynamicWarpWithCoords(
            0i32,
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdivewarp(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetFixedDiveWarp(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setholewarp(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetFixedHoleWarp(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setescapewarp(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetEscapeWarp(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_getplayerxy(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut pX: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        let mut pY: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (pX).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as u16),
        );
        (pY).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as u16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_getpartysize(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((CalculatePlayerPartyCount()) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playse(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        PlaySE(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitForSoundEffectFinish() -> u8 {
    unsafe {
        if !((IsSEPlaying()) != 0) {
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
pub unsafe extern "C" fn ScrCmd_waitse(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetupNativeScript(ctx, Some(WaitForSoundEffectFinish));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playfanfare(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        PlayFanfare(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitForFanfareFinish() -> u8 {
    unsafe {
        return IsFanfareTaskInactive();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitfanfare(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetupNativeScript(ctx, Some(WaitForFanfareFinish));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playbgm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut songId: u16 = ScriptReadHalfword(ctx);
        let mut save: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if ((save) as i32) == 1i32 {
            Overworld_SetSavedMusic(songId);
        }
        PlayNewMapMusic(songId);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_savebgm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        Overworld_SetSavedMusic(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadedefaultbgm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        Overworld_ChangeMusicToDefault();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadenewbgm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        Overworld_ChangeMusicTo(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadeoutbgm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut speed: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if ((speed) as i32) != 0i32 {
            FadeOutBGMTemporarily((((4i32).wrapping_mul(((speed) as i32))) as u8));
        } else {
            FadeOutBGMTemporarily(4u8);
        }
        SetupNativeScript(ctx, Some(IsBGMPausedOrStopped));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_fadeinbgm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut speed: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if ((speed) as i32) != 0i32 {
            FadeInBGM((((4i32).wrapping_mul(((speed) as i32))) as u8));
        } else {
            FadeInBGM(4u8);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_applymovement(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut movementScript: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        ScriptMovement_StartObjectMovementScript(
            ((localId) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            movementScript,
        );
        ((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>()).write(localId);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_applymovementat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut movementScript: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ScriptMovement_StartObjectMovementScript(
            ((localId) as u8),
            mapNum,
            mapGroup,
            movementScript,
        );
        ((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>()).write(localId);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitForMovementFinish() -> u8 {
    unsafe {
        return ScriptMovement_IsObjectMovementFinished(
            ((((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>()).read()) as u8),
            ((((&raw mut sMovingNpcMapNum).cast::<u8>().cast::<u16>()).read()) as u8),
            ((((&raw mut sMovingNpcMapGroup).cast::<u8>().cast::<u16>()).read()) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmovement(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        if ((localId) as i32) != 0i32 {
            ((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>()).write(localId);
        }
        ((&raw mut sMovingNpcMapGroup).cast::<u8>().cast::<u16>()).write(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u16),
        );
        ((&raw mut sMovingNpcMapNum).cast::<u8>().cast::<u16>()).write(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u16),
        );
        SetupNativeScript(ctx, Some(WaitForMovementFinish));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmovementat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = 0u8;
        let mut mapNum: u8 = 0u8;
        if ((localId) as i32) != 0i32 {
            ((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>()).write(localId);
        }
        mapGroup = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        mapNum = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ((&raw mut sMovingNpcMapGroup).cast::<u8>().cast::<u16>()).write(((mapGroup) as u16));
        ((&raw mut sMovingNpcMapNum).cast::<u8>().cast::<u16>()).write(((mapNum) as u16));
        SetupNativeScript(ctx, Some(WaitForMovementFinish));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removeobject(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        RemoveObjectEventByLocalIdAndMap(
            ((localId) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removeobjectat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut objectId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        RemoveObjectEventByLocalIdAndMap(((objectId) as u8), mapNum, mapGroup);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addobject(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut objectId: u16 = VarGet(ScriptReadHalfword(ctx));
        TrySpawnObjectEvent(
            ((objectId) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addobjectat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut objectId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        TrySpawnObjectEvent(((objectId) as u8), mapNum, mapGroup);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectxy(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        TryMoveObjectEventToMapCoords(
            ((localId) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            ((x) as i16),
            ((y) as i16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectxyperm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetObjEventTemplateCoords(((localId) as u8), ((x) as i16), ((y) as i16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_copyobjectxytoperm(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        TryOverrideObjectEventTemplateCoords(
            ((localId) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showobjectat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        SetObjectInvisibility(((localId) as u8), mapNum, mapGroup, 0u8);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hideobjectat(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        SetObjectInvisibility(((localId) as u8), mapNum, mapGroup, 1u8);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectsubpriority(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut priority: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        SetObjectSubpriority(
            ((localId) as u8),
            mapNum,
            mapGroup,
            ((((priority) as i32).wrapping_add(83i32)) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_resetobjectsubpriority(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ResetObjectSubpriority(((localId) as u8), mapNum, mapGroup);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_faceplayer(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        if (crate::c::bf_read(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
            ))
            .wrapping_add(0),
            0,
            1,
            false,
        ) as u32)
            != 0
        {
            ObjectEventFaceOppositeDirection(
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
                ),
                GetPlayerFacingDirection(),
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_turnobject(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut direction: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ObjectEventTurnByLocalIdAndMap(
            ((localId) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            direction,
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setobjectmovementtype(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut localId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut movementType: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        SetObjEventTemplateMovementType(((localId) as u8), movementType);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_createvobject(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut graphicsId: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut virtualObjId: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u32 = ((VarGet(ScriptReadHalfword(ctx))) as u32);
        let mut elevation: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut direction: u8 = ({
            let __p15 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t16 = (__p15).read();
            (__p15).write(((__p15).read()).wrapping_offset(1));
            __t16
        })
        .read();
        CreateVirtualObject(
            graphicsId,
            virtualObjId,
            ((x) as i16),
            ((y) as i16),
            elevation,
            direction,
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_turnvobject(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut virtualObjId: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut direction: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        TurnVirtualObject(virtualObjId, direction);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_lockall(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        if (IsOverworldLinkActive()) != 0 {
            return 0u8;
        } else {
            FreezeObjects_WaitForPlayer();
            SetupNativeScript(ctx, Some(IsFreezePlayerFinished));
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_lock(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        if (IsOverworldLinkActive()) != 0 {
            return 0u8;
        } else {
            if (crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(0),
                0,
                1,
                false,
            ) as u32)
                != 0
            {
                FreezeObjects_WaitForPlayerAndSelected();
                SetupNativeScript(ctx, Some(IsFreezeSelectedObjectAndPlayerFinished));
            } else {
                FreezeObjects_WaitForPlayer();
                SetupNativeScript(ctx, Some(IsFreezePlayerFinished));
            }
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_releaseall(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut playerObjectId: u8 = 0u8;
        HideFieldMessageBox();
        playerObjectId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
        ObjectEventClearHeldMovementIfFinished(
            ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((playerObjectId) as i32) as isize * 36),
        );
        ScriptMovement_UnfreezeObjectEvents();
        UnfreezeObjectEvents();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_release(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut playerObjectId: u8 = 0u8;
        HideFieldMessageBox();
        if (crate::c::bf_read(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
            ))
            .wrapping_add(0),
            0,
            1,
            false,
        ) as u32)
            != 0
        {
            ObjectEventClearHeldMovementIfFinished(
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
                ),
            );
        }
        playerObjectId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
        ObjectEventClearHeldMovementIfFinished(
            ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((playerObjectId) as i32) as isize * 36),
        );
        ScriptMovement_UnfreezeObjectEvents();
        UnfreezeObjectEvents();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_message(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut msg: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        if ((msg) as usize) == 0usize {
            msg = (((((ctx).wrapping_add(100)).cast::<u32>()).read()) as usize as *mut u8);
        }
        ShowFieldMessage(msg);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokenavcall(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut msg: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        if ((msg) as usize) == 0usize {
            msg = (((((ctx).wrapping_add(100)).cast::<u32>()).read()) as usize as *mut u8);
        }
        ShowPokenavFieldMessage(msg);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_messageautoscroll(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut msg: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        if ((msg) as usize) == 0usize {
            msg = (((((ctx).wrapping_add(100)).cast::<u32>()).read()) as usize as *mut u8);
        }
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            2,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            3,
            1,
            (1u8) as i32,
        );
        ShowFieldAutoScrollMessage(msg);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_messageinstant(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut msg: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        if ((msg) as usize) == 0usize {
            msg = (((((ctx).wrapping_add(100)).cast::<u32>()).read()) as usize as *mut u8);
        }
        LoadMessageBoxAndBorderGfx();
        DrawDialogueFrame(0u8, 1u8);
        AddTextPrinterParameterized(0u8, 1u8, msg, 0u8, 1u8, 0u8, None);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmessage(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetupNativeScript(ctx, Some(IsFieldMessageBoxHidden));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_closemessage(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        HideFieldMessageBox();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitForAorBPress() -> u8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            return 1u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitbuttonpress(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetupNativeScript(ctx, Some(WaitForAorBPress));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_yesnobox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut left: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut top: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        if ((ScriptMenu_YesNo(left, top)) as i32) == 1i32 {
            ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_multichoice(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut left: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut top: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut multichoiceId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut ignoreBPress: u8 = ({
            let __p15 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t16 = (__p15).read();
            (__p15).write(((__p15).read()).wrapping_offset(1));
            __t16
        })
        .read();
        if ((ScriptMenu_Multichoice(left, top, multichoiceId, ignoreBPress)) as i32) == 1i32 {
            ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_multichoicedefault(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut left: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut top: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut multichoiceId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut defaultChoice: u8 = ({
            let __p15 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t16 = (__p15).read();
            (__p15).write(((__p15).read()).wrapping_offset(1));
            __t16
        })
        .read();
        let mut ignoreBPress: u8 = ({
            let __p19 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t20 = (__p19).read();
            (__p19).write(((__p19).read()).wrapping_offset(1));
            __t20
        })
        .read();
        if ((ScriptMenu_MultichoiceWithDefault(
            left,
            top,
            multichoiceId,
            ignoreBPress,
            defaultChoice,
        )) as i32)
            == 1i32
        {
            ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_drawbox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_multichoicegrid(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut left: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut top: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut multichoiceId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut numColumns: u8 = ({
            let __p15 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t16 = (__p15).read();
            (__p15).write(((__p15).read()).wrapping_offset(1));
            __t16
        })
        .read();
        let mut ignoreBPress: u8 = ({
            let __p19 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t20 = (__p19).read();
            (__p19).write(((__p19).read()).wrapping_offset(1));
            __t20
        })
        .read();
        if ((ScriptMenu_MultichoiceGrid(left, top, multichoiceId, ignoreBPress, numColumns)) as i32)
            == 1i32
        {
            ScriptContext_Stop();
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
pub unsafe extern "C" fn ScrCmd_erasebox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut left: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut top: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut right: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut bottom: u8 = ({
            let __p15 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t16 = (__p15).read();
            (__p15).write(((__p15).read()).wrapping_offset(1));
            __t16
        })
        .read();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_drawboxtext(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut left: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut top: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut multichoiceId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut ignoreBPress: u8 = ({
            let __p15 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t16 = (__p15).read();
            (__p15).write(((__p15).read()).wrapping_offset(1));
            __t16
        })
        .read();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showmonpic(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut x: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut y: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ScriptMenu_ShowPokemonPic(species, x, y);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hidemonpic(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut func: Option<unsafe extern "C" fn() -> u8> = ScriptMenu_HidePokemonPic();
        if core::mem::transmute::<_, usize>(func) == 0usize {
            return 0u8;
        }
        SetupNativeScript(ctx, func);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showcontestpainting(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut contestWinnerId: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if ((contestWinnerId) as i32) != 0i32 {
            SetContestWinnerForPainting(((contestWinnerId) as i32));
        }
        ShowContestPainting();
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_braillemessage(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut width: u8 = 0u8;
        let mut height: u8 = 0u8;
        let mut xWindow: u8 = 0u8;
        let mut yWindow: u8 = 0u8;
        let mut xText: u8 = 0u8;
        let mut yText: u8 = 0u8;
        let mut temp: u8 = 0u8;
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (ptr).wrapping_offset(6),
        );
        width = ((crate::c::div_u32(
            ((GetStringWidth(6u8, (&raw mut gStringVar4).cast::<u8>(), (-1i16))) as u32),
            8u32,
        )) as u8);
        if ((width) as i32) > 28i32 {
            width = 28u8;
        }
        {
            i = 0i32;
            height = 4u8;
            'l1: loop {
                if !((((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset((i) as isize)).read())
                    as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                        ({
                            let __t1 = i;
                            i = (i).wrapping_add(1);
                            __t1
                        }) as isize,
                    ))
                    .read()) as i32)
                        == 254i32
                    {
                        height = ((((height) as i32).wrapping_add(3i32)) as u8);
                    }
                }
            }
        }
        if ((height) as i32) > 18i32 {
            height = 18u8;
        }
        temp = ((((width) as i32).wrapping_add(2i32)) as u8);
        xWindow = ((crate::c::div_i32((30i32).wrapping_sub(((temp) as i32)), 2i32)) as u8);
        temp = ((((height) as i32).wrapping_add(2i32)) as u8);
        yText = ((crate::c::div_i32((20i32).wrapping_sub(((temp) as i32)), 2i32)) as u8);
        xText = xWindow;
        xWindow = ((((xWindow) as i32).wrapping_add(1i32)) as u8);
        yWindow = yText;
        yText = ((((yText) as i32).wrapping_add(2i32)) as u8);
        xText = (((((((xWindow) as i32).wrapping_sub(((xText) as i32))).wrapping_sub(1i32))
            .wrapping_mul(8i32))
        .wrapping_add(3i32)) as u8);
        yText = ((((((yText) as i32).wrapping_sub(((yWindow) as i32))).wrapping_sub(1i32))
            .wrapping_mul(8i32)) as u8);
        (&raw mut winTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(CreateWindowTemplate(
                0u8,
                xWindow,
                ((((yWindow) as i32).wrapping_add(1i32)) as u8),
                width,
                height,
                15u8,
                1u16,
            ));
        ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>())
            .write(((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8));
        LoadUserWindowBorderGfx(
            ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read(),
            532u16,
            224u8,
        );
        DrawStdWindowFrame(
            ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        PutWindowTilemap(((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read());
        FillWindowPixelBuffer(
            ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read(),
            17u8,
        );
        AddTextPrinterParameterized(
            ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read(),
            6u8,
            (&raw mut gStringVar4).cast::<u8>(),
            xText,
            yText,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read(),
            3u8,
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_closebraillemessage(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        CloseBrailleWindow();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vmessage(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut msg: u32 = ScriptReadWord(ctx);
        ShowFieldMessage(
            (((msg).wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
                as usize as *mut u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferspeciesname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 11))
            .cast::<u8>(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferleadmonspeciesname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut dest: *mut u8 = ((((&raw const sScriptStringVars)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((stringVarIndex) as i32) as isize))
        .read();
        let mut partyIndex: u8 = GetLeadMonIndex();
        let mut species: u32 = GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            11i32,
            core::ptr::null_mut(),
        );
        StringCopy(
            dest,
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 11))
            .cast::<u8>(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferpartymonnick(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            2i32,
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
        );
        StringGet_Nickname(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferitemname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        CopyItemName(
            itemId,
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferitemnameplural(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut itemId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut quantity: u16 = VarGet(ScriptReadHalfword(ctx));
        CopyItemNameHandlePlural(
            itemId,
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            ((quantity) as u32),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferdecorationname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut decorId: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            ((((&raw mut gDecorations).cast::<u8>())
                .wrapping_offset(((decorId) as i32) as isize * 32))
            .wrapping_add(1))
            .cast::<u8>(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffermovename(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut r#move: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffernumberstring(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut num: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut numDigits: u8 = ((CountDigits(((num) as i32))) as u8);
        ConvertIntToDecimalStringN(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            ((num) as i32),
            0i32,
            numDigits,
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferstdstring(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((index) as i32) as isize))
            .read(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffercontestname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut category: u16 = VarGet(ScriptReadHalfword(ctx));
        BufferContestName(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            ((category) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferstring(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut text: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            text,
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vbuffermessage(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = (((ScriptReadWord(ctx))
            .wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
            as usize as *mut u8);
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), ptr);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_vbufferstring(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut addr: u32 = ScriptReadWord(ctx);
        let mut src: *mut u8 = (((addr)
            .wrapping_sub(((&raw mut sAddressOffset).cast::<u8>().cast::<u32>()).read()))
            as usize as *mut u8);
        let mut dest: *mut u8 = ((((&raw const sScriptStringVars)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((stringVarIndex) as i32) as isize))
        .read();
        StringCopy(dest, src);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_bufferboxname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut boxId: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            GetBoxNamePtr(((boxId) as u8)),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_givemon(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut level: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut item: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut unkParam1: u32 = ScriptReadWord(ctx);
        let mut unkParam2: u32 = ScriptReadWord(ctx);
        let mut unkParam3: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((ScriptGiveMon(species, level, item, unkParam1, unkParam2, unkParam3)) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_giveegg(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((ScriptGiveEgg(species)) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmonmove(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut partyIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut slot: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut r#move: u16 = ScriptReadHalfword(ctx);
        ScriptSetMonMoveSlot(partyIndex, r#move, slot);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkpartymove(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut i: u8 = 0u8;
        let mut r#move: u16 = ScriptReadHalfword(ctx);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(6u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u16 = ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) as u16);
                    if !((species) != 0) {
                        break 'l1;
                    }
                    if (!((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        45i32,
                    )) != 0))
                        && (((MonKnowsMove(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            r#move,
                        )) as i32)
                            == 1i32)
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((i) as u16));
                        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(species);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addmoney(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut amount: u32 = ScriptReadWord(ctx);
        let mut ignore: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if !((ignore) != 0) {
            AddMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
                amount,
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removemoney(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut amount: u32 = ScriptReadWord(ctx);
        let mut ignore: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if !((ignore) != 0) {
            RemoveMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
                amount,
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkmoney(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut amount: u32 = ScriptReadWord(ctx);
        let mut ignore: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if !((ignore) != 0) {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                ((IsEnoughMoney(
                    (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1168)
                        .cast::<u32>(),
                    amount,
                )) as u16),
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showmoneybox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut y: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut ignore: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        if !((ignore) != 0) {
            DrawMoneyBox(
                ((GetMoney(
                    (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1168)
                        .cast::<u32>(),
                )) as i32),
                x,
                y,
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hidemoneybox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        HideMoneyBox();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_updatemoneybox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut y: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut ignore: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        if !((ignore) != 0) {
            ChangeAmountInMoneyBox(
                ((GetMoney(
                    (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1168)
                        .cast::<u32>(),
                )) as i32),
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showcoinsbox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut y: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        ShowCoinsWindow(((GetCoins()) as u32), x, y);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_hidecoinsbox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut y: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        HideCoinsWindow();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_updatecoinsbox(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut y: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        PrintCoinsString(((GetCoins()) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_trainerbattle(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((ctx).wrapping_add(8).cast::<*mut u8>()).write(BattleSetup_ConfigureTrainerBattle(
            ((ctx).wrapping_add(8).cast::<*mut u8>()).read(),
        ));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dotrainerbattle(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        BattleSetup_StartTrainerBattle();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotopostbattlescript(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((ctx).wrapping_add(8).cast::<*mut u8>()).write(BattleSetup_GetScriptAddrAfterBattle());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_gotobeatenscript(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((ctx).wrapping_add(8).cast::<*mut u8>()).write(BattleSetup_GetTrainerPostBattleScript());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checktrainerflag(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
        ((ctx).wrapping_add(2)).write(HasTrainerBeenFought(index));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_settrainerflag(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
        SetTrainerFlag(index);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_cleartrainerflag(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut index: u16 = VarGet(ScriptReadHalfword(ctx));
        ClearTrainerFlag(index);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setwildbattle(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut species: u16 = ScriptReadHalfword(ctx);
        let mut level: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut item: u16 = ScriptReadHalfword(ctx);
        CreateScriptedWildMon(species, level, item);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dowildbattle(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        BattleSetup_StartScriptedWildBattle();
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokemart(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        CreatePokemartMenu((ptr).cast::<u16>());
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokemartdecoration(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        CreateDecorationShop1Menu((ptr).cast::<u16>());
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_pokemartdecoration2(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u8 = ((ScriptReadWord(ctx)) as usize as *mut u8);
        CreateDecorationShop2Menu((ptr).cast::<u16>());
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playslotmachine(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut machineId: u8 = ((VarGet(ScriptReadHalfword(ctx))) as u8);
        PlaySlotMachine(machineId, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setberrytree(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut treeId: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut berry: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut growthStage: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        if ((berry) as i32) == 0i32 {
            PlantBerryTree(treeId, berry, growthStage, 0u8);
        } else {
            PlantBerryTree(treeId, berry, growthStage, 0u8);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_getpokenewsactive(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut newsKind: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((IsPokeNewsActive(((newsKind) as u8))) as u16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_choosecontestmon(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ChooseContestMon();
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_startcontest(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        StartContest();
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showcontestresults(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ShowContestResults();
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_contestlinktransfer(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ContestLinkTransfer(
            ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8),
        );
        ScriptContext_Stop();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_dofieldeffect(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut effectId: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut sFieldEffectScriptId).cast::<u8>().cast::<u16>()).write(effectId);
        FieldEffectStart(
            ((((&raw mut sFieldEffectScriptId).cast::<u8>().cast::<u16>()).read()) as u8),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setfieldeffectargument(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut argNum: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .wrapping_offset(((argNum) as i32) as isize))
        .write((((VarGet(ScriptReadHalfword(ctx))) as i16) as i32));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitForFieldEffectFinish() -> u8 {
    unsafe {
        if !((FieldEffectActiveListContains(
            ((((&raw mut sFieldEffectScriptId).cast::<u8>().cast::<u16>()).read()) as u8),
        )) != 0)
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
pub unsafe extern "C" fn ScrCmd_waitfieldeffect(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((&raw mut sFieldEffectScriptId).cast::<u8>().cast::<u16>())
            .write(VarGet(ScriptReadHalfword(ctx)));
        SetupNativeScript(ctx, Some(WaitForFieldEffectFinish));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setrespawn(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut healLocationId: u16 = VarGet(ScriptReadHalfword(ctx));
        SetLastHealLocationWarp(((healLocationId) as u8));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkplayergender(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_playmoncry(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut species: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut mode: u16 = VarGet(ScriptReadHalfword(ctx));
        PlayCry_Script(species, ((mode) as u8));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_waitmoncry(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetupNativeScript(ctx, Some(IsCryFinished));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmetatile(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut metatileId: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut isImpassable: u16 = VarGet(ScriptReadHalfword(ctx));
        x = ((((x) as i32).wrapping_add(7i32)) as u16);
        y = ((((y) as i32).wrapping_add(7i32)) as u16);
        if !((isImpassable) != 0) {
            MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), metatileId);
        } else {
            MapGridSetMetatileIdAt(
                ((x) as i32),
                ((y) as i32),
                ((((metatileId) as i32) | 3072i32) as u16),
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_opendoor(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        x = ((((x) as i32).wrapping_add(7i32)) as u16);
        y = ((((y) as i32).wrapping_add(7i32)) as u16);
        PlaySE(((GetDoorSoundEffect(((x) as u32), ((y) as u32))) as u16));
        FieldAnimateDoorOpen(((x) as u32), ((y) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_closedoor(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        x = ((((x) as i32).wrapping_add(7i32)) as u16);
        y = ((((y) as i32).wrapping_add(7i32)) as u16);
        FieldAnimateDoorClose(((x) as u32), ((y) as u32));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsDoorAnimationStopped() -> u8 {
    unsafe {
        if !((FieldIsDoorAnimationRunning()) != 0) {
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
pub unsafe extern "C" fn ScrCmd_waitdooranim(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        SetupNativeScript(ctx, Some(IsDoorAnimationStopped));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdooropen(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        x = ((((x) as i32).wrapping_add(7i32)) as u16);
        y = ((((y) as i32).wrapping_add(7i32)) as u16);
        FieldSetDoorOpened(((x) as u32), ((y) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setdoorclosed(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        x = ((((x) as i32).wrapping_add(7i32)) as u16);
        y = ((((y) as i32).wrapping_add(7i32)) as u16);
        FieldSetDoorClosed(((x) as u32), ((y) as u32));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addelevmenuitem(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut v3: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut v5: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut v7: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut v9: u16 = VarGet(ScriptReadHalfword(ctx));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_showelevmenu(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkcoins(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
        (ptr).write(GetCoins());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_addcoins(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut coins: u16 = VarGet(ScriptReadHalfword(ctx));
        if ((AddCoins(coins)) as i32) == 1i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_removecoins(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut coins: u16 = VarGet(ScriptReadHalfword(ctx));
        if ((RemoveCoins(coins)) as i32) == 1i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_moverotatingtileobjects(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut puzzleNumber: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>())
            .write(MoveRotatingTileObjects(((puzzleNumber) as u8)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_turnrotatingtileobjects(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        TurnRotatingTileObjects();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_initrotatingtilepuzzle(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut isTrickHouse: u16 = VarGet(ScriptReadHalfword(ctx));
        InitRotatingTilePuzzle(((isTrickHouse) as u8));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_freerotatingtilepuzzle(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        FreeRotatingTilePuzzle();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_selectapproachingtrainer(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        ((&raw mut gSelectedObjectEvent).cast::<u8>())
            .write(GetCurrentApproachingTrainerObjectEventId());
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_lockfortrainer(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        if (IsOverworldLinkActive()) != 0 {
            return 0u8;
        } else {
            if (crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSelectedObjectEvent).cast::<u8>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(0),
                0,
                1,
                false,
            ) as u32)
                != 0
            {
                FreezeForApproachingTrainers();
                SetupNativeScript(ctx, Some(IsFreezeObjectAndPlayerFinished));
            }
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmodernfatefulencounter(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut isModernFatefulEncounter: u8 = 1u8;
        let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
        SetMonData(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            80i32,
            &raw mut isModernFatefulEncounter,
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_checkmodernfatefulencounter(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                80i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_trywondercardscript(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut script: *mut u8 = GetSavedRamScriptIfValid();
        if !(script).is_null() {
            ((&raw mut gRamScriptRetAddr).cast::<u8>().cast::<*mut u8>())
                .write(((ctx).wrapping_add(8).cast::<*mut u8>()).read());
            ScriptJump(ctx, script);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpspinenter(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        SetSpinStartFacingDir(GetPlayerFacingDirection());
        DoSpinEnterWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_setmonmetlocation(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut location: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        if ((partyIndex) as i32) < 6i32 {
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                35i32,
                &raw mut location,
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CloseBrailleWindow() {
    unsafe {
        ClearStdWindowAndFrame(
            ((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
        );
        RemoveWindow(((&raw mut sBrailleWindowId).cast::<u8>().cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffertrainerclassname(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut trainerClassId: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            GetTrainerClassNameFromId(trainerClassId),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_buffertrainername(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut stringVarIndex: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut trainerClassId: u16 = VarGet(ScriptReadHalfword(ctx));
        StringCopy(
            ((((&raw const sScriptStringVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringVarIndex) as i32) as isize))
            .read(),
            GetTrainerNameFromId(trainerClassId),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMovingNpcId(npcId: u16) {
    unsafe {
        let mut npcId = npcId;
        ((&raw mut sMovingNpcId).cast::<u8>().cast::<u16>()).write(npcId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrCmd_warpwhitefade(ctx: *mut u8) -> u8 {
    unsafe {
        let mut ctx = ctx;
        let mut mapGroup: u8 = ({
            let __p3 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_offset(1));
            __t4
        })
        .read();
        let mut mapNum: u8 = ({
            let __p7 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_offset(1));
            __t8
        })
        .read();
        let mut warpId: u8 = ({
            let __p11 = (ctx).wrapping_add(8).cast::<*mut u8>();
            let __t12 = (__p11).read();
            (__p11).write(((__p11).read()).wrapping_offset(1));
            __t12
        })
        .read();
        let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
        let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
        SetWarpDestination(
            ((mapGroup) as i8),
            ((mapNum) as i8),
            ((warpId) as i8),
            ((x) as i8),
            ((y) as i8),
        );
        DoWhiteFadeWarp();
        ResetInitialPlayerAvatarState();
        return 1u8;
    }
}
