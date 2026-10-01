//! Translated from `src/battle_bg.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gGameVersion;
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattleAnimBgTilemapBuffer,
    gBattleEnvironment, gBattleOutcome, gBattleScripting, gBattleStruct, gBattleTypeFlags,
};
use crate::battle_message::BattlePutTextOnWindow;
use crate::battle_setup::{gPartnerTrainerId, gTrainerBattleOpponent_A};
use crate::bg::{CopyBgTilemapBufferToVram, ResetBgsAndClearDma3BusyFlags, SetBgAttribute};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{DisableInterrupts, EnableInterrupts, SetGpuReg};
use crate::link::gLinkPlayers;
use crate::menu::Menu_LoadStdPalAt;
use crate::overworld::GetCurrentMapBattleScene;
use crate::palette::LoadCompressedPalette;
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{AllocSpritePalette, AnimateSprites, BuildOamBuffer, ResetSpriteData};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
use crate::trig::{Cos2, Sin2};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
    }
}
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
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
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
// Data tables (translate with cdata.py): sUnrefArray sVsLetter_V_OamData sVsLetter_S_OamData sVsLetterAffineAnimCmds0 sVsLetterAffineAnimCmds1 sVsLetterAffineAnimTable sVsLetter_V_SpriteTemplate sVsLetter_S_SpriteTemplate sVsLettersSpriteSheet gBattleBgTemplates sStandardBattleWindowTemplates sBattleArenaWindowTemplates gBattleWindowTemplates sBattleEnvironmentTable

/// `struct BattleBackground`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BattleBackground {
    pub tileset: *mut core::ffi::c_void,
    pub tilemap: *mut core::ffi::c_void,
    pub entryTileset: *mut core::ffi::c_void,
    pub entryTilemap: *mut core::ffi::c_void,
    pub palette: *mut core::ffi::c_void,
}

unsafe impl Sync for BattleBackground {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<BattleBackground>() == 20);
    assert!(offset_of!(BattleBackground, tileset) == 0);
    assert!(offset_of!(BattleBackground, tilemap) == 4);
    assert!(offset_of!(BattleBackground, entryTileset) == 8);
    assert!(offset_of!(BattleBackground, entryTilemap) == 12);
    assert!(offset_of!(BattleBackground, palette) == 16);
};

const TAG_VS_LETTERS: u16 = 10000;

static gBattleBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::battle_bg::gBattleBgTemplates).cast());
static gBattleWindowTemplates: Table<CArray<*mut WindowTemplate, 2>> =
    Table((&raw const crate::data::battle_bg::gBattleWindowTemplates).cast());
static sBattleEnvironmentTable: Table<CArray<BattleBackground, 10>> =
    Table((&raw const crate::data::battle_bg::sBattleEnvironmentTable).cast());
