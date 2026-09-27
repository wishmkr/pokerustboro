//! Translated from `src/ereader_helpers.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sTrainerHillTrainerTemplates_JP
#[allow(unused_imports)]
use crate::data::ereader_helpers::*;

pub(crate) static mut sSendRecvMgr: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
pub(crate) static mut sJoyNewOrRepeated: u16 = 0u16;
pub(crate) static mut sJoyNew: u16 = 0u16;
pub(crate) static mut sSendRecvStatus: u16 = 0u16;
pub(crate) static mut sCounter1: u16 = 0u16;
pub(crate) static mut sCounter2: u32 = 0u32;
pub(crate) static mut sSavedIme: u16 = 0u16;
pub(crate) static mut sSavedIe: u16 = 0u16;
pub(crate) static mut sSavedTm3Cnt: u16 = 0u16;
pub(crate) static mut sSavedSioCnt: u16 = 0u16;
pub(crate) static mut sSavedRCnt: u16 = 0u16;

unsafe extern "C" {
    static mut gSaveBlock1Ptr: u8;
    static mut gShouldAdvanceLinkState: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CalcByteArraySum(a0: *mut u8, a1: u32) -> u32;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn Free(a0: *mut u8);
    fn TryReadSpecialSaveSector(a0: u8, a1: *mut u8) -> u32;
    fn TryWriteSpecialSaveSector(a0: u8, a1: *mut u8) -> u32;
    fn VBlankIntrWait();
}

