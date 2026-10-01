//! The MYSTERY EVENT menu: receive an event script over the link cable from
//! another game, run it and save. The script itself runs in
//! mystery_event_script.c.

use crate::bg::{
    FillBgTilemapBufferRect_Palette0, InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
use crate::ffi::{
    A_BUTTON, B_BUTTON, BgTemplate, COPYWIN_FULL_MODE, CreateTask, DUMMY_WIN_TEMPLATE,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, MainCallback, PlaySE, SE_SELECT,
    WindowTemplate, gStringVar4, joy_new, main_state, palette_fade_active,
};
use crate::gpu_regs::SetGpuReg;
use crate::link::{gLinkType, gReceivedRemoteLinkPlayers};
use crate::mystery_event_script::RunMysteryEventScript;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::StringCopy;
use crate::task::{ResetTasks, RunTasks};
use crate::text::{DeactivateAllTextPrinters, IsTextPrinterActive, RunTextPrinters};
use crate::text_window::LoadUserWindowBorderGfx;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, InitWindows, PutWindowTilemap};

const WIN_MSG: u8 = 0;
const WIN_LOADING: u8 = 1;

const MEVENT_STATUS_LOAD_OK: u32 = 0;
const MEVENT_STATUS_LOAD_ERROR: u32 = 1;
const MEVENT_STATUS_SUCCESS: u32 = 2;
const LINKTYPE_MYSTERY_EVENT: u16 = 0x5501;
const LINK_STAT_MASTER: u32 = 0x20;
const LINK_STAT_PLAYER_COUNT: u32 = 0x1c;
const LINK_STAT_CONN_ESTABLISHED: u32 = 0x40;
const SE_PIN: u16 = 0x15;
const EXCHANGE_DIFF_SELECTIONS: u8 = 3;
const SAVE_NORMAL: u8 = 0;
const LINK_PLAYER_SIZE: usize = 0x1c;
const LINK_PLAYER_LANGUAGE: usize = 0x1a;
const REG_OFFSET_DISPCNT: u8 = 0;
const REG_OFFSET_BLDCNT: u8 = 0x50;
const DISPCNT_MODE0_OBJ1D_BG0: u16 = 0x140;
const PALETTES_ALL: u32 = 0xffff_ffff;
const RGB_BLACK: u16 = 0;

#[unsafe(link_section = "ewram_data")]
static mut UNUSED: u8 = 0;

static BG_TEMPLATES: [BgTemplate; 1] = [BgTemplate::new(0, 2, 31, 0, 0, 0, 0)];

static WINDOW_TEMPLATES: [WindowTemplate; 3] = [
    WindowTemplate {
        bg: 0,
        tilemap_left: 4,
        tilemap_top: 15,
        width: 22,
        height: 4,
        palette_num: 14,
        base_block: 20,
    },
    WindowTemplate {
        bg: 0,
        tilemap_left: 7,
        tilemap_top: 6,
        width: 16,
        height: 4,
        palette_num: 14,
        base_block: 0x6c,
    },
    DUMMY_WIN_TEMPLATE,
];

static TEXT_COLOR: [u8; 3] = [1, 2, 3];

