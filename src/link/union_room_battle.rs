//! The hand-off screen between accepting a Union Room battle and the battle
//! itself: both players confirm, then the chosen two Pokémon are set up.

use crate::bg::{
    FillBgTilemapBufferRect, InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
use crate::ffi::{
    BgTemplate, COPYWIN_FULL_MODE, DUMMY_WIN_TEMPLATE, FONT_NORMAL, MainCallback, PARTY_SIZE,
    POKEMON_SIZE, WindowTemplate, gEnemyParty, gPlayerParty, main_state,
};
use crate::gpu_regs::SetGpuReg;
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

unsafe extern "C" {
    static mut gSelectedOrderFromParty: [u8; 3];
    static mut gTrainerBattleOpponent_A: u16;
    static mut gBlockSendBuffer: [u8; 0x100];
    static gBlockRecvBuffer: u8;
    static gReceivedRemoteLinkPlayers: u8;
    static gText_CommStandbyAwaitingOtherPlayer: u8;
    static gText_RefusedBattle: u8;
    static gText_BattleWasRefused: u8;

    fn StartUnionRoomBattle(battle_flags: u16);
    fn ZeroMonData(mon: *mut u8);
    fn IncrementGameStat(index: u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn CB2_InitBattle();
    fn CB2_ReturnToField();
    fn SetMainCallback2(callback: MainCallback);
    fn SetVBlankCallback(callback: Option<unsafe extern "C" fn()>);
    fn BeginNormalPaletteFade(
        selected_palettes: u32,
        delay: i8,
        start_y: u8,
        target_y: u8,
        blend_color: u16,
    ) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn TransferPlttBuffer();
    fn ResetTempTileDataBuffers();
    fn DeactivateAllTextPrinters();
    fn Menu_LoadStdPal();
    fn RunTextPrinters();
    fn IsTextPrinterActive(id: u8) -> u16;
    fn AddTextPrinterParameterized4(
        window_id: u8,
        font_id: u8,
        left: u8,
        top: u8,
        letter_spacing: u8,
        line_spacing: u8,
        color: *const u8,
        speed: i8,
        string: *const u8,
    );
    fn SendBlock(unused: u8, src: *const u8, size: u16) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn ResetBlockReceivedFlags();
    fn SetCloseLinkCallback();
    fn SetLinkStandbyCallback();
    fn IsLinkTaskFinished() -> u8;
    fn GetMultiplayerId() -> u8;
}

#[inline]
unsafe fn mon(party: *mut u8, index: usize) -> *mut u8 {
    unsafe { party.add(index * POKEMON_SIZE) }
}

unsafe extern "C" fn cb2_set_up_parties_and_start_battle() {
    unsafe { StartUnionRoomBattle(BATTLE_TYPE_LINK_TRAINER) };
    let player = (&raw mut gPlayerParty).cast::<u8>();
    let enemy = (&raw mut gEnemyParty).cast::<u8>();
    let order = (&raw const gSelectedOrderFromParty).cast::<u8>();
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

unsafe extern "C" fn vblank_cb() {
    unsafe { LoadOam() };
    unsafe { ProcessSpriteCopyRequests() };
    unsafe { TransferPlttBuffer() };
}

#[inline]
unsafe fn received(player: usize) -> u16 {
    let buffer = &raw const gBlockRecvBuffer;
    unsafe { buffer.add(player * BLOCK_RECV_ROW).cast::<u16>().read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_UnionRoomBattle() {
    let state = unsafe { main_state() };
    let text_state = unsafe { (&raw const BATTLE).read() };
    match unsafe { state.read() } {
        0 => {
            unsafe { SetGpuReg(0, 0) };
            let battle = unsafe { AllocZeroed(2) }.cast::<i16>();
            unsafe { (&raw mut BATTLE).write(battle) };
            unsafe { ResetSpriteData() };
            unsafe { FreeAllSpritePalettes() };
            unsafe { ResetTasks() };
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
                    &raw const gText_CommStandbyAwaitingOtherPlayer,
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
                let send = (&raw mut gBlockSendBuffer).cast::<u8>();
                unsafe { send.write_bytes(0, 0x20) };
                let order = (&raw const gSelectedOrderFromParty).cast::<u8>();
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
            if unsafe { print_message(text_state, &raw const gText_RefusedBattle, 1) } {
                unsafe { SetMainCallback2(CB2_ReturnToField) };
            }
        }
        9 => {
            if unsafe { print_message(text_state, &raw const gText_BattleWasRefused, 1) } {
                unsafe { SetMainCallback2(CB2_ReturnToField) };
            }
        }
        _ => {}
    }
    unsafe { RunTasks() };
    unsafe { RunTextPrinters() };
    unsafe { AnimateSprites() };
    unsafe { BuildOamBuffer() };
    unsafe { UpdatePaletteFade() };
}
