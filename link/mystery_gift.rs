//! Translated from `src/mystery_gift.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sReceivedGiftFlags
#[allow(unused_imports)]
use crate::data::mystery_gift::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStatsEnabled: u32 = 0u32;

unsafe extern "C" {
    static mut RomHeaderGameCode: u8;
    static mut RomHeaderSoftwareVersion: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    fn CalcCRC16WithTable(a0: *mut u8, a1: u32) -> u16;
    fn ClearEReaderTrainer(a0: *mut u8);
    fn ClearMysteryGiftFlags();
    fn ClearMysteryGiftVars();
    fn ClearRamScript();
    fn CopyTrainerId(a0: *mut u8, a1: *mut u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn FlagGet(a0: u16) -> u8;
    fn InitQuestionnaireWords();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn ValidateSavedRamScript() -> u32;
    fn WonderNews_Reset();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearMysteryGift() {
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
                                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12844),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        876u32,
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
        ClearSavedWonderNewsMetadata();
        InitQuestionnaireWords();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWonderNews() -> *mut u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(4);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWonderCard() -> *mut u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(452);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWonderCardMetadata() -> *mut u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(788);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWonderNewsMetadata() -> *mut u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(832);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetQuestionnaireWordsPtr() -> *mut u16 {
    unsafe {
        return (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(824))
        .cast::<u16>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSavedWonderNewsAndRelated() {
    unsafe {
        ClearSavedWonderNews();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveWonderNews(news: *mut u8) -> u32 {
    unsafe {
        let mut news = news;
        if !((ValidateWonderNews(news)) != 0) {
            return 0u32;
        }
        ClearSavedWonderNews();
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(4)
            .cast::<crate::c::Rec4<444>>()
            .write_unaligned(news.cast::<crate::c::Rec4<444>>().read_unaligned());
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .cast::<u32>())
        .write(
            ((CalcCRC16WithTable(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                    .wrapping_add(4),
                444u32,
            )) as u32),
        );
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateSavedWonderNews() -> u32 {
    unsafe {
        if ((CalcCRC16WithTable(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(4),
            444u32,
        )) as u32)
            != (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .cast::<u32>())
            .read()
        {
            return 0u32;
        }
        if !((ValidateWonderNews(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(4),
        )) != 0)
        {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ValidateWonderNews(news: *mut u8) -> u32 {
    unsafe {
        let mut news = news;
        if ((((news).cast::<u16>()).read()) as i32) == 0i32 {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingSavedWonderNewsAllowed() -> u32 {
    unsafe {
        let mut news: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(12844))
        .wrapping_add(4);
        if ((((news).wrapping_add(2)).read()) as i32) == 0i32 {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ClearSavedWonderNews() {
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
                                GetSavedWonderNews(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        444u32,
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
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .cast::<u32>())
        .write(0u32);
    }
}
pub(crate) unsafe extern "C" fn ClearSavedWonderNewsMetadata() {
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
                                GetSavedWonderNewsMetadata(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        4u32,
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
        WonderNews_Reset();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWonderNewsSameAsSaved(news: *mut u8) -> u32 {
    unsafe {
        let mut news = news;
        let mut savedNews: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(12844))
        .wrapping_add(4);
        let mut i: u32 = 0u32;
        if !((ValidateSavedWonderNews()) != 0) {
            return 0u32;
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < 444u32) {
                    break 'l1;
                }
                'l2: {
                    if ((((savedNews).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        != ((((news).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSavedWonderCardAndRelated() {
    unsafe {
        ClearSavedWonderCard();
        ClearSavedWonderCardMetadata();
        ClearSavedTrainerIds();
        ClearRamScript();
        ClearMysteryGiftFlags();
        ClearMysteryGiftVars();
        ClearEReaderTrainer(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1440),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveWonderCard(card: *mut u8) -> u32 {
    unsafe {
        let mut card = card;
        let mut metadata: *mut u8 = core::ptr::null_mut();
        if !((ValidateWonderCard(card)) != 0) {
            return 0u32;
        }
        ClearSavedWonderCardAndRelated();
        crate::c::memcpy(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(452),
            card,
            332u32,
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(448)
            .cast::<u32>())
        .write(
            ((CalcCRC16WithTable(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                    .wrapping_add(452),
                332u32,
            )) as u32),
        );
        metadata = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(788);
        ((metadata).wrapping_add(6).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(452))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateSavedWonderCard() -> u32 {
    unsafe {
        if (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(448)
            .cast::<u32>())
        .read()
            != ((CalcCRC16WithTable(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                    .wrapping_add(452),
                332u32,
            )) as u32)
        {
            return 0u32;
        }
        if !((ValidateWonderCard(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(452),
        )) != 0)
        {
            return 0u32;
        }
        if !((ValidateSavedRamScript()) != 0) {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ValidateWonderCard(card: *mut u8) -> u32 {
    unsafe {
        let mut card = card;
        if ((((card).cast::<u16>()).read()) as i32) == 0i32 {
            return 0u32;
        }
        if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32) >= 3i32 {
            return 0u32;
        }
        if !(((((crate::c::bf_read((card).wrapping_add(8), 6, 2, false) as u8) as i32) == 0i32)
            || (((crate::c::bf_read((card).wrapping_add(8), 6, 2, false) as u8) as i32) == 1i32))
            || (((crate::c::bf_read((card).wrapping_add(8), 6, 2, false) as u8) as i32) == 2i32))
        {
            return 0u32;
        }
        if ((crate::c::bf_read((card).wrapping_add(8), 2, 4, false) as u8) as i32) >= 8i32 {
            return 0u32;
        }
        if ((((card).wrapping_add(9)).read()) as i32) > 7i32 {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingSavedWonderCardAllowed() -> u32 {
    unsafe {
        let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(12844))
        .wrapping_add(452);
        if ((crate::c::bf_read((card).wrapping_add(8), 6, 2, false) as u8) as i32) == 0i32 {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ClearSavedWonderCard() {
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
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12844))
                                .wrapping_add(452),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        332u32,
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
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(448)
            .cast::<u32>())
        .write(0u32);
    }
}
pub(crate) unsafe extern "C" fn ClearSavedWonderCardMetadata() {
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
                                GetSavedWonderCardMetadata(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        36u32,
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
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(784)
            .cast::<u32>())
        .write(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWonderCardFlagID() -> u16 {
    unsafe {
        if (ValidateSavedWonderCard()) != 0 {
            return ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(12844))
            .wrapping_add(452))
            .cast::<u16>())
            .read();
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableWonderCardSending(card: *mut u8) {
    unsafe {
        let mut card = card;
        if ((crate::c::bf_read((card).wrapping_add(8), 6, 2, false) as u8) as i32) == 1i32 {
            crate::c::bf_write((card).wrapping_add(8), 6, 2, (0u8) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn IsWonderCardFlagIDInValidRange(flagId: u16) -> u32 {
    unsafe {
        let mut flagId = flagId;
        if (((flagId) as i32) >= 1000i32) && (((flagId) as i32) < 1020i32) {
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSavedWonderCardGiftNotReceived() -> u32 {
    unsafe {
        let mut value: u16 = GetWonderCardFlagID();
        if !((IsWonderCardFlagIDInValidRange(value)) != 0) {
            return 0u32;
        }
        if ((FlagGet(
            ((((&raw const sReceivedGiftFlags)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((((value) as i32).wrapping_sub(1000i32)) as isize))
            .read(),
        )) as i32)
            == 1i32
        {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GetNumStampsInMetadata(data: *mut u8, size: i32) -> i32 {
    unsafe {
        let mut data = data;
        let mut size = size;
        let mut numStamps: i32 = 0i32;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < size) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((data).wrapping_add(8)).cast::<u8>()).wrapping_offset(14))
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read())
                        != 0)
                        && ((((((((data).wrapping_add(8)).cast::<u8>()).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32)
                    {
                        numStamps = (numStamps).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return numStamps;
    }
}
pub(crate) unsafe extern "C" fn IsStampInMetadata(
    metadata: *mut u8,
    stamp: *mut u16,
    maxStamps: i32,
) -> u32 {
    unsafe {
        let mut metadata = metadata;
        let mut stamp = stamp;
        let mut maxStamps = maxStamps;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < maxStamps) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((metadata).wrapping_add(8)).cast::<u8>()).wrapping_offset(14))
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((((stamp).wrapping_offset(1)).read()) as i32)
                    {
                        return 1u32;
                    }
                    if (((((((metadata).wrapping_add(8)).cast::<u8>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == (((stamp).read()) as i32)
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
pub(crate) unsafe extern "C" fn ValidateStamp(stamp: *mut u16) -> u32 {
    unsafe {
        let mut stamp = stamp;
        if ((((stamp).wrapping_offset(1)).read()) as i32) == 0i32 {
            return 0u32;
        }
        if (((stamp).read()) as i32) == 0i32 {
            return 0u32;
        }
        if (((stamp).read()) as i32) >= 412i32 {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GetNumStampsInSavedCard() -> i32 {
    unsafe {
        let mut card: *mut u8 = core::ptr::null_mut();
        if !((ValidateSavedWonderCard()) != 0) {
            return 0i32;
        }
        card = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(452);
        if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32) != 1i32 {
            return 0i32;
        }
        return GetNumStampsInMetadata(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(788),
            ((((card).wrapping_add(9)).read()) as i32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_TrySaveStamp(stamp: *mut u16) -> u32 {
    unsafe {
        let mut stamp = stamp;
        let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(12844))
        .wrapping_add(452);
        let mut maxStamps: i32 = ((((card).wrapping_add(9)).read()) as i32);
        let mut i: i32 = 0i32;
        if !((ValidateStamp(stamp)) != 0) {
            return 0u32;
        }
        if (IsStampInMetadata(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
                .wrapping_add(788),
            stamp,
            maxStamps,
        )) != 0
        {
            return 0u32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < maxStamps) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(12844))
                    .wrapping_add(788))
                    .wrapping_add(8))
                    .cast::<u8>())
                    .wrapping_offset(14))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32)
                        && ((((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12844))
                        .wrapping_add(788))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 0i32)
                    {
                        ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12844))
                        .wrapping_add(788))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(14))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(((stamp).wrapping_offset(1)).read());
                        (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12844))
                        .wrapping_add(788))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write((stamp).read());
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
pub unsafe extern "C" fn MysteryGift_LoadLinkGameData(data: *mut u8, isWonderNews: u32) {
    unsafe {
        let mut data = data;
        let mut isWonderNews = isWonderNews;
        let mut i: i32 = 0i32;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                data,
                                (83886080u32
                                    | (crate::c::div_u32(
                                        100u32,
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
        ((data).cast::<u32>()).write(257u32);
        ((data).wrapping_add(4).cast::<u16>()).write(1u16);
        ((data).wrapping_add(8).cast::<u32>()).write(1u32);
        if (isWonderNews) != 0 {
            ((data).wrapping_add(12).cast::<u16>()).write(5u16);
            ((data).wrapping_add(16).cast::<u32>()).write(513u32);
        } else {
            ((data).wrapping_add(12).cast::<u16>()).write(4u16);
            ((data).wrapping_add(16).cast::<u32>()).write(512u32);
        }
        if (ValidateSavedWonderCard()) != 0 {
            ((data).wrapping_add(20).cast::<u16>())
                .write(((GetSavedWonderCard()).cast::<u16>()).read());
            (data)
                .wrapping_add(32)
                .cast::<crate::c::Rec4<36>>()
                .write_unaligned(
                    GetSavedWonderCardMetadata()
                        .cast::<crate::c::Rec4<36>>()
                        .read_unaligned(),
                );
            ((data).wrapping_add(68)).write(((GetSavedWonderCard()).wrapping_add(9)).read());
        } else {
            ((data).wrapping_add(20).cast::<u16>()).write(0u16);
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    ((((data).wrapping_add(22)).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12844))
                            .wrapping_add(824))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyTrainerId(
            ((data).wrapping_add(76)).cast::<u8>(),
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10)).cast::<u8>(),
        );
        StringCopy(
            ((data).wrapping_add(69)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        {
            i = 0i32;
            'l7: loop {
                if !(i < 6i32) {
                    break 'l7;
                }
                'l8: {
                    ((((data).wrapping_add(80)).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(11184))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::memcpy(
            ((data).wrapping_add(92)).cast::<u8>(),
            (&raw mut RomHeaderGameCode).cast::<u8>(),
            4u32,
        );
        ((data).wrapping_add(96)).write(((&raw mut RomHeaderSoftwareVersion).cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_ValidateLinkGameData(data: *mut u8, isWonderNews: u32) -> u32 {
    unsafe {
        let mut data = data;
        let mut isWonderNews = isWonderNews;
        if ((data).cast::<u32>()).read() != 257u32 {
            return 0u32;
        }
        if !((((((data).wrapping_add(4).cast::<u16>()).read()) as i32) & 1i32) != 0) {
            return 0u32;
        }
        if !((((data).wrapping_add(8).cast::<u32>()).read() & 1u32) != 0) {
            return 0u32;
        }
        if !((isWonderNews) != 0) {
            if !((((((data).wrapping_add(12).cast::<u16>()).read()) as i32) & 4i32) != 0) {
                return 0u32;
            }
            if !((((data).wrapping_add(16).cast::<u32>()).read() & 896u32) != 0) {
                return 0u32;
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_CompareCardFlags(
    flagId: *mut u16,
    data: *mut u8,
    unused: *mut u8,
) -> u32 {
    unsafe {
        let mut flagId = flagId;
        let mut data = data;
        let mut unused = unused;
        if ((((data).wrapping_add(20).cast::<u16>()).read()) as i32) == 0i32 {
            return 0u32;
        }
        if (((flagId).read()) as i32) == ((((data).wrapping_add(20).cast::<u16>()).read()) as i32) {
            return 1u32;
        }
        return 2u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_CheckStamps(
    stamp: *mut u16,
    data: *mut u8,
    unused: *mut u8,
) -> u32 {
    unsafe {
        let mut stamp = stamp;
        let mut data = data;
        let mut unused = unused;
        let mut stampsMissing: i32 =
            ((((data).wrapping_add(68)).read()) as i32).wrapping_sub(GetNumStampsInMetadata(
                (data).wrapping_add(32),
                ((((data).wrapping_add(68)).read()) as i32),
            ));
        if stampsMissing == 0i32 {
            return 1u32;
        }
        if (IsStampInMetadata(
            (data).wrapping_add(32),
            stamp,
            ((((data).wrapping_add(68)).read()) as i32),
        )) != 0
        {
            return 3u32;
        }
        if stampsMissing == 1i32 {
            return 4u32;
        }
        return 2u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_DoesQuestionnaireMatch(data: *mut u8, words: *mut u16) -> u32 {
    unsafe {
        let mut data = data;
        let mut words = words;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((data).wrapping_add(22)).cast::<u16>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        != ((((words).wrapping_offset((i) as isize)).read()) as i32)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GetNumStampsInLinkData(data: *mut u8) -> i32 {
    unsafe {
        let mut data = data;
        return GetNumStampsInMetadata(
            (data).wrapping_add(32),
            ((((data).wrapping_add(68)).read()) as i32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_GetCardStatFromLinkData(data: *mut u8, stat: u32) -> u16 {
    unsafe {
        let mut data = data;
        let mut stat = stat;
        'l1: {
            let __sw1 = stat;
            let __matched =
                __sw1 == 0u32 || __sw1 == 1u32 || __sw1 == 2u32 || __sw1 == 3u32 || __sw1 == 4u32;
            if __sw1 == 0u32 {
                return (((data).wrapping_add(32)).cast::<u16>()).read();
            }
            if __sw1 == 1u32 {
                return (((data).wrapping_add(32)).wrapping_add(2).cast::<u16>()).read();
            }
            if __sw1 == 2u32 {
                return (((data).wrapping_add(32)).wrapping_add(4).cast::<u16>()).read();
            }
            if __sw1 == 3u32 {
                return ((GetNumStampsInLinkData(data)) as u16);
            }
            if __sw1 == 4u32 {
                return ((((data).wrapping_add(68)).read()) as u16);
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn IncrementCardStat(statType: u32) {
    unsafe {
        let mut statType = statType;
        let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(12844))
        .wrapping_add(452);
        if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32) == 2i32 {
            let mut stat: *mut u16 = core::ptr::null_mut();
            'l1: {
                let __sw1 = statType;
                if __sw1 == 0u32 {
                    stat = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(12844))
                    .wrapping_add(788))
                    .cast::<u16>();
                    break 'l1;
                }
                if __sw1 == 1u32 {
                    stat = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(12844))
                    .wrapping_add(788))
                    .wrapping_add(2)
                    .cast::<u16>();
                    break 'l1;
                }
                if __sw1 == 2u32 {
                    stat = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(12844))
                    .wrapping_add(788))
                    .wrapping_add(4)
                    .cast::<u16>();
                    break 'l1;
                }
                if __sw1 == 3u32 || __sw1 == 4u32 {
                    break 'l1;
                }
            }
            if ((stat) as usize) == 0usize {
            } else {
                if (({
                    let __t2 = ((stat).read()).wrapping_add(1);
                    (stat).write(__t2);
                    __t2
                }) as i32)
                    > 999i32
                {
                    (stat).write(999u16);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_GetCardStat(stat: u32) -> u16 {
    unsafe {
        let mut stat = stat;
        'l1: {
            let __sw1 = stat;
            if __sw1 == 0u32 {
                {
                    let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(12844))
                    .wrapping_add(452);
                    if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32)
                        == 2i32
                    {
                        let mut metadata: *mut u8 =
                            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12844))
                            .wrapping_add(788);
                        return ((metadata).cast::<u16>()).read();
                    }
                    break 'l1;
                }
            }
            if __sw1 == 1u32 {
                {
                    let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(12844))
                    .wrapping_add(452);
                    if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32)
                        == 2i32
                    {
                        let mut metadata: *mut u8 =
                            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12844))
                            .wrapping_add(788);
                        return ((metadata).wrapping_add(2).cast::<u16>()).read();
                    }
                    break 'l1;
                }
            }
            if __sw1 == 2u32 {
                {
                    let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(12844))
                    .wrapping_add(452);
                    if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32)
                        == 2i32
                    {
                        let mut metadata: *mut u8 =
                            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12844))
                            .wrapping_add(788);
                        return ((metadata).wrapping_add(4).cast::<u16>()).read();
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3u32 {
                {
                    let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(12844))
                    .wrapping_add(452);
                    if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32)
                        == 1i32
                    {
                        return ((GetNumStampsInSavedCard()) as u16);
                    }
                    break 'l1;
                }
            }
            if __sw1 == 4u32 {
                {
                    let mut card: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(12844))
                    .wrapping_add(452);
                    if ((crate::c::bf_read((card).wrapping_add(8), 0, 2, false) as u8) as i32)
                        == 1i32
                    {
                        return ((((card).wrapping_add(9)).read()) as u16);
                    }
                    break 'l1;
                }
            }
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_DisableStats() {
    unsafe {
        ((&raw mut sStatsEnabled).cast::<u8>().cast::<u32>()).write(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_TryEnableStatsByFlagId(flagId: u16) -> u32 {
    unsafe {
        let mut flagId = flagId;
        ((&raw mut sStatsEnabled).cast::<u8>().cast::<u32>()).write(0u32);
        if ((flagId) as i32) == 0i32 {
            return 0u32;
        }
        if !((ValidateSavedWonderCard()) != 0) {
            return 0u32;
        }
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12844))
            .wrapping_add(452))
        .cast::<u16>())
        .read()) as i32)
            != ((flagId) as i32)
        {
            return 0u32;
        }
        ((&raw mut sStatsEnabled).cast::<u8>().cast::<u32>()).write(1u32);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGift_TryIncrementStat(stat: u32, trainerId: u32) {
    unsafe {
        let mut stat = stat;
        let mut trainerId = trainerId;
        if (((&raw mut sStatsEnabled).cast::<u8>().cast::<u32>()).read()) != 0 {
            'l1: {
                let __sw1 = stat;
                let __matched = __sw1 == 2u32 || __sw1 == 0u32 || __sw1 == 1u32;
                if __sw1 == 2u32 {
                    IncrementCardStatForNewTrainer(
                        2u32,
                        trainerId,
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12844))
                        .wrapping_add(836))
                        .cast::<u8>())
                        .wrapping_offset(20))
                        .cast::<u32>(),
                        ((crate::c::div_u32(20u32, 4u32)) as i32),
                    );
                    break 'l1;
                }
                if __sw1 == 0u32 {
                    IncrementCardStatForNewTrainer(
                        0u32,
                        trainerId,
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12844))
                        .wrapping_add(836))
                        .cast::<u8>())
                        .cast::<u32>(),
                        ((crate::c::div_u32(20u32, 4u32)) as i32),
                    );
                    break 'l1;
                }
                if __sw1 == 1u32 {
                    IncrementCardStatForNewTrainer(
                        1u32,
                        trainerId,
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12844))
                        .wrapping_add(836))
                        .cast::<u8>())
                        .cast::<u32>(),
                        ((crate::c::div_u32(20u32, 4u32)) as i32),
                    );
                    break 'l1;
                }
                if !__matched {
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearSavedTrainerIds() {
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
                                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12844))
                                .wrapping_add(836))
                                .cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        40u32,
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
pub(crate) unsafe extern "C" fn RecordTrainerId(
    trainerId: u32,
    trainerIds: *mut u32,
    size: i32,
) -> u32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut trainerIds = trainerIds;
        let mut size = size;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < size) {
                    break 'l1;
                }
                'l2: {
                    if ((trainerIds).wrapping_offset((i) as isize)).read() == trainerId {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i == size {
            {
                j = (size).wrapping_sub(1i32);
                'l3: loop {
                    if !(j > 0i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((trainerIds).wrapping_offset((j) as isize)).write(
                            ((trainerIds).wrapping_offset(((j).wrapping_sub(1i32)) as isize))
                                .read(),
                        );
                    }
                    j = (j).wrapping_sub(1);
                }
            }
            (trainerIds).write(trainerId);
            return 1u32;
        } else {
            {
                j = i;
                'l5: loop {
                    if !(j > 0i32) {
                        break 'l5;
                    }
                    'l6: {
                        ((trainerIds).wrapping_offset((j) as isize)).write(
                            ((trainerIds).wrapping_offset(((j).wrapping_sub(1i32)) as isize))
                                .read(),
                        );
                    }
                    j = (j).wrapping_sub(1);
                }
            }
            (trainerIds).write(trainerId);
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn IncrementCardStatForNewTrainer(
    stat: u32,
    trainerId: u32,
    trainerIds: *mut u32,
    size: i32,
) {
    unsafe {
        let mut stat = stat;
        let mut trainerId = trainerId;
        let mut trainerIds = trainerIds;
        let mut size = size;
        if (RecordTrainerId(trainerId, trainerIds, size)) != 0 {
            IncrementCardStat(stat);
        }
    }
}
