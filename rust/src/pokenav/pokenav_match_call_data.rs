//! Translated from `src/pokenav_match_call_data.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    dead_code,
    unused_variables
)]

use crate::battle_setup::CountBattledRematchTeams;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, FlagSet};
use crate::ffi::gSpecialVar_0x8004;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::match_call::BufferPokedexRatingForMatchCall;
use crate::string_util::StringExpandPlaceholders;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sMrStoneTextScripts sMrStoneMatchCallHeader sNormanTextScripts sNormanMatchCallHeader sProfBirchMatchCallHeader sMomTextScripts sMomMatchCallHeader sStevenTextScripts sStevenMatchCallHeader sMayTextScripts sMayMatchCallHeader sBrendanTextScripts sBrendanMatchCallHeader sWallyTextScripts sWallyLocationData sWallyMatchCallHeader sScottTextScripts sScottMatchCallHeader sRoxanneTextScripts sRoxanneMatchCallHeader sBrawlyTextScripts sBrawlyMatchCallHeader sWattsonTextScripts sWattsonMatchCallHeader sFlanneryTextScripts sFlanneryMatchCallHeader sWinonaTextScripts sWinonaMatchCallHeader sTateLizaTextScripts sTateLizaMatchCallHeader sJuanTextScripts sJuanMatchCallHeader sSidneyTextScripts sSidneyMatchCallHeader sPhoebeTextScripts sPhoebeMatchCallHeader sGlaciaTextScripts sGlaciaMatchCallHeader sDrakeTextScripts sDrakeMatchCallHeader sWallaceTextScripts sWallaceMatchCallHeader sMatchCallHeaders sMatchCallGetEnabledFuncs sMatchCallGetMapSecFuncs sMatchCall_IsRematchableFunctions sMatchCall_HasCheckPageFunctions sMatchCall_GetRematchTableIdxFunctions sMatchCall_GetMessageFunctions sMatchCall_GetNameAndDescFunctions sCheckPageOverrides

/// `match_call_t`
#[repr(C)]
#[derive(Clone, Copy)]
pub union match_call_t {
    pub common: *mut MatchCallStructCommon,
    pub npc: *mut MatchCallStructNPC,
    pub trainer: *mut MatchCallStructTrainer,
    pub wally: *mut MatchCallWally,
    pub birch: *mut MatchCallBirch,
    pub rival: *mut MatchCallRival,
    pub leader: *mut MatchCallStructTrainer,
}

unsafe impl Sync for match_call_t {}

/// `struct MatchCallTextDataStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallTextDataStruct {
    pub text: *mut u8,
    pub availabilityFlag: u16,
    pub flagToSetOnCompletion: u16,
}

unsafe impl Sync for MatchCallTextDataStruct {}

/// `struct MatchCallCheckPageOverride`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallCheckPageOverride {
    pub idx: u16,
    pub facilityClass: u16,
    pub flag: u32,
    pub flavorTexts: CArray<*mut u8, 4>,
}

unsafe impl Sync for MatchCallCheckPageOverride {}

/// `struct MatchCallStructCommon`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MatchCallStructCommon {
    pub r#type: u8,
    pub mapSec: u8,
    pub flag: u16,
}

unsafe impl Sync for MatchCallStructCommon {}

/// `struct MatchCallStructNPC`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallStructNPC {
    pub r#type: u8,
    pub mapSec: u8,
    pub flag: u16,
    pub desc: *mut u8,
    pub name: *mut u8,
    pub textData: *mut MatchCallTextDataStruct,
}

unsafe impl Sync for MatchCallStructNPC {}

/// `struct MatchCallStructTrainer`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallStructTrainer {
    pub r#type: u8,
    pub mapSec: u8,
    pub flag: u16,
    pub rematchTableIdx: u16,
    pub desc: *mut u8,
    pub name: *mut u8,
    pub textData: *mut MatchCallTextDataStruct,
}

