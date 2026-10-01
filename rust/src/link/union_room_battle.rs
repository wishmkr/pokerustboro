//! The hand-off screen between accepting a Union Room battle and the battle
//! itself: both players confirm, then the chosen two Pokémon are set up.

use crate::battle_setup::gTrainerBattleOpponent_A;
use crate::bg::{
    FillBgTilemapBufferRect, InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
use crate::ffi::{
    BgTemplate, COPYWIN_FULL_MODE, DUMMY_WIN_TEMPLATE, FONT_NORMAL, MainCallback, PARTY_SIZE,
    POKEMON_SIZE, WindowTemplate, main_state,
};
use crate::gpu_regs::SetGpuReg;
use crate::link::gReceivedRemoteLinkPlayers;
use crate::malloc::AllocZeroed;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::task::{ResetTasks, RunTasks};
use crate::text_window::{DrawTextBorderOuter, LoadUserWindowBorderGfx, LoadUserWindowBorderGfx_};
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, InitWindows, PutWindowTilemap,
};

const BATTLE_TYPE_LINK_TRAINER: u16 = 0x0a;
const UNION_ROOM_PARTY_SIZE: usize = 2;
const GAME_STAT_NUM_UNION_ROOM_BATTLES: u8 = 0x32;
const TRAINER_UNION_ROOM: u16 = 0xc00;
const DECLINE: u16 = 0x52;
const ACCEPT: u16 = 0x51;
/// Bytes per player row of `gBlockRecvBuffer`.
const BLOCK_RECV_ROW: usize = 0x100;
const DISPLAY_TILE_WIDTH: u8 = 30;
const DISPLAY_TILE_HEIGHT: u8 = 20;
const PALETTES_ALL: u32 = 0xffff_ffff;
const RGB_BLACK: u16 = 0;

/// `struct UnionRoomBattle { s16 textState; }`
#[unsafe(link_section = "ewram_data")]
static mut BATTLE: *mut i16 = core::ptr::null_mut();

static BG_TEMPLATES: [BgTemplate; 1] = [BgTemplate::new(0, 3, 31, 0, 0, 0, 0)];

static WINDOW_TEMPLATES: [WindowTemplate; 2] = [
    WindowTemplate {
        bg: 0,
        tilemap_left: 3,
        tilemap_top: 15,
        width: 24,
        height: 4,
        palette_num: 14,
        base_block: 0x014,
    },
    DUMMY_WIN_TEMPLATE,
];

/// White background, dark grey text, light grey shadow.
static TEXT_COLORS: [u8; 3] = [1, 2, 3];