static sVsLetter_S_SpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_bg::sVsLetter_S_SpriteTemplate).cast());
static sVsLetter_V_SpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_bg::sVsLetter_V_SpriteTemplate).cast());
static sVsLettersSpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::battle_bg::sVsLettersSpriteSheet).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn UnusedBattleInit() {
    ResetSpriteData();
    let spriteId: u8 = CreateSprite(
        (&raw const (*(&raw const crate::data::battle_main::gUnusedBattleInitSprite)
            .cast::<SpriteTemplate>()))
            .cast_mut(),
        0,
        0,
        0,
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
    SetMainCallback2(Some(CB2_UnusedBattleInit));
}
pub(crate) unsafe fn CB2_UnusedBattleInit() {
    AnimateSprites();
    BuildOamBuffer();
}
#[unsafe(no_mangle)]
pub unsafe fn BattleInitBgsAndWindows() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, gBattleBgTemplates.as_ptr().cast_mut(), 4);
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        gBattleScripting.windowsType = B_WIN_TYPE_ARENA;
        SetBgTilemapBuffer(1, gBattleAnimBgTilemapBuffer as *mut c_void);
        SetBgTilemapBuffer(2, gBattleAnimBgTilemapBuffer as *mut c_void);
    } else {
        gBattleScripting.windowsType = B_WIN_TYPE_NORMAL;
    }
    InitWindows(gBattleWindowTemplates[gBattleScripting.windowsType]);
    DeactivateAllTextPrinters();
}
pub unsafe fn InitBattleBgsVideo() {
    DisableInterrupts(INTR_FLAG_HBLANK);
    EnableInterrupts(197);
    BattleInitBgsAndWindows();
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_DISPCNT, 45120);
}
pub unsafe fn LoadBattleMenuWindowGfx() {
    LoadUserWindowBorderGfx(2, 0x12, 16);
    LoadUserWindowBorderGfx(2, 0x22, 16);
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleWindowTextPalette).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        80,
        32,
    );
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        Menu_LoadStdPalAt(112);
        LoadMessageBoxGfx(0, 0x30, 112);
        gPlttBufferUnfaded[118] = 0;
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[118] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[118] as *mut c_void,
            1,
        );
    }
}
pub unsafe fn DrawMainBattleBackground() {
    if gBattleTypeFlags & 0x23f0902 != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6008000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600d000_usize as *mut c_void,
        );
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Frontier)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            32,
            96,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_GROUDON != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Cave)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6008000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Cave)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600d000_usize as *mut c_void,
        );
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Groudon)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            32,
            96,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Water)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6008000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Water)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600d000_usize as *mut c_void,
        );
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Kyogre)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            32,
            96,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_RAYQUAZA != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Rayquaza)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6008000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Rayquaza)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600d000_usize as *mut c_void,
        );
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Rayquaza)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            32,
            96,
        );
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
            let trainerClass: u8 = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                .trainerClass;
            if trainerClass == TRAINER_CLASS_LEADER {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_BuildingLeader)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
                return;
            } else if trainerClass == TRAINER_CLASS_CHAMPION {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumWallace)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
                return;
            }
        }
        match GetCurrentMapBattleScene() {
            MAP_BATTLE_SCENE_GYM => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_BuildingGym)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_MAGMA => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumMagma)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_AQUA => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumAqua)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_SIDNEY => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumSidney)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_PHOEBE => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumPhoebe)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_GLACIA => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumGlacia)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_DRAKE => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumDrake)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_FRONTIER => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Frontier)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    32,
                    96,
                );
            }
            _ => {
                LZDecompressVram(
                    sBattleEnvironmentTable[gBattleEnvironment].tileset as *mut u32,
                    0x6008000_usize as *mut c_void,
                );
                LZDecompressVram(
                    sBattleEnvironmentTable[gBattleEnvironment].tilemap as *mut u32,
                    0x600d000_usize as *mut c_void,
                );
                LoadCompressedPalette(
                    sBattleEnvironmentTable[gBattleEnvironment].palette as *mut u32,
                    32,
                    96,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LoadBattleTextboxAndBackground() {
    LZDecompressVram(
        (*(&raw const crate::data::graphics::gBattleTextboxTiles).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x6000000_usize as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        0,
        (*(&raw const crate::data::graphics::gBattleTextboxTilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(0);
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleTextboxPalette).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        64,
    );
    LoadBattleMenuWindowGfx();
    DrawMainBattleBackground();
}
unsafe fn DrawLinkBattleParticipantPokeballs(
    taskId: u8,
    multiplayerId: u8,
    bgId: u8,
    destX: u8,
    destY: u8,
) {
    let mut pokeballStatuses: u16 = 0;
    let mut tiles: CArray<u16, 6> = zeroed();
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if task_get(taskId, 5) != 0 {
            match multiplayerId {
                0 => {
                    pokeballStatuses = 0x3F & task_get(taskId, 3) as u16;
                }
                1 => {
                    pokeballStatuses = ((0xFC0 & task_get(taskId, 4) as i32) >> 6) as u16;
                }
                2 => {
                    pokeballStatuses = ((0xFC0 & task_get(taskId, 3) as i32) >> 6) as u16;
                }
                3 => {
                    pokeballStatuses = 0x3F & task_get(taskId, 4) as u16;
                }
                _ => {}
            }
        } else {
            match multiplayerId {
                0 => {
                    pokeballStatuses = 0x3F & task_get(taskId, 3) as u16;
                }
                1 => {
                    pokeballStatuses = 0x3F & task_get(taskId, 4) as u16;
                }
                2 => {
                    pokeballStatuses = ((0xFC0 & task_get(taskId, 3) as i32) >> 6) as u16;
                }
                3 => {
                    pokeballStatuses = ((0xFC0 & task_get(taskId, 4) as i32) >> 6) as u16;
                }
                _ => {}
            }
        }
        for i in 0..3i32 {
            tiles[i] = shr_i32(
                pokeballStatuses as i32 & shl_i32(3, i as u32 * 2),
                i as u32 * 2,
            ) as u16
                + 0x6001;
        }
        CopyToBgTilemapBufferRect_ChangePalette(
            bgId,
            tiles.as_mut_ptr() as *mut c_void,
            destX,
            destY,
            3,
            1,
            0x11,
        );
        CopyBgTilemapBufferToVram(bgId);
    } else {
        if multiplayerId == gBattleScripting.multiplayerId {
            pokeballStatuses = task_get(taskId, 3) as u16;
        } else {
            pokeballStatuses = task_get(taskId, 4) as u16;
        }
        for i in 0..6i32 {
            tiles[i] = shr_i32(
                pokeballStatuses as i32 & shl_i32(3, i as u32 * 2),
                i as u32 * 2,
            ) as u16
                + 0x6001;
        }
        CopyToBgTilemapBufferRect_ChangePalette(
            bgId,
            tiles.as_mut_ptr() as *mut c_void,
            destX,
            destY,
            6,
            1,
            0x11,
        );
        CopyBgTilemapBufferToVram(bgId);
    }
}
unsafe fn DrawLinkBattleVsScreenOutcomeText() {
    if gBattleOutcome == B_OUTCOME_DREW {
        BattlePutTextOnWindow(
            (*(&raw const crate::data::battle_message::gText_Draw).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            B_WIN_VS_OUTCOME_DRAW,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gBattleOutcome == B_OUTCOME_WON {
            match gLinkPlayers[gBattleScripting.multiplayerId].id {
                0 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                }
                1 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                }
                2 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                }
                3 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                }
                _ => {}
            }
        } else {
            match gLinkPlayers[gBattleScripting.multiplayerId].id {
                0 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                }
                1 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                }
                2 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                }
                3 => {
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Win)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_LEFT,
                    );
                    BattlePutTextOnWindow(
                        (*(&raw const crate::data::battle_message::gText_Loss)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        B_WIN_VS_OUTCOME_RIGHT,
                    );
                }
                _ => {}
            }
        }
    } else if gBattleOutcome == B_OUTCOME_WON {
        if gLinkPlayers[gBattleScripting.multiplayerId].id != 0 {
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Win).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_RIGHT,
            );
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Loss).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_LEFT,
            );
        } else {
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Win).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_LEFT,
            );
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Loss).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_RIGHT,
            );
        }
    } else {
        if gLinkPlayers[gBattleScripting.multiplayerId].id != 0 {
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Win).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_LEFT,
            );
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Loss).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_RIGHT,
            );
        } else {
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Win).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_RIGHT,
            );
            BattlePutTextOnWindow(
                (*(&raw const crate::data::battle_message::gText_Loss).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                B_WIN_VS_OUTCOME_LEFT,
            );
        }
    }
}
pub unsafe fn InitLinkBattleVsScreen(taskId: u8) {
    let mut linkPlayer: *mut LinkPlayer = null_mut();
    let mut name: *mut u8 = null_mut();
    let mut palId: i32 = 0;
    match task_get(taskId, 0) {
        0 => {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                for i in 0..MAX_LINK_PLAYERS {
                    name = gLinkPlayers[i].name.as_mut_ptr();
                    linkPlayer = &raw mut gLinkPlayers[i];
                    match (*linkPlayer).id {
                        0 => {
                            BattlePutTextOnWindow(name, B_WIN_VS_MULTI_PLAYER_1);
                            DrawLinkBattleParticipantPokeballs(
                                taskId,
                                (*linkPlayer).id as u8,
                                1,
                                2,
                                4,
                            );
                        }
                        1 => {
                            BattlePutTextOnWindow(name, B_WIN_VS_MULTI_PLAYER_2);
                            DrawLinkBattleParticipantPokeballs(
                                taskId,
                                (*linkPlayer).id as u8,
                                2,
                                2,
                                4,
                            );
                        }
                        2 => {
                            BattlePutTextOnWindow(name, B_WIN_VS_MULTI_PLAYER_3);
                            DrawLinkBattleParticipantPokeballs(
                                taskId,
                                (*linkPlayer).id as u8,
                                1,
                                2,
                                8,
                            );
                        }
                        3 => {
                            BattlePutTextOnWindow(name, B_WIN_VS_MULTI_PLAYER_4);
                            DrawLinkBattleParticipantPokeballs(
                                taskId,
                                (*linkPlayer).id as u8,
                                2,
                                2,
                                8,
                            );
                        }
                        _ => {}
                    }
                }
            } else {
                let mut playerId: u8 = gBattleScripting.multiplayerId;
                let mut opponentId: u8 = playerId ^ BIT_SIDE;
                let opponentId_copy: u8 = opponentId;
                if gLinkPlayers[playerId].id != 0 {
                    opponentId = playerId;
                    playerId = opponentId_copy;
                }
                name = gLinkPlayers[playerId].name.as_mut_ptr();
                BattlePutTextOnWindow(name, B_WIN_VS_PLAYER);
                name = gLinkPlayers[opponentId].name.as_mut_ptr();
                BattlePutTextOnWindow(name, B_WIN_VS_OPPONENT);
                DrawLinkBattleParticipantPokeballs(taskId, playerId, 1, 2, 7);
                DrawLinkBattleParticipantPokeballs(taskId, opponentId, 2, 2, 7);
            }
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 => {
            palId = AllocSpritePalette(TAG_VS_LETTERS) as i32;
            gPlttBufferUnfaded[0x100 + palId * 16 + 15] = {
                gPlttBufferFaded[0x100 + palId * 16 + 15] = 32767;
                gPlttBufferFaded[0x100 + palId * 16 + 15]
            };
            (*gBattleStruct).linkBattleVsSpriteId_V = CreateSprite(
                (&raw const *sVsLetter_V_SpriteTemplate).cast_mut(),
                111,
                80,
                0,
            );
            (*gBattleStruct).linkBattleVsSpriteId_S = CreateSprite(
                (&raw const *sVsLetter_S_SpriteTemplate).cast_mut(),
                129,
                80,
                0,
            );
            gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].set_invisible(TRUE as u16);
            gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].set_invisible(TRUE as u16);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        2 => {
            if task_get(taskId, 5) != 0 {
                gBattle_BG1_X = 65516 - (Sin2(task_get(taskId, 1) as u16) / 32) as u16;
                gBattle_BG2_X = 65396 - (Sin2(task_get(taskId, 2) as u16) / 32) as u16;
                gBattle_BG1_Y = 65500;
                gBattle_BG2_Y = 65500;
            } else {
                gBattle_BG1_X = 65516 - (Sin2(task_get(taskId, 1) as u16) / 32) as u16;
                gBattle_BG1_Y = (Cos2(task_get(taskId, 1) as u16) / 32) as u16 - 164;
                gBattle_BG2_X = 65396 - (Sin2(task_get(taskId, 2) as u16) / 32) as u16;
                gBattle_BG2_Y = (Cos2(task_get(taskId, 2) as u16) / 32) as u16 - 164;
            }
            if task_get(taskId, 2) != 0 {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
                task_set(taskId, 1, task_get(taskId, 1) + 2);
            } else {
                if task_get(taskId, 5) != 0 {
                    DrawLinkBattleVsScreenOutcomeText();
                }
                PlaySE(SE_M_HARDEN);
                DestroyTask(taskId);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].set_invisible(FALSE as u16);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].set_invisible(FALSE as u16);
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S]
                    .oam
                    .set_tileNum(
                        gSprites[(*gBattleStruct).linkBattleVsSpriteId_S]
                            .oam
                            .tileNum()
                            + 0x40,
                    );
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].data[0] = 0;
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].data[0] = 1;
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].data[1] =
                    gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].x;
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].data[1] =
                    gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].x;
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_V].data[2] = 0;
                gSprites[(*gBattleStruct).linkBattleVsSpriteId_S].data[2] = 0;
            }
        }
        _ => {}
    }
}
pub unsafe fn DrawBattleEntryBackground() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleVSFrame_Gfx).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            0x6004000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gVsLettersGfx).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            OBJ_VRAM0 as usize as *mut c_void,
        );
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gBattleVSFrame_Pal).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            96,
            32,
        );
        SetBgAttribute(1, BG_ATTR_SCREENSIZE, 1);
        SetGpuReg(REG_OFFSET_BG1CNT, 0x5C04);
        CopyToBgTilemapBuffer(
            1,
            (*(&raw const crate::data::graphics::gBattleVSFrame_Tilemap).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
            0,
            0,
        );
        CopyToBgTilemapBuffer(
            2,
            (*(&raw const crate::data::graphics::gBattleVSFrame_Tilemap).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
            0,
            0,
        );
        CopyBgTilemapBufferToVram(1);
        CopyBgTilemapBufferToVram(2);
        SetGpuReg(REG_OFFSET_WININ, 54);
        SetGpuReg(REG_OFFSET_WINOUT, 54);
        gBattle_BG1_Y = 0xFF5C;
        gBattle_BG2_Y = 0xFF5C;
        LoadCompressedSpriteSheetUsingHeap((&raw const *sVsLettersSpriteSheet).cast_mut());
    } else if gBattleTypeFlags & 0x23f0902 != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER == 0
            || gPartnerTrainerId == TRAINER_STEVEN_PARTNER
        {
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Building)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                0x6004000_usize as *mut c_void,
            );
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Building)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                0x600e000_usize as *mut c_void,
            );
        } else {
            SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 2);
            SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 2);
            CopyToBgTilemapBuffer(
                1,
                (*(&raw const crate::data::graphics::gMultiBattleIntroBg_Opponent_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                2,
                (*(&raw const crate::data::graphics::gMultiBattleIntroBg_Player_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_GROUDON != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Cave)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6004000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Cave)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600e000_usize as *mut c_void,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Underwater)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6004000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Underwater)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600e000_usize as *mut c_void,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_RAYQUAZA != 0 {
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Rayquaza)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x6004000_usize as *mut c_void,
        );
        LZDecompressVram(
            (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Rayquaza)
                .cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
            0x600e000_usize as *mut c_void,
        );
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
            let trainerClass: u8 = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                .trainerClass;
            if trainerClass == TRAINER_CLASS_LEADER {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6004000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600e000_usize as *mut c_void,
                );
                return;
            } else if trainerClass == TRAINER_CLASS_CHAMPION {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6004000_usize as *mut c_void,
                );
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Building)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x600e000_usize as *mut c_void,
                );
                return;
            }
        }
        if GetCurrentMapBattleScene() == MAP_BATTLE_SCENE_NORMAL {
            LZDecompressVram(
                sBattleEnvironmentTable[gBattleEnvironment].entryTileset as *mut u32,
                0x6004000_usize as *mut c_void,
            );
            LZDecompressVram(
                sBattleEnvironmentTable[gBattleEnvironment].entryTilemap as *mut u32,
                0x600e000_usize as *mut c_void,
            );
        } else {
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTiles_Building)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                0x6004000_usize as *mut c_void,
            );
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gBattleEnvironmentAnimTilemap_Building)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                0x600e000_usize as *mut c_void,
            );
        }
    }
}
pub unsafe fn LoadChosenBattleElement(caseId: u8) -> u8 {
    let mut ret: u8 = FALSE;
    'l1: {
        match caseId {
            0 => {
                LZDecompressVram(
                    (*(&raw const crate::data::graphics::gBattleTextboxTiles)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0x6000000_usize as *mut c_void,
                );
            }
            1 => {
                CopyToBgTilemapBuffer(
                    0,
                    (*(&raw const crate::data::graphics::gBattleTextboxTilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                    0,
                    0,
                );
                CopyBgTilemapBufferToVram(0);
            }
            2 => {
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBattleTextboxPalette)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0,
                    64,
                );
            }
            3 => {
                if gBattleTypeFlags & 0x23f0902 != 0 {
                    LZDecompressVram(
                        (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building)
                            .cast::<CArray<u32, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        0x6008000_usize as *mut c_void,
                    );
                } else if gBattleTypeFlags & BATTLE_TYPE_GROUDON != 0 {
                    LZDecompressVram(
                        (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Cave)
                            .cast::<CArray<u32, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        0x6008000_usize as *mut c_void,
                    );
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        let trainerClass: u8 = (*(&raw const crate::data::data_tables::gTrainers)
                            .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                            .trainerClass;
                        if trainerClass == TRAINER_CLASS_LEADER {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                            break 'l1;
                        } else if trainerClass == TRAINER_CLASS_CHAMPION {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                            break 'l1;
                        }
                    }
                    match GetCurrentMapBattleScene() {
                        MAP_BATTLE_SCENE_GYM => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_MAGMA => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_AQUA => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_SIDNEY => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_PHOEBE => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_GLACIA => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_DRAKE => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_FRONTIER => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTiles_Building).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x6008000_usize as *mut c_void,
                            );
                        }
                        _ => {
                            LZDecompressVram(
                                sBattleEnvironmentTable[gBattleEnvironment].tileset as *mut u32,
                                0x6008000_usize as *mut c_void,
                            );
                        }
                    }
                }
            }
            4 => {
                if gBattleTypeFlags & 0x23f0902 != 0 {
                    LZDecompressVram(
                        (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building)
                            .cast::<CArray<u32, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        0x600d000_usize as *mut c_void,
                    );
                } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE_GROUDON != 0 {
                    if gGameVersion == VERSION_RUBY as u8 {
                        LZDecompressVram(
                            (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Cave)
                                .cast::<CArray<u32, 0>>())
                            .as_ptr()
                            .cast_mut(),
                            0x600d000_usize as *mut c_void,
                        );
                    } else {
                        LZDecompressVram(
                            (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Water)
                                .cast::<CArray<u32, 0>>())
                            .as_ptr()
                            .cast_mut(),
                            0x600d000_usize as *mut c_void,
                        );
                    }
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        let trainerClass: u8 = (*(&raw const crate::data::data_tables::gTrainers)
                            .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                            .trainerClass;
                        if trainerClass == TRAINER_CLASS_LEADER {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                            break 'l1;
                        } else if trainerClass == TRAINER_CLASS_CHAMPION {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                            break 'l1;
                        }
                    }
                    match GetCurrentMapBattleScene() {
                        MAP_BATTLE_SCENE_GYM => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_MAGMA => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_AQUA => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_SIDNEY => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_PHOEBE => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_GLACIA => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_DRAKE => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Stadium).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_FRONTIER => {
                            LZDecompressVram(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentTilemap_Building).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                0x600d000_usize as *mut c_void,
                            );
                        }
                        _ => {
                            LZDecompressVram(
                                sBattleEnvironmentTable[gBattleEnvironment].tilemap as *mut u32,
                                0x600d000_usize as *mut c_void,
                            );
                        }
                    }
                }
            }
            5 => {
                if gBattleTypeFlags & 0x23f0902 != 0 {
                    LoadCompressedPalette(
                        (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Frontier)
                            .cast::<CArray<u32, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        32,
                        96,
                    );
                } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE_GROUDON != 0 {
                    if gGameVersion == VERSION_RUBY as u8 {
                        LoadCompressedPalette(
                            (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Groudon).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                            32,
                            96,
                        );
                    } else {
                        LoadCompressedPalette(
                            (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Kyogre)
                                .cast::<CArray<u32, 0>>())
                            .as_ptr()
                            .cast_mut(),
                            32,
                            96,
                        );
                    }
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        let trainerClass: u8 = (*(&raw const crate::data::data_tables::gTrainers)
                            .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                            .trainerClass;
                        if trainerClass == TRAINER_CLASS_LEADER {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_BuildingLeader).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                            break 'l1;
                        } else if trainerClass == TRAINER_CLASS_CHAMPION {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumWallace).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                            break 'l1;
                        }
                    }
                    match GetCurrentMapBattleScene() {
                        MAP_BATTLE_SCENE_GYM => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_BuildingGym).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_MAGMA => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumMagma).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_AQUA => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumAqua).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_SIDNEY => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumSidney).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_PHOEBE => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumPhoebe).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_GLACIA => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumGlacia).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_DRAKE => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_StadiumDrake).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_FRONTIER => {
                            LoadCompressedPalette(
                                (*(&raw const crate::data::graphics::gBattleEnvironmentPalette_Frontier).cast::<CArray<u32, 0>>()).as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        _ => {
                            LoadCompressedPalette(
                                sBattleEnvironmentTable[gBattleEnvironment].palette as *mut u32,
                                32,
                                96,
                            );
                        }
                    }
                }
            }
            6 => {
                LoadBattleMenuWindowGfx();
            }
            _ => {
                ret = TRUE;
            }
        }
    }
    ret
}