unsafe impl Sync for MatchCallStructTrainer {}

/// `struct MatchCallWally`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallWally {
    pub r#type: u8,
    pub mapSec: u8,
    pub flag: u16,
    pub rematchTableIdx: u16,
    pub desc: *mut u8,
    pub textData: *mut MatchCallTextDataStruct,
    pub locationData: *mut MatchCallLocationOverride,
}

unsafe impl Sync for MatchCallWally {}

/// `struct MatchCallBirch`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallBirch {
    pub r#type: u8,
    pub mapSec: u8,
    pub flag: u16,
    pub desc: *mut u8,
    pub name: *mut u8,
}

unsafe impl Sync for MatchCallBirch {}

/// `struct MatchCallRival`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallRival {
    pub r#type: u8,
    pub playerGender: u8,
    pub flag: u16,
    pub desc: *mut u8,
    pub name: *mut u8,
    pub textData: *mut MatchCallTextDataStruct,
}

unsafe impl Sync for MatchCallRival {}

/// `struct MatchCallLocationOverride`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MatchCallLocationOverride {
    pub flag: u16,
    pub mapSec: u8,
}

unsafe impl Sync for MatchCallLocationOverride {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<match_call_t>() == 4);
    assert!(size_of::<MatchCallTextDataStruct>() == 8);
    assert!(offset_of!(MatchCallTextDataStruct, text) == 0);
    assert!(offset_of!(MatchCallTextDataStruct, availabilityFlag) == 4);
    assert!(offset_of!(MatchCallTextDataStruct, flagToSetOnCompletion) == 6);
    assert!(size_of::<MatchCallCheckPageOverride>() == 24);
    assert!(offset_of!(MatchCallCheckPageOverride, idx) == 0);
    assert!(offset_of!(MatchCallCheckPageOverride, facilityClass) == 2);
    assert!(offset_of!(MatchCallCheckPageOverride, flag) == 4);
    assert!(offset_of!(MatchCallCheckPageOverride, flavorTexts) == 8);
    assert!(size_of::<MatchCallStructCommon>() == 4);
    assert!(offset_of!(MatchCallStructCommon, r#type) == 0);
    assert!(offset_of!(MatchCallStructCommon, mapSec) == 1);
    assert!(offset_of!(MatchCallStructCommon, flag) == 2);
    assert!(size_of::<MatchCallStructNPC>() == 16);
    assert!(offset_of!(MatchCallStructNPC, r#type) == 0);
    assert!(offset_of!(MatchCallStructNPC, mapSec) == 1);
    assert!(offset_of!(MatchCallStructNPC, flag) == 2);
    assert!(offset_of!(MatchCallStructNPC, desc) == 4);
    assert!(offset_of!(MatchCallStructNPC, name) == 8);
    assert!(offset_of!(MatchCallStructNPC, textData) == 12);
    assert!(size_of::<MatchCallStructTrainer>() == 20);
    assert!(offset_of!(MatchCallStructTrainer, r#type) == 0);
    assert!(offset_of!(MatchCallStructTrainer, mapSec) == 1);
    assert!(offset_of!(MatchCallStructTrainer, flag) == 2);
    assert!(offset_of!(MatchCallStructTrainer, rematchTableIdx) == 4);
    assert!(offset_of!(MatchCallStructTrainer, desc) == 8);
    assert!(offset_of!(MatchCallStructTrainer, name) == 12);
    assert!(offset_of!(MatchCallStructTrainer, textData) == 16);
    assert!(size_of::<MatchCallWally>() == 20);
    assert!(offset_of!(MatchCallWally, r#type) == 0);
    assert!(offset_of!(MatchCallWally, mapSec) == 1);
    assert!(offset_of!(MatchCallWally, flag) == 2);
    assert!(offset_of!(MatchCallWally, rematchTableIdx) == 4);
    assert!(offset_of!(MatchCallWally, desc) == 8);
    assert!(offset_of!(MatchCallWally, textData) == 12);
    assert!(offset_of!(MatchCallWally, locationData) == 16);
    assert!(size_of::<MatchCallBirch>() == 12);
    assert!(offset_of!(MatchCallBirch, r#type) == 0);
    assert!(offset_of!(MatchCallBirch, mapSec) == 1);
    assert!(offset_of!(MatchCallBirch, flag) == 2);
    assert!(offset_of!(MatchCallBirch, desc) == 4);
    assert!(offset_of!(MatchCallBirch, name) == 8);
    assert!(size_of::<MatchCallRival>() == 16);
    assert!(offset_of!(MatchCallRival, r#type) == 0);
    assert!(offset_of!(MatchCallRival, playerGender) == 1);
    assert!(offset_of!(MatchCallRival, flag) == 2);
    assert!(offset_of!(MatchCallRival, desc) == 4);
    assert!(offset_of!(MatchCallRival, name) == 8);
    assert!(offset_of!(MatchCallRival, textData) == 12);
    assert!(size_of::<MatchCallLocationOverride>() == 4);
    assert!(offset_of!(MatchCallLocationOverride, flag) == 0);
    assert!(offset_of!(MatchCallLocationOverride, mapSec) == 2);
};

const ALWAYS_AVAILABLE: u16 = 65535;
const MC_TYPE_BIRCH: u8 = 3;
const MC_TYPE_LEADER: u8 = 5;
const MC_TYPE_RIVAL: u8 = 4;
const MC_TYPE_TRAINER: u8 = 1;
const MC_TYPE_WALLY: u8 = 2;
const NO_FLAG_TO_SET: u16 = 65535;
const REMATCH_CALL_START: u16 = 65534;

static sCheckPageOverrides: Table<CArray<MatchCallCheckPageOverride, 4>> =
    Table((&raw const crate::data::pokenav_match_call_data::sCheckPageOverrides).cast());
static sMatchCallGetEnabledFuncs: Table<CArray<Option<unsafe fn(match_call_t) -> u32>, 5>> =
    Table((&raw const crate::data::pokenav_match_call_data::sMatchCallGetEnabledFuncs).cast());
static sMatchCallGetMapSecFuncs: Table<CArray<Option<unsafe fn(match_call_t) -> u8>, 5>> =
    Table((&raw const crate::data::pokenav_match_call_data::sMatchCallGetMapSecFuncs).cast());
static sMatchCallHeaders: Table<CArray<match_call_t, 21>> =
    Table((&raw const crate::data::pokenav_match_call_data::sMatchCallHeaders).cast());
static sMatchCall_GetMessageFunctions: Table<CArray<Option<unsafe fn(match_call_t, *mut u8)>, 5>> =
    Table((&raw const crate::data::pokenav_match_call_data::sMatchCall_GetMessageFunctions).cast());
static sMatchCall_GetNameAndDescFunctions: Table<
    CArray<Option<unsafe fn(match_call_t, *mut *mut u8, *mut *mut u8)>, 5>,
> = Table(
    (&raw const crate::data::pokenav_match_call_data::sMatchCall_GetNameAndDescFunctions).cast(),
);
static sMatchCall_GetRematchTableIdxFunctions: Table<
    CArray<Option<unsafe fn(match_call_t) -> u32>, 5>,
> = Table(
    (&raw const crate::data::pokenav_match_call_data::sMatchCall_GetRematchTableIdxFunctions)
        .cast(),
);
static sMatchCall_HasCheckPageFunctions: Table<CArray<Option<unsafe fn(match_call_t) -> u32>, 5>> =
    Table(
        (&raw const crate::data::pokenav_match_call_data::sMatchCall_HasCheckPageFunctions).cast(),
    );
static sMatchCall_IsRematchableFunctions: Table<CArray<Option<unsafe fn(match_call_t) -> u32>, 5>> =
    Table(
        (&raw const crate::data::pokenav_match_call_data::sMatchCall_IsRematchableFunctions).cast(),
    );

unsafe fn MatchCallGetFunctionIndex(matchCall: match_call_t) -> u32 {
    match (*matchCall.common).r#type {
        MC_TYPE_TRAINER | MC_TYPE_LEADER => {
            return 1;
        }
        MC_TYPE_WALLY => {
            return 2;
        }
        MC_TYPE_RIVAL => {
            return 3;
        }
        MC_TYPE_BIRCH => {
            return 4;
        }
        _ => {
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetTrainerIdxByRematchIdx(rematchIdx: u32) -> u32 {
    (*(&raw const crate::data::battle_setup::gRematchTable).cast::<CArray<RematchTrainer, 78>>())
        [rematchIdx]
        .trainerIds[0] as u32
}
pub unsafe fn GetRematchIdxByTrainerIdx(trainerIdx: i32) -> i32 {
    for rematchIdx in 0..REMATCH_TABLE_ENTRIES {
        if (*(&raw const crate::data::battle_setup::gRematchTable)
            .cast::<CArray<RematchTrainer, 78>>())[rematchIdx]
            .trainerIds[0] as i32
            == trainerIdx
        {
            return rematchIdx;
        }
    }
    -1
}
pub unsafe fn MatchCall_GetEnabled(idx: u32) -> u32 {
    if idx >= 21 {
        return FALSE as u32;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    sMatchCallGetEnabledFuncs[i].unwrap_unchecked()(matchCall)
}
pub(crate) unsafe fn MatchCall_GetEnabled_NPC(matchCall: match_call_t) -> u32 {
    if (*matchCall.npc).flag == 0xFFFF {
        return TRUE as u32;
    }
    FlagGet((*matchCall.npc).flag) as u32
}
pub(crate) unsafe fn MatchCall_GetEnabled_Trainer(matchCall: match_call_t) -> u32 {
    if (*matchCall.trainer).flag == 0xFFFF {
        return TRUE as u32;
    }
    FlagGet((*matchCall.trainer).flag) as u32
}
pub(crate) unsafe fn MatchCall_GetEnabled_Wally(matchCall: match_call_t) -> u32 {
    if (*matchCall.wally).flag == 0xFFFF {
        return TRUE as u32;
    }
    FlagGet((*matchCall.wally).flag) as u32
}
pub(crate) unsafe fn MatchCall_GetEnabled_Rival(matchCall: match_call_t) -> u32 {
    if (*matchCall.rival).playerGender != (*gSaveBlock2Ptr).playerGender {
        return FALSE as u32;
    }
    if (*matchCall.rival).flag == 0xFFFF {
        return TRUE as u32;
    }
    FlagGet((*matchCall.rival).flag) as u32
}
pub(crate) unsafe fn MatchCall_GetEnabled_Birch(matchCall: match_call_t) -> u32 {
    FlagGet((*matchCall.birch).flag) as u32
}
pub unsafe fn MatchCall_GetMapSec(idx: u32) -> u8 {
    if idx >= 21 {
        return 0;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    sMatchCallGetMapSecFuncs[i].unwrap_unchecked()(matchCall)
}
pub(crate) unsafe fn MatchCall_GetMapSec_NPC(matchCall: match_call_t) -> u8 {
    (*matchCall.npc).mapSec
}
pub(crate) unsafe fn MatchCall_GetMapSec_Trainer(matchCall: match_call_t) -> u8 {
    (*matchCall.trainer).mapSec
}
pub(crate) unsafe fn MatchCall_GetMapSec_Wally(matchCall: match_call_t) -> u8 {
    let mut i: i32 = 0;
    while (*(*matchCall.wally).locationData.at(i)).flag != 0xFFFF {
        if FlagGet((*(*matchCall.wally).locationData.at(i)).flag) == 0 {
            break;
        }
        i += 1;
    }
    (*(*matchCall.wally).locationData.at(i)).mapSec
}
pub(crate) fn MatchCall_GetMapSec_Rival(matchCall: match_call_t) -> u8 {
    MAPSEC_NONE as u8
}
pub(crate) fn MatchCall_GetMapSec_Birch(matchCall: match_call_t) -> u8 {
    MAPSEC_NONE as u8
}
pub unsafe fn MatchCall_IsRematchable(idx: u32) -> u32 {
    if idx >= 21 {
        return 0;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    sMatchCall_IsRematchableFunctions[i].unwrap_unchecked()(matchCall)
}
pub(crate) fn MatchCall_IsRematchable_NPC(matchCall: match_call_t) -> u32 {
    FALSE as u32
}
pub(crate) unsafe fn MatchCall_IsRematchable_Trainer(matchCall: match_call_t) -> u32 {
    if (*matchCall.trainer).rematchTableIdx >= REMATCH_SIDNEY as u16 {
        return FALSE as u32;
    }
    (if (*gSaveBlock1Ptr).trainerRematches[(*matchCall.trainer).rematchTableIdx] != 0 {
        TRUE as i32
    } else {
        FALSE as i32
    }) as u32
}
pub(crate) unsafe fn MatchCall_IsRematchable_Wally(matchCall: match_call_t) -> u32 {
    (if (*gSaveBlock1Ptr).trainerRematches[(*matchCall.wally).rematchTableIdx] != 0 {
        TRUE as i32
    } else {
        FALSE as i32
    }) as u32
}
pub(crate) fn MatchCall_IsRematchable_Rival(matchCall: match_call_t) -> u32 {
    FALSE as u32
}
pub(crate) fn MatchCall_IsRematchable_Birch(matchCall: match_call_t) -> u32 {
    FALSE as u32
}
pub unsafe fn MatchCall_HasCheckPage(idx: u32) -> u32 {
    if idx >= 21 {
        return FALSE as u32;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    if sMatchCall_HasCheckPageFunctions[i].unwrap_unchecked()(matchCall) != 0 {
        return TRUE as u32;
    }
    for i in 0..4u32 {
        if sCheckPageOverrides[i].idx as u32 == idx {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub(crate) fn MatchCall_HasCheckPage_NPC(matchCall: match_call_t) -> u32 {
    FALSE as u32
}
pub(crate) fn MatchCall_HasCheckPage_Trainer(matchCall: match_call_t) -> u32 {
    TRUE as u32
}
pub(crate) fn MatchCall_HasCheckPage_Wally(matchCall: match_call_t) -> u32 {
    TRUE as u32
}
pub(crate) fn MatchCall_HasCheckPage_Rival(matchCall: match_call_t) -> u32 {
    FALSE as u32
}
pub(crate) fn MatchCall_HasCheckPage_Birch(matchCall: match_call_t) -> u32 {
    FALSE as u32
}
pub unsafe fn MatchCall_GetRematchTableIdx(idx: u32) -> u32 {
    if idx >= 21 {
        return REMATCH_TABLE_ENTRIES as u32;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    sMatchCall_GetRematchTableIdxFunctions[i].unwrap_unchecked()(matchCall)
}
pub(crate) fn MatchCall_GetRematchTableIdx_NPC(matchCall: match_call_t) -> u32 {
    REMATCH_TABLE_ENTRIES as u32
}
pub(crate) unsafe fn MatchCall_GetRematchTableIdx_Trainer(matchCall: match_call_t) -> u32 {
    (*matchCall.trainer).rematchTableIdx as u32
}
pub(crate) unsafe fn MatchCall_GetRematchTableIdx_Wally(matchCall: match_call_t) -> u32 {
    (*matchCall.wally).rematchTableIdx as u32
}
pub(crate) fn MatchCall_GetRematchTableIdx_Rival(matchCall: match_call_t) -> u32 {
    REMATCH_TABLE_ENTRIES as u32
}
pub(crate) fn MatchCall_GetRematchTableIdx_Birch(matchCall: match_call_t) -> u32 {
    REMATCH_TABLE_ENTRIES as u32
}
pub unsafe fn MatchCall_GetMessage(idx: u32, dest: *mut u8) {
    if idx >= 21 {
        return;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    sMatchCall_GetMessageFunctions[i].unwrap_unchecked()(matchCall, dest);
}
pub(crate) unsafe fn MatchCall_GetMessage_NPC(matchCall: match_call_t, dest: *mut u8) {
    MatchCall_BufferCallMessageText((*matchCall.npc).textData, dest);
}
pub(crate) unsafe fn MatchCall_GetMessage_Trainer(matchCall: match_call_t, dest: *mut u8) {
    if (*matchCall.common).r#type != MC_TYPE_LEADER {
        MatchCall_BufferCallMessageText((*matchCall.trainer).textData, dest);
    } else {
        MatchCall_BufferCallMessageTextByRematchTeam(
            (*matchCall.leader).textData,
            (*matchCall.leader).rematchTableIdx,
            dest,
        );
    }
}
pub(crate) unsafe fn MatchCall_GetMessage_Wally(matchCall: match_call_t, dest: *mut u8) {
    MatchCall_BufferCallMessageText((*matchCall.wally).textData, dest);
}
pub(crate) unsafe fn MatchCall_GetMessage_Rival(matchCall: match_call_t, dest: *mut u8) {
    MatchCall_BufferCallMessageText((*matchCall.rival).textData, dest);
}
pub(crate) unsafe fn MatchCall_GetMessage_Birch(matchCall: match_call_t, dest: *mut u8) {
    BufferPokedexRatingForMatchCall(dest);
}
unsafe fn MatchCall_BufferCallMessageText(textData: *mut MatchCallTextDataStruct, dest: *mut u8) {
    let mut i: u32 = 0;
    while !(*textData.at(i)).text.is_null() {
        i += 1;
    }
    i = i.saturating_sub(1);
    while i != 0 {
        if (*textData.at(i)).availabilityFlag != ALWAYS_AVAILABLE
            && FlagGet((*textData.at(i)).availabilityFlag) == TRUE
        {
            break;
        }
        i -= 1;
    }
    if (*textData.at(i)).flagToSetOnCompletion != NO_FLAG_TO_SET {
        FlagSet((*textData.at(i)).flagToSetOnCompletion);
    }
    StringExpandPlaceholders(dest, (*textData.at(i)).text);
}
unsafe fn MatchCall_BufferCallMessageTextByRematchTeam(
    textData: *mut MatchCallTextDataStruct,
    idx: u16,
    dest: *mut u8,
) {
    let mut i: u32 = 0;
    while !(*textData.at(i)).text.is_null() {
        if (*textData.at(i)).availabilityFlag == REMATCH_CALL_START {
            break;
        }
        if (*textData.at(i)).availabilityFlag != ALWAYS_AVAILABLE
            && FlagGet((*textData.at(i)).availabilityFlag) == 0
        {
            break;
        }
        i += 1;
    }
    if (*textData.at(i)).availabilityFlag != REMATCH_CALL_START {
        i = i.saturating_sub(1);
        if (*textData.at(i)).flagToSetOnCompletion != NO_FLAG_TO_SET {
            FlagSet((*textData.at(i)).flagToSetOnCompletion);
        }
        StringExpandPlaceholders(dest, (*textData.at(i)).text);
    } else {
        if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
            if (*gSaveBlock1Ptr).trainerRematches[idx] != 0 {
                i += 2;
            } else if CountBattledRematchTeams(idx) >= 2 {
                i += 3;
            } else {
                i += 1;
            }
        }
        StringExpandPlaceholders(dest, (*textData.at(i)).text);
    }
}
pub unsafe fn MatchCall_GetNameAndDesc(idx: u32, desc: *mut *mut u8, name: *mut *mut u8) {
    if idx >= 21 {
        return;
    }
    let matchCall: match_call_t = sMatchCallHeaders[idx];
    let i: u32 = MatchCallGetFunctionIndex(matchCall);
    sMatchCall_GetNameAndDescFunctions[i].unwrap_unchecked()(matchCall, desc, name);
}
pub(crate) unsafe fn MatchCall_GetNameAndDesc_NPC(
    matchCall: match_call_t,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    *desc = (*matchCall.npc).desc;
    *name = (*matchCall.npc).name;
}
pub(crate) unsafe fn MatchCall_GetNameAndDesc_Trainer(
    matchCall: match_call_t,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    let mut _matchCall: match_call_t = matchCall;
    if (*_matchCall.trainer).name.is_null() {
        MatchCall_GetNameAndDescByRematchIdx(
            (*_matchCall.trainer).rematchTableIdx as u32,
            desc,
            name,
        );
    } else {
        *name = (*_matchCall.trainer).name;
    }
    *desc = (*_matchCall.trainer).desc;
}
pub(crate) unsafe fn MatchCall_GetNameAndDesc_Wally(
    matchCall: match_call_t,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    MatchCall_GetNameAndDescByRematchIdx((*matchCall.wally).rematchTableIdx as u32, desc, name);
    *desc = (*matchCall.wally).desc;
}
pub(crate) unsafe fn MatchCall_GetNameAndDesc_Rival(
    matchCall: match_call_t,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    *desc = (*matchCall.rival).desc;
    *name = (*matchCall.rival).name;
}
pub(crate) unsafe fn MatchCall_GetNameAndDesc_Birch(
    matchCall: match_call_t,
    desc: *mut *mut u8,
    name: *mut *mut u8,
) {
    *desc = (*matchCall.birch).desc;
    *name = (*matchCall.birch).name;
}
unsafe fn MatchCall_GetNameAndDescByRematchIdx(idx: u32, desc: *mut *mut u8, name: *mut *mut u8) {
    let trainer: *mut Trainer = (*(&raw const crate::data::data_tables::gTrainers)
        .cast::<CArray<Trainer, 0>>())
    .as_ptr()
    .cast_mut()
    .at(GetTrainerIdxByRematchIdx(idx));
    *desc = (*(&raw const crate::data::data_tables::gTrainerClassNames)
        .cast::<CArray<CArray<u8, 13>, 0>>())[(*trainer).trainerClass]
        .as_ptr()
        .cast_mut();
    *name = (*trainer).trainerName.as_mut_ptr();
}
pub unsafe fn MatchCall_GetOverrideFlavorText(idx: u32, offset: u32) -> *mut u8 {
    let mut i: u32 = 0;
    while i < 4 {
        if sCheckPageOverrides[i].idx as u32 == idx {
            while i + 1 < 4
                && sCheckPageOverrides[i + 1].idx as u32 == idx
                && FlagGet(sCheckPageOverrides[i + 1].flag as u16) != 0
            {
                i += 1;
            }
            return sCheckPageOverrides[i].flavorTexts[offset];
        }
        i += 1;
    }
    null_mut()
}
pub fn MatchCall_GetOverrideFacilityClass(idx: u32) -> i32 {
    for i in 0..4u32 {
        if sCheckPageOverrides[i].idx as u32 == idx {
            return sCheckPageOverrides[i].facilityClass as i32;
        }
    }
    -1
}
pub unsafe fn MatchCall_HasRematchId(idx: u32) -> u32 {
    for i in 0..21i32 {
        let id: u32 = MatchCall_GetRematchTableIdx(i as u32);
        if id != REMATCH_TABLE_ENTRIES as u32 && id == idx {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn SetMatchCallRegisteredFlag() {
    let index: i32 = GetRematchIdxByTrainerIdx(gSpecialVar_0x8004 as i32);
    if index >= 0 {
        FlagSet(TRAINER_REGISTERED_FLAGS_START + index as u16);
    }
}