/// `StartUnionRoomBattle` with this module's view of its types.
#[inline]
unsafe fn StartUnionRoomBattle(a0: u16) {
    unsafe {
        crate::union_room::StartUnionRoomBattle(a0);
    }
}
/// `ZeroMonData` with this module's view of its types.
#[inline]
unsafe fn ZeroMonData(a0: *mut u8) {
    unsafe {
        crate::pokemon::ZeroMonData(a0 as _);
    }
}
/// `IncrementGameStat` with this module's view of its types.
#[inline]
unsafe fn IncrementGameStat(a0: u8) {
    unsafe {
        crate::overworld::IncrementGameStat(a0);
    }
}
/// `CalculatePlayerPartyCount` with this module's view of its types.
#[inline]
unsafe fn CalculatePlayerPartyCount() -> u8 {
    unsafe { crate::pokemon::CalculatePlayerPartyCount() }
}
/// `CB2_InitBattle` with this module's view of its types.
#[inline]
unsafe fn CB2_InitBattle() {
    unsafe {
        crate::battle_main::CB2_InitBattle();
    }
}
/// `CB2_ReturnToField` with this module's view of its types.
#[inline]
unsafe fn CB2_ReturnToField() {
    unsafe {
        crate::overworld::CB2_ReturnToField();
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: MainCallback) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `SetVBlankCallback` with this module's view of its types.
#[inline]
unsafe fn SetVBlankCallback(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetVBlankCallback(a0);
    }
}
/// `BeginNormalPaletteFade` with this module's view of its types.
#[inline]
unsafe fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8 {
    unsafe { crate::palette::BeginNormalPaletteFade(a0, a1, a2, a3, a4) }
}
/// `UpdatePaletteFade` with this module's view of its types.
#[inline]
unsafe fn UpdatePaletteFade() -> u8 {
    unsafe { crate::palette::UpdatePaletteFade() }
}
/// `TransferPlttBuffer` with this module's view of its types.
#[inline]
unsafe fn TransferPlttBuffer() {
    unsafe {
        crate::palette::TransferPlttBuffer();
    }
}
/// `ResetTempTileDataBuffers` with this module's view of its types.
#[inline]
unsafe fn ResetTempTileDataBuffers() {
    unsafe {
        crate::menu::ResetTempTileDataBuffers();
    }
}
/// `DeactivateAllTextPrinters` with this module's view of its types.
#[inline]
unsafe fn DeactivateAllTextPrinters() {
    unsafe {
        crate::text::DeactivateAllTextPrinters();
    }
}
/// `Menu_LoadStdPal` with this module's view of its types.
#[inline]
unsafe fn Menu_LoadStdPal() {
    unsafe {
        crate::menu::Menu_LoadStdPal();
    }
}
/// `RunTextPrinters` with this module's view of its types.
#[inline]
unsafe fn RunTextPrinters() {
    unsafe {
        crate::text::RunTextPrinters();
    }
}
/// `IsTextPrinterActive` with this module's view of its types.
#[inline]
unsafe fn IsTextPrinterActive(a0: u8) -> u16 {
    unsafe { crate::text::IsTextPrinterActive(a0) }
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
/// `SendBlock` with this module's view of its types.
#[inline]
unsafe fn SendBlock(a0: u8, a1: *const u8, a2: u16) -> u8 {
    unsafe { crate::link::SendBlock(a0, a1 as _, a2) }
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
/// `SetCloseLinkCallback` with this module's view of its types.
#[inline]
unsafe fn SetCloseLinkCallback() {
    unsafe {
        crate::link::SetCloseLinkCallback();
    }
}
/// `SetLinkStandbyCallback` with this module's view of its types.
#[inline]
unsafe fn SetLinkStandbyCallback() {
    unsafe {
        crate::link::SetLinkStandbyCallback();
    }
}
/// `IsLinkTaskFinished` with this module's view of its types.
#[inline]
unsafe fn IsLinkTaskFinished() -> u8 {
    unsafe { crate::link::IsLinkTaskFinished() }
}
/// `GetMultiplayerId` with this module's view of its types.
#[inline]
unsafe fn GetMultiplayerId() -> u8 {
    unsafe { crate::link::GetMultiplayerId() }
}

#[inline]
unsafe fn mon(party: *mut u8, index: usize) -> *mut u8 {
    unsafe { party.add(index * POKEMON_SIZE) }
}

unsafe fn cb2_set_up_parties_and_start_battle() {
    unsafe { StartUnionRoomBattle(BATTLE_TYPE_LINK_TRAINER) };
    let player = (&raw mut (*(&raw const crate::pokemon::gPlayerParty)
        .cast::<u8>()
        .cast_mut()))
        .cast::<u8>();
    let enemy = (&raw mut (*(&raw const crate::pokemon::gEnemyParty)
        .cast::<u8>()
        .cast_mut()))
        .cast::<u8>();
    let order = (&raw const (*(&raw const crate::party_menu::gSelectedOrderFromParty)
        .cast::<[u8; 3]>()
        .cast_mut()))
        .cast::<u8>();
    for i in 0..UNION_ROOM_PARTY_SIZE {
        let slot = usize::from(unsafe { order.add(i).read() }).wrapping_sub(1);
        unsafe { core::ptr::copy(mon(player, slot), mon(enemy, i), POKEMON_SIZE) };
    }
    for i in 0..PARTY_SIZE {
        unsafe { ZeroMonData(mon(player, i)) };
    }
    unsafe { core::ptr::copy(enemy, player, UNION_ROOM_PARTY_SIZE * POKEMON_SIZE) };
    unsafe { IncrementGameStat(GAME_STAT_NUM_UNION_ROOM_BATTLES) };
    unsafe { CalculatePlayerPartyCount() };
    unsafe { (&raw mut gTrainerBattleOpponent_A).write(TRAINER_UNION_ROOM) };
    unsafe { SetMainCallback2(CB2_InitBattle) };
}

unsafe fn print_message(state: *mut i16, string: *const u8, speed: i8) -> bool {
    match unsafe { state.read() } {
        0 => {
            unsafe { DrawTextBorderOuter(0, 0x001, 0xd) };
            let background = TEXT_COLORS[0];
            unsafe { FillWindowPixelBuffer(0, (background << 4) | background) };
            unsafe {
                AddTextPrinterParameterized4(
                    0,
                    FONT_NORMAL,
                    0,
                    1,
                    0,
                    1,
                    TEXT_COLORS.as_ptr(),
                    speed,
                    string,
                )
            };
            unsafe { PutWindowTilemap(0) };
            unsafe { CopyWindowToVram(0, COPYWIN_FULL_MODE) };
            unsafe { state.write(1) };
        }
        1 => {
            if unsafe { IsTextPrinterActive(0) } == 0 {
                unsafe { state.write(0) };
                return true;
            }
        }
        _ => {}
    }
    false
}

unsafe fn vblank_cb() {
    unsafe { LoadOam() };
    unsafe { ProcessSpriteCopyRequests() };
    unsafe { TransferPlttBuffer() };
}

#[inline]
unsafe fn received(player: usize) -> u16 {
    let buffer = &raw const (*(&raw const crate::link::gBlockRecvBuffer).cast::<u8>());
    unsafe { buffer.add(player * BLOCK_RECV_ROW).cast::<u16>().read() }
}

#[unsafe(no_mangle)]
pub unsafe fn CB2_UnionRoomBattle() {
    let state = unsafe { main_state() };
    let text_state = unsafe { (&raw const BATTLE).read() };
    match unsafe { state.read() } {
        0 => {
            unsafe { SetGpuReg(0, 0) };
            let battle = unsafe { AllocZeroed(2) }.cast::<i16>();
            unsafe { (&raw mut BATTLE).write(battle) };
            unsafe { ResetSpriteData() };
            unsafe { FreeAllSpritePalettes() };
            ResetTasks();
            unsafe { ResetBgsAndClearDma3BusyFlags(0) };
            unsafe { InitBgsFromTemplates(0, BG_TEMPLATES.as_ptr().cast(), 1) };
            unsafe { ResetTempTileDataBuffers() };
            if unsafe { InitWindows(WINDOW_TEMPLATES.as_ptr()) } == 0 {
                return;
            }
            unsafe { DeactivateAllTextPrinters() };
            unsafe { ClearWindowTilemap(0) };
            unsafe { FillWindowPixelBuffer(0, 0x00) };
            unsafe { FillWindowPixelBuffer(0, 0x11) };
            unsafe {
                FillBgTilemapBufferRect(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT, 0xf)
            };
            unsafe { LoadUserWindowBorderGfx(0, 1, 13 * 16) };
            unsafe { LoadUserWindowBorderGfx_(0, 1, 13 * 16) };
            unsafe { Menu_LoadStdPal() };
            unsafe { SetVBlankCallback(Some(vblank_cb)) };
            unsafe { state.write(1) };
        }
        1 => {
            if unsafe {
                print_message(
                    text_state,
                    &raw const (*(&raw const crate::data::strings::gText_CommStandbyAwaitingOtherPlayer).cast::<u8>()),
                    0,
                )
            } {
                unsafe { state.write(2) };
            }
        }
        2 => {
            unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, RGB_BLACK) };
            unsafe { ShowBg(0) };
            unsafe { state.write(3) };
        }
        3 => {
            if unsafe { UpdatePaletteFade() } == 0 {
                let send = (&raw mut (*(&raw const crate::link::gBlockSendBuffer)
                    .cast::<[u8; 0x100]>()
                    .cast_mut()))
                    .cast::<u8>();
                unsafe { send.write_bytes(0, 0x20) };
                let order = (&raw const (*(&raw const crate::party_menu::gSelectedOrderFromParty)
                    .cast::<[u8; 3]>()
                    .cast_mut()))
                    .cast::<u8>();
                let first = i32::from(unsafe { order.read() });
                let second = i32::from(unsafe { order.add(1).read() });
                // Only true when no Pokémon were chosen at all.
                let response = if first == -second { DECLINE } else { ACCEPT };
                unsafe { send.write(response as u8) };
                unsafe { SendBlock(0, send, 0x20) };
                unsafe { state.write(4) };
            }
        }
        4 => {
            if unsafe { GetBlockReceivedStatus() } == 3 {
                if unsafe { received(0) } == ACCEPT && unsafe { received(1) } == ACCEPT {
                    unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, RGB_BLACK) };
                    unsafe { state.write(50) };
                } else {
                    unsafe { SetCloseLinkCallback() };
                    let me = usize::from(unsafe { GetMultiplayerId() });
                    let next = if unsafe { received(me) } == DECLINE {
                        6
                    } else {
                        8
                    };
                    unsafe { state.write(next) };
                }
                unsafe { ResetBlockReceivedFlags() };
            }
        }
        50 => {
            if unsafe { UpdatePaletteFade() } == 0 {
                unsafe { SetLinkStandbyCallback() };
                unsafe { state.write(51) };
            }
        }
        51 => {
            if unsafe { IsLinkTaskFinished() } != 0 {
                unsafe { SetMainCallback2(cb2_set_up_parties_and_start_battle) };
            }
        }
        6 | 8 => {
            if unsafe { (&raw const gReceivedRemoteLinkPlayers).read() } == 0 {
                let current = unsafe { state.read() };
                unsafe { state.write(current + 1) };
            }
        }
        7 => {
            if unsafe {
                print_message(
                    text_state,
                    &raw const (*(&raw const crate::data::strings::gText_RefusedBattle)
                        .cast::<u8>()),
                    1,
                )
            } {
                unsafe { SetMainCallback2(CB2_ReturnToField) };
            }
        }
        9 => {
            if unsafe {
                print_message(
                    text_state,
                    &raw const (*(&raw const crate::data::strings::gText_BattleWasRefused)
                        .cast::<u8>()),
                    1,
                )
            } {
                unsafe { SetMainCallback2(CB2_ReturnToField) };
            }
        }
        _ => {}
    }
    RunTasks();
    unsafe { RunTextPrinters() };
    unsafe { AnimateSprites() };
    unsafe { BuildOamBuffer() };
    unsafe { UpdatePaletteFade() };
}
