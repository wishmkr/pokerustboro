//! Translated from `src/battle_bg.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gBattleAnimBgTilemapBuffer: *mut u8;
    static mut gBattleEnvironment: u8;
    static gBattleEnvironmentAnimTilemap_Building: CArray<u32, 0>;
    static gBattleEnvironmentAnimTilemap_Cave: CArray<u32, 0>;
    static gBattleEnvironmentAnimTilemap_Rayquaza: CArray<u32, 0>;
    static gBattleEnvironmentAnimTilemap_Underwater: CArray<u32, 0>;
    static gBattleEnvironmentAnimTiles_Building: CArray<u32, 0>;
    static gBattleEnvironmentAnimTiles_Cave: CArray<u32, 0>;
    static gBattleEnvironmentAnimTiles_Rayquaza: CArray<u32, 0>;
    static gBattleEnvironmentAnimTiles_Underwater: CArray<u32, 0>;
    static gBattleEnvironmentPalette_BuildingGym: CArray<u32, 0>;
    static gBattleEnvironmentPalette_BuildingLeader: CArray<u32, 0>;
    static gBattleEnvironmentPalette_Frontier: CArray<u32, 0>;
    static gBattleEnvironmentPalette_Groudon: CArray<u32, 0>;
    static gBattleEnvironmentPalette_Kyogre: CArray<u32, 0>;
    static gBattleEnvironmentPalette_Rayquaza: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumAqua: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumDrake: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumGlacia: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumMagma: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumPhoebe: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumSidney: CArray<u32, 0>;
    static gBattleEnvironmentPalette_StadiumWallace: CArray<u32, 0>;
    static gBattleEnvironmentTilemap_Building: CArray<u32, 0>;
    static gBattleEnvironmentTilemap_Cave: CArray<u32, 0>;
    static gBattleEnvironmentTilemap_Rayquaza: CArray<u32, 0>;
    static gBattleEnvironmentTilemap_Stadium: CArray<u32, 0>;
    static gBattleEnvironmentTilemap_Water: CArray<u32, 0>;
    static gBattleEnvironmentTiles_Building: CArray<u32, 0>;
    static gBattleEnvironmentTiles_Cave: CArray<u32, 0>;
    static gBattleEnvironmentTiles_Rayquaza: CArray<u32, 0>;
    static gBattleEnvironmentTiles_Stadium: CArray<u32, 0>;
    static gBattleEnvironmentTiles_Water: CArray<u32, 0>;
    static mut gBattleOutcome: u8;
    static mut gBattleScripting: BattleScripting;
    static mut gBattleStruct: *mut BattleStruct;
    static gBattleTextboxPalette: CArray<u32, 0>;
    static gBattleTextboxTilemap: CArray<u32, 0>;
    static gBattleTextboxTiles: CArray<u32, 0>;
    static mut gBattleTypeFlags: u32;
    static gBattleVSFrame_Gfx: CArray<u32, 0>;
    static gBattleVSFrame_Pal: CArray<u32, 0>;
    static gBattleVSFrame_Tilemap: CArray<u32, 0>;
    static gBattleWindowTextPalette: CArray<u32, 0>;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static gGameVersion: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static gMultiBattleIntroBg_Opponent_Tilemap: CArray<u32, 0>;
    static gMultiBattleIntroBg_Player_Tilemap: CArray<u32, 0>;
    static mut gPartnerTrainerId: u16;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Draw: CArray<u8, 0>;
    static gText_Loss: CArray<u8, 0>;
    static gText_Win: CArray<u8, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static gTrainers: CArray<Trainer, 0>;
    static gUnusedBattleInitSprite: SpriteTemplate;
    static gVsLettersGfx: CArray<u32, 0>;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimateSprites();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BuildOamBuffer();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn Cos2(a0: u16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DisableInterrupts(a0: u16);
    fn EnableInterrupts(a0: u16);
    fn GetCurrentMapBattleScene() -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_LoadStdPalAt(a0: u16);
    fn PlaySE(a0: u16);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn Sin2(a0: u16) -> i16;
}

