//! Translated from `src/record_mixing.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPlayerIdxOrders_2Player sPlayerIdxOrders_3Player sPlayerIdxOrders_4Player sDaycareMailSwapIds_3Player sDaycareMailSwapIds_4Player
#[allow(unused_imports)]
use crate::data::record_mixing::*;

pub(crate) static mut sReadyToReceive: u8 = 0u8;
pub(crate) static mut sSecretBasesSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sTvShowsSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sPokeNewsSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sOldManSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sDewfordTrendsSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sRecordMixMailSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sBattleTowerSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sLilycoveLadySave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sApprenticesSave: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sBattleTowerSave_Duplicate: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sRecordStructSize: u32 = 0u32;
pub(crate) static mut sDaycareMailRandSum: u8 = 0u8;
pub(crate) static mut sPartnerHallRecords: crate::ffi::Align4<[u8; 12]> =
    crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixMail: crate::ffi::Align4<[u8; 120]> = crate::ffi::Align4([0; 120]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReceivedRecords: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSentRecord: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBlockRecvBuffer: u8;
    static mut gBlockSendBuffer: u8;
    static mut gLinkPlayers: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gStringVar1: u8;
    static mut gTasks: u8;
    static mut gText_MixingRecords: u8;
    static mut gText_RecordMixingComplete: u8;
    static mut gWirelessCommType: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CalcEmeraldBattleTowerChecksum(a0: *mut u8);
    fn CheckBagHasItem(a0: u16, a1: u16) -> u8;
    fn CheckPCHasItem(a0: u16, a1: u16) -> u8;
    fn CheckShouldAdvanceLinkState();
    fn ClearContinueGameWarpStatus2();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearJapaneseSecretBases(a0: *mut u8);
    fn ClearLinkCallback_2();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyTrainerId(a0: *mut u8, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateRecordMixingLights() -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_EnterCableClubSeat(a0: Option<unsafe extern "C" fn(u8)>);
    fn CreateTask_ReestablishCableClubLink() -> u8;
    fn DeactivateAllNormalTVShows();
    fn DestroyRecordMixingLights();
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn EmeraldBattleTowerRecordToRuby(a0: *mut u8, a1: *mut u8) -> u32;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLilycoveLadyId() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetLinkPlayerCount_2() -> u8;
    fn GetLinkPlayerTrainerId(a0: u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetPocketByItemId(a0: u16) -> u8;
    fn GetRecordMixingGift() -> u16;
    fn GetSavedPlayerCount() -> u8;
    fn GetTrainerId(a0: *mut u8) -> u32;
    fn InitDaycareMailRecordMixing(a0: *mut u8, a1: *mut u8);
    fn IsLinkMaster() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsStringJapanese(a0: *mut u8) -> u32;
    fn LinkDummy_Return2() -> u32;
    fn Link_AnyPartnersPlayingRubyOrSapphire() -> u32;
    fn PadNameString(a0: *mut u8, a1: u8);
    fn PlaySE(a0: u16);
    fn PutNewBattleTowerRecord(a0: *mut u8);
    fn QuizLadyClearQuestionForRecordMix(a0: *mut u8);
    fn Random2() -> u16;
    fn ReceiveDewfordTrendData(a0: *mut u8, a1: u32, a2: u8);
    fn ReceivePokeNewsData(a0: *mut u8, a1: u32, a2: u8);
    fn ReceiveSecretBasesData(a0: *mut u8, a1: u32, a2: u8);
    fn ReceiveTvShowsData(a0: *mut u8, a1: u32, a2: u8);
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetLilycoveLadyForRecordMix();
    fn ResetMauvilleOldManFlag();
    fn Rfu_SetLinkRecovery(a0: u32) -> u8;
    fn RubyBattleTowerRecordToEmerald(a0: *mut u8, a1: *mut u8) -> u32;
    fn SanitizeMauvilleOldManForRuby(a0: *mut u8);
    fn SanitizeReceivedEmeraldOldMan(a0: *mut u8, a1: u32, a2: u32);
    fn SanitizeReceivedRubyOldMan(a0: *mut u8, a1: u32, a2: u32);
    fn SanitizeTVShowLocationsForRuby(a0: *mut u8);
    fn SanitizeTVShowsForRuby(a0: *mut u8);
    fn ScriptContext_Enable();
    fn SeedRng(a0: u16);
    fn SeedRng2(a0: u16);
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetCloseLinkCallback();
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetLinkStandbyCallback();
    fn SetLinkWaitingForScript() -> u16;
    fn SetLocalLinkPlayerId(a0: u8);
    fn SetPlayerSecretBaseParty();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn Task_LinkFullSave(a0: u8);
    fn Task_ReturnToFieldRecordMixing(a0: u8);
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WriteSaveBlock1Sector() -> u8;
    fn WriteSaveBlock2() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordMixingPlayerSpotTriggered() {
    unsafe {
        CreateTask_EnterCableClubSeat(Some(Task_RecordMixing_Main));
    }
}
pub(crate) unsafe extern "C" fn SetSrcLookupPointers() {
    unsafe {
        ((&raw mut sSecretBasesSave).cast::<u8>().cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>(),
        );
        ((&raw mut sTvShowsSave).cast::<u8>().cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>(),
        );
        ((&raw mut sPokeNewsSave).cast::<u8>().cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
                .cast::<u8>(),
        );
        ((&raw mut sOldManSave).cast::<u8>().cast::<*mut u8>())
            .write((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        ((&raw mut sDewfordTrendsSave).cast::<u8>().cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11880))
                .cast::<u8>(),
        );
        ((&raw mut sRecordMixMailSave).cast::<u8>().cast::<*mut u8>())
            .write((&raw mut sRecordMixMail).cast::<u8>());
        ((&raw mut sBattleTowerSave).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)));
        ((&raw mut sLilycoveLadySave).cast::<u8>().cast::<*mut u8>())
            .write((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192));
        ((&raw mut sApprenticesSave).cast::<u8>().cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220)).cast::<u8>(),
        );
        ((&raw mut sBattleTowerSave_Duplicate)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)));
    }
}
pub(crate) unsafe extern "C" fn PrepareUnknownExchangePacket(dest: *mut u8) {
    unsafe {
        let mut dest = dest;
        crate::c::memcpy(
            (dest).cast::<u8>(),
            ((&raw mut sSecretBasesSave).cast::<u8>().cast::<*mut u8>()).read(),
            3200u32,
        );
        crate::c::memcpy(
            ((dest).wrapping_add(3200)).cast::<u8>(),
            ((&raw mut sTvShowsSave).cast::<u8>().cast::<*mut u8>()).read(),
            900u32,
        );
        SanitizeTVShowLocationsForRuby(((dest).wrapping_add(3200)).cast::<u8>());
        crate::c::memcpy(
            ((dest).wrapping_add(4100)).cast::<u8>(),
            ((&raw mut sPokeNewsSave).cast::<u8>().cast::<*mut u8>()).read(),
            64u32,
        );
        crate::c::memcpy(
            (dest).wrapping_add(4164),
            ((&raw mut sOldManSave).cast::<u8>().cast::<*mut u8>()).read(),
            64u32,
        );
        crate::c::memcpy(
            ((dest).wrapping_add(4228)).cast::<u8>(),
            ((&raw mut sDewfordTrendsSave).cast::<u8>().cast::<*mut u8>()).read(),
            40u32,
        );
        GetRecordMixingDaycareMail((dest).wrapping_add(4268));
        EmeraldBattleTowerRecordToRuby(
            ((&raw mut sBattleTowerSave).cast::<u8>().cast::<*mut u8>()).read(),
            (dest).wrapping_add(4388),
        );
        if ((GetMultiplayerId()) as i32) == 0i32 {
            ((dest).wrapping_add(4552).cast::<u16>()).write(GetRecordMixingGift());
        }
    }
}
pub(crate) unsafe extern "C" fn PrepareExchangePacketForRubySapphire(dest: *mut u8) {
    unsafe {
        let mut dest = dest;
        crate::c::memcpy(
            (dest).cast::<u8>(),
            ((&raw mut sSecretBasesSave).cast::<u8>().cast::<*mut u8>()).read(),
            3200u32,
        );
        ClearJapaneseSecretBases((dest).cast::<u8>());
        crate::c::memcpy(
            ((dest).wrapping_add(3200)).cast::<u8>(),
            ((&raw mut sTvShowsSave).cast::<u8>().cast::<*mut u8>()).read(),
            900u32,
        );
        SanitizeTVShowsForRuby(((dest).wrapping_add(3200)).cast::<u8>());
        crate::c::memcpy(
            ((dest).wrapping_add(4100)).cast::<u8>(),
            ((&raw mut sPokeNewsSave).cast::<u8>().cast::<*mut u8>()).read(),
            64u32,
        );
        crate::c::memcpy(
            (dest).wrapping_add(4164),
            ((&raw mut sOldManSave).cast::<u8>().cast::<*mut u8>()).read(),
            64u32,
        );
        SanitizeMauvilleOldManForRuby((dest).wrapping_add(4164));
        crate::c::memcpy(
            ((dest).wrapping_add(4228)).cast::<u8>(),
            ((&raw mut sDewfordTrendsSave).cast::<u8>().cast::<*mut u8>()).read(),
            40u32,
        );
        GetRecordMixingDaycareMail((dest).wrapping_add(4268));
        SanitizeDaycareMailForRuby((dest).wrapping_add(4268));
        EmeraldBattleTowerRecordToRuby(
            ((&raw mut sBattleTowerSave).cast::<u8>().cast::<*mut u8>()).read(),
            (dest).wrapping_add(4388),
        );
        SanitizeRubyBattleTowerRecord((dest).wrapping_add(4388));
        if ((GetMultiplayerId()) as i32) == 0i32 {
            ((dest).wrapping_add(4552).cast::<u16>()).write(GetRecordMixingGift());
        }
    }
}
pub(crate) unsafe extern "C" fn PrepareExchangePacket() {
    unsafe {
        SetPlayerSecretBaseParty();
        DeactivateAllNormalTVShows();
        SetSrcLookupPointers();
        if (Link_AnyPartnersPlayingRubyOrSapphire()) != 0 {
            if LinkDummy_Return2() == 0u32 {
                PrepareUnknownExchangePacket(
                    (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()),
                );
            } else {
                PrepareExchangePacketForRubySapphire(
                    (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()),
                );
            }
        } else {
            crate::c::memcpy(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
                ((&raw mut sSecretBasesSave).cast::<u8>().cast::<*mut u8>()).read(),
                3200u32,
            );
            crate::c::memcpy(
                ((((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
                ((&raw mut sTvShowsSave).cast::<u8>().cast::<*mut u8>()).read(),
                900u32,
            );
            crate::c::memcpy(
                ((((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4100))
                .cast::<u8>(),
                ((&raw mut sPokeNewsSave).cast::<u8>().cast::<*mut u8>()).read(),
                64u32,
            );
            crate::c::memcpy(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4164),
                ((&raw mut sOldManSave).cast::<u8>().cast::<*mut u8>()).read(),
                64u32,
            );
            crate::c::memcpy(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4628),
                ((&raw mut sLilycoveLadySave).cast::<u8>().cast::<*mut u8>()).read(),
                64u32,
            );
            crate::c::memcpy(
                ((((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4228))
                .cast::<u8>(),
                ((&raw mut sDewfordTrendsSave).cast::<u8>().cast::<*mut u8>()).read(),
                40u32,
            );
            GetRecordMixingDaycareMail(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4268),
            );
            crate::c::memcpy(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4388),
                ((&raw mut sBattleTowerSave).cast::<u8>().cast::<*mut u8>()).read(),
                236u32,
            );
            SanitizeEmeraldBattleTowerRecord(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4388),
            );
            if ((GetMultiplayerId()) as i32) == 0i32 {
                ((((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4624)
                    .cast::<u16>())
                .write(GetRecordMixingGift());
            }
            GetSavedApprentices(
                ((((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4692))
                .cast::<u8>(),
                ((&raw mut sApprenticesSave).cast::<u8>().cast::<*mut u8>()).read(),
            );
            GetPlayerHallRecords(
                (((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4828),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ReceiveExchangePacket(multiplayerId: u32) {
    unsafe {
        let mut multiplayerId = multiplayerId;
        if (Link_AnyPartnersPlayingRubyOrSapphire()) != 0 {
            CalculateDaycareMailRandSum(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
            );
            ReceiveSecretBasesData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
                4656u32,
                ((multiplayerId) as u8),
            );
            ReceiveDaycareMailData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4268),
                4656u32,
                ((multiplayerId) as u8),
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
            );
            ReceiveBattleTowerData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4388),
                4656u32,
                ((multiplayerId) as u8),
            );
            ReceiveTvShowsData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
                4656u32,
                ((multiplayerId) as u8),
            );
            ReceivePokeNewsData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4100))
                .cast::<u8>(),
                4656u32,
                ((multiplayerId) as u8),
            );
            ReceiveOldManData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4164),
                4656u32,
                ((multiplayerId) as u8),
            );
            ReceiveDewfordTrendData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4228))
                .cast::<u8>(),
                4656u32,
                ((multiplayerId) as u8),
            );
            ReceiveGiftItem(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4552)
                    .cast::<u16>(),
                ((multiplayerId) as u8),
            );
        } else {
            CalculateDaycareMailRandSum(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
            );
            ReceiveSecretBasesData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceiveTvShowsData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceivePokeNewsData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4100))
                .cast::<u8>(),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceiveOldManData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4164),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceiveDewfordTrendData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4228))
                .cast::<u8>(),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceiveDaycareMailData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4268),
                5188u32,
                ((multiplayerId) as u8),
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3200))
                .cast::<u8>(),
            );
            ReceiveBattleTowerData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4388),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceiveGiftItem(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4624)
                    .cast::<u16>(),
                ((multiplayerId) as u8),
            );
            ReceiveLilycoveLadyData(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4628),
                5188u32,
                ((multiplayerId) as u8),
            );
            ReceiveApprenticeData(
                ((((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4692))
                .cast::<u8>(),
                5188u32,
                (((multiplayerId) as u8) as u32),
            );
            ReceiveRankingHallRecords(
                (((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4828),
                5188u32,
                (((multiplayerId) as u8) as u32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintTextOnRecordMixing(src: *mut u8) {
    unsafe {
        let mut src = src;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(0u8, 1u8, src, 0u8, 1u8, 0u8, None);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Task_RecordMixing_SoundEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 50i32
        {
            PlaySE(226u16);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RecordMixing_Main(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).write(Alloc(5188u32));
                ((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).write(Alloc(20752u32));
                SetLocalLinkPlayerId(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                );
                VarSet(16384u16, 1u16);
                ((&raw mut sReadyToReceive).cast::<u8>().cast::<u8>()).write(0u8);
                PrepareExchangePacket();
                CreateRecordMixingLights();
                (data).write(1i16);
                ((data).wrapping_offset(10))
                    .write(((CreateTask(Some(Task_MixingRecordsRecv), 80u8)) as i16));
                ((data).wrapping_offset(15))
                    .write(((CreateTask(Some(Task_RecordMixing_SoundEffect), 81u8)) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((data).wrapping_offset(10)).read()) as i32) as isize * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    (data).write(2i16);
                    FlagSet(2196u16);
                    DestroyRecordMixingLights();
                    DestroyTask(((((data).wrapping_offset(15)).read()) as u8));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((data).wrapping_offset(10))
                    .write(((CreateTask(Some(Task_DoRecordMixing), 10u8)) as i16));
                (data).write(3i16);
                PlaySE(224u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((data).wrapping_offset(10)).read()) as i32) as isize * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    (data).write(4i16);
                    if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32 {
                        ((data).wrapping_offset(10))
                            .write(((CreateTask_ReestablishCableClubLink()) as i16));
                    }
                    PrintTextOnRecordMixing((&raw mut gText_RecordMixingComplete).cast::<u8>());
                    ((data).wrapping_offset(8)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p2 = (data).wrapping_offset(8);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 60i32
                {
                    (data).write(5i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((data).wrapping_offset(10)).read()) as i32) as isize * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    Free(((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read());
                    Free(((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read());
                    SetLinkWaitingForScript();
                    if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
                        CreateTask(Some(Task_ReturnToFieldRecordMixing), 10u8);
                    }
                    ClearDialogWindowAndFrame(0u8, 1u8);
                    DestroyTask(taskId);
                    ScriptContext_Enable();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_MixingRecordsRecv(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                PrintTextOnRecordMixing((&raw mut gText_MixingRecords).cast::<u8>());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(1800i16);
                (((task).wrapping_add(8)).cast::<i16>()).write(400i16);
                ClearLinkCallback_2();
                break 'l1;
            }
            if __sw1 == 100i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 20i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(101i16);
                }
                break 'l1;
            }
            if __sw1 == 101i32 {
                {
                    let mut players: u8 = GetLinkPlayerCount_2();
                    if ((IsLinkMaster()) as i32) == 1i32 {
                        if ((players) as i32) == ((GetSavedPlayerCount()) as i32) {
                            PlaySE(21u16);
                            (((task).wrapping_add(8)).cast::<i16>()).write(201i16);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                                .write(0i16);
                        }
                    } else {
                        PlaySE(22u16);
                        (((task).wrapping_add(8)).cast::<i16>()).write(301i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 201i32 {
                if (((GetSavedPlayerCount()) as i32) == ((GetLinkPlayerCount_2()) as i32))
                    && ((({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        > ((GetLinkPlayerCount_2()) as i32).wrapping_mul(30i32))
                {
                    CheckShouldAdvanceLinkState();
                    (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 301i32 {
                if ((GetSavedPlayerCount()) as i32) == ((GetLinkPlayerCount_2()) as i32) {
                    (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 400i32 {
                if (({
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 20i32
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((GetMultiplayerId_()) as i32),
                        2i32,
                        2u8,
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(5i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    let mut subTaskId: u8 = 0u8;
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                        .write(((GetLinkPlayerCount_2()) as i16));
                    (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                        .write(((GetMultiplayerId_()) as i16));
                    ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_SendPacket));
                    if (Link_AnyPartnersPlayingRubyOrSapphire()) != 0 {
                        StorePtrInTaskData(
                            ((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read(),
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .cast::<u16>(),
                        );
                        subTaskId = CreateTask(Some(Task_CopyReceiveBuffer), 80u8);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                            .write(((subTaskId) as i16));
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((subTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(((taskId) as i16));
                        StorePtrInTaskData(
                            ((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read(),
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((subTaskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(5))
                            .cast::<u16>(),
                        );
                        ((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>()).write(4656u32);
                    } else {
                        StorePtrInTaskData(
                            ((&raw mut sSentRecord).cast::<u8>().cast::<*mut u8>()).read(),
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .cast::<u16>(),
                        );
                        subTaskId = CreateTask(Some(Task_CopyReceiveBuffer), 80u8);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                            .write(((subTaskId) as i16));
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((subTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(((taskId) as i16));
                        StorePtrInTaskData(
                            ((&raw mut sReceivedRecords).cast::<u8>().cast::<*mut u8>()).read(),
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((subTaskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(5))
                            .cast::<u16>(),
                        );
                        ((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>()).write(5188u32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 60i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(0i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SendPacket(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                {
                    let mut recordData: *mut u8 = (LoadPtrFromTaskData(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).cast::<u16>(),
                    ))
                    .wrapping_offset(
                        (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)
                            .wrapping_mul(200i32)) as isize
                            * 1,
                    );
                    crate::c::memcpy((&raw mut gBlockSendBuffer).cast::<u8>(), recordData, 200u32);
                    let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    SendBlockRequest(1u8);
                }
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u32)
                    == (crate::c::div_u32(
                        ((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>()).read(),
                        200u32,
                    ))
                    .wrapping_add(1u32)
                {
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                } else {
                    (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                        as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_SendPacket_SwitchToReceive));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CopyReceiveBuffer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut status: u8 = GetBlockReceivedStatus();
        let mut handledPlayers: u8 = 0u8;
        if ((status) as i32) == ((GetLinkPlayerCountAsBitFlags()) as i32) {
            let mut i: u8 = 0u8;
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::shr_i32(((status) as i32), ((i) as u32)) & 1i32) != 0 {
                            let mut dest: *mut u8 = ((LoadPtrFromTaskData(
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .cast::<u16>(),
                            ))
                            .wrapping_offset(
                                (((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(((1i32).wrapping_add(((i) as i32))) as isize))
                                .read()) as i32)
                                    .wrapping_mul(200i32)) as isize
                                    * 1,
                            ))
                            .wrapping_offset(
                                (((((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>())
                                    .read())
                                .wrapping_mul(((i) as u32)))
                                    as i32) as isize
                                    * 1,
                            );
                            let mut src: *mut u8 = GetPlayerRecvBuffer(i);
                            if (((((((((task).wrapping_add(8)).cast::<i16>())
                                .wrapping_offset(((1i32).wrapping_add(((i) as i32))) as isize))
                            .read()) as i32)
                                .wrapping_add(1i32))
                            .wrapping_mul(200i32)) as u32)
                                > ((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>()).read()
                            {
                                crate::c::memcpy(
                                    dest,
                                    src,
                                    (((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>())
                                        .read())
                                    .wrapping_sub(
                                        ((((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(
                                                ((1i32).wrapping_add(((i) as i32))) as isize,
                                            ))
                                        .read()) as i32)
                                            .wrapping_mul(200i32))
                                            as u32),
                                    ),
                                );
                            } else {
                                crate::c::memcpy(dest, src, 200u32);
                            }
                            ResetBlockReceivedFlag(i);
                            let __p1 = (((task).wrapping_add(8)).cast::<i16>())
                                .wrapping_offset(((1i32).wrapping_add(((i) as i32))) as isize);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                            if ((((((task).wrapping_add(8)).cast::<i16>())
                                .wrapping_offset(((1i32).wrapping_add(((i) as i32))) as isize))
                            .read()) as u32)
                                == (crate::c::div_u32(
                                    ((&raw mut sRecordStructSize).cast::<u8>().cast::<u32>())
                                        .read(),
                                    200u32,
                                ))
                                .wrapping_add(1u32)
                            {
                                handledPlayers = (handledPlayers).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p2 = ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((handledPlayers) as i32) == ((GetLinkPlayerCount()) as i32) {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitReceivePacket(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(4))
        .read())
            != 0)
        {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReceivePacket(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(Task_WaitReceivePacket));
        if ((((&raw mut sReadyToReceive).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            ReceiveExchangePacket(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SendPacket_SwitchToReceive(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ReceivePacket));
        ((&raw mut sReadyToReceive).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn LoadPtrFromTaskData(asShort: *mut u16) -> *mut u8 {
    unsafe {
        let mut asShort = asShort;
        return (((((asShort).read()) as i32)
            | (((((asShort).wrapping_offset(1)).read()) as i32) << 16)) as usize
            as *mut u8);
    }
}
pub(crate) unsafe extern "C" fn StorePtrInTaskData(records: *mut u8, asShort: *mut u16) {
    unsafe {
        let mut records = records;
        let mut asShort = asShort;
        (asShort).write((((records) as usize as u32) as u16));
        ((asShort).wrapping_offset(1)).write(((((records) as usize as u32) >> 16) as u16));
    }
}
pub(crate) unsafe extern "C" fn GetMultiplayerId_() -> u8 {
    unsafe {
        return GetMultiplayerId();
    }
}
pub(crate) unsafe extern "C" fn GetPlayerRecvBuffer(id: u8) -> *mut u8 {
    unsafe {
        let mut id = id;
        return ((((&raw mut gBlockRecvBuffer).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 256))
        .cast::<u16>())
        .cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn ShufflePlayerIndices(data: *mut u32) {
    unsafe {
        let mut data = data;
        let mut i: u32 = 0u32;
        let mut linkTrainerId: u32 = 0u32;
        let mut players: u32 = ((GetLinkPlayerCount()) as u32);
        'l1: {
            let __sw1 = players;
            if __sw1 == 2u32 {
                {
                    i = 0u32;
                    'l2: loop {
                        if !(i < crate::c::div_u32(2u32, 1u32)) {
                            break 'l2;
                        }
                        'l3: {
                            ((data).wrapping_offset(((i) as i32) as isize)).write(
                                ((((((&raw const sPlayerIdxOrders_2Player)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as u32),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3u32 {
                linkTrainerId =
                    crate::c::rem_u32(GetLinkPlayerTrainerId(0u8), crate::c::div_u32(6u32, 3u32));
                {
                    i = 0u32;
                    'l4: loop {
                        if !(i < crate::c::div_u32(3u32, 1u32)) {
                            break 'l4;
                        }
                        'l5: {
                            ((data).wrapping_offset(((i) as i32) as isize)).write(
                                ((((((((&raw const sPlayerIdxOrders_3Player)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((linkTrainerId) as i32) as isize * 3))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as u32),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4u32 {
                linkTrainerId =
                    crate::c::rem_u32(GetLinkPlayerTrainerId(0u8), crate::c::div_u32(36u32, 4u32));
                {
                    i = 0u32;
                    'l6: loop {
                        if !(i < crate::c::div_u32(4u32, 1u32)) {
                            break 'l6;
                        }
                        'l7: {
                            ((data).wrapping_offset(((i) as i32) as isize)).write(
                                ((((((((&raw const sPlayerIdxOrders_4Player)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((linkTrainerId) as i32) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as u32),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReceiveOldManData(
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u8,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut version: u8 = 0u8;
        let mut language: u16 = 0u16;
        let mut oldMan: *mut u8 = core::ptr::null_mut();
        let mut mixIndices = crate::ffi::Align4([0u8; 16]);
        ShufflePlayerIndices((&raw mut mixIndices).cast::<u32>());
        oldMan = (records).wrapping_offset(
            (((recordSize).wrapping_mul(
                (((&raw mut mixIndices).cast::<u32>())
                    .wrapping_offset(((multiplayerId) as i32) as isize))
                .read(),
            )) as i32) as isize
                * 1,
        );
        version = ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
            (((((&raw mut mixIndices).cast::<u32>())
                .wrapping_offset(((multiplayerId) as i32) as isize))
            .read()) as i32) as isize
                * 28,
        ))
        .cast::<u16>())
        .read()) as u8);
        language = ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
            (((((&raw mut mixIndices).cast::<u32>())
                .wrapping_offset(((multiplayerId) as i32) as isize))
            .read()) as i32) as isize
                * 28,
        ))
        .wrapping_add(26)
        .cast::<u16>())
        .read();
        if (Link_AnyPartnersPlayingRubyOrSapphire()) != 0 {
            SanitizeReceivedRubyOldMan(oldMan, ((version) as u32), ((language) as u32));
        } else {
            SanitizeReceivedEmeraldOldMan(oldMan, ((version) as u32), ((language) as u32));
        }
        crate::c::memcpy(
            ((&raw mut sOldManSave).cast::<u8>().cast::<*mut u8>()).read(),
            (records).wrapping_offset(
                (((recordSize).wrapping_mul(
                    (((&raw mut mixIndices).cast::<u32>())
                        .wrapping_offset(((multiplayerId) as i32) as isize))
                    .read(),
                )) as i32) as isize
                    * 1,
            ),
            64u32,
        );
        ResetMauvilleOldManFlag();
    }
}
pub(crate) unsafe extern "C" fn ReceiveBattleTowerData(
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u8,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut battleTowerRecord: *mut u8 = core::ptr::null_mut();
        let mut btPokemon: *mut u8 = core::ptr::null_mut();
        let mut mixIndices = crate::ffi::Align4([0u8; 16]);
        let mut i: i32 = 0i32;
        ShufflePlayerIndices((&raw mut mixIndices).cast::<u32>());
        if (Link_AnyPartnersPlayingRubyOrSapphire()) != 0 {
            if RubyBattleTowerRecordToEmerald(
                (records).wrapping_offset(
                    (((recordSize).wrapping_mul(
                        (((&raw mut mixIndices).cast::<u32>())
                            .wrapping_offset(((multiplayerId) as i32) as isize))
                        .read(),
                    )) as i32) as isize
                        * 1,
                ),
                (records).wrapping_offset(
                    (((recordSize).wrapping_mul(((multiplayerId) as u32))) as i32) as isize * 1,
                ),
            ) == 1u32
            {
                battleTowerRecord = (records).wrapping_offset(
                    (((recordSize).wrapping_mul(((multiplayerId) as u32))) as i32) as isize * 1,
                );
                ((battleTowerRecord).wrapping_add(228)).write(
                    ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut mixIndices).cast::<u32>())
                            .wrapping_offset(((multiplayerId) as i32) as isize))
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read()) as u8),
                );
                CalcEmeraldBattleTowerChecksum(battleTowerRecord);
            }
        } else {
            crate::c::memcpy(
                (records).wrapping_offset(
                    (((recordSize).wrapping_mul(((multiplayerId) as u32))) as i32) as isize * 1,
                ),
                (records).wrapping_offset(
                    (((recordSize).wrapping_mul(
                        (((&raw mut mixIndices).cast::<u32>())
                            .wrapping_offset(((multiplayerId) as i32) as isize))
                        .read(),
                    )) as i32) as isize
                        * 1,
                ),
                236u32,
            );
            battleTowerRecord = (records).wrapping_offset(
                (((recordSize).wrapping_mul(((multiplayerId) as u32))) as i32) as isize * 1,
            );
            {
                i = 0i32;
                'l1: loop {
                    if !(i
                        < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                            3i32
                        } else {
                            (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                        }))
                    {
                        break 'l1;
                    }
                    'l2: {
                        btPokemon = (((battleTowerRecord).wrapping_add(52)).cast::<u8>())
                            .wrapping_offset((i) as isize * 44);
                        if (((((btPokemon).cast::<u16>()).read()) as i32) != 0i32)
                            && ((IsStringJapanese(((btPokemon).wrapping_add(32)).cast::<u8>()))
                                != 0)
                        {
                            ConvertInternationalString(
                                ((btPokemon).wrapping_add(32)).cast::<u8>(),
                                1u8,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            CalcEmeraldBattleTowerChecksum(battleTowerRecord);
        }
        PutNewBattleTowerRecord((records).wrapping_offset(
            (((recordSize).wrapping_mul(((multiplayerId) as u32))) as i32) as isize * 1,
        ));
    }
}
pub(crate) unsafe extern "C" fn ReceiveLilycoveLadyData(
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u8,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut lilycoveLady: *mut u8 = core::ptr::null_mut();
        let mut mixIndices = crate::ffi::Align4([0u8; 16]);
        ShufflePlayerIndices((&raw mut mixIndices).cast::<u32>());
        crate::c::memcpy(
            (records).wrapping_offset(
                (((recordSize).wrapping_mul(((multiplayerId) as u32))) as i32) as isize * 1,
            ),
            ((&raw mut sLilycoveLadySave).cast::<u8>().cast::<*mut u8>()).read(),
            64u32,
        );
        if ((GetLilycoveLadyId()) as i32) == 0i32 {
            lilycoveLady = Alloc(64u32);
            if ((lilycoveLady) as usize) == 0usize {
                return;
            }
            crate::c::memcpy(
                lilycoveLady,
                ((&raw mut sLilycoveLadySave).cast::<u8>().cast::<*mut u8>()).read(),
                64u32,
            );
        } else {
            lilycoveLady = core::ptr::null_mut();
        }
        crate::c::memcpy(
            ((&raw mut sLilycoveLadySave).cast::<u8>().cast::<*mut u8>()).read(),
            (records).wrapping_offset(
                (((recordSize).wrapping_mul(
                    (((&raw mut mixIndices).cast::<u32>())
                        .wrapping_offset(((multiplayerId) as i32) as isize))
                    .read(),
                )) as i32) as isize
                    * 1,
            ),
            64u32,
        );
        ResetLilycoveLadyForRecordMix();
        if ((lilycoveLady) as usize) != 0usize {
            QuizLadyClearQuestionForRecordMix(lilycoveLady);
            Free(lilycoveLady);
        }
    }
}
pub(crate) unsafe extern "C" fn GetDaycareMailItemId(mail: *mut u8) -> u8 {
    unsafe {
        let mut mail = mail;
        return ((((mail).wrapping_add(32).cast::<u16>()).read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn SwapDaycareMail(
    records: *mut u8,
    recordSize: u32,
    idxs: *mut u8,
    playerSlot1: u8,
    playerSlot2: u8,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut idxs = idxs;
        let mut playerSlot1 = playerSlot1;
        let mut playerSlot2 = playerSlot2;
        let mut temp = crate::ffi::Align4([0u8; 56]);
        let mut mixMail1: *mut u8 = core::ptr::null_mut();
        let mut mixMail2: *mut u8 = core::ptr::null_mut();
        mixMail1 = (records).wrapping_offset(
            (((recordSize).wrapping_mul(
                (((((idxs).wrapping_offset(((playerSlot1) as i32) as isize * 2)).cast::<u8>())
                    .read()) as u32),
            )) as i32) as isize
                * 1,
        );
        crate::c::memcpy(
            (&raw mut temp).cast::<u8>(),
            ((mixMail1).cast::<u8>()).wrapping_offset(
                ((((((idxs).wrapping_offset(((playerSlot1) as i32) as isize * 2)).cast::<u8>())
                    .wrapping_offset(1))
                .read()) as i32) as isize
                    * 56,
            ),
            56u32,
        );
        mixMail2 = (records).wrapping_offset(
            (((recordSize).wrapping_mul(
                (((((idxs).wrapping_offset(((playerSlot2) as i32) as isize * 2)).cast::<u8>())
                    .read()) as u32),
            )) as i32) as isize
                * 1,
        );
        crate::c::memcpy(
            ((mixMail1).cast::<u8>()).wrapping_offset(
                ((((((idxs).wrapping_offset(((playerSlot1) as i32) as isize * 2)).cast::<u8>())
                    .wrapping_offset(1))
                .read()) as i32) as isize
                    * 56,
            ),
            ((mixMail2).cast::<u8>()).wrapping_offset(
                ((((((idxs).wrapping_offset(((playerSlot2) as i32) as isize * 2)).cast::<u8>())
                    .wrapping_offset(1))
                .read()) as i32) as isize
                    * 56,
            ),
            56u32,
        );
        crate::c::memcpy(
            ((mixMail2).cast::<u8>()).wrapping_offset(
                ((((((idxs).wrapping_offset(((playerSlot2) as i32) as isize * 2)).cast::<u8>())
                    .wrapping_offset(1))
                .read()) as i32) as isize
                    * 56,
            ),
            (&raw mut temp).cast::<u8>(),
            56u32,
        );
    }
}
pub(crate) unsafe extern "C" fn CalculateDaycareMailRandSum(src: *mut u8) {
    unsafe {
        let mut src = src;
        let mut sum: u8 = 0u8;
        let mut i: i32 = 0i32;
        sum = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 256i32) {
                    break 'l1;
                }
                'l2: {
                    sum = ((((sum) as i32)
                        .wrapping_add(((((src).wrapping_offset((i) as isize)).read()) as i32)))
                        as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sDaycareMailRandSum).cast::<u8>().cast::<u8>()).write(sum);
    }
}
pub(crate) unsafe extern "C" fn GetDaycareMailRandSum() -> u8 {
    unsafe {
        return ((&raw mut sDaycareMailRandSum).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn ReceiveDaycareMailData(
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u8,
    shows: *mut u8,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut shows = shows;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut linkPlayerCount: u8 = 0u8;
        let mut tableId: u8 = 0u8;
        let mut mixMail: *mut u8 = core::ptr::null_mut();
        let mut playerSlot1: u8 = 0u8;
        let mut playerSlot2: u8 = 0u8;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        let mut unusedArr1 = crate::ffi::Align4([0u8; 4]);
        let mut unusedArr2 = crate::ffi::Align4([0u8; 4]);
        let mut unusedMixMail = crate::ffi::Align4([0u8; 16]);
        let mut canHoldItem = crate::ffi::Align4([0u8; 8]);
        let mut idxs = crate::ffi::Align4([0u8; 8]);
        let mut numDaycareCanHold: u8 = 0u8;
        let mut oldSeed: u16 = 0u16;
        let mut anyRS: u32 = 0u32;
        oldSeed = Random2();
        SeedRng2(
            (((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as u16),
        );
        linkPlayerCount = GetLinkPlayerCount();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut unusedArr1).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                    (((&raw mut unusedArr2).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    ((((&raw mut canHoldItem).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                    .cast::<u8>())
                    .write(0u8);
                    (((((&raw mut canHoldItem).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        anyRS = Link_AnyPartnersPlayingRubyOrSapphire();
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    let mut language: u32 = 0u32;
                    let mut version: u32 = 0u32;
                    mixMail = (records).wrapping_offset(
                        ((((i) as u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                    );
                    language = ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 28))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read()) as u32);
                    version = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32) as u32);
                    {
                        j = 0u16;
                        'l5: loop {
                            if !(((j) as u32) < ((mixMail).wrapping_add(112).cast::<u32>()).read())
                            {
                                break 'l5;
                            }
                            'l6: {
                                let mut otNameLanguage: u16 = 0u16;
                                let mut nicknameLanguage: u16 = 0u16;
                                let mut daycareMail: *mut u8 = ((mixMail).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 56);
                                if ((((daycareMail).wrapping_add(32).cast::<u16>()).read()) as i32)
                                    == 0i32
                                {
                                    break 'l6;
                                }
                                if (anyRS) != 0 {
                                    if ((StringLength(
                                        ((daycareMail).wrapping_add(36)).cast::<u8>(),
                                    )) as i32)
                                        <= 5i32
                                    {
                                        otNameLanguage = 1u16;
                                    } else {
                                        StripExtCtrlCodes(
                                            ((daycareMail).wrapping_add(36)).cast::<u8>(),
                                        );
                                        otNameLanguage = ((language) as u16);
                                    }
                                    if ((((((daycareMail).wrapping_add(44)).cast::<u8>()).read())
                                        as i32)
                                        == 252i32)
                                        && (((((((daycareMail).wrapping_add(44)).cast::<u8>())
                                            .wrapping_offset(1))
                                        .read())
                                            as i32)
                                            == 21i32)
                                    {
                                        StripExtCtrlCodes(
                                            ((daycareMail).wrapping_add(44)).cast::<u8>(),
                                        );
                                        nicknameLanguage = 1u16;
                                    } else {
                                        nicknameLanguage = ((language) as u16);
                                    }
                                    if (version == 2u32) || (version == 1u32) {
                                        crate::c::bf_write(
                                            (daycareMail).wrapping_add(55),
                                            0,
                                            4,
                                            ((otNameLanguage) as u8) as i32,
                                        );
                                        crate::c::bf_write(
                                            (daycareMail).wrapping_add(55),
                                            4,
                                            4,
                                            ((nicknameLanguage) as u8) as i32,
                                        );
                                    }
                                } else {
                                    if language == 1u32 {
                                        if (IsStringJapanese(
                                            ((daycareMail).wrapping_add(36)).cast::<u8>(),
                                        )) != 0
                                        {
                                            crate::c::bf_write(
                                                (daycareMail).wrapping_add(55),
                                                0,
                                                4,
                                                (1u8) as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (daycareMail).wrapping_add(55),
                                                0,
                                                4,
                                                (2u8) as i32,
                                            );
                                        }
                                        if (IsStringJapanese(
                                            ((daycareMail).wrapping_add(44)).cast::<u8>(),
                                        )) != 0
                                        {
                                            crate::c::bf_write(
                                                (daycareMail).wrapping_add(55),
                                                4,
                                                4,
                                                (1u8) as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (daycareMail).wrapping_add(55),
                                                4,
                                                4,
                                                (2u8) as i32,
                                            );
                                        }
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        numDaycareCanHold = 0u8;
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as i32) < ((linkPlayerCount) as i32)) {
                    break 'l7;
                }
                'l8: {
                    mixMail = (records).wrapping_offset(
                        ((((i) as u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                    );
                    if ((mixMail).wrapping_add(112).cast::<u32>()).read() == 0u32 {
                        break 'l8;
                    }
                    {
                        j = 0u16;
                        'l9: loop {
                            if !(((j) as u32) < ((mixMail).wrapping_add(112).cast::<u32>()).read())
                            {
                                break 'l9;
                            }
                            'l10: {
                                if !((((((mixMail).wrapping_add(116)).cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                .read())
                                    != 0)
                                {
                                    (((((&raw mut canHoldItem).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .write(1u8);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        j = 0u16;
        {
            i = 0u16;
            'l11: loop {
                if !(((i) as i32) < ((linkPlayerCount) as i32)) {
                    break 'l11;
                }
                'l12: {
                    mixMail = (records).wrapping_offset(
                        ((((i) as u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                    );
                    if (((((((&raw mut canHoldItem).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                    .cast::<u8>())
                    .read()) as i32)
                        == 1i32)
                        || ((((((((&raw mut canHoldItem).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            == 1i32)
                    {
                        numDaycareCanHold = (numDaycareCanHold).wrapping_add(1);
                    }
                    if (((((((&raw mut canHoldItem).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                    .cast::<u8>())
                    .read()) as i32)
                        == 1i32)
                        && ((((((((&raw mut canHoldItem).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            == 0i32)
                    {
                        ((((&raw mut idxs).cast::<u8>())
                            .wrapping_offset(((j) as i32) as isize * 2))
                        .cast::<u8>())
                        .write(((i) as u8));
                        (((((&raw mut idxs).cast::<u8>())
                            .wrapping_offset(((j) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .write(0u8);
                        j = (j).wrapping_add(1);
                    } else {
                        if (((((((&raw mut canHoldItem).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32)
                            == 0i32)
                            && ((((((((&raw mut canHoldItem).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                == 1i32)
                        {
                            ((((&raw mut idxs).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 2))
                            .cast::<u8>())
                            .write(((i) as u8));
                            (((((&raw mut idxs).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .write(1u8);
                            j = (j).wrapping_add(1);
                        } else {
                            if (((((((&raw mut canHoldItem).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .read()) as i32)
                                == 1i32)
                                && ((((((((&raw mut canHoldItem).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    == 1i32)
                            {
                                let mut itemId1: u32 = 0u32;
                                let mut itemId2: u32 = 0u32;
                                ((((&raw mut idxs).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 2))
                                .cast::<u8>())
                                .write(((i) as u8));
                                itemId1 = ((GetDaycareMailItemId((mixMail).cast::<u8>())) as u32);
                                itemId2 = ((GetDaycareMailItemId(
                                    ((mixMail).cast::<u8>()).wrapping_offset(56),
                                )) as u32);
                                if ((!((itemId1) != 0)) && (!((itemId2) != 0)))
                                    || (((itemId1) != 0) && ((itemId2) != 0))
                                {
                                    (((((&raw mut idxs).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .write(((crate::c::rem_i32(((Random2()) as i32), 2i32)) as u8));
                                } else {
                                    if ((itemId1) != 0) && (!((itemId2) != 0)) {
                                        (((((&raw mut idxs).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 2))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .write(0u8);
                                    } else {
                                        if (!((itemId1) != 0)) && ((itemId2) != 0) {
                                            (((((&raw mut idxs).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize * 2))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .write(1u8);
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l13: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l13;
                }
                'l14: {
                    mixMail = (records).wrapping_offset(
                        ((((multiplayerId) as u32).wrapping_mul(recordSize)) as i32) as isize * 120,
                    );
                    (((&raw mut unusedMixMail).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(mixMail);
                }
                i = (i).wrapping_add(1);
            }
        }
        tableId = ((crate::c::rem_i32(((GetDaycareMailRandSum()) as i32), 3i32)) as u8);
        'l15: {
            let __sw1 = ((numDaycareCanHold) as i32);
            if __sw1 == 2i32 {
                SwapDaycareMail(records, recordSize, (&raw mut idxs).cast::<u8>(), 0u8, 1u8);
                break 'l15;
            }
            if __sw1 == 3i32 {
                playerSlot1 = (((((&raw const sDaycareMailSwapIds_3Player)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tableId) as i32) as isize * 2))
                .cast::<u8>())
                .read();
                playerSlot2 = ((((((&raw const sDaycareMailSwapIds_3Player)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tableId) as i32) as isize * 2))
                .cast::<u8>())
                .wrapping_offset(1))
                .read();
                SwapDaycareMail(
                    records,
                    recordSize,
                    (&raw mut idxs).cast::<u8>(),
                    playerSlot1,
                    playerSlot2,
                );
                break 'l15;
            }
            if __sw1 == 4i32 {
                ptr = (&raw mut idxs).cast::<u8>();
                playerSlot1 = (((((&raw const sDaycareMailSwapIds_4Player)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tableId) as i32) as isize * 4))
                .cast::<u8>())
                .read();
                playerSlot2 = ((((((&raw const sDaycareMailSwapIds_4Player)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tableId) as i32) as isize * 4))
                .cast::<u8>())
                .wrapping_offset(1))
                .read();
                SwapDaycareMail(records, recordSize, ptr, playerSlot1, playerSlot2);
                playerSlot1 = ((((((&raw const sDaycareMailSwapIds_4Player)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tableId) as i32) as isize * 4))
                .cast::<u8>())
                .wrapping_offset(2))
                .read();
                playerSlot2 = ((((((&raw const sDaycareMailSwapIds_4Player)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tableId) as i32) as isize * 4))
                .cast::<u8>())
                .wrapping_offset(3))
                .read();
                SwapDaycareMail(records, recordSize, ptr, playerSlot1, playerSlot2);
                break 'l15;
            }
        }
        mixMail = (records).wrapping_offset(
            ((((multiplayerId) as u32).wrapping_mul(recordSize)) as i32) as isize * 1,
        );
        crate::c::memcpy(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
                .cast::<u8>())
            .wrapping_add(80),
            (mixMail).cast::<u8>(),
            56u32,
        );
        crate::c::memcpy(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
                .cast::<u8>())
            .wrapping_offset(140))
            .wrapping_add(80),
            ((mixMail).cast::<u8>()).wrapping_offset(56),
            56u32,
        );
        SeedRng(oldSeed);
    }
}
pub(crate) unsafe extern "C" fn ReceiveGiftItem(item: *mut u16, multiplayerId: u8) {
    unsafe {
        let mut item = item;
        let mut multiplayerId = multiplayerId;
        if ((((multiplayerId) as i32) != 0i32) && ((((item).read()) as i32) != 0i32))
            && (((GetPocketByItemId((item).read())) as i32) == 5i32)
        {
            if ((!((CheckBagHasItem((item).read(), 1u16)) != 0))
                && (!((CheckPCHasItem((item).read(), 1u16)) != 0)))
                && ((AddBagItem((item).read(), 1u16)) != 0)
            {
                VarSet(16385u16, (item).read());
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                );
                if (((item).read()) as i32) == 275i32 {
                    FlagSet(2227u16);
                }
            } else {
                VarSet(16385u16, 0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoRecordMixing(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (Link_AnyPartnersPlayingRubyOrSapphire()) != 0 {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetContinueGameWarpStatusToDynamicWarp();
                WriteSaveBlock2();
                let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (WriteSaveBlock1Sector()) != 0 {
                    ClearContinueGameWarpStatus2();
                    (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 10i32
                {
                    SetCloseLinkCallback();
                    let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((Rfu_SetLinkRecovery(0u32)) != 0) {
                    CreateTask(Some(Task_LinkFullSave), 5u8);
                    let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((FuncIsActiveTask(Some(Task_LinkFullSave))) != 0) {
                    if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                        Rfu_SetLinkRecovery(1u32);
                        (((task).wrapping_add(8)).cast::<i16>()).write(8i16);
                    } else {
                        (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetLinkStandbyCallback();
                let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (IsLinkTaskFinished()) != 0 {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetSavedApprentices(dst: *mut u8, src: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut src = src;
        let mut i: i32 = 0i32;
        let mut id: i32 = 0i32;
        let mut apprenticeSaveId: i32 = 0i32;
        let mut oldPlayerApprenticeSaveId: i32 = 0i32;
        let mut numOldPlayerApprentices: i32 = 0i32;
        let mut numMixApprentices: i32 = 0i32;
        (((dst).wrapping_add(56)).cast::<u8>()).write(255u8);
        ((((dst).wrapping_offset(68)).wrapping_add(56)).cast::<u8>()).write(255u8);
        dst.cast::<crate::c::Rec4<68>>()
            .write_unaligned(src.cast::<crate::c::Rec4<68>>().read_unaligned());
        oldPlayerApprenticeSaveId = 0i32;
        numOldPlayerApprentices = 0i32;
        apprenticeSaveId = 0i32;
        numMixApprentices = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    id = (crate::c::rem_i32(
                        (i).wrapping_add(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(2),
                                3,
                                2,
                                false,
                            ) as u8) as i32),
                        ),
                        3i32,
                    ))
                    .wrapping_add(1i32);
                    if ((((((src).wrapping_offset((id) as isize * 68)).wrapping_add(56))
                        .cast::<u8>())
                    .read()) as i32)
                        != 255i32
                    {
                        if GetTrainerId(
                            (((src).wrapping_offset((id) as isize * 68)).wrapping_add(52))
                                .cast::<u8>(),
                        ) != GetTrainerId(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>(),
                        ) {
                            numMixApprentices = (numMixApprentices).wrapping_add(1);
                            apprenticeSaveId = id;
                        }
                        if GetTrainerId(
                            (((src).wrapping_offset((id) as isize * 68)).wrapping_add(52))
                                .cast::<u8>(),
                        ) == GetTrainerId(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>(),
                        ) {
                            numOldPlayerApprentices = (numOldPlayerApprentices).wrapping_add(1);
                            oldPlayerApprenticeSaveId = id;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (numMixApprentices == 0i32) && (numOldPlayerApprentices != 0i32) {
            numMixApprentices = numOldPlayerApprentices;
            apprenticeSaveId = oldPlayerApprenticeSaveId;
        }
        'l3: {
            let __sw1 = numMixApprentices;
            if __sw1 == 1i32 {
                (dst)
                    .wrapping_offset(68)
                    .cast::<crate::c::Rec4<68>>()
                    .write_unaligned(
                        (src)
                            .wrapping_offset((apprenticeSaveId) as isize * 68)
                            .cast::<crate::c::Rec4<68>>()
                            .read_unaligned(),
                    );
                break 'l3;
            }
            if __sw1 == 2i32 {
                if ((Random2()) as i32) > 13107i32 {
                    (dst)
                        .wrapping_offset(68)
                        .cast::<crate::c::Rec4<68>>()
                        .write_unaligned(
                            (src)
                                .wrapping_offset(
                                    (((crate::c::bf_read(
                                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(176))
                                        .wrapping_add(2),
                                        3,
                                        2,
                                        false,
                                    ) as u8) as i32)
                                        .wrapping_add(1i32))
                                        as isize
                                        * 68,
                                )
                                .cast::<crate::c::Rec4<68>>()
                                .read_unaligned(),
                        );
                } else {
                    (dst)
                        .wrapping_offset(68)
                        .cast::<crate::c::Rec4<68>>()
                        .write_unaligned(
                            (src)
                                .wrapping_offset(
                                    ((crate::c::rem_i32(
                                        ((crate::c::bf_read(
                                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(176))
                                            .wrapping_add(2),
                                            3,
                                            2,
                                            false,
                                        ) as u8) as i32)
                                            .wrapping_add(1i32),
                                        3i32,
                                    ))
                                    .wrapping_add(1i32))
                                        as isize
                                        * 68,
                                )
                                .cast::<crate::c::Rec4<68>>()
                                .read_unaligned(),
                        );
                }
                break 'l3;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerHallRecords(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 9i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                CopyTrainerId(
                                    (((((dst).cast::<u8>()).wrapping_offset((i) as isize * 32))
                                        .cast::<u8>())
                                    .wrapping_offset((j) as isize * 16))
                                    .cast::<u8>(),
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(10))
                                    .cast::<u8>(),
                                );
                                ((((((dst).cast::<u8>()).wrapping_offset((i) as isize * 32))
                                    .cast::<u8>())
                                .wrapping_offset((j) as isize * 16))
                                .wrapping_add(14))
                                .write(2u8);
                                StringCopy(
                                    ((((((dst).cast::<u8>()).wrapping_offset((i) as isize * 32))
                                        .cast::<u8>())
                                    .wrapping_offset((j) as isize * 16))
                                    .wrapping_add(6))
                                    .cast::<u8>(),
                                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .cast::<u8>(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0i32;
            'l5: loop {
                if !(j < 2i32) {
                    break 'l5;
                }
                'l6: {
                    (((((dst).wrapping_add(288)).cast::<u8>()).wrapping_offset((j) as isize * 28))
                        .wrapping_add(26))
                    .write(2u8);
                    CopyTrainerId(
                        ((((dst).wrapping_add(288)).cast::<u8>())
                            .wrapping_offset((j) as isize * 28))
                        .cast::<u8>(),
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                            .cast::<u8>(),
                    );
                    CopyTrainerId(
                        (((((dst).wrapping_add(288)).cast::<u8>())
                            .wrapping_offset((j) as isize * 28))
                        .wrapping_add(4))
                        .cast::<u8>(),
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2213))
                        .cast::<u8>())
                        .wrapping_offset((j) as isize * 4))
                        .cast::<u8>(),
                    );
                    StringCopy(
                        (((((dst).wrapping_add(288)).cast::<u8>())
                            .wrapping_offset((j) as isize * 28))
                        .wrapping_add(10))
                        .cast::<u8>(),
                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    );
                    StringCopy(
                        (((((dst).wrapping_add(288)).cast::<u8>())
                            .wrapping_offset((j) as isize * 28))
                        .wrapping_add(18))
                        .cast::<u8>(),
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2197))
                        .cast::<u8>())
                        .wrapping_offset((j) as isize * 8))
                        .cast::<u8>(),
                    );
                }
                j = (j).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 2i32) {
                    break 'l7;
                }
                'l8: {
                    (((((dst).cast::<u8>()).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .wrapping_add(4)
                        .cast::<u16>())
                    .write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1700))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(32)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1700))
                        .cast::<u8>())
                        .wrapping_offset(4))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(64)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1700))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(96)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1736))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(128)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1924))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(160)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1938))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(192)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1950))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(224)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1980))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((((dst).cast::<u8>()).wrapping_offset(256)).cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2002))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    (((((dst).wrapping_add(288)).cast::<u8>()).wrapping_offset((i) as isize * 28))
                        .wrapping_add(8)
                        .cast::<u16>())
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1700))
                        .cast::<u8>())
                        .wrapping_offset(12))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsApprenticeAlreadySaved(
    mixApprentice: *mut u8,
    apprentices: *mut u8,
) -> u32 {
    unsafe {
        let mut mixApprentice = mixApprentice;
        let mut apprentices = apprentices;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetTrainerId(((mixApprentice).wrapping_add(52)).cast::<u8>())
                        == GetTrainerId(
                            (((apprentices).wrapping_offset((i) as isize * 68)).wrapping_add(52))
                                .cast::<u8>(),
                        ))
                        && (((((mixApprentice).wrapping_add(2)).read()) as i32)
                            == (((((apprentices).wrapping_offset((i) as isize * 68))
                                .wrapping_add(2))
                            .read()) as i32))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ReceiveApprenticeData(
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u32,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut i: i32 = 0i32;
        let mut numApprentices: i32 = 0i32;
        let mut apprenticeId: i32 = 0i32;
        let mut mixApprentice: *mut u8 = core::ptr::null_mut();
        let mut mixIndices = crate::ffi::Align4([0u8; 16]);
        let mut apprenticeSaveId: u32 = 0u32;
        ShufflePlayerIndices((&raw mut mixIndices).cast::<u32>());
        mixApprentice = (records).wrapping_offset(
            (((recordSize).wrapping_mul(
                (((&raw mut mixIndices).cast::<u32>())
                    .wrapping_offset(((multiplayerId) as i32) as isize))
                .read(),
            )) as i32) as isize
                * 1,
        );
        numApprentices = 0i32;
        apprenticeId = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((mixApprentice).wrapping_offset((i) as isize * 68)).wrapping_add(56))
                        .cast::<u8>())
                    .read()) as i32)
                        != 255i32)
                        && (!((IsApprenticeAlreadySaved(
                            (mixApprentice).wrapping_offset((i) as isize * 68),
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>(),
                        )) != 0))
                    {
                        numApprentices = (numApprentices).wrapping_add(1);
                        apprenticeId = i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: {
            let __sw1 = numApprentices;
            if __sw1 == 1i32 {
                apprenticeSaveId = ((((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(2),
                    3,
                    2,
                    false,
                ) as u8) as i32)
                    .wrapping_add(1i32)) as u32);
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                    .cast::<u8>())
                .wrapping_offset(((apprenticeSaveId) as i32) as isize * 68)
                .cast::<crate::c::Rec4<68>>()
                .write_unaligned(
                    (mixApprentice)
                        .wrapping_offset((apprenticeId) as isize * 68)
                        .cast::<crate::c::Rec4<68>>()
                        .read_unaligned(),
                );
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(2),
                    3,
                    2,
                    ((crate::c::rem_i32(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(2),
                            3,
                            2,
                            false,
                        ) as u8) as i32)
                            .wrapping_add(1i32),
                        3i32,
                    )) as u8) as i32,
                );
                break 'l3;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 2i32) {
                            break 'l4;
                        }
                        'l5: {
                            apprenticeSaveId = (((crate::c::rem_i32(
                                (i ^ 1i32).wrapping_add(
                                    ((crate::c::bf_read(
                                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(176))
                                        .wrapping_add(2),
                                        3,
                                        2,
                                        false,
                                    ) as u8) as i32),
                                ),
                                3i32,
                            ))
                            .wrapping_add(1i32))
                                as u32);
                            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(((apprenticeSaveId) as i32) as isize * 68)
                            .cast::<crate::c::Rec4<68>>()
                            .write_unaligned(
                                (mixApprentice)
                                    .wrapping_offset((i) as isize * 68)
                                    .cast::<crate::c::Rec4<68>>()
                                    .read_unaligned(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(2),
                    3,
                    2,
                    ((crate::c::rem_i32(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(2),
                            3,
                            2,
                            false,
                        ) as u8) as i32)
                            .wrapping_add(2i32),
                        3i32,
                    )) as u8) as i32,
                );
                break 'l3;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNewHallRecords(
    dst: *mut u8,
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u32,
    linkPlayerCount: i32,
) {
    unsafe {
        let mut dst = dst;
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut linkPlayerCount = linkPlayerCount;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut l: i32 = 0i32;
        let mut repeatTrainers: i32 = 0i32;
        k = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < linkPlayerCount) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as u32) != multiplayerId {
                        ((((&raw mut sPartnerHallRecords)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ({
                                let __t1 = k;
                                k = (k).wrapping_add(1);
                                __t1
                            }) as isize,
                        ))
                        .write(records);
                    }
                    if k == 3i32 {
                        break 'l1;
                    }
                    records = (records).wrapping_offset(((recordSize) as i32) as isize * 1);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 9i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 2i32) {
                                break 'l5;
                            }
                            'l6: {
                                {
                                    k = 0i32;
                                    'l7: loop {
                                        if !(k < 3i32) {
                                            break 'l7;
                                        }
                                        'l8: {
                                            ((((((dst).cast::<u8>())
                                                .wrapping_offset((i) as isize * 192))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 96))
                                            .cast::<u8>())
                                            .wrapping_offset((k) as isize * 16)
                                            .cast::<crate::c::Rec4<16>>()
                                            .write_unaligned(
                                                (((((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(540))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 96))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 48))
                                                .cast::<u8>())
                                                .wrapping_offset((k) as isize * 16)
                                                .cast::<crate::c::Rec4<16>>()
                                                .read_unaligned(),
                                            );
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                                {
                                    k = 0i32;
                                    'l9: loop {
                                        if !(k < (linkPlayerCount).wrapping_sub(1i32)) {
                                            break 'l9;
                                        }
                                        'l10: {
                                            repeatTrainers = 0i32;
                                            {
                                                l = 0i32;
                                                'l11: loop {
                                                    if !(l < 3i32) {
                                                        break 'l11;
                                                    }
                                                    'l12: {
                                                        if GetTrainerId((((((((((dst)).cast::<u8>()).wrapping_offset((i) as isize * 192)).cast::<u8>()).wrapping_offset((j) as isize * 96)).cast::<u8>()).wrapping_offset((l) as isize * 16))).cast::<u8>()) == GetTrainerId((((((((((((&raw mut sPartnerHallRecords).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset((k) as isize)).read())).cast::<u8>()).wrapping_offset((i) as isize * 32)).cast::<u8>()).wrapping_offset((j) as isize * 16))).cast::<u8>()) {
repeatTrainers = (repeatTrainers).wrapping_add(1);
if ((((((((((((dst)).cast::<u8>()).wrapping_offset((i) as isize * 192)).cast::<u8>()).wrapping_offset((j) as isize * 96)).cast::<u8>()).wrapping_offset((l) as isize * 16)).wrapping_add(4).cast::<u16>()).read()) as i32)) < ((((((((((((((&raw mut sPartnerHallRecords).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset((k) as isize)).read())).cast::<u8>()).wrapping_offset((i) as isize * 32)).cast::<u8>()).wrapping_offset((j) as isize * 16)).wrapping_add(4).cast::<u16>()).read()) as i32)) {
(((((((dst)).cast::<u8>()).wrapping_offset((i) as isize * 192)).cast::<u8>()).wrapping_offset((j) as isize * 96)).cast::<u8>()).wrapping_offset((l) as isize * 16).cast::<crate::c::Rec4<16>>().write_unaligned((((((((((&raw mut sPartnerHallRecords).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset((k) as isize)).read())).cast::<u8>()).wrapping_offset((i) as isize * 32)).cast::<u8>()).wrapping_offset((j) as isize * 16).cast::<crate::c::Rec4<16>>().read_unaligned());
}
}
                                                    }
                                                    l = (l).wrapping_add(1);
                                                }
                                            }
                                            if repeatTrainers == 0i32 {
                                                ((((((dst).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 192))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 96))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((k).wrapping_add(3i32)) as isize * 16,
                                                )
                                                .cast::<crate::c::Rec4<16>>()
                                                .write_unaligned(
                                                    ((((((((&raw mut sPartnerHallRecords)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .cast::<*mut u8>())
                                                    .wrapping_offset((k) as isize))
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize * 32))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize * 16)
                                                    .cast::<crate::c::Rec4<16>>()
                                                    .read_unaligned(),
                                                );
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0i32;
            'l13: loop {
                if !(j < 2i32) {
                    break 'l13;
                }
                'l14: {
                    {
                        k = 0i32;
                        'l15: loop {
                            if !(k < 3i32) {
                                break 'l15;
                            }
                            'l16: {
                                (((((dst).wrapping_add(1728)).cast::<u8>())
                                    .wrapping_offset((j) as isize * 168))
                                .cast::<u8>())
                                .wrapping_offset((k) as isize * 28)
                                .cast::<crate::c::Rec4<28>>()
                                .write_unaligned(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1404))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 84))
                                    .cast::<u8>())
                                    .wrapping_offset((k) as isize * 28)
                                    .cast::<crate::c::Rec4<28>>()
                                    .read_unaligned(),
                                );
                            }
                            k = (k).wrapping_add(1);
                        }
                    }
                    {
                        k = 0i32;
                        'l17: loop {
                            if !(k < (linkPlayerCount).wrapping_sub(1i32)) {
                                break 'l17;
                            }
                            'l18: {
                                repeatTrainers = 0i32;
                                {
                                    l = 0i32;
                                    'l19: loop {
                                        if !(l < 3i32) {
                                            break 'l19;
                                        }
                                        'l20: {
                                            if (GetTrainerId(
                                                ((((((dst).wrapping_add(1728)).cast::<u8>())
                                                    .wrapping_offset((j) as isize * 168))
                                                .cast::<u8>())
                                                .wrapping_offset((l) as isize * 28))
                                                .cast::<u8>(),
                                            ) == GetTrainerId(
                                                ((((((((&raw mut sPartnerHallRecords)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .cast::<*mut u8>())
                                                .wrapping_offset((k) as isize))
                                                .read())
                                                .wrapping_add(288))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 28))
                                                .cast::<u8>(),
                                            )) && (GetTrainerId(
                                                (((((((dst).wrapping_add(1728)).cast::<u8>())
                                                    .wrapping_offset((j) as isize * 168))
                                                .cast::<u8>())
                                                .wrapping_offset((l) as isize * 28))
                                                .wrapping_add(4))
                                                .cast::<u8>(),
                                            ) == GetTrainerId(
                                                (((((((((&raw mut sPartnerHallRecords)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .cast::<*mut u8>())
                                                .wrapping_offset((k) as isize))
                                                .read())
                                                .wrapping_add(288))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 28))
                                                .wrapping_add(4))
                                                .cast::<u8>(),
                                            )) {
                                                repeatTrainers = (repeatTrainers).wrapping_add(1);
                                                if ((((((((((dst).wrapping_add(1728)).cast::<u8>()).wrapping_offset((j) as isize * 168)).cast::<u8>()).wrapping_offset((l) as isize * 28)).wrapping_add(8).cast::<u16>()).read()) as i32)) < ((((((((((((&raw mut sPartnerHallRecords).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset((k) as isize)).read()).wrapping_add(288)).cast::<u8>()).wrapping_offset((j) as isize * 28)).wrapping_add(8).cast::<u16>()).read()) as i32)) {
(((((dst).wrapping_add(1728)).cast::<u8>()).wrapping_offset((j) as isize * 168)).cast::<u8>()).wrapping_offset((l) as isize * 28).cast::<crate::c::Rec4<28>>().write_unaligned((((((((&raw mut sPartnerHallRecords).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset((k) as isize)).read()).wrapping_add(288)).cast::<u8>()).wrapping_offset((j) as isize * 28).cast::<crate::c::Rec4<28>>().read_unaligned());
}
                                            }
                                        }
                                        l = (l).wrapping_add(1);
                                    }
                                }
                                if repeatTrainers == 0i32 {
                                    (((((dst).wrapping_add(1728)).cast::<u8>())
                                        .wrapping_offset((j) as isize * 168))
                                    .cast::<u8>())
                                    .wrapping_offset(((k).wrapping_add(3i32)) as isize * 28)
                                    .cast::<crate::c::Rec4<28>>()
                                    .write_unaligned(
                                        (((((((&raw mut sPartnerHallRecords)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .cast::<*mut u8>())
                                        .wrapping_offset((k) as isize))
                                        .read())
                                        .wrapping_add(288))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 28)
                                        .cast::<crate::c::Rec4<28>>()
                                        .read_unaligned(),
                                    );
                                }
                            }
                            k = (k).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillWinStreakRecords1P(
    playerRecords: *mut u8,
    mixRecords: *mut u8,
) {
    unsafe {
        let mut playerRecords = playerRecords;
        let mut mixRecords = mixRecords;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut highestWinStreak: i32 = 0i32;
                    let mut highestId: i32 = (-1i32);
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((mixRecords).wrapping_offset((j) as isize * 16))
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                .read()) as i32)
                                    > highestWinStreak
                                {
                                    highestId = j;
                                    highestWinStreak =
                                        (((((mixRecords).wrapping_offset((j) as isize * 16))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                        .read()) as i32);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if highestId >= 0i32 {
                        (playerRecords)
                            .wrapping_offset((i) as isize * 16)
                            .cast::<crate::c::Rec4<16>>()
                            .write_unaligned(
                                (mixRecords)
                                    .wrapping_offset((highestId) as isize * 16)
                                    .cast::<crate::c::Rec4<16>>()
                                    .read_unaligned(),
                            );
                        (((mixRecords).wrapping_offset((highestId) as isize * 16))
                            .wrapping_add(4)
                            .cast::<u16>())
                        .write(0u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillWinStreakRecords2P(
    playerRecords: *mut u8,
    mixRecords: *mut u8,
) {
    unsafe {
        let mut playerRecords = playerRecords;
        let mut mixRecords = mixRecords;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut highestWinStreak: i32 = 0i32;
                    let mut highestId: i32 = (-1i32);
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((mixRecords).wrapping_offset((j) as isize * 28))
                                    .wrapping_add(8)
                                    .cast::<u16>())
                                .read()) as i32)
                                    > highestWinStreak
                                {
                                    highestId = j;
                                    highestWinStreak =
                                        (((((mixRecords).wrapping_offset((j) as isize * 28))
                                            .wrapping_add(8)
                                            .cast::<u16>())
                                        .read()) as i32);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if highestId >= 0i32 {
                        (playerRecords)
                            .wrapping_offset((i) as isize * 28)
                            .cast::<crate::c::Rec4<28>>()
                            .write_unaligned(
                                (mixRecords)
                                    .wrapping_offset((highestId) as isize * 28)
                                    .cast::<crate::c::Rec4<28>>()
                                    .read_unaligned(),
                            );
                        (((mixRecords).wrapping_offset((highestId) as isize * 28))
                            .wrapping_add(8)
                            .cast::<u16>())
                        .write(0u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveHighestWinStreakRecords(mixHallRecords: *mut u8) {
    unsafe {
        let mut mixHallRecords = mixHallRecords;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 9i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                FillWinStreakRecords1P(
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(540))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 96))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 48))
                                    .cast::<u8>(),
                                    (((((mixHallRecords).cast::<u8>())
                                        .wrapping_offset((i) as isize * 192))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 96))
                                    .cast::<u8>(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0i32;
            'l5: loop {
                if !(j < 2i32) {
                    break 'l5;
                }
                'l6: {
                    FillWinStreakRecords2P(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1404))
                        .cast::<u8>())
                        .wrapping_offset((j) as isize * 84))
                        .cast::<u8>(),
                        ((((mixHallRecords).wrapping_add(1728)).cast::<u8>())
                            .wrapping_offset((j) as isize * 168))
                        .cast::<u8>(),
                    );
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReceiveRankingHallRecords(
    records: *mut u8,
    recordSize: u32,
    multiplayerId: u32,
) {
    unsafe {
        let mut records = records;
        let mut recordSize = recordSize;
        let mut multiplayerId = multiplayerId;
        let mut linkPlayerCount: u8 = GetLinkPlayerCount();
        let mut mixHallRecords: *mut u8 = AllocZeroed(2064u32);
        GetNewHallRecords(
            mixHallRecords,
            records,
            recordSize,
            multiplayerId,
            ((linkPlayerCount) as i32),
        );
        SaveHighestWinStreakRecords(mixHallRecords);
        Free(mixHallRecords);
    }
}
pub(crate) unsafe extern "C" fn GetRecordMixingDaycareMail(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        ((&raw mut sRecordMixMail).cast::<u8>())
            .cast::<u8>()
            .cast::<crate::c::Rec4<56>>()
            .write_unaligned(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
                    .cast::<u8>())
                .wrapping_add(80)
                .cast::<crate::c::Rec4<56>>()
                .read_unaligned(),
            );
        (((&raw mut sRecordMixMail).cast::<u8>()).cast::<u8>())
            .wrapping_offset(56)
            .cast::<crate::c::Rec4<56>>()
            .write_unaligned(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
                    .cast::<u8>())
                .wrapping_offset(140))
                .wrapping_add(80)
                .cast::<crate::c::Rec4<56>>()
                .read_unaligned(),
            );
        InitDaycareMailRecordMixing(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
            (&raw mut sRecordMixMail).cast::<u8>(),
        );
        dst.cast::<crate::c::Rec4<120>>().write_unaligned(
            ((&raw mut sRecordMixMailSave).cast::<u8>().cast::<*mut u8>())
                .read()
                .cast::<crate::c::Rec4<120>>()
                .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn SanitizeDaycareMailForRuby(src: *mut u8) {
    unsafe {
        let mut src = src;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < ((src).wrapping_add(112).cast::<u32>()).read()) {
                    break 'l1;
                }
                'l2: {
                    let mut mail: *mut u8 = ((src).cast::<u8>()).wrapping_offset((i) as isize * 56);
                    if ((((mail).wrapping_add(32).cast::<u16>()).read()) as i32) != 0i32 {
                        if ((crate::c::bf_read((mail).wrapping_add(55), 0, 4, false) as u8) as i32)
                            != 1i32
                        {
                            PadNameString(((mail).wrapping_add(36)).cast::<u8>(), 252u8);
                        }
                        ConvertInternationalString(
                            ((mail).wrapping_add(44)).cast::<u8>(),
                            (crate::c::bf_read((mail).wrapping_add(55), 4, 4, false) as u8),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SanitizeRubyBattleTowerRecord(src: *mut u8) {
    unsafe {
        let mut src = src;
    }
}
pub(crate) unsafe extern "C" fn SanitizeEmeraldBattleTowerRecord(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                        3i32
                    } else {
                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                    }))
                {
                    break 'l1;
                }
                'l2: {
                    let mut towerMon: *mut u8 =
                        (((dst).wrapping_add(52)).cast::<u8>()).wrapping_offset((i) as isize * 44);
                    if ((((towerMon).cast::<u16>()).read()) as i32) != 0i32 {
                        StripExtCtrlCodes(((towerMon).wrapping_add(32)).cast::<u8>());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CalcEmeraldBattleTowerChecksum(dst);
    }
}
