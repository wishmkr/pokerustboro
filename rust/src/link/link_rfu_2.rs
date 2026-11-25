//! Translated from `src/link_rfu_2.c` by tools/rustport/c2rs.py.
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
    clippy::manual_is_multiple_of,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unreachable_code,
    unused_assignments,
    unused_variables
)]

use crate::AgbRfu_LinkManager::{
    lman, rfu_LMAN_CHILD_connectParent, rfu_LMAN_REQ_sendData, rfu_LMAN_establishConnection,
    rfu_LMAN_forceChangeSP, rfu_LMAN_initializeManager, rfu_LMAN_initializeRFU,
    rfu_LMAN_manager_entity, rfu_LMAN_powerDownRFU, rfu_LMAN_requestChangeAgbClockMaster,
    rfu_LMAN_setLinkRecovery, rfu_LMAN_setMSCCallback, rfu_LMAN_stopManager, rfu_LMAN_syncVBlank,
};
use crate::agb_main::gMain;
use crate::agb_main::{SetVBlankCallback, gLinkTransferringData};
use crate::battle_main::gBattleTypeFlags;
use crate::berry_blender::GetBlenderArrowPosition;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::librfu_rfu::{
    gRfuLinkStatus, gRfuSlotStatusNI, gRfuSlotStatusUNI, rfu_NI_setSendData,
    rfu_REQ_PARENT_resumeRetransmitAndChange, rfu_REQ_configGameData, rfu_REQ_disconnect,
    rfu_REQ_recvData, rfu_REQ_stopMode, rfu_UNI_clearRecvNewDataFlag, rfu_UNI_readySendData,
    rfu_UNI_setSendData, rfu_clearAllSlot, rfu_clearSlot, rfu_initializeAPI, rfu_setRecvBuffer,
    rfu_setTimerInterrupt, rfu_waitREQComplete,
};
use crate::link::{
    CB2_LinkError, ClearSavedLinkPlayers, CloseLink, ConvertLinkPlayerName, GetBlockReceivedStatus,
    GetLinkPlayerCount, GetMultiplayerId, IsLinkTaskFinished, IsWirelessAdapterConnected,
    LinkPlayerFromBlock, LocalLinkPlayerToBlock, OpenLink, ResetBlockReceivedFlag,
    ResetBlockReceivedFlags, SendBlock, SetLinkErrorBuffer, SetWirelessCommType1,
    gBerryBlenderKeySendAttempts, gLinkPlayers, gLinkType, gReceivedRemoteLinkPlayers,
    gWirelessCommType,
};
use crate::link::{gBlockRecvBuffer, gBlockSendBuffer, gLinkPartnersHeldKeys, gRecvCmds, gSendCmd};
use crate::link_rfu_3::{
    InitHostRfuGameData, RfuBackupQueue_Dequeue, RfuBackupQueue_Enqueue, RfuRecvQueue_Dequeue,
    RfuRecvQueue_Enqueue, RfuRecvQueue_Reset, RfuSendQueue_Dequeue, RfuSendQueue_Enqueue,
    RfuSendQueue_Reset,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::mystery_gift_menu::CB2_MysteryGiftEReader;
use crate::overworld::gHeldKeyCodeToSend;
use crate::palette::{ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade};
use crate::random::{Random, Random2, SeedRng};
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{StringCompare, StringCopy};
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
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
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
// The C's names for task and sprite data slots.
const tDisconnectPlayers: usize = 0;
const tState: usize = 0;
const tActivity: usize = 1;
const tDisconnectMode: usize = 1;
const tConnectingForChat: usize = 7;
// Data tables (translate with cdata.py): sRfuReqConfigTemplate sAvailSlots sAllBlocksReceived sSlotToLinkPlayerTableId sPlayerBitsToCount sPlayerBitsToNewChildIdx sBlockRequests sAcceptedSerialNos sASCII_RfuCmds sASCII_RecoverCmds sShutdownTasks sASCII_PokemonSioInfo sASCII_LinkLossDisconnect sASCII_LinkLossRecoveryNow sASCII_30Spaces sASCII_15Spaces sASCII_8Spaces sASCII_Space sASCII_Asterisk sASCII_NowSlot sASCII_ClockCmds sASCII_ChildParentSearch

/// `struct RfuDebug`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RfuDebug {
    pub unused0: CArray<u8, 6>,
    pub recvCount: u16,
    pub unused1: CArray<u8, 6>,
    pub unkFlag: u8,
    pub childJoinCount: u8,
    pub unused2: CArray<u8, 84>,
    pub blockSendFailures: u16,
    pub unused3: CArray<u8, 29>,
    pub blockSendTime: u8,
    pub unused4: CArray<u8, 88>,
}

unsafe impl Sync for RfuDebug {}

/// `struct SioInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SioInfo {
    pub magic: CArray<u8, 15>,
    pub playerCount: u8,
    pub linkPlayerIdx: CArray<u8, 4>,
    pub linkPlayers: CArray<LinkPlayer, 5>,
    pub filler: CArray<u8, 92>,
}

unsafe impl Sync for SioInfo {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RfuDebug>() == 220);
    assert!(offset_of!(RfuDebug, unused0) == 0);
    assert!(offset_of!(RfuDebug, recvCount) == 6);
    assert!(offset_of!(RfuDebug, unused1) == 8);
    assert!(offset_of!(RfuDebug, unkFlag) == 14);
    assert!(offset_of!(RfuDebug, childJoinCount) == 15);
    assert!(offset_of!(RfuDebug, unused2) == 16);
    assert!(offset_of!(RfuDebug, blockSendFailures) == 100);
    assert!(offset_of!(RfuDebug, unused3) == 102);
    assert!(offset_of!(RfuDebug, blockSendTime) == 131);
    assert!(offset_of!(RfuDebug, unused4) == 132);
    assert!(size_of::<SioInfo>() == 252);
    assert!(offset_of!(SioInfo, magic) == 0);
    assert!(offset_of!(SioInfo, playerCount) == 15);
    assert!(offset_of!(SioInfo, linkPlayerIdx) == 16);
    assert!(offset_of!(SioInfo, linkPlayers) == 20);
    assert!(offset_of!(SioInfo, filler) == 160);
};

const RECV_STATE_FINISHED: u8 = 2;
const RECV_STATE_READY: u8 = 0;
const RECV_STATE_RECEIVING: u8 = 1;
const RFUSTATE_CHILD_CONNECT: u16 = 6;
const RFUSTATE_CHILD_CONNECT_END: u16 = 7;
const RFUSTATE_CHILD_JOINED: u16 = 12;
const RFUSTATE_CHILD_TRY_JOIN: u16 = 11;
const RFUSTATE_CONNECTED: u16 = 10;
const RFUSTATE_FINALIZED: u16 = 20;
const RFUSTATE_INIT: u16 = 0;
const RFUSTATE_INIT_END: u16 = 1;
const RFUSTATE_PARENT_CONNECT: u16 = 2;
const RFUSTATE_PARENT_CONNECT_END: u16 = 3;
const RFUSTATE_PARENT_FINALIZE: u16 = 18;
const RFUSTATE_PARENT_FINALIZE_START: u16 = 17;
const RFUSTATE_RECONNECTED: u16 = 9;
const RFUSTATE_STOP_MANAGER: u16 = 4;
const RFUSTATE_STOP_MANAGER_END: u16 = 5;
const RFUSTATE_UR_CONNECT: u16 = 17;
const RFUSTATE_UR_CONNECT_END: u16 = 18;
const RFUSTATE_UR_FINALIZE: u16 = 16;
const RFUSTATE_UR_PLAYER_EXCHANGE: u16 = 13;
const RFUSTATE_UR_STOP_MANAGER: u16 = 14;
const RFUSTATE_UR_STOP_MANAGER_END: u16 = 15;

static sASCII_15Spaces: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_15Spaces).cast());
static sASCII_30Spaces: Table<CArray<u8, 31>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_30Spaces).cast());
static sASCII_8Spaces: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_8Spaces).cast());
static sASCII_LinkLossDisconnect: Table<CArray<u8, 22>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_LinkLossDisconnect).cast());
static sASCII_LinkLossRecoveryNow: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_LinkLossRecoveryNow).cast());
static sASCII_NowSlot: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_NowSlot).cast());
static sASCII_PokemonSioInfo: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::link_rfu_2::sASCII_PokemonSioInfo).cast());
static sAcceptedSerialNos: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::link_rfu_2::sAcceptedSerialNos).cast());
static sAllBlocksReceived: Table<CArray<u32, 25>> =
    Table((&raw const crate::data::link_rfu_2::sAllBlocksReceived).cast());
static sAvailSlots: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::link_rfu_2::sAvailSlots).cast());
static sBlockRequests: Table<CArray<BlockRequest, 5>> =
    Table((&raw const crate::data::link_rfu_2::sBlockRequests).cast());
static sPlayerBitsToCount: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::link_rfu_2::sPlayerBitsToCount).cast());
static sPlayerBitsToNewChildIdx: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::link_rfu_2::sPlayerBitsToNewChildIdx).cast());
static sRfuReqConfigTemplate: Table<InitializeParametersTag> =
    Table((&raw const crate::data::link_rfu_2::sRfuReqConfigTemplate).cast());
static sShutdownTasks: Table<CArray<Option<unsafe fn(u8)>, 3>> =
    Table((&raw const crate::data::link_rfu_2::sShutdownTasks).cast());
static sSlotToLinkPlayerTableId: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::link_rfu_2::sSlotToLinkPlayerTableId).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gRfuAPIBuffer: crate::global::Global<CArray<u32, 921>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gRfu: crate::global::Global<RfuManager> =
    crate::global::Global::new(unsafe { zeroed() });
pub(crate) static sHeldKeyCount: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sResendBlock8: crate::global::Global<Aligned<CArray<u8, 16>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
pub(crate) static sResendBlock16: crate::global::Global<Aligned<CArray<u16, 8>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gHostRfuGameData: crate::global::Global<Aligned<RfuGameData>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gHostRfuUsername: crate::global::Global<Aligned<CArray<u8, 8>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRfuReqConfig: crate::global::Global<InitializeParametersTag> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRfuDebug: crate::global::Global<RfuDebug> =
    crate::global::Global::new(unsafe { zeroed() });

/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

