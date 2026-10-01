//! Translated from `src/battle_pike.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_main::gBattleOutcome;
use crate::battle_setup::{gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
use crate::battle_tower::{
    FrontierSpeechToString, GetHighestLevelInPlayerParty, GetRandomScaledFrontierTrainerId,
    SetBattleFacilityTrainerGfxId, gFacilityTrainers,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarSet;
use crate::ffi::{gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_0x8007, gSpecialVar_Result};
use crate::fieldmap::{SaveMapView, gMapHeader};
use crate::frontier_util::{GetPlayerSymbolCountForFacility, SetFrontierBrainObjEventGfx};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::palette::BlendPalettes;
use crate::party_menu::GetAilmentFromStatus;
use crate::pokemon::{
    CalculateMonStats, CalculatePPWithBonus, GetMonAbility, GetMonData2, GetMonData3, SetMonData,
    SetMonMoveSlot, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::save::TrySavingData;
use crate::script::ScriptContext_Enable;
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// Data tables (translate with cdata.py): sLvl50_Mons1 sLvl50_Mons2 sLvl50_Mons3 sLvl50_Mons4 sLvl50Mons sLvlOpen_Mons1 sLvlOpen_Mons2 sLvlOpen_Mons3 sLvlOpen_Mons4 sLvlOpenMons sWildMons sNPCTable sNPCSpeeches sFrontierBrainStreakAppearances sBattlePikeFunctions sRoomTypeHints sNumMonsToHealBeforePikeQueen sStatusInflictionScreenFlashFuncs sWinStreakFlags

/// `struct PikeWildMon`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PikeWildMon {
    pub species: u16,
    pub levelDelta: u8,
    pub moves: CArray<u16, 4>,
}

unsafe impl Sync for PikeWildMon {}

/// `struct PikeRoomNPC`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PikeRoomNPC {
    pub graphicsId: u16,
    pub speechId1: u8,
    pub speechId2: u8,
    pub speechId3: u8,
}

unsafe impl Sync for PikeRoomNPC {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PikeWildMon>() == 12);
    assert!(offset_of!(PikeWildMon, species) == 0);
    assert!(offset_of!(PikeWildMon, levelDelta) == 2);
    assert!(offset_of!(PikeWildMon, moves) == 4);
    assert!(size_of::<PikeRoomNPC>() == 8);
    assert!(offset_of!(PikeRoomNPC, graphicsId) == 0);
    assert!(offset_of!(PikeRoomNPC, speechId1) == 2);
    assert!(offset_of!(PikeRoomNPC, speechId2) == 3);
    assert!(offset_of!(PikeRoomNPC, speechId3) == 4);
};

static sBattlePikeFunctions: Table<CArray<Option<unsafe fn()>, 29>> =
    Table((&raw const crate::data::battle_pike::sBattlePikeFunctions).cast());
static sFrontierBrainStreakAppearances: Table<CArray<CArray<u8, 4>, 7>> =
    Table((&raw const crate::data::battle_pike::sFrontierBrainStreakAppearances).cast());
static sNPCSpeeches: Table<CArray<CArray<u16, 6>, 42>> =
    Table((&raw const crate::data::battle_pike::sNPCSpeeches).cast());
static sNPCTable: Table<CArray<PikeRoomNPC, 25>> =
    Table((&raw const crate::data::battle_pike::sNPCTable).cast());
static sNumMonsToHealBeforePikeQueen: Table<CArray<CArray<u8, 3>, 6>> =
    Table((&raw const crate::data::battle_pike::sNumMonsToHealBeforePikeQueen).cast());
static sRoomTypeHints: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::battle_pike::sRoomTypeHints).cast());
static sStatusInflictionScreenFlashFuncs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 2>> =
    Table((&raw const crate::data::battle_pike::sStatusInflictionScreenFlashFuncs).cast());
static sWildMons: Table<CArray<*mut *mut PikeWildMon, 2>> =
    Table((&raw const crate::data::battle_pike::sWildMons).cast());
static sWinStreakFlags: Table<CArray<u32, 2>> =
    Table((&raw const crate::data::battle_pike::sWinStreakFlags).cast());

pub(crate) static sRoomType: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sStatusMon: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sInWildMonRoom: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sStatusFlags: crate::global::Global<u32> = crate::global::Global::new(0);
pub(crate) static sNpcId: crate::global::Global<u8> = crate::global::Global::new(0);

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}

