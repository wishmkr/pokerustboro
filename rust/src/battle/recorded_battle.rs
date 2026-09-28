//! Translated from `src/recorded_battle.c` by tools/rustport/c2rs.py.
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
        ((self.bits_1279 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_battleScene(&mut self, v: u8) {
        self.bits_1279 = (self.bits_1279 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn textSpeed(&self) -> u8 {
        ((self.bits_1279 as u32 >> 1) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_textSpeed(&mut self, v: u8) {
        self.bits_1279 = (self.bits_1279 & !(0x7 << 1)) | ((v as u8 & 0x7) << 1);
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRecordedBattleRngSeed: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlePalaceMoveSelectionRngValue: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleRecords: Aligned<CArray<CArray<u8, 664>, 4>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerRecordSizes: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerPrevRecordSizes: Aligned<CArray<u16, 4>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerSavedRecordSizes: Aligned<CArray<u16, 4>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMode: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLvlMode: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierFacility: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierBrainSymbol: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCallback2_AfterRecordedBattle: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRecordedBattleMultiplayerId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierPassFlag: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleScene: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTextSpeed: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleFlags: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAI_Scripts: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedPlayerParty: CArray<Pokemon, 6> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedOpponentParty: CArray<Pokemon, 6> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerMonMoves: Aligned<CArray<CArray<u16, 4>, 2>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayers: CArray<PlayerInfo, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsPlaybackFinished: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixFriendName: Aligned<CArray<u8, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixFriendClass: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sApprenticeId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEasyChatSpeech: Aligned<CArray<u16, 6>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleOutcome: u8 = 0;
pub(crate) static mut sRecordMixFriendLanguage: u8 = 0;
pub(crate) static mut sApprenticeLanguage: u8 = 0;

unsafe extern "C" {
    static mut gActiveBattler: u8;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static mut gBattleOutcome: u8;
    static mut gBattleResources: *mut BattleResources;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlersCount: u8;
    static gBitTable: CArray<u32, 0>;
    static mut gChosenMoveByBattler: CArray<u16, 4>;
    static mut gDisableStructs: CArray<DisableStruct, 4>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static gGameLanguage: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static mut gPartnerTrainerId: u16;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gRngValue: u32;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_Result: u16;
    static mut gTasks: CArray<Task, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static mut gTrainerBattleOpponent_B: u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_InitBattle();
    fn CB2_QuitRecordedBattle();
    fn CalcByteArraySum(a0: *mut u8, a1: u32) -> u32;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn Free(a0: *mut c_void);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetFronterBrainSymbol() -> i32;
    fn GetLinkPlayerCount() -> u8;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn PlayMapChosenOrBattleBGM(a0: u16);
    fn ResetPaletteFadeControl();
    fn RunTasks();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TryReadSpecialSaveSector(a0: u8, a1: *mut u8) -> u32;
    fn TryWriteSpecialSaveSector(a0: u8, a1: *mut u8) -> u32;
    fn VarGet(a0: u16) -> u16;
    fn ZeroEnemyPartyMons();
    fn ZeroPlayerPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_Init(mode: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    sRecordMode = mode;
    sIsPlaybackFinished = FALSE;
    i = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        sBattlerRecordSizes[i] = 0;
        sBattlerPrevRecordSizes[i] = 0;
        sBattlerSavedRecordSizes[i] = 0;
        if mode == B_RECORD_MODE_RECORDING {
            j = 0;
            while j < BATTLER_RECORD_SIZE {
                sBattleRecords[i][j] = 0xFF;
                j += 1;
            }
            sBattleFlags = gBattleTypeFlags;
            sAI_Scripts = (*(*gBattleResources).ai).aiFlags;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetTrainerInfo() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if sRecordMode == B_RECORD_MODE_RECORDING {
        gRecordedBattleRngSeed = gRngValue;
        sFrontierFacility = VarGet(VAR_FRONTIER_FACILITY) as u8;
        sFrontierBrainSymbol = GetFronterBrainSymbol() as u8;
    } else if sRecordMode == B_RECORD_MODE_PLAYBACK {
        gRngValue = gRecordedBattleRngSeed;
    }
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let mut linkPlayersCount: u8 = 0;
        let mut text: CArray<u8, 30> = zeroed();
        gRecordedBattleMultiplayerId = GetMultiplayerId();
        linkPlayersCount = GetLinkPlayerCount();
        i = 0;
        while i < MAX_LINK_PLAYERS {
            sPlayers[i].trainerId = gLinkPlayers[i].trainerId;
            sPlayers[i].gender = gLinkPlayers[i].gender;
            sPlayers[i].battler = gLinkPlayers[i].id;
            sPlayers[i].language = gLinkPlayers[i].language;
            if i < linkPlayersCount as i32 {
                StringCopy(text.as_mut_ptr(), gLinkPlayers[i].name.as_mut_ptr());
                StripExtCtrlCodes(text.as_mut_ptr());
                StringCopy(sPlayers[i].name.as_mut_ptr(), text.as_mut_ptr());
            } else {
                j = 0;
                while j < 8 {
                    sPlayers[i].name[j] = gLinkPlayers[i].name[j];
                    j += 1;
                }
            }
            i += 1;
        }
    } else {
        sPlayers[0].trainerId = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
            | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
            | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
            | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
        sPlayers[0].gender = (*gSaveBlock2Ptr).playerGender;
        sPlayers[0].battler = 0;
        sPlayers[0].language = gGameLanguage as u16;
        i = 0;
        while i < 8 {
            sPlayers[0].name[i] = (*gSaveBlock2Ptr).playerName[i];
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetBattlerAction(battler: u8, action: u8) {
    if sBattlerRecordSizes[battler] < BATTLER_RECORD_SIZE as u16
        && sRecordMode != B_RECORD_MODE_PLAYBACK
    {
        sBattleRecords[battler][{
            let t1 = sBattlerRecordSizes[battler];
            sBattlerRecordSizes[battler] += 1;
            t1
        }] = action;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_ClearBattlerAction(battler: u8, bytesToClear: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < bytesToClear as i32 {
        sBattlerRecordSizes[battler] -= 1;
        sBattleRecords[battler][sBattlerRecordSizes[battler]] = 0xFF;
        if sBattlerRecordSizes[battler] == 0 {
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_GetBattlerAction(battler: u8) -> u8 {
    if sBattlerRecordSizes[battler] >= BATTLER_RECORD_SIZE as u16
        || sBattleRecords[battler][sBattlerRecordSizes[battler]] == 0xFF
    {
        gSpecialVar_Result = ({
            gBattleOutcome = B_OUTCOME_PLAYER_TELEPORTED;
            gBattleOutcome
        }) as u16;
        ResetPaletteFadeControl();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        SetMainCallback2(Some(CB2_QuitRecordedBattle));
        return B_ACTION_NONE;
    } else {
        return sBattleRecords[battler][{
            let t1 = sBattlerRecordSizes[battler];
            sBattlerRecordSizes[battler] += 1;
            t1
        }];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetRecordedBattleMode() -> u8 {
    return sRecordMode;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_BufferNewBattlerData(mut dst: *mut u8) -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut idx: u8 = 0;
    i = 0;
    while i < MAX_BATTLERS_COUNT {
        if sBattlerRecordSizes[i] != sBattlerPrevRecordSizes[i] {
            *dst.at({
                let t1 = idx;
                idx += 1;
                t1
            }) = i;
            *dst.at({
                let t2 = idx;
                idx += 1;
                t2
            }) = sBattlerRecordSizes[i] as u8 - sBattlerPrevRecordSizes[i] as u8;
            j = 0;
            while (j as i32) < sBattlerRecordSizes[i] as i32 - sBattlerPrevRecordSizes[i] as i32 {
                *dst.at({
                    let t3 = idx;
                    idx += 1;
                    t3
                }) = sBattleRecords[i][sBattlerPrevRecordSizes[i] as i32 + j as i32];
                j += 1;
            }
            sBattlerPrevRecordSizes[i] = sBattlerRecordSizes[i];
        }
        i += 1;
    }
    return idx;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_RecordAllBattlerData(src: *mut u8) {
    let mut i: i32 = 0;
    let mut idx: u8 = 2;
    let mut size: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
        return;
    }
    i = 0;
    while i < GetLinkPlayerCount() as i32 {
        if gLinkPlayers[i].version as i32 & 0xFF != VERSION_EMERALD as i32 {
            return;
        }
        i += 1;
    }
    if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 {
        size = *src;
        while size != 0 {
            let mut battler: u8 = GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
            let mut numActions: u8 = GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
            i = 0;
            while i < numActions as i32 {
                sBattleRecords[battler][{
                    let t1 = sBattlerSavedRecordSizes[battler];
                    sBattlerSavedRecordSizes[battler] += 1;
                    t1
                }] = GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNextRecordedDataByte(
    data: *mut u8,
    idx: *mut u8,
    size: *mut u8,
) -> u8 {
    *size -= 1;
    return *data.at({
        let t1 = *idx;
        *idx += 1;
        t1
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanCopyRecordedBattleSaveData() -> u32 {
    let mut dst: *mut RecordedBattleSave = AllocZeroed(3968) as *mut RecordedBattleSave;
    let mut ret: u32 = CopyRecordedBattleFromSave(dst);
    Free(dst as *mut c_void);
    return ret;
}
pub(crate) unsafe extern "C" fn IsRecordedBattleSaveValid(save: *mut RecordedBattleSave) -> u32 {
    if (*save).battleFlags == 0 {
        return FALSE as u32;
    }
    if (*save).battleFlags & BATTLE_TYPE_RECORDED_INVALID != 0 {
        return FALSE as u32;
    }
    if CalcByteArraySum(save as *mut c_void as *mut u8, 3964) != (*save).checksum {
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn RecordedBattleToSave(
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRecordedBattleToSaveData() -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: u32 = 0;
    let mut battleSave: *mut RecordedBattleSave = null_mut();
    let mut savSection: *mut RecordedBattleSave = null_mut();
    let mut saveAttempts: u8 = 0;
    saveAttempts = 0;
    battleSave = AllocZeroed(3968) as *mut RecordedBattleSave;
    savSection = AllocZeroed(SECTOR_SIZE) as *mut RecordedBattleSave;
    i = 0;
    while i < PARTY_SIZE {
        (*battleSave).playerParty[i] = sSavedPlayerParty[i];
        (*battleSave).opponentParty[i] = sSavedOpponentParty[i];
        i += 1;
    }
    i = 0;
    while i < MAX_LINK_PLAYERS {
        j = 0;
        while j < 8 {
            (*battleSave).playersName[i][j] = sPlayers[i].name[j];
            j += 1;
        }
        (*battleSave).playersGender[i] = sPlayers[i].gender;
        (*battleSave).playersLanguage[i] = sPlayers[i].language as u8;
        (*battleSave).playersBattlers[i] = sPlayers[i].battler as u8;
        (*battleSave).playersTrainerId[i] = sPlayers[i].trainerId;
        i += 1;
    }
    (*battleSave).rngSeed = gRecordedBattleRngSeed;
    if sBattleFlags & BATTLE_TYPE_LINK != 0 {
        (*battleSave).battleFlags = sBattleFlags & 0xffffffdd | BATTLE_TYPE_RECORDED_LINK;
        if sBattleFlags & BATTLE_TYPE_IS_MASTER != 0 {
            (*battleSave).battleFlags |= 0x80000000;
        } else if sBattleFlags & BATTLE_TYPE_MULTI != 0 {
            match sPlayers[0].battler {
                0 | 2 => {
                    if sPlayers[gRecordedBattleMultiplayerId].battler as i32 & 1 == 0 {
                        (*battleSave).battleFlags |= 0x80000000;
                    }
                }
                1 | 3 => {
                    if sPlayers[gRecordedBattleMultiplayerId].battler as i32 & 1 != 0 {
                        (*battleSave).battleFlags |= 0x80000000;
                    }
                }
                _ => {}
            }
        }
    } else {
        (*battleSave).battleFlags = sBattleFlags;
    }
    (*battleSave).opponentA = gTrainerBattleOpponent_A;
    (*battleSave).opponentB = gTrainerBattleOpponent_B;
    (*battleSave).partnerId = gPartnerTrainerId;
    (*battleSave).multiplayerId = gRecordedBattleMultiplayerId as u16;
    (*battleSave).lvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
    (*battleSave).frontierFacility = sFrontierFacility;
    (*battleSave).frontierBrainSymbol = sFrontierBrainSymbol;
    (*battleSave).set_battleScene((*gSaveBlock2Ptr).optionsBattleSceneOff() as u8);
    (*battleSave).set_textSpeed((*gSaveBlock2Ptr).optionsTextSpeed() as u8);
    (*battleSave).AI_scripts = sAI_Scripts;
    if gTrainerBattleOpponent_A >= TRAINER_RECORD_MIXING_FRIEND as u16
        && gTrainerBattleOpponent_A < TRAINER_RECORD_MIXING_APPRENTICE as u16
    {
        i = 0;
        while i < 8 {
            (*battleSave).recordMixFriendName[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .name[i];
            i += 1;
        }
        (*battleSave).recordMixFriendClass = (*gSaveBlock2Ptr).frontier.towerRecords
            [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
        if sBattleOutcome == B_OUTCOME_WON {
            i = 0;
            while i < EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                    [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_FRIEND]
                    .speechLost[i];
                i += 1;
            }
        } else {
            i = 0;
            while i < EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                    [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_FRIEND]
                    .speechWon[i];
                i += 1;
            }
        }
        (*battleSave).recordMixFriendLanguage = (*gSaveBlock2Ptr).frontier.towerRecords
            [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .language;
    } else if gTrainerBattleOpponent_B >= TRAINER_RECORD_MIXING_FRIEND as u16
        && gTrainerBattleOpponent_B < TRAINER_RECORD_MIXING_APPRENTICE as u16
    {
        i = 0;
        while i < 8 {
            (*battleSave).recordMixFriendName[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .name[i];
            i += 1;
        }
        (*battleSave).recordMixFriendClass = (*gSaveBlock2Ptr).frontier.towerRecords
            [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
        if sBattleOutcome == B_OUTCOME_WON {
            i = 0;
            while i < EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                    [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_FRIEND]
                    .speechLost[i];
                i += 1;
            }
        } else {
            i = 0;
            while i < EASY_CHAT_BATTLE_WORDS_COUNT {
                (*battleSave).easyChatSpeech[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                    [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_FRIEND]
                    .speechWon[i];
                i += 1;
            }
        }
        (*battleSave).recordMixFriendLanguage = (*gSaveBlock2Ptr).frontier.towerRecords
            [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .language;
    } else if gPartnerTrainerId >= TRAINER_RECORD_MIXING_FRIEND as u16
        && gPartnerTrainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16
    {
        i = 0;
        while i < 8 {
            (*battleSave).recordMixFriendName[i] = (*gSaveBlock2Ptr).frontier.towerRecords
                [gPartnerTrainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .name[i];
            i += 1;
        }
        (*battleSave).recordMixFriendClass = (*gSaveBlock2Ptr).frontier.towerRecords
            [gPartnerTrainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
        (*battleSave).recordMixFriendLanguage = (*gSaveBlock2Ptr).frontier.towerRecords
            [gPartnerTrainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .language;
    }
    if gTrainerBattleOpponent_A >= TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        (*battleSave).apprenticeId = (*gSaveBlock2Ptr).apprentices
            [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id();
        i = 0;
        while i < EASY_CHAT_BATTLE_WORDS_COUNT {
            (*battleSave).easyChatSpeech[i] = (*gSaveBlock2Ptr).apprentices
                [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .speechWon[i];
            i += 1;
        }
        (*battleSave).apprenticeLanguage = (*gSaveBlock2Ptr).apprentices
            [gTrainerBattleOpponent_A as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .language;
    } else if gTrainerBattleOpponent_B >= TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        (*battleSave).apprenticeId = (*gSaveBlock2Ptr).apprentices
            [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id();
        i = 0;
        while i < EASY_CHAT_BATTLE_WORDS_COUNT {
            (*battleSave).easyChatSpeech[i] = (*gSaveBlock2Ptr).apprentices
                [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .speechWon[i];
            i += 1;
        }
        (*battleSave).apprenticeLanguage = (*gSaveBlock2Ptr).apprentices
            [gTrainerBattleOpponent_B as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .language;
    } else if gPartnerTrainerId >= TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        (*battleSave).apprenticeId = (*gSaveBlock2Ptr).apprentices
            [gPartnerTrainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id();
        (*battleSave).apprenticeLanguage = (*gSaveBlock2Ptr).apprentices
            [gPartnerTrainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .language;
    }
    i = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        j = 0;
        while j < BATTLER_RECORD_SIZE {
            (*battleSave).battleRecord[i][j] = sBattleRecords[i][j];
            j += 1;
        }
        i += 1;
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
    return ret;
}
pub(crate) unsafe extern "C" fn TryCopyRecordedBattleSaveData(
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
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn CopyRecordedBattleFromSave(dst: *mut RecordedBattleSave) -> u32 {
    let mut savBuffer: *mut SaveSector = AllocZeroed(SECTOR_SIZE) as *mut SaveSector;
    let mut ret: u32 = TryCopyRecordedBattleSaveData(dst, savBuffer);
    Free(savBuffer as *mut c_void);
    return ret;
}
pub(crate) unsafe extern "C" fn CB2_RecordedBattleEnd() {
    (*gSaveBlock2Ptr).frontier.set_lvlMode(sLvlMode);
    gBattleOutcome = 0;
    gBattleTypeFlags = 0;
    gTrainerBattleOpponent_A = 0;
    gTrainerBattleOpponent_B = 0;
    gPartnerTrainerId = 0;
    RecordedBattle_RestoreSavedParties();
    SetMainCallback2(sCallback2_AfterRecordedBattle);
}
pub(crate) unsafe extern "C" fn Task_StartAfterCountdown(taskId: u8) {
    if ({
        gTasks[taskId].data[0] -= 1;
        gTasks[taskId].data[0]
    }) == 0
    {
        gMain.savedCallback = Some(CB2_RecordedBattleEnd);
        SetMainCallback2(Some(CB2_InitBattle));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetVariablesForRecordedBattle(src: *mut RecordedBattleSave) {
    let mut var: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    ZeroPlayerPartyMons();
    ZeroEnemyPartyMons();
    i = 0;
    while i < PARTY_SIZE {
        gPlayerParty[i] = (*src).playerParty[i];
        gEnemyParty[i] = (*src).opponentParty[i];
        i += 1;
    }
    i = 0;
    while i < MAX_LINK_PLAYERS {
        var = 0;
        j = 0;
        while j < 8 {
            gLinkPlayers[i].name[j] = (*src).playersName[i][j];
            if (*src).playersName[i][j] == EOS {
                var = TRUE;
            }
            j += 1;
        }
        gLinkPlayers[i].gender = (*src).playersGender[i];
        gLinkPlayers[i].language = (*src).playersLanguage[i] as u16;
        gLinkPlayers[i].id = (*src).playersBattlers[i] as u16;
        gLinkPlayers[i].trainerId = (*src).playersTrainerId[i];
        if var != 0 {
            ConvertInternationalString(
                gLinkPlayers[i].name.as_mut_ptr(),
                gLinkPlayers[i].language as u8,
            );
        }
        i += 1;
    }
    gRecordedBattleRngSeed = (*src).rngSeed;
    gBattleTypeFlags = (*src).battleFlags | BATTLE_TYPE_RECORDED;
    gTrainerBattleOpponent_A = (*src).opponentA;
    gTrainerBattleOpponent_B = (*src).opponentB;
    gPartnerTrainerId = (*src).partnerId;
    gRecordedBattleMultiplayerId = (*src).multiplayerId as u8;
    sLvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
    sFrontierFacility = (*src).frontierFacility;
    sFrontierBrainSymbol = (*src).frontierBrainSymbol;
    sBattleScene = (*src).battleScene();
    sTextSpeed = (*src).textSpeed();
    sAI_Scripts = (*src).AI_scripts;
    i = 0;
    while i < 8 {
        sRecordMixFriendName[i] = (*src).recordMixFriendName[i];
        i += 1;
    }
    sRecordMixFriendClass = (*src).recordMixFriendClass;
    sApprenticeId = (*src).apprenticeId;
    sRecordMixFriendLanguage = (*src).recordMixFriendLanguage;
    sApprenticeLanguage = (*src).apprenticeLanguage;
    i = 0;
    while i < EASY_CHAT_BATTLE_WORDS_COUNT {
        sEasyChatSpeech[i] = (*src).easyChatSpeech[i];
        i += 1;
    }
    (*gSaveBlock2Ptr).frontier.set_lvlMode((*src).lvlMode);
    i = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        j = 0;
        while j < BATTLER_RECORD_SIZE {
            sBattleRecords[i][j] = (*src).battleRecord[i][j];
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayRecordedBattle(CB2_After: Option<unsafe extern "C" fn()>) {
    let mut battleSave: *mut RecordedBattleSave = AllocZeroed(3968) as *mut RecordedBattleSave;
    if CopyRecordedBattleFromSave(battleSave) == TRUE as u32 {
        let mut taskId: u8 = 0;
        RecordedBattle_SaveParties();
        SetVariablesForRecordedBattle(battleSave);
        taskId = CreateTask(Some(Task_StartAfterCountdown), 1);
        gTasks[taskId].data[0] = 128;
        sCallback2_AfterRecordedBattle = CB2_After;
        PlayMapChosenOrBattleBGM(FALSE as u16);
        SetMainCallback2(Some(CB2_RecordedBattle));
    }
    Free(battleSave as *mut c_void);
}
pub(crate) unsafe extern "C" fn CB2_RecordedBattle() {
    AnimateSprites();
    BuildOamBuffer();
    RunTasks();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleFrontierFacility() -> u8 {
    return sFrontierFacility;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleFronterBrainSymbol() -> u8 {
    return sFrontierBrainSymbol;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SaveParties() {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        sSavedPlayerParty[i] = gPlayerParty[i];
        sSavedOpponentParty[i] = gEnemyParty[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RecordedBattle_RestoreSavedParties() {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        gPlayerParty[i] = sSavedPlayerParty[i];
        gEnemyParty[i] = sSavedOpponentParty[i];
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetActiveBattlerLinkPlayerGender() -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS {
        if gLinkPlayers[i].id == gActiveBattler as u16 {
            break;
        }
        i += 1;
    }
    if i != MAX_LINK_PLAYERS {
        return gLinkPlayers[i].gender;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_ClearFrontierPassFlag() {
    sFrontierPassFlag = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetFrontierPassFlagFromHword(flags: u16) {
    sFrontierPassFlag |= ((flags as i32 & 32768) >> 15) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_GetFrontierPassFlag() -> u8 {
    return sFrontierPassFlag;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleSceneInRecordedBattle() -> u8 {
    return sBattleScene;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTextSpeedInRecordedBattle() -> u8 {
    return sTextSpeed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_CopyBattlerMoves() {
    let mut i: i32 = 0;
    if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT {
        return;
    }
    if gBattleTypeFlags & 0x2000002 != 0 {
        return;
    }
    if sRecordMode == B_RECORD_MODE_PLAYBACK {
        return;
    }
    i = 0;
    while i < MAX_MON_MOVES {
        sPlayerMonMoves[gActiveBattler as i32 / 2][i] = gBattleMons[gActiveBattler].moves[i];
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_CheckMovesetChanges(mode: u8) {
    let mut battler: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if gBattleTypeFlags & 0x2000002 != 0 {
        return;
    }
    battler = 0;
    while battler < gBattlersCount as i32 {
        if GetBattlerSide(battler as u8) != B_SIDE_OPPONENT {
            if mode == B_RECORD_MODE_RECORDING {
                j = 0;
                while j < MAX_MON_MOVES {
                    if gBattleMons[battler].moves[j] != sPlayerMonMoves[battler / 2][j] {
                        break;
                    }
                    j += 1;
                }
                if j != MAX_MON_MOVES {
                    RecordedBattle_SetBattlerAction(battler as u8, ACTION_MOVE_CHANGE);
                    j = 0;
                    while j < MAX_MON_MOVES {
                        k = 0;
                        while k < MAX_MON_MOVES {
                            if gBattleMons[battler].moves[j] == sPlayerMonMoves[battler / 2][k] {
                                RecordedBattle_SetBattlerAction(battler as u8, k as u8);
                                break;
                            }
                            k += 1;
                        }
                        j += 1;
                    }
                }
            } else {
                if sBattleRecords[battler][sBattlerRecordSizes[battler]] == ACTION_MOVE_CHANGE {
                    let mut ppBonuses: CArray<u8, 4> = zeroed();
                    let mut moveSlots: CArray<u8, 4> = zeroed();
                    let mut mimickedMoveSlots: CArray<u8, 4> = zeroed();
                    let mut movePP: ChooseMoveStruct = zeroed();
                    let mut ppBonusSet: u8 = 0;
                    RecordedBattle_GetBattlerAction(battler as u8);
                    j = 0;
                    while j < MAX_MON_MOVES {
                        ppBonuses[j] = shr_i32(
                            gBattleMons[battler].ppBonuses as i32 & shl_i32(3, (j as u32) << 1),
                            (j as u32) << 1,
                        ) as u8;
                        j += 1;
                    }
                    j = 0;
                    while j < MAX_MON_MOVES {
                        moveSlots[j] = RecordedBattle_GetBattlerAction(battler as u8);
                        movePP.moves[j] = gBattleMons[battler].moves[moveSlots[j]];
                        movePP.currentPP[j] = gBattleMons[battler].pp[moveSlots[j]];
                        movePP.maxPP[j] = ppBonuses[moveSlots[j]];
                        mimickedMoveSlots[j] = shr_u32(
                            gDisableStructs[battler].mimickedMoves() as u32 & gBitTable[j],
                            j as u32,
                        ) as u8;
                        j += 1;
                    }
                    j = 0;
                    while j < MAX_MON_MOVES {
                        gBattleMons[battler].moves[j] = movePP.moves[j];
                        gBattleMons[battler].pp[j] = movePP.currentPP[j];
                        j += 1;
                    }
                    gBattleMons[battler].ppBonuses = 0;
                    gDisableStructs[battler].set_mimickedMoves(0);
                    j = 0;
                    while j < MAX_MON_MOVES {
                        gBattleMons[battler].ppBonuses |=
                            shl_i32(movePP.maxPP[j] as i32, (j as u32) << 1) as u8;
                        gDisableStructs[battler].set_mimickedMoves(
                            gDisableStructs[battler].mimickedMoves()
                                | shl_i32(mimickedMoveSlots[j] as i32, j as u32) as u8,
                        );
                        j += 1;
                    }
                    if gBattleMons[battler].status2 & STATUS2_TRANSFORMED == 0 {
                        j = 0;
                        while j < MAX_MON_MOVES {
                            ppBonuses[j] = shr_u32(
                                GetMonData3(
                                    &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                                    MON_DATA_PP_BONUSES,
                                    null_mut(),
                                ) & shl_i32(3, (j as u32) << 1) as u32,
                                (j as u32) << 1,
                            ) as u8;
                            j += 1;
                        }
                        j = 0;
                        while j < MAX_MON_MOVES {
                            movePP.moves[j] = GetMonData3(
                                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                                MON_DATA_MOVE1 + moveSlots[j] as i32,
                                null_mut(),
                            ) as u16;
                            movePP.currentPP[j] = GetMonData3(
                                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                                MON_DATA_PP1 + moveSlots[j] as i32,
                                null_mut(),
                            ) as u8;
                            movePP.maxPP[j] = ppBonuses[moveSlots[j]];
                            j += 1;
                        }
                        j = 0;
                        while j < MAX_MON_MOVES {
                            SetMonData(
                                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                                MON_DATA_MOVE1 + j,
                                &raw mut movePP.moves[j] as *mut c_void,
                            );
                            SetMonData(
                                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                                MON_DATA_PP1 + j,
                                &raw mut movePP.currentPP[j] as *mut c_void,
                            );
                            j += 1;
                        }
                        ppBonusSet = 0;
                        j = 0;
                        while j < MAX_MON_MOVES {
                            ppBonusSet |= shl_i32(movePP.maxPP[j] as i32, (j as u32) << 1) as u8;
                            j += 1;
                        }
                        SetMonData(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                            MON_DATA_PP_BONUSES,
                            &raw mut ppBonusSet as *mut c_void,
                        );
                    }
                    gChosenMoveByBattler[battler] = gBattleMons[battler].moves[*(*gBattleStruct)
                        .chosenMovePositions
                        .as_mut_ptr()
                        .at(battler)];
                }
            }
        }
        battler += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAiScriptsInRecordedBattle() -> u32 {
    return sAI_Scripts;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetPlaybackFinished() {
    sIsPlaybackFinished = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_CanStopPlayback() -> u8 {
    return (sIsPlaybackFinished == FALSE) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleRecordMixFriendName(mut dst: *mut u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        *dst.at(i) = sRecordMixFriendName[i];
        i += 1;
    }
    *dst.at(7) = EOS;
    ConvertInternationalString(dst, sRecordMixFriendLanguage);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleRecordMixFriendClass() -> u8 {
    return sRecordMixFriendClass;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleApprenticeId() -> u8 {
    return sApprenticeId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleRecordMixFriendLanguage() -> u8 {
    return sRecordMixFriendLanguage;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleApprenticeLanguage() -> u8 {
    return sApprenticeLanguage;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SaveBattleOutcome() {
    sBattleOutcome = gBattleOutcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleEasyChatSpeech() -> *mut u16 {
    return sEasyChatSpeech.as_mut_ptr();
}
