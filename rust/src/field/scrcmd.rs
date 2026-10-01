//! Translated from `src/scrcmd.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    unused_variables
)]

use crate::battle_setup::{
    BattleSetup_ConfigureTrainerBattle, BattleSetup_GetScriptAddrAfterBattle,
    BattleSetup_GetTrainerPostBattleScript, BattleSetup_StartScriptedWildBattle,
    BattleSetup_StartTrainerBattle, ClearTrainerFlag, HasTrainerBeenFought, SetTrainerFlag,
};
use crate::berry::PlantBerryTree;
#[allow(unused_imports)]
use crate::c::*;
use crate::clock::DoTimeBasedEvents;
use crate::coins::{
    AddCoins, GetCoins, HideCoinsWindow, PrintCoinsString, RemoveCoins, ShowCoinsWindow,
};
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gSpecialVar_ContestCategory;
use crate::contest_painting::SetContestWinnerForPainting;
use crate::contest_util::{
    ContestLinkTransfer, ShowContestPainting, ShowContestResults, StartContest,
};
use crate::decoration_inventory::{
    CheckHasDecoration, DecorationAdd, DecorationCheckSpace, DecorationRemove,
};
use crate::event_data::{FlagClear, FlagGet, FlagSet, GetVarPointer, VarGet};
use crate::event_object_lock::{
    FreezeForApproachingTrainers, FreezeObjects_WaitForPlayer,
    FreezeObjects_WaitForPlayerAndSelected, IsFreezeObjectAndPlayerFinished,
    IsFreezePlayerFinished, IsFreezeSelectedObjectAndPlayerFinished,
};
use crate::event_object_movement::{
    CreateVirtualObject, GetObjectEventIdByLocalIdAndMap, ObjectEventClearHeldMovementIfFinished,
    ObjectEventFaceOppositeDirection, ObjectEventTurnByLocalIdAndMap,
    RemoveObjectEventByLocalIdAndMap, ResetObjectSubpriority, SetObjectInvisibility,
    SetObjectSubpriority, TryMoveObjectEventToMapCoords, TryOverrideObjectEventTemplateCoords,
    TrySpawnObjectEvent, TurnVirtualObject, UnfreezeObjectEvents,
};
use crate::field_control_avatar::gSelectedObjectEvent;
use crate::field_door::{
    FieldAnimateDoorClose, FieldAnimateDoorOpen, FieldIsDoorAnimationRunning, FieldSetDoorClosed,
    FieldSetDoorOpened, GetDoorSoundEffect,
};
use crate::field_effect::{FieldEffectActiveListContains, FieldEffectStart, gFieldEffectArguments};
use crate::field_message_box::{HideFieldMessageBox, IsFieldMessageBoxHidden};
use crate::field_message_box::{
    ShowFieldAutoScrollMessage, ShowFieldMessage, ShowPokenavFieldMessage,
};
use crate::field_player_avatar::{
    GetPlayerFacingDirection, PlayerGetDestCoords, SetSpinStartFacingDir, gObjectEvents,
};
use crate::field_screen_effect::{
    AnimateFlash, DoDiveWarp, DoDoorWarp, DoFallWarp, DoMossdeepGymWarp, DoSpinEnterWarp,
    DoTeleportTileWarp, DoWarp, DoWhiteFadeWarp,
};
use crate::field_specials::GetLeadMonIndex;
use crate::field_tasks::ActivatePerStepCallback;
use crate::field_weather::FadeScreen;
use crate::field_weather_effect::{
    DoCurrentWeather, SetSavedWeather, SetSavedWeatherFromCurrMapHeader,
};
use crate::fieldmap::MapGridSetMetatileIdAt;
use crate::item::{
    AddBagItem, AddPCItem, CheckBagHasItem, CheckBagHasSpace, CheckPCHasItem, CopyItemName,
    CopyItemNameHandlePlural, GetPocketByItemId, RemoveBagItem,
};
use crate::lilycove_lady::BufferContestName;
use crate::menu::{
    ClearStdWindowAndFrame, CreateWindowTemplate, DrawDialogueFrame, DrawStdWindowFrame,
    LoadMessageBoxAndBorderGfx,
};
use crate::money::{
    AddMoney, ChangeAmountInMoneyBox, DrawMoneyBox, GetMoney, HideMoneyBox, IsEnoughMoney,
    RemoveMoney,
};
use crate::mystery_event_script::SetMysteryEventScriptStatus;
use crate::overworld::{
    CB2_ReturnToFieldContinueScriptPlayMapMusic, IncrementGameStat, IsOverworldLinkActive,
    Overworld_ChangeMusicTo, Overworld_ChangeMusicToDefault, Overworld_SetSavedMusic,
    ResetInitialPlayerAvatarState, SetCurrentMapLayout, SetDynamicWarpWithCoords, SetEscapeWarp,
    SetFixedDiveWarp, SetFixedHoleWarp, SetFlashLevel, SetLastHealLocationWarp,
    SetObjEventTemplateCoords, SetObjEventTemplateMovementType, SetWarpDestination,
    SetWarpDestinationToFixedHoleWarp,
};
use crate::palette::gPaletteFade;
use crate::palette::{gPaletteDecompressionBuffer, gPlttBufferUnfaded};
use crate::party_menu::{ChooseContestMon, MonKnowsMove};
use crate::pokemon::{
    CalculatePlayerPartyCount, GetMonData2, GetMonData3, GetTrainerClassNameFromId,
    GetTrainerNameFromId, SetMonData, gPlayerParty,
};
use crate::pokemon_storage_system::GetBoxNamePtr;
use crate::random::Random;
use crate::rotating_tile_puzzle::{
    FreeRotatingTilePuzzle, InitRotatingTilePuzzle, MoveRotatingTileObjects,
    TurnRotatingTileObjects,
};
use crate::rtc::{RtcCalcLocalTime, RtcInitLocalTimeOffset, gLocalTime};
use crate::script::{ClearRamScript, GetSavedRamScriptIfValid, ScriptContext_Stop};
use crate::script_menu::{
    ScriptMenu_Multichoice, ScriptMenu_MultichoiceGrid, ScriptMenu_MultichoiceWithDefault,
    ScriptMenu_ShowPokemonPic, ScriptMenu_YesNo,
};
use crate::script_movement::ScriptMovement_StartObjectMovementScript;
use crate::script_movement::{
    ScriptMovement_IsObjectMovementFinished, ScriptMovement_UnfreezeObjectEvents,
};
use crate::script_pokemon_util::{
    CreateScriptedWildMon, ScriptGiveEgg, ScriptGiveMon, ScriptSetMonMoveSlot,
};
use crate::shop::{CreateDecorationShop1Menu, CreateDecorationShop2Menu, CreatePokemartMenu};
use crate::slot_machine::PlaySlotMachine;
use crate::sound::{
    FadeInBGM, FadeOutBGMTemporarily, IsBGMPausedOrStopped, IsCryFinished, IsFanfareTaskInactive,
    IsSEPlaying, PlayCry_Script, PlayFanfare, PlayNewMapMusic, PlaySE,
};
use crate::string_util::StringGet_Nickname;
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringExpandPlaceholders};
use crate::text::GetStringWidth;
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trainer_see::GetCurrentApproachingTrainerObjectEventId;
use crate::tv::{CountDigits, IsPokeNewsActive};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `ScriptCall` with this module's view of its types.
#[inline]
unsafe fn ScriptCall(a0: *mut ScriptContext, a1: *mut u8) {
    unsafe {
        crate::script::ScriptCall(a0 as _, a1 as _);
    }
}
/// `ScriptJump` with this module's view of its types.
#[inline]
unsafe fn ScriptJump(a0: *mut ScriptContext, a1: *mut u8) {
    unsafe {
        crate::script::ScriptJump(a0 as _, a1 as _);
    }
}
/// `ScriptReadHalfword` with this module's view of its types.
#[inline]
unsafe fn ScriptReadHalfword(a0: *mut ScriptContext) -> u16 {
    unsafe { crate::script::ScriptReadHalfword(a0 as _) }
}
/// `ScriptReadWord` with this module's view of its types.
#[inline]
unsafe fn ScriptReadWord(a0: *mut ScriptContext) -> u32 {
    unsafe { crate::script::ScriptReadWord(a0 as _) }
}
/// `ScriptReturn` with this module's view of its types.
#[inline]
unsafe fn ScriptReturn(a0: *mut ScriptContext) {
    unsafe {
        crate::script::ScriptReturn(a0 as _);
    }
}
/// `SetupNativeScript` with this module's view of its types.
#[inline]
unsafe fn SetupNativeScript(a0: *mut ScriptContext, a1: Option<unsafe fn() -> u8>) {
    unsafe {
        crate::script::SetupNativeScript(a0 as _, a1);
    }
}
/// `StopScript` with this module's view of its types.
#[inline]
unsafe fn StopScript(a0: *mut ScriptContext) {
    unsafe {
        crate::script::StopScript(a0 as _);
    }
}
// Data tables (translate with cdata.py): gNullScriptPtr sScriptConditionTable sScriptStringVars