pub(crate) unsafe extern "C" fn UnusedBattleInit() {
    let mut spriteId: u8 = 0;
    ResetSpriteData();
    spriteId = CreateSprite((&raw const gUnusedBattleInitSprite).cast_mut(), 0, 0, 0);
    gSprites[spriteId].set_invisible(TRUE as u16);
    SetMainCallback2(Some(CB2_UnusedBattleInit));
}
pub(crate) unsafe extern "C" fn CB2_UnusedBattleInit() {
    AnimateSprites();
    BuildOamBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleInitBgsAndWindows() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattleBgsVideo() {
    DisableInterrupts(INTR_FLAG_HBLANK);
    EnableInterrupts(197);
    BattleInitBgsAndWindows();
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_DISPCNT, 45120);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleMenuWindowGfx() {
    LoadUserWindowBorderGfx(2, 0x12, 16);
    LoadUserWindowBorderGfx(2, 0x22, 16);
    LoadCompressedPalette(gBattleWindowTextPalette.as_ptr().cast_mut(), 80, 32);
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        Menu_LoadStdPalAt(112);
        LoadMessageBoxGfx(0, 0x30, 112);
        gPlttBufferUnfaded[118] = 0;
        CpuSet(
            &raw mut gPlttBufferUnfaded[118] as *mut c_void,
            &raw mut gPlttBufferFaded[118] as *mut c_void,
            1,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawMainBattleBackground() {
    if gBattleTypeFlags & 0x23f0902 != 0 {
        LZDecompressVram(
            gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
            0x6008000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
            0x600d000 as usize as *mut c_void,
        );
        LoadCompressedPalette(
            gBattleEnvironmentPalette_Frontier.as_ptr().cast_mut(),
            32,
            96,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_GROUDON != 0 {
        LZDecompressVram(
            gBattleEnvironmentTiles_Cave.as_ptr().cast_mut(),
            0x6008000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentTilemap_Cave.as_ptr().cast_mut(),
            0x600d000 as usize as *mut c_void,
        );
        LoadCompressedPalette(
            gBattleEnvironmentPalette_Groudon.as_ptr().cast_mut(),
            32,
            96,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE != 0 {
        LZDecompressVram(
            gBattleEnvironmentTiles_Water.as_ptr().cast_mut(),
            0x6008000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentTilemap_Water.as_ptr().cast_mut(),
            0x600d000 as usize as *mut c_void,
        );
        LoadCompressedPalette(gBattleEnvironmentPalette_Kyogre.as_ptr().cast_mut(), 32, 96);
    } else if gBattleTypeFlags & BATTLE_TYPE_RAYQUAZA != 0 {
        LZDecompressVram(
            gBattleEnvironmentTiles_Rayquaza.as_ptr().cast_mut(),
            0x6008000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentTilemap_Rayquaza.as_ptr().cast_mut(),
            0x600d000 as usize as *mut c_void,
        );
        LoadCompressedPalette(
            gBattleEnvironmentPalette_Rayquaza.as_ptr().cast_mut(),
            32,
            96,
        );
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
            let mut trainerClass: u8 = gTrainers[gTrainerBattleOpponent_A].trainerClass;
            if trainerClass == TRAINER_CLASS_LEADER {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_BuildingLeader.as_ptr().cast_mut(),
                    32,
                    96,
                );
                return;
            } else if trainerClass == TRAINER_CLASS_CHAMPION {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumWallace.as_ptr().cast_mut(),
                    32,
                    96,
                );
                return;
            }
        }
        match GetCurrentMapBattleScene() {
            MAP_BATTLE_SCENE_GYM => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_BuildingGym.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_MAGMA => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumMagma.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_AQUA => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumAqua.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_SIDNEY => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumSidney.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_PHOEBE => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumPhoebe.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_GLACIA => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumGlacia.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_DRAKE => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_StadiumDrake.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            MAP_BATTLE_SCENE_FRONTIER => {
                LZDecompressVram(
                    gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LoadCompressedPalette(
                    gBattleEnvironmentPalette_Frontier.as_ptr().cast_mut(),
                    32,
                    96,
                );
            }
            _ => {
                LZDecompressVram(
                    sBattleEnvironmentTable[gBattleEnvironment].tileset as *mut u32,
                    0x6008000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    sBattleEnvironmentTable[gBattleEnvironment].tilemap as *mut u32,
                    0x600d000 as usize as *mut c_void,
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
pub unsafe extern "C" fn LoadBattleTextboxAndBackground() {
    LZDecompressVram(
        gBattleTextboxTiles.as_ptr().cast_mut(),
        0x6000000 as usize as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        0,
        gBattleTextboxTilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(0);
    LoadCompressedPalette(gBattleTextboxPalette.as_ptr().cast_mut(), 0, 64);
    LoadBattleMenuWindowGfx();
    DrawMainBattleBackground();
}
pub(crate) unsafe extern "C" fn DrawLinkBattleParticipantPokeballs(
    taskId: u8,
    multiplayerId: u8,
    bgId: u8,
    destX: u8,
    destY: u8,
) {
    let mut i: i32 = 0;
    let mut pokeballStatuses: u16 = 0;
    let mut tiles: CArray<u16, 6> = zeroed();
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gTasks[taskId].data[5] != 0 {
            match multiplayerId {
                0 => {
                    pokeballStatuses = 0x3F & gTasks[taskId].data[3] as u16;
                }
                1 => {
                    pokeballStatuses = ((0xFC0 & gTasks[taskId].data[4] as i32) >> 6) as u16;
                }
                2 => {
                    pokeballStatuses = ((0xFC0 & gTasks[taskId].data[3] as i32) >> 6) as u16;
                }
                3 => {
                    pokeballStatuses = 0x3F & gTasks[taskId].data[4] as u16;
                }
                _ => {}
            }
        } else {
            match multiplayerId {
                0 => {
                    pokeballStatuses = 0x3F & gTasks[taskId].data[3] as u16;
                }
                1 => {
                    pokeballStatuses = 0x3F & gTasks[taskId].data[4] as u16;
                }
                2 => {
                    pokeballStatuses = ((0xFC0 & gTasks[taskId].data[3] as i32) >> 6) as u16;
                }
                3 => {
                    pokeballStatuses = ((0xFC0 & gTasks[taskId].data[4] as i32) >> 6) as u16;
                }
                _ => {}
            }
        }
        i = 0;
        while i < 3 {
            tiles[i] = shr_i32(
                pokeballStatuses as i32 & shl_i32(3, i as u32 * 2),
                i as u32 * 2,
            ) as u16
                + 0x6001;
            i += 1;
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
            pokeballStatuses = gTasks[taskId].data[3] as u16;
        } else {
            pokeballStatuses = gTasks[taskId].data[4] as u16;
        }
        i = 0;
        while i < 6 {
            tiles[i] = shr_i32(
                pokeballStatuses as i32 & shl_i32(3, i as u32 * 2),
                i as u32 * 2,
            ) as u16
                + 0x6001;
            i += 1;
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
pub(crate) unsafe extern "C" fn DrawLinkBattleVsScreenOutcomeText() {
    if gBattleOutcome == B_OUTCOME_DREW {
        BattlePutTextOnWindow(gText_Draw.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_DRAW);
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gBattleOutcome == B_OUTCOME_WON {
            match gLinkPlayers[gBattleScripting.multiplayerId].id {
                0 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                }
                1 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                }
                2 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                }
                3 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                }
                _ => {}
            }
        } else {
            match gLinkPlayers[gBattleScripting.multiplayerId].id {
                0 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                }
                1 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                }
                2 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                }
                3 => {
                    BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
                    BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
                }
                _ => {}
            }
        }
    } else if gBattleOutcome == B_OUTCOME_WON {
        if gLinkPlayers[gBattleScripting.multiplayerId].id != 0 {
            BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
            BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
        } else {
            BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
            BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
        }
    } else {
        if gLinkPlayers[gBattleScripting.multiplayerId].id != 0 {
            BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
            BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
        } else {
            BattlePutTextOnWindow(gText_Win.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_RIGHT);
            BattlePutTextOnWindow(gText_Loss.as_ptr().cast_mut(), B_WIN_VS_OUTCOME_LEFT);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitLinkBattleVsScreen(taskId: u8) {
    let mut linkPlayer: *mut LinkPlayer = null_mut();
    let mut name: *mut u8 = null_mut();
    let mut i: i32 = 0;
    let mut palId: i32 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                i = 0;
                while i < MAX_LINK_PLAYERS {
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
                    i += 1;
                }
            } else {
                let mut playerId: u8 = gBattleScripting.multiplayerId;
                let mut opponentId: u8 = playerId ^ BIT_SIDE;
                let mut opponentId_copy: u8 = opponentId;
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
            gTasks[taskId].data[0] += 1;
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
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            if gTasks[taskId].data[5] != 0 {
                gBattle_BG1_X = 65516 - (Sin2(gTasks[taskId].data[1] as u16) / 32) as u16;
                gBattle_BG2_X = 65396 - (Sin2(gTasks[taskId].data[2] as u16) / 32) as u16;
                gBattle_BG1_Y = 65500;
                gBattle_BG2_Y = 65500;
            } else {
                gBattle_BG1_X = 65516 - (Sin2(gTasks[taskId].data[1] as u16) / 32) as u16;
                gBattle_BG1_Y = (Cos2(gTasks[taskId].data[1] as u16) / 32) as u16 - 164;
                gBattle_BG2_X = 65396 - (Sin2(gTasks[taskId].data[2] as u16) / 32) as u16;
                gBattle_BG2_Y = (Cos2(gTasks[taskId].data[2] as u16) / 32) as u16 - 164;
            }
            if gTasks[taskId].data[2] != 0 {
                gTasks[taskId].data[2] -= 2;
                gTasks[taskId].data[1] += 2;
            } else {
                if gTasks[taskId].data[5] != 0 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawBattleEntryBackground() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        LZDecompressVram(
            gBattleVSFrame_Gfx.as_ptr().cast_mut(),
            0x6004000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gVsLettersGfx.as_ptr().cast_mut(),
            OBJ_VRAM0 as usize as *mut c_void,
        );
        LoadCompressedPalette(gBattleVSFrame_Pal.as_ptr().cast_mut(), 96, 32);
        SetBgAttribute(1, BG_ATTR_SCREENSIZE, 1);
        SetGpuReg(REG_OFFSET_BG1CNT, 0x5C04);
        CopyToBgTilemapBuffer(
            1,
            gBattleVSFrame_Tilemap.as_ptr().cast_mut() as *mut c_void,
            0,
            0,
        );
        CopyToBgTilemapBuffer(
            2,
            gBattleVSFrame_Tilemap.as_ptr().cast_mut() as *mut c_void,
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
                gBattleEnvironmentAnimTiles_Building.as_ptr().cast_mut(),
                0x6004000 as usize as *mut c_void,
            );
            LZDecompressVram(
                gBattleEnvironmentAnimTilemap_Building.as_ptr().cast_mut(),
                0x600e000 as usize as *mut c_void,
            );
        } else {
            SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 2);
            SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 2);
            CopyToBgTilemapBuffer(
                1,
                gMultiBattleIntroBg_Opponent_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                2,
                gMultiBattleIntroBg_Player_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_GROUDON != 0 {
        LZDecompressVram(
            gBattleEnvironmentAnimTiles_Cave.as_ptr().cast_mut(),
            0x6004000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentAnimTilemap_Cave.as_ptr().cast_mut(),
            0x600e000 as usize as *mut c_void,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE != 0 {
        LZDecompressVram(
            gBattleEnvironmentAnimTiles_Underwater.as_ptr().cast_mut(),
            0x6004000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentAnimTilemap_Underwater.as_ptr().cast_mut(),
            0x600e000 as usize as *mut c_void,
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_RAYQUAZA != 0 {
        LZDecompressVram(
            gBattleEnvironmentAnimTiles_Rayquaza.as_ptr().cast_mut(),
            0x6004000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleEnvironmentAnimTilemap_Rayquaza.as_ptr().cast_mut(),
            0x600e000 as usize as *mut c_void,
        );
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
            let mut trainerClass: u8 = gTrainers[gTrainerBattleOpponent_A].trainerClass;
            if trainerClass == TRAINER_CLASS_LEADER {
                LZDecompressVram(
                    gBattleEnvironmentAnimTiles_Building.as_ptr().cast_mut(),
                    0x6004000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentAnimTilemap_Building.as_ptr().cast_mut(),
                    0x600e000 as usize as *mut c_void,
                );
                return;
            } else if trainerClass == TRAINER_CLASS_CHAMPION {
                LZDecompressVram(
                    gBattleEnvironmentAnimTiles_Building.as_ptr().cast_mut(),
                    0x6004000 as usize as *mut c_void,
                );
                LZDecompressVram(
                    gBattleEnvironmentAnimTilemap_Building.as_ptr().cast_mut(),
                    0x600e000 as usize as *mut c_void,
                );
                return;
            }
        }
        if GetCurrentMapBattleScene() == MAP_BATTLE_SCENE_NORMAL {
            LZDecompressVram(
                sBattleEnvironmentTable[gBattleEnvironment].entryTileset as *mut u32,
                0x6004000 as usize as *mut c_void,
            );
            LZDecompressVram(
                sBattleEnvironmentTable[gBattleEnvironment].entryTilemap as *mut u32,
                0x600e000 as usize as *mut c_void,
            );
        } else {
            LZDecompressVram(
                gBattleEnvironmentAnimTiles_Building.as_ptr().cast_mut(),
                0x6004000 as usize as *mut c_void,
            );
            LZDecompressVram(
                gBattleEnvironmentAnimTilemap_Building.as_ptr().cast_mut(),
                0x600e000 as usize as *mut c_void,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadChosenBattleElement(caseId: u8) -> u8 {
    let mut ret: u8 = FALSE;
    'l1: {
        match caseId {
            0 => {
                LZDecompressVram(
                    gBattleTextboxTiles.as_ptr().cast_mut(),
                    0x6000000 as usize as *mut c_void,
                );
            }
            1 => {
                CopyToBgTilemapBuffer(
                    0,
                    gBattleTextboxTilemap.as_ptr().cast_mut() as *mut c_void,
                    0,
                    0,
                );
                CopyBgTilemapBufferToVram(0);
            }
            2 => {
                LoadCompressedPalette(gBattleTextboxPalette.as_ptr().cast_mut(), 0, 64);
            }
            3 => {
                if gBattleTypeFlags & 0x23f0902 != 0 {
                    LZDecompressVram(
                        gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                        0x6008000 as usize as *mut c_void,
                    );
                } else if gBattleTypeFlags & BATTLE_TYPE_GROUDON != 0 {
                    LZDecompressVram(
                        gBattleEnvironmentTiles_Cave.as_ptr().cast_mut(),
                        0x6008000 as usize as *mut c_void,
                    );
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        let mut trainerClass: u8 = gTrainers[gTrainerBattleOpponent_A].trainerClass;
                        if trainerClass == TRAINER_CLASS_LEADER {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                            break 'l1;
                        } else if trainerClass == TRAINER_CLASS_CHAMPION {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                            break 'l1;
                        }
                    }
                    match GetCurrentMapBattleScene() {
                        MAP_BATTLE_SCENE_GYM => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_MAGMA => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_AQUA => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_SIDNEY => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_PHOEBE => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_GLACIA => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_DRAKE => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Stadium.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_FRONTIER => {
                            LZDecompressVram(
                                gBattleEnvironmentTiles_Building.as_ptr().cast_mut(),
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                        _ => {
                            LZDecompressVram(
                                sBattleEnvironmentTable[gBattleEnvironment].tileset as *mut u32,
                                0x6008000 as usize as *mut c_void,
                            );
                        }
                    }
                }
            }
            4 => {
                if gBattleTypeFlags & 0x23f0902 != 0 {
                    LZDecompressVram(
                        gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                        0x600d000 as usize as *mut c_void,
                    );
                } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE_GROUDON != 0 {
                    if gGameVersion == VERSION_RUBY as u8 {
                        LZDecompressVram(
                            gBattleEnvironmentTilemap_Cave.as_ptr().cast_mut(),
                            0x600d000 as usize as *mut c_void,
                        );
                    } else {
                        LZDecompressVram(
                            gBattleEnvironmentTilemap_Water.as_ptr().cast_mut(),
                            0x600d000 as usize as *mut c_void,
                        );
                    }
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        let mut trainerClass: u8 = gTrainers[gTrainerBattleOpponent_A].trainerClass;
                        if trainerClass == TRAINER_CLASS_LEADER {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                            break 'l1;
                        } else if trainerClass == TRAINER_CLASS_CHAMPION {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                            break 'l1;
                        }
                    }
                    match GetCurrentMapBattleScene() {
                        MAP_BATTLE_SCENE_GYM => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_MAGMA => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_AQUA => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_SIDNEY => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_PHOEBE => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_GLACIA => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_DRAKE => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Stadium.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        MAP_BATTLE_SCENE_FRONTIER => {
                            LZDecompressVram(
                                gBattleEnvironmentTilemap_Building.as_ptr().cast_mut(),
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                        _ => {
                            LZDecompressVram(
                                sBattleEnvironmentTable[gBattleEnvironment].tilemap as *mut u32,
                                0x600d000 as usize as *mut c_void,
                            );
                        }
                    }
                }
            }
            5 => {
                if gBattleTypeFlags & 0x23f0902 != 0 {
                    LoadCompressedPalette(
                        gBattleEnvironmentPalette_Frontier.as_ptr().cast_mut(),
                        32,
                        96,
                    );
                } else if gBattleTypeFlags & BATTLE_TYPE_KYOGRE_GROUDON != 0 {
                    if gGameVersion == VERSION_RUBY as u8 {
                        LoadCompressedPalette(
                            gBattleEnvironmentPalette_Groudon.as_ptr().cast_mut(),
                            32,
                            96,
                        );
                    } else {
                        LoadCompressedPalette(
                            gBattleEnvironmentPalette_Kyogre.as_ptr().cast_mut(),
                            32,
                            96,
                        );
                    }
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        let mut trainerClass: u8 = gTrainers[gTrainerBattleOpponent_A].trainerClass;
                        if trainerClass == TRAINER_CLASS_LEADER {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_BuildingLeader.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                            break 'l1;
                        } else if trainerClass == TRAINER_CLASS_CHAMPION {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumWallace.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                            break 'l1;
                        }
                    }
                    match GetCurrentMapBattleScene() {
                        MAP_BATTLE_SCENE_GYM => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_BuildingGym.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_MAGMA => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumMagma.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_AQUA => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumAqua.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_SIDNEY => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumSidney.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_PHOEBE => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumPhoebe.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_GLACIA => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumGlacia.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_DRAKE => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_StadiumDrake.as_ptr().cast_mut(),
                                32,
                                96,
                            );
                        }
                        MAP_BATTLE_SCENE_FRONTIER => {
                            LoadCompressedPalette(
                                gBattleEnvironmentPalette_Frontier.as_ptr().cast_mut(),
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
    return ret;
}
