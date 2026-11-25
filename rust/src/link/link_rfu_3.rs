//! Translated from `src/link_rfu_3.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    dead_code,
    unreachable_code,
    unused_assignments
)]

use crate::AgbRfu_LinkManager::lman;
use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, IsNationalPokedexEnabled};
use crate::librfu_rfu::gRfuLinkStatus;
use crate::link::{GetLinkPlayerCount, GetMultiplayerId, gLinkPlayers, gWirelessCommType};
use crate::link_rfu_2::gHostRfuUsername;
use crate::link_rfu_2::{IsRfuRecoveringFromLinkLoss, IsRfuSerialNumberValid, RfuGetStatus};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::random::Random;
use crate::sprite::GetSpriteTileStartByTag;
use crate::sprite::gSprites;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `StringCompare` with this module's view of its types.
#[inline]
unsafe fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompare(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const sNextAnimNum: usize = 0;
const sSavedAnimNum: usize = 1;
const sCurrAnimNum: usize = 2;
const sFrameDelay: usize = 3;
const sFrameIdx: usize = 4;
const sTileStart: usize = 6;
const sValidator: usize = 7;
// Data tables (translate with cdata.py): sWirelessLinkIconPalette sWirelessLinkIconPic sWireless_ASCIItoRSETable sWireless_RSEtoASCIITable sWirelessStatusIndicatorOamData sWirelessStatusIndicator_3Bars sWirelessStatusIndicator_2Bars sWirelessStatusIndicator_1Bar sWirelessStatusIndicator_Searching sWirelessStatusIndicator_Error sWirelessStatusIndicatorAnims sWirelessStatusIndicatorSpriteSheet sWirelessStatusIndicatorSpritePalette sWirelessStatusIndicatorSpriteTemplate

/// `struct RfuUnusedQueue`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RfuUnusedQueue {
    pub slots: CArray<CArray<u8, 256>, 2>,
    pub recvSlot: u8,
    pub sendSlot: u8,
    pub count: u8,
    pub full: u8,
}

unsafe impl Sync for RfuUnusedQueue {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RfuUnusedQueue>() == 516);
    assert!(offset_of!(RfuUnusedQueue, slots) == 0);
    assert!(offset_of!(RfuUnusedQueue, recvSlot) == 512);
    assert!(offset_of!(RfuUnusedQueue, sendSlot) == 513);
    assert!(offset_of!(RfuUnusedQueue, count) == 514);
    assert!(offset_of!(RfuUnusedQueue, full) == 515);
};

const SEQ_ARRAY_MAX_SIZE: i32 = 200;
const STATUS_INDICATOR_ACTIVE: i16 = 4660;
const UNUSED_QUEUE_NUM_SLOTS: i32 = 2;
const UNUSED_QUEUE_SLOT_LENGTH: i32 = 256;
const WIRELESS_STATUS_ANIM_1_BAR: i16 = 2;
const WIRELESS_STATUS_ANIM_2_BARS: i16 = 1;
const WIRELESS_STATUS_ANIM_3_BARS: i16 = 0;
const WIRELESS_STATUS_ANIM_ERROR: i16 = 4;
const WIRELESS_STATUS_ANIM_SEARCHING: i16 = 3;

static sWirelessStatusIndicatorOamData: Table<OamData> =
    Table((&raw const crate::data::link_rfu_3::sWirelessStatusIndicatorOamData).cast());
static sWirelessStatusIndicatorSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::link_rfu_3::sWirelessStatusIndicatorSpritePalette).cast());
static sWirelessStatusIndicatorSpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::link_rfu_3::sWirelessStatusIndicatorSpriteSheet).cast());
static sWirelessStatusIndicatorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::link_rfu_3::sWirelessStatusIndicatorSpriteTemplate).cast());
static sWireless_ASCIItoRSETable: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::link_rfu_3::sWireless_ASCIItoRSETable).cast());
static sWireless_RSEtoASCIITable: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::link_rfu_3::sWireless_RSEtoASCIITable).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gWirelessStatusIndicatorSpriteId: crate::global::Global<u8> =
    crate::global::Global::new(0);