fn Debug_PrintString(str: *mut c_void, x: u8, y: u8) {}
unsafe fn Debug_PrintNum(num: u16, x: u8, y: u8, numDigits: u8) {}
pub unsafe fn ResetLinkRfuGFLayer() {
    let errorState: u8 = (crate::gl!(&raw gRfu.errorState)).read_volatile();
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                crate::gl!(&raw gRfu) as *mut c_void,
                0x100067a,
            );
        }
    }
    volatile_write(crate::gl!(&raw gRfu.errorState), errorState);
    crate::gl!(gRfu.parentChild = 0xFF);
    if (crate::gl!(&raw gRfu.errorState)).read_volatile() != RFU_ERROR_STATE_IGNORE {
        volatile_write(crate::gl!(&raw gRfu.errorState), RFU_ERROR_STATE_NONE);
    }
    for i in 0..MAX_RFU_PLAYERS {
        ResetSendDataManager(crate::gl!(&raw gRfu.recvBlock[i]));
    }
    ResetSendDataManager(crate::gl!(&raw gRfu.sendBlock));
    RfuRecvQueue_Reset(crate::gl!(&raw gRfu.recvQueue));
    RfuSendQueue_Reset(crate::gl!(&raw gRfu.sendQueue));
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gSendCmd.as_ptr()).as_mut_ptr() as *mut c_void,
                0x1000008,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gRecvCmds.as_ptr()).as_mut_ptr() as *mut c_void,
                0x1000028,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gLinkPlayers.as_ptr()).as_mut_ptr() as *mut c_void,
                0x1000046,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn InitRFU() {
    let serialIntr: Option<crate::agb_main::IntrFunc> =
        (*(crate::gl!(&raw @at crate::agb_main::gIntrTable;).cast_const())
            .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
            .cast_mut())[1];
    let timerIntr: Option<crate::agb_main::IntrFunc> =
        (*(crate::gl!(&raw @at crate::agb_main::gIntrTable;).cast_const())
            .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
            .cast_mut())[2];
    InitRFUAPI();
    rfu_REQ_stopMode();
    rfu_waitREQComplete();
    volatile_write(67109384_usize as *mut u16, 0);
    (*(crate::gl!(&raw @at crate::agb_main::gIntrTable;).cast_const())
        .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
        .cast_mut())[1] = serialIntr;
    (*(crate::gl!(&raw @at crate::agb_main::gIntrTable;).cast_const())
        .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
        .cast_mut())[2] = timerIntr;
    volatile_write(67109384_usize as *mut u16, INTR_FLAG_VBLANK);
}
pub unsafe fn InitRFUAPI() {
    if rfu_initializeAPI(
        (*gRfuAPIBuffer.as_ptr()).as_mut_ptr() as *mut c_void as *mut u32,
        3684,
        &raw mut (*(crate::gl!(&raw @at crate::agb_main::gIntrTable;).cast_const())
            .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
            .cast_mut())[1],
        1,
    ) == 0
    {
        gLinkType.set(0);
        ClearSavedLinkPlayers();
        RfuSetIgnoreError(FALSE as u32);
        ResetLinkRfuGFLayer();
        rfu_setTimerInterrupt(
            3,
            &raw mut (*(crate::gl!(&raw @at crate::agb_main::gIntrTable;).cast_const())
                .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
                .cast_mut())[2],
        );
    }
}
pub(crate) unsafe fn Task_ParentSearchForChildren(taskId: u8) {
    UpdateChildStatuses();
    match crate::gl!(gRfu.state) {
        RFUSTATE_INIT => {
            rfu_LMAN_initializeRFU(crate::gl!(&raw sRfuReqConfig));
            crate::gl!(gRfu.state = RFUSTATE_INIT_END);
            task_set(taskId, 1, 1);
        }
        RFUSTATE_INIT_END => {}
        RFUSTATE_PARENT_CONNECT => {
            rfu_LMAN_establishConnection(
                crate::gl!(gRfu.parentChild),
                0,
                240,
                sAcceptedSerialNos.as_ptr().cast_mut(),
            );
            crate::gl!(gRfu.state = RFUSTATE_PARENT_CONNECT_END);
            task_set(taskId, 1, 6);
        }
        RFUSTATE_PARENT_CONNECT_END => {}
        RFUSTATE_STOP_MANAGER => {
            rfu_LMAN_stopManager(FALSE);
            crate::gl!(gRfu.state = RFUSTATE_STOP_MANAGER_END);
        }
        RFUSTATE_STOP_MANAGER_END => {}
        RFUSTATE_PARENT_FINALIZE => {
            volatile_write(crate::gl!(&raw gRfu.parentFinished), FALSE);
            rfu_LMAN_setMSCCallback(Some(MSCCallback_Parent));
            InitChildRecvBuffers();
            InitParentSendData();
            crate::gl!(gRfu.state = RFUSTATE_FINALIZED);
            task_set(taskId, 1, 8);
            CreateTask(Some(Task_PlayerExchange), 5);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub unsafe fn Rfu_GetIndexOfNewestChild(bits: u8) -> i32 {
    sPlayerBitsToNewChildIdx[bits] as i32
}
fn SetLinkPlayerIdsFromSlots(mut baseSlots: i32, mut addSlots: i32) {
    let mut i: u8 = 0;
    let mut baseId: u8 = 1;
    let mut baseSlotsCopy: i32 = baseSlots;
    let mut newId: i32 = 0;
    if addSlots == -1 {
        for i in 0..RFU_CHILD_MAX {
            if baseSlots & 1 != 0 {
                crate::gl!(gRfu.linkPlayerIdx[i] = baseId);
                baseId += 1;
            }
            baseSlots >>= 1;
        }
    } else {
        i = 0;
        while i < RFU_CHILD_MAX {
            if baseSlotsCopy & 1 == 0 {
                crate::gl!(gRfu.linkPlayerIdx[i] = 0);
            }
            baseSlotsCopy >>= 1;
            i += 1;
        }
        baseId = RFU_CHILD_MAX;
        while baseId != 0 {
            i = 0;
            while i < RFU_CHILD_MAX && crate::gl!(gRfu.linkPlayerIdx[i]) != baseId {
                i += 1;
            }
            if i == RFU_CHILD_MAX {
                newId = baseId as i32;
            }
            baseId -= 1;
        }
        addSlots &= !baseSlots;
        for i in 0..RFU_CHILD_MAX {
            if addSlots & 1 != 0 {
                crate::gl!(
                    gRfu.linkPlayerIdx[i] = ({
                        let t1 = newId;
                        newId += 1;
                        t1
                    }) as u8
                );
            }
            addSlots >>= 1;
        }
    }
}
pub(crate) unsafe fn Task_ChildSearchForParent(taskId: u8) {
    'l1: {
        match crate::gl!(gRfu.state) {
            RFUSTATE_INIT => {
                rfu_LMAN_initializeRFU((&raw const *sRfuReqConfigTemplate).cast_mut());
                crate::gl!(gRfu.state = RFUSTATE_INIT_END);
                task_set(taskId, 1, 1);
            }
            RFUSTATE_INIT_END => {}
            RFUSTATE_CHILD_CONNECT => {
                rfu_LMAN_establishConnection(
                    crate::gl!(gRfu.parentChild),
                    0,
                    240,
                    sAcceptedSerialNos.as_ptr().cast_mut(),
                );
                crate::gl!(gRfu.state = RFUSTATE_CHILD_CONNECT_END);
                task_set(taskId, 1, 7);
            }
            RFUSTATE_CHILD_CONNECT_END => {}
            RFUSTATE_RECONNECTED => {
                task_set(taskId, 1, 10);
            }
            RFUSTATE_CHILD_TRY_JOIN => match GetJoinGroupStatus() {
                5 => {
                    crate::gl!(gRfu.state = RFUSTATE_CHILD_JOINED);
                }
                6 | 9 => {
                    rfu_LMAN_requestChangeAgbClockMaster();
                    crate::gl!(gRfu.disconnectMode = RFU_DISCONNECT_NORMAL);
                    DestroyTask(taskId);
                }
                _ => {}
            },
            RFUSTATE_CHILD_JOINED => {
                let bmChildSlot: u8 =
                    shl_i32(1, (crate::gl!(&raw gRfu.childSlot)).read_volatile() as u32) as u8;
                rfu_clearSlot(12, (crate::gl!(&raw gRfu.childSlot)).read_volatile());
                rfu_setRecvBuffer(
                    TYPE_UNI,
                    (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
                    (*gRfu.as_ptr()).childRecvQueue.as_mut_ptr() as *mut c_void,
                    70,
                );
                rfu_UNI_setSendData(
                    bmChildSlot,
                    (*gRfu.as_ptr()).childSendBuffer.as_mut_ptr() as *mut c_void,
                    14,
                );
                task_set(taskId, 1, 8);
                DestroyTask(taskId);
                if crate::gl!(sRfuDebug.childJoinCount) == 0 {
                    Debug_PrintEmpty();
                    crate::gl!(sRfuDebug.childJoinCount += 1);
                }
                CreateTask(Some(Task_PlayerExchange), 5);
                break 'l1;
            }
            _ => {}
        }
    }
}
unsafe fn InitChildRecvBuffers() {
    let mut acceptSlot: u8 = crate::gl!(lman.acceptSlot_flag);
    for i in 0..RFU_CHILD_MAX {
        if acceptSlot as i32 & 1 != 0 {
            rfu_setRecvBuffer(
                TYPE_UNI,
                i,
                (*gRfu.as_ptr()).childRecvBuffer[i].as_mut_ptr() as *mut c_void,
                14,
            );
            rfu_clearSlot(3, i);
        }
        acceptSlot >>= 1;
    }
}
unsafe fn InitParentSendData() {
    let acceptSlot: u8 = crate::gl!(lman.acceptSlot_flag);
    rfu_UNI_setSendData(
        acceptSlot,
        (*gRfu.as_ptr()).recvCmds.as_mut_ptr() as *mut c_void,
        70,
    );
    crate::gl!(gRfu.parentSendSlot = Rfu_GetIndexOfNewestChild(acceptSlot) as u8);
    crate::gl!(gRfu.parentSlots = acceptSlot);
    SetLinkPlayerIdsFromSlots(acceptSlot as i32, -1);
    crate::gl!(gRfu.parentChild = MODE_PARENT);
}
pub(crate) unsafe fn Task_UnionRoomListen(taskId: u8) {
    if (*GetHostRfuGameData()).activity() == 84 && RfuGetStatus() == RFU_STATUS_NEW_CHILD_DETECTED {
        rfu_REQ_disconnect(crate::gl!(lman.acceptSlot_flag));
        rfu_waitREQComplete();
        RfuSetStatus(0, 0);
    }
    match crate::gl!(gRfu.state) {
        RFUSTATE_INIT => {
            rfu_LMAN_initializeRFU(crate::gl!(&raw sRfuReqConfig));
            crate::gl!(gRfu.state = RFUSTATE_INIT_END);
            task_set(taskId, 1, 1);
        }
        RFUSTATE_INIT_END => {}
        RFUSTATE_UR_CONNECT => {
            rfu_LMAN_establishConnection(
                MODE_P_C_SWITCH,
                0,
                240,
                sAcceptedSerialNos.as_ptr().cast_mut(),
            );
            rfu_LMAN_setMSCCallback(Some(MSCCallback_Child));
            crate::gl!(gRfu.state = RFUSTATE_UR_CONNECT_END);
        }
        RFUSTATE_UR_CONNECT_END => {}
        RFUSTATE_UR_PLAYER_EXCHANGE => {
            if rfu_UNI_setSendData(
                shl_i32(1, (crate::gl!(&raw gRfu.childSlot)).read_volatile() as u32) as u8,
                (*gRfu.as_ptr()).childSendBuffer.as_mut_ptr() as *mut c_void,
                14,
            ) == 0
            {
                crate::gl!(gRfu.parentChild = MODE_CHILD);
                DestroyTask(taskId);
                if task_get(taskId, tConnectingForChat) != 0 {
                    CreateTask(Some(Task_PlayerExchangeChat), 1);
                } else {
                    CreateTask(Some(Task_PlayerExchange), 5);
                }
            }
        }
        RFUSTATE_UR_STOP_MANAGER => {
            rfu_LMAN_stopManager(FALSE);
            crate::gl!(gRfu.state = RFUSTATE_UR_STOP_MANAGER_END);
        }
        RFUSTATE_UR_STOP_MANAGER_END => {}
        RFUSTATE_UR_FINALIZE => {
            volatile_write(crate::gl!(&raw gRfu.parentFinished), FALSE);
            rfu_LMAN_setMSCCallback(Some(MSCCallback_Parent));
            UpdateGameData_GroupLockedIn(TRUE);
            InitChildRecvBuffers();
            InitParentSendData();
            crate::gl!(gRfu.state = RFUSTATE_FINALIZED);
            task_set(taskId, 1, 8);
            crate::gl!(gRfu.parentChild = MODE_PARENT);
            CreateTask(Some(Task_PlayerExchange), 5);
            crate::gl!(gRfu.playerExchangeActive = TRUE);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub unsafe fn LinkRfu_CreateConnectionAsParent() {
    rfu_LMAN_establishConnection(MODE_PARENT, 0, 240, sAcceptedSerialNos.as_ptr().cast_mut());
}
pub unsafe fn LinkRfu_StopManagerBeforeEnteringChat() {
    rfu_LMAN_stopManager(FALSE);
}
pub(crate) unsafe fn MSCCallback_Child(REQ_commandID: u16) {
    for i in 0..COMM_SLOT_LENGTH {
        crate::gl!(gRfu.childSendBuffer[i] = 0);
    }
    rfu_REQ_recvData();
    rfu_waitREQComplete();
    if (*(*gRfuSlotStatusUNI.as_ptr())[(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
        .recv
        .newDataFlag
        != 0
    {
        volatile_write(
            crate::gl!(&raw gRfu.childSendCount),
            (crate::gl!(&raw gRfu.childSendCount)).read_volatile() + 1,
        );
        RfuRecvQueue_Enqueue(
            crate::gl!(&raw gRfu.recvQueue),
            (*gRfu.as_ptr()).childRecvQueue.as_mut_ptr(),
        );
        crate::gl!(sRfuDebug.recvCount += 1);
        UpdateBackupQueue();
        rfu_UNI_readySendData((crate::gl!(&raw gRfu.childSlot)).read_volatile());
        rfu_UNI_clearRecvNewDataFlag((crate::gl!(&raw gRfu.childSlot)).read_volatile());
    }
    rfu_LMAN_REQ_sendData(TRUE);
}
pub(crate) unsafe fn MSCCallback_Parent(REQ_commandID: u16) {
    volatile_write(crate::gl!(&raw gRfu.parentFinished), TRUE);
}
pub unsafe fn LinkRfu_Shutdown() {
    rfu_LMAN_powerDownRFU();
    if crate::gl!(gRfu.parentChild) == MODE_PARENT {
        if FuncIsActiveTask(Some(Task_ParentSearchForChildren)) == TRUE {
            DestroyTask(crate::gl!(gRfu.searchTaskId));
            ResetLinkRfuGFLayer();
        }
    } else if crate::gl!(gRfu.parentChild) == MODE_CHILD {
        if FuncIsActiveTask(Some(Task_ChildSearchForParent)) == TRUE {
            DestroyTask(crate::gl!(gRfu.searchTaskId));
            ResetLinkRfuGFLayer();
        }
    } else if crate::gl!(gRfu.parentChild) == MODE_P_C_SWITCH
        && FuncIsActiveTask(Some(Task_UnionRoomListen)) == TRUE
    {
        DestroyTask(crate::gl!(gRfu.searchTaskId));
        ResetLinkRfuGFLayer();
    }
    for i in 0..3u8 {
        if FuncIsActiveTask(sShutdownTasks[i]) == TRUE {
            DestroyTask(FindTaskIdByFunc(sShutdownTasks[i]));
        }
    }
}
unsafe fn CreateTask_ParentSearchForChildren() {
    crate::gl!(gRfu.searchTaskId = CreateTask(Some(Task_ParentSearchForChildren), 1));
}
fn CanTryReconnectParent() -> u8 {
    if crate::gl!(gRfu.state) == RFUSTATE_CHILD_CONNECT_END && crate::gl!(gRfu.parentId) != 0 {
        return TRUE;
    }
    FALSE
}
unsafe fn TryReconnectParent() -> u32 {
    if crate::gl!(gRfu.state) == RFUSTATE_CHILD_CONNECT_END
        && rfu_LMAN_CHILD_connectParent(
            (*crate::gl!(gRfuLinkStatus)).partner[crate::gl!(gRfu.reconnectParentId)].id,
            240,
        ) == 0
    {
        crate::gl!(gRfu.state = RFUSTATE_RECONNECTED);
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn CreateTask_ChildSearchForParent() {
    crate::gl!(gRfu.searchTaskId = CreateTask(Some(Task_ChildSearchForParent), 1));
}
pub fn LmanAcceptSlotFlagIsNotZero() -> u8 {
    if crate::gl!(lman.acceptSlot_flag) != 0 {
        return TRUE;
    }
    FALSE
}
pub fn LinkRfu_StopManagerAndFinalizeSlots() {
    crate::gl!(gRfu.state = RFUSTATE_STOP_MANAGER);
    crate::gl!(gRfu.acceptSlot_flag = crate::gl!(lman.acceptSlot_flag));
}
pub fn WaitRfuState(force: u32) -> u32 {
    if crate::gl!(gRfu.state) == RFUSTATE_PARENT_FINALIZE_START || force != 0 {
        crate::gl!(gRfu.state = RFUSTATE_PARENT_FINALIZE);
        return TRUE as u32;
    }
    FALSE as u32
}
pub fn StopUnionRoomLinkManager() {
    crate::gl!(gRfu.state = RFUSTATE_UR_STOP_MANAGER);
}
unsafe fn ReadySendDataForSlots(mut slots: u8) {
    for i in 0..RFU_CHILD_MAX {
        if slots as i32 & 1 != 0 {
            rfu_UNI_readySendData(i);
            break;
        }
        slots >>= 1;
    }
}
unsafe fn ReadAllPlayerRecvCmds() {
    for i in 0..MAX_RFU_PLAYERS {
        let rfu: *mut RfuManager = crate::gl!(&raw gRfu);
        for j in 0..7i32 {
            (*rfu).recvCmds[i][j][1] = (crate::gl!(gRecvCmds[i][j]) >> 8) as u8;
            (*rfu).recvCmds[i][j][0] = crate::gl!(gRecvCmds[i][j]) as u8;
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gRecvCmds.as_ptr()).as_mut_ptr() as *mut c_void,
                0x1000028,
            );
        }
    }
}
fn MoveSendCmdToRecv() {
    for i in 0..7i32 {
        crate::gl!(gRecvCmds[0][i] = crate::gl!(gSendCmd[i]));
    }
    for i in 0..7i32 {
        crate::gl!(gSendCmd[i] = 0);
    }
}
unsafe fn UpdateBackupQueue() {
    if (crate::gl!(&raw gRfu.linkRecovered)).read_volatile() != 0 {
        let backupEmpty: u8 = RfuBackupQueue_Dequeue(
            crate::gl!(&raw gRfu.backupQueue),
            (*gRfu.as_ptr()).childSendBuffer.as_mut_ptr(),
        );
        if (crate::gl!(&raw gRfu.backupQueue.count)).read_volatile() == 0 {
            volatile_write(crate::gl!(&raw gRfu.linkRecovered), FALSE);
        }
        if backupEmpty != 0 {
            return;
        }
    }
    if (crate::gl!(&raw gRfu.linkRecovered)).read_volatile() == 0 {
        RfuSendQueue_Dequeue(
            crate::gl!(&raw gRfu.sendQueue),
            (*gRfu.as_ptr()).childSendBuffer.as_mut_ptr(),
        );
        RfuBackupQueue_Enqueue(
            crate::gl!(&raw gRfu.backupQueue),
            (*gRfu.as_ptr()).childSendBuffer.as_mut_ptr(),
        );
    }
}
pub unsafe fn IsRfuRecvQueueEmpty() -> u32 {
    if (*crate::gl!(gRfuLinkStatus)).sendSlotUNIFlag == 0 {
        return FALSE as u32;
    }
    for i in 0..MAX_RFU_PLAYERS {
        for j in 0..7i32 {
            if crate::gl!(gRecvCmds[i][j]) != 0 {
                return FALSE as u32;
            }
        }
    }
    TRUE as u32
}
unsafe fn RfuMain1_Parent() -> u32 {
    if crate::gl!(gRfu.state) < RFUSTATE_FINALIZED {
        rfu_REQ_recvData();
        rfu_waitREQComplete();
        rfu_LMAN_REQ_sendData(FALSE);
    } else {
        volatile_write(crate::gl!(&raw gRfu.parentFinished), FALSE);
        if crate::gl!(gRfu.parentSlots) as i32 & (*crate::gl!(gRfuLinkStatus)).connSlotFlag as i32
            == crate::gl!(gRfu.parentSlots) as i32
            && crate::gl!(gRfu.parentSlots) as i32
                & (*crate::gl!(gRfuLinkStatus)).connSlotFlag as i32
                != 0
        {
            if (crate::gl!(&raw gRfu.parentMain2Failed)).read_volatile() == 0 {
                if crate::gl!(gRfu.disconnectSlots) != 0 {
                    RfuReqDisconnectSlot(crate::gl!(gRfu.disconnectSlots) as u32);
                    crate::gl!(gRfu.disconnectSlots = 0);
                    if crate::gl!(gRfu.disconnectMode) == RFU_DISCONNECT_ERROR {
                        RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, F_RFU_ERROR_8);
                        RfuSetErrorParams(F_RFU_ERROR_8 as u32);
                        return FALSE as u32;
                    }
                    if crate::gl!(lman.acceptSlot_flag) == 0 {
                        LinkRfu_Shutdown();
                        gReceivedRemoteLinkPlayers.set(0);
                        return FALSE as u32;
                    }
                }
                ReadAllPlayerRecvCmds();
                rfu_UNI_readySendData(crate::gl!(gRfu.parentSendSlot));
                rfu_LMAN_REQ_sendData(TRUE);
            } else {
                rfu_REQ_PARENT_resumeRetransmitAndChange();
            }
            crate::gl!(gRfu.runParentMain2 = TRUE);
        }
    }
    FALSE as u32
}
unsafe fn RfuMain2_Parent() -> u32 {
    let mut i: u16 = 0;
    let mut flags: u16 = 0;
    let mut r0: u8 = 0;
    if crate::gl!(gRfu.state) >= RFUSTATE_FINALIZED && crate::gl!(gRfu.runParentMain2) == TRUE {
        rfu_waitREQComplete();
        while (crate::gl!(&raw gRfu.parentFinished)).read_volatile() == FALSE {
            if (crate::gl!(&raw gRfu.errorState)).read_volatile() != RFU_ERROR_STATE_NONE {
                return FALSE as u32;
            }
        }
        rfu_REQ_recvData();
        rfu_waitREQComplete();
        if (crate::gl!(&raw lman.parentAck_flag)).read_volatile() as i32
            & crate::gl!(gRfu.parentSlots) as i32
            == crate::gl!(gRfu.parentSlots) as i32
        {
            volatile_write(crate::gl!(&raw gRfu.parentMain2Failed), FALSE);
            crate::gl!(sRfuDebug.recvCount += 1);
            flags = crate::gl!(lman.acceptSlot_flag) as u16;
            i = 0;
            while i < RFU_CHILD_MAX as u16 {
                if flags as i32 & 1 != 0 {
                    if crate::gl!(gRfu.childRecvBuffer[i][1]) != 0 {
                        if crate::gl!(gRfu.childRecvIds[i]) != 0xFF
                            && (crate::gl!(gRfu.childRecvBuffer[i][0]) >> 5) as i32
                                != (crate::gl!(gRfu.childRecvIds[i]) as i32 + 1) & 7
                        {
                            if ({
                                crate::gl!(gRfu.numChildRecvErrors[i] += 1);
                                crate::gl!(gRfu.numChildRecvErrors[i])
                            }) > 4
                            {
                                RfuSetErrorParams(33024);
                            }
                        } else {
                            crate::gl!(
                                gRfu.childRecvIds[i] =
                                    (crate::gl!(gRfu.childRecvBuffer[i][0]) as i32 / 32) as u8
                            );
                            crate::gl!(gRfu.numChildRecvErrors[i] = 0);
                            crate::gl!(gRfu.childRecvBuffer[i][0] &= 0x1f);
                            r0 = crate::gl!(gRfu.linkPlayerIdx[i]);
                            for j in 0..7u16 {
                                crate::gl!(
                                    gRecvCmds[r0][j] = ({
                                        let gl_i0 = ((j as i32) << 1) + 1;
                                        crate::gl!(gRfu.childRecvBuffer[i][gl_i0])
                                    }
                                        as u16)
                                        << 8
                                        | {
                                            let gl_i1 = (j as i32) << 1;
                                            crate::gl!(gRfu.childRecvBuffer[i][gl_i1])
                                        } as u16
                                );
                                {
                                    let gl_v = 0;
                                    let gl_i2 = ((j as i32) << 1) + 1;
                                    crate::gl!(gRfu.childRecvBuffer[i][gl_i2] = gl_v)
                                };
                                {
                                    let gl_v = 0;
                                    let gl_i3 = (j as i32) << 1;
                                    crate::gl!(gRfu.childRecvBuffer[i][gl_i3] = gl_v)
                                };
                            }
                        }
                    }
                    rfu_UNI_clearRecvNewDataFlag(i as u8);
                }
                flags >>= 1;
                i += 1;
            }
            MoveSendCmdToRecv();
            RfuHandleReceiveCommand(0);
            CallRfuFunc();
            if crate::gl!(gRfu.nextChildBits) != 0 && crate::gl!(gRfu.stopNewConnections) == 0 {
                volatile_write(crate::gl!(&raw sRfuDebug.unkFlag), FALSE);
                rfu_clearSlot(3, crate::gl!(gRfu.parentSendSlot));
                for i in 0..(RFU_CHILD_MAX as u16) {
                    if shr_i32(crate::gl!(gRfu.nextChildBits) as i32, i as u32) & 1 != 0 {
                        rfu_setRecvBuffer(
                            TYPE_UNI,
                            i as u8,
                            (*gRfu.as_ptr()).childRecvBuffer[i].as_mut_ptr() as *mut c_void,
                            14,
                        );
                    }
                }
                SetLinkPlayerIdsFromSlots(
                    crate::gl!(gRfu.parentSlots) as i32,
                    crate::gl!(gRfu.parentSlots) as i32 | crate::gl!(gRfu.nextChildBits) as i32,
                );
                crate::gl!(gRfu.incomingChild = crate::gl!(gRfu.nextChildBits));
                crate::gl!(gRfu.parentSlots |= crate::gl!(gRfu.nextChildBits));
                crate::gl!(gRfu.nextChildBits = 0);
                rfu_UNI_setSendData(
                    crate::gl!(gRfu.parentSlots),
                    (*gRfu.as_ptr()).recvCmds.as_mut_ptr() as *mut c_void,
                    70,
                );
                crate::gl!(
                    gRfu.parentSendSlot =
                        Rfu_GetIndexOfNewestChild(crate::gl!(gRfu.parentSlots)) as u8
                );
                CreateTask(Some(Task_PlayerExchangeUpdate), 0);
            }
        } else {
            volatile_write(crate::gl!(&raw gRfu.parentMain2Failed), TRUE);
            crate::gl!(gRfu.runParentMain2 = FALSE);
        }
        crate::gl!(gRfu.runParentMain2 = FALSE);
    }
    let failed: u8 = (crate::gl!(&raw gRfu.parentMain2Failed)).read_volatile();
    (if (*crate::gl!(gRfuLinkStatus)).sendSlotUNIFlag != 0 {
        failed as i32 & 1
    } else {
        FALSE as i32
    }) as u32
}
unsafe fn ChildBuildSendCmd(sendCmd: *mut u16, dst: *mut u8) {
    if *sendCmd != 0 {
        *sendCmd |= (crate::gl!(gRfu.childSendCmdId) as u16) << 5;
        crate::gl!(gRfu.childSendCmdId = (crate::gl!(gRfu.childSendCmdId) + 1) & 7);
        for i in 0..7i32 {
            *dst.at(2 * i + 1) = (*sendCmd.at(i) >> 8) as u8;
            *dst.at(2 * i) = *sendCmd.at(i) as u8;
        }
    } else {
        for i in 0..COMM_SLOT_LENGTH {
            *dst.at(i) = 0;
        }
    }
}
unsafe fn RfuMain1_Child() -> u32 {
    let mut recv: CArray<u8, 70> = zeroed();
    let mut send: CArray<u8, 14> = zeroed();
    let mut status: u8 = 0;
    RfuRecvQueue_Dequeue(crate::gl!(&raw gRfu.recvQueue), recv.as_mut_ptr());
    let mut i: u8 = 0;
    while i < MAX_RFU_PLAYERS as u8 {
        for j in 0..7u8 {
            crate::gl!(
                gRecvCmds[i][j] = (recv[i as i32 * COMM_SLOT_LENGTH + j as i32 * 2 + 1] as u16)
                    << 8
                    | recv[i as i32 * COMM_SLOT_LENGTH + j as i32 * 2] as u16
            );
        }
        i += 1;
    }
    RfuHandleReceiveCommand(0);
    if (crate::gl!(&raw lman.childClockSlave_flag)).read_volatile() == 0
        && crate::gl!(gRfu.disconnectMode) != RFU_DISCONNECT_NONE
    {
        rfu_REQ_disconnect(
            (*crate::gl!(gRfuLinkStatus)).connSlotFlag
                | (*crate::gl!(gRfuLinkStatus)).linkLossSlotFlag,
        );
        rfu_waitREQComplete();
        status = RfuGetStatus();
        if status != RFU_STATUS_FATAL_ERROR
            && status != RFU_STATUS_JOIN_GROUP_NO
            && status != RFU_STATUS_LEAVE_GROUP
        {
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, 36864);
        }
        rfu_clearAllSlot();
        gReceivedRemoteLinkPlayers.set(FALSE);
        crate::gl!(gRfu.callback = None);
        if crate::gl!(gRfu.disconnectMode) == RFU_DISCONNECT_ERROR {
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, 36864);
            RfuSetErrorParams(36864);
        }
        crate::gl!(
            lman.state = {
                crate::gl!(lman.next_state = 0);
                crate::gl!(lman.next_state)
            }
        );
        crate::gl!(gRfu.disconnectMode = RFU_DISCONNECT_NONE);
    }
    if (crate::gl!(&raw gRfu.childSendCount)).read_volatile() != 0 {
        volatile_write(
            crate::gl!(&raw gRfu.childSendCount),
            (crate::gl!(&raw gRfu.childSendCount)).read_volatile() - 1,
        );
        CallRfuFunc();
        ChildBuildSendCmd((*gSendCmd.as_ptr()).as_mut_ptr(), send.as_mut_ptr());
        RfuSendQueue_Enqueue(crate::gl!(&raw gRfu.sendQueue), send.as_mut_ptr());
        for i in 0..7u8 {
            crate::gl!(gSendCmd[i] = 0);
        }
    }
    IsRfuRecvQueueEmpty()
}
unsafe fn HandleSendFailure(unused: u8, mut flags: u32) {
    let mut temp: i32 = 0;
    let payload: *mut u8 = crate::gl!(gRfu.sendBlock.payload);
    let mut i: i32 = 0;
    while i < crate::gl!(gRfu.sendBlock.count) as i32 {
        if flags & 1 == 0 {
            crate::gl!(sResendBlock16[0] = RFUCMD_SEND_BLOCK | i as u16);
            for j in 0..7i32 {
                temp = j * 2;
                {
                    let gl_v = (*payload.at(12 * i + temp + 1) as u16) << 8
                        | *payload.at(12 * i + temp) as u16;
                    let gl_i4 = j + 1;
                    crate::gl!(sResendBlock16[gl_i4] = gl_v)
                };
            }
            for j in 0..7i32 {
                temp = j * 2;
                {
                    let gl_v = (crate::gl!(sResendBlock16[j]) >> 8) as u8;
                    let gl_i5 = temp + 1;
                    crate::gl!(sResendBlock8[gl_i5] = gl_v)
                };
                crate::gl!(sResendBlock8[temp] = crate::gl!(sResendBlock16[j]) as u8);
            }
            RfuSendQueue_Enqueue(
                crate::gl!(&raw gRfu.sendQueue),
                (*sResendBlock8.as_ptr()).as_mut_ptr(),
            );
            crate::gl!(gRfu.sendBlock.failedFlags |= shl_i32(1, i as u32) as u32);
        }
        flags >>= 1;
        i += 1;
    }
}
pub fn Rfu_SetBlockReceivedFlag(linkPlayerId: u8) {
    if crate::gl!(gRfu.parentChild) == MODE_PARENT && linkPlayerId != 0 {
        crate::gl!(gRfu.numBlocksReceived[linkPlayerId] = 1);
    } else {
        crate::gl!(gRfu.blockReceived[linkPlayerId] = TRUE);
    }
}
pub fn Rfu_ResetBlockReceivedFlag(linkPlayerId: u8) {
    crate::gl!(gRfu.blockReceived[linkPlayerId] = FALSE);
    crate::gl!(gRfu.recvBlock[linkPlayerId].receiving = RECV_STATE_READY);
}
unsafe fn LoadLinkPlayerIds(ids: *mut u8) -> u8 {
    if crate::gl!(gRfu.parentChild) == MODE_PARENT {
        return FALSE;
    }
    for i in 0..RFU_CHILD_MAX {
        crate::gl!(gRfu.linkPlayerIdx[i] = *ids.at(i));
    }
    *ids.at((crate::gl!(&raw gRfu.childSlot)).read_volatile())
}
pub(crate) unsafe fn SendKeysToRfu() {
    if gReceivedRemoteLinkPlayers.get() != 0
        && gHeldKeyCodeToSend.get() != LINK_KEY_CODE_NULL
        && gLinkTransferringData.get() != TRUE
    {
        sHeldKeyCount.set(sHeldKeyCount.get() + 1);
        gHeldKeyCodeToSend.set(gHeldKeyCodeToSend.get() | ((sHeldKeyCount.get() as u16) << 8));
        RfuPrepareSendBuffer(RFUCMD_SEND_HELD_KEYS);
    }
}
pub unsafe fn GetHostRfuGameData() -> *mut RfuGameData {
    (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>()
}
pub fn IsSendingKeysToRfu() -> u32 {
    (crate::gl!(gRfu.callback) == Some(SendKeysToRfu as unsafe fn())) as u32
}
pub fn StartSendingKeysToRfu() {
    crate::gl!(gRfu.callback = Some(SendKeysToRfu));
}
pub fn ClearLinkRfuCallback() {
    crate::gl!(gRfu.callback = None);
}
pub(crate) unsafe fn Rfu_BerryBlenderSendHeldKeys() {
    RfuPrepareSendBuffer(RFUCMD_BLENDER_SEND_KEYS);
    if GetMultiplayerId() == 0 {
        crate::gl!(gSendCmd[6] = GetBlenderArrowPosition());
    }
    gBerryBlenderKeySendAttempts.set(gBerryBlenderKeySendAttempts.get() + (1));
}
pub unsafe fn Rfu_SetBerryBlenderLinkCallback() {
    if (*gRfu.as_ptr()).callback.is_none() {
        crate::gl!(gRfu.callback = Some(Rfu_BerryBlenderSendHeldKeys));
    }
}
unsafe fn RfuHandleReceiveCommand(unused: u8) {
    for i in 0..(MAX_RFU_PLAYERS as u16) {
        'l2: {
            let sw1: i32 = crate::gl!(gRecvCmds[i][0]) as i32 & RFUCMD_MASK;
            let mut fall = false;
            if sw1 == 30720 {
                fall = true;
                if crate::gl!(gRfu.parentChild) == MODE_CHILD
                    && gReceivedRemoteLinkPlayers.get() != 0
                {
                    return;
                }
            }
            if fall || sw1 == 30464 {
                if (*crate::gl!(gRfuLinkStatus)).parentChild == MODE_CHILD {
                    crate::gl!(gRfu.playerCount = crate::gl!(gRecvCmds[i][1]) as u8);
                    crate::gl!(
                        gRfu.multiplayerId = LoadLinkPlayerIds(
                            (*gRecvCmds.as_ptr())[i].as_mut_ptr().at(2) as *mut u8
                        )
                    );
                }
                break 'l2;
            }
            if sw1 == 34816 {
                if crate::gl!(gRfu.recvBlock[i].receiving) == RECV_STATE_READY {
                    crate::gl!(gRfu.recvBlock[i].next = 0);
                    crate::gl!(gRfu.recvBlock[i].count = crate::gl!(gRecvCmds[i][1]));
                    crate::gl!(gRfu.recvBlock[i].owner = crate::gl!(gRecvCmds[i][2]) as u8);
                    crate::gl!(gRfu.recvBlock[i].receivedFlags = 0);
                    crate::gl!(gRfu.recvBlock[i].receiving = RECV_STATE_RECEIVING);
                    crate::gl!(gRfu.blockReceived[i] = FALSE);
                }
                break 'l2;
            }
            if sw1 == 35072 {
                if crate::gl!(gRfu.recvBlock[i].receiving) == RECV_STATE_RECEIVING {
                    crate::gl!(gRfu.recvBlock[i].next = crate::gl!(gRecvCmds[i][0]) & 0xff);
                    crate::gl!(
                        gRfu.recvBlock[i].receivedFlags |=
                            shl_i32(1, crate::gl!(gRfu.recvBlock[i].next) as u32) as u32
                    );
                    for j in 0..6u16 {
                        (*gBlockRecvBuffer.as_ptr())[i]
                            [crate::gl!(gRfu.recvBlock[i].next) as i32 * 6 + j as i32] = {
                            let gl_i6 = j as i32 + 1;
                            crate::gl!(gRecvCmds[i][gl_i6])
                        };
                    }
                    if crate::gl!(gRfu.recvBlock[i].receivedFlags)
                        == sAllBlocksReceived[crate::gl!(gRfu.recvBlock[i].count)]
                    {
                        crate::gl!(gRfu.recvBlock[i].receiving = RECV_STATE_FINISHED);
                        Rfu_SetBlockReceivedFlag(i as u8);
                        if (*GetHostRfuGameData()).activity() == 69
                            && gReceivedRemoteLinkPlayers.get() != 0
                            && crate::gl!(gRfu.parentChild) == MODE_CHILD
                        {
                            ValidateAndReceivePokemonSioInfo(
                                (*gBlockRecvBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                            );
                        }
                    }
                }
                break 'l2;
            }
            if sw1 == 41216 {
                Rfu_InitBlockSend(
                    sBlockRequests[crate::gl!(gRecvCmds[i][1])].address as *mut u8,
                    sBlockRequests[crate::gl!(gRecvCmds[i][1])].size as u16 as u32,
                );
                break 'l2;
            }
            if sw1 == 24320 {
                crate::gl!(gRfu.readyCloseLink[i] = TRUE);
                break 'l2;
            }
            if sw1 == 26112 {
                if crate::gl!(gRfu.allReadyNum) == crate::gl!(gRecvCmds[i][1]) {
                    crate::gl!(gRfu.readyExitStandby[i] = TRUE);
                }
                break 'l2;
            }
            if sw1 == 60672 {
                if crate::gl!(gRfu.parentChild) == MODE_CHILD {
                    if gReceivedRemoteLinkPlayers.get() != 0 {
                        if crate::gl!(gRecvCmds[i][1]) as i32
                            & (*crate::gl!(gRfuLinkStatus)).connSlotFlag as i32
                            != 0
                        {
                            gReceivedRemoteLinkPlayers.set(0);
                            rfu_LMAN_requestChangeAgbClockMaster();
                            crate::gl!(gRfu.disconnectMode = crate::gl!(gRecvCmds[i][2]) as u8);
                        }
                        crate::gl!(gRfu.playerCount = crate::gl!(gRecvCmds[i][3]) as u8);
                        ClearSelectedLinkPlayerIds(crate::gl!(gRecvCmds[i][1]));
                    }
                } else {
                    RfuPrepareSendBuffer(RFUCMD_DISCONNECT_PARENT);
                    crate::gl!(gSendCmd[1] = crate::gl!(gRecvCmds[i][1]));
                    crate::gl!(gSendCmd[2] = crate::gl!(gRecvCmds[i][2]));
                    crate::gl!(gSendCmd[3] = crate::gl!(gRecvCmds[i][3]));
                }
                break 'l2;
            }
            if sw1 == 60928 {
                if crate::gl!(gRfu.parentChild) == MODE_PARENT {
                    crate::gl!(gRfu.disconnectSlots |= crate::gl!(gRecvCmds[i][1]) as u8);
                    crate::gl!(gRfu.disconnectMode = crate::gl!(gRecvCmds[i][2]) as u8);
                    ClearSelectedLinkPlayerIds(crate::gl!(gRecvCmds[i][1]));
                }
                break 'l2;
            }
            if sw1 == 17408 || sw1 == 48640 {
                crate::gl!(gLinkPartnersHeldKeys[i] = crate::gl!(gRecvCmds[i][1]));
                break 'l2;
            }
        }
        if crate::gl!(gRfu.parentChild) == MODE_PARENT && crate::gl!(gRfu.numBlocksReceived[i]) != 0
        {
            if crate::gl!(gRfu.numBlocksReceived[i]) == 4 {
                crate::gl!(gRfu.blockReceived[i] = TRUE);
                crate::gl!(gRfu.numBlocksReceived[i] = 0);
            } else {
                crate::gl!(gRfu.numBlocksReceived[i] += 1);
            }
        }
    }
}
fn AreAllPlayersReadyToReceive() -> u8 {
    for i in 0..MAX_RFU_PLAYERS {
        if crate::gl!(gRfu.recvBlock[i].receiving) != RECV_STATE_READY {
            return FALSE;
        }
    }
    TRUE
}
fn AreAllPlayersFinishedReceiving() -> u8 {
    for i in 0..(crate::gl!(gRfu.playerCount) as i32) {
        if crate::gl!(gRfu.recvBlock[i].receiving) != RECV_STATE_FINISHED
            || crate::gl!(gRfu.blockReceived[i]) != TRUE
        {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn ResetSendDataManager(data: *mut RfuBlockSend) {
    (*data).next = 0;
    (*data).count = 0;
    (*data).payload = null_mut();
    (*data).receivedFlags = 0;
    (*data).sending = FALSE;
    (*data).owner = 0;
    (*data).receiving = RECV_STATE_READY;
}
pub fn Rfu_GetBlockReceivedStatus() -> u8 {
    let mut flags: u8 = 0;
    for i in 0..MAX_RFU_PLAYERS {
        if crate::gl!(gRfu.recvBlock[i].receiving) == RECV_STATE_FINISHED
            && crate::gl!(gRfu.blockReceived[i]) == TRUE
        {
            flags |= shl_i32(1, i as u32) as u8;
        }
    }
    flags
}
unsafe fn RfuPrepareSendBuffer(command: u16) {
    let mut buff: *mut u8 = null_mut();
    let mut tmp: u8 = 0;
    crate::gl!(gSendCmd[0] = command);
    match command {
        RFUCMD_SEND_BLOCK_INIT => {
            crate::gl!(gSendCmd[1] = crate::gl!(gRfu.sendBlock.count));
            crate::gl!(gSendCmd[2] = crate::gl!(gRfu.sendBlock.owner) as u16 + 0x80);
        }
        RFUCMD_SEND_BLOCK_REQ => {
            if AreAllPlayersReadyToReceive() != 0 {
                crate::gl!(gSendCmd[1] = crate::gl!(gRfu.blockRequestType) as u16);
            }
        }
        RFUCMD_SEND_PLAYER_IDS | RFUCMD_SEND_PLAYER_IDS_NEW => {
            tmp = crate::gl!(gRfu.parentSlots) ^ crate::gl!(gRfu.disconnectSlots);
            crate::gl!(gRfu.playerCount = sPlayerBitsToCount[tmp] + 1);
            crate::gl!(gSendCmd[1] = crate::gl!(gRfu.playerCount) as u16);
            buff = crate::gl!(&raw gSendCmd[2]) as *mut u8;
            for i in 0..RFU_CHILD_MAX {
                *buff.at(i) = crate::gl!(gRfu.linkPlayerIdx[i]);
            }
        }
        RFUCMD_READY_EXIT_STANDBY | RFUCMD_READY_CLOSE_LINK => {
            crate::gl!(gSendCmd[1] = crate::gl!(gRfu.allReadyNum));
        }
        RFUCMD_BLENDER_SEND_KEYS => {
            crate::gl!(gSendCmd[0] = command);
            crate::gl!(gSendCmd[1] = crate::gl!(gMain.heldKeys));
        }
        12032 => {
            for i in 0..RFU_PACKET_SIZE {
                {
                    let gl_v = crate::gl!(gRfu.packet[i]);
                    let gl_i7 = 1 + i as i32;
                    crate::gl!(gSendCmd[gl_i7] = gl_v)
                };
            }
        }
        RFUCMD_SEND_HELD_KEYS => {
            crate::gl!(gSendCmd[1] = gHeldKeyCodeToSend.get());
        }
        RFUCMD_DISCONNECT_PARENT | RFUCMD_DISCONNECT => {}
        _ => {}
    }
}
pub unsafe fn Rfu_SendPacket(data: *mut c_void) {
    if crate::gl!(gSendCmd[0]) == 0 && RfuHasErrored() == 0 {
        memcpy(
            (*gRfu.as_ptr()).packet.as_mut_ptr() as *mut u8,
            data as *mut u8,
            12,
        );
        RfuPrepareSendBuffer(RFUCMD_SEND_PACKET as u16);
    }
}
pub unsafe fn Rfu_InitBlockSend(src: *mut u8, size: u32) -> u32 {
    if (*gRfu.as_ptr()).callback.is_some() {
        return FALSE as u32;
    }
    if crate::gl!(gSendCmd[0]) != 0 {
        return FALSE as u32;
    }
    if crate::gl!(gRfu.sendBlock.sending) != 0 {
        crate::gl!(sRfuDebug.blockSendTime += 1);
        return FALSE as u32;
    }
    let r4: u8 = (size % 12 != 0) as u8;
    crate::gl!(gRfu.sendBlock.owner = GetMultiplayerId());
    crate::gl!(gRfu.sendBlock.sending = TRUE);
    crate::gl!(gRfu.sendBlock.count = (size / 12) as u16 + r4 as u16);
    crate::gl!(gRfu.sendBlock.next = 0);
    if size > BLOCK_BUFFER_SIZE {
        crate::gl!(gRfu.sendBlock.payload = src);
    } else {
        if src != (*gBlockSendBuffer.as_ptr()).as_mut_ptr() {
            memcpy((*gBlockSendBuffer.as_ptr()).as_mut_ptr(), src, size);
        }
        crate::gl!(gRfu.sendBlock.payload = (*gBlockSendBuffer.as_ptr()).as_mut_ptr());
    }
    RfuPrepareSendBuffer(RFUCMD_SEND_BLOCK_INIT);
    crate::gl!(gRfu.callback = Some(HandleBlockSend));
    crate::gl!(gRfu.blockSendAttempts = 0);
    TRUE as u32
}
pub(crate) unsafe fn HandleBlockSend() {
    if crate::gl!(gSendCmd[0]) == 0 {
        RfuPrepareSendBuffer(RFUCMD_SEND_BLOCK_INIT);
        if crate::gl!(gRfu.parentChild) == MODE_PARENT {
            if ({
                crate::gl!(gRfu.blockSendAttempts += 1);
                crate::gl!(gRfu.blockSendAttempts)
            }) > 2
            {
                crate::gl!(gRfu.callback = Some(SendNextBlock));
            }
        } else {
            if {
                let gl_i8 = GetMultiplayerId();
                crate::gl!(gRecvCmds[gl_i8][0])
            } as i32
                & RFUCMD_MASK
                == RFUCMD_SEND_BLOCK_INIT as i32
            {
                crate::gl!(gRfu.callback = Some(SendNextBlock));
            }
        }
    }
}
pub(crate) unsafe fn SendNextBlock() {
    let src: *mut u8 = crate::gl!(gRfu.sendBlock.payload);
    crate::gl!(gSendCmd[0] = RFUCMD_SEND_BLOCK | crate::gl!(gRfu.sendBlock.next));
    for i in 0..7i32 {
        {
            let gl_v = (*src.at((i << 1) + crate::gl!(gRfu.sendBlock.next) as i32 * 12 + 1) as u16)
                << 8
                | *src.at((i << 1) + crate::gl!(gRfu.sendBlock.next) as i32 * 12) as u16;
            let gl_i9 = i + 1;
            crate::gl!(gSendCmd[gl_i9] = gl_v)
        };
    }
    crate::gl!(gRfu.sendBlock.next += 1);
    if crate::gl!(gRfu.sendBlock.count) <= crate::gl!(gRfu.sendBlock.next) {
        crate::gl!(gRfu.sendBlock.sending = FALSE);
        crate::gl!(gRfu.callback = Some(SendLastBlock));
    }
}
pub(crate) unsafe fn SendLastBlock() {
    let src: *mut u8 = crate::gl!(gRfu.sendBlock.payload);
    let mpId: u8 = GetMultiplayerId();
    if crate::gl!(gRfu.parentChild) == MODE_CHILD {
        crate::gl!(gSendCmd[0] = RFUCMD_SEND_BLOCK | (crate::gl!(gRfu.sendBlock.count) - 1));
        for i in 0..7i32 {
            {
                let gl_v = (*src
                    .at((i << 1) + (crate::gl!(gRfu.sendBlock.count) as i32 - 1) * 12 + 1)
                    as u16)
                    << 8
                    | *src.at((i << 1) + (crate::gl!(gRfu.sendBlock.count) as i32 - 1) * 12) as u16;
                let gl_i10 = i + 1;
                crate::gl!(gSendCmd[gl_i10] = gl_v)
            };
        }
        if crate::gl!(gRecvCmds[mpId][0]) as u8 as i32
            == crate::gl!(gRfu.sendBlock.count) as i32 - 1
        {
            if crate::gl!(gRfu.recvBlock[mpId].receivedFlags)
                != sAllBlocksReceived[crate::gl!(gRfu.recvBlock[mpId].count)]
            {
                HandleSendFailure(mpId, crate::gl!(gRfu.recvBlock[mpId].receivedFlags));
                crate::gl!(sRfuDebug.blockSendFailures += 1);
            } else {
                crate::gl!(gRfu.callback = None);
            }
        }
    } else {
        crate::gl!(gRfu.callback = None);
    }
}
pub unsafe fn Rfu_SendBlockRequest(r#type: u8) -> u8 {
    crate::gl!(gRfu.blockRequestType = r#type);
    RfuPrepareSendBuffer(RFUCMD_SEND_BLOCK_REQ);
    TRUE
}
unsafe fn RfuShutdownAfterDisconnect() {
    rfu_clearAllSlot();
    rfu_LMAN_powerDownRFU();
    gReceivedRemoteLinkPlayers.set(0);
    crate::gl!(gRfu.isShuttingDown = TRUE);
    crate::gl!(gRfu.callback = None);
}
pub(crate) unsafe fn DisconnectRfu() {
    rfu_REQ_disconnect(
        (*crate::gl!(gRfuLinkStatus)).connSlotFlag | (*crate::gl!(gRfuLinkStatus)).linkLossSlotFlag,
    );
    rfu_waitREQComplete();
    RfuShutdownAfterDisconnect();
}
pub(crate) unsafe fn TryDisconnectRfu() {
    if crate::gl!(gRfu.parentChild) == MODE_CHILD {
        rfu_LMAN_requestChangeAgbClockMaster();
        crate::gl!(gRfu.disconnectMode = RFU_DISCONNECT_NORMAL);
    } else {
        crate::gl!(gRfu.callback = Some(DisconnectRfu));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LinkRfu_FatalError() {
    rfu_LMAN_requestChangeAgbClockMaster();
    crate::gl!(gRfu.disconnectMode = RFU_DISCONNECT_ERROR);
    crate::gl!(
        gRfu.disconnectSlots = (*crate::gl!(gRfuLinkStatus)).connSlotFlag
            | (*crate::gl!(gRfuLinkStatus)).linkLossSlotFlag
    );
}
pub(crate) unsafe fn WaitAllReadyToCloseLink() {
    let playerCount: u8 = crate::gl!(gRfu.playerCount);
    let mut count: i32 = 0;
    for i in 0..MAX_RFU_PLAYERS {
        if crate::gl!(gRfu.readyCloseLink[i]) != 0 {
            count += 1;
        }
    }
    if count == playerCount as i32 {
        gBattleTypeFlags.set(gBattleTypeFlags.get() & (0xffffffdf));
        if crate::gl!(gRfu.parentChild) == MODE_CHILD {
            volatile_write(
                crate::gl!(&raw gRfu.errorState),
                RFU_ERROR_STATE_DISCONNECTING,
            );
            TryDisconnectRfu();
        } else {
            crate::gl!(gRfu.callback = Some(TryDisconnectRfu));
        }
    }
}
pub(crate) unsafe fn SendReadyCloseLink() {
    if crate::gl!(gSendCmd[0]) == 0 && crate::gl!(gRfu.playerExchangeActive) == 0 {
        RfuPrepareSendBuffer(RFUCMD_READY_CLOSE_LINK);
        crate::gl!(gRfu.callback = Some(WaitAllReadyToCloseLink));
    }
}
pub(crate) unsafe fn Task_TryReadyCloseLink(taskId: u8) {
    if (*gRfu.as_ptr()).callback.is_none() {
        crate::gl!(gRfu.stopNewConnections = TRUE);
        crate::gl!(gRfu.callback = Some(SendReadyCloseLink));
        DestroyTask(taskId);
    }
}
pub unsafe fn Rfu_SetCloseLinkCallback() {
    if FuncIsActiveTask(Some(Task_TryReadyCloseLink)) == 0 {
        CreateTask(Some(Task_TryReadyCloseLink), 5);
    }
}
pub(crate) unsafe fn SendReadyExitStandbyUntilAllReady() {
    if GetMultiplayerId() != 0
        && (crate::gl!(&raw gRfu.recvQueue.count)).read_volatile() == 0
        && crate::gl!(gRfu.resendExitStandbyTimer) > 60
    {
        RfuPrepareSendBuffer(RFUCMD_READY_EXIT_STANDBY);
        crate::gl!(gRfu.resendExitStandbyTimer = 0);
    }
    let playerCount: u8 = GetLinkPlayerCount();
    let mut i: u8 = 0;
    while i < playerCount {
        if crate::gl!(gRfu.readyExitStandby[i]) == 0 {
            break;
        }
        i += 1;
    }
    if i == playerCount {
        for i in 0..(MAX_RFU_PLAYERS as u8) {
            crate::gl!(gRfu.readyExitStandby[i] = FALSE);
        }
        crate::gl!(gRfu.allReadyNum += 1);
        crate::gl!(gRfu.callback = None);
    }
    crate::gl!(gRfu.resendExitStandbyTimer += 1);
}
pub(crate) unsafe fn LinkLeaderReadyToExitStandby() {
    if (crate::gl!(&raw gRfu.recvQueue.count)).read_volatile() == 0 && crate::gl!(gSendCmd[0]) == 0
    {
        RfuPrepareSendBuffer(RFUCMD_READY_EXIT_STANDBY);
        crate::gl!(gRfu.callback = Some(SendReadyExitStandbyUntilAllReady));
    }
}
pub(crate) unsafe fn Rfu_LinkStandby() {
    let mut i: u8 = 0;
    let mut playerCount: u8 = 0;
    if GetMultiplayerId() != 0 {
        if (crate::gl!(&raw gRfu.recvQueue.count)).read_volatile() == 0
            && crate::gl!(gSendCmd[0]) == 0
        {
            RfuPrepareSendBuffer(RFUCMD_READY_EXIT_STANDBY);
            crate::gl!(gRfu.callback = Some(SendReadyExitStandbyUntilAllReady));
        }
    } else {
        playerCount = GetLinkPlayerCount();
        i = 1;
        while i < playerCount {
            if crate::gl!(gRfu.readyExitStandby[i]) == 0 {
                break;
            }
            i += 1;
        }
        if i == playerCount
            && (crate::gl!(&raw gRfu.recvQueue.count)).read_volatile() == 0
            && crate::gl!(gSendCmd[0]) == 0
        {
            RfuPrepareSendBuffer(RFUCMD_READY_EXIT_STANDBY);
            crate::gl!(gRfu.callback = Some(LinkLeaderReadyToExitStandby));
        }
    }
}
pub unsafe fn Rfu_SetLinkStandbyCallback() {
    if (*gRfu.as_ptr()).callback.is_none() {
        crate::gl!(gRfu.callback = Some(Rfu_LinkStandby));
        crate::gl!(gRfu.resendExitStandbyTimer = 0);
    }
}
pub unsafe fn IsRfuSerialNumberValid(serialNo: u32) -> u32 {
    let mut i: i32 = 0;
    while sAcceptedSerialNos[i] as u32 != serialNo {
        if sAcceptedSerialNos[i] == RFU_SERIAL_END {
            return FALSE as u32;
        }
        i += 1;
    }
    TRUE as u32
}
pub unsafe fn Rfu_SetLinkRecovery(enable: u32) -> u8 {
    if enable == FALSE as u32 {
        return rfu_LMAN_setLinkRecovery(0, 0);
    }
    rfu_LMAN_setLinkRecovery(1, 600);
    0
}
pub unsafe fn Rfu_StopPartnerSearch() {
    crate::gl!(gRfu.stopNewConnections = TRUE);
    rfu_LMAN_stopManager(FALSE);
}
pub fn Rfu_GetMultiplayerId() -> u8 {
    if crate::gl!(gRfu.parentChild) == MODE_PARENT {
        return 0;
    }
    crate::gl!(gRfu.multiplayerId)
}
pub fn Rfu_GetLinkPlayerCount() -> u8 {
    crate::gl!(gRfu.playerCount)
}
pub unsafe fn IsLinkRfuTaskFinished() -> u8 {
    if crate::gl!(gRfu.status) == RFU_STATUS_CONNECTION_ERROR {
        return FALSE;
    }
    (if (*gRfu.as_ptr()).callback.is_some() {
        FALSE as i32
    } else {
        TRUE as i32
    }) as u8
}
unsafe fn CallRfuFunc() {
    if (*gRfu.as_ptr()).callback.is_some() {
        (*gRfu.as_ptr()).callback.unwrap_unchecked()();
    }
}
unsafe fn CheckForLeavingGroupMembers() -> u8 {
    let mut memberLeft: u8 = FALSE;
    for i in 0..(RFU_CHILD_MAX as i32) {
        if crate::gl!(gRfu.partnerSendStatuses[i]) < RFU_STATUS_JOIN_GROUP_OK
            || crate::gl!(gRfu.partnerSendStatuses[i]) > RFU_STATUS_JOIN_GROUP_NO
        {
            if (*crate::gl!(gRfuSlotStatusNI[i])).recv.state == SLOT_STATE_RECV_SUCCESS
                || (*crate::gl!(gRfuSlotStatusNI[i])).recv.state
                    == SLOT_STATE_RECV_SUCCESS_AND_SENDSIDE_UNKNOWN
            {
                if crate::gl!(gRfu.partnerRecvStatuses[i]) == RFU_STATUS_LEAVE_GROUP_NOTICE {
                    crate::gl!(gRfu.partnerSendStatuses[i] = RFU_STATUS_LEAVE_GROUP);
                    crate::gl!(gRfu.partnerRecvStatuses[i] = RFU_STATUS_CHILD_LEAVE_READY);
                    rfu_clearSlot(TYPE_NI_RECV, i as u8);
                    rfu_NI_setSendData(
                        shl_i32(1, i as u32) as u8,
                        8,
                        crate::gl!(&raw gRfu.partnerSendStatuses[i]) as *mut c_void,
                        1,
                    );
                    memberLeft = TRUE;
                }
            } else if (*(*gRfuSlotStatusNI.as_ptr())
                [(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
            .recv
            .state
                == SLOT_STATE_RECV_FAILED
            {
                rfu_clearSlot(TYPE_NI_RECV, i as u8);
            }
        }
    }
    memberLeft
}
pub unsafe fn RfuTryDisconnectLeavingChildren() -> u32 {
    let mut childrenLeaving: u8 = 0;
    let mut i: i32 = 0;
    while i < RFU_CHILD_MAX as i32 {
        if crate::gl!(gRfu.partnerRecvStatuses[i]) == RFU_STATUS_CHILD_LEAVE {
            childrenLeaving |= shl_i32(1, i as u32) as u8;
            crate::gl!(gRfu.partnerRecvStatuses[i] = RFU_STATUS_OK);
        }
        i += 1;
    }
    if childrenLeaving != 0 {
        rfu_REQ_disconnect(childrenLeaving);
        rfu_waitREQComplete();
    }
    for i in 0..(RFU_CHILD_MAX as i32) {
        if crate::gl!(gRfu.partnerRecvStatuses[i]) == RFU_STATUS_CHILD_LEAVE_READY
            || crate::gl!(gRfu.partnerRecvStatuses[i]) == RFU_STATUS_CHILD_LEAVE
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn HasTrainerLeftPartnersList(trainerId: u16, name: *mut u8) -> u32 {
    let idx: u8 = GetPartnerIndexByNameAndTrainerID(name, trainerId);
    if idx == 0xFF {
        return TRUE as u32;
    }
    if crate::gl!(gRfu.partnerSendStatuses[idx]) == RFU_STATUS_LEAVE_GROUP {
        return TRUE as u32;
    }
    FALSE as u32
}
pub unsafe fn SendRfuStatusToPartner(status: u8, trainerId: u16, name: *mut u8) {
    let idx: u8 = GetPartnerIndexByNameAndTrainerID(name, trainerId);
    crate::gl!(gRfu.partnerSendStatuses[idx] = status);
    rfu_clearSlot(TYPE_NI_SEND, idx);
    rfu_NI_setSendData(
        shl_i32(1, idx as u32) as u8,
        8,
        crate::gl!(&raw gRfu.partnerSendStatuses[idx]) as *mut c_void,
        1,
    );
}
pub unsafe fn SendLeaveGroupNotice() {
    crate::gl!(gRfu.leaveGroupStatus = RFU_STATUS_LEAVE_GROUP_NOTICE);
    rfu_clearSlot(
        TYPE_NI_SEND,
        (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
    );
    rfu_NI_setSendData(
        shl_i32(1, (crate::gl!(&raw gRfu.childSlot)).read_volatile() as u32) as u8,
        8,
        crate::gl!(&raw gRfu.leaveGroupStatus) as *mut c_void,
        1,
    );
}
pub unsafe fn WaitSendRfuStatusToPartner(trainerId: u16, name: *mut u8) -> u32 {
    let idx: u8 = GetPartnerIndexByNameAndTrainerID(name, trainerId);
    if idx == 0xFF {
        return 2;
    }
    if (*crate::gl!(gRfuSlotStatusNI[idx])).send.state == 0 {
        return 1;
    }
    0
}
unsafe fn UpdateChildStatuses() {
    CheckForLeavingGroupMembers();
    for i in 0..(RFU_CHILD_MAX as i32) {
        if (*crate::gl!(gRfuSlotStatusNI[i])).send.state == SLOT_STATE_SEND_SUCCESS
            || (*crate::gl!(gRfuSlotStatusNI[i])).send.state == SLOT_STATE_SEND_FAILED
        {
            if crate::gl!(gRfu.partnerRecvStatuses[i]) == RFU_STATUS_CHILD_LEAVE_READY {
                crate::gl!(gRfu.partnerRecvStatuses[i] = RFU_STATUS_CHILD_LEAVE);
            }
            rfu_clearSlot(TYPE_NI_SEND, i as u8);
        }
    }
}
unsafe fn GetJoinGroupStatus() -> i32 {
    let mut status: i32 = RFU_STATUS_OK as i32;
    if crate::gl!(gRfu.leaveGroupStatus) == RFU_STATUS_LEAVE_GROUP_NOTICE
        && ((*(*gRfuSlotStatusNI.as_ptr())[(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
            .send
            .state
            == SLOT_STATE_SEND_SUCCESS
            || (*(*gRfuSlotStatusNI.as_ptr())[(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
                .send
                .state
                == SLOT_STATE_SEND_FAILED)
    {
        rfu_clearSlot(
            TYPE_NI_SEND,
            (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
        );
    }
    if (*(*gRfuSlotStatusNI.as_ptr())[(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
        .recv
        .state
        == SLOT_STATE_RECV_SUCCESS
        || (*(*gRfuSlotStatusNI.as_ptr())[(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
            .recv
            .state
            == SLOT_STATE_RECV_SUCCESS_AND_SENDSIDE_UNKNOWN
    {
        rfu_clearSlot(
            TYPE_NI_RECV,
            (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
        );
        RfuSetStatus(crate::gl!(gRfu.childRecvStatus), 0);
        status = crate::gl!(gRfu.childRecvStatus) as i32;
    } else if (*(*gRfuSlotStatusNI.as_ptr())[(crate::gl!(&raw gRfu.childSlot)).read_volatile()])
        .recv
        .state
        == SLOT_STATE_RECV_FAILED
    {
        rfu_clearSlot(
            TYPE_NI_RECV,
            (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
        );
        status = RFU_STATUS_JOIN_GROUP_NO as i32;
    }
    status
}
pub(crate) unsafe fn Task_PlayerExchange(taskId: u8) {
    let mut i: i32 = 0;
    if crate::gl!(gRfu.status) == RFU_STATUS_FATAL_ERROR
        || crate::gl!(gRfu.status) == RFU_STATUS_CONNECTION_ERROR
    {
        crate::gl!(gRfu.playerExchangeActive = FALSE);
        DestroyTask(taskId);
    }
    match task_get(taskId, tState) {
        0 => {
            if AreAllPlayersReadyToReceive() != 0 {
                ResetBlockReceivedFlags();
                LocalLinkPlayerToBlock();
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if crate::gl!(gRfu.parentChild) == MODE_PARENT {
                if gReceivedRemoteLinkPlayers.get() != 0 {
                    RfuPrepareSendBuffer(RFUCMD_SEND_PLAYER_IDS_NEW);
                } else {
                    RfuPrepareSendBuffer(RFUCMD_SEND_PLAYER_IDS);
                }
                task_set(taskId, tState, 101);
            } else {
                task_set(taskId, tState, 2);
            }
        }
        101 => {
            if crate::gl!(gSendCmd[0]) == 0 {
                task_set(taskId, tState, 2);
            }
        }
        2 => {
            if crate::gl!(gRfu.playerCount) != 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            if crate::gl!(gRfu.parentChild) == MODE_PARENT {
                if AreAllPlayersReadyToReceive() != 0 {
                    crate::gl!(gRfu.blockRequestType = BLOCK_REQ_SIZE_NONE);
                    RfuPrepareSendBuffer(RFUCMD_SEND_BLOCK_REQ);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            } else {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        4 => {
            if AreAllPlayersFinishedReceiving() != 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        5 => {
            i = 0;
            while i < crate::gl!(gRfu.playerCount) as i32 {
                LinkPlayerFromBlock(i as u32);
                Rfu_ResetBlockReceivedFlag(i as u8);
                i += 1;
            }
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        6 => {
            DestroyTask(taskId);
            gReceivedRemoteLinkPlayers.set(TRUE);
            crate::gl!(gRfu.playerExchangeActive = FALSE);
            rfu_LMAN_setLinkRecovery(1, 600);
            if crate::gl!(gRfu.newChildQueue) != 0 {
                for i in 0..(RFU_CHILD_MAX as i32) {
                    if shr_i32(crate::gl!(gRfu.newChildQueue) as i32, i as u32) & 1 != 0 {
                        crate::gl!(gRfu.nextChildBits = shl_i32(1, i as u32) as u8);
                        crate::gl!(gRfu.newChildQueue ^= shl_i32(1, i as u32) as u8);
                    }
                }
            }
        }
        _ => {}
    }
}
fn ClearSelectedLinkPlayerIds(selected: u16) {
    for i in 0..(RFU_CHILD_MAX as i32) {
        if shr_i32(selected as i32, i as u32) & 1 != 0 {
            crate::gl!(gRfu.linkPlayerIdx[i] = 0);
        }
    }
}
unsafe fn ReceiveRfuLinkPlayers(sioInfo: *mut SioInfo) {
    crate::gl!(gRfu.playerCount = (*sioInfo).playerCount);
    let mut i: i32 = 0;
    while i < RFU_CHILD_MAX as i32 {
        crate::gl!(gRfu.linkPlayerIdx[i] = (*sioInfo).linkPlayerIdx[i]);
        i += 1;
    }
    for i in 0..MAX_RFU_PLAYERS {
        crate::gl!(gLinkPlayers[i] = (*sioInfo).linkPlayers[i]);
        ConvertLinkPlayerName((*gLinkPlayers.as_ptr()).as_mut_ptr().at(i));
    }
}
unsafe fn ValidateAndReceivePokemonSioInfo(recvBuffer: *mut c_void) {
    if strcmp(
        sASCII_PokemonSioInfo.as_ptr().cast_mut(),
        recvBuffer as *mut u8,
    ) == 0
    {
        ReceiveRfuLinkPlayers(recvBuffer as *mut SioInfo);
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(&raw mut tmp as *mut c_void, recvBuffer, 0x100007e);
            }
        }
        ResetBlockReceivedFlag(0);
    }
}
pub(crate) unsafe fn Task_PlayerExchangeUpdate(taskId: u8) {
    let mut playerBlock: *mut LinkPlayerBlock = null_mut();
    let mut sio: *mut SioInfo = null_mut();
    let playerId: u8 =
        (*gRfu.as_ptr()).linkPlayerIdx[sSlotToLinkPlayerTableId[crate::gl!(gRfu.incomingChild)]];
    if crate::gl!(gRfu.status) == RFU_STATUS_FATAL_ERROR
        || crate::gl!(gRfu.status) == RFU_STATUS_CONNECTION_ERROR
    {
        crate::gl!(gRfu.playerExchangeActive = FALSE);
        DestroyTask(taskId);
    }
    'l1: {
        let sw1: i16 = task_get(taskId, tState);
        let mut fall = false;
        if sw1 == 0 {
            if crate::gl!(gSendCmd[0]) == 0 {
                ResetBlockReceivedFlag(playerId);
                RfuPrepareSendBuffer(RFUCMD_SEND_PLAYER_IDS_NEW);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            break 'l1;
        }
        if sw1 == 1 {
            if crate::gl!(gSendCmd[0]) == 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            break 'l1;
        }
        if sw1 == 2 {
            if shr_i32(GetBlockReceivedStatus() as i32, playerId as u32) & 1 != 0 {
                ResetBlockReceivedFlag(playerId);
                playerBlock =
                    (*gBlockRecvBuffer.as_ptr())[playerId].as_mut_ptr() as *mut LinkPlayerBlock;
                crate::gl!(gLinkPlayers[playerId] = (*playerBlock).linkPlayer);
                ConvertLinkPlayerName(crate::gl!(&raw gLinkPlayers[playerId]));
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            sio = (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut SioInfo;
            memcpy(
                (*sio).magic.as_mut_ptr(),
                sASCII_PokemonSioInfo.as_ptr().cast_mut(),
                15,
            );
            (*sio).playerCount = crate::gl!(gRfu.playerCount);
            for i in 0..(RFU_CHILD_MAX as i32) {
                (*sio).linkPlayerIdx[i] = crate::gl!(gRfu.linkPlayerIdx[i]);
            }
            memcpy(
                (*sio).linkPlayers.as_mut_ptr() as *mut u8,
                (*gLinkPlayers.as_ptr()).as_mut_ptr() as *mut u8,
                140,
            );
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        if fall || sw1 == 4 {
            sio = (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut SioInfo;
            (*sio).playerCount = crate::gl!(gRfu.playerCount);
            for i in 0..(RFU_CHILD_MAX as i32) {
                (*sio).linkPlayerIdx[i] = crate::gl!(gRfu.linkPlayerIdx[i]);
            }
            memcpy(
                (*sio).linkPlayers.as_mut_ptr() as *mut u8,
                (*gLinkPlayers.as_ptr()).as_mut_ptr() as *mut u8,
                140,
            );
            if SendBlock(
                0,
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                160,
            ) != 0
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            break 'l1;
        }
        if sw1 == 5 {
            if IsLinkTaskFinished() != 0 && GetBlockReceivedStatus() as i32 & 1 != 0 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            (*gBlockRecvBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                            0x100007e,
                        );
                    }
                }
                ResetBlockReceivedFlag(0);
                crate::gl!(gRfu.playerExchangeActive = FALSE);
                if crate::gl!(gRfu.newChildQueue) != 0 {
                    for i in 0..(RFU_CHILD_MAX as i32) {
                        if shr_i32(crate::gl!(gRfu.newChildQueue) as i32, i as u32) & 1 != 0 {
                            crate::gl!(gRfu.nextChildBits = shl_i32(1, i as u32) as u8);
                            crate::gl!(gRfu.newChildQueue ^= shl_i32(1, i as u32) as u8);
                            crate::gl!(gRfu.playerExchangeActive = TRUE);
                            break;
                        }
                    }
                }
                DestroyTask(taskId);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_PlayerExchangeChat(taskId: u8) {
    if crate::gl!(gRfu.status) == RFU_STATUS_FATAL_ERROR
        || crate::gl!(gRfu.status) == RFU_STATUS_CONNECTION_ERROR
    {
        DestroyTask(taskId);
    }
    match task_get(taskId, tState) {
        0 => {
            if crate::gl!(gRfu.playerCount) != 0 {
                LocalLinkPlayerToBlock();
                SendBlock(
                    0,
                    (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                    60,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 if GetBlockReceivedStatus() as i32 & 1 != 0 => {
            ReceiveRfuLinkPlayers((*gBlockRecvBuffer.as_ptr()).as_mut_ptr() as *mut SioInfo);
            ResetBlockReceivedFlag(0);
            gReceivedRemoteLinkPlayers.set(1);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
unsafe fn RfuCheckErrorStatus() {
    if (crate::gl!(&raw gRfu.errorState)).read_volatile() == RFU_ERROR_STATE_OCCURRED
        && (crate::gl!(&raw lman.childClockSlave_flag)).read_volatile() == 0
    {
        if crate::gl!(gMain.callback2) == Some(CB2_MysteryGiftEReader as unsafe fn())
            || (*crate::gl!(lman.init_param)).mboot_flag != 0
        {
            gWirelessCommType.set(2);
        }
        SetMainCallback2(Some(CB2_LinkError));
        crate::gl!(gMain.savedCallback = Some(CB2_LinkError));
        SetLinkErrorBuffer(
            (crate::gl!(gRfu.errorInfo) as u32) << 16
                | (crate::gl!(gRfu.errorParam0) as u32) << 8
                | crate::gl!(gRfu.errorParam1) as u32,
            (crate::gl!(&raw gRfu.recvQueue.count)).read_volatile(),
            (crate::gl!(&raw gRfu.sendQueue.count)).read_volatile(),
            (RfuGetStatus() == RFU_STATUS_CONNECTION_ERROR) as u8,
        );
        volatile_write(crate::gl!(&raw gRfu.errorState), RFU_ERROR_STATE_PROCESSED);
        CloseLink();
    } else if (crate::gl!(&raw gRfu.sendQueue.full)).read_volatile() == TRUE
        || (crate::gl!(&raw gRfu.recvQueue.full)).read_volatile() == TRUE
    {
        if (crate::gl!(&raw lman.childClockSlave_flag)).read_volatile() != 0 {
            rfu_LMAN_requestChangeAgbClockMaster();
        }
        RfuSetStatus(RFU_STATUS_FATAL_ERROR, 28672);
        RfuSetErrorParams(28672);
    }
}
unsafe fn RfuMain1_UnionRoom() {
    if crate::gl!(lman.parent_child) == MODE_PARENT {
        rfu_REQ_recvData();
        rfu_waitREQComplete();
        rfu_LMAN_REQ_sendData(FALSE);
    }
}
pub unsafe fn RfuMain1() -> u32 {
    let mut retval: u32 = FALSE as u32;
    crate::gl!(gRfu.parentId = 0);
    rfu_LMAN_manager_entity(Random2() as u32);
    if crate::gl!(gRfu.isShuttingDown) == 0 {
        match crate::gl!(gRfu.parentChild) {
            MODE_PARENT => {
                RfuMain1_Parent();
            }
            MODE_CHILD => {
                retval = RfuMain1_Child();
            }
            MODE_P_C_SWITCH => {
                RfuMain1_UnionRoom();
            }
            _ => {}
        }
    }
    retval
}
pub unsafe fn RfuMain2() -> u32 {
    let mut retval: u32 = FALSE as u32;
    if crate::gl!(gRfu.isShuttingDown) == 0 {
        if crate::gl!(gRfu.parentChild) == MODE_PARENT {
            retval = RfuMain2_Parent();
        }
        RfuCheckErrorStatus();
    }
    retval
}
unsafe fn SetHostRfuUsername() {
    StringCopy(
        (*gHostRfuUsername.as_ptr()).as_mut_ptr(),
        (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
    );
}
pub unsafe fn ResetHostRfuGameData() {
    memset(
        (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>() as *mut u8,
        0,
        RFU_GAME_NAME_LENGTH,
    );
    InitHostRfuGameData(
        (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>(),
        0,
        0,
        0,
    );
}
pub unsafe fn SetHostRfuGameData(activity: u8, partnerInfo: u32, startedActivity: u32) {
    InitHostRfuGameData(
        (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>(),
        activity,
        startedActivity,
        partnerInfo as i32,
    );
}
pub unsafe fn SetHostRfuWonderFlags(hasNews: u32, hasCard: u32) {
    (*gHostRfuGameData.as_ptr())
        .compatibility
        .set_hasNews(hasNews as u16);
    (*gHostRfuGameData.as_ptr())
        .compatibility
        .set_hasCard(hasCard as u16);
}
pub fn SetTradeBoardRegisteredMonInfo(r#type: u32, species: u32, level: u32) {
    (*crate::gl!(gHostRfuGameData)).set_tradeType(r#type as u16);
    (*crate::gl!(gHostRfuGameData)).set_tradeSpecies(species as u16);
    (*crate::gl!(gHostRfuGameData)).set_tradeLevel(level as u8);
}
pub unsafe fn GetLinkPlayerInfoFlags(playerId: i32) -> u8 {
    let mut retval: u8 = PINFO_ACTIVE_FLAG;
    retval |= crate::gl!(gLinkPlayers[playerId].gender) << 3;
    retval |= crate::gl!(gLinkPlayers[playerId].trainerId) as u8 & PINFO_TID_MASK;
    retval
}
pub unsafe fn GetOtherPlayersInfoFlags() {
    let data: *mut RfuGameData = (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>();
    let mut i: i32 = 1;
    while i < GetLinkPlayerCount() as i32 {
        (*data).partnerInfo[i - 1] = GetLinkPlayerInfoFlags(i);
        i += 1;
    }
}
pub unsafe fn UpdateGameData_GroupLockedIn(startedActivity: u8) {
    (*crate::gl!(gHostRfuGameData)).set_startedActivity(startedActivity);
    rfu_REQ_configGameData(
        0,
        RFU_SERIAL_GAME,
        (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>() as *mut c_void as *mut u8,
        (*gHostRfuUsername.as_ptr()).as_mut_ptr(),
    );
}
pub unsafe fn UpdateGameData_SetActivity(activity: u8, partnerInfo: u32, startedActivity: u32) {
    if activity != ACTIVITY_NONE {
        SetHostRfuGameData(activity, partnerInfo, startedActivity);
    }
    rfu_REQ_configGameData(
        0,
        RFU_SERIAL_GAME,
        (crate::gl!(&raw gHostRfuGameData)).cast::<RfuGameData>() as *mut c_void as *mut u8,
        (*gHostRfuUsername.as_ptr()).as_mut_ptr(),
    );
}
pub unsafe fn SetUnionRoomChatPlayerData(numPlayers: u32) {
    let mut numConnectedChildren: u32 = 0;
    let mut partnerInfo: u32 = 0;
    let mut slots: i32 = 0;
    if (*GetHostRfuGameData()).activity() == 69 {
        numConnectedChildren = 0;
        partnerInfo = 0;
        slots = crate::gl!(gRfu.parentSlots) as i32 ^ crate::gl!(gRfu.disconnectSlots) as i32;
        for i in 0..(RFU_CHILD_MAX as i32) {
            if shr_i32(slots, i as u32) & 1 != 0 {
                partnerInfo |= shl_u32(
                    PINFO_ACTIVE_FLAG as u32
                        | (({
                            let gl_i0 = crate::gl!(gRfu.linkPlayerIdx[i]);
                            crate::gl!(gLinkPlayers[gl_i0].gender)
                        }) as u32
                            & 1)
                            << 3
                        | ({
                            let gl_i1 = crate::gl!(gRfu.linkPlayerIdx[i]);
                            crate::gl!(gLinkPlayers[gl_i1].trainerId)
                        }) & PINFO_TID_MASK as u32,
                    numConnectedChildren * 8,
                );
                numConnectedChildren += 1;
                if numConnectedChildren == numPlayers - 1 {
                    break;
                }
            }
        }
        UpdateGameData_SetActivity(69, partnerInfo, FALSE as u32);
    }
}
pub unsafe fn RfuSetErrorParams(errorInfo: u32) {
    if (crate::gl!(&raw gRfu.errorState)).read_volatile() == RFU_ERROR_STATE_NONE {
        crate::gl!(gRfu.errorParam0 = crate::gl!(lman.param[0]));
        crate::gl!(gRfu.errorParam1 = crate::gl!(lman.param[1]));
        crate::gl!(gRfu.errorInfo = errorInfo as u16);
        volatile_write(crate::gl!(&raw gRfu.errorState), RFU_ERROR_STATE_OCCURRED);
    }
}
unsafe fn ResetErrorState() {
    volatile_write(crate::gl!(&raw gRfu.errorState), RFU_ERROR_STATE_NONE);
}
pub unsafe fn RfuSetIgnoreError(enable: u32) {
    if enable == 0 {
        volatile_write(crate::gl!(&raw gRfu.errorState), RFU_ERROR_STATE_NONE);
    } else {
        volatile_write(crate::gl!(&raw gRfu.errorState), RFU_ERROR_STATE_IGNORE);
    }
}
pub(crate) unsafe fn DisconnectNewChild() {
    SendDisconnectCommand(
        crate::gl!(lman.acceptSlot_flag) as u32,
        RFU_DISCONNECT_ERROR as u32,
    );
    crate::gl!(gRfu.callback = None);
}
fn StartDisconnectNewChild() {
    crate::gl!(gRfu.callback = Some(DisconnectNewChild));
}
pub(crate) unsafe fn LinkManagerCB_Parent(msg: u8, paramCount: u8) {
    let mut disconnectFlag: u8 = 0;
    match msg {
        LMAN_MSG_INITIALIZE_COMPLETED => {
            crate::gl!(gRfu.state = RFUSTATE_PARENT_CONNECT);
        }
        LMAN_MSG_NEW_CHILD_CONNECT_DETECTED => {}
        LMAN_MSG_NEW_CHILD_CONNECT_ACCEPTED => {
            ParentResetChildRecvMetadata(crate::gl!(lman.param[0]) as i32);
            for i in 0..RFU_CHILD_MAX {
                if shr_i32(crate::gl!(lman.param[0]) as i32, i as u32) & 1 != 0 {
                    let data: *mut RfuGameData =
                        (*crate::gl!(gRfuLinkStatus)).partner[i].gname.as_mut_ptr() as *mut c_void
                            as *mut RfuGameData;
                    if (*data).activity() == (*GetHostRfuGameData()).activity() {
                        crate::gl!(gRfu.partnerSendStatuses[i] = RFU_STATUS_OK);
                        crate::gl!(gRfu.partnerRecvStatuses[i] = RFU_STATUS_OK);
                        rfu_setRecvBuffer(
                            TYPE_NI,
                            i,
                            crate::gl!(&raw gRfu.partnerRecvStatuses[i]) as *mut c_void,
                            1,
                        );
                    } else {
                        disconnectFlag |= shl_i32(1, i as u32) as u8;
                    }
                }
            }
            if disconnectFlag != 0 {
                rfu_REQ_disconnect(disconnectFlag);
                rfu_waitREQComplete();
            }
        }
        LMAN_MSG_NEW_CHILD_CONNECT_REJECTED => {}
        LMAN_MSG_SEARCH_CHILD_PERIOD_EXPIRED => {}
        LMAN_MSG_END_WAIT_CHILD_NAME => {
            if crate::gl!(gRfu.acceptSlot_flag) != crate::gl!(lman.acceptSlot_flag) {
                rfu_REQ_disconnect(
                    crate::gl!(gRfu.acceptSlot_flag) ^ crate::gl!(lman.acceptSlot_flag),
                );
                rfu_waitREQComplete();
            }
            crate::gl!(gRfu.state = RFUSTATE_PARENT_FINALIZE_START);
        }
        LMAN_MSG_LINK_LOSS_DETECTED_AND_START_RECOVERY => {
            crate::gl!(gRfu.linkLossRecoveryState = 1);
        }
        LMAN_MSG_LINK_RECOVERY_SUCCESSED => {
            crate::gl!(gRfu.linkLossRecoveryState = 3);
        }
        LMAN_MSG_LINK_LOSS_DETECTED_AND_DISCONNECTED
        | LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED => {
            crate::gl!(gRfu.linkLossRecoveryState = 4);
            crate::gl!(gRfu.parentSlots &= !(crate::gl!(lman.param[0]) as u8));
            if gReceivedRemoteLinkPlayers.get() == 1 {
                if crate::gl!(gRfu.parentSlots) == 0 {
                    RfuSetErrorParams(msg as u32);
                } else {
                    StartDisconnectNewChild();
                }
            }
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
        }
        52
        | LMAN_MSG_RFU_POWER_DOWN
        | LMAN_MSG_MANAGER_STOPPED
        | LMAN_MSG_MANAGER_FORCED_STOPPED_AND_RFU_RESET => {}
        LMAN_MSG_LMAN_API_ERROR_RETURN => {
            RfuSetStatus(RFU_STATUS_FATAL_ERROR, msg as u16);
            RfuSetErrorParams(msg as u32);
            crate::gl!(gRfu.isShuttingDown = TRUE);
        }
        LMAN_MSG_REQ_API_ERROR
        | LMAN_MSG_WATCH_DOG_TIMER_ERROR
        | LMAN_MSG_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA
        | LMAN_MSG_RFU_FATAL_ERROR => {
            RfuSetErrorParams(msg as u32);
            RfuSetStatus(RFU_STATUS_FATAL_ERROR, msg as u16);
            volatile_write(crate::gl!(&raw gRfu.parentFinished), TRUE);
        }
        _ => {}
    }
}
pub(crate) unsafe fn LinkManagerCB_Child(msg: u8, unused1: u8) {
    'l1: {
        let sw1: u8 = msg;
        let mut fall = false;
        if sw1 == LMAN_MSG_INITIALIZE_COMPLETED {
            crate::gl!(gRfu.state = RFUSTATE_CHILD_CONNECT);
            break 'l1;
        }
        if sw1 == LMAN_MSG_PARENT_FOUND {
            crate::gl!(gRfu.parentId = crate::gl!(lman.param[0]) as u8);
            break 'l1;
        }
        if sw1 == LMAN_MSG_SEARCH_PARENT_PERIOD_EXPIRED {
            break 'l1;
        }
        if sw1 == LMAN_MSG_CONNECT_PARENT_SUCCESSED {
            volatile_write(
                crate::gl!(&raw gRfu.childSlot),
                crate::gl!(lman.param[0]) as u8,
            );
            break 'l1;
        }
        if sw1 == LMAN_MSG_CONNECT_PARENT_FAILED {
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
            break 'l1;
        }
        if sw1 == LMAN_MSG_CHILD_NAME_SEND_COMPLETED {
            crate::gl!(gRfu.state = RFUSTATE_CHILD_TRY_JOIN);
            crate::gl!(gRfu.leaveGroupStatus = RFU_STATUS_OK);
            crate::gl!(gRfu.childRecvStatus = RFU_STATUS_OK);
            rfu_setRecvBuffer(
                TYPE_NI,
                (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
                crate::gl!(&raw gRfu.childRecvStatus) as *mut c_void,
                1,
            );
            rfu_setRecvBuffer(
                TYPE_UNI,
                (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
                (*gRfu.as_ptr()).childRecvQueue.as_mut_ptr() as *mut c_void,
                70,
            );
            break 'l1;
        }
        if sw1 == LMAN_MSG_CHILD_NAME_SEND_FAILED_AND_DISCONNECTED {
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_LOSS_DETECTED_AND_DISCONNECTED {
            fall = true;
            crate::gl!(gRfu.linkLossRecoveryState = 2);
            if crate::gl!(gRfu.childRecvStatus) == RFU_STATUS_JOIN_GROUP_NO {
                break 'l1;
            }
        }
        if fall || sw1 == LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED {
            if crate::gl!(gRfu.linkLossRecoveryState) != 2 {
                crate::gl!(gRfu.linkLossRecoveryState = 4);
            }
            if crate::gl!(gRfu.childRecvStatus) != RFU_STATUS_LEAVE_GROUP {
                RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
            }
            Debug_PrintString(
                sASCII_LinkLossDisconnect.as_ptr().cast_mut() as *mut c_void,
                5,
                5,
            );
            if gReceivedRemoteLinkPlayers.get() == 1 {
                RfuSetErrorParams(msg as u32);
            }
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_LOSS_DETECTED_AND_START_RECOVERY {
            crate::gl!(gRfu.linkLossRecoveryState = 1);
            Debug_PrintString(
                sASCII_LinkLossRecoveryNow.as_ptr().cast_mut() as *mut c_void,
                5,
                5,
            );
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_RECOVERY_SUCCESSED {
            crate::gl!(gRfu.linkLossRecoveryState = 3);
            volatile_write(crate::gl!(&raw gRfu.linkRecovered), TRUE);
            break 'l1;
        }
        if sw1 == 52 {
            break 'l1;
        }
        if sw1 == LMAN_MSG_RFU_POWER_DOWN
            || sw1 == LMAN_MSG_MANAGER_STOPPED
            || sw1 == LMAN_MSG_MANAGER_FORCED_STOPPED_AND_RFU_RESET
        {
            break 'l1;
        }
        if sw1 == LMAN_MSG_LMAN_API_ERROR_RETURN {
            RfuSetStatus(RFU_STATUS_FATAL_ERROR, msg as u16);
            RfuSetErrorParams(msg as u32);
            crate::gl!(gRfu.isShuttingDown = TRUE);
            break 'l1;
        }
        if sw1 == LMAN_MSG_REQ_API_ERROR
            || sw1 == LMAN_MSG_WATCH_DOG_TIMER_ERROR
            || sw1 == LMAN_MSG_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA
            || sw1 == LMAN_MSG_RFU_FATAL_ERROR
        {
            RfuSetStatus(RFU_STATUS_FATAL_ERROR, msg as u16);
            RfuSetErrorParams(msg as u32);
            volatile_write(crate::gl!(&raw gRfu.parentFinished), TRUE);
            break 'l1;
        }
    }
}
fn ParentResetChildRecvMetadata(slot: i32) {
    for i in 0..(RFU_CHILD_MAX as i32) {
        if shr_i32(slot, i as u32) & 1 != 0 {
            crate::gl!(gRfu.numChildRecvErrors[i] = 0);
            crate::gl!(gRfu.childRecvIds[i] = 0xFF);
        }
    }
}
unsafe fn GetNewChildrenInUnionRoomChat(emptySlotMask: i32) -> u8 {
    let mut ret: u8 = 0;
    for i in 0..RFU_CHILD_MAX {
        if shr_i32(emptySlotMask, i as u32) & 1 != 0 {
            let data: *mut RfuGameData = (*crate::gl!(gRfuLinkStatus)).partner[i].gname.as_mut_ptr()
                as *mut c_void as *mut RfuGameData;
            if (*data).activity() == 69 {
                ret |= shl_i32(1, i as u32) as u8;
            }
        }
    }
    ret
}
pub(crate) unsafe fn LinkManagerCB_UnionRoom(msg: u8, paramCount: u8) {
    let mut acceptSlot: u8 = 0;
    'l1: {
        let sw1: u8 = msg;
        let mut fall = false;
        if sw1 == LMAN_MSG_INITIALIZE_COMPLETED {
            crate::gl!(gRfu.state = RFUSTATE_UR_CONNECT);
            break 'l1;
        }
        if sw1 == LMAN_MSG_NEW_CHILD_CONNECT_DETECTED {
            RfuSetStatus(RFU_STATUS_NEW_CHILD_DETECTED, 0);
            break 'l1;
        }
        if sw1 == LMAN_MSG_NEW_CHILD_CONNECT_ACCEPTED {
            if (*GetHostRfuGameData()).activity() == 69 && crate::gl!(gRfu.stopNewConnections) == 0
            {
                let newChildren: u8 =
                    GetNewChildrenInUnionRoomChat(crate::gl!(lman.param[0]) as i32);
                if newChildren != 0 {
                    acceptSlot = shl_i32(1, Rfu_GetIndexOfNewestChild(newChildren) as u32) as u8;
                    if crate::gl!(gRfu.newChildQueue) == 0
                        && crate::gl!(gRfu.playerExchangeActive) == 0
                    {
                        crate::gl!(gRfu.nextChildBits = acceptSlot);
                        crate::gl!(gRfu.newChildQueue |= acceptSlot ^ newChildren);
                        crate::gl!(gRfu.playerExchangeActive = TRUE);
                    } else {
                        crate::gl!(gRfu.newChildQueue |= newChildren);
                    }
                }
                if newChildren as u16 != crate::gl!(lman.param[0]) {
                    crate::gl!(
                        gRfu.disconnectSlots |= newChildren ^ crate::gl!(lman.param[0]) as u8
                    );
                    crate::gl!(gRfu.disconnectMode = RFU_DISCONNECT_NORMAL);
                }
            } else if (*GetHostRfuGameData()).activity() == 84 {
                rfu_REQ_disconnect(crate::gl!(lman.acceptSlot_flag));
                rfu_waitREQComplete();
            }
            ParentResetChildRecvMetadata(crate::gl!(lman.param[0]) as i32);
            break 'l1;
        }
        if sw1 == LMAN_MSG_NEW_CHILD_CONNECT_REJECTED {
            break 'l1;
        }
        if sw1 == LMAN_MSG_SEARCH_CHILD_PERIOD_EXPIRED {
            break 'l1;
        }
        if sw1 == LMAN_MSG_END_WAIT_CHILD_NAME {
            if (*GetHostRfuGameData()).activity() != 69 && crate::gl!(lman.acceptCount) > 1 {
                acceptSlot = shl_i32(
                    1,
                    Rfu_GetIndexOfNewestChild(crate::gl!(lman.param[0]) as u8) as u32,
                ) as u8;
                rfu_REQ_disconnect(crate::gl!(lman.acceptSlot_flag) ^ acceptSlot);
                rfu_waitREQComplete();
            }
            if crate::gl!(gRfu.state) == RFUSTATE_UR_STOP_MANAGER_END {
                crate::gl!(gRfu.state = RFUSTATE_UR_FINALIZE);
            }
            break 'l1;
            break 'l1;
        }
        if sw1 == LMAN_MSG_PARENT_FOUND {
            crate::gl!(gRfu.parentId = crate::gl!(lman.param[0]) as u8);
            break 'l1;
        }
        if sw1 == LMAN_MSG_SEARCH_PARENT_PERIOD_EXPIRED {
            break 'l1;
        }
        if sw1 == LMAN_MSG_CONNECT_PARENT_SUCCESSED {
            volatile_write(
                crate::gl!(&raw gRfu.childSlot),
                crate::gl!(lman.param[0]) as u8,
            );
            break 'l1;
        }
        if sw1 == LMAN_MSG_CONNECT_PARENT_FAILED {
            crate::gl!(gRfu.state = RFUSTATE_UR_CONNECT_END);
            if crate::gl!(gRfu.connectParentFailures) < 2 {
                crate::gl!(gRfu.connectParentFailures += 1);
                CreateTask(Some(Task_TryConnectToUnionRoomParent), 2);
            } else {
                RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
            }
            break 'l1;
        }
        if sw1 == LMAN_MSG_CHILD_NAME_SEND_COMPLETED {
            crate::gl!(gRfu.state = RFUSTATE_UR_PLAYER_EXCHANGE);
            RfuSetStatus(RFU_STATUS_CHILD_SEND_COMPLETE, 0);
            rfu_setRecvBuffer(
                TYPE_UNI,
                (crate::gl!(&raw gRfu.childSlot)).read_volatile(),
                (*gRfu.as_ptr()).childRecvQueue.as_mut_ptr() as *mut c_void,
                70,
            );
            break 'l1;
        }
        if sw1 == LMAN_MSG_CHILD_NAME_SEND_FAILED_AND_DISCONNECTED {
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_LOSS_DETECTED_AND_START_RECOVERY {
            if crate::gl!(lman.acceptSlot_flag) as i32 & crate::gl!(lman.param[0]) as i32 != 0 {
                crate::gl!(gRfu.linkLossRecoveryState = 1);
            }
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_RECOVERY_SUCCESSED {
            crate::gl!(gRfu.linkLossRecoveryState = 3);
            if (*crate::gl!(gRfuLinkStatus)).parentChild == MODE_CHILD {
                volatile_write(crate::gl!(&raw gRfu.linkRecovered), TRUE);
            }
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_LOSS_DETECTED_AND_DISCONNECTED {
            fall = true;
            crate::gl!(gRfu.linkLossRecoveryState = 2);
        }
        if fall || sw1 == LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED {
            if crate::gl!(gRfu.linkLossRecoveryState) != 2 {
                crate::gl!(gRfu.linkLossRecoveryState = 4);
            }
            if crate::gl!(gRfu.parentChild) == MODE_PARENT {
                if gReceivedRemoteLinkPlayers.get() == 1 {
                    crate::gl!(gRfu.parentSlots &= !(crate::gl!(lman.param[0]) as u8));
                    if crate::gl!(gRfu.parentSlots) == 0 {
                        RfuSetErrorParams(msg as u32);
                    } else {
                        StartDisconnectNewChild();
                    }
                }
            } else if crate::gl!(gRfu.disconnectMode) != RFU_DISCONNECT_NORMAL
                && gReceivedRemoteLinkPlayers.get() == 1
            {
                RfuSetErrorParams(msg as u32);
                rfu_LMAN_stopManager(FALSE);
            }
            if (*crate::gl!(gRfuLinkStatus)).parentChild == MODE_NEUTRAL
                && crate::gl!(lman.pcswitch_flag) == 0
                && FuncIsActiveTask(Some(Task_UnionRoomListen)) == TRUE
            {
                crate::gl!(gRfu.state = RFUSTATE_UR_CONNECT);
            }
            RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, msg as u16);
            break 'l1;
        }
        if sw1 == LMAN_MSG_LINK_DISCONNECTED_BY_USER {
            crate::gl!(gRfu.disconnectSlots = 0);
            break 'l1;
        }
        if sw1 == LMAN_MSG_RFU_POWER_DOWN
            || sw1 == LMAN_MSG_MANAGER_STOPPED
            || sw1 == LMAN_MSG_MANAGER_FORCED_STOPPED_AND_RFU_RESET
        {
            break 'l1;
        }
        if sw1 == LMAN_MSG_LMAN_API_ERROR_RETURN {
            RfuSetStatus(RFU_STATUS_FATAL_ERROR, msg as u16);
            RfuSetErrorParams(msg as u32);
            crate::gl!(gRfu.isShuttingDown = TRUE);
            break 'l1;
        }
        if sw1 == LMAN_MSG_REQ_API_ERROR
            || sw1 == LMAN_MSG_WATCH_DOG_TIMER_ERROR
            || sw1 == LMAN_MSG_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA
            || sw1 == LMAN_MSG_RFU_FATAL_ERROR
        {
            RfuSetErrorParams(msg as u32);
            RfuSetStatus(RFU_STATUS_FATAL_ERROR, msg as u16);
            volatile_write(crate::gl!(&raw gRfu.parentFinished), FALSE);
            break 'l1;
        }
    }
}
pub fn RfuSetNormalDisconnectMode() {
    crate::gl!(gRfu.disconnectMode = RFU_DISCONNECT_NORMAL);
}
pub fn RfuSetStatus(status: u8, errorInfo: u16) {
    crate::gl!(gRfu.status = status);
    crate::gl!(gRfu.errorInfo = errorInfo);
}
pub unsafe fn RfuGetStatus() -> u8 {
    crate::gl!(gRfu.status)
}
pub unsafe fn RfuHasErrored() -> u32 {
    let status: u32 = RfuGetStatus() as u32;
    if status == RFU_STATUS_FATAL_ERROR as u32 || status == RFU_STATUS_CONNECTION_ERROR as u32 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn Rfu_IsPlayerExchangeActive() -> u32 {
    crate::gl!(gRfu.playerExchangeActive) as u32
}
pub fn Rfu_IsMaster() -> u8 {
    crate::gl!(gRfu.parentChild)
}
#[unsafe(no_mangle)]
pub unsafe fn RfuVSync() {
    rfu_LMAN_syncVBlank();
}
pub unsafe fn ClearRecvCommands() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gRecvCmds.as_ptr()).as_mut_ptr() as *mut c_void,
                0x5000014,
            );
        }
    }
}
pub(crate) unsafe fn VBlank_RfuIdle() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn Debug_RfuIdle() {
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    ResetPaletteFade();
    SetVBlankCallback(Some(VBlank_RfuIdle));
    if IsWirelessAdapterConnected() != 0 {
        gLinkType.set(LINKTYPE_TRADE);
        SetWirelessCommType1();
        OpenLink();
        SeedRng(crate::gl!(gMain.vblankCounter2) as u16);
        for i in 0..(TRAINER_ID_LENGTH as i32) {
            (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[i] = (Random() as i32 % 256) as u8;
        }
        SetGpuReg(REG_OFFSET_DISPCNT, 5440);
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        CreateTask_RfuIdle();
        SetMainCallback2(Some(CB2_RfuIdle));
    }
}
pub unsafe fn IsUnionRoomListenTaskActive() -> u32 {
    FuncIsActiveTask(Some(Task_UnionRoomListen)) as u32
}
pub unsafe fn CreateTask_RfuIdle() {
    if FuncIsActiveTask(Some(Task_Idle)) == 0 {
        crate::gl!(gRfu.idleTaskId = CreateTask(Some(Task_Idle), 0));
    }
}
pub unsafe fn DestroyTask_RfuIdle() {
    if FuncIsActiveTask(Some(Task_Idle)) == TRUE {
        DestroyTask(crate::gl!(gRfu.idleTaskId));
    }
}
pub(crate) unsafe fn CB2_RfuIdle() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub unsafe fn InitializeRfuLinkManager_LinkLeader(groupMax: u32) {
    crate::gl!(gRfu.parentChild = MODE_PARENT);
    SetHostRfuUsername();
    rfu_LMAN_initializeManager(Some(LinkManagerCB_Parent), None);
    crate::gl!(sRfuReqConfig = *sRfuReqConfigTemplate);
    crate::gl!(sRfuReqConfig.availSlot_flag = sAvailSlots[groupMax - 1] as u16);
    CreateTask_ParentSearchForChildren();
}
pub unsafe fn InitializeRfuLinkManager_JoinGroup() {
    crate::gl!(gRfu.parentChild = MODE_CHILD);
    SetHostRfuUsername();
    rfu_LMAN_initializeManager(Some(LinkManagerCB_Child), Some(MSCCallback_Child));
    CreateTask_ChildSearchForParent();
}
pub unsafe fn InitializeRfuLinkManager_EnterUnionRoom() {
    crate::gl!(gRfu.parentChild = MODE_P_C_SWITCH);
    SetHostRfuUsername();
    rfu_LMAN_initializeManager(Some(LinkManagerCB_UnionRoom), None);
    crate::gl!(sRfuReqConfig = *sRfuReqConfigTemplate);
    crate::gl!(sRfuReqConfig.linkRecovery_enable = 0);
    crate::gl!(sRfuReqConfig.linkRecovery_period = 600);
    crate::gl!(gRfu.searchTaskId = CreateTask(Some(Task_UnionRoomListen), 1));
}
unsafe fn ReadU16(ptr: *mut c_void) -> u16 {
    let ptr_: *mut u8 = ptr as *mut u8;
    (*ptr_.at(1) as u16) << 8 | *ptr_ as u16
}
unsafe fn GetPartnerIndexByNameAndTrainerID(name: *mut u8, id: u16) -> u8 {
    let mut idx: u8 = 0xFF;
    for i in 0..RFU_CHILD_MAX {
        let trainerId: u16 = ReadU16(
            (*((*crate::gl!(gRfuLinkStatus)).partner[i].gname.as_mut_ptr() as *mut RfuGameData))
                .compatibility
                .playerTrainerId
                .as_mut_ptr() as *mut c_void,
        );
        if IsRfuSerialNumberValid((*crate::gl!(gRfuLinkStatus)).partner[i].serialNo as u32) != 0
            && StringCompare(
                name,
                (*crate::gl!(gRfuLinkStatus)).partner[i].uname.as_mut_ptr(),
            ) == 0
            && id == trainerId
        {
            idx = i;
            if (*crate::gl!(gRfuLinkStatus)).partner[i].slot != 0xFF {
                break;
            }
        }
    }
    idx
}
unsafe fn RfuReqDisconnectSlot(slot: u32) {
    rfu_REQ_disconnect(slot as u8);
    rfu_waitREQComplete();
    crate::gl!(gRfu.parentSlots &= !(slot as u8));
    rfu_clearSlot(1, crate::gl!(gRfu.parentSendSlot));
    rfu_UNI_setSendData(
        crate::gl!(gRfu.parentSlots),
        (*gRfu.as_ptr()).recvCmds.as_mut_ptr() as *mut c_void,
        70,
    );
    crate::gl!(gRfu.parentSendSlot = Rfu_GetIndexOfNewestChild(crate::gl!(gRfu.parentSlots)) as u8);
}
pub unsafe fn RequestDisconnectSlotByTrainerNameAndId(name: *mut u8, id: u16) {
    let index: u8 = GetPartnerIndexByNameAndTrainerID(name, id);
    if index != 0xFF {
        RfuReqDisconnectSlot(shl_i32(1, index as u32) as u32);
    }
}
pub unsafe fn Rfu_DisconnectPlayerById(playerIdx: u32) {
    if playerIdx != 0 {
        let mut toDisconnect: u8 = 0;
        for i in 0..(RFU_CHILD_MAX as i32) {
            if crate::gl!(gRfu.linkPlayerIdx[i]) as u32 == playerIdx
                && shr_i32(crate::gl!(gRfu.parentSlots) as i32, i as u32) & 1 != 0
            {
                toDisconnect |= shl_i32(1, i as u32) as u8;
            }
        }
        if toDisconnect != 0 {
            SendDisconnectCommand(toDisconnect as u32, RFU_DISCONNECT_NORMAL as u32);
        }
    }
}
pub(crate) unsafe fn Task_SendDisconnectCommand(taskId: u8) {
    if crate::gl!(gSendCmd[0]) == 0 && crate::gl!(gRfu.playerExchangeActive) == 0 {
        RfuPrepareSendBuffer(RFUCMD_DISCONNECT);
        crate::gl!(gSendCmd[1] = task_get(taskId, tDisconnectPlayers) as u16);
        crate::gl!(gSendCmd[2] = task_get(taskId, tDisconnectMode) as u16);
        crate::gl!(gRfu.playerCount -= sPlayerBitsToCount[task_get(taskId, tDisconnectPlayers)]);
        crate::gl!(gSendCmd[3] = crate::gl!(gRfu.playerCount) as u16);
        DestroyTask(taskId);
    }
}
unsafe fn SendDisconnectCommand(playersToDisconnect: u32, disconnectMode: u32) {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_SendDisconnectCommand));
    if taskId == TASK_NONE {
        taskId = CreateTask(Some(Task_SendDisconnectCommand), 5);
        task_set(taskId, tDisconnectPlayers, playersToDisconnect as i16);
    } else {
        task_set(
            taskId,
            tDisconnectPlayers,
            task_get(taskId, tDisconnectPlayers) | (playersToDisconnect as i16),
        );
    }
    task_set(taskId, tDisconnectMode, disconnectMode as i16);
}
pub(crate) unsafe fn Task_RfuReconnectWithParent(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if CanTryReconnectParent() != 0 {
        let id: u8 =
            GetPartnerIndexByNameAndTrainerID(data as *mut u8, ReadU16(data.at(8) as *mut c_void));
        if id != 0xFF {
            if (*crate::gl!(gRfuLinkStatus)).partner[id].slot != 0xFF {
                crate::gl!(gRfu.reconnectParentId = id);
                if TryReconnectParent() != 0 {
                    DestroyTask(taskId);
                }
            } else if (*GetHostRfuGameData()).activity() == ACTIVITY_WONDER_CARD
                || (*GetHostRfuGameData()).activity() == ACTIVITY_WONDER_NEWS
            {
                *data.at(15) += 1;
            } else {
                RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, 28672);
                DestroyTask(taskId);
            }
        } else {
            *data.at(15) += 1;
            crate::gl!(gRfu.reconnectParentId = id);
        }
    } else {
        *data.at(15) += 1;
    }
    if *data.at(15) > 240 {
        RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, 28672);
        DestroyTask(taskId);
    }
}
pub unsafe fn CreateTask_RfuReconnectWithParent(name: *mut u8, trainerId: u16) {
    let mut data: *mut i16 = null_mut();
    crate::gl!(gRfu.status = RFU_STATUS_OK);
    let taskId: u8 = CreateTask(Some(Task_RfuReconnectWithParent), 3);
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    StringCopy(data as *mut u8, name);
    *data.at(8) = trainerId as i16;
}
unsafe fn IsPartnerActivityIncompatible(activity: i16, partner: *mut RfuGameData) -> u32 {
    if (*GetHostRfuGameData()).activity() == 69 {
        if (*partner).activity() != 69 {
            return TRUE as u32;
        }
    } else if (*partner).activity() != IN_UNION_ROOM {
        return TRUE as u32;
    } else if activity == 68 {
        let original: *mut RfuGameData = crate::gl!(&raw gRfu.parent);
        if (*original).tradeSpecies() == SPECIES_EGG as u16 {
            if (*partner).tradeSpecies() == (*original).tradeSpecies() {
                return FALSE as u32;
            } else {
                return TRUE as u32;
            }
        } else if (*partner).tradeSpecies() != (*original).tradeSpecies()
            || (*partner).tradeLevel() != (*original).tradeLevel()
            || (*partner).tradeType() != (*original).tradeType()
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub(crate) unsafe fn Task_TryConnectToUnionRoomParent(taskId: u8) {
    if crate::gl!(gRfu.status) == RFU_STATUS_NEW_CHILD_DETECTED {
        DestroyTask(taskId);
    }
    if ({
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_get(taskId, 0)
    }) > 300
    {
        RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, 28672);
        DestroyTask(taskId);
    }
    if crate::gl!(gRfu.parentId) != 0 && crate::gl!(lman.parent_child) == 0x00 {
        let trainerId: u16 = ReadU16(
            (*gRfu.as_ptr())
                .parent
                .compatibility
                .playerTrainerId
                .as_mut_ptr() as *mut c_void,
        );
        let id: u8 =
            GetPartnerIndexByNameAndTrainerID((*gRfu.as_ptr()).parentName.as_mut_ptr(), trainerId);
        if id != 0xFF {
            if IsPartnerActivityIncompatible(
                task_get(taskId, tActivity),
                (*crate::gl!(gRfuLinkStatus)).partner[id].gname.as_mut_ptr() as *mut c_void
                    as *mut RfuGameData,
            ) == 0
            {
                if (*crate::gl!(gRfuLinkStatus)).partner[id].slot != 0xFF
                    && rfu_LMAN_CHILD_connectParent(
                        (*crate::gl!(gRfuLinkStatus)).partner[id].id,
                        90,
                    ) == 0
                {
                    crate::gl!(gRfu.state = RFUSTATE_CONNECTED);
                    DestroyTask(taskId);
                }
            } else {
                RfuSetStatus(RFU_STATUS_CONNECTION_ERROR, 28672);
                DestroyTask(taskId);
            }
        }
    }
}
pub unsafe fn TryConnectToUnionRoomParent(name: *mut u8, parent: *mut RfuGameData, activity: u8) {
    crate::gl!(gRfu.connectParentFailures = 0);
    crate::gl!(gRfu.status = RFU_STATUS_OK);
    StringCopy((*gRfu.as_ptr()).parentName.as_mut_ptr(), name);
    memcpy(
        crate::gl!(&raw gRfu.parent) as *mut u8,
        parent as *mut u8,
        RFU_GAME_NAME_LENGTH,
    );
    rfu_LMAN_forceChangeSP();
    let taskId: u8 = CreateTask(Some(Task_TryConnectToUnionRoomParent), 2);
    task_set(taskId, tActivity, activity as i16);
    let listenTaskId: u8 = FindTaskIdByFunc(Some(Task_UnionRoomListen));
    if activity == 69 {
        if listenTaskId != TASK_NONE {
            task_set(listenTaskId, tConnectingForChat, TRUE as i16);
        }
    } else {
        if listenTaskId != TASK_NONE {
            task_set(listenTaskId, tConnectingForChat, FALSE as i16);
        }
    }
}
pub fn IsRfuRecoveringFromLinkLoss() -> u8 {
    if crate::gl!(gRfu.linkLossRecoveryState) == 1 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub fn IsRfuCommunicatingWithAllChildren() -> u32 {
    for i in 0..(RFU_CHILD_MAX as i32) {
        if shr_i32(crate::gl!(lman.acceptSlot_flag) as i32, i as u32) & 1 != 0
            && crate::gl!(gRfu.partnerSendStatuses[i]) == RFU_STATUS_OK
        {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
fn Debug_PrintEmpty() {
    for i in 0..20i32 {
        Debug_PrintString(
            sASCII_30Spaces.as_ptr().cast_mut() as *mut c_void,
            0,
            i as u8,
        );
    }
}
unsafe fn Debug_PrintStatus() {
    let mut i: i32 = 0;
    Debug_PrintNum(GetBlockReceivedStatus() as u16, 28, 19, 2);
    Debug_PrintNum((*crate::gl!(gRfuLinkStatus)).connSlotFlag as u16, 20, 1, 1);
    Debug_PrintNum(
        (*crate::gl!(gRfuLinkStatus)).linkLossSlotFlag as u16,
        23,
        1,
        1,
    );
    if crate::gl!(gRfu.parentChild) == MODE_PARENT {
        i = 0;
        while i < RFU_CHILD_MAX as i32 {
            if shr_i32((*crate::gl!(gRfuLinkStatus)).getNameFlag as i32, i as u32) & 1 != 0 {
                Debug_PrintNum(
                    (*crate::gl!(gRfuLinkStatus)).partner[i].serialNo,
                    1,
                    i as u8 + 3,
                    4,
                );
                Debug_PrintString(
                    (*crate::gl!(gRfuLinkStatus)).partner[i].gname.as_mut_ptr() as *mut c_void,
                    6,
                    i as u8 + 3,
                );
                Debug_PrintString(
                    (*crate::gl!(gRfuLinkStatus)).partner[i].uname.as_mut_ptr() as *mut c_void,
                    22,
                    i as u8 + 3,
                );
            }
            i += 1;
        }
        for i in 0..(RFU_CHILD_MAX as i32) {
            for j in 0..COMM_SLOT_LENGTH {
                Debug_PrintNum(
                    crate::gl!(gRfu.childRecvBuffer[i][j]) as u16,
                    j as u8 * 2,
                    i as u8 + 11,
                    2,
                );
            }
        }
        Debug_PrintString(sASCII_NowSlot.as_ptr().cast_mut() as *mut c_void, 1, 15);
    } else if (*crate::gl!(gRfuLinkStatus)).connSlotFlag != 0
        && (*crate::gl!(gRfuLinkStatus)).getNameFlag != 0
    {
        for i in 0..(RFU_CHILD_MAX as i32) {
            Debug_PrintNum(0, 1, i as u8 + 3, 4);
            Debug_PrintString(
                sASCII_15Spaces.as_ptr().cast_mut() as *mut c_void,
                6,
                i as u8 + 3,
            );
            Debug_PrintString(
                sASCII_8Spaces.as_ptr().cast_mut() as *mut c_void,
                22,
                i as u8 + 3,
            );
        }
        Debug_PrintNum(
            (*crate::gl!(gRfuLinkStatus)).partner
                [(crate::gl!(&raw gRfu.childSlot)).read_volatile()]
            .serialNo,
            1,
            3,
            4,
        );
        Debug_PrintString(
            (*crate::gl!(gRfuLinkStatus)).partner[(crate::gl!(&raw gRfu.childSlot)).read_volatile()]
                .gname
                .as_mut_ptr() as *mut c_void,
            6,
            3,
        );
        Debug_PrintString(
            (*crate::gl!(gRfuLinkStatus)).partner[(crate::gl!(&raw gRfu.childSlot)).read_volatile()]
                .uname
                .as_mut_ptr() as *mut c_void,
            22,
            3,
        );
    } else {
        i = 0;
        while i < (*crate::gl!(gRfuLinkStatus)).findParentCount as i32 {
            if (*crate::gl!(gRfuLinkStatus)).partner[i].slot != 0xFF {
                Debug_PrintNum(
                    (*crate::gl!(gRfuLinkStatus)).partner[i].serialNo,
                    1,
                    i as u8 + 3,
                    4,
                );
                Debug_PrintNum(
                    (*crate::gl!(gRfuLinkStatus)).partner[i].id,
                    6,
                    i as u8 + 3,
                    4,
                );
                Debug_PrintString(
                    (*crate::gl!(gRfuLinkStatus)).partner[i].uname.as_mut_ptr() as *mut c_void,
                    22,
                    i as u8 + 3,
                );
            }
            i += 1;
        }
        while i < RFU_CHILD_MAX as i32 {
            Debug_PrintNum(0, 1, i as u8 + 3, 4);
            Debug_PrintString(
                sASCII_15Spaces.as_ptr().cast_mut() as *mut c_void,
                6,
                i as u8 + 3,
            );
            Debug_PrintString(
                sASCII_8Spaces.as_ptr().cast_mut() as *mut c_void,
                22,
                i as u8 + 3,
            );
            i += 1;
        }
    }
}
unsafe fn GetRfuSendQueueLength() -> u32 {
    (crate::gl!(&raw gRfu.sendQueue.count)).read_volatile() as u32
}
pub unsafe fn GetRfuRecvQueueLength() -> u32 {
    (crate::gl!(&raw gRfu.recvQueue.count)).read_volatile() as u32
}
pub(crate) unsafe fn Task_Idle(taskId: u8) {}
