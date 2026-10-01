//! Translated from `src/record_mixing.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_tower::{
    CalcEmeraldBattleTowerChecksum, EmeraldBattleTowerRecordToRuby, PutNewBattleTowerRecord,
    RubyBattleTowerRecordToEmerald,
};
#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::{CreateTask_EnterCableClubSeat, CreateTask_ReestablishCableClubLink};
#[allow(unused_imports)]
use crate::consts::*;
use crate::daycare::InitDaycareMailRecordMixing;
use crate::event_data::{FlagSet, VarSet};
use crate::ffi::gSpecialVar_0x8005;
use crate::field_screen_effect::Task_ReturnToFieldRecordMixing;
use crate::fldeff_misc::{CreateRecordMixingLights, DestroyRecordMixingLights};
use crate::international_string_util::PadNameString;
use crate::item::{AddBagItem, CheckBagHasItem, CheckPCHasItem, GetPocketByItemId};
use crate::lilycove_lady::{
    GetLilycoveLadyId, QuizLadyClearQuestionForRecordMix, ResetLilycoveLadyForRecordMix,
};
use crate::link::{
    CheckShouldAdvanceLinkState, ClearLinkCallback_2, GetBlockReceivedStatus, GetLinkPlayerCount,
    GetLinkPlayerCount_2, GetLinkPlayerCountAsBitFlags, GetLinkPlayerTrainerId, GetMultiplayerId,
    GetSavedPlayerCount, IsLinkMaster, IsLinkTaskFinished, Link_AnyPartnersPlayingRubyOrSapphire,
    LinkDummy_Return2, ResetBlockReceivedFlag, SendBlockRequest, SetCloseLinkCallback,
    SetLinkStandbyCallback, SetLocalLinkPlayerId, gLinkPlayers, gReceivedRemoteLinkPlayers,
    gWirelessCommType,
};
use crate::link::{gBlockRecvBuffer, gBlockSendBuffer};
use crate::link_rfu_2::Rfu_SetLinkRecovery;
use crate::load_save::{ClearContinueGameWarpStatus2, SetContinueGameWarpStatusToDynamicWarp};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::mauville_old_man::{
    ResetMauvilleOldManFlag, SanitizeMauvilleOldManForRuby, SanitizeReceivedEmeraldOldMan,
    SanitizeReceivedRubyOldMan,
};
use crate::menu::{ClearDialogWindowAndFrame, DrawDialogueFrame};
use crate::mystery_event_script::GetRecordMixingGift;
use crate::new_game::{CopyTrainerId, GetTrainerId};
use crate::overworld::SetLinkWaitingForScript;
use crate::random::{Random2, SeedRng, SeedRng2};
use crate::save::{Task_LinkFullSave, WriteSaveBlock1Sector, WriteSaveBlock2};
use crate::script::ScriptContext_Enable;
use crate::secret_base::{
    ClearJapaneseSecretBases, ReceiveSecretBasesData, SetPlayerSecretBaseParty,
};
use crate::sound::PlaySE;
use crate::string_util::gStringVar1;
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringLength};
use crate::string_util::{ConvertInternationalString, IsStringJapanese, StripExtCtrlCodes};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_data_ptr, task_get, task_set, task_set_func};
use crate::tv::{
    DeactivateAllNormalTVShows, ReceivePokeNewsData, ReceiveTvShowsData,
    SanitizeTVShowLocationsForRuby, SanitizeTVShowsForRuby,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::CopyWindowToVram;
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
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
// The C's names for task and sprite data slots.
const tCounter: usize = 0;
const tState: usize = 0;
const tSentRecord: usize = 2;
const tNumChunksSent: usize = 4;
const tMultiplayerId: usize = 5;
const tRecvRecords: usize = 5;
const tCopyTaskId: usize = 10;
// Data tables (translate with cdata.py): sPlayerIdxOrders_2Player sPlayerIdxOrders_3Player sPlayerIdxOrders_4Player sDaycareMailSwapIds_3Player sDaycareMailSwapIds_4Player

/// `union PlayerRecord`
#[repr(C)]
#[derive(Clone, Copy)]
pub union PlayerRecord {
    pub ruby: PlayerRecordRS,
    pub emerald: PlayerRecordEmerald,
}

unsafe impl Sync for PlayerRecord {}

/// `struct PlayerRecordRS`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PlayerRecordRS {
    pub secretBases: CArray<SecretBase, 20>,
    pub tvShows: CArray<TVShow, 25>,
    pub pokeNews: CArray<PokeNews, 16>,
    pub oldMan: OldMan,
    pub dewfordTrends: CArray<DewfordTrend, 5>,
    pub daycareMail: RecordMixingDaycareMail,
    pub battleTowerRecord: RSBattleTowerRecord,
    pub giftItem: u16,
    pub filler: CArray<u16, 50>,
}

unsafe impl Sync for PlayerRecordRS {}

/// `struct RecordMixingHallRecords`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RecordMixingHallRecords {
    pub hallRecords1P: CArray<CArray<CArray<RankingHall1P, 6>, 2>, 9>,
    pub hallRecords2P: CArray<CArray<RankingHall2P, 6>, 2>,
}

unsafe impl Sync for RecordMixingHallRecords {}

/// `struct PlayerRecordEmerald`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PlayerRecordEmerald {
    pub secretBases: CArray<SecretBase, 20>,
    pub tvShows: CArray<TVShow, 25>,
    pub pokeNews: CArray<PokeNews, 16>,
    pub oldMan: OldMan,
    pub dewfordTrends: CArray<DewfordTrend, 5>,
    pub daycareMail: RecordMixingDaycareMail,
    pub battleTowerRecord: EmeraldBattleTowerRecord,
    pub giftItem: u16,
    pub lilycoveLady: LilycoveLady,
    pub apprentices: CArray<Apprentice, 2>,
    pub hallRecords: PlayerHallRecords,
    pub filler_1434: CArray<u8, 16>,
}

unsafe impl Sync for PlayerRecordEmerald {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PlayerRecord>() == 5188);
    assert!(size_of::<PlayerRecordRS>() == 4656);
    assert!(offset_of!(PlayerRecordRS, secretBases) == 0);
    assert!(offset_of!(PlayerRecordRS, tvShows) == 3200);
    assert!(offset_of!(PlayerRecordRS, pokeNews) == 4100);
    assert!(offset_of!(PlayerRecordRS, oldMan) == 4164);
    assert!(offset_of!(PlayerRecordRS, dewfordTrends) == 4228);
    assert!(offset_of!(PlayerRecordRS, daycareMail) == 4268);
    assert!(offset_of!(PlayerRecordRS, battleTowerRecord) == 4388);
    assert!(offset_of!(PlayerRecordRS, giftItem) == 4552);
    assert!(offset_of!(PlayerRecordRS, filler) == 4554);
    assert!(size_of::<RecordMixingHallRecords>() == 2064);
    assert!(offset_of!(RecordMixingHallRecords, hallRecords1P) == 0);
    assert!(offset_of!(RecordMixingHallRecords, hallRecords2P) == 1728);
    assert!(size_of::<PlayerRecordEmerald>() == 5188);
    assert!(offset_of!(PlayerRecordEmerald, secretBases) == 0);
    assert!(offset_of!(PlayerRecordEmerald, tvShows) == 3200);
    assert!(offset_of!(PlayerRecordEmerald, pokeNews) == 4100);
    assert!(offset_of!(PlayerRecordEmerald, oldMan) == 4164);
    assert!(offset_of!(PlayerRecordEmerald, dewfordTrends) == 4228);
    assert!(offset_of!(PlayerRecordEmerald, daycareMail) == 4268);
    assert!(offset_of!(PlayerRecordEmerald, battleTowerRecord) == 4388);
    assert!(offset_of!(PlayerRecordEmerald, giftItem) == 4624);
    assert!(offset_of!(PlayerRecordEmerald, lilycoveLady) == 4628);
    assert!(offset_of!(PlayerRecordEmerald, apprentices) == 4692);
    assert!(offset_of!(PlayerRecordEmerald, hallRecords) == 4828);
    assert!(offset_of!(PlayerRecordEmerald, filler_1434) == 5172);
};