static sScriptConditionTable: Table<CArray<CArray<u8, 3>, 6>> =
    Table((&raw const crate::data::scrcmd::sScriptConditionTable).cast());
static sScriptStringVars: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::scrcmd::sScriptStringVars).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRamScriptRetAddr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sAddressOffset: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sPauseCounter: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sMovingNpcId: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sMovingNpcMapGroup: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sMovingNpcMapNum: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sFieldEffectScriptId: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sBrailleWindowId: crate::global::Global<u8> = crate::global::Global::new(0);

use crate::agb_main::gMain;
use crate::ffi::{
    gSpecialVar_0x8000, gSpecialVar_0x8001, gSpecialVar_0x8002, gSpecialVar_0x8004,
    gSpecialVar_Result,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::script_menu::ScriptMenu_HidePokemonPic;
use crate::string_util::gStringVar4;
/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

#[unsafe(no_mangle)]
pub fn ScrCmd_nop(ctx: *mut ScriptContext) -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub fn ScrCmd_nop1(ctx: *mut ScriptContext) -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_end(ctx: *mut ScriptContext) -> u8 {
    StopScript(ctx);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_gotonative(ctx: *mut ScriptContext) -> u8 {
    let addr: Option<unsafe fn() -> u8> =
        core::mem::transmute::<usize, Option<unsafe fn() -> u8>>(ScriptReadWord(ctx) as usize);
    SetupNativeScript(ctx, addr);
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_special(ctx: *mut ScriptContext) -> u8 {
    let index: u16 = ScriptReadHalfword(ctx);
    (*crate::asmdata::gSpecials.cast::<CArray<Option<unsafe fn() -> u16>, 0>>())[index]
        .unwrap_unchecked()();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_specialvar(ctx: *mut ScriptContext) -> u8 {
    let var: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *var = (*crate::asmdata::gSpecials.cast::<CArray<Option<unsafe fn() -> u16>, 0>>())
        [ScriptReadHalfword(ctx)]
    .unwrap_unchecked()();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_callnative(ctx: *mut ScriptContext) -> u8 {
    let func: Option<unsafe fn()> =
        core::mem::transmute::<usize, Option<unsafe fn()>>(ScriptReadWord(ctx) as usize);
    func.unwrap_unchecked()();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitstate(ctx: *mut ScriptContext) -> u8 {
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_goto(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    ScriptJump(ctx, ptr);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_return(ctx: *mut ScriptContext) -> u8 {
    ScriptReturn(ctx);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_call(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    ScriptCall(ctx, ptr);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_goto_if(ctx: *mut ScriptContext) -> u8 {
    let condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptJump(ctx, ptr);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_call_if(ctx: *mut ScriptContext) -> u8 {
    let condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptCall(ctx, ptr);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setvaddress(ctx: *mut ScriptContext) -> u8 {
    let addr1: u32 = (*ctx).scriptPtr as usize as u32 - 1;
    let addr2: u32 = ScriptReadWord(ctx);
    sAddressOffset.set(addr2 - addr1);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vgoto(ctx: *mut ScriptContext) -> u8 {
    let addr: u32 = ScriptReadWord(ctx);
    ScriptJump(ctx, (addr - sAddressOffset.get()) as usize as *mut u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vcall(ctx: *mut ScriptContext) -> u8 {
    let addr: u32 = ScriptReadWord(ctx);
    ScriptCall(ctx, (addr - sAddressOffset.get()) as usize as *mut u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vgoto_if(ctx: *mut ScriptContext) -> u8 {
    let condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let ptr: *mut u8 = (ScriptReadWord(ctx) - sAddressOffset.get()) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptJump(ctx, ptr);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vcall_if(ctx: *mut ScriptContext) -> u8 {
    let condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let ptr: *mut u8 = (ScriptReadWord(ctx) - sAddressOffset.get()) as usize as *mut u8;
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        ScriptCall(ctx, ptr);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_gotostd(ctx: *mut ScriptContext) -> u8 {
    let index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let ptr: *mut *mut u8 = &raw mut (*crate::asmdata::gStdScripts
        .cast::<CArray<*mut u8, 0>>()
        .cast_mut())[index];
    if ptr
        < (*crate::asmdata::gStdScripts_End
            .cast::<CArray<*mut u8, 0>>()
            .cast_mut())
        .as_mut_ptr()
    {
        ScriptJump(ctx, *ptr);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_callstd(ctx: *mut ScriptContext) -> u8 {
    let index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let ptr: *mut *mut u8 = &raw mut (*crate::asmdata::gStdScripts
        .cast::<CArray<*mut u8, 0>>()
        .cast_mut())[index];
    if ptr
        < (*crate::asmdata::gStdScripts_End
            .cast::<CArray<*mut u8, 0>>()
            .cast_mut())
        .as_mut_ptr()
    {
        ScriptCall(ctx, *ptr);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_gotostd_if(ctx: *mut ScriptContext) -> u8 {
    let condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let index: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        let ptr: *mut *mut u8 = &raw mut (*crate::asmdata::gStdScripts
            .cast::<CArray<*mut u8, 0>>()
            .cast_mut())[index];
        if ptr
            < (*crate::asmdata::gStdScripts_End
                .cast::<CArray<*mut u8, 0>>()
                .cast_mut())
            .as_mut_ptr()
        {
            ScriptJump(ctx, *ptr);
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_callstd_if(ctx: *mut ScriptContext) -> u8 {
    let condition: i32 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    }) as i32;
    let index: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    if sScriptConditionTable[condition][(*ctx).comparisonResult] == 1 {
        let ptr: *mut *mut u8 = &raw mut (*crate::asmdata::gStdScripts
            .cast::<CArray<*mut u8, 0>>()
            .cast_mut())[index];
        if ptr
            < (*crate::asmdata::gStdScripts_End
                .cast::<CArray<*mut u8, 0>>()
                .cast_mut())
            .as_mut_ptr()
        {
            ScriptCall(ctx, *ptr);
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_returnram(ctx: *mut ScriptContext) -> u8 {
    ScriptJump(ctx, gRamScriptRetAddr);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_endram(ctx: *mut ScriptContext) -> u8 {
    ClearRamScript();
    StopScript(ctx);
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setmysteryeventstatus(ctx: *mut ScriptContext) -> u8 {
    let status: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    SetMysteryEventScriptStatus(status as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_loadword(ctx: *mut ScriptContext) -> u8 {
    let index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).data[index] = ScriptReadWord(ctx);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_loadbytefromptr(ctx: *mut ScriptContext) -> u8 {
    let index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).data[index] = *(ScriptReadWord(ctx) as usize as *mut u8) as u32;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setptr(ctx: *mut ScriptContext) -> u8 {
    let value: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    *(ScriptReadWord(ctx) as usize as *mut u8) = value;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_loadbyte(ctx: *mut ScriptContext) -> u8 {
    let index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).data[index] = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    }) as u32;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setptrbyte(ctx: *mut ScriptContext) -> u8 {
    let index: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    *(ScriptReadWord(ctx) as usize as *mut u8) = (*ctx).data[index] as u8;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_copylocal(ctx: *mut ScriptContext) -> u8 {
    let destIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let srcIndex: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    (*ctx).data[destIndex] = (*ctx).data[srcIndex];
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_copybyte(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    *ptr = *(ScriptReadWord(ctx) as usize as *mut u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setvar(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = ScriptReadHalfword(ctx);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_copyvar(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = *GetVarPointer(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setorcopyvar(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = VarGet(ScriptReadHalfword(ctx));
    FALSE
}
pub unsafe fn Compare(a: u16, b: u16) -> u8 {
    if a < b {
        return 0;
    }
    if a == b {
        return 1;
    }
    2
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_local_to_local(ctx: *mut ScriptContext) -> u8 {
    let value1: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    let value2: u8 = (*ctx).data[*({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    })] as u8;
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_local_to_value(ctx: *mut ScriptContext) -> u8 {
    let value1: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    let value2: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_local_to_ptr(ctx: *mut ScriptContext) -> u8 {
    let value1: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    let value2: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_ptr_to_local(ctx: *mut ScriptContext) -> u8 {
    let value1: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    let value2: u8 = (*ctx).data[*({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    })] as u8;
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_ptr_to_value(ctx: *mut ScriptContext) -> u8 {
    let value1: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    let value2: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_ptr_to_ptr(ctx: *mut ScriptContext) -> u8 {
    let value1: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    let value2: u8 = *(ScriptReadWord(ctx) as usize as *mut u8);
    (*ctx).comparisonResult = Compare(value1 as u16, value2 as u16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_var_to_value(ctx: *mut ScriptContext) -> u8 {
    let value1: u16 = *GetVarPointer(ScriptReadHalfword(ctx));
    let value2: u16 = ScriptReadHalfword(ctx);
    (*ctx).comparisonResult = Compare(value1, value2);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_compare_var_to_var(ctx: *mut ScriptContext) -> u8 {
    let ptr1: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    let ptr2: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    (*ctx).comparisonResult = Compare(*ptr1, *ptr2);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addvar(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr += ScriptReadHalfword(ctx);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_subvar(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr -= VarGet(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_random(ctx: *mut ScriptContext) -> u8 {
    let max: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = rem_i32(Random() as i32, max as i32) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_additem(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = AddBagItem(itemId, quantity as u8 as u16) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_removeitem(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = RemoveBagItem(itemId, quantity as u8 as u16) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkitemspace(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = CheckBagHasSpace(itemId, quantity as u8 as u16) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkitem(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = CheckBagHasItem(itemId, quantity as u8 as u16) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkitemtype(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = GetPocketByItemId(itemId) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addpcitem(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = AddPCItem(itemId, quantity) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkpcitem(ctx: *mut ScriptContext) -> u8 {
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = CheckPCHasItem(itemId, quantity) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_adddecoration(ctx: *mut ScriptContext) -> u8 {
    let decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = DecorationAdd(decorId as u8) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_removedecoration(ctx: *mut ScriptContext) -> u8 {
    let decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = DecorationRemove(decorId as u8) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkdecorspace(ctx: *mut ScriptContext) -> u8 {
    let decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = DecorationCheckSpace(decorId as u8) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkdecor(ctx: *mut ScriptContext) -> u8 {
    let decorId: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    gSpecialVar_Result = CheckHasDecoration(decorId as u8) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setflag(ctx: *mut ScriptContext) -> u8 {
    FlagSet(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_clearflag(ctx: *mut ScriptContext) -> u8 {
    FlagClear(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkflag(ctx: *mut ScriptContext) -> u8 {
    (*ctx).comparisonResult = FlagGet(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_incrementgamestat(ctx: *mut ScriptContext) -> u8 {
    IncrementGameStat(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_animateflash(ctx: *mut ScriptContext) -> u8 {
    AnimateFlash(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
    );
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setflashlevel(ctx: *mut ScriptContext) -> u8 {
    SetFlashLevel(VarGet(ScriptReadHalfword(ctx)) as i32);
    FALSE
}
pub(crate) unsafe fn IsPaletteNotActive() -> u8 {
    if gPaletteFade.active() == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadescreen(ctx: *mut ScriptContext) -> u8 {
    FadeScreen(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
        0,
    );
    SetupNativeScript(ctx, Some(IsPaletteNotActive));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadescreenspeed(ctx: *mut ScriptContext) -> u8 {
    let mode: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let speed: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    FadeScreen(mode, speed as i8);
    SetupNativeScript(ctx, Some(IsPaletteNotActive));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadescreenswapbuffers(ctx: *mut ScriptContext) -> u8 {
    let mode: u8 = *({
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
    TRUE
}
pub(crate) unsafe fn RunPauseTimer() -> u8 {
    if ({
        sPauseCounter.set(sPauseCounter.get() - 1);
        sPauseCounter.get()
    }) == 0
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
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_delay(ctx: *mut ScriptContext) -> u8 {
    sPauseCounter.set(ScriptReadHalfword(ctx));
    SetupNativeScript(ctx, Some(RunPauseTimer));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_initclock(ctx: *mut ScriptContext) -> u8 {
    let hour: u8 = VarGet(ScriptReadHalfword(ctx)) as u8;
    let minute: u8 = VarGet(ScriptReadHalfword(ctx)) as u8;
    RtcInitLocalTimeOffset(hour as i32, minute as i32);
    FALSE
}
#[unsafe(no_mangle)]
pub fn ScrCmd_dotimebasedevents(ctx: *mut ScriptContext) -> u8 {
    DoTimeBasedEvents();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_gettime(ctx: *mut ScriptContext) -> u8 {
    RtcCalcLocalTime();
    gSpecialVar_0x8000 = gLocalTime.hours as u16;
    gSpecialVar_0x8001 = gLocalTime.minutes as u16;
    gSpecialVar_0x8002 = gLocalTime.seconds as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setweather(ctx: *mut ScriptContext) -> u8 {
    let weather: u16 = VarGet(ScriptReadHalfword(ctx));
    SetSavedWeather(weather as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_resetweather(ctx: *mut ScriptContext) -> u8 {
    SetSavedWeatherFromCurrMapHeader();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_doweather(ctx: *mut ScriptContext) -> u8 {
    DoCurrentWeather();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setstepcallback(ctx: *mut ScriptContext) -> u8 {
    ActivatePerStepCallback(
        *({
            let t2 = (*ctx).scriptPtr;
            (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
            t2
        }),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setmaplayoutindex(ctx: *mut ScriptContext) -> u8 {
    let value: u16 = VarGet(ScriptReadHalfword(ctx));
    SetCurrentMapLayout(value);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warp(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warpsilent(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoDiveWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warpdoor(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoDoorWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warphole(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
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
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warpteleport(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoTeleportTileWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warpmossdeepgym(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoMossdeepGymWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setwarp(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setdynamicwarp(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetDynamicWarpWithCoords(
        0,
        mapGroup as i8,
        mapNum as i8,
        warpId as i8,
        x as i8,
        y as i8,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setdivewarp(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetFixedDiveWarp(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setholewarp(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetFixedHoleWarp(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setescapewarp(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetEscapeWarp(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_getplayerxy(ctx: *mut ScriptContext) -> u8 {
    let pX: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    let pY: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *pX = (*gSaveBlock1Ptr).pos.x as u16;
    *pY = (*gSaveBlock1Ptr).pos.y as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_getpartysize(ctx: *mut ScriptContext) -> u8 {
    gSpecialVar_Result = CalculatePlayerPartyCount() as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_playse(ctx: *mut ScriptContext) -> u8 {
    PlaySE(ScriptReadHalfword(ctx));
    FALSE
}
pub(crate) unsafe fn WaitForSoundEffectFinish() -> u8 {
    if IsSEPlaying() == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitse(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(WaitForSoundEffectFinish));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_playfanfare(ctx: *mut ScriptContext) -> u8 {
    PlayFanfare(ScriptReadHalfword(ctx));
    FALSE
}
pub(crate) unsafe fn WaitForFanfareFinish() -> u8 {
    IsFanfareTaskInactive()
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitfanfare(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(WaitForFanfareFinish));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_playbgm(ctx: *mut ScriptContext) -> u8 {
    let songId: u16 = ScriptReadHalfword(ctx);
    let save: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if save == TRUE {
        Overworld_SetSavedMusic(songId);
    }
    PlayNewMapMusic(songId);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_savebgm(ctx: *mut ScriptContext) -> u8 {
    Overworld_SetSavedMusic(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadedefaultbgm(ctx: *mut ScriptContext) -> u8 {
    Overworld_ChangeMusicToDefault();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadenewbgm(ctx: *mut ScriptContext) -> u8 {
    Overworld_ChangeMusicTo(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadeoutbgm(ctx: *mut ScriptContext) -> u8 {
    let speed: u8 = *({
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
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_fadeinbgm(ctx: *mut ScriptContext) -> u8 {
    let speed: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if speed != 0 {
        FadeInBGM(4 * speed);
    } else {
        FadeInBGM(4);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_applymovement(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let movementScript: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    ScriptMovement_StartObjectMovementScript(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        movementScript as *mut u8,
    );
    sMovingNpcId.set(localId);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_applymovementat(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let movementScript: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
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
    sMovingNpcId.set(localId);
    FALSE
}
pub(crate) unsafe fn WaitForMovementFinish() -> u8 {
    ScriptMovement_IsObjectMovementFinished(
        sMovingNpcId.get() as u8,
        sMovingNpcMapNum.get() as u8,
        sMovingNpcMapGroup.get() as u8,
    )
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitmovement(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    if localId != LOCALID_NONE as u16 {
        sMovingNpcId.set(localId);
    }
    sMovingNpcMapGroup.set((*gSaveBlock1Ptr).location.mapGroup as u16);
    sMovingNpcMapNum.set((*gSaveBlock1Ptr).location.mapNum as u16);
    SetupNativeScript(ctx, Some(WaitForMovementFinish));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitmovementat(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    if localId != LOCALID_NONE as u16 {
        sMovingNpcId.set(localId);
    }
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    sMovingNpcMapGroup.set(mapGroup as u16);
    sMovingNpcMapNum.set(mapNum as u16);
    SetupNativeScript(ctx, Some(WaitForMovementFinish));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_removeobject(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    RemoveObjectEventByLocalIdAndMap(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_removeobjectat(ctx: *mut ScriptContext) -> u8 {
    let objectId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    RemoveObjectEventByLocalIdAndMap(objectId as u8, mapNum, mapGroup);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addobject(ctx: *mut ScriptContext) -> u8 {
    let objectId: u16 = VarGet(ScriptReadHalfword(ctx));
    TrySpawnObjectEvent(
        objectId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addobjectat(ctx: *mut ScriptContext) -> u8 {
    let objectId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    TrySpawnObjectEvent(objectId as u8, mapNum, mapGroup);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setobjectxy(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    TryMoveObjectEventToMapCoords(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        x as i16,
        y as i16,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setobjectxyperm(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetObjEventTemplateCoords(localId as u8, x as i16, y as i16);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_copyobjectxytoperm(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    TryOverrideObjectEventTemplateCoords(
        localId as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_showobjectat(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    SetObjectInvisibility(localId as u8, mapNum, mapGroup, FALSE);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_hideobjectat(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    SetObjectInvisibility(localId as u8, mapNum, mapGroup, TRUE);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setobjectsubpriority(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let priority: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    SetObjectSubpriority(localId as u8, mapNum, mapGroup, priority + 83);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_resetobjectsubpriority(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ResetObjectSubpriority(localId as u8, mapNum, mapGroup);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_faceplayer(ctx: *mut ScriptContext) -> u8 {
    if gObjectEvents[gSelectedObjectEvent].active() != 0 {
        ObjectEventFaceOppositeDirection(
            &raw mut gObjectEvents[gSelectedObjectEvent],
            GetPlayerFacingDirection(),
        );
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_turnobject(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let direction: u8 = *({
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
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setobjectmovementtype(ctx: *mut ScriptContext) -> u8 {
    let localId: u16 = VarGet(ScriptReadHalfword(ctx));
    let movementType: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    SetObjEventTemplateMovementType(localId as u8, movementType);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_createvobject(ctx: *mut ScriptContext) -> u8 {
    let graphicsId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let virtualObjId: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u32 = VarGet(ScriptReadHalfword(ctx)) as u32;
    let elevation: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let direction: u8 = *({
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
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_turnvobject(ctx: *mut ScriptContext) -> u8 {
    let virtualObjId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let direction: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    TurnVirtualObject(virtualObjId, direction);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_lockall(ctx: *mut ScriptContext) -> u8 {
    if IsOverworldLinkActive() != 0 {
        return FALSE;
    } else {
        FreezeObjects_WaitForPlayer();
        SetupNativeScript(ctx, Some(IsFreezePlayerFinished));
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_lock(ctx: *mut ScriptContext) -> u8 {
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
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_releaseall(ctx: *mut ScriptContext) -> u8 {
    HideFieldMessageBox();
    let playerObjectId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
    ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[playerObjectId]);
    ScriptMovement_UnfreezeObjectEvents();
    UnfreezeObjectEvents();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_release(ctx: *mut ScriptContext) -> u8 {
    HideFieldMessageBox();
    if gObjectEvents[gSelectedObjectEvent].active() != 0 {
        ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[gSelectedObjectEvent]);
    }
    let playerObjectId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
    ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[playerObjectId]);
    ScriptMovement_UnfreezeObjectEvents();
    UnfreezeObjectEvents();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_message(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    ShowFieldMessage(msg);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_pokenavcall(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    ShowPokenavFieldMessage(msg);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_messageautoscroll(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_autoScroll(TRUE);
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_forceMidTextSpeed(TRUE);
    ShowFieldAutoScrollMessage(msg);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_messageinstant(ctx: *mut ScriptContext) -> u8 {
    let mut msg: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    if msg.is_null() {
        msg = (*ctx).data[0] as usize as *mut u8;
    }
    LoadMessageBoxAndBorderGfx();
    DrawDialogueFrame(0, TRUE);
    AddTextPrinterParameterized(0, FONT_NORMAL, msg, 0, 1, 0, None);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitmessage(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(IsFieldMessageBoxHidden));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_closemessage(ctx: *mut ScriptContext) -> u8 {
    HideFieldMessageBox();
    FALSE
}
pub(crate) unsafe fn WaitForAorBPress() -> u8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        return TRUE;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitbuttonpress(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(WaitForAorBPress));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_yesnobox(ctx: *mut ScriptContext) -> u8 {
    let left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let top: u8 = *({
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
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_multichoice(ctx: *mut ScriptContext) -> u8 {
    let left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let ignoreBPress: u8 = *({
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
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_multichoicedefault(ctx: *mut ScriptContext) -> u8 {
    let left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let defaultChoice: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    let ignoreBPress: u8 = *({
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
        0
    }
}
#[unsafe(no_mangle)]
pub fn ScrCmd_drawbox(ctx: *mut ScriptContext) -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_multichoicegrid(ctx: *mut ScriptContext) -> u8 {
    let left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let numColumns: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    let ignoreBPress: u8 = *({
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
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_erasebox(ctx: *mut ScriptContext) -> u8 {
    let left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let right: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let bottom: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_drawboxtext(ctx: *mut ScriptContext) -> u8 {
    let left: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let top: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let multichoiceId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let ignoreBPress: u8 = *({
        let t8 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t8
    });
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_showmonpic(ctx: *mut ScriptContext) -> u8 {
    let species: u16 = VarGet(ScriptReadHalfword(ctx));
    let x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ScriptMenu_ShowPokemonPic(species, x, y);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_hidemonpic(ctx: *mut ScriptContext) -> u8 {
    let func: Option<unsafe fn() -> u8> = ScriptMenu_HidePokemonPic();
    if func.is_none() {
        return FALSE;
    }
    SetupNativeScript(ctx, func);
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_showcontestpainting(ctx: *mut ScriptContext) -> u8 {
    let contestWinnerId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if contestWinnerId != CONTEST_WINNER_ARTIST {
        SetContestWinnerForPainting(contestWinnerId as i32);
    }
    ShowContestPainting();
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_braillemessage(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), ptr.at(6));
    let mut width: u8 = (GetStringWidth(FONT_BRAILLE, gStringVar4.as_mut_ptr(), -1) / 8) as u8;
    if width > 28 {
        width = 28;
    }
    let mut i: i32 = 0;
    let mut height: u8 = 4;
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
    let mut temp: u8 = width + 2;
    let mut xWindow: u8 = ((30 - temp as i32) / 2) as u8;
    temp = height + 2;
    let mut yText: u8 = ((20 - temp as i32) / 2) as u8;
    let mut xText: u8 = xWindow;
    xWindow += 1;
    let yWindow: u8 = yText;
    yText += 2;
    xText = (xWindow - xText - 1) * 8 + 3;
    yText = (yText - yWindow - 1) * 8;
    let mut winTemplate: WindowTemplate =
        CreateWindowTemplate(0, xWindow, yWindow + 1, width, height, 0xF, 0x1);
    sBrailleWindowId.set(AddWindow(&raw mut winTemplate) as u8);
    LoadUserWindowBorderGfx(sBrailleWindowId.get(), 0x214, 224);
    DrawStdWindowFrame(sBrailleWindowId.get(), FALSE);
    PutWindowTilemap(sBrailleWindowId.get());
    FillWindowPixelBuffer(sBrailleWindowId.get(), 17);
    AddTextPrinterParameterized(
        sBrailleWindowId.get(),
        FONT_BRAILLE,
        gStringVar4.as_mut_ptr(),
        xText,
        yText,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sBrailleWindowId.get(), COPYWIN_FULL);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_closebraillemessage(ctx: *mut ScriptContext) -> u8 {
    CloseBrailleWindow();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vmessage(ctx: *mut ScriptContext) -> u8 {
    let msg: u32 = ScriptReadWord(ctx);
    ShowFieldMessage((msg - sAddressOffset.get()) as usize as *mut u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferspeciesname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let species: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [species]
            .as_ptr()
            .cast_mut(),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferleadmonspeciesname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let dest: *mut u8 = sScriptStringVars[stringVarIndex];
    let partyIndex: u8 = GetLeadMonIndex();
    let species: u32 = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPECIES,
        null_mut(),
    );
    StringCopy(
        dest,
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [species]
            .as_ptr()
            .cast_mut(),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferpartymonnick(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_NICKNAME,
        sScriptStringVars[stringVarIndex],
    );
    StringGet_Nickname(sScriptStringVars[stringVarIndex]);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferitemname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    CopyItemName(itemId, sScriptStringVars[stringVarIndex]);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferitemnameplural(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let itemId: u16 = VarGet(ScriptReadHalfword(ctx));
    let quantity: u16 = VarGet(ScriptReadHalfword(ctx));
    CopyItemNameHandlePlural(itemId, sScriptStringVars[stringVarIndex], quantity as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferdecorationname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let decorId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        (*(&raw const crate::data::decoration::gDecorations).cast::<CArray<Decoration, 0>>())
            [decorId]
            .name
            .as_ptr()
            .cast_mut(),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_buffermovename(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let r#move: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [r#move]
            .as_ptr()
            .cast_mut(),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_buffernumberstring(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let num: u16 = VarGet(ScriptReadHalfword(ctx));
    let numDigits: u8 = CountDigits(num as i32) as u8;
    ConvertIntToDecimalStringN(
        sScriptStringVars[stringVarIndex],
        num as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        numDigits,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferstdstring(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let index: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        (*(&raw const crate::data::script_menu::gStdStrings).cast::<CArray<*mut u8, 0>>())[index],
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_buffercontestname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let category: u16 = VarGet(ScriptReadHalfword(ctx));
    BufferContestName(sScriptStringVars[stringVarIndex], category as u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferstring(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let text: *mut u8 = ScriptReadWord(ctx) as usize as *mut u8;
    StringCopy(sScriptStringVars[stringVarIndex], text);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vbuffermessage(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u8 = (ScriptReadWord(ctx) - sAddressOffset.get()) as usize as *mut u8;
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), ptr);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_vbufferstring(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let addr: u32 = ScriptReadWord(ctx);
    let src: *mut u8 = (addr - sAddressOffset.get()) as usize as *mut u8;
    let dest: *mut u8 = sScriptStringVars[stringVarIndex];
    StringCopy(dest, src);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_bufferboxname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let boxId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        GetBoxNamePtr(boxId as u8),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_givemon(ctx: *mut ScriptContext) -> u8 {
    let species: u16 = VarGet(ScriptReadHalfword(ctx));
    let level: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let item: u16 = VarGet(ScriptReadHalfword(ctx));
    let unkParam1: u32 = ScriptReadWord(ctx);
    let unkParam2: u32 = ScriptReadWord(ctx);
    let unkParam3: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    gSpecialVar_Result =
        ScriptGiveMon(species, level, item, unkParam1, unkParam2, unkParam3) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_giveegg(ctx: *mut ScriptContext) -> u8 {
    let species: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = ScriptGiveEgg(species) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setmonmove(ctx: *mut ScriptContext) -> u8 {
    let partyIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let slot: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let r#move: u16 = ScriptReadHalfword(ctx);
    ScriptSetMonMoveSlot(partyIndex, r#move, slot);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkpartymove(ctx: *mut ScriptContext) -> u8 {
    let r#move: u16 = ScriptReadHalfword(ctx);
    gSpecialVar_Result = PARTY_SIZE as u16;
    for i in 0..(PARTY_SIZE as u8) {
        let species: u16 =
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
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addmoney(ctx: *mut ScriptContext) -> u8 {
    let amount: u32 = ScriptReadWord(ctx);
    let ignore: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if ignore == 0 {
        AddMoney(&raw mut (*gSaveBlock1Ptr).money, amount);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_removemoney(ctx: *mut ScriptContext) -> u8 {
    let amount: u32 = ScriptReadWord(ctx);
    let ignore: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if ignore == 0 {
        RemoveMoney(&raw mut (*gSaveBlock1Ptr).money, amount);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkmoney(ctx: *mut ScriptContext) -> u8 {
    let amount: u32 = ScriptReadWord(ctx);
    let ignore: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    if ignore == 0 {
        gSpecialVar_Result = IsEnoughMoney(&raw mut (*gSaveBlock1Ptr).money, amount) as u16;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_showmoneybox(ctx: *mut ScriptContext) -> u8 {
    let x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let ignore: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    if ignore == 0 {
        DrawMoneyBox(GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32, x, y);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_hidemoneybox(ctx: *mut ScriptContext) -> u8 {
    HideMoneyBox();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_updatemoneybox(ctx: *mut ScriptContext) -> u8 {
    let x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let ignore: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    if ignore == 0 {
        ChangeAmountInMoneyBox(GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_showcoinsbox(ctx: *mut ScriptContext) -> u8 {
    let x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    ShowCoinsWindow(GetCoins() as u32, x, y);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_hidecoinsbox(ctx: *mut ScriptContext) -> u8 {
    let x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    HideCoinsWindow();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_updatecoinsbox(ctx: *mut ScriptContext) -> u8 {
    let x: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let y: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    PrintCoinsString(GetCoins() as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_trainerbattle(ctx: *mut ScriptContext) -> u8 {
    (*ctx).scriptPtr = BattleSetup_ConfigureTrainerBattle((*ctx).scriptPtr);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_dotrainerbattle(ctx: *mut ScriptContext) -> u8 {
    BattleSetup_StartTrainerBattle();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_gotopostbattlescript(ctx: *mut ScriptContext) -> u8 {
    (*ctx).scriptPtr = BattleSetup_GetScriptAddrAfterBattle();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_gotobeatenscript(ctx: *mut ScriptContext) -> u8 {
    (*ctx).scriptPtr = BattleSetup_GetTrainerPostBattleScript();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checktrainerflag(ctx: *mut ScriptContext) -> u8 {
    let index: u16 = VarGet(ScriptReadHalfword(ctx));
    (*ctx).comparisonResult = HasTrainerBeenFought(index);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_settrainerflag(ctx: *mut ScriptContext) -> u8 {
    let index: u16 = VarGet(ScriptReadHalfword(ctx));
    SetTrainerFlag(index);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_cleartrainerflag(ctx: *mut ScriptContext) -> u8 {
    let index: u16 = VarGet(ScriptReadHalfword(ctx));
    ClearTrainerFlag(index);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setwildbattle(ctx: *mut ScriptContext) -> u8 {
    let species: u16 = ScriptReadHalfword(ctx);
    let level: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let item: u16 = ScriptReadHalfword(ctx);
    CreateScriptedWildMon(species, level, item);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_dowildbattle(ctx: *mut ScriptContext) -> u8 {
    BattleSetup_StartScriptedWildBattle();
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_pokemart(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    CreatePokemartMenu(ptr as *mut u16);
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_pokemartdecoration(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    CreateDecorationShop1Menu(ptr as *mut u16);
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_pokemartdecoration2(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut c_void = ScriptReadWord(ctx) as usize as *mut c_void;
    CreateDecorationShop2Menu(ptr as *mut u16);
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_playslotmachine(ctx: *mut ScriptContext) -> u8 {
    let machineId: u8 = VarGet(ScriptReadHalfword(ctx)) as u8;
    PlaySlotMachine(machineId, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setberrytree(ctx: *mut ScriptContext) -> u8 {
    let treeId: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let berry: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let growthStage: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    if berry == 0 {
        PlantBerryTree(treeId, berry, growthStage, FALSE);
    } else {
        PlantBerryTree(treeId, berry, growthStage, FALSE);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_getpokenewsactive(ctx: *mut ScriptContext) -> u8 {
    let newsKind: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = IsPokeNewsActive(newsKind as u8) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_choosecontestmon(ctx: *mut ScriptContext) -> u8 {
    ChooseContestMon();
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_startcontest(ctx: *mut ScriptContext) -> u8 {
    StartContest();
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_showcontestresults(ctx: *mut ScriptContext) -> u8 {
    ShowContestResults();
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_contestlinktransfer(ctx: *mut ScriptContext) -> u8 {
    ContestLinkTransfer(gSpecialVar_ContestCategory as u8);
    ScriptContext_Stop();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_dofieldeffect(ctx: *mut ScriptContext) -> u8 {
    let effectId: u16 = VarGet(ScriptReadHalfword(ctx));
    sFieldEffectScriptId.set(effectId);
    FieldEffectStart(sFieldEffectScriptId.get() as u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setfieldeffectargument(ctx: *mut ScriptContext) -> u8 {
    let argNum: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    gFieldEffectArguments[argNum] = VarGet(ScriptReadHalfword(ctx)) as i16 as i32;
    FALSE
}
pub(crate) unsafe fn WaitForFieldEffectFinish() -> u8 {
    if FieldEffectActiveListContains(sFieldEffectScriptId.get() as u8) == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitfieldeffect(ctx: *mut ScriptContext) -> u8 {
    sFieldEffectScriptId.set(VarGet(ScriptReadHalfword(ctx)));
    SetupNativeScript(ctx, Some(WaitForFieldEffectFinish));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setrespawn(ctx: *mut ScriptContext) -> u8 {
    let healLocationId: u16 = VarGet(ScriptReadHalfword(ctx));
    SetLastHealLocationWarp(healLocationId as u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkplayergender(ctx: *mut ScriptContext) -> u8 {
    gSpecialVar_Result = (*gSaveBlock2Ptr).playerGender as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_playmoncry(ctx: *mut ScriptContext) -> u8 {
    let species: u16 = VarGet(ScriptReadHalfword(ctx));
    let mode: u16 = VarGet(ScriptReadHalfword(ctx));
    PlayCry_Script(species, mode as u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitmoncry(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(IsCryFinished));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setmetatile(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    let metatileId: u16 = VarGet(ScriptReadHalfword(ctx));
    let isImpassable: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    if isImpassable == 0 {
        MapGridSetMetatileIdAt(x as i32, y as i32, metatileId);
    } else {
        MapGridSetMetatileIdAt(x as i32, y as i32, metatileId | MAPGRID_IMPASSABLE);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_opendoor(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    PlaySE(GetDoorSoundEffect(x as u32, y as u32) as u16);
    FieldAnimateDoorOpen(x as u32, y as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_closedoor(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    FieldAnimateDoorClose(x as u32, y as u32);
    FALSE
}
pub(crate) unsafe fn IsDoorAnimationStopped() -> u8 {
    if FieldIsDoorAnimationRunning() == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_waitdooranim(ctx: *mut ScriptContext) -> u8 {
    SetupNativeScript(ctx, Some(IsDoorAnimationStopped));
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setdooropen(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    FieldSetDoorOpened(x as u32, y as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setdoorclosed(ctx: *mut ScriptContext) -> u8 {
    let mut x: u16 = VarGet(ScriptReadHalfword(ctx));
    let mut y: u16 = VarGet(ScriptReadHalfword(ctx));
    x += MAP_OFFSET as u16;
    y += MAP_OFFSET as u16;
    FieldSetDoorClosed(x as u32, y as u32);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addelevmenuitem(ctx: *mut ScriptContext) -> u8 {
    let v3: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let v5: u16 = VarGet(ScriptReadHalfword(ctx));
    let v7: u16 = VarGet(ScriptReadHalfword(ctx));
    let v9: u16 = VarGet(ScriptReadHalfword(ctx));
    FALSE
}
#[unsafe(no_mangle)]
pub fn ScrCmd_showelevmenu(ctx: *mut ScriptContext) -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkcoins(ctx: *mut ScriptContext) -> u8 {
    let ptr: *mut u16 = GetVarPointer(ScriptReadHalfword(ctx));
    *ptr = GetCoins();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_addcoins(ctx: *mut ScriptContext) -> u8 {
    let coins: u16 = VarGet(ScriptReadHalfword(ctx));
    if AddCoins(coins) == TRUE {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_removecoins(ctx: *mut ScriptContext) -> u8 {
    let coins: u16 = VarGet(ScriptReadHalfword(ctx));
    if RemoveCoins(coins) == TRUE {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_moverotatingtileobjects(ctx: *mut ScriptContext) -> u8 {
    let puzzleNumber: u16 = VarGet(ScriptReadHalfword(ctx));
    sMovingNpcId.set(MoveRotatingTileObjects(puzzleNumber as u8));
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_turnrotatingtileobjects(ctx: *mut ScriptContext) -> u8 {
    TurnRotatingTileObjects();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_initrotatingtilepuzzle(ctx: *mut ScriptContext) -> u8 {
    let isTrickHouse: u16 = VarGet(ScriptReadHalfword(ctx));
    InitRotatingTilePuzzle(isTrickHouse as u8);
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_freerotatingtilepuzzle(ctx: *mut ScriptContext) -> u8 {
    FreeRotatingTilePuzzle();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_selectapproachingtrainer(ctx: *mut ScriptContext) -> u8 {
    gSelectedObjectEvent = GetCurrentApproachingTrainerObjectEventId();
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_lockfortrainer(ctx: *mut ScriptContext) -> u8 {
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
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setmodernfatefulencounter(ctx: *mut ScriptContext) -> u8 {
    let mut isModernFatefulEncounter: u8 = TRUE;
    let partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    SetMonData(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        &raw mut isModernFatefulEncounter as *mut c_void,
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_checkmodernfatefulencounter(ctx: *mut ScriptContext) -> u8 {
    let partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
    gSpecialVar_Result = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        null_mut(),
    ) as u16;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_trywondercardscript(ctx: *mut ScriptContext) -> u8 {
    let script: *mut u8 = GetSavedRamScriptIfValid();
    if !script.is_null() {
        gRamScriptRetAddr = (*ctx).scriptPtr;
        ScriptJump(ctx, script);
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warpspinenter(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    SetSpinStartFacingDir(GetPlayerFacingDirection());
    DoSpinEnterWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_setmonmetlocation(ctx: *mut ScriptContext) -> u8 {
    let partyIndex: u16 = VarGet(ScriptReadHalfword(ctx));
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
    FALSE
}
unsafe fn CloseBrailleWindow() {
    ClearStdWindowAndFrame(sBrailleWindowId.get(), TRUE);
    RemoveWindow(sBrailleWindowId.get());
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_buffertrainerclassname(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let trainerClassId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        GetTrainerClassNameFromId(trainerClassId),
    );
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_buffertrainername(ctx: *mut ScriptContext) -> u8 {
    let stringVarIndex: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let trainerClassId: u16 = VarGet(ScriptReadHalfword(ctx));
    StringCopy(
        sScriptStringVars[stringVarIndex],
        GetTrainerNameFromId(trainerClassId),
    );
    FALSE
}
pub fn SetMovingNpcId(npcId: u16) {
    sMovingNpcId.set(npcId);
}
#[unsafe(no_mangle)]
pub unsafe fn ScrCmd_warpwhitefade(ctx: *mut ScriptContext) -> u8 {
    let mapGroup: u8 = *({
        let t2 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t2
    });
    let mapNum: u8 = *({
        let t4 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t4
    });
    let warpId: u8 = *({
        let t6 = (*ctx).scriptPtr;
        (*ctx).scriptPtr = (*ctx).scriptPtr.at(1);
        t6
    });
    let x: u16 = VarGet(ScriptReadHalfword(ctx));
    let y: u16 = VarGet(ScriptReadHalfword(ctx));
    SetWarpDestination(mapGroup as i8, mapNum as i8, warpId as i8, x as i8, y as i8);
    DoWhiteFadeWarp();
    ResetInitialPlayerAvatarState();
    TRUE
}