pub(crate) static sSequenceArrayValOffset: crate::global::Global<u8> =
    crate::global::Global::new(0);

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn RfuRecvQueue_Reset(queue: *mut RfuRecvQueue) {
    for i in 0..RECV_QUEUE_NUM_SLOTS {
        for j in 0..70i32 {
            (*queue).slots[i][j] = 0;
        }
    }
    volatile_write(&raw mut (*queue).sendSlot, 0);
    volatile_write(&raw mut (*queue).recvSlot, 0);
    volatile_write(&raw mut (*queue).count, 0);
    volatile_write(&raw mut (*queue).full, FALSE);
}
pub unsafe fn RfuSendQueue_Reset(queue: *mut RfuSendQueue) {
    for i in 0..SEND_QUEUE_NUM_SLOTS {
        for j in 0..COMM_SLOT_LENGTH {
            (*queue).slots[i][j] = 0;
        }
    }
    volatile_write(&raw mut (*queue).sendSlot, 0);
    volatile_write(&raw mut (*queue).recvSlot, 0);
    volatile_write(&raw mut (*queue).count, 0);
    volatile_write(&raw mut (*queue).full, FALSE);
}
unsafe fn RfuUnusedQueue_Reset(queue: *mut RfuUnusedQueue) {
    for i in 0..UNUSED_QUEUE_NUM_SLOTS {
        for j in 0..UNUSED_QUEUE_SLOT_LENGTH {
            (*queue).slots[i][j] = 0;
        }
    }
    volatile_write(&raw mut (*queue).sendSlot, 0);
    volatile_write(&raw mut (*queue).recvSlot, 0);
    volatile_write(&raw mut (*queue).count, 0);
    volatile_write(&raw mut (*queue).full, FALSE);
}
pub unsafe fn RfuRecvQueue_Enqueue(queue: *mut RfuRecvQueue, data: *mut u8) {
    let mut i: i32 = 0;
    let mut imeBak: u16 = 0;
    let mut count: u8 = 0;
    if (&raw mut (*queue).count).read_volatile() < RECV_QUEUE_NUM_SLOTS as u8 {
        imeBak = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        count = 0;
        i = 0;
        while i < 70 {
            if *data.at(i) == 0 && *data.at(i + 1) == 0 {
                count += 1;
            }
            i += COMM_SLOT_LENGTH;
        }
        if count != MAX_RFU_PLAYERS as u8 {
            for i in 0..70i32 {
                (*queue).slots[(&raw mut (*queue).recvSlot).read_volatile()][i] = *data.at(i);
            }
            volatile_write(
                &raw mut (*queue).recvSlot,
                (&raw mut (*queue).recvSlot).read_volatile() + 1,
            );
            volatile_write(
                &raw mut (*queue).recvSlot,
                ((&raw mut (*queue).recvSlot).read_volatile() as i32 % 32) as u8,
            );
            volatile_write(
                &raw mut (*queue).count,
                (&raw mut (*queue).count).read_volatile() + 1,
            );
            for i in 0..70i32 {
                *data.at(i) = 0;
            }
        }
        volatile_write(67109384_usize as *mut u16, imeBak);
    } else {
        volatile_write(&raw mut (*queue).full, TRUE);
    }
}
pub unsafe fn RfuSendQueue_Enqueue(queue: *mut RfuSendQueue, data: *mut u8) {
    let mut i: i32 = 0;
    let mut imeBak: u16 = 0;
    if (&raw mut (*queue).count).read_volatile() < SEND_QUEUE_NUM_SLOTS as u8 {
        imeBak = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        i = 0;
        while i < COMM_SLOT_LENGTH {
            if *data.at(i) != 0 {
                break;
            }
            i += 1;
        }
        if i != COMM_SLOT_LENGTH {
            i = 0;
            while i < COMM_SLOT_LENGTH {
                (*queue).slots[(&raw mut (*queue).recvSlot).read_volatile()][i] = *data.at(i);
                i += 1;
            }
            volatile_write(
                &raw mut (*queue).recvSlot,
                (&raw mut (*queue).recvSlot).read_volatile() + 1,
            );
            volatile_write(
                &raw mut (*queue).recvSlot,
                ((&raw mut (*queue).recvSlot).read_volatile() as i32 % 40) as u8,
            );
            volatile_write(
                &raw mut (*queue).count,
                (&raw mut (*queue).count).read_volatile() + 1,
            );
            for i in 0..COMM_SLOT_LENGTH {
                *data.at(i) = 0;
            }
        }
        volatile_write(67109384_usize as *mut u16, imeBak);
    } else {
        volatile_write(&raw mut (*queue).full, TRUE);
    }
}
pub unsafe fn RfuRecvQueue_Dequeue(queue: *mut RfuRecvQueue, src: *mut u8) -> u8 {
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    if (&raw mut (*queue).recvSlot).read_volatile() == (&raw mut (*queue).sendSlot).read_volatile()
        || (&raw mut (*queue).full).read_volatile() != 0
    {
        for i in 0..70i32 {
            *src.at(i) = 0;
        }
        volatile_write(67109384_usize as *mut u16, imeBak);
        return FALSE;
    }
    for i in 0..70i32 {
        *src.at(i) = (*queue).slots[(&raw mut (*queue).sendSlot).read_volatile()][i];
    }
    volatile_write(
        &raw mut (*queue).sendSlot,
        (&raw mut (*queue).sendSlot).read_volatile() + 1,
    );
    volatile_write(
        &raw mut (*queue).sendSlot,
        ((&raw mut (*queue).sendSlot).read_volatile() as i32 % 32) as u8,
    );
    volatile_write(
        &raw mut (*queue).count,
        (&raw mut (*queue).count).read_volatile() - 1,
    );
    volatile_write(67109384_usize as *mut u16, imeBak);
    TRUE
}
pub unsafe fn RfuSendQueue_Dequeue(queue: *mut RfuSendQueue, src: *mut u8) -> u8 {
    if (&raw mut (*queue).recvSlot).read_volatile() == (&raw mut (*queue).sendSlot).read_volatile()
        || (&raw mut (*queue).full).read_volatile() != 0
    {
        return FALSE;
    }
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    for i in 0..COMM_SLOT_LENGTH {
        *src.at(i) = (*queue).slots[(&raw mut (*queue).sendSlot).read_volatile()][i];
    }
    volatile_write(
        &raw mut (*queue).sendSlot,
        (&raw mut (*queue).sendSlot).read_volatile() + 1,
    );
    volatile_write(
        &raw mut (*queue).sendSlot,
        ((&raw mut (*queue).sendSlot).read_volatile() as i32 % 40) as u8,
    );
    volatile_write(
        &raw mut (*queue).count,
        (&raw mut (*queue).count).read_volatile() - 1,
    );
    volatile_write(67109384_usize as *mut u16, imeBak);
    TRUE
}
pub unsafe fn RfuBackupQueue_Enqueue(queue: *mut RfuBackupQueue, data: *mut u8) {
    if *data.at(1) == 0 {
        RfuBackupQueue_Dequeue(queue, null_mut());
    } else {
        for i in 0..COMM_SLOT_LENGTH {
            (*queue).slots[(&raw mut (*queue).recvSlot).read_volatile()][i] = *data.at(i);
        }
        volatile_write(
            &raw mut (*queue).recvSlot,
            (&raw mut (*queue).recvSlot).read_volatile() + 1,
        );
        volatile_write(
            &raw mut (*queue).recvSlot,
            ((&raw mut (*queue).recvSlot).read_volatile() as i32 % 2) as u8,
        );
        if (&raw mut (*queue).count).read_volatile() < BACKUP_QUEUE_NUM_SLOTS {
            volatile_write(
                &raw mut (*queue).count,
                (&raw mut (*queue).count).read_volatile() + 1,
            );
        } else {
            volatile_write(
                &raw mut (*queue).sendSlot,
                (&raw mut (*queue).recvSlot).read_volatile(),
            );
        }
    }
}
pub unsafe fn RfuBackupQueue_Dequeue(queue: *mut RfuBackupQueue, src: *mut u8) -> u8 {
    if (&raw mut (*queue).count).read_volatile() == 0 {
        return FALSE;
    }
    if !src.is_null() {
        for i in 0..COMM_SLOT_LENGTH {
            *src.at(i) = (*queue).slots[(&raw mut (*queue).sendSlot).read_volatile()][i];
        }
    }
    volatile_write(
        &raw mut (*queue).sendSlot,
        (&raw mut (*queue).sendSlot).read_volatile() + 1,
    );
    volatile_write(
        &raw mut (*queue).sendSlot,
        ((&raw mut (*queue).sendSlot).read_volatile() as i32 % 2) as u8,
    );
    volatile_write(
        &raw mut (*queue).count,
        (&raw mut (*queue).count).read_volatile() - 1,
    );
    TRUE
}
unsafe fn RfuUnusedQueue_Enqueue(queue: *mut RfuUnusedQueue, data: *mut u8) {
    if (&raw mut (*queue).count).read_volatile() < UNUSED_QUEUE_NUM_SLOTS as u8 {
        for i in 0..UNUSED_QUEUE_SLOT_LENGTH {
            (*queue).slots[(&raw mut (*queue).recvSlot).read_volatile()][i] = *data.at(i);
        }
        volatile_write(
            &raw mut (*queue).recvSlot,
            (&raw mut (*queue).recvSlot).read_volatile() + 1,
        );
        volatile_write(
            &raw mut (*queue).recvSlot,
            ((&raw mut (*queue).recvSlot).read_volatile() as i32 % 2) as u8,
        );
        volatile_write(
            &raw mut (*queue).count,
            (&raw mut (*queue).count).read_volatile() + 1,
        );
    } else {
        volatile_write(&raw mut (*queue).full, TRUE);
    }
}
unsafe fn RfuUnusedQueue_Dequeue(queue: *mut RfuUnusedQueue, dest: *mut u8) -> u8 {
    if (&raw mut (*queue).recvSlot).read_volatile() == (&raw mut (*queue).sendSlot).read_volatile()
        || (&raw mut (*queue).full).read_volatile() != 0
    {
        return FALSE;
    }
    for i in 0..UNUSED_QUEUE_SLOT_LENGTH {
        *dest.at(i) = (*queue).slots[(&raw mut (*queue).sendSlot).read_volatile()][i];
    }
    volatile_write(
        &raw mut (*queue).sendSlot,
        (&raw mut (*queue).sendSlot).read_volatile() + 1,
    );
    volatile_write(
        &raw mut (*queue).sendSlot,
        ((&raw mut (*queue).sendSlot).read_volatile() as i32 % 2) as u8,
    );
    volatile_write(
        &raw mut (*queue).count,
        (&raw mut (*queue).count).read_volatile() - 1,
    );
    TRUE
}
unsafe fn PopulateArrayWithSequence(arr: *mut u8, mode: u8) {
    let mut i: i32 = 0;
    let mut rval: u8 = 0;
    let mut total: u16 = 0;
    match mode {
        0 => {
            i = 0;
            while i < SEQ_ARRAY_MAX_SIZE {
                *arr.at(i) = i as u8 + 1;
                total += i as u16 + 1;
                i += 1;
            }
            *(arr.at(i) as *mut u16) = total;
        }
        1 => {
            for i in 0..100i32 {
                *arr.at(i) = i as u8 + 1;
                total += i as u16 + 1;
            }
            *(arr.at(200) as *mut u16) = total;
        }
        2 => {
            i = 0;
            while i < SEQ_ARRAY_MAX_SIZE {
                rval = Random() as u8;
                *arr.at(i) = rval;
                total += rval as u16;
                i += 1;
            }
            *(arr.at(i) as *mut u16) = total;
        }
        3 => {
            i = 0;
            while i < SEQ_ARRAY_MAX_SIZE {
                *arr.at(i) = i as u8 + 1 + sSequenceArrayValOffset.get();
                total += (i as u16 + 1 + sSequenceArrayValOffset.get() as u16) & 0xFF;
                i += 1;
            }
            *(arr.at(i) as *mut u16) = total;
            sSequenceArrayValOffset.set(sSequenceArrayValOffset.get() + 1);
        }
        _ => {}
    }
}
unsafe fn PkmnStrToASCII(asciiStr: *mut u8, pkmnStr: *mut u8) {
    let mut i: i32 = 0;
    while *pkmnStr.at(i) != EOS {
        *asciiStr.at(i) = sWireless_RSEtoASCIITable[*pkmnStr.at(i)];
        i += 1;
    }
    *asciiStr.at(i) = 0;
}
unsafe fn ASCIIToPkmnStr(pkmnStr: *mut u8, asciiStr: *mut u8) {
    let mut i: i32 = 0;
    while *asciiStr.at(i) != 0 {
        *pkmnStr.at(i) = sWireless_ASCIItoRSETable[*asciiStr.at(i)];
        i += 1;
    }
    *pkmnStr.at(i) = EOS;
}
unsafe fn GetConnectedChildStrength(maxFlags: u8) -> u8 {
    let mut flagCount: u8 = 0;
    let mut flags: u32 = (*crate::gl!(gRfuLinkStatus)).connSlotFlag as u32;
    if (*crate::gl!(gRfuLinkStatus)).parentChild == MODE_PARENT {
        for i in 0..4u8 {
            if flags & 1 != 0 {
                if maxFlags as i32 == flagCount as i32 + 1 {
                    return (*crate::gl!(gRfuLinkStatus)).strength[i];
                    break;
                }
                flagCount += 1;
            }
            flags >>= 1;
        }
    } else {
        for i in 0..4u8 {
            if flags & 1 != 0 {
                return (*crate::gl!(gRfuLinkStatus)).strength[i];
            }
            flags >>= 1;
        }
    }
    0
}
pub unsafe fn InitHostRfuGameData(
    data: *mut RfuGameData,
    activity: u8,
    startedActivity: u32,
    mut partnerInfo: i32,
) {
    let mut i: i32 = 0;
    while i < 2 {
        (*data).compatibility.playerTrainerId[i] = (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[i];
        i += 1;
    }
    for i in 0..(RFU_CHILD_MAX as i32) {
        (*data).partnerInfo[i] = partnerInfo as u8;
        partnerInfo >>= 8;
    }
    (*data).set_playerGender((*crate::gl!(gSaveBlock2Ptr)).playerGender);
    (*data).set_activity(activity);
    (*data).set_startedActivity(startedActivity as u8);
    (*data).compatibility.set_language(GAME_LANGUAGE as u16);
    (*data).compatibility.set_version(GAME_VERSION as u16);
    (*data).compatibility.set_hasNews(FALSE as u16);
    (*data).compatibility.set_hasCard(FALSE as u16);
    (*data).compatibility.set_unknown(FALSE as u16);
    (*data)
        .compatibility
        .set_canLinkNationally(FlagGet(FLAG_IS_CHAMPION) as u16);
    (*data)
        .compatibility
        .set_hasNationalDex(IsNationalPokedexEnabled() as u16);
    (*data)
        .compatibility
        .set_gameClear(FlagGet(FLAG_SYS_GAME_CLEAR) as u16);
}
pub unsafe fn Rfu_GetCompatiblePlayerData(
    gameData: *mut RfuGameData,
    username: *mut u8,
    idx: u8,
) -> u8 {
    let mut retVal: u8 = 0;
    if crate::gl!(lman.parent_child) == MODE_PARENT {
        retVal = TRUE;
        if IsRfuSerialNumberValid((*crate::gl!(gRfuLinkStatus)).partner[idx].serialNo as u32) != 0
            && shr_i32((*crate::gl!(gRfuLinkStatus)).getNameFlag as i32, idx as u32) & 1 != 0
        {
            memcpy(
                gameData as *mut u8,
                (*crate::gl!(gRfuLinkStatus)).partner[idx]
                    .gname
                    .as_mut_ptr(),
                RFU_GAME_NAME_LENGTH,
            );
            memcpy(
                username,
                (*crate::gl!(gRfuLinkStatus)).partner[idx]
                    .uname
                    .as_mut_ptr(),
                RFU_USER_NAME_LENGTH,
            );
        } else {
            memset(gameData as *mut u8, 0, RFU_GAME_NAME_LENGTH);
            memset(username, 0, RFU_USER_NAME_LENGTH);
        }
    } else {
        retVal = FALSE;
        if IsRfuSerialNumberValid((*crate::gl!(gRfuLinkStatus)).partner[idx].serialNo as u32) != 0 {
            memcpy(
                gameData as *mut u8,
                (*crate::gl!(gRfuLinkStatus)).partner[idx]
                    .gname
                    .as_mut_ptr(),
                RFU_GAME_NAME_LENGTH,
            );
            memcpy(
                username,
                (*crate::gl!(gRfuLinkStatus)).partner[idx]
                    .uname
                    .as_mut_ptr(),
                RFU_USER_NAME_LENGTH,
            );
        } else {
            memset(gameData as *mut u8, 0, RFU_GAME_NAME_LENGTH);
            memset(username, 0, RFU_USER_NAME_LENGTH);
        }
    }
    retVal
}
pub unsafe fn Rfu_GetWonderDistributorPlayerData(
    gameData: *mut RfuGameData,
    username: *mut u8,
    idx: u8,
) -> u8 {
    let mut retVal: u8 = FALSE;
    if (*crate::gl!(gRfuLinkStatus)).partner[idx].serialNo == RFU_SERIAL_WONDER_DISTRIBUTOR {
        memcpy(
            gameData as *mut u8,
            (*crate::gl!(gRfuLinkStatus)).partner[idx]
                .gname
                .as_mut_ptr(),
            RFU_GAME_NAME_LENGTH,
        );
        memcpy(
            username,
            (*crate::gl!(gRfuLinkStatus)).partner[idx]
                .uname
                .as_mut_ptr(),
            RFU_USER_NAME_LENGTH,
        );
        retVal = TRUE;
    } else {
        memset(gameData as *mut u8, 0, RFU_GAME_NAME_LENGTH);
        memset(username, 0, RFU_USER_NAME_LENGTH);
    }
    retVal
}
pub unsafe fn CopyHostRfuGameDataAndUsername(gameData: *mut RfuGameData, username: *mut u8) {
    memcpy(
        gameData as *mut u8,
        &raw mut (*(crate::gl!(&raw @at crate::link_rfu_2::gHostRfuGameData;).cast_const())
            .cast::<RfuGameData>()
            .cast_mut()) as *mut u8,
        RFU_GAME_NAME_LENGTH,
    );
    memcpy(
        username,
        (*gHostRfuUsername.as_ptr()).as_mut_ptr(),
        RFU_USER_NAME_LENGTH,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn CreateWirelessStatusIndicatorSprite(mut x: u8, mut y: u8) {
    let mut sprId: u8 = 0;
    if x == 0 && y == 0 {
        x = 231;
        y = 8;
    }
    if (*crate::gl!(gRfuLinkStatus)).parentChild == MODE_PARENT {
        sprId = CreateSprite(
            (&raw const *sWirelessStatusIndicatorSpriteTemplate).cast_mut(),
            x as i16,
            y as i16,
            0,
        );
        {
            let gl_i0 = sValidator;
            crate::gl!(gSprites[sprId].data[gl_i0] = STATUS_INDICATOR_ACTIVE)
        };
        {
            let gl_v = GetSpriteTileStartByTag(sWirelessStatusIndicatorSpriteSheet.tag) as i16;
            let gl_i1 = sTileStart;
            crate::gl!(gSprites[sprId].data[gl_i1] = gl_v)
        };
        (*gSprites.as_ptr())[sprId].set_invisible(TRUE as u16);
        gWirelessStatusIndicatorSpriteId.set(sprId);
    } else {
        gWirelessStatusIndicatorSpriteId.set(CreateSprite(
            (&raw const *sWirelessStatusIndicatorSpriteTemplate).cast_mut(),
            x as i16,
            y as i16,
            0,
        ));
        {
            let gl_i2 = gWirelessStatusIndicatorSpriteId.get();
            let gl_i3 = sValidator;
            crate::gl!(gSprites[gl_i2].data[gl_i3] = STATUS_INDICATOR_ACTIVE)
        };
        {
            let gl_v = GetSpriteTileStartByTag(sWirelessStatusIndicatorSpriteSheet.tag) as i16;
            let gl_i4 = gWirelessStatusIndicatorSpriteId.get();
            let gl_i5 = sTileStart;
            crate::gl!(gSprites[gl_i4].data[gl_i5] = gl_v)
        };
        (*gSprites.as_ptr())[gWirelessStatusIndicatorSpriteId.get()].set_invisible(TRUE as u16);
    }
}
pub unsafe fn DestroyWirelessStatusIndicatorSprite() {
    if ({
        let gl_i6 = gWirelessStatusIndicatorSpriteId.get();
        let gl_i7 = sValidator;
        crate::gl!(gSprites[gl_i6].data[gl_i7])
    }) == STATUS_INDICATOR_ACTIVE
    {
        {
            let gl_i8 = gWirelessStatusIndicatorSpriteId.get();
            let gl_i9 = sValidator;
            crate::gl!(gSprites[gl_i8].data[gl_i9] = 0)
        };
        DestroySprite({
            let gl_i10 = gWirelessStatusIndicatorSpriteId.get();
            crate::gl!(&raw gSprites[gl_i10])
        });
        crate::gl!(
            gMain.oamBuffer[125] = *(&raw const crate::sprite::gDummyOamData).cast::<OamData>()
        );
        CpuSet(
            (&raw const (*(&raw const crate::sprite::gDummyOamData).cast::<OamData>())).cast_mut()
                as *mut c_void,
            (OAM as i32 as usize as *mut OamData).at(125) as *mut c_void,
            4,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LoadWirelessStatusIndicatorSpriteGfx() {
    if GetSpriteTileStartByTag(sWirelessStatusIndicatorSpriteSheet.tag) == 0xFFFF {
        LoadCompressedSpriteSheet((&raw const *sWirelessStatusIndicatorSpriteSheet).cast_mut());
    }
    LoadSpritePalette((&raw const *sWirelessStatusIndicatorSpritePalette).cast_mut());
    gWirelessStatusIndicatorSpriteId.set(SPRITE_NONE);
}
unsafe fn GetParentSignalStrength() -> u8 {
    let mut flags: u8 = (*crate::gl!(gRfuLinkStatus)).connSlotFlag;
    for i in 0..RFU_CHILD_MAX {
        if flags as i32 & 1 != 0 {
            return (*crate::gl!(gRfuLinkStatus)).strength[i];
        }
        flags >>= 1;
    }
    0
}
unsafe fn SetWirelessStatusIndicatorAnim(sprite: *mut Sprite, animNum: i32) {
    if (*sprite).data[sCurrAnimNum] as i32 != animNum {
        (*sprite).data[sCurrAnimNum] = animNum as i16;
        (*sprite).data[sFrameDelay] = 0;
        (*sprite).data[sFrameIdx] = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateWirelessStatusIndicatorSprite() {
    if gWirelessStatusIndicatorSpriteId.get() != SPRITE_NONE
        && ({
            let gl_i11 = gWirelessStatusIndicatorSpriteId.get();
            let gl_i12 = sValidator;
            crate::gl!(gSprites[gl_i11].data[gl_i12])
        }) == STATUS_INDICATOR_ACTIVE
    {
        let sprite: *mut Sprite = {
            let gl_i13 = gWirelessStatusIndicatorSpriteId.get();
            crate::gl!(&raw gSprites[gl_i13])
        };
        let mut signalStrength: u8 = RFU_LINK_ICON_LEVEL4_MAX;
        let mut i: u8 = 0;
        if (*crate::gl!(gRfuLinkStatus)).parentChild == MODE_PARENT {
            i = 0;
            while (i as i32) < GetLinkPlayerCount() as i32 - 1 {
                if signalStrength >= GetConnectedChildStrength(i + 1) {
                    signalStrength = GetConnectedChildStrength(i + 1);
                }
                i += 1;
            }
        } else {
            signalStrength = GetParentSignalStrength();
        }
        if IsRfuRecoveringFromLinkLoss() == TRUE {
            (*sprite).data[sNextAnimNum] = WIRELESS_STATUS_ANIM_ERROR;
        } else if signalStrength <= RFU_LINK_ICON_LEVEL1_MAX {
            (*sprite).data[sNextAnimNum] = WIRELESS_STATUS_ANIM_SEARCHING;
        } else if (RFU_LINK_ICON_LEVEL2_MIN..=RFU_LINK_ICON_LEVEL2_MAX).contains(&signalStrength) {
            (*sprite).data[sNextAnimNum] = WIRELESS_STATUS_ANIM_1_BAR;
        } else if (RFU_LINK_ICON_LEVEL3_MIN..=RFU_LINK_ICON_LEVEL3_MAX).contains(&signalStrength) {
            (*sprite).data[sNextAnimNum] = WIRELESS_STATUS_ANIM_2_BARS;
        } else if signalStrength >= RFU_LINK_ICON_LEVEL4_MIN {
            (*sprite).data[sNextAnimNum] = WIRELESS_STATUS_ANIM_3_BARS;
        }
        if (*sprite).data[sNextAnimNum] != (*sprite).data[sSavedAnimNum] {
            SetWirelessStatusIndicatorAnim(sprite, (*sprite).data[sNextAnimNum] as i32);
            (*sprite).data[sSavedAnimNum] = (*sprite).data[sNextAnimNum];
        }
        if (*(*(*sprite).anims.at((*sprite).data[sCurrAnimNum])).at((*sprite).data[sFrameIdx]))
            .frame
            .duration()
            < (*sprite).data[sFrameDelay] as u32
        {
            (*sprite).data[sFrameIdx] += 1;
            (*sprite).data[sFrameDelay] = 0;
            if (*(*(*sprite).anims.at((*sprite).data[sCurrAnimNum])).at((*sprite).data[sFrameIdx]))
                .r#type
                == -2
            {
                (*sprite).data[sFrameIdx] = 0;
            }
        } else {
            (*sprite).data[sFrameDelay] += 1;
        }
        crate::gl!(gMain.oamBuffer[125] = *sWirelessStatusIndicatorOamData);
        (*gMain.as_ptr()).oamBuffer[125]
            .set_x((*sprite).x as u32 + (*sprite).centerToCornerVecX as u32);
        (*gMain.as_ptr()).oamBuffer[125]
            .set_y((*sprite).y as u32 + (*sprite).centerToCornerVecY as u32);
        (*gMain.as_ptr()).oamBuffer[125].set_paletteNum((*sprite).oam.paletteNum());
        (*gMain.as_ptr()).oamBuffer[125].set_tileNum(
            (*sprite).data[sTileStart] as u16
                + (*(*(*sprite).anims.at((*sprite).data[sCurrAnimNum]))
                    .at((*sprite).data[sFrameIdx]))
                .frame
                .imageValue() as u16,
        );
        CpuSet(
            crate::gl!(&raw gMain.oamBuffer[125]) as *mut c_void,
            (OAM as i32 as usize as *mut OamData).at(125) as *mut c_void,
            4,
        );
        if RfuGetStatus() == RFU_STATUS_FATAL_ERROR {
            DestroyWirelessStatusIndicatorSprite();
        }
    }
}
unsafe fn CopyTrainerRecord(dest: *mut TrainerNameRecord, trainerId: u32, name: *mut u8) {
    (*dest).trainerId = trainerId;
    StringCopy((*dest).trainerName.as_mut_ptr(), name);
}
unsafe fn NameIsNotEmpty(name: *mut u8) -> u32 {
    for i in 0..8i32 {
        if *name.at(i) != 0 {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn SaveLinkTrainerNames() {
    if gWirelessCommType.get() != 0 {
        let mut connectedTrainerRecordIndices: CArray<i32, 5> = zeroed();
        let newRecords: *mut TrainerNameRecord = AllocZeroed(240) as *mut TrainerNameRecord;
        let mut i: i32 = 0;
        while i < GetLinkPlayerCount() as i32 {
            connectedTrainerRecordIndices[i] = -1;
            for j in 0..20i32 {
                if crate::gl!(gLinkPlayers[i].trainerId) as u16 as u32
                    == (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[j].trainerId
                    && StringCompare(
                        (*gLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
                        (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[j]
                            .trainerName
                            .as_mut_ptr(),
                    ) == 0
                {
                    connectedTrainerRecordIndices[i] = j;
                }
            }
            i += 1;
        }
        let mut nextSpace: i32 = 0;
        i = 0;
        while i < GetLinkPlayerCount() as i32 {
            if i != GetMultiplayerId() as i32
                && crate::gl!(gLinkPlayers[i].language) != LANGUAGE_JAPANESE as u16
            {
                CopyTrainerRecord(
                    newRecords.at(nextSpace),
                    crate::gl!(gLinkPlayers[i].trainerId) as u16 as u32,
                    (*gLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
                );
                if connectedTrainerRecordIndices[i] >= 0 {
                    memset(
                        (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords
                            [connectedTrainerRecordIndices[i]]
                            .trainerName
                            .as_mut_ptr(),
                        0,
                        8,
                    );
                }
                nextSpace += 1;
            }
            i += 1;
        }
        for i in 0..20i32 {
            if NameIsNotEmpty(
                (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i]
                    .trainerName
                    .as_mut_ptr(),
            ) != 0
            {
                CopyTrainerRecord(
                    newRecords.at(nextSpace),
                    (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i].trainerId,
                    (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i]
                        .trainerName
                        .as_mut_ptr(),
                );
                if ({
                    nextSpace += 1;
                    nextSpace
                }) >= 20
                {
                    break;
                }
            }
        }
        memcpy(
            (*crate::gl!(gSaveBlock1Ptr))
                .trainerNameRecords
                .as_mut_ptr() as *mut u8,
            newRecords as *mut u8,
            240,
        );
        Free(newRecords as *mut c_void);
    }
}
pub unsafe fn PlayerHasMetTrainerBefore(id: u16, name: *mut u8) -> u32 {
    for i in 0..20i32 {
        if StringCompare(
            (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i]
                .trainerName
                .as_mut_ptr(),
            name,
        ) == 0
            && (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i].trainerId == id as u32
        {
            return TRUE as u32;
        }
        if NameIsNotEmpty(
            (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i]
                .trainerName
                .as_mut_ptr(),
        ) == 0
        {
            return FALSE as u32;
        }
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn WipeTrainerNameRecords() {
    for i in 0..20i32 {
        (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i].trainerId = 0;
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    (*crate::gl!(gSaveBlock1Ptr)).trainerNameRecords[i]
                        .trainerName
                        .as_mut_ptr() as *mut c_void,
                    0x1000004,
                );
            }
        }
    }
}
