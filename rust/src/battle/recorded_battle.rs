//! Translated from `src/recorded_battle.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gGameLanguage;
use crate::agb_main::gMain;
use crate::battle_anim_mons::GetBattlerSide;
use crate::battle_main::{
    CB2_InitBattle, CB2_QuitRecordedBattle, gActiveBattler, gBattleMons, gBattleOutcome,
    gBattleResources, gBattleStruct, gBattleTypeFlags, gBattlersCount, gDisableStructs,
};
use crate::battle_main::{gBattlerPartyIndexes, gChosenMoveByBattler};
use crate::battle_setup::{gPartnerTrainerId, gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarGet;
use crate::ffi::gSpecialVar_Result;
use crate::frontier_util::GetFronterBrainSymbol;
use crate::link::{GetLinkPlayerCount, GetMultiplayerId, gLinkPlayers};
use crate::load_save::gSaveBlock2Ptr;
use crate::palette::{BeginNormalPaletteFade, ResetPaletteFadeControl};
use crate::pokemon::{
    GetMonData3, PlayMapChosenOrBattleBGM, SetMonData, ZeroEnemyPartyMons, ZeroPlayerPartyMons,
    gEnemyParty, gPlayerParty,
};
use crate::save::{TryReadSpecialSaveSector, TryWriteSpecialSaveSector};
use crate::sprite::{AnimateSprites, BuildOamBuffer};
use crate::string_util::StringCopy;
use crate::string_util::{ConvertInternationalString, StripExtCtrlCodes};
use crate::task::{DestroyTask, RunTasks};
use crate::task::{task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::CalcByteArraySum;
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const tFramesToWait: usize = 0;

/// `struct PlayerInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PlayerInfo {
    pub trainerId: u32,
    pub name: CArray<u8, 8>,
    pub gender: u8,
    pub battler: u16,
    pub language: u16,
}

unsafe impl Sync for PlayerInfo {}

/// `struct RecordedBattleSave`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RecordedBattleSave {
    pub playerParty: CArray<Pokemon, 6>,
    pub opponentParty: CArray<Pokemon, 6>,
    pub playersName: CArray<CArray<u8, 8>, 4>,
    pub playersGender: CArray<u8, 4>,
    pub playersTrainerId: CArray<u32, 4>,
    pub playersLanguage: CArray<u8, 4>,
    pub rngSeed: u32,
    pub battleFlags: u32,
    pub playersBattlers: CArray<u8, 4>,
    pub opponentA: u16,
    pub opponentB: u16,
    pub partnerId: u16,
    pub multiplayerId: u16,
    pub lvlMode: u8,
    pub frontierFacility: u8,
    pub frontierBrainSymbol: u8,
    bits_1279: u8,
    pub AI_scripts: u32,
    pub recordMixFriendName: CArray<u8, 8>,
    pub recordMixFriendClass: u8,
    pub apprenticeId: u8,
    pub easyChatSpeech: CArray<u16, 6>,
    pub recordMixFriendLanguage: u8,
    pub apprenticeLanguage: u8,
    pub battleRecord: CArray<CArray<u8, 664>, 4>,
    pub checksum: u32,
}