const BUFFER_CHUNK_SIZE: u32 = 200;
const DAYCARE_SLOT: i32 = 1;
const MULTIPLAYER_ID: i32 = 0;
const NUM_SWAP_COMBOS: i32 = 3;

static sDaycareMailSwapIds_3Player: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::record_mixing::sDaycareMailSwapIds_3Player).cast());
static sDaycareMailSwapIds_4Player: Table<CArray<CArray<u8, 4>, 3>> =
    Table((&raw const crate::data::record_mixing::sDaycareMailSwapIds_4Player).cast());
static sPlayerIdxOrders_2Player: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::record_mixing::sPlayerIdxOrders_2Player).cast());
static sPlayerIdxOrders_3Player: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::record_mixing::sPlayerIdxOrders_3Player).cast());
static sPlayerIdxOrders_4Player: Table<CArray<CArray<u8, 4>, 9>> =
    Table((&raw const crate::data::record_mixing::sPlayerIdxOrders_4Player).cast());

pub(crate) static sReadyToReceive: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static mut sSecretBasesSave: *mut SecretBase = null_mut();
pub(crate) static mut sTvShowsSave: *mut TVShow = null_mut();
pub(crate) static mut sPokeNewsSave: *mut PokeNews = null_mut();
pub(crate) static mut sOldManSave: *mut OldMan = null_mut();
pub(crate) static mut sDewfordTrendsSave: *mut DewfordTrend = null_mut();
pub(crate) static mut sRecordMixMailSave: *mut RecordMixingDaycareMail = null_mut();
pub(crate) static mut sBattleTowerSave: *mut c_void = null_mut();
pub(crate) static mut sLilycoveLadySave: *mut LilycoveLady = null_mut();
pub(crate) static mut sApprenticesSave: *mut c_void = null_mut();
pub(crate) static mut sBattleTowerSave_Duplicate: *mut c_void = null_mut();
pub(crate) static sRecordStructSize: crate::global::Global<u32> = crate::global::Global::new(0);
pub(crate) static sDaycareMailRandSum: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static mut sPartnerHallRecords: CArray<*mut PlayerHallRecords, 3> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixMail: RecordMixingDaycareMail = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReceivedRecords: *mut PlayerRecord = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSentRecord: *mut PlayerRecord = null_mut();

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
/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `ReceiveDewfordTrendData` with this module's view of its types.
#[inline]
unsafe fn ReceiveDewfordTrendData(a0: *mut DewfordTrend, a1: u32, a2: u8) {
    unsafe {
        crate::dewford_trend::ReceiveDewfordTrendData(a0 as _, a1 as _, a2);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn RecordMixingPlayerSpotTriggered() {
    CreateTask_EnterCableClubSeat(Some(Task_RecordMixing_Main));
}
unsafe fn SetSrcLookupPointers() {
    sSecretBasesSave = (*gSaveBlock1Ptr).secretBases.as_mut_ptr();
    sTvShowsSave = (*gSaveBlock1Ptr).tvShows.as_mut_ptr();
    sPokeNewsSave = (*gSaveBlock1Ptr).pokeNews.as_mut_ptr();
    sOldManSave = &raw mut (*gSaveBlock1Ptr).oldMan;
    sDewfordTrendsSave = (*gSaveBlock1Ptr).dewfordTrends.as_mut_ptr();
    sRecordMixMailSave = &raw mut sRecordMixMail;
    sBattleTowerSave = &raw mut (*gSaveBlock2Ptr).frontier.towerPlayer as *mut c_void;
    sLilycoveLadySave = &raw mut (*gSaveBlock1Ptr).lilycoveLady;
    sApprenticesSave = (*gSaveBlock2Ptr).apprentices.as_mut_ptr() as *mut c_void;
    sBattleTowerSave_Duplicate = &raw mut (*gSaveBlock2Ptr).frontier.towerPlayer as *mut c_void;
}
unsafe fn PrepareUnknownExchangePacket(dest: *mut PlayerRecordRS) {
    memcpy(
        (*dest).secretBases.as_mut_ptr() as *mut u8,
        sSecretBasesSave as *mut u8,
        3200,
    );
    memcpy(
        (*dest).tvShows.as_mut_ptr() as *mut u8,
        sTvShowsSave as *mut u8,
        900,
    );
    SanitizeTVShowLocationsForRuby((*dest).tvShows.as_mut_ptr());
    memcpy(
        (*dest).pokeNews.as_mut_ptr() as *mut u8,
        sPokeNewsSave as *mut u8,
        64,
    );
    memcpy(
        &raw mut (*dest).oldMan as *mut u8,
        sOldManSave as *mut u8,
        64,
    );
    memcpy(
        (*dest).dewfordTrends.as_mut_ptr() as *mut u8,
        sDewfordTrendsSave as *mut u8,
        40,
    );
    GetRecordMixingDaycareMail(&raw mut (*dest).daycareMail);
    EmeraldBattleTowerRecordToRuby(
        sBattleTowerSave as *mut EmeraldBattleTowerRecord,
        &raw mut (*dest).battleTowerRecord,
    );
    if GetMultiplayerId() == 0 {
        (*dest).giftItem = GetRecordMixingGift();
    }
}
unsafe fn PrepareExchangePacketForRubySapphire(dest: *mut PlayerRecordRS) {
    memcpy(
        (*dest).secretBases.as_mut_ptr() as *mut u8,
        sSecretBasesSave as *mut u8,
        3200,
    );
    ClearJapaneseSecretBases((*dest).secretBases.as_mut_ptr());
    memcpy(
        (*dest).tvShows.as_mut_ptr() as *mut u8,
        sTvShowsSave as *mut u8,
        900,
    );
    SanitizeTVShowsForRuby((*dest).tvShows.as_mut_ptr());
    memcpy(
        (*dest).pokeNews.as_mut_ptr() as *mut u8,
        sPokeNewsSave as *mut u8,
        64,
    );
    memcpy(
        &raw mut (*dest).oldMan as *mut u8,
        sOldManSave as *mut u8,
        64,
    );
    SanitizeMauvilleOldManForRuby(&raw mut (*dest).oldMan);
    memcpy(
        (*dest).dewfordTrends.as_mut_ptr() as *mut u8,
        sDewfordTrendsSave as *mut u8,
        40,
    );
    GetRecordMixingDaycareMail(&raw mut (*dest).daycareMail);
    SanitizeDaycareMailForRuby(&raw mut (*dest).daycareMail);
    EmeraldBattleTowerRecordToRuby(
        sBattleTowerSave as *mut EmeraldBattleTowerRecord,
        &raw mut (*dest).battleTowerRecord,
    );
    SanitizeRubyBattleTowerRecord(&raw mut (*dest).battleTowerRecord);
    if GetMultiplayerId() == 0 {
        (*dest).giftItem = GetRecordMixingGift();
    }
}
unsafe fn PrepareExchangePacket() {
    SetPlayerSecretBaseParty();
    DeactivateAllNormalTVShows();
    SetSrcLookupPointers();
    if Link_AnyPartnersPlayingRubyOrSapphire() != 0 {
        if LinkDummy_Return2() == 0 {
            PrepareUnknownExchangePacket(&raw mut (*sSentRecord).ruby);
        } else {
            PrepareExchangePacketForRubySapphire(&raw mut (*sSentRecord).ruby);
        }
    } else {
        memcpy(
            (*sSentRecord).emerald.secretBases.as_mut_ptr() as *mut u8,
            sSecretBasesSave as *mut u8,
            3200,
        );
        memcpy(
            (*sSentRecord).emerald.tvShows.as_mut_ptr() as *mut u8,
            sTvShowsSave as *mut u8,
            900,
        );
        memcpy(
            (*sSentRecord).emerald.pokeNews.as_mut_ptr() as *mut u8,
            sPokeNewsSave as *mut u8,
            64,
        );
        memcpy(
            &raw mut (*sSentRecord).emerald.oldMan as *mut u8,
            sOldManSave as *mut u8,
            64,
        );
        memcpy(
            &raw mut (*sSentRecord).emerald.lilycoveLady as *mut u8,
            sLilycoveLadySave as *mut u8,
            64,
        );
        memcpy(
            (*sSentRecord).emerald.dewfordTrends.as_mut_ptr() as *mut u8,
            sDewfordTrendsSave as *mut u8,
            40,
        );
        GetRecordMixingDaycareMail(&raw mut (*sSentRecord).emerald.daycareMail);
        memcpy(
            &raw mut (*sSentRecord).emerald.battleTowerRecord as *mut u8,
            sBattleTowerSave as *mut u8,
            236,
        );
        SanitizeEmeraldBattleTowerRecord(&raw mut (*sSentRecord).emerald.battleTowerRecord);
        if GetMultiplayerId() == 0 {
            (*sSentRecord).emerald.giftItem = GetRecordMixingGift();
        }
        GetSavedApprentices(
            (*sSentRecord).emerald.apprentices.as_mut_ptr(),
            sApprenticesSave as *mut Apprentice,
        );
        GetPlayerHallRecords(&raw mut (*sSentRecord).emerald.hallRecords);
    }
}
unsafe fn ReceiveExchangePacket(multiplayerId: u32) {
    if Link_AnyPartnersPlayingRubyOrSapphire() != 0 {
        CalculateDaycareMailRandSum(
            (*sReceivedRecords).ruby.tvShows.as_mut_ptr() as *mut c_void as *mut u8
        );
        ReceiveSecretBasesData(
            (*sReceivedRecords).ruby.secretBases.as_mut_ptr() as *mut c_void,
            4656,
            multiplayerId as u8,
        );
        ReceiveDaycareMailData(
            &raw mut (*sReceivedRecords).ruby.daycareMail,
            4656,
            multiplayerId as u8,
            (*sReceivedRecords).ruby.tvShows.as_mut_ptr(),
        );
        ReceiveBattleTowerData(
            &raw mut (*sReceivedRecords).ruby.battleTowerRecord as *mut c_void,
            4656,
            multiplayerId as u8,
        );
        ReceiveTvShowsData(
            (*sReceivedRecords).ruby.tvShows.as_mut_ptr() as *mut c_void,
            4656,
            multiplayerId as u8,
        );
        ReceivePokeNewsData(
            (*sReceivedRecords).ruby.pokeNews.as_mut_ptr() as *mut c_void,
            4656,
            multiplayerId as u8,
        );
        ReceiveOldManData(
            &raw mut (*sReceivedRecords).ruby.oldMan,
            4656,
            multiplayerId as u8,
        );
        ReceiveDewfordTrendData(
            (*sReceivedRecords).ruby.dewfordTrends.as_mut_ptr(),
            4656,
            multiplayerId as u8,
        );
        ReceiveGiftItem(
            &raw mut (*sReceivedRecords).ruby.giftItem,
            multiplayerId as u8,
        );
    } else {
        CalculateDaycareMailRandSum(
            (*sReceivedRecords).emerald.tvShows.as_mut_ptr() as *mut c_void as *mut u8,
        );
        ReceiveSecretBasesData(
            (*sReceivedRecords).emerald.secretBases.as_mut_ptr() as *mut c_void,
            5188,
            multiplayerId as u8,
        );
        ReceiveTvShowsData(
            (*sReceivedRecords).emerald.tvShows.as_mut_ptr() as *mut c_void,
            5188,
            multiplayerId as u8,
        );
        ReceivePokeNewsData(
            (*sReceivedRecords).emerald.pokeNews.as_mut_ptr() as *mut c_void,
            5188,
            multiplayerId as u8,
        );
        ReceiveOldManData(
            &raw mut (*sReceivedRecords).emerald.oldMan,
            5188,
            multiplayerId as u8,
        );
        ReceiveDewfordTrendData(
            (*sReceivedRecords).emerald.dewfordTrends.as_mut_ptr(),
            5188,
            multiplayerId as u8,
        );
        ReceiveDaycareMailData(
            &raw mut (*sReceivedRecords).emerald.daycareMail,
            5188,
            multiplayerId as u8,
            (*sReceivedRecords).emerald.tvShows.as_mut_ptr(),
        );
        ReceiveBattleTowerData(
            &raw mut (*sReceivedRecords).emerald.battleTowerRecord as *mut c_void,
            5188,
            multiplayerId as u8,
        );
        ReceiveGiftItem(
            &raw mut (*sReceivedRecords).emerald.giftItem,
            multiplayerId as u8,
        );
        ReceiveLilycoveLadyData(
            &raw mut (*sReceivedRecords).emerald.lilycoveLady,
            5188,
            multiplayerId as u8,
        );
        ReceiveApprenticeData(
            (*sReceivedRecords).emerald.apprentices.as_mut_ptr(),
            5188,
            multiplayerId as u8 as u32,
        );
        ReceiveRankingHallRecords(
            &raw mut (*sReceivedRecords).emerald.hallRecords,
            5188,
            multiplayerId as u8 as u32,
        );
    }
}
unsafe fn PrintTextOnRecordMixing(src: *mut u8) {
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized(0, FONT_NORMAL, src, 0, 1, 0, None);
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub(crate) unsafe fn Task_RecordMixing_SoundEffect(taskId: u8) {
    if ({
        task_set(taskId, tCounter, task_get(taskId, tCounter) + 1);
        task_get(taskId, tCounter)
    }) == 50
    {
        PlaySE(SE_M_ATTRACT);
        task_set(taskId, tCounter, 0);
    }
}
pub(crate) unsafe fn Task_RecordMixing_Main(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            sSentRecord = Alloc(5188) as *mut PlayerRecord;
            sReceivedRecords = Alloc(20752) as *mut PlayerRecord;
            SetLocalLinkPlayerId(gSpecialVar_0x8005 as u8);
            VarSet(VAR_TEMP_MIXED_RECORDS, 1);
            sReadyToReceive.set(FALSE);
            PrepareExchangePacket();
            CreateRecordMixingLights();
            *data = 1;
            *data.at(10) = CreateTask(Some(Task_MixingRecordsRecv), 80) as i16;
            *data.at(15) = CreateTask(Some(Task_RecordMixing_SoundEffect), 81) as i16;
        }
        1 => {
            if (*gTasks.as_ptr())[*data.at(10)].isActive == 0 {
                *data = 2;
                FlagSet(FLAG_SYS_MIX_RECORD);
                DestroyRecordMixingLights();
                DestroyTask(*data.at(15) as u8);
            }
        }
        2 => {
            *data.at(10) = CreateTask(Some(Task_DoRecordMixing), 10) as i16;
            *data = 3;
            PlaySE(SE_M_BATON_PASS);
        }
        3 => {
            if (*gTasks.as_ptr())[*data.at(10)].isActive == 0 {
                *data = 4;
                if gWirelessCommType == 0 {
                    *data.at(10) = CreateTask_ReestablishCableClubLink() as i16;
                }
                PrintTextOnRecordMixing(
                    (*(&raw const crate::data::strings::gText_RecordMixingComplete)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                *data.at(8) = 0;
            }
        }
        4 => {
            if ({
                *data.at(8) += 1;
                *data.at(8)
            }) > 60
            {
                *data = 5;
            }
        }
        5 if (*gTasks.as_ptr())[*data.at(10)].isActive == 0 => {
            Free(sReceivedRecords as *mut c_void);
            Free(sSentRecord as *mut c_void);
            SetLinkWaitingForScript();
            if gWirelessCommType != 0 {
                CreateTask(Some(Task_ReturnToFieldRecordMixing), 10);
            }
            ClearDialogWindowAndFrame(0, TRUE);
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_MixingRecordsRecv(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            PrintTextOnRecordMixing(
                (*(&raw const crate::data::strings::gText_MixingRecords).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            (*task).data[8] = 0x708;
            (*task).data[0] = 400;
            ClearLinkCallback_2();
        }
        100 => {
            if ({
                (*task).data[12] += 1;
                (*task).data[12]
            }) > 20
            {
                (*task).data[12] = 0;
                (*task).data[0] = 101;
            }
        }
        101 => {
            let players: u8 = GetLinkPlayerCount_2();
            if IsLinkMaster() == TRUE {
                if players == GetSavedPlayerCount() {
                    PlaySE(SE_PIN);
                    (*task).data[0] = 201;
                    (*task).data[12] = 0;
                }
            } else {
                PlaySE(SE_BOO);
                (*task).data[0] = 301;
            }
        }
        201 => {
            if GetSavedPlayerCount() == GetLinkPlayerCount_2()
                && ({
                    (*task).data[12] += 1;
                    (*task).data[12]
                }) as i32
                    > GetLinkPlayerCount_2() as i32 * 30
            {
                CheckShouldAdvanceLinkState();
                (*task).data[0] = 1;
            }
        }
        301 => {
            if GetSavedPlayerCount() == GetLinkPlayerCount_2() {
                (*task).data[0] = 1;
            }
        }
        400 => {
            if ({
                (*task).data[12] += 1;
                (*task).data[12]
            }) > 20
            {
                (*task).data[0] = 1;
                (*task).data[12] = 0;
            }
        }
        1 => {
            if gReceivedRemoteLinkPlayers != 0 {
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    GetMultiplayerId_() as i32,
                    STR_CONV_MODE_LEADING_ZEROS,
                    2,
                );
                (*task).data[0] = 5;
            }
        }
        2 => {
            let mut subTaskId: u8 = 0;
            (*task).data[6] = GetLinkPlayerCount_2() as i16;
            (*task).data[0] = 0;
            (*task).data[5] = GetMultiplayerId_() as i16;
            (*task).func = Some(Task_SendPacket);
            if Link_AnyPartnersPlayingRubyOrSapphire() != 0 {
                StorePtrInTaskData(
                    sSentRecord as *mut c_void,
                    &raw mut (*task).data[tSentRecord] as *mut u16,
                );
                subTaskId = CreateTask(Some(Task_CopyReceiveBuffer), 80);
                (*task).data[10] = subTaskId as i16;
                task_set(subTaskId, 0, taskId as i16);
                StorePtrInTaskData(
                    sReceivedRecords as *mut c_void,
                    task_data_ptr(subTaskId, 5) as *mut u16,
                );
                sRecordStructSize.set(4656);
            } else {
                StorePtrInTaskData(
                    sSentRecord as *mut c_void,
                    &raw mut (*task).data[tSentRecord] as *mut u16,
                );
                subTaskId = CreateTask(Some(Task_CopyReceiveBuffer), 80);
                (*task).data[10] = subTaskId as i16;
                task_set(subTaskId, 0, taskId as i16);
                StorePtrInTaskData(
                    sReceivedRecords as *mut c_void,
                    task_data_ptr(subTaskId, 5) as *mut u16,
                );
                sRecordStructSize.set(5188);
            }
        }
        5 if ({
            (*task).data[10] += 1;
            (*task).data[10]
        }) > 60 =>
        {
            (*task).data[10] = 0;
            (*task).data[0] = 2;
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_SendPacket(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            let recordData: *mut c_void =
                (LoadPtrFromTaskData(&raw mut (*task).data[tSentRecord] as *mut u16) as *mut u8)
                    .at((*task).data[tNumChunksSent] as i32 * BUFFER_CHUNK_SIZE as i32)
                    as *mut c_void;
            memcpy(
                gBlockSendBuffer.as_mut_ptr(),
                recordData as *mut u8,
                BUFFER_CHUNK_SIZE,
            );
            (*task).data[tState] += 1;
        }
        1 => {
            if GetMultiplayerId() == 0 {
                SendBlockRequest(BLOCK_REQ_SIZE_200);
            }
            (*task).data[tState] += 1;
        }
        2 => {}
        3 => {
            (*task).data[tNumChunksSent] += 1;
            if (*task).data[tNumChunksSent] as u32 == sRecordStructSize.get() / 200 + 1 {
                (*task).data[tState] += 1;
            } else {
                (*task).data[tState] = 0;
            }
        }
        4 if (*gTasks.as_ptr())[(*task).data[tCopyTaskId]].isActive == 0 => {
            (*task).func = Some(Task_SendPacket_SwitchToReceive);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_CopyReceiveBuffer(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let status: u8 = GetBlockReceivedStatus();
    let mut handledPlayers: u8 = 0;
    if status == GetLinkPlayerCountAsBitFlags() {
        let mut i: u8 = 0;
        while i < GetLinkPlayerCount() {
            if shr_i32(status as i32, i as u32) & 1 != 0 {
                let dest: *mut c_void =
                    ((LoadPtrFromTaskData(&raw mut (*task).data[tRecvRecords] as *mut u16)
                        as *mut u8)
                        .at((*task).data[1 + i as i32] as i32 * BUFFER_CHUNK_SIZE as i32)
                        as *mut c_void as *mut u8)
                        .at(sRecordStructSize.get() * i as u32) as *mut c_void;
                let src: *mut c_void = GetPlayerRecvBuffer(i);
                if ((*task).data[1 + i as i32] as u32 + 1) * BUFFER_CHUNK_SIZE
                    > sRecordStructSize.get()
                {
                    memcpy(
                        dest as *mut u8,
                        src as *mut u8,
                        sRecordStructSize.get()
                            - (*task).data[1 + i as i32] as u32 * BUFFER_CHUNK_SIZE,
                    );
                } else {
                    memcpy(dest as *mut u8, src as *mut u8, BUFFER_CHUNK_SIZE);
                }
                ResetBlockReceivedFlag(i);
                (*task).data[1 + i as i32] += 1;
                if (*task).data[1 + i as i32] as u32 == sRecordStructSize.get() / 200 + 1 {
                    handledPlayers += 1;
                }
            }
            i += 1;
        }
        task_set((*task).data[0], 0, task_get((*task).data[0], 0) + 1);
    }
    if handledPlayers == GetLinkPlayerCount() {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_WaitReceivePacket(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*gTasks.as_ptr())[(*task).data[tCopyTaskId]].isActive == 0 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_ReceivePacket(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).func = Some(Task_WaitReceivePacket);
    if sReadyToReceive.get() == TRUE {
        ReceiveExchangePacket((*task).data[tMultiplayerId] as u32);
    }
}
pub(crate) unsafe fn Task_SendPacket_SwitchToReceive(taskId: u8) {
    task_set_func(taskId, Some(Task_ReceivePacket));
    sReadyToReceive.set(TRUE);
}
unsafe fn LoadPtrFromTaskData(asShort: *mut u16) -> *mut c_void {
    (*asShort as i32 | (*asShort.at(1) as i32) << 16) as usize as *mut c_void
}
unsafe fn StorePtrInTaskData(records: *mut c_void, asShort: *mut u16) {
    *asShort = records as usize as u32 as u16;
    *asShort.at(1) = (records as usize as u32 >> 16) as u16;
}
unsafe fn GetMultiplayerId_() -> u8 {
    GetMultiplayerId()
}
unsafe fn GetPlayerRecvBuffer(id: u8) -> *mut c_void {
    gBlockRecvBuffer[id].as_mut_ptr() as *mut c_void
}
unsafe fn ShufflePlayerIndices(data: *mut u32) {
    let mut linkTrainerId: u32 = 0;
    let players: u32 = GetLinkPlayerCount() as u32;
    match players {
        2 => {
            for i in 0..2u32 {
                *data.at(i) = sPlayerIdxOrders_2Player[i] as u32;
            }
        }
        3 => {
            linkTrainerId = GetLinkPlayerTrainerId(0) % 2;
            for i in 0..3u32 {
                *data.at(i) = sPlayerIdxOrders_3Player[linkTrainerId][i] as u32;
            }
        }
        4 => {
            linkTrainerId = GetLinkPlayerTrainerId(0) % 9;
            for i in 0..4u32 {
                *data.at(i) = sPlayerIdxOrders_4Player[linkTrainerId][i] as u32;
            }
        }
        _ => {}
    }
}
unsafe fn ReceiveOldManData(records: *mut OldMan, recordSize: u32, multiplayerId: u8) {
    let mut version: u8 = 0;
    let mut language: u16 = 0;
    let mut mixIndices: CArray<u32, 4> = zeroed();
    ShufflePlayerIndices(mixIndices.as_mut_ptr());
    let oldMan: *mut OldMan = (records as *mut c_void as *mut u8)
        .at(recordSize * mixIndices[multiplayerId]) as *mut c_void
        as *mut OldMan;
    version = gLinkPlayers[mixIndices[multiplayerId]].version as u8;
    language = gLinkPlayers[mixIndices[multiplayerId]].language;
    if Link_AnyPartnersPlayingRubyOrSapphire() != 0 {
        SanitizeReceivedRubyOldMan(oldMan, version as u32, language as u32);
    } else {
        SanitizeReceivedEmeraldOldMan(oldMan, version as u32, language as u32);
    }
    memcpy(
        sOldManSave as *mut u8,
        (records as *mut c_void as *mut u8).at(recordSize * mixIndices[multiplayerId])
            as *mut c_void as *mut u8,
        64,
    );
    ResetMauvilleOldManFlag();
}
unsafe fn ReceiveBattleTowerData(records: *mut c_void, recordSize: u32, multiplayerId: u8) {
    let mut battleTowerRecord: *mut EmeraldBattleTowerRecord = null_mut();
    let mut btPokemon: *mut BattleTowerPokemon = null_mut();
    let mut mixIndices: CArray<u32, 4> = zeroed();
    let mut i: i32 = 0;
    ShufflePlayerIndices(mixIndices.as_mut_ptr());
    if Link_AnyPartnersPlayingRubyOrSapphire() != 0 {
        if RubyBattleTowerRecordToEmerald(
            (records as *mut u8).at(recordSize * mixIndices[multiplayerId]) as *mut c_void
                as *mut RSBattleTowerRecord,
            (records as *mut u8).at(recordSize * multiplayerId as u32) as *mut c_void
                as *mut EmeraldBattleTowerRecord,
        ) == TRUE as u32
        {
            battleTowerRecord = (records as *mut u8).at(recordSize * multiplayerId as u32)
                as *mut c_void as *mut EmeraldBattleTowerRecord;
            (*battleTowerRecord).language = gLinkPlayers[mixIndices[multiplayerId]].language as u8;
            CalcEmeraldBattleTowerChecksum(battleTowerRecord);
        }
    } else {
        memcpy(
            (records as *mut u8).at(recordSize * multiplayerId as u32) as *mut c_void as *mut u8,
            (records as *mut u8).at(recordSize * mixIndices[multiplayerId]) as *mut c_void
                as *mut u8,
            236,
        );
        battleTowerRecord = (records as *mut u8).at(recordSize * multiplayerId as u32)
            as *mut c_void as *mut EmeraldBattleTowerRecord;
        i = 0;
        while i
            < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
                3
            } else {
                if 4 >= 2 { 4 } else { 2 }
            })
        {
            btPokemon = &raw mut (*battleTowerRecord).party[i];
            if (*btPokemon).species != SPECIES_NONE
                && IsStringJapanese((*btPokemon).nickname.as_mut_ptr()) != 0
            {
                ConvertInternationalString((*btPokemon).nickname.as_mut_ptr(), LANGUAGE_JAPANESE);
            }
            i += 1;
        }
        CalcEmeraldBattleTowerChecksum(battleTowerRecord);
    }
    PutNewBattleTowerRecord((records as *mut u8).at(recordSize * multiplayerId as u32)
        as *mut c_void as *mut EmeraldBattleTowerRecord);
}
unsafe fn ReceiveLilycoveLadyData(records: *mut LilycoveLady, recordSize: u32, multiplayerId: u8) {
    let mut lilycoveLady: *mut LilycoveLady = null_mut();
    let mut mixIndices: CArray<u32, 4> = zeroed();
    ShufflePlayerIndices(mixIndices.as_mut_ptr());
    memcpy(
        (records as *mut c_void as *mut u8).at(recordSize * multiplayerId as u32) as *mut c_void
            as *mut u8,
        sLilycoveLadySave as *mut u8,
        64,
    );
    if GetLilycoveLadyId() == 0 {
        lilycoveLady = Alloc(64) as *mut LilycoveLady;
        if lilycoveLady.is_null() {
            return;
        }
        memcpy(lilycoveLady as *mut u8, sLilycoveLadySave as *mut u8, 64);
    } else {
        lilycoveLady = null_mut();
    }
    memcpy(
        sLilycoveLadySave as *mut u8,
        (records as *mut c_void as *mut u8).at(recordSize * mixIndices[multiplayerId])
            as *mut c_void as *mut u8,
        64,
    );
    ResetLilycoveLadyForRecordMix();
    if !lilycoveLady.is_null() {
        QuizLadyClearQuestionForRecordMix(lilycoveLady);
        Free(lilycoveLady as *mut c_void);
    }
}
unsafe fn GetDaycareMailItemId(mail: *mut DaycareMail) -> u8 {
    (*mail).message.itemId as u8
}
unsafe fn SwapDaycareMail(
    records: *mut RecordMixingDaycareMail,
    recordSize: u32,
    idxs: *mut CArray<u8, 2>,
    playerSlot1: u8,
    playerSlot2: u8,
) {
    let mut temp: DaycareMail = zeroed();
    let mixMail1: *mut RecordMixingDaycareMail =
        (records as *mut c_void as *mut u8).at(recordSize * (*idxs.at(playerSlot1))[0] as u32)
            as *mut c_void as *mut RecordMixingDaycareMail;
    memcpy(
        &raw mut temp as *mut u8,
        &raw mut (*mixMail1).mail[(*idxs.at(playerSlot1))[1]] as *mut u8,
        56,
    );
    let mixMail2: *mut RecordMixingDaycareMail =
        (records as *mut c_void as *mut u8).at(recordSize * (*idxs.at(playerSlot2))[0] as u32)
            as *mut c_void as *mut RecordMixingDaycareMail;
    memcpy(
        &raw mut (*mixMail1).mail[(*idxs.at(playerSlot1))[1]] as *mut u8,
        &raw mut (*mixMail2).mail[(*idxs.at(playerSlot2))[1]] as *mut u8,
        56,
    );
    memcpy(
        &raw mut (*mixMail2).mail[(*idxs.at(playerSlot2))[1]] as *mut u8,
        &raw mut temp as *mut u8,
        56,
    );
}
unsafe fn CalculateDaycareMailRandSum(src: *mut u8) {
    let mut sum: u8 = 0;
    for i in 0..256i32 {
        sum += *src.at(i);
    }
    sDaycareMailRandSum.set(sum);
}
fn GetDaycareMailRandSum() -> u8 {
    sDaycareMailRandSum.get()
}
unsafe fn ReceiveDaycareMailData(
    records: *mut RecordMixingDaycareMail,
    recordSize: u32,
    multiplayerId: u8,
    shows: *mut TVShow,
) {
    let mut j: u16 = 0;
    let mut mixMail: *mut RecordMixingDaycareMail = null_mut();
    let mut playerSlot1: u8 = 0;
    let mut playerSlot2: u8 = 0;
    let mut ptr: *mut c_void = null_mut();
    let mut unusedArr1: CArray<u8, 4> = zeroed();
    let mut unusedArr2: CArray<u8, 4> = zeroed();
    let mut unusedMixMail: CArray<*mut RecordMixingDaycareMail, 4> = zeroed();
    let mut canHoldItem: CArray<CArray<u8, 2>, 4> = zeroed();
    let mut idxs: CArray<CArray<u8, 2>, 4> = zeroed();
    let oldSeed: u16 = Random2();
    SeedRng2(gLinkPlayers[0].trainerId as u16);
    let linkPlayerCount: u8 = GetLinkPlayerCount();
    for i in 0..(MAX_LINK_PLAYERS as u16) {
        unusedArr1[i] = 0xFF;
        unusedArr2[i] = 0;
        canHoldItem[i][0] = 0;
        canHoldItem[i][1] = FALSE;
    }
    let anyRS: u32 = Link_AnyPartnersPlayingRubyOrSapphire();
    let mut i: u16 = 0;
    while i < GetLinkPlayerCount() as u16 {
        let mut language: u32 = 0;
        let mut version: u32 = 0;
        mixMail = (records as *mut c_void as *mut u8).at(i as u32 * recordSize) as *mut c_void
            as *mut RecordMixingDaycareMail;
        language = gLinkPlayers[i].language as u32;
        version = gLinkPlayers[i].version as u32 & 0xFF;
        j = 0;
        while (j as u32) < (*mixMail).numDaycareMons {
            'l3: {
                let mut otNameLanguage: u16 = 0;
                let mut nicknameLanguage: u16 = 0;
                let daycareMail: *mut DaycareMail = &raw mut (*mixMail).mail[j];
                if (*daycareMail).message.itemId == ITEM_NONE {
                    break 'l3;
                }
                if anyRS != 0 {
                    if StringLength((*daycareMail).otName.as_mut_ptr()) <= 5 {
                        otNameLanguage = LANGUAGE_JAPANESE as u16;
                    } else {
                        StripExtCtrlCodes((*daycareMail).otName.as_mut_ptr());
                        otNameLanguage = language as u16;
                    }
                    if (*daycareMail).monName[0] == EXT_CTRL_CODE_BEGIN
                        && (*daycareMail).monName[1] == EXT_CTRL_CODE_JPN
                    {
                        StripExtCtrlCodes((*daycareMail).monName.as_mut_ptr());
                        nicknameLanguage = LANGUAGE_JAPANESE as u16;
                    } else {
                        nicknameLanguage = language as u16;
                    }
                    if version == VERSION_RUBY as u32 || version == VERSION_SAPPHIRE as u32 {
                        (*daycareMail).set_gameLanguage(otNameLanguage as u8);
                        (*daycareMail).set_monLanguage(nicknameLanguage as u8);
                    }
                } else if language == LANGUAGE_JAPANESE as u32 {
                    if IsStringJapanese((*daycareMail).otName.as_mut_ptr()) != 0 {
                        (*daycareMail).set_gameLanguage(LANGUAGE_JAPANESE);
                    } else {
                        (*daycareMail).set_gameLanguage(GAME_LANGUAGE);
                    }
                    if IsStringJapanese((*daycareMail).monName.as_mut_ptr()) != 0 {
                        (*daycareMail).set_monLanguage(LANGUAGE_JAPANESE);
                    } else {
                        (*daycareMail).set_monLanguage(GAME_LANGUAGE);
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    let mut numDaycareCanHold: u8 = 0;
    for i in 0..(linkPlayerCount as u16) {
        'l5: {
            mixMail = (records as *mut c_void as *mut u8).at(i as u32 * recordSize) as *mut c_void
                as *mut RecordMixingDaycareMail;
            if (*mixMail).numDaycareMons == 0 {
                break 'l5;
            }
            j = 0;
            while (j as u32) < (*mixMail).numDaycareMons {
                if (*mixMail).cantHoldItem[j] == 0 {
                    canHoldItem[i][j] = TRUE;
                }
                j += 1;
            }
        }
    }
    j = 0;
    i = 0;
    while i < linkPlayerCount as u16 {
        mixMail = (records as *mut c_void as *mut u8).at(i as u32 * recordSize) as *mut c_void
            as *mut RecordMixingDaycareMail;
        if canHoldItem[i][0] == 1 || canHoldItem[i][1] == 1 {
            numDaycareCanHold += 1;
        }
        if canHoldItem[i][0] == 1 && canHoldItem[i][1] == 0 {
            idxs[j][0] = i as u8;
            idxs[j][1] = 0;
            j += 1;
        } else if canHoldItem[i][0] == 0 && canHoldItem[i][1] == 1 {
            idxs[j][0] = i as u8;
            idxs[j][1] = 1;
            j += 1;
        } else if canHoldItem[i][0] == 1 && canHoldItem[i][1] == 1 {
            idxs[j][0] = i as u8;
            let itemId1: u32 = GetDaycareMailItemId(&raw mut (*mixMail).mail[0]) as u32;
            let itemId2: u32 = GetDaycareMailItemId(&raw mut (*mixMail).mail[1]) as u32;
            if itemId1 == 0 && itemId2 == 0 || itemId1 != 0 && itemId2 != 0 {
                idxs[j][1] = (Random2() as i32 % 2) as u8;
            } else if itemId1 != 0 && itemId2 == 0 {
                idxs[j][1] = 0;
            } else if itemId1 == 0 && itemId2 != 0 {
                idxs[j][1] = 1;
            }
            j += 1;
        }
        i += 1;
    }
    for i in 0..(MAX_LINK_PLAYERS as u16) {
        mixMail = records.at(multiplayerId as u32 * recordSize);
        unusedMixMail[i] = mixMail;
    }
    let tableId: u8 = (GetDaycareMailRandSum() as i32 % 3) as u8;
    match numDaycareCanHold {
        2 => {
            SwapDaycareMail(records, recordSize, idxs.as_mut_ptr(), 0, 1);
        }
        3 => {
            playerSlot1 = sDaycareMailSwapIds_3Player[tableId][0];
            playerSlot2 = sDaycareMailSwapIds_3Player[tableId][1];
            SwapDaycareMail(
                records,
                recordSize,
                idxs.as_mut_ptr(),
                playerSlot1,
                playerSlot2,
            );
        }
        4 => {
            ptr = idxs.as_mut_ptr() as *mut c_void;
            playerSlot1 = sDaycareMailSwapIds_4Player[tableId][0];
            playerSlot2 = sDaycareMailSwapIds_4Player[tableId][1];
            SwapDaycareMail(
                records,
                recordSize,
                ptr as *mut CArray<u8, 2>,
                playerSlot1,
                playerSlot2,
            );
            playerSlot1 = sDaycareMailSwapIds_4Player[tableId][2];
            playerSlot2 = sDaycareMailSwapIds_4Player[tableId][3];
            SwapDaycareMail(
                records,
                recordSize,
                ptr as *mut CArray<u8, 2>,
                playerSlot1,
                playerSlot2,
            );
        }
        _ => {}
    }
    mixMail = (records as *mut c_void as *mut u8).at(multiplayerId as u32 * recordSize)
        as *mut c_void as *mut RecordMixingDaycareMail;
    memcpy(
        &raw mut (*gSaveBlock1Ptr).daycare.mons[0].mail as *mut u8,
        &raw mut (*mixMail).mail[0] as *mut u8,
        56,
    );
    memcpy(
        &raw mut (*gSaveBlock1Ptr).daycare.mons[1].mail as *mut u8,
        &raw mut (*mixMail).mail[1] as *mut u8,
        56,
    );
    SeedRng(oldSeed);
}
unsafe fn ReceiveGiftItem(item: *mut u16, multiplayerId: u8) {
    if multiplayerId != 0 && *item != ITEM_NONE && GetPocketByItemId(*item) == POCKET_KEY_ITEMS {
        if CheckBagHasItem(*item, 1) == 0
            && CheckPCHasItem(*item, 1) == 0
            && AddBagItem(*item, 1) != 0
        {
            VarSet(VAR_TEMP_RECORD_MIX_GIFT_ITEM, *item);
            StringCopy(gStringVar1.as_mut_ptr(), gLinkPlayers[0].name.as_mut_ptr());
            if *item == ITEM_EON_TICKET {
                FlagSet(FLAG_ENABLE_SHIP_SOUTHERN_ISLAND);
            }
        } else {
            VarSet(VAR_TEMP_RECORD_MIX_GIFT_ITEM, ITEM_NONE);
        }
    }
}
pub(crate) unsafe fn Task_DoRecordMixing(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tState] += 1;
        }
        1 => {
            if Link_AnyPartnersPlayingRubyOrSapphire() != 0 {
                (*task).data[tState] += 1;
            } else {
                (*task).data[tState] = 6;
            }
        }
        2 => {
            SetContinueGameWarpStatusToDynamicWarp();
            WriteSaveBlock2();
            (*task).data[tState] += 1;
        }
        3 => {
            if WriteSaveBlock1Sector() != 0 {
                ClearContinueGameWarpStatus2();
                (*task).data[tState] = 4;
                (*task).data[1] = 0;
            }
        }
        4 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 10
            {
                SetCloseLinkCallback();
                (*task).data[tState] += 1;
            }
        }
        5 => {
            if gReceivedRemoteLinkPlayers == FALSE {
                DestroyTask(taskId);
            }
        }
        6 => {
            if Rfu_SetLinkRecovery(FALSE as u32) == 0 {
                CreateTask(Some(Task_LinkFullSave), 5);
                (*task).data[tState] += 1;
            }
        }
        7 => {
            if FuncIsActiveTask(Some(Task_LinkFullSave)) == 0 {
                if gWirelessCommType != 0 {
                    Rfu_SetLinkRecovery(TRUE as u32);
                    (*task).data[tState] = 8;
                } else {
                    (*task).data[tState] = 4;
                }
            }
        }
        8 => {
            SetLinkStandbyCallback();
            (*task).data[tState] += 1;
        }
        9 if IsLinkTaskFinished() != 0 => {
            DestroyTask(taskId);
        }
        _ => {}
    }
}
unsafe fn GetSavedApprentices(dst: *mut Apprentice, src: *mut Apprentice) {
    let mut id: i32 = 0;
    (*dst).playerName[0] = EOS;
    (*dst.at(1)).playerName[0] = EOS;
    *dst = *src;
    let mut oldPlayerApprenticeSaveId: i32 = 0;
    let mut numOldPlayerApprentices: i32 = 0;
    let mut apprenticeSaveId: i32 = 0;
    let mut numMixApprentices: i32 = 0;
    for i in 0..2i32 {
        id = (i + (*gSaveBlock2Ptr).playerApprentice.saveId() as i32) % 3 + 1;
        if (*src.at(id)).playerName[0] != EOS {
            if GetTrainerId((*src.at(id)).playerId.as_mut_ptr())
                != GetTrainerId((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr())
            {
                numMixApprentices += 1;
                apprenticeSaveId = id;
            }
            if GetTrainerId((*src.at(id)).playerId.as_mut_ptr())
                == GetTrainerId((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr())
            {
                numOldPlayerApprentices += 1;
                oldPlayerApprenticeSaveId = id;
            }
        }
    }
    if numMixApprentices == 0 && numOldPlayerApprentices != 0 {
        numMixApprentices = numOldPlayerApprentices;
        apprenticeSaveId = oldPlayerApprenticeSaveId;
    }
    match numMixApprentices {
        1 => {
            *dst.at(1) = *src.at(apprenticeSaveId);
        }
        2 => {
            if Random2() > 0x3333 {
                *dst.at(1) = *src.at((*gSaveBlock2Ptr).playerApprentice.saveId() as i32 + 1);
            } else {
                *dst.at(1) =
                    *src.at(((*gSaveBlock2Ptr).playerApprentice.saveId() as i32 + 1) % 3 + 1);
            }
        }
        _ => {}
    }
}
pub unsafe fn GetPlayerHallRecords(dst: *mut PlayerHallRecords) {
    let mut i: i32 = 0;
    while i < HALL_FACILITIES_COUNT {
        for j in 0..FRONTIER_LVL_MODE_COUNT {
            CopyTrainerId(
                (*dst).onePlayer[i][j].id.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr(),
            );
            (*dst).onePlayer[i][j].language = GAME_LANGUAGE;
            StringCopy(
                (*dst).onePlayer[i][j].name.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
        }
        i += 1;
    }
    for j in 0..FRONTIER_LVL_MODE_COUNT {
        (*dst).twoPlayers[j].language = GAME_LANGUAGE;
        CopyTrainerId(
            (*dst).twoPlayers[j].id1.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr(),
        );
        CopyTrainerId(
            (*dst).twoPlayers[j].id2.as_mut_ptr(),
            (*gSaveBlock2Ptr).frontier.opponentTrainerIds[j].as_mut_ptr(),
        );
        StringCopy(
            (*dst).twoPlayers[j].name1.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StringCopy(
            (*dst).twoPlayers[j].name2.as_mut_ptr(),
            (*gSaveBlock2Ptr).frontier.opponentNames[j].as_mut_ptr(),
        );
    }
    for i in 0..FRONTIER_LVL_MODE_COUNT {
        (*dst).onePlayer[0][i].winStreak = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[0][i];
        (*dst).onePlayer[1][i].winStreak = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[1][i];
        (*dst).onePlayer[2][i].winStreak = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[2][i];
        (*dst).onePlayer[3][i].winStreak = (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[0][i];
        (*dst).onePlayer[4][i].winStreak = (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[0][i];
        (*dst).onePlayer[5][i].winStreak = (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[i];
        (*dst).onePlayer[6][i].winStreak = (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[0][i];
        (*dst).onePlayer[7][i].winStreak = (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[i];
        (*dst).onePlayer[8][i].winStreak = (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[i];
        (*dst).twoPlayers[i].winStreak = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[3][i];
    }
}
unsafe fn IsApprenticeAlreadySaved(
    mixApprentice: *mut Apprentice,
    apprentices: *mut Apprentice,
) -> u32 {
    for i in 0..APPRENTICE_COUNT {
        if GetTrainerId((*mixApprentice).playerId.as_mut_ptr())
            == GetTrainerId((*apprentices.at(i)).playerId.as_mut_ptr())
            && (*mixApprentice).number == (*apprentices.at(i)).number
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn ReceiveApprenticeData(records: *mut Apprentice, recordSize: u32, multiplayerId: u32) {
    let mut mixIndices: CArray<u32, 4> = zeroed();
    let mut apprenticeSaveId: u32 = 0;
    ShufflePlayerIndices(mixIndices.as_mut_ptr());
    let mixApprentice: *mut Apprentice = (records as *mut c_void as *mut u8)
        .at(recordSize * mixIndices[multiplayerId])
        as *mut c_void as *mut Apprentice;
    let mut numApprentices: i32 = 0;
    let mut apprenticeId: i32 = 0;
    let mut i: i32 = 0;
    while i < 2 {
        if (*mixApprentice.at(i)).playerName[0] != EOS
            && IsApprenticeAlreadySaved(
                mixApprentice.at(i),
                &raw mut (*gSaveBlock2Ptr).apprentices[0],
            ) == 0
        {
            numApprentices += 1;
            apprenticeId = i;
        }
        i += 1;
    }
    match numApprentices {
        1 => {
            apprenticeSaveId = (*gSaveBlock2Ptr).playerApprentice.saveId() as u32 + 1;
            (*gSaveBlock2Ptr).apprentices[apprenticeSaveId] = *mixApprentice.at(apprenticeId);
            (*gSaveBlock2Ptr)
                .playerApprentice
                .set_saveId((((*gSaveBlock2Ptr).playerApprentice.saveId() as i32 + 1) % 3) as u8);
        }
        2 => {
            for i in 0..2i32 {
                apprenticeSaveId =
                    (((i ^ 1) + (*gSaveBlock2Ptr).playerApprentice.saveId() as i32) % 3) as u32 + 1;
                (*gSaveBlock2Ptr).apprentices[apprenticeSaveId] = *mixApprentice.at(i);
            }
            (*gSaveBlock2Ptr)
                .playerApprentice
                .set_saveId((((*gSaveBlock2Ptr).playerApprentice.saveId() as i32 + 2) % 3) as u8);
        }
        _ => {}
    }
}
unsafe fn GetNewHallRecords(
    dst: *mut RecordMixingHallRecords,
    mut records: *mut c_void,
    recordSize: u32,
    multiplayerId: u32,
    linkPlayerCount: i32,
) {
    let mut repeatTrainers: i32 = 0;
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    while i < linkPlayerCount {
        if i as u32 != multiplayerId {
            sPartnerHallRecords[{
                let t1 = k;
                k += 1;
                t1
            }] = records as *mut PlayerHallRecords;
        }
        if k == HALL_RECORDS_COUNT {
            break;
        }
        records = (records as *mut u8).at(recordSize) as *mut c_void;
        i += 1;
    }
    for i in 0..HALL_FACILITIES_COUNT {
        for j in 0..FRONTIER_LVL_MODE_COUNT {
            k = 0;
            while k < HALL_RECORDS_COUNT {
                (*dst).hallRecords1P[i][j][k] = (*gSaveBlock2Ptr).hallRecords1P[i][j][k];
                k += 1;
            }
            for k in 0..(linkPlayerCount - 1) {
                repeatTrainers = 0;
                for l in 0..HALL_RECORDS_COUNT {
                    if GetTrainerId((*dst).hallRecords1P[i][j][l].id.as_mut_ptr())
                        == GetTrainerId((*sPartnerHallRecords[k]).onePlayer[i][j].id.as_mut_ptr())
                    {
                        repeatTrainers += 1;
                        if (*dst).hallRecords1P[i][j][l].winStreak
                            < (*sPartnerHallRecords[k]).onePlayer[i][j].winStreak
                        {
                            (*dst).hallRecords1P[i][j][l] =
                                (*sPartnerHallRecords[k]).onePlayer[i][j];
                        }
                    }
                }
                if repeatTrainers == 0 {
                    (*dst).hallRecords1P[i][j][k + HALL_RECORDS_COUNT] =
                        (*sPartnerHallRecords[k]).onePlayer[i][j];
                }
            }
        }
    }
    for j in 0..FRONTIER_LVL_MODE_COUNT {
        k = 0;
        while k < HALL_RECORDS_COUNT {
            (*dst).hallRecords2P[j][k] = (*gSaveBlock2Ptr).hallRecords2P[j][k];
            k += 1;
        }
        for k in 0..(linkPlayerCount - 1) {
            repeatTrainers = 0;
            for l in 0..HALL_RECORDS_COUNT {
                if GetTrainerId((*dst).hallRecords2P[j][l].id1.as_mut_ptr())
                    == GetTrainerId((*sPartnerHallRecords[k]).twoPlayers[j].id1.as_mut_ptr())
                    && GetTrainerId((*dst).hallRecords2P[j][l].id2.as_mut_ptr())
                        == GetTrainerId((*sPartnerHallRecords[k]).twoPlayers[j].id2.as_mut_ptr())
                {
                    repeatTrainers += 1;
                    if (*dst).hallRecords2P[j][l].winStreak
                        < (*sPartnerHallRecords[k]).twoPlayers[j].winStreak
                    {
                        (*dst).hallRecords2P[j][l] = (*sPartnerHallRecords[k]).twoPlayers[j];
                    }
                }
            }
            if repeatTrainers == 0 {
                (*dst).hallRecords2P[j][k + HALL_RECORDS_COUNT] =
                    (*sPartnerHallRecords[k]).twoPlayers[j];
            }
        }
    }
}
unsafe fn FillWinStreakRecords1P(
    playerRecords: *mut RankingHall1P,
    mixRecords: *mut RankingHall1P,
) {
    for i in 0..HALL_RECORDS_COUNT {
        let mut highestWinStreak: i32 = 0;
        let mut highestId: i32 = -1;
        for j in 0..6i32 {
            if (*mixRecords.at(j)).winStreak as i32 > highestWinStreak {
                highestId = j;
                highestWinStreak = (*mixRecords.at(j)).winStreak as i32;
            }
        }
        if highestId >= 0 {
            *playerRecords.at(i) = *mixRecords.at(highestId);
            (*mixRecords.at(highestId)).winStreak = 0;
        }
    }
}
unsafe fn FillWinStreakRecords2P(
    playerRecords: *mut RankingHall2P,
    mixRecords: *mut RankingHall2P,
) {
    for i in 0..HALL_RECORDS_COUNT {
        let mut highestWinStreak: i32 = 0;
        let mut highestId: i32 = -1;
        for j in 0..6i32 {
            if (*mixRecords.at(j)).winStreak as i32 > highestWinStreak {
                highestId = j;
                highestWinStreak = (*mixRecords.at(j)).winStreak as i32;
            }
        }
        if highestId >= 0 {
            *playerRecords.at(i) = *mixRecords.at(highestId);
            (*mixRecords.at(highestId)).winStreak = 0;
        }
    }
}
unsafe fn SaveHighestWinStreakRecords(mixHallRecords: *mut RecordMixingHallRecords) {
    for i in 0..HALL_FACILITIES_COUNT {
        for j in 0..FRONTIER_LVL_MODE_COUNT {
            FillWinStreakRecords1P(
                (*gSaveBlock2Ptr).hallRecords1P[i][j].as_mut_ptr(),
                (*mixHallRecords).hallRecords1P[i][j].as_mut_ptr(),
            );
        }
    }
    for j in 0..FRONTIER_LVL_MODE_COUNT {
        FillWinStreakRecords2P(
            (*gSaveBlock2Ptr).hallRecords2P[j].as_mut_ptr(),
            (*mixHallRecords).hallRecords2P[j].as_mut_ptr(),
        );
    }
}
unsafe fn ReceiveRankingHallRecords(
    records: *mut PlayerHallRecords,
    recordSize: u32,
    multiplayerId: u32,
) {
    let linkPlayerCount: u8 = GetLinkPlayerCount();
    let mixHallRecords: *mut RecordMixingHallRecords =
        AllocZeroed(2064) as *mut RecordMixingHallRecords;
    GetNewHallRecords(
        mixHallRecords,
        records as *mut c_void,
        recordSize,
        multiplayerId,
        linkPlayerCount as i32,
    );
    SaveHighestWinStreakRecords(mixHallRecords);
    Free(mixHallRecords as *mut c_void);
}
unsafe fn GetRecordMixingDaycareMail(dst: *mut RecordMixingDaycareMail) {
    sRecordMixMail.mail[0] = (*gSaveBlock1Ptr).daycare.mons[0].mail;
    sRecordMixMail.mail[1] = (*gSaveBlock1Ptr).daycare.mons[1].mail;
    InitDaycareMailRecordMixing(&raw mut (*gSaveBlock1Ptr).daycare, &raw mut sRecordMixMail);
    *dst = *sRecordMixMailSave;
}
unsafe fn SanitizeDaycareMailForRuby(src: *mut RecordMixingDaycareMail) {
    let mut i: i32 = 0;
    while (i as u32) < (*src).numDaycareMons {
        let mail: *mut DaycareMail = &raw mut (*src).mail[i];
        if (*mail).message.itemId != ITEM_NONE {
            if (*mail).gameLanguage() != LANGUAGE_JAPANESE {
                PadNameString((*mail).otName.as_mut_ptr(), EXT_CTRL_CODE_BEGIN);
            }
            ConvertInternationalString((*mail).monName.as_mut_ptr(), (*mail).monLanguage());
        }
        i += 1;
    }
}
unsafe fn SanitizeRubyBattleTowerRecord(src: *mut RSBattleTowerRecord) {}
unsafe fn SanitizeEmeraldBattleTowerRecord(dst: *mut EmeraldBattleTowerRecord) {
    let mut i: i32 = 0;
    while i
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        let towerMon: *mut BattleTowerPokemon = &raw mut (*dst).party[i];
        if (*towerMon).species != SPECIES_NONE {
            StripExtCtrlCodes((*towerMon).nickname.as_mut_ptr());
        }
        i += 1;
    }
    CalcEmeraldBattleTowerChecksum(dst);
}