#[unsafe(no_mangle)]
pub unsafe fn CallBattlePikeFunction() {
    sBattlePikeFunctions[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn SetRoomType() {
    let roomType: u8 = GetNextRoomType();
    sRoomType.set(roomType);
}
pub(crate) unsafe fn SetupRoomObjectEvents() {
    VarSet(VAR_OBJ_GFX_ID_0, OBJ_EVENT_GFX_LINK_RECEPTIONIST);
    VarSet(VAR_OBJ_GFX_ID_1, OBJ_EVENT_GFX_DUSCLOPS);
    let mut setObjGfx1: u32 = TRUE as u32;
    let mut setObjGfx2: u32 = FALSE as u32;
    let mut objGfx1: u32 = 0;
    let mut objGfx2: u16 = 0;
    match sRoomType.get() {
        PIKE_ROOM_SINGLE_BATTLE => {
            PrepareOneTrainer(FALSE);
            setObjGfx1 = FALSE as u32;
        }
        PIKE_ROOM_HEAL_FULL => {
            objGfx1 = OBJ_EVENT_GFX_LINK_RECEPTIONIST as u32;
        }
        PIKE_ROOM_NPC => {
            objGfx1 = GetNPCRoomGraphicsId() as u8 as u32;
        }
        PIKE_ROOM_STATUS => {
            objGfx1 = OBJ_EVENT_GFX_GENTLEMAN;
            if sStatusMon.get() == PIKE_STATUSMON_DUSCLOPS {
                objGfx2 = OBJ_EVENT_GFX_DUSCLOPS;
            } else {
                objGfx2 = OBJ_EVENT_GFX_KIRLIA;
            }
            setObjGfx2 = TRUE as u32;
        }
        PIKE_ROOM_HEAL_PART => {
            objGfx1 = OBJ_EVENT_GFX_GENTLEMAN;
        }
        PIKE_ROOM_WILD_MONS => {
            setObjGfx1 = FALSE as u32;
        }
        PIKE_ROOM_HARD_BATTLE => {
            PrepareOneTrainer(TRUE);
            objGfx2 = OBJ_EVENT_GFX_LINK_RECEPTIONIST;
            setObjGfx1 = FALSE as u32;
            setObjGfx2 = TRUE as u32;
        }
        PIKE_ROOM_DOUBLE_BATTLE => {
            PrepareTwoTrainers();
            setObjGfx1 = FALSE as u32;
        }
        PIKE_ROOM_BRAIN => {
            SetFrontierBrainObjEventGfx(FRONTIER_FACILITY_PIKE as u8);
            objGfx2 = OBJ_EVENT_GFX_LINK_RECEPTIONIST;
            setObjGfx1 = FALSE as u32;
            setObjGfx2 = TRUE as u32;
        }
        _ => {
            return;
        }
    }
    if setObjGfx1 == TRUE as u32 {
        VarSet(VAR_OBJ_GFX_ID_0, objGfx1 as u16);
    }
    if setObjGfx2 == TRUE as u32 {
        VarSet(VAR_OBJ_GFX_ID_1, objGfx2);
    }
}
pub(crate) unsafe fn GetBattlePikeData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        PIKE_DATA_PRIZE => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.pikePrize;
        }
        PIKE_DATA_WIN_STREAK => {
            gSpecialVar_Result =
                (*gSaveBlock2Ptr).frontier.pikeWinStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()];
        }
        PIKE_DATA_RECORD_STREAK => {
            gSpecialVar_Result =
                (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()];
        }
        PIKE_DATA_TOTAL_STREAKS => {
            gSpecialVar_Result =
                (*gSaveBlock2Ptr).frontier.pikeTotalStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()];
        }
        PIKE_DATA_WIN_STREAK_ACTIVE => {
            if lvlMode != FRONTIER_LVL_50 as u32 {
                gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16
                    & STREAK_PIKE_OPEN as u16;
            } else {
                gSpecialVar_Result =
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16 & STREAK_PIKE_50 as u16;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetBattlePikeData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        PIKE_DATA_PRIZE => {
            (*gSaveBlock2Ptr).frontier.pikePrize = *(&raw const crate::ffi::gSpecialVar_0x8006)
                .cast::<u16>()
                .cast_mut();
        }
        PIKE_DATA_WIN_STREAK => {
            if gSpecialVar_0x8006 <= MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.pikeWinStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()] =
                    *(&raw const crate::ffi::gSpecialVar_0x8006)
                        .cast::<u16>()
                        .cast_mut();
            }
        }
        PIKE_DATA_RECORD_STREAK => {
            if gSpecialVar_0x8006 <= MAX_STREAK
                && (*gSaveBlock2Ptr).frontier.pikeRecordStreaks
                    [(*gSaveBlock2Ptr).frontier.lvlMode()]
                    < gSpecialVar_0x8006
            {
                (*gSaveBlock2Ptr).frontier.pikeRecordStreaks
                    [(*gSaveBlock2Ptr).frontier.lvlMode()] =
                    *(&raw const crate::ffi::gSpecialVar_0x8006)
                        .cast::<u16>()
                        .cast_mut();
            }
        }
        PIKE_DATA_TOTAL_STREAKS => {
            if gSpecialVar_0x8006 <= MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.pikeTotalStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()] =
                    *(&raw const crate::ffi::gSpecialVar_0x8006)
                        .cast::<u16>()
                        .cast_mut();
            }
        }
        PIKE_DATA_WIN_STREAK_ACTIVE => {
            if lvlMode != FRONTIER_LVL_50 as u32 {
                if gSpecialVar_0x8006 != 0 {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |= STREAK_PIKE_OPEN;
                } else {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &= 0xfffff7ff;
                }
            } else {
                if gSpecialVar_0x8006 != 0 {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |= STREAK_PIKE_50;
                } else {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &= 0xfffffbff;
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn IsNextRoomFinal() {
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum > NUM_PIKE_ROOMS {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe fn GetRoomType() {
    gSpecialVar_Result = sRoomType.get() as u16;
}
pub(crate) fn SetInWildMonRoom() {
    sInWildMonRoom.set(TRUE);
}
pub(crate) fn ClearInWildMonRoom() {
    sInWildMonRoom.set(FALSE);
}
pub(crate) unsafe fn SavePikeChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveMapView();
    TrySavingData(SAVE_LINK);
}
pub(crate) fn PikeDummy1() {}
pub(crate) fn PikeDummy2() {}
pub(crate) unsafe fn GetRoomInflictedStatus() {
    match sStatusFlags.get() {
        STATUS1_FREEZE => {
            gSpecialVar_Result = PIKE_STATUS_FREEZE;
        }
        STATUS1_BURN => {
            gSpecialVar_Result = PIKE_STATUS_BURN;
        }
        STATUS1_TOXIC_POISON => {
            gSpecialVar_Result = PIKE_STATUS_TOXIC;
        }
        STATUS1_PARALYSIS => {
            gSpecialVar_Result = PIKE_STATUS_PARALYSIS;
        }
        STATUS1_SLEEP => {
            gSpecialVar_Result = PIKE_STATUS_SLEEP;
        }
        _ => {}
    }
}
pub(crate) unsafe fn GetRoomInflictedStatusMon() {
    gSpecialVar_Result = sStatusMon.get() as u16;
}
pub(crate) unsafe fn HealOneOrTwoMons() {
    let toHeal: u16 = (Random() as i32 % 2) as u16 + 1;
    TryHealMons(toHeal as u8);
    gSpecialVar_Result = toHeal;
}
pub(crate) unsafe fn BufferNPCMessage() {
    let mut speechId: i32 = 0;
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= 4 {
        speechId = sNPCTable[sNpcId.get()].speechId1 as i32;
    } else if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= 10 {
        speechId = sNPCTable[sNpcId.get()].speechId2 as i32;
    } else {
        speechId = sNPCTable[sNpcId.get()].speechId3 as i32;
    }
    FrontierSpeechToString(sNPCSpeeches[speechId].as_ptr().cast_mut());
}
pub(crate) unsafe fn StatusInflictionScreenFlash() {
    CreateTask(Some(Task_DoStatusInflictionScreenFlash), 2);
}
unsafe fn HealMon(mon: *mut Pokemon) {
    let mut data: CArray<u8, 4> = zeroed();
    let mut i: u8 = 0;
    while i < 4 {
        data[i] = 0;
        i += 1;
    }
    let hp: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
    data[0] = hp as u8;
    data[1] = (hp >> 8) as u8;
    SetMonData(mon, MON_DATA_HP, data.as_mut_ptr() as *mut c_void);
    let ppBonuses: u8 = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
    for i in 0..(MAX_MON_MOVES as u8) {
        let r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + i as i32) as u16;
        data[0] = CalculatePPWithBonus(r#move, ppBonuses, i);
        SetMonData(
            mon,
            MON_DATA_PP1 + i as i32,
            data.as_mut_ptr() as *mut c_void,
        );
    }
    data[0] = 0;
    data[1] = 0;
    data[2] = 0;
    data[3] = 0;
    SetMonData(mon, MON_DATA_STATUS, data.as_mut_ptr() as *mut c_void);
}
unsafe fn DoesAbilityPreventStatus(mon: *mut Pokemon, status: u32) -> u8 {
    let ability: u8 = GetMonAbility(mon);
    let mut ret: u8 = FALSE;
    match status {
        STATUS1_FREEZE => {
            if ability == ABILITY_MAGMA_ARMOR {
                ret = TRUE;
            }
        }
        STATUS1_BURN => {
            if ability == ABILITY_WATER_VEIL {
                ret = TRUE;
            }
        }
        STATUS1_PARALYSIS => {
            if ability == ABILITY_LIMBER {
                ret = TRUE;
            }
        }
        STATUS1_SLEEP => {
            if ability == ABILITY_INSOMNIA || ability == ABILITY_VITAL_SPIRIT {
                ret = TRUE;
            }
        }
        STATUS1_TOXIC_POISON if ability == ABILITY_IMMUNITY => {
            ret = TRUE;
        }
        _ => {}
    }
    ret
}
unsafe fn DoesTypePreventStatus(species: u16, status: u32) -> u8 {
    let mut ret: u8 = FALSE;
    match status {
        STATUS1_TOXIC_POISON => {
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .types[0]
                == TYPE_STEEL
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[0]
                    == TYPE_POISON
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_STEEL
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_POISON
            {
                ret = TRUE;
            }
        }
        STATUS1_FREEZE => {
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .types[0]
                == TYPE_ICE
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_ICE
            {
                ret = TRUE;
            }
        }
        STATUS1_PARALYSIS => {
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .types[0]
                == TYPE_GROUND
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[0]
                    == TYPE_ELECTRIC
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_GROUND
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_ELECTRIC
            {
                ret = TRUE;
            }
        }
        STATUS1_BURN => {
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .types[0]
                == TYPE_FIRE
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_FIRE
            {
                ret = TRUE;
            }
        }
        STATUS1_SLEEP => {}
        _ => {}
    }
    ret
}
unsafe fn TryInflictRandomStatus() -> u8 {
    let mut count: u8 = 0;
    let mut indices: CArray<u8, 3> = zeroed();
    let mut species: u16 = 0;
    let mut statusChosen: u8 = 0;
    let mut mon: *mut Pokemon = null_mut();
    let mut i: u8 = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        indices[i] = i;
        i += 1;
    }
    let mut j: u8 = 0;
    while j < 10 {
        i = (Random() as i32 % 3) as u8;
        let id: u8 = (Random() as i32 % 3) as u8;
        let temp: u8 = indices[i];
        indices[i] = indices[id];
        indices[id] = temp;
        j += 1;
    }
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= 4 {
        count = 1;
    } else if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= 9 {
        count = 2;
    } else {
        count = 3;
    }
    let mut status: u32 = 0;
    loop {
        statusChosen = FALSE;
        let rand: u8 = (Random() as i32 % 100) as u8;
        if rand < 35 {
            sStatusFlags.set(STATUS1_TOXIC_POISON);
        } else if rand < 60 {
            sStatusFlags.set(STATUS1_FREEZE);
        } else if rand < 80 {
            sStatusFlags.set(STATUS1_PARALYSIS);
        } else if rand < 90 {
            sStatusFlags.set(STATUS1_SLEEP);
        } else {
            sStatusFlags.set(STATUS1_BURN);
        }
        if status != sStatusFlags.get() {
            status = sStatusFlags.get();
            j = 0;
            for i in 0..(FRONTIER_PARTY_SIZE as u8) {
                mon = &raw mut gPlayerParty[indices[i]];
                if GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) == AILMENT_NONE
                    && GetMonData2(mon, MON_DATA_HP) != 0
                {
                    j += 1;
                    species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
                    if DoesTypePreventStatus(species, sStatusFlags.get()) == 0 {
                        statusChosen = TRUE;
                        break;
                    }
                }
                if j == count {
                    break;
                }
            }
            if j == 0 {
                return FALSE;
            }
        }
        if statusChosen != 0 {
            break;
        }
    }
    match sStatusFlags.get() {
        STATUS1_FREEZE => {
            sStatusMon.set(PIKE_STATUSMON_DUSCLOPS);
        }
        STATUS1_BURN if Random() as i32 % 2 != 0 => {
            sStatusMon.set(PIKE_STATUSMON_DUSCLOPS);
        }
        _ => {
            sStatusMon.set(PIKE_STATUSMON_KIRLIA);
        }
    }
    j = 0;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        mon = &raw mut gPlayerParty[indices[i]];
        if GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) == AILMENT_NONE
            && GetMonData2(mon, MON_DATA_HP) != 0
        {
            j += 1;
            species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
            if DoesAbilityPreventStatus(mon, sStatusFlags.get()) == 0
                && DoesTypePreventStatus(species, sStatusFlags.get()) == 0
            {
                SetMonData(mon, MON_DATA_STATUS, sStatusFlags.as_ptr() as *mut c_void);
            }
        }
        if j == count {
            break;
        }
    }
    TRUE
}
unsafe fn AtLeastOneHealthyMon() -> u8 {
    let mut count: u8 = 0;
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= 4 {
        count = 1;
    } else if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= 9 {
        count = 2;
    } else {
        count = 3;
    }
    let mut healthyMonsCount: u8 = 0;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        let mon: *mut Pokemon = &raw mut gPlayerParty[i];
        if GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) == AILMENT_NONE
            && GetMonData2(mon, MON_DATA_HP) != 0
        {
            healthyMonsCount += 1;
        }
        if healthyMonsCount == count {
            break;
        }
    }
    if healthyMonsCount == 0 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetNextRoomType() -> u8 {
    let mut roomTypesDisabled: CArray<u8, 8> = zeroed();
    if (*gSaveBlock2Ptr).frontier.pikeHintedRoomType() == PIKE_ROOM_BRAIN {
        return (*gSaveBlock2Ptr).frontier.pikeHintedRoomType();
    }
    if gSpecialVar_0x8007 == (*gSaveBlock2Ptr).frontier.pikeHintedRoomIndex() as u16 {
        if (*gSaveBlock2Ptr).frontier.pikeHintedRoomType() == PIKE_ROOM_STATUS {
            TryInflictRandomStatus();
        }
        return (*gSaveBlock2Ptr).frontier.pikeHintedRoomType();
    }
    for i in 0..8u8 {
        roomTypesDisabled[i] = FALSE;
    }
    let mut numRoomCandidates: u8 = 8;
    let roomHint: u8 = sRoomTypeHints[(*gSaveBlock2Ptr).frontier.pikeHintedRoomType()];
    let mut i: u8 = 0;
    while i < 8 {
        if sRoomTypeHints[i] == roomHint {
            roomTypesDisabled[i] = TRUE;
            numRoomCandidates -= 1;
        }
        i += 1;
    }
    if roomTypesDisabled[7] != TRUE && AtLeastTwoAliveMons() == 0 {
        roomTypesDisabled[7] = TRUE;
        numRoomCandidates -= 1;
    }
    if roomTypesDisabled[3] != TRUE && AtLeastOneHealthyMon() == 0 {
        roomTypesDisabled[3] = TRUE;
        numRoomCandidates -= 1;
    }
    if (*gSaveBlock2Ptr).frontier.pikeHealingRoomsDisabled() != 0 {
        if roomTypesDisabled[1] != 1 {
            roomTypesDisabled[1] = 1;
            numRoomCandidates -= 1;
        }
        if roomTypesDisabled[4] != TRUE {
            roomTypesDisabled[4] = TRUE;
            numRoomCandidates -= 1;
        }
    }
    let roomCandidates: *mut u8 = AllocZeroed(numRoomCandidates as u32) as *mut u8;
    let mut id: u8 = 0;
    for i in 0..8u8 {
        if roomTypesDisabled[i] == FALSE {
            *roomCandidates.at({
                let t1 = id;
                id += 1;
                t1
            }) = i;
        }
    }
    let nextRoomType: u8 = *roomCandidates.at(rem_i32(Random() as i32, numRoomCandidates as i32));
    Free(roomCandidates as *mut c_void);
    if nextRoomType == PIKE_ROOM_STATUS {
        TryInflictRandomStatus();
    }
    nextRoomType
}
fn GetNPCRoomGraphicsId() -> u16 {
    sNpcId.set((Random() % 25) as u8);
    sNPCTable[sNpcId.get()].graphicsId
}
fn GetInWildMonRoom() -> u8 {
    sInWildMonRoom.get()
}
pub unsafe fn TryGenerateBattlePikeWildMon(checkKeenEyeIntimidate: u8) -> u32 {
    let mut monLevel: i32 = 0;
    let headerId: u8 = GetBattlePikeWildMonHeaderId();
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let wildMons: *mut *mut PikeWildMon = sWildMons[lvlMode];
    let mut abilityNum: u32 = 0;
    let mut pikeMonId: i32 =
        GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) as i32;
    pikeMonId = SpeciesToPikeMonId(pikeMonId as u16) as i32;
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
        monLevel = GetHighestLevelInPlayerParty();
        if monLevel < FRONTIER_MIN_LEVEL_OPEN {
            monLevel = FRONTIER_MIN_LEVEL_OPEN;
        } else {
            monLevel -= (*(*wildMons.at(headerId)).at(pikeMonId)).levelDelta as i32;
            if monLevel < FRONTIER_MIN_LEVEL_OPEN {
                monLevel = FRONTIER_MIN_LEVEL_OPEN;
            }
        }
    } else {
        monLevel = FRONTIER_MAX_LEVEL_50 as i32
            - (*(*wildMons.at(headerId)).at(pikeMonId)).levelDelta as i32;
    }
    if checkKeenEyeIntimidate == TRUE && CanEncounterWildMon(monLevel as u8) == 0 {
        return FALSE as u32;
    }
    SetMonData(
        &raw mut gEnemyParty[0],
        MON_DATA_EXP,
        (&raw const (*(&raw const crate::data::pokemon::gExperienceTables)
            .cast::<CArray<CArray<u32, 101>, 0>>())
            [(*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*(*wildMons.at(headerId)).at(pikeMonId)).species]
                .growthRate][monLevel])
            .cast_mut() as *mut c_void,
    );
    if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
        [(*(*wildMons.at(headerId)).at(pikeMonId)).species]
        .abilities[1]
        != 0
    {
        abilityNum = (Random() as i32 % 2) as u32;
    } else {
        abilityNum = 0;
    }
    SetMonData(
        &raw mut gEnemyParty[0],
        MON_DATA_ABILITY_NUM,
        &raw mut abilityNum as *mut c_void,
    );
    for i in 0..MAX_MON_MOVES {
        SetMonMoveSlot(
            &raw mut gEnemyParty[0],
            (*(*wildMons.at(headerId)).at(pikeMonId)).moves[i],
            i as u8,
        );
    }
    CalculateMonStats(&raw mut gEnemyParty[0]);
    TRUE as u32
}
pub unsafe fn GetBattlePikeWildMonHeaderId() -> u8 {
    let mut headerId: u8 = 0;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode];
    if winStreak <= 280 {
        headerId = 0;
    } else if winStreak <= 560 {
        headerId = 1;
    } else if winStreak <= 840 {
        headerId = 2;
    } else {
        headerId = 3;
    }
    headerId
}
pub(crate) unsafe fn DoStatusInflictionScreenFlash(taskId: u8) {
    while sStatusInflictionScreenFlashFuncs[task_get(taskId, 0)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn StatusInflictionFadeOut(task: *mut Task) -> u8 {
    if (*task).data[6] == 0
        || ({
            (*task).data[6] -= 1;
            (*task).data[6]
        }) == 0
    {
        (*task).data[6] = (*task).data[1];
        (*task).data[7] += (*task).data[4];
        if (*task).data[7] > 16 {
            (*task).data[7] = 16;
        }
        BlendPalettes(PALETTES_ALL, (*task).data[7] as u8, 11627);
    }
    if (*task).data[7] >= 16 {
        (*task).data[0] += 1;
        (*task).data[6] = (*task).data[2];
    }
    FALSE
}
pub(crate) unsafe fn StatusInflictionFadeIn(task: *mut Task) -> u8 {
    if (*task).data[6] == 0
        || ({
            (*task).data[6] -= 1;
            (*task).data[6]
        }) == 0
    {
        (*task).data[6] = (*task).data[2];
        (*task).data[7] -= (*task).data[5];
        if (*task).data[7] < 0 {
            (*task).data[7] = 0;
        }
        BlendPalettes(PALETTES_ALL, (*task).data[7] as u8, 11627);
    }
    if (*task).data[7] == 0 {
        if ({
            (*task).data[3] -= 1;
            (*task).data[3]
        }) == 0
        {
            DestroyTask(FindTaskIdByFunc(Some(DoStatusInflictionScreenFlash)));
        } else {
            (*task).data[6] = (*task).data[1];
            (*task).data[0] = 0;
        }
    }
    FALSE
}
unsafe fn StartStatusInflictionScreenFlash(
    fadeOutDelay: i16,
    fadeInDelay: i16,
    numFades: i16,
    fadeOutSpeed: i16,
    fadeInSpped: i16,
) {
    let taskId: u8 = CreateTask(Some(DoStatusInflictionScreenFlash), 3);
    task_set(taskId, 1, fadeOutDelay);
    task_set(taskId, 2, fadeInDelay);
    task_set(taskId, 3, numFades);
    task_set(taskId, 4, fadeOutSpeed);
    task_set(taskId, 5, fadeInSpped);
    task_set(taskId, 6, fadeOutDelay);
}
unsafe fn IsStatusInflictionScreenFlashTaskFinished() -> u8 {
    if FindTaskIdByFunc(Some(DoStatusInflictionScreenFlash)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_DoStatusInflictionScreenFlash(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        StartStatusInflictionScreenFlash(0, 0, 3, 2, 2);
    } else {
        if IsStatusInflictionScreenFlashTaskFinished() != 0 {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
    }
}
unsafe fn TryHealMons(mut healCount: u8) {
    let mut j: u8 = 0;
    let mut indices: CArray<u8, 3> = zeroed();
    if healCount == 0 {
        return;
    }
    let mut i: u8 = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        indices[i] = i;
        i += 1;
    }
    for k in 0..10u8 {
        i = (Random() as i32 % 3) as u8;
        j = (Random() as i32 % 3) as u8;
        let temp: u8 = indices[i];
        indices[i] = indices[j];
        indices[j] = temp;
    }
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        let mut canBeHealed: u32 = FALSE as u32;
        let mon: *mut Pokemon = &raw mut gPlayerParty[indices[i]];
        let mut curr: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
        let mut max: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
        if curr < max {
            canBeHealed = TRUE as u32;
        } else if GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) != AILMENT_NONE {
            canBeHealed = TRUE as u32;
        } else {
            let ppBonuses: u8 = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
            for j in 0..(MAX_MON_MOVES as u8) {
                let r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + j as i32) as u16;
                max = CalculatePPWithBonus(r#move, ppBonuses, j) as u16;
                curr = GetMonData2(mon, MON_DATA_PP1 + j as i32) as u16;
                if curr < max {
                    canBeHealed = TRUE as u32;
                    break;
                }
            }
        }
        if canBeHealed == TRUE as u32 {
            HealMon(&raw mut gPlayerParty[indices[i]]);
            if ({
                healCount -= 1;
                healCount
            }) == 0
            {
                break;
            }
        }
    }
}
pub(crate) unsafe fn GetInBattlePike() {
    gSpecialVar_Result = InBattlePike() as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn InBattlePike() -> u8 {
    (gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_THREE_PATH_ROOM
        || gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_ROOM_NORMAL
        || gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_ROOM_WILD_MONS
        || gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PIKE_ROOM_UNUSED) as u8
}
pub(crate) unsafe fn SetHintedRoom() {
    let mut count: u8 = 0;
    let mut id: u8 = 0;
    let mut roomCandidates: *mut u8 = null_mut();
    gSpecialVar_Result = FALSE as u16;
    if GetPikeQueenFightType(1) != 0 {
        gSpecialVar_Result = TRUE as u16;
        (*gSaveBlock2Ptr)
            .frontier
            .set_pikeHintedRoomIndex((Random() as i32 % 6) as u8);
        (*gSaveBlock2Ptr)
            .frontier
            .set_pikeHintedRoomType(PIKE_ROOM_BRAIN);
    } else {
        (*gSaveBlock2Ptr)
            .frontier
            .set_pikeHintedRoomIndex((Random() as i32 % 3) as u8);
        if (*gSaveBlock2Ptr).frontier.pikeHealingRoomsDisabled() != 0 {
            count = 6;
        } else {
            count = 8;
        }
        roomCandidates = AllocZeroed(count as u32) as *mut u8;
        id = 0;
        for i in 0..count {
            if (*gSaveBlock2Ptr).frontier.pikeHealingRoomsDisabled() != 0 {
                if i != PIKE_ROOM_HEAL_FULL && i != PIKE_ROOM_HEAL_PART {
                    *roomCandidates.at({
                        let t1 = id;
                        id += 1;
                        t1
                    }) = i;
                }
            } else {
                *roomCandidates.at(i) = i;
            }
        }
        (*gSaveBlock2Ptr)
            .frontier
            .set_pikeHintedRoomType(*roomCandidates.at(rem_i32(Random() as i32, count as i32)));
        Free(roomCandidates as *mut c_void);
        if (*gSaveBlock2Ptr).frontier.pikeHintedRoomType() == PIKE_ROOM_STATUS
            && AtLeastOneHealthyMon() == 0
        {
            (*gSaveBlock2Ptr)
                .frontier
                .set_pikeHintedRoomType(PIKE_ROOM_NPC);
        }
        if (*gSaveBlock2Ptr).frontier.pikeHintedRoomType() == PIKE_ROOM_DOUBLE_BATTLE
            && AtLeastTwoAliveMons() == 0
        {
            (*gSaveBlock2Ptr)
                .frontier
                .set_pikeHintedRoomType(PIKE_ROOM_NPC);
        }
    }
}
pub(crate) unsafe fn GetHintedRoomIndex() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.pikeHintedRoomIndex() as u16;
}
pub(crate) unsafe fn GetRoomTypeHint() {
    gSpecialVar_Result = sRoomTypeHints[(*gSaveBlock2Ptr).frontier.pikeHintedRoomType()] as u16;
}
unsafe fn PrepareOneTrainer(difficult: u8) {
    let mut i: i32 = 0;
    let mut lvlMode: u8 = 0;
    let mut battleNum: u8 = 0;
    let mut trainerId: u16 = 0;
    if difficult == 0 {
        battleNum = 1;
    } else {
        battleNum = 6;
    }
    lvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
    let challengeNum: u16 = ((*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] as i32 / 14) as u16;
    loop {
        trainerId = GetRandomScaledFrontierTrainerId(challengeNum as u8, battleNum);
        i = 0;
        while i < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1 {
            if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                break;
            }
            i += 1;
        }
        if i == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1 {
            break;
        }
    }
    gTrainerBattleOpponent_A = trainerId;
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gBattleFrontierTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum < NUM_PIKE_ROOMS {
        (*gSaveBlock2Ptr).frontier.trainerIds
            [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1] =
            gTrainerBattleOpponent_A;
    }
}
unsafe fn PrepareTwoTrainers() {
    let mut i: i32 = 0;
    let mut trainerId: u16 = 0;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let challengeNum: u16 = ((*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] as i32 / 14) as u16;
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gBattleFrontierTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
    loop {
        trainerId = GetRandomScaledFrontierTrainerId(challengeNum as u8, 1);
        i = 0;
        while i < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1 {
            if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                break;
            }
            i += 1;
        }
        if i == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1 {
            break;
        }
    }
    gTrainerBattleOpponent_A = trainerId;
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum <= NUM_PIKE_ROOMS {
        (*gSaveBlock2Ptr).frontier.trainerIds
            [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1] =
            gTrainerBattleOpponent_A;
    }
    loop {
        trainerId = GetRandomScaledFrontierTrainerId(challengeNum as u8, 1);
        i = 0;
        while i < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
            if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                break;
            }
            i += 1;
        }
        if i == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
            break;
        }
    }
    gTrainerBattleOpponent_B = trainerId;
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_B, 1);
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum < NUM_PIKE_ROOMS {
        (*gSaveBlock2Ptr).frontier.trainerIds
            [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 2] =
            gTrainerBattleOpponent_B;
    }
}
pub(crate) unsafe fn ClearPikeTrainerIds() {
    for i in 0..(NUM_PIKE_ROOMS as u8) {
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = 0xFFFF;
    }
}
pub(crate) unsafe fn BufferTrainerIntro() {
    if gSpecialVar_0x8005 == 0 {
        if gTrainerBattleOpponent_A < FRONTIER_TRAINERS_COUNT {
            FrontierSpeechToString(
                (*gFacilityTrainers.at(gTrainerBattleOpponent_A))
                    .speechBefore
                    .as_mut_ptr(),
            );
        }
    } else if gSpecialVar_0x8005 == 1 && gTrainerBattleOpponent_B < FRONTIER_TRAINERS_COUNT {
        FrontierSpeechToString(
            (*gFacilityTrainers.at(gTrainerBattleOpponent_B))
                .speechBefore
                .as_mut_ptr(),
        );
    }
}
unsafe fn AtLeastTwoAliveMons() -> u8 {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[0];
    let mut countDead: u8 = 0;
    let mut i: u8 = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        if GetMonData2(mon, MON_DATA_HP) == 0 {
            countDead += 1;
        }
        i += 1;
        mon = mon.at(1);
    }
    if countDead >= 2 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetPikeQueenFightType(nextRoom: u8) -> u8 {
    let facility: u8 = FRONTIER_FACILITY_PIKE as u8;
    let mut ret: u8 = FRONTIER_BRAIN_NOT_READY;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode];
    winStreak += nextRoom as u16;
    let numPikeSymbols: u8 = GetPlayerSymbolCountForFacility(FRONTIER_FACILITY_PIKE as u8);
    match numPikeSymbols {
        0 | 1 => {
            if winStreak as i32
                == sFrontierBrainStreakAppearances[facility][numPikeSymbols] as i32
                    - sFrontierBrainStreakAppearances[facility][3] as i32
            {
                ret = numPikeSymbols + 1;
            }
        }
        _ => {
            if winStreak as i32
                == sFrontierBrainStreakAppearances[facility][0] as i32
                    - sFrontierBrainStreakAppearances[facility][3] as i32
            {
                ret = FRONTIER_BRAIN_STREAK;
            } else if winStreak as i32
                == sFrontierBrainStreakAppearances[facility][1] as i32
                    - sFrontierBrainStreakAppearances[facility][3] as i32
                || winStreak > sFrontierBrainStreakAppearances[facility][1] as u16
                    && rem_i32(
                        winStreak as i32 - sFrontierBrainStreakAppearances[facility][1] as i32
                            + sFrontierBrainStreakAppearances[facility][3] as i32,
                        sFrontierBrainStreakAppearances[facility][2] as i32,
                    ) == 0
            {
                ret = FRONTIER_BRAIN_STREAK_LONG as u8;
            }
        }
    }
    ret
}
pub(crate) unsafe fn GetCurrentRoomPikeQueenFightType() {
    gSpecialVar_Result = GetPikeQueenFightType(0) as u16;
}
pub(crate) unsafe fn HealSomeMonsBeforePikeQueen() {
    let toHealCount: u8 = sNumMonsToHealBeforePikeQueen
        [(*gSaveBlock2Ptr).frontier.pikeHintedRoomIndex()]
        [*(&raw const crate::ffi::gSpecialVar_0x8007)
            .cast::<u16>()
            .cast_mut()];
    TryHealMons(toHealCount);
    gSpecialVar_Result = toHealCount as u16;
}
pub(crate) unsafe fn SetHealingroomTypesDisabled() {
    (*gSaveBlock2Ptr)
        .frontier
        .set_pikeHealingRoomsDisabled(gSpecialVar_0x8005 as u8);
}
pub(crate) unsafe fn IsPartyFullHealed() {
    gSpecialVar_Result = TRUE as u16;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        let mut canBeHealed: u32 = FALSE as u32;
        let mon: *mut Pokemon = &raw mut gPlayerParty[i];
        let mut curr: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
        let mut max: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
        if curr >= max && GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) == AILMENT_NONE {
            let ppBonuses: u8 = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
            for j in 0..(MAX_MON_MOVES as u8) {
                let r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + j as i32) as u16;
                max = CalculatePPWithBonus(r#move, ppBonuses, j) as u16;
                curr = GetMonData2(mon, MON_DATA_PP1 + j as i32) as u16;
                if curr < max {
                    canBeHealed = TRUE as u32;
                    break;
                }
            }
        } else {
            canBeHealed = TRUE as u32;
        }
        if canBeHealed == TRUE as u32 {
            gSpecialVar_Result = FALSE as u16;
            break;
        }
    }
}
pub(crate) unsafe fn SaveMonHeldItems() {
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        let heldItem: i32 = GetMonData2(
            &raw mut (*gSaveBlock1Ptr).playerParty
                [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
            MON_DATA_HELD_ITEM,
        ) as i32;
        (*gSaveBlock2Ptr).frontier.pikeHeldItemsBackup[i] = heldItem as u16;
    }
}
pub(crate) unsafe fn RestoreMonHeldItems() {
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        SetMonData(
            &raw mut gPlayerParty[(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
            MON_DATA_HELD_ITEM,
            &raw mut (*gSaveBlock2Ptr).frontier.pikeHeldItemsBackup[i] as *mut c_void,
        );
    }
}
pub(crate) unsafe fn InitPikeChallenge() {
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    if (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & sWinStreakFlags[lvlMode] == 0 {
        (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] = 0;
    }
    gTrainerBattleOpponent_A = 0;
    gBattleOutcome = 0;
}
unsafe fn CanEncounterWildMon(enemyMonLevel: u8) -> u8 {
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0 {
        let monAbility: u8 = GetMonAbility(&raw mut gPlayerParty[0]);
        if monAbility == ABILITY_KEEN_EYE || monAbility == ABILITY_INTIMIDATE {
            let playerMonLevel: u8 = GetMonData2(&raw mut gPlayerParty[0], MON_DATA_LEVEL) as u8;
            if playerMonLevel > 5
                && enemyMonLevel as i32 <= playerMonLevel as i32 - 5
                && Random() as i32 % 2 == 0
            {
                return FALSE;
            }
        }
    }
    TRUE
}
fn SpeciesToPikeMonId(species: u16) -> u8 {
    let mut ret: u8 = 0;
    if species == SPECIES_SEVIPER {
        ret = 0;
    } else if species == SPECIES_MILOTIC {
        ret = 1;
    } else {
        ret = 2;
    }
    ret
}