impl RecordedBattleSave {
    #[inline(always)]
    pub fn battleScene(&self) -> u8 {
        ((self.bits_1279 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_battleScene(&mut self, v: u8) {
        self.bits_1279 = (self.bits_1279 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn textSpeed(&self) -> u8 {
        ((self.bits_1279 as u32 >> 1) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_textSpeed(&mut self, v: u8) {
        self.bits_1279 = (self.bits_1279 & !(0x7 << 1)) | ((v & 0x7) << 1);
    }
}

unsafe impl Sync for RecordedBattleSave {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PlayerInfo>() == 20);
    assert!(offset_of!(PlayerInfo, trainerId) == 0);
    assert!(offset_of!(PlayerInfo, name) == 4);
    assert!(offset_of!(PlayerInfo, gender) == 12);
    assert!(offset_of!(PlayerInfo, battler) == 14);
    assert!(offset_of!(PlayerInfo, language) == 16);
    assert!(size_of::<RecordedBattleSave>() == 3968);
    assert!(offset_of!(RecordedBattleSave, playerParty) == 0);
    assert!(offset_of!(RecordedBattleSave, opponentParty) == 600);
    assert!(offset_of!(RecordedBattleSave, playersName) == 1200);
    assert!(offset_of!(RecordedBattleSave, playersGender) == 1232);
    assert!(offset_of!(RecordedBattleSave, playersTrainerId) == 1236);
    assert!(offset_of!(RecordedBattleSave, playersLanguage) == 1252);
    assert!(offset_of!(RecordedBattleSave, rngSeed) == 1256);
    assert!(offset_of!(RecordedBattleSave, battleFlags) == 1260);
    assert!(offset_of!(RecordedBattleSave, playersBattlers) == 1264);
    assert!(offset_of!(RecordedBattleSave, opponentA) == 1268);
    assert!(offset_of!(RecordedBattleSave, opponentB) == 1270);
    assert!(offset_of!(RecordedBattleSave, partnerId) == 1272);
    assert!(offset_of!(RecordedBattleSave, multiplayerId) == 1274);
    assert!(offset_of!(RecordedBattleSave, lvlMode) == 1276);
    assert!(offset_of!(RecordedBattleSave, frontierFacility) == 1277);
    assert!(offset_of!(RecordedBattleSave, frontierBrainSymbol) == 1278);
    assert!(offset_of!(RecordedBattleSave, bits_1279) == 1279);
    assert!(offset_of!(RecordedBattleSave, AI_scripts) == 1280);
    assert!(offset_of!(RecordedBattleSave, recordMixFriendName) == 1284);
    assert!(offset_of!(RecordedBattleSave, recordMixFriendClass) == 1292);
    assert!(offset_of!(RecordedBattleSave, apprenticeId) == 1293);
    assert!(offset_of!(RecordedBattleSave, easyChatSpeech) == 1294);
    assert!(offset_of!(RecordedBattleSave, recordMixFriendLanguage) == 1306);
    assert!(offset_of!(RecordedBattleSave, apprenticeLanguage) == 1307);
    assert!(offset_of!(RecordedBattleSave, battleRecord) == 1308);
    assert!(offset_of!(RecordedBattleSave, checksum) == 3964);
};

const ACTION_MOVE_CHANGE: u8 = 6;
const BATTLER_RECORD_SIZE: i32 = 664;

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gRecordedBattleRngSeed: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlePalaceMoveSelectionRngValue: crate::global::Global<u32> =
    crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattleRecords: crate::global::Global<Aligned<CArray<CArray<u8, 664>, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattlerRecordSizes: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattlerPrevRecordSizes: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattlerSavedRecordSizes: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRecordMode: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sLvlMode: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sFrontierFacility: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sFrontierBrainSymbol: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sCallback2_AfterRecordedBattle: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gRecordedBattleMultiplayerId: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sFrontierPassFlag: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattleScene: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sTextSpeed: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattleFlags: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sAI_Scripts: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSavedPlayerParty: crate::global::Global<CArray<Pokemon, 6>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSavedOpponentParty: crate::global::Global<CArray<Pokemon, 6>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPlayerMonMoves: crate::global::Global<Aligned<CArray<CArray<u16, 4>, 2>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPlayers: crate::global::Global<CArray<PlayerInfo, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sIsPlaybackFinished: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRecordMixFriendName: crate::global::Global<Aligned<CArray<u8, 8>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRecordMixFriendClass: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sApprenticeId: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sEasyChatSpeech: crate::global::Global<Aligned<CArray<u16, 6>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattleOutcome: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sRecordMixFriendLanguage: crate::global::Global<u8> =
    crate::global::Global::new(0);
pub(crate) static sApprenticeLanguage: crate::global::Global<u8> = crate::global::Global::new(0);

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn RecordedBattle_Init(mode: u8) {
    sRecordMode.set(mode);
    sIsPlaybackFinished.set(FALSE);
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        crate::gl!(sBattlerRecordSizes[i] = 0);
        crate::gl!(sBattlerPrevRecordSizes[i] = 0);
        crate::gl!(sBattlerSavedRecordSizes[i] = 0);
        if mode == B_RECORD_MODE_RECORDING {
            for j in 0..BATTLER_RECORD_SIZE {
                crate::gl!(sBattleRecords[i][j] = 0xFF);
            }
            sBattleFlags.set(gBattleTypeFlags.get());
            sAI_Scripts.set((*(*crate::gl!(gBattleResources)).ai).aiFlags);
        }
    }
}
pub unsafe fn RecordedBattle_SetTrainerInfo() {
    if sRecordMode.get() == B_RECORD_MODE_RECORDING {
        gRecordedBattleRngSeed.set(*crate::random::gRngValue.as_ptr().cast::<u32>());
        sFrontierFacility.set(VarGet(VAR_FRONTIER_FACILITY) as u8);
        sFrontierBrainSymbol.set(GetFronterBrainSymbol() as u8);
    } else if sRecordMode.get() == B_RECORD_MODE_PLAYBACK {
        (*crate::random::gRngValue.as_ptr().cast::<u32>()) = gRecordedBattleRngSeed.get();
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        let mut text: CArray<u8, 30> = zeroed();
        gRecordedBattleMultiplayerId.set(GetMultiplayerId());
        let linkPlayersCount: u8 = GetLinkPlayerCount();
        for i in 0..MAX_LINK_PLAYERS {
            crate::gl!(sPlayers[i].trainerId = crate::gl!(gLinkPlayers[i].trainerId));
            crate::gl!(sPlayers[i].gender = crate::gl!(gLinkPlayers[i].gender));
            crate::gl!(sPlayers[i].battler = crate::gl!(gLinkPlayers[i].id));
            crate::gl!(sPlayers[i].language = crate::gl!(gLinkPlayers[i].language));
            if i < linkPlayersCount as i32 {
                StringCopy(
                    text.as_mut_ptr(),
                    (*gLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
                );
                StripExtCtrlCodes(text.as_mut_ptr());
                StringCopy((*sPlayers.as_ptr())[i].name.as_mut_ptr(), text.as_mut_ptr());
            } else {
                for j in 0..8i32 {
                    crate::gl!(sPlayers[i].name[j] = crate::gl!(gLinkPlayers[i].name[j]));
                }
            }
        }
    } else {
        crate::gl!(
            sPlayers[0].trainerId = (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[0] as u32
                | ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[1] as u32) << 8
                | ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[2] as u32) << 16
                | ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[3] as u32) << 24
        );
        crate::gl!(sPlayers[0].gender = (*crate::gl!(gSaveBlock2Ptr)).playerGender);
        crate::gl!(sPlayers[0].battler = 0);
        crate::gl!(sPlayers[0].language = gGameLanguage as u16);
        for i in 0..8i32 {
            crate::gl!(sPlayers[0].name[i] = (*crate::gl!(gSaveBlock2Ptr)).playerName[i]);
        }
    }
}
pub unsafe fn RecordedBattle_SetBattlerAction(battler: u8, action: u8) {
    if crate::gl!(sBattlerRecordSizes[battler]) < BATTLER_RECORD_SIZE as u16
        && sRecordMode.get() != B_RECORD_MODE_PLAYBACK
    {
        (*sBattleRecords.as_ptr())[battler][{
            let t1 = crate::gl!(sBattlerRecordSizes[battler]);
            crate::gl!(sBattlerRecordSizes[battler] += 1);
            t1
        }] = action;
    }
}
pub unsafe fn RecordedBattle_ClearBattlerAction(battler: u8, bytesToClear: u8) {
    for i in 0..(bytesToClear as i32) {
        crate::gl!(sBattlerRecordSizes[battler] -= 1);
        (*sBattleRecords.as_ptr())[battler][crate::gl!(sBattlerRecordSizes[battler])] = 0xFF;
        if crate::gl!(sBattlerRecordSizes[battler]) == 0 {
            break;
        }
    }
}
pub unsafe fn RecordedBattle_GetBattlerAction(battler: u8) -> u8 {
    if crate::gl!(sBattlerRecordSizes[battler]) >= BATTLER_RECORD_SIZE as u16
        || (*sBattleRecords.as_ptr())[battler][crate::gl!(sBattlerRecordSizes[battler])] == 0xFF
    {
        gSpecialVar_Result = ({
            gBattleOutcome.set(B_OUTCOME_PLAYER_TELEPORTED);
            gBattleOutcome.get()
        }) as u16;
        ResetPaletteFadeControl();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        SetMainCallback2(Some(CB2_QuitRecordedBattle));
        return B_ACTION_NONE;
    } else {
        return (*sBattleRecords.as_ptr())[battler][{
            let t1 = crate::gl!(sBattlerRecordSizes[battler]);
            crate::gl!(sBattlerRecordSizes[battler] += 1);
            t1
        }];
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn GetRecordedBattleMode() -> u8 {
    sRecordMode.get()
}
pub unsafe fn RecordedBattle_BufferNewBattlerData(dst: *mut u8) -> u8 {
    let mut j: u8 = 0;
    let mut idx: u8 = 0;
    for i in 0..MAX_BATTLERS_COUNT {
        if crate::gl!(sBattlerRecordSizes[i]) != crate::gl!(sBattlerPrevRecordSizes[i]) {
            *dst.at({
                let t1 = idx;
                idx += 1;
                t1
            }) = i;
            *dst.at({
                let t2 = idx;
                idx += 1;
                t2
            }) = crate::gl!(sBattlerRecordSizes[i]) as u8
                - crate::gl!(sBattlerPrevRecordSizes[i]) as u8;
            j = 0;
            while (j as i32)
                < crate::gl!(sBattlerRecordSizes[i]) as i32
                    - crate::gl!(sBattlerPrevRecordSizes[i]) as i32
            {
                *dst.at({
                    let t3 = idx;
                    idx += 1;
                    t3
                }) = (*sBattleRecords.as_ptr())[i]
                    [crate::gl!(sBattlerPrevRecordSizes[i]) as i32 + j as i32];
                j += 1;
            }
            crate::gl!(sBattlerPrevRecordSizes[i] = crate::gl!(sBattlerRecordSizes[i]));
        }
    }
    idx
}
pub unsafe fn RecordedBattle_RecordAllBattlerData(src: *mut u8) {
    let mut idx: u8 = 2;
    let mut size: u8 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0 {
        return;
    }
    let mut i: i32 = 0;
    while i < GetLinkPlayerCount() as i32 {
        if crate::gl!(gLinkPlayers[i].version) as i32 & 0xFF != VERSION_EMERALD as i32 {
            return;
        }
        i += 1;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0 {
        size = *src;
        while size != 0 {
            let battler: u8 = GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
            let numActions: u8 = GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
            for i in 0..(numActions as i32) {
                (*sBattleRecords.as_ptr())[battler][{
                    let t1 = crate::gl!(sBattlerSavedRecordSizes[battler]);
                    crate::gl!(sBattlerSavedRecordSizes[battler] += 1);
                    t1
                }] = GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
            }
        }
    }
}
unsafe fn GetNextRecordedDataByte(data: *mut u8, idx: *mut u8, size: *mut u8) -> u8 {
    *size -= 1;
    *data.at({
        let t1 = *idx;
        *idx += 1;
        t1
    })
}
pub unsafe fn CanCopyRecordedBattleSaveData() -> u32 {
    let dst: *mut RecordedBattleSave = AllocZeroed(3968) as *mut RecordedBattleSave;
    let ret: u32 = CopyRecordedBattleFromSave(dst);
    Free(dst as *mut c_void);
    ret
}
unsafe fn IsRecordedBattleSaveValid(save: *mut RecordedBattleSave) -> u32 {
    if (*save).battleFlags == 0 {
        return FALSE as u32;
    }
    if (*save).battleFlags & BATTLE_TYPE_RECORDED_INVALID != 0 {
        return FALSE as u32;
    }
    if CalcByteArraySum(save as *mut c_void as *mut u8, 3964) != (*save).checksum {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn RecordedBattleToSave(
    battleSave: *mut RecordedBattleSave,
    saveSector: *mut RecordedBattleSave,
) -> u32 {
    memset(saveSector as *mut u8, 0, SECTOR_SIZE);
    memcpy(saveSector as *mut u8, battleSave as *mut u8, 3968);
    (*saveSector).checksum = CalcByteArraySum(saveSector as *mut c_void as *mut u8, 3964);
    if TryWriteSpecialSaveSector(
        SECTOR_ID_RECORDED_BATTLE,
        saveSector as *mut c_void as *mut u8,
    ) != SAVE_STATUS_OK as u32
    {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn MoveRecordedBattleToSaveData() -> u32 {
    let mut ret: u32 = 0;
    let mut saveAttempts: u8 = 0;
    let battleSave: *mut RecordedBattleSave = AllocZeroed(3968) as *mut RecordedBattleSave;
    let savSection: *mut RecordedBattleSave = AllocZeroed(SECTOR_SIZE) as *mut RecordedBattleSave;
    for i in 0..PARTY_SIZE {
        (*battleSave).playerParty[i] = crate::gl!(sSavedPlayerParty[i]);
        (*battleSave).opponentParty[i] = crate::gl!(sSavedOpponentParty[i]);
    }
    let mut i: i32 = 0;
    while i < MAX_LINK_PLAYERS {
        for j in 0..8i32 {
            (*battleSave).playersName[i][j] = crate::gl!(sPlayers[i].name[j]);
        }
        (*battleSave).playersGender[i] = crate::gl!(sPlayers[i].gender);
        (*battleSave).playersLanguage[i] = crate::gl!(sPlayers[i].language) as u8;
        (*battleSave).playersBattlers[i] = crate::gl!(sPlayers[i].battler) as u8;
        (*battleSave).playersTrainerId[i] = crate::gl!(sPlayers[i].trainerId);
        i += 1;
    }
    (*battleSave).rngSeed = gRecordedBattleRngSeed.get();
    if sBattleFlags.get() & BATTLE_TYPE_LINK != 0 {
        (*battleSave).battleFlags = sBattleFlags.get() & 0xffffffdd | BATTLE_TYPE_RECORDED_LINK;
        if sBattleFlags.get() & BATTLE_TYPE_IS_MASTER != 0 {
            (*battleSave).battleFlags |= 0x80000000;
        } else if sBattleFlags.get() & BATTLE_TYPE_MULTI != 0 {
            match crate::gl!(sPlayers[0].battler) {
                0 | 2 => {
                    if {
                        let gl_i0 = gRecordedBattleMultiplayerId.get();
                        crate::gl!(sPlayers[gl_i0].battler)
                    } as i32
                        & 1
                        == 0
                    {
                        (*battleSave).battleFlags |= 0x80000000;
                    }
                }
                1 | 3
                    if {
                        let gl_i1 = gRecordedBattleMultiplayerId.get();
                        crate::gl!(sPlayers[gl_i1].battler)
                    } as i32
                        & 1
                        != 0 =>
                {
                    (*battleSave).battleFlags |= 0x80000000;
                }
                _ => {}
            }
        }
    } else {
        (*battleSave).battleFlags = sBattleFlags.get();
    }
    (*battleSave).opponentA = gTrainerBattleOpponent_A.get();
    (*battleSave).opponentB = gTrainerBattleOpponent_B.get();
    (*battleSave).partnerId = gPartnerTrainerId.get();
    (*battleSave).multiplayerId = gRecordedBattleMultiplayerId.get() as u16;
    (*battleSave).lvlMode = (*crate::gl!(gSaveBlock2Ptr)).frontier.lvlMode();
    (*battleSave).frontierFacility = sFrontierFacility.get();
    (*battleSave).frontierBrainSymbol = sFrontierBrainSymbol.get();
    (*battleSave).set_battleScene((*crate::gl!(gSaveBlock2Ptr)).optionsBattleSceneOff() as u8);
    (*battleSave).set_textSpeed((*crate::gl!(gSaveBlock2Ptr)).optionsTextSpeed() as u8);
    (*battleSave).AI_scripts = sAI_Scripts.get();
    if gTrainerBattleOpponent_A.get() >= TRAINER_RECORD_MIXING_FRIEND as u16
        && gTrainerBattleOpponent_A.get() < TRAINER_RECORD_MIXING_APPRENTICE as u16
    {
        i = 0;
        while i < 8 {
            (*battleSave).recordMixFriendName[i] = (*crate::gl!(gSaveBlock2Ptr))
                .frontier
                .towerRecords[gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .name[i];
            i += 1;
        }
        (*battleSave).recordMixFriendClass = (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
            [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
        if sBattleOutcome.get() == B_OUTCOME_WON {
            for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] =
                    (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
                        [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                        .speechLost[i];
            }
        } else {
            for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] =
                    (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
                        [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                        .speechWon[i];
            }
        }
        (*battleSave).recordMixFriendLanguage = (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
            [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .language;
    } else if gTrainerBattleOpponent_B.get() >= TRAINER_RECORD_MIXING_FRIEND as u16
        && gTrainerBattleOpponent_B.get() < TRAINER_RECORD_MIXING_APPRENTICE as u16
    {
        i = 0;
        while i < 8 {
            (*battleSave).recordMixFriendName[i] = (*crate::gl!(gSaveBlock2Ptr))
                .frontier
                .towerRecords[gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .name[i];
            i += 1;
        }
        (*battleSave).recordMixFriendClass = (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
            [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
        if sBattleOutcome.get() == B_OUTCOME_WON {
            for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] =
                    (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
                        [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                        .speechLost[i];
            }
        } else {
            for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] =
                    (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
                        [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                        .speechWon[i];
            }
        }
        (*battleSave).recordMixFriendLanguage = (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
            [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .language;
    } else if gPartnerTrainerId.get() >= TRAINER_RECORD_MIXING_FRIEND as u16
        && gPartnerTrainerId.get() < TRAINER_RECORD_MIXING_APPRENTICE as u16
    {
        for i in 0..8i32 {
            (*battleSave).recordMixFriendName[i] = (*crate::gl!(gSaveBlock2Ptr))
                .frontier
                .towerRecords[gPartnerTrainerId.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .name[i];
        }
        (*battleSave).recordMixFriendClass = (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
            [gPartnerTrainerId.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
        (*battleSave).recordMixFriendLanguage = (*crate::gl!(gSaveBlock2Ptr)).frontier.towerRecords
            [gPartnerTrainerId.get() as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .language;
    }
    if gTrainerBattleOpponent_A.get() >= TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        (*battleSave).apprenticeId = (*crate::gl!(gSaveBlock2Ptr)).apprentices
            [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id();
        for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
            (*battleSave).easyChatSpeech[i] = (*crate::gl!(gSaveBlock2Ptr)).apprentices
                [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .speechWon[i];
        }
        (*battleSave).apprenticeLanguage = (*crate::gl!(gSaveBlock2Ptr)).apprentices
            [gTrainerBattleOpponent_A.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .language;
    } else if gTrainerBattleOpponent_B.get() >= TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        (*battleSave).apprenticeId = (*crate::gl!(gSaveBlock2Ptr)).apprentices
            [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id();
        for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
            (*battleSave).easyChatSpeech[i] = (*crate::gl!(gSaveBlock2Ptr)).apprentices
                [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .speechWon[i];
        }
        (*battleSave).apprenticeLanguage = (*crate::gl!(gSaveBlock2Ptr)).apprentices
            [gTrainerBattleOpponent_B.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .language;
    } else if gPartnerTrainerId.get() >= TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        (*battleSave).apprenticeId = (*crate::gl!(gSaveBlock2Ptr)).apprentices
            [gPartnerTrainerId.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id();
        (*battleSave).apprenticeLanguage = (*crate::gl!(gSaveBlock2Ptr)).apprentices
            [gPartnerTrainerId.get() as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .language;
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        for j in 0..BATTLER_RECORD_SIZE {
            (*battleSave).battleRecord[i][j] = crate::gl!(sBattleRecords[i][j]);
        }
    }
    loop {
        ret = RecordedBattleToSave(battleSave, savSection);
        if ret == TRUE as u32 {
            break;
        }
        saveAttempts += 1;
        if saveAttempts >= 3 {
            break;
        }
    }
    Free(battleSave as *mut c_void);
    Free(savSection as *mut c_void);
    ret
}
unsafe fn TryCopyRecordedBattleSaveData(
    dst: *mut RecordedBattleSave,
    saveBuffer: *mut SaveSector,
) -> u32 {
    if TryReadSpecialSaveSector(
        SECTOR_ID_RECORDED_BATTLE,
        saveBuffer as *mut c_void as *mut u8,
    ) != SAVE_STATUS_OK as u32
    {
        return FALSE as u32;
    }
    memcpy(dst as *mut u8, saveBuffer as *mut u8, 3968);
    if IsRecordedBattleSaveValid(dst) == 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn CopyRecordedBattleFromSave(dst: *mut RecordedBattleSave) -> u32 {
    let savBuffer: *mut SaveSector = AllocZeroed(SECTOR_SIZE) as *mut SaveSector;
    let ret: u32 = TryCopyRecordedBattleSaveData(dst, savBuffer);
    Free(savBuffer as *mut c_void);
    ret
}
pub(crate) unsafe fn CB2_RecordedBattleEnd() {
    (*crate::gl!(gSaveBlock2Ptr))
        .frontier
        .set_lvlMode(sLvlMode.get());
    gBattleOutcome.set(0);
    gBattleTypeFlags.set(0);
    gTrainerBattleOpponent_A.set(0);
    gTrainerBattleOpponent_B.set(0);
    gPartnerTrainerId.set(0);
    RecordedBattle_RestoreSavedParties();
    SetMainCallback2(crate::gl!(sCallback2_AfterRecordedBattle));
}
pub(crate) unsafe fn Task_StartAfterCountdown(taskId: u8) {
    if ({
        task_set(taskId, tFramesToWait, task_get(taskId, tFramesToWait) - 1);
        task_get(taskId, tFramesToWait)
    }) == 0
    {
        crate::gl!(gMain.savedCallback = Some(CB2_RecordedBattleEnd));
        SetMainCallback2(Some(CB2_InitBattle));
        DestroyTask(taskId);
    }
}
unsafe fn SetVariablesForRecordedBattle(src: *mut RecordedBattleSave) {
    let mut var: u8 = 0;
    ZeroPlayerPartyMons();
    ZeroEnemyPartyMons();
    for i in 0..PARTY_SIZE {
        crate::gl!(gPlayerParty[i] = (*src).playerParty[i]);
        crate::gl!(gEnemyParty[i] = (*src).opponentParty[i]);
    }
    let mut i: i32 = 0;
    while i < MAX_LINK_PLAYERS {
        var = 0;
        for j in 0..8i32 {
            crate::gl!(gLinkPlayers[i].name[j] = (*src).playersName[i][j]);
            if (*src).playersName[i][j] == EOS {
                var = TRUE;
            }
        }
        crate::gl!(gLinkPlayers[i].gender = (*src).playersGender[i]);
        crate::gl!(gLinkPlayers[i].language = (*src).playersLanguage[i] as u16);
        crate::gl!(gLinkPlayers[i].id = (*src).playersBattlers[i] as u16);
        crate::gl!(gLinkPlayers[i].trainerId = (*src).playersTrainerId[i]);
        if var != 0 {
            ConvertInternationalString(
                (*gLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
                crate::gl!(gLinkPlayers[i].language) as u8,
            );
        }
        i += 1;
    }
    gRecordedBattleRngSeed.set((*src).rngSeed);
    gBattleTypeFlags.set((*src).battleFlags | BATTLE_TYPE_RECORDED);
    gTrainerBattleOpponent_A.set((*src).opponentA);
    gTrainerBattleOpponent_B.set((*src).opponentB);
    gPartnerTrainerId.set((*src).partnerId);
    gRecordedBattleMultiplayerId.set((*src).multiplayerId as u8);
    sLvlMode.set((*crate::gl!(gSaveBlock2Ptr)).frontier.lvlMode());
    sFrontierFacility.set((*src).frontierFacility);
    sFrontierBrainSymbol.set((*src).frontierBrainSymbol);
    sBattleScene.set((*src).battleScene());
    sTextSpeed.set((*src).textSpeed());
    sAI_Scripts.set((*src).AI_scripts);
    for i in 0..8i32 {
        crate::gl!(sRecordMixFriendName[i] = (*src).recordMixFriendName[i]);
    }
    sRecordMixFriendClass.set((*src).recordMixFriendClass);
    sApprenticeId.set((*src).apprenticeId);
    sRecordMixFriendLanguage.set((*src).recordMixFriendLanguage);
    sApprenticeLanguage.set((*src).apprenticeLanguage);
    i = 0;
    while i < EASY_CHAT_BATTLE_WORDS_COUNT {
        crate::gl!(sEasyChatSpeech[i] = (*src).easyChatSpeech[i]);
        i += 1;
    }
    (*crate::gl!(gSaveBlock2Ptr))
        .frontier
        .set_lvlMode((*src).lvlMode);
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        for j in 0..BATTLER_RECORD_SIZE {
            crate::gl!(sBattleRecords[i][j] = (*src).battleRecord[i][j]);
        }
    }
}
pub unsafe fn PlayRecordedBattle(CB2_After: Option<unsafe fn()>) {
    let battleSave: *mut RecordedBattleSave = AllocZeroed(3968) as *mut RecordedBattleSave;
    if CopyRecordedBattleFromSave(battleSave) == TRUE as u32 {
        RecordedBattle_SaveParties();
        SetVariablesForRecordedBattle(battleSave);
        let taskId: u8 = CreateTask(Some(Task_StartAfterCountdown), 1);
        task_set(taskId, tFramesToWait, 128);
        crate::gl!(sCallback2_AfterRecordedBattle = CB2_After);
        PlayMapChosenOrBattleBGM(FALSE as u16);
        SetMainCallback2(Some(CB2_RecordedBattle));
    }
    Free(battleSave as *mut c_void);
}
pub(crate) unsafe fn CB2_RecordedBattle() {
    AnimateSprites();
    BuildOamBuffer();
    RunTasks();
}
pub fn GetRecordedBattleFrontierFacility() -> u8 {
    sFrontierFacility.get()
}
pub fn GetRecordedBattleFronterBrainSymbol() -> u8 {
    sFrontierBrainSymbol.get()
}
pub fn RecordedBattle_SaveParties() {
    for i in 0..PARTY_SIZE {
        crate::gl!(sSavedPlayerParty[i] = crate::gl!(gPlayerParty[i]));
        crate::gl!(sSavedOpponentParty[i] = crate::gl!(gEnemyParty[i]));
    }
}
fn RecordedBattle_RestoreSavedParties() {
    for i in 0..PARTY_SIZE {
        crate::gl!(gPlayerParty[i] = crate::gl!(sSavedPlayerParty[i]));
        crate::gl!(gEnemyParty[i] = crate::gl!(sSavedOpponentParty[i]));
    }
}
pub fn GetActiveBattlerLinkPlayerGender() -> u8 {
    let mut i: i32 = 0;
    while i < MAX_LINK_PLAYERS {
        if crate::gl!(gLinkPlayers[i].id) == gActiveBattler.get() as u16 {
            break;
        }
        i += 1;
    }
    if i != MAX_LINK_PLAYERS {
        return crate::gl!(gLinkPlayers[i].gender);
    }
    0
}
pub fn RecordedBattle_ClearFrontierPassFlag() {
    sFrontierPassFlag.set(0);
}
pub fn RecordedBattle_SetFrontierPassFlagFromHword(flags: u16) {
    sFrontierPassFlag.set(sFrontierPassFlag.get() | (((flags as i32 & 32768) >> 15) as u8));
}
pub unsafe fn RecordedBattle_GetFrontierPassFlag() -> u8 {
    sFrontierPassFlag.get()
}
pub fn GetBattleSceneInRecordedBattle() -> u8 {
    sBattleScene.get()
}
pub fn GetTextSpeedInRecordedBattle() -> u8 {
    sTextSpeed.get()
}
pub unsafe fn RecordedBattle_CopyBattlerMoves() {
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT {
        return;
    }
    if gBattleTypeFlags.get() & 0x2000002 != 0 {
        return;
    }
    if sRecordMode.get() == B_RECORD_MODE_PLAYBACK {
        return;
    }
    for i in 0..MAX_MON_MOVES {
        {
            let gl_v = {
                let gl_i0 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i0].moves[i])
            };
            let gl_i2 = gActiveBattler.get() as i32 / 2;
            crate::gl!(sPlayerMonMoves[gl_i2][i] = gl_v)
        };
    }
}
pub unsafe fn RecordedBattle_CheckMovesetChanges(mode: u8) {
    let mut j: i32 = 0;
    if gBattleTypeFlags.get() & 0x2000002 != 0 {
        return;
    }
    let mut battler: i32 = 0;
    while battler < gBattlersCount.get() as i32 {
        if GetBattlerSide(battler as u8) != B_SIDE_OPPONENT {
            if mode == B_RECORD_MODE_RECORDING {
                j = 0;
                while j < MAX_MON_MOVES {
                    if crate::gl!(gBattleMons[battler].moves[j]) != {
                        let gl_i3 = battler / 2;
                        crate::gl!(sPlayerMonMoves[gl_i3][j])
                    } {
                        break;
                    }
                    j += 1;
                }
                if j != MAX_MON_MOVES {
                    RecordedBattle_SetBattlerAction(battler as u8, ACTION_MOVE_CHANGE);
                    for j in 0..MAX_MON_MOVES {
                        for k in 0..MAX_MON_MOVES {
                            if crate::gl!(gBattleMons[battler].moves[j]) == {
                                let gl_i4 = battler / 2;
                                crate::gl!(sPlayerMonMoves[gl_i4][k])
                            } {
                                RecordedBattle_SetBattlerAction(battler as u8, k as u8);
                                break;
                            }
                        }
                    }
                }
            } else {
                if (*sBattleRecords.as_ptr())[battler][crate::gl!(sBattlerRecordSizes[battler])]
                    == ACTION_MOVE_CHANGE
                {
                    let mut ppBonuses: CArray<u8, 4> = zeroed();
                    let mut moveSlots: CArray<u8, 4> = zeroed();
                    let mut mimickedMoveSlots: CArray<u8, 4> = zeroed();
                    let mut movePP: ChooseMoveStruct = zeroed();
                    let mut ppBonusSet: u8 = 0;
                    RecordedBattle_GetBattlerAction(battler as u8);
                    for j in 0..MAX_MON_MOVES {
                        ppBonuses[j] = shr_i32(
                            crate::gl!(gBattleMons[battler].ppBonuses) as i32
                                & shl_i32(3, (j as u32) << 1),
                            (j as u32) << 1,
                        ) as u8;
                    }
                    for j in 0..MAX_MON_MOVES {
                        moveSlots[j] = RecordedBattle_GetBattlerAction(battler as u8);
                        movePP.moves[j] = {
                            let gl_i1 = moveSlots[j];
                            crate::gl!(gBattleMons[battler].moves[gl_i1])
                        };
                        movePP.currentPP[j] = {
                            let gl_i2 = moveSlots[j];
                            crate::gl!(gBattleMons[battler].pp[gl_i2])
                        };
                        movePP.maxPP[j] = ppBonuses[moveSlots[j]];
                        mimickedMoveSlots[j] = shr_u32(
                            (*gDisableStructs.as_ptr())[battler].mimickedMoves() as u32
                                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                    [j],
                            j as u32,
                        ) as u8;
                    }
                    for j in 0..MAX_MON_MOVES {
                        crate::gl!(gBattleMons[battler].moves[j] = movePP.moves[j]);
                        crate::gl!(gBattleMons[battler].pp[j] = movePP.currentPP[j]);
                    }
                    crate::gl!(gBattleMons[battler].ppBonuses = 0);
                    (*gDisableStructs.as_ptr())[battler].set_mimickedMoves(0);
                    j = 0;
                    while j < MAX_MON_MOVES {
                        crate::gl!(
                            gBattleMons[battler].ppBonuses |=
                                shl_i32(movePP.maxPP[j] as i32, (j as u32) << 1) as u8
                        );
                        (*gDisableStructs.as_ptr())[battler].set_mimickedMoves(
                            (*gDisableStructs.as_ptr())[battler].mimickedMoves()
                                | shl_i32(mimickedMoveSlots[j] as i32, j as u32) as u8,
                        );
                        j += 1;
                    }
                    if crate::gl!(gBattleMons[battler].status2) & STATUS2_TRANSFORMED == 0 {
                        for j in 0..MAX_MON_MOVES {
                            ppBonuses[j] = shr_u32(
                                GetMonData3(
                                    &raw mut (*gPlayerParty.as_ptr())
                                        [crate::gl!(gBattlerPartyIndexes[battler])],
                                    MON_DATA_PP_BONUSES,
                                    null_mut(),
                                ) & shl_i32(3, (j as u32) << 1) as u32,
                                (j as u32) << 1,
                            ) as u8;
                        }
                        for j in 0..MAX_MON_MOVES {
                            movePP.moves[j] = GetMonData3(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [crate::gl!(gBattlerPartyIndexes[battler])],
                                MON_DATA_MOVE1 + moveSlots[j] as i32,
                                null_mut(),
                            ) as u16;
                            movePP.currentPP[j] = GetMonData3(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [crate::gl!(gBattlerPartyIndexes[battler])],
                                MON_DATA_PP1 + moveSlots[j] as i32,
                                null_mut(),
                            ) as u8;
                            movePP.maxPP[j] = ppBonuses[moveSlots[j]];
                        }
                        j = 0;
                        while j < MAX_MON_MOVES {
                            SetMonData(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [crate::gl!(gBattlerPartyIndexes[battler])],
                                MON_DATA_MOVE1 + j,
                                &raw mut movePP.moves[j] as *mut c_void,
                            );
                            SetMonData(
                                &raw mut (*gPlayerParty.as_ptr())
                                    [crate::gl!(gBattlerPartyIndexes[battler])],
                                MON_DATA_PP1 + j,
                                &raw mut movePP.currentPP[j] as *mut c_void,
                            );
                            j += 1;
                        }
                        ppBonusSet = 0;
                        for j in 0..MAX_MON_MOVES {
                            ppBonusSet |= shl_i32(movePP.maxPP[j] as i32, (j as u32) << 1) as u8;
                        }
                        SetMonData(
                            &raw mut (*gPlayerParty.as_ptr())
                                [crate::gl!(gBattlerPartyIndexes[battler])],
                            MON_DATA_PP_BONUSES,
                            &raw mut ppBonusSet as *mut c_void,
                        );
                    }
                    crate::gl!(
                        gChosenMoveByBattler[battler] = (*gBattleMons.as_ptr())[battler].moves
                            [*(*crate::gl!(gBattleStruct))
                                .chosenMovePositions
                                .as_mut_ptr()
                                .at(battler)]
                    );
                }
            }
        }
        battler += 1;
    }
}
pub unsafe fn GetAiScriptsInRecordedBattle() -> u32 {
    sAI_Scripts.get()
}
pub fn RecordedBattle_SetPlaybackFinished() {
    sIsPlaybackFinished.set(TRUE);
}
pub fn RecordedBattle_CanStopPlayback() -> u8 {
    (sIsPlaybackFinished.get() == FALSE) as u8
}
pub unsafe fn GetRecordedBattleRecordMixFriendName(dst: *mut u8) {
    for i in 0..8i32 {
        *dst.at(i) = crate::gl!(sRecordMixFriendName[i]);
    }
    *dst.at(7) = EOS;
    ConvertInternationalString(dst, sRecordMixFriendLanguage.get());
}
pub unsafe fn GetRecordedBattleRecordMixFriendClass() -> u8 {
    sRecordMixFriendClass.get()
}
pub unsafe fn GetRecordedBattleApprenticeId() -> u8 {
    sApprenticeId.get()
}
pub unsafe fn GetRecordedBattleRecordMixFriendLanguage() -> u8 {
    sRecordMixFriendLanguage.get()
}
pub unsafe fn GetRecordedBattleApprenticeLanguage() -> u8 {
    sApprenticeLanguage.get()
}
pub fn RecordedBattle_SaveBattleOutcome() {
    sBattleOutcome.set(gBattleOutcome.get());
}
pub unsafe fn GetRecordedBattleEasyChatSpeech() -> *mut u16 {
    (*sEasyChatSpeech.as_ptr()).as_mut_ptr()
}