/// `SetVBlankCallback` with this module's view of its types.
#[inline]
unsafe fn SetVBlankCallback(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetVBlankCallback(a0);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: MainCallback) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `TransferPlttBuffer` with this module's view of its types.
#[inline]
unsafe fn TransferPlttBuffer() {
    unsafe {
        crate::palette::TransferPlttBuffer();
    }
}
/// `UpdatePaletteFade` with this module's view of its types.
#[inline]
unsafe fn UpdatePaletteFade() -> u8 {
    unsafe { crate::palette::UpdatePaletteFade() }
}
/// `BeginNormalPaletteFade` with this module's view of its types.
#[inline]
unsafe fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8 {
    unsafe { crate::palette::BeginNormalPaletteFade(a0, a1, a2, a3, a4) }
}
/// `FillPalette` with this module's view of its types.
#[inline]
unsafe fn FillPalette(a0: u16, a1: u16, a2: u16) {
    unsafe {
        crate::palette::FillPalette(a0, a1, a2);
    }
}
/// `Menu_LoadStdPalAt` with this module's view of its types.
#[inline]
unsafe fn Menu_LoadStdPalAt(a0: u16) {
    unsafe {
        crate::menu::Menu_LoadStdPalAt(a0);
    }
}
/// `Task_DestroySelf` with this module's view of its types.
#[inline]
unsafe fn Task_DestroySelf(a0: u8) {
    unsafe {
        crate::link::Task_DestroySelf(a0);
    }
}
/// `StopMapMusic` with this module's view of its types.
#[inline]
unsafe fn StopMapMusic() {
    {
        crate::sound::StopMapMusic();
    }
}
/// `DoSoftReset` with this module's view of its types.
#[inline]
unsafe fn DoSoftReset() {
    unsafe {
        crate::agb_main::DoSoftReset();
    }
}
/// `AddTextPrinterParameterized4` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized4(
    a0: u8,
    a1: u8,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: *const u8,
    a7: i8,
    a8: *const u8,
) {
    unsafe {
        crate::menu::AddTextPrinterParameterized4(a0, a1, a2, a3, a4, a5, a6 as _, a7, a8 as _);
    }
}
/// `OpenLink` with this module's view of its types.
#[inline]
unsafe fn OpenLink() {
    unsafe {
        crate::link::OpenLink();
    }
}
/// `CloseLink` with this module's view of its types.
#[inline]
unsafe fn CloseLink() {
    unsafe {
        crate::link::CloseLink();
    }
}
/// `GetLinkPlayerCount_2` with this module's view of its types.
#[inline]
unsafe fn GetLinkPlayerCount_2() -> u8 {
    unsafe { crate::link::GetLinkPlayerCount_2() }
}
/// `CheckShouldAdvanceLinkState` with this module's view of its types.
#[inline]
unsafe fn CheckShouldAdvanceLinkState() {
    unsafe {
        crate::link::CheckShouldAdvanceLinkState();
    }
}
/// `IsLinkConnectionEstablished` with this module's view of its types.
#[inline]
unsafe fn IsLinkConnectionEstablished() -> u8 {
    unsafe { crate::link::IsLinkConnectionEstablished() }
}
/// `GetLinkPlayerDataExchangeStatusTimed` with this module's view of its types.
#[inline]
unsafe fn GetLinkPlayerDataExchangeStatusTimed(a0: i32, a1: i32) -> u8 {
    unsafe { crate::link::GetLinkPlayerDataExchangeStatusTimed(a0, a1) }
}
/// `SetCloseLinkCallback` with this module's view of its types.
#[inline]
unsafe fn SetCloseLinkCallback() {
    unsafe {
        crate::link::SetCloseLinkCallback();
    }
}
/// `GetBlockReceivedStatus` with this module's view of its types.
#[inline]
unsafe fn GetBlockReceivedStatus() -> u8 {
    unsafe { crate::link::GetBlockReceivedStatus() }
}
/// `ResetBlockReceivedFlags` with this module's view of its types.
#[inline]
unsafe fn ResetBlockReceivedFlags() {
    unsafe {
        crate::link::ResetBlockReceivedFlags();
    }
}
/// `IsLinkMaster` with this module's view of its types.
#[inline]
unsafe fn IsLinkMaster() -> u8 {
    unsafe { crate::link::IsLinkMaster() }
}
/// `TrySavingData` with this module's view of its types.
#[inline]
unsafe fn TrySavingData(a0: u8) -> u8 {
    unsafe { crate::save::TrySavingData(a0) }
}

unsafe fn vblank_cb() {
    unsafe { LoadOam() };
    unsafe { ProcessSpriteCopyRequests() };
    unsafe { TransferPlttBuffer() };
}

unsafe fn language_matches() -> bool {
    let players = &raw const (*(&raw const crate::link::gLinkPlayers).cast::<u8>());
    let first = unsafe { players.add(LINK_PLAYER_LANGUAGE).cast::<u16>().read() };
    let second = unsafe {
        players
            .add(LINK_PLAYER_SIZE + LINK_PLAYER_LANGUAGE)
            .cast::<u16>()
            .read()
    };
    first == second
}

