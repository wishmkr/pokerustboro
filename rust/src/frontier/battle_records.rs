//! Translated from `src/battle_records.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sTrainerHillWindowTileset sTrainerHillWindowPalette sTrainerHillWindowTilemap sTrainerHillRecordsBgTemplates sTrainerHillRecordsWindowTemplates sLinkBattleRecordsWindow sText_DashesNoPlayer sText_DashesNoScore
#[allow(unused_imports)]
use crate::data::battle_records::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRecordsWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapBuffer: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleOutcome: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_PlayersBattleResults: u8;
    static mut gText_TotalRecordWLD: u8;
    static mut gText_WinLoseDraw: u8;
    static mut gTrainerCards: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetGameStat(a0: u8) -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn InUnionRoom() -> u32;
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PrintOnTrainerHillRecordsWindow();
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGameStat(a0: u8, a1: u32);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StringCompareN(a0: *mut u8, a1: *mut u8, a2: u32) -> i32;
    fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringFillWithTerminator(a0: *mut u8, a1: u16) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn ClearLinkBattleRecord(record: *mut u8) {
    unsafe {
        let mut record = record;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                record,
                                (16777216u32
                                    | (crate::c::div_u32(
                                        16u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
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
        ((record).cast::<u8>()).write(255u8);
        ((record).wrapping_add(8).cast::<u16>()).write(0u16);
        ((record).wrapping_add(10).cast::<u16>()).write(0u16);
        ((record).wrapping_add(12).cast::<u16>()).write(0u16);
        ((record).wrapping_add(14).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn ClearLinkBattleRecords(records: *mut u8) {
    unsafe {
        let mut records = records;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ClearLinkBattleRecord((records).wrapping_offset((i) as isize * 16));
                }
                i = (i).wrapping_add(1);
            }
        }
        SetGameStat(23u8, 0u32);
        SetGameStat(24u8, 0u32);
        SetGameStat(25u8, 0u32);
    }
}
pub(crate) unsafe extern "C" fn GetLinkBattleRecordTotalBattles(record: *mut u8) -> i32 {
    unsafe {
        let mut record = record;
        return (((((record).wrapping_add(10).cast::<u16>()).read()) as i32)
            .wrapping_add(((((record).wrapping_add(12).cast::<u16>()).read()) as i32)))
        .wrapping_add(((((record).wrapping_add(14).cast::<u16>()).read()) as i32));
    }
}
pub(crate) unsafe extern "C" fn FindLinkBattleRecord(
    records: *mut u8,
    name: *mut u8,
    trainerId: u16,
) -> i32 {
    unsafe {
        let mut records = records;
        let mut name = name;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if (!((StringCompareN(
                        ((records).wrapping_offset((i) as isize * 16)).cast::<u8>(),
                        name,
                        7u32,
                    )) != 0))
                        && ((((((records).wrapping_offset((i) as isize * 16))
                            .wrapping_add(8)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((trainerId) as i32))
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 5i32;
    }
}
pub(crate) unsafe extern "C" fn SortLinkBattleRecords(records: *mut u8) {
    unsafe {
        let mut records = records;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 4i32;
            'l1: loop {
                if !(i > 0i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = (i).wrapping_sub(1i32);
                        'l3: loop {
                            if !(j >= 0i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut totalBattlesI: i32 = GetLinkBattleRecordTotalBattles(
                                    ((records).cast::<u8>()).wrapping_offset((i) as isize * 16),
                                );
                                let mut totalBattlesJ: i32 = GetLinkBattleRecordTotalBattles(
                                    ((records).cast::<u8>()).wrapping_offset((j) as isize * 16),
                                );
                                if totalBattlesI > totalBattlesJ {
                                    let mut temp1 = crate::ffi::Align4([0u8; 16]);
                                    let mut temp2: u8 = 0u8;
                                    (&raw mut temp1)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<16>>()
                                        .write_unaligned(
                                            ((records).cast::<u8>())
                                                .wrapping_offset((i) as isize * 16)
                                                .cast::<crate::c::Rec4<16>>()
                                                .read_unaligned(),
                                        );
                                    ((records).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16)
                                        .cast::<crate::c::Rec4<16>>()
                                        .write_unaligned(
                                            ((records).cast::<u8>())
                                                .wrapping_offset((j) as isize * 16)
                                                .cast::<crate::c::Rec4<16>>()
                                                .read_unaligned(),
                                        );
                                    ((records).cast::<u8>())
                                        .wrapping_offset((j) as isize * 16)
                                        .cast::<crate::c::Rec4<16>>()
                                        .write_unaligned(
                                            (&raw mut temp1)
                                                .cast::<u8>()
                                                .cast::<crate::c::Rec4<16>>()
                                                .read_unaligned(),
                                        );
                                    temp2 = ((((records).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read();
                                    ((((records).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(
                                        ((((records).wrapping_add(80)).cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                        .read(),
                                    );
                                    ((((records).wrapping_add(80)).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .write(temp2);
                                }
                            }
                            j = (j).wrapping_sub(1);
                        }
                    }
                }
                i = (i).wrapping_sub(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkBattleRecord(record: *mut u8, battleOutcome: i32) {
    unsafe {
        let mut record = record;
        let mut battleOutcome = battleOutcome;
        'l1: {
            let __sw1 = battleOutcome;
            if __sw1 == 1i32 {
                let __p2 = (record).wrapping_add(10).cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                if ((((record).wrapping_add(10).cast::<u16>()).read()) as i32) > 9999i32 {
                    ((record).wrapping_add(10).cast::<u16>()).write(9999u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = (record).wrapping_add(12).cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                if ((((record).wrapping_add(12).cast::<u16>()).read()) as i32) > 9999i32 {
                    ((record).wrapping_add(12).cast::<u16>()).write(9999u16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p4 = (record).wrapping_add(14).cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((record).wrapping_add(14).cast::<u16>()).read()) as i32) > 9999i32 {
                    ((record).wrapping_add(14).cast::<u16>()).write(9999u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkBattleGameStats(battleOutcome: i32) {
    unsafe {
        let mut battleOutcome = battleOutcome;
        let mut stat: u8 = 0u8;
        'l1: {
            let __sw1 = battleOutcome;
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 1i32 {
                stat = 23u8;
                break 'l1;
            }
            if __sw1 == 2i32 {
                stat = 24u8;
                break 'l1;
            }
            if __sw1 == 3i32 {
                stat = 25u8;
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        if GetGameStat(stat) < 9999u32 {
            IncrementGameStat(stat);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkBattleRecords(
    records: *mut u8,
    name: *mut u8,
    trainerId: u16,
    battleOutcome: i32,
    battler: u8,
) {
    unsafe {
        let mut records = records;
        let mut name = name;
        let mut trainerId = trainerId;
        let mut battleOutcome = battleOutcome;
        let mut battler = battler;
        let mut index: i32 = 0i32;
        UpdateLinkBattleGameStats(battleOutcome);
        SortLinkBattleRecords(records);
        index = FindLinkBattleRecord((records).cast::<u8>(), name, trainerId);
        if index == 5i32 {
            index = 4i32;
            ClearLinkBattleRecord(((records).cast::<u8>()).wrapping_offset((index) as isize * 16));
            StringCopyN(
                (((records).cast::<u8>()).wrapping_offset((index) as isize * 16)).cast::<u8>(),
                name,
                7u8,
            );
            ((((records).cast::<u8>()).wrapping_offset((index) as isize * 16))
                .wrapping_add(8)
                .cast::<u16>())
            .write(trainerId);
            ((((records).wrapping_add(80)).cast::<u8>()).wrapping_offset((index) as isize)).write(
                ((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 28))
                .wrapping_add(26)
                .cast::<u16>())
                .read()) as u8),
            );
        }
        UpdateLinkBattleRecord(
            ((records).cast::<u8>()).wrapping_offset((index) as isize * 16),
            battleOutcome,
        );
        SortLinkBattleRecords(records);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPlayerLinkBattleRecords() {
    unsafe {
        ClearLinkBattleRecords(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624))
                .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn IncTrainerCardWins(battler: i32) {
    unsafe {
        let mut battler = battler;
        let mut wins: *mut u16 = (((&raw mut gTrainerCards).cast::<u8>())
            .wrapping_offset((battler) as isize * 100))
        .wrapping_add(20)
        .cast::<u16>();
        (wins).write(((wins).read()).wrapping_add(1));
        if (((wins).read()) as i32) > 9999i32 {
            (wins).write(9999u16);
        }
    }
}
pub(crate) unsafe extern "C" fn IncTrainerCardLosses(battler: i32) {
    unsafe {
        let mut battler = battler;
        let mut losses: *mut u16 = (((&raw mut gTrainerCards).cast::<u8>())
            .wrapping_offset((battler) as isize * 100))
        .wrapping_add(22)
        .cast::<u16>();
        (losses).write(((losses).read()).wrapping_add(1));
        if (((losses).read()) as i32) > 9999i32 {
            (losses).write(9999u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateTrainerCardWinsLosses(battler: i32) {
    unsafe {
        let mut battler = battler;
        'l1: {
            let __sw1 = ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32);
            if __sw1 == 1i32 {
                IncTrainerCardWins((battler ^ 1i32));
                IncTrainerCardLosses(battler);
                break 'l1;
            }
            if __sw1 == 2i32 {
                IncTrainerCardLosses((battler ^ 1i32));
                IncTrainerCardWins(battler);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePlayerLinkBattleRecords(battler: i32) {
    unsafe {
        let mut battler = battler;
        if InUnionRoom() != 1u32 {
            UpdateTrainerCardWinsLosses(battler);
            UpdateLinkBattleRecords(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624),
                ((((&raw mut gTrainerCards).cast::<u8>())
                    .wrapping_offset((battler) as isize * 100))
                .wrapping_add(48))
                .cast::<u8>(),
                ((((&raw mut gTrainerCards).cast::<u8>())
                    .wrapping_offset((battler) as isize * 100))
                .wrapping_add(14)
                .cast::<u16>())
                .read(),
                ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32),
                ((battler) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintLinkBattleWinsLossesDraws(records: *mut u8) {
    unsafe {
        let mut records = records;
        let mut x: i32 = 0i32;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((GetGameStat(23u8)) as i32),
            0i32,
            4u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((GetGameStat(24u8)) as i32),
            0i32,
            4u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar3).cast::<u8>(),
            ((GetGameStat(25u8)) as i32),
            0i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_TotalRecordWLD).cast::<u8>(),
        );
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 208i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            17u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintLinkBattleRecord(record: *mut u8, y: u8, language: i32) {
    unsafe {
        let mut record = record;
        let mut y = y;
        let mut language = language;
        if ((((((record).wrapping_add(10).cast::<u16>()).read()) as i32) == 0i32)
            && (((((record).wrapping_add(12).cast::<u16>()).read()) as i32) == 0i32))
            && (((((record).wrapping_add(14).cast::<u16>()).read()) as i32) == 0i32)
        {
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                ((&raw const sText_DashesNoPlayer).cast::<u8>().cast_mut()).cast::<u8>(),
                8u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                ((&raw const sText_DashesNoScore).cast::<u8>().cast_mut()).cast::<u8>(),
                80u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                ((&raw const sText_DashesNoScore).cast::<u8>().cast_mut()).cast::<u8>(),
                128u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                ((&raw const sText_DashesNoScore).cast::<u8>().cast_mut()).cast::<u8>(),
                176u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
        } else {
            StringFillWithTerminator((&raw mut gStringVar1).cast::<u8>(), 8u16);
            StringCopyN(
                (&raw mut gStringVar1).cast::<u8>(),
                (record).cast::<u8>(),
                7u8,
            );
            ConvertInternationalString((&raw mut gStringVar1).cast::<u8>(), ((language) as u8));
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar1).cast::<u8>(),
                8u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((record).wrapping_add(10).cast::<u16>()).read()) as i32),
                1i32,
                4u8,
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar1).cast::<u8>(),
                80u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((record).wrapping_add(12).cast::<u16>()).read()) as i32),
                1i32,
                4u8,
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar1).cast::<u8>(),
                128u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((record).wrapping_add(14).cast::<u16>()).read()) as i32),
                1i32,
                4u8,
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar1).cast::<u8>(),
                176u8,
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(1i32)) as u8),
                0u8,
                None,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowLinkBattleRecords() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).write(
            ((AddWindow(
                (&raw const sLinkBattleRecordsWindow)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        FillWindowPixelBuffer(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            17u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PlayersBattleResults).cast::<u8>(),
        );
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 208i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            1u8,
            0u8,
            None,
        );
        PrintLinkBattleWinsLossesDraws(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624))
                .cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_WinLoseDraw).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            41u8,
            0u8,
            None,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    PrintLinkBattleRecord(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12624))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16),
                        (((7i32).wrapping_add((i).wrapping_mul(2i32))) as u8),
                        (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12624))
                        .wrapping_add(80))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read());
        CopyWindowToVram(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveRecordsWindow() {
    unsafe {
        ClearStdWindowAndFrame(
            ((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        RemoveWindow(((&raw mut gRecordsWindowId).cast::<u8>().cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn Task_TrainerHillWaitForPaletteFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_CloseTrainerHillRecordsOnButton));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CloseTrainerHillRecordsOnButton(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
        {
            PlaySE(5u16);
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(Task_BeginPaletteFade));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BeginPaletteFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitTrainerHillRecords));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitTrainerHillRecords(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
            Free(((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read());
            RemoveTrainerHillRecordsWindow(0u8);
            FreeAllWindowBuffers();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn RemoveTrainerHillRecordsWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        FillWindowPixelBuffer(windowId, 0u8);
        ClearWindowTilemap(windowId);
        CopyWindowToVram(windowId, 2u8);
        RemoveWindow(windowId);
    }
}
pub(crate) unsafe extern "C" fn ClearVramOamPlttRegs() {
    unsafe {
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    4096i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
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
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
        }
        'l10: loop {
            'l11: {
                {
                    let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u32);
                    let mut _size: u32 = 1024u32;
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l14: loop {
                                    'l15: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2231369728u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l14;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l12;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l10;
            }
        }
        'l16: loop {
            'l17: {
                {
                    let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u16);
                    let mut _size: u32 = 1024u32;
                    'l18: loop {
                        'l19: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l20: loop {
                                    'l21: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l18;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l16;
            }
        }
        SetGpuReg(0u8, 0u16);
        SetGpuReg(8u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(14u8, 0u16);
        SetGpuReg(28u8, 0u16);
        SetGpuReg(30u8, 0u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn ClearTasksAndGraphicalStructs() {
    unsafe {
        ScanlineEffect_Stop();
        ResetTasks();
        ResetSpriteData();
        ResetPaletteFade();
        FreeAllSpritePalettes();
    }
}
pub(crate) unsafe extern "C" fn ResetBgCoordinates() {
    unsafe {
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
    }
}
pub(crate) unsafe extern "C" fn SetDispcntReg() {
    unsafe {
        SetGpuReg(0u8, 2368u16);
    }
}
pub(crate) unsafe extern "C" fn LoadTrainerHillRecordsWindowGfx(bgId: u8) {
    unsafe {
        let mut bgId = bgId;
        LoadBgTiles(
            bgId,
            (((&raw const sTrainerHillWindowTileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            192u16,
            0u16,
        );
        CopyToBgTilemapBufferRect(
            bgId,
            (((&raw const sTrainerHillWindowTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u8,
            0u8,
            32u8,
            32u8,
        );
        LoadPalette(
            (((&raw const sTrainerHillWindowPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn VblankCB_TrainerHillRecords() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn MainCB2_TrainerHillRecords() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerHillRecords() {
    unsafe {
        SetVBlankCallback(None);
        SetMainCallback2(Some(CB2_ShowTrainerHillRecords));
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowTrainerHillRecords() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ClearVramOamPlttRegs();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ClearTasksAndGraphicalStructs();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(2048u32));
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sTrainerHillRecordsBgTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(8u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    3u8,
                    ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read(),
                );
                ResetBgCoordinates();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadTrainerHillRecordsWindowGfx(3u8);
                LoadPalette((GetTextWindowPalette(0u8)).cast::<u8>(), 240u16, 32u16);
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((IsDma3ManagerBusyWithBgCopy()) as i32) != 1i32 {
                    ShowBg(0u8);
                    ShowBg(3u8);
                    CopyBgTilemapBufferToVram(3u8);
                    let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                InitWindows(
                    ((&raw const sTrainerHillRecordsWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                SetDispcntReg();
                SetVBlankCallback(Some(VblankCB_TrainerHillRecords));
                PrintOnTrainerHillRecordsWindow();
                CreateTask(Some(Task_TrainerHillWaitForPaletteFade), 8u8);
                SetMainCallback2(Some(MainCB2_TrainerHillRecords));
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(0u8);
                break 'l1;
            }
        }
    }
}
