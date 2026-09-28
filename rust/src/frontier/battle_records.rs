//! Translated from `src/battle_records.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sTrainerHillWindowTileset sTrainerHillWindowPalette sTrainerHillWindowTilemap sTrainerHillRecordsBgTemplates sTrainerHillRecordsWindowTemplates sLinkBattleRecordsWindow sText_DashesNoPlayer sText_DashesNoScore

static sLinkBattleRecordsWindow: Table<WindowTemplate> =
    Table((&raw const crate::data::battle_records::sLinkBattleRecordsWindow).cast());
static sText_DashesNoPlayer: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::battle_records::sText_DashesNoPlayer).cast());
static sText_DashesNoScore: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_records::sText_DashesNoScore).cast());
static sTrainerHillRecordsBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::battle_records::sTrainerHillRecordsBgTemplates).cast());
static sTrainerHillRecordsWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::battle_records::sTrainerHillRecordsWindowTemplates).cast());
static sTrainerHillWindowPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_records::sTrainerHillWindowPalette).cast());
static sTrainerHillWindowTilemap: Table<CArray<u32, 512>> =
    Table((&raw const crate::data::battle_records::sTrainerHillWindowTilemap).cast());
static sTrainerHillWindowTileset: Table<CArray<u32, 48>> =
    Table((&raw const crate::data::battle_records::sTrainerHillWindowTileset).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRecordsWindowId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapBuffer: *mut u8 = null_mut();

unsafe extern "C" {
    static mut gBattleOutcome: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_PlayersBattleResults: CArray<u8, 0>;
    static gText_TotalRecordWLD: CArray<u8, 0>;
    static gText_WinLoseDraw: CArray<u8, 0>;
    static mut gTrainerCards: CArray<TrainerCard, 4>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
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
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetGameStat(a0: u8) -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn InUnionRoom() -> u32;
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
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

pub(crate) unsafe extern "C" fn ClearLinkBattleRecord(record: *mut LinkBattleRecord) {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                record as *mut c_void,
                0x1000008,
            );
        }
    }
    (*record).name[0] = EOS;
    (*record).trainerId = 0;
    (*record).wins = 0;
    (*record).losses = 0;
    (*record).draws = 0;
}
pub(crate) unsafe extern "C" fn ClearLinkBattleRecords(records: *mut LinkBattleRecord) {
    let mut i: i32 = 0;
    i = 0;
    while i < LINK_B_RECORDS_COUNT {
        ClearLinkBattleRecord(records.at(i));
        i += 1;
    }
    SetGameStat(GAME_STAT_LINK_BATTLE_WINS, 0);
    SetGameStat(GAME_STAT_LINK_BATTLE_LOSSES, 0);
    SetGameStat(GAME_STAT_LINK_BATTLE_DRAWS, 0);
}
pub(crate) unsafe extern "C" fn GetLinkBattleRecordTotalBattles(
    record: *mut LinkBattleRecord,
) -> i32 {
    return (*record).wins as i32 + (*record).losses as i32 + (*record).draws as i32;
}
pub(crate) unsafe extern "C" fn FindLinkBattleRecord(
    records: *mut LinkBattleRecord,
    name: *mut u8,
    trainerId: u16,
) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < LINK_B_RECORDS_COUNT {
        if StringCompareN(
            (*records.at(i)).name.as_mut_ptr(),
            name,
            PLAYER_NAME_LENGTH as u32,
        ) == 0
            && (*records.at(i)).trainerId == trainerId
        {
            return i;
        }
        i += 1;
    }
    return LINK_B_RECORDS_COUNT;
}
pub(crate) unsafe extern "C" fn SortLinkBattleRecords(records: *mut LinkBattleRecords) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 4;
    while i > 0 {
        j = i - 1;
        while j >= 0 {
            let mut totalBattlesI: i32 =
                GetLinkBattleRecordTotalBattles(&raw mut (*records).entries[i]);
            let mut totalBattlesJ: i32 =
                GetLinkBattleRecordTotalBattles(&raw mut (*records).entries[j]);
            if totalBattlesI > totalBattlesJ {
                let mut temp1: LinkBattleRecord = zeroed();
                let mut temp2: u8 = 0;
                temp1 = (*records).entries[i];
                (*records).entries[i] = (*records).entries[j];
                (*records).entries[j] = temp1;
                temp2 = (*records).languages[i];
                (*records).languages[i] = (*records).languages[j];
                (*records).languages[j] = temp2;
            }
            j -= 1;
        }
        i -= 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkBattleRecord(
    record: *mut LinkBattleRecord,
    battleOutcome: i32,
) {
    match battleOutcome {
        1 => {
            (*record).wins += 1;
            if (*record).wins > 9999 {
                (*record).wins = 9999;
            }
        }
        2 => {
            (*record).losses += 1;
            if (*record).losses > 9999 {
                (*record).losses = 9999;
            }
        }
        3 => {
            (*record).draws += 1;
            if (*record).draws > 9999 {
                (*record).draws = 9999;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkBattleGameStats(battleOutcome: i32) {
    let mut stat: u8 = 0;
    match battleOutcome {
        1 => {
            stat = GAME_STAT_LINK_BATTLE_WINS;
        }
        2 => {
            stat = GAME_STAT_LINK_BATTLE_LOSSES;
        }
        3 => {
            stat = GAME_STAT_LINK_BATTLE_DRAWS;
        }
        _ => {
            return;
        }
    }
    if GetGameStat(stat) < 9999 {
        IncrementGameStat(stat);
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkBattleRecords(
    records: *mut LinkBattleRecords,
    name: *mut u8,
    trainerId: u16,
    battleOutcome: i32,
    battler: u8,
) {
    let mut index: i32 = 0;
    UpdateLinkBattleGameStats(battleOutcome);
    SortLinkBattleRecords(records);
    index = FindLinkBattleRecord((*records).entries.as_mut_ptr(), name, trainerId);
    if index == LINK_B_RECORDS_COUNT {
        index = 4;
        ClearLinkBattleRecord(&raw mut (*records).entries[index]);
        StringCopyN(
            (*records).entries[index].name.as_mut_ptr(),
            name,
            PLAYER_NAME_LENGTH as u8,
        );
        (*records).entries[index].trainerId = trainerId;
        (*records).languages[index] = gLinkPlayers[battler].language as u8;
    }
    UpdateLinkBattleRecord(&raw mut (*records).entries[index], battleOutcome);
    SortLinkBattleRecords(records);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPlayerLinkBattleRecords() {
    ClearLinkBattleRecords((*gSaveBlock1Ptr).linkBattleRecords.entries.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn IncTrainerCardWins(battler: i32) {
    let mut wins: *mut u16 = &raw mut gTrainerCards[battler].linkBattleWins;
    *wins += 1;
    if *wins > 9999 {
        *wins = 9999;
    }
}
pub(crate) unsafe extern "C" fn IncTrainerCardLosses(battler: i32) {
    let mut losses: *mut u16 = &raw mut gTrainerCards[battler].linkBattleLosses;
    *losses += 1;
    if *losses > 9999 {
        *losses = 9999;
    }
}
pub(crate) unsafe extern "C" fn UpdateTrainerCardWinsLosses(battler: i32) {
    match gBattleOutcome {
        B_OUTCOME_WON => {
            IncTrainerCardWins(battler ^ 1);
            IncTrainerCardLosses(battler);
        }
        B_OUTCOME_LOST => {
            IncTrainerCardLosses(battler ^ 1);
            IncTrainerCardWins(battler);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePlayerLinkBattleRecords(battler: i32) {
    if InUnionRoom() != TRUE as u32 {
        UpdateTrainerCardWinsLosses(battler);
        UpdateLinkBattleRecords(
            &raw mut (*gSaveBlock1Ptr).linkBattleRecords,
            gTrainerCards[battler].playerName.as_mut_ptr(),
            gTrainerCards[battler].trainerId,
            gBattleOutcome as i32,
            battler as u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintLinkBattleWinsLossesDraws(records: *mut LinkBattleRecord) {
    let mut x: i32 = 0;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetGameStat(GAME_STAT_LINK_BATTLE_WINS) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        GetGameStat(GAME_STAT_LINK_BATTLE_LOSSES) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    ConvertIntToDecimalStringN(
        gStringVar3.as_mut_ptr(),
        GetGameStat(GAME_STAT_LINK_BATTLE_DRAWS) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_TotalRecordWLD.as_ptr().cast_mut(),
    );
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0xD0);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        0x11,
        0,
        None,
    );
}
pub(crate) unsafe extern "C" fn PrintLinkBattleRecord(
    record: *mut LinkBattleRecord,
    y: u8,
    language: i32,
) {
    if (*record).wins == 0 && (*record).losses == 0 && (*record).draws == 0 {
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            sText_DashesNoPlayer.as_ptr().cast_mut(),
            8,
            y * 8 + 1,
            0,
            None,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            sText_DashesNoScore.as_ptr().cast_mut(),
            80,
            y * 8 + 1,
            0,
            None,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            sText_DashesNoScore.as_ptr().cast_mut(),
            128,
            y * 8 + 1,
            0,
            None,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            sText_DashesNoScore.as_ptr().cast_mut(),
            176,
            y * 8 + 1,
            0,
            None,
        );
    } else {
        StringFillWithTerminator(gStringVar1.as_mut_ptr(), 8);
        StringCopyN(gStringVar1.as_mut_ptr(), (*record).name.as_mut_ptr(), 7);
        ConvertInternationalString(gStringVar1.as_mut_ptr(), language as u8);
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            8,
            y * 8 + 1,
            0,
            None,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*record).wins as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            80,
            y * 8 + 1,
            0,
            None,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*record).losses as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            128,
            y * 8 + 1,
            0,
            None,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*record).draws as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            176,
            y * 8 + 1,
            0,
            None,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowLinkBattleRecords() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    gRecordsWindowId = AddWindow((&raw const *sLinkBattleRecordsWindow).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PlayersBattleResults.as_ptr().cast_mut(),
    );
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 208);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        1,
        0,
        None,
    );
    PrintLinkBattleWinsLossesDraws((*gSaveBlock1Ptr).linkBattleRecords.entries.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_WinLoseDraw.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        41,
        0,
        None,
    );
    i = 0;
    while i < LINK_B_RECORDS_COUNT {
        PrintLinkBattleRecord(
            &raw mut (*gSaveBlock1Ptr).linkBattleRecords.entries[i],
            7 + i as u8 * 2,
            (*gSaveBlock1Ptr).linkBattleRecords.languages[i] as i32,
        );
        i += 1;
    }
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveRecordsWindow() {
    ClearStdWindowAndFrame(gRecordsWindowId, FALSE);
    RemoveWindow(gRecordsWindowId);
}
pub(crate) unsafe extern "C" fn Task_TrainerHillWaitForPaletteFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gTasks[taskId].func = Some(Task_CloseTrainerHillRecordsOnButton);
    }
}
pub(crate) unsafe extern "C" fn Task_CloseTrainerHillRecordsOnButton(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        (*task).func = Some(Task_BeginPaletteFade);
    }
}
pub(crate) unsafe extern "C" fn Task_BeginPaletteFade(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    gTasks[taskId].func = Some(Task_ExitTrainerHillRecords);
}
pub(crate) unsafe extern "C" fn Task_ExitTrainerHillRecords(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        Free(sTilemapBuffer as *mut c_void);
        RemoveTrainerHillRecordsWindow(0);
        FreeAllWindowBuffers();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn RemoveTrainerHillRecordsWindow(windowId: u8) {
    FillWindowPixelBuffer(windowId, 0);
    ClearWindowTilemap(windowId);
    CopyWindowToVram(windowId, COPYWIN_GFX);
    RemoveWindow(windowId);
}
pub(crate) unsafe extern "C" fn ClearVramOamPlttRegs() {
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    {
        {
            let mut _dest: *mut u32 = OAM as i32 as usize as *mut u32;
            let mut _size: u32 = OAM_SIZE;
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    {
        {
            let mut _dest: *mut u16 = PLTT as i32 as usize as *mut u16;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
}
pub(crate) unsafe extern "C" fn ClearTasksAndGraphicalStructs() {
    ScanlineEffect_Stop();
    ResetTasks();
    ResetSpriteData();
    ResetPaletteFade();
    FreeAllSpritePalettes();
}
pub(crate) unsafe extern "C" fn ResetBgCoordinates() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
}
pub(crate) unsafe extern "C" fn SetDispcntReg() {
    SetGpuReg(REG_OFFSET_DISPCNT, 2368);
}
pub(crate) unsafe extern "C" fn LoadTrainerHillRecordsWindowGfx(bgId: u8) {
    LoadBgTiles(
        bgId,
        sTrainerHillWindowTileset.as_ptr().cast_mut() as *mut c_void,
        192,
        0,
    );
    CopyToBgTilemapBufferRect(
        bgId,
        sTrainerHillWindowTilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0x20,
        0x20,
    );
    LoadPalette(
        sTrainerHillWindowPalette.as_ptr().cast_mut() as *mut c_void,
        0,
        32,
    );
}
pub(crate) unsafe extern "C" fn VblankCB_TrainerHillRecords() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn MainCB2_TrainerHillRecords() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerHillRecords() {
    SetVBlankCallback(None);
    SetMainCallback2(Some(CB2_ShowTrainerHillRecords));
}
pub(crate) unsafe extern "C" fn CB2_ShowTrainerHillRecords() {
    match gMain.state {
        0 => {
            SetVBlankCallback(None);
            ClearVramOamPlttRegs();
            gMain.state += 1;
        }
        1 => {
            ClearTasksAndGraphicalStructs();
            gMain.state += 1;
        }
        2 => {
            sTilemapBuffer = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sTrainerHillRecordsBgTemplates.as_ptr().cast_mut(), 2);
            SetBgTilemapBuffer(3, sTilemapBuffer as *mut c_void);
            ResetBgCoordinates();
            gMain.state += 1;
        }
        3 => {
            LoadTrainerHillRecordsWindowGfx(3);
            LoadPalette(GetTextWindowPalette(0) as *mut c_void, 240, 32);
            gMain.state += 1;
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() != TRUE {
                ShowBg(0);
                ShowBg(3);
                CopyBgTilemapBufferToVram(3);
                gMain.state += 1;
            }
        }
        5 => {
            InitWindows(sTrainerHillRecordsWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            gMain.state += 1;
        }
        6 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            gMain.state += 1;
        }
        7 => {
            SetDispcntReg();
            SetVBlankCallback(Some(VblankCB_TrainerHillRecords));
            PrintOnTrainerHillRecordsWindow();
            CreateTask(Some(Task_TrainerHillWaitForPaletteFade), 8);
            SetMainCallback2(Some(MainCB2_TrainerHillRecords));
            gMain.state = 0;
        }
        _ => {}
    }
}