pub(crate) unsafe extern "C" fn GetTrainerHillUnkVal() -> u8 {
    unsafe {
        return ((crate::c::rem_i32(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15716))
                .wrapping_add(9))
            .read()) as i32)
                .wrapping_add(1i32),
            256i32,
        )) as u8);
    }
}
pub(crate) unsafe extern "C" fn ValidateTrainerChecksum(hillTrainer: *mut u8) -> u32 {
    unsafe {
        let mut hillTrainer = hillTrainer;
        let mut checksum: i32 = ((CalcByteArraySum(hillTrainer, 624u32)) as i32);
        if ((checksum) as u32) != ((hillTrainer).wrapping_add(624).cast::<u32>()).read() {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateTrainerHillData(hillSet: *mut u8) -> u8 {
    unsafe {
        let mut hillSet = hillSet;
        let mut i: u32 = 0u32;
        let mut checksum: u32 = 0u32;
        let mut numTrainers: i32 = (((hillSet).read()) as i32);
        if (numTrainers < 1i32) || (numTrainers > 8i32) {
            return 0u8;
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < ((numTrainers) as u32)) {
                    break 'l1;
                }
                'l2: {
                    if !((ValidateTrainerChecksum(
                        (((hillSet).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 628),
                    )) != 0)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        checksum = CalcByteArraySum(
            ((hillSet).wrapping_add(8)).cast::<u8>(),
            ((numTrainers) as u32).wrapping_mul(628u32),
        );
        if checksum != ((hillSet).wrapping_add(4).cast::<u32>()).read() {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ValidateTrainerHillChecksum(hillSet: *mut u8) -> u32 {
    unsafe {
        let mut hillSet = hillSet;
        let mut checksum: u32 = 0u32;
        let mut numTrainers: i32 = (((hillSet).read()) as i32);
        if (numTrainers < 1i32) || (numTrainers > 8i32) {
            return 0u32;
        }
        checksum = CalcByteArraySum(((hillSet).wrapping_add(8)).cast::<u8>(), 3808u32);
        if checksum != ((hillSet).wrapping_add(4).cast::<u32>()).read() {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn TryWriteTrainerHill_Internal(
    hillSet: *mut u8,
    challenge: *mut u8,
) -> u32 {
    unsafe {
        let mut hillSet = hillSet;
        let mut challenge = challenge;
        let mut i: i32 = 0i32;
        crate::c::memset(challenge, 0i32, 4096u32);
        (challenge).write((hillSet).read());
        ((challenge).wrapping_add(1)).write(GetTrainerHillUnkVal());
        ((challenge).wrapping_add(2)).write(
            ((crate::c::div_i32((((hillSet).read()) as i32).wrapping_add(1i32), 2i32)) as u8),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < (((hillSet).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((i & 1i32) != 0) {
                        ((((challenge).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 952))
                        .write(
                            ((((hillSet).wrapping_add(8)).cast::<u8>())
                                .wrapping_offset((i) as isize * 628))
                            .read(),
                        );
                        ((((challenge).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 952))
                        .wrapping_add(660)
                        .cast::<crate::c::Rec4<292>>()
                        .write_unaligned(
                            ((((hillSet).wrapping_add(8)).cast::<u8>())
                                .wrapping_offset((i) as isize * 628))
                            .wrapping_add(332)
                            .cast::<crate::c::Rec4<292>>()
                            .read_unaligned(),
                        );
                        (((((challenge).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 952))
                        .wrapping_add(4))
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<328>>()
                        .write_unaligned(
                            ((((hillSet).wrapping_add(8)).cast::<u8>())
                                .wrapping_offset((i) as isize * 628))
                            .wrapping_add(4)
                            .cast::<crate::c::Rec4<328>>()
                            .read_unaligned(),
                        );
                    } else {
                        (((((challenge).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 952))
                        .wrapping_add(1))
                        .write(
                            ((((hillSet).wrapping_add(8)).cast::<u8>())
                                .wrapping_offset((i) as isize * 628))
                            .read(),
                        );
                        ((((((challenge).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 952))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(328)
                        .cast::<crate::c::Rec4<328>>()
                        .write_unaligned(
                            ((((hillSet).wrapping_add(8)).cast::<u8>())
                                .wrapping_offset((i) as isize * 628))
                            .wrapping_add(4)
                            .cast::<crate::c::Rec4<328>>()
                            .read_unaligned(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (i & 1i32) != 0 {
            ((((((challenge).wrapping_add(8)).cast::<u8>())
                .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 952))
            .wrapping_add(4))
            .cast::<u8>())
            .wrapping_offset(328)
            .cast::<crate::c::Rec4<328>>()
            .write_unaligned(
                (((&raw const sTrainerHillTrainerTemplates_JP)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((crate::c::div_i32(i, 2i32)) as isize * 328)
                .cast::<crate::c::Rec4<328>>()
                .read_unaligned(),
            );
        }
        ((challenge).wrapping_add(4).cast::<u32>()).write(CalcByteArraySum(
            ((challenge).wrapping_add(8)).cast::<u8>(),
            3808u32,
        ));
        if TryWriteSpecialSaveSector(30u8, challenge) != 1u32 {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryWriteTrainerHill(hillSet: *mut u8) -> u32 {
    unsafe {
        let mut hillSet = hillSet;
        let mut buffer: *mut u8 = AllocZeroed(4096u32);
        let mut result: u32 = TryWriteTrainerHill_Internal(hillSet, buffer);
        Free(buffer);
        return result;
    }
}
pub(crate) unsafe extern "C" fn TryReadTrainerHill_Internal(dest: *mut u8, buffer: *mut u8) -> u32 {
    unsafe {
        let mut dest = dest;
        let mut buffer = buffer;
        if TryReadSpecialSaveSector(30u8, buffer) != 1u32 {
            return 0u32;
        }
        crate::c::memcpy(dest, buffer, 3816u32);
        if !((ValidateTrainerHillChecksum(dest)) != 0) {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn TryReadTrainerHill(hillSet: *mut u8) -> u32 {
    unsafe {
        let mut hillSet = hillSet;
        let mut buffer: *mut u8 = AllocZeroed(4096u32);
        let mut result: u32 = TryReadTrainerHill_Internal(hillSet, buffer);
        Free(buffer);
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadTrainerHillAndValidate() -> u32 {
    unsafe {
        let mut hillSet: *mut u8 = AllocZeroed(4096u32);
        let mut result: u32 = TryReadTrainerHill(hillSet);
        Free(hillSet);
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReader_Send(size: i32, src: *mut u8) -> i32 {
    unsafe {
        let mut size = size;
        let mut src = src;
        let mut result: i32 = 0i32;
        let mut sendStatus: i32 = 0i32;
        EReaderHelper_SaveRegsState();
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            GetKeyInput();
            if (((((&raw mut sJoyNew).cast::<u8>().cast::<u16>()).read()) as i32) & 2i32) != 0 {
                ((&raw mut gShouldAdvanceLinkState).cast::<u8>()).write(2u8);
            }
            sendStatus = EReaderHandleTransfer(1u8, ((size) as u32), src, core::ptr::null_mut());
            ((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).write(((sendStatus) as u16));
            if ((((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32) & 3i32)
                == 0i32)
                && ((((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32)
                    & 16i32)
                    != 0)
            {
                result = 0i32;
                break 'l1;
            } else {
                if (((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32)
                    & 8i32)
                    != 0
                {
                    result = 1i32;
                    break 'l1;
                } else {
                    if (((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32)
                        & 4i32)
                        != 0
                    {
                        result = 2i32;
                        break 'l1;
                    } else {
                        ((&raw mut gShouldAdvanceLinkState).cast::<u8>()).write(0u8);
                        VBlankIntrWait();
                    }
                }
            }
        }
        'l2: loop {
            'l3: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l4: loop {
                        'l5: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut sSendRecvMgr).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        24u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l4;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l2;
            }
        }
        EReaderHelper_RestoreRegsState();
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReader_Recv(dest: *mut u8) -> i32 {
    unsafe {
        let mut dest = dest;
        let mut result: i32 = 0i32;
        let mut recvStatus: i32 = 0i32;
        EReaderHelper_SaveRegsState();
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            GetKeyInput();
            if (((((&raw mut sJoyNew).cast::<u8>().cast::<u16>()).read()) as i32) & 2i32) != 0 {
                ((&raw mut gShouldAdvanceLinkState).cast::<u8>()).write(2u8);
            }
            recvStatus = EReaderHandleTransfer(0u8, 0u32, core::ptr::null_mut(), dest);
            ((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).write(((recvStatus) as u16));
            if ((((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32) & 3i32)
                == 0i32)
                && ((((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32)
                    & 16i32)
                    != 0)
            {
                result = 0i32;
                break 'l1;
            } else {
                if (((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32)
                    & 8i32)
                    != 0
                {
                    result = 1i32;
                    break 'l1;
                } else {
                    if (((((&raw mut sSendRecvStatus).cast::<u8>().cast::<u16>()).read()) as i32)
                        & 4i32)
                        != 0
                    {
                        result = 2i32;
                        break 'l1;
                    } else {
                        ((&raw mut gShouldAdvanceLinkState).cast::<u8>()).write(0u8);
                        VBlankIntrWait();
                    }
                }
            }
        }
        'l2: loop {
            'l3: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l4: loop {
                        'l5: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut sSendRecvMgr).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        24u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l4;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l2;
            }
        }
        EReaderHelper_RestoreRegsState();
        return result;
    }
}
pub(crate) unsafe extern "C" fn CloseSerial() {
    unsafe {
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32) & (-193i32)) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109134i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109378i32) as usize as *mut u16), 192u16);
    }
}
pub(crate) unsafe extern "C" fn OpenSerialMulti() {
    unsafe {
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32) & (-193i32)) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 8192u16);
        let __p2 = ((67109160i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p2,
            (((((__p2).read_volatile()) as i32) | 16387i32) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        let __p3 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(__p3, (((((__p3).read_volatile()) as i32) | 128i32) as u16));
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
        if (((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).read()) as i32) == 0i32 {
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u32 = 0u32;
                        (&raw mut tmp).write_volatile(0u32);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (&raw mut sSendRecvMgr).cast::<u8>(),
                                    (83886080u32
                                        | (crate::c::div_u32(
                                            24u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn OpenSerial32() {
    unsafe {
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20480u16);
        let __p1 = ((67109160i32) as usize as *mut u16);
        crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 8i32) as u16));
        ((&raw mut gShouldAdvanceLinkState).cast::<u8>()).write(0u8);
        ((&raw mut sCounter1).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sCounter2).cast::<u8>().cast::<u32>()).write(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReaderHandleTransfer(
    mode: u8,
    size: u32,
    data: *mut u8,
    recvBuffer: *mut u8,
) -> i32 {
    unsafe {
        let mut mode = mode;
        let mut size = size;
        let mut data = data;
        let mut recvBuffer = recvBuffer;
        'l1: {
            let __sw1 = (((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                OpenSerialMulti();
                (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).write(1u8);
                (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (DetermineSendRecvState(mode)) != 0 {
                    EnableSio();
                }
                if ((((&raw mut gShouldAdvanceLinkState).cast::<u8>()).read()) as i32) == 2i32 {
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(4)).write(2u8);
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                OpenSerial32();
                SetUpTransferManager(size, data, recvBuffer);
                (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(3u8);
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if ((((&raw mut gShouldAdvanceLinkState).cast::<u8>()).read()) as i32) == 2i32 {
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(4)).write(2u8);
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(6u8);
                } else {
                    let __p2 = (&raw mut sCounter1).cast::<u8>().cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    let __p3 = (&raw mut sCounter2).cast::<u8>().cast::<u32>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    if (!((((&raw mut sSendRecvMgr).cast::<u8>()).read()) != 0))
                        && (((&raw mut sCounter2).cast::<u8>().cast::<u32>()).read() > 60u32)
                    {
                        (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(4)).write(1u8);
                        (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(6u8);
                    }
                    if (((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).read()) as i32)
                        != 2i32
                    {
                        if ((((&raw mut sSendRecvMgr).cast::<u8>()).read()) != 0)
                            && (((((&raw mut sCounter1).cast::<u8>().cast::<u16>()).read()) as i32)
                                > 2i32)
                        {
                            EnableSio();
                            (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).write(2u8);
                        } else {
                            EnableSio();
                            (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).write(2u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                OpenSerialMulti();
                (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (((((&raw mut sSendRecvMgr).cast::<u8>()).read()) as i32) == 1i32)
                    && (((((&raw mut sCounter1).cast::<u8>().cast::<u16>()).read()) as i32) > 2i32)
                {
                    EnableSio();
                }
                if (({
                    let __p4 = (&raw mut sCounter1).cast::<u8>().cast::<u16>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 60i32
                {
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(4)).write(1u8);
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if ((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).read()) != 0 {
                    CloseSerial();
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).write(0u8);
                }
                break 'l1;
            }
        }
        return ((((((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(2)).read()) as i32)
            << 0)
            | ((((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(4)).read()) as i32) << 2))
            | ((((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(3)).read()) as i32) << 4));
    }
}
pub(crate) unsafe extern "C" fn DetermineSendRecvState(mode: u8) -> u16 {
    unsafe {
        let mut mode = mode;
        let mut resp: u16 = 0u16;
        if ((((67109160i32) as usize as *mut u32).read_volatile() & 12u32) == 8u32) && ((mode) != 0)
        {
            resp = (({
                let __v1 = 1u8;
                ((&raw mut sSendRecvMgr).cast::<u8>()).write(__v1);
                __v1
            }) as u16);
        } else {
            resp = (({
                let __v2 = 0u8;
                ((&raw mut sSendRecvMgr).cast::<u8>()).write(__v2);
                __v2
            }) as u16);
        }
        return resp;
    }
}
pub(crate) unsafe extern "C" fn SetUpTransferManager(
    size: u32,
    data: *mut u8,
    recvBuffer: *mut u8,
) {
    unsafe {
        let mut size = size;
        let mut data = data;
        let mut recvBuffer = recvBuffer;
        if (((&raw mut sSendRecvMgr).cast::<u8>()).read()) != 0 {
            let __p1 = ((67109160i32) as usize as *mut u16);
            crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 1i32) as u16));
            (((&raw mut sSendRecvMgr).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u32>())
            .write((data).cast::<u32>());
            crate::c::volatile_write(((67109152i32) as usize as *mut u32), size);
            (((&raw mut sSendRecvMgr).cast::<u8>())
                .wrapping_add(16)
                .cast::<i32>())
            .write((((crate::c::div_u32(size, 4u32)).wrapping_add(1u32)) as i32));
            StartTm3();
        } else {
            crate::c::volatile_write(
                ((67109160i32) as usize as *mut u16),
                ((67109160i32) as usize as *mut u16).read_volatile(),
            );
            (((&raw mut sSendRecvMgr).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u32>())
            .write((recvBuffer).cast::<u32>());
        }
    }
}
pub(crate) unsafe extern "C" fn StartTm3() {
    unsafe {
        crate::c::volatile_write(((67109132i32) as usize as *mut u16), 64935u16);
        crate::c::volatile_write(((67109134i32) as usize as *mut u16), 64u16);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 64i32) as u16));
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReaderHelper_Timer3Callback() {
    unsafe {
        DisableTm3();
        EnableSio();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReaderHelper_SerialCallback() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut cnt1: u16 = 0u16;
        let mut cnt2: u16 = 0u16;
        let mut recv32: u32 = 0u32;
        let mut recv = crate::ffi::Align4([0u8; 8]);
        'l1: {
            let __sw1 = (((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).read()) as i32);
            if __sw1 == 1i32 {
                crate::c::volatile_write(((67109162i32) as usize as *mut u16), 52432u16);
                (((&raw mut recv).cast::<u16>()).cast::<u64>())
                    .write(((67109152i32) as usize as *mut u64).read_volatile());
                {
                    i = 0u16;
                    cnt1 = 0u16;
                    cnt2 = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (((((&raw mut recv).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                == 52432i32
                            {
                                cnt1 = (cnt1).wrapping_add(1);
                            } else {
                                if (((((&raw mut recv).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != 65535i32
                                {
                                    cnt2 = (cnt2).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (((cnt1) as i32) == 2i32) && (((cnt2) as i32) == 0i32) {
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                recv32 = ((67109152i32) as usize as *mut u32).read_volatile();
                if (!(((((&raw mut sSendRecvMgr).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<i32>())
                .read())
                    != 0))
                    && (!((((&raw mut sSendRecvMgr).cast::<u8>()).read()) != 0))
                {
                    (((&raw mut sSendRecvMgr).cast::<u8>())
                        .wrapping_add(16)
                        .cast::<i32>())
                    .write((((crate::c::div_u32(recv32, 4u32)).wrapping_add(1u32)) as i32));
                }
                if ((((&raw mut sSendRecvMgr).cast::<u8>()).read()) as i32) == 1i32 {
                    if (((&raw mut sSendRecvMgr).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<i32>())
                    .read()
                        < (((&raw mut sSendRecvMgr).cast::<u8>())
                            .wrapping_add(16)
                            .cast::<i32>())
                        .read()
                    {
                        crate::c::volatile_write(
                            ((67109152i32) as usize as *mut u32),
                            (((((&raw mut sSendRecvMgr).cast::<u8>())
                                .wrapping_add(8)
                                .cast::<*mut u32>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut sSendRecvMgr).cast::<u8>())
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                .read()) as isize,
                            ))
                            .read(),
                        );
                        let __p2 = ((&raw mut sSendRecvMgr).cast::<u8>())
                            .wrapping_add(20)
                            .cast::<i32>();
                        (__p2).write(
                            (((((__p2).read()) as u32).wrapping_add(
                                (((((&raw mut sSendRecvMgr).cast::<u8>())
                                    .wrapping_add(8)
                                    .cast::<*mut u32>())
                                .read())
                                .wrapping_offset(
                                    ((((&raw mut sSendRecvMgr).cast::<u8>())
                                        .wrapping_add(12)
                                        .cast::<i32>())
                                    .read()) as isize,
                                ))
                                .read(),
                            )) as i32),
                        );
                    } else {
                        crate::c::volatile_write(
                            ((67109152i32) as usize as *mut u32),
                            (((((&raw mut sSendRecvMgr).cast::<u8>())
                                .wrapping_add(20)
                                .cast::<i32>())
                            .read()) as u32),
                        );
                    }
                } else {
                    if ((((&raw mut sSendRecvMgr).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<i32>())
                    .read()
                        > 0i32)
                        && ((((&raw mut sSendRecvMgr).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<i32>())
                        .read()
                            < ((((&raw mut sSendRecvMgr).cast::<u8>())
                                .wrapping_add(16)
                                .cast::<i32>())
                            .read())
                            .wrapping_add(1i32))
                    {
                        (((((&raw mut sSendRecvMgr).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<*mut u32>())
                        .read())
                        .wrapping_offset(
                            (((((&raw mut sSendRecvMgr).cast::<u8>())
                                .wrapping_add(12)
                                .cast::<i32>())
                            .read())
                            .wrapping_sub(1i32)) as isize,
                        ))
                        .write(recv32);
                        let __p3 = ((&raw mut sSendRecvMgr).cast::<u8>())
                            .wrapping_add(20)
                            .cast::<i32>();
                        (__p3).write((((((__p3).read()) as u32).wrapping_add(recv32)) as i32));
                    } else {
                        if ((((&raw mut sSendRecvMgr).cast::<u8>())
                            .wrapping_add(12)
                            .cast::<i32>())
                        .read())
                            != 0
                        {
                            if (((((&raw mut sSendRecvMgr).cast::<u8>())
                                .wrapping_add(20)
                                .cast::<i32>())
                            .read()) as u32)
                                == recv32
                            {
                                (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(3)).write(1u8);
                            } else {
                                (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(3)).write(2u8);
                            }
                        }
                    }
                    ((&raw mut sCounter2).cast::<u8>().cast::<u32>()).write(0u32);
                }
                if {
                    let __p4 = ((&raw mut sSendRecvMgr).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<i32>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                } < ((((&raw mut sSendRecvMgr).cast::<u8>())
                    .wrapping_add(16)
                    .cast::<i32>())
                .read())
                .wrapping_add(2i32)
                {
                    if (((&raw mut sSendRecvMgr).cast::<u8>()).read()) != 0 {
                        let __p6 = ((67109134i32) as usize as *mut u16);
                        crate::c::volatile_write(
                            __p6,
                            (((((__p6).read_volatile()) as i32) | 128i32) as u16),
                        );
                    } else {
                        EnableSio();
                    }
                } else {
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(4u8);
                    ((&raw mut sCounter1).cast::<u8>().cast::<u16>()).write(0u16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((&raw mut sSendRecvMgr).cast::<u8>()).read()) != 0) {
                    crate::c::volatile_write(
                        ((67109162i32) as usize as *mut u16),
                        (((((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(3)).read()) as u16),
                    );
                }
                crate::c::volatile_write(
                    ((&raw mut recv).cast::<u16>()).cast::<u64>(),
                    ((67109152i32) as usize as *mut u64).read_volatile(),
                );
                if ((((((&raw mut recv).cast::<u16>()).wrapping_offset(1)).read()) as i32) == 1i32)
                    || ((((((&raw mut recv).cast::<u16>()).wrapping_offset(1)).read()) as i32)
                        == 2i32)
                {
                    if ((((&raw mut sSendRecvMgr).cast::<u8>()).read()) as i32) == 1i32 {
                        (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(3)).write(
                            (((((&raw mut recv).cast::<u16>()).wrapping_offset(1)).read()) as u8),
                        );
                    }
                    (((&raw mut sSendRecvMgr).cast::<u8>()).wrapping_add(1)).write(6u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EnableSio() {
    unsafe {
        let __p1 = ((67109160i32) as usize as *mut u16);
        crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 128i32) as u16));
    }
}
pub(crate) unsafe extern "C" fn DisableTm3() {
    unsafe {
        let __p1 = ((67109134i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32) & (-129i32)) as u16),
        );
        crate::c::volatile_write(((67109132i32) as usize as *mut u16), 64935u16);
    }
}
pub(crate) unsafe extern "C" fn GetKeyInput() {
    unsafe {
        let mut rawKeys: i32 =
            (((((67109168i32) as usize as *mut u16).read_volatile()) as i32) ^ 1023i32);
        ((&raw mut sJoyNew).cast::<u8>().cast::<u16>()).write(
            ((rawKeys
                & !((((&raw mut sJoyNewOrRepeated).cast::<u8>().cast::<u16>()).read()) as i32))
                as u16),
        );
        ((&raw mut sJoyNewOrRepeated).cast::<u8>().cast::<u16>()).write(((rawKeys) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReaderHelper_SaveRegsState() {
    unsafe {
        ((&raw mut sSavedIme).cast::<u8>().cast::<u16>())
            .write(((67109384i32) as usize as *mut u16).read_volatile());
        ((&raw mut sSavedIe).cast::<u8>().cast::<u16>())
            .write(((67109376i32) as usize as *mut u16).read_volatile());
        ((&raw mut sSavedTm3Cnt).cast::<u8>().cast::<u16>())
            .write(((67109134i32) as usize as *mut u16).read_volatile());
        ((&raw mut sSavedSioCnt).cast::<u8>().cast::<u16>())
            .write(((67109160i32) as usize as *mut u16).read_volatile());
        ((&raw mut sSavedRCnt).cast::<u8>().cast::<u16>())
            .write(((67109172i32) as usize as *mut u16).read_volatile());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReaderHelper_RestoreRegsState() {
    unsafe {
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            ((&raw mut sSavedIme).cast::<u8>().cast::<u16>()).read(),
        );
        crate::c::volatile_write(
            ((67109376i32) as usize as *mut u16),
            ((&raw mut sSavedIe).cast::<u8>().cast::<u16>()).read(),
        );
        crate::c::volatile_write(
            ((67109134i32) as usize as *mut u16),
            ((&raw mut sSavedTm3Cnt).cast::<u8>().cast::<u16>()).read(),
        );
        crate::c::volatile_write(
            ((67109160i32) as usize as *mut u16),
            ((&raw mut sSavedSioCnt).cast::<u8>().cast::<u16>()).read(),
        );
        crate::c::volatile_write(
            ((67109172i32) as usize as *mut u16),
            ((&raw mut sSavedRCnt).cast::<u8>().cast::<u16>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EReaderHelper_ClearSendRecvMgr() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut sSendRecvMgr).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        24u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
