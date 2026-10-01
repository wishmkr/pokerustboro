//! Translated from `src/battle_records.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_main::gBattleOutcome;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, IsDma3ManagerBusyWithBgCopy,
    ResetBgsAndClearDma3BusyFlags, ShowBg,
};
use crate::bg::{CopyToBgTilemapBufferRect, LoadBgTiles};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::link::gLinkPlayers;
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::{ClearStdWindowAndFrame, DrawStdWindowFrame};
use crate::overworld::{
    CB2_ReturnToFieldContinueScriptPlayMapMusic, GetGameStat, IncrementGameStat, SetGameStat,
};
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::PlaySE;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{
    ConvertIntToDecimalStringN, StringCompareN, StringCopyN, StringExpandPlaceholders,
};
use crate::string_util::{ConvertInternationalString, StringFillWithTerminator};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::gTasks;
use crate::task::task_set_func;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::text::DeactivateAllTextPrinters;
use crate::trainer_card::gTrainerCards;
use crate::trainer_hill::PrintOnTrainerHillRecordsWindow;
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::InUnionRoom;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers,
    PutWindowTilemap, RemoveWindow,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
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

#[unsafe(link_section = "ewram_data")]
pub static gRecordsWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapBuffer: *mut u8 = null_mut();

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetTextWindowPalette` with this module's view of its types.
#[inline]
unsafe fn GetTextWindowPalette(a0: u8) -> *mut u16 {
    unsafe { crate::text_window::GetTextWindowPalette(a0) as *mut u16 }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn ClearLinkBattleRecord(record: *mut LinkBattleRecord) {
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
unsafe fn ClearLinkBattleRecords(records: *mut LinkBattleRecord) {
    for i in 0..LINK_B_RECORDS_COUNT {
        ClearLinkBattleRecord(records.at(i));
    }
    SetGameStat(GAME_STAT_LINK_BATTLE_WINS, 0);
    SetGameStat(GAME_STAT_LINK_BATTLE_LOSSES, 0);
    SetGameStat(GAME_STAT_LINK_BATTLE_DRAWS, 0);
}
unsafe fn GetLinkBattleRecordTotalBattles(record: *mut LinkBattleRecord) -> i32 {
    (*record).wins as i32 + (*record).losses as i32 + (*record).draws as i32
}
unsafe fn FindLinkBattleRecord(
    records: *mut LinkBattleRecord,
    name: *mut u8,
    trainerId: u16,
) -> i32 {
    for i in 0..LINK_B_RECORDS_COUNT {
        if StringCompareN(
            (*records.at(i)).name.as_mut_ptr(),
            name,
            PLAYER_NAME_LENGTH as u32,
        ) == 0
            && (*records.at(i)).trainerId == trainerId
        {
            return i;
        }
    }
    LINK_B_RECORDS_COUNT
}
unsafe fn SortLinkBattleRecords(records: *mut LinkBattleRecords) {
    let mut j: i32 = 0;
    let mut i: i32 = 4;
    while i > 0 {
        j = i - 1;
        while j >= 0 {
            let totalBattlesI: i32 =
                GetLinkBattleRecordTotalBattles(&raw mut (*records).entries[i]);
            let totalBattlesJ: i32 =
                GetLinkBattleRecordTotalBattles(&raw mut (*records).entries[j]);
            if totalBattlesI > totalBattlesJ {
                let temp1: LinkBattleRecord = (*records).entries[i];
                (*records).entries[i] = (*records).entries[j];
                (*records).entries[j] = temp1;
                let temp2: u8 = (*records).languages[i];
                (*records).languages[i] = (*records).languages[j];
                (*records).languages[j] = temp2;
            }
            j -= 1;
        }
        i -= 1;
    }
}
unsafe fn UpdateLinkBattleRecord(record: *mut LinkBattleRecord, battleOutcome: i32) {
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
unsafe fn UpdateLinkBattleGameStats(battleOutcome: i32) {
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
unsafe fn UpdateLinkBattleRecords(
    records: *mut LinkBattleRecords,
    name: *mut u8,
    trainerId: u16,
    battleOutcome: i32,
    battler: u8,
) {
    UpdateLinkBattleGameStats(battleOutcome);
    SortLinkBattleRecords(records);
    let mut index: i32 = FindLinkBattleRecord((*records).entries.as_mut_ptr(), name, trainerId);
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
pub unsafe fn ClearPlayerLinkBattleRecords() {
    ClearLinkBattleRecords((*gSaveBlock1Ptr).linkBattleRecords.entries.as_mut_ptr());
}
unsafe fn IncTrainerCardWins(battler: i32) {
    let wins: *mut u16 = &raw mut gTrainerCards[battler].linkBattleWins;
    *wins += 1;
    if *wins > 9999 {
        *wins = 9999;
    }
}
unsafe fn IncTrainerCardLosses(battler: i32) {
    let losses: *mut u16 = &raw mut gTrainerCards[battler].linkBattleLosses;
    *losses += 1;
    if *losses > 9999 {
        *losses = 9999;
    }
}
unsafe fn UpdateTrainerCardWinsLosses(battler: i32) {
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
pub unsafe fn UpdatePlayerLinkBattleRecords(battler: i32) {
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
unsafe fn PrintLinkBattleWinsLossesDraws(records: *mut LinkBattleRecord) {
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
        (*(&raw const crate::data::strings::gText_TotalRecordWLD).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0xD0);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        0x11,
        0,
        None,
    );
}
unsafe fn PrintLinkBattleRecord(record: *mut LinkBattleRecord, y: u8, language: i32) {
    if (*record).wins == 0 && (*record).losses == 0 && (*record).draws == 0 {
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            sText_DashesNoPlayer.as_ptr().cast_mut(),
            8,
            y * 8 + 1,
            0,
            None,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            sText_DashesNoScore.as_ptr().cast_mut(),
            80,
            y * 8 + 1,
            0,
            None,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            sText_DashesNoScore.as_ptr().cast_mut(),
            128,
            y * 8 + 1,
            0,
            None,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
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
            gRecordsWindowId.get(),
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
            gRecordsWindowId.get(),
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
            gRecordsWindowId.get(),
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
            gRecordsWindowId.get(),
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
pub unsafe fn ShowLinkBattleRecords() {
    gRecordsWindowId.set(AddWindow((&raw const *sLinkBattleRecordsWindow).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PlayersBattleResults).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 208);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
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
        (*(&raw const crate::data::strings::gText_WinLoseDraw).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        41,
        0,
        None,
    );
    for i in 0..LINK_B_RECORDS_COUNT {
        PrintLinkBattleRecord(
            &raw mut (*gSaveBlock1Ptr).linkBattleRecords.entries[i],
            7 + i as u8 * 2,
            (*gSaveBlock1Ptr).linkBattleRecords.languages[i] as i32,
        );
    }
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
#[unsafe(no_mangle)]
pub unsafe fn RemoveRecordsWindow() {
    ClearStdWindowAndFrame(gRecordsWindowId.get(), FALSE);
    RemoveWindow(gRecordsWindowId.get());
}
pub(crate) unsafe fn Task_TrainerHillWaitForPaletteFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        task_set_func(taskId, Some(Task_CloseTrainerHillRecordsOnButton));
    }
}
pub(crate) unsafe fn Task_CloseTrainerHillRecordsOnButton(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        (*task).func = Some(Task_BeginPaletteFade);
    }
}
pub(crate) unsafe fn Task_BeginPaletteFade(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    task_set_func(taskId, Some(Task_ExitTrainerHillRecords));
}
pub(crate) unsafe fn Task_ExitTrainerHillRecords(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        Free(sTilemapBuffer as *mut c_void);
        RemoveTrainerHillRecordsWindow(0);
        FreeAllWindowBuffers();
        DestroyTask(taskId);
    }
}
unsafe fn RemoveTrainerHillRecordsWindow(windowId: u8) {
    FillWindowPixelBuffer(windowId, 0);
    ClearWindowTilemap(windowId);
    CopyWindowToVram(windowId, COPYWIN_GFX);
    RemoveWindow(windowId);
}
unsafe fn ClearVramOamPlttRegs() {
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
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
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
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | (_size / 4));
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
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
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
unsafe fn ClearTasksAndGraphicalStructs() {
    ScanlineEffect_Stop();
    ResetTasks();
    ResetSpriteData();
    ResetPaletteFade();
    FreeAllSpritePalettes();
}
unsafe fn ResetBgCoordinates() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
}
unsafe fn SetDispcntReg() {
    SetGpuReg(REG_OFFSET_DISPCNT, 2368);
}
unsafe fn LoadTrainerHillRecordsWindowGfx(bgId: u8) {
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
pub(crate) unsafe fn VblankCB_TrainerHillRecords() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn MainCB2_TrainerHillRecords() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe fn ShowTrainerHillRecords() {
    SetVBlankCallback(None);
    SetMainCallback2(Some(CB2_ShowTrainerHillRecords));
}
pub(crate) unsafe fn CB2_ShowTrainerHillRecords() {
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
