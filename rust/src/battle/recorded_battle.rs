//! Translated from `src/recorded_battle.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRecordedBattleRngSeed: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlePalaceMoveSelectionRngValue: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleRecords: crate::ffi::Align4<[u8; 2656]> =
    crate::ffi::Align4([0; 2656]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerRecordSizes: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerPrevRecordSizes: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerSavedRecordSizes: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMode: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLvlMode: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierFacility: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierBrainSymbol: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCallback2_AfterRecordedBattle: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRecordedBattleMultiplayerId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierPassFlag: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleScene: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTextSpeed: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleFlags: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAI_Scripts: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedPlayerParty: crate::ffi::Align4<[u8; 600]> =
    crate::ffi::Align4([0; 600]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedOpponentParty: crate::ffi::Align4<[u8; 600]> =
    crate::ffi::Align4([0; 600]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerMonMoves: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayers: crate::ffi::Align4<[u8; 80]> = crate::ffi::Align4([0; 80]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsPlaybackFinished: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixFriendName: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixFriendClass: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sApprenticeId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEasyChatSpeech: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleOutcome: u8 = 0u8;
pub(crate) static mut sRecordMixFriendLanguage: u8 = 0u8;
pub(crate) static mut sApprenticeLanguage: u8 = 0u8;

unsafe extern "C" {
    static mut gActiveBattler: u8;
    static mut gBattleMons: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleResources: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlersCount: u8;
    static mut gBitTable: u8;
    static mut gChosenMoveByBattler: u8;
    static mut gDisableStructs: u8;
    static mut gEnemyParty: u8;
    static mut gGameLanguage: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPlayerParty: u8;
    static mut gRngValue: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_Result: u8;
    static mut gTasks: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_InitBattle();
    fn CB2_QuitRecordedBattle();
    fn CalcByteArraySum(a0: *mut u8, a1: u32) -> u32;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn Free(a0: *mut u8);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetFronterBrainSymbol() -> i32;
    fn GetLinkPlayerCount() -> u8;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn PlayMapChosenOrBattleBGM(a0: u16);
    fn ResetPaletteFadeControl();
    fn RunTasks();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
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
    unsafe {
        let mut mode = mode;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        ((&raw mut sRecordMode).cast::<u8>().cast::<u8>()).write(mode);
        ((&raw mut sIsPlaybackFinished).cast::<u8>().cast::<u8>()).write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut sBattlerPrevRecordSizes)
                        .cast::<u8>()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut sBattlerSavedRecordSizes)
                        .cast::<u8>()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                    if ((mode) as i32) == 1i32 {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 664i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset((i) as isize * 664))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .write(255u8);
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        ((&raw mut sBattleFlags).cast::<u8>().cast::<u32>())
                            .write(((&raw mut gBattleTypeFlags).cast::<u32>()).read());
                        ((&raw mut sAI_Scripts).cast::<u8>().cast::<u32>()).write(
                            ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(12)
                            .cast::<u32>())
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetTrainerInfo() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if ((((&raw mut sRecordMode).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            ((&raw mut gRecordedBattleRngSeed).cast::<u8>().cast::<u32>())
                .write(((&raw mut gRngValue).cast::<u32>()).read());
            ((&raw mut sFrontierFacility).cast::<u8>().cast::<u8>())
                .write(((VarGet(16591u16)) as u8));
            ((&raw mut sFrontierBrainSymbol).cast::<u8>().cast::<u8>())
                .write(((GetFronterBrainSymbol()) as u8));
        } else {
            if ((((&raw mut sRecordMode).cast::<u8>().cast::<u8>()).read()) as i32) == 2i32 {
                ((&raw mut gRngValue).cast::<u32>())
                    .write(((&raw mut gRecordedBattleRngSeed).cast::<u8>().cast::<u32>()).read());
            }
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            let mut linkPlayersCount: u8 = 0u8;
            let mut text = crate::ffi::Align4([0u8; 30]);
            ((&raw mut gRecordedBattleMultiplayerId)
                .cast::<u8>()
                .cast::<u8>())
            .write(GetMultiplayerId());
            linkPlayersCount = GetLinkPlayerCount();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .cast::<u32>())
                        .write(
                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read(),
                        );
                        (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .wrapping_add(12))
                        .write(
                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(19))
                            .read(),
                        );
                        (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .wrapping_add(14)
                        .cast::<u16>())
                        .write(
                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(24)
                            .cast::<u16>())
                            .read(),
                        );
                        (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .wrapping_add(16)
                        .cast::<u16>())
                        .write(
                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(26)
                            .cast::<u16>())
                            .read(),
                        );
                        if i < ((linkPlayersCount) as i32) {
                            StringCopy(
                                (&raw mut text).cast::<u8>(),
                                ((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .wrapping_add(8))
                                .cast::<u8>(),
                            );
                            StripExtCtrlCodes((&raw mut text).cast::<u8>());
                            StringCopy(
                                (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 20))
                                .wrapping_add(4))
                                .cast::<u8>(),
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            {
                                j = 0i32;
                                'l3: loop {
                                    if !(j < 8i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        (((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset((i) as isize * 20))
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(8))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read(),
                                        );
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            ((((&raw mut sPlayers).cast::<u8>()).cast::<u8>()).cast::<u32>()).write(
                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .read()) as i32)
                    | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as u32),
            );
            ((((&raw mut sPlayers).cast::<u8>()).cast::<u8>()).wrapping_add(12)).write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
            );
            ((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                .wrapping_add(16)
                .cast::<u16>())
            .write(((((&raw mut gGameLanguage).cast::<u8>()).read()) as u16));
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 8i32) {
                        break 'l5;
                    }
                    'l6: {
                        ((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>()).wrapping_add(4))
                            .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(
                            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetBattlerAction(battler: u8, action: u8) {
    unsafe {
        let mut battler = battler;
        let mut action = action;
        if (((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(((battler) as i32) as isize))
        .read()) as i32)
            < 664i32)
            && (((((&raw mut sRecordMode).cast::<u8>().cast::<u8>()).read()) as i32) != 2i32)
        {
            ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 664))
            .cast::<u8>())
            .wrapping_offset(
                (({
                    let __p1 = (((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize);
                    let __t2 = (__p1).read();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                    __t2
                }) as i32) as isize,
            ))
            .write(action);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_ClearBattlerAction(battler: u8, bytesToClear: u8) {
    unsafe {
        let mut battler = battler;
        let mut bytesToClear = bytesToClear;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((bytesToClear) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize);
                    (__p1).write(((__p1).read()).wrapping_sub(1));
                    ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 664))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize,
                    ))
                    .write(255u8);
                    if ((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_GetBattlerAction(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        if (((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(((battler) as i32) as isize))
        .read()) as i32)
            >= 664i32)
            || (((((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 664))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize,
            ))
            .read()) as i32)
                == 255i32)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                (({
                    let __v1 = 5u8;
                    ((&raw mut gBattleOutcome).cast::<u8>()).write(__v1);
                    __v1
                }) as u16),
            );
            ResetPaletteFadeControl();
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            SetMainCallback2(Some(CB2_QuitRecordedBattle));
            return 255u8;
        } else {
            return ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 664))
            .cast::<u8>())
            .wrapping_offset(
                (({
                    let __p2 = (((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize);
                    let __t3 = (__p2).read();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    __t3
                }) as i32) as isize,
            ))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetRecordedBattleMode() -> u8 {
    unsafe {
        return ((&raw mut sRecordMode).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_BufferNewBattlerData(dst: *mut u8) -> u8 {
    unsafe {
        let mut dst = dst;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut idx: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((((((&raw mut sBattlerPrevRecordSizes)
                            .cast::<u8>()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        ((dst).wrapping_offset(
                            (({
                                let __t1 = idx;
                                idx = (idx).wrapping_add(1);
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(i);
                        ((dst).wrapping_offset(
                            (({
                                let __t2 = idx;
                                idx = (idx).wrapping_add(1);
                                __t2
                            }) as i32) as isize,
                        ))
                        .write(
                            ((((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_sub(
                                    ((((((&raw mut sBattlerPrevRecordSizes)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                )) as u8),
                        );
                        {
                            j = 0u8;
                            'l3: loop {
                                if !(((j) as i32)
                                    < ((((((&raw mut sBattlerRecordSizes)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        .wrapping_sub(
                                            ((((((&raw mut sBattlerPrevRecordSizes)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32),
                                        ))
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    ((dst).wrapping_offset(
                                        (({
                                            let __t3 = idx;
                                            idx = (idx).wrapping_add(1);
                                            __t3
                                        }) as i32) as isize,
                                    ))
                                    .write(
                                        ((((((&raw mut sBattleRecords).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 664))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((((&raw mut sBattlerPrevRecordSizes)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        ((((&raw mut sBattlerPrevRecordSizes)
                            .cast::<u8>()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return idx;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_RecordAllBattlerData(src: *mut u8) {
    unsafe {
        let mut src = src;
        let mut i: i32 = 0i32;
        let mut idx: u8 = 2u8;
        let mut size: u8 = 0u8;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0) {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32)
                        != 3i32
                    {
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0) {
            {
                size = (src).read();
                'l3: loop {
                    if !(((size) as i32) != 0i32) {
                        break 'l3;
                    }
                    'l4: {
                        let mut battler: u8 =
                            GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
                        let mut numActions: u8 =
                            GetNextRecordedDataByte(src, &raw mut idx, &raw mut size);
                        {
                            i = 0i32;
                            'l5: loop {
                                if !(i < ((numActions) as i32)) {
                                    break 'l5;
                                }
                                'l6: {
                                    ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 664))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (({
                                            let __p1 = (((&raw mut sBattlerSavedRecordSizes)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(((battler) as i32) as isize);
                                            let __t2 = (__p1).read();
                                            (__p1).write(((__p1).read()).wrapping_add(1));
                                            __t2
                                        }) as i32) as isize,
                                    ))
                                    .write(
                                        GetNextRecordedDataByte(src, &raw mut idx, &raw mut size),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNextRecordedDataByte(
    data: *mut u8,
    idx: *mut u8,
    size: *mut u8,
) -> u8 {
    unsafe {
        let mut data = data;
        let mut idx = idx;
        let mut size = size;
        (size).write(((size).read()).wrapping_sub(1));
        return ((data).wrapping_offset(
            (({
                let __t1 = (idx).read();
                (idx).write(((idx).read()).wrapping_add(1));
                __t1
            }) as i32) as isize,
        ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanCopyRecordedBattleSaveData() -> u32 {
    unsafe {
        let mut dst: *mut u8 = AllocZeroed(3968u32);
        let mut ret: u32 = CopyRecordedBattleFromSave(dst);
        Free(dst);
        return ret;
    }
}
pub(crate) unsafe extern "C" fn IsRecordedBattleSaveValid(save: *mut u8) -> u32 {
    unsafe {
        let mut save = save;
        if ((save).wrapping_add(1260).cast::<u32>()).read() == 0u32 {
            return 0u32;
        }
        if (((save).wrapping_add(1260).cast::<u32>()).read() & 2097184402u32) != 0 {
            return 0u32;
        }
        if CalcByteArraySum(save, 3964u32) != ((save).wrapping_add(3964).cast::<u32>()).read() {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn RecordedBattleToSave(
    battleSave: *mut u8,
    saveSector: *mut u8,
) -> u32 {
    unsafe {
        let mut battleSave = battleSave;
        let mut saveSector = saveSector;
        crate::c::memset(saveSector, 0i32, 4096u32);
        crate::c::memcpy(saveSector, battleSave, 3968u32);
        ((saveSector).wrapping_add(3964).cast::<u32>())
            .write(CalcByteArraySum(saveSector, 3964u32));
        if TryWriteSpecialSaveSector(31u8, saveSector) != 1u32 {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRecordedBattleToSaveData() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut ret: u32 = 0u32;
        let mut battleSave: *mut u8 = core::ptr::null_mut();
        let mut savSection: *mut u8 = core::ptr::null_mut();
        let mut saveAttempts: u8 = 0u8;
        saveAttempts = 0u8;
        battleSave = AllocZeroed(3968u32);
        savSection = AllocZeroed(4096u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((battleSave).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            (((&raw mut sSavedPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                    (((battleSave).wrapping_add(600)).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            (((&raw mut sSavedOpponentParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 8i32) {
                                break 'l5;
                            }
                            'l6: {
                                ((((((battleSave).wrapping_add(1200)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    (((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset((i) as isize * 20))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    ((((battleSave).wrapping_add(1232)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .wrapping_add(12))
                        .read(),
                    );
                    ((((battleSave).wrapping_add(1252)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .wrapping_add(16)
                        .cast::<u16>())
                        .read()) as u8),
                    );
                    ((((battleSave).wrapping_add(1264)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .wrapping_add(14)
                        .cast::<u16>())
                        .read()) as u8),
                    );
                    ((((battleSave).wrapping_add(1236)).cast::<u32>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .cast::<u32>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((battleSave).wrapping_add(1256).cast::<u32>())
            .write(((&raw mut gRecordedBattleRngSeed).cast::<u8>().cast::<u32>()).read());
        if (((&raw mut sBattleFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
            ((battleSave).wrapping_add(1260).cast::<u32>()).write(
                ((((&raw mut sBattleFlags).cast::<u8>().cast::<u32>()).read() & 4294967261u32)
                    | 33554432u32),
            );
            if (((&raw mut sBattleFlags).cast::<u8>().cast::<u32>()).read() & 4u32) != 0 {
                let __p1 = (battleSave).wrapping_add(1260).cast::<u32>();
                (__p1).write(((__p1).read() | 2147483648u32));
            } else {
                if (((&raw mut sBattleFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0 {
                    'l7: {
                        let __sw2 = ((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_add(14)
                            .cast::<u16>())
                        .read()) as i32);
                        if __sw2 == 0i32 || __sw2 == 2i32 {
                            if !(((((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gRecordedBattleMultiplayerId)
                                        .cast::<u8>()
                                        .cast::<u8>())
                                    .read()) as i32) as isize
                                        * 20,
                                ))
                            .wrapping_add(14)
                            .cast::<u16>())
                            .read()) as i32)
                                & 1i32)
                                != 0)
                            {
                                let __p3 = (battleSave).wrapping_add(1260).cast::<u32>();
                                (__p3).write(((__p3).read() | 2147483648u32));
                            }
                            break 'l7;
                        }
                        if __sw2 == 1i32 || __sw2 == 3i32 {
                            if ((((((((&raw mut sPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gRecordedBattleMultiplayerId)
                                        .cast::<u8>()
                                        .cast::<u8>())
                                    .read()) as i32) as isize
                                        * 20,
                                ))
                            .wrapping_add(14)
                            .cast::<u16>())
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                let __p4 = (battleSave).wrapping_add(1260).cast::<u32>();
                                (__p4).write(((__p4).read() | 2147483648u32));
                            }
                            break 'l7;
                        }
                    }
                }
            }
        } else {
            ((battleSave).wrapping_add(1260).cast::<u32>())
                .write(((&raw mut sBattleFlags).cast::<u8>().cast::<u32>()).read());
        }
        ((battleSave).wrapping_add(1268).cast::<u16>())
            .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
        ((battleSave).wrapping_add(1270).cast::<u16>())
            .write(((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read());
        ((battleSave).wrapping_add(1272).cast::<u16>())
            .write(((&raw mut gPartnerTrainerId).cast::<u16>()).read());
        ((battleSave).wrapping_add(1274).cast::<u16>()).write(
            ((((&raw mut gRecordedBattleMultiplayerId)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as u16),
        );
        ((battleSave).wrapping_add(1276)).write(
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8),
        );
        ((battleSave).wrapping_add(1277))
            .write(((&raw mut sFrontierFacility).cast::<u8>().cast::<u8>()).read());
        ((battleSave).wrapping_add(1278))
            .write(((&raw mut sFrontierBrainSymbol).cast::<u8>().cast::<u8>()).read());
        crate::c::bf_write(
            (battleSave).wrapping_add(1279),
            0,
            1,
            ((crate::c::bf_read(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                2,
                1,
                false,
            ) as u16) as u8) as i32,
        );
        crate::c::bf_write(
            (battleSave).wrapping_add(1279),
            1,
            3,
            ((crate::c::bf_read(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                0,
                3,
                false,
            ) as u16) as u8) as i32,
        );
        ((battleSave).wrapping_add(1280).cast::<u32>())
            .write(((&raw mut sAI_Scripts).cast::<u8>().cast::<u32>()).read());
        if (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) >= 300i32)
            && (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) < 400i32)
        {
            {
                i = 0i32;
                'l8: loop {
                    if !(i < 8i32) {
                        break 'l8;
                    }
                    'l9: {
                        ((((battleSave).wrapping_add(1284)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(
                            ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(236))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                    as i32)
                                    .wrapping_sub(300i32)) as isize
                                    * 236,
                            ))
                            .wrapping_add(4))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((battleSave).wrapping_add(1292)).write(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(236))
                .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                        .wrapping_sub(300i32)) as isize
                        * 236,
                ))
                .wrapping_add(1))
                .read(),
            );
            if ((((&raw mut sBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < 6i32) {
                            break 'l10;
                        }
                        'l11: {
                            ((((battleSave).wrapping_add(1294)).cast::<u16>())
                                .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(236))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32)
                                        .wrapping_sub(300i32))
                                        as isize
                                        * 236,
                                ))
                                .wrapping_add(40))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = 0i32;
                    'l12: loop {
                        if !(i < 6i32) {
                            break 'l12;
                        }
                        'l13: {
                            ((((battleSave).wrapping_add(1294)).cast::<u16>())
                                .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(236))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32)
                                        .wrapping_sub(300i32))
                                        as isize
                                        * 236,
                                ))
                                .wrapping_add(28))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            ((battleSave).wrapping_add(1306)).write(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(236))
                .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                        .wrapping_sub(300i32)) as isize
                        * 236,
                ))
                .wrapping_add(228))
                .read(),
            );
        } else {
            if (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32) >= 300i32)
                && (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32) < 400i32)
            {
                {
                    i = 0i32;
                    'l14: loop {
                        if !(i < 8i32) {
                            break 'l14;
                        }
                        'l15: {
                            ((((battleSave).wrapping_add(1284)).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(236))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read())
                                        as i32)
                                        .wrapping_sub(300i32))
                                        as isize
                                        * 236,
                                ))
                                .wrapping_add(4))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((battleSave).wrapping_add(1292)).write(
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32)
                            .wrapping_sub(300i32)) as isize
                            * 236,
                    ))
                    .wrapping_add(1))
                    .read(),
                );
                if ((((&raw mut sBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
                    {
                        i = 0i32;
                        'l16: loop {
                            if !(i < 6i32) {
                                break 'l16;
                            }
                            'l17: {
                                ((((battleSave).wrapping_add(1294)).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>())
                                            .read())
                                            as i32)
                                            .wrapping_sub(300i32))
                                            as isize
                                            * 236,
                                    ))
                                    .wrapping_add(40))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    {
                        i = 0i32;
                        'l18: loop {
                            if !(i < 6i32) {
                                break 'l18;
                            }
                            'l19: {
                                ((((battleSave).wrapping_add(1294)).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>())
                                            .read())
                                            as i32)
                                            .wrapping_sub(300i32))
                                            as isize
                                            * 236,
                                    ))
                                    .wrapping_add(28))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                ((battleSave).wrapping_add(1306)).write(
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32)
                            .wrapping_sub(300i32)) as isize
                            * 236,
                    ))
                    .wrapping_add(228))
                    .read(),
                );
            } else {
                if (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) >= 300i32)
                    && (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) < 400i32)
                {
                    {
                        i = 0i32;
                        'l20: loop {
                            if !(i < 8i32) {
                                break 'l20;
                            }
                            'l21: {
                                ((((battleSave).wrapping_add(1284)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gPartnerTrainerId).cast::<u16>()).read())
                                            as i32)
                                            .wrapping_sub(300i32))
                                            as isize
                                            * 236,
                                    ))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((battleSave).wrapping_add(1292)).write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32)
                                .wrapping_sub(300i32)) as isize
                                * 236,
                        ))
                        .wrapping_add(1))
                        .read(),
                    );
                    ((battleSave).wrapping_add(1306)).write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32)
                                .wrapping_sub(300i32)) as isize
                                * 236,
                        ))
                        .wrapping_add(228))
                        .read(),
                    );
                }
            }
        }
        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) >= 400i32 {
            ((battleSave).wrapping_add(1293)).write(
                (crate::c::bf_read(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                            .wrapping_sub(400i32)) as isize
                            * 68,
                    ))
                    .wrapping_add(0),
                    0,
                    5,
                    false,
                ) as u8),
            );
            {
                i = 0i32;
                'l22: loop {
                    if !(i < 6i32) {
                        break 'l22;
                    }
                    'l23: {
                        ((((battleSave).wrapping_add(1294)).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .write(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                    as i32)
                                    .wrapping_sub(400i32)) as isize
                                    * 68,
                            ))
                            .wrapping_add(40))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((battleSave).wrapping_add(1307)).write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                    .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                        .wrapping_sub(400i32)) as isize
                        * 68,
                ))
                .wrapping_add(63))
                .read(),
            );
        } else {
            if ((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32) >= 400i32 {
                ((battleSave).wrapping_add(1293)).write(
                    (crate::c::bf_read(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32)
                                .wrapping_sub(400i32)) as isize
                                * 68,
                        ))
                        .wrapping_add(0),
                        0,
                        5,
                        false,
                    ) as u8),
                );
                {
                    i = 0i32;
                    'l24: loop {
                        if !(i < 6i32) {
                            break 'l24;
                        }
                        'l25: {
                            ((((battleSave).wrapping_add(1294)).cast::<u16>())
                                .wrapping_offset((i) as isize))
                            .write(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read())
                                        as i32)
                                        .wrapping_sub(400i32))
                                        as isize
                                        * 68,
                                ))
                                .wrapping_add(40))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((battleSave).wrapping_add(1307)).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read()) as i32)
                            .wrapping_sub(400i32)) as isize
                            * 68,
                    ))
                    .wrapping_add(63))
                    .read(),
                );
            } else {
                if ((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) >= 400i32 {
                    ((battleSave).wrapping_add(1293)).write(
                        (crate::c::bf_read(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32)
                                    .wrapping_sub(400i32)) as isize
                                    * 68,
                            ))
                            .wrapping_add(0),
                            0,
                            5,
                            false,
                        ) as u8),
                    );
                    ((battleSave).wrapping_add(1307)).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32)
                                .wrapping_sub(400i32)) as isize
                                * 68,
                        ))
                        .wrapping_add(63))
                        .read(),
                    );
                }
            }
        }
        {
            i = 0i32;
            'l26: loop {
                if !(i < 4i32) {
                    break 'l26;
                }
                'l27: {
                    {
                        j = 0i32;
                        'l28: loop {
                            if !(j < 664i32) {
                                break 'l28;
                            }
                            'l29: {
                                ((((((battleSave).wrapping_add(1308)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 664))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset((i) as isize * 664))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        'l30: loop {
            if !((1i32) != 0) {
                break 'l30;
            }
            ret = RecordedBattleToSave(battleSave, savSection);
            if ret == 1u32 {
                break 'l30;
            }
            saveAttempts = (saveAttempts).wrapping_add(1);
            if ((saveAttempts) as i32) >= 3i32 {
                break 'l30;
            }
        }
        Free(battleSave);
        Free(savSection);
        return ret;
    }
}
pub(crate) unsafe extern "C" fn TryCopyRecordedBattleSaveData(
    dst: *mut u8,
    saveBuffer: *mut u8,
) -> u32 {
    unsafe {
        let mut dst = dst;
        let mut saveBuffer = saveBuffer;
        if TryReadSpecialSaveSector(31u8, saveBuffer) != 1u32 {
            return 0u32;
        }
        crate::c::memcpy(dst, saveBuffer, 3968u32);
        if !((IsRecordedBattleSaveValid(dst)) != 0) {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn CopyRecordedBattleFromSave(dst: *mut u8) -> u32 {
    unsafe {
        let mut dst = dst;
        let mut savBuffer: *mut u8 = AllocZeroed(4096u32);
        let mut ret: u32 = TryCopyRecordedBattleSaveData(dst, savBuffer);
        Free(savBuffer);
        return ret;
    }
}
pub(crate) unsafe extern "C" fn CB2_RecordedBattleEnd() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            (((&raw mut sLvlMode).cast::<u8>().cast::<u8>()).read()) as i32,
        );
        ((&raw mut gBattleOutcome).cast::<u8>()).write(0u8);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
        ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).write(0u16);
        ((&raw mut gPartnerTrainerId).cast::<u16>()).write(0u16);
        RecordedBattle_RestoreSavedParties();
        SetMainCallback2(
            ((&raw mut sCallback2_AfterRecordedBattle)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StartAfterCountdown(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_RecordedBattleEnd));
            SetMainCallback2(Some(CB2_InitBattle));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SetVariablesForRecordedBattle(src: *mut u8) {
    unsafe {
        let mut src = src;
        let mut var: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        ZeroPlayerPartyMons();
        ZeroEnemyPartyMons();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((src).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            (((src).wrapping_add(600)).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        var = 0u8;
                        j = 0i32;
                        'l5: loop {
                            if !(j < 8i32) {
                                break 'l5;
                            }
                            'l6: {
                                ((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    ((((((src).wrapping_add(1200)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                                if ((((((((src).wrapping_add(1200)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    == 255i32
                                {
                                    var = 1u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28))
                        .wrapping_add(19))
                    .write(
                        ((((src).wrapping_add(1232)).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28))
                        .wrapping_add(26)
                        .cast::<u16>())
                    .write(
                        ((((((src).wrapping_add(1252)).cast::<u8>()).wrapping_offset((i) as isize))
                            .read()) as u16),
                    );
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28))
                        .wrapping_add(24)
                        .cast::<u16>())
                    .write(
                        ((((((src).wrapping_add(1264)).cast::<u8>()).wrapping_offset((i) as isize))
                            .read()) as u16),
                    );
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((src).wrapping_add(1236)).cast::<u32>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    if (var) != 0 {
                        ConvertInternationalString(
                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(8))
                            .cast::<u8>(),
                            ((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(26)
                            .cast::<u16>())
                            .read()) as u8),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gRecordedBattleRngSeed).cast::<u8>().cast::<u32>())
            .write(((src).wrapping_add(1256).cast::<u32>()).read());
        ((&raw mut gBattleTypeFlags).cast::<u32>())
            .write((((src).wrapping_add(1260).cast::<u32>()).read() | 16777216u32));
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
            .write(((src).wrapping_add(1268).cast::<u16>()).read());
        ((&raw mut gTrainerBattleOpponent_B).cast::<u16>())
            .write(((src).wrapping_add(1270).cast::<u16>()).read());
        ((&raw mut gPartnerTrainerId).cast::<u16>())
            .write(((src).wrapping_add(1272).cast::<u16>()).read());
        ((&raw mut gRecordedBattleMultiplayerId)
            .cast::<u8>()
            .cast::<u8>())
        .write(((((src).wrapping_add(1274).cast::<u16>()).read()) as u8));
        ((&raw mut sLvlMode).cast::<u8>().cast::<u8>()).write(
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8),
        );
        ((&raw mut sFrontierFacility).cast::<u8>().cast::<u8>())
            .write(((src).wrapping_add(1277)).read());
        ((&raw mut sFrontierBrainSymbol).cast::<u8>().cast::<u8>())
            .write(((src).wrapping_add(1278)).read());
        ((&raw mut sBattleScene).cast::<u8>().cast::<u8>())
            .write((crate::c::bf_read((src).wrapping_add(1279), 0, 1, false) as u8));
        ((&raw mut sTextSpeed).cast::<u8>().cast::<u8>())
            .write((crate::c::bf_read((src).wrapping_add(1279), 1, 3, false) as u8));
        ((&raw mut sAI_Scripts).cast::<u8>().cast::<u32>())
            .write(((src).wrapping_add(1280).cast::<u32>()).read());
        {
            i = 0i32;
            'l7: loop {
                if !(i < 8i32) {
                    break 'l7;
                }
                'l8: {
                    ((((&raw mut sRecordMixFriendName).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((src).wrapping_add(1284)).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sRecordMixFriendClass).cast::<u8>().cast::<u8>())
            .write(((src).wrapping_add(1292)).read());
        ((&raw mut sApprenticeId).cast::<u8>().cast::<u8>())
            .write(((src).wrapping_add(1293)).read());
        ((&raw mut sRecordMixFriendLanguage)
            .cast::<u8>()
            .cast::<u8>())
        .write(((src).wrapping_add(1306)).read());
        ((&raw mut sApprenticeLanguage).cast::<u8>().cast::<u8>())
            .write(((src).wrapping_add(1307)).read());
        {
            i = 0i32;
            'l9: loop {
                if !(i < 6i32) {
                    break 'l9;
                }
                'l10: {
                    ((((&raw mut sEasyChatSpeech).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((src).wrapping_add(1294)).cast::<u16>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            (((src).wrapping_add(1276)).read()) as i32,
        );
        {
            i = 0i32;
            'l11: loop {
                if !(i < 4i32) {
                    break 'l11;
                }
                'l12: {
                    {
                        j = 0i32;
                        'l13: loop {
                            if !(j < 664i32) {
                                break 'l13;
                            }
                            'l14: {
                                ((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 664))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    ((((((src).wrapping_add(1308)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 664))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayRecordedBattle(CB2_After: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut CB2_After = CB2_After;
        let mut battleSave: *mut u8 = AllocZeroed(3968u32);
        if CopyRecordedBattleFromSave(battleSave) == 1u32 {
            let mut taskId: u8 = 0u8;
            RecordedBattle_SaveParties();
            SetVariablesForRecordedBattle(battleSave);
            taskId = CreateTask(Some(Task_StartAfterCountdown), 1u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(128i16);
            ((&raw mut sCallback2_AfterRecordedBattle)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(CB2_After);
            PlayMapChosenOrBattleBGM(0u16);
            SetMainCallback2(Some(CB2_RecordedBattle));
        }
        Free(battleSave);
    }
}
pub(crate) unsafe extern "C" fn CB2_RecordedBattle() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTasks();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleFrontierFacility() -> u8 {
    unsafe {
        return ((&raw mut sFrontierFacility).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleFronterBrainSymbol() -> u8 {
    unsafe {
        return ((&raw mut sFrontierBrainSymbol).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SaveParties() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut sSavedPlayerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                    (((&raw mut sSavedOpponentParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecordedBattle_RestoreSavedParties() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            (((&raw mut sSavedPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            (((&raw mut sSavedOpponentParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetActiveBattlerLinkPlayerGender() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        == ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != 4i32 {
            return ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28))
                .wrapping_add(19))
            .read();
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_ClearFrontierPassFlag() {
    unsafe {
        ((&raw mut sFrontierPassFlag).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetFrontierPassFlagFromHword(flags: u16) {
    unsafe {
        let mut flags = flags;
        let __p1 = (&raw mut sFrontierPassFlag).cast::<u8>().cast::<u8>();
        (__p1).write((((((__p1).read()) as i32) | ((((flags) as i32) & 32768i32) >> 15)) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_GetFrontierPassFlag() -> u8 {
    unsafe {
        return ((&raw mut sFrontierPassFlag).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleSceneInRecordedBattle() -> u8 {
    unsafe {
        return ((&raw mut sBattleScene).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTextSpeedInRecordedBattle() -> u8 {
    unsafe {
        return ((&raw mut sTextSpeed).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_CopyBattlerMoves() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 1i32 {
            return;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
            return;
        }
        if ((((&raw mut sRecordMode).cast::<u8>().cast::<u8>()).read()) as i32) == 2i32 {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sPlayerMonMoves).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (crate::c::div_i32(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                            2i32,
                        )) as isize
                            * 8,
                    ))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(12))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_CheckMovesetChanges(mode: u8) {
    unsafe {
        let mut mode = mode;
        let mut battler: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
            return;
        }
        {
            battler = 0i32;
            'l1: loop {
                if !(battler < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((GetBattlerSide(((battler) as u8))) as i32) != 1i32 {
                        if ((mode) as i32) == 1i32 {
                            {
                                j = 0i32;
                                'l3: loop {
                                    if !(j < 4i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if ((((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset((battler) as isize * 88))
                                        .wrapping_add(12))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            != ((((((((&raw mut sPlayerMonMoves).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                (crate::c::div_i32(battler, 2i32)) as isize * 8,
                                            ))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                        {
                                            break 'l3;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            if j != 4i32 {
                                RecordedBattle_SetBattlerAction(((battler) as u8), 6u8);
                                {
                                    j = 0i32;
                                    'l5: loop {
                                        if !(j < 4i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            {
                                                k = 0i32;
                                                'l7: loop {
                                                    if !(k < 4i32) {
                                                        break 'l7;
                                                    }
                                                    'l8: {
                                                        if ((((((((((&raw mut gBattleMons)).cast::<u8>()).wrapping_offset((battler) as isize * 88)).wrapping_add(12)).cast::<u16>()).wrapping_offset((j) as isize)).read()) as i32)) == (((((((((&raw mut sPlayerMonMoves).cast::<u8>()).cast::<u8>()).wrapping_offset((crate::c::div_i32(battler, 2i32)) as isize * 8)).cast::<u16>()).wrapping_offset((k) as isize)).read()) as i32)) {
RecordedBattle_SetBattlerAction(((battler) as u8), ((k) as u8));
break 'l7;
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
                        } else {
                            if ((((((((&raw mut sBattleRecords).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((battler) as isize * 664))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sBattlerRecordSizes).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset((battler) as isize))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                == 6i32
                            {
                                let mut ppBonuses = crate::ffi::Align4([0u8; 4]);
                                let mut moveSlots = crate::ffi::Align4([0u8; 4]);
                                let mut mimickedMoveSlots = crate::ffi::Align4([0u8; 4]);
                                let mut movePP = crate::ffi::Align4([0u8; 20]);
                                let mut ppBonusSet: u8 = 0u8;
                                RecordedBattle_GetBattlerAction(((battler) as u8));
                                {
                                    j = 0i32;
                                    'l9: loop {
                                        if !(j < 4i32) {
                                            break 'l9;
                                        }
                                        'l10: {
                                            (((&raw mut ppBonuses).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .write(
                                                ((crate::c::shr_i32(
                                                    (((((((&raw mut gBattleMons).cast::<u8>())
                                                        .wrapping_offset((battler) as isize * 88))
                                                    .wrapping_add(59))
                                                    .read())
                                                        as i32)
                                                        & crate::c::shl_i32(
                                                            3i32,
                                                            ((j << 1) as u32),
                                                        )),
                                                    ((j << 1) as u32),
                                                ))
                                                    as u8),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                {
                                    j = 0i32;
                                    'l11: loop {
                                        if !(j < 4i32) {
                                            break 'l11;
                                        }
                                        'l12: {
                                            (((&raw mut moveSlots).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .write(RecordedBattle_GetBattlerAction(
                                                ((battler) as u8),
                                            ));
                                            ((((&raw mut movePP).cast::<u8>()).cast::<u16>())
                                                .wrapping_offset((j) as isize))
                                            .write(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset((battler) as isize * 88))
                                                .wrapping_add(12))
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((((&raw mut moveSlots).cast::<u8>())
                                                        .wrapping_offset((j) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read(),
                                            );
                                            (((((&raw mut movePP).cast::<u8>()).wrapping_add(8))
                                                .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .write(
                                                ((((((&raw mut gBattleMons).cast::<u8>())
                                                    .wrapping_offset((battler) as isize * 88))
                                                .wrapping_add(36))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((((&raw mut moveSlots).cast::<u8>())
                                                        .wrapping_offset((j) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read(),
                                            );
                                            (((((&raw mut movePP).cast::<u8>()).wrapping_add(12))
                                                .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .write(
                                                (((&raw mut ppBonuses).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut moveSlots).cast::<u8>())
                                                            .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                .read(),
                                            );
                                            (((&raw mut mimickedMoveSlots).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .write(
                                                ((crate::c::shr_u32(
                                                    (((crate::c::bf_read(
                                                        (((&raw mut gDisableStructs).cast::<u8>())
                                                            .wrapping_offset(
                                                                (battler) as isize * 28,
                                                            ))
                                                        .wrapping_add(24),
                                                        4,
                                                        4,
                                                        false,
                                                    )
                                                        as u8)
                                                        as u32)
                                                        & ((((&raw mut gBitTable).cast::<u32>())
                                                            .cast::<u32>())
                                                        .wrapping_offset((j) as isize))
                                                        .read()),
                                                    ((j) as u32),
                                                ))
                                                    as u8),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                {
                                    j = 0i32;
                                    'l13: loop {
                                        if !(j < 4i32) {
                                            break 'l13;
                                        }
                                        'l14: {
                                            ((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset((battler) as isize * 88))
                                            .wrapping_add(12))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .write(
                                                ((((&raw mut movePP).cast::<u8>()).cast::<u16>())
                                                    .wrapping_offset((j) as isize))
                                                .read(),
                                            );
                                            ((((((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset((battler) as isize * 88))
                                            .wrapping_add(36))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .write(
                                                (((((&raw mut movePP).cast::<u8>())
                                                    .wrapping_add(8))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                                .read(),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                ((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((battler) as isize * 88))
                                .wrapping_add(59))
                                .write(0u8);
                                crate::c::bf_write(
                                    (((&raw mut gDisableStructs).cast::<u8>())
                                        .wrapping_offset((battler) as isize * 28))
                                    .wrapping_add(24),
                                    4,
                                    4,
                                    (0u8) as i32,
                                );
                                {
                                    j = 0i32;
                                    'l15: loop {
                                        if !(j < 4i32) {
                                            break 'l15;
                                        }
                                        'l16: {
                                            let __p1 = (((&raw mut gBattleMons).cast::<u8>())
                                                .wrapping_offset((battler) as isize * 88))
                                            .wrapping_add(59);
                                            (__p1).write(
                                                (((((__p1).read()) as i32)
                                                    | crate::c::shl_i32(
                                                        (((((((&raw mut movePP).cast::<u8>())
                                                            .wrapping_add(12))
                                                        .cast::<u8>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32),
                                                        ((j << 1) as u32),
                                                    ))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (((&raw mut gDisableStructs).cast::<u8>())
                                                    .wrapping_offset((battler) as isize * 28))
                                                .wrapping_add(24),
                                                4,
                                                4,
                                                ((((crate::c::bf_read(
                                                    (((&raw mut gDisableStructs).cast::<u8>())
                                                        .wrapping_offset((battler) as isize * 28))
                                                    .wrapping_add(24),
                                                    4,
                                                    4,
                                                    false,
                                                )
                                                    as u8)
                                                    as i32)
                                                    | crate::c::shl_i32(
                                                        (((((&raw mut mimickedMoveSlots)
                                                            .cast::<u8>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32),
                                                        ((j) as u32),
                                                    ))
                                                    as u8)
                                                    as i32,
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                if !((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset((battler) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 2097152u32)
                                    != 0)
                                {
                                    {
                                        j = 0i32;
                                        'l17: loop {
                                            if !(j < 4i32) {
                                                break 'l17;
                                            }
                                            'l18: {
                                                (((&raw mut ppBonuses).cast::<u8>()).wrapping_offset((j) as isize)).write(((crate::c::shr_u32((GetMonData3((((&raw mut gPlayerParty)).cast::<u8>()).wrapping_offset((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset((battler) as isize)).read()) as i32)) as isize * 100), 21i32, core::ptr::null_mut()) & (((crate::c::shl_i32(3i32, ((((j << 1)) as u32)))) as u32))), ((((j << 1)) as u32)))) as u8));
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    {
                                        j = 0i32;
                                        'l19: loop {
                                            if !(j < 4i32) {
                                                break 'l19;
                                            }
                                            'l20: {
                                                ((((&raw mut movePP).cast::<u8>()).cast::<u16>())
                                                    .wrapping_offset((j) as isize))
                                                .write(
                                                    ((GetMonData3(
                                                        ((&raw mut gPlayerParty).cast::<u8>())
                                                            .wrapping_offset(
                                                            ((((((&raw mut gBattlerPartyIndexes)
                                                                .cast::<u16>())
                                                            .cast::<u16>())
                                                            .wrapping_offset((battler) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 100,
                                                        ),
                                                        (13i32).wrapping_add(
                                                            (((((&raw mut moveSlots).cast::<u8>())
                                                                .wrapping_offset((j) as isize))
                                                            .read())
                                                                as i32),
                                                        ),
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as u16),
                                                );
                                                (((((&raw mut movePP).cast::<u8>())
                                                    .wrapping_add(8))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                                .write(
                                                    ((GetMonData3(
                                                        ((&raw mut gPlayerParty).cast::<u8>())
                                                            .wrapping_offset(
                                                            ((((((&raw mut gBattlerPartyIndexes)
                                                                .cast::<u16>())
                                                            .cast::<u16>())
                                                            .wrapping_offset((battler) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 100,
                                                        ),
                                                        (17i32).wrapping_add(
                                                            (((((&raw mut moveSlots).cast::<u8>())
                                                                .wrapping_offset((j) as isize))
                                                            .read())
                                                                as i32),
                                                        ),
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as u8),
                                                );
                                                (((((&raw mut movePP).cast::<u8>())
                                                    .wrapping_add(12))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                                .write(
                                                    (((&raw mut ppBonuses).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((&raw mut moveSlots).cast::<u8>())
                                                                .wrapping_offset((j) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                    .read(),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    {
                                        j = 0i32;
                                        'l21: loop {
                                            if !(j < 4i32) {
                                                break 'l21;
                                            }
                                            'l22: {
                                                SetMonData(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gBattlerPartyIndexes)
                                                                .cast::<u16>())
                                                            .cast::<u16>())
                                                            .wrapping_offset((battler) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 100,
                                                        ),
                                                    (13i32).wrapping_add(j),
                                                    ((((&raw mut movePP).cast::<u8>())
                                                        .cast::<u16>())
                                                    .wrapping_offset((j) as isize))
                                                    .cast::<u8>(),
                                                );
                                                SetMonData(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gBattlerPartyIndexes)
                                                                .cast::<u16>())
                                                            .cast::<u16>())
                                                            .wrapping_offset((battler) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 100,
                                                        ),
                                                    (17i32).wrapping_add(j),
                                                    ((((&raw mut movePP).cast::<u8>())
                                                        .wrapping_add(8))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    ppBonusSet = 0u8;
                                    {
                                        j = 0i32;
                                        'l23: loop {
                                            if !(j < 4i32) {
                                                break 'l23;
                                            }
                                            'l24: {
                                                ppBonusSet = ((((ppBonusSet) as i32)
                                                    | crate::c::shl_i32(
                                                        (((((((&raw mut movePP).cast::<u8>())
                                                            .wrapping_add(12))
                                                        .cast::<u8>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32),
                                                        ((j << 1) as u32),
                                                    ))
                                                    as u8);
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    SetMonData(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset((battler) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        21i32,
                                        &raw mut ppBonusSet,
                                    );
                                }
                                ((((&raw mut gChosenMoveByBattler).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset((battler) as isize))
                                .write(
                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset((battler) as isize * 88))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(128))
                                        .cast::<u8>())
                                        .wrapping_offset((battler) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                            }
                        }
                    }
                }
                battler = (battler).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAiScriptsInRecordedBattle() -> u32 {
    unsafe {
        return ((&raw mut sAI_Scripts).cast::<u8>().cast::<u32>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SetPlaybackFinished() {
    unsafe {
        ((&raw mut sIsPlaybackFinished).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_CanStopPlayback() -> u8 {
    unsafe {
        return ((((((&raw mut sIsPlaybackFinished).cast::<u8>().cast::<u8>()).read()) as i32)
            == 0i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleRecordMixFriendName(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset((i) as isize)).write(
                        ((((&raw mut sRecordMixFriendName).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((dst).wrapping_offset(7)).write(255u8);
        ConvertInternationalString(
            dst,
            ((&raw mut sRecordMixFriendLanguage)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleRecordMixFriendClass() -> u8 {
    unsafe {
        return ((&raw mut sRecordMixFriendClass).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleApprenticeId() -> u8 {
    unsafe {
        return ((&raw mut sApprenticeId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleRecordMixFriendLanguage() -> u8 {
    unsafe {
        return ((&raw mut sRecordMixFriendLanguage)
            .cast::<u8>()
            .cast::<u8>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleApprenticeLanguage() -> u8 {
    unsafe {
        return ((&raw mut sApprenticeLanguage).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordedBattle_SaveBattleOutcome() {
    unsafe {
        ((&raw mut sBattleOutcome).cast::<u8>().cast::<u8>())
            .write(((&raw mut gBattleOutcome).cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedBattleEasyChatSpeech() -> *mut u16 {
    unsafe {
        return ((&raw mut sEasyChatSpeech).cast::<u8>().cast::<u16>()).cast::<u16>();
    }
}