#[unsafe(no_mangle)]
pub unsafe fn CB2_InitMysteryEventMenu() {
    unsafe { ResetSpriteData() };
    unsafe { FreeAllSpritePalettes() };
    ResetTasks();
    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    unsafe { ResetBgsAndClearDma3BusyFlags(0) };
    unsafe { InitBgsFromTemplates(0, BG_TEMPLATES.as_ptr().cast(), 1) };
    if unsafe { InitWindows(WINDOW_TEMPLATES.as_ptr()) } == 0 {
        return;
    }
    unsafe { DeactivateAllTextPrinters() };
    for window in 0..(WINDOW_TEMPLATES.len() - 1) as u8 {
        unsafe { FillWindowPixelBuffer(window, 0) };
    }
    unsafe { FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 30, 20) };
    unsafe { LoadUserWindowBorderGfx(0, 1, 13 * 16) };
    unsafe { Menu_LoadStdPalAt(14 * 16) };
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_MODE0_OBJ1D_BG0) };
    unsafe { SetGpuReg(REG_OFFSET_BLDCNT, 0) };
    unsafe { CreateTask(Task_DestroySelf, 0) };
    unsafe { StopMapMusic() };
    RunTasks();
    unsafe { AnimateSprites() };
    unsafe { BuildOamBuffer() };
    unsafe { RunTextPrinters() };
    unsafe { UpdatePaletteFade() };
    unsafe { FillPalette(RGB_BLACK, 0, 2) };
    unsafe { SetMainCallback2(cb2_mystery_event_menu) };
}

/// Copies the message for `status` into `dest` (if it has one) and says
/// whether the load failed.
unsafe fn get_event_load_message(dest: *mut u8, status: u32) -> bool {
    let mut failed = true;
    if status == MEVENT_STATUS_LOAD_OK {
        unsafe {
            StringCopy(
                dest,
                &raw const (*(&raw const crate::data::strings::gText_EventSafelyLoaded)
                    .cast::<u8>()),
            )
        };
        failed = false;
    }
    if status == MEVENT_STATUS_SUCCESS {
        failed = false;
    }
    if status == MEVENT_STATUS_LOAD_ERROR {
        unsafe {
            StringCopy(
                dest,
                &raw const (*(&raw const crate::data::strings::gText_LoadErrorEndingSession)
                    .cast::<u8>()),
            )
        };
    }
    failed
}

unsafe fn print_text(window_id: u8, text: *const u8, speed: i8) {
    unsafe { FillWindowPixelBuffer(window_id, (TEXT_COLOR[0] << 4) | TEXT_COLOR[0]) };
    unsafe {
        AddTextPrinterParameterized4(
            window_id,
            FONT_NORMAL,
            1,
            2,
            0,
            1,
            TEXT_COLOR.as_ptr(),
            speed,
            text,
        )
    };
}

unsafe fn fail_with_load_error() -> u8 {
    let buffer = (&raw mut gStringVar4).cast::<u8>();
    unsafe { get_event_load_message(buffer, MEVENT_STATUS_LOAD_ERROR) };
    unsafe { print_text(WIN_MSG, buffer, 1) };
    13
}

unsafe fn cancel() -> u8 {
    unsafe { PlaySE(SE_SELECT) };
    unsafe { CloseLink() };
    15
}

