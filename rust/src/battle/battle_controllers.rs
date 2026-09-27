//! Translated from `src/battle_controllers.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkSendTaskId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkReceiveTaskId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnused: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedControllerStruct: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleBuffersTransferData: crate::ffi::Align4<[u8; 256]> =
    crate::ffi::Align4([0; 256]);

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActionSelectionCursor: u8;
    static mut gActiveBattler: u8;
    static mut gBattleBufferA: u8;
    static mut gBattleBufferB: u8;
    static mut gBattleControllerExecFlags: u8;
    static mut gBattleMainFunc: u8;
    static mut gBattleMons: u8;
    static mut gBattleMoves: u8;
    static mut gBattleOutcome: u8;
    static mut gBattlePartyCurrentOrder: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattleTextBuff3: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattleWeather: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerControllerFuncs: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerPositions: u8;
    static mut gBattlerTarget: u8;
    static mut gBattlersCount: u8;
    static mut gBitTable: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gChosenMove: u8;
    static mut gCurrentMove: u8;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: u8;
    static mut gLastUsedAbility: u8;
    static mut gLastUsedItem: u8;
    static mut gLinkBattleRecvBuffer: u8;
    static mut gLinkBattleSendBuffer: u8;
    static mut gLinkPlayers: u8;
    static mut gMoveSelectionCursor: u8;
    static mut gPlayerParty: u8;
    static mut gPotentialItemEffectBattler: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecordedBattleMultiplayerId: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gTasks: u8;
    static mut gUnusedFirstBattleVar1: u8;
    static mut gUnusedFirstBattleVar2: u8;
    static mut gWirelessCommType: u8;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BattleAI_HandleItemUseBeforeAISetup(a0: u8);
    fn BattleControllerDummy();
    fn BeginBattleIntro();
    fn BeginBattleIntroDummy();
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BufferBattlePartyCurrentOrderBySide(a0: u8, a1: u8);
    fn CheckShouldAdvanceLinkState();
    fn ClearBattleAnimationVars();
    fn ClearBattleMonForms();
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask_RfuIdle();
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCount_2() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMultiplayerId() -> u8;
    fn IsLinkMaster() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn MarkBattlerReceivedLinkData(a0: u8);
    fn OpenLink();
    fn RecordedBattle_BufferNewBattlerData(a0: *mut u8) -> u8;
    fn RecordedBattle_Init(a0: u8);
    fn RecordedBattle_SaveParties();
    fn ResetBlockReceivedFlag(a0: u8);
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetControllerToLinkOpponent();
    fn SetControllerToLinkPartner();
    fn SetControllerToOpponent();
    fn SetControllerToPlayer();
    fn SetControllerToPlayerPartner();
    fn SetControllerToRecordedOpponent();
    fn SetControllerToRecordedPlayer();
    fn SetControllerToSafari();
    fn SetControllerToWally();
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetWirelessCommType1();
    fn Task_WaitForLinkPlayerConnection(a0: u8);
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleLinkBattleSetup() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                SetWirelessCommType1();
            }
            if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                OpenLink();
            }
            CreateTask(Some(Task_WaitForLinkPlayerConnection), 0u8);
            CreateTasksForSendRecvLinkBuffers();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpBattleVarsAndBirchZigzagoon() {
    unsafe {
        let mut i: i32 = 0i32;
        ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BeginBattleIntroDummy));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset((i) as isize))
                    .write(Some(BattleControllerDummy));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(255u8);
                    (((&raw mut gActionSelectionCursor).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                    (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        HandleLinkBattleSetup();
        ((&raw mut gBattleControllerExecFlags).cast::<u32>()).write(0u32);
        ClearBattleAnimationVars();
        ClearBattleMonForms();
        ((&raw mut gActiveBattler).cast::<u8>()).write(0u8);
        BattleAI_HandleItemUseBeforeAISetup(15u8);
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16u32) != 0 {
            ZeroEnemyPartyMons();
            CreateMon(
                (&raw mut gEnemyParty).cast::<u8>(),
                288u16,
                2u8,
                32u8,
                0u8,
                0u32,
                0u8,
                0u32,
            );
            i = 0i32;
            SetMonData(
                (&raw mut gEnemyParty).cast::<u8>(),
                12i32,
                (&raw mut i).cast::<u8>(),
            );
        }
        ((&raw mut gUnusedFirstBattleVar1).cast::<u32>()).write(0u32);
        ((&raw mut gUnusedFirstBattleVar2).cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattleControllers() {
    unsafe {
        let mut i: i32 = 0i32;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0) {
            RecordedBattle_Init(1u8);
        } else {
            RecordedBattle_Init(2u8);
        }
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0) {
            RecordedBattle_SaveParties();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            InitLinkBtlControllers();
        } else {
            InitSinglePlayerBtlControllers();
        }
        SetBattlePartyIds();
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0) {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        BufferBattlePartyCurrentOrderBySide(((i) as u8), 0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(((i) as u32) < 96u32) {
                    break 'l3;
                }
                'l4: {
                    (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(420))
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(((i) as u32) < 104u32) {
                    break 'l5;
                }
                'l6: {
                    (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(516))
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitSinglePlayerBtlControllers() {
    unsafe {
        let mut i: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0 {
            ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(BeginBattleIntro));
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SetControllerToRecordedPlayer));
                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(1))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(2))
                .write(Some(SetControllerToPlayerPartner));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(3))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
            } else {
                (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SetControllerToPlayer));
                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(1))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(2))
                .write(Some(SetControllerToPlayerPartner));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(3))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
            }
            ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
            BufferBattlePartyCurrentOrderBySide(0u8, 0u8);
            BufferBattlePartyCurrentOrderBySide(1u8, 0u8);
            BufferBattlePartyCurrentOrderBySide(2u8, 1u8);
            BufferBattlePartyCurrentOrderBySide(3u8, 1u8);
            (((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).write(0u16);
            ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                .write(0u16);
            ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                .write(3u16);
            ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(3))
                .write(3u16);
        } else {
            if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
                ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(BeginBattleIntro));
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0 {
                    (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(SetControllerToSafari));
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 512u32) != 0 {
                        (((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SetControllerToWally));
                    } else {
                        (((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SetControllerToPlayer));
                    }
                }
                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(1))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                ((&raw mut gBattlersCount).cast::<u8>()).write(2u8);
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554432u32) != 0 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2147483648u32) != 0
                        {
                            ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                                .write(Some(BeginBattleIntro));
                            (((&raw mut gBattlerControllerFuncs)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(SetControllerToRecordedPlayer));
                            ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                            ((((&raw mut gBattlerControllerFuncs)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .wrapping_offset(1))
                            .write(Some(SetControllerToRecordedOpponent));
                            (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1))
                                .write(1u8);
                            ((&raw mut gBattlersCount).cast::<u8>()).write(2u8);
                        } else {
                            ((((&raw mut gBattlerControllerFuncs)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .wrapping_offset(1))
                            .write(Some(SetControllerToRecordedPlayer));
                            (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1))
                                .write(0u8);
                            (((&raw mut gBattlerControllerFuncs)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(SetControllerToRecordedOpponent));
                            ((&raw mut gBattlerPositions).cast::<u8>()).write(1u8);
                            ((&raw mut gBattlersCount).cast::<u8>()).write(2u8);
                        }
                    } else {
                        (((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SetControllerToRecordedPlayer));
                        ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(1))
                        .write(Some(SetControllerToOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                    }
                }
            } else {
                ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(BeginBattleIntro));
                (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SetControllerToPlayer));
                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(1))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(2))
                .write(Some(SetControllerToPlayer));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(3))
                .write(Some(SetControllerToOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
                ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                    if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0)
                        && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 256u32) != 0)
                    {
                        ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(BeginBattleIntro));
                        (((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SetControllerToRecordedPlayer));
                        ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(1))
                        .write(Some(SetControllerToOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(2))
                        .write(Some(SetControllerToRecordedPlayer));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(3))
                        .write(Some(SetControllerToOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
                        ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                        BufferBattlePartyCurrentOrderBySide(0u8, 0u8);
                        BufferBattlePartyCurrentOrderBySide(1u8, 0u8);
                        BufferBattlePartyCurrentOrderBySide(2u8, 1u8);
                        BufferBattlePartyCurrentOrderBySide(3u8, 1u8);
                        (((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).write(0u16);
                        ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(1))
                        .write(0u16);
                        ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(2))
                        .write(3u16);
                        ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(3))
                        .write(3u16);
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                            let mut multiplayerId: u8 = 0u8;
                            {
                                multiplayerId =
                                    ((&raw mut gRecordedBattleMultiplayerId).cast::<u8>()).read();
                                i = 0i32;
                                'l1: loop {
                                    if !(i < 4i32) {
                                        break 'l1;
                                    }
                                    'l2: {
                                        'l3: {
                                            let __sw1 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw1 == 0i32 || __sw1 == 3i32 {
                                                BufferBattlePartyCurrentOrderBySide(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as u8),
                                                    0u8,
                                                );
                                                break 'l3;
                                            }
                                            if __sw1 == 1i32 || __sw1 == 2i32 {
                                                BufferBattlePartyCurrentOrderBySide(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as u8),
                                                    1u8,
                                                );
                                                break 'l3;
                                            }
                                        }
                                        if i == ((multiplayerId) as i32) {
                                            ((((&raw mut gBattlerControllerFuncs)
                                                .cast::<Option<unsafe extern "C" fn()>>())
                                            .cast::<Option<unsafe extern "C" fn()>>())
                                            .wrapping_offset(
                                                ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 28))
                                                .wrapping_add(24)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(Some(SetControllerToRecordedPlayer));
                                            'l4: {
                                                let __sw2 = ((((((&raw mut gLinkPlayers)
                                                    .cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                                .wrapping_add(24)
                                                .cast::<u16>())
                                                .read())
                                                    as i32);
                                                if __sw2 == 0i32 || __sw2 == 3i32 {
                                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                    .write(0u8);
                                                    ((((&raw mut gBattlerPartyIndexes)
                                                        .cast::<u16>())
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gLinkPlayers)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                    .write(0u16);
                                                    break 'l4;
                                                }
                                                if __sw2 == 1i32 || __sw2 == 2i32 {
                                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                    .write(2u8);
                                                    ((((&raw mut gBattlerPartyIndexes)
                                                        .cast::<u16>())
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gLinkPlayers)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                    .write(3u16);
                                                    break 'l4;
                                                }
                                            }
                                        } else {
                                            if ((!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                & 1i32)
                                                != 0))
                                                && (!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((multiplayerId) as i32) as isize * 28,
                                                    ))
                                                .wrapping_add(24)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    & 1i32)
                                                    != 0)))
                                                || (((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 28))
                                                .wrapping_add(24)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    & 1i32)
                                                    != 0)
                                                    && ((((((((&raw mut gLinkPlayers)
                                                        .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((multiplayerId) as i32) as isize * 28,
                                                    ))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        & 1i32)
                                                        != 0))
                                            {
                                                ((((&raw mut gBattlerControllerFuncs)
                                                    .cast::<Option<unsafe extern "C" fn()>>())
                                                .cast::<Option<unsafe extern "C" fn()>>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(Some(SetControllerToRecordedPlayer));
                                                'l5: {
                                                    let __sw3 = ((((((&raw mut gLinkPlayers)
                                                        .cast::<u8>())
                                                    .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32);
                                                    if __sw3 == 0i32 || __sw3 == 3i32 {
                                                        (((&raw mut gBattlerPositions)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(0u8);
                                                        ((((&raw mut gBattlerPartyIndexes)
                                                            .cast::<u16>())
                                                        .cast::<u16>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(0u16);
                                                        break 'l5;
                                                    }
                                                    if __sw3 == 1i32 || __sw3 == 2i32 {
                                                        (((&raw mut gBattlerPositions)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(2u8);
                                                        ((((&raw mut gBattlerPartyIndexes)
                                                            .cast::<u16>())
                                                        .cast::<u16>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(3u16);
                                                        break 'l5;
                                                    }
                                                }
                                            } else {
                                                ((((&raw mut gBattlerControllerFuncs)
                                                    .cast::<Option<unsafe extern "C" fn()>>())
                                                .cast::<Option<unsafe extern "C" fn()>>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(Some(SetControllerToRecordedOpponent));
                                                'l6: {
                                                    let __sw4 = ((((((&raw mut gLinkPlayers)
                                                        .cast::<u8>())
                                                    .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32);
                                                    if __sw4 == 0i32 || __sw4 == 3i32 {
                                                        (((&raw mut gBattlerPositions)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(1u8);
                                                        ((((&raw mut gBattlerPartyIndexes)
                                                            .cast::<u16>())
                                                        .cast::<u16>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(0u16);
                                                        break 'l6;
                                                    }
                                                    if __sw4 == 1i32 || __sw4 == 2i32 {
                                                        (((&raw mut gBattlerPositions)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(3u8);
                                                        ((((&raw mut gBattlerPartyIndexes)
                                                            .cast::<u16>())
                                                        .cast::<u16>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gLinkPlayers)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize * 28))
                                                            .wrapping_add(24)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(3u16);
                                                        break 'l6;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0 {
                                (((&raw mut gBattlerControllerFuncs)
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .cast::<Option<unsafe extern "C" fn()>>())
                                .write(Some(SetControllerToRecordedPlayer));
                                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                                ((((&raw mut gBattlerControllerFuncs)
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .cast::<Option<unsafe extern "C" fn()>>())
                                .wrapping_offset(2))
                                .write(Some(SetControllerToRecordedPlayer));
                                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2))
                                    .write(2u8);
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 33554432u32)
                                    != 0
                                {
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(1))
                                    .write(Some(SetControllerToRecordedOpponent));
                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                        .wrapping_offset(1))
                                    .write(1u8);
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(3))
                                    .write(Some(SetControllerToRecordedOpponent));
                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                        .wrapping_offset(3))
                                    .write(3u8);
                                } else {
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(1))
                                    .write(Some(SetControllerToOpponent));
                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                        .wrapping_offset(1))
                                    .write(1u8);
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(3))
                                    .write(Some(SetControllerToOpponent));
                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                        .wrapping_offset(3))
                                    .write(3u8);
                                }
                            } else {
                                ((((&raw mut gBattlerControllerFuncs)
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .cast::<Option<unsafe extern "C" fn()>>())
                                .wrapping_offset(1))
                                .write(Some(SetControllerToRecordedPlayer));
                                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1))
                                    .write(0u8);
                                ((((&raw mut gBattlerControllerFuncs)
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .cast::<Option<unsafe extern "C" fn()>>())
                                .wrapping_offset(3))
                                .write(Some(SetControllerToRecordedPlayer));
                                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3))
                                    .write(2u8);
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 33554432u32)
                                    != 0
                                {
                                    (((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .write(Some(SetControllerToRecordedOpponent));
                                    ((&raw mut gBattlerPositions).cast::<u8>()).write(1u8);
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(2))
                                    .write(Some(SetControllerToRecordedOpponent));
                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                        .wrapping_offset(2))
                                    .write(3u8);
                                } else {
                                    (((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .write(Some(SetControllerToOpponent));
                                    ((&raw mut gBattlerPositions).cast::<u8>()).write(1u8);
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(2))
                                    .write(Some(SetControllerToOpponent));
                                    (((&raw mut gBattlerPositions).cast::<u8>())
                                        .wrapping_offset(2))
                                    .write(3u8);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitLinkBtlControllers() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut multiplayerId: u8 = 0u8;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0 {
                ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(BeginBattleIntro));
                (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SetControllerToPlayer));
                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(1))
                .write(Some(SetControllerToLinkOpponent));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                ((&raw mut gBattlersCount).cast::<u8>()).write(2u8);
            } else {
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(1))
                .write(Some(SetControllerToPlayer));
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(0u8);
                (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SetControllerToLinkOpponent));
                ((&raw mut gBattlerPositions).cast::<u8>()).write(1u8);
                ((&raw mut gBattlersCount).cast::<u8>()).write(2u8);
            }
        } else {
            if (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0))
                && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
            {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0 {
                    ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(BeginBattleIntro));
                    (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(SetControllerToPlayer));
                    ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(1))
                    .write(Some(SetControllerToLinkOpponent));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(2))
                    .write(Some(SetControllerToPlayer));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(3))
                    .write(Some(SetControllerToLinkOpponent));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
                    ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                } else {
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(1))
                    .write(Some(SetControllerToPlayer));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(0u8);
                    (((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(SetControllerToLinkOpponent));
                    ((&raw mut gBattlerPositions).cast::<u8>()).write(1u8);
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(3))
                    .write(Some(SetControllerToPlayer));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(2u8);
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(2))
                    .write(Some(SetControllerToLinkOpponent));
                    (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(3u8);
                    ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                }
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 256u32) != 0 {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0 {
                        ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(BeginBattleIntro));
                        (((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SetControllerToPlayer));
                        ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(1))
                        .write(Some(SetControllerToOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(2))
                        .write(Some(SetControllerToLinkPartner));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(3))
                        .write(Some(SetControllerToOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
                        ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                    } else {
                        (((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SetControllerToLinkPartner));
                        ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(1))
                        .write(Some(SetControllerToLinkOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(2))
                        .write(Some(SetControllerToPlayer));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(2u8);
                        ((((&raw mut gBattlerControllerFuncs)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .wrapping_offset(3))
                        .write(Some(SetControllerToLinkOpponent));
                        (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(3u8);
                        ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                    }
                    BufferBattlePartyCurrentOrderBySide(0u8, 0u8);
                    BufferBattlePartyCurrentOrderBySide(1u8, 0u8);
                    BufferBattlePartyCurrentOrderBySide(2u8, 1u8);
                    BufferBattlePartyCurrentOrderBySide(3u8, 1u8);
                    (((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).write(0u16);
                    ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(1))
                    .write(0u16);
                    ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(2))
                    .write(3u16);
                    ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(3))
                    .write(3u16);
                } else {
                    multiplayerId = GetMultiplayerId();
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0 {
                        ((&raw mut gBattleMainFunc).cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(BeginBattleIntro));
                    }
                    {
                        i = 0i32;
                        'l1: loop {
                            if !(i < 4i32) {
                                break 'l1;
                            }
                            'l2: {
                                'l3: {
                                    let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read())
                                        as i32);
                                    if __sw1 == 0i32 || __sw1 == 3i32 {
                                        BufferBattlePartyCurrentOrderBySide(
                                            ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as u8),
                                            0u8,
                                        );
                                        break 'l3;
                                    }
                                    if __sw1 == 1i32 || __sw1 == 2i32 {
                                        BufferBattlePartyCurrentOrderBySide(
                                            ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as u8),
                                            1u8,
                                        );
                                        break 'l3;
                                    }
                                }
                                if i == ((multiplayerId) as i32) {
                                    ((((&raw mut gBattlerControllerFuncs)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(
                                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .write(Some(SetControllerToPlayer));
                                    'l4: {
                                        let __sw2 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        if __sw2 == 0i32 || __sw2 == 3i32 {
                                            (((&raw mut gBattlerPositions).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                            .write(0u8);
                                            ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(
                                                ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 28))
                                                .wrapping_add(24)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(0u16);
                                            break 'l4;
                                        }
                                        if __sw2 == 1i32 || __sw2 == 2i32 {
                                            (((&raw mut gBattlerPositions).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                            .write(2u8);
                                            ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(
                                                ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 28))
                                                .wrapping_add(24)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(3u16);
                                            break 'l4;
                                        }
                                    }
                                } else {
                                    if ((!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read())
                                        as i32)
                                        & 1i32)
                                        != 0))
                                        && (!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((multiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)))
                                        || (((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)
                                            && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset(
                                                    ((multiplayerId) as i32) as isize * 28,
                                                ))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                & 1i32)
                                                != 0))
                                    {
                                        ((((&raw mut gBattlerControllerFuncs)
                                            .cast::<Option<unsafe extern "C" fn()>>())
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                        .wrapping_offset(
                                            ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(Some(SetControllerToLinkPartner));
                                        'l5: {
                                            let __sw3 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw3 == 0i32 || __sw3 == 3i32 {
                                                (((&raw mut gBattlerPositions).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gLinkPlayers)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                .write(0u8);
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(0u16);
                                                break 'l5;
                                            }
                                            if __sw3 == 1i32 || __sw3 == 2i32 {
                                                (((&raw mut gBattlerPositions).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gLinkPlayers)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                .write(2u8);
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(3u16);
                                                break 'l5;
                                            }
                                        }
                                    } else {
                                        ((((&raw mut gBattlerControllerFuncs)
                                            .cast::<Option<unsafe extern "C" fn()>>())
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                        .wrapping_offset(
                                            ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(Some(SetControllerToLinkOpponent));
                                        'l6: {
                                            let __sw4 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw4 == 0i32 || __sw4 == 3i32 {
                                                (((&raw mut gBattlerPositions).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gLinkPlayers)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                .write(1u8);
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(0u16);
                                                break 'l6;
                                            }
                                            if __sw4 == 1i32 || __sw4 == 2i32 {
                                                (((&raw mut gBattlerPositions).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gLinkPlayers)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                .write(3u8);
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                                        .wrapping_offset((i) as isize * 28))
                                                    .wrapping_add(24)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(3u16);
                                                break 'l6;
                                            }
                                        }
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((&raw mut gBattlersCount).cast::<u8>()).write(4u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBattlePartyIds() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0) {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 6i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if i < 2i32 {
                                        if ((((((&raw mut gBattlerPositions).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read())
                                            as i32)
                                            & 1i32)
                                            == 0i32
                                        {
                                            if (((GetMonData2(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset((j) as isize * 100),
                                                57i32,
                                            ) != 0u32)
                                                && (GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 0u32))
                                                && (GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 412u32))
                                                && (!((GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    45i32,
                                                )) != 0))
                                            {
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .write(((j) as u16));
                                                break 'l3;
                                            }
                                        } else {
                                            if (((GetMonData2(
                                                ((&raw mut gEnemyParty).cast::<u8>())
                                                    .wrapping_offset((j) as isize * 100),
                                                57i32,
                                            ) != 0u32)
                                                && (GetMonData2(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 0u32))
                                                && (GetMonData2(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 412u32))
                                                && (!((GetMonData2(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    45i32,
                                                )) != 0))
                                            {
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .write(((j) as u16));
                                                break 'l3;
                                            }
                                        }
                                    } else {
                                        if ((((((&raw mut gBattlerPositions).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read())
                                            as i32)
                                            & 1i32)
                                            == 0i32
                                        {
                                            if ((((GetMonData2(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset((j) as isize * 100),
                                                57i32,
                                            ) != 0u32)
                                                && (GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    11i32,
                                                ) != 0u32))
                                                && (GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 412u32))
                                                && (!((GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    45i32,
                                                )) != 0)))
                                                && (((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((i).wrapping_sub(2i32)) as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    != j)
                                            {
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .write(((j) as u16));
                                                break 'l3;
                                            }
                                        } else {
                                            if ((((GetMonData2(
                                                ((&raw mut gEnemyParty).cast::<u8>())
                                                    .wrapping_offset((j) as isize * 100),
                                                57i32,
                                            ) != 0u32)
                                                && (GetMonData2(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 0u32))
                                                && (GetMonData2(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    65i32,
                                                ) != 412u32))
                                                && (!((GetMonData2(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset((j) as isize * 100),
                                                    45i32,
                                                )) != 0)))
                                                && (((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((i).wrapping_sub(2i32)) as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    != j)
                                            {
                                                ((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .write(((j) as u16));
                                                break 'l3;
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
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0 {
                ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(0u16);
                ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(3))
                .write(3u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrepareBufferDataTransfer(bufferId: u8, data: *mut u8, size: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut data = data;
        let mut size = size;
        let mut i: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            PrepareBufferDataTransferLink(bufferId, size, data);
        } else {
            'l1: {
                let __sw1 = ((bufferId) as i32);
                if __sw1 == 0i32 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < ((size) as i32)) {
                                break 'l2;
                            }
                            'l3: {
                                (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 512,
                                ))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write((data).read());
                            }
                            data = (data).wrapping_offset(1);
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < ((size) as i32)) {
                                break 'l4;
                            }
                            'l5: {
                                (((((&raw mut gBattleBufferB).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 512,
                                ))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write((data).read());
                            }
                            data = (data).wrapping_offset(1);
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTasksForSendRecvLinkBuffers() {
    unsafe {
        ((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>())
            .write(CreateTask(Some(Task_HandleSendLinkBuffersData), 0u8));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        ((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>()).write(CreateTask(
            Some(Task_HandleCopyReceivedLinkBuffersData),
            0u8,
        ));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        ((&raw mut sUnused).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrepareBufferDataTransferLink(bufferId: u8, size: u16, data: *mut u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut size = size;
        let mut data = data;
        let mut alignedSize: i32 = 0i32;
        let mut i: i32 = 0i32;
        alignedSize = (((size) as i32).wrapping_sub(crate::c::rem_i32(((size) as i32), 4i32)))
            .wrapping_add(4i32);
        if ((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .read()) as i32)
            .wrapping_add(alignedSize))
        .wrapping_add(8i32))
        .wrapping_add(1i32)
            > 4096i32
        {
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .write(
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(14))
                .read(),
            );
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .write(0i16);
        }
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(0i32)) as isize,
        ))
        .write(bufferId);
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(1i32)) as isize,
        ))
        .write(((&raw mut gActiveBattler).cast::<u8>()).read());
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(2i32)) as isize,
        ))
        .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(3i32)) as isize,
        ))
        .write(((&raw mut gBattlerTarget).cast::<u8>()).read());
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(4i32)) as isize,
        ))
        .write(((alignedSize) as u8));
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(5i32)) as isize,
        ))
        .write((((alignedSize & 65280i32) >> 8) as u8));
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(6i32)) as isize,
        ))
        .write(((&raw mut gAbsentBattlerFlags).cast::<u8>()).read());
        ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read()).wrapping_offset(
            (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(7i32)) as isize,
        ))
        .write(((&raw mut gEffectBattler).cast::<u8>()).read());
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((size) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read())
                        .wrapping_offset(
                            ((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 40,
                            ))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(14))
                            .read()) as i32)
                                .wrapping_add(8i32))
                            .wrapping_add(i)) as isize,
                        ))
                    .write(((data).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(
            (((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sLinkSendTaskId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
                .wrapping_add(alignedSize))
            .wrapping_add(8i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSendLinkBuffersData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut numPlayers: u16 = 0u16;
        let mut blockSize: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(100i16);
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p3).write(((__p3).read()).wrapping_sub(1));
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    == 0i32
                {
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 256u32) != 0 {
                        numPlayers = 2u16;
                    } else {
                        numPlayers = ((if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                            & 64u32)
                            != 0
                        {
                            4i32
                        } else {
                            2i32
                        }) as u16);
                    }
                    if ((GetLinkPlayerCount_2()) as i32) >= ((numPlayers) as i32) {
                        if (IsLinkMaster()) != 0 {
                            CheckShouldAdvanceLinkState();
                            let __p6 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11);
                            (__p6).write(((__p6).read()).wrapping_add(1));
                        } else {
                            let __p7 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11);
                            (__p7).write(((__p7).read()).wrapping_add(1));
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32)
                    != ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(14))
                    .read()) as i32)
                {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .read()) as i32)
                        == 0i32
                    {
                        if (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .read()) as i32)
                            > ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(14))
                            .read()) as i32))
                            && (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .read()) as i32)
                                == ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(12))
                                .read()) as i32))
                        {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .write(0i16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .write(0i16);
                        }
                        blockSize = (((((((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>())
                            .read())
                        .wrapping_offset(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .read()) as i32)
                                .wrapping_add(4i32)) as isize,
                        ))
                        .read()) as i32)
                            | (((((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(15))
                                    .read()) as i32)
                                        .wrapping_add(5i32))
                                        as isize,
                                ))
                            .read()) as i32)
                                << 8))
                            .wrapping_add(8i32)) as u16);
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            (((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(15))
                                    .read()) as i32)
                                        .wrapping_add(0i32))
                                        as isize,
                                ),
                            blockSize,
                        );
                        let __p8 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11);
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    } else {
                        let __p9 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(13);
                        (__p9).write(((__p9).read()).wrapping_sub(1));
                        break 'l1;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (IsLinkTaskFinished()) != 0 {
                    blockSize = ((((((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read())
                        .wrapping_offset(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .read()) as i32)
                                .wrapping_add(4i32)) as isize,
                        ))
                    .read()) as i32)
                        | (((((((&raw mut gLinkBattleSendBuffer).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(15))
                                .read()) as i32)
                                    .wrapping_add(5i32)) as isize,
                            ))
                        .read()) as i32)
                            << 8)) as u16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .write(
                        (((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .read()) as i32)
                            .wrapping_add(((blockSize) as i32)))
                        .wrapping_add(8i32)) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (({
                    let __p10 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13);
                    let __t11 = ((__p10).read()).wrapping_sub(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(3i16);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryReceiveLinkBattleData() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: i32 = 0i32;
        let mut recvBuffer: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32u32) != 0)
        {
            DestroyTask_RfuIdle();
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((GetBlockReceivedStatus()) as u32)
                            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            != 0
                        {
                            ResetBlockReceivedFlag(i);
                            recvBuffer = ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>();
                            {
                                let mut dest: *mut u8 = core::ptr::null_mut();
                                let mut src: *mut u8 = core::ptr::null_mut();
                                let mut dataSize: u16 = (((((&raw mut gBlockRecvBuffer)
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 256))
                                .cast::<u16>())
                                .wrapping_offset(2))
                                .read();
                                if (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>())
                                        .read()) as i32)
                                        as isize
                                        * 40,
                                ))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(14))
                                .read()) as i32)
                                    .wrapping_add(9i32))
                                .wrapping_add(((dataSize) as i32))
                                    > 4096i32
                                {
                                    ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(12))
                                    .write(
                                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                            ((((&raw mut sLinkReceiveTaskId)
                                                .cast::<u8>()
                                                .cast::<u8>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 40,
                                        ))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(14))
                                        .read(),
                                    );
                                    ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(14))
                                    .write(0i16);
                                }
                                dest = (((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>())
                                    .read())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(14))
                                    .read()) as i32) as isize,
                                );
                                src = recvBuffer;
                                {
                                    j = 0i32;
                                    'l3: loop {
                                        if !(j < ((dataSize) as i32).wrapping_add(8i32)) {
                                            break 'l3;
                                        }
                                        'l4: {
                                            ((dest).wrapping_offset((j) as isize)).write(
                                                ((src).wrapping_offset((j) as isize)).read(),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>())
                                        .read()) as i32)
                                        as isize
                                        * 40,
                                ))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(14))
                                .write(
                                    (((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut sLinkReceiveTaskId).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(14))
                                    .read()) as i32)
                                        .wrapping_add(((dataSize) as i32)))
                                    .wrapping_add(8i32))
                                        as i16),
                                );
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCopyReceivedLinkBuffersData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut blockSize: u16 = 0u16;
        let mut battler: u8 = 0u8;
        let mut playerId: u8 = 0u8;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            != ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read()) as i32)
        {
            if (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                > ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(14))
                .read()) as i32))
                && (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32)
                    == ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32))
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .write(0i16);
            }
            battler = ((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                .wrapping_offset(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32)
                        .wrapping_add(1i32)) as isize,
                ))
            .read();
            blockSize = ((((((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                .wrapping_offset(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32)
                        .wrapping_add(4i32)) as isize,
                ))
            .read()) as i32)
                | (((((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .read()) as i32)
                            .wrapping_add(5i32)) as isize,
                    ))
                .read()) as i32)
                    << 8)) as u16);
            'l1: {
                let __sw1 = ((((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .read()) as i32)
                            .wrapping_add(0i32)) as isize,
                    ))
                .read()) as i32);
                if __sw1 == 0i32 {
                    if (((&raw mut gBattleControllerExecFlags).cast::<u32>()).read()
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read())
                        != 0
                    {
                        return;
                    }
                    crate::c::memcpy(
                        (((&raw mut gBattleBufferA).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 512))
                        .cast::<u8>(),
                        (((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(15))
                                .read()) as i32)
                                    .wrapping_add(8i32)) as isize,
                            ),
                        ((blockSize) as u32),
                    );
                    MarkBattlerReceivedLinkData(battler);
                    if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0) {
                        ((&raw mut gBattlerAttacker).cast::<u8>()).write(
                            ((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(15))
                                    .read()) as i32)
                                        .wrapping_add(2i32))
                                        as isize,
                                ))
                            .read(),
                        );
                        ((&raw mut gBattlerTarget).cast::<u8>()).write(
                            ((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(15))
                                    .read()) as i32)
                                        .wrapping_add(3i32))
                                        as isize,
                                ))
                            .read(),
                        );
                        ((&raw mut gAbsentBattlerFlags).cast::<u8>()).write(
                            ((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(15))
                                    .read()) as i32)
                                        .wrapping_add(6i32))
                                        as isize,
                                ))
                            .read(),
                        );
                        ((&raw mut gEffectBattler).cast::<u8>()).write(
                            ((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(15))
                                    .read()) as i32)
                                        .wrapping_add(7i32))
                                        as isize,
                                ))
                            .read(),
                        );
                    }
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    crate::c::memcpy(
                        (((&raw mut gBattleBufferB).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize * 512))
                        .cast::<u8>(),
                        (((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(15))
                                .read()) as i32)
                                    .wrapping_add(8i32)) as isize,
                            ),
                        ((blockSize) as u32),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    playerId = ((((&raw mut gLinkBattleRecvBuffer).cast::<*mut u8>()).read())
                        .wrapping_offset(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .read()) as i32)
                                .wrapping_add(8i32)) as isize,
                        ))
                    .read();
                    let __p2 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
                    (__p2).write(
                        ((__p2).read()
                            & !(crate::c::shl_u32(
                                ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                .read(),
                                ((((playerId) as i32).wrapping_mul(4i32)) as u32),
                            ))),
                    );
                    break 'l1;
                }
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write(
                (((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32)
                    .wrapping_add(((blockSize) as i32)))
                .wrapping_add(8i32)) as i16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitGetMonData(bufferId: u8, requestId: u8, monToCheck: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut requestId = requestId;
        let mut monToCheck = monToCheck;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(0u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(requestId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(monToCheck);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitGetRawMonData(
    bufferId: u8,
    monId: u8,
    bytes: u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut monId = monId;
        let mut bytes = bytes;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(1u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(monId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(bytes);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitSetMonData(
    bufferId: u8,
    requestId: u8,
    monToCheck: u8,
    bytes: u8,
    data: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut requestId = requestId;
        let mut monToCheck = monToCheck;
        let mut bytes = bytes;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(2u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(requestId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(monToCheck);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((bytes) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            (((3i32).wrapping_add(((bytes) as i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitSetRawMonData(
    bufferId: u8,
    monId: u8,
    bytes: u8,
    data: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut monId = monId;
        let mut bytes = bytes;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(3u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(monId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(bytes);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((bytes) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((bytes) as i32).wrapping_add(3i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitLoadMonSprite(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(4u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(4u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(4u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(4u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitSwitchInAnim(
    bufferId: u8,
    partyId: u8,
    dontClearSubstituteBit: u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut partyId = partyId;
        let mut dontClearSubstituteBit = dontClearSubstituteBit;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(5u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(partyId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(dontClearSubstituteBit);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(5u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitReturnMonToBall(bufferId: u8, skipAnim: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut skipAnim = skipAnim;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(6u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(skipAnim);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            2u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitDrawTrainerPic(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(7u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(7u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(7u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(7u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitTrainerSlide(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(8u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(8u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(8u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(8u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitTrainerSlideBack(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(9u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(9u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(9u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(9u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitFaintAnimation(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(10u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(10u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(10u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(10u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitPaletteFade(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(11u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(11u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(11u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(11u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitSuccessBallThrowAnim(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(12u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(12u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(12u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(12u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitBallThrowAnim(bufferId: u8, caseId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut caseId = caseId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(13u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(caseId);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            2u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitPause(bufferId: u8, toWait: u8, data: *mut u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut toWait = toWait;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(14u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(toWait);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((toWait) as i32).wrapping_mul(3i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((2i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            (((((toWait) as i32).wrapping_mul(3i32)).wrapping_add(2i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitMoveAnimation(
    bufferId: u8,
    r#move: u16,
    turnOfMove: u8,
    movePower: u16,
    dmg: i32,
    friendship: u8,
    disableStructPtr: *mut u8,
    multihit: u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut r#move = r#move;
        let mut turnOfMove = turnOfMove;
        let mut movePower = movePower;
        let mut dmg = dmg;
        let mut friendship = friendship;
        let mut disableStructPtr = disableStructPtr;
        let mut multihit = multihit;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(15u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((r#move) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((r#move) as i32) & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(turnOfMove);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .write(((movePower) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
            .write((((((movePower) as i32) & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(6))
            .write(((dmg) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(7))
            .write((((dmg & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(8))
            .write((((dmg & 16711680i32) >> 16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(9))
            .write((((((dmg) as u32) & 4278190080u32) >> 24) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(10))
            .write(friendship);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(11))
            .write(multihit);
        if (!((AbilityBattleEffects(14u8, 0u8, 13u8, 0u8, 0u16)) != 0))
            && (!((AbilityBattleEffects(14u8, 0u8, 77u8, 0u8, 0u16)) != 0))
        {
            ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(12))
            .write(((((&raw mut gBattleWeather).cast::<u16>()).read()) as u8));
            ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(13))
            .write(
                (((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 65280i32) >> 8)
                    as u8),
            );
        } else {
            ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(12))
            .write(0u8);
            ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(13))
            .write(0u8);
        }
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(14))
            .write(0u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(15))
            .write(0u8);
        crate::c::memcpy(
            (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(16),
            disableStructPtr,
            28u32,
        );
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            44u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitPrintString(bufferId: u8, stringId: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut stringId = stringId;
        let mut i: i32 = 0i32;
        let mut stringInfo: *mut u8 = core::ptr::null_mut();
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(16u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((&raw mut gBattleOutcome).cast::<u8>()).read());
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((stringId) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((stringId) as i32) & 65280i32) >> 8) as u8));
        stringInfo =
            (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(4);
        ((stringInfo).cast::<u16>()).write(((&raw mut gCurrentMove).cast::<u16>()).read());
        ((stringInfo).wrapping_add(2).cast::<u16>())
            .write(((&raw mut gChosenMove).cast::<u16>()).read());
        ((stringInfo).wrapping_add(4).cast::<u16>())
            .write(((&raw mut gLastUsedItem).cast::<u16>()).read());
        ((stringInfo).wrapping_add(6)).write(((&raw mut gLastUsedAbility).cast::<u8>()).read());
        ((stringInfo).wrapping_add(7))
            .write((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).read());
        ((stringInfo).wrapping_add(8))
            .write(((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(82)).read());
        ((stringInfo).wrapping_add(9)).write(
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(177)).read(),
        );
        ((stringInfo).wrapping_add(10))
            .write(((&raw mut gPotentialItemEffectBattler).cast::<u8>()).read());
        ((stringInfo).wrapping_add(11)).write(
            ((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(2))
            .read(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((stringInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i
                    < (if 16i32 >= (if 14i32 >= 11i32 { 14i32 } else { 11i32 }) {
                        16i32
                    } else {
                        (if 14i32 >= 11i32 { 14i32 } else { 11i32 })
                    }))
                {
                    break 'l3;
                }
                'l4: {
                    (((((stringInfo).wrapping_add(16)).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    ((((((stringInfo).wrapping_add(16)).cast::<u8>()).wrapping_offset(16))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    ((((((stringInfo).wrapping_add(16)).cast::<u8>()).wrapping_offset(32))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut gBattleTextBuff3).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            68u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitPrintSelectionString(bufferId: u8, stringId: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut stringId = stringId;
        let mut i: i32 = 0i32;
        let mut stringInfo: *mut u8 = core::ptr::null_mut();
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(17u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(17u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((stringId) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((stringId) as i32) & 65280i32) >> 8) as u8));
        stringInfo =
            (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(4);
        ((stringInfo).cast::<u16>()).write(((&raw mut gCurrentMove).cast::<u16>()).read());
        ((stringInfo).wrapping_add(2).cast::<u16>())
            .write(((&raw mut gChosenMove).cast::<u16>()).read());
        ((stringInfo).wrapping_add(4).cast::<u16>())
            .write(((&raw mut gLastUsedItem).cast::<u16>()).read());
        ((stringInfo).wrapping_add(6)).write(((&raw mut gLastUsedAbility).cast::<u8>()).read());
        ((stringInfo).wrapping_add(7))
            .write((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).read());
        ((stringInfo).wrapping_add(8))
            .write(((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(82)).read());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((stringInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i
                    < (if 16i32 >= (if 14i32 >= 11i32 { 14i32 } else { 11i32 }) {
                        16i32
                    } else {
                        (if 14i32 >= 11i32 { 14i32 } else { 11i32 })
                    }))
                {
                    break 'l3;
                }
                'l4: {
                    (((((stringInfo).wrapping_add(16)).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    ((((((stringInfo).wrapping_add(16)).cast::<u8>()).wrapping_offset(16))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    ((((((stringInfo).wrapping_add(16)).cast::<u8>()).wrapping_offset(32))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut gBattleTextBuff3).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            68u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitChooseAction(bufferId: u8, action: u8, itemId: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut action = action;
        let mut itemId = itemId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(18u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(action);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((itemId) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((itemId) as i32) & 65280i32) >> 8) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitYesNoBox(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(19u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(19u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(19u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(19u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitChooseMove(
    bufferId: u8,
    isDoubleBattle: u8,
    noPPNumber: u8,
    movePPData: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut isDoubleBattle = isDoubleBattle;
        let mut noPPNumber = noPPNumber;
        let mut movePPData = movePPData;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(20u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(isDoubleBattle);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(noPPNumber);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < 20u32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((4i32).wrapping_add(i)) as isize))
                    .write(((movePPData).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            24u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitChooseItem(bufferId: u8, battlePartyOrder: *mut u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut battlePartyOrder = battlePartyOrder;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(21u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < crate::c::div_i32(6i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((1i32).wrapping_add(i)) as isize))
                    .write(((battlePartyOrder).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitChoosePokemon(
    bufferId: u8,
    caseId: u8,
    slotId: u8,
    abilityId: u8,
    data: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut caseId = caseId;
        let mut slotId = slotId;
        let mut abilityId = abilityId;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(22u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(caseId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(slotId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(abilityId);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((4i32).wrapping_add(i)) as isize))
                    .write(((data).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            8u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitCmd23(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(23u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(23u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(23u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(23u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitHealthBarUpdate(bufferId: u8, hpValue: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut hpValue = hpValue;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(24u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(0u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((hpValue) as i16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(((((((hpValue) as i16) as i32) & 65280i32) >> 8) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitExpUpdate(bufferId: u8, partyId: u8, expPoints: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut partyId = partyId;
        let mut expPoints = expPoints;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(25u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(partyId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((expPoints) as i16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(((((((expPoints) as i16) as i32) & 65280i32) >> 8) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitStatusIconUpdate(
    bufferId: u8,
    status1: u32,
    status2: u32,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut status1 = status1;
        let mut status2 = status2;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(26u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((status1) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((status1 & 65280u32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((status1 & 16711680u32) >> 16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .write((((status1 & 4278190080u32) >> 24) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
            .write(((status2) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(6))
            .write((((status2 & 65280u32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(7))
            .write((((status2 & 16711680u32) >> 16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(8))
            .write((((status2 & 4278190080u32) >> 24) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            9u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitStatusAnimation(bufferId: u8, status2: u8, status: u32) {
    unsafe {
        let mut bufferId = bufferId;
        let mut status2 = status2;
        let mut status = status;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(27u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(status2);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((status) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((status & 65280u32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .write((((status & 16711680u32) >> 16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
            .write((((status & 4278190080u32) >> 24) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            6u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitStatusXor(bufferId: u8, b: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut b = b;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(28u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(b);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            2u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitDataTransfer(bufferId: u8, size: u16, data: *mut u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut size = size;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(29u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(29u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((size) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((size) as i32) & 65280i32) >> 8) as u8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((size) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((4i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((size) as i32).wrapping_add(4i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitDMA3Transfer(
    bufferId: u8,
    dst: *mut u8,
    size: u16,
    data: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut dst = dst;
        let mut size = size;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(30u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write((((dst) as usize as u32) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((dst) as usize as u32) & 65280u32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((dst) as usize as u32) & 16711680u32) >> 16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .write((((((dst) as usize as u32) & 4278190080u32) >> 24) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
            .write(((size) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(6))
            .write((((((size) as i32) & 65280i32) >> 8) as u8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((size) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((7i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((size) as i32).wrapping_add(7i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitPlayBGM(
    bufferId: u8,
    songId: u16,
    data: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut songId = songId;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(31u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((songId) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((songId) as i32) & 65280i32) >> 8) as u8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((songId) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((songId) as i32).wrapping_add(3i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitCmd32(bufferId: u8, size: u16, data: *mut u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut size = size;
        let mut data = data;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(32u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((size) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((size) as i32) & 65280i32) >> 8) as u8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((size) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                    .write(
                        ({
                            let __t2 = data;
                            data = (data).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((size) as i32).wrapping_add(3i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitTwoReturnValues(bufferId: u8, ret8: u8, ret16: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut ret8 = ret8;
        let mut ret16 = ret16;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(33u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(ret8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((ret16) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((ret16) as i32) & 65280i32) >> 8) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitChosenMonReturnValue(
    bufferId: u8,
    partyId: u8,
    battlePartyOrder: *mut u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut partyId = partyId;
        let mut battlePartyOrder = battlePartyOrder;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(34u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(partyId);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((2i32).wrapping_add(i)) as isize))
                    .write(((battlePartyOrder).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            5u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitOneReturnValue(bufferId: u8, ret: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut ret = ret;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(35u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((ret) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((ret) as i32) & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitOneReturnValue_Duplicate(bufferId: u8, ret: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut ret = ret;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(36u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((ret) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((ret) as i32) & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitClearUnkVar(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(37u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(37u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(37u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(37u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitSetUnkVar(bufferId: u8, b: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut b = b;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(38u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(b);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            2u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitClearUnkFlag(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(39u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(39u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(39u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(39u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BtlController_EmitToggleUnkFlag(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(40u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(40u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(40u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(40u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitHitAnimation(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(41u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(41u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(41u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(41u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitCantSwitch(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(42u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(42u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(42u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(42u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitPlaySE(bufferId: u8, songId: u16) {
    unsafe {
        let mut bufferId = bufferId;
        let mut songId = songId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(43u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((songId) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((songId) as i32) & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitPlayFanfareOrBGM(
    bufferId: u8,
    songId: u16,
    playBGM: u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut songId = songId;
        let mut playBGM = playBGM;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(44u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((songId) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((songId) as i32) & 65280i32) >> 8) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(playBGM);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitFaintingCry(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(45u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(45u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(45u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(45u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitIntroSlide(bufferId: u8, environmentId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut environmentId = environmentId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(46u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(environmentId);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            2u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitIntroTrainerBallThrow(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(47u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(47u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(47u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(47u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitDrawPartyStatusSummary(
    bufferId: u8,
    hpAndStatus: *mut u8,
    flags: u8,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut hpAndStatus = hpAndStatus;
        let mut flags = flags;
        let mut i: i32 = 0i32;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(48u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(((((flags) as i32) & (-129i32)) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write((((((flags) as i32) & 128i32) >> 7) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(48u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 48i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((4i32).wrapping_add(i)) as isize))
                    .write(((hpAndStatus).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            52u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitHidePartyStatusSummary(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(49u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(49u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(49u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(49u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitEndBounceEffect(bufferId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(50u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(50u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(50u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(50u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitSpriteInvisibility(bufferId: u8, isInvisible: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut isInvisible = isInvisible;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(51u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(isInvisible);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(51u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(51u8);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitBattleAnimation(
    bufferId: u8,
    animationId: u8,
    argument: u16,
) {
    unsafe {
        let mut bufferId = bufferId;
        let mut animationId = animationId;
        let mut argument = argument;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(52u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(animationId);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(((argument) as u8));
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write((((((argument) as i32) & 65280i32) >> 8) as u8));
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            4u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitLinkStandbyMsg(bufferId: u8, mode: u8, record: u32) {
    unsafe {
        let mut bufferId = bufferId;
        let mut mode = mode;
        let mut record = record;
        let mut record_: u8 = ((record) as u8);
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(53u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(mode);
        if (record_) != 0 {
            ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(3))
            .write({
                let __v1 = RecordedBattle_BufferNewBattlerData(
                    (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(4),
                );
                ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(2))
                .write(__v1);
                __v1
            });
        } else {
            ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(3))
            .write({
                let __v2 = 0u8;
                ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(2))
                .write(__v2);
                __v2
            });
        }
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(2))
            .read()) as i32)
                .wrapping_add(4i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitResetActionMoveSelection(bufferId: u8, caseId: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut caseId = caseId;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(54u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(caseId);
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            2u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BtlController_EmitEndLinkBattle(bufferId: u8, battleOutcome: u8) {
    unsafe {
        let mut bufferId = bufferId;
        let mut battleOutcome = battleOutcome;
        (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).write(55u8);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(battleOutcome);
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .write(
                (crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    3,
                    1,
                    false,
                ) as u8),
            );
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(
                (crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    3,
                    1,
                    false,
                ) as u8),
            );
        ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
            .write({
                let __v1 = RecordedBattle_BufferNewBattlerData(
                    (((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(6),
                );
                ((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(4))
                .write(__v1);
                __v1
            });
        PrepareBufferDataTransfer(
            bufferId,
            ((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>(),
            ((((((((&raw mut sBattleBuffersTransferData).cast::<u8>()).cast::<u8>())
                .wrapping_offset(4))
            .read()) as i32)
                .wrapping_add(6i32)) as u16),
        );
    }
}
