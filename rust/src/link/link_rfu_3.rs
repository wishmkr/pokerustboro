//! Translated from `src/link_rfu_3.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWirelessLinkIconPalette sWirelessLinkIconPic sWireless_ASCIItoRSETable sWireless_RSEtoASCIITable sWirelessStatusIndicatorOamData sWirelessStatusIndicator_3Bars sWirelessStatusIndicator_2Bars sWirelessStatusIndicator_1Bar sWirelessStatusIndicator_Searching sWirelessStatusIndicator_Error sWirelessStatusIndicatorAnims sWirelessStatusIndicatorSpriteSheet sWirelessStatusIndicatorSpritePalette sWirelessStatusIndicatorSpriteTemplate
#[allow(unused_imports)]
use crate::data::link_rfu_3::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWirelessStatusIndicatorSpriteId: u8 = 0u8;
pub(crate) static mut sSequenceArrayValOffset: u8 = 0u8;

unsafe extern "C" {
    static mut gDummyOamData: u8;
    static mut gHostRfuGameData: u8;
    static mut gHostRfuUsername: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gRfuLinkStatus: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    static mut gWirelessCommType: u8;
    static mut lman: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn GetLinkPlayerCount() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsRfuRecoveringFromLinkLoss() -> u8;
    fn IsRfuSerialNumberValid(a0: u32) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn Random() -> u16;
    fn RfuGetStatus() -> u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuRecvQueue_Reset(queue: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 32i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 70i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((queue).cast::<u8>()).wrapping_offset((i) as isize * 70))
                                    .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write((queue).wrapping_add(2241), 0u8);
        crate::c::volatile_write((queue).wrapping_add(2240), 0u8);
        crate::c::volatile_write((queue).wrapping_add(2242), 0u8);
        crate::c::volatile_write((queue).wrapping_add(2243), 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSendQueue_Reset(queue: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 40i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 14i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((queue).cast::<u8>()).wrapping_offset((i) as isize * 14))
                                    .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write((queue).wrapping_add(561), 0u8);
        crate::c::volatile_write((queue).wrapping_add(560), 0u8);
        crate::c::volatile_write((queue).wrapping_add(562), 0u8);
        crate::c::volatile_write((queue).wrapping_add(563), 0u8);
    }
}
pub(crate) unsafe extern "C" fn RfuUnusedQueue_Reset(queue: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 256i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((queue).cast::<u8>()).wrapping_offset((i) as isize * 256))
                                    .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write((queue).wrapping_add(513), 0u8);
        crate::c::volatile_write((queue).wrapping_add(512), 0u8);
        crate::c::volatile_write((queue).wrapping_add(514), 0u8);
        crate::c::volatile_write((queue).wrapping_add(515), 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuRecvQueue_Enqueue(queue: *mut u8, data: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut data = data;
        let mut i: i32 = 0i32;
        let mut imeBak: u16 = 0u16;
        let mut count: u8 = 0u8;
        if ((((queue).wrapping_add(2242)).read_volatile()) as i32) < 32i32 {
            imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            count = 0u8;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 70i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((data).wrapping_offset((i) as isize)).read()) as i32) == 0i32)
                            && (((((data).wrapping_offset(((i).wrapping_add(1i32)) as isize))
                                .read()) as i32)
                                == 0i32)
                        {
                            count = (count).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(14i32);
                }
            }
            if ((count) as i32) != 5i32 {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 70i32) {
                            break 'l3;
                        }
                        'l4: {
                            (((((queue).cast::<u8>()).wrapping_offset(
                                ((((queue).wrapping_add(2240)).read_volatile()) as i32) as isize
                                    * 70,
                            ))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(((data).wrapping_offset((i) as isize)).read());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p1 = (queue).wrapping_add(2240);
                crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
                let __p2 = (queue).wrapping_add(2240);
                crate::c::volatile_write(
                    __p2,
                    ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 32i32)) as u8),
                );
                let __p3 = (queue).wrapping_add(2242);
                crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < 70i32) {
                            break 'l5;
                        }
                        'l6: {
                            ((data).wrapping_offset((i) as isize)).write(0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        } else {
            crate::c::volatile_write((queue).wrapping_add(2243), 1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSendQueue_Enqueue(queue: *mut u8, data: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut data = data;
        let mut i: i32 = 0i32;
        let mut imeBak: u16 = 0u16;
        if ((((queue).wrapping_add(562)).read_volatile()) as i32) < 40i32 {
            imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 14i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((data).wrapping_offset((i) as isize)).read()) as i32) != 0i32 {
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if i != 14i32 {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 14i32) {
                            break 'l3;
                        }
                        'l4: {
                            (((((queue).cast::<u8>()).wrapping_offset(
                                ((((queue).wrapping_add(560)).read_volatile()) as i32) as isize
                                    * 14,
                            ))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(((data).wrapping_offset((i) as isize)).read());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p1 = (queue).wrapping_add(560);
                crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
                let __p2 = (queue).wrapping_add(560);
                crate::c::volatile_write(
                    __p2,
                    ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 40i32)) as u8),
                );
                let __p3 = (queue).wrapping_add(562);
                crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < 14i32) {
                            break 'l5;
                        }
                        'l6: {
                            ((data).wrapping_offset((i) as isize)).write(0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        } else {
            crate::c::volatile_write((queue).wrapping_add(563), 1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuRecvQueue_Dequeue(queue: *mut u8, src: *mut u8) -> u8 {
    unsafe {
        let mut queue = queue;
        let mut src = src;
        let mut imeBak: u16 = 0u16;
        let mut i: i32 = 0i32;
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        if (((((queue).wrapping_add(2240)).read_volatile()) as i32)
            == ((((queue).wrapping_add(2241)).read_volatile()) as i32))
            || ((((queue).wrapping_add(2243)).read_volatile()) != 0)
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 70i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((src).wrapping_offset((i) as isize)).write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
            return 0u8;
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 70i32) {
                    break 'l3;
                }
                'l4: {
                    ((src).wrapping_offset((i) as isize)).write(
                        (((((queue).cast::<u8>()).wrapping_offset(
                            ((((queue).wrapping_add(2241)).read_volatile()) as i32) as isize * 70,
                        ))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (queue).wrapping_add(2241);
        crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
        let __p2 = (queue).wrapping_add(2241);
        crate::c::volatile_write(
            __p2,
            ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 32i32)) as u8),
        );
        let __p3 = (queue).wrapping_add(2242);
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_sub(1));
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSendQueue_Dequeue(queue: *mut u8, src: *mut u8) -> u8 {
    unsafe {
        let mut queue = queue;
        let mut src = src;
        let mut i: i32 = 0i32;
        let mut imeBak: u16 = 0u16;
        if (((((queue).wrapping_add(560)).read_volatile()) as i32)
            == ((((queue).wrapping_add(561)).read_volatile()) as i32))
            || ((((queue).wrapping_add(563)).read_volatile()) != 0)
        {
            return 0u8;
        }
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    ((src).wrapping_offset((i) as isize)).write(
                        (((((queue).cast::<u8>()).wrapping_offset(
                            ((((queue).wrapping_add(561)).read_volatile()) as i32) as isize * 14,
                        ))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (queue).wrapping_add(561);
        crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
        let __p2 = (queue).wrapping_add(561);
        crate::c::volatile_write(
            __p2,
            ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 40i32)) as u8),
        );
        let __p3 = (queue).wrapping_add(562);
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_sub(1));
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuBackupQueue_Enqueue(queue: *mut u8, data: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut data = data;
        let mut i: i32 = 0i32;
        if ((((data).wrapping_offset(1)).read()) as i32) == 0i32 {
            RfuBackupQueue_Dequeue(queue, core::ptr::null_mut());
        } else {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 14i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((queue).cast::<u8>()).wrapping_offset(
                            ((((queue).wrapping_add(28)).read_volatile()) as i32) as isize * 14,
                        ))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(((data).wrapping_offset((i) as isize)).read());
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p1 = (queue).wrapping_add(28);
            crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
            let __p2 = (queue).wrapping_add(28);
            crate::c::volatile_write(
                __p2,
                ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 2i32)) as u8),
            );
            if ((((queue).wrapping_add(30)).read_volatile()) as i32) < 2i32 {
                let __p3 = (queue).wrapping_add(30);
                crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
            } else {
                crate::c::volatile_write(
                    (queue).wrapping_add(29),
                    ((queue).wrapping_add(28)).read_volatile(),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuBackupQueue_Dequeue(queue: *mut u8, src: *mut u8) -> u8 {
    unsafe {
        let mut queue = queue;
        let mut src = src;
        let mut i: i32 = 0i32;
        if ((((queue).wrapping_add(30)).read_volatile()) as i32) == 0i32 {
            return 0u8;
        }
        if ((src) as usize) != 0usize {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 14i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((src).wrapping_offset((i) as isize)).write(
                            (((((queue).cast::<u8>()).wrapping_offset(
                                ((((queue).wrapping_add(29)).read_volatile()) as i32) as isize * 14,
                            ))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        let __p1 = (queue).wrapping_add(29);
        crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
        let __p2 = (queue).wrapping_add(29);
        crate::c::volatile_write(
            __p2,
            ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 2i32)) as u8),
        );
        let __p3 = (queue).wrapping_add(30);
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_sub(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn RfuUnusedQueue_Enqueue(queue: *mut u8, data: *mut u8) {
    unsafe {
        let mut queue = queue;
        let mut data = data;
        let mut i: i32 = 0i32;
        if ((((queue).wrapping_add(514)).read_volatile()) as i32) < 2i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 256i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((queue).cast::<u8>()).wrapping_offset(
                            ((((queue).wrapping_add(512)).read_volatile()) as i32) as isize * 256,
                        ))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(((data).wrapping_offset((i) as isize)).read());
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p1 = (queue).wrapping_add(512);
            crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
            let __p2 = (queue).wrapping_add(512);
            crate::c::volatile_write(
                __p2,
                ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 2i32)) as u8),
            );
            let __p3 = (queue).wrapping_add(514);
            crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        } else {
            crate::c::volatile_write((queue).wrapping_add(515), 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn RfuUnusedQueue_Dequeue(queue: *mut u8, dest: *mut u8) -> u8 {
    unsafe {
        let mut queue = queue;
        let mut dest = dest;
        let mut i: i32 = 0i32;
        if (((((queue).wrapping_add(512)).read_volatile()) as i32)
            == ((((queue).wrapping_add(513)).read_volatile()) as i32))
            || ((((queue).wrapping_add(515)).read_volatile()) != 0)
        {
            return 0u8;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 256i32) {
                    break 'l1;
                }
                'l2: {
                    ((dest).wrapping_offset((i) as isize)).write(
                        (((((queue).cast::<u8>()).wrapping_offset(
                            ((((queue).wrapping_add(513)).read_volatile()) as i32) as isize * 256,
                        ))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (queue).wrapping_add(513);
        crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
        let __p2 = (queue).wrapping_add(513);
        crate::c::volatile_write(
            __p2,
            ((crate::c::rem_i32((((__p2).read_volatile()) as i32), 2i32)) as u8),
        );
        let __p3 = (queue).wrapping_add(514);
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_sub(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn PopulateArrayWithSequence(arr: *mut u8, mode: u8) {
    unsafe {
        let mut arr = arr;
        let mut mode = mode;
        let mut i: i32 = 0i32;
        let mut rval: u8 = 0u8;
        let mut total: u16 = 0u16;
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 200i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((arr).wrapping_offset((i) as isize))
                                .write((((i).wrapping_add(1i32)) as u8));
                            total =
                                ((((total) as i32).wrapping_add((i).wrapping_add(1i32))) as u16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((arr).wrapping_offset((i) as isize)).cast::<u16>()).write(total);
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 100i32) {
                            break 'l4;
                        }
                        'l5: {
                            ((arr).wrapping_offset((i) as isize))
                                .write((((i).wrapping_add(1i32)) as u8));
                            total =
                                ((((total) as i32).wrapping_add((i).wrapping_add(1i32))) as u16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((arr).wrapping_offset(200)).cast::<u16>()).write(total);
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 200i32) {
                            break 'l6;
                        }
                        'l7: {
                            rval = ((Random()) as u8);
                            ((arr).wrapping_offset((i) as isize)).write(rval);
                            total = ((((total) as i32).wrapping_add(((rval) as i32))) as u16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((arr).wrapping_offset((i) as isize)).cast::<u16>()).write(total);
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i < 200i32) {
                            break 'l8;
                        }
                        'l9: {
                            ((arr).wrapping_offset((i) as isize)).write(
                                ((((i).wrapping_add(1i32)).wrapping_add(
                                    ((((&raw mut sSequenceArrayValOffset)
                                        .cast::<u8>()
                                        .cast::<u8>())
                                    .read()) as i32),
                                )) as u8),
                            );
                            total = ((((total) as i32).wrapping_add(
                                (((i).wrapping_add(1i32)).wrapping_add(
                                    ((((&raw mut sSequenceArrayValOffset)
                                        .cast::<u8>()
                                        .cast::<u8>())
                                    .read()) as i32),
                                ) & 255i32),
                            )) as u16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((arr).wrapping_offset((i) as isize)).cast::<u16>()).write(total);
                let __p2 = (&raw mut sSequenceArrayValOffset).cast::<u8>().cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PkmnStrToASCII(asciiStr: *mut u8, pkmnStr: *mut u8) {
    unsafe {
        let mut asciiStr = asciiStr;
        let mut pkmnStr = pkmnStr;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((pkmnStr).wrapping_offset((i) as isize)).read()) as i32) != 255i32) {
                    break 'l1;
                }
                'l2: {
                    ((asciiStr).wrapping_offset((i) as isize)).write(
                        ((((&raw const sWireless_RSEtoASCIITable)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((pkmnStr).wrapping_offset((i) as isize)).read()) as i32) as isize,
                        ))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((asciiStr).wrapping_offset((i) as isize)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn ASCIIToPkmnStr(pkmnStr: *mut u8, asciiStr: *mut u8) {
    unsafe {
        let mut pkmnStr = pkmnStr;
        let mut asciiStr = asciiStr;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((asciiStr).wrapping_offset((i) as isize)).read()) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {
                    ((pkmnStr).wrapping_offset((i) as isize)).write(
                        ((((&raw const sWireless_ASCIItoRSETable)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((asciiStr).wrapping_offset((i) as isize)).read()) as i32) as isize,
                        ))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((pkmnStr).wrapping_offset((i) as isize)).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn GetConnectedChildStrength(maxFlags: u8) -> u8 {
    unsafe {
        let mut maxFlags = maxFlags;
        let mut flagCount: u8 = 0u8;
        let mut flags: u32 = ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read()) as u32);
        let mut i: u8 = 0u8;
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 1i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (flags & 1u32) != 0 {
                            if ((maxFlags) as i32) == ((flagCount) as i32).wrapping_add(1i32) {
                                return ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(10))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read();
                                break 'l1;
                            }
                            flagCount = (flagCount).wrapping_add(1);
                        }
                    }
                    flags = (flags >> 1);
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (flags & 1u32) != 0 {
                            return ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                        }
                    }
                    flags = (flags >> 1);
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitHostRfuGameData(
    data: *mut u8,
    activity: u8,
    startedActivity: u32,
    partnerInfo: i32,
) {
    unsafe {
        let mut data = data;
        let mut activity = activity;
        let mut startedActivity = startedActivity;
        let mut partnerInfo = partnerInfo;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(2u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((data).wrapping_add(2)).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
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
                    ((((data).wrapping_add(4)).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(((partnerInfo) as u8));
                    partnerInfo = (partnerInfo >> 8);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (data).wrapping_add(11),
            0,
            1,
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32,
        );
        crate::c::bf_write((data).wrapping_add(10), 0, 7, (activity) as i32);
        crate::c::bf_write(
            (data).wrapping_add(10),
            7,
            1,
            ((startedActivity) as u8) as i32,
        );
        crate::c::bf_write((data).wrapping_add(0), 0, 4, (2u16) as i32);
        crate::c::bf_write((data).wrapping_add(1), 2, 4, (3u16) as i32);
        crate::c::bf_write((data).wrapping_add(0), 4, 1, (0u16) as i32);
        crate::c::bf_write((data).wrapping_add(0), 5, 1, (0u16) as i32);
        crate::c::bf_write((data).wrapping_add(0), 6, 1, (0u16) as i32);
        crate::c::bf_write(
            (data).wrapping_add(0),
            7,
            1,
            ((FlagGet(2175u16)) as u16) as i32,
        );
        crate::c::bf_write(
            (data).wrapping_add(1),
            0,
            1,
            ((IsNationalPokedexEnabled()) as u16) as i32,
        );
        crate::c::bf_write(
            (data).wrapping_add(1),
            1,
            1,
            ((FlagGet(2148u16)) as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_GetCompatiblePlayerData(
    gameData: *mut u8,
    username: *mut u8,
    idx: u8,
) -> u8 {
    unsafe {
        let mut gameData = gameData;
        let mut username = username;
        let mut idx = idx;
        let mut retVal: u8 = 0u8;
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(6)).read()) as i32) == 1i32 {
            retVal = 1u8;
            if ((IsRfuSerialNumberValid(
                (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                    .cast::<u8>())
                .wrapping_offset(((idx) as i32) as isize * 32))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as u32),
            )) != 0)
                && ((crate::c::shr_i32(
                    ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                        .read()) as i32),
                    ((idx) as u32),
                ) & 1i32)
                    != 0)
            {
                crate::c::memcpy(
                    gameData,
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 32))
                    .wrapping_add(6))
                    .cast::<u8>(),
                    13u32,
                );
                crate::c::memcpy(
                    username,
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 32))
                    .wrapping_add(21))
                    .cast::<u8>(),
                    8u32,
                );
            } else {
                crate::c::memset(gameData, 0i32, 13u32);
                crate::c::memset(username, 0i32, 8u32);
            }
        } else {
            retVal = 0u8;
            if (IsRfuSerialNumberValid(
                (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                    .cast::<u8>())
                .wrapping_offset(((idx) as i32) as isize * 32))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as u32),
            )) != 0
            {
                crate::c::memcpy(
                    gameData,
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 32))
                    .wrapping_add(6))
                    .cast::<u8>(),
                    13u32,
                );
                crate::c::memcpy(
                    username,
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((idx) as i32) as isize * 32))
                    .wrapping_add(21))
                    .cast::<u8>(),
                    8u32,
                );
            } else {
                crate::c::memset(gameData, 0i32, 13u32);
                crate::c::memset(username, 0i32, 8u32);
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_GetWonderDistributorPlayerData(
    gameData: *mut u8,
    username: *mut u8,
    idx: u8,
) -> u8 {
    unsafe {
        let mut gameData = gameData;
        let mut username = username;
        let mut idx = idx;
        let mut retVal: u8 = 0u8;
        if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
            .cast::<u8>())
        .wrapping_offset(((idx) as i32) as isize * 32))
        .wrapping_add(4)
        .cast::<u16>())
        .read()) as i32)
            == 32637i32
        {
            crate::c::memcpy(
                gameData,
                (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                    .cast::<u8>())
                .wrapping_offset(((idx) as i32) as isize * 32))
                .wrapping_add(6))
                .cast::<u8>(),
                13u32,
            );
            crate::c::memcpy(
                username,
                (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                    .cast::<u8>())
                .wrapping_offset(((idx) as i32) as isize * 32))
                .wrapping_add(21))
                .cast::<u8>(),
                8u32,
            );
            retVal = 1u8;
        } else {
            crate::c::memset(gameData, 0i32, 13u32);
            crate::c::memset(username, 0i32, 8u32);
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyHostRfuGameDataAndUsername(gameData: *mut u8, username: *mut u8) {
    unsafe {
        let mut gameData = gameData;
        let mut username = username;
        crate::c::memcpy(gameData, (&raw mut gHostRfuGameData).cast::<u8>(), 13u32);
        crate::c::memcpy(username, (&raw mut gHostRfuUsername).cast::<u8>(), 8u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateWirelessStatusIndicatorSprite(x: u8, y: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut sprId: u8 = 0u8;
        if (((x) as i32) == 0i32) && (((y) as i32) == 0i32) {
            x = 231u8;
            y = 8u8;
        }
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 1i32 {
            sprId = CreateSprite(
                (&raw const sWirelessStatusIndicatorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((x) as i16),
                ((y) as i16),
                0u8,
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((sprId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(4660i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((sprId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(
                ((GetSpriteTileStartByTag(
                    (((&raw const sWirelessStatusIndicatorSpriteSheet)
                        .cast::<u8>()
                        .cast_mut())
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read(),
                )) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((sprId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            ((&raw mut gWirelessStatusIndicatorSpriteId)
                .cast::<u8>()
                .cast::<u8>())
            .write(sprId);
        } else {
            ((&raw mut gWirelessStatusIndicatorSpriteId)
                .cast::<u8>()
                .cast::<u8>())
            .write(CreateSprite(
                (&raw const sWirelessStatusIndicatorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((x) as i16),
                ((y) as i16),
                0u8,
            ));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut gWirelessStatusIndicatorSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(4660i16);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut gWirelessStatusIndicatorSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(
                ((GetSpriteTileStartByTag(
                    (((&raw const sWirelessStatusIndicatorSpriteSheet)
                        .cast::<u8>()
                        .cast_mut())
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read(),
                )) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gWirelessStatusIndicatorSpriteId)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyWirelessStatusIndicatorSprite() {
    unsafe {
        if ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut gWirelessStatusIndicatorSpriteId)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read()) as i32)
            == 4660i32
        {
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut gWirelessStatusIndicatorSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(0i16);
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gWirelessStatusIndicatorSpriteId)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                .wrapping_offset(1000)
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    (&raw mut gDummyOamData)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<8>>()
                        .read_unaligned(),
                );
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut gDummyOamData).cast::<u8>(),
                                ((117440512i32) as usize as *mut u8).wrapping_offset(1000),
                                (0u32
                                    | (crate::c::div_u32(
                                        8u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadWirelessStatusIndicatorSpriteGfx() {
    unsafe {
        if ((GetSpriteTileStartByTag(
            (((&raw const sWirelessStatusIndicatorSpriteSheet)
                .cast::<u8>()
                .cast_mut())
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        )) as i32)
            == 65535i32
        {
            LoadCompressedSpriteSheet(
                (&raw const sWirelessStatusIndicatorSpriteSheet)
                    .cast::<u8>()
                    .cast_mut(),
            );
        }
        LoadSpritePalette(
            (&raw const sWirelessStatusIndicatorSpritePalette)
                .cast::<u8>()
                .cast_mut(),
        );
        ((&raw mut gWirelessStatusIndicatorSpriteId)
            .cast::<u8>()
            .cast::<u8>())
        .write(255u8);
    }
}
pub(crate) unsafe extern "C" fn GetParentSignalStrength() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut flags: u8 =
            ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((flags) as i32) & 1i32) != 0 {
                        return ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                    }
                    flags = ((((flags) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetWirelessStatusIndicatorAnim(sprite: *mut u8, animNum: i32) {
    unsafe {
        let mut sprite = sprite;
        let mut animNum = animNum;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            != animNum
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((animNum) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateWirelessStatusIndicatorSprite() {
    unsafe {
        if (((((&raw mut gWirelessStatusIndicatorSpriteId)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            != 255i32)
            && (((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut gWirelessStatusIndicatorSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .read()) as i32)
                == 4660i32)
        {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut gWirelessStatusIndicatorSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            );
            let mut signalStrength: u8 = 255u8;
            let mut i: u8 = 0u8;
            if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 1i32 {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < ((GetLinkPlayerCount()) as i32).wrapping_sub(1i32)) {
                            break 'l1;
                        }
                        'l2: {
                            if ((signalStrength) as i32)
                                >= ((GetConnectedChildStrength(
                                    ((((i) as i32).wrapping_add(1i32)) as u8),
                                )) as i32)
                            {
                                signalStrength = GetConnectedChildStrength(
                                    ((((i) as i32).wrapping_add(1i32)) as u8),
                                );
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                signalStrength = GetParentSignalStrength();
            }
            if ((IsRfuRecoveringFromLinkLoss()) as i32) == 1i32 {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(4i16);
            } else {
                if ((signalStrength) as i32) <= 24i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
                } else {
                    if (((signalStrength) as i32) >= 25i32) && (((signalStrength) as i32) <= 126i32)
                    {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(2i16);
                    } else {
                        if (((signalStrength) as i32) >= 127i32)
                            && (((signalStrength) as i32) <= 228i32)
                        {
                            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                        } else {
                            if ((signalStrength) as i32) >= 229i32 {
                                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                            }
                        }
                    }
                }
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
            {
                SetWirelessStatusIndicatorAnim(
                    sprite,
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
            }
            if (crate::c::bf_read(
                ((((((sprite).wrapping_add(8).cast::<*mut *mut u8>()).read()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32) as isize,
                ))
                .read())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 4,
                ))
                .wrapping_add(2),
                0,
                6,
                false,
            ) as u32)
                < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u32)
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                if (((((((((sprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .read())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 4,
                ))
                .cast::<i16>())
                .read()) as i32)
                    == (-2i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                }
            } else {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                .wrapping_offset(1000)
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    (&raw const sWirelessStatusIndicatorOamData)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<8>>()
                        .read_unaligned(),
                );
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(1000))
                .wrapping_add(2),
                0,
                9,
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)))
                    as u32) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(1000))
                .wrapping_add(0),
                0,
                8,
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
                    as u32) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(1000))
                .wrapping_add(5),
                4,
                4,
                (crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(1000))
                .wrapping_add(4),
                0,
                10,
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as u32)
                    .wrapping_add(
                        (crate::c::bf_read(
                            ((((((sprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
                                .wrapping_offset(
                                    ((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(2))
                                    .read()) as i32) as isize,
                                ))
                            .read())
                            .wrapping_offset(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                    .read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            0,
                            16,
                            false,
                        ) as u32),
                    )) as u16) as i32,
            );
            'l3: loop {
                'l4: {
                    'l5: loop {
                        'l6: {
                            CpuSet(
                                ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                                    .wrapping_offset(1000),
                                ((117440512i32) as usize as *mut u8).wrapping_offset(1000),
                                (0u32
                                    | (crate::c::div_u32(
                                        8u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l5;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l3;
                }
            }
            if ((RfuGetStatus()) as i32) == 1i32 {
                DestroyWirelessStatusIndicatorSprite();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CopyTrainerRecord(dest: *mut u8, trainerId: u32, name: *mut u8) {
    unsafe {
        let mut dest = dest;
        let mut trainerId = trainerId;
        let mut name = name;
        ((dest).cast::<u32>()).write(trainerId);
        StringCopy(((dest).wrapping_add(4)).cast::<u8>(), name);
    }
}
pub(crate) unsafe extern "C" fn NameIsNotEmpty(name: *mut u8) -> u32 {
    unsafe {
        let mut name = name;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((name).wrapping_offset((i) as isize)).read()) as i32) != 0i32 {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveLinkTrainerNames() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            let mut i: i32 = 0i32;
            let mut j: i32 = 0i32;
            let mut nextSpace: i32 = 0i32;
            let mut connectedTrainerRecordIndices = crate::ffi::Align4([0u8; 20]);
            let mut newRecords: *mut u8 = AllocZeroed(240u32);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((GetLinkPlayerCount()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut connectedTrainerRecordIndices).cast::<i32>())
                            .wrapping_offset((i) as isize))
                        .write((-1i32));
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < ((crate::c::div_u32(240u32, 12u32)) as i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(4)
                                    .cast::<u32>())
                                    .read()) as u16)
                                        as u32)
                                        == (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(15256))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 12))
                                        .cast::<u32>())
                                        .read())
                                        && (StringCompare(
                                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset((i) as isize * 28))
                                            .wrapping_add(8))
                                            .cast::<u8>(),
                                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(15256))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 12))
                                            .wrapping_add(4))
                                            .cast::<u8>(),
                                        ) == 0i32)
                                    {
                                        (((&raw mut connectedTrainerRecordIndices).cast::<i32>())
                                            .wrapping_offset((i) as isize))
                                        .write(j);
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            nextSpace = 0i32;
            {
                i = 0i32;
                'l5: loop {
                    if !(i < ((GetLinkPlayerCount()) as i32)) {
                        break 'l5;
                    }
                    'l6: {
                        if (i != ((GetMultiplayerId()) as i32))
                            && (((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(26)
                            .cast::<u16>())
                            .read()) as i32)
                                != 1i32)
                        {
                            CopyTrainerRecord(
                                (newRecords).wrapping_offset((nextSpace) as isize * 12),
                                (((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .wrapping_add(4)
                                .cast::<u32>())
                                .read()) as u16) as u32),
                                ((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .wrapping_add(8))
                                .cast::<u8>(),
                            );
                            if (((&raw mut connectedTrainerRecordIndices).cast::<i32>())
                                .wrapping_offset((i) as isize))
                            .read()
                                >= 0i32
                            {
                                crate::c::memset(
                                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(15256))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut connectedTrainerRecordIndices).cast::<i32>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as isize
                                            * 12,
                                    ))
                                    .wrapping_add(4))
                                    .cast::<u8>(),
                                    0i32,
                                    8u32,
                                );
                            }
                            nextSpace = (nextSpace).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l7: loop {
                    if !(i < ((crate::c::div_u32(240u32, 12u32)) as i32)) {
                        break 'l7;
                    }
                    'l8: {
                        if (NameIsNotEmpty(
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(15256))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                            .wrapping_add(4))
                            .cast::<u8>(),
                        )) != 0
                        {
                            CopyTrainerRecord(
                                (newRecords).wrapping_offset((nextSpace) as isize * 12),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(15256))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 12))
                                .cast::<u32>())
                                .read(),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(15256))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 12))
                                .wrapping_add(4))
                                .cast::<u8>(),
                            );
                            if {
                                let __t1 = (nextSpace).wrapping_add(1);
                                nextSpace = __t1;
                                __t1
                            } >= ((crate::c::div_u32(240u32, 12u32)) as i32)
                            {
                                break 'l7;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::memcpy(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15256))
                    .cast::<u8>(),
                newRecords,
                240u32,
            );
            Free(newRecords);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerHasMetTrainerBefore(id: u16, name: *mut u8) -> u32 {
    unsafe {
        let mut id = id;
        let mut name = name;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(240u32, 12u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (StringCompare(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .wrapping_add(4))
                        .cast::<u8>(),
                        name,
                    ) == 0i32)
                        && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .cast::<u32>())
                        .read()
                            == ((id) as u32))
                    {
                        return 1u32;
                    }
                    if !((NameIsNotEmpty(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(15256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .wrapping_add(4))
                        .cast::<u8>(),
                    )) != 0)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WipeTrainerNameRecords() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(240u32, 12u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(15256))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 12))
                    .cast::<u32>())
                    .write(0u32);
                    'l3: loop {
                        'l4: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l5: loop {
                                    'l6: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(15256))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 12))
                                            .wrapping_add(4))
                                            .cast::<u8>(),
                                            ((16777216i32
                                                | (crate::c::div_i32(
                                                    8i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l5;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