unsafe fn cb2_mystery_event_menu() {
    let state = unsafe { main_state() };
    let current = unsafe { state.read() };
    let link_status =
        || unsafe { (&raw const (*crate::link::gLinkStatus.as_ptr().cast::<u32>())).read() };
    let mut next = current;

    match current {
        0 => {
            unsafe { DrawStdFrameWithCustomTileAndPalette(WIN_MSG, 1, 1, 0xd) };
            unsafe { PutWindowTilemap(WIN_MSG) };
            unsafe { CopyWindowToVram(WIN_MSG, COPYWIN_FULL_MODE) };
            unsafe { ShowBg(0) };
            unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, RGB_BLACK) };
            next = 1;
        }
        1 => {
            if !unsafe { palette_fade_active() } {
                unsafe {
                    print_text(
                        WIN_MSG,
                        &raw const (*(&raw const crate::data::strings::gText_LinkStandby2)
                            .cast::<u8>()),
                        1,
                    )
                };
                next = 2;
            }
        }
        2 => {
            if unsafe { IsTextPrinterActive(WIN_MSG) } == 0 {
                next = 3;
                unsafe { (&raw mut gLinkType).write(LINKTYPE_MYSTERY_EVENT) };
                unsafe { OpenLink() };
            }
        }
        3 => {
            if link_status() & LINK_STAT_MASTER != 0 && link_status() & LINK_STAT_PLAYER_COUNT > 4 {
                unsafe { PlaySE(SE_PIN) };
                unsafe {
                    print_text(
                        WIN_MSG,
                        &raw const (*(&raw const crate::data::strings::gText_PressAToLoadEvent)
                            .cast::<u8>()),
                        1,
                    )
                };
                next = 4;
            }
            if unsafe { joy_new(B_BUTTON) } {
                next = unsafe { cancel() };
            }
        }
        4 => {
            if unsafe { IsTextPrinterActive(WIN_MSG) } == 0 {
                next = 5;
            }
        }
        5 => {
            if unsafe { GetLinkPlayerCount_2() } == 2 {
                if unsafe { joy_new(A_BUTTON) } {
                    unsafe { PlaySE(SE_SELECT) };
                    unsafe { CheckShouldAdvanceLinkState() };
                    unsafe { DrawStdFrameWithCustomTileAndPalette(WIN_LOADING, 1, 1, 0xd) };
                    unsafe {
                        print_text(
                            WIN_LOADING,
                            &raw const (*(&raw const crate::data::strings::gText_LoadingEvent)
                                .cast::<u8>()),
                            0,
                        )
                    };
                    unsafe { PutWindowTilemap(WIN_LOADING) };
                    unsafe { CopyWindowToVram(WIN_LOADING, COPYWIN_FULL_MODE) };
                    next = 6;
                } else if unsafe { joy_new(B_BUTTON) } {
                    next = unsafe { cancel() };
                }
            } else {
                next = unsafe { fail_with_load_error() };
            }
        }
        6 => {
            if unsafe { IsLinkConnectionEstablished() } != 0 {
                if unsafe { (&raw const gReceivedRemoteLinkPlayers).read() } != 0 {
                    if unsafe { GetLinkPlayerDataExchangeStatusTimed(2, 2) }
                        == EXCHANGE_DIFF_SELECTIONS
                    {
                        unsafe { SetCloseLinkCallback() };
                        next = unsafe { fail_with_load_error() };
                    } else if unsafe { language_matches() } {
                        unsafe {
                            print_text(WIN_MSG, &raw const (*(&raw const crate::data::strings::gText_DontRemoveCableTurnOff).cast::<u8>()), 1)
                        };
                        next = 7;
                    } else {
                        unsafe { CloseLink() };
                        next = unsafe { fail_with_load_error() };
                    }
                }
            } else if unsafe { joy_new(B_BUTTON) } {
                next = unsafe { cancel() };
            }
        }
        7 => {
            if unsafe { IsTextPrinterActive(WIN_MSG) } == 0 {
                next = 8;
            }
        }
        8 => {
            if unsafe { GetBlockReceivedStatus() } != 0 {
                unsafe { ResetBlockReceivedFlags() };
                next = 9;
            }
        }
        9 => next = 10,
        10 => {
            unsafe { SetCloseLinkCallback() };
            next = 11;
        }
        11 => {
            if unsafe { (&raw const gReceivedRemoteLinkPlayers).read() } == 0 {
                let buffer = (&raw mut crate::decompress::gDecompressionBuffer).cast::<u8>();
                let status = unsafe { RunMysteryEventScript(buffer) } as u16;
                unsafe { buffer.write_bytes(0, 0x7d4) };
                if !unsafe {
                    get_event_load_message((&raw mut gStringVar4).cast(), u32::from(status))
                } {
                    unsafe { TrySavingData(SAVE_NORMAL) };
                }
                next = 12;
            }
        }
        12 => {
            unsafe { print_text(WIN_MSG, (&raw const gStringVar4).cast(), 1) };
            next = 13;
        }
        13 => {
            if unsafe { IsTextPrinterActive(WIN_MSG) } == 0 {
                next = 14;
                unsafe { (&raw mut UNUSED).write(0) };
            }
        }
        14 => {
            if unsafe { joy_new(A_BUTTON) } {
                unsafe { PlaySE(SE_SELECT) };
                next = 15;
            }
        }
        15 => {
            unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, RGB_BLACK) };
            next = 16;
        }
        16 => {
            if !unsafe { palette_fade_active() } {
                unsafe { DoSoftReset() };
            }
        }
        _ => {}
    }
    unsafe { state.write(next) };

    if link_status() & LINK_STAT_CONN_ESTABLISHED != 0 && unsafe { IsLinkMaster() } == 0 {
        unsafe { CloseLink() };
        let failed = unsafe { fail_with_load_error() };
        unsafe { state.write(failed) };
    }
    RunTasks();
    unsafe { AnimateSprites() };
    unsafe { BuildOamBuffer() };
    unsafe { RunTextPrinters() };
    unsafe { UpdatePaletteFade() };
}
